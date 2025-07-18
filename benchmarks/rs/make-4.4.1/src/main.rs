use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn putenv(__string: *mut libc::c_char) -> libc::c_int;
    static mut handling_fatal_signal: sig_atomic_t;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: size_t,
    ) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    static mut environ: *mut *mut libc::c_char;
    fn _exit(_: libc::c_int) -> !;
    fn ttyname(__fd: libc::c_int) -> *mut libc::c_char;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut opterr: libc::c_int;
    static mut optind: libc::c_int;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
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
    fn concat(_: libc::c_uint, _: ...) -> *const libc::c_char;
    fn error(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...) -> !;
    fn remote_setup();
    fn load_file(
        flocp: *const floc,
        file: *mut file,
        noerror: libc::c_int,
    ) -> libc::c_int;
    fn guile_gmake_setup(flocp: *const floc) -> libc::c_int;
    fn strcache_init();
    fn build_vpath_lists();
    fn install_default_implicit_rules();
    fn construct_include_path(arg_dirs: *mut *const libc::c_char);
    fn install_default_suffix_rules();
    fn set_default_suffixes();
    fn undefine_default_variables();
    fn define_default_variables();
    fn hash_init_directories();
    fn get_tmpfile(_: *mut *mut libc::c_char) -> *mut FILE;
    fn get_tmpdir() -> *const libc::c_char;
    fn strcache_print_stats(prefix: *const libc::c_char);
    fn print_vpath_data_base();
    fn pfatal_with_name(_: *const libc::c_char) -> !;
    fn print_dir_data_base();
    fn print_variable_data_base();
    static mut remote_description: *mut libc::c_char;
    static mut make_host: *mut libc::c_char;
    static mut version_string: *mut libc::c_char;
    fn remote_cleanup();
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn make_toui(_: *const libc::c_char, _: *mut *const libc::c_char) -> libc::c_uint;
    fn perror_with_name(_: *const libc::c_char, _: *const libc::c_char);
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t) -> *mut libc::c_void;
    fn strcache_add(str: *const libc::c_char) -> *const libc::c_char;
    fn check_io_state() -> libc::c_uint;
    fn jobserver_enabled() -> libc::c_uint;
    fn jobserver_setup(
        job_slots_0: libc::c_int,
        style: *const libc::c_char,
    ) -> libc::c_uint;
    fn jobserver_parse_auth(auth: *const libc::c_char) -> libc::c_uint;
    fn jobserver_get_auth() -> *mut libc::c_char;
    fn jobserver_clear();
    fn jobserver_acquire_all() -> libc::c_uint;
    fn jobserver_release(is_fatal: libc::c_int);
    fn jobserver_pre_child(_: libc::c_int);
    fn jobserver_post_child(_: libc::c_int);
    fn osync_setup();
    fn osync_get_mutex() -> *mut libc::c_char;
    fn osync_parse_mutex(mutex: *const libc::c_char) -> libc::c_uint;
    fn osync_clear();
    fn enter_file(name: *const libc::c_char) -> *mut file;
    fn init_hash_files();
    fn print_file_data_base();
    fn verify_file_data_base();
    fn lookup_file(name: *const libc::c_char) -> *mut file;
    fn remove_intermediates(sig: libc::c_int);
    fn snap_deps();
    fn f_mtime(file: *mut file, search: libc::c_int) -> uintmax_t;
    fn parse_file_seq(
        stringp: *mut *mut libc::c_char,
        size: size_t,
        stopmap: libc::c_int,
        prefix: *const libc::c_char,
        flags: libc::c_int,
    ) -> *mut libc::c_void;
    fn tilde_expand(name: *const libc::c_char) -> *mut libc::c_char;
    fn free_ns_chain(n: *mut nameseq);
    fn read_all_makefiles(makefiles_0: *mut *const libc::c_char) -> *mut goaldep;
    fn eval_buffer(buffer: *mut libc::c_char, floc: *const floc);
    fn update_goal_chain(goals_0: *mut goaldep) -> update_status;
    static mut variable_buffer: *mut libc::c_char;
    static mut current_variable_set_list: *mut variable_set_list;
    fn variable_buffer_output(
        ptr: *mut libc::c_char,
        string_0: *const libc::c_char,
        length: size_t,
    ) -> *mut libc::c_char;
    fn variable_expand(line: *const libc::c_char) -> *mut libc::c_char;
    fn initialize_variable_output() -> *mut libc::c_char;
    fn define_automatic_variables();
    fn try_variable_definition(
        flocp: *const floc,
        line: *const libc::c_char,
        origin: variable_origin,
        target_var: libc::c_int,
    ) -> *mut variable;
    fn init_hash_global_variable_set();
    fn hash_init_function_table();
    fn lookup_variable(name: *const libc::c_char, length: size_t) -> *mut variable;
    fn define_variable_in_set(
        name: *const libc::c_char,
        length: size_t,
        value: *const libc::c_char,
        origin: variable_origin,
        recursive: libc::c_int,
        set: *mut variable_set,
        flocp: *const floc,
    ) -> *mut variable;
    static mut job_slots_used: libc::c_uint;
    fn reap_children(block: libc::c_int, err: libc::c_int);
    fn exec_command(argv: *mut *mut libc::c_char, envp: *mut *mut libc::c_char) -> pid_t;
    static mut jobserver_tokens: libc::c_uint;
    fn child_handler(sig: libc::c_int);
    static mut output_context: *mut output;
    static mut stdio_traced: libc::c_uint;
    fn output_init(out: *mut output);
    fn output_close(out: *mut output);
    fn fatal_error_signal(sig: libc::c_int);
    static mut suffix_file: *mut file;
    fn snap_implicit_rules();
    fn convert_to_pattern();
    fn print_rule_data_base();
    fn getopt_long(
        argc: libc::c_int,
        argv: *const *mut libc::c_char,
        shortopts: *const libc::c_char,
        longopts: *const option,
        longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn shuffle_set_mode(cmdarg: *const libc::c_char);
    fn shuffle_get_mode() -> *const libc::c_char;
    fn shuffle_deps_recursive(g: *mut dep);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uintmax_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
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
pub type C2RustUnnamed_10 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_10 = 8;
pub const _ISpunct: C2RustUnnamed_10 = 4;
pub const _IScntrl: C2RustUnnamed_10 = 2;
pub const _ISblank: C2RustUnnamed_10 = 1;
pub const _ISgraph: C2RustUnnamed_10 = 32768;
pub const _ISprint: C2RustUnnamed_10 = 16384;
pub const _ISspace: C2RustUnnamed_10 = 8192;
pub const _ISxdigit: C2RustUnnamed_10 = 4096;
pub const _ISdigit: C2RustUnnamed_10 = 2048;
pub const _ISalpha: C2RustUnnamed_10 = 1024;
pub const _ISlower: C2RustUnnamed_10 = 512;
pub const _ISupper: C2RustUnnamed_10 = 256;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stringlist {
    pub list: *mut *const libc::c_char,
    pub idx: libc::c_uint,
    pub max: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flag {
    pub next: *mut flag,
    pub cs: *const command_switch,
    pub arg: *const libc::c_char,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct command_switch {
    pub c: libc::c_int,
    pub type_0: C2RustUnnamed_11,
    pub value_ptr: *mut libc::c_void,
    #[bitfield(name = "env", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "toenv", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "no_makefile", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "specified", ty = "libc::c_uint", bits = "3..=3")]
    pub env_toenv_no_makefile_specified: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub noarg_value: *const libc::c_void,
    pub default_value: *const libc::c_void,
    pub long_name: *const libc::c_char,
    pub origin: *mut variable_origin,
}
pub type C2RustUnnamed_11 = libc::c_uint;
pub const ignore: C2RustUnnamed_11 = 7;
pub const floating: C2RustUnnamed_11 = 6;
pub const positive_int: C2RustUnnamed_11 = 5;
pub const filename: C2RustUnnamed_11 = 4;
pub const strlist: C2RustUnnamed_11 = 3;
pub const string: C2RustUnnamed_11 = 2;
pub const flag_off: C2RustUnnamed_11 = 1;
pub const flag: C2RustUnnamed_11 = 0;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct goaldep {
    pub next: *mut goaldep,
    pub name: *const libc::c_char,
    pub file: *mut file,
    pub shuf: *mut goaldep,
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
    pub c2rust_padding: [u8; 2],
    pub error: libc::c_int,
    pub floc: floc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct command_variable {
    pub next: *mut command_variable,
    pub variable: *mut variable,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const libc::c_char,
}
pub type bsd_signal_ret_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
pub static mut verify_flag: libc::c_int = 0;
static mut silent_flag: libc::c_int = 0;
static mut default_silent_flag: libc::c_int = 0 as libc::c_int;
static mut silent_origin: variable_origin = o_default;
pub static mut run_silent: libc::c_int = 0 as libc::c_int;
pub static mut touch_flag: libc::c_int = 0;
pub static mut just_print_flag: libc::c_int = 0;
static mut db_flags: *mut stringlist = 0 as *const stringlist as *mut stringlist;
static mut debug_flag: libc::c_int = 0 as libc::c_int;
pub static mut db_level: libc::c_int = 0 as libc::c_int;
pub static mut output_sync_option: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut env_overrides: libc::c_int = 0 as libc::c_int;
pub static mut ignore_errors_flag: libc::c_int = 0 as libc::c_int;
pub static mut print_data_base_flag: libc::c_int = 0 as libc::c_int;
pub static mut question_flag: libc::c_int = 0 as libc::c_int;
pub static mut no_builtin_rules_flag: libc::c_int = 0 as libc::c_int;
pub static mut no_builtin_variables_flag: libc::c_int = 0 as libc::c_int;
pub static mut keep_going_flag: libc::c_int = 0;
static mut default_keep_going_flag: libc::c_int = 0 as libc::c_int;
static mut keep_going_origin: variable_origin = o_default;
pub static mut check_symlink_flag: libc::c_int = 0 as libc::c_int;
static mut print_directory_flag: libc::c_int = -(1 as libc::c_int);
static mut default_print_directory_flag: libc::c_int = -(1 as libc::c_int);
static mut print_directory_origin: variable_origin = o_default;
pub static mut print_version_flag: libc::c_int = 0 as libc::c_int;
static mut makefiles: *mut stringlist = 0 as *const stringlist as *mut stringlist;
pub static mut job_slots: libc::c_uint = 0;
static mut master_job_slots: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut arg_job_slots: libc::c_int = -(1 as libc::c_int);
static mut default_job_slots: libc::c_int = -(1 as libc::c_int);
static mut inf_jobs: libc::c_int = 0 as libc::c_int;
pub static mut jobserver_auth: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut jobserver_style: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut shuffle_mode: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut sync_mutex: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut max_load_average: libc::c_double = -1.0f64;
pub static mut default_load_average: libc::c_double = -1.0f64;
static mut directories: *mut stringlist = 0 as *const stringlist as *mut stringlist;
static mut include_dirs: *mut stringlist = 0 as *const stringlist as *mut stringlist;
static mut old_files: *mut stringlist = 0 as *const stringlist as *mut stringlist;
static mut new_files: *mut stringlist = 0 as *const stringlist as *mut stringlist;
static mut eval_strings: *mut stringlist = 0 as *const stringlist as *mut stringlist;
static mut print_usage_flag: libc::c_int = 0 as libc::c_int;
pub static mut warn_undefined_variables_flag: libc::c_int = 0;
static mut always_make_set: libc::c_int = 0 as libc::c_int;
pub static mut always_make_flag: libc::c_int = 0 as libc::c_int;
pub static mut rebuilding_makefiles: libc::c_int = 0 as libc::c_int;
pub static mut shell_var: variable = variable {
    name: 0 as *const libc::c_char as *mut libc::c_char,
    value: 0 as *const libc::c_char as *mut libc::c_char,
    fileinfo: floc {
        filenm: 0 as *const libc::c_char,
        lineno: 0,
        offset: 0,
    },
    length: 0,
    recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
};
pub static mut cmd_prefix: libc::c_char = '\t' as i32 as libc::c_char;
pub static mut no_intermediates: libc::c_uint = 0;
pub static mut command_count: libc::c_ulong = 1 as libc::c_int as libc::c_ulong;
static mut stdin_offset: libc::c_int = -(1 as libc::c_int);
static mut usage: [*const libc::c_char; 36] = [
    b"Options:\n\0" as *const u8 as *const libc::c_char,
    b"  -b, -m                      Ignored for compatibility.\n\0" as *const u8
        as *const libc::c_char,
    b"  -B, --always-make           Unconditionally make all targets.\n\0" as *const u8
        as *const libc::c_char,
    b"  -C DIRECTORY, --directory=DIRECTORY\n                              Change to DIRECTORY before doing anything.\n\0"
        as *const u8 as *const libc::c_char,
    b"  -d                          Print lots of debugging information.\n\0"
        as *const u8 as *const libc::c_char,
    b"  --debug[=FLAGS]             Print various types of debugging information.\n\0"
        as *const u8 as *const libc::c_char,
    b"  -e, --environment-overrides\n                              Environment variables override makefiles.\n\0"
        as *const u8 as *const libc::c_char,
    b"  -E STRING, --eval=STRING    Evaluate STRING as a makefile statement.\n\0"
        as *const u8 as *const libc::c_char,
    b"  -f FILE, --file=FILE, --makefile=FILE\n                              Read FILE as a makefile.\n\0"
        as *const u8 as *const libc::c_char,
    b"  -h, --help                  Print this message and exit.\n\0" as *const u8
        as *const libc::c_char,
    b"  -i, --ignore-errors         Ignore errors from recipes.\n\0" as *const u8
        as *const libc::c_char,
    b"  -I DIRECTORY, --include-dir=DIRECTORY\n                              Search DIRECTORY for included makefiles.\n\0"
        as *const u8 as *const libc::c_char,
    b"  -j [N], --jobs[=N]          Allow N jobs at once; infinite jobs with no arg.\n\0"
        as *const u8 as *const libc::c_char,
    b"  --jobserver-style=STYLE     Select the style of jobserver to use.\n\0"
        as *const u8 as *const libc::c_char,
    b"  -k, --keep-going            Keep going when some targets can't be made.\n\0"
        as *const u8 as *const libc::c_char,
    b"  -l [N], --load-average[=N], --max-load[=N]\n                              Don't start multiple jobs unless load is below N.\n\0"
        as *const u8 as *const libc::c_char,
    b"  -L, --check-symlink-times   Use the latest mtime between symlinks and target.\n\0"
        as *const u8 as *const libc::c_char,
    b"  -n, --just-print, --dry-run, --recon\n                              Don't actually run any recipe; just print them.\n\0"
        as *const u8 as *const libc::c_char,
    b"  -o FILE, --old-file=FILE, --assume-old=FILE\n                              Consider FILE to be very old and don't remake it.\n\0"
        as *const u8 as *const libc::c_char,
    b"  -O[TYPE], --output-sync[=TYPE]\n                              Synchronize output of parallel jobs by TYPE.\n\0"
        as *const u8 as *const libc::c_char,
    b"  -p, --print-data-base       Print make's internal database.\n\0" as *const u8
        as *const libc::c_char,
    b"  -q, --question              Run no recipe; exit status says if up to date.\n\0"
        as *const u8 as *const libc::c_char,
    b"  -r, --no-builtin-rules      Disable the built-in implicit rules.\n\0"
        as *const u8 as *const libc::c_char,
    b"  -R, --no-builtin-variables  Disable the built-in variable settings.\n\0"
        as *const u8 as *const libc::c_char,
    b"  --shuffle[={SEED|random|reverse|none}]\n                              Perform shuffle of prerequisites and goals.\n\0"
        as *const u8 as *const libc::c_char,
    b"  -s, --silent, --quiet       Don't echo recipes.\n\0" as *const u8
        as *const libc::c_char,
    b"  --no-silent                 Echo recipes (disable --silent mode).\n\0"
        as *const u8 as *const libc::c_char,
    b"  -S, --no-keep-going, --stop\n                              Turns off -k.\n\0"
        as *const u8 as *const libc::c_char,
    b"  -t, --touch                 Touch targets instead of remaking them.\n\0"
        as *const u8 as *const libc::c_char,
    b"  --trace                     Print tracing information.\n\0" as *const u8
        as *const libc::c_char,
    b"  -v, --version               Print the version number of make and exit.\n\0"
        as *const u8 as *const libc::c_char,
    b"  -w, --print-directory       Print the current directory.\n\0" as *const u8
        as *const libc::c_char,
    b"  --no-print-directory        Turn off -w, even if it was turned on implicitly.\n\0"
        as *const u8 as *const libc::c_char,
    b"  -W FILE, --what-if=FILE, --new-file=FILE, --assume-new=FILE\n                              Consider FILE to be infinitely new.\n\0"
        as *const u8 as *const libc::c_char,
    b"  --warn-undefined-variables  Warn when an undefined variable is referenced.\n\0"
        as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut trace_flag: libc::c_int = 0 as libc::c_int;
static mut switches: [command_switch; 40] = [command_switch {
    c: 0,
    type_0: flag,
    value_ptr: 0 as *mut libc::c_void,
    env_toenv_no_makefile_specified: [0; 1],
    c2rust_padding: [0; 7],
    noarg_value: 0 as *const libc::c_void,
    default_value: 0 as *const libc::c_void,
    long_name: 0 as *const libc::c_char,
    origin: 0 as *mut variable_origin,
}; 40];
static mut long_option_aliases: [option; 9] = [
    {
        let mut init = option {
            name: b"quiet\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"stop\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"new-file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'W' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"assume-new\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'W' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"assume-old\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"max-load\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"dry-run\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"recon\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"makefile\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
];
static mut goals: *mut goaldep = 0 as *const goaldep as *mut goaldep;
static mut lastgoal: *mut goaldep = 0 as *const goaldep as *mut goaldep;
static mut command_variables: *mut command_variable = 0 as *const command_variable
    as *mut command_variable;
pub static mut program: *const libc::c_char = 0 as *const libc::c_char;
pub static mut directory_before_chdir: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut starting_directory: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut makelevel: libc::c_uint = 0;
pub static mut default_goal_var: *mut variable = 0 as *const variable as *mut variable;
pub static mut default_file: *mut file = 0 as *const file as *mut file;
pub static mut posix_pedantic: libc::c_int = 0;
pub static mut second_expansion: libc::c_int = 0;
pub static mut one_shell: libc::c_int = 0;
pub static mut output_sync: libc::c_int = 0 as libc::c_int;
pub static mut not_parallel: libc::c_int = 0;
pub static mut clock_skew_detected: libc::c_int = 0;
pub static mut stopchar_map: [libc::c_ushort; 256] = [
    0 as libc::c_int as libc::c_ushort,
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
pub static mut make_sync: output = output {
    out: 0,
    err: 0,
    syncout: [0; 1],
    c2rust_padding: [0; 3],
};
pub static mut fatal_signal_set: sigset_t = sigset_t { __val: [0; 16] };
unsafe extern "C" fn bsd_signal(
    mut sig: libc::c_int,
    mut func: bsd_signal_ret_t,
) -> bsd_signal_ret_t {
    let mut act: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut oact: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    act.__sigaction_handler.sa_handler = func;
    act.sa_flags = 0x10000000 as libc::c_int;
    sigemptyset(&mut act.sa_mask);
    sigaddset(&mut act.sa_mask, sig);
    if sigaction(sig, &mut act, &mut oact) != 0 as libc::c_int {
        return ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(-(1 as libc::c_int) as libc::intptr_t);
    }
    return oact.__sigaction_handler.sa_handler;
}
unsafe extern "C" fn initialize_global_hash_tables() {
    init_hash_global_variable_set();
    strcache_init();
    init_hash_files();
    hash_init_directories();
    hash_init_function_table();
}
unsafe extern "C" fn initialize_stopchar_map() {
    let mut i: libc::c_int = 0;
    stopchar_map['\0' as i32 as usize] = 0x1 as libc::c_int as libc::c_ushort;
    stopchar_map['#' as i32 as usize] = 0x8 as libc::c_int as libc::c_ushort;
    stopchar_map[';' as i32 as usize] = 0x10 as libc::c_int as libc::c_ushort;
    stopchar_map['=' as i32 as usize] = 0x20 as libc::c_int as libc::c_ushort;
    stopchar_map[':' as i32 as usize] = 0x40 as libc::c_int as libc::c_ushort;
    stopchar_map['|' as i32 as usize] = 0x100 as libc::c_int as libc::c_ushort;
    stopchar_map['.' as i32
        as usize] = (0x200 as libc::c_int | 0x2000 as libc::c_int) as libc::c_ushort;
    stopchar_map[',' as i32 as usize] = 0x400 as libc::c_int as libc::c_ushort;
    stopchar_map['(' as i32 as usize] = 0x80 as libc::c_int as libc::c_ushort;
    stopchar_map['{' as i32 as usize] = 0x80 as libc::c_int as libc::c_ushort;
    stopchar_map['}' as i32 as usize] = 0x80 as libc::c_int as libc::c_ushort;
    stopchar_map[')' as i32 as usize] = 0x80 as libc::c_int as libc::c_ushort;
    stopchar_map['$' as i32 as usize] = 0x4000 as libc::c_int as libc::c_ushort;
    stopchar_map['-' as i32 as usize] = 0x2000 as libc::c_int as libc::c_ushort;
    stopchar_map['_' as i32 as usize] = 0x2000 as libc::c_int as libc::c_ushort;
    stopchar_map[' ' as i32 as usize] = 0x2 as libc::c_int as libc::c_ushort;
    stopchar_map['\t' as i32 as usize] = 0x2 as libc::c_int as libc::c_ushort;
    stopchar_map['/' as i32 as usize] = 0x8000 as libc::c_int as libc::c_ushort;
    i = 1 as libc::c_int;
    while i <= 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int {
        if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            && !(stopchar_map[i as usize] as libc::c_int & 0x2 as libc::c_int
                != 0 as libc::c_int)
        {
            stopchar_map[i
                as usize] = (stopchar_map[i as usize] as libc::c_int
                | 0x4 as libc::c_int) as libc::c_ushort;
        } else if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            stopchar_map[i
                as usize] = (stopchar_map[i as usize] as libc::c_int
                | 0x2000 as libc::c_int) as libc::c_ushort;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn close_stdout() {
    let mut prev_fail: libc::c_int = ferror(stdout);
    let mut fclose_fail: libc::c_int = fclose(stdout);
    if prev_fail != 0 || fclose_fail != 0 {
        if fclose_fail != 0 {
            perror_with_name(
                dcgettext(
                    0 as *const libc::c_char,
                    b"write error: stdout\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else {
            error(
                0 as *mut floc,
                0 as libc::c_int as size_t,
                dcgettext(
                    0 as *const libc::c_char,
                    b"write error: stdout\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn expand_command_line_file(
    mut name: *const libc::c_char,
) -> *const libc::c_char {
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        fatal(
            0 as *mut floc,
            0 as libc::c_int as size_t,
            dcgettext(
                0 as *const libc::c_char,
                b"empty string invalid as file name\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == '~' as i32 {
        expanded = tilde_expand(name);
        if !expanded.is_null()
            && *expanded.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
        {
            name = expanded;
        }
    }
    while *name.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
        && *name.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        name = name.offset(2 as libc::c_int as isize);
        while *name.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
            name = name.offset(1);
            name;
        }
    }
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        name = b"./\0" as *const u8 as *const libc::c_char;
    }
    cp = strcache_add(name);
    free(expanded as *mut libc::c_void);
    return cp;
}
unsafe extern "C" fn debug_signal_handler(mut sig: libc::c_int) {
    db_level = if db_level != 0 { 0 as libc::c_int } else { 0x1 as libc::c_int };
}
unsafe extern "C" fn decode_debug_flags() {
    let mut pp: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    if debug_flag != 0 {
        db_level = 0xfff as libc::c_int;
    }
    if trace_flag != 0 {
        db_level |= 0x10 as libc::c_int | 0x20 as libc::c_int;
    }
    if !db_flags.is_null() {
        pp = (*db_flags).list;
        while !(*pp).is_null() {
            let mut p: *const libc::c_char = *pp;
            loop {
                match ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *p
                                .offset(0 as libc::c_int as isize) as libc::c_int;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = tolower(
                                *p.offset(0 as libc::c_int as isize) as libc::c_int,
                            );
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(
                                *p.offset(0 as libc::c_int as isize) as libc::c_int as isize,
                            );
                    }
                    __res
                }) {
                    97 => {
                        db_level |= 0xfff as libc::c_int;
                    }
                    98 => {
                        db_level |= 0x1 as libc::c_int;
                    }
                    105 => {
                        db_level |= 0x1 as libc::c_int | 0x8 as libc::c_int;
                    }
                    106 => {
                        db_level |= 0x4 as libc::c_int;
                    }
                    109 => {
                        db_level |= 0x1 as libc::c_int | 0x100 as libc::c_int;
                    }
                    110 => {
                        db_level = 0 as libc::c_int;
                    }
                    112 => {
                        db_level |= 0x10 as libc::c_int;
                    }
                    118 => {
                        db_level |= 0x1 as libc::c_int | 0x2 as libc::c_int;
                    }
                    119 => {
                        db_level |= 0x20 as libc::c_int;
                    }
                    _ => {
                        fatal(
                            0 as *mut floc,
                            strlen(p),
                            dcgettext(
                                0 as *const libc::c_char,
                                b"unknown debug level specification '%s'\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            p,
                        );
                    }
                }
                loop {
                    p = p.offset(1);
                    if !(*p as libc::c_int != '\0' as i32) {
                        break;
                    }
                    if !(*p as libc::c_int == ',' as i32
                        || *p as libc::c_int == ' ' as i32)
                    {
                        continue;
                    }
                    p = p.offset(1);
                    p;
                    break;
                }
                if *p as libc::c_int == '\0' as i32 {
                    break;
                }
            }
            pp = pp.offset(1);
            pp;
        }
    }
    if db_level != 0 {
        verify_flag = 1 as libc::c_int;
    }
    if db_level == 0 {
        debug_flag = 0 as libc::c_int;
    }
}
unsafe extern "C" fn decode_output_sync_flags() {
    if !output_sync_option.is_null() {
        if output_sync_option
            == b"none\0" as *const u8 as *const libc::c_char as *mut libc::c_char
            || *output_sync_option as libc::c_int
                == *(b"none\0" as *const u8 as *const libc::c_char) as libc::c_int
                && (*output_sync_option as libc::c_int == '\0' as i32
                    || strcmp(
                        output_sync_option.offset(1 as libc::c_int as isize),
                        (b"none\0" as *const u8 as *const libc::c_char)
                            .offset(1 as libc::c_int as isize),
                    ) == 0)
        {
            output_sync = 0 as libc::c_int;
        } else if output_sync_option
            == b"line\0" as *const u8 as *const libc::c_char as *mut libc::c_char
            || *output_sync_option as libc::c_int
                == *(b"line\0" as *const u8 as *const libc::c_char) as libc::c_int
                && (*output_sync_option as libc::c_int == '\0' as i32
                    || strcmp(
                        output_sync_option.offset(1 as libc::c_int as isize),
                        (b"line\0" as *const u8 as *const libc::c_char)
                            .offset(1 as libc::c_int as isize),
                    ) == 0)
        {
            output_sync = 1 as libc::c_int;
        } else if output_sync_option
            == b"target\0" as *const u8 as *const libc::c_char as *mut libc::c_char
            || *output_sync_option as libc::c_int
                == *(b"target\0" as *const u8 as *const libc::c_char) as libc::c_int
                && (*output_sync_option as libc::c_int == '\0' as i32
                    || strcmp(
                        output_sync_option.offset(1 as libc::c_int as isize),
                        (b"target\0" as *const u8 as *const libc::c_char)
                            .offset(1 as libc::c_int as isize),
                    ) == 0)
        {
            output_sync = 2 as libc::c_int;
        } else if output_sync_option
            == b"recurse\0" as *const u8 as *const libc::c_char as *mut libc::c_char
            || *output_sync_option as libc::c_int
                == *(b"recurse\0" as *const u8 as *const libc::c_char) as libc::c_int
                && (*output_sync_option as libc::c_int == '\0' as i32
                    || strcmp(
                        output_sync_option.offset(1 as libc::c_int as isize),
                        (b"recurse\0" as *const u8 as *const libc::c_char)
                            .offset(1 as libc::c_int as isize),
                    ) == 0)
        {
            output_sync = 3 as libc::c_int;
        } else {
            fatal(
                0 as *mut floc,
                strlen(output_sync_option),
                dcgettext(
                    0 as *const libc::c_char,
                    b"unknown output-sync type '%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                output_sync_option,
            );
        }
    }
    if !sync_mutex.is_null() {
        osync_parse_mutex(sync_mutex);
    }
}
unsafe extern "C" fn print_usage(mut bad: libc::c_int) -> ! {
    let mut cpp: *const *const libc::c_char = 0 as *const *const libc::c_char;
    let mut usageto: *mut FILE = 0 as *mut FILE;
    if print_version_flag != 0 {
        print_version();
        fputs(b"\n\0" as *const u8 as *const libc::c_char, stdout);
    }
    usageto = if bad != 0 { stderr } else { stdout };
    fprintf(
        usageto,
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: %s [options] [target] ...\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        program,
    );
    cpp = usage.as_ptr();
    while !(*cpp).is_null() {
        fputs(dcgettext(0 as *const libc::c_char, *cpp, 5 as libc::c_int), usageto);
        cpp = cpp.offset(1);
        cpp;
    }
    if remote_description.is_null() || *remote_description as libc::c_int == '\0' as i32
    {
        fprintf(
            usageto,
            dcgettext(
                0 as *const libc::c_char,
                b"\nThis program built for %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            make_host,
        );
    } else {
        fprintf(
            usageto,
            dcgettext(
                0 as *const libc::c_char,
                b"\nThis program built for %s (%s)\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            make_host,
            remote_description,
        );
    }
    fprintf(
        usageto,
        dcgettext(
            0 as *const libc::c_char,
            b"Report bugs to <bug-make@gnu.org>\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    die(if bad != 0 { 2 as libc::c_int } else { 0 as libc::c_int });
}
unsafe extern "C" fn reset_jobserver() {
    jobserver_clear();
    free(jobserver_auth as *mut libc::c_void);
    jobserver_auth = 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn temp_stdin_unlink() {
    if stdin_offset >= 0 as libc::c_int {
        let mut nm: *const libc::c_char = *((*makefiles).list)
            .offset(stdin_offset as isize);
        let mut r: libc::c_int = 0 as libc::c_int;
        stdin_offset = -(1 as libc::c_int);
        loop {
            r = unlink(nm);
            if !(r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if r < 0 as libc::c_int && *__errno_location() != 2 as libc::c_int
            && handling_fatal_signal == 0
        {
            perror_with_name(
                dcgettext(
                    0 as *const libc::c_char,
                    b"unlink (temporary file): \0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                nm,
            );
        }
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut envp: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut makefile_status: libc::c_int = 0 as libc::c_int;
    let mut read_files: *mut goaldep = 0 as *mut goaldep;
    let mut current_directory: [libc::c_char; 4097] = [0; 4097];
    let mut restarts: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut syncing: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut argv_slots: libc::c_int = 0;
    initialize_variable_output();
    if check_io_state() & 0x8 as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    }
    output_init(&mut make_sync);
    initialize_stopchar_map();
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"make\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"make\0" as *const u8 as *const libc::c_char);
    sigemptyset(&mut fatal_signal_set);
    if bsd_signal(
        1 as libc::c_int,
        Some(fatal_error_signal as unsafe extern "C" fn(libc::c_int) -> ()),
    )
        == ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t)
    {
        bsd_signal(
            1 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
    } else {
        sigaddset(&mut fatal_signal_set, 1 as libc::c_int);
    }
    if bsd_signal(
        3 as libc::c_int,
        Some(fatal_error_signal as unsafe extern "C" fn(libc::c_int) -> ()),
    )
        == ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t)
    {
        bsd_signal(
            3 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
    } else {
        sigaddset(&mut fatal_signal_set, 3 as libc::c_int);
    }
    if bsd_signal(
        13 as libc::c_int,
        Some(fatal_error_signal as unsafe extern "C" fn(libc::c_int) -> ()),
    )
        == ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t)
    {
        bsd_signal(
            13 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
    } else {
        sigaddset(&mut fatal_signal_set, 13 as libc::c_int);
    }
    if bsd_signal(
        2 as libc::c_int,
        Some(fatal_error_signal as unsafe extern "C" fn(libc::c_int) -> ()),
    )
        == ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t)
    {
        bsd_signal(
            2 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
    } else {
        sigaddset(&mut fatal_signal_set, 2 as libc::c_int);
    }
    if bsd_signal(
        15 as libc::c_int,
        Some(fatal_error_signal as unsafe extern "C" fn(libc::c_int) -> ()),
    )
        == ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t)
    {
        bsd_signal(
            15 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
    } else {
        sigaddset(&mut fatal_signal_set, 15 as libc::c_int);
    }
    if bsd_signal(
        24 as libc::c_int,
        Some(fatal_error_signal as unsafe extern "C" fn(libc::c_int) -> ()),
    )
        == ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t)
    {
        bsd_signal(
            24 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
    } else {
        sigaddset(&mut fatal_signal_set, 24 as libc::c_int);
    }
    if bsd_signal(
        25 as libc::c_int,
        Some(fatal_error_signal as unsafe extern "C" fn(libc::c_int) -> ()),
    )
        == ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t)
    {
        bsd_signal(
            25 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
    } else {
        sigaddset(&mut fatal_signal_set, 25 as libc::c_int);
    }
    bsd_signal(17 as libc::c_int, None);
    output_init(0 as *mut output);
    if (*argv.offset(0 as libc::c_int as isize)).is_null() {
        let ref mut fresh0 = *argv.offset(0 as libc::c_int as isize);
        *fresh0 = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if *(*argv.offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
        as libc::c_int == '\0' as i32
    {
        program = b"make\0" as *const u8 as *const libc::c_char;
    } else {
        program = strrchr(*argv.offset(0 as libc::c_int as isize), '/' as i32);
        if program.is_null() {
            program = *argv.offset(0 as libc::c_int as isize);
        } else {
            program = program.offset(1);
            program;
        }
    }
    initialize_global_hash_tables();
    get_tmpdir();
    if (getcwd(current_directory.as_mut_ptr(), 4096 as libc::c_int as size_t)).is_null()
    {
        perror_with_name(
            b"getcwd\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
        current_directory[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        directory_before_chdir = 0 as *mut libc::c_char;
    } else {
        directory_before_chdir = xstrdup(current_directory.as_mut_ptr());
    }
    let ref mut fresh1 = *define_variable_in_set(
        b".VARIABLES\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"\0" as *const u8 as *const libc::c_char,
        o_default,
        0 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    (*fresh1).set_special(1 as libc::c_int as libc::c_uint);
    let ref mut fresh2 = *define_variable_in_set(
        b".RECIPEPREFIX\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"\0" as *const u8 as *const libc::c_char,
        o_default,
        0 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    (*fresh2).set_special(1 as libc::c_int as libc::c_uint);
    define_variable_in_set(
        b".SHELLFLAGS\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"-c\0" as *const u8 as *const libc::c_char,
        o_default,
        0 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b".LOADED\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"\0" as *const u8 as *const libc::c_char,
        o_default,
        0 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    let mut features: *const libc::c_char = b"target-specific order-only second-expansion else-if shortest-stem undefine oneshell nocomment grouped-target extra-prereqs notintermediate shell-export archives jobserver jobserver-fifo output-sync check-symlink load\0"
        as *const u8 as *const libc::c_char;
    define_variable_in_set(
        b".FEATURES\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        features,
        o_default,
        0 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    guile_gmake_setup(0 as *mut floc);
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while !(*envp.offset(i as isize)).is_null() {
        let mut v: *mut variable = 0 as *mut variable;
        let mut ep: *const libc::c_char = *envp.offset(i as isize);
        let mut export: variable_export = v_export;
        let mut len: size_t = 0;
        while !(stopchar_map[*ep as libc::c_uchar as usize] as libc::c_int
            & (0x20 as libc::c_int | 0x1 as libc::c_int) != 0 as libc::c_int)
        {
            ep = ep.offset(1);
            ep;
        }
        if !(*ep as libc::c_int == '\0' as i32) {
            let fresh3 = ep;
            ep = ep.offset(1);
            len = fresh3.offset_from(*envp.offset(i as isize)) as libc::c_long as size_t;
            if len == 13 as libc::c_int as libc::c_ulong
                && memcmp(
                    *envp.offset(i as isize) as *const libc::c_void,
                    b"MAKE_RESTARTS\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) == 0 as libc::c_int
            {
                if *ep as libc::c_int == '-' as i32 {
                    stdio_traced = 1 as libc::c_int as libc::c_uint;
                    ep = ep.offset(1);
                    ep;
                }
                restarts = make_toui(ep, 0 as *mut *const libc::c_char);
                export = v_noexport;
            }
            v = define_variable_in_set(
                *envp.offset(i as isize),
                len,
                ep,
                o_env,
                1 as libc::c_int,
                (*current_variable_set_list).set,
                0 as *mut floc,
            );
            if (*v).name
                == b"SHELL\0" as *const u8 as *const libc::c_char as *mut libc::c_char
                || *(*v).name as libc::c_int
                    == *(b"SHELL\0" as *const u8 as *const libc::c_char) as libc::c_int
                    && (*(*v).name as libc::c_int == '\0' as i32
                        || strcmp(
                            ((*v).name).offset(1 as libc::c_int as isize),
                            (b"SHELL\0" as *const u8 as *const libc::c_char)
                                .offset(1 as libc::c_int as isize),
                        ) == 0)
            {
                export = v_noexport;
                shell_var.name = xstrdup(b"SHELL\0" as *const u8 as *const libc::c_char);
                shell_var.length = 5 as libc::c_int as libc::c_uint;
                shell_var.value = xstrdup(ep);
            }
            (*v).set_export(export);
        }
        i = i.wrapping_add(1);
        i;
    }
    if !(lookup_variable(
        b"GNUMAKEFLAGS\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ))
        .is_null()
    {
        decode_env_switches(
            b"GNUMAKEFLAGS\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            o_command,
        );
        define_variable_in_set(
            b"GNUMAKEFLAGS\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            b"\0" as *const u8 as *const libc::c_char,
            o_env,
            0 as libc::c_int,
            (*current_variable_set_list).set,
            0 as *mut floc,
        );
    }
    decode_env_switches(
        b"MAKEFLAGS\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        o_command,
    );
    make_sync
        .set_syncout(
            (output_sync == 1 as libc::c_int || output_sync == 2 as libc::c_int)
                as libc::c_int as libc::c_uint,
        );
    syncing = make_sync.syncout();
    output_context = if make_sync.syncout() as libc::c_int != 0 {
        &mut make_sync
    } else {
        0 as *mut output
    };
    let mut env_slots: libc::c_int = arg_job_slots;
    arg_job_slots = -(1 as libc::c_int);
    decode_switches(argc, argv as *mut *const libc::c_char, o_command);
    argv_slots = arg_job_slots;
    if arg_job_slots == -(1 as libc::c_int) {
        arg_job_slots = env_slots;
    }
    if print_usage_flag != 0 {
        print_usage(0 as libc::c_int);
    }
    if print_version_flag != 0 {
        print_version();
        die(0 as libc::c_int);
    }
    setvbuf(
        stdout,
        0 as *mut libc::c_char,
        1 as libc::c_int,
        8192 as libc::c_int as size_t,
    );
    if !shuffle_mode.is_null() {
        let mut effective_mode: *const libc::c_char = 0 as *const libc::c_char;
        shuffle_set_mode(shuffle_mode);
        free(shuffle_mode as *mut libc::c_void);
        effective_mode = shuffle_get_mode();
        if !effective_mode.is_null() {
            shuffle_mode = xstrdup(effective_mode);
        } else {
            shuffle_mode = 0 as *mut libc::c_char;
        }
    }
    if isatty(fileno(stdout)) != 0 {
        if (lookup_variable(
            b"MAKE_TERMOUT\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ))
            .is_null()
        {
            let mut tty: *const libc::c_char = ttyname(fileno(stdout));
            let ref mut fresh4 = *define_variable_in_set(
                b"MAKE_TERMOUT\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                if !tty.is_null() {
                    tty
                } else {
                    b"true\0" as *const u8 as *const libc::c_char
                },
                o_default,
                0 as libc::c_int,
                (*current_variable_set_list).set,
                0 as *mut floc,
            );
            (*fresh4).set_export(v_export);
        }
    }
    if isatty(fileno(stderr)) != 0 {
        if (lookup_variable(
            b"MAKE_TERMERR\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ))
            .is_null()
        {
            let mut tty_0: *const libc::c_char = ttyname(fileno(stderr));
            let ref mut fresh5 = *define_variable_in_set(
                b"MAKE_TERMERR\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                if !tty_0.is_null() {
                    tty_0
                } else {
                    b"true\0" as *const u8 as *const libc::c_char
                },
                o_default,
                0 as libc::c_int,
                (*current_variable_set_list).set,
                0 as *mut floc,
            );
            (*fresh5).set_export(v_export);
        }
    }
    syncing = (output_sync == 1 as libc::c_int || output_sync == 2 as libc::c_int)
        as libc::c_int as libc::c_uint;
    if make_sync.syncout() as libc::c_int != 0 && syncing == 0 {
        output_close(&mut make_sync);
    }
    make_sync.set_syncout(syncing);
    output_context = if make_sync.syncout() as libc::c_int != 0 {
        &mut make_sync
    } else {
        0 as *mut output
    };
    let mut v_0: *mut variable = lookup_variable(
        b"MAKELEVEL\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    if !v_0.is_null()
        && *((*v_0).value).offset(0 as libc::c_int as isize) as libc::c_int
            != '\0' as i32
        && *((*v_0).value).offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
    {
        makelevel = make_toui((*v_0).value, 0 as *mut *const libc::c_char);
    } else {
        makelevel = 0 as libc::c_int as libc::c_uint;
    }
    always_make_flag = (always_make_set != 0
        && restarts == 0 as libc::c_int as libc::c_uint) as libc::c_int;
    if no_builtin_variables_flag != 0 {
        no_builtin_rules_flag = 1 as libc::c_int;
    }
    if 0x1 as libc::c_int & db_level != 0 {
        print_version();
        fflush(stdout);
    }
    if current_directory[0 as libc::c_int as usize] as libc::c_int != '\0' as i32
        && !(*argv.offset(0 as libc::c_int as isize)).is_null()
        && *(*argv.offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int != '/' as i32
        && !(strchr(*argv.offset(0 as libc::c_int as isize), '/' as i32)).is_null()
    {
        let ref mut fresh6 = *argv.offset(0 as libc::c_int as isize);
        *fresh6 = xstrdup(
            concat(
                3 as libc::c_int as libc::c_uint,
                current_directory.as_mut_ptr(),
                b"/\0" as *const u8 as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize),
            ),
        );
    }
    starting_directory = current_directory.as_mut_ptr();
    if !directories.is_null() {
        let mut i_0: libc::c_uint = 0;
        i_0 = 0 as libc::c_int as libc::c_uint;
        while !(*((*directories).list).offset(i_0 as isize)).is_null() {
            let mut dir: *const libc::c_char = *((*directories).list)
                .offset(i_0 as isize);
            if chdir(dir) < 0 as libc::c_int {
                pfatal_with_name(dir);
            }
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
    }
    if !directories.is_null() {
        if (getcwd(current_directory.as_mut_ptr(), 4096 as libc::c_int as size_t))
            .is_null()
        {
            perror_with_name(
                b"getcwd\0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
            );
            starting_directory = 0 as *mut libc::c_char;
        } else {
            starting_directory = current_directory.as_mut_ptr();
        }
    }
    define_variable_in_set(
        b"CURDIR\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        current_directory.as_mut_ptr(),
        o_file,
        0 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    construct_include_path(
        if !include_dirs.is_null() {
            (*include_dirs).list
        } else {
            0 as *mut *const libc::c_char
        },
    );
    if !jobserver_auth.is_null() {
        if argv_slots == -(1 as libc::c_int) {
            if jobserver_parse_auth(jobserver_auth) != 0 {
                current_block = 17252238883942993517;
            } else {
                error(
                    0 as *mut floc,
                    0 as libc::c_int as size_t,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"warning: jobserver unavailable: using -j1.  Add '+' to parent make rule.\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                arg_job_slots = 1 as libc::c_int;
                current_block = 7157669805658135323;
            }
        } else {
            if restarts == 0 {
                error(
                    0 as *mut floc,
                    (53 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<uintmax_t>() as libc::c_ulong,
                        )
                        .wrapping_div(22 as libc::c_int as libc::c_ulong)
                        .wrapping_add(3 as libc::c_int as libc::c_ulong),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"warning: -j%d forced in submake: resetting jobserver mode.\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    argv_slots,
                );
            }
            current_block = 7157669805658135323;
        }
        match current_block {
            17252238883942993517 => {}
            _ => {
                reset_jobserver();
            }
        }
    }
    define_variable_in_set(
        b"MAKE_COMMAND\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        *argv.offset(0 as libc::c_int as isize),
        o_default,
        0 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"MAKE\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"$(MAKE_COMMAND)\0" as *const u8 as *const libc::c_char,
        o_default,
        1 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    if !command_variables.is_null() {
        let mut cv: *mut command_variable = 0 as *mut command_variable;
        let mut v_1: *mut variable = 0 as *mut variable;
        let mut len_0: size_t = 0 as libc::c_int as size_t;
        let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        cv = command_variables;
        while !cv.is_null() {
            v_1 = (*cv).variable;
            len_0 = (len_0 as libc::c_ulong)
                .wrapping_add(
                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(strlen((*v_1).name)),
                ) as size_t as size_t;
            if (*v_1).recursive() == 0 {
                len_0 = len_0.wrapping_add(1);
                len_0;
            }
            len_0 = len_0.wrapping_add(1);
            len_0;
            len_0 = (len_0 as libc::c_ulong)
                .wrapping_add(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(strlen((*v_1).value)),
                ) as size_t as size_t;
            len_0 = len_0.wrapping_add(1);
            len_0;
            cv = (*cv).next;
        }
        let mut fresh7 = ::std::vec::from_elem(0, len_0 as usize);
        value = fresh7.leak().as_mut_ptr() as *mut libc::c_char;
        p = value;
        cv = command_variables;
        while !cv.is_null() {
            v_1 = (*cv).variable;
            p = quote_for_env(p, (*v_1).name);
            if (*v_1).recursive() == 0 {
                let fresh8 = p;
                p = p.offset(1);
                *fresh8 = ':' as i32 as libc::c_char;
            }
            let fresh9 = p;
            p = p.offset(1);
            *fresh9 = '=' as i32 as libc::c_char;
            p = quote_for_env(p, (*v_1).value);
            let fresh10 = p;
            p = p.offset(1);
            *fresh10 = ' ' as i32 as libc::c_char;
            cv = (*cv).next;
        }
        *p.offset(-(1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        define_variable_in_set(
            b"-*-command-variables-*-\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            value,
            o_automatic,
            0 as libc::c_int,
            (*current_variable_set_list).set,
            0 as *mut floc,
        );
        define_variable_in_set(
            b"MAKEOVERRIDES\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            b"${-*-command-variables-*-}\0" as *const u8 as *const libc::c_char,
            o_default,
            1 as libc::c_int,
            (*current_variable_set_list).set,
            0 as *mut floc,
        );
    }
    if !makefiles.is_null() {
        let mut i_1: libc::c_uint = 0;
        i_1 = 0 as libc::c_int as libc::c_uint;
        while i_1 < (*makefiles).idx {
            if *(*((*makefiles).list).offset(i_1 as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
                && *(*((*makefiles).list).offset(i_1 as isize))
                    .offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
            {
                let mut outfile: *mut FILE = 0 as *mut FILE;
                let mut newnm: *mut libc::c_char = 0 as *mut libc::c_char;
                if stdin_offset >= 0 as libc::c_int {
                    fatal(
                        0 as *mut floc,
                        0 as libc::c_int as size_t,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Makefile from standard input specified twice\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                outfile = get_tmpfile(&mut newnm);
                if outfile.is_null() {
                    fatal(
                        0 as *mut floc,
                        0 as libc::c_int as size_t,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot store makefile from stdin to a temporary file\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                while feof(stdin) == 0 && ferror(stdin) == 0 {
                    let mut buf: [libc::c_char; 2048] = [0; 2048];
                    let mut n: size_t = fread(
                        buf.as_mut_ptr() as *mut libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong,
                        stdin,
                    );
                    if n > 0 as libc::c_int as libc::c_ulong
                        && fwrite(
                            buf.as_mut_ptr() as *const libc::c_void,
                            1 as libc::c_int as libc::c_ulong,
                            n,
                            outfile,
                        ) != n
                    {
                        fatal(
                            0 as *mut floc,
                            (strlen(newnm))
                                .wrapping_add(strlen(strerror(*__errno_location()))),
                            dcgettext(
                                0 as *const libc::c_char,
                                b"fwrite: temporary file %s: %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            newnm,
                            strerror(*__errno_location()),
                        );
                    }
                }
                fclose(outfile);
                let ref mut fresh11 = *((*makefiles).list).offset(i_1 as isize);
                *fresh11 = strcache_add(newnm);
                stdin_offset = i_1 as libc::c_int;
                free(newnm as *mut libc::c_void);
            }
            i_1 = i_1.wrapping_add(1);
            i_1;
        }
    }
    if stdin_offset >= 0 as libc::c_int {
        let mut f: *mut file = enter_file(
            *((*makefiles).list).offset(stdin_offset as isize),
        );
        (*f).set_updated(1 as libc::c_int as libc::c_uint);
        (*f).set_update_status(us_success);
        (*f).set_command_state(cs_finished);
        (*f).set_intermediate(0 as libc::c_int as libc::c_uint);
        (*f).set_dontcare(0 as libc::c_int as libc::c_uint);
        (*f).mtime_before_update = f_mtime(f, 0 as libc::c_int);
        (*f).last_mtime = (*f).mtime_before_update;
    }
    bsd_signal(
        17 as libc::c_int,
        Some(child_handler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    let mut block: sigset_t = sigset_t { __val: [0; 16] };
    sigemptyset(&mut block);
    sigaddset(&mut block, 17 as libc::c_int);
    if sigprocmask(2 as libc::c_int, &mut block, 0 as *mut sigset_t) < 0 as libc::c_int {
        pfatal_with_name(
            b"sigprocmask(SIG_SETMASK, SIGCHLD)\0" as *const u8 as *const libc::c_char,
        );
    }
    bsd_signal(
        10 as libc::c_int,
        Some(debug_signal_handler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    set_default_suffixes();
    define_automatic_variables();
    let ref mut fresh12 = *define_makeflags(0 as libc::c_int);
    (*fresh12).set_export(v_export);
    define_default_variables();
    default_file = enter_file(
        strcache_add(b".DEFAULT\0" as *const u8 as *const libc::c_char),
    );
    default_goal_var = define_variable_in_set(
        b".DEFAULT_GOAL\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"\0" as *const u8 as *const libc::c_char,
        o_file,
        0 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    if !eval_strings.is_null() {
        let mut p_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut value_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut i_2: libc::c_uint = 0;
        let mut len_1: size_t = (::std::mem::size_of::<[libc::c_char; 8]>()
            as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul((*eval_strings).idx as libc::c_ulong);
        i_2 = 0 as libc::c_int as libc::c_uint;
        while i_2 < (*eval_strings).idx {
            p_0 = xstrdup(*((*eval_strings).list).offset(i_2 as isize));
            len_1 = (len_1 as libc::c_ulong)
                .wrapping_add(
                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(strlen(p_0)),
                ) as size_t as size_t;
            eval_buffer(p_0, 0 as *const floc);
            free(p_0 as *mut libc::c_void);
            i_2 = i_2.wrapping_add(1);
            i_2;
        }
        let mut fresh13 = ::std::vec::from_elem(0, len_1 as usize);
        value_0 = fresh13.leak().as_mut_ptr() as *mut libc::c_char;
        endp = value_0;
        p_0 = endp;
        i_2 = 0 as libc::c_int as libc::c_uint;
        while i_2 < (*eval_strings).idx {
            p_0 = stpcpy(p_0, b"--eval=\0" as *const u8 as *const libc::c_char);
            p_0 = quote_for_env(p_0, *((*eval_strings).list).offset(i_2 as isize));
            let fresh14 = p_0;
            p_0 = p_0.offset(1);
            endp = fresh14;
            *endp = ' ' as i32 as libc::c_char;
            i_2 = i_2.wrapping_add(1);
            i_2;
        }
        *endp = '\0' as i32 as libc::c_char;
        define_variable_in_set(
            b"-*-eval-flags-*-\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            value_0,
            o_automatic,
            0 as libc::c_int,
            (*current_variable_set_list).set,
            0 as *mut floc,
        );
    }
    let mut old_builtin_rules_flag: libc::c_int = no_builtin_rules_flag;
    let mut old_builtin_variables_flag: libc::c_int = no_builtin_variables_flag;
    let mut old_arg_job_slots: libc::c_int = arg_job_slots;
    read_files = read_all_makefiles(
        if makefiles.is_null() {
            0 as *mut *const libc::c_char
        } else {
            (*makefiles).list
        },
    );
    arg_job_slots = -(1 as libc::c_int);
    decode_env_switches(
        b"GNUMAKEFLAGS\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        o_env,
    );
    define_variable_in_set(
        b"GNUMAKEFLAGS\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"\0" as *const u8 as *const libc::c_char,
        o_override,
        0 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    decode_env_switches(
        b"MAKEFLAGS\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        o_env,
    );
    if arg_job_slots == -(1 as libc::c_int) || argv_slots != -(1 as libc::c_int) {
        arg_job_slots = old_arg_job_slots;
    } else if !jobserver_auth.is_null() && arg_job_slots != old_arg_job_slots {
        if restarts == 0 {
            error(
                0 as *mut floc,
                (53 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                    .wrapping_div(22 as libc::c_int as libc::c_ulong)
                    .wrapping_add(3 as libc::c_int as libc::c_ulong),
                dcgettext(
                    0 as *const libc::c_char,
                    b"warning: -j%d forced in makefile: resetting jobserver mode.\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                arg_job_slots,
            );
        }
        reset_jobserver();
    }
    syncing = (output_sync == 1 as libc::c_int || output_sync == 2 as libc::c_int)
        as libc::c_int as libc::c_uint;
    if make_sync.syncout() as libc::c_int != 0 && syncing == 0 {
        output_close(&mut make_sync);
    }
    make_sync.set_syncout(syncing);
    output_context = if make_sync.syncout() as libc::c_int != 0 {
        &mut make_sync
    } else {
        0 as *mut output
    };
    if no_builtin_variables_flag != 0 {
        no_builtin_rules_flag = 1 as libc::c_int;
    }
    if no_builtin_rules_flag != 0 && old_builtin_rules_flag == 0 {
        if (*suffix_file).builtin() != 0 {
            free_ns_chain((*suffix_file).deps as *mut nameseq);
            (*suffix_file).deps = 0 as *mut dep;
        }
        define_variable_in_set(
            b"SUFFIXES\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            b"\0" as *const u8 as *const libc::c_char,
            o_default,
            0 as libc::c_int,
            (*current_variable_set_list).set,
            0 as *mut floc,
        );
    }
    if no_builtin_variables_flag != 0 && old_builtin_variables_flag == 0 {
        undefine_default_variables();
    }
    if !jobserver_auth.is_null() {
        job_slots = 0 as libc::c_int as libc::c_uint;
    } else if arg_job_slots == -(1 as libc::c_int) {
        job_slots = 1 as libc::c_int as libc::c_uint;
    } else {
        job_slots = arg_job_slots as libc::c_uint;
    }
    if job_slots > 1 as libc::c_int as libc::c_uint
        && jobserver_setup(
            job_slots.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int,
            jobserver_style,
        ) != 0
    {
        jobserver_auth = jobserver_get_auth();
        if !jobserver_auth.is_null() {
            master_job_slots = job_slots;
            job_slots = 0 as libc::c_int as libc::c_uint;
        }
    }
    if syncing != 0 && job_slots == 1 as libc::c_int as libc::c_uint {
        output_context = 0 as *mut output;
        output_close(&mut make_sync);
        syncing = 0 as libc::c_int as libc::c_uint;
        output_sync = 0 as libc::c_int;
    }
    if syncing != 0 {
        if sync_mutex.is_null() {
            osync_setup();
            sync_mutex = osync_get_mutex();
        } else if osync_parse_mutex(sync_mutex) == 0 {
            osync_clear();
            free(sync_mutex as *mut libc::c_void);
            sync_mutex = 0 as *mut libc::c_char;
            syncing = 0 as libc::c_int as libc::c_uint;
        }
    }
    if !jobserver_auth.is_null() {
        if (0x2 as libc::c_int | 0x4 as libc::c_int) & db_level != 0 {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Using jobserver controller %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                jobserver_auth,
            );
            fflush(stdout);
        }
    }
    if !sync_mutex.is_null() {
        if 0x2 as libc::c_int & db_level != 0 {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Using output-sync mutex %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                sync_mutex,
            );
            fflush(stdout);
        }
    }
    define_makeflags(0 as libc::c_int);
    snap_deps();
    install_default_suffix_rules();
    convert_to_pattern();
    install_default_implicit_rules();
    snap_implicit_rules();
    build_vpath_lists();
    if !old_files.is_null() {
        let mut p_1: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
        p_1 = (*old_files).list;
        while !(*p_1).is_null() {
            let mut f_0: *mut file = enter_file(*p_1);
            (*f_0).mtime_before_update = 2 as libc::c_int as uintmax_t;
            (*f_0).last_mtime = (*f_0).mtime_before_update;
            (*f_0).set_updated(1 as libc::c_int as libc::c_uint);
            (*f_0).set_update_status(us_success);
            (*f_0).set_command_state(cs_finished);
            p_1 = p_1.offset(1);
            p_1;
        }
    }
    if restarts == 0 && !new_files.is_null() {
        let mut p_2: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
        p_2 = (*new_files).list;
        while !(*p_2).is_null() {
            let mut f_1: *mut file = enter_file(*p_2);
            (*f_1)
                .mtime_before_update = (!(0 as libc::c_int as uintmax_t))
                .wrapping_sub(
                    (if !(-(1 as libc::c_int) as uintmax_t
                        <= 0 as libc::c_int as libc::c_ulong)
                    {
                        0 as libc::c_int as uintmax_t
                    } else {
                        !(0 as libc::c_int as uintmax_t)
                            << (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    }),
                );
            (*f_1).last_mtime = (*f_1).mtime_before_update;
            p_2 = p_2.offset(1);
            p_2;
        }
    }
    remote_setup();
    output_context = 0 as *mut output;
    output_close(&mut make_sync);
    if !shuffle_mode.is_null() {
        if 0x1 as libc::c_int & db_level != 0 {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Enabled shuffle mode: %s\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                shuffle_mode,
            );
            fflush(stdout);
        }
    }
    if !read_files.is_null() {
        let mut makefile_mtimes: *mut uintmax_t = 0 as *mut uintmax_t;
        let mut skipped_makefiles: *mut goaldep = 0 as *mut goaldep;
        let mut nargv: *mut *const libc::c_char = argv as *mut *const libc::c_char;
        let mut any_failed: libc::c_int = 0 as libc::c_int;
        let mut status: update_status = us_success;
        if 0x1 as libc::c_int & db_level != 0 {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Updating makefiles....\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            fflush(stdout);
        }
        let mut num_mkfiles: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut d: *mut goaldep = read_files;
        read_files = 0 as *mut goaldep;
        while !d.is_null() {
            let mut t: *mut goaldep = d;
            d = (*d).next;
            (*t).next = read_files;
            read_files = t;
            num_mkfiles = num_mkfiles.wrapping_add(1);
            num_mkfiles;
        }
        let mut fresh15 = ::std::vec::from_elem(
            0,
            (num_mkfiles as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                as usize,
        );
        makefile_mtimes = fresh15.leak().as_mut_ptr() as *mut uintmax_t;
        let mut d_0: *mut goaldep = read_files;
        let mut last: *mut goaldep = 0 as *mut goaldep;
        let mut mm_idx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while !d_0.is_null() {
            let mut skip: libc::c_int = 0 as libc::c_int;
            let mut f_2: *mut file = (*d_0).file;
            if (*f_2).phony() != 0 {
                skip = 1 as libc::c_int;
            } else {
                f_2 = (*f_2).double_colon;
                while !f_2.is_null() {
                    if ((*f_2).deps).is_null() && !((*f_2).cmds).is_null() {
                        skip = 1 as libc::c_int;
                        break;
                    } else {
                        f_2 = (*f_2).prev;
                    }
                }
            }
            if skip == 0 {
                let fresh16 = mm_idx;
                mm_idx = mm_idx.wrapping_add(1);
                *makefile_mtimes
                    .offset(
                        fresh16 as isize,
                    ) = if (*(*d_0).file).last_mtime == 0 as libc::c_int as libc::c_ulong
                {
                    f_mtime((*d_0).file, 0 as libc::c_int)
                } else {
                    (*(*d_0).file).last_mtime
                };
                last = d_0;
                d_0 = (*d_0).next;
            } else {
                if 0x2 as libc::c_int & db_level != 0 {
                    printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Makefile '%s' might loop; not remaking it.\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*f_2).name,
                    );
                    fflush(stdout);
                }
                if !last.is_null() {
                    (*last).next = (*d_0).next;
                } else {
                    read_files = (*d_0).next;
                }
                if (*d_0).error != 0
                    && (*d_0).flags() as libc::c_int
                        & (1 as libc::c_int) << 2 as libc::c_int == 0
                {
                    (*d_0).next = skipped_makefiles;
                    skipped_makefiles = d_0;
                    any_failed = 1 as libc::c_int;
                } else {
                    free(d_0 as *mut libc::c_void);
                }
                d_0 = if !last.is_null() { (*last).next } else { read_files };
            }
        }
        define_makeflags(1 as libc::c_int);
        let mut orig_db_level: libc::c_int = db_level;
        if 0x100 as libc::c_int & db_level == 0 {
            db_level = 0 as libc::c_int;
        }
        rebuilding_makefiles = 1 as libc::c_int;
        status = update_goal_chain(read_files);
        rebuilding_makefiles = 0 as libc::c_int;
        db_level = orig_db_level;
        while !skipped_makefiles.is_null() {
            let mut d_1: *mut goaldep = skipped_makefiles;
            let mut err: *const libc::c_char = strerror((*d_1).error);
            error(
                &mut (*d_1).floc as *mut floc,
                (strlen(
                    (if !((*d_1).name).is_null() {
                        (*d_1).name
                    } else {
                        (*(*d_1).file).name
                    }),
                ))
                    .wrapping_add(strlen(err)),
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                if !((*d_1).name).is_null() { (*d_1).name } else { (*(*d_1).file).name },
                err,
            );
            skipped_makefiles = (*skipped_makefiles).next;
            free(d_1 as *mut libc::c_void);
        }
        if any_failed != 0
            && status as libc::c_uint == us_success as libc::c_int as libc::c_uint
        {
            status = us_none;
        }
        let mut current_block_550: u64;
        match status as libc::c_uint {
            1 => {
                let mut d_2: *mut goaldep = 0 as *mut goaldep;
                d_2 = read_files;
                while !d_2.is_null() {
                    if (*(*d_2).file).unloaded() != 0 {
                        let mut f_3: *mut file = (*d_2).file;
                        if load_file(&mut (*d_2).floc, f_3, 0 as libc::c_int)
                            == 0 as libc::c_int
                        {
                            fatal(
                                &mut (*d_2).floc as *mut floc,
                                strlen((*f_3).name),
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"%s: failed to load\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                (*f_3).name,
                            );
                        }
                        (*f_3).set_unloaded(0 as libc::c_int as libc::c_uint);
                        (*f_3).set_loaded(1 as libc::c_int as libc::c_uint);
                    }
                    d_2 = (*d_2).next;
                }
                current_block_550 = 10824877258417480809;
            }
            3 => {
                let mut any_remade: libc::c_int = 0 as libc::c_int;
                let mut i_3: libc::c_uint = 0;
                let mut d_4: *mut goaldep = 0 as *mut goaldep;
                i_3 = 0 as libc::c_int as libc::c_uint;
                d_4 = read_files;
                while !d_4.is_null() {
                    if (*(*d_4).file).updated() != 0 {
                        if (*(*d_4).file).update_status() as libc::c_int
                            == us_success as libc::c_int
                        {
                            any_remade
                                |= ((if (*(*d_4).file).last_mtime
                                    == 0 as libc::c_int as libc::c_ulong
                                {
                                    f_mtime((*d_4).file, 0 as libc::c_int)
                                } else {
                                    (*(*d_4).file).last_mtime
                                }) != *makefile_mtimes.offset(i_3 as isize)) as libc::c_int;
                        } else if (*d_4).flags() as libc::c_int
                            & (1 as libc::c_int) << 2 as libc::c_int == 0
                        {
                            let mut mtime: uintmax_t = 0;
                            error(
                                &mut (*d_4).floc as *mut floc,
                                strlen((*(*d_4).file).name),
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"Failed to remake makefile '%s'.\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                (*(*d_4).file).name,
                            );
                            mtime = if (*(*d_4).file).last_mtime
                                == 0 as libc::c_int as libc::c_ulong
                            {
                                f_mtime((*d_4).file, 0 as libc::c_int)
                            } else {
                                (*(*d_4).file).last_mtime
                            };
                            any_remade
                                |= (mtime != 1 as libc::c_int as libc::c_ulong
                                    && mtime != *makefile_mtimes.offset(i_3 as isize))
                                    as libc::c_int;
                            makefile_status = 2 as libc::c_int;
                            any_failed = 1 as libc::c_int;
                        }
                    } else if (*d_4).flags() as libc::c_int
                        & (1 as libc::c_int) << 2 as libc::c_int == 0
                    {
                        let mut dnm: *const libc::c_char = if !((*d_4).name).is_null() {
                            (*d_4).name
                        } else {
                            (*(*d_4).file).name
                        };
                        if (*d_4).flags() as libc::c_int
                            & (1 as libc::c_int) << 1 as libc::c_int != 0
                        {
                            error(
                                &mut (*d_4).floc as *mut floc,
                                strlen(dnm),
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"Included makefile '%s' was not found.\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                dnm,
                            );
                        } else {
                            error(
                                0 as *mut floc,
                                strlen(dnm),
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"Makefile '%s' was not found\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                dnm,
                            );
                            any_failed = 1 as libc::c_int;
                        }
                    }
                    i_3 = i_3.wrapping_add(1);
                    i_3;
                    d_4 = (*d_4).next;
                }
                if any_remade != 0 {
                    current_block_550 = 17271069569505794647;
                } else {
                    current_block_550 = 10824877258417480809;
                }
            }
            0 => {
                current_block_550 = 17271069569505794647;
            }
            2 | _ => {
                current_block_550 = 10824877258417480809;
            }
        }
        match current_block_550 {
            10824877258417480809 => {}
            _ => {
                remove_intermediates(0 as libc::c_int);
                if print_data_base_flag != 0 {
                    print_data_base();
                }
                clean_jobserver(0 as libc::c_int);
                if !makefiles.is_null() {
                    let mut mfidx: libc::c_int = 0 as libc::c_int;
                    let mut av: *mut *mut libc::c_char = argv;
                    let mut nv: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
                    let mut fresh17 = ::std::vec::from_elem(
                        0,
                        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                            .wrapping_mul(
                                (argc + 1 as libc::c_int + 1 as libc::c_int)
                                    as libc::c_ulong,
                            ) as usize,
                    );
                    nargv = fresh17.leak().as_mut_ptr() as *mut *const libc::c_char;
                    nv = nargv;
                    let fresh18 = av;
                    av = av.offset(1);
                    let fresh19 = nv;
                    nv = nv.offset(1);
                    *fresh19 = *fresh18;
                    let mut current_block_503: u64;
                    while !(*av).is_null() {
                        let mut f_4: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut a: *mut libc::c_char = *av;
                        let mut mf: *const libc::c_char = *((*makefiles).list)
                            .offset(mfidx as isize);
                        *nv = a;
                        if !(*a.offset(0 as libc::c_int as isize) as libc::c_int
                            != '-' as i32)
                        {
                            if *a.offset(1 as libc::c_int as isize) as libc::c_int
                                == '-' as i32
                            {
                                if strcmp(
                                    a,
                                    b"--file\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                    || strcmp(
                                        a,
                                        b"--makefile\0" as *const u8 as *const libc::c_char,
                                    ) == 0 as libc::c_int
                                {
                                    av = av.offset(1);
                                    av;
                                    current_block_503 = 14440951884744367330;
                                } else if !(strncmp(
                                    a,
                                    b"--file=\0" as *const u8 as *const libc::c_char,
                                    7 as libc::c_int as libc::c_ulong,
                                ) == 0 as libc::c_int)
                                    && !(strncmp(
                                        a,
                                        b"--makefile=\0" as *const u8 as *const libc::c_char,
                                        11 as libc::c_int as libc::c_ulong,
                                    ) == 0 as libc::c_int)
                                {
                                    current_block_503 = 4032406349246985652;
                                } else {
                                    current_block_503 = 14440951884744367330;
                                }
                                match current_block_503 {
                                    4032406349246985652 => {}
                                    _ => {
                                        if mfidx == stdin_offset {
                                            let mut fresh20 = ::std::vec::from_elem(
                                                0,
                                                (::std::mem::size_of::<[libc::c_char; 14]>()
                                                    as libc::c_ulong)
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    .wrapping_add(strlen(mf))
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
                                            );
                                            let mut na: *mut libc::c_char = fresh20.leak().as_mut_ptr()
                                                as *mut libc::c_char;
                                            sprintf(
                                                na,
                                                b"--temp-stdin=%s\0" as *const u8 as *const libc::c_char,
                                                mf,
                                            );
                                            *nv = na;
                                        } else {
                                            let mut fresh21 = ::std::vec::from_elem(
                                                0,
                                                (strlen(mf)).wrapping_add(3 as libc::c_int as libc::c_ulong)
                                                    as usize,
                                            );
                                            let mut na_0: *mut libc::c_char = fresh21
                                                .leak()
                                                .as_mut_ptr() as *mut libc::c_char;
                                            sprintf(
                                                na_0,
                                                b"-f%s\0" as *const u8 as *const libc::c_char,
                                                mf,
                                            );
                                            *nv = na_0;
                                        }
                                        mfidx += 1;
                                        mfidx;
                                    }
                                }
                            } else {
                                f_4 = strchr(a, 'f' as i32);
                                if !f_4.is_null() {
                                    if *f_4.offset(1 as libc::c_int as isize) as libc::c_int
                                        == '\0' as i32
                                    {
                                        av = av.offset(1);
                                        av;
                                    }
                                    if mfidx == stdin_offset {
                                        let al: size_t = f_4.offset_from(a) as libc::c_long
                                            as size_t;
                                        let mut na_1: *mut libc::c_char = 0 as *mut libc::c_char;
                                        if al > 1 as libc::c_int as libc::c_ulong {
                                            let mut fresh22 = ::std::vec::from_elem(
                                                0,
                                                al.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
                                            );
                                            na_1 = fresh22.leak().as_mut_ptr() as *mut libc::c_char;
                                            memcpy(
                                                na_1 as *mut libc::c_void,
                                                a as *const libc::c_void,
                                                al,
                                            );
                                            *na_1.offset(al as isize) = '\0' as i32 as libc::c_char;
                                            let fresh23 = nv;
                                            nv = nv.offset(1);
                                            *fresh23 = na_1;
                                        }
                                        let mut fresh24 = ::std::vec::from_elem(
                                            0,
                                            (::std::mem::size_of::<[libc::c_char; 14]>()
                                                as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_add(strlen(mf))
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
                                        );
                                        na_1 = fresh24.leak().as_mut_ptr() as *mut libc::c_char;
                                        sprintf(
                                            na_1,
                                            b"--temp-stdin=%s\0" as *const u8 as *const libc::c_char,
                                            mf,
                                        );
                                        *nv = na_1;
                                    } else if *f_4.offset(1 as libc::c_int as isize)
                                        as libc::c_int == '\0' as i32
                                    {
                                        nv = nv.offset(1);
                                        *nv = mf;
                                    } else {
                                        let al_0: size_t = (f_4.offset_from(a) as libc::c_long
                                            + 1 as libc::c_int as libc::c_long) as size_t;
                                        let ml: size_t = (strlen(mf))
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong);
                                        let mut fresh25 = ::std::vec::from_elem(
                                            0,
                                            al_0.wrapping_add(ml) as usize,
                                        );
                                        let mut na_2: *mut libc::c_char = fresh25
                                            .leak()
                                            .as_mut_ptr() as *mut libc::c_char;
                                        memcpy(
                                            na_2 as *mut libc::c_void,
                                            a as *const libc::c_void,
                                            al_0,
                                        );
                                        memcpy(
                                            na_2.offset(al_0 as isize) as *mut libc::c_void,
                                            mf as *const libc::c_void,
                                            ml,
                                        );
                                        *nv = na_2;
                                    }
                                    mfidx += 1;
                                    mfidx;
                                }
                            }
                        }
                        av = av.offset(1);
                        av;
                        nv = nv.offset(1);
                        nv;
                    }
                    *nv = 0 as *const libc::c_char;
                }
                if !directories.is_null()
                    && (*directories).idx > 0 as libc::c_int as libc::c_uint
                {
                    let mut bad: libc::c_int = 1 as libc::c_int;
                    if !directory_before_chdir.is_null() {
                        if chdir(directory_before_chdir) < 0 as libc::c_int {
                            perror_with_name(
                                b"chdir\0" as *const u8 as *const libc::c_char,
                                b"\0" as *const u8 as *const libc::c_char,
                            );
                        } else {
                            bad = 0 as libc::c_int;
                        }
                    }
                    if bad != 0 {
                        fatal(
                            0 as *mut floc,
                            0 as libc::c_int as size_t,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Couldn't change back to original directory\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                }
                restarts = restarts.wrapping_add(1);
                restarts;
                if 0x1 as libc::c_int & db_level != 0 {
                    let mut p_3: *mut *const libc::c_char = 0
                        as *mut *const libc::c_char;
                    printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Re-executing[%u]:\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        restarts,
                    );
                    p_3 = nargv;
                    while !(*p_3).is_null() {
                        printf(b" %s\0" as *const u8 as *const libc::c_char, *p_3);
                        p_3 = p_3.offset(1);
                        p_3;
                    }
                    putchar('\n' as i32);
                    fflush(stdout);
                }
                let mut p_4: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
                p_4 = environ;
                while !(*p_4).is_null() {
                    if strncmp(
                        *p_4,
                        b"MAKELEVEL=\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) == 0 as libc::c_int
                    {
                        let mut fresh26 = ::std::vec::from_elem(
                            0,
                            40 as libc::c_int as libc::c_ulong as usize,
                        );
                        *p_4 = fresh26.leak().as_mut_ptr() as *mut libc::c_char;
                        sprintf(
                            *p_4,
                            b"%s=%u\0" as *const u8 as *const libc::c_char,
                            b"MAKELEVEL\0" as *const u8 as *const libc::c_char,
                            makelevel,
                        );
                    } else if strncmp(
                        *p_4,
                        b"MAKE_RESTARTS=\0" as *const u8 as *const libc::c_char,
                        (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0 as libc::c_int
                    {
                        let mut fresh27 = ::std::vec::from_elem(
                            0,
                            40 as libc::c_int as libc::c_ulong as usize,
                        );
                        *p_4 = fresh27.leak().as_mut_ptr() as *mut libc::c_char;
                        sprintf(
                            *p_4,
                            b"MAKE_RESTARTS=%s%u\0" as *const u8 as *const libc::c_char,
                            if stdio_traced != 0 {
                                b"-\0" as *const u8 as *const libc::c_char
                            } else {
                                b"\0" as *const u8 as *const libc::c_char
                            },
                            restarts,
                        );
                        restarts = 0 as libc::c_int as libc::c_uint;
                    }
                    p_4 = p_4.offset(1);
                    p_4;
                }
                if restarts != 0 {
                    let mut fresh28 = ::std::vec::from_elem(
                        0,
                        40 as libc::c_int as libc::c_ulong as usize,
                    );
                    let mut b: *mut libc::c_char = fresh28.leak().as_mut_ptr()
                        as *mut libc::c_char;
                    sprintf(
                        b,
                        b"MAKE_RESTARTS=%s%u\0" as *const u8 as *const libc::c_char,
                        if stdio_traced != 0 {
                            b"-\0" as *const u8 as *const libc::c_char
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
                        },
                        restarts,
                    );
                    putenv(b);
                }
                fflush(stdout);
                fflush(stderr);
                osync_clear();
                jobserver_pre_child(1 as libc::c_int);
                exec_command(nargv as *mut *mut libc::c_char, environ);
                jobserver_post_child(1 as libc::c_int);
                temp_stdin_unlink();
                _exit(127 as libc::c_int);
            }
        }
        if any_failed != 0 {
            die(2 as libc::c_int);
        }
    }
    define_makeflags(0 as libc::c_int);
    always_make_flag = always_make_set;
    if restarts != 0 && !new_files.is_null() {
        let mut p_5: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
        p_5 = (*new_files).list;
        while !(*p_5).is_null() {
            let mut f_5: *mut file = enter_file(*p_5);
            (*f_5)
                .mtime_before_update = (!(0 as libc::c_int as uintmax_t))
                .wrapping_sub(
                    (if !(-(1 as libc::c_int) as uintmax_t
                        <= 0 as libc::c_int as libc::c_ulong)
                    {
                        0 as libc::c_int as uintmax_t
                    } else {
                        !(0 as libc::c_int as uintmax_t)
                            << (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    }),
                );
            (*f_5).last_mtime = (*f_5).mtime_before_update;
            p_5 = p_5.offset(1);
            p_5;
        }
    }
    temp_stdin_unlink();
    if goals.is_null() {
        let mut p_6: *mut libc::c_char = 0 as *mut libc::c_char;
        if (*default_goal_var).recursive() != 0 {
            p_6 = variable_expand((*default_goal_var).value);
        } else {
            p_6 = variable_buffer_output(
                variable_buffer,
                (*default_goal_var).value,
                strlen((*default_goal_var).value),
            );
            *p_6 = '\0' as i32 as libc::c_char;
            p_6 = variable_buffer;
        }
        if *p_6 as libc::c_int != '\0' as i32 {
            let mut f_6: *mut file = lookup_file(p_6);
            if f_6.is_null() {
                let mut ns: *mut nameseq = 0 as *mut nameseq;
                ns = parse_file_seq(
                    &mut p_6,
                    ::std::mem::size_of::<nameseq>() as libc::c_ulong,
                    0x1 as libc::c_int,
                    0 as *const libc::c_char,
                    0 as libc::c_int,
                ) as *mut nameseq;
                if !ns.is_null() {
                    if !((*ns).next).is_null() {
                        fatal(
                            0 as *mut floc,
                            0 as libc::c_int as size_t,
                            dcgettext(
                                0 as *const libc::c_char,
                                b".DEFAULT_GOAL contains more than one target\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    f_6 = enter_file(strcache_add((*ns).name));
                    (*ns).name = 0 as *const libc::c_char;
                    free_ns_chain(ns);
                }
            }
            if !f_6.is_null() {
                goals = xcalloc(::std::mem::size_of::<goaldep>() as libc::c_ulong)
                    as *mut goaldep;
                (*goals).file = f_6;
            }
        }
    } else {
        (*lastgoal).next = 0 as *mut goaldep;
    }
    if goals.is_null() {
        let mut v_2: *mut variable = lookup_variable(
            b"MAKEFILE_LIST\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        if !v_2.is_null() && !((*v_2).value).is_null()
            && *((*v_2).value).offset(0 as libc::c_int as isize) as libc::c_int
                != '\0' as i32
        {
            fatal(
                0 as *mut floc,
                0 as libc::c_int as size_t,
                dcgettext(
                    0 as *const libc::c_char,
                    b"No targets\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        fatal(
            0 as *mut floc,
            0 as libc::c_int as size_t,
            dcgettext(
                0 as *const libc::c_char,
                b"No targets specified and no makefile found\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    shuffle_deps_recursive(goals as *mut dep);
    if 0x1 as libc::c_int & db_level != 0 {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Updating goal targets....\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fflush(stdout);
    }
    match update_goal_chain(goals) as libc::c_uint {
        0 => {}
        2 => {
            makefile_status = 1 as libc::c_int;
        }
        3 => {
            makefile_status = 2 as libc::c_int;
        }
        1 | _ => {}
    }
    if clock_skew_detected != 0 {
        error(
            0 as *mut floc,
            0 as libc::c_int as size_t,
            dcgettext(
                0 as *const libc::c_char,
                b"warning:  Clock skew detected.  Your build may be incomplete.\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    die(makefile_status);
}
static mut options: [libc::c_char; 121] = [0; 121];
static mut long_options: [option; 49] = [option {
    name: 0 as *const libc::c_char,
    has_arg: 0,
    flag: 0 as *const libc::c_int as *mut libc::c_int,
    val: 0,
}; 49];
unsafe extern "C" fn init_switches() {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    if options[0 as libc::c_int as usize] as libc::c_int != '\0' as i32 {
        return;
    }
    p = options.as_mut_ptr();
    let fresh29 = p;
    p = p.offset(1);
    *fresh29 = '-' as i32 as libc::c_char;
    i = 0 as libc::c_int as libc::c_uint;
    while switches[i as usize].c != '\0' as i32 {
        long_options[i as usize]
            .name = (if (switches[i as usize].long_name).is_null() {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            switches[i as usize].long_name
        }) as *mut libc::c_char;
        long_options[i as usize].flag = 0 as *mut libc::c_int;
        long_options[i as usize].val = switches[i as usize].c;
        if switches[i as usize].c <= 127 as libc::c_int {
            let fresh30 = p;
            p = p.offset(1);
            *fresh30 = switches[i as usize].c as libc::c_char;
        }
        match switches[i as usize].type_0 as libc::c_uint {
            0 | 1 | 7 => {
                long_options[i as usize].has_arg = 0 as libc::c_int;
            }
            2 | 3 | 4 | 5 | 6 => {
                if switches[i as usize].c <= 127 as libc::c_int {
                    let fresh31 = p;
                    p = p.offset(1);
                    *fresh31 = ':' as i32 as libc::c_char;
                }
                if !(switches[i as usize].noarg_value).is_null() {
                    if switches[i as usize].c <= 127 as libc::c_int {
                        let fresh32 = p;
                        p = p.offset(1);
                        *fresh32 = ':' as i32 as libc::c_char;
                    }
                    long_options[i as usize].has_arg = 2 as libc::c_int;
                } else {
                    long_options[i as usize].has_arg = 1 as libc::c_int;
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    *p = '\0' as i32 as libc::c_char;
    c = 0 as libc::c_int as libc::c_uint;
    while (c as libc::c_ulong)
        < (::std::mem::size_of::<[option; 9]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<option>() as libc::c_ulong)
    {
        let fresh33 = i;
        i = i.wrapping_add(1);
        long_options[fresh33 as usize] = long_option_aliases[c as usize];
        c = c.wrapping_add(1);
        c;
    }
    long_options[i as usize].name = 0 as *const libc::c_char;
}
unsafe extern "C" fn handle_non_switch_argument(
    mut arg: *const libc::c_char,
    mut origin: variable_origin,
) {
    let mut v: *mut variable = 0 as *mut variable;
    if *arg.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
        && *arg.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return;
    }
    v = try_variable_definition(0 as *const floc, arg, origin, 0 as libc::c_int);
    if !v.is_null() {
        let mut cv: *mut command_variable = 0 as *mut command_variable;
        cv = command_variables;
        while !cv.is_null() {
            if (*cv).variable == v {
                break;
            }
            cv = (*cv).next;
        }
        if cv.is_null() {
            cv = xmalloc(::std::mem::size_of::<command_variable>() as libc::c_ulong)
                as *mut command_variable;
            (*cv).variable = v;
            (*cv).next = command_variables;
            command_variables = cv;
        }
    } else if *arg.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
        && origin as libc::c_uint == o_command as libc::c_int as libc::c_uint
    {
        let mut f: *mut file = enter_file(strcache_add(expand_command_line_file(arg)));
        (*f).set_cmd_target(1 as libc::c_int as libc::c_uint);
        if goals.is_null() {
            goals = xcalloc(::std::mem::size_of::<goaldep>() as libc::c_ulong)
                as *mut goaldep;
            lastgoal = goals;
        } else {
            (*lastgoal)
                .next = xcalloc(::std::mem::size_of::<goaldep>() as libc::c_ulong)
                as *mut goaldep;
            lastgoal = (*lastgoal).next;
        }
        (*lastgoal).file = f;
        let mut gv: *mut variable = 0 as *mut variable;
        let mut value: *const libc::c_char = 0 as *const libc::c_char;
        gv = lookup_variable(
            b"MAKECMDGOALS\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        if gv.is_null() {
            value = (*f).name;
        } else {
            let mut oldlen: size_t = 0;
            let mut newlen: size_t = 0;
            let mut vp: *mut libc::c_char = 0 as *mut libc::c_char;
            oldlen = strlen((*gv).value);
            newlen = strlen((*f).name);
            let mut fresh34 = ::std::vec::from_elem(
                0,
                oldlen
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(newlen)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
            );
            vp = fresh34.leak().as_mut_ptr() as *mut libc::c_char;
            memcpy(vp as *mut libc::c_void, (*gv).value as *const libc::c_void, oldlen);
            *vp.offset(oldlen as isize) = ' ' as i32 as libc::c_char;
            memcpy(
                &mut *vp
                    .offset(
                        oldlen.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as *mut libc::c_char as *mut libc::c_void,
                (*f).name as *const libc::c_void,
                newlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            value = vp;
        }
        define_variable_in_set(
            b"MAKECMDGOALS\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            value,
            o_default,
            0 as libc::c_int,
            (*current_variable_set_list).set,
            0 as *mut floc,
        );
    }
}
pub unsafe extern "C" fn reset_makeflags(mut origin: variable_origin) {
    decode_env_switches(
        b"MAKEFLAGS\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        origin,
    );
    construct_include_path(
        if !include_dirs.is_null() {
            (*include_dirs).list
        } else {
            0 as *mut *const libc::c_char
        },
    );
    define_makeflags(rebuilding_makefiles);
}
unsafe extern "C" fn decode_switches(
    mut argc: libc::c_int,
    mut argv: *mut *const libc::c_char,
    mut origin: variable_origin,
) {
    let mut bad: libc::c_int = 0 as libc::c_int;
    let mut cs: *mut command_switch = 0 as *mut command_switch;
    let mut sl: *mut stringlist = 0 as *mut stringlist;
    let mut c: libc::c_int = 0;
    init_switches();
    opterr = (origin as libc::c_uint == o_command as libc::c_int as libc::c_uint)
        as libc::c_int;
    optind = 0 as libc::c_int;
    while optind < argc {
        let mut coptarg: *const libc::c_char = 0 as *const libc::c_char;
        c = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            options.as_mut_ptr(),
            long_options.as_mut_ptr(),
            0 as *mut libc::c_int,
        );
        coptarg = optarg;
        if c == -(1 as libc::c_int) {
            break;
        }
        if c == 1 as libc::c_int {
            handle_non_switch_argument(coptarg, origin);
        } else if c == '?' as i32 {
            bad = 1 as libc::c_int;
        } else {
            cs = switches.as_mut_ptr();
            while (*cs).c != '\0' as i32 {
                if (*cs).c == c {
                    let mut doit: libc::c_int = (origin as libc::c_uint
                        == o_command as libc::c_int as libc::c_uint
                        || (*cs).env() as libc::c_int != 0
                            && (((*cs).origin).is_null()
                                || origin as libc::c_uint >= *(*cs).origin as libc::c_uint))
                        as libc::c_int;
                    if doit != 0 {
                        (*cs).set_specified(1 as libc::c_int as libc::c_uint);
                    }
                    let mut current_block_85: u64;
                    match (*cs).type_0 as libc::c_uint {
                        7 => {}
                        0 | 1 => {
                            if doit != 0 {
                                *((*cs).value_ptr
                                    as *mut libc::c_int) = ((*cs).type_0 as libc::c_uint
                                    == flag as libc::c_int as libc::c_uint) as libc::c_int;
                                if !((*cs).origin).is_null() {
                                    *(*cs).origin = origin;
                                }
                            }
                        }
                        2 | 3 | 4 => {
                            if !(doit == 0) {
                                if coptarg.is_null() {
                                    coptarg = (*cs).noarg_value as *const libc::c_char;
                                    current_block_85 = 5689316957504528238;
                                } else if *coptarg as libc::c_int == '\0' as i32 {
                                    let mut opt: [libc::c_char; 2] = *::std::mem::transmute::<
                                        &[u8; 2],
                                        &mut [libc::c_char; 2],
                                    >(b"c\0");
                                    let mut op: *const libc::c_char = opt.as_mut_ptr();
                                    if (*cs).c <= 127 as libc::c_int {
                                        opt[0 as libc::c_int as usize] = (*cs).c as libc::c_char;
                                    } else {
                                        op = (*cs).long_name;
                                    }
                                    error(
                                        0 as *mut floc,
                                        strlen(op),
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"the '%s%s' option requires a non-empty string argument\0"
                                                as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        if (*cs).c <= 127 as libc::c_int {
                                            b"-\0" as *const u8 as *const libc::c_char
                                        } else {
                                            b"--\0" as *const u8 as *const libc::c_char
                                        },
                                        op,
                                    );
                                    bad = 1 as libc::c_int;
                                    current_block_85 = 14541395414537699361;
                                } else {
                                    current_block_85 = 5689316957504528238;
                                }
                                match current_block_85 {
                                    14541395414537699361 => {}
                                    _ => {
                                        if (*cs).type_0 as libc::c_uint
                                            == string as libc::c_int as libc::c_uint
                                        {
                                            let mut val: *mut *mut libc::c_char = (*cs).value_ptr
                                                as *mut *mut libc::c_char;
                                            free(*val as *mut libc::c_void);
                                            *val = xstrdup(coptarg);
                                            if !((*cs).origin).is_null() {
                                                *(*cs).origin = origin;
                                            }
                                        } else {
                                            sl = *((*cs).value_ptr as *mut *mut stringlist);
                                            if sl.is_null() {
                                                sl = xmalloc(
                                                    ::std::mem::size_of::<stringlist>() as libc::c_ulong,
                                                ) as *mut stringlist;
                                                (*sl).max = 5 as libc::c_int as libc::c_uint;
                                                (*sl).idx = 0 as libc::c_int as libc::c_uint;
                                                (*sl)
                                                    .list = xmalloc(
                                                    (5 as libc::c_int as libc::c_ulong)
                                                        .wrapping_mul(
                                                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                                                        ),
                                                ) as *mut *const libc::c_char;
                                                let ref mut fresh35 = *((*cs).value_ptr
                                                    as *mut *mut stringlist);
                                                *fresh35 = sl;
                                            } else if (*sl).idx
                                                == ((*sl).max)
                                                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                            {
                                                (*sl)
                                                    .max = ((*sl).max)
                                                    .wrapping_add(5 as libc::c_int as libc::c_uint);
                                                (*sl)
                                                    .list = xrealloc(
                                                    (*sl).list as *mut libc::c_void,
                                                    ((*sl).max as libc::c_ulong)
                                                        .wrapping_mul(
                                                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                                                        ),
                                                ) as *mut *const libc::c_char;
                                            }
                                            if (*cs).c != 'f' as i32 {
                                                let mut k: libc::c_uint = 0;
                                                k = 0 as libc::c_int as libc::c_uint;
                                                while k < (*sl).idx {
                                                    if *((*sl).list).offset(k as isize) == coptarg
                                                        || **((*sl).list).offset(k as isize) as libc::c_int
                                                            == *coptarg as libc::c_int
                                                            && (**((*sl).list).offset(k as isize) as libc::c_int
                                                                == '\0' as i32
                                                                || strcmp(
                                                                    (*((*sl).list).offset(k as isize))
                                                                        .offset(1 as libc::c_int as isize),
                                                                    coptarg.offset(1 as libc::c_int as isize),
                                                                ) == 0)
                                                    {
                                                        break;
                                                    }
                                                    k = k.wrapping_add(1);
                                                    k;
                                                }
                                                if k < (*sl).idx {
                                                    current_block_85 = 14541395414537699361;
                                                } else {
                                                    current_block_85 = 16738040538446813684;
                                                }
                                            } else {
                                                current_block_85 = 16738040538446813684;
                                            }
                                            match current_block_85 {
                                                14541395414537699361 => {}
                                                _ => {
                                                    if (*cs).type_0 as libc::c_uint
                                                        == strlist as libc::c_int as libc::c_uint
                                                    {
                                                        let fresh36 = (*sl).idx;
                                                        (*sl).idx = ((*sl).idx).wrapping_add(1);
                                                        let ref mut fresh37 = *((*sl).list)
                                                            .offset(fresh36 as isize);
                                                        *fresh37 = xstrdup(coptarg);
                                                        if !((*cs).origin).is_null() {
                                                            *(*cs).origin = origin;
                                                        }
                                                    } else if (*cs).c == 127 as libc::c_int + 10 as libc::c_int
                                                    {
                                                        if stdin_offset > 0 as libc::c_int {
                                                            fatal(
                                                                0 as *mut floc,
                                                                0 as libc::c_int as size_t,
                                                                b"INTERNAL: multiple --temp-stdin options provided!\0"
                                                                    as *const u8 as *const libc::c_char,
                                                            );
                                                        }
                                                        stdin_offset = (*sl).idx as libc::c_int;
                                                        let fresh38 = (*sl).idx;
                                                        (*sl).idx = ((*sl).idx).wrapping_add(1);
                                                        let ref mut fresh39 = *((*sl).list)
                                                            .offset(fresh38 as isize);
                                                        *fresh39 = strcache_add(coptarg);
                                                        if !((*cs).origin).is_null() {
                                                            *(*cs).origin = origin;
                                                        }
                                                    } else {
                                                        let fresh40 = (*sl).idx;
                                                        (*sl).idx = ((*sl).idx).wrapping_add(1);
                                                        let ref mut fresh41 = *((*sl).list)
                                                            .offset(fresh40 as isize);
                                                        *fresh41 = expand_command_line_file(coptarg);
                                                        if !((*cs).origin).is_null() {
                                                            *(*cs).origin = origin;
                                                        }
                                                    }
                                                    let ref mut fresh42 = *((*sl).list)
                                                        .offset((*sl).idx as isize);
                                                    *fresh42 = 0 as *const libc::c_char;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        5 => {
                            if coptarg.is_null() && argc > optind {
                                let mut cp: *const libc::c_char = 0 as *const libc::c_char;
                                cp = *argv.offset(optind as isize);
                                while (*cp.offset(0 as libc::c_int as isize)
                                    as libc::c_uint)
                                    .wrapping_sub('0' as i32 as libc::c_uint)
                                    <= 9 as libc::c_int as libc::c_uint
                                {
                                    cp = cp.offset(1);
                                    cp;
                                }
                                if *cp.offset(0 as libc::c_int as isize) as libc::c_int
                                    == '\0' as i32
                                {
                                    let fresh43 = optind;
                                    optind = optind + 1;
                                    coptarg = *argv.offset(fresh43 as isize);
                                }
                            }
                            if !(doit == 0) {
                                if !coptarg.is_null() {
                                    let mut err: *const libc::c_char = 0 as *const libc::c_char;
                                    let mut i: libc::c_uint = make_toui(coptarg, &mut err);
                                    if !err.is_null() || i == 0 as libc::c_int as libc::c_uint {
                                        error(
                                            0 as *mut floc,
                                            0 as libc::c_int as size_t,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"the '-%c' option requires a positive integer argument\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            (*cs).c,
                                        );
                                        bad = 1 as libc::c_int;
                                    } else {
                                        *((*cs).value_ptr as *mut libc::c_uint) = i;
                                        if !((*cs).origin).is_null() {
                                            *(*cs).origin = origin;
                                        }
                                    }
                                } else {
                                    *((*cs).value_ptr
                                        as *mut libc::c_uint) = *((*cs).noarg_value
                                        as *mut libc::c_uint);
                                    if !((*cs).origin).is_null() {
                                        *(*cs).origin = origin;
                                    }
                                }
                            }
                        }
                        6 => {
                            if coptarg.is_null() && optind < argc
                                && ((*(*argv.offset(optind as isize))
                                    .offset(0 as libc::c_int as isize) as libc::c_uint)
                                    .wrapping_sub('0' as i32 as libc::c_uint)
                                    <= 9 as libc::c_int as libc::c_uint
                                    || *(*argv.offset(optind as isize))
                                        .offset(0 as libc::c_int as isize) as libc::c_int
                                        == '.' as i32)
                            {
                                let fresh44 = optind;
                                optind = optind + 1;
                                coptarg = *argv.offset(fresh44 as isize);
                            }
                            if doit != 0 {
                                *((*cs).value_ptr
                                    as *mut libc::c_double) = if !coptarg.is_null() {
                                    atof(coptarg)
                                } else {
                                    *((*cs).noarg_value as *mut libc::c_double)
                                };
                                if !((*cs).origin).is_null() {
                                    *(*cs).origin = origin;
                                }
                            }
                        }
                        _ => {
                            abort();
                        }
                    }
                    break;
                } else {
                    cs = cs.offset(1);
                    cs;
                }
            }
        }
    }
    while optind < argc {
        let fresh45 = optind;
        optind = optind + 1;
        handle_non_switch_argument(*argv.offset(fresh45 as isize), origin);
    }
    if bad != 0 && origin as libc::c_uint == o_command as libc::c_int as libc::c_uint {
        print_usage(bad);
    }
    decode_debug_flags();
    decode_output_sync_flags();
    run_silent = silent_flag;
}
unsafe extern "C" fn decode_env_switches(
    mut envar: *const libc::c_char,
    mut len: size_t,
    mut origin: variable_origin,
) {
    let mut fresh46 = ::std::vec::from_elem(
        0,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as usize,
    );
    let mut varref: *mut libc::c_char = fresh46.leak().as_mut_ptr() as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argc: libc::c_int = 0;
    let mut argv: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    p = varref;
    let fresh47 = p;
    p = p.offset(1);
    *fresh47 = '$' as i32 as libc::c_char;
    let fresh48 = p;
    p = p.offset(1);
    *fresh48 = '(' as i32 as libc::c_char;
    p = mempcpy(p as *mut libc::c_void, envar as *const libc::c_void, len)
        as *mut libc::c_char;
    let fresh49 = p;
    p = p.offset(1);
    *fresh49 = ')' as i32 as libc::c_char;
    *p = '\0' as i32 as libc::c_char;
    value = variable_expand(varref);
    while stopchar_map[*value as libc::c_uchar as usize] as libc::c_int
        & (0x2 as libc::c_int | 0x4 as libc::c_int) != 0 as libc::c_int
    {
        value = value.offset(1);
        value;
    }
    len = strlen(value);
    if len == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    let mut fresh50 = ::std::vec::from_elem(
        0,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as usize,
    );
    argv = fresh50.leak().as_mut_ptr() as *mut *const libc::c_char;
    let ref mut fresh51 = *argv.offset(0 as libc::c_int as isize);
    *fresh51 = b"\0" as *const u8 as *const libc::c_char;
    argc = 1 as libc::c_int;
    let mut fresh52 = ::std::vec::from_elem(
        0,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
    );
    buf = fresh52.leak().as_mut_ptr() as *mut libc::c_char;
    *buf.offset(0 as libc::c_int as isize) = '-' as i32 as libc::c_char;
    p = buf.offset(1 as libc::c_int as isize);
    let ref mut fresh53 = *argv.offset(argc as isize);
    *fresh53 = p;
    while *value as libc::c_int != '\0' as i32 {
        if *value as libc::c_int == '\\' as i32
            && *value.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
        {
            value = value.offset(1);
            value;
        } else if stopchar_map[*value as libc::c_uchar as usize] as libc::c_int
            & 0x2 as libc::c_int != 0 as libc::c_int
        {
            let fresh54 = p;
            p = p.offset(1);
            *fresh54 = '\0' as i32 as libc::c_char;
            argc += 1;
            let ref mut fresh55 = *argv.offset(argc as isize);
            *fresh55 = p;
            loop {
                value = value.offset(1);
                value;
                if !(stopchar_map[*value as libc::c_uchar as usize] as libc::c_int
                    & 0x2 as libc::c_int != 0 as libc::c_int)
                {
                    break;
                }
            }
            continue;
        }
        let fresh56 = value;
        value = value.offset(1);
        let fresh57 = p;
        p = p.offset(1);
        *fresh57 = *fresh56;
    }
    *p = '\0' as i32 as libc::c_char;
    argc += 1;
    let ref mut fresh58 = *argv.offset(argc as isize);
    *fresh58 = 0 as *const libc::c_char;
    if *(*argv.offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
        as libc::c_int != '-' as i32
        && (strchr(*argv.offset(1 as libc::c_int as isize), '=' as i32)).is_null()
    {
        let ref mut fresh59 = *argv.offset(1 as libc::c_int as isize);
        *fresh59 = buf;
    }
    decode_switches(argc, argv, origin);
}
unsafe extern "C" fn quote_for_env(
    mut out: *mut libc::c_char,
    mut in_0: *const libc::c_char,
) -> *mut libc::c_char {
    while *in_0 as libc::c_int != '\0' as i32 {
        if *in_0 as libc::c_int == '$' as i32 {
            let fresh60 = out;
            out = out.offset(1);
            *fresh60 = '$' as i32 as libc::c_char;
        } else if stopchar_map[*in_0 as libc::c_uchar as usize] as libc::c_int
            & 0x2 as libc::c_int != 0 as libc::c_int
            || *in_0 as libc::c_int == '\\' as i32
        {
            let fresh61 = out;
            out = out.offset(1);
            *fresh61 = '\\' as i32 as libc::c_char;
        }
        let fresh62 = in_0;
        in_0 = in_0.offset(1);
        let fresh63 = out;
        out = out.offset(1);
        *fresh63 = *fresh62;
    }
    return out;
}
pub unsafe extern "C" fn define_makeflags(mut makefile: libc::c_int) -> *mut variable {
    let ref_0: [libc::c_char; 14] = *::std::mem::transmute::<
        &[u8; 14],
        &[libc::c_char; 14],
    >(b"MAKEOVERRIDES\0");
    let posixref: [libc::c_char; 24] = *::std::mem::transmute::<
        &[u8; 24],
        &[libc::c_char; 24],
    >(b"-*-command-variables-*-\0");
    let evalref: [libc::c_char; 20] = *::std::mem::transmute::<
        &[u8; 20],
        &[libc::c_char; 20],
    >(b"$(-*-eval-flags-*-)\0");
    let mut cs: *const command_switch = 0 as *const command_switch;
    let mut v: *mut variable = 0 as *mut variable;
    let mut flagstring: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flags: *mut flag = 0 as *mut flag;
    let mut last: *mut flag = 0 as *mut flag;
    let mut flagslen: size_t = 0 as libc::c_int as size_t;
    cs = switches.as_mut_ptr();
    while (*cs).c != '\0' as i32 {
        if (*cs).toenv() as libc::c_int != 0
            && (makefile == 0 || (*cs).no_makefile() == 0)
        {
            match (*cs).type_0 as libc::c_uint {
                7 => {}
                0 | 1 => {
                    if (*((*cs).value_ptr as *mut libc::c_int) == 0) as libc::c_int
                        == ((*cs).type_0 as libc::c_uint
                            == flag_off as libc::c_int as libc::c_uint) as libc::c_int
                        && (((*cs).default_value).is_null()
                            || (*cs).specified() as libc::c_int != 0
                            || *((*cs).value_ptr as *mut libc::c_int)
                                != *((*cs).default_value as *mut libc::c_int))
                    {
                        let mut fresh64 = ::std::vec::from_elem(
                            0,
                            ::std::mem::size_of::<flag>() as libc::c_ulong as usize,
                        );
                        let mut new: *mut flag = fresh64.leak().as_mut_ptr()
                            as *mut flag;
                        (*new).cs = cs;
                        (*new).arg = 0 as *const libc::c_char;
                        (*new).next = 0 as *mut flag;
                        if flags.is_null() {
                            flags = new;
                        } else {
                            (*last).next = new;
                        }
                        last = new;
                        if ((*new).arg).is_null() {
                            flagslen = (flagslen as libc::c_ulong)
                                .wrapping_add(3 as libc::c_int as libc::c_ulong) as size_t
                                as size_t;
                        } else {
                            flagslen = (flagslen as libc::c_ulong)
                                .wrapping_add(
                                    (1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                                        + 3 as libc::c_int * 0 as libc::c_int) as libc::c_ulong,
                                ) as size_t as size_t;
                        }
                        if !((*cs).c <= 127 as libc::c_int) {
                            flagslen = (flagslen as libc::c_ulong)
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(strlen((*cs).long_name)),
                                ) as size_t as size_t;
                        }
                    }
                }
                5 => {
                    if !(!((*cs).default_value).is_null()
                        && *((*cs).value_ptr as *mut libc::c_uint)
                            == *((*cs).default_value as *mut libc::c_uint))
                    {
                        if !((*cs).noarg_value).is_null()
                            && *((*cs).value_ptr as *mut libc::c_uint)
                                == *((*cs).noarg_value as *mut libc::c_uint)
                        {
                            let mut fresh65 = ::std::vec::from_elem(
                                0,
                                ::std::mem::size_of::<flag>() as libc::c_ulong as usize,
                            );
                            let mut new_0: *mut flag = fresh65.leak().as_mut_ptr()
                                as *mut flag;
                            (*new_0).cs = cs;
                            (*new_0).arg = b"\0" as *const u8 as *const libc::c_char;
                            (*new_0).next = 0 as *mut flag;
                            if flags.is_null() {
                                flags = new_0;
                            } else {
                                (*last).next = new_0;
                            }
                            last = new_0;
                            if ((*new_0).arg).is_null() {
                                flagslen = (flagslen as libc::c_ulong)
                                    .wrapping_add(3 as libc::c_int as libc::c_ulong) as size_t
                                    as size_t;
                            } else {
                                flagslen = (flagslen as libc::c_ulong)
                                    .wrapping_add(
                                        (1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                                            + 3 as libc::c_int * 0 as libc::c_int) as libc::c_ulong,
                                    ) as size_t as size_t;
                            }
                            if !((*cs).c <= 127 as libc::c_int) {
                                flagslen = (flagslen as libc::c_ulong)
                                    .wrapping_add(
                                        (2 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(strlen((*cs).long_name)),
                                    ) as size_t as size_t;
                            }
                        } else {
                            let mut fresh66 = ::std::vec::from_elem(
                                0,
                                30 as libc::c_int as libc::c_ulong as usize,
                            );
                            let mut buf: *mut libc::c_char = fresh66.leak().as_mut_ptr()
                                as *mut libc::c_char;
                            sprintf(
                                buf,
                                b"%u\0" as *const u8 as *const libc::c_char,
                                *((*cs).value_ptr as *mut libc::c_uint),
                            );
                            let mut fresh67 = ::std::vec::from_elem(
                                0,
                                ::std::mem::size_of::<flag>() as libc::c_ulong as usize,
                            );
                            let mut new_1: *mut flag = fresh67.leak().as_mut_ptr()
                                as *mut flag;
                            (*new_1).cs = cs;
                            (*new_1).arg = buf;
                            (*new_1).next = 0 as *mut flag;
                            if flags.is_null() {
                                flags = new_1;
                            } else {
                                (*last).next = new_1;
                            }
                            last = new_1;
                            if ((*new_1).arg).is_null() {
                                flagslen = (flagslen as libc::c_ulong)
                                    .wrapping_add(3 as libc::c_int as libc::c_ulong) as size_t
                                    as size_t;
                            } else {
                                flagslen = (flagslen as libc::c_ulong)
                                    .wrapping_add(
                                        ((1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int)
                                            as libc::c_ulong)
                                            .wrapping_add(
                                                (3 as libc::c_int as libc::c_ulong)
                                                    .wrapping_mul(strlen(buf)),
                                            ),
                                    ) as size_t as size_t;
                            }
                            if !((*cs).c <= 127 as libc::c_int) {
                                flagslen = (flagslen as libc::c_ulong)
                                    .wrapping_add(
                                        (2 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(strlen((*cs).long_name)),
                                    ) as size_t as size_t;
                            }
                        }
                    }
                }
                6 => {
                    if !(!((*cs).default_value).is_null()
                        && *((*cs).value_ptr as *mut libc::c_double)
                            == *((*cs).default_value as *mut libc::c_double))
                    {
                        if !((*cs).noarg_value).is_null()
                            && *((*cs).value_ptr as *mut libc::c_double)
                                == *((*cs).noarg_value as *mut libc::c_double)
                        {
                            let mut fresh68 = ::std::vec::from_elem(
                                0,
                                ::std::mem::size_of::<flag>() as libc::c_ulong as usize,
                            );
                            let mut new_2: *mut flag = fresh68.leak().as_mut_ptr()
                                as *mut flag;
                            (*new_2).cs = cs;
                            (*new_2).arg = b"\0" as *const u8 as *const libc::c_char;
                            (*new_2).next = 0 as *mut flag;
                            if flags.is_null() {
                                flags = new_2;
                            } else {
                                (*last).next = new_2;
                            }
                            last = new_2;
                            if ((*new_2).arg).is_null() {
                                flagslen = (flagslen as libc::c_ulong)
                                    .wrapping_add(3 as libc::c_int as libc::c_ulong) as size_t
                                    as size_t;
                            } else {
                                flagslen = (flagslen as libc::c_ulong)
                                    .wrapping_add(
                                        (1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                                            + 3 as libc::c_int * 0 as libc::c_int) as libc::c_ulong,
                                    ) as size_t as size_t;
                            }
                            if !((*cs).c <= 127 as libc::c_int) {
                                flagslen = (flagslen as libc::c_ulong)
                                    .wrapping_add(
                                        (2 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(strlen((*cs).long_name)),
                                    ) as size_t as size_t;
                            }
                        } else {
                            let mut fresh69 = ::std::vec::from_elem(
                                0,
                                100 as libc::c_int as libc::c_ulong as usize,
                            );
                            let mut buf_0: *mut libc::c_char = fresh69
                                .leak()
                                .as_mut_ptr() as *mut libc::c_char;
                            sprintf(
                                buf_0,
                                b"%g\0" as *const u8 as *const libc::c_char,
                                *((*cs).value_ptr as *mut libc::c_double),
                            );
                            let mut fresh70 = ::std::vec::from_elem(
                                0,
                                ::std::mem::size_of::<flag>() as libc::c_ulong as usize,
                            );
                            let mut new_3: *mut flag = fresh70.leak().as_mut_ptr()
                                as *mut flag;
                            (*new_3).cs = cs;
                            (*new_3).arg = buf_0;
                            (*new_3).next = 0 as *mut flag;
                            if flags.is_null() {
                                flags = new_3;
                            } else {
                                (*last).next = new_3;
                            }
                            last = new_3;
                            if ((*new_3).arg).is_null() {
                                flagslen = (flagslen as libc::c_ulong)
                                    .wrapping_add(3 as libc::c_int as libc::c_ulong) as size_t
                                    as size_t;
                            } else {
                                flagslen = (flagslen as libc::c_ulong)
                                    .wrapping_add(
                                        ((1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int)
                                            as libc::c_ulong)
                                            .wrapping_add(
                                                (3 as libc::c_int as libc::c_ulong)
                                                    .wrapping_mul(strlen(buf_0)),
                                            ),
                                    ) as size_t as size_t;
                            }
                            if !((*cs).c <= 127 as libc::c_int) {
                                flagslen = (flagslen as libc::c_ulong)
                                    .wrapping_add(
                                        (2 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(strlen((*cs).long_name)),
                                    ) as size_t as size_t;
                            }
                        }
                    }
                }
                2 => {
                    p = *((*cs).value_ptr as *mut *mut libc::c_char);
                    if !p.is_null() {
                        let mut fresh71 = ::std::vec::from_elem(
                            0,
                            ::std::mem::size_of::<flag>() as libc::c_ulong as usize,
                        );
                        let mut new_4: *mut flag = fresh71.leak().as_mut_ptr()
                            as *mut flag;
                        (*new_4).cs = cs;
                        (*new_4).arg = p;
                        (*new_4).next = 0 as *mut flag;
                        if flags.is_null() {
                            flags = new_4;
                        } else {
                            (*last).next = new_4;
                        }
                        last = new_4;
                        if ((*new_4).arg).is_null() {
                            flagslen = (flagslen as libc::c_ulong)
                                .wrapping_add(3 as libc::c_int as libc::c_ulong) as size_t
                                as size_t;
                        } else {
                            flagslen = (flagslen as libc::c_ulong)
                                .wrapping_add(
                                    ((1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int)
                                        as libc::c_ulong)
                                        .wrapping_add(
                                            (3 as libc::c_int as libc::c_ulong).wrapping_mul(strlen(p)),
                                        ),
                                ) as size_t as size_t;
                        }
                        if !((*cs).c <= 127 as libc::c_int) {
                            flagslen = (flagslen as libc::c_ulong)
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(strlen((*cs).long_name)),
                                ) as size_t as size_t;
                        }
                    }
                }
                4 | 3 => {
                    let mut sl: *mut stringlist = *((*cs).value_ptr
                        as *mut *mut stringlist);
                    if !sl.is_null() {
                        let mut i: libc::c_uint = 0;
                        i = 0 as libc::c_int as libc::c_uint;
                        while i < (*sl).idx {
                            let mut fresh72 = ::std::vec::from_elem(
                                0,
                                ::std::mem::size_of::<flag>() as libc::c_ulong as usize,
                            );
                            let mut new_5: *mut flag = fresh72.leak().as_mut_ptr()
                                as *mut flag;
                            (*new_5).cs = cs;
                            (*new_5).arg = *((*sl).list).offset(i as isize);
                            (*new_5).next = 0 as *mut flag;
                            if flags.is_null() {
                                flags = new_5;
                            } else {
                                (*last).next = new_5;
                            }
                            last = new_5;
                            if ((*new_5).arg).is_null() {
                                flagslen = (flagslen as libc::c_ulong)
                                    .wrapping_add(3 as libc::c_int as libc::c_ulong) as size_t
                                    as size_t;
                            } else {
                                flagslen = (flagslen as libc::c_ulong)
                                    .wrapping_add(
                                        ((1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int)
                                            as libc::c_ulong)
                                            .wrapping_add(
                                                (3 as libc::c_int as libc::c_ulong)
                                                    .wrapping_mul(strlen(*((*sl).list).offset(i as isize))),
                                            ),
                                    ) as size_t as size_t;
                            }
                            if !((*cs).c <= 127 as libc::c_int) {
                                flagslen = (flagslen as libc::c_ulong)
                                    .wrapping_add(
                                        (2 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(strlen((*cs).long_name)),
                                    ) as size_t as size_t;
                            }
                            i = i.wrapping_add(1);
                            i;
                        }
                    }
                }
                _ => {
                    abort();
                }
            }
        }
        cs = cs.offset(1);
        cs;
    }
    flagslen = (flagslen as libc::c_ulong)
        .wrapping_add(
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(4 as libc::c_int as libc::c_ulong),
        ) as size_t as size_t;
    let mut fresh73 = ::std::vec::from_elem(
        0,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_add(flagslen)
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
    );
    flagstring = fresh73.leak().as_mut_ptr() as *mut libc::c_char;
    memset(
        flagstring as *mut libc::c_void,
        '\0' as i32,
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_add(flagslen)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    p = flagstring;
    let fresh74 = p;
    p = p.offset(1);
    *fresh74 = '-' as i32 as libc::c_char;
    while !flags.is_null() && ((*flags).arg).is_null()
        && (*(*flags).cs).c <= 127 as libc::c_int
    {
        let fresh75 = p;
        p = p.offset(1);
        *fresh75 = (*(*flags).cs).c as libc::c_char;
        flags = (*flags).next;
    }
    while !flags.is_null() {
        let fresh76 = p;
        p = p.offset(1);
        *fresh76 = ' ' as i32 as libc::c_char;
        let fresh77 = p;
        p = p.offset(1);
        *fresh77 = '-' as i32 as libc::c_char;
        if (*(*flags).cs).c <= 127 as libc::c_int {
            let fresh78 = p;
            p = p.offset(1);
            *fresh78 = (*(*flags).cs).c as libc::c_char;
        } else {
            let fresh79 = p;
            p = p.offset(1);
            *fresh79 = '-' as i32 as libc::c_char;
            p = stpcpy(p, (*(*flags).cs).long_name);
        }
        if !((*flags).arg).is_null()
            && *((*flags).arg).offset(0 as libc::c_int as isize) as libc::c_int
                != '\0' as i32
        {
            if !((*(*flags).cs).c <= 127 as libc::c_int) {
                let fresh80 = p;
                p = p.offset(1);
                *fresh80 = '=' as i32 as libc::c_char;
            }
            p = quote_for_env(p, (*flags).arg);
        }
        flags = (*flags).next;
    }
    if p == &mut *flagstring.offset(1 as libc::c_int as isize) as *mut libc::c_char {
        *flagstring.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        p = flagstring;
    }
    define_variable_in_set(
        b"MFLAGS\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        flagstring
            .offset(
                (if *flagstring.offset(0 as libc::c_int as isize) as libc::c_int
                    == '-' as i32
                    && *flagstring.offset(1 as libc::c_int as isize) as libc::c_int
                        == ' ' as i32
                {
                    2 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as isize,
            ),
        o_env,
        1 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    if !eval_strings.is_null() {
        let fresh81 = p;
        p = p.offset(1);
        *fresh81 = ' ' as i32 as libc::c_char;
        p = mempcpy(
            p as *mut libc::c_void,
            evalref.as_ptr() as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
    }
    let mut r: *const libc::c_char = if posix_pedantic != 0 {
        posixref.as_ptr()
    } else {
        ref_0.as_ptr()
    };
    let mut l: size_t = strlen(r);
    v = lookup_variable(r, l);
    if !v.is_null() && !((*v).value).is_null()
        && *((*v).value).offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        p = stpcpy(p, b" -- \0" as *const u8 as *const libc::c_char);
        let fresh82 = p;
        p = p.offset(1);
        *fresh82 = '$' as i32 as libc::c_char;
        let fresh83 = p;
        p = p.offset(1);
        *fresh83 = '(' as i32 as libc::c_char;
        p = mempcpy(p as *mut libc::c_void, r as *const libc::c_void, l)
            as *mut libc::c_char;
        let fresh84 = p;
        p = p.offset(1);
        *fresh84 = ')' as i32 as libc::c_char;
    }
    if *flagstring.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
        flagstring = flagstring.offset(1);
        flagstring;
    }
    v = define_variable_in_set(
        b"MAKEFLAGS\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        flagstring,
        (if env_overrides != 0 {
            o_env_override as libc::c_int
        } else {
            o_file as libc::c_int
        }) as variable_origin,
        1 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    (*v).set_special(1 as libc::c_int as libc::c_uint);
    return v;
}
pub unsafe extern "C" fn should_print_dir() -> libc::c_int {
    if print_directory_flag >= 0 as libc::c_int {
        return print_directory_flag;
    }
    return (silent_flag == 0
        && (makelevel > 0 as libc::c_int as libc::c_uint || !directories.is_null()))
        as libc::c_int;
}
unsafe extern "C" fn print_version() {
    static mut printed_version: libc::c_int = 0 as libc::c_int;
    let mut precede: *const libc::c_char = if print_data_base_flag != 0 {
        b"# \0" as *const u8 as *const libc::c_char
    } else {
        b"\0" as *const u8 as *const libc::c_char
    };
    if printed_version != 0 {
        return;
    }
    printf(
        b"%sGNU Make %s\n\0" as *const u8 as *const libc::c_char,
        precede,
        version_string,
    );
    if remote_description.is_null() || *remote_description as libc::c_int == '\0' as i32
    {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"%sBuilt for %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            precede,
            make_host,
        );
    } else {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"%sBuilt for %s (%s)\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            precede,
            make_host,
            remote_description,
        );
    }
    printf(
        b"%sCopyright (C) 1988-2023 Free Software Foundation, Inc.\n\0" as *const u8
            as *const libc::c_char,
        precede,
    );
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"%sLicense GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>\n%sThis is free software: you are free to change and redistribute it.\n%sThere is NO WARRANTY, to the extent permitted by law.\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        precede,
        precede,
        precede,
    );
    printed_version = 1 as libc::c_int;
}
unsafe extern "C" fn print_data_base() {
    let mut when: time_t = time(0 as *mut time_t);
    print_version();
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"\n# Make data base, printed on %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        ctime(&mut when),
    );
    print_variable_data_base();
    print_dir_data_base();
    print_rule_data_base();
    print_file_data_base();
    print_vpath_data_base();
    strcache_print_stats(b"#\0" as *const u8 as *const libc::c_char);
    when = time(0 as *mut time_t);
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"\n# Finished Make data base on %s\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        ctime(&mut when),
    );
}
unsafe extern "C" fn clean_jobserver(mut status: libc::c_int) {
    if jobserver_enabled() != 0 && jobserver_tokens != 0 {
        if status != 2 as libc::c_int {
            error(
                0 as *mut floc,
                (53 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                    .wrapping_div(22 as libc::c_int as libc::c_ulong)
                    .wrapping_add(3 as libc::c_int as libc::c_ulong),
                b"INTERNAL: Exiting with %u jobserver tokens (should be 0)!\0"
                    as *const u8 as *const libc::c_char,
                jobserver_tokens,
            );
        } else {
            loop {
                jobserver_tokens = jobserver_tokens.wrapping_sub(1);
                if !(jobserver_tokens != 0) {
                    break;
                }
                jobserver_release(0 as libc::c_int);
            }
        }
    }
    if master_job_slots != 0 {
        let mut tokens: libc::c_uint = (1 as libc::c_int as libc::c_uint)
            .wrapping_add(jobserver_acquire_all());
        if tokens != master_job_slots {
            error(
                0 as *mut floc,
                (53 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                    .wrapping_div(22 as libc::c_int as libc::c_ulong)
                    .wrapping_add(3 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong),
                b"INTERNAL: Exiting with %u jobserver tokens available; should be %u!\0"
                    as *const u8 as *const libc::c_char,
                tokens,
                master_job_slots,
            );
        }
        reset_jobserver();
    }
}
pub unsafe extern "C" fn die(mut status: libc::c_int) -> ! {
    static mut dying: libc::c_char = 0 as libc::c_int as libc::c_char;
    if dying == 0 {
        let mut err: libc::c_int = 0;
        dying = 1 as libc::c_int as libc::c_char;
        if print_version_flag != 0 {
            print_version();
        }
        temp_stdin_unlink();
        err = (status != 0 as libc::c_int) as libc::c_int;
        while job_slots_used > 0 as libc::c_int as libc::c_uint {
            reap_children(1 as libc::c_int, err);
        }
        remote_cleanup();
        remove_intermediates(0 as libc::c_int);
        if print_data_base_flag != 0 {
            print_data_base();
        }
        if verify_flag != 0 {
            verify_file_data_base();
        }
        clean_jobserver(status);
        if !output_context.is_null() {
            output_close(output_context);
            if output_context != &mut make_sync as *mut output {
                output_close(&mut make_sync);
            }
            output_context = 0 as *mut output;
        }
        output_close(0 as *mut output);
        osync_clear();
        if !directory_before_chdir.is_null() {
            let mut _x: libc::c_int = 0;
            _x = chdir(directory_before_chdir);
        }
    }
    exit(status);
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
    let mut vars: Vec::<*mut libc::c_char> = Vec::new();
    for (var_name, var_value) in ::std::env::vars() {
        let var: String = format!("{}={}", var_name, var_value);
        vars.push((::std::ffi::CString::new(var)).unwrap().into_raw());
    }
    vars.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
                vars.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
unsafe extern "C" fn run_static_initializers() {
    switches = [
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'b' as i32,
                type_0: ignore,
                value_ptr: 0 as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: 0 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(0 as libc::c_int as libc::c_uint);
            init.set_toenv(0 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'B' as i32,
                type_0: flag,
                value_ptr: &mut always_make_set as *mut libc::c_int as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"always-make\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'd' as i32,
                type_0: flag,
                value_ptr: &mut debug_flag as *mut libc::c_int as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: 0 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'e' as i32,
                type_0: flag,
                value_ptr: &mut env_overrides as *mut libc::c_int as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"environment-overrides\0" as *const u8
                    as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'E' as i32,
                type_0: strlist,
                value_ptr: &mut eval_strings as *mut *mut stringlist
                    as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"eval\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(0 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'h' as i32,
                type_0: flag,
                value_ptr: &mut print_usage_flag as *mut libc::c_int
                    as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"help\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(0 as libc::c_int as libc::c_uint);
            init.set_toenv(0 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'i' as i32,
                type_0: flag,
                value_ptr: &mut ignore_errors_flag as *mut libc::c_int
                    as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"ignore-errors\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'k' as i32,
                type_0: flag,
                value_ptr: &mut keep_going_flag as *mut libc::c_int as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: &default_keep_going_flag as *const libc::c_int
                    as *const libc::c_void,
                long_name: b"keep-going\0" as *const u8 as *const libc::c_char,
                origin: &mut keep_going_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'L' as i32,
                type_0: flag,
                value_ptr: &mut check_symlink_flag as *mut libc::c_int
                    as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"check-symlink-times\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'm' as i32,
                type_0: ignore,
                value_ptr: 0 as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: 0 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(0 as libc::c_int as libc::c_uint);
            init.set_toenv(0 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'n' as i32,
                type_0: flag,
                value_ptr: &mut just_print_flag as *mut libc::c_int as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"just-print\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(1 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'p' as i32,
                type_0: flag,
                value_ptr: &mut print_data_base_flag as *mut libc::c_int
                    as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"print-data-base\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'q' as i32,
                type_0: flag,
                value_ptr: &mut question_flag as *mut libc::c_int as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"question\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(1 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'r' as i32,
                type_0: flag,
                value_ptr: &mut no_builtin_rules_flag as *mut libc::c_int
                    as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"no-builtin-rules\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'R' as i32,
                type_0: flag,
                value_ptr: &mut no_builtin_variables_flag as *mut libc::c_int
                    as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"no-builtin-variables\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 's' as i32,
                type_0: flag,
                value_ptr: &mut silent_flag as *mut libc::c_int as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: &default_silent_flag as *const libc::c_int
                    as *const libc::c_void,
                long_name: b"silent\0" as *const u8 as *const libc::c_char,
                origin: &mut silent_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'S' as i32,
                type_0: flag_off,
                value_ptr: &mut keep_going_flag as *mut libc::c_int as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: &default_keep_going_flag as *const libc::c_int
                    as *const libc::c_void,
                long_name: b"no-keep-going\0" as *const u8 as *const libc::c_char,
                origin: &mut keep_going_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 't' as i32,
                type_0: flag,
                value_ptr: &mut touch_flag as *mut libc::c_int as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"touch\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(1 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'v' as i32,
                type_0: flag,
                value_ptr: &mut print_version_flag as *mut libc::c_int
                    as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"version\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(0 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'w' as i32,
                type_0: flag,
                value_ptr: &mut print_directory_flag as *mut libc::c_int
                    as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: &default_print_directory_flag as *const libc::c_int
                    as *const libc::c_void,
                long_name: b"print-directory\0" as *const u8 as *const libc::c_char,
                origin: &mut print_directory_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'C' as i32,
                type_0: filename,
                value_ptr: &mut directories as *mut *mut stringlist as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"directory\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(0 as libc::c_int as libc::c_uint);
            init.set_toenv(0 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'f' as i32,
                type_0: filename,
                value_ptr: &mut makefiles as *mut *mut stringlist as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"file\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(0 as libc::c_int as libc::c_uint);
            init.set_toenv(0 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'I' as i32,
                type_0: filename,
                value_ptr: &mut include_dirs as *mut *mut stringlist
                    as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"include-dir\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'j' as i32,
                type_0: positive_int,
                value_ptr: &mut arg_job_slots as *mut libc::c_int as *mut libc::c_void,
                noarg_value: &inf_jobs as *const libc::c_int as *const libc::c_void,
                default_value: &default_job_slots as *const libc::c_int
                    as *const libc::c_void,
                long_name: b"jobs\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'l' as i32,
                type_0: floating,
                value_ptr: &mut max_load_average as *mut libc::c_double
                    as *mut libc::c_void,
                noarg_value: &mut default_load_average as *mut libc::c_double
                    as *const libc::c_void,
                default_value: &mut default_load_average as *mut libc::c_double
                    as *const libc::c_void,
                long_name: b"load-average\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'o' as i32,
                type_0: filename,
                value_ptr: &mut old_files as *mut *mut stringlist as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"old-file\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(0 as libc::c_int as libc::c_uint);
            init.set_toenv(0 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'O' as i32,
                type_0: string,
                value_ptr: &mut output_sync_option as *mut *mut libc::c_char
                    as *mut libc::c_void,
                noarg_value: b"target\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"output-sync\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 'W' as i32,
                type_0: filename,
                value_ptr: &mut new_files as *mut *mut stringlist as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"what-if\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(0 as libc::c_int as libc::c_uint);
            init.set_toenv(0 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 127 as libc::c_int + 1 as libc::c_int,
                type_0: strlist,
                value_ptr: &mut db_flags as *mut *mut stringlist as *mut libc::c_void,
                noarg_value: b"basic\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"debug\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 127 as libc::c_int + 2 as libc::c_int,
                type_0: string,
                value_ptr: &mut jobserver_auth as *mut *mut libc::c_char
                    as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"jobserver-auth\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 127 as libc::c_int + 3 as libc::c_int,
                type_0: flag,
                value_ptr: &mut trace_flag as *mut libc::c_int as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"trace\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 127 as libc::c_int + 4 as libc::c_int,
                type_0: flag_off,
                value_ptr: &mut print_directory_flag as *mut libc::c_int
                    as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: &default_print_directory_flag as *const libc::c_int
                    as *const libc::c_void,
                long_name: b"no-print-directory\0" as *const u8 as *const libc::c_char,
                origin: &mut print_directory_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 127 as libc::c_int + 5 as libc::c_int,
                type_0: flag,
                value_ptr: &mut warn_undefined_variables_flag as *mut libc::c_int
                    as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"warn-undefined-variables\0" as *const u8
                    as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 127 as libc::c_int + 7 as libc::c_int,
                type_0: string,
                value_ptr: &mut sync_mutex as *mut *mut libc::c_char
                    as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"sync-mutex\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 127 as libc::c_int + 8 as libc::c_int,
                type_0: flag_off,
                value_ptr: &mut silent_flag as *mut libc::c_int as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: &default_silent_flag as *const libc::c_int
                    as *const libc::c_void,
                long_name: b"no-silent\0" as *const u8 as *const libc::c_char,
                origin: &mut silent_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 127 as libc::c_int + 9 as libc::c_int,
                type_0: string,
                value_ptr: &mut jobserver_auth as *mut *mut libc::c_char
                    as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"jobserver-fds\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(0 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 127 as libc::c_int + 10 as libc::c_int,
                type_0: filename,
                value_ptr: &mut makefiles as *mut *mut stringlist as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"temp-stdin\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(0 as libc::c_int as libc::c_uint);
            init.set_toenv(0 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 127 as libc::c_int + 11 as libc::c_int,
                type_0: string,
                value_ptr: &mut shuffle_mode as *mut *mut libc::c_char
                    as *mut libc::c_void,
                noarg_value: b"random\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"shuffle\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(1 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 127 as libc::c_int + 12 as libc::c_int,
                type_0: string,
                value_ptr: &mut jobserver_style as *mut *mut libc::c_char
                    as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: b"jobserver-style\0" as *const u8 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(1 as libc::c_int as libc::c_uint);
            init.set_toenv(0 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = command_switch {
                env_toenv_no_makefile_specified: [0; 1],
                c2rust_padding: [0; 7],
                c: 0 as libc::c_int,
                type_0: flag,
                value_ptr: 0 as *mut libc::c_void,
                noarg_value: 0 as *const libc::c_void,
                default_value: 0 as *const libc::c_void,
                long_name: 0 as *const libc::c_char,
                origin: 0 as *mut variable_origin,
            };
            init.set_env(0 as libc::c_int as libc::c_uint);
            init.set_toenv(0 as libc::c_int as libc::c_uint);
            init.set_no_makefile(0 as libc::c_int as libc::c_uint);
            init.set_specified(0 as libc::c_int as libc::c_uint);
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
