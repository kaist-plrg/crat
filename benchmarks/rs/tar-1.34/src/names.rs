use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type hash_table;
    pub type quoting_options;
    pub type wordsplit_node;
    pub type exclist;
    pub type exclude;
    pub type directory;
    pub type textual_date;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn openat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __oflag: libc::c_int,
        _: ...
    ) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    static mut stdin: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn __uflow(_: *mut FILE) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xzalloc(s: size_t) -> *mut libc::c_void;
    fn x2realloc(p: *mut libc::c_void, pn: *mut size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn getgrgid(__gid: __gid_t) -> *mut group;
    fn getgrnam(__name: *const libc::c_char) -> *mut group;
    static mut error_hook: Option::<unsafe extern "C" fn() -> ()>;
    static mut exit_status: libc::c_int;
    fn open_fatal(_: *const libc::c_char) -> !;
    fn stat_fatal(_: *const libc::c_char) -> !;
    fn fatal_exit() -> !;
    fn hash_lookup(
        table: *const Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_string(string: *const libc::c_char, n_buckets: size_t) -> size_t;
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_free(table: *mut Hash_table);
    fn hash_insert(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_remove(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn set_char_quoting(
        o: *mut quoting_options,
        c: libc::c_char,
        i: libc::c_int,
    ) -> libc::c_int;
    fn quotearg_n(n: libc::c_int, arg: *const libc::c_char) -> *mut libc::c_char;
    fn quotearg_colon(arg: *const libc::c_char) -> *mut libc::c_char;
    fn wordsplit(
        s: *const libc::c_char,
        ws: *mut wordsplit_t,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn wordsplit_free(ws: *mut wordsplit_t);
    fn wordsplit_strerror(ws: *mut wordsplit_t) -> *const libc::c_char;
    static mut subcommand_option: subcommand;
    static mut excluded: *mut exclude;
    static mut occurrence_option: uintmax_t;
    static mut listed_incremental_option: *const libc::c_char;
    static mut recursion_option: libc::c_int;
    static mut same_order_option: bool;
    static mut starting_file_option: bool;
    static mut verbose_option: libc::c_int;
    static mut open_read_flags: libc::c_int;
    static mut unquote_option: bool;
    fn add_exclusion_tag(
        name: *const libc::c_char,
        type_0: exclusion_tag_type,
        predicate: Option::<unsafe extern "C" fn(libc::c_int) -> bool>,
    );
    fn cachedir_file_p(fd: libc::c_int) -> bool;
    fn subfile_open(
        dir: *const tar_stat_info,
        file: *const libc::c_char,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn restore_parent_fd(st: *const tar_stat_info);
    fn scan_directory(st: *mut tar_stat_info) -> *mut directory;
    fn directory_contents(dir: *mut directory) -> *const libc::c_char;
    fn rebase_directory(
        dir: *mut directory,
        samp: *const libc::c_char,
        slen: size_t,
        repl: *const libc::c_char,
        rlen: size_t,
    );
    fn append_incremental_renames(dir: *mut directory);
    fn read_directory_file();
    fn assign_string(dest: *mut *mut libc::c_char, src: *const libc::c_char);
    fn unquote_string(str: *mut libc::c_char) -> libc::c_int;
    fn normalize_filename(
        cdidx: libc::c_int,
        name: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn deref_stat(name: *const libc::c_char, buf: *mut stat) -> libc::c_int;
    static mut chdir_fd: libc::c_int;
    fn chdir_arg(dir: *const libc::c_char) -> libc::c_int;
    fn chdir_do(dir: libc::c_int);
    fn chdir_count() -> libc::c_int;
    fn open_diag(name: *const libc::c_char);
    fn stat_diag(name: *const libc::c_char);
    fn usage(_: libc::c_int);
    fn tar_stat_init(st: *mut tar_stat_info);
    fn tar_stat_destroy(st: *mut tar_stat_info);
    fn set_exit_status(val: libc::c_int);
    fn request_stdin(rpl_option: *const libc::c_char);
    fn more_options(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        loc: *mut option_locus,
    );
    static mut warning_option: libc::c_int;
    fn excfile_add(name: *const libc::c_char, flags: libc::c_int);
    fn exclude_vcs_ignores();
    static mut program_name: *const libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn fnmatch_pattern_has_wildcards(_: *const libc::c_char, _: libc::c_int) -> bool;
    fn add_exclude(_: *mut exclude, _: *const libc::c_char, _: libc::c_int);
    fn add_exclude_file(
        _: Option::<
            unsafe extern "C" fn(*mut exclude, *const libc::c_char, libc::c_int) -> (),
        >,
        _: *mut exclude,
        _: *const libc::c_char,
        _: libc::c_int,
        _: libc::c_char,
    ) -> libc::c_int;
    fn exclude_fnmatch(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> bool;
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
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type error_t = libc::c_int;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut libc::c_char,
    pub next_free: *mut libc::c_char,
    pub chunk_limit: *mut libc::c_char,
    pub temp: C2RustUnnamed_2,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_1,
    pub freefun: C2RustUnnamed_0,
    pub extra_arg: *mut libc::c_void,
    #[bitfield(name = "use_extra_arg", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "maybe_empty_object", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "alloc_failed", ty = "libc::c_uint", bits = "2..=2")]
    pub use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub plain: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub plain: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub i: size_t,
    pub p: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut libc::c_char,
    pub prev: *mut _obstack_chunk,
    pub contents: [libc::c_char; 0],
}
pub type uintmax_t = __uintmax_t;
pub type DIR = __dirstream;
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
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: libc::c_float,
    pub shrink_factor: libc::c_float,
    pub growth_threshold: libc::c_float,
    pub growth_factor: libc::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_tuning = hash_tuning;
pub type Hash_table = hash_table;
pub type Hash_hasher = Option::<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
pub type Hash_comparator = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_data_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wordsplit {
    pub ws_wordc: size_t,
    pub ws_wordv: *mut *mut libc::c_char,
    pub ws_offs: size_t,
    pub ws_wordn: size_t,
    pub ws_flags: libc::c_uint,
    pub ws_options: libc::c_uint,
    pub ws_maxwords: size_t,
    pub ws_wordi: size_t,
    pub ws_delim: *const libc::c_char,
    pub ws_comment: *const libc::c_char,
    pub ws_escape: [*const libc::c_char; 2],
    pub ws_alloc_die: Option::<unsafe extern "C" fn(*mut wordsplit_t) -> ()>,
    pub ws_error: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>,
    pub ws_debug: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>,
    pub ws_env: *mut *const libc::c_char,
    pub ws_envbuf: *mut *mut libc::c_char,
    pub ws_envidx: size_t,
    pub ws_envsiz: size_t,
    pub ws_getvar: Option::<
        unsafe extern "C" fn(
            *mut *mut libc::c_char,
            *const libc::c_char,
            size_t,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub ws_closure: *mut libc::c_void,
    pub ws_command: Option::<
        unsafe extern "C" fn(
            *mut *mut libc::c_char,
            *const libc::c_char,
            size_t,
            *mut *mut libc::c_char,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub ws_input: *const libc::c_char,
    pub ws_len: size_t,
    pub ws_endp: size_t,
    pub ws_errno: libc::c_int,
    pub ws_usererr: *mut libc::c_char,
    pub ws_head: *mut wordsplit_node,
    pub ws_tail: *mut wordsplit_node,
    pub ws_lvl: libc::c_int,
}
pub type wordsplit_t = wordsplit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_option {
    pub name: *const libc::c_char,
    pub key: libc::c_int,
    pub arg: *const libc::c_char,
    pub flags: libc::c_int,
    pub doc: *const libc::c_char,
    pub group: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp {
    pub options: *const argp_option,
    pub parser: argp_parser_t,
    pub args_doc: *const libc::c_char,
    pub doc: *const libc::c_char,
    pub children: *const argp_child,
    pub help_filter: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_char,
            *mut libc::c_void,
        ) -> *mut libc::c_char,
    >,
    pub argp_domain: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_child {
    pub argp: *const argp,
    pub flags: libc::c_int,
    pub header: *const libc::c_char,
    pub group: libc::c_int,
}
pub type argp_parser_t = Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_char, *mut argp_state) -> error_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_state {
    pub root_argp: *const argp,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub next: libc::c_int,
    pub flags: libc::c_uint,
    pub arg_num: libc::c_uint,
    pub quoted: libc::c_int,
    pub input: *mut libc::c_void,
    pub child_inputs: *mut *mut libc::c_void,
    pub hook: *mut libc::c_void,
    pub name: *mut libc::c_char,
    pub err_stream: *mut FILE,
    pub out_stream: *mut FILE,
    pub pstate: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sp_array {
    pub offset: off_t,
    pub numbytes: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xheader {
    pub stk: *mut obstack,
    pub size: size_t,
    pub buffer: *mut libc::c_char,
    pub string_length: uintmax_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xattr_array {
    pub xkey: *mut libc::c_char,
    pub xval_ptr: *mut libc::c_char,
    pub xval_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tar_stat_info {
    pub orig_file_name: *mut libc::c_char,
    pub file_name: *mut libc::c_char,
    pub had_trailing_slash: bool,
    pub link_name: *mut libc::c_char,
    pub uname: *mut libc::c_char,
    pub gname: *mut libc::c_char,
    pub cntx_name: *mut libc::c_char,
    pub acls_a_ptr: *mut libc::c_char,
    pub acls_a_len: size_t,
    pub acls_d_ptr: *mut libc::c_char,
    pub acls_d_len: size_t,
    pub stat: stat,
    pub atime: timespec,
    pub mtime: timespec,
    pub ctime: timespec,
    pub archive_file_size: off_t,
    pub is_sparse: bool,
    pub sparse_major: libc::c_uint,
    pub sparse_minor: libc::c_uint,
    pub sparse_map_avail: size_t,
    pub sparse_map_size: size_t,
    pub sparse_map: *mut sp_array,
    pub real_size: off_t,
    pub real_size_set: bool,
    pub sparse_name_done: bool,
    pub xattr_map_size: size_t,
    pub xattr_map: *mut xattr_array,
    pub xhdr: xheader,
    pub is_dumpdir: bool,
    pub skipped: bool,
    pub dumpdir: *mut libc::c_char,
    pub parent: *mut tar_stat_info,
    pub dirstream: *mut DIR,
    pub fd: libc::c_int,
    pub exclude_list: *mut exclist,
}
pub type subcommand = libc::c_uint;
pub const TEST_LABEL_SUBCOMMAND: subcommand = 9;
pub const UPDATE_SUBCOMMAND: subcommand = 8;
pub const LIST_SUBCOMMAND: subcommand = 7;
pub const EXTRACT_SUBCOMMAND: subcommand = 6;
pub const DIFF_SUBCOMMAND: subcommand = 5;
pub const DELETE_SUBCOMMAND: subcommand = 4;
pub const CREATE_SUBCOMMAND: subcommand = 3;
pub const CAT_SUBCOMMAND: subcommand = 2;
pub const APPEND_SUBCOMMAND: subcommand = 1;
pub const UNKNOWN_SUBCOMMAND: subcommand = 0;
pub type exclusion_tag_type = libc::c_uint;
pub const exclusion_tag_all: exclusion_tag_type = 3;
pub const exclusion_tag_under: exclusion_tag_type = 2;
pub const exclusion_tag_contents: exclusion_tag_type = 1;
pub const exclusion_tag_none: exclusion_tag_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct name {
    pub next: *mut name,
    pub prev: *mut name,
    pub name: *mut libc::c_char,
    pub length: size_t,
    pub matching_flags: libc::c_int,
    pub cmdline: bool,
    pub change_dir: libc::c_int,
    pub found_count: uintmax_t,
    pub directory: *mut directory,
    pub parent: *mut name,
    pub child: *mut name,
    pub sibling: *mut name,
    pub caname: *mut libc::c_char,
}
pub type files_count = libc::c_uint;
pub const FILES_MANY: files_count = 2;
pub const FILES_ONE: files_count = 1;
pub const FILES_NONE: files_count = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct name_elt {
    pub next: *mut name_elt,
    pub prev: *mut name_elt,
    pub type_0: nelt_type,
    pub v: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub name: *const libc::c_char,
    pub file: C2RustUnnamed_5,
    pub opt: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub rpl_option: libc::c_int,
    pub arg: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub name: *const libc::c_char,
    pub line: size_t,
    pub term: libc::c_int,
    pub verbatim: bool,
    pub fp: *mut FILE,
}
pub type nelt_type = libc::c_uint;
pub const NELT_OPTION: nelt_type = 4;
pub const NELT_NOOP: nelt_type = 3;
pub const NELT_FILE: nelt_type = 2;
pub const NELT_CHDIR: nelt_type = 1;
pub const NELT_NAME: nelt_type = 0;
pub const GRID_MATCH: C2RustUnnamed_7 = 3;
pub const NO_WILDCARDS_MATCH_SLASH_OPTION: C2RustUnnamed_6 = 278;
pub const WILDCARDS_MATCH_SLASH_OPTION: C2RustUnnamed_6 = 283;
pub const NO_WILDCARDS_OPTION: C2RustUnnamed_6 = 279;
pub const WILDCARDS_OPTION: C2RustUnnamed_6 = 284;
pub const NO_IGNORE_CASE_OPTION: C2RustUnnamed_6 = 270;
pub const IGNORE_CASE_OPTION: C2RustUnnamed_6 = 269;
pub const NO_ANCHORED_OPTION: C2RustUnnamed_6 = 272;
pub const ANCHORED_OPTION: C2RustUnnamed_6 = 271;
pub const GRH_MATCH: C2RustUnnamed_7 = 2;
pub const GRID_LOCAL: C2RustUnnamed_7 = 1;
pub const NO_RECURSION_OPTION: C2RustUnnamed_6 = 274;
pub const RECURSION_OPTION: C2RustUnnamed_6 = 273;
pub const EXCLUDE_BACKUPS_OPTION: C2RustUnnamed_6 = 257;
pub const EXCLUDE_VCS_IGNORES_OPTION: C2RustUnnamed_6 = 268;
pub const EXCLUDE_VCS_OPTION: C2RustUnnamed_6 = 267;
pub const EXCLUDE_TAG_ALL_OPTION: C2RustUnnamed_6 = 266;
pub const EXCLUDE_TAG_UNDER_OPTION: C2RustUnnamed_6 = 265;
pub const EXCLUDE_IGNORE_RECURSIVE_OPTION: C2RustUnnamed_6 = 263;
pub const EXCLUDE_IGNORE_OPTION: C2RustUnnamed_6 = 262;
pub const EXCLUDE_TAG_OPTION: C2RustUnnamed_6 = 264;
pub const EXCLUDE_CACHES_ALL_OPTION: C2RustUnnamed_6 = 260;
pub const EXCLUDE_CACHES_UNDER_OPTION: C2RustUnnamed_6 = 259;
pub const EXCLUDE_CACHES_OPTION: C2RustUnnamed_6 = 258;
pub const EXCLUDE_OPTION: C2RustUnnamed_6 = 261;
pub const NO_VERBATIM_FILES_FROM_OPTION: C2RustUnnamed_6 = 277;
pub const VERBATIM_FILES_FROM_OPTION: C2RustUnnamed_6 = 282;
pub const NO_UNQUOTE_OPTION: C2RustUnnamed_6 = 276;
pub const UNQUOTE_OPTION: C2RustUnnamed_6 = 275;
pub const NO_NULL_OPTION: C2RustUnnamed_6 = 281;
pub const NULL_OPTION: C2RustUnnamed_6 = 280;
pub const ADD_FILE_OPTION: C2RustUnnamed_6 = 256;
pub const GRH_LOCAL: C2RustUnnamed_7 = 0;
pub type wildcards = libc::c_uint;
pub const enable_wildcards: wildcards = 2;
pub const disable_wildcards: wildcards = 1;
pub const default_wildcards: wildcards = 0;
pub const file_list_end: read_file_list_state = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option_locus {
    pub source: option_source,
    pub name: *const libc::c_char,
    pub line: size_t,
    pub prev: *mut option_locus,
}
pub type option_source = libc::c_uint;
pub const OPTS_FILE: option_source = 2;
pub const OPTS_COMMAND_LINE: option_source = 1;
pub const OPTS_ENVIRON: option_source = 0;
pub const file_list_success: read_file_list_state = 0;
pub const file_list_zero: read_file_list_state = 2;
pub const file_list_skip: read_file_list_state = 3;
pub type read_file_list_state = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_id_list {
    pub next: *mut file_id_list,
    pub ino: ino_t,
    pub dev: dev_t,
    pub from_file: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tar_args {
    pub loc: *mut option_locus,
    pub textual_date: *mut textual_date,
    pub o_option: bool,
    pub pax_option: bool,
    pub compress_autodetect: bool,
    pub backup_suffix_string: *const libc::c_char,
    pub version_control_string: *const libc::c_char,
}
pub type C2RustUnnamed_6 = libc::c_uint;
pub type C2RustUnnamed_7 = libc::c_uint;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int
        as libc::c_long != 0
    {
        __uflow(__fp)
    } else {
        let fresh0 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = ((*__fp)._IO_read_ptr).offset(1);
        *(fresh0 as *mut libc::c_uchar) as libc::c_int
    };
}
static mut names_options: [argp_option; 35] = [
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Local file name selection:\0" as *const u8 as *const libc::c_char,
            group: GRH_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"add-file\0" as *const u8 as *const libc::c_char,
            key: ADD_FILE_OPTION as libc::c_int,
            arg: b"FILE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"add given FILE to the archive (useful if its name starts with a dash)\0"
                as *const u8 as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"directory\0" as *const u8 as *const libc::c_char,
            key: 'C' as i32,
            arg: b"DIR\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"change to directory DIR\0" as *const u8 as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"files-from\0" as *const u8 as *const libc::c_char,
            key: 'T' as i32,
            arg: b"FILE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"get names to extract or create from FILE\0" as *const u8
                as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"null\0" as *const u8 as *const libc::c_char,
            key: NULL_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"-T reads null-terminated names; implies --verbatim-files-from\0"
                as *const u8 as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-null\0" as *const u8 as *const libc::c_char,
            key: NO_NULL_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"disable the effect of the previous --null option\0" as *const u8
                as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"unquote\0" as *const u8 as *const libc::c_char,
            key: UNQUOTE_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"unquote input file or member names (default)\0" as *const u8
                as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-unquote\0" as *const u8 as *const libc::c_char,
            key: NO_UNQUOTE_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"do not unquote input file or member names\0" as *const u8
                as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"verbatim-files-from\0" as *const u8 as *const libc::c_char,
            key: VERBATIM_FILES_FROM_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"-T reads file names verbatim (no escape or option handling)\0"
                as *const u8 as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-verbatim-files-from\0" as *const u8 as *const libc::c_char,
            key: NO_VERBATIM_FILES_FROM_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"-T treats file names starting with dash as options (default)\0"
                as *const u8 as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude\0" as *const u8 as *const libc::c_char,
            key: EXCLUDE_OPTION as libc::c_int,
            arg: b"PATTERN\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"exclude files, given as a PATTERN\0" as *const u8
                as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-from\0" as *const u8 as *const libc::c_char,
            key: 'X' as i32,
            arg: b"FILE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"exclude patterns listed in FILE\0" as *const u8
                as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-caches\0" as *const u8 as *const libc::c_char,
            key: EXCLUDE_CACHES_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"exclude contents of directories containing CACHEDIR.TAG, except for the tag file itself\0"
                as *const u8 as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-caches-under\0" as *const u8 as *const libc::c_char,
            key: EXCLUDE_CACHES_UNDER_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"exclude everything under directories containing CACHEDIR.TAG\0"
                as *const u8 as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-caches-all\0" as *const u8 as *const libc::c_char,
            key: EXCLUDE_CACHES_ALL_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"exclude directories containing CACHEDIR.TAG\0" as *const u8
                as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-tag\0" as *const u8 as *const libc::c_char,
            key: EXCLUDE_TAG_OPTION as libc::c_int,
            arg: b"FILE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"exclude contents of directories containing FILE, except for FILE itself\0"
                as *const u8 as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-ignore\0" as *const u8 as *const libc::c_char,
            key: EXCLUDE_IGNORE_OPTION as libc::c_int,
            arg: b"FILE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"read exclude patterns for each directory from FILE, if it exists\0"
                as *const u8 as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-ignore-recursive\0" as *const u8 as *const libc::c_char,
            key: EXCLUDE_IGNORE_RECURSIVE_OPTION as libc::c_int,
            arg: b"FILE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"read exclude patterns for each directory and its subdirectories from FILE, if it exists\0"
                as *const u8 as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-tag-under\0" as *const u8 as *const libc::c_char,
            key: EXCLUDE_TAG_UNDER_OPTION as libc::c_int,
            arg: b"FILE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"exclude everything under directories containing FILE\0" as *const u8
                as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-tag-all\0" as *const u8 as *const libc::c_char,
            key: EXCLUDE_TAG_ALL_OPTION as libc::c_int,
            arg: b"FILE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"exclude directories containing FILE\0" as *const u8
                as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-vcs\0" as *const u8 as *const libc::c_char,
            key: EXCLUDE_VCS_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"exclude version control system directories\0" as *const u8
                as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-vcs-ignores\0" as *const u8 as *const libc::c_char,
            key: EXCLUDE_VCS_IGNORES_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"read exclude patterns from the VCS ignore files\0" as *const u8
                as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-backups\0" as *const u8 as *const libc::c_char,
            key: EXCLUDE_BACKUPS_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"exclude backup and lock files\0" as *const u8 as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"recursion\0" as *const u8 as *const libc::c_char,
            key: RECURSION_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"recurse into directories (default)\0" as *const u8
                as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-recursion\0" as *const u8 as *const libc::c_char,
            key: NO_RECURSION_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"avoid descending automatically in directories\0" as *const u8
                as *const libc::c_char,
            group: GRID_LOCAL as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"File name matching options (affect both exclude and include patterns):\0"
                as *const u8 as *const libc::c_char,
            group: GRH_MATCH as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"anchored\0" as *const u8 as *const libc::c_char,
            key: ANCHORED_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"patterns match file name start\0" as *const u8 as *const libc::c_char,
            group: GRID_MATCH as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-anchored\0" as *const u8 as *const libc::c_char,
            key: NO_ANCHORED_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"patterns match after any '/' (default for exclusion)\0" as *const u8
                as *const libc::c_char,
            group: GRID_MATCH as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"ignore-case\0" as *const u8 as *const libc::c_char,
            key: IGNORE_CASE_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"ignore case\0" as *const u8 as *const libc::c_char,
            group: GRID_MATCH as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-ignore-case\0" as *const u8 as *const libc::c_char,
            key: NO_IGNORE_CASE_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"case sensitive matching (default)\0" as *const u8
                as *const libc::c_char,
            group: GRID_MATCH as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"wildcards\0" as *const u8 as *const libc::c_char,
            key: WILDCARDS_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"use wildcards (default for exclusion)\0" as *const u8
                as *const libc::c_char,
            group: GRID_MATCH as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-wildcards\0" as *const u8 as *const libc::c_char,
            key: NO_WILDCARDS_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"verbatim string matching\0" as *const u8 as *const libc::c_char,
            group: GRID_MATCH as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"wildcards-match-slash\0" as *const u8 as *const libc::c_char,
            key: WILDCARDS_MATCH_SLASH_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"wildcards match '/' (default for exclusion)\0" as *const u8
                as *const libc::c_char,
            group: GRID_MATCH as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-wildcards-match-slash\0" as *const u8 as *const libc::c_char,
            key: NO_WILDCARDS_MATCH_SLASH_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"wildcards do not match '/'\0" as *const u8 as *const libc::c_char,
            group: GRID_MATCH as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0,
            arg: 0 as *const libc::c_char,
            flags: 0,
            doc: 0 as *const libc::c_char,
            group: 0,
        };
        init
    },
];
unsafe extern "C" fn file_selection_option(mut key: libc::c_int) -> *const argp_option {
    let mut p: *mut argp_option = 0 as *mut argp_option;
    p = names_options.as_mut_ptr();
    while !(((*p).name).is_null() && (*p).key == 0 as libc::c_int
        && ((*p).doc).is_null())
    {
        if (*p).key == key {
            return p;
        }
        p = p.offset(1);
        p;
    }
    return 0 as *const argp_option;
}
unsafe extern "C" fn file_selection_option_name(
    mut key: libc::c_int,
) -> *const libc::c_char {
    let mut opt: *const argp_option = file_selection_option(key);
    return if !opt.is_null() { (*opt).name } else { 0 as *const libc::c_char };
}
unsafe extern "C" fn is_file_selection_option(mut key: libc::c_int) -> bool {
    return !(file_selection_option(key)).is_null();
}
static mut filename_terminator: libc::c_char = '\n' as i32 as libc::c_char;
static mut verbatim_files_from_option: bool = false;
unsafe extern "C" fn names_parse_opt(
    mut key: libc::c_int,
    mut arg: *mut libc::c_char,
    mut state: *mut argp_state,
) -> error_t {
    match key {
        67 => {
            name_add_dir(arg);
        }
        84 => {
            name_add_file(arg);
        }
        256 => {
            name_add_name(arg);
        }
        16777221 => {
            let mut args: *mut tar_args = (*state).input as *mut tar_args;
            if (*(*args).loc).source as libc::c_uint
                == OPTS_FILE as libc::c_int as libc::c_uint
            {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s:%lu: unrecognized option\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*(*args).loc).name,
                    (*(*args).loc).line,
                );
                set_exit_status(2 as libc::c_int);
            }
            return 7 as libc::c_int;
        }
        _ => {
            if is_file_selection_option(key) {
                name_add_option(key, arg);
            } else {
                return 7 as libc::c_int
            }
        }
    }
    return 0 as libc::c_int;
}
static mut wildcards: wildcards = default_wildcards;
static mut matching_flags: libc::c_int = 0 as libc::c_int;
static mut include_anchored: libc::c_int = (1 as libc::c_int) << 30 as libc::c_int;
static mut vcs_file_table: [*const libc::c_char; 22] = [
    b"CVS\0" as *const u8 as *const libc::c_char,
    b".cvsignore\0" as *const u8 as *const libc::c_char,
    b"RCS\0" as *const u8 as *const libc::c_char,
    b"SCCS\0" as *const u8 as *const libc::c_char,
    b".svn\0" as *const u8 as *const libc::c_char,
    b".git\0" as *const u8 as *const libc::c_char,
    b".gitignore\0" as *const u8 as *const libc::c_char,
    b".gitattributes\0" as *const u8 as *const libc::c_char,
    b".gitmodules\0" as *const u8 as *const libc::c_char,
    b".arch-ids\0" as *const u8 as *const libc::c_char,
    b"{arch}\0" as *const u8 as *const libc::c_char,
    b"=RELEASE-ID\0" as *const u8 as *const libc::c_char,
    b"=meta-update\0" as *const u8 as *const libc::c_char,
    b"=update\0" as *const u8 as *const libc::c_char,
    b".bzr\0" as *const u8 as *const libc::c_char,
    b".bzrignore\0" as *const u8 as *const libc::c_char,
    b".bzrtags\0" as *const u8 as *const libc::c_char,
    b".hg\0" as *const u8 as *const libc::c_char,
    b".hgignore\0" as *const u8 as *const libc::c_char,
    b".hgtags\0" as *const u8 as *const libc::c_char,
    b"_darcs\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut backup_file_table: [*const libc::c_char; 4] = [
    b".#*\0" as *const u8 as *const libc::c_char,
    b"*~\0" as *const u8 as *const libc::c_char,
    b"#*#\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
unsafe extern "C" fn add_exclude_array(
    mut fv: *const *const libc::c_char,
    mut opts: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(*fv.offset(i as isize)).is_null() {
        add_exclude(excluded, *fv.offset(i as isize), opts);
        i += 1;
        i;
    }
}
unsafe extern "C" fn handle_file_selection_option(
    mut key: libc::c_int,
    mut arg: *const libc::c_char,
) {
    match key {
        257 => {
            add_exclude_array(
                backup_file_table.as_ptr(),
                (1 as libc::c_int) << 28 as libc::c_int,
            );
        }
        261 => {
            add_exclude(
                excluded,
                arg,
                (if wildcards as libc::c_uint
                    != disable_wildcards as libc::c_int as libc::c_uint
                {
                    (1 as libc::c_int) << 28 as libc::c_int
                } else {
                    0 as libc::c_int
                }) | matching_flags | recursion_option,
            );
        }
        258 => {
            add_exclusion_tag(
                b"CACHEDIR.TAG\0" as *const u8 as *const libc::c_char,
                exclusion_tag_contents,
                Some(cachedir_file_p as unsafe extern "C" fn(libc::c_int) -> bool),
            );
        }
        259 => {
            add_exclusion_tag(
                b"CACHEDIR.TAG\0" as *const u8 as *const libc::c_char,
                exclusion_tag_under,
                Some(cachedir_file_p as unsafe extern "C" fn(libc::c_int) -> bool),
            );
        }
        260 => {
            add_exclusion_tag(
                b"CACHEDIR.TAG\0" as *const u8 as *const libc::c_char,
                exclusion_tag_all,
                Some(cachedir_file_p as unsafe extern "C" fn(libc::c_int) -> bool),
            );
        }
        262 => {
            excfile_add(arg, 0x2 as libc::c_int);
        }
        263 => {
            excfile_add(arg, 0x1 as libc::c_int);
        }
        264 => {
            add_exclusion_tag(arg, exclusion_tag_contents, None);
        }
        265 => {
            add_exclusion_tag(arg, exclusion_tag_under, None);
        }
        266 => {
            add_exclusion_tag(arg, exclusion_tag_all, None);
        }
        267 => {
            add_exclude_array(vcs_file_table.as_ptr(), 0 as libc::c_int);
        }
        268 => {
            exclude_vcs_ignores();
        }
        273 => {
            recursion_option = (1 as libc::c_int) << 3 as libc::c_int;
        }
        274 => {
            recursion_option = 0 as libc::c_int;
        }
        275 => {
            unquote_option = 1 as libc::c_int != 0;
        }
        276 => {
            unquote_option = 0 as libc::c_int != 0;
        }
        280 => {
            filename_terminator = '\0' as i32 as libc::c_char;
            verbatim_files_from_option = 1 as libc::c_int != 0;
        }
        281 => {
            filename_terminator = '\n' as i32 as libc::c_char;
            verbatim_files_from_option = 0 as libc::c_int != 0;
        }
        88 => {
            if add_exclude_file(
                Some(
                    add_exclude
                        as unsafe extern "C" fn(
                            *mut exclude,
                            *const libc::c_char,
                            libc::c_int,
                        ) -> (),
                ),
                excluded,
                arg,
                (if wildcards as libc::c_uint
                    != disable_wildcards as libc::c_int as libc::c_uint
                {
                    (1 as libc::c_int) << 28 as libc::c_int
                } else {
                    0 as libc::c_int
                }) | matching_flags | recursion_option,
                '\n' as i32 as libc::c_char,
            ) != 0 as libc::c_int
            {
                let mut e: libc::c_int = *__errno_location();
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    e,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_colon(arg),
                );
                fatal_exit();
            }
        }
        271 => {
            matching_flags |= (1 as libc::c_int) << 30 as libc::c_int;
        }
        272 => {
            include_anchored = 0 as libc::c_int;
            matching_flags &= !((1 as libc::c_int) << 30 as libc::c_int);
        }
        269 => {
            matching_flags |= (1 as libc::c_int) << 4 as libc::c_int;
        }
        270 => {
            matching_flags &= !((1 as libc::c_int) << 4 as libc::c_int);
        }
        284 => {
            wildcards = enable_wildcards;
        }
        279 => {
            wildcards = disable_wildcards;
        }
        283 => {
            matching_flags &= !((1 as libc::c_int) << 0 as libc::c_int);
        }
        278 => {
            matching_flags |= (1 as libc::c_int) << 0 as libc::c_int;
        }
        282 => {
            verbatim_files_from_option = 1 as libc::c_int != 0;
        }
        277 => {
            verbatim_files_from_option = 0 as libc::c_int != 0;
        }
        _ => {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                b"unhandled positional option %d\0" as *const u8 as *const libc::c_char,
                key,
            );
            fatal_exit();
        }
    };
}
pub static mut names_argp: argp = unsafe {
    {
        let mut init = argp {
            options: names_options.as_ptr() as *mut _,
            parser: Some(
                names_parse_opt
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut libc::c_char,
                        *mut argp_state,
                    ) -> error_t,
            ),
            args_doc: 0 as *const libc::c_char,
            doc: 0 as *const libc::c_char,
            children: 0 as *const argp_child,
            help_filter: None,
            argp_domain: 0 as *const libc::c_char,
        };
        init
    }
};
static mut cached_uname: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut cached_gname: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut cached_uid: uid_t = 0;
static mut cached_gid: gid_t = 0;
static mut cached_no_such_uname: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut cached_no_such_gname: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut cached_no_such_uid: uid_t = 0;
static mut cached_no_such_gid: gid_t = 0;
pub unsafe extern "C" fn uid_to_uname(
    mut uid: uid_t,
    mut uname: *mut *mut libc::c_char,
) {
    let mut passwd: *mut passwd = 0 as *mut passwd;
    if uid != 0 as libc::c_int as libc::c_uint && uid == cached_no_such_uid {
        *uname = xstrdup(b"\0" as *const u8 as *const libc::c_char);
        return;
    }
    if cached_uname.is_null() || uid != cached_uid {
        passwd = getpwuid(uid);
        if !passwd.is_null() {
            cached_uid = uid;
            assign_string(&mut cached_uname, (*passwd).pw_name);
        } else {
            cached_no_such_uid = uid;
            *uname = xstrdup(b"\0" as *const u8 as *const libc::c_char);
            return;
        }
    }
    *uname = xstrdup(cached_uname);
}
pub unsafe extern "C" fn gid_to_gname(
    mut gid: gid_t,
    mut gname: *mut *mut libc::c_char,
) {
    let mut group: *mut group = 0 as *mut group;
    if gid != 0 as libc::c_int as libc::c_uint && gid == cached_no_such_gid {
        *gname = xstrdup(b"\0" as *const u8 as *const libc::c_char);
        return;
    }
    if cached_gname.is_null() || gid != cached_gid {
        group = getgrgid(gid);
        if !group.is_null() {
            cached_gid = gid;
            assign_string(&mut cached_gname, (*group).gr_name);
        } else {
            cached_no_such_gid = gid;
            *gname = xstrdup(b"\0" as *const u8 as *const libc::c_char);
            return;
        }
    }
    *gname = xstrdup(cached_gname);
}
pub unsafe extern "C" fn uname_to_uid(
    mut uname: *const libc::c_char,
    mut uidp: *mut uid_t,
) -> libc::c_int {
    let mut passwd: *mut passwd = 0 as *mut passwd;
    if !cached_no_such_uname.is_null()
        && strcmp(uname, cached_no_such_uname) == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if cached_uname.is_null()
        || *uname.offset(0 as libc::c_int as isize) as libc::c_int
            != *cached_uname.offset(0 as libc::c_int as isize) as libc::c_int
        || strcmp(uname, cached_uname) != 0 as libc::c_int
    {
        passwd = getpwnam(uname);
        if !passwd.is_null() {
            cached_uid = (*passwd).pw_uid;
            assign_string(&mut cached_uname, (*passwd).pw_name);
        } else {
            assign_string(&mut cached_no_such_uname, uname);
            return 0 as libc::c_int;
        }
    }
    *uidp = cached_uid;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn gname_to_gid(
    mut gname: *const libc::c_char,
    mut gidp: *mut gid_t,
) -> libc::c_int {
    let mut group: *mut group = 0 as *mut group;
    if !cached_no_such_gname.is_null()
        && strcmp(gname, cached_no_such_gname) == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if cached_gname.is_null()
        || *gname.offset(0 as libc::c_int as isize) as libc::c_int
            != *cached_gname.offset(0 as libc::c_int as isize) as libc::c_int
        || strcmp(gname, cached_gname) != 0 as libc::c_int
    {
        group = getgrnam(gname);
        if !group.is_null() {
            cached_gid = (*group).gr_gid;
            assign_string(&mut cached_gname, gname);
        } else {
            assign_string(&mut cached_no_such_gname, gname);
            return 0 as libc::c_int;
        }
    }
    *gidp = cached_gid;
    return 1 as libc::c_int;
}
unsafe extern "C" fn make_name(mut file_name: *const libc::c_char) -> *mut name {
    let mut p: *mut name = xzalloc(::std::mem::size_of::<name>() as libc::c_ulong)
        as *mut name;
    if file_name.is_null() {
        file_name = b"\0" as *const u8 as *const libc::c_char;
    }
    (*p).name = xstrdup(file_name);
    (*p).length = strlen((*p).name);
    return p;
}
unsafe extern "C" fn free_name(mut p: *mut name) {
    if !p.is_null() {
        rpl_free((*p).name as *mut libc::c_void);
        rpl_free((*p).caname as *mut libc::c_void);
        rpl_free(p as *mut libc::c_void);
    }
}
static mut namelist: *mut name = 0 as *const name as *mut name;
static mut nametail: *mut name = 0 as *const name as *mut name;
static mut name_head: *mut name_elt = 0 as *const name_elt as *mut name_elt;
pub static mut filename_args: files_count = FILES_NONE;
unsafe extern "C" fn name_elt_alloc() -> *mut name_elt {
    let mut elt: *mut name_elt = 0 as *mut name_elt;
    elt = xmalloc(::std::mem::size_of::<name_elt>() as libc::c_ulong) as *mut name_elt;
    if name_head.is_null() {
        name_head = elt;
        (*name_head).next = 0 as *mut name_elt;
        (*name_head).prev = (*name_head).next;
        (*name_head).type_0 = NELT_NOOP;
        elt = xmalloc(::std::mem::size_of::<name_elt>() as libc::c_ulong)
            as *mut name_elt;
    }
    (*elt).prev = (*name_head).prev;
    if !((*name_head).prev).is_null() {
        (*(*name_head).prev).next = elt;
    }
    (*elt).next = name_head;
    (*name_head).prev = elt;
    return elt;
}
unsafe extern "C" fn name_list_adjust() {
    if !name_head.is_null() {
        while !((*name_head).prev).is_null() {
            name_head = (*name_head).prev;
        }
    }
}
pub static mut unconsumed_option_tail: *mut name_elt = 0 as *const name_elt
    as *mut name_elt;
unsafe extern "C" fn unconsumed_option_push(mut elt: *mut name_elt) {
    (*elt).next = 0 as *mut name_elt;
    (*elt).prev = unconsumed_option_tail;
    if !unconsumed_option_tail.is_null() {
        (*unconsumed_option_tail).next = elt;
    }
    unconsumed_option_tail = elt;
}
unsafe extern "C" fn unconsumed_option_free() {
    while !unconsumed_option_tail.is_null() {
        let mut elt: *mut name_elt = unconsumed_option_tail;
        unconsumed_option_tail = (*unconsumed_option_tail).prev;
        rpl_free(elt as *mut libc::c_void);
    }
}
unsafe extern "C" fn unconsumed_option_report() {
    if !unconsumed_option_tail.is_null() {
        let mut elt: *mut name_elt = 0 as *mut name_elt;
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"The following options were used after any non-optional arguments in archive create or update mode.  These options are positional and affect only arguments that follow them.  Please, rearrange them properly.\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        exit_status = 2 as libc::c_int;
        elt = unconsumed_option_tail;
        while !((*elt).prev).is_null() {
            elt = (*elt).prev;
        }
        while !elt.is_null() {
            match (*elt).type_0 as libc::c_uint {
                1 => {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"-C %s has no effect\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote((*elt).v.name),
                    );
                    exit_status = 2 as libc::c_int;
                }
                4 => {
                    if !((*elt).v.opt.arg).is_null() {
                        if error_hook.is_some() {
                            error_hook.unwrap()();
                        }
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"--%s %s has no effect\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            file_selection_option_name((*elt).v.opt.rpl_option),
                            quote((*elt).v.opt.arg),
                        );
                        exit_status = 2 as libc::c_int;
                    } else {
                        if error_hook.is_some() {
                            error_hook.unwrap()();
                        }
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"--%s has no effect\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            file_selection_option_name((*elt).v.opt.rpl_option),
                        );
                        exit_status = 2 as libc::c_int;
                    }
                }
                _ => {}
            }
            elt = (*elt).next;
        }
        unconsumed_option_free();
    }
}
unsafe extern "C" fn name_list_advance() {
    let mut elt: *mut name_elt = name_head;
    name_head = (*elt).next;
    if !name_head.is_null() {
        (*name_head).prev = 0 as *mut name_elt;
    }
    if (*elt).type_0 as libc::c_uint == NELT_OPTION as libc::c_int as libc::c_uint
        || (*elt).type_0 as libc::c_uint == NELT_CHDIR as libc::c_int as libc::c_uint
    {
        if subcommand_option as libc::c_uint
            == CREATE_SUBCOMMAND as libc::c_int as libc::c_uint
            || subcommand_option as libc::c_uint
                == UPDATE_SUBCOMMAND as libc::c_int as libc::c_uint
        {
            unconsumed_option_push(elt);
        }
    } else {
        if (*elt).type_0 as libc::c_uint != NELT_NOOP as libc::c_int as libc::c_uint {
            unconsumed_option_free();
        }
        rpl_free(elt as *mut libc::c_void);
    };
}
pub unsafe extern "C" fn name_add_name(mut name: *const libc::c_char) {
    let mut ep: *mut name_elt = name_elt_alloc();
    (*ep).type_0 = NELT_NAME;
    (*ep).v.name = name;
    match filename_args as libc::c_uint {
        0 => {
            filename_args = FILES_ONE;
        }
        1 => {
            filename_args = FILES_MANY;
        }
        _ => {}
    };
}
unsafe extern "C" fn name_add_option(
    mut rpl_option: libc::c_int,
    mut arg: *const libc::c_char,
) {
    let mut elt: *mut name_elt = name_elt_alloc();
    (*elt).type_0 = NELT_OPTION;
    (*elt).v.opt.rpl_option = rpl_option;
    (*elt).v.opt.arg = arg;
}
unsafe extern "C" fn name_add_dir(mut name: *const libc::c_char) {
    let mut ep: *mut name_elt = name_elt_alloc();
    (*ep).type_0 = NELT_CHDIR;
    (*ep).v.name = name;
}
unsafe extern "C" fn name_add_file(mut name: *const libc::c_char) {
    let mut ep: *mut name_elt = name_elt_alloc();
    (*ep).type_0 = NELT_FILE;
    (*ep).v.file.name = name;
    (*ep).v.file.line = 0 as libc::c_int as size_t;
    (*ep).v.file.fp = 0 as *mut FILE;
    filename_args = FILES_MANY;
}
static mut name_buffer: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut name_buffer_length: size_t = 0;
pub unsafe extern "C" fn name_init() {
    name_buffer = xmalloc((100 as libc::c_int + 2 as libc::c_int) as size_t)
        as *mut libc::c_char;
    name_buffer_length = 100 as libc::c_int as size_t;
    name_list_adjust();
}
pub unsafe extern "C" fn name_term() {
    rpl_free(name_buffer as *mut libc::c_void);
}
static mut file_id_list: *mut file_id_list = 0 as *const file_id_list
    as *mut file_id_list;
unsafe extern "C" fn file_list_name() -> *const libc::c_char {
    let mut elt: *mut name_elt = 0 as *mut name_elt;
    elt = name_head;
    while !elt.is_null() {
        if (*elt).type_0 as libc::c_uint == NELT_FILE as libc::c_int as libc::c_uint
            && !((*elt).v.file.fp).is_null()
        {
            return (*elt).v.file.name;
        }
        elt = (*elt).next;
    }
    return dcgettext(
        0 as *const libc::c_char,
        b"command line\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
}
unsafe extern "C" fn add_file_id(mut filename: *const libc::c_char) -> libc::c_int {
    let mut p: *mut file_id_list = 0 as *mut file_id_list;
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
    let mut reading_from: *const libc::c_char = 0 as *const libc::c_char;
    if stat(filename, &mut st) != 0 {
        stat_fatal(filename);
    }
    reading_from = file_list_name();
    p = file_id_list;
    while !p.is_null() {
        if (*p).ino == st.st_ino && (*p).dev == st.st_dev {
            let mut oldc: libc::c_int = set_char_quoting(
                0 as *mut quoting_options,
                ':' as i32 as libc::c_char,
                1 as libc::c_int,
            );
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: file list requested from %s already read from %s\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n(0 as libc::c_int, filename),
                reading_from,
                (*p).from_file,
            );
            exit_status = 2 as libc::c_int;
            set_char_quoting(
                0 as *mut quoting_options,
                ':' as i32 as libc::c_char,
                oldc,
            );
            return 1 as libc::c_int;
        }
        p = (*p).next;
    }
    p = xmalloc(::std::mem::size_of::<file_id_list>() as libc::c_ulong)
        as *mut file_id_list;
    (*p).next = file_id_list;
    (*p).ino = st.st_ino;
    (*p).dev = st.st_dev;
    (*p).from_file = reading_from;
    file_id_list = p;
    return 0 as libc::c_int;
}
unsafe extern "C" fn chopslash(mut str: *mut libc::c_char) {
    let mut p: *mut libc::c_char = str
        .offset(strlen(str) as isize)
        .offset(-(1 as libc::c_int as isize));
    while p > str && *p as libc::c_int == '/' as i32 {
        let fresh1 = p;
        p = p.offset(-1);
        *fresh1 = '\0' as i32 as libc::c_char;
    }
}
unsafe extern "C" fn read_name_from_file(
    mut ent: *mut name_elt,
) -> read_file_list_state {
    let mut c: libc::c_int = 0;
    let mut counter: size_t = 0 as libc::c_int as size_t;
    let mut fp: *mut FILE = (*ent).v.file.fp;
    let mut term: libc::c_int = (*ent).v.file.term;
    (*ent).v.file.line = ((*ent).v.file.line).wrapping_add(1);
    (*ent).v.file.line;
    c = getc_unlocked(fp);
    while c != -(1 as libc::c_int) && c != term {
        if counter == name_buffer_length {
            name_buffer = x2realloc(
                name_buffer as *mut libc::c_void,
                &mut name_buffer_length,
            ) as *mut libc::c_char;
        }
        let fresh2 = counter;
        counter = counter.wrapping_add(1);
        *name_buffer.offset(fresh2 as isize) = c as libc::c_char;
        if c == 0 as libc::c_int {
            return file_list_zero;
        }
        c = getc_unlocked(fp);
    }
    if counter == 0 as libc::c_int as libc::c_ulong && c != -(1 as libc::c_int) {
        return file_list_skip;
    }
    if counter == name_buffer_length {
        name_buffer = x2realloc(
            name_buffer as *mut libc::c_void,
            &mut name_buffer_length,
        ) as *mut libc::c_char;
    }
    *name_buffer.offset(counter as isize) = 0 as libc::c_int as libc::c_char;
    chopslash(name_buffer);
    return (if counter == 0 as libc::c_int as libc::c_ulong && c == -(1 as libc::c_int) {
        file_list_end as libc::c_int
    } else {
        file_list_success as libc::c_int
    }) as read_file_list_state;
}
unsafe extern "C" fn handle_option(
    mut str: *const libc::c_char,
    mut ent: *const name_elt,
) -> libc::c_int {
    let mut ws: wordsplit = wordsplit {
        ws_wordc: 0,
        ws_wordv: 0 as *mut *mut libc::c_char,
        ws_offs: 0,
        ws_wordn: 0,
        ws_flags: 0,
        ws_options: 0,
        ws_maxwords: 0,
        ws_wordi: 0,
        ws_delim: 0 as *const libc::c_char,
        ws_comment: 0 as *const libc::c_char,
        ws_escape: [0 as *const libc::c_char; 2],
        ws_alloc_die: None,
        ws_error: None,
        ws_debug: None,
        ws_env: 0 as *mut *const libc::c_char,
        ws_envbuf: 0 as *mut *mut libc::c_char,
        ws_envidx: 0,
        ws_envsiz: 0,
        ws_getvar: None,
        ws_closure: 0 as *mut libc::c_void,
        ws_command: None,
        ws_input: 0 as *const libc::c_char,
        ws_len: 0,
        ws_endp: 0,
        ws_errno: 0,
        ws_usererr: 0 as *mut libc::c_char,
        ws_head: 0 as *mut wordsplit_node,
        ws_tail: 0 as *mut wordsplit_node,
        ws_lvl: 0,
    };
    let mut i: libc::c_int = 0;
    let mut loc: option_locus = option_locus {
        source: OPTS_ENVIRON,
        name: 0 as *const libc::c_char,
        line: 0,
        prev: 0 as *mut option_locus,
    };
    while *str as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*str as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        str = str.offset(1);
        str;
    }
    if *str as libc::c_int != '-' as i32 {
        return 1 as libc::c_int;
    }
    ws.ws_offs = 1 as libc::c_int as size_t;
    if wordsplit(
        str,
        &mut ws,
        (0x40 as libc::c_int | 0x4 as libc::c_int
            | (0x200 as libc::c_int | 0x400 as libc::c_int) | 0x800 as libc::c_int
            | 0x2000000 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint,
    ) != 0
    {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"cannot split string '%s': %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            str,
            wordsplit_strerror(&mut ws),
        );
        fatal_exit();
    }
    let ref mut fresh3 = *(ws.ws_wordv).offset(0 as libc::c_int as isize);
    *fresh3 = program_name as *mut libc::c_char;
    loc.source = OPTS_FILE;
    loc.name = (*ent).v.file.name;
    loc.line = (*ent).v.file.line;
    more_options(
        (ws.ws_wordc).wrapping_add(ws.ws_offs) as libc::c_int,
        ws.ws_wordv,
        &mut loc,
    );
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < (ws.ws_wordc).wrapping_add(ws.ws_offs) {
        let ref mut fresh4 = *(ws.ws_wordv).offset(i as isize);
        *fresh4 = 0 as *mut libc::c_char;
        i += 1;
        i;
    }
    wordsplit_free(&mut ws);
    return 0 as libc::c_int;
}
unsafe extern "C" fn read_next_name(
    mut ent: *mut name_elt,
    mut ret: *mut name_elt,
) -> libc::c_int {
    if ((*ent).v.file.fp).is_null() {
        if strcmp((*ent).v.file.name, b"-\0" as *const u8 as *const libc::c_char) == 0 {
            request_stdin(b"-T\0" as *const u8 as *const libc::c_char);
            (*ent).v.file.fp = stdin;
        } else {
            if add_file_id((*ent).v.file.name) != 0 {
                name_list_advance();
                return 1 as libc::c_int;
            }
            (*ent)
                .v
                .file
                .fp = fopen(
                (*ent).v.file.name,
                b"r\0" as *const u8 as *const libc::c_char,
            );
            if ((*ent).v.file.fp).is_null() {
                open_fatal((*ent).v.file.name);
            }
        }
        (*ent).v.file.term = filename_terminator as libc::c_int;
        (*ent).v.file.verbatim = verbatim_files_from_option;
    }
    loop {
        match read_name_from_file(ent) as libc::c_uint {
            2 => {
                if warning_option & 0x200 as libc::c_int != 0 {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"%s: file name read contains nul character\0" as *const u8
                            as *const libc::c_char,
                        quotearg_colon((*ent).v.file.name),
                    );
                }
                (*ent).v.file.term = 0 as libc::c_int;
            }
            0 => {}
            1 => {
                if strcmp((*ent).v.file.name, b"-\0" as *const u8 as *const libc::c_char)
                    != 0
                {
                    fclose((*ent).v.file.fp);
                }
                (*ent).v.file.fp = 0 as *mut FILE;
                name_list_advance();
                return 1 as libc::c_int;
            }
            3 | _ => {
                continue;
            }
        }
        if !(*ent).v.file.verbatim {
            if unquote_option {
                unquote_string(name_buffer);
            }
            if handle_option(name_buffer, ent) == 0 as libc::c_int {
                name_list_adjust();
                return 1 as libc::c_int;
            }
        }
        (*ret).type_0 = NELT_NAME;
        (*ret).v.name = name_buffer;
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn copy_name(mut ep: *mut name_elt) {
    let mut source: *const libc::c_char = 0 as *const libc::c_char;
    let mut source_len: size_t = 0;
    source = (*ep).v.name;
    source_len = strlen(source);
    while name_buffer_length <= source_len {
        name_buffer = x2realloc(
            name_buffer as *mut libc::c_void,
            &mut name_buffer_length,
        ) as *mut libc::c_char;
    }
    strcpy(name_buffer, source);
    chopslash(name_buffer);
}
unsafe extern "C" fn name_next_elt(mut change_dirs: libc::c_int) -> *mut name_elt {
    static mut entry: name_elt = name_elt {
        next: 0 as *const name_elt as *mut name_elt,
        prev: 0 as *const name_elt as *mut name_elt,
        type_0: NELT_NAME,
        v: C2RustUnnamed_3 {
            name: 0 as *const libc::c_char,
        },
    };
    let mut ep: *mut name_elt = 0 as *mut name_elt;
    loop {
        ep = name_head;
        if ep.is_null() {
            break;
        }
        match (*ep).type_0 as libc::c_uint {
            3 => {
                name_list_advance();
                continue;
            }
            2 => {
                if read_next_name(ep, &mut entry) == 0 as libc::c_int {
                    return &mut entry;
                }
                continue;
            }
            1 => {
                if change_dirs != 0 {
                    chdir_do(chdir_arg(xstrdup((*ep).v.name)));
                    name_list_advance();
                    continue;
                }
            }
            0 => {}
            4 => {
                handle_file_selection_option((*ep).v.opt.rpl_option, (*ep).v.opt.arg);
                name_list_advance();
                continue;
            }
            _ => {
                continue;
            }
        }
        copy_name(ep);
        if unquote_option {
            unquote_string(name_buffer);
        }
        entry.type_0 = (*ep).type_0;
        entry.v.name = name_buffer;
        name_list_advance();
        return &mut entry;
    }
    unconsumed_option_report();
    return 0 as *mut name_elt;
}
pub unsafe extern "C" fn name_next(mut change_dirs: libc::c_int) -> *const libc::c_char {
    let mut nelt: *mut name_elt = name_next_elt(change_dirs);
    return if !nelt.is_null() { (*nelt).v.name } else { 0 as *const libc::c_char };
}
pub unsafe extern "C" fn name_gather() {
    static mut buffer: *mut name = 0 as *const name as *mut name;
    let mut ep: *mut name_elt = 0 as *mut name_elt;
    if same_order_option {
        static mut change_dir: libc::c_int = 0;
        loop {
            ep = name_next_elt(0 as libc::c_int);
            if !(!ep.is_null()
                && (*ep).type_0 as libc::c_uint
                    == NELT_CHDIR as libc::c_int as libc::c_uint)
            {
                break;
            }
            change_dir = chdir_arg(xstrdup((*ep).v.name));
        }
        if !ep.is_null() {
            free_name(buffer);
            buffer = make_name((*ep).v.name);
            (*buffer).change_dir = change_dir;
            (*buffer).next = 0 as *mut name;
            (*buffer).found_count = 0 as libc::c_int as uintmax_t;
            (*buffer)
                .matching_flags = (if wildcards as libc::c_uint
                == enable_wildcards as libc::c_int as libc::c_uint
            {
                (1 as libc::c_int) << 28 as libc::c_int
            } else {
                0 as libc::c_int
            }) | include_anchored | matching_flags | recursion_option;
            (*buffer).directory = 0 as *mut directory;
            (*buffer).parent = 0 as *mut name;
            (*buffer).cmdline = 1 as libc::c_int != 0;
            nametail = buffer;
            namelist = nametail;
        } else if change_dir != 0 {
            addname(
                0 as *const libc::c_char,
                change_dir,
                0 as libc::c_int != 0,
                0 as *mut name,
            );
        }
    } else {
        let mut change_dir_0: libc::c_int = 0 as libc::c_int;
        loop {
            let mut change_dir0: libc::c_int = change_dir_0;
            loop {
                ep = name_next_elt(0 as libc::c_int);
                if !(!ep.is_null()
                    && (*ep).type_0 as libc::c_uint
                        == NELT_CHDIR as libc::c_int as libc::c_uint)
                {
                    break;
                }
                change_dir_0 = chdir_arg(xstrdup((*ep).v.name));
            }
            if !ep.is_null() {
                addname(
                    (*ep).v.name,
                    change_dir_0,
                    1 as libc::c_int != 0,
                    0 as *mut name,
                );
            } else {
                if change_dir_0 != change_dir0 {
                    addname(
                        0 as *const libc::c_char,
                        change_dir_0,
                        0 as libc::c_int != 0,
                        0 as *mut name,
                    );
                }
                break;
            }
        }
    };
}
pub unsafe extern "C" fn addname(
    mut string: *const libc::c_char,
    mut change_dir: libc::c_int,
    mut cmdline: bool,
    mut parent: *mut name,
) -> *mut name {
    let mut name: *mut name = make_name(string);
    (*name).prev = nametail;
    (*name).next = 0 as *mut name;
    (*name).found_count = 0 as libc::c_int as uintmax_t;
    (*name)
        .matching_flags = (if wildcards as libc::c_uint
        == enable_wildcards as libc::c_int as libc::c_uint
    {
        (1 as libc::c_int) << 28 as libc::c_int
    } else {
        0 as libc::c_int
    }) | include_anchored | matching_flags | recursion_option;
    (*name).change_dir = change_dir;
    (*name).directory = 0 as *mut directory;
    (*name).parent = parent;
    (*name).cmdline = cmdline;
    if !nametail.is_null() {
        (*nametail).next = name;
    } else {
        namelist = name;
    }
    nametail = name;
    return name;
}
pub unsafe extern "C" fn add_starting_file(mut file_name: *const libc::c_char) {
    let mut name: *mut name = make_name(file_name);
    if starting_file_option {
        let mut head: *mut name = namelist;
        remname(head);
        free_name(head);
    }
    (*name).prev = 0 as *mut name;
    (*name).next = namelist;
    namelist = name;
    if nametail.is_null() {
        nametail = namelist;
    }
    (*name).found_count = 0 as libc::c_int as uintmax_t;
    (*name)
        .matching_flags = (if wildcards as libc::c_uint
        == enable_wildcards as libc::c_int as libc::c_uint
    {
        (1 as libc::c_int) << 28 as libc::c_int
    } else {
        0 as libc::c_int
    }) | include_anchored | matching_flags | recursion_option;
    (*name).change_dir = 0 as libc::c_int;
    (*name).directory = 0 as *mut directory;
    (*name).parent = 0 as *mut name;
    (*name).cmdline = 1 as libc::c_int != 0;
    starting_file_option = 1 as libc::c_int != 0;
}
unsafe extern "C" fn namelist_match(
    mut file_name: *const libc::c_char,
    mut length: size_t,
) -> *mut name {
    let mut p: *mut name = 0 as *mut name;
    p = namelist;
    while !p.is_null() {
        if *((*p).name).offset(0 as libc::c_int as isize) as libc::c_int != 0
            && exclude_fnmatch((*p).name, file_name, (*p).matching_flags) as libc::c_int
                != 0
        {
            return p;
        }
        p = (*p).next;
    }
    return 0 as *mut name;
}
pub unsafe extern "C" fn remname(mut name: *mut name) {
    let mut p: *mut name = 0 as *mut name;
    p = (*name).prev;
    if !p.is_null() {
        (*p).next = (*name).next;
    } else {
        namelist = (*name).next;
    }
    p = (*name).next;
    if !p.is_null() {
        (*p).prev = (*name).prev;
    } else {
        nametail = (*name).prev;
    };
}
pub unsafe extern "C" fn name_match(mut file_name: *const libc::c_char) -> bool {
    let mut length: size_t = strlen(file_name);
    loop {
        let mut cursor: *mut name = namelist;
        if cursor.is_null() {
            return 1 as libc::c_int != 0;
        }
        if *((*cursor).name).offset(0 as libc::c_int as isize) as libc::c_int
            == 0 as libc::c_int
        {
            chdir_do((*cursor).change_dir);
            namelist = 0 as *mut name;
            nametail = 0 as *mut name;
            return 1 as libc::c_int != 0;
        }
        cursor = namelist_match(file_name, length);
        if starting_file_option {
            if cursor == namelist {
                starting_file_option = 0 as libc::c_int != 0;
            } else {
                cursor = 0 as *mut name;
            }
        }
        if !cursor.is_null() {
            if !(*file_name.offset((*cursor).length as isize) as libc::c_int
                == '/' as i32 && recursion_option != 0)
                || (*cursor).found_count == 0 as libc::c_int as libc::c_ulong
            {
                (*cursor).found_count = ((*cursor).found_count).wrapping_add(1);
                (*cursor).found_count;
            }
            chdir_do((*cursor).change_dir);
            return if occurrence_option == 0 as libc::c_int as libc::c_ulong {
                ((*cursor).found_count != 0 as libc::c_int as libc::c_ulong)
                    as libc::c_int
            } else {
                ((*cursor).found_count == occurrence_option) as libc::c_int
            } != 0;
        }
        if same_order_option as libc::c_int != 0 && (*namelist).found_count != 0 {
            name_gather();
            if (*namelist).found_count != 0 {
                return 0 as libc::c_int != 0;
            }
        } else {
            return 0 as libc::c_int != 0
        }
    };
}
pub unsafe extern "C" fn all_names_found(mut p: *mut tar_stat_info) -> bool {
    let mut cursor: *const name = 0 as *const name;
    let mut len: size_t = 0;
    if ((*p).file_name).is_null()
        || occurrence_option == 0 as libc::c_int as libc::c_ulong
        || (*p).had_trailing_slash as libc::c_int != 0
    {
        return 0 as libc::c_int != 0;
    }
    len = strlen((*p).file_name);
    cursor = namelist;
    while !cursor.is_null() {
        if *((*cursor).name).offset(0 as libc::c_int as isize) as libc::c_int != 0
            && (if occurrence_option == 0 as libc::c_int as libc::c_ulong {
                ((*cursor).found_count != 0 as libc::c_int as libc::c_ulong)
                    as libc::c_int
            } else {
                ((*cursor).found_count >= occurrence_option) as libc::c_int
            }) == 0
            || len >= (*cursor).length
                && *((*p).file_name).offset((*cursor).length as isize) as libc::c_int
                    == '/' as i32
        {
            return 0 as libc::c_int != 0;
        }
        cursor = (*cursor).next;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn regex_usage_warning(mut name: *const libc::c_char) -> libc::c_int {
    static mut warned_once: libc::c_int = 0 as libc::c_int;
    if wildcards as libc::c_uint == default_wildcards as libc::c_int as libc::c_uint
        && fnmatch_pattern_has_wildcards(name, 0 as libc::c_int) as libc::c_int != 0
    {
        warned_once = 1 as libc::c_int;
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Pattern matching characters used in file names\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Use --wildcards to enable pattern matching, or --no-wildcards to suppress this warning\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    return warned_once;
}
pub unsafe extern "C" fn names_notfound() {
    let mut cursor: *const name = 0 as *const name;
    cursor = namelist;
    while !cursor.is_null() {
        if (if occurrence_option == 0 as libc::c_int as libc::c_ulong {
            ((*cursor).found_count != 0 as libc::c_int as libc::c_ulong) as libc::c_int
        } else {
            ((*cursor).found_count >= occurrence_option) as libc::c_int
        }) == 0
            && *((*cursor).name).offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            regex_usage_warning((*cursor).name);
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                if (*cursor).found_count == 0 as libc::c_int as libc::c_ulong {
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: Not found in archive\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    )
                } else {
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: Required occurrence not found in archive\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    )
                },
                quotearg_colon((*cursor).name),
            );
            exit_status = 2 as libc::c_int;
        }
        cursor = (*cursor).next;
    }
    namelist = 0 as *mut name;
    nametail = 0 as *mut name;
    if same_order_option {
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        loop {
            name = name_next(1 as libc::c_int);
            if name.is_null() {
                break;
            }
            regex_usage_warning(name);
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Not found in archive\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_colon(name),
            );
            exit_status = 2 as libc::c_int;
        }
    }
}
pub unsafe extern "C" fn label_notfound() {
    let mut cursor: *const name = 0 as *const name;
    if namelist.is_null() {
        return;
    }
    cursor = namelist;
    while !cursor.is_null() {
        if if occurrence_option == 0 as libc::c_int as libc::c_ulong {
            ((*cursor).found_count != 0 as libc::c_int as libc::c_ulong) as libc::c_int
        } else {
            ((*cursor).found_count >= occurrence_option) as libc::c_int
        } != 0
        {
            return;
        }
        cursor = (*cursor).next;
    }
    if verbose_option != 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Archive label mismatch\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    set_exit_status(1 as libc::c_int);
    cursor = namelist;
    while !cursor.is_null() {
        if regex_usage_warning((*cursor).name) != 0 {
            break;
        }
        cursor = (*cursor).next;
    }
    namelist = 0 as *mut name;
    nametail = 0 as *mut name;
    if same_order_option {
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        loop {
            name = name_next(1 as libc::c_int);
            if !(!name.is_null() && regex_usage_warning(name) == 0 as libc::c_int) {
                break;
            }
        }
    }
}
unsafe extern "C" fn merge_sort_sll(
    mut list: *mut name,
    mut length: libc::c_int,
    mut compare: Option::<unsafe extern "C" fn(*const name, *const name) -> libc::c_int>,
) -> *mut name {
    let mut first_list: *mut name = 0 as *mut name;
    let mut second_list: *mut name = 0 as *mut name;
    let mut first_length: libc::c_int = 0;
    let mut second_length: libc::c_int = 0;
    let mut result: *mut name = 0 as *mut name;
    let mut merge_point: *mut *mut name = 0 as *mut *mut name;
    let mut cursor: *mut name = 0 as *mut name;
    let mut counter: libc::c_int = 0;
    if length == 1 as libc::c_int {
        return list;
    }
    if length == 2 as libc::c_int {
        if (Some(compare.unwrap())).unwrap()(list, (*list).next) > 0 as libc::c_int {
            result = (*list).next;
            (*result).next = list;
            (*list).next = 0 as *mut name;
            return result;
        }
        return list;
    }
    first_list = list;
    first_length = (length + 1 as libc::c_int) / 2 as libc::c_int;
    second_length = length / 2 as libc::c_int;
    cursor = list;
    counter = first_length - 1 as libc::c_int;
    while counter != 0 {
        cursor = (*cursor).next;
        counter -= 1;
        counter;
    }
    second_list = (*cursor).next;
    (*cursor).next = 0 as *mut name;
    first_list = merge_sort_sll(first_list, first_length, compare);
    second_list = merge_sort_sll(second_list, second_length, compare);
    merge_point = &mut result;
    while !first_list.is_null() && !second_list.is_null() {
        if (Some(compare.unwrap())).unwrap()(first_list, second_list) < 0 as libc::c_int
        {
            cursor = (*first_list).next;
            *merge_point = first_list;
            merge_point = &mut (*first_list).next;
            first_list = cursor;
        } else {
            cursor = (*second_list).next;
            *merge_point = second_list;
            merge_point = &mut (*second_list).next;
            second_list = cursor;
        }
    }
    if !first_list.is_null() {
        *merge_point = first_list;
    } else {
        *merge_point = second_list;
    }
    return result;
}
unsafe extern "C" fn merge_sort(
    mut list: *mut name,
    mut length: libc::c_int,
    mut compare: Option::<unsafe extern "C" fn(*const name, *const name) -> libc::c_int>,
) -> *mut name {
    let mut head: *mut name = 0 as *mut name;
    let mut p: *mut name = 0 as *mut name;
    let mut prev: *mut name = 0 as *mut name;
    head = merge_sort_sll(list, length, compare);
    prev = 0 as *mut name;
    p = head;
    while !p.is_null() {
        (*p).prev = prev;
        prev = p;
        p = (*p).next;
    }
    return head;
}
unsafe extern "C" fn compare_names_found(
    mut n1: *const name,
    mut n2: *const name,
) -> libc::c_int {
    let mut found_diff: libc::c_int = (if occurrence_option
        == 0 as libc::c_int as libc::c_ulong
    {
        ((*n2).found_count != 0 as libc::c_int as libc::c_ulong) as libc::c_int
    } else {
        ((*n2).found_count >= occurrence_option) as libc::c_int
    })
        - (if occurrence_option == 0 as libc::c_int as libc::c_ulong {
            ((*n1).found_count != 0 as libc::c_int as libc::c_ulong) as libc::c_int
        } else {
            ((*n1).found_count >= occurrence_option) as libc::c_int
        });
    return if found_diff != 0 { found_diff } else { strcmp((*n1).name, (*n2).name) };
}
unsafe extern "C" fn compare_names(
    mut n1: *const name,
    mut n2: *const name,
) -> libc::c_int {
    return strcmp((*n1).name, (*n2).name);
}
unsafe extern "C" fn add_hierarchy_to_namelist(
    mut st: *mut tar_stat_info,
    mut name: *mut name,
) {
    let mut buffer: *const libc::c_char = 0 as *const libc::c_char;
    (*name).directory = scan_directory(st);
    buffer = directory_contents((*name).directory);
    if !buffer.is_null() {
        let mut child_head: *mut name = 0 as *mut name;
        let mut child_tail: *mut name = 0 as *mut name;
        let mut name_length: size_t = (*name).length;
        let mut allocated_length: size_t = (if name_length
            >= 100 as libc::c_int as libc::c_ulong
        {
            name_length.wrapping_add(100 as libc::c_int as libc::c_ulong)
        } else {
            100 as libc::c_int as libc::c_ulong
        })
            .wrapping_add(2 as libc::c_int as libc::c_ulong);
        let mut namebuf: *mut libc::c_char = xmalloc(allocated_length)
            as *mut libc::c_char;
        let mut string: *const libc::c_char = 0 as *const libc::c_char;
        let mut string_length: size_t = 0;
        let mut change_dir: libc::c_int = (*name).change_dir;
        strcpy(namebuf, (*name).name);
        if !(*namebuf
            .offset(name_length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '/' as i32)
        {
            let fresh5 = name_length;
            name_length = name_length.wrapping_add(1);
            *namebuf.offset(fresh5 as isize) = '/' as i32 as libc::c_char;
            *namebuf.offset(name_length as isize) = '\0' as i32 as libc::c_char;
        }
        string = buffer;
        while *string != 0 {
            string_length = strlen(string);
            if *string as libc::c_int == 'D' as i32 {
                let mut np: *mut name = 0 as *mut name;
                let mut subdir: tar_stat_info = tar_stat_info {
                    orig_file_name: 0 as *mut libc::c_char,
                    file_name: 0 as *mut libc::c_char,
                    had_trailing_slash: false,
                    link_name: 0 as *mut libc::c_char,
                    uname: 0 as *mut libc::c_char,
                    gname: 0 as *mut libc::c_char,
                    cntx_name: 0 as *mut libc::c_char,
                    acls_a_ptr: 0 as *mut libc::c_char,
                    acls_a_len: 0,
                    acls_d_ptr: 0 as *mut libc::c_char,
                    acls_d_len: 0,
                    stat: stat {
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
                    },
                    atime: timespec { tv_sec: 0, tv_nsec: 0 },
                    mtime: timespec { tv_sec: 0, tv_nsec: 0 },
                    ctime: timespec { tv_sec: 0, tv_nsec: 0 },
                    archive_file_size: 0,
                    is_sparse: false,
                    sparse_major: 0,
                    sparse_minor: 0,
                    sparse_map_avail: 0,
                    sparse_map_size: 0,
                    sparse_map: 0 as *mut sp_array,
                    real_size: 0,
                    real_size_set: false,
                    sparse_name_done: false,
                    xattr_map_size: 0,
                    xattr_map: 0 as *mut xattr_array,
                    xhdr: xheader {
                        stk: 0 as *mut obstack,
                        size: 0,
                        buffer: 0 as *mut libc::c_char,
                        string_length: 0,
                    },
                    is_dumpdir: false,
                    skipped: false,
                    dumpdir: 0 as *mut libc::c_char,
                    parent: 0 as *mut tar_stat_info,
                    dirstream: 0 as *mut DIR,
                    fd: 0,
                    exclude_list: 0 as *mut exclist,
                };
                let mut subfd: libc::c_int = 0;
                while allocated_length < name_length.wrapping_add(string_length) {
                    namebuf = x2realloc(
                        namebuf as *mut libc::c_void,
                        &mut allocated_length,
                    ) as *mut libc::c_char;
                }
                strcpy(
                    namebuf.offset(name_length as isize),
                    string.offset(1 as libc::c_int as isize),
                );
                np = addname(namebuf, change_dir, 0 as libc::c_int != 0, name);
                if child_head.is_null() {
                    child_head = np;
                } else {
                    (*child_tail).sibling = np;
                }
                child_tail = np;
                tar_stat_init(&mut subdir);
                subdir.parent = st;
                if (*st).fd < 0 as libc::c_int {
                    subfd = -(1 as libc::c_int);
                    *__errno_location() = -(*st).fd;
                } else {
                    subfd = subfile_open(
                        st,
                        string.offset(1 as libc::c_int as isize),
                        open_read_flags | 0o200000 as libc::c_int,
                    );
                }
                if subfd < 0 as libc::c_int {
                    open_diag(namebuf);
                } else {
                    subdir.fd = subfd;
                    if fstat(subfd, &mut subdir.stat) != 0 as libc::c_int {
                        stat_diag(namebuf);
                    } else if !(0o200000 as libc::c_int != 0
                        || subdir.stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                            == 0o40000 as libc::c_int as libc::c_uint)
                    {
                        *__errno_location() = 20 as libc::c_int;
                        open_diag(namebuf);
                    } else {
                        subdir.orig_file_name = xstrdup(namebuf);
                        add_hierarchy_to_namelist(&mut subdir, np);
                        restore_parent_fd(&mut subdir);
                    }
                }
                tar_stat_destroy(&mut subdir);
            }
            string = string
                .offset(
                    string_length.wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as isize,
                );
        }
        rpl_free(namebuf as *mut libc::c_void);
        (*name).child = child_head;
    }
}
unsafe extern "C" fn name_hash(
    mut entry: *const libc::c_void,
    mut n_buckets: size_t,
) -> size_t {
    let mut name: *const name = entry as *const name;
    return hash_string((*name).caname, n_buckets);
}
unsafe extern "C" fn name_compare(
    mut entry1: *const libc::c_void,
    mut entry2: *const libc::c_void,
) -> bool {
    let mut name1: *const name = entry1 as *const name;
    let mut name2: *const name = entry2 as *const name;
    return strcmp((*name1).caname, (*name2).caname) == 0 as libc::c_int;
}
unsafe extern "C" fn rebase_child_list(mut child: *mut name, mut parent: *mut name) {
    let mut old_prefix_len: size_t = (*(*child).parent).length;
    let mut new_prefix_len: size_t = (*parent).length;
    let mut new_prefix: *mut libc::c_char = (*parent).name;
    while !child.is_null() {
        let mut size: size_t = ((*child).length)
            .wrapping_sub(old_prefix_len)
            .wrapping_add(new_prefix_len);
        let mut newp: *mut libc::c_char = xmalloc(
            size.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy(newp, new_prefix);
        strcat(newp, ((*child).name).offset(old_prefix_len as isize));
        rpl_free((*child).name as *mut libc::c_void);
        (*child).name = newp;
        (*child).length = size;
        rebase_directory(
            (*child).directory,
            (*(*child).parent).name,
            old_prefix_len,
            new_prefix,
            new_prefix_len,
        );
        child = (*child).sibling;
    }
}
pub unsafe extern "C" fn collect_and_sort_names() {
    let mut name: *mut name = 0 as *mut name;
    let mut next_name: *mut name = 0 as *mut name;
    let mut prev_name: *mut name = 0 as *mut name;
    let mut num_names: libc::c_int = 0;
    let mut nametab: *mut Hash_table = 0 as *mut Hash_table;
    name_gather();
    if namelist.is_null() {
        addname(
            b".\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int != 0,
            0 as *mut name,
        );
    }
    if !listed_incremental_option.is_null() {
        match chdir_count() {
            0 => {}
            1 => {
                if (*namelist).change_dir == 0 as libc::c_int {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Using -C option inside file list is not allowed with --listed-incremental\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    usage(2 as libc::c_int);
                }
            }
            _ => {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Only one -C option is allowed with --listed-incremental\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                usage(2 as libc::c_int);
            }
        }
        read_directory_file();
    }
    num_names = 0 as libc::c_int;
    name = namelist;
    while !name.is_null() {
        let mut st: tar_stat_info = tar_stat_info {
            orig_file_name: 0 as *mut libc::c_char,
            file_name: 0 as *mut libc::c_char,
            had_trailing_slash: false,
            link_name: 0 as *mut libc::c_char,
            uname: 0 as *mut libc::c_char,
            gname: 0 as *mut libc::c_char,
            cntx_name: 0 as *mut libc::c_char,
            acls_a_ptr: 0 as *mut libc::c_char,
            acls_a_len: 0,
            acls_d_ptr: 0 as *mut libc::c_char,
            acls_d_len: 0,
            stat: stat {
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
            },
            atime: timespec { tv_sec: 0, tv_nsec: 0 },
            mtime: timespec { tv_sec: 0, tv_nsec: 0 },
            ctime: timespec { tv_sec: 0, tv_nsec: 0 },
            archive_file_size: 0,
            is_sparse: false,
            sparse_major: 0,
            sparse_minor: 0,
            sparse_map_avail: 0,
            sparse_map_size: 0,
            sparse_map: 0 as *mut sp_array,
            real_size: 0,
            real_size_set: false,
            sparse_name_done: false,
            xattr_map_size: 0,
            xattr_map: 0 as *mut xattr_array,
            xhdr: xheader {
                stk: 0 as *mut obstack,
                size: 0,
                buffer: 0 as *mut libc::c_char,
                string_length: 0,
            },
            is_dumpdir: false,
            skipped: false,
            dumpdir: 0 as *mut libc::c_char,
            parent: 0 as *mut tar_stat_info,
            dirstream: 0 as *mut DIR,
            fd: 0,
            exclude_list: 0 as *mut exclist,
        };
        if !((*name).found_count != 0 || !((*name).directory).is_null()) {
            if !((*name).matching_flags & (1 as libc::c_int) << 28 as libc::c_int != 0) {
                chdir_do((*name).change_dir);
                if !(*((*name).name).offset(0 as libc::c_int as isize) as libc::c_int
                    == 0 as libc::c_int)
                {
                    tar_stat_init(&mut st);
                    if deref_stat((*name).name, &mut st.stat) != 0 as libc::c_int {
                        stat_diag((*name).name);
                    } else {
                        if st.stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                            == 0o40000 as libc::c_int as libc::c_uint
                        {
                            let mut dir_fd: libc::c_int = openat(
                                chdir_fd,
                                (*name).name,
                                open_read_flags | 0o200000 as libc::c_int,
                            );
                            if dir_fd < 0 as libc::c_int {
                                open_diag((*name).name);
                            } else {
                                st.fd = dir_fd;
                                if fstat(dir_fd, &mut st.stat) != 0 as libc::c_int {
                                    stat_diag((*name).name);
                                } else if 0o200000 as libc::c_int != 0
                                    || st.stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                        == 0o40000 as libc::c_int as libc::c_uint
                                {
                                    st.orig_file_name = xstrdup((*name).name);
                                    (*name).found_count = ((*name).found_count).wrapping_add(1);
                                    (*name).found_count;
                                    add_hierarchy_to_namelist(&mut st, name);
                                } else {
                                    *__errno_location() = 20 as libc::c_int;
                                    open_diag((*name).name);
                                }
                            }
                        }
                        tar_stat_destroy(&mut st);
                    }
                }
            }
        }
        name = (*name).next;
        num_names += 1;
        num_names;
    }
    namelist = merge_sort(
        namelist,
        num_names,
        Some(
            compare_names
                as unsafe extern "C" fn(*const name, *const name) -> libc::c_int,
        ),
    );
    num_names = 0 as libc::c_int;
    nametab = hash_initialize(
        0 as libc::c_int as size_t,
        0 as *const Hash_tuning,
        Some(name_hash as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t),
        Some(
            name_compare
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
        ),
        None,
    );
    let mut current_block_64: u64;
    name = namelist;
    while !name.is_null() {
        next_name = (*name).next;
        (*name).caname = normalize_filename((*name).change_dir, (*name).name);
        if !prev_name.is_null() {
            let mut p: *mut name = hash_lookup(nametab, name as *const libc::c_void)
                as *mut name;
            if !p.is_null() {
                if ((*name).parent).is_null() {
                    if !((*p).child).is_null() {
                        rebase_child_list((*p).child, name);
                    }
                    hash_remove(nametab, name as *const libc::c_void);
                    remname(p);
                    free_name(p);
                    num_names -= 1;
                    num_names;
                    current_block_64 = 2116367355679836638;
                } else {
                    if !((*name).child).is_null() {
                        rebase_child_list((*name).child, p);
                    }
                    remname(name);
                    free_name(name);
                    current_block_64 = 1622411330066726685;
                }
            } else {
                current_block_64 = 2116367355679836638;
            }
        } else {
            current_block_64 = 2116367355679836638;
        }
        match current_block_64 {
            2116367355679836638 => {
                (*name).found_count = 0 as libc::c_int as uintmax_t;
                if (hash_insert(nametab, name as *const libc::c_void)).is_null() {
                    xalloc_die();
                }
                prev_name = name;
                num_names += 1;
                num_names;
            }
            _ => {}
        }
        name = next_name;
    }
    nametail = prev_name;
    hash_free(nametab);
    namelist = merge_sort(
        namelist,
        num_names,
        Some(
            compare_names_found
                as unsafe extern "C" fn(*const name, *const name) -> libc::c_int,
        ),
    );
    if !listed_incremental_option.is_null() {
        name = namelist;
        while !name.is_null()
            && *((*name).name).offset(0 as libc::c_int as isize) as libc::c_int
                == 0 as libc::c_int
        {
            name = (*name).next;
        }
        if !name.is_null() {
            append_incremental_renames((*name).directory);
        }
    }
}
pub unsafe extern "C" fn name_scan(mut file_name: *const libc::c_char) -> *mut name {
    let mut length: size_t = strlen(file_name);
    loop {
        let mut cursor: *mut name = namelist_match(file_name, length);
        if !cursor.is_null() {
            return cursor;
        }
        if same_order_option as libc::c_int != 0 && !namelist.is_null()
            && (*namelist).found_count != 0
        {
            name_gather();
            if (*namelist).found_count != 0 {
                return 0 as *mut name;
            }
        } else {
            return 0 as *mut name
        }
    };
}
pub static mut gnu_list_name: *mut name = 0 as *const name as *mut name;
pub unsafe extern "C" fn name_from_list() -> *const name {
    if gnu_list_name.is_null() {
        gnu_list_name = namelist;
    }
    while !gnu_list_name.is_null()
        && ((*gnu_list_name).found_count != 0
            || *((*gnu_list_name).name).offset(0 as libc::c_int as isize) as libc::c_int
                == 0 as libc::c_int)
    {
        gnu_list_name = (*gnu_list_name).next;
    }
    if !gnu_list_name.is_null() {
        (*gnu_list_name).found_count = ((*gnu_list_name).found_count).wrapping_add(1);
        (*gnu_list_name).found_count;
        chdir_do((*gnu_list_name).change_dir);
        return gnu_list_name;
    }
    return 0 as *const name;
}
pub unsafe extern "C" fn blank_name_list() {
    let mut name: *mut name = 0 as *mut name;
    gnu_list_name = 0 as *mut name;
    name = namelist;
    while !name.is_null() {
        (*name).found_count = 0 as libc::c_int as uintmax_t;
        name = (*name).next;
    }
}
pub unsafe extern "C" fn make_file_name(
    mut directory_name: *const libc::c_char,
    mut name: *const libc::c_char,
) -> *mut libc::c_char {
    let mut dirlen: size_t = strlen(directory_name);
    let mut namelen: size_t = (strlen(name))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut slash: libc::c_int = (dirlen != 0
        && !(*directory_name
            .offset(dirlen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '/' as i32)) as libc::c_int;
    let mut buffer: *mut libc::c_char = xmalloc(
        dirlen.wrapping_add(slash as libc::c_ulong).wrapping_add(namelen),
    ) as *mut libc::c_char;
    memcpy(buffer as *mut libc::c_void, directory_name as *const libc::c_void, dirlen);
    *buffer.offset(dirlen as isize) = '/' as i32 as libc::c_char;
    memcpy(
        buffer.offset(dirlen as isize).offset(slash as isize) as *mut libc::c_void,
        name as *const libc::c_void,
        namelen,
    );
    return buffer;
}
pub unsafe extern "C" fn stripped_prefix_len(
    mut file_name: *const libc::c_char,
    mut num: size_t,
) -> size_t {
    let mut p: *const libc::c_char = file_name.offset(0 as libc::c_int as isize);
    while *p as libc::c_int == '/' as i32 {
        p = p.offset(1);
        p;
    }
    while *p != 0 {
        let mut slash: bool = *p as libc::c_int == '/' as i32;
        p = p.offset(1);
        p;
        if slash {
            num = num.wrapping_sub(1);
            if num == 0 as libc::c_int as libc::c_ulong {
                return p.offset_from(file_name) as libc::c_long as size_t;
            }
            while *p as libc::c_int == '/' as i32 {
                p = p.offset(1);
                p;
            }
        }
    }
    return -(1 as libc::c_int) as size_t;
}
pub unsafe extern "C" fn contains_dot_dot(mut name: *const libc::c_char) -> bool {
    let mut p: *const libc::c_char = name.offset(0 as libc::c_int as isize);
    loop {
        if *p.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
            && (*p.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32
                || *p.offset(2 as libc::c_int as isize) == 0)
        {
            return 1 as libc::c_int != 0;
        }
        while !(*p as libc::c_int == '/' as i32) {
            let fresh6 = p;
            p = p.offset(1);
            if *fresh6 == 0 {
                return 0 as libc::c_int != 0;
            }
        }
        p = p.offset(1);
        p;
    };
}
