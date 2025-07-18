use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    pub type mode_change;
    pub type quoting_options;
    pub type saved_cwd;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn endgrent();
    fn getgrnam(__name: *const libc::c_char) -> *mut group;
    fn modf(_: libc::c_double, _: *mut libc::c_double) -> libc::c_double;
    fn endpwent();
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn rpl_re_set_syntax(__syntax: reg_syntax_t) -> reg_syntax_t;
    fn rpl_re_compile_pattern(
        __pattern: *const libc::c_char,
        __length: size_t,
        __buffer: *mut re_pattern_buffer,
    ) -> *const libc::c_char;
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
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
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn mode_compile(_: *const libc::c_char) -> *mut mode_change;
    fn mode_adjust(
        _: mode_t,
        _: bool,
        _: mode_t,
        _: *const mode_change,
        _: *mut mode_t,
    ) -> mode_t;
    fn rpl_free(ptr: *mut libc::c_void);
    fn parse_datetime(
        _: *mut timespec,
        _: *const libc::c_char,
        _: *const timespec,
    ) -> bool;
    fn mbsstr(
        haystack: *const libc::c_char,
        needle: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn is_selinux_enabled() -> libc::c_int;
    fn bc_push_arg(
        ctl: *mut buildcmd_control,
        state_0: *mut buildcmd_state,
        arg: *const libc::c_char,
        len: size_t,
        prefix: *const libc::c_char,
        pfxlen: size_t,
        initial_args: libc::c_int,
    );
    fn bc_init_state(
        ctl: *const buildcmd_control,
        state_0: *mut buildcmd_state,
        usercontext: *mut libc::c_void,
    );
    fn bc_init_controlinfo(
        ctl: *mut buildcmd_control,
        arglen_headroom: size_t,
    ) -> BC_INIT_STATUS;
    fn bc_use_sensible_arg_max(ctl: *mut buildcmd_control);
    fn clone_quoting_options(o: *mut quoting_options) -> *mut quoting_options;
    fn quotearg_n_style(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sharefile_fopen(_: sharefile_handle, filename: *const libc::c_char) -> *mut FILE;
    fn optionl_stat(name: *const libc::c_char, p: *mut stat) -> libc::c_int;
    fn optionp_stat(name: *const libc::c_char, p: *mut stat) -> libc::c_int;
    fn optionh_stat(name: *const libc::c_char, p: *mut stat) -> libc::c_int;
    fn debug_stat(file: *const libc::c_char, bufp: *mut stat) -> libc::c_int;
    fn set_stat_placeholders(p: *mut stat);
    fn is_fts_enabled(ftsoptions: *mut libc::c_int) -> bool;
    static mut options: options;
    fn pred_newerXY(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_negate(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn get_new_pred_chk_op(
        entry: *const parser_table,
        arg: *const libc::c_char,
    ) -> *mut predicate;
    fn pred_openparen(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_closeparen(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_comma(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_amin(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_and(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_anewer(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    static mut state: state;
    fn pred_cmin(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_cnewer(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn filesystem_type(
        statp: *const stat,
        path: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn is_used_fs_type(name: *const libc::c_char) -> bool;
    fn pred_ctime(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_context(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_delete(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_empty(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_exec(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_fls(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_fprint(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_fprint0(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_fprintf(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn usage(status: libc::c_int) -> !;
    fn pred_true(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_fstype(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn insert_primary_noarg(entry: *const parser_table) -> *mut predicate;
    fn pred_false(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_gid(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_group(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_xtype(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_ilname(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_iname(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_inum(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn insert_primary_withpred(
        entry: *const parser_table,
        fptr: PRED_FUNC,
        arg: *const libc::c_char,
    ) -> *mut predicate;
    fn pred_writable(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_executable(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_path(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_links(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn matches_start_point(glob: *const libc::c_char, foldcase: bool) -> bool;
    fn pred_lname(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_ls(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_mmin(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_ipath(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_mtime(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_user(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_name(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn insert_primary(
        entry: *const parser_table,
        arg: *const libc::c_char,
    ) -> *mut predicate;
    fn pred_newer(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_used(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_atime(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_nogroup(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_nouser(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn get_new_pred_noarg(entry: *const parser_table) -> *mut predicate;
    fn pred_or(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_uid(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_ok(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn safely_quote_err_filename(
        n: libc::c_int,
        arg: *const libc::c_char,
    ) -> *const libc::c_char;
    static mut initial_wd: *mut saved_cwd;
    fn pred_execdir(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn launch(
        ctl: *mut buildcmd_control,
        usercontext: *mut libc::c_void,
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn pred_okdir(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_type(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_perm(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_size(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_samefile(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_print(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_print0(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn fatal_nontarget_file_error(
        errno_value: libc::c_int,
        name: *const libc::c_char,
    ) -> !;
    fn insert_fprintf(
        vec: *mut format_val,
        entry: *const parser_table,
        format: *mut libc::c_char,
    ) -> bool;
    fn fatal_target_file_error(errno_value: libc::c_int, name: *const libc::c_char) -> !;
    fn pred_prune(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_quit(
        pathname: *const libc::c_char,
        stat_buf: *mut stat,
        pred_ptr: *mut predicate,
    ) -> !;
    fn pred_readable(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_regex(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn get_regex_type(s: *const libc::c_char) -> libc::c_int;
    fn safe_atoi(s: *const libc::c_char, style: quoting_style) -> libc::c_int;
    fn getfileconat(
        dir_fd: libc::c_int,
        file: *const libc::c_char,
        con: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn lgetfileconat(
        dir_fd: libc::c_int,
        file: *const libc::c_char,
        con: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn splitstring(
        s: *const libc::c_char,
        separators: *const libc::c_char,
        first: bool,
        pos: *mut size_t,
        len: *mut size_t,
    ) -> bool;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrtod(
        str: *const libc::c_char,
        ptr: *mut *const libc::c_char,
        result: *mut libc::c_double,
        convert: Option::<
            unsafe extern "C" fn(
                *const libc::c_char,
                *mut *mut libc::c_char,
            ) -> libc::c_double,
        >,
    ) -> bool;
    fn xstrtoumax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut uintmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn open_cloexec(
        path: *const libc::c_char,
        flags: libc::c_int,
        _: ...
    ) -> libc::c_int;
    fn display_findutils_version(official_name: *const libc::c_char);
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
pub type BC_INIT_STATUS = libc::c_uint;
pub const BC_INIT_CANNOT_ACCOMODATE_HEADROOM: BC_INIT_STATUS = 2;
pub const BC_INIT_ENV_TOO_BIG: BC_INIT_STATUS = 1;
pub const BC_INIT_OK: BC_INIT_STATUS = 0;
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
pub const DebugStat: DebugOption = 2;
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
pub const DebugExpressionTree: DebugOption = 1;
pub const LONGINT_OK: strtol_error = 0;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
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
pub const MsgBufSize: C2RustUnnamed_18 = 19;
pub type C2RustUnnamed_18 = libc::c_uint;
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
pub struct C2RustUnnamed_25 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub _gl_dummy: libc::c_int,
}
pub const DebugTreeOpt: DebugOption = 8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
    pub _gl_dummy: libc::c_int,
}
pub const seen_delete: C2RustUnnamed_33 = 1;
pub const seen_prune: C2RustUnnamed_33 = 2;
pub type C2RustUnnamed_33 = libc::c_uint;
pub type DebugOption = libc::c_int;
pub const DebugAll: DebugOption = -17;
pub const DebugTime: DebugOption = 128;
pub const DebugSuccessRates: DebugOption = 64;
pub const DebugExec: DebugOption = 32;
pub const DebugHelp: DebugOption = 16;
pub const DebugSearch: DebugOption = 4;
pub const DebugNone: DebugOption = 0;
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
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
static mut parse_entry_newerXY: parser_table = unsafe {
    {
        let mut init = parser_table {
            type_0: ARG_SPECIAL_PARSE,
            parser_name: b"newerXY\0" as *const u8 as *const libc::c_char,
            parser_func: Some(
                parse_newerXY
                    as unsafe extern "C" fn(
                        *const parser_table,
                        *mut *mut libc::c_char,
                        *mut libc::c_int,
                    ) -> bool,
            ),
            pred_func: Some(pred_newerXY as PREDICATEFUNCTION),
        };
        init
    }
};
static mut parse_table: [parser_table; 87] = unsafe {
    [
        {
            let mut init = parser_table {
                type_0: ARG_PUNCTUATION,
                parser_name: b"!\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_negate
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_negate as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_PUNCTUATION,
                parser_name: b"not\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_negate
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_negate as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_PUNCTUATION,
                parser_name: b"(\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_openparen
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_openparen as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_PUNCTUATION,
                parser_name: b")\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_closeparen
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_closeparen as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_PUNCTUATION,
                parser_name: b",\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_comma
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_comma as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_PUNCTUATION,
                parser_name: b"a\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_and
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_and as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"amin\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_amin
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_amin as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_PUNCTUATION,
                parser_name: b"and\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_and
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_and as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"anewer\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_anewer
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_anewer as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"atime\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_time
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_atime as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"cmin\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_cmin
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_cmin as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"cnewer\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_cnewer
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_cnewer as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"ctime\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_time
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_ctime as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"context\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_context
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_context as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_POSITIONAL_OPTION,
                parser_name: b"daystart\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_daystart
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_ACTION,
                parser_name: b"delete\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_delete
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_delete as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_OPTION,
                parser_name: b"d\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_d
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_OPTION,
                parser_name: b"depth\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_depth
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"empty\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_empty
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_empty as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_ACTION,
                parser_name: b"exec\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_exec
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_exec as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"executable\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_accesscheck
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_executable as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_ACTION,
                parser_name: b"execdir\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_execdir
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_execdir as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_OPTION,
                parser_name: b"files0-from\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_files0_from
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_ACTION,
                parser_name: b"fls\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_fls
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_fls as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_POSITIONAL_OPTION,
                parser_name: b"follow\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_follow
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_ACTION,
                parser_name: b"fprint\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_fprint
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_fprint as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_ACTION,
                parser_name: b"fprint0\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_fprint0
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_fprint0 as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_ACTION,
                parser_name: b"fprintf\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_fprintf
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_fprintf as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"fstype\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_fstype
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_fstype as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"gid\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_gid
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_gid as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"group\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_group
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_group as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_OPTION,
                parser_name: b"ignore_readdir_race\0" as *const u8
                    as *const libc::c_char,
                parser_func: Some(
                    parse_ignore_race
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"ilname\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_ilname
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_ilname as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"iname\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_iname
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_iname as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"inum\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_inum
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_inum as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"ipath\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_ipath
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_ipath as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"iregex\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_iregex
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"iwholename\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_iwholename
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"links\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_links
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_links as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"lname\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_lname
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_lname as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_ACTION,
                parser_name: b"ls\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_ls
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_ls as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_OPTION,
                parser_name: b"maxdepth\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_maxdepth
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_OPTION,
                parser_name: b"mindepth\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_mindepth
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"mmin\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_mmin
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_mmin as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_OPTION,
                parser_name: b"mount\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_xdev
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"mtime\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_time
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_mtime as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"name\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_name
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_name as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"newer\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_newer
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_newer as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"atime\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_time
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_atime as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_OPTION,
                parser_name: b"noleaf\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_noleaf
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"nogroup\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_nogroup
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_nogroup as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"nouser\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_nouser
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_nouser as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_OPTION,
                parser_name: b"noignore_readdir_race\0" as *const u8
                    as *const libc::c_char,
                parser_func: Some(
                    parse_noignore_race
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_POSITIONAL_OPTION,
                parser_name: b"nowarn\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_nowarn
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_POSITIONAL_OPTION,
                parser_name: b"warn\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_warn
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_PUNCTUATION,
                parser_name: b"o\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_or
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_or as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_PUNCTUATION,
                parser_name: b"or\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_or
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_or as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_ACTION,
                parser_name: b"ok\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_ok
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_ok as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_ACTION,
                parser_name: b"okdir\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_okdir
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_okdir as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"path\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_path
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_path as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"perm\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_perm
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_perm as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_ACTION,
                parser_name: b"print\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_print
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_print as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_ACTION,
                parser_name: b"print0\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_print0
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_print0 as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_ACTION,
                parser_name: b"printf\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_printf
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_ACTION,
                parser_name: b"prune\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_prune
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_prune as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_ACTION,
                parser_name: b"quit\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_quit
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: ::std::mem::transmute::<
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
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"readable\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_accesscheck
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_readable as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"regex\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_regex
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_regex as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_POSITIONAL_OPTION,
                parser_name: b"regextype\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_regextype
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"samefile\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_samefile
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_samefile as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"size\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_size
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_size as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"type\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_type
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_type as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"uid\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_uid
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_uid as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"used\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_used
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_used as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"user\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_user
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_user as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"wholename\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_wholename
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"writable\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_accesscheck
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_writable as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_OPTION,
                parser_name: b"xdev\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_xdev
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"xtype\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_xtype
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_xtype as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"false\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_false
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_false as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"true\0" as *const u8 as *const libc::c_char,
                parser_func: Some(
                    parse_true
                        as unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> bool,
                ),
                pred_func: Some(pred_true as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_NOOP,
                parser_name: b"--noop\0" as *const u8 as *const libc::c_char,
                parser_func: None,
                pred_func: Some(pred_true as PREDICATEFUNCTION),
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"help\0" as *const u8 as *const libc::c_char,
                parser_func: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> !,
                    >,
                    PARSE_FUNC,
                >(
                    Some(
                        parse_help
                            as unsafe extern "C" fn(
                                *const parser_table,
                                *mut *mut libc::c_char,
                                *mut libc::c_int,
                            ) -> !,
                    ),
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"-help\0" as *const u8 as *const libc::c_char,
                parser_func: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> !,
                    >,
                    PARSE_FUNC,
                >(
                    Some(
                        parse_help
                            as unsafe extern "C" fn(
                                *const parser_table,
                                *mut *mut libc::c_char,
                                *mut libc::c_int,
                            ) -> !,
                    ),
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"version\0" as *const u8 as *const libc::c_char,
                parser_func: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> !,
                    >,
                    PARSE_FUNC,
                >(
                    Some(
                        parse_version
                            as unsafe extern "C" fn(
                                *const parser_table,
                                *mut *mut libc::c_char,
                                *mut libc::c_int,
                            ) -> !,
                    ),
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_TEST,
                parser_name: b"-version\0" as *const u8 as *const libc::c_char,
                parser_func: ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const parser_table,
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> !,
                    >,
                    PARSE_FUNC,
                >(
                    Some(
                        parse_version
                            as unsafe extern "C" fn(
                                *const parser_table,
                                *mut *mut libc::c_char,
                                *mut libc::c_int,
                            ) -> !,
                    ),
                ),
                pred_func: None,
            };
            init
        },
        {
            let mut init = parser_table {
                type_0: ARG_OPTION,
                parser_name: 0 as *const libc::c_char,
                parser_func: None,
                pred_func: None,
            };
            init
        },
    ]
};
static mut first_nonoption_arg: *const libc::c_char = 0 as *const libc::c_char;
static mut noop: *const parser_table = 0 as *const parser_table;
unsafe extern "C" fn fallback_getfilecon(
    mut fd: libc::c_int,
    mut name: *const libc::c_char,
    mut p: *mut *mut libc::c_char,
    mut prev_rv: libc::c_int,
) -> libc::c_int {
    match *__errno_location() {
        2 | 20 => {
            if options.debug_options & DebugStat as libc::c_int as libc::c_ulong != 0 {
                fprintf(
                    stderr,
                    b"fallback_getfilecon(): getfilecon(%s) failed; falling back on lgetfilecon()\n\0"
                        as *const u8 as *const libc::c_char,
                    name,
                );
            }
            return lgetfileconat(fd, name, p);
        }
        13 | 5 | 40 | 36 | 75 | _ => return prev_rv,
    };
}
unsafe extern "C" fn optionh_getfilecon(
    mut fd: libc::c_int,
    mut name: *const libc::c_char,
    mut p: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut rv: libc::c_int = 0;
    if 0 as libc::c_int == state.curdepth {
        rv = getfileconat(fd, name, p);
        if 0 as libc::c_int == rv {
            return 0 as libc::c_int
        } else {
            return fallback_getfilecon(fd, name, p, rv)
        }
    } else {
        return lgetfileconat(fd, name, p)
    };
}
unsafe extern "C" fn optionl_getfilecon(
    mut fd: libc::c_int,
    mut name: *const libc::c_char,
    mut p: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut rv: libc::c_int = getfileconat(fd, name, p);
    if 0 as libc::c_int == rv {
        return 0 as libc::c_int
    } else {
        return fallback_getfilecon(fd, name, p, rv)
    };
}
unsafe extern "C" fn optionp_getfilecon(
    mut fd: libc::c_int,
    mut name: *const libc::c_char,
    mut p: *mut *mut libc::c_char,
) -> libc::c_int {
    return lgetfileconat(fd, name, p);
}
pub unsafe extern "C" fn check_option_combinations(mut p: *const predicate) {
    let mut predicates: libc::c_uint = 0 as libc::c_uint;
    while !p.is_null() {
        if (*p).pred_func == Some(pred_delete as PREDICATEFUNCTION) {
            predicates |= seen_delete as libc::c_int as libc::c_uint;
        } else if (*p).pred_func == Some(pred_prune as PREDICATEFUNCTION) {
            predicates |= seen_prune as libc::c_int as libc::c_uint;
        }
        p = (*p).pred_next;
    }
    if predicates & seen_prune as libc::c_int as libc::c_uint != 0
        && predicates & seen_delete as libc::c_int as libc::c_uint != 0
    {
        if !options.explicit_depth {
            if ::std::mem::size_of::<C2RustUnnamed_32>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"The -delete action automatically turns on -depth, but -prune does nothing when -depth is in effect.  If you want to carry on anyway, just explicitly use the -depth option.\0"
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
                        b"The -delete action automatically turns on -depth, but -prune does nothing when -depth is in effect.  If you want to carry on anyway, just explicitly use the -depth option.\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
}
unsafe extern "C" fn get_noop() -> *const parser_table {
    let mut i: libc::c_int = 0;
    if noop.is_null() {
        i = 0 as libc::c_int;
        while !(parse_table[i as usize].parser_name).is_null() {
            if ARG_NOOP as libc::c_int as libc::c_uint
                == parse_table[i as usize].type_0 as libc::c_uint
            {
                noop = &*parse_table.as_ptr().offset(i as isize) as *const parser_table;
                break;
            } else {
                i += 1;
                i;
            }
        }
    }
    return noop;
}
unsafe extern "C" fn get_stat_Ytime(
    mut p: *const stat,
    mut what: libc::c_char,
    mut ret: *mut timespec,
) -> libc::c_int {
    match what as libc::c_int {
        97 => {
            *ret = get_stat_atime(p);
            return 1 as libc::c_int;
        }
        66 => {
            *ret = get_stat_birthtime(p);
            return ((*ret).tv_nsec >= 0 as libc::c_int as libc::c_long) as libc::c_int;
        }
        99 => {
            *ret = get_stat_ctime(p);
            return 1 as libc::c_int;
        }
        109 => {
            *ret = get_stat_mtime(p);
            return 1 as libc::c_int;
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"parser.c\0" as *const u8 as *const libc::c_char,
                498 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"int get_stat_Ytime(const struct stat *, char, struct timespec *)\0"))
                    .as_ptr(),
            );
            'c_22395: {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"parser.c\0" as *const u8 as *const libc::c_char,
                    498 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 65],
                        &[libc::c_char; 65],
                    >(
                        b"int get_stat_Ytime(const struct stat *, char, struct timespec *)\0",
                    ))
                        .as_ptr(),
                );
            };
            abort();
        }
    };
}
pub unsafe extern "C" fn set_follow_state(mut opt: SymlinkOption) {
    match opt as libc::c_uint {
        1 => {
            options
                .xstat = Some(
                optionl_stat
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut stat,
                    ) -> libc::c_int,
            );
            options
                .x_getfilecon = Some(
                optionl_getfilecon
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *const libc::c_char,
                        *mut *mut libc::c_char,
                    ) -> libc::c_int,
            );
            options.no_leaf_check = 1 as libc::c_int != 0;
        }
        0 => {
            options
                .xstat = Some(
                optionp_stat
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut stat,
                    ) -> libc::c_int,
            );
            options
                .x_getfilecon = Some(
                optionp_getfilecon
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *const libc::c_char,
                        *mut *mut libc::c_char,
                    ) -> libc::c_int,
            );
        }
        2 => {
            options
                .xstat = Some(
                optionh_stat
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut stat,
                    ) -> libc::c_int,
            );
            options
                .x_getfilecon = Some(
                optionh_getfilecon
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *const libc::c_char,
                        *mut *mut libc::c_char,
                    ) -> libc::c_int,
            );
            options.no_leaf_check = 1 as libc::c_int != 0;
        }
        _ => {}
    }
    options.symlink_handling = opt;
    if options.debug_options & DebugStat as libc::c_int as libc::c_ulong != 0 {
        options
            .xstat = Some(
            debug_stat
                as unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
        );
    }
}
pub unsafe extern "C" fn parse_begin_user_args(
    mut args: *mut *mut libc::c_char,
    mut argno: libc::c_int,
    mut last: *const predicate,
    mut predicates: *const predicate,
) {
    first_nonoption_arg = 0 as *const libc::c_char;
}
pub unsafe extern "C" fn parse_end_user_args(
    mut args: *mut *mut libc::c_char,
    mut argno: libc::c_int,
    mut last: *const predicate,
    mut predicates: *const predicate,
) {}
unsafe extern "C" fn should_issue_warnings() -> bool {
    if options.posixly_correct {
        return 0 as libc::c_int != 0
    } else {
        return options.warnings
    };
}
unsafe extern "C" fn found_parser(
    mut original_arg: *const libc::c_char,
    mut entry: *const parser_table,
) -> *const parser_table {
    if (*entry).type_0 as libc::c_uint
        != ARG_POSITIONAL_OPTION as libc::c_int as libc::c_uint
    {
        if (*entry).type_0 as libc::c_uint == ARG_NOOP as libc::c_int as libc::c_uint {
            return 0 as *const parser_table;
        }
        if (*entry).type_0 as libc::c_uint == ARG_OPTION as libc::c_int as libc::c_uint {
            if !first_nonoption_arg.is_null()
                && should_issue_warnings() as libc::c_int != 0
            {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"warning: you have specified the global option %s after the argument %s, but global options are not positional, i.e., %s affects tests specified before it as well as those specified after it.  Please specify global options before other arguments.\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    original_arg,
                    first_nonoption_arg,
                    original_arg,
                );
            }
        } else if first_nonoption_arg.is_null() {
            first_nonoption_arg = original_arg;
        }
    }
    return entry;
}
pub unsafe extern "C" fn find_parser(
    mut search_name: *const libc::c_char,
) -> *const parser_table {
    let mut i: libc::c_int = 0;
    let mut original_arg: *const libc::c_char = search_name;
    if 0 as libc::c_int
        == strncmp(
            b"-newer\0" as *const u8 as *const libc::c_char,
            search_name,
            6 as libc::c_int as libc::c_ulong,
        ) && 8 as libc::c_int as libc::c_ulong == strlen(search_name)
    {
        return found_parser(original_arg, &parse_entry_newerXY);
    }
    if *search_name as libc::c_int == '-' as i32 {
        search_name = search_name.offset(1);
        search_name;
    }
    i = 0 as libc::c_int;
    while !(parse_table[i as usize].parser_name).is_null() {
        if strcmp(parse_table[i as usize].parser_name, search_name) == 0 as libc::c_int {
            return found_parser(original_arg, &*parse_table.as_ptr().offset(i as isize));
        }
        i += 1;
        i;
    }
    return 0 as *const parser_table;
}
unsafe extern "C" fn estimate_file_age_success_rate(
    mut num_days: libc::c_float,
) -> libc::c_float {
    if num_days < 0.1f32 {
        return 0.01f32
    } else if num_days < 1.0f32 {
        return 0.3f32
    } else if num_days > 100.0f32 {
        return 0.3f32
    } else {
        return 0.39f32
    };
}
unsafe extern "C" fn estimate_timestamp_success_rate(mut when: time_t) -> libc::c_float {
    let mut num_days: libc::c_int = ((options.cur_day_start.tv_sec - when)
        / 86400 as libc::c_int as libc::c_long) as libc::c_int;
    return estimate_file_age_success_rate(num_days as libc::c_float);
}
unsafe extern "C" fn collect_arg_nonconst(
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
    mut collected_arg: *mut *mut libc::c_char,
) -> bool {
    if argv.is_null() || (*argv.offset(*arg_ptr as isize)).is_null() {
        *collected_arg = 0 as *mut libc::c_char;
        return 0 as libc::c_int != 0;
    } else {
        *collected_arg = *argv.offset(*arg_ptr as isize);
        *arg_ptr += 1;
        *arg_ptr;
        return 1 as libc::c_int != 0;
    };
}
unsafe extern "C" fn collect_arg(
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
    mut collected_arg: *mut *const libc::c_char,
) -> bool {
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let result: bool = collect_arg_nonconst(argv, arg_ptr, &mut arg);
    *collected_arg = arg;
    return result;
}
unsafe extern "C" fn collect_arg_stat_info(
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
    mut p: *mut stat,
    mut argument: *mut *const libc::c_char,
) -> bool {
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    if collect_arg(argv, arg_ptr, &mut filename) {
        *argument = filename;
        if 0 as libc::c_int != (options.xstat).unwrap()(filename, p) {
            fatal_target_file_error(*__errno_location(), filename);
        }
        return 1 as libc::c_int != 0;
    } else {
        *argument = 0 as *const libc::c_char;
        return 0 as libc::c_int != 0;
    };
}
unsafe extern "C" fn parse_and(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    our_pred = get_new_pred_noarg(entry);
    (*our_pred).pred_func = Some(pred_and as PREDICATEFUNCTION);
    (*our_pred).p_type = BI_OP;
    (*our_pred).p_prec = AND_PREC;
    (*our_pred).need_type = 0 as libc::c_int != 0;
    (*our_pred).need_stat = (*our_pred).need_type;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_anewer(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut stat_newer: stat = stat {
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
    let mut arg: *const libc::c_char = 0 as *const libc::c_char;
    set_stat_placeholders(&mut stat_newer);
    if collect_arg_stat_info(argv, arg_ptr, &mut stat_newer, &mut arg) {
        let mut our_pred: *mut predicate = insert_primary(entry, arg);
        (*our_pred).args.reftime.xval = XVAL_ATIME;
        (*our_pred).args.reftime.ts = get_stat_mtime(&mut stat_newer);
        (*our_pred).args.reftime.kind = COMP_GT;
        (*our_pred)
            .est_success_rate = estimate_timestamp_success_rate(
            stat_newer.st_mtim.tv_sec,
        );
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn parse_closeparen(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    our_pred = get_new_pred_noarg(entry);
    (*our_pred).pred_func = Some(pred_closeparen as PREDICATEFUNCTION);
    (*our_pred).p_type = CLOSE_PAREN;
    (*our_pred).p_prec = NO_PREC;
    (*our_pred).need_type = 0 as libc::c_int != 0;
    (*our_pred).need_stat = (*our_pred).need_type;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_cnewer(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut stat_newer: stat = stat {
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
    let mut arg: *const libc::c_char = 0 as *const libc::c_char;
    set_stat_placeholders(&mut stat_newer);
    if collect_arg_stat_info(argv, arg_ptr, &mut stat_newer, &mut arg) {
        let mut our_pred: *mut predicate = insert_primary(entry, arg);
        (*our_pred).args.reftime.xval = XVAL_CTIME;
        (*our_pred).args.reftime.ts = get_stat_mtime(&mut stat_newer);
        (*our_pred).args.reftime.kind = COMP_GT;
        (*our_pred)
            .est_success_rate = estimate_timestamp_success_rate(
            stat_newer.st_mtim.tv_sec,
        );
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn parse_comma(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    our_pred = get_new_pred_noarg(entry);
    (*our_pred).pred_func = Some(pred_comma as PREDICATEFUNCTION);
    (*our_pred).p_type = BI_OP;
    (*our_pred).p_prec = COMMA_PREC;
    (*our_pred).need_type = 0 as libc::c_int != 0;
    (*our_pred).need_stat = (*our_pred).need_type;
    (*our_pred).est_success_rate = 1.0f32;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_daystart(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut local: *mut tm = 0 as *mut tm;
    if options.full_days as libc::c_int == 0 as libc::c_int {
        options.cur_day_start.tv_sec += 86400 as libc::c_int as libc::c_long;
        options.cur_day_start.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
        local = localtime(&mut options.cur_day_start.tv_sec);
        options.cur_day_start.tv_sec
            -= if !local.is_null() {
                ((*local).tm_sec + (*local).tm_min * 60 as libc::c_int
                    + (*local).tm_hour * 3600 as libc::c_int) as libc::c_long
            } else {
                options.cur_day_start.tv_sec % 86400 as libc::c_int as libc::c_long
            };
        options.full_days = 1 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_delete(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    our_pred = insert_primary_noarg(entry);
    (*our_pred).no_default_print = 1 as libc::c_int != 0;
    (*our_pred).side_effects = (*our_pred).no_default_print;
    options.do_dir_first = 0 as libc::c_int != 0;
    (*our_pred).need_type = 0 as libc::c_int != 0;
    (*our_pred).need_stat = (*our_pred).need_type;
    (*our_pred).est_success_rate = 1.0f32;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_depth(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    options.do_dir_first = 0 as libc::c_int != 0;
    options.explicit_depth = 1 as libc::c_int != 0;
    return parse_noop(entry, argv, arg_ptr);
}
unsafe extern "C" fn parse_d(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    if should_issue_warnings() {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"warning: the -d option is deprecated; please use -depth instead, because the latter is a POSIX-compliant feature.\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    return parse_depth(entry, argv, arg_ptr);
}
unsafe extern "C" fn parse_empty(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    our_pred = insert_primary_noarg(entry);
    (*our_pred).est_success_rate = 0.01f32;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_exec(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    return insert_exec_ok(
        b"-exec\0" as *const u8 as *const libc::c_char,
        entry,
        argv,
        arg_ptr,
    );
}
unsafe extern "C" fn parse_execdir(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    return insert_exec_ok(
        b"-execdir\0" as *const u8 as *const libc::c_char,
        entry,
        argv,
        arg_ptr,
    );
}
unsafe extern "C" fn insert_false() -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    let mut entry_false: *const parser_table = 0 as *const parser_table;
    entry_false = find_parser(b"false\0" as *const u8 as *const libc::c_char);
    our_pred = insert_primary_noarg(entry_false);
    (*our_pred).need_type = 0 as libc::c_int != 0;
    (*our_pred).need_stat = (*our_pred).need_type;
    (*our_pred).no_default_print = 0 as libc::c_int != 0;
    (*our_pred).side_effects = (*our_pred).no_default_print;
    (*our_pred).est_success_rate = 0.0f32;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_false(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    return insert_false();
}
unsafe extern "C" fn parse_files0_from(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    if collect_arg(argv, arg_ptr, &mut filename) {
        options.files0_from = filename;
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn insert_fls(
    mut entry: *const parser_table,
    mut filename: *const libc::c_char,
) -> bool {
    let mut our_pred: *mut predicate = insert_primary_noarg(entry);
    if !filename.is_null() {
        open_output_file(filename, &mut (*our_pred).args.printf_vec);
    } else {
        open_stdout(&mut (*our_pred).args.printf_vec);
    }
    (*our_pred).no_default_print = 1 as libc::c_int != 0;
    (*our_pred).side_effects = (*our_pred).no_default_print;
    (*our_pred).est_success_rate = 1.0f32;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_fls(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    if collect_arg(argv, arg_ptr, &mut filename) {
        if insert_fls(entry, filename) {
            return 1 as libc::c_int != 0
        } else {
            *arg_ptr -= 1;
            *arg_ptr;
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn parse_follow(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    set_follow_state(SYMLINK_ALWAYS_DEREF);
    return parse_noop(entry, argv, arg_ptr);
}
unsafe extern "C" fn parse_fprint(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    if collect_arg(argv, arg_ptr, &mut filename) {
        our_pred = insert_primary(entry, filename);
        open_output_file(filename, &mut (*our_pred).args.printf_vec);
        (*our_pred).no_default_print = 1 as libc::c_int != 0;
        (*our_pred).side_effects = (*our_pred).no_default_print;
        (*our_pred).need_type = 0 as libc::c_int != 0;
        (*our_pred).need_stat = (*our_pred).need_type;
        (*our_pred).est_success_rate = 1.0f32;
        return 1 as libc::c_int != 0;
    } else {
        return 0 as libc::c_int != 0
    };
}
unsafe extern "C" fn insert_fprint(
    mut entry: *const parser_table,
    mut filename: *const libc::c_char,
) -> bool {
    let mut our_pred: *mut predicate = insert_primary(entry, filename);
    if !filename.is_null() {
        open_output_file(filename, &mut (*our_pred).args.printf_vec);
    } else {
        open_stdout(&mut (*our_pred).args.printf_vec);
    }
    (*our_pred).no_default_print = 1 as libc::c_int != 0;
    (*our_pred).side_effects = (*our_pred).no_default_print;
    (*our_pred).need_type = 0 as libc::c_int != 0;
    (*our_pred).need_stat = (*our_pred).need_type;
    (*our_pred).est_success_rate = 1.0f32;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_fprint0(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    if collect_arg(argv, arg_ptr, &mut filename) {
        if insert_fprint(entry, filename) {
            return 1 as libc::c_int != 0
        } else {
            *arg_ptr -= 1;
            *arg_ptr;
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn estimate_fstype_success_rate(
    mut fsname: *const libc::c_char,
) -> libc::c_float {
    let mut dir_stat: stat = stat {
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
    let mut the_root_dir: *const libc::c_char = b"/\0" as *const u8
        as *const libc::c_char;
    if 0 as libc::c_int == stat(the_root_dir, &mut dir_stat) {
        let mut fstype: *const libc::c_char = filesystem_type(
            &mut dir_stat,
            the_root_dir,
        );
        if 0 as libc::c_int == strcmp(fsname, fstype) {
            return 0.7f32
        } else {
            return 0.3f32
        }
    }
    return 1.0f32;
}
unsafe extern "C" fn parse_fstype(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut typename: *const libc::c_char = 0 as *const libc::c_char;
    if collect_arg(argv, arg_ptr, &mut typename) {
        if (options.optimisation_level as libc::c_int) < 2 as libc::c_int
            || is_used_fs_type(typename) as libc::c_int != 0
        {
            let mut our_pred: *mut predicate = insert_primary(entry, typename);
            (*our_pred).args.str_0 = typename;
            (*our_pred).est_success_rate = estimate_fstype_success_rate(typename);
            return 1 as libc::c_int != 0;
        } else {
            if options.debug_options & DebugTreeOpt as libc::c_int as libc::c_ulong != 0
            {
                fprintf(
                    stderr,
                    b"-fstype %s can never succeed, substituting -false\n\0" as *const u8
                        as *const libc::c_char,
                    typename,
                );
            }
            return insert_false();
        }
    } else {
        return 0 as libc::c_int != 0
    };
}
unsafe extern "C" fn parse_gid(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut p: *mut predicate = insert_num(argv, arg_ptr, entry);
    if !p.is_null() {
        (*p)
            .est_success_rate = (if (*p).args.numinfo.l_val
            < 100 as libc::c_int as libc::c_ulong
        {
            0.99f64
        } else {
            0.2f64
        }) as libc::c_float;
        return 1 as libc::c_int != 0;
    } else {
        *arg_ptr -= 1;
        *arg_ptr;
        return 0 as libc::c_int != 0;
    };
}
unsafe extern "C" fn parse_group(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut groupname: *const libc::c_char = 0 as *const libc::c_char;
    let saved_argc: libc::c_int = *arg_ptr;
    if collect_arg(argv, arg_ptr, &mut groupname) {
        let mut gid: gid_t = 0;
        let mut our_pred: *mut predicate = 0 as *mut predicate;
        let mut cur_gr: *mut group = getgrnam(groupname);
        endgrent();
        if !cur_gr.is_null() {
            gid = (*cur_gr).gr_gid;
        } else {
            let gid_len: libc::c_int = strspn(
                groupname,
                b"0123456789\0" as *const u8 as *const libc::c_char,
            ) as libc::c_int;
            if gid_len != 0 {
                if *groupname.offset(gid_len as isize) as libc::c_int == 0 as libc::c_int
                {
                    gid = safe_atoi(groupname, options.err_quoting_style) as gid_t;
                } else {
                    if ::std::mem::size_of::<C2RustUnnamed_27>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s is not the name of an existing group and it does not look like a numeric group ID because it has the unexpected suffix %s\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quotearg_n_style(
                                0 as libc::c_int,
                                options.err_quoting_style,
                                groupname,
                            ),
                            quotearg_n_style(
                                1 as libc::c_int,
                                options.err_quoting_style,
                                groupname.offset(gid_len as isize),
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
                                b"%s is not the name of an existing group and it does not look like a numeric group ID because it has the unexpected suffix %s\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quotearg_n_style(
                                0 as libc::c_int,
                                options.err_quoting_style,
                                groupname,
                            ),
                            quotearg_n_style(
                                1 as libc::c_int,
                                options.err_quoting_style,
                                groupname.offset(gid_len as isize),
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                    *arg_ptr = saved_argc;
                    return 0 as libc::c_int != 0;
                }
            } else {
                if *groupname != 0 {
                    if ::std::mem::size_of::<C2RustUnnamed_26>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s is not the name of an existing group\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quotearg_n_style(
                                0 as libc::c_int,
                                options.err_quoting_style,
                                groupname,
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
                                b"%s is not the name of an existing group\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quotearg_n_style(
                                0 as libc::c_int,
                                options.err_quoting_style,
                                groupname,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                } else {
                    if ::std::mem::size_of::<C2RustUnnamed_25>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"argument to -group is empty, but should be a group name\0"
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
                                b"argument to -group is empty, but should be a group name\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                *arg_ptr = saved_argc;
                return 0 as libc::c_int != 0;
            }
        }
        our_pred = insert_primary(entry, groupname);
        (*our_pred).args.gid = gid;
        (*our_pred)
            .est_success_rate = (if (*our_pred).args.numinfo.l_val
            < 100 as libc::c_int as libc::c_ulong
        {
            0.99f64
        } else {
            0.2f64
        }) as libc::c_float;
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn parse_help(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> ! {
    usage(0 as libc::c_int);
}
unsafe extern "C" fn estimate_pattern_match_rate(
    mut pattern: *const libc::c_char,
    mut is_regex: libc::c_int,
) -> libc::c_float {
    if !(strpbrk(pattern, b"*?[\0" as *const u8 as *const libc::c_char)).is_null()
        || is_regex != 0
            && !(strpbrk(pattern, b".\0" as *const u8 as *const libc::c_char)).is_null()
    {
        return 0.8f32
    } else {
        return 0.1f32
    };
}
unsafe extern "C" fn parse_ilname(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if collect_arg(argv, arg_ptr, &mut name) {
        let mut our_pred: *mut predicate = insert_primary(entry, name);
        (*our_pred).args.str_0 = name;
        (*our_pred)
            .est_success_rate = 0.1f32
            * estimate_pattern_match_rate(name, 0 as libc::c_int);
        return 1 as libc::c_int != 0;
    } else {
        return 0 as libc::c_int != 0
    };
}
unsafe extern "C" fn fnmatch_sanitycheck() -> bool {
    static mut checked: bool = 0 as libc::c_int != 0;
    if !checked {
        if 0 as libc::c_int
            != fnmatch(
                b"foo\0" as *const u8 as *const libc::c_char,
                b"foo\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            )
            || 0 as libc::c_int
                == fnmatch(
                    b"Foo\0" as *const u8 as *const libc::c_char,
                    b"foo\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                )
            || 0 as libc::c_int
                != fnmatch(
                    b"Foo\0" as *const u8 as *const libc::c_char,
                    b"foo\0" as *const u8 as *const libc::c_char,
                    (1 as libc::c_int) << 4 as libc::c_int,
                )
        {
            if ::std::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"sanity check of the fnmatch() library function failed.\0"
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
                        b"sanity check of the fnmatch() library function failed.\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
            return 0 as libc::c_int != 0;
        }
        checked = 1 as libc::c_int != 0;
    }
    return checked;
}
unsafe extern "C" fn check_name_arg(
    mut pred: *const libc::c_char,
    mut alt: *const libc::c_char,
    mut arg: *const libc::c_char,
) -> bool {
    if should_issue_warnings() as libc::c_int != 0
        && !(strchr(arg, '/' as i32)).is_null()
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"warning: %s matches against basenames only, but the given pattern contains a directory separator (%s), thus the expression will evaluate to false all the time.  Did you mean %s?\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            safely_quote_err_filename(0 as libc::c_int, pred),
            safely_quote_err_filename(
                1 as libc::c_int,
                b"/\0" as *const u8 as *const libc::c_char,
            ),
            safely_quote_err_filename(2 as libc::c_int, alt),
        );
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_iname(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    fnmatch_sanitycheck();
    if collect_arg(argv, arg_ptr, &mut name) {
        if check_name_arg(
            b"-iname\0" as *const u8 as *const libc::c_char,
            b"-iwholename\0" as *const u8 as *const libc::c_char,
            name,
        ) {
            let mut our_pred: *mut predicate = insert_primary(entry, name);
            (*our_pred).need_type = 0 as libc::c_int != 0;
            (*our_pred).need_stat = (*our_pred).need_type;
            (*our_pred).args.str_0 = name;
            (*our_pred)
                .est_success_rate = estimate_pattern_match_rate(name, 0 as libc::c_int);
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn parse_inum(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut p: *mut predicate = insert_num(argv, arg_ptr, entry);
    if !p.is_null() {
        (*p).est_success_rate = 1e-6f64 as libc::c_float;
        (*p).need_inum = 1 as libc::c_int != 0;
        (*p).need_stat = 0 as libc::c_int != 0;
        (*p).need_type = 0 as libc::c_int != 0;
        return 1 as libc::c_int != 0;
    } else {
        *arg_ptr -= 1;
        *arg_ptr;
        return 0 as libc::c_int != 0;
    };
}
unsafe extern "C" fn parse_iregex(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    return insert_regex(
        argv,
        arg_ptr,
        entry,
        (((((((((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | options.regex_options as libc::c_ulong) as libc::c_int,
    );
}
unsafe extern "C" fn parse_links(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut p: *mut predicate = insert_num(argv, arg_ptr, entry);
    if !p.is_null() {
        if (*p).args.numinfo.l_val == 1 as libc::c_int as libc::c_ulong {
            (*p).est_success_rate = 0.99f64 as libc::c_float;
        } else if (*p).args.numinfo.l_val == 2 as libc::c_int as libc::c_ulong {
            (*p).est_success_rate = 0.01f64 as libc::c_float;
        } else {
            (*p).est_success_rate = 1e-3f64 as libc::c_float;
        }
        return 1 as libc::c_int != 0;
    } else {
        *arg_ptr -= 1;
        *arg_ptr;
        return 0 as libc::c_int != 0;
    };
}
unsafe extern "C" fn parse_lname(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    fnmatch_sanitycheck();
    if collect_arg(argv, arg_ptr, &mut name) {
        let mut our_pred: *mut predicate = insert_primary(entry, name);
        (*our_pred).args.str_0 = name;
        (*our_pred)
            .est_success_rate = 0.1f32
            * estimate_pattern_match_rate(name, 0 as libc::c_int);
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn parse_ls(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    &mut argv;
    &mut arg_ptr;
    return insert_fls(entry, 0 as *const libc::c_char);
}
unsafe extern "C" fn insert_depthspec(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
    mut limitptr: *mut libc::c_int,
) -> bool {
    let mut depthstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut depth_len: libc::c_int = 0;
    let mut predicate: *const libc::c_char = *argv
        .offset((*arg_ptr - 1 as libc::c_int) as isize);
    if collect_arg(argv, arg_ptr, &mut depthstr) {
        depth_len = strspn(depthstr, b"0123456789\0" as *const u8 as *const libc::c_char)
            as libc::c_int;
        if depth_len > 0 as libc::c_int
            && *depthstr.offset(depth_len as isize) as libc::c_int == 0 as libc::c_int
        {
            *limitptr = safe_atoi(depthstr, options.err_quoting_style);
            if *limitptr >= 0 as libc::c_int {
                return parse_noop(entry, argv, arg_ptr);
            }
        }
        if ::std::mem::size_of::<C2RustUnnamed_24>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Expected a positive decimal integer argument to %s, but got %s\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                predicate,
                quotearg_n_style(0 as libc::c_int, options.err_quoting_style, depthstr),
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
                    b"Expected a positive decimal integer argument to %s, but got %s\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                predicate,
                quotearg_n_style(0 as libc::c_int, options.err_quoting_style, depthstr),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
        return 0 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn parse_maxdepth(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    return insert_depthspec(entry, argv, arg_ptr, &mut options.maxdepth);
}
unsafe extern "C" fn parse_mindepth(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    return insert_depthspec(entry, argv, arg_ptr, &mut options.mindepth);
}
unsafe extern "C" fn do_parse_xmin(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
    mut xv: xval,
) -> bool {
    let mut minutes: *const libc::c_char = 0 as *const libc::c_char;
    let saved_argc: libc::c_int = *arg_ptr;
    if collect_arg(argv, arg_ptr, &mut minutes) {
        let mut tval: time_val = time_val {
            xval: XVAL_ATIME,
            kind: COMP_GT,
            ts: timespec { tv_sec: 0, tv_nsec: 0 },
        };
        let mut origin: timespec = options.cur_day_start;
        tval.xval = xv;
        origin.tv_sec += 86400 as libc::c_int as libc::c_long;
        if get_relative_timestamp(
            minutes,
            &mut tval,
            origin,
            60 as libc::c_int as libc::c_double,
            b"arithmetic overflow while converting %s minutes to a number of seconds\0"
                as *const u8 as *const libc::c_char,
        ) {
            let mut our_pred: *mut predicate = insert_primary(entry, minutes);
            (*our_pred).args.reftime = tval;
            (*our_pred)
                .est_success_rate = estimate_timestamp_success_rate(tval.ts.tv_sec);
            return 1 as libc::c_int != 0;
        } else {
            *arg_ptr = saved_argc;
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn parse_amin(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    return do_parse_xmin(entry, argv, arg_ptr, XVAL_ATIME);
}
unsafe extern "C" fn parse_cmin(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    return do_parse_xmin(entry, argv, arg_ptr, XVAL_CTIME);
}
unsafe extern "C" fn parse_mmin(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    return do_parse_xmin(entry, argv, arg_ptr, XVAL_MTIME);
}
unsafe extern "C" fn parse_name(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let saved_argc: libc::c_int = *arg_ptr;
    if collect_arg(argv, arg_ptr, &mut name) {
        fnmatch_sanitycheck();
        if check_name_arg(
            b"-name\0" as *const u8 as *const libc::c_char,
            b"-wholename\0" as *const u8 as *const libc::c_char,
            name,
        ) {
            let mut our_pred: *mut predicate = insert_primary(entry, name);
            (*our_pred).need_type = 0 as libc::c_int != 0;
            (*our_pred).need_stat = (*our_pred).need_type;
            (*our_pred).args.str_0 = name;
            (*our_pred)
                .est_success_rate = estimate_pattern_match_rate(name, 0 as libc::c_int);
            return 1 as libc::c_int != 0;
        } else {
            *arg_ptr = saved_argc;
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn parse_negate(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    &mut argv;
    &mut arg_ptr;
    our_pred = get_new_pred_chk_op(entry, 0 as *const libc::c_char);
    (*our_pred).pred_func = Some(pred_negate as PREDICATEFUNCTION);
    (*our_pred).p_type = UNI_OP;
    (*our_pred).p_prec = NEGATE_PREC;
    (*our_pred).need_type = 0 as libc::c_int != 0;
    (*our_pred).need_stat = (*our_pred).need_type;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_newer(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    let mut stat_newer: stat = stat {
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
    let mut arg: *const libc::c_char = 0 as *const libc::c_char;
    set_stat_placeholders(&mut stat_newer);
    if collect_arg_stat_info(argv, arg_ptr, &mut stat_newer, &mut arg) {
        our_pred = insert_primary(entry, arg);
        (*our_pred).args.reftime.ts = get_stat_mtime(&mut stat_newer);
        (*our_pred).args.reftime.xval = XVAL_MTIME;
        (*our_pred).args.reftime.kind = COMP_GT;
        (*our_pred)
            .est_success_rate = estimate_timestamp_success_rate(
            stat_newer.st_mtim.tv_sec,
        );
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn parse_newerXY(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    if argv.is_null() || (*argv.offset(*arg_ptr as isize)).is_null() {
        return 0 as libc::c_int != 0
    } else if 8 as libc::c_uint as libc::c_ulong
        != strlen(*argv.offset(*arg_ptr as isize))
    {
        return 0 as libc::c_int != 0
    } else {
        let mut x: libc::c_char = 0;
        let mut y: libc::c_char = 0;
        let validchars: [libc::c_char; 6] = *::std::mem::transmute::<
            &[u8; 6],
            &[libc::c_char; 6],
        >(b"aBcmt\0");
        if 0 as libc::c_int
            == strncmp(
                b"-newer\0" as *const u8 as *const libc::c_char,
                *argv.offset(*arg_ptr as isize),
                6 as libc::c_int as libc::c_ulong,
            )
        {} else {
            __assert_fail(
                b"0 == strncmp (\"-newer\", argv[*arg_ptr], 6)\0" as *const u8
                    as *const libc::c_char,
                b"parser.c\0" as *const u8 as *const libc::c_char,
                1555 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"_Bool parse_newerXY(const struct parser_table *, char **, int *)\0"))
                    .as_ptr(),
            );
        }
        'c_23072: {
            if 0 as libc::c_int
                == strncmp(
                    b"-newer\0" as *const u8 as *const libc::c_char,
                    *argv.offset(*arg_ptr as isize),
                    6 as libc::c_int as libc::c_ulong,
                )
            {} else {
                __assert_fail(
                    b"0 == strncmp (\"-newer\", argv[*arg_ptr], 6)\0" as *const u8
                        as *const libc::c_char,
                    b"parser.c\0" as *const u8 as *const libc::c_char,
                    1555 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 65],
                        &[libc::c_char; 65],
                    >(
                        b"_Bool parse_newerXY(const struct parser_table *, char **, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        x = *(*argv.offset(*arg_ptr as isize)).offset(6 as libc::c_int as isize);
        y = *(*argv.offset(*arg_ptr as isize)).offset(7 as libc::c_int as isize);
        if 'B' as i32 == x as libc::c_int || 'B' as i32 == y as libc::c_int {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"This system does not provide a way to find the birth time of a file.\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as libc::c_int != 0;
        }
        if x as libc::c_int == 't' as i32
            || (strchr(validchars.as_ptr(), x as libc::c_int)).is_null()
            || (strchr(validchars.as_ptr(), y as libc::c_int)).is_null()
        {
            return 0 as libc::c_int != 0
        } else {
            let mut our_pred: *mut predicate = 0 as *mut predicate;
            if (*argv.offset((1 as libc::c_int + *arg_ptr) as isize)).is_null() {
                if ::std::mem::size_of::<C2RustUnnamed_31>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"The %s test needs an argument\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_n_style(
                            0 as libc::c_int,
                            options.err_quoting_style,
                            *argv.offset(*arg_ptr as isize),
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
                            b"The %s test needs an argument\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_n_style(
                            0 as libc::c_int,
                            options.err_quoting_style,
                            *argv.offset(*arg_ptr as isize),
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            } else {
                *arg_ptr += 1;
                *arg_ptr;
            }
            our_pred = insert_primary(entry, *argv.offset(*arg_ptr as isize));
            match x as libc::c_int {
                97 => {
                    (*our_pred).args.reftime.xval = XVAL_ATIME;
                }
                66 => {
                    (*our_pred).args.reftime.xval = XVAL_BIRTHTIME;
                }
                99 => {
                    (*our_pred).args.reftime.xval = XVAL_CTIME;
                }
                109 => {
                    (*our_pred).args.reftime.xval = XVAL_MTIME;
                }
                _ => {
                    if !(strchr(validchars.as_ptr(), x as libc::c_int)).is_null()
                    {} else {
                        __assert_fail(
                            b"strchr (validchars, x)\0" as *const u8
                                as *const libc::c_char,
                            b"parser.c\0" as *const u8 as *const libc::c_char,
                            1611 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 65],
                                &[libc::c_char; 65],
                            >(
                                b"_Bool parse_newerXY(const struct parser_table *, char **, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_22743: {
                        if !(strchr(validchars.as_ptr(), x as libc::c_int)).is_null()
                        {} else {
                            __assert_fail(
                                b"strchr (validchars, x)\0" as *const u8
                                    as *const libc::c_char,
                                b"parser.c\0" as *const u8 as *const libc::c_char,
                                1611 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 65],
                                    &[libc::c_char; 65],
                                >(
                                    b"_Bool parse_newerXY(const struct parser_table *, char **, int *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    __assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"parser.c\0" as *const u8 as *const libc::c_char,
                        1612 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 65],
                            &[libc::c_char; 65],
                        >(
                            b"_Bool parse_newerXY(const struct parser_table *, char **, int *)\0",
                        ))
                            .as_ptr(),
                    );
                    'c_22712: {
                        __assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"parser.c\0" as *const u8 as *const libc::c_char,
                            1612 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 65],
                                &[libc::c_char; 65],
                            >(
                                b"_Bool parse_newerXY(const struct parser_table *, char **, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                }
            }
            if 't' as i32 == y as libc::c_int {
                if !parse_datetime(
                    &mut (*our_pred).args.reftime.ts,
                    *argv.offset(*arg_ptr as isize),
                    &mut options.start_time,
                ) {
                    if ::std::mem::size_of::<C2RustUnnamed_30>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"I cannot figure out how to interpret %s as a date or time\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quotearg_n_style(
                                0 as libc::c_int,
                                options.err_quoting_style,
                                *argv.offset(*arg_ptr as isize),
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
                                b"I cannot figure out how to interpret %s as a date or time\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quotearg_n_style(
                                0 as libc::c_int,
                                options.err_quoting_style,
                                *argv.offset(*arg_ptr as isize),
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
            } else {
                let mut stat_newer: stat = stat {
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
                set_stat_placeholders(&mut stat_newer);
                if (Some((options.xstat).unwrap()))
                    .unwrap()(*argv.offset(*arg_ptr as isize), &mut stat_newer) != 0
                {
                    fatal_target_file_error(
                        *__errno_location(),
                        *argv.offset(*arg_ptr as isize),
                    );
                }
                if get_stat_Ytime(&mut stat_newer, y, &mut (*our_pred).args.reftime.ts)
                    == 0
                {
                    if ::std::mem::size_of::<C2RustUnnamed_29>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Cannot obtain birth time of file %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            safely_quote_err_filename(
                                0 as libc::c_int,
                                *argv.offset(*arg_ptr as isize),
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
                                b"Cannot obtain birth time of file %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            safely_quote_err_filename(
                                0 as libc::c_int,
                                *argv.offset(*arg_ptr as isize),
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
            }
            (*our_pred).args.reftime.kind = COMP_GT;
            (*our_pred)
                .est_success_rate = estimate_timestamp_success_rate(
                (*our_pred).args.reftime.ts.tv_sec,
            );
            *arg_ptr += 1;
            *arg_ptr;
            if ((*our_pred).pred_func).is_some() {} else {
                __assert_fail(
                    b"our_pred->pred_func != NULL\0" as *const u8 as *const libc::c_char,
                    b"parser.c\0" as *const u8 as *const libc::c_char,
                    1647 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 65],
                        &[libc::c_char; 65],
                    >(
                        b"_Bool parse_newerXY(const struct parser_table *, char **, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_22179: {
                if ((*our_pred).pred_func).is_some() {} else {
                    __assert_fail(
                        b"our_pred->pred_func != NULL\0" as *const u8
                            as *const libc::c_char,
                        b"parser.c\0" as *const u8 as *const libc::c_char,
                        1647 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 65],
                            &[libc::c_char; 65],
                        >(
                            b"_Bool parse_newerXY(const struct parser_table *, char **, int *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if (*our_pred).pred_func == Some(pred_newerXY as PREDICATEFUNCTION) {} else {
                __assert_fail(
                    b"our_pred->pred_func == pred_newerXY\0" as *const u8
                        as *const libc::c_char,
                    b"parser.c\0" as *const u8 as *const libc::c_char,
                    1648 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 65],
                        &[libc::c_char; 65],
                    >(
                        b"_Bool parse_newerXY(const struct parser_table *, char **, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_22136: {
                if (*our_pred).pred_func == Some(pred_newerXY as PREDICATEFUNCTION)
                {} else {
                    __assert_fail(
                        b"our_pred->pred_func == pred_newerXY\0" as *const u8
                            as *const libc::c_char,
                        b"parser.c\0" as *const u8 as *const libc::c_char,
                        1648 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 65],
                            &[libc::c_char; 65],
                        >(
                            b"_Bool parse_newerXY(const struct parser_table *, char **, int *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if (*our_pred).need_stat {} else {
                __assert_fail(
                    b"our_pred->need_stat\0" as *const u8 as *const libc::c_char,
                    b"parser.c\0" as *const u8 as *const libc::c_char,
                    1649 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 65],
                        &[libc::c_char; 65],
                    >(
                        b"_Bool parse_newerXY(const struct parser_table *, char **, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_22097: {
                if (*our_pred).need_stat {} else {
                    __assert_fail(
                        b"our_pred->need_stat\0" as *const u8 as *const libc::c_char,
                        b"parser.c\0" as *const u8 as *const libc::c_char,
                        1649 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 65],
                            &[libc::c_char; 65],
                        >(
                            b"_Bool parse_newerXY(const struct parser_table *, char **, int *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            return 1 as libc::c_int != 0;
        }
    };
}
unsafe extern "C" fn parse_noleaf(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    options.no_leaf_check = 1 as libc::c_int != 0;
    return parse_noop(entry, argv, arg_ptr);
}
unsafe extern "C" fn parse_nogroup(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    &mut argv;
    &mut arg_ptr;
    our_pred = insert_primary(entry, 0 as *const libc::c_char);
    (*our_pred).est_success_rate = 1e-4f64 as libc::c_float;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_nouser(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    our_pred = insert_primary_noarg(entry);
    (*our_pred).est_success_rate = 1e-3f64 as libc::c_float;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_nowarn(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    options.warnings = 0 as libc::c_int != 0;
    return parse_noop(entry, argv, arg_ptr);
}
unsafe extern "C" fn parse_ok(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    return insert_exec_ok(
        b"-ok\0" as *const u8 as *const libc::c_char,
        entry,
        argv,
        arg_ptr,
    );
}
unsafe extern "C" fn parse_okdir(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    return insert_exec_ok(
        b"-okdir\0" as *const u8 as *const libc::c_char,
        entry,
        argv,
        arg_ptr,
    );
}
pub unsafe extern "C" fn parse_openparen(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    our_pred = get_new_pred_chk_op(entry, 0 as *const libc::c_char);
    (*our_pred).pred_func = Some(pred_openparen as PREDICATEFUNCTION);
    (*our_pred).p_type = OPEN_PAREN;
    (*our_pred).p_prec = NO_PREC;
    (*our_pred).need_type = 0 as libc::c_int != 0;
    (*our_pred).need_stat = (*our_pred).need_type;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_or(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    our_pred = get_new_pred_noarg(entry);
    (*our_pred).pred_func = Some(pred_or as PREDICATEFUNCTION);
    (*our_pred).p_type = BI_OP;
    (*our_pred).p_prec = OR_PREC;
    (*our_pred).need_type = 0 as libc::c_int != 0;
    (*our_pred).need_stat = (*our_pred).need_type;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn is_feasible_path_argument(
    mut arg: *const libc::c_char,
    mut foldcase: bool,
) -> bool {
    let mut last: *const libc::c_char = strrchr(arg, '/' as i32);
    if !last.is_null() && *last.offset(1 as libc::c_int as isize) == 0 {
        if matches_start_point(arg, foldcase) {
            return 1 as libc::c_int != 0
        } else {
            return 0 as libc::c_int != 0
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn insert_path_check(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
    mut predicate_name: *const libc::c_char,
    mut pred: Option::<PREDICATEFUNCTION>,
) -> bool {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut foldcase: bool = 0 as libc::c_int != 0;
    if pred == Some(pred_ipath as PREDICATEFUNCTION) {
        foldcase = 1 as libc::c_int != 0;
    }
    fnmatch_sanitycheck();
    if collect_arg(argv, arg_ptr, &mut name) {
        let mut our_pred: *mut predicate = insert_primary_withpred(entry, pred, name);
        (*our_pred).need_type = 0 as libc::c_int != 0;
        (*our_pred).need_stat = (*our_pred).need_type;
        (*our_pred).args.str_0 = name;
        (*our_pred)
            .est_success_rate = estimate_pattern_match_rate(name, 0 as libc::c_int);
        if !options.posixly_correct && !is_feasible_path_argument(name, foldcase) {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"warning: -%s %s will not match anything because it ends with /.\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                predicate_name,
                name,
            );
            (*our_pred).est_success_rate = 1.0e-8f64 as libc::c_float;
        }
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn parse_path(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    return insert_path_check(
        entry,
        argv,
        arg_ptr,
        b"path\0" as *const u8 as *const libc::c_char,
        Some(pred_path as PREDICATEFUNCTION),
    );
}
unsafe extern "C" fn parse_wholename(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    return insert_path_check(
        entry,
        argv,
        arg_ptr,
        b"wholename\0" as *const u8 as *const libc::c_char,
        Some(pred_path as PREDICATEFUNCTION),
    );
}
unsafe extern "C" fn parse_ipath(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    return insert_path_check(
        entry,
        argv,
        arg_ptr,
        b"ipath\0" as *const u8 as *const libc::c_char,
        Some(pred_ipath as PREDICATEFUNCTION),
    );
}
unsafe extern "C" fn parse_iwholename(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    return insert_path_check(
        entry,
        argv,
        arg_ptr,
        b"iwholename\0" as *const u8 as *const libc::c_char,
        Some(pred_ipath as PREDICATEFUNCTION),
    );
}
unsafe extern "C" fn parse_perm(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut perm_val: [mode_t; 2] = [0; 2];
    let mut rate: libc::c_float = 0.;
    let mut mode_start: libc::c_int = 0 as libc::c_int;
    let mut kind: permissions_type = PERM_EXACT;
    let mut change: *mut mode_change = 0 as *mut mode_change;
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    let mut perm_expr: *const libc::c_char = 0 as *const libc::c_char;
    if !collect_arg(argv, arg_ptr, &mut perm_expr) {
        return 0 as libc::c_int != 0;
    }
    match *perm_expr.offset(0 as libc::c_int as isize) as libc::c_int {
        45 => {
            mode_start = 1 as libc::c_int;
            kind = PERM_AT_LEAST;
            rate = 0.2f64 as libc::c_float;
        }
        47 => {
            mode_start = 1 as libc::c_int;
            kind = PERM_ANY;
            rate = 0.3f64 as libc::c_float;
        }
        _ => {
            mode_start = 0 as libc::c_int;
            kind = PERM_EXACT;
            rate = 0.01f64 as libc::c_float;
        }
    }
    change = mode_compile(perm_expr.offset(mode_start as isize));
    if change.is_null()
        || *perm_expr.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
            && '0' as i32 <= *perm_expr.offset(1 as libc::c_int as isize) as libc::c_int
            && (*perm_expr.offset(1 as libc::c_int as isize) as libc::c_int) < '8' as i32
    {
        if ::std::mem::size_of::<C2RustUnnamed_15>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"invalid mode %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style(0 as libc::c_int, options.err_quoting_style, perm_expr),
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
                    b"invalid mode %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style(0 as libc::c_int, options.err_quoting_style, perm_expr),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    perm_val[0 as libc::c_int
        as usize] = mode_adjust(
        0 as libc::c_int as mode_t,
        0 as libc::c_int != 0,
        0 as libc::c_int as mode_t,
        change,
        0 as *mut mode_t,
    );
    perm_val[1 as libc::c_int
        as usize] = mode_adjust(
        0 as libc::c_int as mode_t,
        1 as libc::c_int != 0,
        0 as libc::c_int as mode_t,
        change,
        0 as *mut mode_t,
    );
    rpl_free(change as *mut libc::c_void);
    if '/' as i32 == *perm_expr.offset(0 as libc::c_int as isize) as libc::c_int
        && 0 as libc::c_int as libc::c_uint == perm_val[0 as libc::c_int as usize]
        && 0 as libc::c_int as libc::c_uint == perm_val[1 as libc::c_int as usize]
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"warning: you have specified a mode pattern %s (which is equivalent to /000). The meaning of -perm /000 has now been changed to be consistent with -perm -000; that is, while it used to match no files, it now matches all files.\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            perm_expr,
        );
        kind = PERM_AT_LEAST;
        rate = 0.9986f64 as libc::c_float;
    }
    our_pred = insert_primary(entry, perm_expr);
    (*our_pred).est_success_rate = rate;
    (*our_pred).args.perm.kind = kind;
    memcpy(
        ((*our_pred).args.perm.val).as_mut_ptr() as *mut libc::c_void,
        perm_val.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[mode_t; 2]>() as libc::c_ulong,
    );
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn parse_print(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    our_pred = insert_primary_noarg(entry);
    (*our_pred).no_default_print = 1 as libc::c_int != 0;
    (*our_pred).side_effects = (*our_pred).no_default_print;
    (*our_pred).need_type = 0 as libc::c_int != 0;
    (*our_pred).need_stat = (*our_pred).need_type;
    open_stdout(&mut (*our_pred).args.printf_vec);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_print0(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    return insert_fprint(entry, 0 as *const libc::c_char);
}
unsafe extern "C" fn parse_printf(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut format: *mut libc::c_char = 0 as *mut libc::c_char;
    let saved_argc: libc::c_int = *arg_ptr;
    if collect_arg_nonconst(argv, arg_ptr, &mut format) {
        let mut fmt: format_val = format_val {
            segment: 0 as *mut segment,
            stream: 0 as *mut FILE,
            filename: 0 as *const libc::c_char,
            dest_is_tty: false,
            quote_opts: 0 as *mut quoting_options,
        };
        open_stdout(&mut fmt);
        if insert_fprintf(&mut fmt, entry, format) {
            return 1 as libc::c_int != 0
        } else {
            *arg_ptr = saved_argc;
            return 0 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn parse_fprintf(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut format: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut saved_argc: libc::c_int = *arg_ptr;
    if collect_arg(argv, arg_ptr, &mut filename) {
        if collect_arg_nonconst(argv, arg_ptr, &mut format) {
            let mut fmt: format_val = format_val {
                segment: 0 as *mut segment,
                stream: 0 as *mut FILE,
                filename: 0 as *const libc::c_char,
                dest_is_tty: false,
                quote_opts: 0 as *mut quoting_options,
            };
            open_output_file(filename, &mut fmt);
            saved_argc = *arg_ptr;
            if insert_fprintf(&mut fmt, entry, format) {
                return 1 as libc::c_int != 0;
            }
        }
    }
    *arg_ptr = saved_argc;
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn parse_prune(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    our_pred = insert_primary_noarg(entry);
    if options.do_dir_first as libc::c_int == 0 as libc::c_int {
        (*our_pred).need_type = 0 as libc::c_int != 0;
        (*our_pred).need_stat = (*our_pred).need_type;
    }
    (*our_pred).side_effects = 1 as libc::c_int != 0;
    (*our_pred).no_default_print = 0 as libc::c_int != 0;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_quit(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut our_pred: *mut predicate = insert_primary_noarg(entry);
    (*our_pred).need_type = 0 as libc::c_int != 0;
    (*our_pred).need_stat = (*our_pred).need_type;
    (*our_pred).side_effects = 1 as libc::c_int != 0;
    (*our_pred).no_default_print = 0 as libc::c_int != 0;
    (*our_pred).est_success_rate = 1.0f32;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_regextype(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut type_name: *const libc::c_char = 0 as *const libc::c_char;
    if collect_arg(argv, arg_ptr, &mut type_name) {
        options.regex_options = get_regex_type(type_name);
        return parse_noop(entry, argv, arg_ptr);
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn parse_regex(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    return insert_regex(argv, arg_ptr, entry, options.regex_options);
}
unsafe extern "C" fn insert_regex(
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
    mut entry: *const parser_table,
    mut regex_options: libc::c_int,
) -> bool {
    let mut rx: *const libc::c_char = 0 as *const libc::c_char;
    if collect_arg(argv, arg_ptr, &mut rx) {
        let mut re: *mut re_pattern_buffer = 0 as *mut re_pattern_buffer;
        let mut error_message: *const libc::c_char = 0 as *const libc::c_char;
        let mut our_pred: *mut predicate = insert_primary_withpred(
            entry,
            Some(pred_regex as PREDICATEFUNCTION),
            rx,
        );
        (*our_pred).need_type = 0 as libc::c_int != 0;
        (*our_pred).need_stat = (*our_pred).need_type;
        re = xmalloc(::std::mem::size_of::<re_pattern_buffer>() as libc::c_ulong)
            as *mut re_pattern_buffer;
        (*our_pred).args.regex = re;
        (*re).allocated = 100 as libc::c_int as __re_long_size_t;
        (*re).buffer = xmalloc((*re).allocated) as *mut re_dfa_t;
        (*re).fastmap = 0 as *mut libc::c_char;
        rpl_re_set_syntax(regex_options as reg_syntax_t);
        (*re).syntax = regex_options as reg_syntax_t;
        (*re).translate = 0 as *mut libc::c_uchar;
        error_message = rpl_re_compile_pattern(rx, strlen(rx), re);
        if !error_message.is_null() {
            if ::std::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"failed to compile regular expression '%s': %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    rx,
                    error_message,
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
                        b"failed to compile regular expression '%s': %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    rx,
                    error_message,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        (*our_pred).est_success_rate = estimate_pattern_match_rate(rx, 1 as libc::c_int);
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn parse_size(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut num: uintmax_t = 0;
    let mut suffix: libc::c_char = 0;
    let mut c_type: comparison_type = COMP_GT;
    let mut blksize: libc::c_int = 512 as libc::c_int;
    let mut len: libc::c_int = 0;
    if argv.is_null() || (*argv.offset(*arg_ptr as isize)).is_null() {
        return 0 as libc::c_int != 0;
    }
    arg = *argv.offset(*arg_ptr as isize);
    len = strlen(arg) as libc::c_int;
    if len == 0 as libc::c_int {
        if ::std::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"invalid null argument to -size\0" as *const u8
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
                    b"invalid null argument to -size\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    suffix = *arg.offset((len - 1 as libc::c_int) as isize);
    match suffix as libc::c_int {
        98 => {
            blksize = 512 as libc::c_int;
            *arg.offset((len - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        }
        99 => {
            blksize = 1 as libc::c_int;
            *arg.offset((len - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        }
        107 => {
            blksize = 1024 as libc::c_int;
            *arg.offset((len - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        }
        77 => {
            blksize = 1024 as libc::c_int * 1024 as libc::c_int;
            *arg.offset((len - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        }
        71 => {
            blksize = 1024 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int;
            *arg.offset((len - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        }
        119 => {
            blksize = 2 as libc::c_int;
            *arg.offset((len - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        }
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
            suffix = 0 as libc::c_int as libc::c_char;
        }
        _ => {
            if ::std::mem::size_of::<C2RustUnnamed_12>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid -size type `%c'\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    *(*argv.offset(*arg_ptr as isize))
                        .offset((len - 1 as libc::c_int) as isize) as libc::c_int,
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
                        b"invalid -size type `%c'\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    *(*argv.offset(*arg_ptr as isize))
                        .offset((len - 1 as libc::c_int) as isize) as libc::c_int,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    if !get_num(arg, &mut num, &mut c_type) {
        let mut tail: [libc::c_char; 2] = [0; 2];
        tail[0 as libc::c_int as usize] = suffix;
        tail[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        if ::std::mem::size_of::<C2RustUnnamed_11>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid argument `%s%s' to -size\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                arg,
                tail.as_mut_ptr(),
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
                    b"Invalid argument `%s%s' to -size\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                arg,
                tail.as_mut_ptr(),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
        return 0 as libc::c_int != 0;
    }
    our_pred = insert_primary(entry, arg);
    (*our_pred).args.size.kind = c_type;
    (*our_pred).args.size.blocksize = blksize;
    (*our_pred).args.size.size = num;
    (*our_pred).need_stat = 1 as libc::c_int != 0;
    (*our_pred).need_type = 0 as libc::c_int != 0;
    if COMP_GT as libc::c_int as libc::c_uint == c_type as libc::c_uint {
        (*our_pred)
            .est_success_rate = (if num.wrapping_mul(blksize as libc::c_ulong)
            > 20480 as libc::c_int as libc::c_ulong
        {
            0.1f64
        } else {
            0.9f64
        }) as libc::c_float;
    } else if COMP_LT as libc::c_int as libc::c_uint == c_type as libc::c_uint {
        (*our_pred)
            .est_success_rate = (if num.wrapping_mul(blksize as libc::c_ulong)
            > 20480 as libc::c_int as libc::c_ulong
        {
            0.9f64
        } else {
            0.1f64
        }) as libc::c_float;
    } else {
        (*our_pred).est_success_rate = 0.01f64 as libc::c_float;
    }
    *arg_ptr += 1;
    *arg_ptr;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_samefile(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
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
    let mut fst: stat = stat {
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
    let mut fd: libc::c_int = 0;
    let mut openflags: libc::c_int = 0;
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    set_stat_placeholders(&mut st);
    if !collect_arg_stat_info(argv, arg_ptr, &mut st, &mut filename) {
        return 0 as libc::c_int != 0;
    }
    set_stat_placeholders(&mut fst);
    fd = -(3 as libc::c_int);
    openflags = 0 as libc::c_int;
    if options.symlink_handling as libc::c_uint
        == SYMLINK_NEVER_DEREF as libc::c_int as libc::c_uint
    {
        if options.open_nofollow_available {
            if 0o400000 as libc::c_int != 0 as libc::c_int {} else {
                __assert_fail(
                    b"O_NOFOLLOW != 0\0" as *const u8 as *const libc::c_char,
                    b"parser.c\0" as *const u8 as *const libc::c_char,
                    2209 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 66],
                        &[libc::c_char; 66],
                    >(
                        b"_Bool parse_samefile(const struct parser_table *, char **, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_14322: {
                if 0o400000 as libc::c_int != 0 as libc::c_int {} else {
                    __assert_fail(
                        b"O_NOFOLLOW != 0\0" as *const u8 as *const libc::c_char,
                        b"parser.c\0" as *const u8 as *const libc::c_char,
                        2209 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 66],
                            &[libc::c_char; 66],
                        >(
                            b"_Bool parse_samefile(const struct parser_table *, char **, int *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            openflags |= 0o400000 as libc::c_int;
            fd = -(1 as libc::c_int);
        } else if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
        {
            fd = -(2 as libc::c_int);
        } else {
            fd = -(1 as libc::c_int);
        }
    } else {
        fd = -(1 as libc::c_int);
    }
    if fd != -(3 as libc::c_int) {} else {
        __assert_fail(
            b"fd != -3\0" as *const u8 as *const libc::c_char,
            b"parser.c\0" as *const u8 as *const libc::c_char,
            2239 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"_Bool parse_samefile(const struct parser_table *, char **, int *)\0"))
                .as_ptr(),
        );
    }
    'c_14232: {
        if fd != -(3 as libc::c_int) {} else {
            __assert_fail(
                b"fd != -3\0" as *const u8 as *const libc::c_char,
                b"parser.c\0" as *const u8 as *const libc::c_char,
                2239 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"_Bool parse_samefile(const struct parser_table *, char **, int *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if fd == -(1 as libc::c_int) {
        fd = open_cloexec(filename, openflags);
        if fd >= 0 as libc::c_int {
            if 0 as libc::c_int != fstat(fd, &mut fst) {
                fatal_target_file_error(*__errno_location(), filename);
            } else {
                if (Some((options.xstat).unwrap())).unwrap()(filename, &mut st) != 0 {
                    fatal_target_file_error(*__errno_location(), filename);
                }
                if options.symlink_handling as libc::c_uint
                    == SYMLINK_NEVER_DEREF as libc::c_int as libc::c_uint
                    && !options.open_nofollow_available
                {
                    if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o120000 as libc::c_int as libc::c_uint
                    {
                        close(fd);
                        fd = -(1 as libc::c_int);
                    } else if !(st.st_dev == fst.st_dev && st.st_ino == fst.st_ino) {
                        close(fd);
                        fd = -(1 as libc::c_int);
                    }
                } else {
                    st = fst;
                }
            }
        }
    }
    our_pred = insert_primary(entry, filename);
    (*our_pred).args.samefileid.ino = st.st_ino;
    (*our_pred).args.samefileid.dev = st.st_dev;
    (*our_pred).args.samefileid.fd = fd;
    (*our_pred).need_type = 0 as libc::c_int != 0;
    (*our_pred).need_stat = 1 as libc::c_int != 0;
    (*our_pred).est_success_rate = 0.01f32;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_true(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    our_pred = insert_primary_noarg(entry);
    (*our_pred).need_type = 0 as libc::c_int != 0;
    (*our_pred).need_stat = (*our_pred).need_type;
    (*our_pred).est_success_rate = 1.0f32;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_noop(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    return parse_true(get_noop(), argv, arg_ptr);
}
unsafe extern "C" fn parse_accesscheck(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    our_pred = insert_primary_noarg(entry);
    (*our_pred).need_type = 0 as libc::c_int != 0;
    (*our_pred).need_stat = (*our_pred).need_type;
    (*our_pred).no_default_print = 0 as libc::c_int != 0;
    (*our_pred).side_effects = (*our_pred).no_default_print;
    if (*our_pred).pred_func == Some(pred_executable as PREDICATEFUNCTION) {
        (*our_pred).est_success_rate = 0.2f64 as libc::c_float;
    } else {
        (*our_pred).est_success_rate = 0.9f64 as libc::c_float;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_type(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    return insert_type(argv, arg_ptr, entry, Some(pred_type as PREDICATEFUNCTION));
}
unsafe extern "C" fn parse_uid(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut p: *mut predicate = insert_num(argv, arg_ptr, entry);
    if !p.is_null() {
        (*p)
            .est_success_rate = (if (*p).args.numinfo.l_val
            < 100 as libc::c_int as libc::c_ulong
        {
            0.99f64
        } else {
            0.2f64
        }) as libc::c_float;
        return 1 as libc::c_int != 0;
    } else {
        *arg_ptr -= 1;
        *arg_ptr;
        return 0 as libc::c_int != 0;
    };
}
unsafe extern "C" fn parse_used(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    let mut tval: time_val = time_val {
        xval: XVAL_ATIME,
        kind: COMP_GT,
        ts: timespec { tv_sec: 0, tv_nsec: 0 },
    };
    let mut offset_str: *const libc::c_char = 0 as *const libc::c_char;
    let mut errmsg: *const libc::c_char = b"arithmetic overflow while converting %s days to a number of seconds\0"
        as *const u8 as *const libc::c_char;
    if collect_arg(argv, arg_ptr, &mut offset_str) {
        let mut zero: timespec = {
            let mut init = timespec {
                tv_sec: 0 as libc::c_int as __time_t,
                tv_nsec: 0 as libc::c_int as __syscall_slong_t,
            };
            init
        };
        if get_relative_timestamp(
            offset_str,
            &mut tval,
            zero,
            86400 as libc::c_int as libc::c_double,
            errmsg,
        ) {
            our_pred = insert_primary(entry, offset_str);
            (*our_pred).args.reftime = tval;
            (*our_pred)
                .est_success_rate = estimate_file_age_success_rate(
                (tval.ts.tv_sec / 86400 as libc::c_int as libc::c_long) as libc::c_float,
            );
            return 1 as libc::c_int != 0;
        } else {
            if ::std::mem::size_of::<C2RustUnnamed_9>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Invalid argument %s to -used\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    offset_str,
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
                        b"Invalid argument %s to -used\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    offset_str,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
            return 0 as libc::c_int != 0;
        }
    } else {
        return 0 as libc::c_int != 0
    };
}
unsafe extern "C" fn parse_user(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut username: *const libc::c_char = 0 as *const libc::c_char;
    if collect_arg(argv, arg_ptr, &mut username) {
        let mut our_pred: *mut predicate = 0 as *mut predicate;
        let mut uid: uid_t = 0;
        let mut cur_pwd: *mut passwd = getpwnam(username);
        endpwent();
        if !cur_pwd.is_null() {
            uid = (*cur_pwd).pw_uid;
        } else {
            let uid_len: size_t = strspn(
                username,
                b"0123456789\0" as *const u8 as *const libc::c_char,
            );
            if uid_len != 0
                && *username.offset(uid_len as isize) as libc::c_int == 0 as libc::c_int
            {
                uid = safe_atoi(username, options.err_quoting_style) as uid_t;
            } else {
                if *username.offset(0 as libc::c_int as isize) != 0 {
                    if ::std::mem::size_of::<C2RustUnnamed_8>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s is not the name of a known user\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quotearg_n_style(
                                0 as libc::c_int,
                                options.err_quoting_style,
                                username,
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
                                b"%s is not the name of a known user\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quotearg_n_style(
                                0 as libc::c_int,
                                options.err_quoting_style,
                                username,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                } else {
                    if ::std::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"The argument to -user should not be empty\0" as *const u8
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
                                b"The argument to -user should not be empty\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                return 0 as libc::c_int != 0;
            }
        }
        our_pred = insert_primary(entry, username);
        (*our_pred).args.uid = uid;
        (*our_pred)
            .est_success_rate = (if (*our_pred).args.uid
            < 100 as libc::c_int as libc::c_uint
        {
            0.99f64
        } else {
            0.2f64
        }) as libc::c_float;
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn parse_version(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> ! {
    let mut has_features: bool = 0 as libc::c_int != 0;
    let mut flags: libc::c_int = 0;
    display_findutils_version(b"find\0" as *const u8 as *const libc::c_char);
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Features enabled: \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    printf(b"D_TYPE \0" as *const u8 as *const libc::c_char);
    has_features = 1 as libc::c_int != 0;
    printf(
        b"O_NOFOLLOW(%s) \0" as *const u8 as *const libc::c_char,
        if options.open_nofollow_available as libc::c_int != 0 {
            b"enabled\0" as *const u8 as *const libc::c_char
        } else {
            b"disabled\0" as *const u8 as *const libc::c_char
        },
    );
    has_features = 1 as libc::c_int != 0;
    printf(b"LEAF_OPTIMISATION \0" as *const u8 as *const libc::c_char);
    has_features = 1 as libc::c_int != 0;
    if (0 as libc::c_int) < is_selinux_enabled() {
        printf(b"SELINUX \0" as *const u8 as *const libc::c_char);
        has_features = 1 as libc::c_int != 0;
    }
    flags = 0 as libc::c_int;
    if is_fts_enabled(&mut flags) {
        printf(b"FTS(\0" as *const u8 as *const libc::c_char);
        has_features = 1 as libc::c_int != 0;
        if flags & 0x200 as libc::c_int != 0 {
            printf(b"FTS_CWDFD\0" as *const u8 as *const libc::c_char);
        }
        printf(b") \0" as *const u8 as *const libc::c_char);
    }
    printf(
        b"CBO(level=%d) \0" as *const u8 as *const libc::c_char,
        options.optimisation_level as libc::c_int,
    );
    has_features = 1 as libc::c_int != 0;
    if !has_features {
        printf(b"none\0" as *const u8 as *const libc::c_char);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    exit(0 as libc::c_int);
}
unsafe extern "C" fn parse_context(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    if argv.is_null() || (*argv.offset(*arg_ptr as isize)).is_null() {
        return 0 as libc::c_int != 0;
    }
    if is_selinux_enabled() <= 0 as libc::c_int {
        if ::std::mem::size_of::<C2RustUnnamed_28>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"invalid predicate -context: SELinux is not enabled.\0" as *const u8
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
                    b"invalid predicate -context: SELinux is not enabled.\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
        return 0 as libc::c_int != 0;
    }
    our_pred = insert_primary(entry, 0 as *const libc::c_char);
    (*our_pred).est_success_rate = 0.01f32;
    (*our_pred).need_stat = 0 as libc::c_int != 0;
    (*our_pred).args.scontext = *argv.offset(*arg_ptr as isize);
    *arg_ptr += 1;
    *arg_ptr;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_xdev(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    options.stay_on_filesystem = 1 as libc::c_int != 0;
    return parse_noop(entry, argv, arg_ptr);
}
unsafe extern "C" fn parse_ignore_race(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    options.ignore_readdir_race = 1 as libc::c_int != 0;
    return parse_noop(entry, argv, arg_ptr);
}
unsafe extern "C" fn parse_noignore_race(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    options.ignore_readdir_race = 0 as libc::c_int != 0;
    return parse_noop(entry, argv, arg_ptr);
}
unsafe extern "C" fn parse_warn(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    options.warnings = 1 as libc::c_int != 0;
    return parse_noop(entry, argv, arg_ptr);
}
unsafe extern "C" fn parse_xtype(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    return insert_type(argv, arg_ptr, entry, Some(pred_xtype as PREDICATEFUNCTION));
}
unsafe extern "C" fn insert_type(
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
    mut entry: *const parser_table,
    mut which_pred: PRED_FUNC,
) -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    let mut typeletter: *const libc::c_char = 0 as *const libc::c_char;
    let mut pred_string: *const libc::c_char = if which_pred
        == Some(pred_xtype as PREDICATEFUNCTION)
    {
        b"-xtype\0" as *const u8 as *const libc::c_char
    } else {
        b"-type\0" as *const u8 as *const libc::c_char
    };
    if !collect_arg(argv, arg_ptr, &mut typeletter) {
        return 0 as libc::c_int != 0;
    }
    if *typeletter == 0 {
        if ::std::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Arguments to %s should contain at least one letter\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                pred_string,
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
                    b"Arguments to %s should contain at least one letter\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                pred_string,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
        return 0 as libc::c_int != 0;
    }
    our_pred = insert_primary_withpred(entry, which_pred, typeletter);
    (*our_pred).est_success_rate = 0.0f64 as libc::c_float;
    if which_pred == Some(pred_xtype as PREDICATEFUNCTION) {
        (*our_pred).need_stat = 1 as libc::c_int != 0;
        (*our_pred).need_type = 0 as libc::c_int != 0;
    } else {
        (*our_pred).need_stat = 0 as libc::c_int != 0;
        (*our_pred).need_type = 1 as libc::c_int != 0;
    }
    while *typeletter != 0 {
        let mut type_cell: libc::c_uint = 0;
        let mut rate: libc::c_float = 0.01f64 as libc::c_float;
        match *typeletter as libc::c_int {
            98 => {
                type_cell = FTYPE_BLK as libc::c_int as libc::c_uint;
                rate = 0.000888f32;
            }
            99 => {
                type_cell = FTYPE_CHR as libc::c_int as libc::c_uint;
                rate = 0.000443f32;
            }
            100 => {
                type_cell = FTYPE_DIR as libc::c_int as libc::c_uint;
                rate = 0.0922f32;
            }
            102 => {
                type_cell = FTYPE_REG as libc::c_int as libc::c_uint;
                rate = 0.875f32;
            }
            108 => {
                type_cell = FTYPE_LNK as libc::c_int as libc::c_uint;
                rate = 0.0311f32;
            }
            112 => {
                type_cell = FTYPE_FIFO as libc::c_int as libc::c_uint;
                rate = 7.554e-6f32;
            }
            115 => {
                type_cell = FTYPE_SOCK as libc::c_int as libc::c_uint;
                rate = 1.59e-5f32;
            }
            68 => {
                type_cell = 0 as libc::c_int as libc::c_uint;
                if ::std::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s %c is not supported because Solaris doors are not supported on the platform find was compiled on.\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        pred_string,
                        *typeletter as libc::c_int,
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
                            b"%s %c is not supported because Solaris doors are not supported on the platform find was compiled on.\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        pred_string,
                        *typeletter as libc::c_int,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            _ => {
                type_cell = 0 as libc::c_int as libc::c_uint;
                if ::std::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Unknown argument to %s: %c\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        pred_string,
                        *typeletter as libc::c_int,
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
                            b"Unknown argument to %s: %c\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        pred_string,
                        *typeletter as libc::c_int,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
                return 0 as libc::c_int != 0;
            }
        }
        if (*our_pred).args.types[type_cell as usize] {
            if ::std::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Duplicate file type '%c' in the argument list to %s.\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    *typeletter as libc::c_int,
                    pred_string,
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
                        b"Duplicate file type '%c' in the argument list to %s.\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    *typeletter as libc::c_int,
                    pred_string,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        (*our_pred).est_success_rate += rate;
        (*our_pred).args.types[type_cell as usize] = 1 as libc::c_int != 0;
        typeletter = typeletter.offset(1);
        typeletter;
        if *typeletter != 0 {
            if *typeletter as libc::c_int != ',' as i32 {
                if ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Must separate multiple arguments to %s using: ','\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        pred_string,
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
                            b"Must separate multiple arguments to %s using: ','\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        pred_string,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
                return 0 as libc::c_int != 0;
            }
            typeletter = typeletter.offset(1);
            typeletter;
            if *typeletter == 0 {
                if ::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Last file type in list argument to %s is missing, i.e., list is ending on: ','\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        pred_string,
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
                            b"Last file type in list argument to %s is missing, i.e., list is ending on: ','\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        pred_string,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
                return 0 as libc::c_int != 0;
            }
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn stream_is_tty(mut fp: *mut FILE) -> bool {
    let mut fd: libc::c_int = fileno(fp);
    if -(1 as libc::c_int) == fd {
        return 0 as libc::c_int != 0
    } else {
        return if isatty(fd) != 0 { 1 as libc::c_int } else { 0 as libc::c_int } != 0
    };
}
unsafe extern "C" fn check_path_safety(mut action: *const libc::c_char) {
    let mut path: *const libc::c_char = getenv(
        b"PATH\0" as *const u8 as *const libc::c_char,
    );
    let mut path_separators: *const libc::c_char = b":\0" as *const u8
        as *const libc::c_char;
    let mut pos: size_t = 0;
    let mut len: size_t = 0;
    if path.is_null() {
        return;
    }
    splitstring(path, path_separators, 1 as libc::c_int != 0, &mut pos, &mut len);
    loop {
        if 0 as libc::c_int as libc::c_ulong == len
            || 1 as libc::c_int as libc::c_ulong == len
                && *path.offset(pos as isize) as libc::c_int == '.' as i32
        {
            if ::std::mem::size_of::<C2RustUnnamed_22>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"The current directory is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove the current directory from your $PATH (that is, remove \".\", doubled colons, or leading or trailing colons)\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    action,
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
                        b"The current directory is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove the current directory from your $PATH (that is, remove \".\", doubled colons, or leading or trailing colons)\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    action,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        } else if *path.offset(pos as isize) as libc::c_int != '/' as i32 {
            let mut relpath: *mut libc::c_char = strndup(
                &*path.offset(pos as isize),
                len,
            );
            if ::std::mem::size_of::<C2RustUnnamed_21>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"The relative path %s is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove that entry from $PATH\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    safely_quote_err_filename(
                        0 as libc::c_int,
                        if !relpath.is_null() {
                            relpath as *const libc::c_char
                        } else {
                            &*path.offset(pos as isize)
                        },
                    ),
                    action,
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
                        b"The relative path %s is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove that entry from $PATH\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    safely_quote_err_filename(
                        0 as libc::c_int,
                        if !relpath.is_null() {
                            relpath as *const libc::c_char
                        } else {
                            &*path.offset(pos as isize)
                        },
                    ),
                    action,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
            rpl_free(relpath as *mut libc::c_void);
        }
        if !splitstring(
            path,
            path_separators,
            0 as libc::c_int != 0,
            &mut pos,
            &mut len,
        ) {
            break;
        }
    };
}
unsafe extern "C" fn insert_exec_ok(
    mut action: *const libc::c_char,
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut saw_braces: libc::c_int = 0;
    let mut allow_plus: bool = false;
    let mut brace_count: libc::c_int = 0;
    let mut brace_arg: *const libc::c_char = 0 as *const libc::c_char;
    let mut func: PRED_FUNC = (*entry).pred_func;
    let mut bcstatus: BC_INIT_STATUS = BC_INIT_OK;
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    let mut execp: *mut exec_val = 0 as *mut exec_val;
    if argv.is_null() || (*argv.offset(*arg_ptr as isize)).is_null() {
        return 0 as libc::c_int != 0;
    }
    our_pred = insert_primary_withpred(
        entry,
        func,
        b"(some -exec* arguments)\0" as *const u8 as *const libc::c_char,
    );
    (*our_pred).no_default_print = 1 as libc::c_int != 0;
    (*our_pred).side_effects = (*our_pred).no_default_print;
    (*our_pred).need_stat = 0 as libc::c_int != 0;
    (*our_pred).need_type = (*our_pred).need_stat;
    execp = &mut (*our_pred).args.exec_vec;
    (*execp).wd_for_exec = 0 as *mut saved_cwd;
    if func != Some(pred_okdir as PREDICATEFUNCTION)
        && func != Some(pred_ok as PREDICATEFUNCTION)
    {
        allow_plus = 1 as libc::c_int != 0;
        (*execp).close_stdin = 0 as libc::c_int != 0;
    } else {
        allow_plus = 0 as libc::c_int != 0;
        options.ok_prompt_stdin = 1 as libc::c_int != 0;
        (*execp).close_stdin = 1 as libc::c_int != 0;
    }
    if func == Some(pred_execdir as PREDICATEFUNCTION)
        || func == Some(pred_okdir as PREDICATEFUNCTION)
    {
        (*execp).wd_for_exec = 0 as *mut saved_cwd;
        options.ignore_readdir_race = 0 as libc::c_int != 0;
        check_path_safety(action);
    } else {
        if !initial_wd.is_null() {} else {
            __assert_fail(
                b"NULL != initial_wd\0" as *const u8 as *const libc::c_char,
                b"parser.c\0" as *const u8 as *const libc::c_char,
                2937 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 80],
                    &[libc::c_char; 80],
                >(
                    b"_Bool insert_exec_ok(const char *, const struct parser_table *, char **, int *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_16921: {
            if !initial_wd.is_null() {} else {
                __assert_fail(
                    b"NULL != initial_wd\0" as *const u8 as *const libc::c_char,
                    b"parser.c\0" as *const u8 as *const libc::c_char,
                    2937 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 80],
                        &[libc::c_char; 80],
                    >(
                        b"_Bool insert_exec_ok(const char *, const struct parser_table *, char **, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        (*execp).wd_for_exec = initial_wd;
    }
    (*our_pred).args.exec_vec.multiple = 0 as libc::c_int != 0;
    start = *arg_ptr;
    end = start;
    saw_braces = 0 as libc::c_int;
    brace_count = 0 as libc::c_int;
    brace_arg = 0 as *const libc::c_char;
    while !(*argv.offset(end as isize)).is_null()
        && (*(*argv.offset(end as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int != ';' as i32
            || *(*argv.offset(end as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int != '\0' as i32)
    {
        if allow_plus as libc::c_int != 0
            && *(*argv.offset(end as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int == '+' as i32
            && *(*argv.offset(end as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int == 0 as libc::c_int && saw_braces != 0
        {
            (*our_pred).args.exec_vec.multiple = 1 as libc::c_int != 0;
            break;
        } else {
            saw_braces = 0 as libc::c_int;
            if !(mbsstr(
                *argv.offset(end as isize),
                b"{}\0" as *const u8 as *const libc::c_char,
            ))
                .is_null()
            {
                saw_braces = 1 as libc::c_int;
                brace_arg = *argv.offset(end as isize);
                brace_count += 1;
                brace_count;
                if 0 as libc::c_int == end
                    && (func == Some(pred_execdir as PREDICATEFUNCTION)
                        || func == Some(pred_okdir as PREDICATEFUNCTION))
                {
                    if ::std::mem::size_of::<C2RustUnnamed_20>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"You may not use {} within the utility name for -execdir and -okdir, because this is a potential security problem.\0"
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
                                b"You may not use {} within the utility name for -execdir and -okdir, because this is a potential security problem.\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
            }
            end += 1;
            end;
        }
    }
    if end == start || (*argv.offset(end as isize)).is_null() {
        *arg_ptr = end;
        rpl_free(our_pred as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    if (*our_pred).args.exec_vec.multiple {
        let mut suffix: *const libc::c_char = 0 as *const libc::c_char;
        if func == Some(pred_execdir as PREDICATEFUNCTION) {
            suffix = b"dir\0" as *const u8 as *const libc::c_char;
        } else {
            suffix = b"\0" as *const u8 as *const libc::c_char;
        }
        if brace_count > 1 as libc::c_int {
            if ::std::mem::size_of::<C2RustUnnamed_19>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Only one instance of {} is supported with -exec%s ... +\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    suffix,
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
                        b"Only one instance of {} is supported with -exec%s ... +\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    suffix,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        } else if strlen(brace_arg) != 2 as libc::c_uint as libc::c_ulong {
            let mut buf: [libc::c_char; 19] = [0; 19];
            let needed: size_t = snprintf(
                buf.as_mut_ptr(),
                MsgBufSize as libc::c_int as libc::c_ulong,
                b"-exec%s ... {} +\0" as *const u8 as *const libc::c_char,
                suffix,
            ) as size_t;
            if needed <= MsgBufSize as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"needed <= MsgBufSize\0" as *const u8 as *const libc::c_char,
                    b"parser.c\0" as *const u8 as *const libc::c_char,
                    3014 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 80],
                        &[libc::c_char; 80],
                    >(
                        b"_Bool insert_exec_ok(const char *, const struct parser_table *, char **, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_16455: {
                if needed <= MsgBufSize as libc::c_int as libc::c_ulong {} else {
                    __assert_fail(
                        b"needed <= MsgBufSize\0" as *const u8 as *const libc::c_char,
                        b"parser.c\0" as *const u8 as *const libc::c_char,
                        3014 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 80],
                            &[libc::c_char; 80],
                        >(
                            b"_Bool insert_exec_ok(const char *, const struct parser_table *, char **, int *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if ::std::mem::size_of::<C2RustUnnamed_17>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"In %s the %s must appear by itself, but you specified %s\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style(
                        0 as libc::c_int,
                        options.err_quoting_style,
                        buf.as_mut_ptr(),
                    ),
                    quotearg_n_style(
                        1 as libc::c_int,
                        options.err_quoting_style,
                        b"{}\0" as *const u8 as *const libc::c_char,
                    ),
                    quotearg_n_style(
                        2 as libc::c_int,
                        options.err_quoting_style,
                        brace_arg,
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
                        b"In %s the %s must appear by itself, but you specified %s\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style(
                        0 as libc::c_int,
                        options.err_quoting_style,
                        buf.as_mut_ptr(),
                    ),
                    quotearg_n_style(
                        1 as libc::c_int,
                        options.err_quoting_style,
                        b"{}\0" as *const u8 as *const libc::c_char,
                    ),
                    quotearg_n_style(
                        2 as libc::c_int,
                        options.err_quoting_style,
                        brace_arg,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    bcstatus = bc_init_controlinfo(&mut (*execp).ctl, 2048 as libc::c_uint as size_t);
    match bcstatus as libc::c_uint {
        1 | 2 => {
            if ::std::mem::size_of::<C2RustUnnamed_16>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"The environment is too large for exec().\0" as *const u8
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
                        b"The environment is too large for exec().\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        0 | _ => {}
    }
    bc_use_sensible_arg_max(&mut (*execp).ctl);
    (*execp)
        .ctl
        .exec_callback = Some(
        launch
            as unsafe extern "C" fn(
                *mut buildcmd_control,
                *mut libc::c_void,
                libc::c_int,
                *mut *mut libc::c_char,
            ) -> libc::c_int,
    );
    if (*our_pred).args.exec_vec.multiple {
        (*execp).replace_vec = 0 as *mut *mut libc::c_char;
        (*execp).ctl.replace_pat = 0 as *const libc::c_char;
        (*execp).ctl.rplen = 0 as libc::c_int as size_t;
        (*execp).ctl.lines_per_exec = 0 as libc::c_int as libc::c_ulong;
        (*execp).ctl.args_per_exec = 0 as libc::c_int as size_t;
        (*execp).ctl.initial_argc = (end - start - 1 as libc::c_int) as size_t;
        bc_init_state(
            &mut (*execp).ctl,
            &mut (*execp).state,
            execp as *mut libc::c_void,
        );
        i = start;
        while i < end - 1 as libc::c_int {
            bc_push_arg(
                &mut (*execp).ctl,
                &mut (*execp).state,
                *argv.offset(i as isize),
                (strlen(*argv.offset(i as isize)))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                0 as *const libc::c_char,
                0 as libc::c_int as size_t,
                1 as libc::c_int,
            );
            i += 1;
            i;
        }
    } else {
        (*execp).num_args = end - start;
        (*execp).ctl.replace_pat = b"{}\0" as *const u8 as *const libc::c_char;
        (*execp).ctl.rplen = strlen((*execp).ctl.replace_pat);
        (*execp).ctl.lines_per_exec = 0 as libc::c_int as libc::c_ulong;
        (*execp).ctl.args_per_exec = 0 as libc::c_int as size_t;
        (*execp)
            .replace_vec = xmalloc(
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul((*execp).num_args as libc::c_ulong),
        ) as *mut *mut libc::c_char;
        bc_init_state(
            &mut (*execp).ctl,
            &mut (*execp).state,
            execp as *mut libc::c_void,
        );
        i = 0 as libc::c_int;
        while i < (*execp).num_args {
            let ref mut fresh0 = *((*execp).replace_vec).offset(i as isize);
            *fresh0 = *argv.offset((i + start) as isize);
            i += 1;
            i;
        }
    }
    if (*argv.offset(end as isize)).is_null() {
        *arg_ptr = end;
    } else {
        *arg_ptr = end + 1 as libc::c_int;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn get_relative_timestamp(
    mut str: *const libc::c_char,
    mut result: *mut time_val,
    mut origin: timespec,
    mut sec_per_unit: libc::c_double,
    mut overflowmessage: *const libc::c_char,
) -> bool {
    let mut offset: libc::c_double = 0.;
    let mut seconds: libc::c_double = 0.;
    let mut nanosec: libc::c_double = 0.;
    static mut nanosec_per_sec: libc::c_long = 1000000000 as libc::c_int as libc::c_long;
    if get_comp_type(&mut str, &mut (*result).kind) {
        match (*result).kind as libc::c_uint {
            1 => {
                (*result).kind = COMP_GT;
            }
            0 => {
                (*result).kind = COMP_LT;
            }
            2 | _ => {}
        }
        if xstrtod(
            str,
            0 as *mut *const libc::c_char,
            &mut offset,
            Some(
                strtod
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut *mut libc::c_char,
                    ) -> libc::c_double,
            ),
        ) {
            nanosec = modf(offset * sec_per_unit, &mut seconds);
            nanosec *= 1.0e9f64;
            if nanosec < nanosec_per_sec as libc::c_double {} else {
                __assert_fail(
                    b"nanosec < nanosec_per_sec\0" as *const u8 as *const libc::c_char,
                    b"parser.c\0" as *const u8 as *const libc::c_char,
                    3149 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 101],
                        &[libc::c_char; 101],
                    >(
                        b"_Bool get_relative_timestamp(const char *, struct time_val *, struct timespec, double, const char *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_12764: {
                if nanosec < nanosec_per_sec as libc::c_double {} else {
                    __assert_fail(
                        b"nanosec < nanosec_per_sec\0" as *const u8
                            as *const libc::c_char,
                        b"parser.c\0" as *const u8 as *const libc::c_char,
                        3149 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 101],
                            &[libc::c_char; 101],
                        >(
                            b"_Bool get_relative_timestamp(const char *, struct time_val *, struct timespec, double, const char *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            (*result)
                .ts
                .tv_sec = (origin.tv_sec as libc::c_double - seconds) as __time_t;
            if (origin.tv_sec < (*result).ts.tv_sec) as libc::c_int
                != (seconds < 0 as libc::c_int as libc::c_double) as libc::c_int
            {
                if ::std::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong != 0 {
                    error(1 as libc::c_int, 0 as libc::c_int, overflowmessage, str);
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(1 as libc::c_int, 0 as libc::c_int, overflowmessage, str);
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            (*result)
                .ts
                .tv_nsec = (origin.tv_nsec as libc::c_double - nanosec)
                as __syscall_slong_t;
            if (origin.tv_nsec as libc::c_double) < nanosec {
                (*result).ts.tv_nsec += nanosec_per_sec;
                (*result).ts.tv_sec -= 1 as libc::c_int as libc::c_long;
            }
            return 1 as libc::c_int != 0;
        } else {
            return 0 as libc::c_int != 0
        }
    } else {
        return 0 as libc::c_int != 0
    };
}
unsafe extern "C" fn parse_time(
    mut entry: *const parser_table,
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
) -> bool {
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    let mut tval: time_val = time_val {
        xval: XVAL_ATIME,
        kind: COMP_GT,
        ts: timespec { tv_sec: 0, tv_nsec: 0 },
    };
    let mut comp: comparison_type = COMP_GT;
    let mut timearg: *const libc::c_char = 0 as *const libc::c_char;
    let mut orig_timearg: *const libc::c_char = 0 as *const libc::c_char;
    let mut errmsg: *const libc::c_char = dcgettext(
        0 as *const libc::c_char,
        b"arithmetic overflow while converting %s days to a number of seconds\0"
            as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    let mut origin: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let saved_argc: libc::c_int = *arg_ptr;
    if !collect_arg(argv, arg_ptr, &mut timearg) {
        return 0 as libc::c_int != 0;
    }
    orig_timearg = timearg;
    origin = options.cur_day_start;
    if get_comp_type(&mut timearg, &mut comp) {
        if COMP_LT as libc::c_int as libc::c_uint == comp as libc::c_uint {
            let mut expected: uintmax_t = (origin.tv_sec
                + (86400 as libc::c_int - 1 as libc::c_int) as libc::c_long)
                as uintmax_t;
            origin.tv_sec += (86400 as libc::c_int - 1 as libc::c_int) as libc::c_long;
            if expected != origin.tv_sec as uintmax_t {
                if ::std::mem::size_of::<C2RustUnnamed_23>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"arithmetic overflow when trying to calculate the end of today\0"
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
                            b"arithmetic overflow when trying to calculate the end of today\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
    }
    timearg = orig_timearg;
    if !get_relative_timestamp(
        timearg,
        &mut tval,
        origin,
        86400 as libc::c_int as libc::c_double,
        errmsg,
    ) {
        *arg_ptr = saved_argc;
        return 0 as libc::c_int != 0;
    }
    our_pred = insert_primary(entry, orig_timearg);
    (*our_pred).args.reftime = tval;
    (*our_pred).est_success_rate = estimate_timestamp_success_rate(tval.ts.tv_sec);
    if options.debug_options & DebugExpressionTree as libc::c_int as libc::c_ulong != 0 {
        let mut t: time_t = 0;
        fprintf(
            stderr,
            b"inserting %s\n\0" as *const u8 as *const libc::c_char,
            (*our_pred).p_name,
        );
        fprintf(
            stderr,
            b"    type: %s    %s  \0" as *const u8 as *const libc::c_char,
            if tval.kind as libc::c_uint == COMP_GT as libc::c_int as libc::c_uint {
                b"gt\0" as *const u8 as *const libc::c_char
            } else if tval.kind as libc::c_uint == COMP_LT as libc::c_int as libc::c_uint
            {
                b"lt\0" as *const u8 as *const libc::c_char
            } else if tval.kind as libc::c_uint == COMP_EQ as libc::c_int as libc::c_uint
            {
                b"eq\0" as *const u8 as *const libc::c_char
            } else {
                b"?\0" as *const u8 as *const libc::c_char
            },
            if tval.kind as libc::c_uint == COMP_GT as libc::c_int as libc::c_uint {
                b" >\0" as *const u8 as *const libc::c_char
            } else if tval.kind as libc::c_uint == COMP_LT as libc::c_int as libc::c_uint
            {
                b" <\0" as *const u8 as *const libc::c_char
            } else if tval.kind as libc::c_uint == COMP_EQ as libc::c_int as libc::c_uint
            {
                b">=\0" as *const u8 as *const libc::c_char
            } else {
                b" ?\0" as *const u8 as *const libc::c_char
            },
        );
        t = (*our_pred).args.reftime.ts.tv_sec;
        fprintf(
            stderr,
            b"%lu %s\0" as *const u8 as *const libc::c_char,
            (*our_pred).args.reftime.ts.tv_sec as uintmax_t,
            ctime(&mut t),
        );
        if tval.kind as libc::c_uint == COMP_EQ as libc::c_int as libc::c_uint {
            t = (*our_pred).args.reftime.ts.tv_sec
                + 86400 as libc::c_int as libc::c_long;
            fprintf(
                stderr,
                b"                 <  %lu %s\0" as *const u8 as *const libc::c_char,
                t as uintmax_t,
                ctime(&mut t),
            );
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn get_comp_type(
    mut str: *mut *const libc::c_char,
    mut comp_type: *mut comparison_type,
) -> bool {
    match **str as libc::c_int {
        43 => {
            *comp_type = COMP_GT;
            *str = (*str).offset(1);
            *str;
        }
        45 => {
            *comp_type = COMP_LT;
            *str = (*str).offset(1);
            *str;
        }
        _ => {
            *comp_type = COMP_EQ;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn get_num(
    mut str: *const libc::c_char,
    mut num: *mut uintmax_t,
    mut comp_type: *mut comparison_type,
) -> bool {
    let mut pend: *mut libc::c_char = 0 as *mut libc::c_char;
    if str.is_null() {
        return 0 as libc::c_int != 0;
    }
    if !comp_type.is_null() {
        if !get_comp_type(&mut str, comp_type) {
            return 0 as libc::c_int != 0;
        }
    }
    return xstrtoumax(
        str,
        &mut pend,
        10 as libc::c_int,
        num,
        b"\0" as *const u8 as *const libc::c_char,
    ) as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn insert_num(
    mut argv: *mut *mut libc::c_char,
    mut arg_ptr: *mut libc::c_int,
    mut entry: *const parser_table,
) -> *mut predicate {
    let mut numstr: *const libc::c_char = 0 as *const libc::c_char;
    if collect_arg(argv, arg_ptr, &mut numstr) {
        let mut num: uintmax_t = 0;
        let mut c_type: comparison_type = COMP_GT;
        if get_num(numstr, &mut num, &mut c_type) {
            let mut our_pred: *mut predicate = insert_primary(entry, numstr);
            (*our_pred).args.numinfo.kind = c_type;
            (*our_pred).args.numinfo.l_val = num;
            if options.debug_options
                & DebugExpressionTree as libc::c_int as libc::c_ulong != 0
            {
                fprintf(
                    stderr,
                    b"inserting %s\n\0" as *const u8 as *const libc::c_char,
                    (*our_pred).p_name,
                );
                fprintf(
                    stderr,
                    b"    type: %s    %s  \0" as *const u8 as *const libc::c_char,
                    if c_type as libc::c_uint == COMP_GT as libc::c_int as libc::c_uint {
                        b"gt\0" as *const u8 as *const libc::c_char
                    } else if c_type as libc::c_uint
                        == COMP_LT as libc::c_int as libc::c_uint
                    {
                        b"lt\0" as *const u8 as *const libc::c_char
                    } else if c_type as libc::c_uint
                        == COMP_EQ as libc::c_int as libc::c_uint
                    {
                        b"eq\0" as *const u8 as *const libc::c_char
                    } else {
                        b"?\0" as *const u8 as *const libc::c_char
                    },
                    if c_type as libc::c_uint == COMP_GT as libc::c_int as libc::c_uint {
                        b" >\0" as *const u8 as *const libc::c_char
                    } else if c_type as libc::c_uint
                        == COMP_LT as libc::c_int as libc::c_uint
                    {
                        b" <\0" as *const u8 as *const libc::c_char
                    } else if c_type as libc::c_uint
                        == COMP_EQ as libc::c_int as libc::c_uint
                    {
                        b" =\0" as *const u8 as *const libc::c_char
                    } else {
                        b" ?\0" as *const u8 as *const libc::c_char
                    },
                );
                fprintf(
                    stderr,
                    b"%lu\n\0" as *const u8 as *const libc::c_char,
                    (*our_pred).args.numinfo.l_val,
                );
            }
            return our_pred;
        }
    }
    return 0 as *mut predicate;
}
unsafe extern "C" fn open_output_file(
    mut path: *const libc::c_char,
    mut p: *mut format_val,
) {
    (*p).segment = 0 as *mut segment;
    (*p).quote_opts = clone_quoting_options(0 as *mut quoting_options);
    if strcmp(path, b"/dev/stderr\0" as *const u8 as *const libc::c_char) == 0 {
        (*p).stream = stderr;
        (*p)
            .filename = dcgettext(
            0 as *const libc::c_char,
            b"standard error\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    } else if strcmp(path, b"/dev/stdout\0" as *const u8 as *const libc::c_char) == 0 {
        (*p).stream = stdout;
        (*p)
            .filename = dcgettext(
            0 as *const libc::c_char,
            b"standard output\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    } else {
        (*p).stream = sharefile_fopen(state.shared_files, path);
        (*p).filename = path;
        if ((*p).stream).is_null() {
            fatal_nontarget_file_error(*__errno_location(), path);
        }
    }
    (*p).dest_is_tty = stream_is_tty((*p).stream);
}
unsafe extern "C" fn open_stdout(mut p: *mut format_val) {
    open_output_file(b"/dev/stdout\0" as *const u8 as *const libc::c_char, p);
}
