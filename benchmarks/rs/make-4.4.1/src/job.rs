use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __spawn_action;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn __errno_location() -> *mut libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    static mut environ: *mut *mut libc::c_char;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn confstr(__name: libc::c_int, __buf: *mut libc::c_char, __len: size_t) -> size_t;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn getloadavg(__loadavg: *mut libc::c_double, __nelem: libc::c_int) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strsignal(__sig: libc::c_int) -> *mut libc::c_char;
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn message(prefix: libc::c_int, length: size_t, fmt: *const libc::c_char, _: ...);
    fn error(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...) -> !;
    fn die(_: libc::c_int) -> !;
    fn pfatal_with_name(_: *const libc::c_char) -> !;
    fn perror_with_name(_: *const libc::c_char, _: *const libc::c_char);
    fn make_toui(_: *const libc::c_char, _: *mut *const libc::c_char) -> libc::c_uint;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn show_goal_error();
    static mut stopchar_map: [libc::c_ushort; 0];
    static mut just_print_flag: libc::c_int;
    static mut run_silent: libc::c_int;
    static mut ignore_errors_flag: libc::c_int;
    static mut keep_going_flag: libc::c_int;
    static mut question_flag: libc::c_int;
    static mut touch_flag: libc::c_int;
    static mut warn_undefined_variables_flag: libc::c_int;
    static mut posix_pedantic: libc::c_int;
    static mut not_parallel: libc::c_int;
    static mut one_shell: libc::c_int;
    static mut output_sync: libc::c_int;
    static mut command_count: libc::c_ulong;
    static mut handling_fatal_signal: sig_atomic_t;
    static mut commands_started: libc::c_uint;
    static mut job_slots: libc::c_uint;
    static mut max_load_average: libc::c_double;
    fn start_remote_job_p(_: libc::c_int) -> libc::c_int;
    fn start_remote_job(
        _: *mut *mut libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut libc::c_int,
        _: *mut pid_t,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn remote_status(
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn output_init(out: *mut output);
    static mut output_context: *mut output;
    fn output_close(out: *mut output);
    fn output_start();
    fn output_dump(out: *mut output);
    static mut db_level: libc::c_int;
    fn lookup_file(name: *const libc::c_char) -> *mut file;
    fn set_command_state(file: *mut file, state: cmd_state);
    fn notice_finished_file(file: *mut file);
    fn delete_child_targets(child: *mut child);
    fn chop_commands(cmds: *mut commands);
    fn allocated_variable_expand_for_file(
        line: *const libc::c_char,
        file: *mut file,
    ) -> *mut libc::c_char;
    fn shell_completed(exit_code: libc::c_int, exit_sig: libc::c_int);
    fn lookup_variable_for_file(
        name: *const libc::c_char,
        length: size_t,
        file: *mut file,
    ) -> *mut variable;
    fn target_environment(
        file: *mut file,
        recursive: libc::c_int,
    ) -> *mut *mut libc::c_char;
    fn fd_noinherit(_: libc::c_int);
    fn jobserver_enabled() -> libc::c_uint;
    fn jobserver_release(is_fatal: libc::c_int);
    fn jobserver_signal();
    fn jobserver_pre_child(_: libc::c_int);
    fn jobserver_post_child(_: libc::c_int);
    fn jobserver_pre_acquire();
    fn jobserver_acquire(timeout: libc::c_int) -> libc::c_uint;
    fn get_bad_stdin() -> libc::c_int;
    fn shuffle_get_mode() -> *const libc::c_char;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn wait(__stat_loc: *mut libc::c_int) -> __pid_t;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn posix_spawn(
        __pid: *mut pid_t,
        __path: *const libc::c_char,
        __file_actions: *const posix_spawn_file_actions_t,
        __attrp: *const posix_spawnattr_t,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn posix_spawnattr_init(__attr: *mut posix_spawnattr_t) -> libc::c_int;
    fn posix_spawnattr_destroy(__attr: *mut posix_spawnattr_t) -> libc::c_int;
    fn posix_spawnattr_setsigmask(
        __attr: *mut posix_spawnattr_t,
        __sigmask: *const sigset_t,
    ) -> libc::c_int;
    fn posix_spawnattr_setflags(
        _attr: *mut posix_spawnattr_t,
        __flags: libc::c_short,
    ) -> libc::c_int;
    fn posix_spawn_file_actions_init(
        __file_actions: *mut posix_spawn_file_actions_t,
    ) -> libc::c_int;
    fn posix_spawn_file_actions_destroy(
        __file_actions: *mut posix_spawn_file_actions_t,
    ) -> libc::c_int;
    fn posix_spawn_file_actions_adddup2(
        __file_actions: *mut posix_spawn_file_actions_t,
        __fd: libc::c_int,
        __newfd: libc::c_int,
    ) -> libc::c_int;
    fn find_in_given_path(
        progname: *const libc::c_char,
        path: *const libc::c_char,
        directory: *const libc::c_char,
        optimize_for_exec: bool,
    ) -> *const libc::c_char;
    static mut fatal_signal_set: sigset_t;
    static mut shell_function_pid: pid_t;
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
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
pub type sig_atomic_t = __sig_atomic_t;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _CS_V7_ENV: C2RustUnnamed = 1149;
pub const _CS_V6_ENV: C2RustUnnamed = 1148;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS: C2RustUnnamed = 1147;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LIBS: C2RustUnnamed = 1146;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS: C2RustUnnamed = 1145;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS: C2RustUnnamed = 1144;
pub const _CS_POSIX_V7_LP64_OFF64_LINTFLAGS: C2RustUnnamed = 1143;
pub const _CS_POSIX_V7_LP64_OFF64_LIBS: C2RustUnnamed = 1142;
pub const _CS_POSIX_V7_LP64_OFF64_LDFLAGS: C2RustUnnamed = 1141;
pub const _CS_POSIX_V7_LP64_OFF64_CFLAGS: C2RustUnnamed = 1140;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS: C2RustUnnamed = 1139;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LIBS: C2RustUnnamed = 1138;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS: C2RustUnnamed = 1137;
pub const _CS_POSIX_V7_ILP32_OFFBIG_CFLAGS: C2RustUnnamed = 1136;
pub const _CS_POSIX_V7_ILP32_OFF32_LINTFLAGS: C2RustUnnamed = 1135;
pub const _CS_POSIX_V7_ILP32_OFF32_LIBS: C2RustUnnamed = 1134;
pub const _CS_POSIX_V7_ILP32_OFF32_LDFLAGS: C2RustUnnamed = 1133;
pub const _CS_POSIX_V7_ILP32_OFF32_CFLAGS: C2RustUnnamed = 1132;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS: C2RustUnnamed = 1131;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LIBS: C2RustUnnamed = 1130;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS: C2RustUnnamed = 1129;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS: C2RustUnnamed = 1128;
pub const _CS_POSIX_V6_LP64_OFF64_LINTFLAGS: C2RustUnnamed = 1127;
pub const _CS_POSIX_V6_LP64_OFF64_LIBS: C2RustUnnamed = 1126;
pub const _CS_POSIX_V6_LP64_OFF64_LDFLAGS: C2RustUnnamed = 1125;
pub const _CS_POSIX_V6_LP64_OFF64_CFLAGS: C2RustUnnamed = 1124;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS: C2RustUnnamed = 1123;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LIBS: C2RustUnnamed = 1122;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS: C2RustUnnamed = 1121;
pub const _CS_POSIX_V6_ILP32_OFFBIG_CFLAGS: C2RustUnnamed = 1120;
pub const _CS_POSIX_V6_ILP32_OFF32_LINTFLAGS: C2RustUnnamed = 1119;
pub const _CS_POSIX_V6_ILP32_OFF32_LIBS: C2RustUnnamed = 1118;
pub const _CS_POSIX_V6_ILP32_OFF32_LDFLAGS: C2RustUnnamed = 1117;
pub const _CS_POSIX_V6_ILP32_OFF32_CFLAGS: C2RustUnnamed = 1116;
pub const _CS_XBS5_LPBIG_OFFBIG_LINTFLAGS: C2RustUnnamed = 1115;
pub const _CS_XBS5_LPBIG_OFFBIG_LIBS: C2RustUnnamed = 1114;
pub const _CS_XBS5_LPBIG_OFFBIG_LDFLAGS: C2RustUnnamed = 1113;
pub const _CS_XBS5_LPBIG_OFFBIG_CFLAGS: C2RustUnnamed = 1112;
pub const _CS_XBS5_LP64_OFF64_LINTFLAGS: C2RustUnnamed = 1111;
pub const _CS_XBS5_LP64_OFF64_LIBS: C2RustUnnamed = 1110;
pub const _CS_XBS5_LP64_OFF64_LDFLAGS: C2RustUnnamed = 1109;
pub const _CS_XBS5_LP64_OFF64_CFLAGS: C2RustUnnamed = 1108;
pub const _CS_XBS5_ILP32_OFFBIG_LINTFLAGS: C2RustUnnamed = 1107;
pub const _CS_XBS5_ILP32_OFFBIG_LIBS: C2RustUnnamed = 1106;
pub const _CS_XBS5_ILP32_OFFBIG_LDFLAGS: C2RustUnnamed = 1105;
pub const _CS_XBS5_ILP32_OFFBIG_CFLAGS: C2RustUnnamed = 1104;
pub const _CS_XBS5_ILP32_OFF32_LINTFLAGS: C2RustUnnamed = 1103;
pub const _CS_XBS5_ILP32_OFF32_LIBS: C2RustUnnamed = 1102;
pub const _CS_XBS5_ILP32_OFF32_LDFLAGS: C2RustUnnamed = 1101;
pub const _CS_XBS5_ILP32_OFF32_CFLAGS: C2RustUnnamed = 1100;
pub const _CS_LFS64_LINTFLAGS: C2RustUnnamed = 1007;
pub const _CS_LFS64_LIBS: C2RustUnnamed = 1006;
pub const _CS_LFS64_LDFLAGS: C2RustUnnamed = 1005;
pub const _CS_LFS64_CFLAGS: C2RustUnnamed = 1004;
pub const _CS_LFS_LINTFLAGS: C2RustUnnamed = 1003;
pub const _CS_LFS_LIBS: C2RustUnnamed = 1002;
pub const _CS_LFS_LDFLAGS: C2RustUnnamed = 1001;
pub const _CS_LFS_CFLAGS: C2RustUnnamed = 1000;
pub const _CS_V7_WIDTH_RESTRICTED_ENVS: C2RustUnnamed = 5;
pub const _CS_V5_WIDTH_RESTRICTED_ENVS: C2RustUnnamed = 4;
pub const _CS_GNU_LIBPTHREAD_VERSION: C2RustUnnamed = 3;
pub const _CS_GNU_LIBC_VERSION: C2RustUnnamed = 2;
pub const _CS_V6_WIDTH_RESTRICTED_ENVS: C2RustUnnamed = 1;
pub const _CS_PATH: C2RustUnnamed = 0;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct file {
    pub name: *const libc::c_char,
    pub hname: *const libc::c_char,
    pub vpath: *const libc::c_char,
    pub deps: *mut dep,
    pub cmds: *mut commands,
    pub stem: *const libc::c_char,
    pub also_make: *mut dep,
    pub prev: *mut file,
    pub last: *mut file,
    pub renamed: *mut file,
    pub variables: *mut variable_set_list,
    pub pat_variables: *mut variable_set_list,
    pub parent: *mut file,
    pub double_colon: *mut file,
    pub last_mtime: uintmax_t,
    pub mtime_before_update: uintmax_t,
    pub considered: libc::c_uint,
    pub command_flags: libc::c_int,
    #[bitfield(name = "update_status", ty = "update_status", bits = "0..=1")]
    #[bitfield(name = "command_state", ty = "cmd_state", bits = "2..=3")]
    #[bitfield(name = "builtin", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "precious", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "loaded", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "unloaded", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "low_resolution_time", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "tried_implicit", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "updating", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "updated", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "is_target", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "cmd_target", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "phony", ty = "libc::c_uint", bits = "14..=14")]
    #[bitfield(name = "intermediate", ty = "libc::c_uint", bits = "15..=15")]
    #[bitfield(name = "is_explicit", ty = "libc::c_uint", bits = "16..=16")]
    #[bitfield(name = "secondary", ty = "libc::c_uint", bits = "17..=17")]
    #[bitfield(name = "notintermediate", ty = "libc::c_uint", bits = "18..=18")]
    #[bitfield(name = "dontcare", ty = "libc::c_uint", bits = "19..=19")]
    #[bitfield(name = "ignore_vpath", ty = "libc::c_uint", bits = "20..=20")]
    #[bitfield(name = "pat_searched", ty = "libc::c_uint", bits = "21..=21")]
    #[bitfield(name = "no_diag", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "was_shuffled", ty = "libc::c_uint", bits = "23..=23")]
    #[bitfield(name = "snapped", ty = "libc::c_uint", bits = "24..=24")]
    pub update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
pub type cmd_state = libc::c_uint;
pub const cs_finished: cmd_state = 3;
pub const cs_running: cmd_state = 2;
pub const cs_deps_running: cmd_state = 1;
pub const cs_not_started: cmd_state = 0;
pub type update_status = libc::c_uint;
pub const us_failed: update_status = 3;
pub const us_question: update_status = 2;
pub const us_none: update_status = 1;
pub const us_success: update_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_set_list {
    pub next: *mut variable_set_list,
    pub set: *mut variable_set,
    pub next_is_parent: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_set {
    pub table: hash_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub ht_vec: *mut *mut libc::c_void,
    pub ht_hash_1: hash_func_t,
    pub ht_hash_2: hash_func_t,
    pub ht_compare: hash_cmp_func_t,
    pub ht_size: libc::c_ulong,
    pub ht_capacity: libc::c_ulong,
    pub ht_fill: libc::c_ulong,
    pub ht_empty_slots: libc::c_ulong,
    pub ht_collisions: libc::c_ulong,
    pub ht_lookups: libc::c_ulong,
    pub ht_rehashes: libc::c_uint,
}
pub type hash_cmp_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type hash_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct dep {
    pub next: *mut dep,
    pub name: *const libc::c_char,
    pub file: *mut file,
    pub shuf: *mut dep,
    pub stem: *const libc::c_char,
    #[bitfield(name = "flags", ty = "libc::c_uint", bits = "0..=7")]
    #[bitfield(name = "changed", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "ignore_mtime", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "staticpattern", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "need_2nd_expansion", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "ignore_automatic_vars", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "is_explicit", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "wait_here", ty = "libc::c_uint", bits = "14..=14")]
    pub flags_changed_ignore_mtime_staticpattern_need_2nd_expansion_ignore_automatic_vars_is_explicit_wait_here: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct commands {
    pub fileinfo: floc,
    pub commands: *mut libc::c_char,
    pub command_lines: *mut *mut libc::c_char,
    pub lines_flags: *mut libc::c_uchar,
    pub ncommand_lines: libc::c_ushort,
    pub recipe_prefix: libc::c_char,
    #[bitfield(name = "any_recurse", ty = "libc::c_uint", bits = "0..=0")]
    pub any_recurse: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const libc::c_char,
    pub lineno: libc::c_ulong,
    pub offset: libc::c_ulong,
}
pub const o_invalid: variable_origin = 7;
pub const o_automatic: variable_origin = 6;
pub const o_override: variable_origin = 5;
pub const o_command: variable_origin = 4;
pub const o_env_override: variable_origin = 3;
pub const o_file: variable_origin = 2;
pub const o_env: variable_origin = 1;
pub const o_default: variable_origin = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct variable {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub fileinfo: floc,
    pub length: libc::c_uint,
    #[bitfield(name = "recursive", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "append", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "conditional", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "per_target", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "special", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "exportable", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "expanding", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "private_var", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "exp_count", ty = "libc::c_uint", bits = "8..=22")]
    #[bitfield(name = "flavor", ty = "variable_flavor", bits = "23..=25")]
    #[bitfield(name = "origin", ty = "variable_origin", bits = "26..=28")]
    #[bitfield(name = "export", ty = "variable_export", bits = "29..=30")]
    pub recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [u8; 4],
}
pub type variable_export = libc::c_uint;
pub const v_ifset: variable_export = 3;
pub const v_noexport: variable_export = 2;
pub const v_export: variable_export = 1;
pub const v_default: variable_export = 0;
pub type variable_origin = libc::c_uint;
pub type variable_flavor = libc::c_uint;
pub const f_append_value: variable_flavor = 7;
pub const f_shell: variable_flavor = 6;
pub const f_conditional: variable_flavor = 5;
pub const f_append: variable_flavor = 4;
pub const f_expand: variable_flavor = 3;
pub const f_recursive: variable_flavor = 2;
pub const f_simple: variable_flavor = 1;
pub const f_bogus: variable_flavor = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct output {
    pub out: libc::c_int,
    pub err: libc::c_int,
    #[bitfield(name = "syncout", ty = "libc::c_uint", bits = "0..=0")]
    pub syncout: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct childbase {
    pub cmd_name: *mut libc::c_char,
    pub environment: *mut *mut libc::c_char,
    pub output: output,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct child {
    pub cmd_name: *mut libc::c_char,
    pub environment: *mut *mut libc::c_char,
    pub output: output,
    pub next: *mut child,
    pub file: *mut file,
    pub sh_batch_file: *mut libc::c_char,
    pub command_lines: *mut *mut libc::c_char,
    pub command_ptr: *mut libc::c_char,
    pub command_line: libc::c_uint,
    pub pid: pid_t,
    #[bitfield(name = "remote", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "noerror", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "good_stdin", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "deleted", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "recursive", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "jobslot", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "dontcare", ty = "libc::c_uint", bits = "6..=6")]
    pub remote_noerror_good_stdin_deleted_recursive_jobslot_dontcare: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_spawnattr_t {
    pub __flags: libc::c_short,
    pub __pgrp: pid_t,
    pub __sd: sigset_t,
    pub __ss: sigset_t,
    pub __sp: sched_param,
    pub __policy: libc::c_int,
    pub __pad: [libc::c_int; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_spawn_file_actions_t {
    pub __allocated: libc::c_int,
    pub __used: libc::c_int,
    pub __actions: *mut __spawn_action,
    pub __pad: [libc::c_int; 16],
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
pub static mut default_shell: *const libc::c_char = b"/bin/sh\0" as *const u8
    as *const libc::c_char;
pub static mut batch_mode_shell: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn pid2str(mut pid: pid_t) -> *const libc::c_char {
    static mut pidstring: [libc::c_char; 100] = [0; 100];
    sprintf(
        pidstring.as_mut_ptr(),
        b"%lu\0" as *const u8 as *const libc::c_char,
        pid as libc::c_ulong,
    );
    return pidstring.as_mut_ptr();
}
pub static mut children: *mut child = 0 as *const child as *mut child;
pub static mut job_slots_used: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut good_stdin_used: libc::c_int = 0 as libc::c_int;
static mut waiting_jobs: *mut child = 0 as *const child as *mut child;
pub static mut unixy_shell: libc::c_int = 1 as libc::c_int;
pub static mut job_counter: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
pub static mut jobserver_tokens: libc::c_uint = 0 as libc::c_int as libc::c_uint;
pub unsafe extern "C" fn is_bourne_compatible_shell(
    mut path: *const libc::c_char,
) -> libc::c_int {
    static mut unix_shells: [*const libc::c_char; 8] = [
        b"sh\0" as *const u8 as *const libc::c_char,
        b"bash\0" as *const u8 as *const libc::c_char,
        b"dash\0" as *const u8 as *const libc::c_char,
        b"ksh\0" as *const u8 as *const libc::c_char,
        b"rksh\0" as *const u8 as *const libc::c_char,
        b"zsh\0" as *const u8 as *const libc::c_char,
        b"ash\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut s: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut cp: *const libc::c_char = path.offset(strlen(path) as isize);
    while cp > path
        && !(*stopchar_map
            .as_mut_ptr()
            .offset(*cp.offset(-(1 as libc::c_int) as isize) as libc::c_uchar as isize)
            as libc::c_int & 0x8000 as libc::c_int != 0 as libc::c_int)
    {
        cp = cp.offset(-1);
        cp;
    }
    s = unix_shells.as_mut_ptr();
    while !(*s).is_null() {
        if strcmp(cp, *s) == 0 as libc::c_int {
            return 1 as libc::c_int;
        }
        s = s.offset(1);
        s;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn block_sigs() {
    sigprocmask(0 as libc::c_int, &mut fatal_signal_set, 0 as *mut sigset_t);
}
unsafe extern "C" fn unblock_sigs() {
    sigprocmask(1 as libc::c_int, &mut fatal_signal_set, 0 as *mut sigset_t);
}
pub unsafe extern "C" fn unblock_all_sigs() {
    let mut empty: sigset_t = sigset_t { __val: [0; 16] };
    sigemptyset(&mut empty);
    sigprocmask(2 as libc::c_int, &mut empty, 0 as *mut sigset_t);
}
unsafe extern "C" fn child_error(
    mut child: *mut child,
    mut exit_code: libc::c_int,
    mut exit_sig: libc::c_int,
    mut coredump: libc::c_int,
    mut ignored: libc::c_int,
) {
    let mut pre: *const libc::c_char = b"*** \0" as *const u8 as *const libc::c_char;
    let mut post: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut dump: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut f: *const file = (*child).file;
    let mut flocp: *const floc = &mut (*(*f).cmds).fileinfo;
    let mut nm: *const libc::c_char = 0 as *const libc::c_char;
    let mut smode: *const libc::c_char = 0 as *const libc::c_char;
    let mut l: size_t = 0;
    if ignored != 0 && run_silent != 0 {
        return;
    }
    if exit_sig != 0 && coredump != 0 {
        dump = dcgettext(
            0 as *const libc::c_char,
            b" (core dumped)\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    }
    if ignored != 0 {
        pre = b"\0" as *const u8 as *const libc::c_char;
        post = dcgettext(
            0 as *const libc::c_char,
            b" (ignored)\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    }
    if ((*flocp).filenm).is_null() {
        nm = dcgettext(
            0 as *const libc::c_char,
            b"<builtin>\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    } else {
        let mut fresh0 = ::std::vec::from_elem(
            0,
            (strlen((*flocp).filenm))
                .wrapping_add(6 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (53 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<uintmax_t>() as libc::c_ulong,
                        )
                        .wrapping_div(22 as libc::c_int as libc::c_ulong)
                        .wrapping_add(3 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
        );
        let mut a: *mut libc::c_char = fresh0.leak().as_mut_ptr() as *mut libc::c_char;
        sprintf(
            a,
            b"%s:%lu\0" as *const u8 as *const libc::c_char,
            (*flocp).filenm,
            ((*flocp).lineno).wrapping_add((*flocp).offset),
        );
        nm = a;
    }
    l = (strlen(pre))
        .wrapping_add(strlen(nm))
        .wrapping_add(strlen((*f).name))
        .wrapping_add(strlen(post));
    smode = shuffle_get_mode();
    if !smode.is_null() {
        let mut fresh1 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen(smode))
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
        );
        let mut a_0: *mut libc::c_char = fresh1.leak().as_mut_ptr() as *mut libc::c_char;
        sprintf(a_0, b" shuffle=%s\0" as *const u8 as *const libc::c_char, smode);
        smode = a_0;
        l = (l as libc::c_ulong).wrapping_add(strlen(smode)) as size_t as size_t;
    }
    output_context = if ((*child).output).syncout() as libc::c_int != 0 {
        &mut (*child).output
    } else {
        0 as *mut output
    };
    show_goal_error();
    if exit_sig == 0 as libc::c_int {
        error(
            0 as *mut floc,
            l
                .wrapping_add(
                    (53 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<uintmax_t>() as libc::c_ulong,
                        )
                        .wrapping_div(22 as libc::c_int as libc::c_ulong)
                        .wrapping_add(3 as libc::c_int as libc::c_ulong),
                ),
            dcgettext(
                0 as *const libc::c_char,
                b"%s[%s: %s] Error %d%s%s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            pre,
            nm,
            (*f).name,
            exit_code,
            post,
            if !smode.is_null() {
                smode
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    } else {
        let mut s: *const libc::c_char = strsignal(exit_sig);
        error(
            0 as *mut floc,
            l.wrapping_add(strlen(s)).wrapping_add(strlen(dump)),
            b"%s[%s: %s] %s%s%s%s\0" as *const u8 as *const libc::c_char,
            pre,
            nm,
            (*f).name,
            s,
            dump,
            post,
            if !smode.is_null() {
                smode
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    }
    output_context = 0 as *mut output;
}
static mut dead_children: libc::c_uint = 0 as libc::c_int as libc::c_uint;
pub unsafe extern "C" fn child_handler(mut sig: libc::c_int) {
    dead_children = dead_children.wrapping_add(1);
    dead_children;
    jobserver_signal();
}
pub unsafe extern "C" fn reap_children(mut block: libc::c_int, mut err: libc::c_int) {
    let mut status: libc::c_int = 0;
    let mut reap_more: libc::c_int = 1 as libc::c_int;
    let mut current_block_143: u64;
    while (!children.is_null() || shell_function_pid != 0 as libc::c_int)
        && (block != 0 || reap_more != 0)
    {
        let mut remote: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut pid: pid_t = 0;
        let mut exit_code: libc::c_int = 0;
        let mut exit_sig: libc::c_int = 0;
        let mut coredump: libc::c_int = 0;
        let mut lastc: *mut child = 0 as *mut child;
        let mut c: *mut child = 0 as *mut child;
        let mut child_failed: libc::c_int = 0;
        let mut any_remote: libc::c_int = 0;
        let mut any_local: libc::c_int = 0;
        let mut dontcare: libc::c_int = 0;
        if err != 0 && block != 0 {
            static mut printed: libc::c_int = 0 as libc::c_int;
            fflush(stdout);
            if printed == 0 {
                error(
                    0 as *mut floc,
                    0 as libc::c_int as size_t,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"*** Waiting for unfinished jobs....\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            printed = 1 as libc::c_int;
        }
        if dead_children > 0 as libc::c_int as libc::c_uint {
            dead_children = dead_children.wrapping_sub(1);
            dead_children;
        }
        any_remote = 0 as libc::c_int;
        any_local = (shell_function_pid != 0 as libc::c_int) as libc::c_int;
        lastc = 0 as *mut child;
        c = children;
        loop {
            if c.is_null() {
                current_block_143 = 17478428563724192186;
                break;
            }
            any_remote |= (*c).remote() as libc::c_int;
            any_local |= ((*c).remote() == 0) as libc::c_int;
            if (*c).pid < 0 as libc::c_int {
                exit_sig = 0 as libc::c_int;
                coredump = 0 as libc::c_int;
                exit_code = 127 as libc::c_int;
                current_block_143 = 3707592317859690367;
                break;
            } else {
                if 0x4 as libc::c_int & db_level != 0 {
                    printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Live child %p (%s) PID %s %s\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        c,
                        (*(*c).file).name,
                        pid2str((*c).pid),
                        if (*c).remote() as libc::c_int != 0 {
                            dcgettext(
                                0 as *const libc::c_char,
                                b" (remote)\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ) as *const libc::c_char
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
                        },
                    );
                    fflush(stdout);
                }
                lastc = c;
                c = (*c).next;
            }
        }
        match current_block_143 {
            17478428563724192186 => {
                if any_remote != 0 {
                    pid = remote_status(
                        &mut exit_code,
                        &mut exit_sig,
                        &mut coredump,
                        0 as libc::c_int,
                    );
                } else {
                    pid = 0 as libc::c_int;
                }
                if pid > 0 as libc::c_int {
                    remote = 1 as libc::c_int as libc::c_uint;
                } else if pid < 0 as libc::c_int {
                    pfatal_with_name(
                        b"remote_status\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    if any_local != 0 {
                        if block == 0 {
                            pid = waitpid(
                                -(1 as libc::c_int),
                                &mut status,
                                1 as libc::c_int,
                            );
                        } else {
                            loop {
                                pid = wait(&mut status);
                                if !(pid == -(1 as libc::c_int)
                                    && *__errno_location() == 4 as libc::c_int)
                                {
                                    break;
                                }
                            }
                        }
                    } else {
                        pid = 0 as libc::c_int;
                    }
                    if pid < 0 as libc::c_int {
                        pfatal_with_name(b"wait\0" as *const u8 as *const libc::c_char);
                    } else if pid > 0 as libc::c_int {
                        exit_code = (status & 0xff00 as libc::c_int) >> 8 as libc::c_int;
                        exit_sig = if ((status & 0x7f as libc::c_int) + 1 as libc::c_int)
                            as libc::c_schar as libc::c_int >> 1 as libc::c_int
                            > 0 as libc::c_int
                        {
                            status & 0x7f as libc::c_int
                        } else {
                            0 as libc::c_int
                        };
                        coredump = status & 0x80 as libc::c_int;
                    } else {
                        reap_more = 0 as libc::c_int;
                        if block == 0 || any_remote == 0 {
                            break;
                        }
                        pid = remote_status(
                            &mut exit_code,
                            &mut exit_sig,
                            &mut coredump,
                            1 as libc::c_int,
                        );
                        if pid < 0 as libc::c_int {
                            pfatal_with_name(
                                b"remote_status\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        if pid == 0 as libc::c_int {
                            break;
                        }
                        remote = 1 as libc::c_int as libc::c_uint;
                    }
                }
                command_count = command_count.wrapping_add(1);
                command_count;
                if remote == 0 && pid == shell_function_pid {
                    shell_completed(exit_code, exit_sig);
                    break;
                } else {
                    lastc = 0 as *mut child;
                    c = children;
                    while !c.is_null() {
                        if (*c).pid == pid && (*c).remote() == remote {
                            break;
                        }
                        lastc = c;
                        c = (*c).next;
                    }
                    if c.is_null() {
                        continue;
                    } else {
                        if 0x4 as libc::c_int & db_level != 0 {
                            printf(
                                if exit_sig == 0 as libc::c_int
                                    && exit_code == 0 as libc::c_int
                                {
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"Reaping winning child %p PID %s %s\n\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    )
                                } else {
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"Reaping losing child %p PID %s %s\n\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    )
                                },
                                c,
                                pid2str((*c).pid),
                                if (*c).remote() as libc::c_int != 0 {
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b" (remote)\0" as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ) as *const libc::c_char
                                } else {
                                    b"\0" as *const u8 as *const libc::c_char
                                },
                            );
                            fflush(stdout);
                        }
                        if job_counter != 0 {
                            job_counter = job_counter.wrapping_sub(1);
                            job_counter;
                        }
                    }
                }
            }
            _ => {}
        }
        if exit_sig == 0 as libc::c_int && exit_code == 127 as libc::c_int
            && !((*c).cmd_name).is_null()
        {
            let mut e: *const libc::c_char = 0 as *const libc::c_char;
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
            let mut r: libc::c_int = 0;
            loop {
                r = stat((*c).cmd_name, &mut st);
                if !(r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
            }
            if r < 0 as libc::c_int {
                e = strerror(*__errno_location());
            } else if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint
                || st.st_mode & 0o100 as libc::c_int as libc::c_uint == 0
            {
                e = strerror(13 as libc::c_int);
            } else if st.st_size == 0 as libc::c_int as libc::c_long {
                e = strerror(8 as libc::c_int);
            }
            if !e.is_null() {
                error(
                    0 as *mut floc,
                    (strlen((*c).cmd_name)).wrapping_add(strlen(e)),
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    (*c).cmd_name,
                    e,
                );
            }
        }
        if exit_sig == 0 as libc::c_int && exit_code == 0 as libc::c_int {
            child_failed = 0 as libc::c_int;
        } else if exit_sig == 0 as libc::c_int && exit_code == 1 as libc::c_int
            && question_flag != 0 && (*c).recursive() as libc::c_int != 0
        {
            child_failed = 1 as libc::c_int;
        } else {
            child_failed = 2 as libc::c_int;
        }
        if !((*c).sh_batch_file).is_null() {
            let mut rm_status: libc::c_int = 0;
            if 0x4 as libc::c_int & db_level != 0 {
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Cleaning up temp batch file %s\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*c).sh_batch_file,
                );
                fflush(stdout);
            }
            *__errno_location() = 0 as libc::c_int;
            rm_status = remove((*c).sh_batch_file);
            if rm_status != 0 {
                if 0x4 as libc::c_int & db_level != 0 {
                    printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Cleaning up temp batch file %s failed (%d)\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*c).sh_batch_file,
                        *__errno_location(),
                    );
                    fflush(stdout);
                }
            }
            free((*c).sh_batch_file as *mut libc::c_void);
            (*c).sh_batch_file = 0 as *mut libc::c_char;
        }
        if (*c).good_stdin() != 0 {
            good_stdin_used = 0 as libc::c_int;
        }
        dontcare = (*c).dontcare() as libc::c_int;
        if child_failed != 0 && (*c).noerror() == 0 && ignore_errors_flag == 0 {
            static mut delete_on_error: libc::c_int = -(1 as libc::c_int);
            if dontcare == 0 && child_failed == 2 as libc::c_int {
                child_error(c, exit_code, exit_sig, coredump, 0 as libc::c_int);
            }
            (*(*c).file)
                .set_update_status(
                    (if child_failed == 2 as libc::c_int {
                        us_failed as libc::c_int
                    } else {
                        us_question as libc::c_int
                    }) as update_status,
                );
            if delete_on_error == -(1 as libc::c_int) {
                let mut f: *mut file = lookup_file(
                    b".DELETE_ON_ERROR\0" as *const u8 as *const libc::c_char,
                );
                delete_on_error = (!f.is_null() && (*f).is_target() as libc::c_int != 0)
                    as libc::c_int;
            }
            if exit_sig != 0 as libc::c_int || delete_on_error != 0 {
                delete_child_targets(c);
            }
        } else {
            if child_failed != 0 {
                child_error(c, exit_code, exit_sig, coredump, 1 as libc::c_int);
                child_failed = 0 as libc::c_int;
            }
            if job_next_command(c) != 0 {
                if handling_fatal_signal != 0 {
                    (*(*c).file).set_update_status(us_failed);
                } else {
                    if output_sync == 1 as libc::c_int {
                        output_dump(&mut (*c).output);
                    }
                    (*c)
                        .set_remote(
                            start_remote_job_p(0 as libc::c_int) as libc::c_uint,
                        );
                    start_job_command(c);
                    unblock_sigs();
                    if (*(*c).file).command_state() as libc::c_int
                        == cs_running as libc::c_int
                    {
                        continue;
                    }
                }
                if (*(*c).file).update_status() as libc::c_int
                    != us_success as libc::c_int
                {
                    delete_child_targets(c);
                }
            } else {
                (*(*c).file).set_update_status(us_success);
            }
        }
        output_dump(&mut (*c).output);
        if handling_fatal_signal == 0 {
            notice_finished_file((*c).file);
        }
        block_sigs();
        if (*c).pid > 0 as libc::c_int {
            if 0x4 as libc::c_int & db_level != 0 {
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Removing child %p PID %s%s from chain.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    c,
                    pid2str((*c).pid),
                    if (*c).remote() as libc::c_int != 0 {
                        dcgettext(
                            0 as *const libc::c_char,
                            b" (remote)\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ) as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                );
                fflush(stdout);
            }
        }
        if job_slots_used > 0 as libc::c_int as libc::c_uint {
            job_slots_used = job_slots_used.wrapping_sub((*c).jobslot());
        }
        if lastc.is_null() {
            children = (*c).next;
        } else {
            (*lastc).next = (*c).next;
        }
        free_child(c);
        unblock_sigs();
        if err == 0 && child_failed != 0 && dontcare == 0 && keep_going_flag == 0
            && handling_fatal_signal == 0
        {
            die(child_failed);
        }
        block = 0 as libc::c_int;
    }
}
pub unsafe extern "C" fn free_childbase(mut child: *mut childbase) {
    if !((*child).environment).is_null() {
        let mut ep: *mut *mut libc::c_char = (*child).environment;
        while !(*ep).is_null() {
            let fresh2 = ep;
            ep = ep.offset(1);
            free(*fresh2 as *mut libc::c_void);
        }
        free((*child).environment as *mut libc::c_void);
    }
    free((*child).cmd_name as *mut libc::c_void);
}
unsafe extern "C" fn free_child(mut child: *mut child) {
    output_close(&mut (*child).output);
    if jobserver_tokens == 0 {
        fatal(
            0 as *mut floc,
            (53 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                .wrapping_div(22 as libc::c_int as libc::c_ulong)
                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen((*(*child).file).name)),
            b"INTERNAL: Freeing child %p (%s) but no tokens left\0" as *const u8
                as *const libc::c_char,
            child,
            (*(*child).file).name,
        );
    }
    if jobserver_enabled() != 0 && jobserver_tokens > 1 as libc::c_int as libc::c_uint {
        jobserver_release(1 as libc::c_int);
        if 0x4 as libc::c_int & db_level != 0 {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Released token for child %p (%s).\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                child,
                (*(*child).file).name,
            );
            fflush(stdout);
        }
    }
    jobserver_tokens = jobserver_tokens.wrapping_sub(1);
    jobserver_tokens;
    if handling_fatal_signal != 0 {
        return;
    }
    if !((*child).command_lines).is_null() {
        let mut i: libc::c_uint = 0;
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*(*(*child).file).cmds).ncommand_lines as libc::c_uint {
            free(*((*child).command_lines).offset(i as isize) as *mut libc::c_void);
            i = i.wrapping_add(1);
            i;
        }
        free((*child).command_lines as *mut libc::c_void);
    }
    free_childbase(child as *mut childbase);
    free(child as *mut libc::c_void);
}
unsafe extern "C" fn start_job_command(mut child: *mut child) {
    let mut flags: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if !((*child).command_ptr).is_null() {
        flags = (*(*child).file).command_flags
            | *((*(*(*child).file).cmds).lines_flags)
                .offset(
                    ((*child).command_line)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                ) as libc::c_int;
        p = (*child).command_ptr;
        (*child)
            .set_noerror(
                (flags & 4 as libc::c_int != 0 as libc::c_int) as libc::c_int
                    as libc::c_uint,
            );
        while *p as libc::c_int != '\0' as i32 {
            if *p as libc::c_int == '@' as i32 {
                flags |= 2 as libc::c_int;
            } else if *p as libc::c_int == '+' as i32 {
                flags |= 1 as libc::c_int;
            } else if *p as libc::c_int == '-' as i32 {
                (*child).set_noerror(1 as libc::c_int as libc::c_uint);
            } else if !(*stopchar_map.as_mut_ptr().offset(*p as libc::c_uchar as isize)
                as libc::c_int & 0x2 as libc::c_int != 0 as libc::c_int)
            {
                break;
            }
            p = p.offset(1);
            p;
        }
        (*child)
            .set_recursive(
                (flags & 1 as libc::c_int != 0 as libc::c_int) as libc::c_int
                    as libc::c_uint,
            );
        let ref mut fresh3 = *((*(*(*child).file).cmds).lines_flags)
            .offset(
                ((*child).command_line).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as isize,
            );
        *fresh3 = (*fresh3 as libc::c_int | flags & 1 as libc::c_int) as libc::c_uchar;
        let mut prefix: libc::c_char = (*(*(*child).file).cmds).recipe_prefix;
        let mut p1: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
        p2 = p;
        p1 = p2;
        while *p1 as libc::c_int != '\0' as i32 {
            let fresh4 = p2;
            p2 = p2.offset(1);
            *fresh4 = *p1;
            if *p1.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32
                && *p1.offset(1 as libc::c_int as isize) as libc::c_int
                    == prefix as libc::c_int
            {
                p1 = p1.offset(1);
                p1;
            }
            p1 = p1.offset(1);
            p1;
        }
        *p2 = *p1;
        let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
        argv = construct_command_argv(
            p,
            &mut end,
            (*child).file,
            *((*(*(*child).file).cmds).lines_flags)
                .offset(
                    ((*child).command_line)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                ) as libc::c_int,
            &mut (*child).sh_batch_file,
        );
        if end.is_null() {
            (*child).command_ptr = 0 as *mut libc::c_char;
        } else {
            let fresh5 = end;
            end = end.offset(1);
            *fresh5 = '\0' as i32 as libc::c_char;
            (*child).command_ptr = end;
        }
        if !argv.is_null() && question_flag != 0
            && !(flags & 1 as libc::c_int != 0 as libc::c_int)
        {
            if !argv.is_null() {
                free(*argv.offset(0 as libc::c_int as isize) as *mut libc::c_void);
                free(argv as *mut libc::c_void);
            }
            (*(*child).file).set_update_status(us_question);
            notice_finished_file((*child).file);
            return;
        }
        if touch_flag != 0 && !(flags & 1 as libc::c_int != 0 as libc::c_int) {
            if !argv.is_null() {
                free(*argv.offset(0 as libc::c_int as isize) as *mut libc::c_void);
                free(argv as *mut libc::c_void);
            }
            argv = 0 as *mut *mut libc::c_char;
        }
        if !argv.is_null() {
            ((*child).output)
                .set_syncout(
                    (output_sync != 0
                        && (output_sync == 3 as libc::c_int
                            || !(flags & 1 as libc::c_int != 0 as libc::c_int)))
                        as libc::c_int as libc::c_uint,
                );
            output_context = if ((*child).output).syncout() as libc::c_int != 0 {
                &mut (*child).output
            } else {
                0 as *mut output
            };
            if ((*child).output).syncout() == 0 {
                output_dump(&mut (*child).output);
            }
            if just_print_flag != 0 || 0x10 as libc::c_int & db_level != 0
                || !(flags & 2 as libc::c_int != 0 as libc::c_int) && run_silent == 0
            {
                message(
                    0 as libc::c_int,
                    strlen(p),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    p,
                );
            }
            commands_started = commands_started.wrapping_add(1);
            commands_started;
            if !(*argv.offset(0 as libc::c_int as isize)).is_null()
                && is_bourne_compatible_shell(*argv.offset(0 as libc::c_int as isize))
                    != 0
                && (!(*argv.offset(1 as libc::c_int as isize)).is_null()
                    && *(*argv.offset(1 as libc::c_int as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
                    && (*(*argv.offset(1 as libc::c_int as isize))
                        .offset(1 as libc::c_int as isize) as libc::c_int == 'c' as i32
                        && *(*argv.offset(1 as libc::c_int as isize))
                            .offset(2 as libc::c_int as isize) as libc::c_int
                            == '\0' as i32
                        || *(*argv.offset(1 as libc::c_int as isize))
                            .offset(1 as libc::c_int as isize) as libc::c_int
                            == 'e' as i32
                            && *(*argv.offset(1 as libc::c_int as isize))
                                .offset(2 as libc::c_int as isize) as libc::c_int
                                == 'c' as i32
                            && *(*argv.offset(1 as libc::c_int as isize))
                                .offset(3 as libc::c_int as isize) as libc::c_int
                                == '\0' as i32))
                && (!(*argv.offset(2 as libc::c_int as isize)).is_null()
                    && *(*argv.offset(2 as libc::c_int as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32
                    && *(*argv.offset(2 as libc::c_int as isize))
                        .offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32)
                && (*argv.offset(3 as libc::c_int as isize)).is_null()
            {
                if !argv.is_null() {
                    free(*argv.offset(0 as libc::c_int as isize) as *mut libc::c_void);
                    free(argv as *mut libc::c_void);
                }
            } else if just_print_flag != 0
                && !(flags & 1 as libc::c_int != 0 as libc::c_int)
            {
                if !argv.is_null() {
                    free(*argv.offset(0 as libc::c_int as isize) as *mut libc::c_void);
                    free(argv as *mut libc::c_void);
                }
            } else {
                output_start();
                fflush(stdout);
                fflush(stderr);
                (*child)
                    .set_good_stdin(
                        (good_stdin_used == 0) as libc::c_int as libc::c_uint,
                    );
                if (*child).good_stdin() != 0 {
                    good_stdin_used = 1 as libc::c_int;
                }
                (*child).set_deleted(0 as libc::c_int as libc::c_uint);
                if ((*child).environment).is_null() {
                    (*child)
                        .environment = target_environment(
                        (*child).file,
                        (*(*(*child).file).cmds).any_recurse() as libc::c_int,
                    );
                }
                let mut current_block_97: u64;
                if (*child).remote() != 0 {
                    let mut is_remote: libc::c_int = 0;
                    let mut used_stdin: libc::c_int = 0;
                    let mut id: pid_t = 0;
                    if start_remote_job(
                        argv,
                        (*child).environment,
                        if (*child).good_stdin() as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            get_bad_stdin()
                        },
                        &mut is_remote,
                        &mut id,
                        &mut used_stdin,
                    ) != 0
                    {
                        current_block_97 = 16799877837230409540;
                    } else {
                        if (*child).good_stdin() as libc::c_int != 0 && used_stdin == 0 {
                            (*child).set_good_stdin(0 as libc::c_int as libc::c_uint);
                            good_stdin_used = 0 as libc::c_int;
                        }
                        (*child).set_remote(is_remote as libc::c_uint);
                        (*child).pid = id;
                        current_block_97 = 10261677128829721533;
                    }
                } else {
                    current_block_97 = 16799877837230409540;
                }
                match current_block_97 {
                    16799877837230409540 => {
                        block_sigs();
                        (*child).set_remote(0 as libc::c_int as libc::c_uint);
                        jobserver_pre_child(
                            (flags & 1 as libc::c_int != 0 as libc::c_int) as libc::c_int,
                        );
                        (*child)
                            .pid = child_execute_job(
                            child as *mut childbase,
                            (*child).good_stdin() as libc::c_int,
                            argv,
                        );
                        jobserver_post_child(
                            (flags & 1 as libc::c_int != 0 as libc::c_int) as libc::c_int,
                        );
                    }
                    _ => {}
                }
                if (*child).pid >= 0 as libc::c_int {
                    job_counter = job_counter.wrapping_add(1);
                    job_counter;
                }
                set_command_state((*child).file, cs_running);
                if !argv.is_null() {
                    free(*argv.offset(0 as libc::c_int as isize) as *mut libc::c_void);
                    free(argv as *mut libc::c_void);
                }
                output_context = 0 as *mut output;
                return;
            }
        }
    }
    if job_next_command(child) != 0 {
        start_job_command(child);
    } else {
        set_command_state((*child).file, cs_running);
        (*(*child).file).set_update_status(us_success);
        notice_finished_file((*child).file);
    }
    output_context = 0 as *mut output;
}
unsafe extern "C" fn start_waiting_job(mut c: *mut child) -> libc::c_int {
    let mut f: *mut file = (*c).file;
    (*c).set_remote(start_remote_job_p(1 as libc::c_int) as libc::c_uint);
    if (*c).remote() == 0
        && (job_slots_used > 0 as libc::c_int as libc::c_uint && load_too_high() != 0)
    {
        set_command_state(f, cs_running);
        (*c).next = waiting_jobs;
        waiting_jobs = c;
        return 0 as libc::c_int;
    }
    start_job_command(c);
    let mut current_block_25: u64;
    match (*f).command_state() as libc::c_int {
        2 => {
            (*c).next = children;
            if (*c).pid > 0 as libc::c_int {
                if 0x4 as libc::c_int & db_level != 0 {
                    printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Putting child %p (%s) PID %s%s on the chain.\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        c,
                        (*(*c).file).name,
                        pid2str((*c).pid),
                        if (*c).remote() as libc::c_int != 0 {
                            dcgettext(
                                0 as *const libc::c_char,
                                b" (remote)\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ) as *const libc::c_char
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
                        },
                    );
                    fflush(stdout);
                }
                job_slots_used = job_slots_used.wrapping_add(1);
                job_slots_used;
                (*c).set_jobslot(1 as libc::c_int as libc::c_uint);
            }
            children = c;
            unblock_sigs();
            current_block_25 = 15089075282327824602;
        }
        0 => {
            (*f).set_update_status(us_success);
            current_block_25 = 14129072355713444981;
        }
        3 => {
            current_block_25 = 14129072355713444981;
        }
        _ => {
            current_block_25 = 15089075282327824602;
        }
    }
    match current_block_25 {
        14129072355713444981 => {
            notice_finished_file(f);
            free_child(c);
        }
        _ => {}
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn new_job(mut file: *mut file) {
    let mut cmds: *mut commands = (*file).cmds;
    let mut c: *mut child = 0 as *mut child;
    let mut lines: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_uint = 0;
    start_waiting_jobs();
    reap_children(0 as libc::c_int, 0 as libc::c_int);
    chop_commands(cmds);
    c = xcalloc(::std::mem::size_of::<child>() as libc::c_ulong) as *mut child;
    output_init(&mut (*c).output);
    (*c).file = file;
    (*c).sh_batch_file = 0 as *mut libc::c_char;
    (*c).set_dontcare((*file).dontcare());
    output_context = if ((*c).output).syncout() as libc::c_int != 0 {
        &mut (*c).output
    } else {
        0 as *mut output
    };
    lines = xmalloc(
        ((*cmds).ncommand_lines as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*cmds).ncommand_lines as libc::c_uint {
        let mut in_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ref_0: *mut libc::c_char = 0 as *mut libc::c_char;
        out = *((*cmds).command_lines).offset(i as isize);
        in_0 = out;
        loop {
            ref_0 = strchr(in_0, '$' as i32);
            if ref_0.is_null() {
                break;
            }
            ref_0 = ref_0.offset(1);
            ref_0;
            if out != in_0 {
                memmove(
                    out as *mut libc::c_void,
                    in_0 as *const libc::c_void,
                    ref_0.offset_from(in_0) as libc::c_long as libc::c_ulong,
                );
            }
            out = out.offset(ref_0.offset_from(in_0) as libc::c_long as isize);
            in_0 = ref_0;
            if *ref_0 as libc::c_int == '(' as i32 || *ref_0 as libc::c_int == '{' as i32
            {
                let mut openparen: libc::c_char = *ref_0;
                let mut closeparen: libc::c_char = (if openparen as libc::c_int
                    == '(' as i32
                {
                    ')' as i32
                } else {
                    '}' as i32
                }) as libc::c_char;
                let mut outref: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut count: libc::c_int = 0;
                let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
                let fresh6 = in_0;
                in_0 = in_0.offset(1);
                let fresh7 = out;
                out = out.offset(1);
                *fresh7 = *fresh6;
                outref = out;
                count = 0 as libc::c_int;
                while *in_0 as libc::c_int != '\0' as i32 {
                    if *in_0 as libc::c_int == closeparen as libc::c_int
                        && {
                            count -= 1;
                            count < 0 as libc::c_int
                        }
                    {
                        break;
                    }
                    if *in_0 as libc::c_int == '\\' as i32
                        && *in_0.offset(1 as libc::c_int as isize) as libc::c_int
                            == '\n' as i32
                    {
                        let mut quoted: libc::c_int = 0 as libc::c_int;
                        p = in_0.offset(-(1 as libc::c_int as isize));
                        while p > ref_0 && *p as libc::c_int == '\\' as i32 {
                            quoted = (quoted == 0) as libc::c_int;
                            p = p.offset(-1);
                            p;
                        }
                        if quoted != 0 {
                            let fresh8 = in_0;
                            in_0 = in_0.offset(1);
                            let fresh9 = out;
                            out = out.offset(1);
                            *fresh9 = *fresh8;
                        } else {
                            in_0 = in_0.offset(2 as libc::c_int as isize);
                            while *stopchar_map
                                .as_mut_ptr()
                                .offset(*in_0 as libc::c_uchar as isize) as libc::c_int
                                & (0x2 as libc::c_int | 0x4 as libc::c_int)
                                != 0 as libc::c_int
                            {
                                in_0 = in_0.offset(1);
                                in_0;
                            }
                            while out > outref
                                && *stopchar_map
                                    .as_mut_ptr()
                                    .offset(
                                        *out.offset(-(1 as libc::c_int) as isize) as libc::c_uchar
                                            as isize,
                                    ) as libc::c_int & 0x2 as libc::c_int != 0 as libc::c_int
                            {
                                out = out.offset(-1);
                                out;
                            }
                            let fresh10 = out;
                            out = out.offset(1);
                            *fresh10 = ' ' as i32 as libc::c_char;
                        }
                    } else {
                        if *in_0 as libc::c_int == openparen as libc::c_int {
                            count += 1;
                            count;
                        }
                        let fresh11 = in_0;
                        in_0 = in_0.offset(1);
                        let fresh12 = out;
                        out = out.offset(1);
                        *fresh12 = *fresh11;
                    }
                }
            }
        }
        if out != in_0 {
            memmove(
                out as *mut libc::c_void,
                in_0 as *const libc::c_void,
                (strlen(in_0)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        }
        (*cmds).fileinfo.offset = i as libc::c_ulong;
        let ref mut fresh13 = *lines.offset(i as isize);
        *fresh13 = allocated_variable_expand_for_file(
            *((*cmds).command_lines).offset(i as isize),
            file,
        );
        i = i.wrapping_add(1);
        i;
    }
    (*cmds).fileinfo.offset = 0 as libc::c_int as libc::c_ulong;
    (*c).command_lines = lines;
    job_next_command(c);
    if job_slots != 0 as libc::c_int as libc::c_uint {
        while job_slots_used == job_slots {
            reap_children(1 as libc::c_int, 0 as libc::c_int);
        }
    } else if jobserver_enabled() != 0 {
        loop {
            let mut got_token: libc::c_int = 0;
            if 0x4 as libc::c_int & db_level != 0 {
                printf(
                    b"Need a job token; we %shave children\n\0" as *const u8
                        as *const libc::c_char,
                    if !children.is_null() {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b"don't \0" as *const u8 as *const libc::c_char
                    },
                );
                fflush(stdout);
            }
            if jobserver_tokens == 0 {
                break;
            }
            jobserver_pre_acquire();
            reap_children(0 as libc::c_int, 0 as libc::c_int);
            start_waiting_jobs();
            if jobserver_tokens == 0 {
                break;
            }
            if children.is_null() {
                fatal(
                    0 as *mut floc,
                    0 as libc::c_int as size_t,
                    b"INTERNAL: no children as we go to sleep on read\0" as *const u8
                        as *const libc::c_char,
                );
            }
            got_token = jobserver_acquire(
                (waiting_jobs != 0 as *mut libc::c_void as *mut child) as libc::c_int,
            ) as libc::c_int;
            if !(got_token == 1 as libc::c_int) {
                continue;
            }
            if 0x4 as libc::c_int & db_level != 0 {
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Obtained token for child %p (%s).\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    c,
                    (*(*c).file).name,
                );
                fflush(stdout);
            }
            break;
        }
    }
    jobserver_tokens = jobserver_tokens.wrapping_add(1);
    jobserver_tokens;
    if 0x20 as libc::c_int & db_level != 0 {
        let mut nm: *const libc::c_char = 0 as *const libc::c_char;
        if ((*cmds).fileinfo.filenm).is_null() {
            nm = dcgettext(
                0 as *const libc::c_char,
                b"<builtin>\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        } else {
            let mut fresh14 = ::std::vec::from_elem(
                0,
                (strlen((*cmds).fileinfo.filenm))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(11 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
            );
            let mut n: *mut libc::c_char = fresh14.leak().as_mut_ptr()
                as *mut libc::c_char;
            sprintf(
                n,
                b"%s:%lu\0" as *const u8 as *const libc::c_char,
                (*cmds).fileinfo.filenm,
                (*cmds).fileinfo.lineno,
            );
            nm = n;
        }
        if (*(*c).file).phony() != 0 {
            message(
                0 as libc::c_int,
                (strlen(nm)).wrapping_add(strlen((*(*c).file).name)),
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: update target '%s' due to: target is .PHONY\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                nm,
                (*(*c).file).name,
            );
        } else if (*(*c).file).last_mtime == 1 as libc::c_int as libc::c_ulong {
            message(
                0 as libc::c_int,
                (strlen(nm)).wrapping_add(strlen((*(*c).file).name)),
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: update target '%s' due to: target does not exist\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                nm,
                (*(*c).file).name,
            );
        } else {
            let mut newer: *mut libc::c_char = allocated_variable_expand_for_file(
                b"$?\0" as *const u8 as *const libc::c_char,
                (*c).file,
            );
            if *newer.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
                message(
                    0 as libc::c_int,
                    (strlen(nm))
                        .wrapping_add(strlen((*(*c).file).name))
                        .wrapping_add(strlen(newer)),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: update target '%s' due to: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    nm,
                    (*(*c).file).name,
                    newer,
                );
                free(newer as *mut libc::c_void);
            } else {
                let mut len: size_t = 0 as libc::c_int as size_t;
                let mut d: *mut dep = 0 as *mut dep;
                d = (*(*c).file).deps;
                while !d.is_null() {
                    if (*(*d).file).last_mtime == 1 as libc::c_int as libc::c_ulong {
                        len = (len as libc::c_ulong)
                            .wrapping_add(
                                (strlen((*(*d).file).name))
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as size_t as size_t;
                    }
                    d = (*d).next;
                }
                if len == 0 {
                    message(
                        0 as libc::c_int,
                        (strlen(nm)).wrapping_add(strlen((*(*c).file).name)),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: update target '%s' due to: unknown reasons\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        nm,
                        (*(*c).file).name,
                    );
                } else {
                    let mut fresh15 = ::std::vec::from_elem(0, len as usize);
                    newer = fresh15.leak().as_mut_ptr() as *mut libc::c_char;
                    let mut cp: *mut libc::c_char = newer;
                    d = (*(*c).file).deps;
                    while !d.is_null() {
                        if (*(*d).file).last_mtime == 1 as libc::c_int as libc::c_ulong {
                            if cp > newer {
                                let fresh16 = cp;
                                cp = cp.offset(1);
                                *fresh16 = ' ' as i32 as libc::c_char;
                            }
                            cp = stpcpy(cp, (*(*d).file).name);
                        }
                        d = (*d).next;
                    }
                    message(
                        0 as libc::c_int,
                        (strlen(nm))
                            .wrapping_add(strlen((*(*c).file).name))
                            .wrapping_add(strlen(newer)),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: update target '%s' due to: %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        nm,
                        (*(*c).file).name,
                        newer,
                    );
                }
            }
        }
    }
    start_waiting_job(c);
    if job_slots == 1 as libc::c_int as libc::c_uint || not_parallel != 0 {
        while (*file).command_state() as libc::c_int == cs_running as libc::c_int {
            reap_children(1 as libc::c_int, 0 as libc::c_int);
        }
    }
    output_context = 0 as *mut output;
}
unsafe extern "C" fn job_next_command(mut child: *mut child) -> libc::c_int {
    while ((*child).command_ptr).is_null()
        || *(*child).command_ptr as libc::c_int == '\0' as i32
    {
        if (*child).command_line
            == (*(*(*child).file).cmds).ncommand_lines as libc::c_uint
        {
            (*child).command_ptr = 0 as *mut libc::c_char;
            (*(*(*child).file).cmds).fileinfo.offset = 0 as libc::c_int as libc::c_ulong;
            return 0 as libc::c_int;
        } else {
            let fresh17 = (*child).command_line;
            (*child).command_line = ((*child).command_line).wrapping_add(1);
            (*child).command_ptr = *((*child).command_lines).offset(fresh17 as isize);
        }
    }
    (*(*(*child).file).cmds)
        .fileinfo
        .offset = ((*child).command_line).wrapping_sub(1 as libc::c_int as libc::c_uint)
        as libc::c_ulong;
    return 1 as libc::c_int;
}
unsafe extern "C" fn load_too_high() -> libc::c_int {
    static mut last_sec: libc::c_double = 0.;
    static mut last_now: time_t = 0;
    static mut proc_fd: libc::c_int = -(2 as libc::c_int);
    let mut load: libc::c_double = 0.;
    let mut guess: libc::c_double = 0.;
    let mut now: time_t = 0;
    if max_load_average < 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int;
    }
    if proc_fd == -(2 as libc::c_int) {
        loop {
            proc_fd = open(
                b"/proc/loadavg\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
            if !(proc_fd == -(1 as libc::c_int)
                && *__errno_location() == 4 as libc::c_int)
            {
                break;
            }
        }
        if proc_fd < 0 as libc::c_int {
            if 0x4 as libc::c_int & db_level != 0 {
                printf(
                    b"Using system load detection method.\n\0" as *const u8
                        as *const libc::c_char,
                );
                fflush(stdout);
            }
        } else {
            if 0x4 as libc::c_int & db_level != 0 {
                printf(
                    b"Using /proc/loadavg load detection method.\n\0" as *const u8
                        as *const libc::c_char,
                );
                fflush(stdout);
            }
            fd_noinherit(proc_fd);
        }
    }
    if proc_fd >= 0 as libc::c_int {
        let mut r: libc::c_int = 0;
        loop {
            r = lseek(proc_fd, 0 as libc::c_int as __off_t, 0 as libc::c_int)
                as libc::c_int;
            if !(r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if r >= 0 as libc::c_int {
            let mut avg: [libc::c_char; 65] = [0; 65];
            loop {
                r = read(
                    proc_fd,
                    avg.as_mut_ptr() as *mut libc::c_void,
                    64 as libc::c_int as size_t,
                ) as libc::c_int;
                if !(r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
            }
            if r >= 0 as libc::c_int {
                let mut p: *const libc::c_char = 0 as *const libc::c_char;
                avg[r as usize] = '\0' as i32 as libc::c_char;
                p = strchr(avg.as_mut_ptr(), ' ' as i32);
                if !p.is_null() {
                    p = strchr(p.offset(1 as libc::c_int as isize), ' ' as i32);
                }
                if !p.is_null() {
                    p = strchr(p.offset(1 as libc::c_int as isize), ' ' as i32);
                }
                if !p.is_null()
                    && (*p.offset(1 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_sub('0' as i32 as libc::c_uint)
                        <= 9 as libc::c_int as libc::c_uint
                {
                    let mut cnt: libc::c_uint = make_toui(
                        p.offset(1 as libc::c_int as isize),
                        0 as *mut *const libc::c_char,
                    );
                    if 0x4 as libc::c_int & db_level != 0 {
                        printf(
                            b"Running: system = %u / make = %u (max requested = %f)\n\0"
                                as *const u8 as *const libc::c_char,
                            cnt,
                            job_slots_used,
                            max_load_average,
                        );
                        fflush(stdout);
                    }
                    return (cnt as libc::c_double > max_load_average) as libc::c_int;
                }
                if 0x4 as libc::c_int & db_level != 0 {
                    printf(
                        b"Failed to parse /proc/loadavg: %s\n\0" as *const u8
                            as *const libc::c_char,
                        avg.as_mut_ptr(),
                    );
                    fflush(stdout);
                }
            }
        }
        if r < 0 as libc::c_int {
            if 0x4 as libc::c_int & db_level != 0 {
                printf(
                    b"Failed to read /proc/loadavg: %s\n\0" as *const u8
                        as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                fflush(stdout);
            }
        }
        close(proc_fd);
        proc_fd = -(1 as libc::c_int);
    }
    *__errno_location() = 0 as libc::c_int;
    if getloadavg(&mut load, 1 as libc::c_int) != 1 as libc::c_int {
        static mut lossage: libc::c_int = -(1 as libc::c_int);
        if lossage == -(1 as libc::c_int) || *__errno_location() != lossage {
            if *__errno_location() == 0 as libc::c_int {
                error(
                    0 as *mut floc,
                    0 as libc::c_int as size_t,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot enforce load limits on this operating system\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            } else {
                perror_with_name(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot enforce load limit: \0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    b"getloadavg\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        lossage = *__errno_location();
        load = 0 as libc::c_int as libc::c_double;
    }
    now = time(0 as *mut time_t);
    if last_now < now {
        if last_now == now - 1 as libc::c_int as libc::c_long {
            last_sec = 0.25f64 * job_counter as libc::c_double;
        } else {
            last_sec = 0.0f64;
        }
        job_counter = 0 as libc::c_int as libc::c_ulong;
        last_now = now;
    }
    guess = load + 0.25f64 * (job_counter as libc::c_double + last_sec);
    if 0x4 as libc::c_int & db_level != 0 {
        printf(
            b"Estimated system load = %f (actual = %f) (max requested = %f)\n\0"
                as *const u8 as *const libc::c_char,
            guess,
            load,
            max_load_average,
        );
        fflush(stdout);
    }
    return (guess >= max_load_average) as libc::c_int;
}
pub unsafe extern "C" fn start_waiting_jobs() {
    let mut job: *mut child = 0 as *mut child;
    if waiting_jobs.is_null() {
        return;
    }
    loop {
        reap_children(0 as libc::c_int, 0 as libc::c_int);
        job = waiting_jobs;
        waiting_jobs = (*job).next;
        if !(start_waiting_job(job) != 0 && !waiting_jobs.is_null()) {
            break;
        }
    };
}
pub unsafe extern "C" fn child_execute_job(
    mut child: *mut childbase,
    mut good_stdin: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> pid_t {
    let mut current_block: u64;
    let fdin: libc::c_int = if good_stdin != 0 {
        fileno(stdin)
    } else {
        get_bad_stdin()
    };
    let mut fdout: libc::c_int = fileno(stdout);
    let mut fderr: libc::c_int = fileno(stderr);
    let mut pid: pid_t = -(1 as libc::c_int);
    let mut r: libc::c_int = 0;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut attr: posix_spawnattr_t = posix_spawnattr_t {
        __flags: 0,
        __pgrp: 0,
        __sd: sigset_t { __val: [0; 16] },
        __ss: sigset_t { __val: [0; 16] },
        __sp: sched_param { sched_priority: 0 },
        __policy: 0,
        __pad: [0; 16],
    };
    let mut fa: posix_spawn_file_actions_t = posix_spawn_file_actions_t {
        __allocated: 0,
        __used: 0,
        __actions: 0 as *mut __spawn_action,
        __pad: [0; 16],
    };
    let mut flags: libc::c_short = 0 as libc::c_int as libc::c_short;
    if ((*child).output).syncout() != 0 {
        if (*child).output.out >= 0 as libc::c_int {
            fdout = (*child).output.out;
        }
        if (*child).output.err >= 0 as libc::c_int {
            fderr = (*child).output.err;
        }
    }
    r = posix_spawnattr_init(&mut attr);
    if !(r != 0 as libc::c_int) {
        r = posix_spawn_file_actions_init(&mut fa);
        if r != 0 as libc::c_int {
            posix_spawnattr_destroy(&mut attr);
        } else {
            let mut mask: sigset_t = sigset_t { __val: [0; 16] };
            sigemptyset(&mut mask);
            r = posix_spawnattr_setsigmask(&mut attr, &mut mask);
            if !(r != 0 as libc::c_int) {
                flags = (flags as libc::c_int | 0x8 as libc::c_int) as libc::c_short;
                flags = (flags as libc::c_int | 0x40 as libc::c_int) as libc::c_short;
                if fdin >= 0 as libc::c_int && fdin != fileno(stdin) {
                    r = posix_spawn_file_actions_adddup2(&mut fa, fdin, fileno(stdin));
                    if r != 0 as libc::c_int {
                        current_block = 11005858626190426727;
                    } else {
                        current_block = 17833034027772472439;
                    }
                } else {
                    current_block = 17833034027772472439;
                }
                match current_block {
                    11005858626190426727 => {}
                    _ => {
                        if fdout != fileno(stdout) {
                            r = posix_spawn_file_actions_adddup2(
                                &mut fa,
                                fdout,
                                fileno(stdout),
                            );
                            if r != 0 as libc::c_int {
                                current_block = 11005858626190426727;
                            } else {
                                current_block = 7175849428784450219;
                            }
                        } else {
                            current_block = 7175849428784450219;
                        }
                        match current_block {
                            11005858626190426727 => {}
                            _ => {
                                if fderr != fileno(stderr) {
                                    r = posix_spawn_file_actions_adddup2(
                                        &mut fa,
                                        fderr,
                                        fileno(stderr),
                                    );
                                    if r != 0 as libc::c_int {
                                        current_block = 11005858626190426727;
                                    } else {
                                        current_block = 5601891728916014340;
                                    }
                                } else {
                                    current_block = 5601891728916014340;
                                }
                                match current_block {
                                    11005858626190426727 => {}
                                    _ => {
                                        r = posix_spawnattr_setflags(&mut attr, flags);
                                        if !(r != 0 as libc::c_int) {
                                            let mut p: *const libc::c_char = 0 as *const libc::c_char;
                                            let mut pp: *mut *mut libc::c_char = 0
                                                as *mut *mut libc::c_char;
                                            pp = (*child).environment;
                                            while !(*pp).is_null() {
                                                if *(*pp).offset(0 as libc::c_int as isize) as libc::c_int
                                                    == 'P' as i32
                                                    && *(*pp).offset(1 as libc::c_int as isize) as libc::c_int
                                                        == 'A' as i32
                                                    && *(*pp).offset(2 as libc::c_int as isize) as libc::c_int
                                                        == 'T' as i32
                                                    && *(*pp).offset(3 as libc::c_int as isize) as libc::c_int
                                                        == 'H' as i32
                                                    && *(*pp).offset(4 as libc::c_int as isize) as libc::c_int
                                                        == '=' as i32
                                                {
                                                    p = (*pp).offset(5 as libc::c_int as isize);
                                                    break;
                                                } else {
                                                    pp = pp.offset(1);
                                                    pp;
                                                }
                                            }
                                            if p.is_null() {
                                                let mut l: size_t = confstr(
                                                    _CS_PATH as libc::c_int,
                                                    0 as *mut libc::c_char,
                                                    0 as libc::c_int as size_t,
                                                );
                                                if l != 0 {
                                                    let mut fresh18 = ::std::vec::from_elem(0, l as usize);
                                                    let mut dp: *mut libc::c_char = fresh18.leak().as_mut_ptr()
                                                        as *mut libc::c_char;
                                                    confstr(_CS_PATH as libc::c_int, dp, l);
                                                    p = dp;
                                                }
                                            }
                                            cmd = find_in_given_path(
                                                *argv.offset(0 as libc::c_int as isize),
                                                p,
                                                0 as *const libc::c_char,
                                                0 as libc::c_int != 0,
                                            ) as *mut libc::c_char;
                                            if cmd.is_null() {
                                                r = *__errno_location();
                                            } else {
                                                loop {
                                                    r = posix_spawn(
                                                        &mut pid,
                                                        cmd,
                                                        &mut fa,
                                                        &mut attr,
                                                        argv,
                                                        (*child).environment,
                                                    );
                                                    if !(r == 4 as libc::c_int) {
                                                        break;
                                                    }
                                                }
                                                if r == 8 as libc::c_int {
                                                    let mut nargv: *mut *mut libc::c_char = 0
                                                        as *mut *mut libc::c_char;
                                                    let mut pp_0: *mut *mut libc::c_char = 0
                                                        as *mut *mut libc::c_char;
                                                    let mut l_0: size_t = 0 as libc::c_int as size_t;
                                                    pp_0 = argv;
                                                    while !(*pp_0).is_null() {
                                                        l_0 = l_0.wrapping_add(1);
                                                        l_0;
                                                        pp_0 = pp_0.offset(1);
                                                        pp_0;
                                                    }
                                                    nargv = xmalloc(
                                                        (::std::mem::size_of::<*mut libc::c_char>()
                                                            as libc::c_ulong)
                                                            .wrapping_mul(
                                                                l_0.wrapping_add(3 as libc::c_int as libc::c_ulong),
                                                            ),
                                                    ) as *mut *mut libc::c_char;
                                                    let ref mut fresh19 = *nargv
                                                        .offset(0 as libc::c_int as isize);
                                                    *fresh19 = default_shell as *mut libc::c_char;
                                                    let ref mut fresh20 = *nargv
                                                        .offset(1 as libc::c_int as isize);
                                                    *fresh20 = cmd;
                                                    memcpy(
                                                        &mut *nargv.offset(2 as libc::c_int as isize)
                                                            as *mut *mut libc::c_char as *mut libc::c_void,
                                                        &mut *argv.offset(1 as libc::c_int as isize)
                                                            as *mut *mut libc::c_char as *const libc::c_void,
                                                        (::std::mem::size_of::<*mut libc::c_char>()
                                                            as libc::c_ulong)
                                                            .wrapping_mul(l_0),
                                                    );
                                                    loop {
                                                        r = posix_spawn(
                                                            &mut pid,
                                                            *nargv.offset(0 as libc::c_int as isize),
                                                            &mut fa,
                                                            &mut attr,
                                                            nargv,
                                                            (*child).environment,
                                                        );
                                                        if !(r == 4 as libc::c_int) {
                                                            break;
                                                        }
                                                    }
                                                    free(nargv as *mut libc::c_void);
                                                }
                                                if r == 0 as libc::c_int {
                                                    free((*child).cmd_name as *mut libc::c_void);
                                                    if cmd != *argv.offset(0 as libc::c_int as isize) {
                                                        (*child).cmd_name = cmd;
                                                    } else {
                                                        (*child).cmd_name = xstrdup(cmd);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            posix_spawn_file_actions_destroy(&mut fa);
            posix_spawnattr_destroy(&mut attr);
        }
    }
    if r != 0 as libc::c_int {
        pid = -(1 as libc::c_int);
    }
    if pid < 0 as libc::c_int {
        error(
            0 as *mut floc,
            (strlen(*argv.offset(0 as libc::c_int as isize)))
                .wrapping_add(strlen(strerror(r))),
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
            strerror(r),
        );
    }
    return pid;
}
pub unsafe extern "C" fn exec_command(
    mut argv: *mut *mut libc::c_char,
    mut envp: *mut *mut libc::c_char,
) -> pid_t {
    let mut pid: pid_t = -(1 as libc::c_int);
    environ = envp;
    execvp(*argv.offset(0 as libc::c_int as isize), argv as *const *mut libc::c_char);
    match *__errno_location() {
        2 => {
            error(
                0 as *mut floc,
                (strlen(*argv.offset(0 as libc::c_int as isize)))
                    .wrapping_add(strlen(strerror(*__errno_location()))),
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize),
                strerror(*__errno_location()),
            );
        }
        8 => {
            let mut shell: *const libc::c_char = 0 as *const libc::c_char;
            let mut new_argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            let mut argc: libc::c_int = 0;
            let mut i: libc::c_int = 1 as libc::c_int;
            shell = getenv(b"SHELL\0" as *const u8 as *const libc::c_char);
            if shell.is_null() {
                shell = default_shell;
            }
            argc = 1 as libc::c_int;
            while !(*argv.offset(argc as isize)).is_null() {
                argc += 1;
                argc;
            }
            let mut fresh21 = ::std::vec::from_elem(
                0,
                ((1 as libc::c_int + argc + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ) as usize,
            );
            new_argv = fresh21.leak().as_mut_ptr() as *mut *mut libc::c_char;
            let ref mut fresh22 = *new_argv.offset(0 as libc::c_int as isize);
            *fresh22 = shell as *mut libc::c_char;
            let ref mut fresh23 = *new_argv.offset(i as isize);
            *fresh23 = *argv.offset(0 as libc::c_int as isize);
            while argc > 0 as libc::c_int {
                let ref mut fresh24 = *new_argv.offset((i + argc) as isize);
                *fresh24 = *argv.offset(argc as isize);
                argc -= 1;
                argc;
            }
            execvp(shell, new_argv as *const *mut libc::c_char);
            error(
                0 as *mut floc,
                (strlen(*new_argv.offset(0 as libc::c_int as isize)))
                    .wrapping_add(strlen(strerror(*__errno_location()))),
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                *new_argv.offset(0 as libc::c_int as isize),
                strerror(*__errno_location()),
            );
        }
        _ => {
            error(
                0 as *mut floc,
                (strlen(*argv.offset(0 as libc::c_int as isize)))
                    .wrapping_add(strlen(strerror(*__errno_location()))),
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize),
                strerror(*__errno_location()),
            );
        }
    }
    return pid;
}
unsafe extern "C" fn construct_command_argv_internal(
    mut line: *mut libc::c_char,
    mut restp: *mut *mut libc::c_char,
    mut shell: *const libc::c_char,
    mut shellflags: *const libc::c_char,
    mut ifs: *const libc::c_char,
    mut flags: libc::c_int,
    mut batch_filename: *mut *mut libc::c_char,
) -> *mut *mut libc::c_char {
    let mut current_block: u64;
    static mut sh_chars: *const libc::c_char = b"#;\"*?[]&|<>(){}$`^~!\0" as *const u8
        as *const libc::c_char;
    static mut sh_cmds: [*const libc::c_char; 38] = [
        b".\0" as *const u8 as *const libc::c_char,
        b":\0" as *const u8 as *const libc::c_char,
        b"alias\0" as *const u8 as *const libc::c_char,
        b"bg\0" as *const u8 as *const libc::c_char,
        b"break\0" as *const u8 as *const libc::c_char,
        b"case\0" as *const u8 as *const libc::c_char,
        b"cd\0" as *const u8 as *const libc::c_char,
        b"command\0" as *const u8 as *const libc::c_char,
        b"continue\0" as *const u8 as *const libc::c_char,
        b"eval\0" as *const u8 as *const libc::c_char,
        b"exec\0" as *const u8 as *const libc::c_char,
        b"exit\0" as *const u8 as *const libc::c_char,
        b"export\0" as *const u8 as *const libc::c_char,
        b"fc\0" as *const u8 as *const libc::c_char,
        b"fg\0" as *const u8 as *const libc::c_char,
        b"for\0" as *const u8 as *const libc::c_char,
        b"getopts\0" as *const u8 as *const libc::c_char,
        b"hash\0" as *const u8 as *const libc::c_char,
        b"if\0" as *const u8 as *const libc::c_char,
        b"jobs\0" as *const u8 as *const libc::c_char,
        b"login\0" as *const u8 as *const libc::c_char,
        b"logout\0" as *const u8 as *const libc::c_char,
        b"read\0" as *const u8 as *const libc::c_char,
        b"readonly\0" as *const u8 as *const libc::c_char,
        b"return\0" as *const u8 as *const libc::c_char,
        b"set\0" as *const u8 as *const libc::c_char,
        b"shift\0" as *const u8 as *const libc::c_char,
        b"test\0" as *const u8 as *const libc::c_char,
        b"times\0" as *const u8 as *const libc::c_char,
        b"trap\0" as *const u8 as *const libc::c_char,
        b"type\0" as *const u8 as *const libc::c_char,
        b"ulimit\0" as *const u8 as *const libc::c_char,
        b"umask\0" as *const u8 as *const libc::c_char,
        b"unalias\0" as *const u8 as *const libc::c_char,
        b"unset\0" as *const u8 as *const libc::c_char,
        b"wait\0" as *const u8 as *const libc::c_char,
        b"while\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut i: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ap: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cap: *const libc::c_char = 0 as *const libc::c_char;
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut instring: libc::c_int = 0;
    let mut word_has_equals: libc::c_int = 0;
    let mut seen_nonequals: libc::c_int = 0;
    let mut last_argument_was_empty: libc::c_int = 0;
    let mut new_argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut argstr: *mut libc::c_char = 0 as *mut libc::c_char;
    if !restp.is_null() {
        *restp = 0 as *mut libc::c_char;
    }
    while *stopchar_map.as_mut_ptr().offset(*line as libc::c_uchar as isize)
        as libc::c_int & 0x2 as libc::c_int != 0 as libc::c_int
    {
        line = line.offset(1);
        line;
    }
    if *line as libc::c_int == '\0' as i32 {
        return 0 as *mut *mut libc::c_char;
    }
    if shellflags.is_null() {
        shellflags = if posix_pedantic != 0
            && !(flags & 4 as libc::c_int != 0 as libc::c_int)
        {
            b"-ec\0" as *const u8 as *const libc::c_char
        } else {
            b"-c\0" as *const u8 as *const libc::c_char
        };
    }
    if shell.is_null() {
        shell = default_shell;
        current_block = 2979737022853876585;
    } else if strcmp(shell, default_shell) != 0 {
        current_block = 8102658916883067714;
    } else {
        current_block = 2979737022853876585;
    }
    match current_block {
        2979737022853876585 => {
            if !ifs.is_null() {
                cap = ifs;
                loop {
                    if !(*cap as libc::c_int != '\0' as i32) {
                        current_block = 7149356873433890176;
                        break;
                    }
                    if *cap as libc::c_int != ' ' as i32
                        && *cap as libc::c_int != '\t' as i32
                        && *cap as libc::c_int != '\n' as i32
                    {
                        current_block = 8102658916883067714;
                        break;
                    }
                    cap = cap.offset(1);
                    cap;
                }
            } else {
                current_block = 7149356873433890176;
            }
            match current_block {
                8102658916883067714 => {}
                _ => {
                    if !shellflags.is_null() {
                        if *shellflags.offset(0 as libc::c_int as isize) as libc::c_int
                            != '-' as i32
                            || (*shellflags.offset(1 as libc::c_int as isize)
                                as libc::c_int != 'c' as i32
                                || *shellflags.offset(2 as libc::c_int as isize)
                                    as libc::c_int != '\0' as i32)
                                && (*shellflags.offset(1 as libc::c_int as isize)
                                    as libc::c_int != 'e' as i32
                                    || *shellflags.offset(2 as libc::c_int as isize)
                                        as libc::c_int != 'c' as i32
                                    || *shellflags.offset(3 as libc::c_int as isize)
                                        as libc::c_int != '\0' as i32)
                        {
                            current_block = 8102658916883067714;
                        } else {
                            current_block = 5948590327928692120;
                        }
                    } else {
                        current_block = 5948590327928692120;
                    }
                    match current_block {
                        8102658916883067714 => {}
                        _ => {
                            i = (strlen(line))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong);
                            new_argv = xmalloc(
                                i
                                    .wrapping_mul(
                                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                                    ),
                            ) as *mut *mut libc::c_char;
                            argstr = xmalloc(i) as *mut libc::c_char;
                            let ref mut fresh25 = *new_argv
                                .offset(0 as libc::c_int as isize);
                            *fresh25 = argstr;
                            ap = *fresh25;
                            i = 0 as libc::c_int as size_t;
                            last_argument_was_empty = 0 as libc::c_int;
                            seen_nonequals = last_argument_was_empty;
                            word_has_equals = seen_nonequals;
                            instring = word_has_equals;
                            p = line;
                            's_110: loop {
                                if !(*p as libc::c_int != '\0' as i32) {
                                    current_block = 5885586228174720522;
                                    break;
                                }
                                if instring != 0 {
                                    if *p as libc::c_int == instring {
                                        instring = 0 as libc::c_int;
                                        if ap == *new_argv.offset(0 as libc::c_int as isize)
                                            || *ap.offset(-(1 as libc::c_int as isize)) as libc::c_int
                                                == '\0' as i32
                                        {
                                            last_argument_was_empty = 1 as libc::c_int;
                                        }
                                    } else if *p as libc::c_int == '\\' as i32
                                        && *p.offset(1 as libc::c_int as isize) as libc::c_int
                                            == '\n' as i32
                                    {
                                        if instring == '"' as i32 {
                                            p = p.offset(1);
                                            p;
                                        } else {
                                            let fresh26 = p;
                                            p = p.offset(1);
                                            let fresh27 = ap;
                                            ap = ap.offset(1);
                                            *fresh27 = *fresh26;
                                            let fresh28 = ap;
                                            ap = ap.offset(1);
                                            *fresh28 = *p;
                                        }
                                    } else if *p as libc::c_int == '\n' as i32
                                        && !restp.is_null()
                                    {
                                        *restp = p;
                                        current_block = 5885586228174720522;
                                        break;
                                    } else {
                                        if instring == '"' as i32
                                            && !(strchr(
                                                b"\\$`\0" as *const u8 as *const libc::c_char,
                                                *p as libc::c_int,
                                            ))
                                                .is_null() && unixy_shell != 0
                                        {
                                            current_block = 8102658916883067714;
                                            break;
                                        }
                                        let fresh29 = ap;
                                        ap = ap.offset(1);
                                        *fresh29 = *p;
                                    }
                                } else if !(strchr(sh_chars, *p as libc::c_int)).is_null() {
                                    current_block = 8102658916883067714;
                                    break;
                                } else if one_shell != 0 && *p as libc::c_int == '\n' as i32
                                {
                                    current_block = 8102658916883067714;
                                    break;
                                } else {
                                    match *p as libc::c_int {
                                        61 => {
                                            if seen_nonequals == 0 && unixy_shell != 0 {
                                                current_block = 8102658916883067714;
                                                break;
                                            }
                                            word_has_equals = 1 as libc::c_int;
                                            let fresh30 = ap;
                                            ap = ap.offset(1);
                                            *fresh30 = '=' as i32 as libc::c_char;
                                        }
                                        92 => {
                                            if *p.offset(1 as libc::c_int as isize) as libc::c_int
                                                == '\n' as i32
                                            {
                                                p = p.offset(1);
                                                p;
                                                if ap == *new_argv.offset(i as isize) {
                                                    while *stopchar_map
                                                        .as_mut_ptr()
                                                        .offset(
                                                            *p.offset(1 as libc::c_int as isize) as libc::c_uchar
                                                                as isize,
                                                        ) as libc::c_int & 0x2 as libc::c_int != 0 as libc::c_int
                                                    {
                                                        p = p.offset(1);
                                                        p;
                                                    }
                                                }
                                            } else if *p.offset(1 as libc::c_int as isize)
                                                as libc::c_int != '\0' as i32
                                            {
                                                p = p.offset(1);
                                                let fresh31 = ap;
                                                ap = ap.offset(1);
                                                *fresh31 = *p;
                                            }
                                        }
                                        39 | 34 => {
                                            instring = *p as libc::c_int;
                                        }
                                        10 => {
                                            if !restp.is_null() {
                                                *restp = p;
                                                current_block = 5885586228174720522;
                                                break;
                                            } else {
                                                let fresh32 = ap;
                                                ap = ap.offset(1);
                                                *fresh32 = '\n' as i32 as libc::c_char;
                                            }
                                        }
                                        32 | 9 => {
                                            let fresh33 = ap;
                                            ap = ap.offset(1);
                                            *fresh33 = '\0' as i32 as libc::c_char;
                                            i = i.wrapping_add(1);
                                            let ref mut fresh34 = *new_argv.offset(i as isize);
                                            *fresh34 = ap;
                                            last_argument_was_empty = 0 as libc::c_int;
                                            seen_nonequals |= (word_has_equals == 0) as libc::c_int;
                                            if word_has_equals != 0 && seen_nonequals == 0 {
                                                current_block = 8102658916883067714;
                                                break;
                                            } else {
                                                word_has_equals = 0 as libc::c_int;
                                                if i == 1 as libc::c_int as libc::c_ulong {
                                                    let mut j: libc::c_int = 0;
                                                    j = 0 as libc::c_int;
                                                    while !(sh_cmds[j as usize]).is_null() {
                                                        if sh_cmds[j as usize]
                                                            == *new_argv.offset(0 as libc::c_int as isize)
                                                                as *const libc::c_char
                                                            || *sh_cmds[j as usize] as libc::c_int
                                                                == **new_argv.offset(0 as libc::c_int as isize)
                                                                    as libc::c_int
                                                                && (*sh_cmds[j as usize] as libc::c_int == '\0' as i32
                                                                    || strcmp(
                                                                        (sh_cmds[j as usize]).offset(1 as libc::c_int as isize),
                                                                        (*new_argv.offset(0 as libc::c_int as isize))
                                                                            .offset(1 as libc::c_int as isize),
                                                                    ) == 0)
                                                        {
                                                            current_block = 8102658916883067714;
                                                            break 's_110;
                                                        }
                                                        j += 1;
                                                        j;
                                                    }
                                                }
                                                while *stopchar_map
                                                    .as_mut_ptr()
                                                    .offset(
                                                        *p.offset(1 as libc::c_int as isize) as libc::c_uchar
                                                            as isize,
                                                    ) as libc::c_int & 0x2 as libc::c_int != 0 as libc::c_int
                                                {
                                                    p = p.offset(1);
                                                    p;
                                                }
                                            }
                                        }
                                        _ => {
                                            let fresh35 = ap;
                                            ap = ap.offset(1);
                                            *fresh35 = *p;
                                        }
                                    }
                                }
                                p = p.offset(1);
                                p;
                            }
                            match current_block {
                                8102658916883067714 => {}
                                _ => {
                                    if !(instring != 0) {
                                        *ap = '\0' as i32 as libc::c_char;
                                        if *(*new_argv.offset(i as isize))
                                            .offset(0 as libc::c_int as isize) as libc::c_int
                                            != '\0' as i32 || last_argument_was_empty != 0
                                        {
                                            i = i.wrapping_add(1);
                                            i;
                                        }
                                        let ref mut fresh36 = *new_argv.offset(i as isize);
                                        *fresh36 = 0 as *mut libc::c_char;
                                        if i == 1 as libc::c_int as libc::c_ulong {
                                            let mut j_0: libc::c_int = 0;
                                            j_0 = 0 as libc::c_int;
                                            loop {
                                                if (sh_cmds[j_0 as usize]).is_null() {
                                                    current_block = 3879520548144599102;
                                                    break;
                                                }
                                                if sh_cmds[j_0 as usize]
                                                    == *new_argv.offset(0 as libc::c_int as isize)
                                                        as *const libc::c_char
                                                    || *sh_cmds[j_0 as usize] as libc::c_int
                                                        == **new_argv.offset(0 as libc::c_int as isize)
                                                            as libc::c_int
                                                        && (*sh_cmds[j_0 as usize] as libc::c_int == '\0' as i32
                                                            || strcmp(
                                                                (sh_cmds[j_0 as usize]).offset(1 as libc::c_int as isize),
                                                                (*new_argv.offset(0 as libc::c_int as isize))
                                                                    .offset(1 as libc::c_int as isize),
                                                            ) == 0)
                                                {
                                                    current_block = 8102658916883067714;
                                                    break;
                                                }
                                                j_0 += 1;
                                                j_0;
                                            }
                                        } else {
                                            current_block = 3879520548144599102;
                                        }
                                        match current_block {
                                            8102658916883067714 => {}
                                            _ => {
                                                if (*new_argv.offset(0 as libc::c_int as isize)).is_null() {
                                                    free(argstr as *mut libc::c_void);
                                                    free(new_argv as *mut libc::c_void);
                                                    return 0 as *mut *mut libc::c_char;
                                                }
                                                return new_argv;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if !new_argv.is_null() {
        free(argstr as *mut libc::c_void);
        free(new_argv as *mut libc::c_void);
    }
    let mut new_line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut shell_len: size_t = strlen(shell);
    let mut line_len: size_t = strlen(line);
    let mut sflags_len: size_t = if !shellflags.is_null() {
        strlen(shellflags)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    if one_shell != 0 {
        if is_bourne_compatible_shell(shell) != 0 {
            let mut f: *const libc::c_char = line;
            let mut t: *mut libc::c_char = line;
            while *f.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
                let mut esc: libc::c_int = 0 as libc::c_int;
                while *stopchar_map.as_mut_ptr().offset(*f as libc::c_uchar as isize)
                    as libc::c_int & 0x2 as libc::c_int != 0 as libc::c_int
                    || *f as libc::c_int == '-' as i32 || *f as libc::c_int == '@' as i32
                    || *f as libc::c_int == '+' as i32
                {
                    f = f.offset(1);
                    f;
                }
                while *f as libc::c_int != '\0' as i32 {
                    let fresh37 = f;
                    f = f.offset(1);
                    let fresh38 = t;
                    t = t.offset(1);
                    *fresh38 = *fresh37;
                    if *f.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '\\' as i32
                    {
                        esc = (esc == 0) as libc::c_int;
                    } else {
                        if *f.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            == '\n' as i32 && esc == 0
                        {
                            break;
                        }
                        esc = 0 as libc::c_int;
                    }
                }
            }
            *t = '\0' as i32 as libc::c_char;
        }
        let mut n: libc::c_int = 1 as libc::c_int;
        let mut nextp: *mut libc::c_char = 0 as *mut libc::c_char;
        new_argv = xmalloc(
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_add(sflags_len.wrapping_div(2 as libc::c_int as libc::c_ulong))
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
        let ref mut fresh39 = *new_argv.offset(0 as libc::c_int as isize);
        *fresh39 = xmalloc(
            shell_len
                .wrapping_add(sflags_len)
                .wrapping_add(line_len)
                .wrapping_add(3 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        nextp = *fresh39;
        nextp = mempcpy(
            nextp as *mut libc::c_void,
            shell as *const libc::c_void,
            shell_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        if shellflags.is_null() {
            let fresh40 = n;
            n = n + 1;
            let ref mut fresh41 = *new_argv.offset(fresh40 as isize);
            *fresh41 = nextp;
            let fresh42 = nextp;
            nextp = nextp.offset(1);
            *fresh42 = '\0' as i32 as libc::c_char;
        } else {
            let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            let mut fresh43 = ::std::vec::from_elem(
                0,
                sflags_len.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
            );
            let mut f_0: *mut libc::c_char = fresh43.leak().as_mut_ptr()
                as *mut libc::c_char;
            memcpy(
                f_0 as *mut libc::c_void,
                shellflags as *const libc::c_void,
                sflags_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            argv = construct_command_argv_internal(
                f_0,
                0 as *mut *mut libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                flags,
                0 as *mut *mut libc::c_char,
            );
            if !argv.is_null() {
                let mut a: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
                a = argv;
                while !(*a).is_null() {
                    let fresh44 = n;
                    n = n + 1;
                    let ref mut fresh45 = *new_argv.offset(fresh44 as isize);
                    *fresh45 = nextp;
                    nextp = (stpcpy(nextp, *a)).offset(1 as libc::c_int as isize);
                    a = a.offset(1);
                    a;
                }
                free(*argv.offset(0 as libc::c_int as isize) as *mut libc::c_void);
                free(argv as *mut libc::c_void);
            }
        }
        let fresh46 = n;
        n = n + 1;
        let ref mut fresh47 = *new_argv.offset(fresh46 as isize);
        *fresh47 = nextp;
        memcpy(
            nextp as *mut libc::c_void,
            line as *const libc::c_void,
            line_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        let fresh48 = n;
        n = n + 1;
        let ref mut fresh49 = *new_argv.offset(fresh48 as isize);
        *fresh49 = 0 as *mut libc::c_char;
        return new_argv;
    }
    new_line = xmalloc(
        shell_len
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(sflags_len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(line_len.wrapping_mul(2 as libc::c_int as libc::c_ulong))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    ap = new_line;
    cp = shell;
    while *cp as libc::c_int != '\0' as i32 {
        if !(strchr(sh_chars, *cp as libc::c_int)).is_null() {
            let fresh50 = ap;
            ap = ap.offset(1);
            *fresh50 = '\\' as i32 as libc::c_char;
        }
        let fresh51 = ap;
        ap = ap.offset(1);
        *fresh51 = *cp;
        cp = cp.offset(1);
        cp;
    }
    let fresh52 = ap;
    ap = ap.offset(1);
    *fresh52 = ' ' as i32 as libc::c_char;
    if !shellflags.is_null() {
        ap = mempcpy(
            ap as *mut libc::c_void,
            shellflags as *const libc::c_void,
            sflags_len,
        ) as *mut libc::c_char;
        let fresh53 = ap;
        ap = ap.offset(1);
        *fresh53 = ' ' as i32 as libc::c_char;
    }
    p = line;
    while *p as libc::c_int != '\0' as i32 {
        if !restp.is_null() && *p as libc::c_int == '\n' as i32 {
            *restp = p;
            break;
        } else {
            if *p as libc::c_int == '\\' as i32
                && *p.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32
            {
                let fresh54 = ap;
                ap = ap.offset(1);
                *fresh54 = '\\' as i32 as libc::c_char;
                if batch_mode_shell == 0 {
                    let fresh55 = ap;
                    ap = ap.offset(1);
                    *fresh55 = '\\' as i32 as libc::c_char;
                }
                let fresh56 = ap;
                ap = ap.offset(1);
                *fresh56 = '\n' as i32 as libc::c_char;
                p = p.offset(1);
                p;
            } else {
                if unixy_shell != 0 && batch_mode_shell == 0
                    && (*p as libc::c_int == '\\' as i32
                        || *p as libc::c_int == '\'' as i32
                        || *p as libc::c_int == '"' as i32
                        || *stopchar_map
                            .as_mut_ptr()
                            .offset(*p as libc::c_uchar as isize) as libc::c_int
                            & (0x2 as libc::c_int | 0x4 as libc::c_int)
                            != 0 as libc::c_int
                        || !(strchr(sh_chars, *p as libc::c_int)).is_null())
                {
                    let fresh57 = ap;
                    ap = ap.offset(1);
                    *fresh57 = '\\' as i32 as libc::c_char;
                }
                let fresh58 = ap;
                ap = ap.offset(1);
                *fresh58 = *p;
            }
            p = p.offset(1);
            p;
        }
    }
    if ap
        == new_line
            .offset(shell_len as isize)
            .offset(sflags_len as isize)
            .offset(2 as libc::c_int as isize)
    {
        free(new_line as *mut libc::c_void);
        return 0 as *mut *mut libc::c_char;
    }
    *ap = '\0' as i32 as libc::c_char;
    if unixy_shell != 0 {
        new_argv = construct_command_argv_internal(
            new_line,
            0 as *mut *mut libc::c_char,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
            flags,
            0 as *mut *mut libc::c_char,
        );
    } else {
        fatal(
            0 as *mut floc,
            (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (53 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<uintmax_t>() as libc::c_ulong,
                        )
                        .wrapping_div(22 as libc::c_int as libc::c_ulong)
                        .wrapping_add(3 as libc::c_int as libc::c_ulong),
                ),
            dcgettext(
                0 as *const libc::c_char,
                b"%s (line %d) Bad shell context (!unixy && !batch_mode_shell)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            b"src/job.c\0" as *const u8 as *const libc::c_char,
            3688 as libc::c_int,
        );
    }
    free(new_line as *mut libc::c_void);
    return new_argv;
}
pub unsafe extern "C" fn construct_command_argv(
    mut line: *mut libc::c_char,
    mut restp: *mut *mut libc::c_char,
    mut file: *mut file,
    mut cmd_flags: libc::c_int,
    mut batch_filename: *mut *mut libc::c_char,
) -> *mut *mut libc::c_char {
    let mut shell: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ifs: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut shellflags: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut var: *mut variable = 0 as *mut variable;
    let mut save: libc::c_int = warn_undefined_variables_flag;
    warn_undefined_variables_flag = 0 as libc::c_int;
    shell = allocated_variable_expand_for_file(
        b"$(SHELL)\0" as *const u8 as *const libc::c_char,
        file,
    );
    var = lookup_variable_for_file(
        b".SHELLFLAGS\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        file,
    );
    if var.is_null() {
        shellflags = xstrdup(b"\0" as *const u8 as *const libc::c_char);
    } else if posix_pedantic != 0
        && (*var).origin() as libc::c_int == o_default as libc::c_int
    {
        shellflags = xstrdup(
            if cmd_flags & 4 as libc::c_int != 0 as libc::c_int {
                b"-c\0" as *const u8 as *const libc::c_char
            } else {
                b"-ec\0" as *const u8 as *const libc::c_char
            },
        );
    } else {
        shellflags = allocated_variable_expand_for_file((*var).value, file);
    }
    ifs = allocated_variable_expand_for_file(
        b"$(IFS)\0" as *const u8 as *const libc::c_char,
        file,
    );
    warn_undefined_variables_flag = save;
    argv = construct_command_argv_internal(
        line,
        restp,
        shell,
        shellflags,
        ifs,
        cmd_flags,
        batch_filename,
    );
    free(shell as *mut libc::c_void);
    free(shellflags as *mut libc::c_void);
    free(ifs as *mut libc::c_void);
    return argv;
}
