use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn realpath(
        __name: *const libc::c_char,
        __resolved: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn error(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...) -> !;
    fn make_lltoa(_: libc::c_longlong, _: *mut libc::c_char) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrndup(_: *const libc::c_char, _: size_t) -> *mut libc::c_char;
    fn find_next_token(_: *mut *const libc::c_char, _: *mut size_t) -> *mut libc::c_char;
    fn next_token(_: *const libc::c_char) -> *mut libc::c_char;
    fn end_of_token(_: *const libc::c_char) -> *mut libc::c_char;
    fn alpha_compare(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int;
    fn find_percent(_: *mut libc::c_char) -> *mut libc::c_char;
    static mut stopchar_map: [libc::c_ushort; 0];
    fn strcache_add(str: *const libc::c_char) -> *const libc::c_char;
    static mut reading_file: *const floc;
    static mut expanding_var: *mut *const floc;
    static mut command_count: libc::c_ulong;
    static mut starting_directory: *mut libc::c_char;
    fn hash_free(ht: *mut hash_table, free_items: libc::c_int);
    fn hash_load(
        ht: *mut hash_table,
        item_table: *mut libc::c_void,
        cardinality: libc::c_ulong,
        size: libc::c_ulong,
    );
    fn hash_find_item(
        ht: *mut hash_table,
        key: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_insert(ht: *mut hash_table, item: *const libc::c_void) -> *mut libc::c_void;
    fn hash_init(
        ht: *mut hash_table,
        size: libc::c_ulong,
        hash_1: hash_func_t,
        hash_2: hash_func_t,
        hash_cmp: hash_cmp_func_t,
    );
    fn jhash(key: *const libc::c_uchar, n: libc::c_int) -> libc::c_uint;
    fn jhash_string(key: *const libc::c_uchar) -> libc::c_uint;
    static mut current_variable_set_list: *mut variable_set_list;
    fn variable_buffer_output(
        ptr: *mut libc::c_char,
        string: *const libc::c_char,
        length: size_t,
    ) -> *mut libc::c_char;
    fn allocated_variable_expand_for_file(
        line: *const libc::c_char,
        file: *mut file,
    ) -> *mut libc::c_char;
    fn expand_argument(
        str: *const libc::c_char,
        end: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn variable_expand_string(
        line: *mut libc::c_char,
        string: *const libc::c_char,
        length: size_t,
    ) -> *mut libc::c_char;
    fn install_variable_buffer(bufp: *mut *mut libc::c_char, lenp: *mut size_t);
    fn restore_variable_buffer(buf: *mut libc::c_char, len: size_t);
    fn push_new_variable_scope() -> *mut variable_set_list;
    fn pop_variable_scope();
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
    fn warn_undefined(name: *const libc::c_char, length: size_t);
    fn target_environment(
        file: *mut file,
        recursive: libc::c_int,
    ) -> *mut *mut libc::c_char;
    fn parse_file_seq(
        stringp: *mut *mut libc::c_char,
        size: size_t,
        stopmap: libc::c_int,
        prefix: *const libc::c_char,
        flags: libc::c_int,
    ) -> *mut libc::c_void;
    fn eval_buffer(buffer: *mut libc::c_char, floc: *const floc);
    static mut output_context: *mut output;
    fn output_start();
    fn free_childbase(child: *mut childbase);
    fn construct_command_argv(
        line: *mut libc::c_char,
        restp: *mut *mut libc::c_char,
        file: *mut file,
        cmd_flags: libc::c_int,
        batch_file: *mut *mut libc::c_char,
    ) -> *mut *mut libc::c_char;
    fn outputs(is_err: libc::c_int, msg: *const libc::c_char);
    fn reap_children(block: libc::c_int, err: libc::c_int);
    fn child_execute_job(
        child: *mut childbase,
        good_stdin: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> pid_t;
    fn fd_noinherit(_: libc::c_int);
    static mut db_level: libc::c_int;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type gmk_func_ptr = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        libc::c_uint,
        *mut *mut libc::c_char,
    ) -> *mut libc::c_char,
>;
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
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub struct function_table_entry {
    pub fptr: C2RustUnnamed,
    pub name: *const libc::c_char,
    pub len: libc::c_uchar,
    pub minimum_args: libc::c_uchar,
    pub maximum_args: libc::c_uchar,
    #[bitfield(name = "expand_args", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "alloc_fn", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "adds_command", ty = "libc::c_uint", bits = "2..=2")]
    pub expand_args_alloc_fn_adds_command: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub func_ptr: Option::<
        unsafe extern "C" fn(
            *mut libc::c_char,
            *mut *mut libc::c_char,
            *const libc::c_char,
        ) -> *mut libc::c_char,
    >,
    pub alloc_func_ptr: gmk_func_ptr,
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
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct a_word {
    pub chain: *mut a_word,
    pub str_0: *mut libc::c_char,
    pub length: size_t,
    pub matched: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct a_pattern {
    pub str_0: *mut libc::c_char,
    pub percent: *mut libc::c_char,
    pub length: size_t,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
unsafe extern "C" fn function_table_entry_hash_1(
    mut keyv: *const libc::c_void,
) -> libc::c_ulong {
    let mut key: *const function_table_entry = keyv as *const function_table_entry;
    let mut _result_: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut _key_: *const libc::c_uchar = (*key).name as *const libc::c_uchar;
    _result_ = _result_
        .wrapping_add(jhash(_key_, (*key).len as libc::c_int) as libc::c_ulong);
    return _result_;
}
unsafe extern "C" fn function_table_entry_hash_2(
    mut keyv: *const libc::c_void,
) -> libc::c_ulong {
    let mut key: *const function_table_entry = keyv as *const function_table_entry;
    let mut _result_: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    return _result_;
}
unsafe extern "C" fn function_table_entry_hash_cmp(
    mut xv: *const libc::c_void,
    mut yv: *const libc::c_void,
) -> libc::c_int {
    let mut x: *const function_table_entry = xv as *const function_table_entry;
    let mut y: *const function_table_entry = yv as *const function_table_entry;
    let mut result: libc::c_int = (*x).len as libc::c_int - (*y).len as libc::c_int;
    if result != 0 {
        return result;
    }
    return if (*x).name == (*y).name {
        0 as libc::c_int
    } else {
        memcmp(
            (*x).name as *const libc::c_void,
            (*y).name as *const libc::c_void,
            (*x).len as libc::c_ulong,
        )
    };
}
static mut function_table: hash_table = hash_table {
    ht_vec: 0 as *const *mut libc::c_void as *mut *mut libc::c_void,
    ht_hash_1: None,
    ht_hash_2: None,
    ht_compare: None,
    ht_size: 0,
    ht_capacity: 0,
    ht_fill: 0,
    ht_empty_slots: 0,
    ht_collisions: 0,
    ht_lookups: 0,
    ht_rehashes: 0,
};
pub unsafe extern "C" fn subst_expand(
    mut o: *mut libc::c_char,
    mut text: *const libc::c_char,
    mut subst: *const libc::c_char,
    mut replace: *const libc::c_char,
    mut slen: size_t,
    mut rlen: size_t,
    mut by_word: libc::c_int,
) -> *mut libc::c_char {
    let mut t: *const libc::c_char = text;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if slen == 0 as libc::c_int as libc::c_ulong && by_word == 0 {
        o = variable_buffer_output(o, t, strlen(t));
        if rlen > 0 as libc::c_int as libc::c_ulong {
            o = variable_buffer_output(o, replace, rlen);
        }
        return o;
    }
    loop {
        if by_word != 0 && slen == 0 as libc::c_int as libc::c_ulong {
            p = end_of_token(next_token(t));
        } else {
            p = strstr(t, subst);
            if p.is_null() {
                o = variable_buffer_output(o, t, strlen(t));
                return o;
            }
        }
        if p > t {
            o = variable_buffer_output(o, t, p.offset_from(t) as libc::c_long as size_t);
        }
        if by_word != 0
            && (p > text
                && !(*stopchar_map
                    .as_mut_ptr()
                    .offset(
                        *p.offset(-(1 as libc::c_int) as isize) as libc::c_uchar as isize,
                    ) as libc::c_int & (0x2 as libc::c_int | 0x4 as libc::c_int)
                    != 0 as libc::c_int)
                || !(*stopchar_map
                    .as_mut_ptr()
                    .offset(*p.offset(slen as isize) as libc::c_uchar as isize)
                    as libc::c_int
                    & (0x2 as libc::c_int | 0x4 as libc::c_int | 0x1 as libc::c_int)
                    != 0 as libc::c_int))
        {
            o = variable_buffer_output(o, subst, slen);
        } else if rlen > 0 as libc::c_int as libc::c_ulong {
            o = variable_buffer_output(o, replace, rlen);
        }
        t = p.offset(slen as isize);
        if !(*t as libc::c_int != '\0' as i32) {
            break;
        }
    }
    return o;
}
pub unsafe extern "C" fn patsubst_expand_pat(
    mut o: *mut libc::c_char,
    mut text: *const libc::c_char,
    mut pattern: *const libc::c_char,
    mut replace: *const libc::c_char,
    mut pattern_percent: *const libc::c_char,
    mut replace_percent: *const libc::c_char,
) -> *mut libc::c_char {
    let mut pattern_prepercent_len: size_t = 0;
    let mut pattern_postpercent_len: size_t = 0;
    let mut replace_prepercent_len: size_t = 0;
    let mut replace_postpercent_len: size_t = 0;
    let mut t: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    let mut doneany: libc::c_int = 0 as libc::c_int;
    if !replace_percent.is_null() {
        replace_prepercent_len = (replace_percent.offset_from(replace) as libc::c_long
            - 1 as libc::c_int as libc::c_long) as size_t;
        replace_postpercent_len = strlen(replace_percent);
    } else {
        replace_prepercent_len = strlen(replace);
        replace_postpercent_len = 0 as libc::c_int as size_t;
    }
    if pattern_percent.is_null() {
        return subst_expand(
            o,
            text,
            pattern,
            replace,
            strlen(pattern),
            strlen(replace),
            1 as libc::c_int,
        );
    }
    pattern_prepercent_len = (pattern_percent.offset_from(pattern) as libc::c_long
        - 1 as libc::c_int as libc::c_long) as size_t;
    pattern_postpercent_len = strlen(pattern_percent);
    loop {
        t = find_next_token(&mut text, &mut len);
        if t.is_null() {
            break;
        }
        let mut fail: libc::c_int = 0 as libc::c_int;
        if len < pattern_prepercent_len.wrapping_add(pattern_postpercent_len) {
            fail = 1 as libc::c_int;
        }
        if fail == 0 && pattern_prepercent_len > 0 as libc::c_int as libc::c_ulong
            && (*t as libc::c_int != *pattern as libc::c_int
                || *t
                    .offset(
                        pattern_prepercent_len
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int
                    != *pattern_percent.offset(-(2 as libc::c_int) as isize)
                        as libc::c_int
                || !(strncmp(
                    t.offset(1 as libc::c_int as isize),
                    pattern.offset(1 as libc::c_int as isize),
                    pattern_prepercent_len
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) == 0 as libc::c_int))
        {
            fail = 1 as libc::c_int;
        }
        if fail == 0 && pattern_postpercent_len > 0 as libc::c_int as libc::c_ulong
            && (*t.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                != *pattern_percent
                    .offset(
                        pattern_postpercent_len
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int
                || *t.offset(len.wrapping_sub(pattern_postpercent_len) as isize)
                    as libc::c_int != *pattern_percent as libc::c_int
                || !(strncmp(
                    &*t.offset(len.wrapping_sub(pattern_postpercent_len) as isize),
                    pattern_percent,
                    pattern_postpercent_len
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) == 0 as libc::c_int))
        {
            fail = 1 as libc::c_int;
        }
        if fail != 0 {
            o = variable_buffer_output(o, t, len);
        } else {
            o = variable_buffer_output(o, replace, replace_prepercent_len);
            if !replace_percent.is_null() {
                o = variable_buffer_output(
                    o,
                    t.offset(pattern_prepercent_len as isize),
                    len
                        .wrapping_sub(
                            pattern_prepercent_len.wrapping_add(pattern_postpercent_len),
                        ),
                );
                o = variable_buffer_output(o, replace_percent, replace_postpercent_len);
            }
        }
        if fail != 0 || replace_prepercent_len > 0 as libc::c_int as libc::c_ulong
            || !replace_percent.is_null()
                && len.wrapping_add(replace_postpercent_len)
                    > 0 as libc::c_int as libc::c_ulong
        {
            o = variable_buffer_output(
                o,
                b" \0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            );
            doneany = 1 as libc::c_int;
        }
    }
    if doneany != 0 {
        o = o.offset(-1);
        o;
    }
    return o;
}
pub unsafe extern "C" fn patsubst_expand(
    mut o: *mut libc::c_char,
    mut text: *const libc::c_char,
    mut pattern: *mut libc::c_char,
    mut replace: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut pattern_percent: *const libc::c_char = find_percent(pattern);
    let mut replace_percent: *const libc::c_char = find_percent(replace);
    if !replace_percent.is_null() {
        replace_percent = replace_percent.offset(1);
        replace_percent;
    }
    if !pattern_percent.is_null() {
        pattern_percent = pattern_percent.offset(1);
        pattern_percent;
    }
    return patsubst_expand_pat(
        o,
        text,
        pattern,
        replace,
        pattern_percent,
        replace_percent,
    );
}
unsafe extern "C" fn lookup_function(
    mut s: *const libc::c_char,
) -> *const function_table_entry {
    let mut function_table_entry_key: function_table_entry = function_table_entry {
        fptr: C2RustUnnamed { func_ptr: None },
        name: 0 as *const libc::c_char,
        len: 0,
        minimum_args: 0,
        maximum_args: 0,
        expand_args_alloc_fn_adds_command: [0; 1],
        c2rust_padding: [0; 4],
    };
    let mut e: *const libc::c_char = s;
    while *stopchar_map.as_mut_ptr().offset(*e as libc::c_uchar as isize) as libc::c_int
        & 0x2000 as libc::c_int != 0 as libc::c_int
    {
        e = e.offset(1);
        e;
    }
    if e == s
        || !(*stopchar_map.as_mut_ptr().offset(*e as libc::c_uchar as isize)
            as libc::c_int
            & (0x1 as libc::c_int | (0x2 as libc::c_int | 0x4 as libc::c_int))
            != 0 as libc::c_int)
    {
        return 0 as *const function_table_entry;
    }
    function_table_entry_key.name = s;
    function_table_entry_key.len = e.offset_from(s) as libc::c_long as libc::c_uchar;
    return hash_find_item(
        &mut function_table,
        &mut function_table_entry_key as *mut function_table_entry as *const libc::c_void,
    ) as *const function_table_entry;
}
pub unsafe extern "C" fn pattern_matches(
    mut pattern: *const libc::c_char,
    mut percent: *const libc::c_char,
    mut str: *const libc::c_char,
) -> libc::c_int {
    let mut sfxlen: size_t = 0;
    let mut strlength: size_t = 0;
    if percent.is_null() {
        let mut len: size_t = (strlen(pattern))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        let mut fresh0 = ::std::vec::from_elem(0, len as usize);
        let mut new_chars: *mut libc::c_char = fresh0.leak().as_mut_ptr()
            as *mut libc::c_char;
        memcpy(new_chars as *mut libc::c_void, pattern as *const libc::c_void, len);
        percent = find_percent(new_chars);
        if percent.is_null() {
            return (new_chars == str as *mut libc::c_char
                || *new_chars as libc::c_int == *str as libc::c_int
                    && (*new_chars as libc::c_int == '\0' as i32
                        || strcmp(
                            new_chars.offset(1 as libc::c_int as isize),
                            str.offset(1 as libc::c_int as isize),
                        ) == 0)) as libc::c_int;
        }
        pattern = new_chars;
    }
    sfxlen = strlen(percent.offset(1 as libc::c_int as isize));
    strlength = strlen(str);
    if strlength
        < (percent.offset_from(pattern) as libc::c_long as libc::c_ulong)
            .wrapping_add(sfxlen)
        || !(strncmp(
            pattern,
            str,
            percent.offset_from(pattern) as libc::c_long as libc::c_ulong,
        ) == 0 as libc::c_int)
    {
        return 0 as libc::c_int;
    }
    return (strcmp(
        percent.offset(1 as libc::c_int as isize),
        str.offset(strlength.wrapping_sub(sfxlen) as isize),
    ) == 0) as libc::c_int;
}
unsafe extern "C" fn find_next_argument(
    mut startparen: libc::c_char,
    mut endparen: libc::c_char,
    mut ptr: *const libc::c_char,
    mut end: *const libc::c_char,
) -> *mut libc::c_char {
    let mut count: libc::c_int = 0 as libc::c_int;
    while ptr < end {
        if *stopchar_map.as_mut_ptr().offset(*ptr as libc::c_uchar as isize)
            as libc::c_int & (0x80 as libc::c_int | 0x400 as libc::c_int)
            != 0 as libc::c_int
        {
            if *ptr as libc::c_int == startparen as libc::c_int {
                count += 1;
                count;
            } else if *ptr as libc::c_int == endparen as libc::c_int {
                count -= 1;
                count;
                if count < 0 as libc::c_int {
                    return 0 as *mut libc::c_char;
                }
            } else if *ptr as libc::c_int == ',' as i32 && count == 0 {
                return ptr as *mut libc::c_char
            }
        }
        ptr = ptr.offset(1);
        ptr;
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn string_glob(mut line: *mut libc::c_char) -> *mut libc::c_char {
    static mut result: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut length: size_t = 0;
    let mut chain: *mut nameseq = 0 as *mut nameseq;
    let mut idx: size_t = 0;
    chain = parse_file_seq(
        &mut line,
        ::std::mem::size_of::<nameseq>() as libc::c_ulong,
        0x1 as libc::c_int,
        0 as *const libc::c_char,
        0x1 as libc::c_int | 0x10 as libc::c_int | 0x8 as libc::c_int,
    ) as *mut nameseq;
    if result.is_null() {
        length = 100 as libc::c_int as size_t;
        result = xmalloc(100 as libc::c_int as size_t) as *mut libc::c_char;
    }
    idx = 0 as libc::c_int as size_t;
    while !chain.is_null() {
        let mut next: *mut nameseq = (*chain).next;
        let mut len: size_t = strlen((*chain).name);
        if idx.wrapping_add(len).wrapping_add(1 as libc::c_int as libc::c_ulong) > length
        {
            length = (length as libc::c_ulong)
                .wrapping_add(
                    len
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong),
                ) as size_t as size_t;
            result = xrealloc(result as *mut libc::c_void, length) as *mut libc::c_char;
        }
        memcpy(
            &mut *result.offset(idx as isize) as *mut libc::c_char as *mut libc::c_void,
            (*chain).name as *const libc::c_void,
            len,
        );
        idx = (idx as libc::c_ulong).wrapping_add(len) as size_t as size_t;
        let fresh1 = idx;
        idx = idx.wrapping_add(1);
        *result.offset(fresh1 as isize) = ' ' as i32 as libc::c_char;
        free((*chain).name as *mut libc::c_char as *mut libc::c_void);
        free(chain as *mut libc::c_void);
        chain = next;
    }
    if idx == 0 as libc::c_int as libc::c_ulong {
        *result.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    } else {
        *result
            .offset(
                idx.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
    }
    return result;
}
unsafe extern "C" fn func_patsubst(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    o = patsubst_expand(
        o,
        *argv.offset(2 as libc::c_int as isize),
        *argv.offset(0 as libc::c_int as isize),
        *argv.offset(1 as libc::c_int as isize),
    );
    return o;
}
unsafe extern "C" fn func_join(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut doneany: libc::c_int = 0 as libc::c_int;
    let mut tp: *const libc::c_char = 0 as *const libc::c_char;
    let mut pp: *const libc::c_char = 0 as *const libc::c_char;
    let mut list1_iterator: *const libc::c_char = *argv
        .offset(0 as libc::c_int as isize);
    let mut list2_iterator: *const libc::c_char = *argv
        .offset(1 as libc::c_int as isize);
    loop {
        let mut len1: size_t = 0;
        let mut len2: size_t = 0;
        tp = find_next_token(&mut list1_iterator, &mut len1);
        if !tp.is_null() {
            o = variable_buffer_output(o, tp, len1);
        }
        pp = find_next_token(&mut list2_iterator, &mut len2);
        if !pp.is_null() {
            o = variable_buffer_output(o, pp, len2);
        }
        if !tp.is_null() || !pp.is_null() {
            o = variable_buffer_output(
                o,
                b" \0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            );
            doneany = 1 as libc::c_int;
        }
        if !(!tp.is_null() || !pp.is_null()) {
            break;
        }
    }
    if doneany != 0 {
        o = o.offset(-1);
        o;
    }
    return o;
}
unsafe extern "C" fn func_origin(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut v: *mut variable = lookup_variable(
        *argv.offset(0 as libc::c_int as isize),
        strlen(*argv.offset(0 as libc::c_int as isize)),
    );
    if v.is_null() {
        o = variable_buffer_output(
            o,
            b"undefined\0" as *const u8 as *const libc::c_char,
            9 as libc::c_int as size_t,
        );
    } else {
        match (*v).origin() as libc::c_int {
            7 => {
                abort();
            }
            0 => {
                o = variable_buffer_output(
                    o,
                    b"default\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as size_t,
                );
            }
            1 => {
                o = variable_buffer_output(
                    o,
                    b"environment\0" as *const u8 as *const libc::c_char,
                    11 as libc::c_int as size_t,
                );
            }
            2 => {
                o = variable_buffer_output(
                    o,
                    b"file\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as size_t,
                );
            }
            3 => {
                o = variable_buffer_output(
                    o,
                    b"environment override\0" as *const u8 as *const libc::c_char,
                    20 as libc::c_int as size_t,
                );
            }
            4 => {
                o = variable_buffer_output(
                    o,
                    b"command line\0" as *const u8 as *const libc::c_char,
                    12 as libc::c_int as size_t,
                );
            }
            5 => {
                o = variable_buffer_output(
                    o,
                    b"override\0" as *const u8 as *const libc::c_char,
                    8 as libc::c_int as size_t,
                );
            }
            6 => {
                o = variable_buffer_output(
                    o,
                    b"automatic\0" as *const u8 as *const libc::c_char,
                    9 as libc::c_int as size_t,
                );
            }
            _ => {}
        }
    }
    return o;
}
unsafe extern "C" fn func_flavor(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut v: *mut variable = lookup_variable(
        *argv.offset(0 as libc::c_int as isize),
        strlen(*argv.offset(0 as libc::c_int as isize)),
    );
    if v.is_null() {
        o = variable_buffer_output(
            o,
            b"undefined\0" as *const u8 as *const libc::c_char,
            9 as libc::c_int as size_t,
        );
    } else if (*v).recursive() != 0 {
        o = variable_buffer_output(
            o,
            b"recursive\0" as *const u8 as *const libc::c_char,
            9 as libc::c_int as size_t,
        );
    } else {
        o = variable_buffer_output(
            o,
            b"simple\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as size_t,
        );
    }
    return o;
}
unsafe extern "C" fn func_notdir_suffix(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut list_iterator: *const libc::c_char = *argv.offset(0 as libc::c_int as isize);
    let mut p2: *const libc::c_char = 0 as *const libc::c_char;
    let mut doneany: libc::c_int = 0 as libc::c_int;
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut is_suffix: libc::c_int = (*funcname.offset(0 as libc::c_int as isize)
        as libc::c_int == 's' as i32) as libc::c_int;
    let mut is_notdir: libc::c_int = (is_suffix == 0) as libc::c_int;
    let mut stop: libc::c_int = 0x8000 as libc::c_int
        | (if is_suffix != 0 { 0x200 as libc::c_int } else { 0 as libc::c_int });
    loop {
        p2 = find_next_token(&mut list_iterator, &mut len);
        if p2.is_null() {
            break;
        }
        let mut p: *const libc::c_char = p2
            .offset(len as isize)
            .offset(-(1 as libc::c_int as isize));
        while p >= p2
            && !(*stopchar_map.as_mut_ptr().offset(*p as libc::c_uchar as isize)
                as libc::c_int & stop != 0 as libc::c_int)
        {
            p = p.offset(-1);
            p;
        }
        if p >= p2 {
            if is_notdir != 0 {
                p = p.offset(1);
                p;
            } else if *p as libc::c_int != '.' as i32 {
                continue;
            }
            o = variable_buffer_output(
                o,
                p,
                len.wrapping_sub(p.offset_from(p2) as libc::c_long as libc::c_ulong),
            );
        } else if is_notdir != 0 {
            o = variable_buffer_output(o, p2, len);
        }
        if is_notdir != 0 || p >= p2 {
            o = variable_buffer_output(
                o,
                b" \0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            );
            doneany = 1 as libc::c_int;
        }
    }
    if doneany != 0 {
        o = o.offset(-1);
        o;
    }
    return o;
}
unsafe extern "C" fn func_basename_dir(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut p3: *const libc::c_char = *argv.offset(0 as libc::c_int as isize);
    let mut p2: *const libc::c_char = 0 as *const libc::c_char;
    let mut doneany: libc::c_int = 0 as libc::c_int;
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut is_basename: libc::c_int = (*funcname.offset(0 as libc::c_int as isize)
        as libc::c_int == 'b' as i32) as libc::c_int;
    let mut is_dir: libc::c_int = (is_basename == 0) as libc::c_int;
    let mut stop: libc::c_int = 0x8000 as libc::c_int
        | (if is_basename != 0 { 0x200 as libc::c_int } else { 0 as libc::c_int })
        | 0x1 as libc::c_int;
    loop {
        p2 = find_next_token(&mut p3, &mut len);
        if p2.is_null() {
            break;
        }
        let mut p: *const libc::c_char = p2
            .offset(len as isize)
            .offset(-(1 as libc::c_int as isize));
        while p >= p2
            && !(*stopchar_map.as_mut_ptr().offset(*p as libc::c_uchar as isize)
                as libc::c_int & stop != 0 as libc::c_int)
        {
            p = p.offset(-1);
            p;
        }
        if p >= p2 && is_dir != 0 {
            p = p.offset(1);
            o = variable_buffer_output(
                o,
                p2,
                p.offset_from(p2) as libc::c_long as size_t,
            );
        } else if p >= p2 && *p as libc::c_int == '.' as i32 {
            o = variable_buffer_output(
                o,
                p2,
                p.offset_from(p2) as libc::c_long as size_t,
            );
        } else if is_dir != 0 {
            o = variable_buffer_output(
                o,
                b"./\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as size_t,
            );
        } else {
            o = variable_buffer_output(o, p2, len);
        }
        o = variable_buffer_output(
            o,
            b" \0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        doneany = 1 as libc::c_int;
    }
    if doneany != 0 {
        o = o.offset(-1);
        o;
    }
    return o;
}
unsafe extern "C" fn func_addsuffix_addprefix(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut fixlen: size_t = strlen(*argv.offset(0 as libc::c_int as isize));
    let mut list_iterator: *const libc::c_char = *argv.offset(1 as libc::c_int as isize);
    let mut is_addprefix: libc::c_int = (*funcname.offset(3 as libc::c_int as isize)
        as libc::c_int == 'p' as i32) as libc::c_int;
    let mut is_addsuffix: libc::c_int = (is_addprefix == 0) as libc::c_int;
    let mut doneany: libc::c_int = 0 as libc::c_int;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    loop {
        p = find_next_token(&mut list_iterator, &mut len);
        if p.is_null() {
            break;
        }
        if is_addprefix != 0 {
            o = variable_buffer_output(
                o,
                *argv.offset(0 as libc::c_int as isize),
                fixlen,
            );
        }
        o = variable_buffer_output(o, p, len);
        if is_addsuffix != 0 {
            o = variable_buffer_output(
                o,
                *argv.offset(0 as libc::c_int as isize),
                fixlen,
            );
        }
        o = variable_buffer_output(
            o,
            b" \0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        doneany = 1 as libc::c_int;
    }
    if doneany != 0 {
        o = o.offset(-1);
        o;
    }
    return o;
}
unsafe extern "C" fn func_subst(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    o = subst_expand(
        o,
        *argv.offset(2 as libc::c_int as isize),
        *argv.offset(0 as libc::c_int as isize),
        *argv.offset(1 as libc::c_int as isize),
        strlen(*argv.offset(0 as libc::c_int as isize)),
        strlen(*argv.offset(1 as libc::c_int as isize)),
        0 as libc::c_int,
    );
    return o;
}
unsafe extern "C" fn func_firstword(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut i: size_t = 0;
    let mut words: *const libc::c_char = *argv.offset(0 as libc::c_int as isize);
    let mut p: *const libc::c_char = find_next_token(&mut words, &mut i);
    if !p.is_null() {
        o = variable_buffer_output(o, p, i);
    }
    return o;
}
unsafe extern "C" fn func_lastword(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut i: size_t = 0;
    let mut words: *const libc::c_char = *argv.offset(0 as libc::c_int as isize);
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut t: *const libc::c_char = 0 as *const libc::c_char;
    loop {
        t = find_next_token(&mut words, &mut i);
        if t.is_null() {
            break;
        }
        p = t;
    }
    if !p.is_null() {
        o = variable_buffer_output(o, p, i);
    }
    return o;
}
unsafe extern "C" fn func_words(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut word_iterator: *const libc::c_char = *argv.offset(0 as libc::c_int as isize);
    let mut buf: [libc::c_char; 22] = [0; 22];
    while !(find_next_token(&mut word_iterator, 0 as *mut size_t)).is_null() {
        i = i.wrapping_add(1);
        i;
    }
    sprintf(buf.as_mut_ptr(), b"%u\0" as *const u8 as *const libc::c_char, i);
    o = variable_buffer_output(o, buf.as_mut_ptr(), strlen(buf.as_mut_ptr()));
    return o;
}
pub unsafe extern "C" fn strip_whitespace(
    mut begpp: *mut *const libc::c_char,
    mut endpp: *mut *const libc::c_char,
) -> *mut libc::c_char {
    while *begpp <= *endpp
        && *stopchar_map.as_mut_ptr().offset(**begpp as libc::c_uchar as isize)
            as libc::c_int & (0x2 as libc::c_int | 0x4 as libc::c_int)
            != 0 as libc::c_int
    {
        *begpp = (*begpp).offset(1);
        *begpp;
    }
    while *endpp >= *begpp
        && *stopchar_map.as_mut_ptr().offset(**endpp as libc::c_uchar as isize)
            as libc::c_int & (0x2 as libc::c_int | 0x4 as libc::c_int)
            != 0 as libc::c_int
    {
        *endpp = (*endpp).offset(-1);
        *endpp;
    }
    return *begpp as *mut libc::c_char;
}
unsafe extern "C" fn parse_numeric(
    mut s: *const libc::c_char,
    mut msg: *const libc::c_char,
) -> libc::c_longlong {
    let mut beg: *const libc::c_char = s;
    let mut end: *const libc::c_char = s
        .offset(strlen(s) as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut num: libc::c_longlong = 0;
    strip_whitespace(&mut beg, &mut end);
    if beg > end {
        fatal(
            *expanding_var,
            strlen(msg),
            dcgettext(
                0 as *const libc::c_char,
                b"%s: empty value\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            msg,
        );
    }
    *__errno_location() = 0 as libc::c_int;
    num = strtoll(beg, &mut endp, 10 as libc::c_int);
    if *__errno_location() == 34 as libc::c_int {
        fatal(
            *expanding_var,
            (strlen(msg)).wrapping_add(strlen(s)),
            dcgettext(
                0 as *const libc::c_char,
                b"%s: '%s' out of range\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            msg,
            s,
        );
    } else if endp == beg as *mut libc::c_char || endp <= end as *mut libc::c_char {
        fatal(
            *expanding_var,
            (strlen(msg)).wrapping_add(strlen(s)),
            b"%s: '%s'\0" as *const u8 as *const libc::c_char,
            msg,
            s,
        );
    }
    return num;
}
unsafe extern "C" fn func_word(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut end_p: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_longlong = 0;
    i = parse_numeric(
        *argv.offset(0 as libc::c_int as isize),
        dcgettext(
            0 as *const libc::c_char,
            b"invalid first argument to 'word' function\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    if i < 1 as libc::c_int as libc::c_longlong {
        fatal(
            *expanding_var,
            0 as libc::c_int as size_t,
            dcgettext(
                0 as *const libc::c_char,
                b"first argument to 'word' function must be greater than 0\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    end_p = *argv.offset(1 as libc::c_int as isize);
    loop {
        p = find_next_token(&mut end_p, 0 as *mut size_t);
        if p.is_null() {
            break;
        }
        i -= 1;
        if i == 0 as libc::c_int as libc::c_longlong {
            break;
        }
    }
    if i == 0 as libc::c_int as libc::c_longlong {
        o = variable_buffer_output(o, p, end_p.offset_from(p) as libc::c_long as size_t);
    }
    return o;
}
unsafe extern "C" fn func_wordlist(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut buf: [libc::c_char; 23] = [0; 23];
    let mut start: libc::c_longlong = 0;
    let mut stop: libc::c_longlong = 0;
    let mut count: libc::c_longlong = 0;
    let mut badfirst: *const libc::c_char = dcgettext(
        0 as *const libc::c_char,
        b"invalid first argument to 'wordlist' function\0" as *const u8
            as *const libc::c_char,
        5 as libc::c_int,
    );
    let mut badsecond: *const libc::c_char = dcgettext(
        0 as *const libc::c_char,
        b"invalid second argument to 'wordlist' function\0" as *const u8
            as *const libc::c_char,
        5 as libc::c_int,
    );
    start = parse_numeric(*argv.offset(0 as libc::c_int as isize), badfirst);
    if start < 1 as libc::c_int as libc::c_longlong {
        fatal(
            *expanding_var,
            (strlen(badfirst)).wrapping_add(strlen(make_lltoa(start, buf.as_mut_ptr()))),
            b"%s: '%s'\0" as *const u8 as *const libc::c_char,
            badfirst,
            make_lltoa(start, buf.as_mut_ptr()),
        );
    }
    stop = parse_numeric(*argv.offset(1 as libc::c_int as isize), badsecond);
    if stop < 0 as libc::c_int as libc::c_longlong {
        fatal(
            *expanding_var,
            (strlen(badsecond)).wrapping_add(strlen(make_lltoa(stop, buf.as_mut_ptr()))),
            b"%s: '%s'\0" as *const u8 as *const libc::c_char,
            badsecond,
            make_lltoa(stop, buf.as_mut_ptr()),
        );
    }
    count = stop - start + 1 as libc::c_int as libc::c_longlong;
    if count > 0 as libc::c_int as libc::c_longlong {
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        let mut end_p: *const libc::c_char = *argv.offset(2 as libc::c_int as isize);
        loop {
            p = find_next_token(&mut end_p, 0 as *mut size_t);
            if !(!p.is_null()
                && {
                    start -= 1;
                    start != 0
                })
            {
                break;
            }
        }
        if !p.is_null() {
            loop {
                count -= 1;
                if !(count != 0
                    && !(find_next_token(&mut end_p, 0 as *mut size_t)).is_null())
                {
                    break;
                }
            }
            o = variable_buffer_output(
                o,
                p,
                end_p.offset_from(p) as libc::c_long as size_t,
            );
        }
    }
    return o;
}
unsafe extern "C" fn func_findstring(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    if !(strstr(
        *argv.offset(1 as libc::c_int as isize),
        *argv.offset(0 as libc::c_int as isize),
    ))
        .is_null()
    {
        o = variable_buffer_output(
            o,
            *argv.offset(0 as libc::c_int as isize),
            strlen(*argv.offset(0 as libc::c_int as isize)),
        );
    }
    return o;
}
unsafe extern "C" fn func_foreach(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut varname: *mut libc::c_char = expand_argument(
        *argv.offset(0 as libc::c_int as isize),
        0 as *const libc::c_char,
    );
    let mut list: *mut libc::c_char = expand_argument(
        *argv.offset(1 as libc::c_int as isize),
        0 as *const libc::c_char,
    );
    let mut body: *const libc::c_char = *argv.offset(2 as libc::c_int as isize);
    let mut doneany: libc::c_int = 0 as libc::c_int;
    let mut list_iterator: *const libc::c_char = list;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    let mut var: *mut variable = 0 as *mut variable;
    let mut vp: *mut libc::c_char = next_token(varname);
    *(end_of_token(vp)).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    push_new_variable_scope();
    var = define_variable_in_set(
        vp,
        strlen(vp),
        b"\0" as *const u8 as *const libc::c_char,
        o_automatic,
        0 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    loop {
        p = find_next_token(&mut list_iterator, &mut len);
        if p.is_null() {
            break;
        }
        let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
        free((*var).value as *mut libc::c_void);
        (*var).value = xstrndup(p, len);
        result = allocated_variable_expand_for_file(body, 0 as *mut file);
        o = variable_buffer_output(o, result, strlen(result));
        o = variable_buffer_output(
            o,
            b" \0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        doneany = 1 as libc::c_int;
        free(result as *mut libc::c_void);
    }
    if doneany != 0 {
        o = o.offset(-1);
        o;
    }
    pop_variable_scope();
    free(varname as *mut libc::c_void);
    free(list as *mut libc::c_void);
    return o;
}
unsafe extern "C" fn func_let(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut varnames: *mut libc::c_char = expand_argument(
        *argv.offset(0 as libc::c_int as isize),
        0 as *const libc::c_char,
    );
    let mut list: *mut libc::c_char = expand_argument(
        *argv.offset(1 as libc::c_int as isize),
        0 as *const libc::c_char,
    );
    let mut body: *const libc::c_char = *argv.offset(2 as libc::c_int as isize);
    let mut vp: *const libc::c_char = 0 as *const libc::c_char;
    let mut vp_next: *const libc::c_char = varnames;
    let mut list_iterator: *const libc::c_char = list;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut vlen: size_t = 0;
    push_new_variable_scope();
    vp = find_next_token(&mut vp_next, &mut vlen);
    while *stopchar_map.as_mut_ptr().offset(*vp_next as libc::c_uchar as isize)
        as libc::c_int & (0x2 as libc::c_int | 0x4 as libc::c_int) != 0 as libc::c_int
    {
        vp_next = vp_next.offset(1);
        vp_next;
    }
    while *vp_next as libc::c_int != '\0' as i32 {
        p = find_next_token(&mut list_iterator, &mut len);
        if *list_iterator as libc::c_int != '\0' as i32 {
            list_iterator = list_iterator.offset(1);
            list_iterator;
            *p.offset(len as isize) = '\0' as i32 as libc::c_char;
        }
        define_variable_in_set(
            vp,
            vlen,
            if !p.is_null() {
                p as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            o_automatic,
            0 as libc::c_int,
            (*current_variable_set_list).set,
            0 as *mut floc,
        );
        vp = find_next_token(&mut vp_next, &mut vlen);
        while *stopchar_map.as_mut_ptr().offset(*vp_next as libc::c_uchar as isize)
            as libc::c_int & (0x2 as libc::c_int | 0x4 as libc::c_int)
            != 0 as libc::c_int
        {
            vp_next = vp_next.offset(1);
            vp_next;
        }
    }
    if !vp.is_null() {
        define_variable_in_set(
            vp,
            vlen,
            next_token(list_iterator),
            o_automatic,
            0 as libc::c_int,
            (*current_variable_set_list).set,
            0 as *mut floc,
        );
    }
    o = variable_expand_string(o, body, 18446744073709551615 as libc::c_ulong);
    pop_variable_scope();
    free(varnames as *mut libc::c_void);
    free(list as *mut libc::c_void);
    return o.offset(strlen(o) as isize);
}
unsafe extern "C" fn a_word_hash_1(mut key: *const libc::c_void) -> libc::c_ulong {
    let mut _result_: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut _key_: *const libc::c_uchar = (*(key as *const a_word)).str_0
        as *const libc::c_uchar;
    _result_ = _result_.wrapping_add(jhash_string(_key_) as libc::c_ulong);
    return _result_;
}
unsafe extern "C" fn a_word_hash_2(mut key: *const libc::c_void) -> libc::c_ulong {
    let mut _result_: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    return _result_;
}
unsafe extern "C" fn a_word_hash_cmp(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> libc::c_int {
    let mut ax: *const a_word = x as *const a_word;
    let mut ay: *const a_word = y as *const a_word;
    if (*ax).length != (*ay).length {
        return if (*ax).length > (*ay).length {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    }
    return if (*ax).str_0 == (*ay).str_0 {
        0 as libc::c_int
    } else {
        memcmp(
            (*ax).str_0 as *const libc::c_void,
            (*ay).str_0 as *const libc::c_void,
            (*ax).length,
        )
    };
}
unsafe extern "C" fn func_filter_filterout(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut words: *mut a_word = 0 as *mut a_word;
    let mut word_end: *mut a_word = 0 as *mut a_word;
    let mut wp: *mut a_word = 0 as *mut a_word;
    let mut patterns: *mut a_pattern = 0 as *mut a_pattern;
    let mut pat_end: *mut a_pattern = 0 as *mut a_pattern;
    let mut pp: *mut a_pattern = 0 as *mut a_pattern;
    let mut pat_count: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut word_count: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut a_word_table: hash_table = hash_table {
        ht_vec: 0 as *const *mut libc::c_void as *mut *mut libc::c_void,
        ht_hash_1: None,
        ht_hash_2: None,
        ht_compare: None,
        ht_size: 0,
        ht_capacity: 0,
        ht_fill: 0,
        ht_empty_slots: 0,
        ht_collisions: 0,
        ht_lookups: 0,
        ht_rehashes: 0,
    };
    let mut is_filter: libc::c_int = (*funcname
        .offset(
            (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) as libc::c_int == '\0' as i32) as libc::c_int;
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut literals: libc::c_int = 0 as libc::c_int;
    let mut hashing: libc::c_int = 0 as libc::c_int;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut doneany: libc::c_int = 0 as libc::c_int;
    cp = *argv.offset(1 as libc::c_int as isize);
    loop {
        p = find_next_token(&mut cp, 0 as *mut size_t);
        if p.is_null() {
            break;
        }
        word_count = word_count.wrapping_add(1);
        word_count;
    }
    if word_count == 0 {
        return o;
    }
    words = xcalloc(
        word_count.wrapping_mul(::std::mem::size_of::<a_word>() as libc::c_ulong),
    ) as *mut a_word;
    word_end = words.offset(word_count as isize);
    cp = *argv.offset(0 as libc::c_int as isize);
    loop {
        p = find_next_token(&mut cp, 0 as *mut size_t);
        if p.is_null() {
            break;
        }
        pat_count = pat_count.wrapping_add(1);
        pat_count;
    }
    patterns = xcalloc(
        pat_count.wrapping_mul(::std::mem::size_of::<a_pattern>() as libc::c_ulong),
    ) as *mut a_pattern;
    pat_end = patterns.offset(pat_count as isize);
    cp = *argv.offset(0 as libc::c_int as isize);
    pp = patterns;
    loop {
        p = find_next_token(&mut cp, &mut len);
        if p.is_null() {
            break;
        }
        if *cp as libc::c_int != '\0' as i32 {
            cp = cp.offset(1);
            cp;
        }
        *p.offset(len as isize) = '\0' as i32 as libc::c_char;
        (*pp).str_0 = p;
        (*pp).percent = find_percent(p);
        if ((*pp).percent).is_null() {
            literals += 1;
            literals;
        }
        (*pp).length = strlen((*pp).str_0);
        pp = pp.offset(1);
        pp;
    }
    cp = *argv.offset(1 as libc::c_int as isize);
    wp = words;
    loop {
        p = find_next_token(&mut cp, &mut len);
        if p.is_null() {
            break;
        }
        if *cp as libc::c_int != '\0' as i32 {
            cp = cp.offset(1);
            cp;
        }
        *p.offset(len as isize) = '\0' as i32 as libc::c_char;
        (*wp).str_0 = p;
        (*wp).length = len;
        wp = wp.offset(1);
        wp;
    }
    hashing = (literals > 1 as libc::c_int
        && (literals as libc::c_ulong).wrapping_mul(word_count)
            >= 10 as libc::c_int as libc::c_ulong) as libc::c_int;
    if hashing != 0 {
        hash_init(
            &mut a_word_table,
            word_count,
            Some(
                a_word_hash_1
                    as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
            ),
            Some(
                a_word_hash_2
                    as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
            ),
            Some(
                a_word_hash_cmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        wp = words;
        while wp < word_end {
            let mut owp: *mut a_word = hash_insert(
                &mut a_word_table,
                wp as *const libc::c_void,
            ) as *mut a_word;
            if !owp.is_null() {
                (*wp).chain = owp;
            }
            wp = wp.offset(1);
            wp;
        }
    }
    pp = patterns;
    while pp < pat_end {
        if !((*pp).percent).is_null() {
            wp = words;
            while wp < word_end {
                (*wp).matched
                    |= pattern_matches((*pp).str_0, (*pp).percent, (*wp).str_0);
                wp = wp.offset(1);
                wp;
            }
        } else if hashing != 0 {
            let mut a_word_key: a_word = a_word {
                chain: 0 as *mut a_word,
                str_0: 0 as *mut libc::c_char,
                length: 0,
                matched: 0,
            };
            a_word_key.str_0 = (*pp).str_0;
            a_word_key.length = (*pp).length;
            wp = hash_find_item(
                &mut a_word_table,
                &mut a_word_key as *mut a_word as *const libc::c_void,
            ) as *mut a_word;
            while !wp.is_null() {
                (*wp).matched |= 1 as libc::c_int;
                wp = (*wp).chain;
            }
        } else {
            wp = words;
            while wp < word_end {
                (*wp).matched
                    |= ((*wp).length == (*pp).length
                        && memcmp(
                            (*pp).str_0 as *const libc::c_void,
                            (*wp).str_0 as *const libc::c_void,
                            (*wp).length,
                        ) == 0 as libc::c_int) as libc::c_int;
                wp = wp.offset(1);
                wp;
            }
        }
        pp = pp.offset(1);
        pp;
    }
    wp = words;
    while wp < word_end {
        if if is_filter != 0 {
            (*wp).matched
        } else {
            ((*wp).matched == 0) as libc::c_int
        } != 0
        {
            o = variable_buffer_output(o, (*wp).str_0, strlen((*wp).str_0));
            o = variable_buffer_output(
                o,
                b" \0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            );
            doneany = 1 as libc::c_int;
        }
        wp = wp.offset(1);
        wp;
    }
    if doneany != 0 {
        o = o.offset(-1);
        o;
    }
    if hashing != 0 {
        hash_free(&mut a_word_table, 0 as libc::c_int);
    }
    free(patterns as *mut libc::c_void);
    free(words as *mut libc::c_void);
    return o;
}
unsafe extern "C" fn func_strip(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut p: *const libc::c_char = *argv.offset(0 as libc::c_int as isize);
    let mut doneany: libc::c_int = 0 as libc::c_int;
    while *p as libc::c_int != '\0' as i32 {
        let mut i: libc::c_int = 0 as libc::c_int;
        let mut word_start: *const libc::c_char = 0 as *const libc::c_char;
        while *stopchar_map.as_mut_ptr().offset(*p as libc::c_uchar as isize)
            as libc::c_int & (0x2 as libc::c_int | 0x4 as libc::c_int)
            != 0 as libc::c_int
        {
            p = p.offset(1);
            p;
        }
        word_start = p;
        i = 0 as libc::c_int;
        while *p as libc::c_int != '\0' as i32
            && !(*stopchar_map.as_mut_ptr().offset(*p as libc::c_uchar as isize)
                as libc::c_int & (0x2 as libc::c_int | 0x4 as libc::c_int)
                != 0 as libc::c_int)
        {
            p = p.offset(1);
            p;
            i += 1;
            i;
        }
        if i == 0 {
            break;
        }
        o = variable_buffer_output(o, word_start, i as size_t);
        o = variable_buffer_output(
            o,
            b" \0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        doneany = 1 as libc::c_int;
    }
    if doneany != 0 {
        o = o.offset(-1);
        o;
    }
    return o;
}
unsafe extern "C" fn func_error(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    match *funcname as libc::c_int {
        101 => {
            fatal(
                reading_file,
                strlen(*argv.offset(0 as libc::c_int as isize)),
                b"%s\0" as *const u8 as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize),
            );
        }
        119 => {
            error(
                reading_file,
                strlen(*argv.offset(0 as libc::c_int as isize)),
                b"%s\0" as *const u8 as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize),
            );
        }
        105 => {
            let mut len: size_t = strlen(*argv.offset(0 as libc::c_int as isize));
            let mut fresh2 = ::std::vec::from_elem(
                0,
                len.wrapping_add(2 as libc::c_int as libc::c_ulong) as usize,
            );
            let mut msg: *mut libc::c_char = fresh2.leak().as_mut_ptr()
                as *mut libc::c_char;
            memcpy(
                msg as *mut libc::c_void,
                *argv.offset(0 as libc::c_int as isize) as *const libc::c_void,
                len,
            );
            *msg.offset(len as isize) = '\n' as i32 as libc::c_char;
            *msg
                .offset(
                    len.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = '\0' as i32 as libc::c_char;
            outputs(0 as libc::c_int, msg);
        }
        _ => {
            fatal(
                *expanding_var,
                strlen(funcname),
                b"Internal error: func_error: '%s'\0" as *const u8
                    as *const libc::c_char,
                funcname,
            );
        }
    }
    return o;
}
unsafe extern "C" fn func_sort(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut t: *const libc::c_char = 0 as *const libc::c_char;
    let mut words: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut wordi: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    t = *argv.offset(0 as libc::c_int as isize);
    wordi = 0 as libc::c_int;
    loop {
        p = find_next_token(&mut t, 0 as *mut size_t);
        if p.is_null() {
            break;
        }
        t = t.offset(1);
        t;
        wordi += 1;
        wordi;
    }
    words = xmalloc(
        ((if wordi == 0 as libc::c_int { 1 as libc::c_int } else { wordi })
            as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    t = *argv.offset(0 as libc::c_int as isize);
    wordi = 0 as libc::c_int;
    loop {
        p = find_next_token(&mut t, &mut len);
        if p.is_null() {
            break;
        }
        t = t.offset(1);
        t;
        *p.offset(len as isize) = '\0' as i32 as libc::c_char;
        let fresh3 = wordi;
        wordi = wordi + 1;
        let ref mut fresh4 = *words.offset(fresh3 as isize);
        *fresh4 = p;
    }
    if wordi != 0 {
        let mut i: libc::c_int = 0;
        qsort(
            words as *mut libc::c_void,
            wordi as size_t,
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            Some(
                alpha_compare
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        i = 0 as libc::c_int;
        while i < wordi {
            len = strlen(*words.offset(i as isize));
            if i == wordi - 1 as libc::c_int
                || strlen(*words.offset((i + 1 as libc::c_int) as isize)) != len
                || memcmp(
                    *words.offset(i as isize) as *const libc::c_void,
                    *words.offset((i + 1 as libc::c_int) as isize)
                        as *const libc::c_void,
                    len,
                ) != 0
            {
                o = variable_buffer_output(o, *words.offset(i as isize), len);
                o = variable_buffer_output(
                    o,
                    b" \0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int as size_t,
                );
            }
            i += 1;
            i;
        }
        o = o.offset(-1);
        o;
    }
    free(words as *mut libc::c_void);
    return o;
}
unsafe extern "C" fn parse_textint(
    mut number: *const libc::c_char,
    mut msg: *const libc::c_char,
    mut sign: *mut libc::c_int,
    mut numstart: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut after_sign: *const libc::c_char = 0 as *const libc::c_char;
    let mut after_number: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = next_token(number);
    let mut negative: libc::c_int = (*p as libc::c_int == '-' as i32) as libc::c_int;
    let mut nonzero: libc::c_int = 0;
    if *p as libc::c_int == '\0' as i32 {
        fatal(
            *expanding_var,
            strlen(msg),
            dcgettext(
                0 as *const libc::c_char,
                b"%s: empty value\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            msg,
        );
    }
    p = p
        .offset(
            (negative != 0 || *p as libc::c_int == '+' as i32) as libc::c_int as isize,
        );
    after_sign = p;
    while *p as libc::c_int == '0' as i32 {
        p = p.offset(1);
        p;
    }
    *numstart = p;
    while (*p as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
        <= 9 as libc::c_int as libc::c_uint
    {
        p = p.offset(1);
        p;
    }
    after_number = p;
    nonzero = (*numstart != after_number) as libc::c_int;
    *sign = if negative != 0 { -nonzero } else { nonzero };
    if after_number == after_sign || *next_token(p) as libc::c_int != '\0' as i32 {
        fatal(
            *expanding_var,
            (strlen(msg)).wrapping_add(strlen(number)),
            b"%s: '%s'\0" as *const u8 as *const libc::c_char,
            msg,
            number,
        );
    }
    return after_number;
}
unsafe extern "C" fn func_intcmp(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut lsign: libc::c_int = 0;
    let mut rsign: libc::c_int = 0;
    let mut lnum: *const libc::c_char = 0 as *const libc::c_char;
    let mut rnum: *const libc::c_char = 0 as *const libc::c_char;
    let mut lhs_str: *mut libc::c_char = expand_argument(
        *argv.offset(0 as libc::c_int as isize),
        0 as *const libc::c_char,
    );
    let mut rhs_str: *mut libc::c_char = expand_argument(
        *argv.offset(1 as libc::c_int as isize),
        0 as *const libc::c_char,
    );
    let mut llim: *const libc::c_char = parse_textint(
        lhs_str,
        dcgettext(
            0 as *const libc::c_char,
            b"non-numeric first argument to 'intcmp' function\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        &mut lsign,
        &mut lnum,
    );
    let mut rlim: *const libc::c_char = parse_textint(
        rhs_str,
        dcgettext(
            0 as *const libc::c_char,
            b"non-numeric second argument to 'intcmp' function\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        &mut rsign,
        &mut rnum,
    );
    let mut llen: ptrdiff_t = llim.offset_from(lnum) as libc::c_long;
    let mut rlen: ptrdiff_t = rlim.offset_from(rnum) as libc::c_long;
    let mut cmp: libc::c_int = lsign - rsign;
    if cmp == 0 as libc::c_int {
        cmp = (llen > rlen) as libc::c_int - (llen < rlen) as libc::c_int;
        if cmp == 0 as libc::c_int {
            cmp = memcmp(
                lnum as *const libc::c_void,
                rnum as *const libc::c_void,
                llen as libc::c_ulong,
            );
        }
    }
    argv = argv.offset(2 as libc::c_int as isize);
    if (*argv).is_null() && cmp == 0 as libc::c_int {
        if lsign < 0 as libc::c_int {
            o = variable_buffer_output(
                o,
                b"-\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            );
        }
        o = variable_buffer_output(
            o,
            lnum.offset(-((lsign == 0) as libc::c_int as isize)),
            (llen + (lsign == 0) as libc::c_int as libc::c_long) as size_t,
        );
    }
    free(lhs_str as *mut libc::c_void);
    free(rhs_str as *mut libc::c_void);
    if !(*argv).is_null() && cmp >= 0 as libc::c_int {
        argv = argv.offset(1);
        argv;
        if cmp > 0 as libc::c_int && !(*argv).is_null()
            && !(*argv.offset(1 as libc::c_int as isize)).is_null()
        {
            argv = argv.offset(1);
            argv;
        }
    }
    if !(*argv).is_null() {
        let mut expansion: *mut libc::c_char = expand_argument(
            *argv,
            0 as *const libc::c_char,
        );
        o = variable_buffer_output(o, expansion, strlen(expansion));
        free(expansion as *mut libc::c_void);
    }
    return o;
}
unsafe extern "C" fn func_if(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut begp: *const libc::c_char = *argv.offset(0 as libc::c_int as isize);
    let mut endp: *const libc::c_char = begp
        .offset(strlen(*argv.offset(0 as libc::c_int as isize)) as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut result: libc::c_int = 0 as libc::c_int;
    strip_whitespace(&mut begp, &mut endp);
    if begp <= endp {
        let mut expansion: *mut libc::c_char = expand_argument(
            begp,
            endp.offset(1 as libc::c_int as isize),
        );
        result = (*expansion.offset(0 as libc::c_int as isize) as libc::c_int
            != '\0' as i32) as libc::c_int;
        free(expansion as *mut libc::c_void);
    }
    argv = argv.offset((1 as libc::c_int + (result == 0) as libc::c_int) as isize);
    if !(*argv).is_null() {
        let mut expansion_0: *mut libc::c_char = expand_argument(
            *argv,
            0 as *const libc::c_char,
        );
        o = variable_buffer_output(o, expansion_0, strlen(expansion_0));
        free(expansion_0 as *mut libc::c_void);
    }
    return o;
}
unsafe extern "C" fn func_or(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    while !(*argv).is_null() {
        let mut begp: *const libc::c_char = *argv;
        let mut endp: *const libc::c_char = begp
            .offset(strlen(*argv) as isize)
            .offset(-(1 as libc::c_int as isize));
        let mut expansion: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut result: size_t = 0 as libc::c_int as size_t;
        strip_whitespace(&mut begp, &mut endp);
        if !(begp > endp) {
            expansion = expand_argument(begp, endp.offset(1 as libc::c_int as isize));
            result = strlen(expansion);
            if result == 0 {
                free(expansion as *mut libc::c_void);
            } else {
                o = variable_buffer_output(o, expansion, result);
                free(expansion as *mut libc::c_void);
                break;
            }
        }
        argv = argv.offset(1);
        argv;
    }
    return o;
}
unsafe extern "C" fn func_and(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut expansion: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        let mut begp: *const libc::c_char = *argv;
        let mut endp: *const libc::c_char = begp
            .offset(strlen(*argv) as isize)
            .offset(-(1 as libc::c_int as isize));
        let mut result: size_t = 0;
        strip_whitespace(&mut begp, &mut endp);
        if begp > endp {
            return o;
        }
        expansion = expand_argument(begp, endp.offset(1 as libc::c_int as isize));
        result = strlen(expansion);
        if result == 0 {
            break;
        }
        argv = argv.offset(1);
        if !(*argv).is_null() {
            free(expansion as *mut libc::c_void);
        } else {
            o = variable_buffer_output(o, expansion, result);
            break;
        }
    }
    free(expansion as *mut libc::c_void);
    return o;
}
unsafe extern "C" fn func_wildcard(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = string_glob(*argv.offset(0 as libc::c_int as isize));
    o = variable_buffer_output(o, p, strlen(p));
    return o;
}
unsafe extern "C" fn func_eval(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    install_variable_buffer(&mut buf, &mut len);
    eval_buffer(*argv.offset(0 as libc::c_int as isize), 0 as *const floc);
    restore_variable_buffer(buf, len);
    return o;
}
unsafe extern "C" fn func_value(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut v: *mut variable = lookup_variable(
        *argv.offset(0 as libc::c_int as isize),
        strlen(*argv.offset(0 as libc::c_int as isize)),
    );
    if !v.is_null() {
        o = variable_buffer_output(o, (*v).value, strlen((*v).value));
    }
    return o;
}
unsafe extern "C" fn fold_newlines(
    mut buffer: *mut libc::c_char,
    mut length: *mut size_t,
    mut trim_newlines: libc::c_int,
) {
    let mut dst: *mut libc::c_char = buffer;
    let mut src: *mut libc::c_char = buffer;
    let mut last_nonnl: *mut libc::c_char = buffer.offset(-(1 as libc::c_int as isize));
    *src.offset(*length as isize) = 0 as libc::c_int as libc::c_char;
    while *src as libc::c_int != '\0' as i32 {
        if !(*src.offset(0 as libc::c_int as isize) as libc::c_int == '\r' as i32
            && *src.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32)
        {
            if *src as libc::c_int == '\n' as i32 {
                let fresh5 = dst;
                dst = dst.offset(1);
                *fresh5 = ' ' as i32 as libc::c_char;
            } else {
                last_nonnl = dst;
                let fresh6 = dst;
                dst = dst.offset(1);
                *fresh6 = *src;
            }
        }
        src = src.offset(1);
        src;
    }
    if trim_newlines == 0 && last_nonnl < dst.offset(-(2 as libc::c_int as isize)) {
        last_nonnl = dst.offset(-(2 as libc::c_int as isize));
    }
    last_nonnl = last_nonnl.offset(1);
    *last_nonnl = '\0' as i32 as libc::c_char;
    *length = last_nonnl.offset_from(buffer) as libc::c_long as size_t;
}
pub static mut shell_function_pid: pid_t = 0 as libc::c_int;
static mut shell_function_completed: libc::c_int = 0;
pub unsafe extern "C" fn shell_completed(
    mut exit_code: libc::c_int,
    mut exit_sig: libc::c_int,
) {
    let mut buf: [libc::c_char; 22] = [0; 22];
    shell_function_pid = 0 as libc::c_int;
    if exit_sig == 0 as libc::c_int && exit_code == 127 as libc::c_int {
        shell_function_completed = -(1 as libc::c_int);
    } else {
        shell_function_completed = 1 as libc::c_int;
    }
    if exit_code == 0 as libc::c_int && exit_sig > 0 as libc::c_int {
        exit_code = 128 as libc::c_int + exit_sig;
    }
    sprintf(buf.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, exit_code);
    define_variable_in_set(
        b".SHELLSTATUS\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        buf.as_mut_ptr(),
        o_override,
        0 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
}
pub unsafe extern "C" fn func_shell_base(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut trim_newlines: libc::c_int,
) -> *mut libc::c_char {
    let mut child: childbase = {
        let mut init = childbase {
            cmd_name: 0 as *mut libc::c_char,
            environment: 0 as *mut *mut libc::c_char,
            output: output {
                out: 0,
                err: 0,
                syncout: [0; 1],
                c2rust_padding: [0; 3],
            },
        };
        init
    };
    let mut batch_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut errfd: libc::c_int = 0;
    let mut command_argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pipedes: [libc::c_int; 2] = [0; 2];
    let mut pid: pid_t = 0;
    command_argv = construct_command_argv(
        *argv.offset(0 as libc::c_int as isize),
        0 as *mut *mut libc::c_char,
        0 as *mut file,
        0 as libc::c_int,
        &mut batch_filename,
    );
    if command_argv.is_null() {
        return o;
    }
    output_start();
    errfd = if !output_context.is_null() && (*output_context).err >= 0 as libc::c_int {
        (*output_context).err
    } else {
        fileno(stderr)
    };
    child.environment = target_environment(0 as *mut file, 0 as libc::c_int);
    if pipe(pipedes.as_mut_ptr()) < 0 as libc::c_int {
        error(
            reading_file,
            strlen(strerror(*__errno_location())),
            b"pipe: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        pid = -(1 as libc::c_int);
    } else {
        fd_noinherit(pipedes[1 as libc::c_int as usize]);
        fd_noinherit(pipedes[0 as libc::c_int as usize]);
        (child.output).set_syncout(1 as libc::c_int as libc::c_uint);
        child.output.out = pipedes[1 as libc::c_int as usize];
        child.output.err = errfd;
        pid = child_execute_job(&mut child, 1 as libc::c_int, command_argv);
        if pid < 0 as libc::c_int {
            shell_completed(127 as libc::c_int, 0 as libc::c_int);
        } else {
            let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut maxlen: size_t = 0;
            let mut i: size_t = 0;
            let mut cc: libc::c_int = 0;
            shell_function_pid = pid;
            shell_function_completed = 0 as libc::c_int;
            if pipedes[1 as libc::c_int as usize] >= 0 as libc::c_int {
                close(pipedes[1 as libc::c_int as usize]);
            }
            maxlen = 200 as libc::c_int as size_t;
            buffer = xmalloc(maxlen.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as *mut libc::c_char;
            i = 0 as libc::c_int as size_t;
            loop {
                if i == maxlen {
                    maxlen = (maxlen as libc::c_ulong)
                        .wrapping_add(512 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                    buffer = xrealloc(
                        buffer as *mut libc::c_void,
                        maxlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as *mut libc::c_char;
                }
                loop {
                    cc = read(
                        pipedes[0 as libc::c_int as usize],
                        &mut *buffer.offset(i as isize) as *mut libc::c_char
                            as *mut libc::c_void,
                        maxlen.wrapping_sub(i),
                    ) as libc::c_int;
                    if !(cc == -(1 as libc::c_int)
                        && *__errno_location() == 4 as libc::c_int)
                    {
                        break;
                    }
                }
                if cc <= 0 as libc::c_int {
                    break;
                }
                i = (i as libc::c_ulong).wrapping_add(cc as libc::c_ulong) as size_t
                    as size_t;
            }
            *buffer.offset(i as isize) = '\0' as i32 as libc::c_char;
            close(pipedes[0 as libc::c_int as usize]);
            while shell_function_completed == 0 as libc::c_int {
                reap_children(1 as libc::c_int, 0 as libc::c_int);
            }
            if !batch_filename.is_null() {
                if 0x2 as libc::c_int & db_level != 0 {
                    printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Cleaning up temporary batch file %s\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        batch_filename,
                    );
                    fflush(stdout);
                }
                remove(batch_filename);
                free(batch_filename as *mut libc::c_void);
            }
            shell_function_pid = 0 as libc::c_int;
            fold_newlines(buffer, &mut i, trim_newlines);
            o = variable_buffer_output(o, buffer, i);
            free(buffer as *mut libc::c_void);
        }
    }
    if !command_argv.is_null() {
        free(*command_argv.offset(0 as libc::c_int as isize) as *mut libc::c_void);
        free(command_argv as *mut libc::c_void);
    }
    free_childbase(&mut child);
    return o;
}
unsafe extern "C" fn func_shell(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    return func_shell_base(o, argv, 1 as libc::c_int);
}
unsafe extern "C" fn abspath(
    mut name: *const libc::c_char,
    mut apath: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut apath_limit: *const libc::c_char = 0 as *const libc::c_char;
    let mut root_len: libc::c_ulong = 1 as libc::c_int as libc::c_ulong;
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        return 0 as *mut libc::c_char;
    }
    apath_limit = apath.offset(4096 as libc::c_int as isize);
    if !(*name.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32) {
        if starting_directory.is_null() {
            return 0 as *mut libc::c_char;
        }
        strcpy(apath, starting_directory);
        dest = strchr(apath, '\0' as i32);
    } else {
        memcpy(apath as *mut libc::c_void, name as *const libc::c_void, root_len);
        *apath.offset(root_len as isize) = '\0' as i32 as libc::c_char;
        dest = apath.offset(root_len as isize);
        name = name.offset(root_len as isize);
    }
    end = name;
    start = end;
    while *start as libc::c_int != '\0' as i32 {
        let mut len: size_t = 0;
        while *stopchar_map.as_mut_ptr().offset(*start as libc::c_uchar as isize)
            as libc::c_int & 0x8000 as libc::c_int != 0 as libc::c_int
        {
            start = start.offset(1);
            start;
        }
        end = start;
        while !(*stopchar_map.as_mut_ptr().offset(*end as libc::c_uchar as isize)
            as libc::c_int & (0x8000 as libc::c_int | 0x1 as libc::c_int)
            != 0 as libc::c_int)
        {
            end = end.offset(1);
            end;
        }
        len = end.offset_from(start) as libc::c_long as size_t;
        if len == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        if !(len == 1 as libc::c_int as libc::c_ulong
            && *start.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32)
        {
            if len == 2 as libc::c_int as libc::c_ulong
                && *start.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *start.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
            {
                if dest > apath.offset(root_len as isize) {
                    dest = dest.offset(-1);
                    dest;
                    while !(*stopchar_map
                        .as_mut_ptr()
                        .offset(
                            *dest.offset(-(1 as libc::c_int) as isize) as libc::c_uchar
                                as isize,
                        ) as libc::c_int & 0x8000 as libc::c_int != 0 as libc::c_int)
                    {
                        dest = dest.offset(-1);
                        dest;
                    }
                }
            } else {
                if !(*stopchar_map
                    .as_mut_ptr()
                    .offset(
                        *dest.offset(-(1 as libc::c_int) as isize) as libc::c_uchar
                            as isize,
                    ) as libc::c_int & 0x8000 as libc::c_int != 0 as libc::c_int)
                {
                    let fresh7 = dest;
                    dest = dest.offset(1);
                    *fresh7 = '/' as i32 as libc::c_char;
                }
                if dest.offset(len as isize) >= apath_limit as *mut libc::c_char {
                    return 0 as *mut libc::c_char;
                }
                dest = mempcpy(
                    dest as *mut libc::c_void,
                    start as *const libc::c_void,
                    len,
                ) as *mut libc::c_char;
                *dest = '\0' as i32 as libc::c_char;
            }
        }
        start = end;
    }
    if dest > apath.offset(root_len as isize)
        && *stopchar_map
            .as_mut_ptr()
            .offset(*dest.offset(-(1 as libc::c_int) as isize) as libc::c_uchar as isize)
            as libc::c_int & 0x8000 as libc::c_int != 0 as libc::c_int
    {
        dest = dest.offset(-1);
        dest;
    }
    *dest = '\0' as i32 as libc::c_char;
    return apath;
}
unsafe extern "C" fn func_realpath(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut p: *const libc::c_char = *argv.offset(0 as libc::c_int as isize);
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut doneany: libc::c_int = 0 as libc::c_int;
    let mut len: size_t = 0 as libc::c_int as size_t;
    loop {
        path = find_next_token(&mut p, &mut len);
        if path.is_null() {
            break;
        }
        if len < 4096 as libc::c_int as libc::c_ulong {
            let mut rp: *mut libc::c_char = 0 as *mut libc::c_char;
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
            let mut in_0: [libc::c_char; 4097] = [0; 4097];
            let mut out: [libc::c_char; 4097] = [0; 4097];
            strncpy(in_0.as_mut_ptr(), path, len);
            in_0[len as usize] = '\0' as i32 as libc::c_char;
            loop {
                *__errno_location() = 0 as libc::c_int;
                rp = realpath(in_0.as_mut_ptr(), out.as_mut_ptr());
                if !(rp.is_null() && *__errno_location() == 4 as libc::c_int) {
                    break;
                }
            }
            if !rp.is_null() {
                let mut r: libc::c_int = 0;
                loop {
                    r = stat(out.as_mut_ptr(), &mut st);
                    if !(r == -(1 as libc::c_int)
                        && *__errno_location() == 4 as libc::c_int)
                    {
                        break;
                    }
                }
                if r == 0 as libc::c_int {
                    o = variable_buffer_output(
                        o,
                        out.as_mut_ptr(),
                        strlen(out.as_mut_ptr()),
                    );
                    o = variable_buffer_output(
                        o,
                        b" \0" as *const u8 as *const libc::c_char,
                        1 as libc::c_int as size_t,
                    );
                    doneany = 1 as libc::c_int;
                }
            }
        }
    }
    if doneany != 0 {
        o = o.offset(-1);
        o;
    }
    return o;
}
unsafe extern "C" fn func_file(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut fn_0: *mut libc::c_char = *argv.offset(0 as libc::c_int as isize);
    if *fn_0.offset(0 as libc::c_int as isize) as libc::c_int == '>' as i32 {
        let mut len: size_t = 0;
        let mut end: *const libc::c_char = 0 as *const libc::c_char;
        let mut start: *const libc::c_char = 0 as *const libc::c_char;
        let mut nm: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut fp: *mut FILE = 0 as *mut FILE;
        let mut mode: *const libc::c_char = b"w\0" as *const u8 as *const libc::c_char;
        fn_0 = fn_0.offset(1);
        fn_0;
        if *fn_0.offset(0 as libc::c_int as isize) as libc::c_int == '>' as i32 {
            mode = b"a\0" as *const u8 as *const libc::c_char;
            fn_0 = fn_0.offset(1);
            fn_0;
        }
        start = next_token(fn_0);
        if *start.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
            fatal(
                *expanding_var,
                0 as libc::c_int as size_t,
                dcgettext(
                    0 as *const libc::c_char,
                    b"file: missing filename\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        end = end_of_token(start);
        len = end.offset_from(start) as libc::c_long as size_t;
        let mut fresh8 = ::std::vec::from_elem(
            0,
            len.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
        );
        nm = fresh8.leak().as_mut_ptr() as *mut libc::c_char;
        memcpy(nm as *mut libc::c_void, start as *const libc::c_void, len);
        *nm.offset(len as isize) = '\0' as i32 as libc::c_char;
        loop {
            *__errno_location() = 0 as libc::c_int;
            fp = fopen(nm, mode);
            if !(fp.is_null() && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if fp.is_null() {
            fatal(
                reading_file,
                (strlen(nm)).wrapping_add(strlen(strerror(*__errno_location()))),
                dcgettext(
                    0 as *const libc::c_char,
                    b"open: %s: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                nm,
                strerror(*__errno_location()),
            );
        }
        command_count = command_count.wrapping_add(1);
        command_count;
        if !(*argv.offset(1 as libc::c_int as isize)).is_null() {
            let mut l: size_t = strlen(*argv.offset(1 as libc::c_int as isize));
            let mut nl: libc::c_int = (l == 0 as libc::c_int as libc::c_ulong
                || *(*argv.offset(1 as libc::c_int as isize))
                    .offset(l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int != '\n' as i32) as libc::c_int;
            if fputs(*argv.offset(1 as libc::c_int as isize), fp) == -(1 as libc::c_int)
                || nl != 0 && fputc('\n' as i32, fp) == -(1 as libc::c_int)
            {
                fatal(
                    reading_file,
                    (strlen(nm)).wrapping_add(strlen(strerror(*__errno_location()))),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"write: %s: %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    nm,
                    strerror(*__errno_location()),
                );
            }
        }
        if fclose(fp) != 0 {
            fatal(
                reading_file,
                (strlen(nm)).wrapping_add(strlen(strerror(*__errno_location()))),
                dcgettext(
                    0 as *const libc::c_char,
                    b"close: %s: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                nm,
                strerror(*__errno_location()),
            );
        }
    } else if *fn_0.offset(0 as libc::c_int as isize) as libc::c_int == '<' as i32 {
        let mut n: size_t = 0 as libc::c_int as size_t;
        let mut len_0: size_t = 0;
        let mut end_0: *const libc::c_char = 0 as *const libc::c_char;
        let mut start_0: *const libc::c_char = 0 as *const libc::c_char;
        let mut nm_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut fp_0: *mut FILE = 0 as *mut FILE;
        start_0 = next_token(fn_0.offset(1 as libc::c_int as isize));
        if *start_0.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
            fatal(
                *expanding_var,
                0 as libc::c_int as size_t,
                dcgettext(
                    0 as *const libc::c_char,
                    b"file: missing filename\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if !(*argv.offset(1 as libc::c_int as isize)).is_null() {
            fatal(
                *expanding_var,
                0 as libc::c_int as size_t,
                dcgettext(
                    0 as *const libc::c_char,
                    b"file: too many arguments\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        end_0 = end_of_token(start_0);
        len_0 = end_0.offset_from(start_0) as libc::c_long as size_t;
        let mut fresh9 = ::std::vec::from_elem(
            0,
            len_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
        );
        nm_0 = fresh9.leak().as_mut_ptr() as *mut libc::c_char;
        memcpy(nm_0 as *mut libc::c_void, start_0 as *const libc::c_void, len_0);
        *nm_0.offset(len_0 as isize) = '\0' as i32 as libc::c_char;
        loop {
            *__errno_location() = 0 as libc::c_int;
            fp_0 = fopen(nm_0, b"r\0" as *const u8 as *const libc::c_char);
            if !(fp_0.is_null() && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if fp_0.is_null() {
            if *__errno_location() == 2 as libc::c_int {
                if 0x2 as libc::c_int & db_level != 0 {
                    printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"file: Failed to open '%s': %s\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        nm_0,
                        strerror(*__errno_location()),
                    );
                    fflush(stdout);
                }
                return o;
            }
            fatal(
                reading_file,
                (strlen(nm_0)).wrapping_add(strlen(strerror(*__errno_location()))),
                dcgettext(
                    0 as *const libc::c_char,
                    b"open: %s: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                nm_0,
                strerror(*__errno_location()),
            );
        }
        loop {
            let mut buf: [libc::c_char; 1024] = [0; 1024];
            let mut l_0: size_t = fread(
                buf.as_mut_ptr() as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                fp_0,
            );
            if l_0 > 0 as libc::c_int as libc::c_ulong {
                o = variable_buffer_output(o, buf.as_mut_ptr(), l_0);
                n = (n as libc::c_ulong).wrapping_add(l_0) as size_t as size_t;
            }
            if ferror(fp_0) != 0 {
                if *__errno_location() != 4 as libc::c_int {
                    fatal(
                        reading_file,
                        (strlen(nm_0))
                            .wrapping_add(strlen(strerror(*__errno_location()))),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"read: %s: %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        nm_0,
                        strerror(*__errno_location()),
                    );
                }
            }
            if feof(fp_0) != 0 {
                break;
            }
        }
        if fclose(fp_0) != 0 {
            fatal(
                reading_file,
                (strlen(nm_0)).wrapping_add(strlen(strerror(*__errno_location()))),
                dcgettext(
                    0 as *const libc::c_char,
                    b"close: %s: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                nm_0,
                strerror(*__errno_location()),
            );
        }
        if n != 0
            && *o.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\n' as i32
        {
            o = o
                .offset(
                    -((1 as libc::c_int
                        + (n > 1 as libc::c_int as libc::c_ulong
                            && *o.offset(-(2 as libc::c_int) as isize) as libc::c_int
                                == '\r' as i32) as libc::c_int) as isize),
                );
        }
    } else {
        fatal(
            *expanding_var,
            strlen(fn_0),
            dcgettext(
                0 as *const libc::c_char,
                b"file: invalid file operation: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            fn_0,
        );
    }
    return o;
}
unsafe extern "C" fn func_abspath(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut p: *const libc::c_char = *argv.offset(0 as libc::c_int as isize);
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut doneany: libc::c_int = 0 as libc::c_int;
    let mut len: size_t = 0 as libc::c_int as size_t;
    loop {
        path = find_next_token(&mut p, &mut len);
        if path.is_null() {
            break;
        }
        if len < 4096 as libc::c_int as libc::c_ulong {
            let mut in_0: [libc::c_char; 4097] = [0; 4097];
            let mut out: [libc::c_char; 4097] = [0; 4097];
            strncpy(in_0.as_mut_ptr(), path, len);
            in_0[len as usize] = '\0' as i32 as libc::c_char;
            if !(abspath(in_0.as_mut_ptr(), out.as_mut_ptr())).is_null() {
                o = variable_buffer_output(
                    o,
                    out.as_mut_ptr(),
                    strlen(out.as_mut_ptr()),
                );
                o = variable_buffer_output(
                    o,
                    b" \0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int as size_t,
                );
                doneany = 1 as libc::c_int;
            }
        }
    }
    if doneany != 0 {
        o = o.offset(-1);
        o;
    }
    return o;
}
static mut function_table_init: [function_table_entry; 38] = [function_table_entry {
    fptr: C2RustUnnamed { func_ptr: None },
    name: 0 as *const libc::c_char,
    len: 0,
    minimum_args: 0,
    maximum_args: 0,
    expand_args_alloc_fn_adds_command: [0; 1],
    c2rust_padding: [0; 4],
}; 38];
unsafe extern "C" fn expand_builtin_function(
    mut o: *mut libc::c_char,
    mut argc: libc::c_uint,
    mut argv: *mut *mut libc::c_char,
    mut entry_p: *const function_table_entry,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if argc < (*entry_p).minimum_args as libc::c_uint {
        fatal(
            *expanding_var,
            strlen((*entry_p).name),
            dcgettext(
                0 as *const libc::c_char,
                b"insufficient number of arguments (%u) to function '%s'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            argc,
            (*entry_p).name,
        );
    }
    if argc == 0 && (*entry_p).alloc_fn() == 0 {
        return o;
    }
    if ((*entry_p).fptr.func_ptr).is_none() {
        fatal(
            *expanding_var,
            strlen((*entry_p).name),
            dcgettext(
                0 as *const libc::c_char,
                b"unimplemented on this platform: function '%s'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*entry_p).name,
        );
    }
    if (*entry_p).adds_command() != 0 {
        command_count = command_count.wrapping_add(1);
        command_count;
    }
    if (*entry_p).alloc_fn() == 0 {
        return ((*entry_p).fptr.func_ptr).unwrap()(o, argv, (*entry_p).name);
    }
    p = ((*entry_p).fptr.alloc_func_ptr).unwrap()((*entry_p).name, argc, argv);
    if !p.is_null() {
        o = variable_buffer_output(o, p, strlen(p));
        free(p as *mut libc::c_void);
    }
    return o;
}
pub unsafe extern "C" fn handle_function(
    mut op: *mut *mut libc::c_char,
    mut stringp: *mut *const libc::c_char,
) -> libc::c_int {
    let mut entry_p: *const function_table_entry = 0 as *const function_table_entry;
    let mut openparen: libc::c_char = *(*stringp).offset(0 as libc::c_int as isize);
    let mut closeparen: libc::c_char = (if openparen as libc::c_int == '(' as i32 {
        ')' as i32
    } else {
        '}' as i32
    }) as libc::c_char;
    let mut beg: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut abeg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut argvp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut nargs: libc::c_uint = 0;
    beg = (*stringp).offset(1 as libc::c_int as isize);
    entry_p = lookup_function(beg);
    if entry_p.is_null() {
        return 0 as libc::c_int;
    }
    beg = beg.offset((*entry_p).len as libc::c_int as isize);
    while *stopchar_map.as_mut_ptr().offset(*beg as libc::c_uchar as isize)
        as libc::c_int & (0x2 as libc::c_int | 0x4 as libc::c_int) != 0 as libc::c_int
    {
        beg = beg.offset(1);
        beg;
    }
    nargs = 1 as libc::c_int as libc::c_uint;
    end = beg;
    while *end as libc::c_int != '\0' as i32 {
        if *stopchar_map.as_mut_ptr().offset(*end as libc::c_uchar as isize)
            as libc::c_int & (0x80 as libc::c_int | 0x400 as libc::c_int)
            != 0 as libc::c_int
        {
            if *end as libc::c_int == ',' as i32 {
                nargs = nargs.wrapping_add(1);
                nargs;
            } else if *end as libc::c_int == openparen as libc::c_int {
                count += 1;
                count;
            } else if *end as libc::c_int == closeparen as libc::c_int
                && {
                    count -= 1;
                    count < 0 as libc::c_int
                }
            {
                break;
            }
        }
        end = end.offset(1);
        end;
    }
    if count >= 0 as libc::c_int {
        fatal(
            *expanding_var,
            strlen((*entry_p).name),
            dcgettext(
                0 as *const libc::c_char,
                b"unterminated call to function '%s': missing '%c'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*entry_p).name,
            closeparen as libc::c_int,
        );
    }
    *stringp = end;
    let mut fresh10 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(
                nargs.wrapping_add(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
            ) as usize,
    );
    argv = fresh10.leak().as_mut_ptr() as *mut *mut libc::c_char;
    argvp = argv;
    if (*entry_p).expand_args() != 0 {
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        p = beg;
        nargs = 0 as libc::c_int as libc::c_uint;
        while p <= end {
            let mut next: *const libc::c_char = 0 as *const libc::c_char;
            nargs = nargs.wrapping_add(1);
            nargs;
            if nargs == (*entry_p).maximum_args as libc::c_uint
                || {
                    next = find_next_argument(openparen, closeparen, p, end);
                    next.is_null()
                }
            {
                next = end;
            }
            *argvp = expand_argument(p, next);
            p = next.offset(1 as libc::c_int as isize);
            argvp = argvp.offset(1);
            argvp;
        }
    } else {
        let mut len: size_t = end.offset_from(beg) as libc::c_long as size_t;
        let mut p_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut aend: *mut libc::c_char = 0 as *mut libc::c_char;
        abeg = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        aend = mempcpy(abeg as *mut libc::c_void, beg as *const libc::c_void, len)
            as *mut libc::c_char;
        *aend = '\0' as i32 as libc::c_char;
        p_0 = abeg;
        nargs = 0 as libc::c_int as libc::c_uint;
        while p_0 <= aend {
            let mut next_0: *mut libc::c_char = 0 as *mut libc::c_char;
            nargs = nargs.wrapping_add(1);
            nargs;
            if nargs == (*entry_p).maximum_args as libc::c_uint
                || {
                    next_0 = find_next_argument(openparen, closeparen, p_0, aend);
                    next_0.is_null()
                }
            {
                next_0 = aend;
            }
            *argvp = p_0;
            *next_0 = '\0' as i32 as libc::c_char;
            p_0 = next_0.offset(1 as libc::c_int as isize);
            argvp = argvp.offset(1);
            argvp;
        }
    }
    *argvp = 0 as *mut libc::c_char;
    *op = expand_builtin_function(*op, nargs, argv, entry_p);
    if (*entry_p).expand_args() != 0 {
        argvp = argv;
        while !(*argvp).is_null() {
            free(*argvp as *mut libc::c_void);
            argvp = argvp.offset(1);
            argvp;
        }
    } else {
        free(abeg as *mut libc::c_void);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn func_call(
    mut o: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut funcname: *const libc::c_char,
) -> *mut libc::c_char {
    static mut max_args: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut body: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flen: size_t = 0;
    let mut i: libc::c_uint = 0;
    let mut saved_args: libc::c_int = 0;
    let mut entry_p: *const function_table_entry = 0 as *const function_table_entry;
    let mut v: *mut variable = 0 as *mut variable;
    fname = next_token(*argv.offset(0 as libc::c_int as isize));
    *(end_of_token(fname))
        .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    if *fname as libc::c_int == '\0' as i32 {
        return o;
    }
    entry_p = lookup_function(fname);
    if !entry_p.is_null() {
        i = 0 as libc::c_int as libc::c_uint;
        while !(*argv.offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize))
            .is_null()
        {
            i = i.wrapping_add(1);
            i;
        }
        return expand_builtin_function(
            o,
            i,
            argv.offset(1 as libc::c_int as isize),
            entry_p,
        );
    }
    flen = strlen(fname);
    v = lookup_variable(fname, flen);
    if v.is_null() {
        warn_undefined(fname, flen);
    }
    if v.is_null() || *(*v).value as libc::c_int == '\0' as i32 {
        return o;
    }
    let mut fresh11 = ::std::vec::from_elem(
        0,
        flen.wrapping_add(4 as libc::c_int as libc::c_ulong) as usize,
    );
    body = fresh11.leak().as_mut_ptr() as *mut libc::c_char;
    *body.offset(0 as libc::c_int as isize) = '$' as i32 as libc::c_char;
    *body.offset(1 as libc::c_int as isize) = '(' as i32 as libc::c_char;
    memcpy(
        body.offset(2 as libc::c_int as isize) as *mut libc::c_void,
        fname as *const libc::c_void,
        flen,
    );
    *body
        .offset(
            flen.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
        ) = ')' as i32 as libc::c_char;
    *body
        .offset(
            flen.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
        ) = '\0' as i32 as libc::c_char;
    push_new_variable_scope();
    i = 0 as libc::c_int as libc::c_uint;
    while !(*argv).is_null() {
        let mut num: [libc::c_char; 22] = [0; 22];
        sprintf(num.as_mut_ptr(), b"%u\0" as *const u8 as *const libc::c_char, i);
        define_variable_in_set(
            num.as_mut_ptr(),
            strlen(num.as_mut_ptr()),
            *argv,
            o_automatic,
            0 as libc::c_int,
            (*current_variable_set_list).set,
            0 as *mut floc,
        );
        i = i.wrapping_add(1);
        i;
        argv = argv.offset(1);
        argv;
    }
    while i < max_args {
        let mut num_0: [libc::c_char; 22] = [0; 22];
        sprintf(num_0.as_mut_ptr(), b"%u\0" as *const u8 as *const libc::c_char, i);
        define_variable_in_set(
            num_0.as_mut_ptr(),
            strlen(num_0.as_mut_ptr()),
            b"\0" as *const u8 as *const libc::c_char,
            o_automatic,
            0 as libc::c_int,
            (*current_variable_set_list).set,
            0 as *mut floc,
        );
        i = i.wrapping_add(1);
        i;
    }
    (*v)
        .set_exp_count(
            (((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int)
                as libc::c_uint,
        );
    saved_args = max_args as libc::c_int;
    max_args = i;
    o = variable_expand_string(
        o,
        body,
        flen.wrapping_add(3 as libc::c_int as libc::c_ulong),
    );
    max_args = saved_args as libc::c_uint;
    (*v).set_exp_count(0 as libc::c_int as libc::c_uint);
    pop_variable_scope();
    return o.offset(strlen(o) as isize);
}
pub unsafe extern "C" fn define_new_function(
    mut flocp: *const floc,
    mut name: *const libc::c_char,
    mut min: libc::c_uint,
    mut max: libc::c_uint,
    mut flags: libc::c_uint,
    mut func: gmk_func_ptr,
) {
    let mut e: *const libc::c_char = name;
    let mut ent: *mut function_table_entry = 0 as *mut function_table_entry;
    let mut len: size_t = 0;
    while *stopchar_map.as_mut_ptr().offset(*e as libc::c_uchar as isize) as libc::c_int
        & 0x2000 as libc::c_int != 0 as libc::c_int
    {
        e = e.offset(1);
        e;
    }
    len = e.offset_from(name) as libc::c_long as size_t;
    if len == 0 as libc::c_int as libc::c_ulong {
        fatal(
            flocp,
            0 as libc::c_int as size_t,
            dcgettext(
                0 as *const libc::c_char,
                b"Empty function name\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if *name as libc::c_int == '.' as i32 || *e as libc::c_int != '\0' as i32 {
        fatal(
            flocp,
            strlen(name),
            dcgettext(
                0 as *const libc::c_char,
                b"Invalid function name: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
    }
    if len > 255 as libc::c_int as libc::c_ulong {
        fatal(
            flocp,
            strlen(name),
            dcgettext(
                0 as *const libc::c_char,
                b"Function name too long: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
    }
    if min > 255 as libc::c_int as libc::c_uint {
        fatal(
            flocp,
            (53 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                .wrapping_div(22 as libc::c_int as libc::c_ulong)
                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen(name)),
            dcgettext(
                0 as *const libc::c_char,
                b"Invalid minimum argument count (%u) for function %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            min,
            name,
        );
    }
    if max > 255 as libc::c_int as libc::c_uint || max != 0 && max < min {
        fatal(
            flocp,
            (53 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                .wrapping_div(22 as libc::c_int as libc::c_ulong)
                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen(name)),
            dcgettext(
                0 as *const libc::c_char,
                b"Invalid maximum argument count (%u) for function %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            max,
            name,
        );
    }
    ent = xmalloc(::std::mem::size_of::<function_table_entry>() as libc::c_ulong)
        as *mut function_table_entry;
    (*ent).name = strcache_add(name);
    (*ent).len = len as libc::c_uchar;
    (*ent).minimum_args = min as libc::c_uchar;
    (*ent).maximum_args = max as libc::c_uchar;
    (*ent)
        .set_expand_args(
            (if flags & 0x1 as libc::c_int as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
            {
                0 as libc::c_int
            } else {
                1 as libc::c_int
            }) as libc::c_uint,
        );
    (*ent).set_alloc_fn(1 as libc::c_int as libc::c_uint);
    (*ent).set_adds_command(1 as libc::c_int as libc::c_uint);
    (*ent).fptr.alloc_func_ptr = func;
    ent = hash_insert(&mut function_table, ent as *const libc::c_void)
        as *mut function_table_entry;
    free(ent as *mut libc::c_void);
}
pub unsafe extern "C" fn hash_init_function_table() {
    hash_init(
        &mut function_table,
        (::std::mem::size_of::<[function_table_entry; 38]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<function_table_entry>() as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        Some(
            function_table_entry_hash_1
                as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
        ),
        Some(
            function_table_entry_hash_2
                as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
        ),
        Some(
            function_table_entry_hash_cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    hash_load(
        &mut function_table,
        function_table_init.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[function_table_entry; 38]>() as libc::c_ulong)
            .wrapping_div(
                ::std::mem::size_of::<function_table_entry>() as libc::c_ulong,
            ),
        ::std::mem::size_of::<function_table_entry>() as libc::c_ulong,
    );
}
unsafe extern "C" fn run_static_initializers() {
    function_table_init = [
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_abspath
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"abspath\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 0 as libc::c_int as libc::c_uchar,
                maximum_args: 1 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_addsuffix_addprefix
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"addprefix\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 2 as libc::c_int as libc::c_uchar,
                maximum_args: 2 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_addsuffix_addprefix
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"addsuffix\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 2 as libc::c_int as libc::c_uchar,
                maximum_args: 2 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_basename_dir
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"basename\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 0 as libc::c_int as libc::c_uchar,
                maximum_args: 1 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_basename_dir
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"dir\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 0 as libc::c_int as libc::c_uchar,
                maximum_args: 1 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_notdir_suffix
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"notdir\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 0 as libc::c_int as libc::c_uchar,
                maximum_args: 1 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_subst
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"subst\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 3 as libc::c_int as libc::c_uchar,
                maximum_args: 3 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_notdir_suffix
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"suffix\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 0 as libc::c_int as libc::c_uchar,
                maximum_args: 1 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_filter_filterout
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"filter\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 2 as libc::c_int as libc::c_uchar,
                maximum_args: 2 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_filter_filterout
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"filter-out\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 2 as libc::c_int as libc::c_uchar,
                maximum_args: 2 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_findstring
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"findstring\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 2 as libc::c_int as libc::c_uchar,
                maximum_args: 2 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_firstword
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"firstword\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 0 as libc::c_int as libc::c_uchar,
                maximum_args: 1 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_flavor
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"flavor\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 0 as libc::c_int as libc::c_uchar,
                maximum_args: 1 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_join
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"join\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 2 as libc::c_int as libc::c_uchar,
                maximum_args: 2 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_lastword
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"lastword\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 0 as libc::c_int as libc::c_uchar,
                maximum_args: 1 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_patsubst
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"patsubst\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 3 as libc::c_int as libc::c_uchar,
                maximum_args: 3 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_realpath
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"realpath\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 0 as libc::c_int as libc::c_uchar,
                maximum_args: 1 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_shell
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"shell\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 0 as libc::c_int as libc::c_uchar,
                maximum_args: 1 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_sort
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"sort\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 0 as libc::c_int as libc::c_uchar,
                maximum_args: 1 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_strip
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"strip\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 0 as libc::c_int as libc::c_uchar,
                maximum_args: 1 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_wildcard
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"wildcard\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 0 as libc::c_int as libc::c_uchar,
                maximum_args: 1 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_word
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"word\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 2 as libc::c_int as libc::c_uchar,
                maximum_args: 2 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_wordlist
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"wordlist\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 3 as libc::c_int as libc::c_uchar,
                maximum_args: 3 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_words
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"words\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 0 as libc::c_int as libc::c_uchar,
                maximum_args: 1 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_origin
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"origin\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 0 as libc::c_int as libc::c_uchar,
                maximum_args: 1 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_foreach
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"foreach\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 3 as libc::c_int as libc::c_uchar,
                maximum_args: 3 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(0 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_let
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"let\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 3 as libc::c_int as libc::c_uchar,
                maximum_args: 3 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(0 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_call
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"call\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 1 as libc::c_int as libc::c_uchar,
                maximum_args: 0 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_error
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"info\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 0 as libc::c_int as libc::c_uchar,
                maximum_args: 1 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_error
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"error\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 0 as libc::c_int as libc::c_uchar,
                maximum_args: 1 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_error
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"warning\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 0 as libc::c_int as libc::c_uchar,
                maximum_args: 1 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_intcmp
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"intcmp\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 2 as libc::c_int as libc::c_uchar,
                maximum_args: 5 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(0 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_if
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"if\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 2 as libc::c_int as libc::c_uchar,
                maximum_args: 3 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(0 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_or
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"or\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 1 as libc::c_int as libc::c_uchar,
                maximum_args: 0 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(0 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_and
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"and\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 1 as libc::c_int as libc::c_uchar,
                maximum_args: 0 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(0 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_value
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"value\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 0 as libc::c_int as libc::c_uchar,
                maximum_args: 1 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_eval
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"eval\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 0 as libc::c_int as libc::c_uchar,
                maximum_args: 1 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = function_table_entry {
                expand_args_alloc_fn_adds_command: [0; 1],
                c2rust_padding: [0; 4],
                fptr: C2RustUnnamed {
                    func_ptr: Some(
                        func_file
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut *mut libc::c_char,
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                },
                name: b"file\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                minimum_args: 1 as libc::c_int as libc::c_uchar,
                maximum_args: 2 as libc::c_int as libc::c_uchar,
            };
            init.set_expand_args(1 as libc::c_int as libc::c_uint);
            init.set_alloc_fn(0 as libc::c_int as libc::c_uint);
            init.set_adds_command(0 as libc::c_int as libc::c_uint);
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
