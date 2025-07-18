use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type dirent;
    fn getlogin() -> *mut libc::c_char;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn glob(
        __pattern: *const libc::c_char,
        __flags: libc::c_int,
        __errfunc: Option::<
            unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int,
        >,
        __pglob: *mut glob_t,
    ) -> libc::c_int;
    fn globfree(__pglob: *mut glob_t);
    fn concat(_: libc::c_uint, _: ...) -> *const libc::c_char;
    fn error(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...) -> !;
    fn out_of_memory() -> !;
    fn pfatal_with_name(_: *const libc::c_char) -> !;
    fn perror_with_name(_: *const libc::c_char, _: *const libc::c_char);
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn xstrndup(_: *const libc::c_char, _: size_t) -> *mut libc::c_char;
    fn find_next_token(_: *mut *const libc::c_char, _: *mut size_t) -> *mut libc::c_char;
    fn next_token(_: *const libc::c_char) -> *mut libc::c_char;
    fn end_of_token(_: *const libc::c_char) -> *mut libc::c_char;
    fn collapse_continuations(_: *mut libc::c_char);
    fn strcache_add(str: *const libc::c_char) -> *const libc::c_char;
    fn ar_name(_: *const libc::c_char) -> libc::c_int;
    fn ar_parse_name(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: *mut *mut libc::c_char,
    );
    fn file_exists_p(_: *const libc::c_char) -> libc::c_int;
    fn dir_setup_glob(_: *mut glob_t);
    fn construct_vpath_list(pattern: *mut libc::c_char, dirpath: *mut libc::c_char);
    fn strcache_add_len(str: *const libc::c_char, len: size_t) -> *const libc::c_char;
    static mut warn_undefined_variables_flag: libc::c_int;
    fn strip_whitespace(
        begpp: *mut *const libc::c_char,
        endpp: *mut *const libc::c_char,
    ) -> *mut libc::c_char;
    fn load_file(
        flocp: *const floc,
        file: *mut file,
        noerror: libc::c_int,
    ) -> libc::c_int;
    static mut stopchar_map: [libc::c_ushort; 0];
    static mut posix_pedantic: libc::c_int;
    static mut second_expansion: libc::c_int;
    static mut one_shell: libc::c_int;
    static mut cmd_prefix: libc::c_char;
    static mut default_file: *mut file;
    fn lookup_file(name: *const libc::c_char) -> *mut file;
    fn enter_file(name: *const libc::c_char) -> *mut file;
    fn split_prereqs(prereqstr: *mut libc::c_char) -> *mut dep;
    fn enter_prereqs(prereqs: *mut dep, stem: *const libc::c_char) -> *mut dep;
    static mut snapped_deps: libc::c_int;
    fn ar_glob(
        arname: *const libc::c_char,
        member_pattern: *const libc::c_char,
        size: size_t,
    ) -> *mut nameseq;
    fn free_ns_chain(n: *mut nameseq);
    fn copy_dep_chain(d: *const dep) -> *mut dep;
    fn fd_noinherit(_: libc::c_int);
    static mut variable_buffer: *mut libc::c_char;
    static mut current_variable_set_list: *mut variable_set_list;
    static mut default_goal_var: *mut variable;
    fn variable_buffer_output(
        ptr: *mut libc::c_char,
        string: *const libc::c_char,
        length: size_t,
    ) -> *mut libc::c_char;
    fn variable_expand(line: *const libc::c_char) -> *mut libc::c_char;
    fn allocated_variable_expand_for_file(
        line: *const libc::c_char,
        file: *mut file,
    ) -> *mut libc::c_char;
    fn variable_expand_string(
        line: *mut libc::c_char,
        string: *const libc::c_char,
        length: size_t,
    ) -> *mut libc::c_char;
    fn pattern_matches(
        pattern: *const libc::c_char,
        percent: *const libc::c_char,
        str: *const libc::c_char,
    ) -> libc::c_int;
    fn patsubst_expand_pat(
        o: *mut libc::c_char,
        text: *const libc::c_char,
        pattern: *const libc::c_char,
        replace: *const libc::c_char,
        pattern_percent: *const libc::c_char,
        replace_percent: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn initialize_file_variables(file: *mut file, reading: libc::c_int);
    fn do_variable_definition(
        flocp: *const floc,
        name: *const libc::c_char,
        value: *const libc::c_char,
        origin: variable_origin,
        flavor: variable_flavor,
        target_var: libc::c_int,
    ) -> *mut variable;
    fn parse_variable_definition(
        line: *const libc::c_char,
        v: *mut variable,
    ) -> *mut libc::c_char;
    fn assign_variable_definition(
        v: *mut variable,
        line: *const libc::c_char,
    ) -> *mut variable;
    fn try_variable_definition(
        flocp: *const floc,
        line: *const libc::c_char,
        origin: variable_origin,
        target_var: libc::c_int,
    ) -> *mut variable;
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
    fn undefine_variable_in_set(
        name: *const libc::c_char,
        length: size_t,
        origin: variable_origin,
        set: *mut variable_set,
    );
    fn create_pattern_var(
        target: *const libc::c_char,
        suffix: *const libc::c_char,
    ) -> *mut pattern_var;
    static mut export_all_variables: libc::c_int;
    static mut suffix_file: *mut file;
    fn create_pattern_rule(
        targets: *mut *const libc::c_char,
        target_percents: *mut *const libc::c_char,
        num: libc::c_ushort,
        terminal: libc::c_int,
        deps: *mut dep,
        commands: *mut commands,
        override_0: libc::c_int,
    );
    static mut db_level: libc::c_int;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
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
pub type __size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut libc::c_char,
    pub gl_offs: __size_t,
    pub gl_flags: libc::c_int,
    pub gl_closedir: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub gl_readdir: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut dirent>,
    pub gl_opendir: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void,
    >,
    pub gl_lstat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
    pub gl_stat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
}
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
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const libc::c_char,
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
pub struct ebuffer {
    pub buffer: *mut libc::c_char,
    pub bufnext: *mut libc::c_char,
    pub bufstart: *mut libc::c_char,
    pub size: size_t,
    pub fp: *mut FILE,
    pub floc: floc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conditionals {
    pub if_cmds: libc::c_uint,
    pub allocated: libc::c_uint,
    pub ignoring: *mut libc::c_char,
    pub seen_else: *mut libc::c_char,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct vmodifiers {
    #[bitfield(name = "assign_v", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "define_v", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "undefine_v", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "override_v", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "private_v", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "export_v", ty = "variable_export", bits = "5..=6")]
    pub assign_v_define_v_undefine_v_override_v_private_v_export_v: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pattern_var {
    pub next: *mut pattern_var,
    pub suffix: *const libc::c_char,
    pub target: *const libc::c_char,
    pub len: size_t,
    pub variable: variable,
}
pub const w_eol: make_word_type = 1;
pub type make_word_type = libc::c_uint;
pub const w_ampdcolon: make_word_type = 9;
pub const w_ampcolon: make_word_type = 8;
pub const w_varassign: make_word_type = 7;
pub const w_semicolon: make_word_type = 6;
pub const w_dcolon: make_word_type = 5;
pub const w_colon: make_word_type = 4;
pub const w_variable: make_word_type = 3;
pub const w_static: make_word_type = 2;
pub const w_bogus: make_word_type = 0;
pub const c_ifneq: C2RustUnnamed = 3;
pub type C2RustUnnamed = libc::c_uint;
pub const c_endif: C2RustUnnamed = 5;
pub const c_else: C2RustUnnamed = 4;
pub const c_ifeq: C2RustUnnamed = 2;
pub const c_ifndef: C2RustUnnamed = 1;
pub const c_ifdef: C2RustUnnamed = 0;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
static mut toplevel_conditionals: conditionals = conditionals {
    if_cmds: 0,
    allocated: 0,
    ignoring: 0 as *const libc::c_char as *mut libc::c_char,
    seen_else: 0 as *const libc::c_char as *mut libc::c_char,
};
static mut conditionals: *mut conditionals = unsafe {
    &toplevel_conditionals as *const conditionals as *mut conditionals
};
static mut default_include_directories: [*const libc::c_char; 4] = [
    b"/usr/gnu/include\0" as *const u8 as *const libc::c_char,
    b"/usr/local/include\0" as *const u8 as *const libc::c_char,
    b"/usr/include\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut include_directories: *mut *const libc::c_char = 0
    as *const *const libc::c_char as *mut *const libc::c_char;
static mut max_incl_len: size_t = 0;
pub static mut reading_file: *const floc = 0 as *const floc;
static mut read_files: *mut goaldep = 0 as *const goaldep as *mut goaldep;
pub unsafe extern "C" fn read_all_makefiles(
    mut makefiles: *mut *const libc::c_char,
) -> *mut goaldep {
    let mut num_makefiles: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    define_variable_in_set(
        b"MAKEFILE_LIST\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"\0" as *const u8 as *const libc::c_char,
        o_file,
        0 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    if 0x1 as libc::c_int & db_level != 0 {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Reading makefiles...\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fflush(stdout);
    }
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut length: size_t = 0;
    value = allocated_variable_expand_for_file(
        b"$(MAKEFILES)\0" as *const u8 as *const libc::c_char,
        0 as *mut file,
    );
    p = value;
    loop {
        name = find_next_token(
            &mut p as *mut *mut libc::c_char as *mut *const libc::c_char,
            &mut length,
        );
        if name.is_null() {
            break;
        }
        if *p as libc::c_int != '\0' as i32 {
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = '\0' as i32 as libc::c_char;
        }
        eval_makefile(
            strcache_add(name),
            ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int) as libc::c_ushort,
        );
    }
    free(value as *mut libc::c_void);
    if !makefiles.is_null() {
        while !(*makefiles).is_null() {
            let mut d: *mut goaldep = eval_makefile(
                *makefiles,
                0 as libc::c_int as libc::c_ushort,
            );
            if *__errno_location() != 0 {
                perror_with_name(b"\0" as *const u8 as *const libc::c_char, *makefiles);
            }
            *makefiles = if !((*d).name).is_null() {
                (*d).name
            } else {
                (*(*d).file).name
            };
            num_makefiles = num_makefiles.wrapping_add(1);
            num_makefiles;
            makefiles = makefiles.offset(1);
            makefiles;
        }
    }
    if num_makefiles == 0 as libc::c_int as libc::c_uint {
        static mut default_makefiles: [*const libc::c_char; 4] = [
            b"GNUmakefile\0" as *const u8 as *const libc::c_char,
            b"makefile\0" as *const u8 as *const libc::c_char,
            b"Makefile\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
        ];
        let mut p_0: *mut *const libc::c_char = default_makefiles.as_mut_ptr();
        while !(*p_0).is_null() && file_exists_p(*p_0) == 0 {
            p_0 = p_0.offset(1);
            p_0;
        }
        if !(*p_0).is_null() {
            eval_makefile(*p_0, 0 as libc::c_int as libc::c_ushort);
            if *__errno_location() != 0 {
                perror_with_name(b"\0" as *const u8 as *const libc::c_char, *p_0);
            }
        } else {
            p_0 = default_makefiles.as_mut_ptr();
            while !(*p_0).is_null() {
                let mut d_0: *mut goaldep = xcalloc(
                    ::std::mem::size_of::<goaldep>() as libc::c_ulong,
                ) as *mut goaldep;
                (*d_0).file = enter_file(strcache_add(*p_0));
                (*d_0)
                    .set_flags(((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint);
                (*d_0).next = read_files;
                read_files = d_0;
                p_0 = p_0.offset(1);
                p_0;
            }
        }
    }
    return read_files;
}
unsafe extern "C" fn install_conditionals(
    mut new: *mut conditionals,
) -> *mut conditionals {
    let mut save: *mut conditionals = conditionals;
    memset(
        new as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<conditionals>() as libc::c_ulong,
    );
    conditionals = new;
    return save;
}
unsafe extern "C" fn restore_conditionals(mut saved: *mut conditionals) {
    free((*conditionals).ignoring as *mut libc::c_void);
    free((*conditionals).seen_else as *mut libc::c_void);
    conditionals = saved;
}
unsafe extern "C" fn eval_makefile(
    mut filename: *const libc::c_char,
    mut flags: libc::c_ushort,
) -> *mut goaldep {
    let mut deps: *mut goaldep = 0 as *mut goaldep;
    let mut ebuf: ebuffer = ebuffer {
        buffer: 0 as *mut libc::c_char,
        bufnext: 0 as *mut libc::c_char,
        bufstart: 0 as *mut libc::c_char,
        size: 0,
        fp: 0 as *mut FILE,
        floc: floc {
            filenm: 0 as *const libc::c_char,
            lineno: 0,
            offset: 0,
        },
    };
    let mut curfile: *const floc = 0 as *const floc;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    deps = xcalloc(::std::mem::size_of::<goaldep>() as libc::c_ulong) as *mut goaldep;
    (*deps).next = read_files;
    read_files = deps;
    ebuf.floc.filenm = filename;
    ebuf.floc.lineno = 1 as libc::c_int as libc::c_ulong;
    ebuf.floc.offset = 0 as libc::c_int as libc::c_ulong;
    if 0x2 as libc::c_int & db_level != 0 {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Reading makefile '%s'\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
        );
        if flags as libc::c_int & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b" (no default goal)\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if flags as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b" (search path)\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if flags as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b" (don't care)\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if flags as libc::c_int & (1 as libc::c_int) << 3 as libc::c_int != 0 {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b" (no ~ expansion)\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        puts(b"...\0" as *const u8 as *const libc::c_char);
    }
    if flags as libc::c_int & (1 as libc::c_int) << 3 as libc::c_int == 0
        && *filename.offset(0 as libc::c_int as isize) as libc::c_int == '~' as i32
    {
        expanded = tilde_expand(filename);
        if !expanded.is_null() {
            filename = expanded;
        }
    }
    *__errno_location() = 0 as libc::c_int;
    loop {
        *__errno_location() = 0 as libc::c_int;
        ebuf.fp = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
        if !((ebuf.fp).is_null() && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    (*deps).error = *__errno_location();
    match (*deps).error {
        24 | 23 | 12 => {
            let mut err: *const libc::c_char = strerror((*deps).error);
            fatal(
                reading_file,
                strlen(err),
                b"%s\0" as *const u8 as *const libc::c_char,
                err,
            );
        }
        _ => {}
    }
    if (ebuf.fp).is_null() && (*deps).error == 2 as libc::c_int
        && !include_directories.is_null()
        && flags as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int
            != 0 as libc::c_int && 0 as libc::c_int == 0
        && !(*stopchar_map.as_mut_ptr().offset(*filename as libc::c_uchar as isize)
            as libc::c_int & 0x8000 as libc::c_int != 0 as libc::c_int)
    {
        let mut dir: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
        dir = include_directories;
        while !(*dir).is_null() {
            let mut included: *const libc::c_char = concat(
                3 as libc::c_int as libc::c_uint,
                *dir,
                b"/\0" as *const u8 as *const libc::c_char,
                filename,
            );
            loop {
                *__errno_location() = 0 as libc::c_int;
                ebuf.fp = fopen(included, b"r\0" as *const u8 as *const libc::c_char);
                if !((ebuf.fp).is_null() && *__errno_location() == 4 as libc::c_int) {
                    break;
                }
            }
            if !(ebuf.fp).is_null() {
                filename = included;
                break;
            } else if *__errno_location() != 2 as libc::c_int {
                filename = included;
                (*deps).error = *__errno_location();
                break;
            } else {
                dir = dir.offset(1);
                dir;
            }
        }
    }
    filename = strcache_add(filename);
    (*deps).file = lookup_file(filename);
    if ((*deps).file).is_null() {
        (*deps).file = enter_file(filename);
    }
    filename = (*(*deps).file).name;
    (*deps).set_flags(flags as libc::c_uint);
    (*(*deps).file).set_is_explicit(1 as libc::c_int as libc::c_uint);
    free(expanded as *mut libc::c_void);
    if (ebuf.fp).is_null() {
        *__errno_location() = (*deps).error;
        (*(*deps).file).last_mtime = 1 as libc::c_int as uintmax_t;
        return deps;
    }
    (*deps).error = 0 as libc::c_int;
    if (*(*deps).file).last_mtime == 1 as libc::c_int as libc::c_ulong {
        (*(*deps).file).last_mtime = 0 as libc::c_int as uintmax_t;
    }
    fd_noinherit(fileno(ebuf.fp));
    do_variable_definition(
        &mut ebuf.floc,
        b"MAKEFILE_LIST\0" as *const u8 as *const libc::c_char,
        filename,
        o_file,
        f_append_value,
        0 as libc::c_int,
    );
    ebuf.size = 200 as libc::c_int as size_t;
    ebuf.bufstart = xmalloc(ebuf.size) as *mut libc::c_char;
    ebuf.bufnext = ebuf.bufstart;
    ebuf.buffer = ebuf.bufnext;
    curfile = reading_file;
    reading_file = &mut ebuf.floc;
    eval(
        &mut ebuf,
        (flags as libc::c_int & (1 as libc::c_int) << 0 as libc::c_int == 0)
            as libc::c_int,
    );
    reading_file = curfile;
    fclose(ebuf.fp);
    free(ebuf.bufstart as *mut libc::c_void);
    *__errno_location() = 0 as libc::c_int;
    return deps;
}
pub unsafe extern "C" fn eval_buffer(
    mut buffer: *mut libc::c_char,
    mut flocp: *const floc,
) {
    let mut ebuf: ebuffer = ebuffer {
        buffer: 0 as *mut libc::c_char,
        bufnext: 0 as *mut libc::c_char,
        bufstart: 0 as *mut libc::c_char,
        size: 0,
        fp: 0 as *mut FILE,
        floc: floc {
            filenm: 0 as *const libc::c_char,
            lineno: 0,
            offset: 0,
        },
    };
    let mut saved: *mut conditionals = 0 as *mut conditionals;
    let mut new: conditionals = conditionals {
        if_cmds: 0,
        allocated: 0,
        ignoring: 0 as *const libc::c_char as *mut libc::c_char,
        seen_else: 0 as *const libc::c_char as *mut libc::c_char,
    };
    let mut curfile: *const floc = 0 as *const floc;
    ebuf.size = strlen(buffer);
    ebuf.bufstart = buffer;
    ebuf.bufnext = ebuf.bufstart;
    ebuf.buffer = ebuf.bufnext;
    ebuf.fp = 0 as *mut FILE;
    if !flocp.is_null() {
        ebuf.floc = *flocp;
    } else if !reading_file.is_null() {
        ebuf.floc = *reading_file;
    } else {
        ebuf.floc.filenm = 0 as *const libc::c_char;
        ebuf.floc.lineno = 1 as libc::c_int as libc::c_ulong;
        ebuf.floc.offset = 0 as libc::c_int as libc::c_ulong;
    }
    curfile = reading_file;
    reading_file = &mut ebuf.floc;
    saved = install_conditionals(&mut new);
    eval(&mut ebuf, 1 as libc::c_int);
    restore_conditionals(saved);
    reading_file = curfile;
}
unsafe extern "C" fn parse_var_assignment(
    mut line: *const libc::c_char,
    mut targvar: libc::c_int,
    mut vmod: *mut vmodifiers,
) -> *mut libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    memset(
        vmod as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<vmodifiers>() as libc::c_ulong,
    );
    while *stopchar_map.as_mut_ptr().offset(*line as libc::c_uchar as isize)
        as libc::c_int & (0x2 as libc::c_int | 0x4 as libc::c_int) != 0 as libc::c_int
    {
        line = line.offset(1);
        line;
    }
    if *line as libc::c_int == '\0' as i32 {
        return line as *mut libc::c_char;
    }
    p = line;
    loop {
        let mut wlen: size_t = 0;
        let mut p2: *const libc::c_char = 0 as *const libc::c_char;
        let mut v: variable = variable {
            name: 0 as *mut libc::c_char,
            value: 0 as *mut libc::c_char,
            fileinfo: floc {
                filenm: 0 as *const libc::c_char,
                lineno: 0,
                offset: 0,
            },
            length: 0,
            recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
        };
        p2 = parse_variable_definition(p, &mut v);
        if !p2.is_null() {
            break;
        }
        p2 = end_of_token(p);
        wlen = p2.offset_from(p) as libc::c_long as size_t;
        if wlen
            == (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && memcmp(
                b"export\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                p as *const libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
        {
            (*vmod).set_export_v(v_export);
        } else if wlen
            == (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && memcmp(
                b"unexport\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                p as *const libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
        {
            (*vmod).set_export_v(v_noexport);
        } else if wlen
            == (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && memcmp(
                b"override\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                p as *const libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
        {
            (*vmod).set_override_v(1 as libc::c_int as libc::c_uint);
        } else if wlen
            == (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && memcmp(
                b"private\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                p as *const libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
        {
            (*vmod).set_private_v(1 as libc::c_int as libc::c_uint);
        } else if targvar == 0
            && (wlen
                == (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                && memcmp(
                    b"define\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    p as *const libc::c_void,
                    (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) == 0 as libc::c_int)
        {
            (*vmod).set_define_v(1 as libc::c_int as libc::c_uint);
            p = next_token(p2);
            break;
        } else if targvar == 0
            && (wlen
                == (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                && memcmp(
                    b"undefine\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    p as *const libc::c_void,
                    (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) == 0 as libc::c_int)
        {
            (*vmod).set_undefine_v(1 as libc::c_int as libc::c_uint);
            p = next_token(p2);
            break;
        } else {
            return line as *mut libc::c_char
        }
        p = next_token(p2);
        if *p as libc::c_int == '\0' as i32 {
            return line as *mut libc::c_char;
        }
    }
    (*vmod).set_assign_v(1 as libc::c_int as libc::c_uint);
    return p as *mut libc::c_char;
}
unsafe extern "C" fn eval(mut ebuf: *mut ebuffer, mut set_default: libc::c_int) {
    let mut collapsed: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut collapsed_length: size_t = 0 as libc::c_int as size_t;
    let mut commands_len: size_t = 200 as libc::c_int as size_t;
    let mut commands: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut commands_idx: size_t = 0 as libc::c_int as size_t;
    let mut cmds_started: libc::c_uint = 0;
    let mut tgts_started: libc::c_uint = 0;
    let mut ignoring: libc::c_int = 0 as libc::c_int;
    let mut in_ignored_define: libc::c_int = 0 as libc::c_int;
    let mut no_targets: libc::c_int = 0 as libc::c_int;
    let mut also_make_targets: libc::c_int = 0 as libc::c_int;
    let mut filenames: *mut nameseq = 0 as *mut nameseq;
    let mut depstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nlines: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut two_colon: libc::c_int = 0 as libc::c_int;
    let mut prefix: libc::c_char = cmd_prefix;
    let mut pattern: *const libc::c_char = 0 as *const libc::c_char;
    let mut pattern_percent: *const libc::c_char = 0 as *const libc::c_char;
    let mut fstart: *mut floc = 0 as *mut floc;
    let mut fi: floc = floc {
        filenm: 0 as *const libc::c_char,
        lineno: 0,
        offset: 0,
    };
    pattern_percent = 0 as *const libc::c_char;
    tgts_started = 1 as libc::c_int as libc::c_uint;
    cmds_started = tgts_started;
    fstart = &mut (*ebuf).floc;
    fi.filenm = (*ebuf).floc.filenm;
    commands = xmalloc(200 as libc::c_int as size_t) as *mut libc::c_char;
    loop {
        let mut linelen: size_t = 0;
        let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut wlen: size_t = 0;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut vmod: vmodifiers = vmodifiers {
            assign_v_define_v_undefine_v_override_v_private_v_export_v: [0; 1],
            c2rust_padding: [0; 3],
        };
        (*ebuf)
            .floc
            .lineno = ((*ebuf).floc.lineno).wrapping_add(nlines as libc::c_ulong);
        nlines = readline(ebuf);
        if nlines < 0 as libc::c_int as libc::c_long {
            break;
        }
        line = (*ebuf).buffer;
        if (*ebuf).floc.lineno == 1 as libc::c_int as libc::c_ulong {
            let mut ul: *mut libc::c_uchar = line as *mut libc::c_uchar;
            if *ul.offset(0 as libc::c_int as isize) as libc::c_int
                == 0xef as libc::c_int
                && *ul.offset(1 as libc::c_int as isize) as libc::c_int
                    == 0xbb as libc::c_int
                && *ul.offset(2 as libc::c_int as isize) as libc::c_int
                    == 0xbf as libc::c_int
            {
                line = line.offset(3 as libc::c_int as isize);
                if 0x1 as libc::c_int & db_level != 0 {
                    if !((*ebuf).floc.filenm).is_null() {
                        printf(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Skipping UTF-8 BOM in makefile '%s'\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*ebuf).floc.filenm,
                        );
                    } else {
                        printf(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Skipping UTF-8 BOM in makefile buffer\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                }
            }
        }
        if *line.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
            continue;
        }
        linelen = strlen(line);
        if *line.offset(0 as libc::c_int as isize) as libc::c_int
            == cmd_prefix as libc::c_int
        {
            if no_targets != 0 {
                continue;
            } else if !filenames.is_null() {
                if ignoring != 0 {
                    continue;
                } else {
                    if commands_idx == 0 as libc::c_int as libc::c_ulong {
                        cmds_started = (*ebuf).floc.lineno as libc::c_uint;
                    }
                    if linelen.wrapping_add(commands_idx) > commands_len {
                        commands_len = linelen
                            .wrapping_add(commands_idx)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong);
                        commands = xrealloc(commands as *mut libc::c_void, commands_len)
                            as *mut libc::c_char;
                    }
                    memcpy(
                        &mut *commands.offset(commands_idx as isize) as *mut libc::c_char
                            as *mut libc::c_void,
                        line.offset(1 as libc::c_int as isize) as *const libc::c_void,
                        linelen.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    commands_idx = (commands_idx as libc::c_ulong)
                        .wrapping_add(
                            linelen.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ) as size_t as size_t;
                    let fresh1 = commands_idx;
                    commands_idx = commands_idx.wrapping_add(1);
                    *commands.offset(fresh1 as isize) = '\n' as i32 as libc::c_char;
                    continue;
                }
            }
        }
        if collapsed_length < linelen.wrapping_add(1 as libc::c_int as libc::c_ulong) {
            collapsed_length = linelen.wrapping_add(1 as libc::c_int as libc::c_ulong);
            free(collapsed as *mut libc::c_void);
            collapsed = xmalloc(collapsed_length) as *mut libc::c_char;
        }
        strcpy(collapsed, line);
        collapse_continuations(collapsed);
        remove_comments(collapsed);
        p = collapsed;
        while *stopchar_map.as_mut_ptr().offset(*p as libc::c_uchar as isize)
            as libc::c_int & (0x2 as libc::c_int | 0x4 as libc::c_int)
            != 0 as libc::c_int
        {
            p = p.offset(1);
            p;
        }
        p = parse_var_assignment(p, 0 as libc::c_int, &mut vmod);
        if vmod.assign_v() != 0 {
            let mut v: *mut variable = 0 as *mut variable;
            let mut origin: variable_origin = (if vmod.override_v() as libc::c_int != 0 {
                o_override as libc::c_int
            } else {
                o_file as libc::c_int
            }) as variable_origin;
            if ignoring != 0 {
                if vmod.define_v() != 0 {
                    in_ignored_define = 1 as libc::c_int;
                }
            } else {
                if !filenames.is_null() {
                    fi.lineno = tgts_started as libc::c_ulong;
                    fi.offset = 0 as libc::c_int as libc::c_ulong;
                    record_files(
                        filenames,
                        also_make_targets,
                        pattern,
                        pattern_percent,
                        depstr,
                        cmds_started,
                        commands,
                        commands_idx,
                        two_colon,
                        prefix,
                        &mut fi,
                    );
                    filenames = 0 as *mut nameseq;
                }
                commands_idx = 0 as libc::c_int as size_t;
                no_targets = 0 as libc::c_int;
                pattern = 0 as *const libc::c_char;
                also_make_targets = 0 as libc::c_int;
                if vmod.undefine_v() != 0 {
                    do_undefine(p, origin, ebuf);
                } else {
                    if vmod.define_v() != 0 {
                        v = do_define(p, origin, ebuf);
                    } else {
                        v = try_variable_definition(fstart, p, origin, 0 as libc::c_int);
                    }
                    if vmod.export_v() as libc::c_int != v_default as libc::c_int {
                        (*v).set_export(vmod.export_v());
                    }
                    if vmod.private_v() != 0 {
                        (*v).set_private_var(1 as libc::c_int as libc::c_uint);
                    }
                }
            }
        } else {
            if *p as libc::c_int == '\0' as i32 {
                continue;
            }
            p2 = end_of_token(p);
            wlen = p2.offset_from(p) as libc::c_long as size_t;
            while *stopchar_map.as_mut_ptr().offset(*p2 as libc::c_uchar as isize)
                as libc::c_int & (0x2 as libc::c_int | 0x4 as libc::c_int)
                != 0 as libc::c_int
            {
                p2 = p2.offset(1);
                p2;
            }
            if in_ignored_define != 0 {
                if wlen
                    == (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    && memcmp(
                        b"endef\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        p as *const libc::c_void,
                        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0 as libc::c_int
                    && *stopchar_map.as_mut_ptr().offset(*p2 as libc::c_uchar as isize)
                        as libc::c_int & (0x8 as libc::c_int | 0x1 as libc::c_int)
                        != 0 as libc::c_int
                {
                    in_ignored_define = 0 as libc::c_int;
                }
            } else {
                let mut i: libc::c_int = conditional_line(p, wlen, fstart);
                if i != -(2 as libc::c_int) {
                    if i == -(1 as libc::c_int) {
                        fatal(
                            fstart,
                            0 as libc::c_int as size_t,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"invalid syntax in conditional\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    ignoring = i;
                } else {
                    if ignoring != 0 {
                        continue;
                    }
                    if wlen
                        == (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        && memcmp(
                            b"export\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            p as *const libc::c_void,
                            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ) == 0 as libc::c_int
                        || wlen
                            == (::std::mem::size_of::<[libc::c_char; 9]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            && memcmp(
                                b"unexport\0" as *const u8 as *const libc::c_char
                                    as *const libc::c_void,
                                p as *const libc::c_void,
                                (::std::mem::size_of::<[libc::c_char; 9]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ) == 0 as libc::c_int
                    {
                        let mut exporting: libc::c_int = if *p as libc::c_int
                            == 'u' as i32
                        {
                            0 as libc::c_int
                        } else {
                            1 as libc::c_int
                        };
                        if !filenames.is_null() {
                            fi.lineno = tgts_started as libc::c_ulong;
                            fi.offset = 0 as libc::c_int as libc::c_ulong;
                            record_files(
                                filenames,
                                also_make_targets,
                                pattern,
                                pattern_percent,
                                depstr,
                                cmds_started,
                                commands,
                                commands_idx,
                                two_colon,
                                prefix,
                                &mut fi,
                            );
                            filenames = 0 as *mut nameseq;
                        }
                        commands_idx = 0 as libc::c_int as size_t;
                        no_targets = 0 as libc::c_int;
                        pattern = 0 as *const libc::c_char;
                        also_make_targets = 0 as libc::c_int;
                        if *p2 as libc::c_int == '\0' as i32 {
                            export_all_variables = exporting;
                        } else {
                            let mut l: size_t = 0;
                            let mut cp: *const libc::c_char = 0 as *const libc::c_char;
                            let mut ap: *mut libc::c_char = 0 as *mut libc::c_char;
                            ap = allocated_variable_expand_for_file(p2, 0 as *mut file);
                            cp = ap;
                            p = find_next_token(&mut cp, &mut l);
                            while !p.is_null() {
                                let mut v_0: *mut variable = lookup_variable(p, l);
                                if v_0.is_null() {
                                    v_0 = define_variable_in_set(
                                        p,
                                        l,
                                        b"\0" as *const u8 as *const libc::c_char,
                                        o_file,
                                        0 as libc::c_int,
                                        0 as *mut variable_set,
                                        fstart,
                                    );
                                }
                                (*v_0)
                                    .set_export(
                                        (if exporting != 0 {
                                            v_export as libc::c_int
                                        } else {
                                            v_noexport as libc::c_int
                                        }) as variable_export,
                                    );
                                p = find_next_token(&mut cp, &mut l);
                            }
                            free(ap as *mut libc::c_void);
                        }
                    } else if wlen
                        == (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        && memcmp(
                            b"vpath\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            p as *const libc::c_void,
                            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ) == 0 as libc::c_int
                    {
                        let mut cp_0: *const libc::c_char = 0 as *const libc::c_char;
                        let mut vpat: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut l_0: size_t = 0;
                        if !filenames.is_null() {
                            fi.lineno = tgts_started as libc::c_ulong;
                            fi.offset = 0 as libc::c_int as libc::c_ulong;
                            record_files(
                                filenames,
                                also_make_targets,
                                pattern,
                                pattern_percent,
                                depstr,
                                cmds_started,
                                commands,
                                commands_idx,
                                two_colon,
                                prefix,
                                &mut fi,
                            );
                            filenames = 0 as *mut nameseq;
                        }
                        commands_idx = 0 as libc::c_int as size_t;
                        no_targets = 0 as libc::c_int;
                        pattern = 0 as *const libc::c_char;
                        also_make_targets = 0 as libc::c_int;
                        cp_0 = variable_expand(p2);
                        p = find_next_token(&mut cp_0, &mut l_0);
                        if !p.is_null() {
                            vpat = xstrndup(p, l_0);
                            p = find_next_token(&mut cp_0, &mut l_0);
                        } else {
                            vpat = 0 as *mut libc::c_char;
                        }
                        construct_vpath_list(vpat, p);
                        free(vpat as *mut libc::c_void);
                    } else if wlen
                        == (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        && memcmp(
                            b"include\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            p as *const libc::c_void,
                            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ) == 0 as libc::c_int
                        || wlen
                            == (::std::mem::size_of::<[libc::c_char; 9]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            && memcmp(
                                b"-include\0" as *const u8 as *const libc::c_char
                                    as *const libc::c_void,
                                p as *const libc::c_void,
                                (::std::mem::size_of::<[libc::c_char; 9]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ) == 0 as libc::c_int
                        || wlen
                            == (::std::mem::size_of::<[libc::c_char; 9]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            && memcmp(
                                b"sinclude\0" as *const u8 as *const libc::c_char
                                    as *const libc::c_void,
                                p as *const libc::c_void,
                                (::std::mem::size_of::<[libc::c_char; 9]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ) == 0 as libc::c_int
                    {
                        let mut save: *mut conditionals = 0 as *mut conditionals;
                        let mut new_conditionals: conditionals = conditionals {
                            if_cmds: 0,
                            allocated: 0,
                            ignoring: 0 as *const libc::c_char as *mut libc::c_char,
                            seen_else: 0 as *const libc::c_char as *mut libc::c_char,
                        };
                        let mut files: *mut nameseq = 0 as *mut nameseq;
                        let mut noerror: libc::c_int = (*p
                            .offset(0 as libc::c_int as isize) as libc::c_int
                            != 'i' as i32) as libc::c_int;
                        if !filenames.is_null() {
                            fi.lineno = tgts_started as libc::c_ulong;
                            fi.offset = 0 as libc::c_int as libc::c_ulong;
                            record_files(
                                filenames,
                                also_make_targets,
                                pattern,
                                pattern_percent,
                                depstr,
                                cmds_started,
                                commands,
                                commands_idx,
                                two_colon,
                                prefix,
                                &mut fi,
                            );
                            filenames = 0 as *mut nameseq;
                        }
                        commands_idx = 0 as libc::c_int as size_t;
                        no_targets = 0 as libc::c_int;
                        pattern = 0 as *const libc::c_char;
                        also_make_targets = 0 as libc::c_int;
                        p = allocated_variable_expand_for_file(p2, 0 as *mut file);
                        if *p as libc::c_int == '\0' as i32 {
                            free(p as *mut libc::c_void);
                        } else {
                            p2 = p;
                            files = parse_file_seq(
                                &mut p2,
                                ::std::mem::size_of::<nameseq>() as libc::c_ulong,
                                0x1 as libc::c_int,
                                0 as *const libc::c_char,
                                0x2 as libc::c_int,
                            ) as *mut nameseq;
                            free(p as *mut libc::c_void);
                            save = install_conditionals(&mut new_conditionals);
                            if !filenames.is_null() {
                                fi.lineno = tgts_started as libc::c_ulong;
                                fi.offset = 0 as libc::c_int as libc::c_ulong;
                                record_files(
                                    filenames,
                                    also_make_targets,
                                    pattern,
                                    pattern_percent,
                                    depstr,
                                    cmds_started,
                                    commands,
                                    commands_idx,
                                    two_colon,
                                    prefix,
                                    &mut fi,
                                );
                                filenames = 0 as *mut nameseq;
                            }
                            commands_idx = 0 as libc::c_int as size_t;
                            no_targets = 0 as libc::c_int;
                            pattern = 0 as *const libc::c_char;
                            also_make_targets = 0 as libc::c_int;
                            while !files.is_null() {
                                let mut next: *mut nameseq = (*files).next;
                                let mut flags: libc::c_ushort = ((1 as libc::c_int)
                                    << 1 as libc::c_int | (1 as libc::c_int) << 3 as libc::c_int
                                    | (if noerror != 0 {
                                        (1 as libc::c_int) << 2 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    })
                                    | (if set_default != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (1 as libc::c_int) << 0 as libc::c_int
                                    })) as libc::c_ushort;
                                let mut d: *mut goaldep = eval_makefile(
                                    (*files).name,
                                    flags,
                                );
                                (*d).floc = *fstart;
                                free(files as *mut libc::c_void);
                                files = next;
                            }
                            restore_conditionals(save);
                        }
                    } else if wlen
                        == (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        && memcmp(
                            b"load\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            p as *const libc::c_void,
                            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ) == 0 as libc::c_int
                        || wlen
                            == (::std::mem::size_of::<[libc::c_char; 6]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            && memcmp(
                                b"-load\0" as *const u8 as *const libc::c_char
                                    as *const libc::c_void,
                                p as *const libc::c_void,
                                (::std::mem::size_of::<[libc::c_char; 6]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ) == 0 as libc::c_int
                    {
                        let mut files_0: *mut nameseq = 0 as *mut nameseq;
                        let mut noerror_0: libc::c_int = (*p
                            .offset(0 as libc::c_int as isize) as libc::c_int
                            == '-' as i32) as libc::c_int;
                        if !filenames.is_null() {
                            fi.lineno = tgts_started as libc::c_ulong;
                            fi.offset = 0 as libc::c_int as libc::c_ulong;
                            record_files(
                                filenames,
                                also_make_targets,
                                pattern,
                                pattern_percent,
                                depstr,
                                cmds_started,
                                commands,
                                commands_idx,
                                two_colon,
                                prefix,
                                &mut fi,
                            );
                            filenames = 0 as *mut nameseq;
                        }
                        commands_idx = 0 as libc::c_int as size_t;
                        no_targets = 0 as libc::c_int;
                        pattern = 0 as *const libc::c_char;
                        also_make_targets = 0 as libc::c_int;
                        p = allocated_variable_expand_for_file(p2, 0 as *mut file);
                        if *p as libc::c_int == '\0' as i32 {
                            free(p as *mut libc::c_void);
                        } else {
                            p2 = p;
                            files_0 = parse_file_seq(
                                &mut p2,
                                ::std::mem::size_of::<nameseq>() as libc::c_ulong,
                                0x1 as libc::c_int,
                                0 as *const libc::c_char,
                                0x2 as libc::c_int,
                            ) as *mut nameseq;
                            free(p as *mut libc::c_void);
                            while !files_0.is_null() {
                                let mut next_0: *mut nameseq = (*files_0).next;
                                let mut name: *const libc::c_char = (*files_0).name;
                                let mut deps: *mut goaldep = 0 as *mut goaldep;
                                let mut f: *mut file = 0 as *mut file;
                                let mut r: libc::c_int = 0;
                                let mut file: file = {
                                    let mut init = file {
                                        update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped: [0; 4],
                                        c2rust_padding: [0; 4],
                                        name: 0 as *const libc::c_char,
                                        hname: 0 as *const libc::c_char,
                                        vpath: 0 as *const libc::c_char,
                                        deps: 0 as *mut dep,
                                        cmds: 0 as *mut commands,
                                        stem: 0 as *const libc::c_char,
                                        also_make: 0 as *mut dep,
                                        prev: 0 as *mut file,
                                        last: 0 as *mut file,
                                        renamed: 0 as *mut file,
                                        variables: 0 as *mut variable_set_list,
                                        pat_variables: 0 as *mut variable_set_list,
                                        parent: 0 as *mut file,
                                        double_colon: 0 as *mut file,
                                        last_mtime: 0,
                                        mtime_before_update: 0,
                                        considered: 0,
                                        command_flags: 0,
                                    };
                                    init.set_update_status(us_success);
                                    init.set_command_state(cs_not_started);
                                    init.set_builtin(0);
                                    init.set_precious(0);
                                    init.set_loaded(0);
                                    init.set_unloaded(0);
                                    init.set_low_resolution_time(0);
                                    init.set_tried_implicit(0);
                                    init.set_updating(0);
                                    init.set_updated(0);
                                    init.set_is_target(0);
                                    init.set_cmd_target(0);
                                    init.set_phony(0);
                                    init.set_intermediate(0);
                                    init.set_is_explicit(0);
                                    init.set_secondary(0);
                                    init.set_notintermediate(0);
                                    init.set_dontcare(0);
                                    init.set_ignore_vpath(0);
                                    init.set_pat_searched(0);
                                    init.set_no_diag(0);
                                    init.set_was_shuffled(0);
                                    init.set_snapped(0);
                                    init
                                };
                                file.name = name;
                                r = load_file(&mut (*ebuf).floc, &mut file, noerror_0);
                                if r == 0 && noerror_0 == 0 {
                                    fatal(
                                        &mut (*ebuf).floc as *mut floc,
                                        strlen(name),
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"%s: failed to load\0" as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        name,
                                    );
                                }
                                name = file.name;
                                f = lookup_file(name);
                                if f.is_null() {
                                    f = enter_file(name);
                                }
                                (*f).set_loaded(1 as libc::c_int as libc::c_uint);
                                (*f).set_unloaded(0 as libc::c_int as libc::c_uint);
                                free(files_0 as *mut libc::c_void);
                                files_0 = next_0;
                                if r == -(1 as libc::c_int) {
                                    continue;
                                }
                                deps = xcalloc(
                                    ::std::mem::size_of::<goaldep>() as libc::c_ulong,
                                ) as *mut goaldep;
                                (*deps).next = read_files;
                                (*deps).floc = (*ebuf).floc;
                                read_files = deps;
                                (*deps).file = f;
                            }
                        }
                    } else {
                        if *line.offset(0 as libc::c_int as isize) as libc::c_int
                            == cmd_prefix as libc::c_int
                        {
                            fatal(
                                fstart,
                                0 as libc::c_int as size_t,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"recipe commences before first target\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                        let mut wtype: make_word_type = w_bogus;
                        let mut cmdleft: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut semip: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut lb_next: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut plen: size_t = 0 as libc::c_int as size_t;
                        let mut colonp: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut end: *const libc::c_char = 0 as *const libc::c_char;
                        let mut beg: *const libc::c_char = 0 as *const libc::c_char;
                        if !filenames.is_null() {
                            fi.lineno = tgts_started as libc::c_ulong;
                            fi.offset = 0 as libc::c_int as libc::c_ulong;
                            record_files(
                                filenames,
                                also_make_targets,
                                pattern,
                                pattern_percent,
                                depstr,
                                cmds_started,
                                commands,
                                commands_idx,
                                two_colon,
                                prefix,
                                &mut fi,
                            );
                            filenames = 0 as *mut nameseq;
                        }
                        commands_idx = 0 as libc::c_int as size_t;
                        no_targets = 0 as libc::c_int;
                        pattern = 0 as *const libc::c_char;
                        also_make_targets = 0 as libc::c_int;
                        tgts_started = (*fstart).lineno as libc::c_uint;
                        cmdleft = find_map_unquote(
                            line,
                            0x10 as libc::c_int | 0x8 as libc::c_int
                                | 0x4000 as libc::c_int,
                        );
                        if !cmdleft.is_null() && *cmdleft as libc::c_int == '#' as i32 {
                            *cmdleft = '\0' as i32 as libc::c_char;
                            cmdleft = 0 as *mut libc::c_char;
                        } else if !cmdleft.is_null() {
                            let fresh2 = cmdleft;
                            cmdleft = cmdleft.offset(1);
                            semip = fresh2;
                            *semip = '\0' as i32 as libc::c_char;
                        }
                        collapse_continuations(line);
                        wtype = get_next_mword(line, &mut lb_next, &mut wlen);
                        match wtype as libc::c_uint {
                            1 => {
                                if !cmdleft.is_null() {
                                    fatal(
                                        fstart,
                                        0 as libc::c_int as size_t,
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"missing rule before recipe\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                }
                            }
                            4 | 5 | 8 | 9 => {
                                no_targets = 1 as libc::c_int;
                            }
                            _ => {
                                p2 = variable_expand_string(
                                    0 as *mut libc::c_char,
                                    lb_next,
                                    wlen,
                                );
                                loop {
                                    lb_next = lb_next.offset(wlen as isize);
                                    if cmdleft.is_null() {
                                        cmdleft = find_char_unquote(p2, ';' as i32);
                                        if !cmdleft.is_null() {
                                            let mut p2_off: size_t = p2.offset_from(variable_buffer)
                                                as libc::c_long as size_t;
                                            let mut cmd_off: size_t = cmdleft
                                                .offset_from(variable_buffer) as libc::c_long as size_t;
                                            let mut pend: *mut libc::c_char = p2
                                                .offset(strlen(p2) as isize);
                                            *cmdleft = '\0' as i32 as libc::c_char;
                                            variable_expand_string(
                                                pend,
                                                lb_next,
                                                18446744073709551615 as libc::c_ulong,
                                            );
                                            lb_next = lb_next.offset(strlen(lb_next) as isize);
                                            p2 = variable_buffer.offset(p2_off as isize);
                                            cmdleft = variable_buffer
                                                .offset(cmd_off as isize)
                                                .offset(1 as libc::c_int as isize);
                                        }
                                    }
                                    colonp = find_char_unquote(p2, ':' as i32);
                                    if !colonp.is_null() {
                                        if colonp > p2
                                            && *colonp.offset(-(1 as libc::c_int) as isize)
                                                as libc::c_int == '&' as i32
                                        {
                                            colonp = colonp.offset(-1);
                                            colonp;
                                        }
                                        break;
                                    } else {
                                        wtype = get_next_mword(lb_next, &mut lb_next, &mut wlen);
                                        if wtype as libc::c_uint
                                            == w_eol as libc::c_int as libc::c_uint
                                        {
                                            break;
                                        }
                                        p2 = p2.offset(strlen(p2) as isize);
                                        let fresh3 = p2;
                                        p2 = p2.offset(1);
                                        *fresh3 = ' ' as i32 as libc::c_char;
                                        p2 = variable_expand_string(p2, lb_next, wlen);
                                    }
                                }
                                p2 = next_token(variable_buffer);
                                if wtype as libc::c_uint
                                    == w_eol as libc::c_int as libc::c_uint
                                {
                                    if *p2 as libc::c_int == '\0' as i32 {
                                        continue;
                                    }
                                    if cmd_prefix as libc::c_int == '\t' as i32
                                        && strncmp(
                                            line,
                                            b"        \0" as *const u8 as *const libc::c_char,
                                            8 as libc::c_int as libc::c_ulong,
                                        ) == 0 as libc::c_int
                                    {
                                        fatal(
                                            fstart,
                                            0 as libc::c_int as size_t,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"missing separator (did you mean TAB instead of 8 spaces?)\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                    }
                                    p2 = next_token(line);
                                    if strncmp(
                                        p2,
                                        b"if\0" as *const u8 as *const libc::c_char,
                                        2 as libc::c_int as libc::c_ulong,
                                    ) == 0 as libc::c_int
                                        && (strncmp(
                                            &mut *p2.offset(2 as libc::c_int as isize),
                                            b"neq\0" as *const u8 as *const libc::c_char,
                                            3 as libc::c_int as libc::c_ulong,
                                        ) == 0 as libc::c_int
                                            && !(*stopchar_map
                                                .as_mut_ptr()
                                                .offset(
                                                    *p2.offset(5 as libc::c_int as isize) as libc::c_uchar
                                                        as isize,
                                                ) as libc::c_int & 0x2 as libc::c_int != 0 as libc::c_int)
                                            || strncmp(
                                                &mut *p2.offset(2 as libc::c_int as isize),
                                                b"eq\0" as *const u8 as *const libc::c_char,
                                                2 as libc::c_int as libc::c_ulong,
                                            ) == 0 as libc::c_int
                                                && !(*stopchar_map
                                                    .as_mut_ptr()
                                                    .offset(
                                                        *p2.offset(4 as libc::c_int as isize) as libc::c_uchar
                                                            as isize,
                                                    ) as libc::c_int & 0x2 as libc::c_int != 0 as libc::c_int))
                                    {
                                        fatal(
                                            fstart,
                                            0 as libc::c_int as size_t,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"missing separator (ifeq/ifneq must be followed by whitespace)\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                        );
                                    }
                                    fatal(
                                        fstart,
                                        0 as libc::c_int as size_t,
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"missing separator\0" as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                } else {
                                    let mut save_0: libc::c_char = *colonp;
                                    if save_0 as libc::c_int == '&' as i32 {
                                        also_make_targets = 1 as libc::c_int;
                                    }
                                    *colonp = '\0' as i32 as libc::c_char;
                                    filenames = parse_file_seq(
                                        &mut p2,
                                        ::std::mem::size_of::<nameseq>() as libc::c_ulong,
                                        0x1 as libc::c_int,
                                        0 as *const libc::c_char,
                                        0 as libc::c_int,
                                    ) as *mut nameseq;
                                    *colonp = save_0;
                                    p2 = colonp
                                        .offset(
                                            (save_0 as libc::c_int == '&' as i32) as libc::c_int
                                                as isize,
                                        );
                                    if filenames.is_null() {
                                        no_targets = 1 as libc::c_int;
                                    } else {
                                        p2 = p2.offset(1);
                                        p2;
                                        two_colon = (*p2 as libc::c_int == ':' as i32)
                                            as libc::c_int;
                                        if two_colon != 0 {
                                            p2 = p2.offset(1);
                                            p2;
                                        }
                                        if *lb_next as libc::c_int != '\0' as i32 {
                                            let mut l_1: size_t = p2.offset_from(variable_buffer)
                                                as libc::c_long as size_t;
                                            plen = strlen(p2);
                                            variable_buffer_output(
                                                p2.offset(plen as isize),
                                                lb_next,
                                                (strlen(lb_next))
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                            );
                                            p2 = variable_buffer.offset(l_1 as isize);
                                        }
                                        p2 = parse_var_assignment(p2, 1 as libc::c_int, &mut vmod);
                                        if vmod.assign_v() != 0 {
                                            if !semip.is_null() {
                                                let mut l_2: size_t = p2.offset_from(variable_buffer)
                                                    as libc::c_long as size_t;
                                                *semip = ';' as i32 as libc::c_char;
                                                collapse_continuations(semip);
                                                variable_buffer_output(
                                                    p2.offset(strlen(p2) as isize),
                                                    semip,
                                                    (strlen(semip))
                                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                                );
                                                p2 = variable_buffer.offset(l_2 as isize);
                                            }
                                            record_target_var(
                                                filenames,
                                                p2,
                                                (if vmod.override_v() as libc::c_int != 0 {
                                                    o_override as libc::c_int
                                                } else {
                                                    o_file as libc::c_int
                                                }) as variable_origin,
                                                &mut vmod,
                                                fstart,
                                            );
                                            filenames = 0 as *mut nameseq;
                                        } else {
                                            find_char_unquote(lb_next, '=' as i32);
                                            prefix = cmd_prefix;
                                            no_targets = 0 as libc::c_int;
                                            if *lb_next as libc::c_int != '\0' as i32 {
                                                let mut l_3: size_t = p2.offset_from(variable_buffer)
                                                    as libc::c_long as size_t;
                                                variable_expand_string(
                                                    p2.offset(plen as isize),
                                                    lb_next,
                                                    18446744073709551615 as libc::c_ulong,
                                                );
                                                p2 = variable_buffer.offset(l_3 as isize);
                                                if cmdleft.is_null() {
                                                    cmdleft = find_char_unquote(p2, ';' as i32);
                                                    if !cmdleft.is_null() {
                                                        let fresh4 = cmdleft;
                                                        cmdleft = cmdleft.offset(1);
                                                        *fresh4 = '\0' as i32 as libc::c_char;
                                                    }
                                                }
                                            }
                                            p = strchr(p2, ':' as i32);
                                            while !p.is_null()
                                                && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                    == '\\' as i32
                                            {
                                                let mut q: *mut libc::c_char = &mut *p
                                                    .offset(-(1 as libc::c_int) as isize) as *mut libc::c_char;
                                                let mut backslash: libc::c_int = 0 as libc::c_int;
                                                loop {
                                                    let fresh5 = q;
                                                    q = q.offset(-1);
                                                    if !(*fresh5 as libc::c_int == '\\' as i32) {
                                                        break;
                                                    }
                                                    backslash = (backslash == 0) as libc::c_int;
                                                }
                                                if !(backslash != 0) {
                                                    break;
                                                }
                                                p = strchr(p.offset(1 as libc::c_int as isize), ':' as i32);
                                            }
                                            if !p.is_null() {
                                                let mut target: *mut nameseq = 0 as *mut nameseq;
                                                target = parse_file_seq(
                                                    &mut p2,
                                                    ::std::mem::size_of::<nameseq>() as libc::c_ulong,
                                                    0x40 as libc::c_int,
                                                    0 as *const libc::c_char,
                                                    0x4 as libc::c_int,
                                                ) as *mut nameseq;
                                                p2 = p2.offset(1);
                                                p2;
                                                if target.is_null() {
                                                    fatal(
                                                        fstart,
                                                        0 as libc::c_int as size_t,
                                                        dcgettext(
                                                            0 as *const libc::c_char,
                                                            b"missing target pattern\0" as *const u8
                                                                as *const libc::c_char,
                                                            5 as libc::c_int,
                                                        ),
                                                    );
                                                } else if !((*target).next).is_null() {
                                                    fatal(
                                                        fstart,
                                                        0 as libc::c_int as size_t,
                                                        dcgettext(
                                                            0 as *const libc::c_char,
                                                            b"multiple target patterns\0" as *const u8
                                                                as *const libc::c_char,
                                                            5 as libc::c_int,
                                                        ),
                                                    );
                                                }
                                                pattern_percent = find_percent_cached(&mut (*target).name);
                                                pattern = (*target).name;
                                                if pattern_percent.is_null() {
                                                    fatal(
                                                        fstart,
                                                        0 as libc::c_int as size_t,
                                                        dcgettext(
                                                            0 as *const libc::c_char,
                                                            b"target pattern contains no '%%'\0" as *const u8
                                                                as *const libc::c_char,
                                                            5 as libc::c_int,
                                                        ),
                                                    );
                                                }
                                                free(target as *mut libc::c_void);
                                            } else {
                                                pattern = 0 as *const libc::c_char;
                                            }
                                            beg = p2;
                                            end = beg
                                                .offset(strlen(beg) as isize)
                                                .offset(-(1 as libc::c_int as isize));
                                            strip_whitespace(&mut beg, &mut end);
                                            if beg <= end && *beg as libc::c_int != '\0' as i32 {
                                                depstr = xstrndup(
                                                    beg,
                                                    (end.offset_from(beg) as libc::c_long
                                                        + 1 as libc::c_int as libc::c_long) as size_t,
                                                );
                                            } else {
                                                depstr = 0 as *mut libc::c_char;
                                            }
                                            commands_idx = 0 as libc::c_int as size_t;
                                            if !cmdleft.is_null() {
                                                let mut l_4: size_t = strlen(cmdleft);
                                                cmds_started = (*fstart).lineno as libc::c_uint;
                                                if l_4.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                                    > commands_len
                                                {
                                                    commands_len = l_4
                                                        .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong);
                                                    commands = xrealloc(
                                                        commands as *mut libc::c_void,
                                                        commands_len,
                                                    ) as *mut libc::c_char;
                                                }
                                                memcpy(
                                                    commands as *mut libc::c_void,
                                                    cmdleft as *const libc::c_void,
                                                    l_4,
                                                );
                                                commands_idx = (commands_idx as libc::c_ulong)
                                                    .wrapping_add(l_4) as size_t as size_t;
                                                let fresh6 = commands_idx;
                                                commands_idx = commands_idx.wrapping_add(1);
                                                *commands
                                                    .offset(fresh6 as isize) = '\n' as i32 as libc::c_char;
                                            }
                                            check_specials(filenames, set_default);
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
    if (*conditionals).if_cmds != 0 {
        fatal(
            fstart,
            0 as libc::c_int as size_t,
            dcgettext(
                0 as *const libc::c_char,
                b"missing 'endif'\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if !filenames.is_null() {
        fi.lineno = tgts_started as libc::c_ulong;
        fi.offset = 0 as libc::c_int as libc::c_ulong;
        record_files(
            filenames,
            also_make_targets,
            pattern,
            pattern_percent,
            depstr,
            cmds_started,
            commands,
            commands_idx,
            two_colon,
            prefix,
            &mut fi,
        );
        filenames = 0 as *mut nameseq;
    }
    commands_idx = 0 as libc::c_int as size_t;
    no_targets = 0 as libc::c_int;
    pattern = 0 as *const libc::c_char;
    also_make_targets = 0 as libc::c_int;
    free(collapsed as *mut libc::c_void);
    free(commands as *mut libc::c_void);
}
unsafe extern "C" fn remove_comments(mut line: *mut libc::c_char) {
    let mut comment: *mut libc::c_char = 0 as *mut libc::c_char;
    comment = find_map_unquote(line, 0x8 as libc::c_int | 0x4000 as libc::c_int);
    if !comment.is_null() {
        *comment = '\0' as i32 as libc::c_char;
    }
}
unsafe extern "C" fn do_undefine(
    mut name: *mut libc::c_char,
    mut origin: variable_origin,
    mut ebuf: *mut ebuffer,
) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut var: *mut libc::c_char = 0 as *mut libc::c_char;
    var = allocated_variable_expand_for_file(name, 0 as *mut file);
    name = next_token(var);
    if *name as libc::c_int == '\0' as i32 {
        fatal(
            &mut (*ebuf).floc as *mut floc,
            0 as libc::c_int as size_t,
            dcgettext(
                0 as *const libc::c_char,
                b"empty variable name\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    p = name.offset(strlen(name) as isize).offset(-(1 as libc::c_int as isize));
    while p > name
        && *stopchar_map.as_mut_ptr().offset(*p as libc::c_uchar as isize) as libc::c_int
            & 0x2 as libc::c_int != 0 as libc::c_int
    {
        p = p.offset(-1);
        p;
    }
    *p.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    undefine_variable_in_set(
        name,
        (p.offset_from(name) as libc::c_long + 1 as libc::c_int as libc::c_long)
            as size_t,
        origin,
        0 as *mut variable_set,
    );
    free(var as *mut libc::c_void);
}
unsafe extern "C" fn do_define(
    mut name: *mut libc::c_char,
    mut origin: variable_origin,
    mut ebuf: *mut ebuffer,
) -> *mut variable {
    let mut v: *mut variable = 0 as *mut variable;
    let mut var: variable = variable {
        name: 0 as *mut libc::c_char,
        value: 0 as *mut libc::c_char,
        fileinfo: floc {
            filenm: 0 as *const libc::c_char,
            lineno: 0,
            offset: 0,
        },
        length: 0,
        recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
    };
    let mut defstart: floc = floc {
        filenm: 0 as *const libc::c_char,
        lineno: 0,
        offset: 0,
    };
    let mut nlevels: libc::c_int = 1 as libc::c_int;
    let mut length: size_t = 100 as libc::c_int as size_t;
    let mut definition: *mut libc::c_char = xmalloc(length) as *mut libc::c_char;
    let mut idx: size_t = 0 as libc::c_int as size_t;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: *mut libc::c_char = 0 as *mut libc::c_char;
    defstart = (*ebuf).floc;
    p = parse_variable_definition(name, &mut var);
    if p.is_null() {
        var.set_flavor(f_recursive);
    } else {
        if *(var.value).offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
            error(
                &mut defstart as *mut floc,
                0 as libc::c_int as size_t,
                dcgettext(
                    0 as *const libc::c_char,
                    b"extraneous text after 'define' directive\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        *(var.name).offset(var.length as isize) = '\0' as i32 as libc::c_char;
    }
    n = allocated_variable_expand_for_file(name, 0 as *mut file);
    name = next_token(n);
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        fatal(
            &mut defstart as *mut floc,
            0 as libc::c_int as size_t,
            dcgettext(
                0 as *const libc::c_char,
                b"empty variable name\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    p = name.offset(strlen(name) as isize).offset(-(1 as libc::c_int as isize));
    while p > name
        && *stopchar_map.as_mut_ptr().offset(*p as libc::c_uchar as isize) as libc::c_int
            & 0x2 as libc::c_int != 0 as libc::c_int
    {
        p = p.offset(-1);
        p;
    }
    *p.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    loop {
        let mut len: size_t = 0;
        let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut nlines: libc::c_long = readline(ebuf);
        if nlines < 0 as libc::c_int as libc::c_long {
            fatal(
                &mut defstart as *mut floc,
                0 as libc::c_int as size_t,
                dcgettext(
                    0 as *const libc::c_char,
                    b"missing 'endef', unterminated 'define'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        (*ebuf)
            .floc
            .lineno = ((*ebuf).floc.lineno).wrapping_add(nlines as libc::c_ulong);
        line = (*ebuf).buffer;
        collapse_continuations(line);
        if *line.offset(0 as libc::c_int as isize) as libc::c_int
            != cmd_prefix as libc::c_int
        {
            p = next_token(line);
            len = strlen(p);
            if (len == 6 as libc::c_int as libc::c_ulong
                || len > 6 as libc::c_int as libc::c_ulong
                    && *stopchar_map
                        .as_mut_ptr()
                        .offset(
                            *p.offset(6 as libc::c_int as isize) as libc::c_uchar
                                as isize,
                        ) as libc::c_int & 0x2 as libc::c_int != 0 as libc::c_int)
                && strncmp(
                    p,
                    b"define\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                nlevels += 1;
                nlevels;
            } else if (len == 5 as libc::c_int as libc::c_ulong
                || len > 5 as libc::c_int as libc::c_ulong
                    && *stopchar_map
                        .as_mut_ptr()
                        .offset(
                            *p.offset(5 as libc::c_int as isize) as libc::c_uchar
                                as isize,
                        ) as libc::c_int & 0x2 as libc::c_int != 0 as libc::c_int)
                && strncmp(
                    p,
                    b"endef\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                p = p.offset(5 as libc::c_int as isize);
                remove_comments(p);
                if *next_token(p) as libc::c_int != '\0' as i32 {
                    error(
                        &mut (*ebuf).floc as *mut floc,
                        0 as libc::c_int as size_t,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"extraneous text after 'endef' directive\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                nlevels -= 1;
                if nlevels == 0 as libc::c_int {
                    break;
                }
            }
        }
        len = strlen(line);
        if idx.wrapping_add(len).wrapping_add(1 as libc::c_int as libc::c_ulong) > length
        {
            length = idx
                .wrapping_add(len)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong);
            definition = xrealloc(
                definition as *mut libc::c_void,
                length.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
        }
        memcpy(
            &mut *definition.offset(idx as isize) as *mut libc::c_char
                as *mut libc::c_void,
            line as *const libc::c_void,
            len,
        );
        idx = (idx as libc::c_ulong).wrapping_add(len) as size_t as size_t;
        let fresh7 = idx;
        idx = idx.wrapping_add(1);
        *definition.offset(fresh7 as isize) = '\n' as i32 as libc::c_char;
    }
    if idx == 0 as libc::c_int as libc::c_ulong {
        *definition.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    } else {
        *definition
            .offset(
                idx.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
    }
    v = do_variable_definition(
        &mut defstart,
        name,
        definition,
        origin,
        var.flavor(),
        0 as libc::c_int,
    );
    free(definition as *mut libc::c_void);
    free(n as *mut libc::c_void);
    return v;
}
unsafe extern "C" fn conditional_line(
    mut line: *mut libc::c_char,
    mut len: size_t,
    mut flocp: *const floc,
) -> libc::c_int {
    let mut cmdname: *const libc::c_char = 0 as *const libc::c_char;
    let mut cmdtype: C2RustUnnamed = c_ifdef;
    let mut i: libc::c_uint = 0;
    let mut o: libc::c_uint = 0;
    if len
        == (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && strncmp(
            b"ifdef\0" as *const u8 as *const libc::c_char,
            line,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
    {
        cmdtype = c_ifdef;
        cmdname = b"ifdef\0" as *const u8 as *const libc::c_char;
    } else if len
        == (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && strncmp(
            b"ifndef\0" as *const u8 as *const libc::c_char,
            line,
            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
    {
        cmdtype = c_ifndef;
        cmdname = b"ifndef\0" as *const u8 as *const libc::c_char;
    } else if len
        == (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && strncmp(
            b"ifeq\0" as *const u8 as *const libc::c_char,
            line,
            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
    {
        cmdtype = c_ifeq;
        cmdname = b"ifeq\0" as *const u8 as *const libc::c_char;
    } else if len
        == (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && strncmp(
            b"ifneq\0" as *const u8 as *const libc::c_char,
            line,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
    {
        cmdtype = c_ifneq;
        cmdname = b"ifneq\0" as *const u8 as *const libc::c_char;
    } else if len
        == (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && strncmp(
            b"else\0" as *const u8 as *const libc::c_char,
            line,
            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
    {
        cmdtype = c_else;
        cmdname = b"else\0" as *const u8 as *const libc::c_char;
    } else if len
        == (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && strncmp(
            b"endif\0" as *const u8 as *const libc::c_char,
            line,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
    {
        cmdtype = c_endif;
        cmdname = b"endif\0" as *const u8 as *const libc::c_char;
    } else {
        return -(2 as libc::c_int)
    }
    line = line.offset(len as isize);
    while *stopchar_map.as_mut_ptr().offset(*line as libc::c_uchar as isize)
        as libc::c_int & (0x2 as libc::c_int | 0x4 as libc::c_int) != 0 as libc::c_int
    {
        line = line.offset(1);
        line;
    }
    if cmdtype as libc::c_uint == c_endif as libc::c_int as libc::c_uint {
        if *line as libc::c_int != '\0' as i32 {
            error(
                flocp,
                strlen(cmdname),
                dcgettext(
                    0 as *const libc::c_char,
                    b"extraneous text after '%s' directive\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                cmdname,
            );
        }
        if (*conditionals).if_cmds == 0 {
            fatal(
                flocp,
                strlen(cmdname),
                dcgettext(
                    0 as *const libc::c_char,
                    b"extraneous '%s'\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                cmdname,
            );
        }
        (*conditionals).if_cmds = ((*conditionals).if_cmds).wrapping_sub(1);
        (*conditionals).if_cmds;
    } else if cmdtype as libc::c_uint == c_else as libc::c_int as libc::c_uint {
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        if (*conditionals).if_cmds == 0 {
            fatal(
                flocp,
                strlen(cmdname),
                dcgettext(
                    0 as *const libc::c_char,
                    b"extraneous '%s'\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                cmdname,
            );
        }
        o = ((*conditionals).if_cmds).wrapping_sub(1 as libc::c_int as libc::c_uint);
        if *((*conditionals).seen_else).offset(o as isize) != 0 {
            fatal(
                flocp,
                0 as libc::c_int as size_t,
                dcgettext(
                    0 as *const libc::c_char,
                    b"only one 'else' per conditional\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        match *((*conditionals).ignoring).offset(o as isize) as libc::c_int {
            0 => {
                *((*conditionals).ignoring)
                    .offset(o as isize) = 2 as libc::c_int as libc::c_char;
            }
            1 => {
                *((*conditionals).ignoring)
                    .offset(o as isize) = 0 as libc::c_int as libc::c_char;
            }
            _ => {}
        }
        if *line as libc::c_int == '\0' as i32 {
            *((*conditionals).seen_else)
                .offset(o as isize) = 1 as libc::c_int as libc::c_char;
        } else {
            p = line.offset(1 as libc::c_int as isize);
            while !(*stopchar_map.as_mut_ptr().offset(*p as libc::c_uchar as isize)
                as libc::c_int
                & (0x2 as libc::c_int | 0x4 as libc::c_int | 0x1 as libc::c_int)
                != 0 as libc::c_int)
            {
                p = p.offset(1);
                p;
            }
            len = p.offset_from(line) as libc::c_long as size_t;
            if len
                == (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                && strncmp(
                    b"else\0" as *const u8 as *const libc::c_char,
                    line,
                    (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) == 0 as libc::c_int
                || len
                    == (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    && strncmp(
                        b"endif\0" as *const u8 as *const libc::c_char,
                        line,
                        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0 as libc::c_int
                || conditional_line(line, len, flocp) < 0 as libc::c_int
            {
                error(
                    flocp,
                    strlen(cmdname),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"extraneous text after '%s' directive\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    cmdname,
                );
            } else {
                if (*((*conditionals).ignoring).offset(o as isize) as libc::c_int)
                    < 2 as libc::c_int
                {
                    *((*conditionals).ignoring)
                        .offset(
                            o as isize,
                        ) = *((*conditionals).ignoring)
                        .offset(
                            o.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        );
                }
                (*conditionals).if_cmds = ((*conditionals).if_cmds).wrapping_sub(1);
                (*conditionals).if_cmds;
            }
        }
    } else {
        if (*conditionals).allocated == 0 as libc::c_int as libc::c_uint {
            (*conditionals).allocated = 5 as libc::c_int as libc::c_uint;
            (*conditionals)
                .ignoring = xmalloc((*conditionals).allocated as size_t)
                as *mut libc::c_char;
            (*conditionals)
                .seen_else = xmalloc((*conditionals).allocated as size_t)
                as *mut libc::c_char;
        }
        let fresh8 = (*conditionals).if_cmds;
        (*conditionals).if_cmds = ((*conditionals).if_cmds).wrapping_add(1);
        o = fresh8;
        if (*conditionals).if_cmds > (*conditionals).allocated {
            (*conditionals)
                .allocated = ((*conditionals).allocated)
                .wrapping_add(5 as libc::c_int as libc::c_uint);
            (*conditionals)
                .ignoring = xrealloc(
                (*conditionals).ignoring as *mut libc::c_void,
                (*conditionals).allocated as size_t,
            ) as *mut libc::c_char;
            (*conditionals)
                .seen_else = xrealloc(
                (*conditionals).seen_else as *mut libc::c_void,
                (*conditionals).allocated as size_t,
            ) as *mut libc::c_char;
        }
        *((*conditionals).seen_else)
            .offset(o as isize) = 0 as libc::c_int as libc::c_char;
        i = 0 as libc::c_int as libc::c_uint;
        while i < o {
            if *((*conditionals).ignoring).offset(i as isize) != 0 {
                *((*conditionals).ignoring)
                    .offset(o as isize) = 1 as libc::c_int as libc::c_char;
                return 1 as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
        if cmdtype as libc::c_uint == c_ifdef as libc::c_int as libc::c_uint
            || cmdtype as libc::c_uint == c_ifndef as libc::c_int as libc::c_uint
        {
            let mut l: size_t = 0;
            let mut var: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut v: *mut variable = 0 as *mut variable;
            let mut p_0: *mut libc::c_char = 0 as *mut libc::c_char;
            var = allocated_variable_expand_for_file(line, 0 as *mut file);
            p_0 = end_of_token(var);
            l = p_0.offset_from(var) as libc::c_long as size_t;
            while *stopchar_map.as_mut_ptr().offset(*p_0 as libc::c_uchar as isize)
                as libc::c_int & (0x2 as libc::c_int | 0x4 as libc::c_int)
                != 0 as libc::c_int
            {
                p_0 = p_0.offset(1);
                p_0;
            }
            if *p_0 as libc::c_int != '\0' as i32 {
                return -(1 as libc::c_int);
            }
            *var.offset(l as isize) = '\0' as i32 as libc::c_char;
            v = lookup_variable(var, l);
            *((*conditionals).ignoring)
                .offset(
                    o as isize,
                ) = ((!v.is_null() && *(*v).value as libc::c_int != '\0' as i32)
                as libc::c_int
                == (cmdtype as libc::c_uint == c_ifndef as libc::c_int as libc::c_uint)
                    as libc::c_int) as libc::c_int as libc::c_char;
            free(var as *mut libc::c_void);
        } else {
            let mut s1: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut l_0: size_t = 0;
            let mut termin: libc::c_char = (if *line as libc::c_int == '(' as i32 {
                ',' as i32
            } else {
                *line as libc::c_int
            }) as libc::c_char;
            if termin as libc::c_int != ',' as i32 && termin as libc::c_int != '"' as i32
                && termin as libc::c_int != '\'' as i32
            {
                return -(1 as libc::c_int);
            }
            line = line.offset(1);
            s1 = line;
            if termin as libc::c_int == ',' as i32 {
                let mut count: libc::c_int = 0 as libc::c_int;
                while *line as libc::c_int != '\0' as i32 {
                    if *line as libc::c_int == '(' as i32 {
                        count += 1;
                        count;
                    } else if *line as libc::c_int == ')' as i32 {
                        count -= 1;
                        count;
                    } else if *line as libc::c_int == ',' as i32
                        && count <= 0 as libc::c_int
                    {
                        break;
                    }
                    line = line.offset(1);
                    line;
                }
            } else {
                while *line as libc::c_int != '\0' as i32
                    && *line as libc::c_int != termin as libc::c_int
                {
                    line = line.offset(1);
                    line;
                }
            }
            if *line as libc::c_int == '\0' as i32 {
                return -(1 as libc::c_int);
            }
            if termin as libc::c_int == ',' as i32 {
                let fresh9 = line;
                line = line.offset(1);
                let mut p_1: *mut libc::c_char = fresh9;
                while *stopchar_map
                    .as_mut_ptr()
                    .offset(
                        *p_1.offset(-(1 as libc::c_int) as isize) as libc::c_uchar
                            as isize,
                    ) as libc::c_int & 0x2 as libc::c_int != 0 as libc::c_int
                {
                    p_1 = p_1.offset(-1);
                    p_1;
                }
                *p_1 = '\0' as i32 as libc::c_char;
            } else {
                let fresh10 = line;
                line = line.offset(1);
                *fresh10 = '\0' as i32 as libc::c_char;
            }
            s2 = variable_expand(s1);
            l_0 = strlen(s2);
            let mut fresh11 = ::std::vec::from_elem(
                0,
                l_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
            );
            s1 = fresh11.leak().as_mut_ptr() as *mut libc::c_char;
            memcpy(
                s1 as *mut libc::c_void,
                s2 as *const libc::c_void,
                l_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            if termin as libc::c_int != ',' as i32 {
                while *stopchar_map.as_mut_ptr().offset(*line as libc::c_uchar as isize)
                    as libc::c_int & (0x2 as libc::c_int | 0x4 as libc::c_int)
                    != 0 as libc::c_int
                {
                    line = line.offset(1);
                    line;
                }
            }
            termin = (if termin as libc::c_int == ',' as i32 {
                ')' as i32
            } else {
                *line as libc::c_int
            }) as libc::c_char;
            if termin as libc::c_int != ')' as i32 && termin as libc::c_int != '"' as i32
                && termin as libc::c_int != '\'' as i32
            {
                return -(1 as libc::c_int);
            }
            if termin as libc::c_int == ')' as i32 {
                let mut count_0: libc::c_int = 0 as libc::c_int;
                s2 = next_token(line);
                line = s2;
                while *line as libc::c_int != '\0' as i32 {
                    if *line as libc::c_int == '(' as i32 {
                        count_0 += 1;
                        count_0;
                    } else if *line as libc::c_int == ')' as i32 {
                        if count_0 <= 0 as libc::c_int {
                            break;
                        }
                        count_0 -= 1;
                        count_0;
                    }
                    line = line.offset(1);
                    line;
                }
            } else {
                line = line.offset(1);
                line;
                s2 = line;
                while *line as libc::c_int != '\0' as i32
                    && *line as libc::c_int != termin as libc::c_int
                {
                    line = line.offset(1);
                    line;
                }
            }
            if *line as libc::c_int == '\0' as i32 {
                return -(1 as libc::c_int);
            }
            let fresh12 = line;
            line = line.offset(1);
            *fresh12 = '\0' as i32 as libc::c_char;
            while *stopchar_map.as_mut_ptr().offset(*line as libc::c_uchar as isize)
                as libc::c_int & (0x2 as libc::c_int | 0x4 as libc::c_int)
                != 0 as libc::c_int
            {
                line = line.offset(1);
                line;
            }
            if *line as libc::c_int != '\0' as i32 {
                error(
                    flocp,
                    strlen(cmdname),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"extraneous text after '%s' directive\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    cmdname,
                );
            }
            s2 = variable_expand(s2);
            *((*conditionals).ignoring)
                .offset(
                    o as isize,
                ) = ((s1 == s2
                || *s1 as libc::c_int == *s2 as libc::c_int
                    && (*s1 as libc::c_int == '\0' as i32
                        || strcmp(
                            s1.offset(1 as libc::c_int as isize),
                            s2.offset(1 as libc::c_int as isize),
                        ) == 0)) as libc::c_int
                == (cmdtype as libc::c_uint == c_ifneq as libc::c_int as libc::c_uint)
                    as libc::c_int) as libc::c_int as libc::c_char;
        }
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*conditionals).if_cmds {
        if *((*conditionals).ignoring).offset(i as isize) != 0 {
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn record_target_var(
    mut filenames: *mut nameseq,
    mut defn: *mut libc::c_char,
    mut origin: variable_origin,
    mut vmod: *mut vmodifiers,
    mut flocp: *const floc,
) {
    let mut nextf: *mut nameseq = 0 as *mut nameseq;
    let mut global: *mut variable_set_list = 0 as *mut variable_set_list;
    global = current_variable_set_list;
    while !filenames.is_null() {
        let mut v: *mut variable = 0 as *mut variable;
        let mut name: *const libc::c_char = (*filenames).name;
        let mut percent: *const libc::c_char = 0 as *const libc::c_char;
        let mut p: *mut pattern_var = 0 as *mut pattern_var;
        nextf = (*filenames).next;
        free(filenames as *mut libc::c_void);
        percent = find_percent_cached(&mut name);
        if !percent.is_null() {
            p = create_pattern_var(name, percent);
            (*p).variable.fileinfo = *flocp;
            v = assign_variable_definition(&mut (*p).variable, defn);
            (*v).set_origin(origin);
            if (*v).flavor() as libc::c_int == f_simple as libc::c_int {
                (*v)
                    .value = allocated_variable_expand_for_file(
                    (*v).value,
                    0 as *mut file,
                );
            } else {
                (*v).value = xstrdup((*v).value);
            }
        } else {
            let mut f: *mut file = 0 as *mut file;
            f = lookup_file(name);
            if f.is_null() {
                f = enter_file(strcache_add(name));
            } else if !((*f).double_colon).is_null() {
                f = (*f).double_colon;
            }
            initialize_file_variables(f, 1 as libc::c_int);
            current_variable_set_list = (*f).variables;
            v = try_variable_definition(flocp, defn, origin, 1 as libc::c_int);
            if v.is_null() {
                fatal(
                    flocp,
                    0 as libc::c_int as size_t,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Malformed target-specific variable definition\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            current_variable_set_list = global;
        }
        (*v).set_per_target(1 as libc::c_int as libc::c_uint);
        (*v).set_private_var((*vmod).private_v());
        if (*vmod).export_v() as libc::c_int != v_default as libc::c_int {
            (*v).set_export((*vmod).export_v());
        }
        if (*v).origin() as libc::c_int != o_override as libc::c_int {
            let mut gv: *mut variable = 0 as *mut variable;
            let mut len: size_t = strlen((*v).name);
            gv = lookup_variable((*v).name, len);
            if !gv.is_null() && v != gv
                && ((*gv).origin() as libc::c_int == o_env_override as libc::c_int
                    || (*gv).origin() as libc::c_int == o_command as libc::c_int)
            {
                free((*v).value as *mut libc::c_void);
                (*v).value = xstrdup((*gv).value);
                (*v).set_origin((*gv).origin());
                (*v).set_recursive((*gv).recursive());
                (*v).set_append(0 as libc::c_int as libc::c_uint);
            }
        }
        filenames = nextf;
    }
}
unsafe extern "C" fn check_specials(
    mut files: *mut nameseq,
    mut set_default: libc::c_int,
) {
    let mut t: *mut nameseq = 0 as *mut nameseq;
    t = files;
    while !t.is_null() {
        let mut nm: *const libc::c_char = (*t).name;
        if posix_pedantic == 0
            && (nm == b".POSIX\0" as *const u8 as *const libc::c_char
                || *nm as libc::c_int
                    == *(b".POSIX\0" as *const u8 as *const libc::c_char) as libc::c_int
                    && (*nm as libc::c_int == '\0' as i32
                        || strcmp(
                            nm.offset(1 as libc::c_int as isize),
                            (b".POSIX\0" as *const u8 as *const libc::c_char)
                                .offset(1 as libc::c_int as isize),
                        ) == 0))
        {
            posix_pedantic = 1 as libc::c_int;
            define_variable_in_set(
                b".SHELLFLAGS\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                b"-ec\0" as *const u8 as *const libc::c_char,
                o_default,
                0 as libc::c_int,
                (*current_variable_set_list).set,
                0 as *mut floc,
            );
            define_variable_in_set(
                b"CC\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                b"c99\0" as *const u8 as *const libc::c_char,
                o_default,
                0 as libc::c_int,
                (*current_variable_set_list).set,
                0 as *mut floc,
            );
            define_variable_in_set(
                b"CFLAGS\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                b"-O1\0" as *const u8 as *const libc::c_char,
                o_default,
                0 as libc::c_int,
                (*current_variable_set_list).set,
                0 as *mut floc,
            );
            define_variable_in_set(
                b"FC\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                b"fort77\0" as *const u8 as *const libc::c_char,
                o_default,
                0 as libc::c_int,
                (*current_variable_set_list).set,
                0 as *mut floc,
            );
            define_variable_in_set(
                b"FFLAGS\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                b"-O1\0" as *const u8 as *const libc::c_char,
                o_default,
                0 as libc::c_int,
                (*current_variable_set_list).set,
                0 as *mut floc,
            );
            define_variable_in_set(
                b"SCCSGETFLAGS\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                b"-s\0" as *const u8 as *const libc::c_char,
                o_default,
                0 as libc::c_int,
                (*current_variable_set_list).set,
                0 as *mut floc,
            );
            define_variable_in_set(
                b"ARFLAGS\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                b"-rv\0" as *const u8 as *const libc::c_char,
                o_default,
                0 as libc::c_int,
                (*current_variable_set_list).set,
                0 as *mut floc,
            );
        } else if second_expansion == 0
            && (nm == b".SECONDEXPANSION\0" as *const u8 as *const libc::c_char
                || *nm as libc::c_int
                    == *(b".SECONDEXPANSION\0" as *const u8 as *const libc::c_char)
                        as libc::c_int
                    && (*nm as libc::c_int == '\0' as i32
                        || strcmp(
                            nm.offset(1 as libc::c_int as isize),
                            (b".SECONDEXPANSION\0" as *const u8 as *const libc::c_char)
                                .offset(1 as libc::c_int as isize),
                        ) == 0))
        {
            second_expansion = 1 as libc::c_int;
        } else if one_shell == 0
            && (nm == b".ONESHELL\0" as *const u8 as *const libc::c_char
                || *nm as libc::c_int
                    == *(b".ONESHELL\0" as *const u8 as *const libc::c_char)
                        as libc::c_int
                    && (*nm as libc::c_int == '\0' as i32
                        || strcmp(
                            nm.offset(1 as libc::c_int as isize),
                            (b".ONESHELL\0" as *const u8 as *const libc::c_char)
                                .offset(1 as libc::c_int as isize),
                        ) == 0))
        {
            one_shell = 1 as libc::c_int;
        } else if set_default != 0
            && *((*default_goal_var).value).offset(0 as libc::c_int as isize)
                as libc::c_int == '\0' as i32
        {
            let mut d: *mut dep = 0 as *mut dep;
            let mut reject: libc::c_int = 0 as libc::c_int;
            if !(strchr(nm, '%' as i32)).is_null() {
                break;
            }
            if !(*nm as libc::c_int == '.' as i32 && (strchr(nm, '/' as i32)).is_null())
            {
                d = (*suffix_file).deps;
                while !d.is_null() {
                    let mut d2: *mut dep = 0 as *mut dep;
                    if *(if !((*d).name).is_null() {
                        (*d).name
                    } else {
                        (*(*d).file).name
                    }) as libc::c_int != '.' as i32
                        && (nm
                            == (if !((*d).name).is_null() {
                                (*d).name
                            } else {
                                (*(*d).file).name
                            })
                            || *nm as libc::c_int
                                == *(if !((*d).name).is_null() {
                                    (*d).name
                                } else {
                                    (*(*d).file).name
                                }) as libc::c_int
                                && (*nm as libc::c_int == '\0' as i32
                                    || strcmp(
                                        nm.offset(1 as libc::c_int as isize),
                                        (if !((*d).name).is_null() {
                                            (*d).name
                                        } else {
                                            (*(*d).file).name
                                        })
                                            .offset(1 as libc::c_int as isize),
                                    ) == 0))
                    {
                        reject = 1 as libc::c_int;
                        break;
                    } else {
                        d2 = (*suffix_file).deps;
                        while !d2.is_null() {
                            let mut l: size_t = strlen(
                                if !((*d2).name).is_null() {
                                    (*d2).name
                                } else {
                                    (*(*d2).file).name
                                },
                            );
                            if strncmp(
                                nm,
                                (if !((*d2).name).is_null() {
                                    (*d2).name
                                } else {
                                    (*(*d2).file).name
                                }),
                                l,
                            ) == 0 as libc::c_int
                            {
                                if nm.offset(l as isize)
                                    == (if !((*d).name).is_null() {
                                        (*d).name
                                    } else {
                                        (*(*d).file).name
                                    })
                                    || *nm.offset(l as isize) as libc::c_int
                                        == *(if !((*d).name).is_null() {
                                            (*d).name
                                        } else {
                                            (*(*d).file).name
                                        }) as libc::c_int
                                        && (*nm.offset(l as isize) as libc::c_int == '\0' as i32
                                            || strcmp(
                                                nm.offset(l as isize).offset(1 as libc::c_int as isize),
                                                (if !((*d).name).is_null() {
                                                    (*d).name
                                                } else {
                                                    (*(*d).file).name
                                                })
                                                    .offset(1 as libc::c_int as isize),
                                            ) == 0)
                                {
                                    reject = 1 as libc::c_int;
                                    break;
                                }
                            }
                            d2 = (*d2).next;
                        }
                        if reject != 0 {
                            break;
                        }
                        d = (*d).next;
                    }
                }
                if reject == 0 {
                    define_variable_in_set(
                        b".DEFAULT_GOAL\0" as *const u8 as *const libc::c_char,
                        13 as libc::c_int as size_t,
                        (*t).name,
                        o_file,
                        0 as libc::c_int,
                        0 as *mut variable_set,
                        0 as *mut floc,
                    );
                }
            }
        }
        t = (*t).next;
    }
}
unsafe extern "C" fn check_special_file(mut file: *mut file, mut flocp: *const floc) {
    if (*file).name == b".WAIT\0" as *const u8 as *const libc::c_char
        || *(*file).name as libc::c_int
            == *(b".WAIT\0" as *const u8 as *const libc::c_char) as libc::c_int
            && (*(*file).name as libc::c_int == '\0' as i32
                || strcmp(
                    ((*file).name).offset(1 as libc::c_int as isize),
                    (b".WAIT\0" as *const u8 as *const libc::c_char)
                        .offset(1 as libc::c_int as isize),
                ) == 0)
    {
        static mut wpre: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        static mut wcmd: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        if wpre == 0 && !((*file).deps).is_null() {
            error(
                flocp,
                0 as libc::c_int as size_t,
                dcgettext(
                    0 as *const libc::c_char,
                    b".WAIT should not have prerequisites\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            wpre = 1 as libc::c_int as libc::c_uint;
        }
        if wcmd == 0 && !((*file).cmds).is_null() {
            error(
                flocp,
                0 as libc::c_int as size_t,
                dcgettext(
                    0 as *const libc::c_char,
                    b".WAIT should not have commands\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            wcmd = 1 as libc::c_int as libc::c_uint;
        }
        return;
    }
}
unsafe extern "C" fn record_files(
    mut filenames: *mut nameseq,
    mut are_also_makes: libc::c_int,
    mut pattern: *const libc::c_char,
    mut pattern_percent: *const libc::c_char,
    mut depstr: *mut libc::c_char,
    mut cmds_started: libc::c_uint,
    mut commands: *mut libc::c_char,
    mut commands_idx: size_t,
    mut two_colon: libc::c_int,
    mut prefix: libc::c_char,
    mut flocp: *const floc,
) {
    let mut cmds: *mut commands = 0 as *mut commands;
    let mut deps: *mut dep = 0 as *mut dep;
    let mut also_make: *mut dep = 0 as *mut dep;
    let mut implicit_percent: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if snapped_deps != 0 {
        fatal(
            flocp,
            0 as libc::c_int as size_t,
            dcgettext(
                0 as *const libc::c_char,
                b"prerequisites cannot be defined in recipes\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    name = (*filenames).name;
    implicit_percent = find_percent_cached(&mut name);
    if commands_idx > 0 as libc::c_int as libc::c_ulong {
        cmds = xmalloc(::std::mem::size_of::<commands>() as libc::c_ulong)
            as *mut commands;
        (*cmds).fileinfo.filenm = (*flocp).filenm;
        (*cmds).fileinfo.lineno = cmds_started as libc::c_ulong;
        (*cmds).fileinfo.offset = 0 as libc::c_int as libc::c_ulong;
        (*cmds).commands = xstrndup(commands, commands_idx);
        (*cmds).command_lines = 0 as *mut *mut libc::c_char;
        (*cmds).recipe_prefix = prefix;
    } else if are_also_makes != 0 {
        fatal(
            flocp,
            0 as libc::c_int as size_t,
            dcgettext(
                0 as *const libc::c_char,
                b"grouped targets must provide a recipe\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        cmds = 0 as *mut commands;
    }
    if depstr.is_null() {
        deps = 0 as *mut dep;
    } else {
        depstr = unescape_char(depstr, ':' as i32);
        if second_expansion != 0 && !(strchr(depstr, '$' as i32)).is_null() {
            deps = xcalloc(::std::mem::size_of::<dep>() as libc::c_ulong) as *mut dep;
            (*deps).name = depstr;
            (*deps).set_need_2nd_expansion(1 as libc::c_int as libc::c_uint);
            (*deps)
                .set_staticpattern(
                    (pattern != 0 as *const libc::c_char) as libc::c_int as libc::c_uint,
                );
        } else {
            deps = split_prereqs(depstr);
            free(depstr as *mut libc::c_void);
            if pattern.is_null() && implicit_percent.is_null() {
                deps = enter_prereqs(deps, 0 as *const libc::c_char);
            }
        }
    }
    if !implicit_percent.is_null() {
        let mut nextf: *mut nameseq = 0 as *mut nameseq;
        let mut targets: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
        let mut target_pats: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
        let mut c: libc::c_ushort = 0;
        if !pattern.is_null() {
            fatal(
                flocp,
                0 as libc::c_int as size_t,
                dcgettext(
                    0 as *const libc::c_char,
                    b"mixed implicit and static pattern rules\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        nextf = (*filenames).next;
        free(filenames as *mut libc::c_void);
        filenames = nextf;
        c = 1 as libc::c_int as libc::c_ushort;
        while !nextf.is_null() {
            c = c.wrapping_add(1);
            c;
            nextf = (*nextf).next;
        }
        targets = xmalloc(
            (c as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *const libc::c_char;
        target_pats = xmalloc(
            (c as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *const libc::c_char;
        let ref mut fresh13 = *targets.offset(0 as libc::c_int as isize);
        *fresh13 = name;
        let ref mut fresh14 = *target_pats.offset(0 as libc::c_int as isize);
        *fresh14 = implicit_percent;
        c = 1 as libc::c_int as libc::c_ushort;
        while !filenames.is_null() {
            name = (*filenames).name;
            implicit_percent = find_percent_cached(&mut name);
            if implicit_percent.is_null() {
                fatal(
                    flocp,
                    0 as libc::c_int as size_t,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"mixed implicit and normal rules\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            let ref mut fresh15 = *targets.offset(c as isize);
            *fresh15 = name;
            let ref mut fresh16 = *target_pats.offset(c as isize);
            *fresh16 = implicit_percent;
            c = c.wrapping_add(1);
            c;
            nextf = (*filenames).next;
            free(filenames as *mut libc::c_void);
            filenames = nextf;
        }
        create_pattern_rule(
            targets,
            target_pats,
            c,
            two_colon,
            deps,
            cmds,
            1 as libc::c_int,
        );
        return;
    }
    loop {
        let mut nextf_0: *mut nameseq = (*filenames).next;
        let mut f: *mut file = 0 as *mut file;
        let mut this: *mut dep = 0 as *mut dep;
        free(filenames as *mut libc::c_void);
        if !pattern.is_null() && pattern_matches(pattern, pattern_percent, name) == 0 {
            error(
                flocp,
                strlen(name),
                dcgettext(
                    0 as *const libc::c_char,
                    b"target '%s' doesn't match the target pattern\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
            );
        } else if !deps.is_null() {
            this = if !nextf_0.is_null() { copy_dep_chain(deps) } else { deps };
        }
        if two_colon == 0 {
            f = enter_file(strcache_add(name));
            if !((*f).double_colon).is_null() {
                fatal(
                    flocp,
                    strlen((*f).name),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"target file '%s' has both : and :: entries\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*f).name,
                );
            }
            if !cmds.is_null() && cmds == (*f).cmds {
                error(
                    flocp,
                    strlen((*f).name),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"target '%s' given more than once in the same rule\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*f).name,
                );
            } else if !cmds.is_null() && !((*f).cmds).is_null()
                && (*f).is_target() as libc::c_int != 0
            {
                let mut l: size_t = strlen((*f).name);
                error(
                    &mut (*cmds).fileinfo as *mut floc,
                    l,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"warning: overriding recipe for target '%s'\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*f).name,
                );
                error(
                    &mut (*(*f).cmds).fileinfo as *mut floc,
                    l,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"warning: ignoring old recipe for target '%s'\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*f).name,
                );
            }
            if f == default_file && this.is_null() && cmds.is_null() {
                (*f).cmds = 0 as *mut commands;
            }
            if !cmds.is_null() {
                (*f).cmds = cmds;
            }
            if f == suffix_file && this.is_null() {
                free_ns_chain((*f).deps as *mut nameseq);
                (*f).deps = 0 as *mut dep;
            }
            (*f).set_is_explicit(1 as libc::c_int as libc::c_uint);
        } else {
            f = lookup_file(name);
            if !f.is_null() && (*f).is_target() as libc::c_int != 0
                && ((*f).double_colon).is_null()
            {
                fatal(
                    flocp,
                    strlen((*f).name),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"target file '%s' has both : and :: entries\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*f).name,
                );
            }
            f = enter_file(strcache_add(name));
            if ((*f).double_colon).is_null() {
                (*f).double_colon = f;
            }
            (*f).cmds = cmds;
        }
        if are_also_makes != 0 {
            let mut also: *mut dep = xcalloc(
                ::std::mem::size_of::<dep>() as libc::c_ulong,
            ) as *mut dep;
            (*also).name = (*f).name;
            (*also).file = f;
            (*also).next = also_make;
            also_make = also;
        }
        (*f).set_is_target(1 as libc::c_int as libc::c_uint);
        if !pattern.is_null() {
            static mut percent: *const libc::c_char = b"%\0" as *const u8
                as *const libc::c_char;
            let mut o: *mut libc::c_char = patsubst_expand_pat(
                variable_buffer,
                name,
                pattern,
                percent,
                pattern_percent.offset(1 as libc::c_int as isize),
                percent.offset(1 as libc::c_int as isize),
            );
            (*f)
                .stem = strcache_add_len(
                variable_buffer,
                o.offset_from(variable_buffer) as libc::c_long as size_t,
            );
            if !this.is_null() {
                if (*this).need_2nd_expansion() == 0 {
                    this = enter_prereqs(this, (*f).stem);
                } else {
                    (*this).stem = (*f).stem;
                }
            }
        }
        if !this.is_null() {
            if ((*f).deps).is_null() {
                (*f).deps = this;
            } else if !cmds.is_null() {
                let mut d: *mut dep = this;
                while !((*d).next).is_null() {
                    d = (*d).next;
                }
                (*d).next = (*f).deps;
                (*f).deps = this;
            } else {
                let mut d_0: *mut dep = (*f).deps;
                while !((*d_0).next).is_null() {
                    d_0 = (*d_0).next;
                }
                (*d_0).next = this;
            }
        }
        name = (*f).name;
        check_special_file(f, flocp);
        if nextf_0.is_null() {
            break;
        }
        filenames = nextf_0;
        name = (*filenames).name;
        if !(find_percent_cached(&mut name)).is_null() {
            error(
                flocp,
                0 as libc::c_int as size_t,
                dcgettext(
                    0 as *const libc::c_char,
                    b"*** mixed implicit and normal rules: deprecated syntax\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    let mut i: *mut dep = 0 as *mut dep;
    i = also_make;
    while !i.is_null() {
        let mut f_0: *mut file = (*i).file;
        let mut cpy: *mut dep = if !((*i).next).is_null() {
            copy_dep_chain(also_make)
        } else {
            also_make
        };
        if !((*f_0).also_make).is_null() {
            error(
                &mut (*cmds).fileinfo as *mut floc,
                strlen((*f_0).name),
                dcgettext(
                    0 as *const libc::c_char,
                    b"warning: overriding group membership for target '%s'\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*f_0).name,
            );
            free_ns_chain((*f_0).also_make as *mut nameseq);
        }
        (*f_0).also_make = cpy;
        i = (*i).next;
    }
}
unsafe extern "C" fn find_map_unquote(
    mut string: *mut libc::c_char,
    mut stopmap: libc::c_int,
) -> *mut libc::c_char {
    let mut string_len: size_t = 0 as libc::c_int as size_t;
    let mut p: *mut libc::c_char = string;
    stopmap |= 0x1 as libc::c_int;
    loop {
        while !(*stopchar_map.as_mut_ptr().offset(*p as libc::c_uchar as isize)
            as libc::c_int & stopmap != 0 as libc::c_int)
        {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int == '\0' as i32 {
            break;
        }
        if *p as libc::c_int == '$' as i32 {
            let mut openparen: libc::c_char = *p.offset(1 as libc::c_int as isize);
            if openparen as libc::c_int == '\0' as i32 {
                break;
            }
            p = p.offset(2 as libc::c_int as isize);
            if openparen as libc::c_int == '(' as i32
                || openparen as libc::c_int == '{' as i32
            {
                let mut pcount: libc::c_uint = 1 as libc::c_int as libc::c_uint;
                let mut closeparen: libc::c_char = (if openparen as libc::c_int
                    == '(' as i32
                {
                    ')' as i32
                } else {
                    '}' as i32
                }) as libc::c_char;
                while *p != 0 {
                    if *p as libc::c_int == openparen as libc::c_int {
                        pcount = pcount.wrapping_add(1);
                        pcount;
                    } else if *p as libc::c_int == closeparen as libc::c_int {
                        pcount = pcount.wrapping_sub(1);
                        if pcount == 0 as libc::c_int as libc::c_uint {
                            p = p.offset(1);
                            p;
                            break;
                        }
                    }
                    p = p.offset(1);
                    p;
                }
            }
        } else if p > string
            && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\\' as i32
        {
            let mut i: libc::c_int = -(2 as libc::c_int);
            while &mut *p.offset(i as isize) as *mut libc::c_char >= string
                && *p.offset(i as isize) as libc::c_int == '\\' as i32
            {
                i -= 1;
                i;
            }
            i += 1;
            i;
            if string_len == 0 as libc::c_int as libc::c_ulong {
                string_len = strlen(string);
            }
            let mut hi: libc::c_int = -(i / 2 as libc::c_int);
            memmove(
                &mut *p.offset(i as isize) as *mut libc::c_char as *mut libc::c_void,
                &mut *p.offset((i / 2 as libc::c_int) as isize) as *mut libc::c_char
                    as *const libc::c_void,
                string_len
                    .wrapping_sub(p.offset_from(string) as libc::c_long as libc::c_ulong)
                    .wrapping_add(hi as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            p = p.offset((i / 2 as libc::c_int) as isize);
            if i % 2 as libc::c_int == 0 as libc::c_int {
                return p;
            }
        } else {
            return p
        }
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn find_char_unquote(
    mut string: *mut libc::c_char,
    mut stop: libc::c_int,
) -> *mut libc::c_char {
    let mut string_len: size_t = 0 as libc::c_int as size_t;
    let mut p: *mut libc::c_char = string;
    loop {
        p = strchr(p, stop);
        if p.is_null() {
            return 0 as *mut libc::c_char;
        }
        if p > string
            && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\\' as i32
        {
            let mut i: libc::c_int = -(2 as libc::c_int);
            while &mut *p.offset(i as isize) as *mut libc::c_char >= string
                && *p.offset(i as isize) as libc::c_int == '\\' as i32
            {
                i -= 1;
                i;
            }
            i += 1;
            i;
            if string_len == 0 as libc::c_int as libc::c_ulong {
                string_len = strlen(string);
            }
            let mut hi: libc::c_int = -(i / 2 as libc::c_int);
            memmove(
                &mut *p.offset(i as isize) as *mut libc::c_char as *mut libc::c_void,
                &mut *p.offset((i / 2 as libc::c_int) as isize) as *mut libc::c_char
                    as *const libc::c_void,
                string_len
                    .wrapping_sub(p.offset_from(string) as libc::c_long as libc::c_ulong)
                    .wrapping_add(hi as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            p = p.offset((i / 2 as libc::c_int) as isize);
            if i % 2 as libc::c_int == 0 as libc::c_int {
                return p;
            }
        } else {
            return p
        }
    };
}
unsafe extern "C" fn unescape_char(
    mut string: *mut libc::c_char,
    mut c: libc::c_int,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = string;
    let mut s: *mut libc::c_char = string;
    while *s as libc::c_int != '\0' as i32 {
        if *s as libc::c_int == '\\' as i32 {
            let mut e: *mut libc::c_char = s;
            let mut l: size_t = 0;
            while *e as libc::c_int == '\\' as i32 {
                e = e.offset(1);
                e;
            }
            l = e.offset_from(s) as libc::c_long as size_t;
            if *e as libc::c_int != c
                || l.wrapping_rem(2 as libc::c_int as libc::c_ulong)
                    == 0 as libc::c_int as libc::c_ulong
            {
                memmove(p as *mut libc::c_void, s as *const libc::c_void, l);
                p = p.offset(l as isize);
                if *e as libc::c_int == '\0' as i32 {
                    break;
                }
            } else if l > 1 as libc::c_int as libc::c_ulong {
                l = (l as libc::c_ulong).wrapping_div(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
                memmove(p as *mut libc::c_void, s as *const libc::c_void, l);
                p = p.offset(l as isize);
            }
            s = e;
        }
        let fresh17 = s;
        s = s.offset(1);
        let fresh18 = p;
        p = p.offset(1);
        *fresh18 = *fresh17;
    }
    *p = '\0' as i32 as libc::c_char;
    return string;
}
pub unsafe extern "C" fn find_percent(
    mut pattern: *mut libc::c_char,
) -> *mut libc::c_char {
    return find_char_unquote(pattern, '%' as i32);
}
pub unsafe extern "C" fn find_percent_cached(
    mut string: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = strchr(*string, '%' as i32);
    let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut np: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut slen: size_t = 0;
    if p.is_null() || p == *string
        || *p.offset(-(1 as libc::c_int) as isize) as libc::c_int != '\\' as i32
    {
        return p;
    }
    slen = strlen(*string);
    let mut fresh19 = ::std::vec::from_elem(
        0,
        slen.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
    );
    new = fresh19.leak().as_mut_ptr() as *mut libc::c_char;
    memcpy(
        new as *mut libc::c_void,
        *string as *const libc::c_void,
        slen.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    np = new.offset(p.offset_from(*string) as libc::c_long as isize);
    loop {
        let mut pp: *mut libc::c_char = np;
        let mut i: libc::c_int = -(2 as libc::c_int);
        while &mut *np.offset(i as isize) as *mut libc::c_char >= new
            && *np.offset(i as isize) as libc::c_int == '\\' as i32
        {
            i -= 1;
            i;
        }
        i += 1;
        i;
        let mut hi: libc::c_int = -(i / 2 as libc::c_int);
        memmove(
            &mut *pp.offset(i as isize) as *mut libc::c_char as *mut libc::c_void,
            &mut *pp.offset((i / 2 as libc::c_int) as isize) as *mut libc::c_char
                as *const libc::c_void,
            slen
                .wrapping_sub(pp.offset_from(new) as libc::c_long as libc::c_ulong)
                .wrapping_add(hi as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        slen = (slen as libc::c_ulong)
            .wrapping_add((i / 2 as libc::c_int + i % 2 as libc::c_int) as libc::c_ulong)
            as size_t as size_t;
        np = np.offset((i / 2 as libc::c_int) as isize);
        if i % 2 as libc::c_int == 0 as libc::c_int {
            break;
        }
        np = strchr(np, '%' as i32);
        if !(!np.is_null()
            && *np.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\\' as i32)
        {
            break;
        }
    }
    *string = strcache_add(new);
    return if !np.is_null() {
        (*string).offset(np.offset_from(new) as libc::c_long as isize)
    } else {
        0 as *const libc::c_char
    };
}
unsafe extern "C" fn readstring(mut ebuf: *mut ebuffer) -> libc::c_long {
    let mut eol: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*ebuf).bufnext >= ((*ebuf).bufstart).offset((*ebuf).size as isize) {
        return -(1 as libc::c_int) as libc::c_long;
    }
    (*ebuf).buffer = (*ebuf).bufnext;
    eol = (*ebuf).buffer;
    loop {
        let mut backslash: libc::c_int = 0 as libc::c_int;
        let mut bol: *const libc::c_char = eol;
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        eol = strchr(eol, '\n' as i32);
        p = eol;
        if eol.is_null() {
            (*ebuf)
                .bufnext = ((*ebuf).bufstart)
                .offset((*ebuf).size as isize)
                .offset(1 as libc::c_int as isize);
            return 0 as libc::c_int as libc::c_long;
        }
        while p > bol
            && {
                p = p.offset(-1);
                *p as libc::c_int == '\\' as i32
            }
        {
            backslash = (backslash == 0) as libc::c_int;
        }
        if backslash == 0 {
            break;
        }
        eol = eol.offset(1);
        eol;
    }
    *eol = '\0' as i32 as libc::c_char;
    (*ebuf).bufnext = eol.offset(1 as libc::c_int as isize);
    return 0 as libc::c_int as libc::c_long;
}
unsafe extern "C" fn readline(mut ebuf: *mut ebuffer) -> libc::c_long {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nlines: libc::c_long = 0 as libc::c_int as libc::c_long;
    if ((*ebuf).fp).is_null() {
        return readstring(ebuf);
    }
    start = (*ebuf).bufstart;
    p = start;
    end = p.offset((*ebuf).size as isize);
    *p = '\0' as i32 as libc::c_char;
    while !(fgets(p, end.offset_from(p) as libc::c_long as libc::c_int, (*ebuf).fp))
        .is_null()
    {
        let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: size_t = 0;
        let mut backslash: libc::c_int = 0;
        len = strlen(p);
        if len == 0 as libc::c_int as libc::c_ulong {
            error(
                &mut (*ebuf).floc as *mut floc,
                0 as libc::c_int as size_t,
                dcgettext(
                    0 as *const libc::c_char,
                    b"warning: NUL character seen; rest of line ignored\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            *p.offset(0 as libc::c_int as isize) = '\n' as i32 as libc::c_char;
            len = 1 as libc::c_int as size_t;
        }
        p = p.offset(len as isize);
        if !(*p.offset(-(1 as libc::c_int) as isize) as libc::c_int != '\n' as i32) {
            nlines += 1;
            nlines;
            if p.offset_from(start) as libc::c_long > 1 as libc::c_int as libc::c_long
                && *p.offset(-(2 as libc::c_int) as isize) as libc::c_int == '\r' as i32
            {
                p = p.offset(-1);
                p;
                memmove(
                    p.offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                    p as *const libc::c_void,
                    (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
            }
            backslash = 0 as libc::c_int;
            p2 = p.offset(-(2 as libc::c_int as isize));
            while p2 >= start {
                if *p2 as libc::c_int != '\\' as i32 {
                    break;
                }
                backslash = (backslash == 0) as libc::c_int;
                p2 = p2.offset(-1);
                p2;
            }
            if backslash == 0 {
                *p.offset(-(1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
                break;
            } else if end.offset_from(p) as libc::c_long
                >= 80 as libc::c_int as libc::c_long
            {
                continue;
            }
        }
        let mut off: size_t = p.offset_from(start) as libc::c_long as size_t;
        (*ebuf)
            .size = ((*ebuf).size as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        (*ebuf)
            .bufstart = xrealloc(start as *mut libc::c_void, (*ebuf).size)
            as *mut libc::c_char;
        (*ebuf).buffer = (*ebuf).bufstart;
        start = (*ebuf).buffer;
        p = start.offset(off as isize);
        end = start.offset((*ebuf).size as isize);
        *p = '\0' as i32 as libc::c_char;
    }
    if ferror((*ebuf).fp) != 0 {
        pfatal_with_name((*ebuf).floc.filenm);
    }
    return if nlines != 0 {
        nlines
    } else {
        (if p == (*ebuf).bufstart { -(1 as libc::c_int) } else { 1 as libc::c_int })
            as libc::c_long
    };
}
unsafe extern "C" fn get_next_mword(
    mut buffer: *mut libc::c_char,
    mut startp: *mut *mut libc::c_char,
    mut length: *mut size_t,
) -> make_word_type {
    let mut current_block: u64;
    let mut wtype: make_word_type = w_bogus;
    let mut p: *mut libc::c_char = buffer;
    let mut beg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    while *stopchar_map.as_mut_ptr().offset(*p as libc::c_uchar as isize) as libc::c_int
        & (0x2 as libc::c_int | 0x4 as libc::c_int) != 0 as libc::c_int
    {
        p = p.offset(1);
        p;
    }
    beg = p;
    let fresh20 = p;
    p = p.offset(1);
    c = *fresh20;
    match c as libc::c_int {
        0 => {
            wtype = w_eol;
            current_block = 8611510689881074683;
        }
        59 => {
            wtype = w_semicolon;
            current_block = 8611510689881074683;
        }
        61 => {
            wtype = w_varassign;
            current_block = 8611510689881074683;
        }
        58 => {
            if *p as libc::c_int == '=' as i32 {
                p = p.offset(1);
                p;
                wtype = w_varassign;
            } else if *p as libc::c_int == ':' as i32 {
                p = p.offset(1);
                p;
                if *p.offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32 {
                    p = p.offset(1);
                    p;
                    wtype = w_varassign;
                } else {
                    wtype = w_dcolon;
                }
            } else {
                wtype = w_colon;
            }
            current_block = 8611510689881074683;
        }
        38 => {
            if *p as libc::c_int == ':' as i32 {
                p = p.offset(1);
                p;
                if *p as libc::c_int != ':' as i32 {
                    wtype = w_ampcolon;
                } else {
                    p = p.offset(1);
                    p;
                    wtype = w_ampdcolon;
                }
                current_block = 8611510689881074683;
            } else {
                current_block = 15897653523371991391;
            }
        }
        43 | 63 | 33 => {
            if *p as libc::c_int == '=' as i32 {
                p = p.offset(1);
                p;
                wtype = w_varassign;
                current_block = 8611510689881074683;
            } else {
                current_block = 15897653523371991391;
            }
        }
        _ => {
            current_block = 15897653523371991391;
        }
    }
    match current_block {
        15897653523371991391 => {
            wtype = w_static;
            loop {
                let mut closeparen: libc::c_char = 0;
                let mut count: libc::c_int = 0;
                if *stopchar_map.as_mut_ptr().offset(c as libc::c_uchar as isize)
                    as libc::c_int
                    & (0x2 as libc::c_int | 0x4 as libc::c_int | 0x1 as libc::c_int)
                    != 0 as libc::c_int
                {
                    break;
                }
                match c as libc::c_int {
                    61 | 58 => {
                        break;
                    }
                    36 => {
                        let fresh21 = p;
                        p = p.offset(1);
                        c = *fresh21;
                        if !(c as libc::c_int == '$' as i32) {
                            if c as libc::c_int == '\0' as i32 {
                                break;
                            }
                            wtype = w_variable;
                            if c as libc::c_int == '(' as i32 {
                                closeparen = ')' as i32 as libc::c_char;
                                current_block = 790185930182612747;
                            } else if c as libc::c_int == '{' as i32 {
                                closeparen = '}' as i32 as libc::c_char;
                                current_block = 790185930182612747;
                            } else {
                                current_block = 9512719473022792396;
                            }
                            match current_block {
                                9512719473022792396 => {}
                                _ => {
                                    count = 0 as libc::c_int;
                                    while *p as libc::c_int != '\0' as i32 {
                                        if *p as libc::c_int == c as libc::c_int {
                                            count += 1;
                                            count;
                                        } else if *p as libc::c_int == closeparen as libc::c_int
                                            && {
                                                count -= 1;
                                                count < 0 as libc::c_int
                                            }
                                        {
                                            p = p.offset(1);
                                            p;
                                            break;
                                        }
                                        p = p.offset(1);
                                        p;
                                    }
                                }
                            }
                        }
                    }
                    63 | 43 => {
                        if *p as libc::c_int == '=' as i32 {
                            break;
                        }
                    }
                    92 => {
                        match *p as libc::c_int {
                            58 | 59 | 61 | 92 => {
                                p = p.offset(1);
                                p;
                            }
                            _ => {}
                        }
                    }
                    38 => {
                        if *p as libc::c_int == ':' as i32 {
                            break;
                        }
                    }
                    _ => {}
                }
                let fresh22 = p;
                p = p.offset(1);
                c = *fresh22;
            }
            p = p.offset(-1);
            p;
        }
        _ => {}
    }
    if !startp.is_null() {
        *startp = beg;
    }
    if !length.is_null() {
        *length = p.offset_from(beg) as libc::c_long as size_t;
    }
    return wtype;
}
pub unsafe extern "C" fn construct_include_path(mut arg_dirs: *mut *const libc::c_char) {
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
    let mut dirs: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut cpp: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut idx: size_t = 0;
    let mut disable: libc::c_int = 0 as libc::c_int;
    idx = (::std::mem::size_of::<[*const libc::c_char; 4]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong);
    if !arg_dirs.is_null() {
        cpp = arg_dirs;
        while !(*cpp).is_null() {
            idx = idx.wrapping_add(1);
            idx;
            cpp = cpp.offset(1);
            cpp;
        }
    }
    dirs = xmalloc(
        idx.wrapping_mul(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong),
    ) as *mut *const libc::c_char;
    idx = 0 as libc::c_int as size_t;
    max_incl_len = 0 as libc::c_int as size_t;
    if !arg_dirs.is_null() {
        while !(*arg_dirs).is_null() {
            let fresh23 = arg_dirs;
            arg_dirs = arg_dirs.offset(1);
            let mut dir: *const libc::c_char = *fresh23;
            let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut e: libc::c_int = 0;
            if *dir.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
                && *dir.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
            {
                disable = 1 as libc::c_int;
                idx = 0 as libc::c_int as size_t;
                max_incl_len = 0 as libc::c_int as size_t;
            } else {
                if *dir.offset(0 as libc::c_int as isize) as libc::c_int == '~' as i32 {
                    expanded = tilde_expand(dir);
                    if !expanded.is_null() {
                        dir = expanded;
                    }
                }
                loop {
                    e = stat(dir, &mut stbuf);
                    if !(e == -(1 as libc::c_int)
                        && *__errno_location() == 4 as libc::c_int)
                    {
                        break;
                    }
                }
                if e == 0 as libc::c_int
                    && stbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint
                {
                    let mut len: size_t = strlen(dir);
                    while len > 1 as libc::c_int as libc::c_ulong
                        && *dir
                            .offset(
                                len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int == '/' as i32
                    {
                        len = len.wrapping_sub(1);
                        len;
                    }
                    if len > max_incl_len {
                        max_incl_len = len;
                    }
                    let fresh24 = idx;
                    idx = idx.wrapping_add(1);
                    let ref mut fresh25 = *dirs.offset(fresh24 as isize);
                    *fresh25 = strcache_add_len(dir, len);
                }
                free(expanded as *mut libc::c_void);
            }
        }
    }
    if disable == 0 {
        cpp = default_include_directories.as_mut_ptr();
        while !(*cpp).is_null() {
            let mut e_0: libc::c_int = 0;
            loop {
                e_0 = stat(*cpp, &mut stbuf);
                if !(e_0 == -(1 as libc::c_int)
                    && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
            }
            if e_0 == 0 as libc::c_int
                && stbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint
            {
                let mut len_0: size_t = strlen(*cpp);
                while len_0 > 1 as libc::c_int as libc::c_ulong
                    && *(*cpp)
                        .offset(
                            len_0.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) as libc::c_int == '/' as i32
                {
                    len_0 = len_0.wrapping_sub(1);
                    len_0;
                }
                if len_0 > max_incl_len {
                    max_incl_len = len_0;
                }
                let fresh26 = idx;
                idx = idx.wrapping_add(1);
                let ref mut fresh27 = *dirs.offset(fresh26 as isize);
                *fresh27 = strcache_add_len(*cpp, len_0);
            }
            cpp = cpp.offset(1);
            cpp;
        }
    }
    let ref mut fresh28 = *dirs.offset(idx as isize);
    *fresh28 = 0 as *const libc::c_char;
    do_variable_definition(
        0 as *mut floc,
        b".INCLUDE_DIRS\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        o_default,
        f_simple,
        0 as libc::c_int,
    );
    cpp = dirs;
    while !(*cpp).is_null() {
        do_variable_definition(
            0 as *mut floc,
            b".INCLUDE_DIRS\0" as *const u8 as *const libc::c_char,
            *cpp,
            o_default,
            f_append,
            0 as libc::c_int,
        );
        cpp = cpp.offset(1);
        cpp;
    }
    free(include_directories as *mut libc::c_void);
    include_directories = dirs;
}
pub unsafe extern "C" fn tilde_expand(
    mut name: *const libc::c_char,
) -> *mut libc::c_char {
    if *name.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        || *name.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        let mut home_dir: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut is_variable: libc::c_int = 0;
        let mut save: libc::c_int = warn_undefined_variables_flag;
        warn_undefined_variables_flag = 0 as libc::c_int;
        home_dir = allocated_variable_expand_for_file(
            b"$(HOME)\0" as *const u8 as *const libc::c_char,
            0 as *mut file,
        );
        warn_undefined_variables_flag = save;
        is_variable = (*home_dir.offset(0 as libc::c_int as isize) as libc::c_int
            != '\0' as i32) as libc::c_int;
        if is_variable == 0 {
            free(home_dir as *mut libc::c_void);
            home_dir = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
        }
        if home_dir.is_null()
            || *home_dir.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            let mut logname: *mut libc::c_char = getlogin();
            home_dir = 0 as *mut libc::c_char;
            if !logname.is_null() {
                let mut p: *mut passwd = getpwnam(logname);
                if !p.is_null() {
                    home_dir = (*p).pw_dir;
                }
            }
        }
        if !home_dir.is_null() {
            let mut new: *mut libc::c_char = xstrdup(
                concat(
                    2 as libc::c_int as libc::c_uint,
                    home_dir,
                    name.offset(1 as libc::c_int as isize),
                ),
            );
            if is_variable != 0 {
                free(home_dir as *mut libc::c_void);
            }
            return new;
        }
    } else {
        let mut pwent: *mut passwd = 0 as *mut passwd;
        let mut userend: *mut libc::c_char = strchr(
            name.offset(1 as libc::c_int as isize),
            '/' as i32,
        );
        if !userend.is_null() {
            *userend = '\0' as i32 as libc::c_char;
        }
        pwent = getpwnam(name.offset(1 as libc::c_int as isize));
        if !pwent.is_null() {
            if userend.is_null() {
                return xstrdup((*pwent).pw_dir);
            }
            *userend = '/' as i32 as libc::c_char;
            return xstrdup(
                concat(
                    3 as libc::c_int as libc::c_uint,
                    (*pwent).pw_dir,
                    b"/\0" as *const u8 as *const libc::c_char,
                    userend.offset(1 as libc::c_int as isize),
                ),
            );
        } else if !userend.is_null() {
            *userend = '/' as i32 as libc::c_char;
        }
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn parse_file_seq(
    mut stringp: *mut *mut libc::c_char,
    mut size: size_t,
    mut stopmap: libc::c_int,
    mut prefix: *const libc::c_char,
    mut flags: libc::c_int,
) -> *mut libc::c_void {
    static mut tmpbuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    let mut cachep: libc::c_int = !(flags & 0x10 as libc::c_int != 0 as libc::c_int)
        as libc::c_int;
    let mut new: *mut nameseq = 0 as *mut nameseq;
    let mut newp: *mut *mut nameseq = &mut new;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gl: glob_t = glob_t {
        gl_pathc: 0,
        gl_pathv: 0 as *mut *mut libc::c_char,
        gl_offs: 0,
        gl_flags: 0,
        gl_closedir: None,
        gl_readdir: None,
        gl_opendir: None,
        gl_lstat: None,
        gl_stat: None,
    };
    let mut tp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut findmap: libc::c_int = stopmap | 0 as libc::c_int | 0x1 as libc::c_int;
    let mut found_wait: libc::c_int = 0 as libc::c_int;
    if !(flags & 0x20 as libc::c_int != 0 as libc::c_int) {
        findmap |= 0x2 as libc::c_int;
    }
    stopmap |= 0x1 as libc::c_int;
    if size < ::std::mem::size_of::<nameseq>() as libc::c_ulong {
        size = ::std::mem::size_of::<nameseq>() as libc::c_ulong;
    }
    if !(flags & 0x4 as libc::c_int != 0 as libc::c_int) {
        dir_setup_glob(&mut gl);
    }
    static mut tmpbuf_len: size_t = 0 as libc::c_int as size_t;
    let mut l: size_t = (strlen(*stringp))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    if l > tmpbuf_len {
        tmpbuf = xrealloc(tmpbuf as *mut libc::c_void, l) as *mut libc::c_char;
        tmpbuf_len = l;
    }
    tp = tmpbuf;
    p = *stringp;
    loop {
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        let mut nlist: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
        let mut tildep: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut globme: libc::c_int = 1 as libc::c_int;
        let mut arname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut memname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut nlen: size_t = 0;
        let mut tot: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        while *stopchar_map.as_mut_ptr().offset(*p as libc::c_uchar as isize)
            as libc::c_int & (0x2 as libc::c_int | 0x4 as libc::c_int)
            != 0 as libc::c_int
        {
            p = p.offset(1);
            p;
        }
        if *stopchar_map.as_mut_ptr().offset(*p as libc::c_uchar as isize) as libc::c_int
            & stopmap != 0 as libc::c_int
        {
            break;
        }
        s = p;
        p = find_map_unquote(p, findmap);
        if p.is_null() {
            p = s.offset(strlen(s) as isize);
        }
        if flags & 0x40 as libc::c_int != 0 as libc::c_int
            && p.offset_from(s) as libc::c_long as libc::c_ulong
                == (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && memcmp(
                s as *const libc::c_void,
                b".WAIT\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
        {
            found_wait = 1 as libc::c_int;
        } else {
            if !(flags & 0x1 as libc::c_int != 0 as libc::c_int) {
                while p.offset_from(s) as libc::c_long > 2 as libc::c_int as libc::c_long
                    && *s.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
                    && *s.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
                {
                    s = s.offset(2 as libc::c_int as isize);
                    while *s as libc::c_int == '/' as i32 {
                        s = s.offset(1);
                        s;
                    }
                }
            }
            if s == p {
                *tp.offset(0 as libc::c_int as isize) = '.' as i32 as libc::c_char;
                *tp.offset(1 as libc::c_int as isize) = '/' as i32 as libc::c_char;
                *tp.offset(2 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                nlen = 2 as libc::c_int as size_t;
            } else {
                nlen = p.offset_from(s) as libc::c_long as size_t;
                memcpy(tp as *mut libc::c_void, s as *const libc::c_void, nlen);
                *tp.offset(nlen as isize) = '\0' as i32 as libc::c_char;
            }
            if !(flags & 0x2 as libc::c_int != 0 as libc::c_int) && tp == tmpbuf
                && *tp.offset(0 as libc::c_int as isize) as libc::c_int != '(' as i32
                && *tp
                    .offset(
                        nlen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int != ')' as i32
            {
                let mut n: *mut libc::c_char = strchr(tp, '(' as i32);
                if !n.is_null() {
                    let mut e: *const libc::c_char = p;
                    loop {
                        let mut o: *const libc::c_char = e;
                        while *stopchar_map
                            .as_mut_ptr()
                            .offset(*e as libc::c_uchar as isize) as libc::c_int
                            & (0x2 as libc::c_int | 0x4 as libc::c_int)
                            != 0 as libc::c_int
                        {
                            e = e.offset(1);
                            e;
                        }
                        while !(*stopchar_map
                            .as_mut_ptr()
                            .offset(*e as libc::c_uchar as isize) as libc::c_int
                            & findmap != 0 as libc::c_int)
                        {
                            e = e.offset(1);
                            e;
                        }
                        if e == o {
                            break;
                        }
                        if *e.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            == ')' as i32
                        {
                            nlen = (nlen as libc::c_ulong)
                                .wrapping_sub(
                                    n.offset(1 as libc::c_int as isize).offset_from(tp)
                                        as libc::c_long as libc::c_ulong,
                                ) as size_t as size_t;
                            tp = n.offset(1 as libc::c_int as isize);
                            break;
                        } else if !(*e as libc::c_int != '\0' as i32) {
                            break;
                        }
                    }
                    if nlen == 0 {
                        continue;
                    }
                }
            }
            if tp > tmpbuf {
                if *tp
                    .offset(
                        nlen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int == ')' as i32
                {
                    tp = tmpbuf;
                    if nlen == 1 as libc::c_int as libc::c_ulong {
                        continue;
                    }
                } else {
                    let fresh29 = nlen;
                    nlen = nlen.wrapping_add(1);
                    *tp.offset(fresh29 as isize) = ')' as i32 as libc::c_char;
                    *tp.offset(nlen as isize) = '\0' as i32 as libc::c_char;
                }
            }
            if flags & 0x4 as libc::c_int != 0 as libc::c_int {
                let mut _ns: *mut nameseq = xcalloc(size) as *mut nameseq;
                let mut __n: *const libc::c_char = concat(
                    2 as libc::c_int as libc::c_uint,
                    prefix,
                    tmpbuf,
                );
                (*_ns)
                    .name = if cachep != 0 {
                    strcache_add(__n)
                } else {
                    xstrdup(__n) as *const libc::c_char
                };
                if found_wait != 0 {
                    let ref mut fresh30 = *(_ns as *mut dep);
                    (*fresh30).set_wait_here(1 as libc::c_int as libc::c_uint);
                    found_wait = 0 as libc::c_int;
                }
                *newp = _ns;
                newp = &mut (*_ns).next;
            } else {
                name = tmpbuf;
                if *tmpbuf.offset(0 as libc::c_int as isize) as libc::c_int == '~' as i32
                {
                    tildep = tilde_expand(tmpbuf);
                    if !tildep.is_null() {
                        name = tildep;
                    }
                }
                if !(flags & 0x2 as libc::c_int != 0 as libc::c_int)
                    && ar_name(name) != 0
                {
                    ar_parse_name(name, &mut arname, &mut memname);
                    name = arname;
                }
                if !(flags & 0x8 as libc::c_int != 0 as libc::c_int)
                    && (strpbrk(name, b"?*[\0" as *const u8 as *const libc::c_char))
                        .is_null()
                {
                    globme = 0 as libc::c_int;
                    tot = 1 as libc::c_int;
                    nlist = &mut name;
                } else {
                    let mut current_block_76: u64;
                    match glob(
                        name,
                        (1 as libc::c_int) << 9 as libc::c_int,
                        None,
                        &mut gl,
                    ) {
                        1 => {
                            out_of_memory();
                        }
                        0 => {
                            tot = gl.gl_pathc as libc::c_int;
                            nlist = gl.gl_pathv as *mut *const libc::c_char;
                            current_block_76 = 15864857819291709765;
                        }
                        3 => {
                            if flags & 0x8 as libc::c_int != 0 as libc::c_int {
                                tot = 0 as libc::c_int;
                                current_block_76 = 15864857819291709765;
                            } else {
                                current_block_76 = 9323778813750240306;
                            }
                        }
                        _ => {
                            current_block_76 = 9323778813750240306;
                        }
                    }
                    match current_block_76 {
                        9323778813750240306 => {
                            tot = 1 as libc::c_int;
                            nlist = &mut name;
                        }
                        _ => {}
                    }
                }
                i = 0 as libc::c_int;
                while i < tot {
                    if !memname.is_null() {
                        let mut found: *mut nameseq = ar_glob(
                            *nlist.offset(i as isize),
                            memname,
                            size,
                        );
                        if found.is_null() {
                            let mut _ns_0: *mut nameseq = xcalloc(size) as *mut nameseq;
                            let mut __n_0: *const libc::c_char = concat(
                                5 as libc::c_int as libc::c_uint,
                                prefix,
                                *nlist.offset(i as isize),
                                b"(\0" as *const u8 as *const libc::c_char,
                                memname,
                                b")\0" as *const u8 as *const libc::c_char,
                            );
                            (*_ns_0)
                                .name = if cachep != 0 {
                                strcache_add(__n_0)
                            } else {
                                xstrdup(__n_0) as *const libc::c_char
                            };
                            if found_wait != 0 {
                                let ref mut fresh31 = *(_ns_0 as *mut dep);
                                (*fresh31).set_wait_here(1 as libc::c_int as libc::c_uint);
                                found_wait = 0 as libc::c_int;
                            }
                            *newp = _ns_0;
                            newp = &mut (*_ns_0).next;
                        } else {
                            if !(*newp).is_null() {
                                (**newp).next = found;
                            } else {
                                *newp = found;
                            }
                            loop {
                                if cachep == 0 {
                                    (*found)
                                        .name = xstrdup(
                                        concat(2 as libc::c_int as libc::c_uint, prefix, name),
                                    );
                                } else if !prefix.is_null() {
                                    (*found)
                                        .name = strcache_add(
                                        concat(2 as libc::c_int as libc::c_uint, prefix, name),
                                    );
                                }
                                if ((*found).next).is_null() {
                                    break;
                                }
                                found = (*found).next;
                            }
                            newp = &mut (*found).next;
                        }
                    } else {
                        let mut _ns_1: *mut nameseq = xcalloc(size) as *mut nameseq;
                        let mut __n_1: *const libc::c_char = concat(
                            2 as libc::c_int as libc::c_uint,
                            prefix,
                            *nlist.offset(i as isize),
                        );
                        (*_ns_1)
                            .name = if cachep != 0 {
                            strcache_add(__n_1)
                        } else {
                            xstrdup(__n_1) as *const libc::c_char
                        };
                        if found_wait != 0 {
                            let ref mut fresh32 = *(_ns_1 as *mut dep);
                            (*fresh32).set_wait_here(1 as libc::c_int as libc::c_uint);
                            found_wait = 0 as libc::c_int;
                        }
                        *newp = _ns_1;
                        newp = &mut (*_ns_1).next;
                    }
                    i += 1;
                    i;
                }
                if globme != 0 {
                    globfree(&mut gl);
                }
                free(arname as *mut libc::c_void);
                free(tildep as *mut libc::c_void);
            }
        }
    }
    *stringp = p;
    return new as *mut libc::c_void;
}
