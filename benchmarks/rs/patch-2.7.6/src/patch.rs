use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type gl_list_impl;
    pub type gl_list_node_impl;
    pub type quoting_options;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn setbuf(__stream: *mut FILE, __buf: *mut libc::c_char);
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn exit(_: libc::c_int) -> !;
    fn merge_hunk(
        hunk: libc::c_int,
        _: *mut outstate,
        where_0: lin,
        _: *mut bool,
    ) -> bool;
    fn __errno_location() -> *mut libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn argmatch(
        arg: *const libc::c_char,
        arglist: *const *const libc::c_char,
        vallist: *const libc::c_char,
        valsize: size_t,
    ) -> ptrdiff_t;
    fn argmatch_invalid(
        context: *const libc::c_char,
        value: *const libc::c_char,
        problem: ptrdiff_t,
    );
    static mut exit_failure: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut input_lines: lin;
    fn ifetch(_: lin, _: bool, _: *mut size_t) -> *const libc::c_char;
    fn get_input_file(_: *const libc::c_char, _: *const libc::c_char, _: mode_t) -> bool;
    fn re_input();
    fn scan_input(_: *mut libc::c_char, _: mode_t);
    fn pch_end() -> lin;
    fn pch_first() -> lin;
    fn pch_hunk_beg() -> lin;
    fn pch_c_function() -> *const libc::c_char;
    fn pch_git_diff() -> bool;
    fn pch_timestr(which: bool) -> *const libc::c_char;
    fn pch_mode(which: bool) -> mode_t;
    fn pch_newfirst() -> lin;
    fn pch_prefix_context() -> lin;
    fn pch_ptrn_lines() -> lin;
    fn pch_repl_lines() -> lin;
    fn pch_suffix_context() -> lin;
    fn pch_swap() -> bool;
    fn pch_write_line(_: lin, _: *mut FILE) -> bool;
    fn there_is_another_patch(_: bool, _: *mut mode_t) -> bool;
    fn pfetch(_: lin) -> *mut libc::c_char;
    fn pch_char(_: lin) -> libc::c_char;
    fn another_hunk(_: diff, _: bool) -> libc::c_int;
    fn pch_says_nonexistent(_: bool) -> libc::c_int;
    fn pch_line_len(_: lin) -> size_t;
    fn pch_name(_: nametype) -> *const libc::c_char;
    fn pch_copy() -> bool;
    fn pch_rename() -> bool;
    fn do_ed_script(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut bool,
        _: *mut FILE,
    );
    fn open_patch_file(_: *const libc::c_char);
    fn re_patch();
    fn pch_normalize(_: diff);
    static mut p_timestamp: [timespec; 2];
    static quoting_style_args: [*const libc::c_char; 0];
    fn set_quoting_style(o: *mut quoting_options, s: quoting_style);
    fn quotearg(arg: *const libc::c_char) -> *mut libc::c_char;
    static mut backup_type: backup_type;
    fn ok_to_reverse(_: *const libc::c_char, _: ...) -> bool;
    fn say(_: *const libc::c_char, _: ...);
    fn fatal(_: *const libc::c_char, _: ...) -> !;
    fn pfatal(_: *const libc::c_char, _: ...) -> !;
    fn create_file(
        _: *const libc::c_char,
        _: libc::c_int,
        _: mode_t,
        _: bool,
    ) -> libc::c_int;
    fn format_linenum(_: *mut libc::c_char, _: lin) -> *mut libc::c_char;
    fn copy_file(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut stat,
        _: libc::c_int,
        _: mode_t,
        _: bool,
    );
    fn append_to_file(_: *const libc::c_char, _: *const libc::c_char);
    fn exit_with_signal(_: libc::c_int) -> !;
    fn ignore_signals();
    fn init_backup_hash_table();
    fn init_time();
    fn xalloc_die() -> !;
    fn create_backup(_: *const libc::c_char, _: *const stat, _: bool);
    fn move_file(
        _: *const libc::c_char,
        _: *mut bool,
        _: *const stat,
        _: *const libc::c_char,
        _: mode_t,
        _: bool,
    );
    fn removedirs(_: *const libc::c_char);
    fn find_backup_file_name(
        _: *const libc::c_char,
        _: backup_type,
    ) -> *mut libc::c_char;
    fn get_version(
        context: *const libc::c_char,
        arg: *const libc::c_char,
    ) -> backup_type;
    fn lookup_file_id(_: *const stat) -> file_id_type;
    fn set_queued_output(_: *const stat, _: bool);
    fn write_fatal() -> !;
    static mut simple_backup_suffix: *const libc::c_char;
    fn set_signals(_: bool);
    fn insert_file_id(_: *const stat, _: file_id_type);
    fn has_queued_output(_: *const stat) -> bool;
    fn make_tempfile(
        _: *mut *const libc::c_char,
        _: libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: mode_t,
    ) -> libc::c_int;
    fn stat_file(_: *const libc::c_char, _: *mut stat) -> libc::c_int;
    fn set_file_attributes(
        _: *const libc::c_char,
        _: file_attributes,
        _: *const libc::c_char,
        _: *const stat,
        _: mode_t,
        _: *mut timespec,
    );
    fn version();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    static gl_linked_list_implementation: gl_list_implementation;
    #[link_name = "unsafe"]
    static mut unsafe_0: bool;
    fn safe_unlink(pathname: *const libc::c_char) -> libc::c_int;
    fn safe_access(pathname: *const libc::c_char, mode: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub type off_t = __off_t;
pub type ptrdiff_t = libc::c_long;
pub type mode_t = __mode_t;
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
pub type lin = off_t;
pub type C2RustUnnamed = libc::c_uint;
pub const VERBOSE: C2RustUnnamed = 2;
pub const SILENT: C2RustUnnamed = 1;
pub const DEFAULT_VERBOSITY: C2RustUnnamed = 0;
pub type diff = libc::c_uint;
pub const GIT_BINARY_DIFF: diff = 6;
pub const UNI_DIFF: diff = 5;
pub const NEW_CONTEXT_DIFF: diff = 4;
pub const ED_DIFF: diff = 3;
pub const NORMAL_DIFF: diff = 2;
pub const CONTEXT_DIFF: diff = 1;
pub const NO_DIFF: diff = 0;
pub type gl_list_t = *mut gl_list_impl;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gl_list_implementation {
    pub nx_create_empty: Option::<
        unsafe extern "C" fn(
            gl_list_implementation_t,
            gl_listelement_equals_fn,
            gl_listelement_hashcode_fn,
            gl_listelement_dispose_fn,
            bool,
        ) -> gl_list_t,
    >,
    pub nx_create: Option::<
        unsafe extern "C" fn(
            gl_list_implementation_t,
            gl_listelement_equals_fn,
            gl_listelement_hashcode_fn,
            gl_listelement_dispose_fn,
            bool,
            size_t,
            *mut *const libc::c_void,
        ) -> gl_list_t,
    >,
    pub size: Option::<unsafe extern "C" fn(gl_list_t) -> size_t>,
    pub node_value: Option::<
        unsafe extern "C" fn(gl_list_t, gl_list_node_t) -> *const libc::c_void,
    >,
    pub node_nx_set_value: Option::<
        unsafe extern "C" fn(
            gl_list_t,
            gl_list_node_t,
            *const libc::c_void,
        ) -> libc::c_int,
    >,
    pub next_node: Option::<
        unsafe extern "C" fn(gl_list_t, gl_list_node_t) -> gl_list_node_t,
    >,
    pub previous_node: Option::<
        unsafe extern "C" fn(gl_list_t, gl_list_node_t) -> gl_list_node_t,
    >,
    pub get_at: Option::<unsafe extern "C" fn(gl_list_t, size_t) -> *const libc::c_void>,
    pub nx_set_at: Option::<
        unsafe extern "C" fn(gl_list_t, size_t, *const libc::c_void) -> gl_list_node_t,
    >,
    pub search_from_to: Option::<
        unsafe extern "C" fn(
            gl_list_t,
            size_t,
            size_t,
            *const libc::c_void,
        ) -> gl_list_node_t,
    >,
    pub indexof_from_to: Option::<
        unsafe extern "C" fn(gl_list_t, size_t, size_t, *const libc::c_void) -> size_t,
    >,
    pub nx_add_first: Option::<
        unsafe extern "C" fn(gl_list_t, *const libc::c_void) -> gl_list_node_t,
    >,
    pub nx_add_last: Option::<
        unsafe extern "C" fn(gl_list_t, *const libc::c_void) -> gl_list_node_t,
    >,
    pub nx_add_before: Option::<
        unsafe extern "C" fn(
            gl_list_t,
            gl_list_node_t,
            *const libc::c_void,
        ) -> gl_list_node_t,
    >,
    pub nx_add_after: Option::<
        unsafe extern "C" fn(
            gl_list_t,
            gl_list_node_t,
            *const libc::c_void,
        ) -> gl_list_node_t,
    >,
    pub nx_add_at: Option::<
        unsafe extern "C" fn(gl_list_t, size_t, *const libc::c_void) -> gl_list_node_t,
    >,
    pub remove_node: Option::<unsafe extern "C" fn(gl_list_t, gl_list_node_t) -> bool>,
    pub remove_at: Option::<unsafe extern "C" fn(gl_list_t, size_t) -> bool>,
    pub remove_elt: Option::<
        unsafe extern "C" fn(gl_list_t, *const libc::c_void) -> bool,
    >,
    pub list_free: Option::<unsafe extern "C" fn(gl_list_t) -> ()>,
    pub iterator: Option::<unsafe extern "C" fn(gl_list_t) -> gl_list_iterator_t>,
    pub iterator_from_to: Option::<
        unsafe extern "C" fn(gl_list_t, size_t, size_t) -> gl_list_iterator_t,
    >,
    pub iterator_next: Option::<
        unsafe extern "C" fn(
            *mut gl_list_iterator_t,
            *mut *const libc::c_void,
            *mut gl_list_node_t,
        ) -> bool,
    >,
    pub iterator_free: Option::<unsafe extern "C" fn(*mut gl_list_iterator_t) -> ()>,
    pub sortedlist_search: Option::<
        unsafe extern "C" fn(
            gl_list_t,
            gl_listelement_compar_fn,
            *const libc::c_void,
        ) -> gl_list_node_t,
    >,
    pub sortedlist_search_from_to: Option::<
        unsafe extern "C" fn(
            gl_list_t,
            gl_listelement_compar_fn,
            size_t,
            size_t,
            *const libc::c_void,
        ) -> gl_list_node_t,
    >,
    pub sortedlist_indexof: Option::<
        unsafe extern "C" fn(
            gl_list_t,
            gl_listelement_compar_fn,
            *const libc::c_void,
        ) -> size_t,
    >,
    pub sortedlist_indexof_from_to: Option::<
        unsafe extern "C" fn(
            gl_list_t,
            gl_listelement_compar_fn,
            size_t,
            size_t,
            *const libc::c_void,
        ) -> size_t,
    >,
    pub sortedlist_nx_add: Option::<
        unsafe extern "C" fn(
            gl_list_t,
            gl_listelement_compar_fn,
            *const libc::c_void,
        ) -> gl_list_node_t,
    >,
    pub sortedlist_remove: Option::<
        unsafe extern "C" fn(
            gl_list_t,
            gl_listelement_compar_fn,
            *const libc::c_void,
        ) -> bool,
    >,
}
pub type gl_listelement_compar_fn = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type gl_list_node_t = *mut gl_list_node_impl;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gl_list_iterator_t {
    pub vtable: *const gl_list_implementation,
    pub list: gl_list_t,
    pub count: size_t,
    pub p: *mut libc::c_void,
    pub q: *mut libc::c_void,
    pub i: size_t,
    pub j: size_t,
}
pub type gl_listelement_dispose_fn = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> (),
>;
pub type gl_listelement_hashcode_fn = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> size_t,
>;
pub type gl_listelement_equals_fn = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type gl_list_implementation_t = *const gl_list_implementation;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gl_list_impl_base {
    pub vtable: *const gl_list_implementation,
    pub equals_fn: gl_listelement_equals_fn,
    pub hashcode_fn: gl_listelement_hashcode_fn,
    pub dispose_fn: gl_listelement_dispose_fn,
    pub allow_duplicates: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_to_output {
    pub from: *mut libc::c_char,
    pub from_st: stat,
    pub to: *mut libc::c_char,
    pub mode: mode_t,
    pub backup: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct outstate {
    pub ofp: *mut FILE,
    pub after_newline: bool,
    pub zero_output: bool,
}
pub type conflict_style = libc::c_uint;
pub const MERGE_DIFF3: conflict_style = 1;
pub const MERGE_MERGE: conflict_style = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type nametype = libc::c_uint;
pub const NONE: nametype = 3;
pub const INDEX: nametype = 2;
pub const NEW: nametype = 1;
pub const OLD: nametype = 0;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const TIMESPEC_RESOLUTION: C2RustUnnamed_0 = 1000000000;
pub type backup_type = libc::c_uint;
pub const numbered_backups: backup_type = 3;
pub const numbered_existing_backups: backup_type = 2;
pub const simple_backups: backup_type = 1;
pub const no_backups: backup_type = 0;
pub type file_id_type = libc::c_uint;
pub const OVERWRITTEN: file_id_type = 3;
pub const DELETE_LATER: file_id_type = 2;
pub const CREATED: file_id_type = 1;
pub const UNKNOWN: file_id_type = 0;
pub type file_attributes = libc::c_uint;
pub const FA_XATTRS: file_attributes = 8;
pub const FA_MODE: file_attributes = 4;
pub const FA_IDS: file_attributes = 2;
pub const FA_TIMES: file_attributes = 1;
pub const OUTSIDE: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IN_ELSE: C2RustUnnamed_1 = 3;
pub const IN_IFDEF: C2RustUnnamed_1 = 2;
pub const IN_IFNDEF: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const RO_FAIL: C2RustUnnamed_2 = 2;
pub const RO_WARN: C2RustUnnamed_2 = 1;
pub const RO_IGNORE: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_to_delete {
    pub name: *mut libc::c_char,
    pub st: stat,
    pub backup: bool,
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
pub static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut bufsize: size_t = 0;
pub static mut using_plan_a: bool = false;
pub static mut inname: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut outfile: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut inerrno: libc::c_int = 0;
pub static mut invc: libc::c_int = 0;
pub static mut instat: stat = stat {
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
pub static mut dry_run: bool = false;
pub static mut posixly_correct: bool = false;
pub static mut origprae: *const libc::c_char = 0 as *const libc::c_char;
pub static mut origbase: *const libc::c_char = 0 as *const libc::c_char;
pub static mut origsuff: *const libc::c_char = 0 as *const libc::c_char;
pub static mut TMPINNAME: *const libc::c_char = 0 as *const libc::c_char;
pub static mut TMPOUTNAME: *const libc::c_char = 0 as *const libc::c_char;
pub static mut TMPPATNAME: *const libc::c_char = 0 as *const libc::c_char;
pub static mut TMPINNAME_needs_removal: bool = false;
pub static mut TMPOUTNAME_needs_removal: bool = false;
pub static mut TMPPATNAME_needs_removal: bool = false;
pub static mut debug: libc::c_int = 0;
pub static mut force: bool = false;
pub static mut batch: bool = false;
pub static mut noreverse: bool = false;
pub static mut reverse: bool = false;
pub static mut verbosity: C2RustUnnamed = DEFAULT_VERBOSITY;
pub static mut skip_rest_of_patch: bool = false;
pub static mut strippath: libc::c_int = 0;
pub static mut canonicalize_ws: bool = false;
pub static mut patch_get: libc::c_int = 0;
pub static mut set_time: bool = false;
pub static mut set_utc: bool = false;
pub static mut follow_symlinks: bool = false;
pub static mut diff_type: diff = NO_DIFF;
pub static mut revision: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut conflict_style: conflict_style = MERGE_MERGE;
pub static mut last_frozen_line: lin = 0;
pub static mut out_offset: lin = 0;
pub static mut in_offset: lin = 0;
pub static mut no_strip_trailing_cr: bool = false;
#[inline]
unsafe extern "C" fn pch_timestamp(mut which: bool) -> timespec {
    return p_timestamp[which as usize];
}
#[inline]
unsafe extern "C" fn timespec_cmp(mut a: timespec, mut b: timespec) -> libc::c_int {
    if a.tv_sec < b.tv_sec {
        return -(1 as libc::c_int);
    }
    if a.tv_sec > b.tv_sec {
        return 1 as libc::c_int;
    }
    if -(1 as libc::c_int) as libc::c_long <= a.tv_nsec
        && a.tv_nsec
            <= (2 as libc::c_int * TIMESPEC_RESOLUTION as libc::c_int) as libc::c_long
    {} else {
        unreachable!();
    };
    if -(1 as libc::c_int) as libc::c_long <= b.tv_nsec
        && b.tv_nsec
            <= (2 as libc::c_int * TIMESPEC_RESOLUTION as libc::c_int) as libc::c_long
    {} else {
        unreachable!();
    };
    return (a.tv_nsec - b.tv_nsec) as libc::c_int;
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
#[inline]
unsafe extern "C" fn gl_list_nx_create_empty(
    mut implementation: gl_list_implementation_t,
    mut equals_fn: gl_listelement_equals_fn,
    mut hashcode_fn: gl_listelement_hashcode_fn,
    mut dispose_fn: gl_listelement_dispose_fn,
    mut allow_duplicates: bool,
) -> gl_list_t {
    return ((*implementation).nx_create_empty)
        .unwrap()(implementation, equals_fn, hashcode_fn, dispose_fn, allow_duplicates);
}
#[inline]
unsafe extern "C" fn gl_list_size(mut list: gl_list_t) -> size_t {
    return ((*(*(list as *const gl_list_impl_base)).vtable).size).unwrap()(list);
}
#[inline]
unsafe extern "C" fn gl_list_get_at(
    mut list: gl_list_t,
    mut position: size_t,
) -> *const libc::c_void {
    return ((*(*(list as *const gl_list_impl_base)).vtable).get_at)
        .unwrap()(list, position);
}
#[inline]
unsafe extern "C" fn gl_list_nx_add_last(
    mut list: gl_list_t,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    return ((*(*(list as *const gl_list_impl_base)).vtable).nx_add_last)
        .unwrap()(list, elt);
}
#[inline]
unsafe extern "C" fn gl_list_remove_at(
    mut list: gl_list_t,
    mut position: size_t,
) -> bool {
    return ((*(*(list as *const gl_list_impl_base)).vtable).remove_at)
        .unwrap()(list, position);
}
#[inline]
unsafe extern "C" fn gl_list_iterator(mut list: gl_list_t) -> gl_list_iterator_t {
    return ((*(*(list as *const gl_list_impl_base)).vtable).iterator).unwrap()(list);
}
#[inline]
unsafe extern "C" fn gl_list_iterator_next(
    mut iterator: *mut gl_list_iterator_t,
    mut eltp: *mut *const libc::c_void,
    mut nodep: *mut gl_list_node_t,
) -> bool {
    return ((*(*iterator).vtable).iterator_next).unwrap()(iterator, eltp, nodep);
}
#[inline]
unsafe extern "C" fn gl_list_iterator_free(mut iterator: *mut gl_list_iterator_t) {
    ((*(*iterator).vtable).iterator_free).unwrap()(iterator);
}
#[inline]
unsafe extern "C" fn gl_list_create_empty(
    mut implementation: gl_list_implementation_t,
    mut equals_fn: gl_listelement_equals_fn,
    mut hashcode_fn: gl_listelement_hashcode_fn,
    mut dispose_fn: gl_listelement_dispose_fn,
    mut allow_duplicates: bool,
) -> gl_list_t {
    let mut result: gl_list_t = gl_list_nx_create_empty(
        implementation,
        equals_fn,
        hashcode_fn,
        dispose_fn,
        allow_duplicates,
    );
    if result.is_null() {
        xalloc_die();
    }
    return result;
}
#[inline]
unsafe extern "C" fn gl_list_add_last(
    mut list: gl_list_t,
    mut elt: *const libc::c_void,
) -> gl_list_node_t {
    let mut result: gl_list_node_t = gl_list_nx_add_last(list, elt);
    if result.is_null() {
        xalloc_die();
    }
    return result;
}
static mut merge: bool = false;
static mut reject_format: diff = NO_DIFF;
static mut make_backups: bool = false;
static mut backup_if_mismatch: bool = false;
static mut version_control: *const libc::c_char = 0 as *const libc::c_char;
static mut version_control_context: *const libc::c_char = 0 as *const libc::c_char;
static mut remove_empty_files: bool = false;
static mut explicit_inname: bool = false;
static mut read_only_behavior: C2RustUnnamed_2 = RO_WARN;
static mut reverse_flag_specified: bool = false;
static mut do_defines: *const libc::c_char = 0 as *const libc::c_char;
static mut if_defined: [libc::c_char; 12] = unsafe {
    *::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"\n#ifdef %s\n\0")
};
static mut not_defined: [libc::c_char; 13] = unsafe {
    *::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"\n#ifndef %s\n\0")
};
static mut else_defined: [libc::c_char; 8] = unsafe {
    *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"\n#else\n\0")
};
static mut end_defined: [libc::c_char; 9] = unsafe {
    *::std::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"\n#endif\n\0")
};
static mut Argc: libc::c_int = 0;
static mut Argv: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut rejfp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut patchname: *const libc::c_char = 0 as *const libc::c_char;
static mut rejname: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut TMPREJNAME: *const libc::c_char = 0 as *const libc::c_char;
static mut TMPREJNAME_needs_removal: bool = false;
static mut maxfuzz: lin = 2 as libc::c_int as lin;
static mut serrbuf: [libc::c_char; 8192] = [0; 8192];
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut val: *const libc::c_char = 0 as *const libc::c_char;
    let mut somefailed: bool = 0 as libc::c_int != 0;
    let mut outstate: outstate = outstate {
        ofp: 0 as *mut FILE,
        after_newline: false,
        zero_output: false,
    };
    let mut tmpoutst: stat = stat {
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
    let mut numbuf: [libc::c_char; 23] = [0; 23];
    let mut written_to_rejname: bool = 0 as libc::c_int != 0;
    let mut skip_reject_file: bool = 0 as libc::c_int != 0;
    let mut apply_empty_patch: bool = 0 as libc::c_int != 0;
    let mut file_type: mode_t = 0;
    let mut outfd: libc::c_int = -(1 as libc::c_int);
    let mut have_git_diff: bool = 0 as libc::c_int != 0;
    ::std::ptr::write_volatile(&mut exit_failure as *mut libc::c_int, 2 as libc::c_int);
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    init_time();
    setbuf(stderr, serrbuf.as_mut_ptr());
    bufsize = (8 as libc::c_int * 1024 as libc::c_int) as size_t;
    buf = xmalloc(bufsize) as *mut libc::c_char;
    strippath = -(1 as libc::c_int);
    val = getenv(b"QUOTING_STYLE\0" as *const u8 as *const libc::c_char);
    let mut i: libc::c_int = (if !val.is_null() {
        argmatch(
            val,
            quoting_style_args.as_ptr(),
            0 as *const libc::c_char,
            0 as libc::c_int as size_t,
        )
    } else {
        -(1 as libc::c_int) as libc::c_long
    }) as libc::c_int;
    set_quoting_style(
        0 as *mut quoting_options,
        (if i < 0 as libc::c_int {
            shell_quoting_style as libc::c_int as libc::c_uint
        } else {
            i as quoting_style as libc::c_uint
        }) as quoting_style,
    );
    posixly_correct = !(getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char))
        .is_null();
    backup_if_mismatch = !posixly_correct;
    val = getenv(b"PATCH_GET\0" as *const u8 as *const libc::c_char);
    patch_get = if !val.is_null() {
        numeric_string(
            val,
            1 as libc::c_int != 0,
            b"PATCH_GET value\0" as *const u8 as *const libc::c_char,
        )
    } else {
        0 as libc::c_int
    };
    val = getenv(b"SIMPLE_BACKUP_SUFFIX\0" as *const u8 as *const libc::c_char);
    simple_backup_suffix = if !val.is_null() && *val as libc::c_int != 0 {
        val
    } else {
        b".orig\0" as *const u8 as *const libc::c_char
    };
    version_control = getenv(
        b"PATCH_VERSION_CONTROL\0" as *const u8 as *const libc::c_char,
    );
    if !version_control.is_null() {
        version_control_context = b"$PATCH_VERSION_CONTROL\0" as *const u8
            as *const libc::c_char;
    } else {
        version_control = getenv(
            b"VERSION_CONTROL\0" as *const u8 as *const libc::c_char,
        );
        if !version_control.is_null() {
            version_control_context = b"$VERSION_CONTROL\0" as *const u8
                as *const libc::c_char;
        }
    }
    init_backup_hash_table();
    init_files_to_delete();
    init_files_to_output();
    Argc = argc;
    Argv = argv;
    get_some_switches();
    if set_utc {
        setenv(
            b"TZ\0" as *const u8 as *const libc::c_char,
            b"UTC\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    if make_backups as libc::c_int | backup_if_mismatch as libc::c_int != 0 {
        backup_type = get_version(version_control_context, version_control);
    }
    init_output(&mut outstate);
    if !outfile.is_null() {
        outstate.ofp = open_outfile(outfile);
    }
    set_signals(0 as libc::c_int != 0);
    if !inname.is_null() {
        unsafe_0 = 1 as libc::c_int != 0;
    }
    if !inname.is_null() && !outfile.is_null() {
        apply_empty_patch = 1 as libc::c_int != 0;
        file_type = 0o100000 as libc::c_int as mode_t;
        inerrno = -(1 as libc::c_int);
    }
    let mut current_block_291: u64;
    open_patch_file(patchname);
    while there_is_another_patch(
        !(!inname.is_null() || posixly_correct as libc::c_int != 0),
        &mut file_type,
    ) as libc::c_int != 0 || apply_empty_patch as libc::c_int != 0
    {
        let mut hunk: libc::c_int = 0 as libc::c_int;
        let mut failed: libc::c_int = 0 as libc::c_int;
        let mut mismatch: bool = 0 as libc::c_int != 0;
        let mut outname: *const libc::c_char = 0 as *const libc::c_char;
        if skip_rest_of_patch {
            somefailed = 1 as libc::c_int != 0;
        }
        if have_git_diff as libc::c_int != pch_git_diff() as libc::c_int {
            if have_git_diff {
                output_files(0 as *const stat);
                inerrno = -(1 as libc::c_int);
            }
            have_git_diff = !have_git_diff;
        }
        if TMPREJNAME_needs_removal {
            if !rejfp.is_null() {
                fclose(rejfp);
                rejfp = 0 as *mut FILE;
            }
            remove_if_needed(TMPREJNAME, &mut TMPREJNAME_needs_removal);
        }
        if TMPOUTNAME_needs_removal {
            if outfd != -(1 as libc::c_int) {
                close(outfd);
                outfd = -(1 as libc::c_int);
            }
            remove_if_needed(TMPOUTNAME, &mut TMPOUTNAME_needs_removal);
        }
        if !skip_rest_of_patch && file_type == 0 {
            say(
                b"File %s: can't change file type from 0%o to 0%o.\n\0" as *const u8
                    as *const libc::c_char,
                quotearg(inname),
                pch_mode(reverse) & 0o170000 as libc::c_int as libc::c_uint,
                pch_mode(!reverse) & 0o170000 as libc::c_int as libc::c_uint,
            );
            skip_rest_of_patch = 1 as libc::c_int != 0;
            somefailed = 1 as libc::c_int != 0;
        }
        if !skip_rest_of_patch {
            if !outfile.is_null() {
                outname = outfile;
            } else if pch_copy() as libc::c_int != 0 || pch_rename() as libc::c_int != 0
            {
                outname = pch_name(!reverse as libc::c_int as nametype);
            } else {
                outname = inname;
            }
        }
        if pch_git_diff() as libc::c_int != 0 && !skip_rest_of_patch {
            let mut outstat: stat = stat {
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
            let mut outerrno: libc::c_int = 0 as libc::c_int;
            if strcmp(inname, outname) == 0 {
                if inerrno == -(1 as libc::c_int) {
                    inerrno = stat_file(inname, &mut instat);
                }
                outstat = instat;
                outerrno = inerrno;
            } else {
                outerrno = stat_file(outname, &mut outstat);
            }
            if outerrno == 0 {
                if has_queued_output(&mut outstat) {
                    output_files(&mut outstat);
                    outerrno = stat_file(outname, &mut outstat);
                    inerrno = -(1 as libc::c_int);
                }
                if outerrno == 0 {
                    set_queued_output(&mut outstat, 1 as libc::c_int != 0);
                }
            }
        }
        if !skip_rest_of_patch {
            if !get_input_file(inname, outname, file_type) {
                skip_rest_of_patch = 1 as libc::c_int != 0;
                somefailed = 1 as libc::c_int != 0;
            }
        }
        if read_only_behavior as libc::c_uint != RO_IGNORE as libc::c_int as libc::c_uint
            && inerrno == 0
            && !(instat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o120000 as libc::c_int as libc::c_uint)
            && safe_access(inname, 2 as libc::c_int) != 0 as libc::c_int
        {
            say(
                b"File %s is read-only; \0" as *const u8 as *const libc::c_char,
                quotearg(inname),
            );
            if read_only_behavior as libc::c_uint
                == RO_WARN as libc::c_int as libc::c_uint
            {
                say(b"trying to patch anyway\n\0" as *const u8 as *const libc::c_char);
            } else {
                say(b"refusing to patch\n\0" as *const u8 as *const libc::c_char);
                skip_rest_of_patch = 1 as libc::c_int != 0;
                somefailed = 1 as libc::c_int != 0;
            }
        }
        tmpoutst.st_size = -(1 as libc::c_int) as __off_t;
        outfd = make_tempfile(
            &mut TMPOUTNAME,
            'o' as i32 as libc::c_char,
            outname,
            0o1 as libc::c_int | 0 as libc::c_int,
            instat.st_mode
                & (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
                    | (0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o100 as libc::c_int) >> 3 as libc::c_int
                    | (0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o100 as libc::c_int) >> 3 as libc::c_int >> 3 as libc::c_int)
                    as libc::c_uint,
        );
        if outfd == -(1 as libc::c_int) {
            if *__errno_location() == 40 as libc::c_int
                || *__errno_location() == 18 as libc::c_int
            {
                say(
                    b"Invalid file name %s -- skipping patch\n\0" as *const u8
                        as *const libc::c_char,
                    quotearg(outname),
                );
                skip_rest_of_patch = 1 as libc::c_int != 0;
                skip_reject_file = 1 as libc::c_int != 0;
                somefailed = 1 as libc::c_int != 0;
            } else {
                pfatal(
                    b"Can't create temporary file %s\0" as *const u8
                        as *const libc::c_char,
                    TMPOUTNAME,
                );
            }
        } else {
            TMPOUTNAME_needs_removal = 1 as libc::c_int != 0;
        }
        if diff_type as libc::c_uint == ED_DIFF as libc::c_int as libc::c_uint {
            outstate.zero_output = 0 as libc::c_int != 0;
            somefailed = (somefailed as libc::c_int | skip_rest_of_patch as libc::c_int) != 0;
            do_ed_script(
                inname,
                TMPOUTNAME,
                &mut TMPOUTNAME_needs_removal,
                outstate.ofp,
            );
            if !dry_run && outfile.is_null() && !skip_rest_of_patch {
                if fstat(outfd, &mut tmpoutst) != 0 as libc::c_int {
                    pfatal(b"%s\0" as *const u8 as *const libc::c_char, TMPOUTNAME);
                }
                outstate
                    .zero_output = tmpoutst.st_size == 0 as libc::c_int as libc::c_long;
            }
            close(outfd);
            outfd = -(1 as libc::c_int);
            current_block_291 = 16708048892964637133;
        } else {
            let mut got_hunk: libc::c_int = 0;
            let mut apply_anyway: bool = merge;
            if !skip_rest_of_patch
                && diff_type as libc::c_uint
                    == GIT_BINARY_DIFF as libc::c_int as libc::c_uint
            {
                say(
                    b"File %s: git binary diffs are not supported.\n\0" as *const u8
                        as *const libc::c_char,
                    quotearg(outname),
                );
                skip_rest_of_patch = 1 as libc::c_int != 0;
                somefailed = 1 as libc::c_int != 0;
            }
            if !skip_rest_of_patch && outfile.is_null() {
                init_output(&mut outstate);
                outstate
                    .ofp = fdopen(
                    outfd,
                    if 0 as libc::c_int != 0 {
                        b"wb\0" as *const u8 as *const libc::c_char
                    } else {
                        b"w\0" as *const u8 as *const libc::c_char
                    },
                );
                if (outstate.ofp).is_null() {
                    pfatal(b"%s\0" as *const u8 as *const libc::c_char, TMPOUTNAME);
                }
                outfd = -(1 as libc::c_int);
            }
            if !skip_rest_of_patch {
                scan_input(inname, file_type);
                if verbosity as libc::c_uint != SILENT as libc::c_int as libc::c_uint {
                    let mut renamed: bool = strcmp(inname, outname) != 0;
                    let mut skip_rename: bool = !renamed
                        && pch_rename() as libc::c_int != 0;
                    say(
                        b"%s %s %s%c\0" as *const u8 as *const libc::c_char,
                        if dry_run as libc::c_int != 0 {
                            b"checking\0" as *const u8 as *const libc::c_char
                        } else {
                            b"patching\0" as *const u8 as *const libc::c_char
                        },
                        if file_type & 0o170000 as libc::c_int as libc::c_uint
                            == 0o120000 as libc::c_int as libc::c_uint
                        {
                            b"symbolic link\0" as *const u8 as *const libc::c_char
                        } else {
                            b"file\0" as *const u8 as *const libc::c_char
                        },
                        quotearg(outname),
                        if renamed as libc::c_int != 0 || skip_rename as libc::c_int != 0
                        {
                            ' ' as i32
                        } else {
                            '\n' as i32
                        },
                    );
                    if renamed as libc::c_int != 0 || skip_rename as libc::c_int != 0 {
                        say(
                            b"(%s%s from %s)\n\0" as *const u8 as *const libc::c_char,
                            if skip_rename as libc::c_int != 0 {
                                b"already \0" as *const u8 as *const libc::c_char
                            } else {
                                b"\0" as *const u8 as *const libc::c_char
                            },
                            if pch_copy() as libc::c_int != 0 {
                                b"copied\0" as *const u8 as *const libc::c_char
                            } else if pch_rename() as libc::c_int != 0 {
                                b"renamed\0" as *const u8 as *const libc::c_char
                            } else {
                                b"read\0" as *const u8 as *const libc::c_char
                            },
                            if !skip_rename {
                                inname as *const libc::c_char
                            } else {
                                pch_name(
                                    (strcmp(inname, pch_name(OLD)) == 0) as libc::c_int
                                        as nametype,
                                )
                            },
                        );
                    }
                    if verbosity as libc::c_uint
                        == VERBOSE as libc::c_int as libc::c_uint
                    {
                        say(
                            b"Using Plan %s...\n\0" as *const u8 as *const libc::c_char,
                            if using_plan_a as libc::c_int != 0 {
                                b"A\0" as *const u8 as *const libc::c_char
                            } else {
                                b"B\0" as *const u8 as *const libc::c_char
                            },
                        );
                    }
                }
            }
            loop {
                got_hunk = another_hunk(diff_type, reverse);
                if !((0 as libc::c_int) < got_hunk) {
                    break;
                }
                let mut where_0: lin = 0 as libc::c_int as lin;
                let mut newwhere: lin = 0;
                let mut fuzz: lin = 0 as libc::c_int as lin;
                let mut mymaxfuzz: lin = 0;
                if merge {
                    mymaxfuzz = 0 as libc::c_int as lin;
                } else {
                    let mut prefix_context: lin = pch_prefix_context();
                    let mut suffix_context: lin = pch_suffix_context();
                    let mut context: lin = if prefix_context < suffix_context {
                        suffix_context
                    } else {
                        prefix_context
                    };
                    mymaxfuzz = if maxfuzz < context { maxfuzz } else { context };
                }
                hunk += 1;
                hunk;
                if !skip_rest_of_patch {
                    loop {
                        where_0 = locate_hunk(fuzz);
                        if where_0 == 0 || fuzz != 0 || in_offset != 0 {
                            mismatch = 1 as libc::c_int != 0;
                        }
                        if hunk == 1 as libc::c_int && where_0 == 0
                            && force as libc::c_int | apply_anyway as libc::c_int == 0
                            && reverse as libc::c_int
                                == reverse_flag_specified as libc::c_int
                        {
                            if !pch_swap() {
                                say(
                                    b"Not enough memory to try swapped hunk!  Assuming unswapped.\n\0"
                                        as *const u8 as *const libc::c_char,
                                );
                            } else {
                                where_0 = locate_hunk(fuzz);
                                if where_0 != 0
                                    && ok_to_reverse(
                                        b"%s patch detected!\0" as *const u8 as *const libc::c_char,
                                        if reverse as libc::c_int != 0 {
                                            b"Unreversed\0" as *const u8 as *const libc::c_char
                                        } else {
                                            b"Reversed (or previously applied)\0" as *const u8
                                                as *const libc::c_char
                                        },
                                    ) as libc::c_int != 0
                                {
                                    reverse = !reverse;
                                } else {
                                    if !pch_swap() {
                                        fatal(
                                            b"lost hunk on alloc error!\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                    }
                                    if where_0 != 0 {
                                        apply_anyway = 1 as libc::c_int != 0;
                                        fuzz -= 1;
                                        fuzz;
                                        where_0 = 0 as libc::c_int as lin;
                                    }
                                }
                            }
                        }
                        if !(!skip_rest_of_patch && where_0 == 0
                            && {
                                fuzz += 1;
                                fuzz <= mymaxfuzz
                            })
                        {
                            break;
                        }
                    }
                    if skip_rest_of_patch {
                        if !(outstate.ofp).is_null() && outfile.is_null() {
                            fclose(outstate.ofp);
                            outstate.ofp = 0 as *mut FILE;
                            outfd = -(1 as libc::c_int);
                        }
                    }
                }
                newwhere = (if where_0 != 0 { where_0 } else { pch_first() })
                    + out_offset;
                if skip_rest_of_patch as libc::c_int != 0
                    || merge as libc::c_int != 0
                        && !merge_hunk(hunk, &mut outstate, where_0, &mut somefailed)
                    || !merge
                        && (where_0 == 1 as libc::c_int as libc::c_long
                            && pch_says_nonexistent(reverse) == 2 as libc::c_int
                            && instat.st_size != 0 || where_0 == 0
                            || !apply_hunk(&mut outstate, where_0))
                {
                    if !skip_reject_file {
                        abort_hunk(outname, failed == 0, reverse);
                    }
                    failed += 1;
                    failed;
                    if verbosity as libc::c_uint
                        == VERBOSE as libc::c_int as libc::c_uint
                        || !skip_rest_of_patch
                            && verbosity as libc::c_uint
                                != SILENT as libc::c_int as libc::c_uint
                    {
                        say(
                            b"Hunk #%d %s at %s%s.\n\0" as *const u8
                                as *const libc::c_char,
                            hunk,
                            if skip_rest_of_patch as libc::c_int != 0 {
                                b"ignored\0" as *const u8 as *const libc::c_char
                            } else {
                                b"FAILED\0" as *const u8 as *const libc::c_char
                            },
                            format_linenum(numbuf.as_mut_ptr(), newwhere),
                            if !skip_rest_of_patch
                                && check_line_endings(newwhere) as libc::c_int != 0
                            {
                                b" (different line endings)\0" as *const u8
                                    as *const libc::c_char
                            } else {
                                b"\0" as *const u8 as *const libc::c_char
                            },
                        );
                    }
                } else if !merge
                    && (verbosity as libc::c_uint
                        == VERBOSE as libc::c_int as libc::c_uint
                        || verbosity as libc::c_uint
                            != SILENT as libc::c_int as libc::c_uint
                            && (fuzz != 0 || in_offset != 0))
                {
                    say(
                        b"Hunk #%d succeeded at %s\0" as *const u8
                            as *const libc::c_char,
                        hunk,
                        format_linenum(numbuf.as_mut_ptr(), newwhere),
                    );
                    if fuzz != 0 {
                        say(
                            b" with fuzz %s\0" as *const u8 as *const libc::c_char,
                            format_linenum(numbuf.as_mut_ptr(), fuzz),
                        );
                    }
                    if in_offset != 0 {
                        say(
                            b" (offset %s line%s)\0" as *const u8 as *const libc::c_char,
                            format_linenum(numbuf.as_mut_ptr(), in_offset),
                            (b"s\0" as *const u8 as *const libc::c_char)
                                .offset(
                                    (in_offset == 1 as libc::c_int as libc::c_long)
                                        as libc::c_int as isize,
                                ),
                        );
                    }
                    say(b".\n\0" as *const u8 as *const libc::c_char);
                }
            }
            if !skip_rest_of_patch {
                if got_hunk < 0 as libc::c_int && using_plan_a as libc::c_int != 0 {
                    if !outfile.is_null() {
                        fatal(
                            b"out of memory using Plan A\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    say(
                        b"\n\nRan out of memory using Plan A -- trying again...\n\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    if !(outstate.ofp).is_null() {
                        fclose(outstate.ofp);
                        outstate.ofp = 0 as *mut FILE;
                    }
                    current_block_291 = 1538046216550696469;
                } else {
                    if !spew_output(&mut outstate, &mut tmpoutst) {
                        say(b"Skipping patch.\n\0" as *const u8 as *const libc::c_char);
                        skip_rest_of_patch = 1 as libc::c_int != 0;
                    }
                    current_block_291 = 16708048892964637133;
                }
            } else {
                current_block_291 = 16708048892964637133;
            }
        }
        match current_block_291 {
            16708048892964637133 => {
                ignore_signals();
                if !skip_rest_of_patch && outfile.is_null() {
                    let mut backup: bool = make_backups as libc::c_int != 0
                        || backup_if_mismatch as libc::c_int != 0
                            && mismatch as libc::c_int | failed != 0;
                    if outstate.zero_output as libc::c_int != 0
                        && (remove_empty_files as libc::c_int != 0
                            || pch_says_nonexistent(!reverse) == 2 as libc::c_int
                                && !posixly_correct
                            || file_type & 0o170000 as libc::c_int as libc::c_uint
                                == 0o120000 as libc::c_int as libc::c_uint)
                    {
                        if !dry_run {
                            output_file(
                                0 as *const libc::c_char,
                                0 as *mut bool,
                                0 as *const stat,
                                outname,
                                if inname == outname as *mut libc::c_char {
                                    &mut instat
                                } else {
                                    0 as *mut stat
                                },
                                file_type | 0 as libc::c_int as libc::c_uint,
                                backup,
                            );
                        }
                    } else {
                        if !outstate.zero_output
                            && pch_says_nonexistent(!reverse) == 2 as libc::c_int
                            && (remove_empty_files as libc::c_int != 0
                                || !posixly_correct)
                            && !(merge as libc::c_int != 0
                                && somefailed as libc::c_int != 0)
                        {
                            mismatch = 1 as libc::c_int != 0;
                            somefailed = 1 as libc::c_int != 0;
                            if verbosity as libc::c_uint
                                != SILENT as libc::c_int as libc::c_uint
                            {
                                say(
                                    b"Not deleting file %s as content differs from patch\n\0"
                                        as *const u8 as *const libc::c_char,
                                    quotearg(outname),
                                );
                            }
                        }
                        if !dry_run {
                            let mut old_mode: mode_t = pch_mode(reverse);
                            let mut new_mode: mode_t = pch_mode(!reverse);
                            let mut set_mode: bool = new_mode != 0
                                && old_mode != new_mode;
                            if failed < hunk
                                || diff_type as libc::c_uint
                                    == ED_DIFF as libc::c_int as libc::c_uint
                                || set_mode as libc::c_int != 0
                                || pch_copy() as libc::c_int != 0
                                || pch_rename() as libc::c_int != 0
                            {
                                let mut attr: file_attributes = 0 as file_attributes;
                                let mut new_time: timespec = pch_timestamp(!reverse);
                                let mut mode: mode_t = file_type
                                    | (if set_mode as libc::c_int != 0 {
                                        new_mode
                                    } else {
                                        instat.st_mode
                                    })
                                        & (0o400 as libc::c_int | 0o200 as libc::c_int
                                            | 0o100 as libc::c_int
                                            | (0o400 as libc::c_int | 0o200 as libc::c_int
                                                | 0o100 as libc::c_int) >> 3 as libc::c_int
                                            | (0o400 as libc::c_int | 0o200 as libc::c_int
                                                | 0o100 as libc::c_int) >> 3 as libc::c_int
                                                >> 3 as libc::c_int) as libc::c_uint;
                                if set_time as libc::c_int | set_utc as libc::c_int != 0
                                    && new_time.tv_sec != -(1 as libc::c_int) as libc::c_long
                                {
                                    let mut old_time: timespec = pch_timestamp(reverse);
                                    if !force && inerrno == 0
                                        && pch_says_nonexistent(reverse) != 2 as libc::c_int
                                        && old_time.tv_sec != -(1 as libc::c_int) as libc::c_long
                                        && timespec_cmp(old_time, get_stat_mtime(&mut instat)) != 0
                                    {
                                        say(
                                            b"Not setting time of file %s (time mismatch)\n\0"
                                                as *const u8 as *const libc::c_char,
                                            quotearg(outname),
                                        );
                                    } else if !force && mismatch as libc::c_int | failed != 0 {
                                        say(
                                            b"Not setting time of file %s (contents mismatch)\n\0"
                                                as *const u8 as *const libc::c_char,
                                            quotearg(outname),
                                        );
                                    } else {
                                        attr = ::std::mem::transmute::<
                                            libc::c_uint,
                                            file_attributes,
                                        >(
                                            attr as libc::c_uint
                                                | FA_TIMES as libc::c_int as libc::c_uint,
                                        );
                                    }
                                }
                                if inerrno != 0 {
                                    if set_mode {
                                        attr = ::std::mem::transmute::<
                                            libc::c_uint,
                                            file_attributes,
                                        >(
                                            attr as libc::c_uint
                                                | FA_MODE as libc::c_int as libc::c_uint,
                                        );
                                    }
                                    set_file_attributes(
                                        TMPOUTNAME,
                                        attr,
                                        0 as *const libc::c_char,
                                        0 as *const stat,
                                        mode,
                                        &mut new_time,
                                    );
                                } else {
                                    attr = ::std::mem::transmute::<
                                        libc::c_uint,
                                        file_attributes,
                                    >(
                                        attr as libc::c_uint
                                            | (FA_IDS as libc::c_int | FA_MODE as libc::c_int
                                                | FA_XATTRS as libc::c_int) as libc::c_uint,
                                    );
                                    set_file_attributes(
                                        TMPOUTNAME,
                                        attr,
                                        inname,
                                        &mut instat,
                                        mode,
                                        &mut new_time,
                                    );
                                }
                                output_file(
                                    TMPOUTNAME,
                                    &mut TMPOUTNAME_needs_removal,
                                    &mut tmpoutst,
                                    outname,
                                    0 as *const stat,
                                    mode,
                                    backup,
                                );
                                if pch_rename() {
                                    output_file(
                                        0 as *const libc::c_char,
                                        0 as *mut bool,
                                        0 as *const stat,
                                        inname,
                                        &mut instat,
                                        mode,
                                        backup,
                                    );
                                }
                            } else {
                                output_file(
                                    outname,
                                    0 as *mut bool,
                                    &mut tmpoutst,
                                    0 as *const libc::c_char,
                                    0 as *const stat,
                                    file_type | 0 as libc::c_int as libc::c_uint,
                                    backup,
                                );
                            }
                        }
                    }
                }
                if diff_type as libc::c_uint != ED_DIFF as libc::c_int as libc::c_uint {
                    let mut rejst: stat = stat {
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
                    if failed != 0 && !skip_reject_file {
                        if fstat(fileno(rejfp), &mut rejst) != 0 as libc::c_int
                            || fclose(rejfp) != 0 as libc::c_int
                        {
                            write_fatal();
                        }
                        rejfp = 0 as *mut FILE;
                        somefailed = 1 as libc::c_int != 0;
                        say(
                            b"%d out of %d hunk%s %s\0" as *const u8
                                as *const libc::c_char,
                            failed,
                            hunk,
                            (b"s\0" as *const u8 as *const libc::c_char)
                                .offset((hunk == 1 as libc::c_int) as libc::c_int as isize),
                            if skip_rest_of_patch as libc::c_int != 0 {
                                b"ignored\0" as *const u8 as *const libc::c_char
                            } else {
                                b"FAILED\0" as *const u8 as *const libc::c_char
                            },
                        );
                        if !outname.is_null()
                            && (rejname.is_null()
                                || strcmp(
                                    rejname,
                                    b"-\0" as *const u8 as *const libc::c_char,
                                ) != 0 as libc::c_int)
                        {
                            let mut rej: *mut libc::c_char = rejname;
                            if rejname.is_null() {
                                let mut s: *const libc::c_char = simple_backup_suffix;
                                let mut len: size_t = 0;
                                simple_backup_suffix = b".rej\0" as *const u8
                                    as *const libc::c_char;
                                rej = find_backup_file_name(outname, simple_backups);
                                len = strlen(rej);
                                if *rej
                                    .offset(
                                        len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                    ) as libc::c_int == '~' as i32
                                {
                                    *rej
                                        .offset(
                                            len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) = '#' as i32 as libc::c_char;
                                }
                                simple_backup_suffix = s;
                            }
                            if !dry_run {
                                say(
                                    b" -- saving rejects to file %s\n\0" as *const u8
                                        as *const libc::c_char,
                                    quotearg(rej),
                                );
                                if !rejname.is_null() {
                                    if !written_to_rejname {
                                        copy_file(
                                            TMPREJNAME,
                                            rejname,
                                            0 as *mut stat,
                                            0 as libc::c_int,
                                            (0o100000 as libc::c_int | 0o666 as libc::c_int) as mode_t,
                                            1 as libc::c_int != 0,
                                        );
                                        written_to_rejname = 1 as libc::c_int != 0;
                                    } else {
                                        append_to_file(TMPREJNAME, rejname);
                                    }
                                } else {
                                    let mut oldst: stat = stat {
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
                                    let mut olderrno: libc::c_int = 0;
                                    olderrno = stat_file(rej, &mut oldst);
                                    if olderrno != 0 && olderrno != 2 as libc::c_int {
                                        write_fatal();
                                    }
                                    if olderrno == 0
                                        && lookup_file_id(&mut oldst) as libc::c_uint
                                            == CREATED as libc::c_int as libc::c_uint
                                    {
                                        append_to_file(TMPREJNAME, rej);
                                    } else {
                                        move_file(
                                            TMPREJNAME,
                                            &mut TMPREJNAME_needs_removal,
                                            &mut rejst,
                                            rej,
                                            (0o100000 as libc::c_int | 0o666 as libc::c_int) as mode_t,
                                            0 as libc::c_int != 0,
                                        );
                                    }
                                }
                            } else {
                                say(b"\n\0" as *const u8 as *const libc::c_char);
                            }
                            if rejname.is_null() {
                                free(rej as *mut libc::c_void);
                            }
                        } else {
                            say(b"\n\0" as *const u8 as *const libc::c_char);
                        }
                    }
                }
                set_signals(1 as libc::c_int != 0);
            }
            _ => {}
        }
        reinitialize_almost_everything();
        skip_reject_file = 0 as libc::c_int != 0;
        apply_empty_patch = 0 as libc::c_int != 0;
    }
    if !(outstate.ofp).is_null()
        && (ferror(outstate.ofp) != 0 || fclose(outstate.ofp) != 0 as libc::c_int)
    {
        write_fatal();
    }
    output_files(0 as *const stat);
    cleanup();
    delete_files();
    if somefailed {
        exit(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn reinitialize_almost_everything() {
    re_patch();
    re_input();
    input_lines = 0 as libc::c_int as lin;
    last_frozen_line = 0 as libc::c_int as lin;
    if !inname.is_null() && !explicit_inname {
        free(inname as *mut libc::c_void);
        inname = 0 as *mut libc::c_char;
    }
    in_offset = 0 as libc::c_int as lin;
    out_offset = 0 as libc::c_int as lin;
    diff_type = NO_DIFF;
    if !revision.is_null() {
        free(revision as *mut libc::c_void);
        revision = 0 as *mut libc::c_char;
    }
    reverse = reverse_flag_specified;
    skip_rest_of_patch = 0 as libc::c_int != 0;
}
static mut shortopts: [libc::c_char; 42] = unsafe {
    *::std::mem::transmute::<
        &[u8; 42],
        &[libc::c_char; 42],
    >(b"bB:cd:D:eEfF:g:i:lnNo:p:r:RstTuvV:x:Y:z:Z\0")
};
static mut longopts: [option; 42] = [
    {
        let mut init = option {
            name: b"backup\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"prefix\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'B' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"context\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"directory\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ifdef\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'D' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ed\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'e' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"remove-empty-files\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'E' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"force\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"fuzz\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'F' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"get\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'g' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"input\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-whitespace\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"merge\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'm' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"normal\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"forward\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'N' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"output\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"strip\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"reject-file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"reverse\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'R' as i32,
        };
        init
    },
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
            name: b"silent\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"batch\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"set-time\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'T' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"unified\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'u' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version-control\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'V' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"debug\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'x' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"basename-prefix\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'Y' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"suffix\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'z' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"set-utc\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'Z' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"dry-run\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 127 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 127 as libc::c_int + 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"binary\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 127 as libc::c_int + 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 127 as libc::c_int + 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"backup-if-mismatch\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 127 as libc::c_int + 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-backup-if-mismatch\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 127 as libc::c_int + 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"posix\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 127 as libc::c_int + 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"quoting-style\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 127 as libc::c_int + 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"reject-format\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 127 as libc::c_int + 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"read-only\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 127 as libc::c_int + 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"follow-symlinks\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 127 as libc::c_int + 11 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
static mut option_help: [*const libc::c_char; 66] = [
    b"Input options:\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"  -p NUM  --strip=NUM  Strip NUM leading components from file names.\0"
        as *const u8 as *const libc::c_char,
    b"  -F LINES  --fuzz LINES  Set the fuzz factor to LINES for inexact matching.\0"
        as *const u8 as *const libc::c_char,
    b"  -l  --ignore-whitespace  Ignore white space changes between patch and input.\0"
        as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"  -c  --context  Interpret the patch as a context difference.\0" as *const u8
        as *const libc::c_char,
    b"  -e  --ed  Interpret the patch as an ed script.\0" as *const u8
        as *const libc::c_char,
    b"  -n  --normal  Interpret the patch as a normal difference.\0" as *const u8
        as *const libc::c_char,
    b"  -u  --unified  Interpret the patch as a unified difference.\0" as *const u8
        as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"  -N  --forward  Ignore patches that appear to be reversed or already applied.\0"
        as *const u8 as *const libc::c_char,
    b"  -R  --reverse  Assume patches were created with old and new files swapped.\0"
        as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"  -i PATCHFILE  --input=PATCHFILE  Read patch from PATCHFILE instead of stdin.\0"
        as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"Output options:\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"  -o FILE  --output=FILE  Output patched files to FILE.\0" as *const u8
        as *const libc::c_char,
    b"  -r FILE  --reject-file=FILE  Output rejects to FILE.\0" as *const u8
        as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"  -D NAME  --ifdef=NAME  Make merged if-then-else output using NAME.\0"
        as *const u8 as *const libc::c_char,
    b"  --merge  Merge using conflict markers instead of creating reject files.\0"
        as *const u8 as *const libc::c_char,
    b"  -E  --remove-empty-files  Remove output files that are empty after patching.\0"
        as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"  -Z  --set-utc  Set times of patched files, assuming diff uses UTC (GMT).\0"
        as *const u8 as *const libc::c_char,
    b"  -T  --set-time  Likewise, assuming local time.\0" as *const u8
        as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"  --quoting-style=WORD   output file names using quoting style WORD.\0"
        as *const u8 as *const libc::c_char,
    b"    Valid WORDs are: literal, shell, shell-always, c, escape.\0" as *const u8
        as *const libc::c_char,
    b"    Default is taken from QUOTING_STYLE env variable, or 'shell' if unset.\0"
        as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"Backup and version control options:\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"  -b  --backup  Back up the original contents of each file.\0" as *const u8
        as *const libc::c_char,
    b"  --backup-if-mismatch  Back up if the patch does not match exactly.\0"
        as *const u8 as *const libc::c_char,
    b"  --no-backup-if-mismatch  Back up mismatches only if otherwise requested.\0"
        as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"  -V STYLE  --version-control=STYLE  Use STYLE version control.\0" as *const u8
        as *const libc::c_char,
    b"\tSTYLE is either 'simple', 'numbered', or 'existing'.\0" as *const u8
        as *const libc::c_char,
    b"  -B PREFIX  --prefix=PREFIX  Prepend PREFIX to backup file names.\0" as *const u8
        as *const libc::c_char,
    b"  -Y PREFIX  --basename-prefix=PREFIX  Prepend PREFIX to backup file basenames.\0"
        as *const u8 as *const libc::c_char,
    b"  -z SUFFIX  --suffix=SUFFIX  Append SUFFIX to backup file names.\0" as *const u8
        as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"  -g NUM  --get=NUM  Get files from RCS etc. if positive; ask if negative.\0"
        as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"Miscellaneous options:\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"  -t  --batch  Ask no questions; skip bad-Prereq patches; assume reversed.\0"
        as *const u8 as *const libc::c_char,
    b"  -f  --force  Like -t, but ignore bad-Prereq patches, and assume unreversed.\0"
        as *const u8 as *const libc::c_char,
    b"  -s  --quiet  --silent  Work silently unless an error occurs.\0" as *const u8
        as *const libc::c_char,
    b"  --verbose  Output extra information about the work being done.\0" as *const u8
        as *const libc::c_char,
    b"  --dry-run  Do not actually change any files; just print what would happen.\0"
        as *const u8 as *const libc::c_char,
    b"  --posix  Conform to the POSIX standard.\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"  -d DIR  --directory=DIR  Change the working directory to DIR first.\0"
        as *const u8 as *const libc::c_char,
    b"  --reject-format=FORMAT  Create 'context' or 'unified' rejects.\0" as *const u8
        as *const libc::c_char,
    b"  --binary  Read and write data in binary mode.\0" as *const u8
        as *const libc::c_char,
    b"  --read-only=BEHAVIOR  How to handle read-only input files: 'ignore' that they\0"
        as *const u8 as *const libc::c_char,
    b"                        are read-only, 'warn' (default), or 'fail'.\0" as *const u8
        as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"  -v  --version  Output version info.\0" as *const u8 as *const libc::c_char,
    b"  --help  Output this help.\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"Report bugs to <bug-patch@gnu.org>.\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
unsafe extern "C" fn usage(mut stream: *mut FILE, mut status: libc::c_int) -> ! {
    let mut p: *const *const libc::c_char = 0 as *const *const libc::c_char;
    if status != 0 as libc::c_int {
        fprintf(
            stream,
            b"%s: Try '%s --help' for more information.\n\0" as *const u8
                as *const libc::c_char,
            program_name,
            *Argv.offset(0 as libc::c_int as isize),
        );
    } else {
        fprintf(
            stream,
            b"Usage: %s [OPTION]... [ORIGFILE [PATCHFILE]]\n\n\0" as *const u8
                as *const libc::c_char,
            *Argv.offset(0 as libc::c_int as isize),
        );
        p = option_help.as_ptr();
        while !(*p).is_null() {
            fprintf(stream, b"%s\n\0" as *const u8 as *const libc::c_char, *p);
            p = p.offset(1);
            p;
        }
    }
    exit(status);
}
unsafe extern "C" fn get_some_switches() {
    let mut optc: libc::c_int = 0;
    free(rejname as *mut libc::c_void);
    rejname = 0 as *mut libc::c_char;
    if optind == Argc {
        return;
    }
    loop {
        optc = getopt_long(
            Argc,
            Argv,
            shortopts.as_ptr(),
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        let mut current_block_80: u64;
        match optc {
            98 => {
                make_backups = 1 as libc::c_int != 0;
                if Argc - optind == 3 as libc::c_int
                    && strcmp(
                        *Argv.offset((optind - 1 as libc::c_int) as isize),
                        b"-b\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    && !(*(*Argv.offset((optind + 0 as libc::c_int) as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
                        && *(*Argv.offset((optind + 0 as libc::c_int) as isize))
                            .offset(1 as libc::c_int as isize) as libc::c_int != 0)
                    && !(*(*Argv.offset((optind + 1 as libc::c_int) as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
                        && *(*Argv.offset((optind + 1 as libc::c_int) as isize))
                            .offset(1 as libc::c_int as isize) as libc::c_int != 0)
                    && !(*(*Argv.offset((optind + 2 as libc::c_int) as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
                        && *(*Argv.offset((optind + 2 as libc::c_int) as isize))
                            .offset(1 as libc::c_int as isize) as libc::c_int != 0)
                {
                    let fresh0 = optind;
                    optind = optind + 1;
                    optarg = *Argv.offset(fresh0 as isize);
                    if verbosity as libc::c_uint != SILENT as libc::c_int as libc::c_uint
                    {
                        say(
                            b"warning: the '-b %s' option is obsolete; use '-b -z %s' instead\n\0"
                                as *const u8 as *const libc::c_char,
                            optarg,
                            optarg,
                        );
                    }
                    current_block_80 = 11656077632963965015;
                } else {
                    current_block_80 = 14541395414537699361;
                }
            }
            66 => {
                if *optarg == 0 {
                    fatal(
                        b"backup prefix is empty\0" as *const u8 as *const libc::c_char,
                    );
                }
                origprae = xstrdup(optarg);
                current_block_80 = 14541395414537699361;
            }
            99 => {
                diff_type = CONTEXT_DIFF;
                current_block_80 = 14541395414537699361;
            }
            100 => {
                if chdir(optarg) < 0 as libc::c_int {
                    pfatal(
                        b"Can't change to directory %s\0" as *const u8
                            as *const libc::c_char,
                        quotearg(optarg),
                    );
                }
                current_block_80 = 14541395414537699361;
            }
            68 => {
                do_defines = xstrdup(optarg);
                current_block_80 = 14541395414537699361;
            }
            101 => {
                diff_type = ED_DIFF;
                current_block_80 = 14541395414537699361;
            }
            69 => {
                remove_empty_files = 1 as libc::c_int != 0;
                current_block_80 = 14541395414537699361;
            }
            102 => {
                force = 1 as libc::c_int != 0;
                current_block_80 = 14541395414537699361;
            }
            70 => {
                maxfuzz = numeric_string(
                    optarg,
                    0 as libc::c_int != 0,
                    b"fuzz factor\0" as *const u8 as *const libc::c_char,
                ) as lin;
                current_block_80 = 14541395414537699361;
            }
            103 => {
                patch_get = numeric_string(
                    optarg,
                    1 as libc::c_int != 0,
                    b"get option value\0" as *const u8 as *const libc::c_char,
                );
                current_block_80 = 14541395414537699361;
            }
            105 => {
                patchname = xstrdup(optarg);
                current_block_80 = 14541395414537699361;
            }
            108 => {
                canonicalize_ws = 1 as libc::c_int != 0;
                current_block_80 = 14541395414537699361;
            }
            109 => {
                merge = 1 as libc::c_int != 0;
                if !optarg.is_null() {
                    if strcmp(optarg, b"merge\0" as *const u8 as *const libc::c_char)
                        == 0
                    {
                        conflict_style = MERGE_MERGE;
                    } else if strcmp(
                        optarg,
                        b"diff3\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        conflict_style = MERGE_DIFF3;
                    } else {
                        usage(stderr, 2 as libc::c_int);
                    }
                } else {
                    conflict_style = MERGE_MERGE;
                }
                current_block_80 = 14541395414537699361;
            }
            110 => {
                diff_type = NORMAL_DIFF;
                current_block_80 = 14541395414537699361;
            }
            78 => {
                noreverse = 1 as libc::c_int != 0;
                current_block_80 = 14541395414537699361;
            }
            111 => {
                outfile = xstrdup(optarg);
                current_block_80 = 14541395414537699361;
            }
            112 => {
                strippath = numeric_string(
                    optarg,
                    0 as libc::c_int != 0,
                    b"strip count\0" as *const u8 as *const libc::c_char,
                );
                current_block_80 = 14541395414537699361;
            }
            114 => {
                rejname = xstrdup(optarg);
                current_block_80 = 14541395414537699361;
            }
            82 => {
                reverse = 1 as libc::c_int != 0;
                reverse_flag_specified = 1 as libc::c_int != 0;
                current_block_80 = 14541395414537699361;
            }
            115 => {
                verbosity = SILENT;
                current_block_80 = 14541395414537699361;
            }
            116 => {
                batch = 1 as libc::c_int != 0;
                current_block_80 = 14541395414537699361;
            }
            84 => {
                set_time = 1 as libc::c_int != 0;
                current_block_80 = 14541395414537699361;
            }
            117 => {
                diff_type = UNI_DIFF;
                current_block_80 = 14541395414537699361;
            }
            118 => {
                version();
                exit(0 as libc::c_int);
            }
            86 => {
                version_control = optarg;
                version_control_context = b"--version-control or -V option\0"
                    as *const u8 as *const libc::c_char;
                current_block_80 = 14541395414537699361;
            }
            120 => {
                debug = numeric_string(
                    optarg,
                    1 as libc::c_int != 0,
                    b"debugging option\0" as *const u8 as *const libc::c_char,
                );
                current_block_80 = 14541395414537699361;
            }
            89 => {
                if *optarg == 0 {
                    fatal(
                        b"backup basename prefix is empty\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                origbase = xstrdup(optarg);
                current_block_80 = 14541395414537699361;
            }
            122 => {
                current_block_80 = 11656077632963965015;
            }
            90 => {
                set_utc = 1 as libc::c_int != 0;
                current_block_80 = 14541395414537699361;
            }
            128 => {
                dry_run = 1 as libc::c_int != 0;
                current_block_80 = 14541395414537699361;
            }
            129 => {
                verbosity = VERBOSE;
                current_block_80 = 14541395414537699361;
            }
            130 => {
                no_strip_trailing_cr = 1 as libc::c_int != 0;
                current_block_80 = 14541395414537699361;
            }
            131 => {
                usage(stdout, 0 as libc::c_int);
            }
            132 => {
                backup_if_mismatch = 1 as libc::c_int != 0;
                current_block_80 = 14541395414537699361;
            }
            133 => {
                backup_if_mismatch = 0 as libc::c_int != 0;
                current_block_80 = 14541395414537699361;
            }
            134 => {
                posixly_correct = 1 as libc::c_int != 0;
                current_block_80 = 14541395414537699361;
            }
            135 => {
                let mut i: libc::c_int = argmatch(
                    optarg,
                    quoting_style_args.as_ptr(),
                    0 as *const libc::c_char,
                    0 as libc::c_int as size_t,
                ) as libc::c_int;
                if i < 0 as libc::c_int {
                    argmatch_invalid(
                        b"quoting style\0" as *const u8 as *const libc::c_char,
                        optarg,
                        i as ptrdiff_t,
                    );
                    usage(stderr, 2 as libc::c_int);
                }
                set_quoting_style(0 as *mut quoting_options, i as quoting_style);
                current_block_80 = 14541395414537699361;
            }
            136 => {
                if strcmp(optarg, b"context\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    reject_format = NEW_CONTEXT_DIFF;
                } else if strcmp(
                    optarg,
                    b"unified\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    reject_format = UNI_DIFF;
                } else {
                    usage(stderr, 2 as libc::c_int);
                }
                current_block_80 = 14541395414537699361;
            }
            137 => {
                if strcmp(optarg, b"ignore\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    read_only_behavior = RO_IGNORE;
                } else if strcmp(optarg, b"warn\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    read_only_behavior = RO_WARN;
                } else if strcmp(optarg, b"fail\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    read_only_behavior = RO_FAIL;
                } else {
                    usage(stderr, 2 as libc::c_int);
                }
                current_block_80 = 14541395414537699361;
            }
            138 => {
                follow_symlinks = 1 as libc::c_int != 0;
                current_block_80 = 14541395414537699361;
            }
            _ => {
                usage(stderr, 2 as libc::c_int);
            }
        }
        match current_block_80 {
            11656077632963965015 => {
                if *optarg == 0 {
                    fatal(
                        b"backup suffix is empty\0" as *const u8 as *const libc::c_char,
                    );
                }
                origsuff = xstrdup(optarg);
            }
            _ => {}
        }
    }
    if optind < Argc {
        let fresh1 = optind;
        optind = optind + 1;
        inname = xstrdup(*Argv.offset(fresh1 as isize));
        explicit_inname = 1 as libc::c_int != 0;
        invc = -(1 as libc::c_int);
        if optind < Argc {
            let fresh2 = optind;
            optind = optind + 1;
            patchname = xstrdup(*Argv.offset(fresh2 as isize));
            if optind < Argc {
                fprintf(
                    stderr,
                    b"%s: %s: extra operand\n\0" as *const u8 as *const libc::c_char,
                    program_name,
                    quotearg(*Argv.offset(optind as isize)),
                );
                usage(stderr, 2 as libc::c_int);
            }
        }
    }
}
unsafe extern "C" fn numeric_string(
    mut string: *const libc::c_char,
    mut negative_allowed: bool,
    mut argtype_msgid: *const libc::c_char,
) -> libc::c_int {
    let mut value: libc::c_int = 0 as libc::c_int;
    let mut p: *const libc::c_char = string;
    let mut sign: libc::c_int = if *p as libc::c_int == '-' as i32 {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
    p = p
        .offset(
            (*p as libc::c_int == '-' as i32 || *p as libc::c_int == '+' as i32)
                as libc::c_int as isize,
        );
    loop {
        let mut v10: libc::c_int = value * 10 as libc::c_int;
        let mut digit: libc::c_int = *p as libc::c_int - '0' as i32;
        let mut signed_digit: libc::c_int = sign * digit;
        let mut next_value: libc::c_int = v10 + signed_digit;
        if (9 as libc::c_int as libc::c_uint) < digit as libc::c_uint {
            fatal(
                b"%s %s is not a number\0" as *const u8 as *const libc::c_char,
                argtype_msgid,
                quotearg(string),
            );
        }
        if v10 / 10 as libc::c_int != value
            || (next_value < v10) as libc::c_int
                != (signed_digit < 0 as libc::c_int) as libc::c_int
        {
            fatal(
                b"%s %s is too large\0" as *const u8 as *const libc::c_char,
                argtype_msgid,
                quotearg(string),
            );
        }
        value = next_value;
        p = p.offset(1);
        if !(*p != 0) {
            break;
        }
    }
    if value < 0 as libc::c_int && !negative_allowed {
        fatal(
            b"%s %s is negative\0" as *const u8 as *const libc::c_char,
            argtype_msgid,
            quotearg(string),
        );
    }
    return value;
}
unsafe extern "C" fn locate_hunk(mut fuzz: lin) -> lin {
    let mut first_guess: lin = pch_first() + in_offset;
    let mut offset: lin = 0;
    let mut pat_lines: lin = pch_ptrn_lines();
    let mut prefix_context: lin = pch_prefix_context();
    let mut suffix_context: lin = pch_suffix_context();
    let mut context: lin = if prefix_context < suffix_context {
        suffix_context
    } else {
        prefix_context
    };
    let mut prefix_fuzz: lin = fuzz + prefix_context - context;
    let mut suffix_fuzz: lin = fuzz + suffix_context - context;
    let mut max_where: lin = input_lines - (pat_lines - suffix_fuzz)
        + 1 as libc::c_int as libc::c_long;
    let mut min_where: lin = last_frozen_line + 1 as libc::c_int as libc::c_long;
    let mut max_pos_offset: lin = max_where - first_guess;
    let mut max_neg_offset: lin = first_guess - min_where;
    let mut max_offset: lin = if max_pos_offset > max_neg_offset {
        max_pos_offset
    } else {
        max_neg_offset
    };
    let mut min_offset: lin = 0;
    if pat_lines == 0 {
        return first_guess;
    }
    if first_guess <= max_neg_offset {
        max_neg_offset = first_guess - 1 as libc::c_int as libc::c_long;
    }
    if prefix_fuzz < 0 as libc::c_int as libc::c_long
        && pch_first() <= 1 as libc::c_int as libc::c_long
    {
        if suffix_fuzz < 0 as libc::c_int as libc::c_long {
            if pat_lines != input_lines || prefix_context < last_frozen_line {
                return 0 as libc::c_int as lin;
            }
        }
        offset = 1 as libc::c_int as libc::c_long - first_guess;
        if last_frozen_line <= prefix_context && offset <= max_pos_offset
            && patch_match(first_guess, offset, 0 as libc::c_int as lin, suffix_fuzz)
                as libc::c_int != 0
        {
            in_offset += offset;
            return first_guess + offset;
        } else {
            return 0 as libc::c_int as lin
        }
    } else if prefix_fuzz < 0 as libc::c_int as libc::c_long {
        prefix_fuzz = 0 as libc::c_int as lin;
    }
    if suffix_fuzz < 0 as libc::c_int as libc::c_long {
        offset = first_guess
            - (input_lines - pat_lines + 1 as libc::c_int as libc::c_long);
        if offset <= max_neg_offset
            && patch_match(first_guess, -offset, prefix_fuzz, 0 as libc::c_int as lin)
                as libc::c_int != 0
        {
            in_offset -= offset;
            return first_guess - offset;
        } else {
            return 0 as libc::c_int as lin
        }
    }
    min_offset = if max_pos_offset < 0 as libc::c_int as libc::c_long {
        first_guess - max_where
    } else if max_neg_offset < 0 as libc::c_int as libc::c_long {
        first_guess - min_where
    } else {
        0 as libc::c_int as libc::c_long
    };
    offset = min_offset;
    while offset <= max_offset {
        let mut numbuf0: [libc::c_char; 23] = [0; 23];
        let mut numbuf1: [libc::c_char; 23] = [0; 23];
        if offset <= max_pos_offset
            && patch_match(first_guess, offset, prefix_fuzz, suffix_fuzz) as libc::c_int
                != 0
        {
            if debug & 1 as libc::c_int != 0 {
                say(
                    b"Offset changing from %s to %s\n\0" as *const u8
                        as *const libc::c_char,
                    format_linenum(numbuf0.as_mut_ptr(), in_offset),
                    format_linenum(numbuf1.as_mut_ptr(), in_offset + offset),
                );
            }
            in_offset += offset;
            return first_guess + offset;
        }
        if offset <= max_neg_offset
            && patch_match(first_guess, -offset, prefix_fuzz, suffix_fuzz) as libc::c_int
                != 0
        {
            if debug & 1 as libc::c_int != 0 {
                say(
                    b"Offset changing from %s to %s\n\0" as *const u8
                        as *const libc::c_char,
                    format_linenum(numbuf0.as_mut_ptr(), in_offset),
                    format_linenum(numbuf1.as_mut_ptr(), in_offset - offset),
                );
            }
            in_offset -= offset;
            return first_guess - offset;
        }
        offset += 1;
        offset;
    }
    return 0 as libc::c_int as lin;
}
unsafe extern "C" fn mangled_patch(mut old: lin, mut new: lin) -> ! {
    let mut numbuf0: [libc::c_char; 23] = [0; 23];
    let mut numbuf1: [libc::c_char; 23] = [0; 23];
    if debug & 1 as libc::c_int != 0 {
        say(
            b"oldchar = '%c', newchar = '%c'\n\0" as *const u8 as *const libc::c_char,
            pch_char(old) as libc::c_int,
            pch_char(new) as libc::c_int,
        );
    }
    fatal(
        b"Out-of-sync patch, lines %s,%s -- mangled text or line numbers, maybe?\0"
            as *const u8 as *const libc::c_char,
        format_linenum(numbuf0.as_mut_ptr(), pch_hunk_beg() + old),
        format_linenum(numbuf1.as_mut_ptr(), pch_hunk_beg() + new),
    );
}
unsafe extern "C" fn print_unidiff_range(
    mut fp: *mut FILE,
    mut start: lin,
    mut count: lin,
) {
    let mut numbuf0: [libc::c_char; 23] = [0; 23];
    let mut numbuf1: [libc::c_char; 23] = [0; 23];
    match count {
        0 => {
            fprintf(
                fp,
                b"%s,0\0" as *const u8 as *const libc::c_char,
                format_linenum(
                    numbuf0.as_mut_ptr(),
                    start - 1 as libc::c_int as libc::c_long,
                ),
            );
        }
        1 => {
            fprintf(
                fp,
                b"%s\0" as *const u8 as *const libc::c_char,
                format_linenum(numbuf0.as_mut_ptr(), start),
            );
        }
        _ => {
            fprintf(
                fp,
                b"%s,%s\0" as *const u8 as *const libc::c_char,
                format_linenum(numbuf0.as_mut_ptr(), start),
                format_linenum(numbuf1.as_mut_ptr(), count),
            );
        }
    };
}
unsafe extern "C" fn print_header_line(
    mut fp: *mut FILE,
    mut tag: *const libc::c_char,
    mut reverse_0: bool,
) {
    let mut name: *const libc::c_char = pch_name(reverse_0 as nametype);
    let mut timestr: *const libc::c_char = pch_timestr(reverse_0);
    fprintf(
        fp,
        b"%s %s%s\n\0" as *const u8 as *const libc::c_char,
        tag,
        if !name.is_null() {
            name
        } else {
            b"/dev/null\0" as *const u8 as *const libc::c_char
        },
        if !timestr.is_null() {
            timestr
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
}
unsafe extern "C" fn abort_hunk_unified(mut header: bool, mut reverse_0: bool) {
    let mut old: lin = 1 as libc::c_int as lin;
    let mut lastline: lin = pch_ptrn_lines();
    let mut new: lin = lastline + 1 as libc::c_int as libc::c_long;
    let mut c_function: *const libc::c_char = pch_c_function();
    if header {
        if !(pch_name(INDEX)).is_null() {
            fprintf(
                rejfp,
                b"Index: %s\n\0" as *const u8 as *const libc::c_char,
                pch_name(INDEX),
            );
        }
        print_header_line(
            rejfp,
            b"---\0" as *const u8 as *const libc::c_char,
            reverse_0,
        );
        print_header_line(
            rejfp,
            b"+++\0" as *const u8 as *const libc::c_char,
            !reverse_0,
        );
    }
    fprintf(rejfp, b"@@ -\0" as *const u8 as *const libc::c_char);
    print_unidiff_range(rejfp, pch_first() + out_offset, lastline);
    fprintf(rejfp, b" +\0" as *const u8 as *const libc::c_char);
    print_unidiff_range(rejfp, pch_newfirst() + out_offset, pch_repl_lines());
    fprintf(
        rejfp,
        b" @@%s\n\0" as *const u8 as *const libc::c_char,
        if !c_function.is_null() {
            c_function
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    while pch_char(new) as libc::c_int == '=' as i32
        || pch_char(new) as libc::c_int == '\n' as i32
    {
        new += 1;
        new;
    }
    if diff_type as libc::c_uint != UNI_DIFF as libc::c_int as libc::c_uint {
        pch_normalize(UNI_DIFF);
    }
    loop {
        while pch_char(old) as libc::c_int == '-' as i32 {
            fputc('-' as i32, rejfp);
            pch_write_line(old, rejfp);
            old += 1;
            old;
        }
        while pch_char(new) as libc::c_int == '+' as i32 {
            fputc('+' as i32, rejfp);
            pch_write_line(new, rejfp);
            new += 1;
            new;
        }
        if old > lastline {
            break;
        }
        if pch_char(new) as libc::c_int != pch_char(old) as libc::c_int {
            mangled_patch(old, new);
        }
        fputc(' ' as i32, rejfp);
        pch_write_line(old, rejfp);
        old += 1;
        old;
        new += 1;
        new;
    }
    if pch_char(new) as libc::c_int != '^' as i32 {
        mangled_patch(old, new);
    }
}
unsafe extern "C" fn abort_hunk_context(mut header: bool, mut reverse_0: bool) {
    let mut i: lin = 0;
    let mut pat_end: lin = pch_end();
    let mut oldfirst: lin = pch_first() + out_offset;
    let mut newfirst: lin = pch_newfirst() + out_offset;
    let mut oldlast: lin = oldfirst + pch_ptrn_lines()
        - 1 as libc::c_int as libc::c_long;
    let mut newlast: lin = newfirst + pch_repl_lines()
        - 1 as libc::c_int as libc::c_long;
    let mut stars: *const libc::c_char = if NEW_CONTEXT_DIFF as libc::c_int
        <= diff_type as libc::c_int
    {
        b" ****\0" as *const u8 as *const libc::c_char
    } else {
        b"\0" as *const u8 as *const libc::c_char
    };
    let mut minuses: *const libc::c_char = if NEW_CONTEXT_DIFF as libc::c_int
        <= diff_type as libc::c_int
    {
        b" ----\0" as *const u8 as *const libc::c_char
    } else {
        b" -----\0" as *const u8 as *const libc::c_char
    };
    let mut c_function: *const libc::c_char = pch_c_function();
    if diff_type as libc::c_uint == UNI_DIFF as libc::c_int as libc::c_uint {
        pch_normalize(NEW_CONTEXT_DIFF);
    }
    if header {
        if !(pch_name(INDEX)).is_null() {
            fprintf(
                rejfp,
                b"Index: %s\n\0" as *const u8 as *const libc::c_char,
                pch_name(INDEX),
            );
        }
        print_header_line(
            rejfp,
            b"***\0" as *const u8 as *const libc::c_char,
            reverse_0,
        );
        print_header_line(
            rejfp,
            b"---\0" as *const u8 as *const libc::c_char,
            !reverse_0,
        );
    }
    fprintf(
        rejfp,
        b"***************%s\n\0" as *const u8 as *const libc::c_char,
        if !c_function.is_null() {
            c_function
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    i = 0 as libc::c_int as lin;
    while i <= pat_end {
        let mut numbuf0: [libc::c_char; 23] = [0; 23];
        let mut numbuf1: [libc::c_char; 23] = [0; 23];
        let mut current_block_22: u64;
        match pch_char(i) as libc::c_int {
            42 => {
                if oldlast < oldfirst {
                    fprintf(
                        rejfp,
                        b"*** 0%s\n\0" as *const u8 as *const libc::c_char,
                        stars,
                    );
                } else if oldlast == oldfirst {
                    fprintf(
                        rejfp,
                        b"*** %s%s\n\0" as *const u8 as *const libc::c_char,
                        format_linenum(numbuf0.as_mut_ptr(), oldfirst),
                        stars,
                    );
                } else {
                    fprintf(
                        rejfp,
                        b"*** %s,%s%s\n\0" as *const u8 as *const libc::c_char,
                        format_linenum(numbuf0.as_mut_ptr(), oldfirst),
                        format_linenum(numbuf1.as_mut_ptr(), oldlast),
                        stars,
                    );
                }
                current_block_22 = 15345278821338558188;
            }
            61 => {
                if newlast < newfirst {
                    fprintf(
                        rejfp,
                        b"--- 0%s\n\0" as *const u8 as *const libc::c_char,
                        minuses,
                    );
                } else if newlast == newfirst {
                    fprintf(
                        rejfp,
                        b"--- %s%s\n\0" as *const u8 as *const libc::c_char,
                        format_linenum(numbuf0.as_mut_ptr(), newfirst),
                        minuses,
                    );
                } else {
                    fprintf(
                        rejfp,
                        b"--- %s,%s%s\n\0" as *const u8 as *const libc::c_char,
                        format_linenum(numbuf0.as_mut_ptr(), newfirst),
                        format_linenum(numbuf1.as_mut_ptr(), newlast),
                        minuses,
                    );
                }
                current_block_22 = 15345278821338558188;
            }
            32 | 45 | 43 | 33 => {
                fprintf(
                    rejfp,
                    b"%c \0" as *const u8 as *const libc::c_char,
                    pch_char(i) as libc::c_int,
                );
                current_block_22 = 6129218247606383141;
            }
            10 => {
                current_block_22 = 6129218247606383141;
            }
            _ => {
                fatal(
                    b"fatal internal error in abort_hunk_context\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        match current_block_22 {
            6129218247606383141 => {
                pch_write_line(i, rejfp);
            }
            _ => {}
        }
        if ferror(rejfp) != 0 {
            write_fatal();
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn abort_hunk(
    mut outname: *const libc::c_char,
    mut header: bool,
    mut reverse_0: bool,
) {
    if !TMPREJNAME_needs_removal {
        init_reject(outname);
    }
    if reject_format as libc::c_uint == UNI_DIFF as libc::c_int as libc::c_uint
        || reject_format as libc::c_uint == NO_DIFF as libc::c_int as libc::c_uint
            && diff_type as libc::c_uint == UNI_DIFF as libc::c_int as libc::c_uint
    {
        abort_hunk_unified(header, reverse_0);
    } else {
        abort_hunk_context(header, reverse_0);
    };
}
unsafe extern "C" fn apply_hunk(mut outstate: *mut outstate, mut where_0: lin) -> bool {
    let mut old: lin = 1 as libc::c_int as lin;
    let mut lastline: lin = pch_ptrn_lines();
    let mut new: lin = lastline + 1 as libc::c_int as libc::c_long;
    let mut def_state: C2RustUnnamed_1 = OUTSIDE;
    let mut R_do_defines: *const libc::c_char = do_defines;
    let mut pat_end: lin = pch_end();
    let mut fp: *mut FILE = (*outstate).ofp;
    where_0 -= 1;
    where_0;
    while pch_char(new) as libc::c_int == '=' as i32
        || pch_char(new) as libc::c_int == '\n' as i32
    {
        new += 1;
        new;
    }
    while old <= lastline {
        if pch_char(old) as libc::c_int == '-' as i32 {
            if (*outstate).after_newline {} else {
                __assert_fail(
                    b"outstate->after_newline\0" as *const u8 as *const libc::c_char,
                    b"patch.c\0" as *const u8 as *const libc::c_char,
                    1429 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 41],
                        &[libc::c_char; 41],
                    >(b"_Bool apply_hunk(struct outstate *, lin)\0"))
                        .as_ptr(),
                );
            }
            'c_9899: {
                if (*outstate).after_newline {} else {
                    __assert_fail(
                        b"outstate->after_newline\0" as *const u8 as *const libc::c_char,
                        b"patch.c\0" as *const u8 as *const libc::c_char,
                        1429 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 41],
                            &[libc::c_char; 41],
                        >(b"_Bool apply_hunk(struct outstate *, lin)\0"))
                            .as_ptr(),
                    );
                }
            };
            if !copy_till(outstate, where_0 + old - 1 as libc::c_int as libc::c_long) {
                return 0 as libc::c_int != 0;
            }
            if !R_do_defines.is_null() {
                if def_state as libc::c_uint == OUTSIDE as libc::c_int as libc::c_uint {
                    fprintf(
                        fp,
                        not_defined
                            .as_ptr()
                            .offset((*outstate).after_newline as libc::c_int as isize),
                        R_do_defines,
                    );
                    def_state = IN_IFNDEF;
                } else if def_state as libc::c_uint
                    == IN_IFDEF as libc::c_int as libc::c_uint
                {
                    fputs(
                        else_defined
                            .as_ptr()
                            .offset((*outstate).after_newline as libc::c_int as isize),
                        fp,
                    );
                    def_state = IN_ELSE;
                }
                if ferror(fp) != 0 {
                    write_fatal();
                }
                (*outstate).after_newline = pch_write_line(old, fp);
                (*outstate).zero_output = 0 as libc::c_int != 0;
            }
            last_frozen_line += 1;
            last_frozen_line;
            old += 1;
            old;
        } else {
            if new > pat_end {
                break;
            }
            if pch_char(new) as libc::c_int == '+' as i32 {
                if !copy_till(
                    outstate,
                    where_0 + old - 1 as libc::c_int as libc::c_long,
                ) {
                    return 0 as libc::c_int != 0;
                }
                if !R_do_defines.is_null() {
                    if def_state as libc::c_uint
                        == IN_IFNDEF as libc::c_int as libc::c_uint
                    {
                        fputs(
                            else_defined
                                .as_ptr()
                                .offset((*outstate).after_newline as libc::c_int as isize),
                            fp,
                        );
                        def_state = IN_ELSE;
                    } else if def_state as libc::c_uint
                        == OUTSIDE as libc::c_int as libc::c_uint
                    {
                        fprintf(
                            fp,
                            if_defined
                                .as_ptr()
                                .offset((*outstate).after_newline as libc::c_int as isize),
                            R_do_defines,
                        );
                        def_state = IN_IFDEF;
                    }
                    if ferror(fp) != 0 {
                        write_fatal();
                    }
                }
                (*outstate).after_newline = pch_write_line(new, fp);
                (*outstate).zero_output = 0 as libc::c_int != 0;
                new += 1;
                new;
            } else if pch_char(new) as libc::c_int != pch_char(old) as libc::c_int {
                mangled_patch(old, new);
            } else if pch_char(new) as libc::c_int == '!' as i32 {
                if (*outstate).after_newline {} else {
                    __assert_fail(
                        b"outstate->after_newline\0" as *const u8 as *const libc::c_char,
                        b"patch.c\0" as *const u8 as *const libc::c_char,
                        1476 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 41],
                            &[libc::c_char; 41],
                        >(b"_Bool apply_hunk(struct outstate *, lin)\0"))
                            .as_ptr(),
                    );
                }
                'c_9537: {
                    if (*outstate).after_newline {} else {
                        __assert_fail(
                            b"outstate->after_newline\0" as *const u8
                                as *const libc::c_char,
                            b"patch.c\0" as *const u8 as *const libc::c_char,
                            1476 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 41],
                                &[libc::c_char; 41],
                            >(b"_Bool apply_hunk(struct outstate *, lin)\0"))
                                .as_ptr(),
                        );
                    }
                };
                if !copy_till(
                    outstate,
                    where_0 + old - 1 as libc::c_int as libc::c_long,
                ) {
                    return 0 as libc::c_int != 0;
                }
                if (*outstate).after_newline {} else {
                    __assert_fail(
                        b"outstate->after_newline\0" as *const u8 as *const libc::c_char,
                        b"patch.c\0" as *const u8 as *const libc::c_char,
                        1479 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 41],
                            &[libc::c_char; 41],
                        >(b"_Bool apply_hunk(struct outstate *, lin)\0"))
                            .as_ptr(),
                    );
                }
                'c_9482: {
                    if (*outstate).after_newline {} else {
                        __assert_fail(
                            b"outstate->after_newline\0" as *const u8
                                as *const libc::c_char,
                            b"patch.c\0" as *const u8 as *const libc::c_char,
                            1479 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 41],
                                &[libc::c_char; 41],
                            >(b"_Bool apply_hunk(struct outstate *, lin)\0"))
                                .as_ptr(),
                        );
                    }
                };
                if !R_do_defines.is_null() {
                    fprintf(
                        fp,
                        not_defined.as_ptr().offset(1 as libc::c_int as isize),
                        R_do_defines,
                    );
                    if ferror(fp) != 0 {
                        write_fatal();
                    }
                    def_state = IN_IFNDEF;
                }
                loop {
                    if !R_do_defines.is_null() {
                        (*outstate).after_newline = pch_write_line(old, fp);
                    }
                    last_frozen_line += 1;
                    last_frozen_line;
                    old += 1;
                    old;
                    if !(pch_char(old) as libc::c_int == '!' as i32) {
                        break;
                    }
                }
                if !R_do_defines.is_null() {
                    fputs(
                        else_defined
                            .as_ptr()
                            .offset((*outstate).after_newline as libc::c_int as isize),
                        fp,
                    );
                    if ferror(fp) != 0 {
                        write_fatal();
                    }
                    def_state = IN_ELSE;
                }
                loop {
                    (*outstate).after_newline = pch_write_line(new, fp);
                    new += 1;
                    new;
                    if !(pch_char(new) as libc::c_int == '!' as i32) {
                        break;
                    }
                }
                (*outstate).zero_output = 0 as libc::c_int != 0;
            } else {
                if pch_char(new) as libc::c_int == ' ' as i32 {} else {
                    __assert_fail(
                        b"pch_char(new) == ' '\0" as *const u8 as *const libc::c_char,
                        b"patch.c\0" as *const u8 as *const libc::c_char,
                        1513 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 41],
                            &[libc::c_char; 41],
                        >(b"_Bool apply_hunk(struct outstate *, lin)\0"))
                            .as_ptr(),
                    );
                }
                'c_9303: {
                    if pch_char(new) as libc::c_int == ' ' as i32 {} else {
                        __assert_fail(
                            b"pch_char(new) == ' '\0" as *const u8
                                as *const libc::c_char,
                            b"patch.c\0" as *const u8 as *const libc::c_char,
                            1513 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 41],
                                &[libc::c_char; 41],
                            >(b"_Bool apply_hunk(struct outstate *, lin)\0"))
                                .as_ptr(),
                        );
                    }
                };
                old += 1;
                old;
                new += 1;
                new;
                if !R_do_defines.is_null()
                    && def_state as libc::c_uint
                        != OUTSIDE as libc::c_int as libc::c_uint
                {
                    fputs(
                        end_defined
                            .as_ptr()
                            .offset((*outstate).after_newline as libc::c_int as isize),
                        fp,
                    );
                    if ferror(fp) != 0 {
                        write_fatal();
                    }
                    (*outstate).after_newline = 1 as libc::c_int != 0;
                    def_state = OUTSIDE;
                }
            }
        }
    }
    if new <= pat_end && pch_char(new) as libc::c_int == '+' as i32 {
        if !copy_till(outstate, where_0 + old - 1 as libc::c_int as libc::c_long) {
            return 0 as libc::c_int != 0;
        }
        if !R_do_defines.is_null() {
            if def_state as libc::c_uint == OUTSIDE as libc::c_int as libc::c_uint {
                fprintf(
                    fp,
                    if_defined
                        .as_ptr()
                        .offset((*outstate).after_newline as libc::c_int as isize),
                    R_do_defines,
                );
                def_state = IN_IFDEF;
            } else if def_state as libc::c_uint
                == IN_IFNDEF as libc::c_int as libc::c_uint
            {
                fputs(
                    else_defined
                        .as_ptr()
                        .offset((*outstate).after_newline as libc::c_int as isize),
                    fp,
                );
                def_state = IN_ELSE;
            }
            if ferror(fp) != 0 {
                write_fatal();
            }
            (*outstate).zero_output = 0 as libc::c_int != 0;
        }
        loop {
            if !(*outstate).after_newline && putc('\n' as i32, fp) == -(1 as libc::c_int)
            {
                write_fatal();
            }
            (*outstate).after_newline = pch_write_line(new, fp);
            (*outstate).zero_output = 0 as libc::c_int != 0;
            new += 1;
            new;
            if !(new <= pat_end && pch_char(new) as libc::c_int == '+' as i32) {
                break;
            }
        }
    }
    if !R_do_defines.is_null()
        && def_state as libc::c_uint != OUTSIDE as libc::c_int as libc::c_uint
    {
        fputs(
            end_defined
                .as_ptr()
                .offset((*outstate).after_newline as libc::c_int as isize),
            fp,
        );
        if ferror(fp) != 0 {
            write_fatal();
        }
        (*outstate).after_newline = 1 as libc::c_int != 0;
    }
    out_offset += pch_repl_lines() - pch_ptrn_lines();
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn create_output_file(
    mut name: *const libc::c_char,
    mut open_flags: libc::c_int,
) -> *mut FILE {
    let mut fd: libc::c_int = create_file(
        name,
        0o1 as libc::c_int | 0 as libc::c_int | open_flags,
        instat.st_mode,
        1 as libc::c_int != 0,
    );
    let mut f: *mut FILE = fdopen(
        fd,
        if 0 as libc::c_int != 0 {
            b"wb\0" as *const u8 as *const libc::c_char
        } else {
            b"w\0" as *const u8 as *const libc::c_char
        },
    );
    if f.is_null() {
        pfatal(
            b"Can't create file %s\0" as *const u8 as *const libc::c_char,
            quotearg(name),
        );
    }
    return f;
}
unsafe extern "C" fn init_output(mut outstate: *mut outstate) {
    (*outstate).ofp = 0 as *mut FILE;
    (*outstate).after_newline = 1 as libc::c_int != 0;
    (*outstate).zero_output = 1 as libc::c_int != 0;
}
unsafe extern "C" fn open_outfile(mut name: *const libc::c_char) -> *mut FILE {
    if strcmp(name, b"-\0" as *const u8 as *const libc::c_char) != 0 as libc::c_int {
        return create_output_file(name, 0 as libc::c_int)
    } else {
        let mut ofp: *mut FILE = 0 as *mut FILE;
        let mut stdout_dup: libc::c_int = dup(fileno(stdout));
        if stdout_dup == -(1 as libc::c_int) {
            pfatal(
                b"Failed to duplicate standard output\0" as *const u8
                    as *const libc::c_char,
            );
        }
        ofp = fdopen(stdout_dup, b"a\0" as *const u8 as *const libc::c_char);
        if ofp.is_null() {
            pfatal(
                b"Failed to duplicate standard output\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if dup2(fileno(stderr), fileno(stdout)) == -(1 as libc::c_int) {
            pfatal(
                b"Failed to redirect messages to standard error\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return ofp;
    };
}
unsafe extern "C" fn init_reject(mut outname: *const libc::c_char) {
    let mut fd: libc::c_int = 0;
    fd = make_tempfile(
        &mut TMPREJNAME,
        'r' as i32 as libc::c_char,
        outname,
        0o1 as libc::c_int | 0 as libc::c_int,
        0o666 as libc::c_int as mode_t,
    );
    if fd == -(1 as libc::c_int) {
        pfatal(
            b"Can't create temporary file %s\0" as *const u8 as *const libc::c_char,
            TMPREJNAME,
        );
    }
    TMPREJNAME_needs_removal = 1 as libc::c_int != 0;
    rejfp = fdopen(
        fd,
        if 0 as libc::c_int != 0 {
            b"wb\0" as *const u8 as *const libc::c_char
        } else {
            b"w\0" as *const u8 as *const libc::c_char
        },
    );
    if rejfp.is_null() {
        pfatal(
            b"Can't open stream for file %s\0" as *const u8 as *const libc::c_char,
            quotearg(TMPREJNAME),
        );
    }
}
pub unsafe extern "C" fn copy_till(
    mut outstate: *mut outstate,
    mut lastline: lin,
) -> bool {
    let mut R_last_frozen_line: lin = last_frozen_line;
    let mut fp: *mut FILE = (*outstate).ofp;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut size: size_t = 0;
    if R_last_frozen_line > lastline {
        say(
            b"misordered hunks! output would be garbled\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    while R_last_frozen_line < lastline {
        R_last_frozen_line += 1;
        s = ifetch(R_last_frozen_line, 0 as libc::c_int != 0, &mut size);
        if size != 0 {
            if !(*outstate).after_newline && putc('\n' as i32, fp) == -(1 as libc::c_int)
                || fwrite(
                    s as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    size,
                    fp,
                ) == 0
            {
                write_fatal();
            }
            (*outstate)
                .after_newline = *s
                .offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int == '\n' as i32;
            (*outstate).zero_output = 0 as libc::c_int != 0;
        }
    }
    last_frozen_line = R_last_frozen_line;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn spew_output(
    mut outstate: *mut outstate,
    mut st: *mut stat,
) -> bool {
    if debug & 256 as libc::c_int != 0 {
        let mut numbuf0: [libc::c_char; 23] = [0; 23];
        let mut numbuf1: [libc::c_char; 23] = [0; 23];
        say(
            b"il=%s lfl=%s\n\0" as *const u8 as *const libc::c_char,
            format_linenum(numbuf0.as_mut_ptr(), input_lines),
            format_linenum(numbuf1.as_mut_ptr(), last_frozen_line),
        );
    }
    if last_frozen_line < input_lines {
        if !copy_till(outstate, input_lines) {
            return 0 as libc::c_int != 0;
        }
    }
    if !((*outstate).ofp).is_null() && outfile.is_null() {
        if fflush((*outstate).ofp) != 0 as libc::c_int
            || fstat(fileno((*outstate).ofp), st) != 0 as libc::c_int
            || fclose((*outstate).ofp) != 0 as libc::c_int
        {
            write_fatal();
        }
        (*outstate).ofp = 0 as *mut FILE;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn patch_match(
    mut base: lin,
    mut offset: lin,
    mut prefix_fuzz: lin,
    mut suffix_fuzz: lin,
) -> bool {
    let mut pline: lin = 1 as libc::c_int as libc::c_long + prefix_fuzz;
    let mut iline: lin = 0;
    let mut pat_lines: lin = pch_ptrn_lines() - suffix_fuzz;
    let mut size: size_t = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    iline = base + offset + prefix_fuzz;
    while pline <= pat_lines {
        p = ifetch(iline, offset >= 0 as libc::c_int as libc::c_long, &mut size);
        if canonicalize_ws {
            if !similar(p, size, pfetch(pline), pch_line_len(pline)) {
                return 0 as libc::c_int != 0;
            }
        } else if size != pch_line_len(pline)
            || memcmp(
                p as *const libc::c_void,
                pfetch(pline) as *const libc::c_void,
                size,
            ) != 0 as libc::c_int
        {
            return 0 as libc::c_int != 0
        }
        pline += 1;
        pline;
        iline += 1;
        iline;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn check_line_endings(mut where_0: lin) -> bool {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut size: size_t = 0;
    let mut input_crlf: bool = false;
    let mut patch_crlf: bool = false;
    p = pfetch(1 as libc::c_int as lin);
    size = pch_line_len(1 as libc::c_int as lin);
    if size == 0 {
        return 0 as libc::c_int != 0;
    }
    patch_crlf = size >= 2 as libc::c_int as libc::c_ulong
        && *p.offset(size.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '\r' as i32
        && *p.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '\n' as i32;
    if input_lines == 0 {
        return 0 as libc::c_int != 0;
    }
    if where_0 > input_lines {
        where_0 = input_lines;
    }
    p = ifetch(where_0, 0 as libc::c_int != 0, &mut size);
    if size == 0 {
        return 0 as libc::c_int != 0;
    }
    input_crlf = size >= 2 as libc::c_int as libc::c_ulong
        && *p.offset(size.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '\r' as i32
        && *p.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '\n' as i32;
    return patch_crlf as libc::c_int != input_crlf as libc::c_int;
}
pub unsafe extern "C" fn similar(
    mut a: *const libc::c_char,
    mut alen: size_t,
    mut b: *const libc::c_char,
    mut blen: size_t,
) -> bool {
    alen = (alen as libc::c_ulong)
        .wrapping_sub(
            (alen != 0
                && *a
                    .offset(
                        alen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int == '\n' as i32) as libc::c_int as libc::c_ulong,
        ) as size_t as size_t;
    blen = (blen as libc::c_ulong)
        .wrapping_sub(
            (blen != 0
                && *b
                    .offset(
                        blen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int == '\n' as i32) as libc::c_int as libc::c_ulong,
        ) as size_t as size_t;
    loop {
        if blen == 0
            || (*b as libc::c_int == ' ' as i32 || *b as libc::c_int == '\t' as i32)
        {
            while blen != 0
                && (*b as libc::c_int == ' ' as i32 || *b as libc::c_int == '\t' as i32)
            {
                b = b.offset(1);
                b;
                blen = blen.wrapping_sub(1);
                blen;
            }
            if alen != 0 {
                if !(*a as libc::c_int == ' ' as i32 || *a as libc::c_int == '\t' as i32)
                {
                    return 0 as libc::c_int != 0;
                }
                loop {
                    a = a.offset(1);
                    a;
                    alen = alen.wrapping_sub(1);
                    alen;
                    if !(alen != 0
                        && (*a as libc::c_int == ' ' as i32
                            || *a as libc::c_int == '\t' as i32))
                    {
                        break;
                    }
                }
            }
            if alen == 0 || blen == 0 {
                return alen == blen;
            }
        } else if alen == 0
            || {
                let fresh3 = a;
                a = a.offset(1);
                let fresh4 = b;
                b = b.offset(1);
                *fresh3 as libc::c_int != *fresh4 as libc::c_int
            }
        {
            return 0 as libc::c_int != 0
        } else {
            alen = alen.wrapping_sub(1);
            alen;
            blen = blen.wrapping_sub(1);
            blen;
        }
    };
}
static mut files_to_delete: gl_list_t = 0 as *const gl_list_impl as *mut gl_list_impl;
unsafe extern "C" fn init_files_to_delete() {
    files_to_delete = gl_list_create_empty(
        &gl_linked_list_implementation,
        None,
        None,
        None,
        1 as libc::c_int != 0,
    );
}
unsafe extern "C" fn delete_file_later(
    mut name: *const libc::c_char,
    mut st: *const stat,
    mut backup: bool,
) {
    let mut file_to_delete: *mut file_to_delete = 0 as *mut file_to_delete;
    let mut st_tmp: stat = stat {
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
    if st.is_null() {
        if stat_file(name, &mut st_tmp) != 0 as libc::c_int {
            pfatal(
                b"Can't get file attributes of %s %s\0" as *const u8
                    as *const libc::c_char,
                b"file\0" as *const u8 as *const libc::c_char,
                name,
            );
        }
        st = &mut st_tmp;
    }
    file_to_delete = xmalloc(::std::mem::size_of::<file_to_delete>() as libc::c_ulong)
        as *mut file_to_delete;
    (*file_to_delete).name = xstrdup(name);
    (*file_to_delete).st = *st;
    (*file_to_delete).backup = backup;
    gl_list_add_last(files_to_delete, file_to_delete as *const libc::c_void);
    insert_file_id(st, DELETE_LATER);
}
unsafe extern "C" fn delete_files() {
    let mut iter: gl_list_iterator_t = gl_list_iterator_t {
        vtable: 0 as *const gl_list_implementation,
        list: 0 as *mut gl_list_impl,
        count: 0,
        p: 0 as *mut libc::c_void,
        q: 0 as *mut libc::c_void,
        i: 0,
        j: 0,
    };
    let mut elt: *const libc::c_void = 0 as *const libc::c_void;
    iter = gl_list_iterator(files_to_delete);
    while gl_list_iterator_next(&mut iter, &mut elt, 0 as *mut gl_list_node_t) {
        let mut file_to_delete: *const file_to_delete = elt as *const file_to_delete;
        if lookup_file_id(&(*file_to_delete).st) as libc::c_uint
            == DELETE_LATER as libc::c_int as libc::c_uint
        {
            let mut mode: mode_t = (*file_to_delete).st.st_mode;
            if verbosity as libc::c_uint == VERBOSE as libc::c_int as libc::c_uint {
                say(
                    b"Removing %s %s\n\0" as *const u8 as *const libc::c_char,
                    if mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o120000 as libc::c_int as libc::c_uint
                    {
                        b"symbolic link\0" as *const u8 as *const libc::c_char
                    } else {
                        b"file\0" as *const u8 as *const libc::c_char
                    },
                    quotearg((*file_to_delete).name),
                );
            }
            move_file(
                0 as *const libc::c_char,
                0 as *mut bool,
                0 as *const stat,
                (*file_to_delete).name,
                mode,
                (*file_to_delete).backup,
            );
            removedirs((*file_to_delete).name);
        }
    }
    gl_list_iterator_free(&mut iter);
}
static mut files_to_output: gl_list_t = 0 as *const gl_list_impl as *mut gl_list_impl;
unsafe extern "C" fn output_file_later(
    mut from: *const libc::c_char,
    mut from_needs_removal: *mut bool,
    mut from_st: *const stat,
    mut to: *const libc::c_char,
    mut mode: mode_t,
    mut backup: bool,
) {
    let mut file_to_output: *mut file_to_output = 0 as *mut file_to_output;
    file_to_output = xmalloc(::std::mem::size_of::<file_to_output>() as libc::c_ulong)
        as *mut file_to_output;
    (*file_to_output).from = xstrdup(from);
    (*file_to_output).from_st = *from_st;
    (*file_to_output)
        .to = if !to.is_null() { xstrdup(to) } else { 0 as *mut libc::c_char };
    (*file_to_output).mode = mode;
    (*file_to_output).backup = backup;
    gl_list_add_last(files_to_output, file_to_output as *const libc::c_void);
    if !from_needs_removal.is_null() {
        *from_needs_removal = 0 as libc::c_int != 0;
    }
}
unsafe extern "C" fn output_file_now(
    mut from: *const libc::c_char,
    mut from_needs_removal: *mut bool,
    mut from_st: *const stat,
    mut to: *const libc::c_char,
    mut mode: mode_t,
    mut backup: bool,
) {
    if to.is_null() {
        if backup {
            create_backup(from, from_st, 1 as libc::c_int != 0);
        }
    } else {
        if (*from_st).st_size != -(1 as libc::c_int) as libc::c_long {} else {
            __assert_fail(
                b"from_st->st_size != -1\0" as *const u8 as *const libc::c_char,
                b"patch.c\0" as *const u8 as *const libc::c_char,
                1873 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 94],
                    &[libc::c_char; 94],
                >(
                    b"void output_file_now(const char *, _Bool *, const struct stat *, const char *, mode_t, _Bool)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_3068: {
            if (*from_st).st_size != -(1 as libc::c_int) as libc::c_long {} else {
                __assert_fail(
                    b"from_st->st_size != -1\0" as *const u8 as *const libc::c_char,
                    b"patch.c\0" as *const u8 as *const libc::c_char,
                    1873 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 94],
                        &[libc::c_char; 94],
                    >(
                        b"void output_file_now(const char *, _Bool *, const struct stat *, const char *, mode_t, _Bool)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        move_file(from, from_needs_removal, from_st, to, mode, backup);
    };
}
unsafe extern "C" fn output_file(
    mut from: *const libc::c_char,
    mut from_needs_removal: *mut bool,
    mut from_st: *const stat,
    mut to: *const libc::c_char,
    mut to_st: *const stat,
    mut mode: mode_t,
    mut backup: bool,
) {
    if from.is_null() {
        delete_file_later(to, to_st, backup);
    } else if pch_git_diff() as libc::c_int != 0
        && pch_says_nonexistent(reverse) != 2 as libc::c_int
    {
        output_file_later(from, from_needs_removal, from_st, to, mode, backup);
    } else {
        output_file_now(from, from_needs_removal, from_st, to, mode, backup);
    };
}
unsafe extern "C" fn dispose_file_to_output(mut elt: *const libc::c_void) {
    let mut file_to_output: *const file_to_output = elt as *const file_to_output;
    free((*file_to_output).from as *mut libc::c_void);
    free((*file_to_output).to as *mut libc::c_void);
}
unsafe extern "C" fn init_files_to_output() {
    files_to_output = gl_list_create_empty(
        &gl_linked_list_implementation,
        None,
        None,
        Some(dispose_file_to_output as unsafe extern "C" fn(*const libc::c_void) -> ()),
        1 as libc::c_int != 0,
    );
}
unsafe extern "C" fn gl_list_clear(mut list: gl_list_t) {
    while gl_list_size(list) > 0 as libc::c_int as libc::c_ulong {
        gl_list_remove_at(list, 0 as libc::c_int as size_t);
    }
}
unsafe extern "C" fn output_files(mut st: *const stat) {
    let mut iter: gl_list_iterator_t = gl_list_iterator_t {
        vtable: 0 as *const gl_list_implementation,
        list: 0 as *mut gl_list_impl,
        count: 0,
        p: 0 as *mut libc::c_void,
        q: 0 as *mut libc::c_void,
        i: 0,
        j: 0,
    };
    let mut elt: *const libc::c_void = 0 as *const libc::c_void;
    iter = gl_list_iterator(files_to_output);
    while gl_list_iterator_next(&mut iter, &mut elt, 0 as *mut gl_list_node_t) {
        let mut file_to_output: *const file_to_output = elt as *const file_to_output;
        let mut from_needs_removal: bool = 1 as libc::c_int != 0;
        let mut from_st: *const stat = &(*file_to_output).from_st;
        output_file_now(
            (*file_to_output).from,
            &mut from_needs_removal,
            from_st,
            (*file_to_output).to,
            (*file_to_output).mode,
            (*file_to_output).backup,
        );
        if !((*file_to_output).to).is_null() && from_needs_removal as libc::c_int != 0 {
            safe_unlink((*file_to_output).from);
        }
        if !st.is_null() && (*st).st_dev == (*from_st).st_dev
            && (*st).st_ino == (*from_st).st_ino
        {
            loop {
                let mut elt2: *const libc::c_void = gl_list_get_at(
                    files_to_output,
                    0 as libc::c_int as size_t,
                );
                gl_list_remove_at(files_to_output, 0 as libc::c_int as size_t);
                if elt == elt2 {
                    break;
                }
            }
            gl_list_iterator_free(&mut iter);
            return;
        }
    }
    gl_list_iterator_free(&mut iter);
    gl_list_clear(files_to_output);
}
pub unsafe extern "C" fn fatal_exit(mut sig: libc::c_int) -> ! {
    cleanup();
    if sig != 0 {
        exit_with_signal(sig);
    }
    exit(2 as libc::c_int);
}
unsafe extern "C" fn remove_if_needed(
    mut name: *const libc::c_char,
    mut needs_removal: *mut bool,
) {
    if *needs_removal {
        safe_unlink(name);
        *needs_removal = 0 as libc::c_int != 0;
    }
}
unsafe extern "C" fn cleanup() {
    remove_if_needed(TMPINNAME, &mut TMPINNAME_needs_removal);
    remove_if_needed(TMPOUTNAME, &mut TMPOUTNAME_needs_removal);
    remove_if_needed(TMPPATNAME, &mut TMPPATNAME_needs_removal);
    remove_if_needed(TMPREJNAME, &mut TMPREJNAME_needs_removal);
    output_files(0 as *const stat);
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
