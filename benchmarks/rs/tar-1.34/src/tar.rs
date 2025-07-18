use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type exclist;
    pub type exclude;
    pub type mode_change;
    pub type quoting_options;
    pub type wordsplit_node;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn rpmatch(__response: *const libc::c_char) -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn base_name(file: *const libc::c_char) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn dcngettext(
        __domainname: *const libc::c_char,
        __msgid1: *const libc::c_char,
        __msgid2: *const libc::c_char,
        __n: libc::c_ulong,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    static mut error_hook: Option::<unsafe extern "C" fn() -> ()>;
    static mut exit_status: libc::c_int;
    fn open_fatal(_: *const libc::c_char) -> !;
    fn stat_error(_: *const libc::c_char);
    fn fatal_exit() -> !;
    fn argp_parse(
        __argp: *const argp,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        __flags: libc::c_uint,
        __arg_index: *mut libc::c_int,
        __input: *mut libc::c_void,
    ) -> error_t;
    fn argp_help(
        __argp: *const argp,
        __stream: *mut FILE,
        __flags: libc::c_uint,
        __name: *mut libc::c_char,
    );
    fn argp_error(__state: *const argp_state, __fmt: *const libc::c_char, _: ...);
    fn argp_version_setup(
        name: *const libc::c_char,
        authors: *const *const libc::c_char,
    );
    static mut stdlis: *mut FILE;
    fn closeout_volume_number();
    fn compute_duration() -> libc::c_double;
    fn init_volume_number();
    fn print_total_stats();
    fn set_start_time();
    fn create_archive();
    fn check_links();
    fn diff_archive();
    fn diff_init();
    fn extr_init();
    fn extract_archive();
    fn extract_finish();
    fn delete_archive_members();
    fn show_snapshot_field_ranges();
    static mut current_header: *mut block;
    static mut current_format: archive_format;
    fn tartime(t: timespec, full_time: bool) -> *const libc::c_char;
    fn list_archive();
    fn test_archive_label();
    fn read_and(do_something: Option::<unsafe extern "C" fn() -> ()>);
    fn code_timespec(ts: timespec, sbuf: *mut libc::c_char) -> *const libc::c_char;
    fn close_diag(name: *const libc::c_char);
    static mut filename_args: files_count;
    fn gname_to_gid(gname: *const libc::c_char, pgid: *mut gid_t) -> libc::c_int;
    fn uname_to_uid(uname: *const libc::c_char, puid: *mut uid_t) -> libc::c_int;
    fn name_init();
    fn name_add_name(name: *const libc::c_char);
    fn name_term();
    fn add_starting_file(file_name: *const libc::c_char);
    fn update_archive();
    fn xheader_destroy(hdr: *mut xheader);
    fn xheader_set_option(string: *mut libc::c_char);
    fn xheader_xattr_free(vals: *mut xattr_array, sz: size_t);
    fn set_transform_expr(expr: *const libc::c_char);
    fn set_compression_program_by_suffix(
        name: *const libc::c_char,
        defprog: *const libc::c_char,
    );
    fn strip_compression_suffix(name: *const libc::c_char) -> *mut libc::c_char;
    fn checkpoint_compile_action(str: *const libc::c_char);
    fn checkpoint_finish_compile();
    fn checkpoint_finish();
    fn checkpoint_flush_actions();
    fn xattrs_mask_add(mask: *const libc::c_char, incl: bool);
    fn owner_map_read(name: *const libc::c_char);
    fn group_map_read(file: *const libc::c_char);
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    static mut simple_backup_suffix: *const libc::c_char;
    fn set_warning_option(arg: *const libc::c_char);
    fn _obstack_newchunk(_: *mut obstack, _: size_t);
    static mut warning_option: libc::c_int;
    fn xget_version(
        context: *const libc::c_char,
        arg: *const libc::c_char,
    ) -> backup_type;
    fn mode_compile(_: *const libc::c_char) -> *mut mode_change;
    fn new_exclude() -> *mut exclude;
    fn info_free_exclist(dir: *mut tar_stat_info);
    fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn _obstack_free(_: *mut obstack, _: *mut libc::c_void);
    fn _obstack_begin(
        _: *mut obstack,
        _: size_t,
        _: size_t,
        _: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    fn xattrs_clear_setup();
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    static mut argmatch_die: argmatch_exit_fn;
    fn __xargmatch_internal(
        context: *const libc::c_char,
        arg: *const libc::c_char,
        arglist: *const *const libc::c_char,
        vallist: *const libc::c_void,
        valsize: size_t,
        exit_fn: argmatch_exit_fn,
    ) -> ptrdiff_t;
    fn close_stdout_set_file_name(file: *const libc::c_char);
    fn close_stdout();
    static mut exit_failure: libc::c_int;
    fn parse_datetime(
        _: *mut timespec,
        _: *const libc::c_char,
        _: *const timespec,
    ) -> bool;
    static mut rmt_command: *const libc::c_char;
    static mut force_local_option: bool;
    fn wordsplit(
        s: *const libc::c_char,
        ws: *mut wordsplit_t,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn wordsplit_free(ws: *mut wordsplit_t);
    fn wordsplit_strerror(ws: *mut wordsplit_t) -> *const libc::c_char;
    static quoting_style_args: [*const libc::c_char; 0];
    fn set_quoting_style(o: *mut quoting_options, s: quoting_style);
    fn set_char_quoting(
        o: *mut quoting_options,
        c: libc::c_char,
        i: libc::c_int,
    ) -> libc::c_int;
    fn quotearg_colon(arg: *const libc::c_char) -> *mut libc::c_char;
    fn xstrtoumax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut uintmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn stdopen() -> libc::c_int;
    static mut names_argp: argp;
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
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type time_t = __time_t;
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
pub type ptrdiff_t = libc::c_long;
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
pub type savedir_option = libc::c_uint;
pub const SAVEDIR_SORT_FASTREAD: savedir_option = 2;
pub const SAVEDIR_SORT_INODE: savedir_option = 2;
pub const SAVEDIR_SORT_NAME: savedir_option = 1;
pub const SAVEDIR_SORT_NONE: savedir_option = 0;
pub const DEFAULT_MXFAST: C2RustUnnamed_3 = 128;
pub type C2RustUnnamed_3 = libc::c_uint;
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
pub struct posix_header {
    pub name: [libc::c_char; 100],
    pub mode: [libc::c_char; 8],
    pub uid: [libc::c_char; 8],
    pub gid: [libc::c_char; 8],
    pub size: [libc::c_char; 12],
    pub mtime: [libc::c_char; 12],
    pub chksum: [libc::c_char; 8],
    pub typeflag: libc::c_char,
    pub linkname: [libc::c_char; 100],
    pub magic: [libc::c_char; 6],
    pub version: [libc::c_char; 2],
    pub uname: [libc::c_char; 32],
    pub gname: [libc::c_char; 32],
    pub devmajor: [libc::c_char; 8],
    pub devminor: [libc::c_char; 8],
    pub prefix: [libc::c_char; 155],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sparse {
    pub offset: [libc::c_char; 12],
    pub numbytes: [libc::c_char; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sparse_header {
    pub sp: [sparse; 21],
    pub isextended: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oldgnu_header {
    pub unused_pad1: [libc::c_char; 345],
    pub atime: [libc::c_char; 12],
    pub ctime: [libc::c_char; 12],
    pub offset: [libc::c_char; 12],
    pub longnames: [libc::c_char; 4],
    pub unused_pad2: libc::c_char,
    pub sp: [sparse; 4],
    pub isextended: libc::c_char,
    pub realsize: [libc::c_char; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_header {
    pub name: [libc::c_char; 100],
    pub mode: [libc::c_char; 8],
    pub uid: [libc::c_char; 8],
    pub gid: [libc::c_char; 8],
    pub size: [libc::c_char; 12],
    pub mtime: [libc::c_char; 12],
    pub chksum: [libc::c_char; 8],
    pub typeflag: libc::c_char,
    pub linkname: [libc::c_char; 100],
    pub magic: [libc::c_char; 6],
    pub version: [libc::c_char; 2],
    pub uname: [libc::c_char; 32],
    pub gname: [libc::c_char; 32],
    pub devmajor: [libc::c_char; 8],
    pub devminor: [libc::c_char; 8],
    pub prefix: [libc::c_char; 131],
    pub atime: [libc::c_char; 12],
    pub ctime: [libc::c_char; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_in_header {
    pub fill: [libc::c_char; 345],
    pub prefix: [libc::c_char; 1],
    pub fill2: libc::c_char,
    pub fill3: [libc::c_char; 8],
    pub isextended: libc::c_char,
    pub sp: [sparse; 4],
    pub realsize: [libc::c_char; 12],
    pub offset: [libc::c_char; 12],
    pub atime: [libc::c_char; 12],
    pub ctime: [libc::c_char; 12],
    pub mfill: [libc::c_char; 8],
    pub xmagic: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_ext_header {
    pub sp: [sparse; 21],
    pub isextended: libc::c_char,
}
pub type archive_format = libc::c_uint;
pub const GNU_FORMAT: archive_format = 6;
pub const STAR_FORMAT: archive_format = 5;
pub const POSIX_FORMAT: archive_format = 4;
pub const USTAR_FORMAT: archive_format = 3;
pub const OLDGNU_FORMAT: archive_format = 2;
pub const V7_FORMAT: archive_format = 1;
pub const DEFAULT_FORMAT: archive_format = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union block {
    pub buffer: [libc::c_char; 512],
    pub header: posix_header,
    pub star_header: star_header,
    pub oldgnu_header: oldgnu_header,
    pub sparse_header: sparse_header,
    pub star_in_header: star_in_header,
    pub star_ext_header: star_ext_header,
}
pub type tarlong = libc::c_double;
pub type backup_type = libc::c_uint;
pub const numbered_backups: backup_type = 3;
pub const numbered_existing_backups: backup_type = 2;
pub const simple_backups: backup_type = 1;
pub const no_backups: backup_type = 0;
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
pub type atime_preserve = libc::c_uint;
pub const system_atime_preserve: atime_preserve = 2;
pub const replace_atime_preserve: atime_preserve = 1;
pub const no_atime_preserve: atime_preserve = 0;
pub type old_files = libc::c_uint;
pub const KEEP_NEWER_FILES: old_files = 6;
pub const SKIP_OLD_FILES: old_files = 5;
pub const KEEP_OLD_FILES: old_files = 4;
pub const UNLINK_FIRST_OLD_FILES: old_files = 3;
pub const OVERWRITE_OLD_FILES: old_files = 2;
pub const NO_OVERWRITE_DIR_OLD_FILES: old_files = 1;
pub const DEFAULT_OLD_FILES: old_files = 0;
pub type set_mtime_option_mode = libc::c_uint;
pub const CLAMP_MTIME: set_mtime_option_mode = 2;
pub const FORCE_MTIME: set_mtime_option_mode = 1;
pub const USE_FILE_MTIME: set_mtime_option_mode = 0;
pub type hole_detection_method = libc::c_uint;
pub const HOLE_DETECTION_SEEK: hole_detection_method = 2;
pub const HOLE_DETECTION_RAW: hole_detection_method = 1;
pub const HOLE_DETECTION_DEFAULT: hole_detection_method = 0;
pub type files_count = libc::c_uint;
pub const FILES_MANY: files_count = 2;
pub const FILES_ONE: files_count = 1;
pub const FILES_NONE: files_count = 0;
pub const escape_quoting_style: quoting_style = 7;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fmttab {
    pub name: *const libc::c_char,
    pub fmt: archive_format,
}
pub const ZSTD_OPTION: C2RustUnnamed_4 = 207;
pub const LZOP_OPTION: C2RustUnnamed_4 = 150;
pub const LZMA_OPTION: C2RustUnnamed_4 = 149;
pub const LZIP_OPTION: C2RustUnnamed_4 = 148;
pub const GRID_FILE_NAME: C2RustUnnamed_5 = 4;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct textual_date {
    pub next: *mut textual_date,
    pub ts: timespec,
    pub rpl_option: *const libc::c_char,
    pub date: *mut libc::c_char,
}
pub const SAME_OWNER_OPTION: C2RustUnnamed_4 = 185;
pub const XATTR_INCLUDE: C2RustUnnamed_4 = 206;
pub const XATTR_EXCLUDE: C2RustUnnamed_4 = 205;
pub const NO_XATTR_OPTION: C2RustUnnamed_4 = 165;
pub const XATTR_OPTION: C2RustUnnamed_4 = 204;
pub const NO_SELINUX_CONTEXT_OPTION: C2RustUnnamed_4 = 164;
pub const SELINUX_CONTEXT_OPTION: C2RustUnnamed_4 = 186;
pub const NO_ACLS_OPTION: C2RustUnnamed_4 = 154;
pub const ACLS_OPTION: C2RustUnnamed_4 = 128;
pub const NO_SAME_PERMISSIONS_OPTION: C2RustUnnamed_4 = 162;
pub const NO_SAME_OWNER_OPTION: C2RustUnnamed_4 = 161;
pub const VOLNO_FILE_OPTION: C2RustUnnamed_4 = 202;
pub const OC_COMPRESS: option_class = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigtab {
    pub name: *const libc::c_char,
    pub signo: libc::c_int,
}
pub const TOTALS_OPTION: C2RustUnnamed_4 = 198;
pub const TO_COMMAND_OPTION: C2RustUnnamed_4 = 199;
pub const SUFFIX_OPTION: C2RustUnnamed_4 = 196;
pub type argmatch_exit_fn = Option::<unsafe extern "C" fn() -> ()>;
pub const SORT_OPTION: C2RustUnnamed_4 = 192;
pub const SHOW_TRANSFORMED_NAMES_OPTION: C2RustUnnamed_4 = 190;
pub const SHOW_OMITTED_DIRS_OPTION: C2RustUnnamed_4 = 188;
pub const LONGINT_OK: strtol_error = 0;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const STRIP_COMPONENTS_OPTION: C2RustUnnamed_4 = 195;
pub const SHOW_SNAPSHOT_FIELD_RANGES_OPTION: C2RustUnnamed_4 = 189;
pub const SHOW_DEFAULTS_OPTION: C2RustUnnamed_4 = 187;
pub const RSH_COMMAND_OPTION: C2RustUnnamed_4 = 184;
pub const RMT_COMMAND_OPTION: C2RustUnnamed_4 = 183;
pub const RESTRICT_OPTION: C2RustUnnamed_4 = 182;
pub const REMOVE_FILES_OPTION: C2RustUnnamed_4 = 181;
pub const RECURSIVE_UNLINK_OPTION: C2RustUnnamed_4 = 180;
pub const RECORD_SIZE_OPTION: C2RustUnnamed_4 = 179;
pub const POSIX_OPTION: C2RustUnnamed_4 = 176;
pub const PAX_OPTION: C2RustUnnamed_4 = 175;
pub type quoting_style = libc::c_uint;
pub const custom_quoting_style: quoting_style = 10;
pub const clocale_quoting_style: quoting_style = 9;
pub const locale_quoting_style: quoting_style = 8;
pub const c_maybe_quoting_style: quoting_style = 6;
pub const c_quoting_style: quoting_style = 5;
pub const shell_escape_always_quoting_style: quoting_style = 4;
pub const shell_escape_quoting_style: quoting_style = 3;
pub const shell_always_quoting_style: quoting_style = 2;
pub const shell_quoting_style: quoting_style = 1;
pub const literal_quoting_style: quoting_style = 0;
pub const QUOTING_STYLE_OPTION: C2RustUnnamed_4 = 178;
pub const QUOTE_CHARS_OPTION: C2RustUnnamed_4 = 177;
pub const OWNER_MAP_OPTION: C2RustUnnamed_4 = 174;
pub const OWNER_OPTION: C2RustUnnamed_4 = 173;
pub const OC_OLD_FILES: option_class = 9;
pub const OVERWRITE_OPTION: C2RustUnnamed_4 = 172;
pub const OVERWRITE_DIR_OPTION: C2RustUnnamed_4 = 171;
pub const OLD_ARCHIVE_OPTION: C2RustUnnamed_4 = 168;
pub const OC_OCCURRENCE: option_class = 1;
pub const OCCURRENCE_OPTION: C2RustUnnamed_4 = 167;
pub const NUMERIC_OWNER_OPTION: C2RustUnnamed_4 = 166;
pub const NO_QUOTE_CHARS_OPTION: C2RustUnnamed_4 = 160;
pub const NO_OVERWRITE_DIR_OPTION: C2RustUnnamed_4 = 159;
pub const NO_IGNORE_COMMAND_ERROR_OPTION: C2RustUnnamed_4 = 158;
pub const MODE_OPTION: C2RustUnnamed_4 = 151;
pub const GROUP_MAP_OPTION: C2RustUnnamed_4 = 141;
pub const GROUP_OPTION: C2RustUnnamed_4 = 140;
pub const KEEP_NEWER_FILES_OPTION: C2RustUnnamed_4 = 146;
pub const KEEP_DIRECTORY_SYMLINK_OPTION: C2RustUnnamed_4 = 145;
pub const IGNORE_FAILED_READ_OPTION: C2RustUnnamed_4 = 143;
pub const IGNORE_COMMAND_ERROR_OPTION: C2RustUnnamed_4 = 142;
pub const INDEX_FILE_OPTION: C2RustUnnamed_4 = 144;
pub const FORCE_LOCAL_OPTION: C2RustUnnamed_4 = 138;
pub const DELETE_OPTION: C2RustUnnamed_4 = 137;
pub const NO_DELAY_DIRECTORY_RESTORE_OPTION: C2RustUnnamed_4 = 157;
pub const DELAY_DIRECTORY_RESTORE_OPTION: C2RustUnnamed_4 = 135;
pub const BACKUP_OPTION: C2RustUnnamed_4 = 130;
pub const CHECKPOINT_ACTION_OPTION: C2RustUnnamed_4 = 133;
pub const CHECKPOINT_OPTION: C2RustUnnamed_4 = 132;
pub const NO_CHECK_DEVICE_OPTION: C2RustUnnamed_4 = 156;
pub const CHECK_DEVICE_OPTION: C2RustUnnamed_4 = 131;
pub const ATIME_PRESERVE_OPTION: C2RustUnnamed_4 = 129;
pub const WARNING_OPTION: C2RustUnnamed_4 = 203;
pub const OC_VERIFY: option_class = 4;
pub const UTC_OPTION: C2RustUnnamed_4 = 201;
pub const TRANSFORM_OPTION: C2RustUnnamed_4 = 200;
pub const TEST_LABEL_OPTION: C2RustUnnamed_4 = 197;
pub const SPARSE_VERSION_OPTION: C2RustUnnamed_4 = 194;
pub const HOLE_DETECTION_OPTION: C2RustUnnamed_4 = 193;
pub const SKIP_OLD_FILES_OPTION: C2RustUnnamed_4 = 191;
pub const OC_SAME_ORDER: option_class = 6;
pub const OC_ABSOLUTE_NAMES: option_class = 8;
pub const OC_NEWER: option_class = 3;
pub const NEWER_MTIME_OPTION: C2RustUnnamed_4 = 153;
pub const NO_SEEK_OPTION: C2RustUnnamed_4 = 163;
pub const MTIME_OPTION: C2RustUnnamed_4 = 152;
pub const LEVEL_OPTION: C2RustUnnamed_4 = 147;
pub const OC_ONE_TOP_LEVEL: option_class = 7;
pub const ONE_TOP_LEVEL_OPTION: C2RustUnnamed_4 = 170;
pub const ONE_FILE_SYSTEM_OPTION: C2RustUnnamed_4 = 169;
pub const OC_STARTING_FILE: option_class = 5;
pub const HARD_DEREFERENCE_OPTION: C2RustUnnamed_4 = 136;
pub const OC_LISTED_INCREMENTAL: option_class = 2;
pub const FULL_TIME_OPTION: C2RustUnnamed_4 = 139;
pub const CLAMP_MTIME_OPTION: C2RustUnnamed_4 = 134;
pub const NO_AUTO_COMPRESS_OPTION: C2RustUnnamed_4 = 155;
pub const GRH_OTHER: C2RustUnnamed_5 = 31;
pub const GRID_COMPAT: C2RustUnnamed_5 = 30;
pub const GRH_COMPAT: C2RustUnnamed_5 = 29;
pub const GRID_INFORMATIVE: C2RustUnnamed_5 = 28;
pub const GRH_INFORMATIVE: C2RustUnnamed_5 = 27;
pub const GRID_NAME_XFORM: C2RustUnnamed_5 = 26;
pub const GRH_NAME_XFORM: C2RustUnnamed_5 = 25;
pub const GRID_FILE: C2RustUnnamed_5 = 24;
pub const GRH_FILE: C2RustUnnamed_5 = 23;
pub const GRID_COMPRESS: C2RustUnnamed_5 = 22;
pub const GRH_COMPRESS: C2RustUnnamed_5 = 21;
pub const GRID_FORMAT_OPT: C2RustUnnamed_5 = 20;
pub const GRDOC_FORMAT: C2RustUnnamed_5 = 19;
pub const GRID_FORMAT: C2RustUnnamed_5 = 18;
pub const GRH_FORMAT: C2RustUnnamed_5 = 17;
pub const GRID_BLOCKING: C2RustUnnamed_5 = 16;
pub const GRH_BLOCKING: C2RustUnnamed_5 = 15;
pub const GRID_DEVICE: C2RustUnnamed_5 = 14;
pub const GRH_DEVICE: C2RustUnnamed_5 = 13;
pub const GRID_XATTR: C2RustUnnamed_5 = 12;
pub const GRH_XATTR: C2RustUnnamed_5 = 11;
pub const GRID_FATTR: C2RustUnnamed_5 = 10;
pub const GRH_FATTR: C2RustUnnamed_5 = 9;
pub const GRID_OUTPUT: C2RustUnnamed_5 = 8;
pub const GRH_OUTPUT: C2RustUnnamed_5 = 7;
pub const GRID_OVERWRITE: C2RustUnnamed_5 = 6;
pub const GRH_OVERWRITE: C2RustUnnamed_5 = 5;
pub const GRID_MODIFIER: C2RustUnnamed_5 = 3;
pub const GRH_MODIFIER: C2RustUnnamed_5 = 2;
pub const GRID_COMMAND: C2RustUnnamed_5 = 1;
pub const GRH_COMMAND: C2RustUnnamed_5 = 0;
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
pub type C2RustUnnamed_4 = libc::c_uint;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const GRID_OTHER: C2RustUnnamed_5 = 32;
pub type option_class = libc::c_uint;
pub const OC_MAX: option_class = 10;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fputc_unlocked(
    mut __c: libc::c_int,
    mut __stream: *mut FILE,
) -> libc::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(__stream, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh0 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh0 = __c as libc::c_char;
        *fresh0 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut libc::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn x2nrealloc(
    mut p: *mut libc::c_void,
    mut pn: *mut size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut n: size_t = *pn;
    if p.is_null() {
        if n == 0 {
            n = (DEFAULT_MXFAST as libc::c_int as libc::c_ulong).wrapping_div(s);
            n = (n as libc::c_ulong)
                .wrapping_add((n == 0) as libc::c_int as libc::c_ulong) as size_t
                as size_t;
        }
        if (if (9223372036854775807 as libc::c_long as libc::c_ulong)
            < 18446744073709551615 as libc::c_ulong
        {
            9223372036854775807 as libc::c_long as libc::c_ulong
        } else {
            (18446744073709551615 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        })
            .wrapping_div(s) < n
        {
            xalloc_die();
        }
    } else {
        if (if (9223372036854775807 as libc::c_long as libc::c_ulong)
            < 18446744073709551615 as libc::c_ulong
        {
            9223372036854775807 as libc::c_long as libc::c_ulong
        } else {
            18446744073709551615 as libc::c_ulong
        })
            .wrapping_div(3 as libc::c_int as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_div(s) <= n
        {
            xalloc_die();
        }
        n = (n as libc::c_ulong)
            .wrapping_add(
                n
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
    }
    *pn = n;
    return xrealloc(p, n.wrapping_mul(s));
}
pub static mut subcommand_option: subcommand = UNKNOWN_SUBCOMMAND;
pub static mut archive_format: archive_format = DEFAULT_FORMAT;
pub static mut blocking_factor: libc::c_int = 0;
pub static mut record_size: size_t = 0;
pub static mut absolute_names_option: bool = false;
pub static mut utc_option: bool = false;
pub static mut full_time_option: bool = false;
pub static mut after_date_option: libc::c_int = 0;
pub static mut atime_preserve_option: atime_preserve = no_atime_preserve;
pub static mut backup_option: bool = false;
pub static mut backup_type: backup_type = no_backups;
pub static mut block_number_option: bool = false;
pub static mut checkpoint_option: libc::c_uint = 0;
pub static mut use_compress_program_option: *const libc::c_char = 0
    as *const libc::c_char;
pub static mut dereference_option: bool = false;
pub static mut hard_dereference_option: bool = false;
pub static mut excluded: *mut exclude = 0 as *const exclude as *mut exclude;
pub static mut group_name_option: *const libc::c_char = 0 as *const libc::c_char;
pub static mut group_option: gid_t = 0;
pub static mut ignore_failed_read_option: bool = false;
pub static mut ignore_zeros_option: bool = false;
pub static mut incremental_option: bool = false;
pub static mut info_script_option: *const libc::c_char = 0 as *const libc::c_char;
pub static mut interactive_option: bool = false;
pub static mut occurrence_option: uintmax_t = 0;
pub static mut old_files_option: old_files = DEFAULT_OLD_FILES;
pub static mut keep_directory_symlink_option: bool = false;
pub static mut listed_incremental_option: *const libc::c_char = 0 as *const libc::c_char;
pub static mut incremental_level: libc::c_int = 0;
pub static mut check_device_option: bool = false;
pub static mut mode_option: *mut mode_change = 0 as *const mode_change
    as *mut mode_change;
pub static mut initial_umask: mode_t = 0;
pub static mut multi_volume_option: bool = false;
pub static mut newer_mtime_option: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
pub static mut set_mtime_option: set_mtime_option_mode = USE_FILE_MTIME;
pub static mut mtime_option: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
pub static mut recursion_option: libc::c_int = 0;
pub static mut numeric_owner_option: bool = false;
pub static mut one_file_system_option: bool = false;
pub static mut one_top_level_option: bool = false;
pub static mut one_top_level_dir: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut owner_name_option: *const libc::c_char = 0 as *const libc::c_char;
pub static mut owner_option: uid_t = 0;
pub static mut recursive_unlink_option: bool = false;
pub static mut read_full_records_option: bool = false;
pub static mut remove_files_option: bool = false;
pub static mut rsh_command_option: *const libc::c_char = 0 as *const libc::c_char;
pub static mut same_order_option: bool = false;
pub static mut same_owner_option: libc::c_int = 0;
pub static mut same_permissions_option: libc::c_int = 0;
pub static mut selinux_context_option: libc::c_int = 0;
pub static mut acls_option: libc::c_int = 0;
pub static mut xattrs_option: libc::c_int = 0;
pub static mut strip_name_components: size_t = 0;
pub static mut show_omitted_dirs_option: bool = false;
pub static mut sparse_option: bool = false;
pub static mut tar_sparse_major: libc::c_uint = 0;
pub static mut tar_sparse_minor: libc::c_uint = 0;
pub static mut hole_detection: hole_detection_method = HOLE_DETECTION_DEFAULT;
pub static mut starting_file_option: bool = false;
pub static mut tape_length_option: tarlong = 0.;
pub static mut to_stdout_option: bool = false;
pub static mut totals_option: bool = false;
pub static mut touch_option: bool = false;
pub static mut to_command_option: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut ignore_command_error_option: bool = false;
pub static mut restrict_option: bool = false;
pub static mut verbose_option: libc::c_int = 0;
pub static mut verify_option: bool = false;
pub static mut volno_file_option: *const libc::c_char = 0 as *const libc::c_char;
pub static mut volume_label_option: *const libc::c_char = 0 as *const libc::c_char;
pub static mut posixly_correct: bool = false;
pub static mut archive: libc::c_int = 0;
pub static mut dev_null_output: bool = false;
pub static mut start_time: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
pub static mut volume_start_time: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
pub static mut last_stat_time: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
pub static mut current_stat_info: tar_stat_info = tar_stat_info {
    orig_file_name: 0 as *const libc::c_char as *mut libc::c_char,
    file_name: 0 as *const libc::c_char as *mut libc::c_char,
    had_trailing_slash: false,
    link_name: 0 as *const libc::c_char as *mut libc::c_char,
    uname: 0 as *const libc::c_char as *mut libc::c_char,
    gname: 0 as *const libc::c_char as *mut libc::c_char,
    cntx_name: 0 as *const libc::c_char as *mut libc::c_char,
    acls_a_ptr: 0 as *const libc::c_char as *mut libc::c_char,
    acls_a_len: 0,
    acls_d_ptr: 0 as *const libc::c_char as *mut libc::c_char,
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
    sparse_map: 0 as *const sp_array as *mut sp_array,
    real_size: 0,
    real_size_set: false,
    sparse_name_done: false,
    xattr_map_size: 0,
    xattr_map: 0 as *const xattr_array as *mut xattr_array,
    xhdr: xheader {
        stk: 0 as *const obstack as *mut obstack,
        size: 0,
        buffer: 0 as *const libc::c_char as *mut libc::c_char,
        string_length: 0,
    },
    is_dumpdir: false,
    skipped: false,
    dumpdir: 0 as *const libc::c_char as *mut libc::c_char,
    parent: 0 as *const tar_stat_info as *mut tar_stat_info,
    dirstream: 0 as *const DIR as *mut DIR,
    fd: 0,
    exclude_list: 0 as *const exclist as *mut exclist,
};
pub static mut archive_name_array: *mut *const libc::c_char = 0
    as *const *const libc::c_char as *mut *const libc::c_char;
pub static mut archive_names: size_t = 0;
pub static mut archive_name_cursor: *mut *const libc::c_char = 0
    as *const *const libc::c_char as *mut *const libc::c_char;
pub static mut index_file_name: *const libc::c_char = 0 as *const libc::c_char;
pub static mut ar_dev: dev_t = 0;
pub static mut ar_ino: ino_t = 0;
pub static mut open_read_flags: libc::c_int = 0;
pub static mut open_searchdir_flags: libc::c_int = 0;
pub static mut fstatat_flags: libc::c_int = 0;
pub static mut seek_option: libc::c_int = 0;
pub static mut seekable_archive: bool = false;
pub static mut root_device: dev_t = 0;
pub static mut unquote_option: bool = false;
pub static mut savedir_sort_order: libc::c_int = 0;
pub static mut show_transformed_names_option: bool = false;
pub static mut delay_directory_restore_option: bool = false;
#[inline]
unsafe extern "C" fn name_more_files() -> bool {
    return filename_args as libc::c_uint != FILES_NONE as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn timespec_cmp(mut a: timespec, mut b: timespec) -> libc::c_int {
    return 2 as libc::c_int
        * ((a.tv_sec > b.tv_sec) as libc::c_int - (a.tv_sec < b.tv_sec) as libc::c_int)
        + ((a.tv_nsec > b.tv_nsec) as libc::c_int
            - (a.tv_nsec < b.tv_nsec) as libc::c_int);
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
#[inline]
unsafe extern "C" fn priv_set_remove_linkdir() -> libc::c_int {
    return -(1 as libc::c_int);
}
static mut check_links_option: libc::c_int = 0;
static mut allocated_archive_names: size_t = 0;
static mut stdin_used_by: *const libc::c_char = 0 as *const libc::c_char;
pub unsafe extern "C" fn request_stdin(mut rpl_option: *const libc::c_char) {
    if !stdin_used_by.is_null() {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Options '%s' and '%s' both want standard input\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdin_used_by,
            rpl_option,
        );
        usage(2 as libc::c_int);
    }
    stdin_used_by = rpl_option;
}
pub unsafe extern "C" fn confirm(
    mut message_action: *const libc::c_char,
    mut message_name: *const libc::c_char,
) -> libc::c_int {
    static mut confirm_file: *mut FILE = 0 as *const FILE as *mut FILE;
    static mut confirm_file_EOF: libc::c_int = 0;
    let mut status: bool = 0 as libc::c_int != 0;
    if confirm_file.is_null() {
        if archive == 0 as libc::c_int || !stdin_used_by.is_null() {
            confirm_file = fopen(
                b"/dev/tty\0" as *const u8 as *const libc::c_char,
                b"r\0" as *const u8 as *const libc::c_char,
            );
            if confirm_file.is_null() {
                open_fatal(b"/dev/tty\0" as *const u8 as *const libc::c_char);
            }
        } else {
            request_stdin(b"-w\0" as *const u8 as *const libc::c_char);
            confirm_file = stdin;
        }
    }
    fprintf(
        stdlis,
        b"%s %s?\0" as *const u8 as *const libc::c_char,
        message_action,
        quote(message_name),
    );
    fflush_unlocked(stdlis);
    if confirm_file_EOF == 0 {
        let mut response: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut response_size: size_t = 0 as libc::c_int as size_t;
        if getline(&mut response, &mut response_size, confirm_file)
            < 0 as libc::c_int as libc::c_long
        {
            confirm_file_EOF = 1 as libc::c_int;
        } else {
            status = rpmatch(response) > 0 as libc::c_int;
        }
        rpl_free(response as *mut libc::c_void);
    }
    if confirm_file_EOF != 0 {
        fputc_unlocked('\n' as i32, stdlis);
        fflush_unlocked(stdlis);
    }
    return status as libc::c_int;
}
static mut fmttab: [fmttab; 7] = [
    {
        let mut init = fmttab {
            name: b"v7\0" as *const u8 as *const libc::c_char,
            fmt: V7_FORMAT,
        };
        init
    },
    {
        let mut init = fmttab {
            name: b"oldgnu\0" as *const u8 as *const libc::c_char,
            fmt: OLDGNU_FORMAT,
        };
        init
    },
    {
        let mut init = fmttab {
            name: b"ustar\0" as *const u8 as *const libc::c_char,
            fmt: USTAR_FORMAT,
        };
        init
    },
    {
        let mut init = fmttab {
            name: b"posix\0" as *const u8 as *const libc::c_char,
            fmt: POSIX_FORMAT,
        };
        init
    },
    {
        let mut init = fmttab {
            name: b"gnu\0" as *const u8 as *const libc::c_char,
            fmt: GNU_FORMAT,
        };
        init
    },
    {
        let mut init = fmttab {
            name: b"pax\0" as *const u8 as *const libc::c_char,
            fmt: POSIX_FORMAT,
        };
        init
    },
    {
        let mut init = fmttab {
            name: 0 as *const libc::c_char,
            fmt: DEFAULT_FORMAT,
        };
        init
    },
];
unsafe extern "C" fn set_archive_format(mut name: *const libc::c_char) {
    let mut p: *const fmttab = 0 as *const fmttab;
    p = fmttab.as_ptr();
    while strcmp((*p).name, name) != 0 as libc::c_int {
        p = p.offset(1);
        if ((*p).name).is_null() {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Invalid archive format\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_colon(name),
            );
            usage(2 as libc::c_int);
        }
    }
    archive_format = (*p).fmt;
}
unsafe extern "C" fn set_xattr_option(mut value: libc::c_int) {
    if value == 1 as libc::c_int {
        set_archive_format(b"posix\0" as *const u8 as *const libc::c_char);
    }
    xattrs_option = value;
}
pub unsafe extern "C" fn archive_format_string(
    mut fmt: archive_format,
) -> *const libc::c_char {
    let mut p: *const fmttab = 0 as *const fmttab;
    p = fmttab.as_ptr();
    while !((*p).name).is_null() {
        if (*p).fmt as libc::c_uint == fmt as libc::c_uint {
            return (*p).name;
        }
        p = p.offset(1);
        p;
    }
    return b"unknown?\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn assert_format(mut fmt_mask: libc::c_uint) {
    if ((1 as libc::c_int) << archive_format as libc::c_uint) as libc::c_uint & fmt_mask
        == 0 as libc::c_int as libc::c_uint
    {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"GNU features wanted on incompatible archive format\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(2 as libc::c_int);
    }
}
pub unsafe extern "C" fn subcommand_string(mut c: subcommand) -> *const libc::c_char {
    match c as libc::c_uint {
        0 => return b"unknown?\0" as *const u8 as *const libc::c_char,
        1 => return b"-r\0" as *const u8 as *const libc::c_char,
        2 => return b"-A\0" as *const u8 as *const libc::c_char,
        3 => return b"-c\0" as *const u8 as *const libc::c_char,
        4 => return b"-D\0" as *const u8 as *const libc::c_char,
        5 => return b"-d\0" as *const u8 as *const libc::c_char,
        6 => return b"-x\0" as *const u8 as *const libc::c_char,
        7 => return b"-t\0" as *const u8 as *const libc::c_char,
        8 => return b"-u\0" as *const u8 as *const libc::c_char,
        9 => return b"--test-label\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    abort();
}
unsafe extern "C" fn tar_list_quoting_styles(
    mut stk: *mut obstack,
    mut prefix: *const libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut prefixlen: size_t = strlen(prefix);
    i = 0 as libc::c_int;
    while !(*quoting_style_args.as_ptr().offset(i as isize)).is_null() {
        let mut __o: *mut obstack = stk;
        let mut __len: size_t = prefixlen;
        if ({
            let mut __o1: *const obstack = __o;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) < __len
        {
            _obstack_newchunk(__o, __len);
        }
        memcpy(
            (*__o).next_free as *mut libc::c_void,
            prefix as *const libc::c_void,
            __len,
        );
        (*__o).next_free = ((*__o).next_free).offset(__len as isize);
        let mut __o_0: *mut obstack = stk;
        let mut __len_0: size_t = strlen(
            *quoting_style_args.as_ptr().offset(i as isize),
        );
        if ({
            let mut __o1: *const obstack = __o_0;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) < __len_0
        {
            _obstack_newchunk(__o_0, __len_0);
        }
        memcpy(
            (*__o_0).next_free as *mut libc::c_void,
            *quoting_style_args.as_ptr().offset(i as isize) as *const libc::c_void,
            __len_0,
        );
        (*__o_0).next_free = ((*__o_0).next_free).offset(__len_0 as isize);
        let mut __o_1: *mut obstack = stk;
        if ({
            let mut __o1: *const obstack = __o_1;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) < 1 as libc::c_int as libc::c_ulong
        {
            _obstack_newchunk(__o_1, 1 as libc::c_int as size_t);
        }
        let fresh1 = (*__o_1).next_free;
        (*__o_1).next_free = ((*__o_1).next_free).offset(1);
        *fresh1 = '\n' as i32 as libc::c_char;
        i += 1;
        i;
    }
}
unsafe extern "C" fn tar_set_quoting_style(mut arg: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(*quoting_style_args.as_ptr().offset(i as isize)).is_null() {
        if strcmp(arg, *quoting_style_args.as_ptr().offset(i as isize))
            == 0 as libc::c_int
        {
            set_quoting_style(0 as *mut quoting_options, i as quoting_style);
            return;
        }
        i += 1;
        i;
    }
    if error_hook.is_some() {
        error_hook.unwrap()();
    }
    error(
        0 as libc::c_int,
        0 as libc::c_int,
        dcgettext(
            0 as *const libc::c_char,
            b"Unknown quoting style '%s'. Try '%s --quoting-style=help' to get a list.\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        arg,
        program_name,
    );
    fatal_exit();
}
static mut doc: [libc::c_char; 702] = unsafe {
    *::std::mem::transmute::<
        &[u8; 702],
        &[libc::c_char; 702],
    >(
        b"GNU 'tar' saves many files together into a single tape or disk archive, and can restore individual files from the archive.\n\nExamples:\n  tar -cf archive.tar foo bar  # Create archive.tar from files foo and bar.\n  tar -tvf archive.tar         # List all files in archive.tar verbosely.\n  tar -xf archive.tar          # Extract all files from archive.tar.\n\x0BThe backup suffix is '~', unless set with --suffix or SIMPLE_BACKUP_SUFFIX.\nThe version control may be set with --backup or VERSION_CONTROL, values are:\n\n  none, off       never make backups\n  t, numbered     make numbered backups\n  nil, existing   numbered if numbered backups exist, simple otherwise\n  never, simple   always make simple backups\n\0",
    )
};
static mut options: [argp_option; 169] = [
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Main operation mode:\0" as *const u8 as *const libc::c_char,
            group: GRH_COMMAND as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"list\0" as *const u8 as *const libc::c_char,
            key: 't' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"list the contents of an archive\0" as *const u8
                as *const libc::c_char,
            group: GRID_COMMAND as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"extract\0" as *const u8 as *const libc::c_char,
            key: 'x' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"extract files from an archive\0" as *const u8 as *const libc::c_char,
            group: GRID_COMMAND as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"get\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x4 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_COMMAND as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"create\0" as *const u8 as *const libc::c_char,
            key: 'c' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"create a new archive\0" as *const u8 as *const libc::c_char,
            group: GRID_COMMAND as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"diff\0" as *const u8 as *const libc::c_char,
            key: 'd' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"find differences between archive and file system\0" as *const u8
                as *const libc::c_char,
            group: GRID_COMMAND as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"compare\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x4 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_COMMAND as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"append\0" as *const u8 as *const libc::c_char,
            key: 'r' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"append files to the end of an archive\0" as *const u8
                as *const libc::c_char,
            group: GRID_COMMAND as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"update\0" as *const u8 as *const libc::c_char,
            key: 'u' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"only append files newer than copy in archive\0" as *const u8
                as *const libc::c_char,
            group: GRID_COMMAND as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"catenate\0" as *const u8 as *const libc::c_char,
            key: 'A' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"append tar files to an archive\0" as *const u8 as *const libc::c_char,
            group: GRID_COMMAND as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"concatenate\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x4 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_COMMAND as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"delete\0" as *const u8 as *const libc::c_char,
            key: DELETE_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"delete from the archive (not on mag tapes!)\0" as *const u8
                as *const libc::c_char,
            group: GRID_COMMAND as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"test-label\0" as *const u8 as *const libc::c_char,
            key: TEST_LABEL_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"test the archive volume label and exit\0" as *const u8
                as *const libc::c_char,
            group: GRID_COMMAND as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Operation modifiers:\0" as *const u8 as *const libc::c_char,
            group: GRH_MODIFIER as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"sparse\0" as *const u8 as *const libc::c_char,
            key: 'S' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"handle sparse files efficiently\0" as *const u8
                as *const libc::c_char,
            group: GRID_MODIFIER as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"hole-detection\0" as *const u8 as *const libc::c_char,
            key: HOLE_DETECTION_OPTION as libc::c_int,
            arg: b"TYPE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"technique to detect holes\0" as *const u8 as *const libc::c_char,
            group: GRID_MODIFIER as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"sparse-version\0" as *const u8 as *const libc::c_char,
            key: SPARSE_VERSION_OPTION as libc::c_int,
            arg: b"MAJOR[.MINOR]\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"set version of the sparse format to use (implies --sparse)\0"
                as *const u8 as *const libc::c_char,
            group: GRID_MODIFIER as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"incremental\0" as *const u8 as *const libc::c_char,
            key: 'G' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"handle old GNU-format incremental backup\0" as *const u8
                as *const libc::c_char,
            group: GRID_MODIFIER as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"listed-incremental\0" as *const u8 as *const libc::c_char,
            key: 'g' as i32,
            arg: b"FILE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"handle new GNU-format incremental backup\0" as *const u8
                as *const libc::c_char,
            group: GRID_MODIFIER as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"level\0" as *const u8 as *const libc::c_char,
            key: LEVEL_OPTION as libc::c_int,
            arg: b"NUMBER\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"dump level for created listed-incremental archive\0" as *const u8
                as *const libc::c_char,
            group: GRID_MODIFIER as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"ignore-failed-read\0" as *const u8 as *const libc::c_char,
            key: IGNORE_FAILED_READ_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"do not exit with nonzero on unreadable files\0" as *const u8
                as *const libc::c_char,
            group: GRID_MODIFIER as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"occurrence\0" as *const u8 as *const libc::c_char,
            key: OCCURRENCE_OPTION as libc::c_int,
            arg: b"NUMBER\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
            doc: b"process only the NUMBERth occurrence of each file in the archive; this option is valid only in conjunction with one of the subcommands --delete, --diff, --extract or --list and when a list of files is given either on the command line or via the -T option; NUMBER defaults to 1\0"
                as *const u8 as *const libc::c_char,
            group: GRID_MODIFIER as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"seek\0" as *const u8 as *const libc::c_char,
            key: 'n' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"archive is seekable\0" as *const u8 as *const libc::c_char,
            group: GRID_MODIFIER as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-seek\0" as *const u8 as *const libc::c_char,
            key: NO_SEEK_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"archive is not seekable\0" as *const u8 as *const libc::c_char,
            group: GRID_MODIFIER as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-check-device\0" as *const u8 as *const libc::c_char,
            key: NO_CHECK_DEVICE_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"do not check device numbers when creating incremental archives\0"
                as *const u8 as *const libc::c_char,
            group: GRID_MODIFIER as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"check-device\0" as *const u8 as *const libc::c_char,
            key: CHECK_DEVICE_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"check device numbers when creating incremental archives (default)\0"
                as *const u8 as *const libc::c_char,
            group: GRID_MODIFIER as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Overwrite control:\0" as *const u8 as *const libc::c_char,
            group: GRH_OVERWRITE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"verify\0" as *const u8 as *const libc::c_char,
            key: 'W' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"attempt to verify the archive after writing it\0" as *const u8
                as *const libc::c_char,
            group: GRID_OVERWRITE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"remove-files\0" as *const u8 as *const libc::c_char,
            key: REMOVE_FILES_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"remove files after adding them to the archive\0" as *const u8
                as *const libc::c_char,
            group: GRID_OVERWRITE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"keep-old-files\0" as *const u8 as *const libc::c_char,
            key: 'k' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"don't replace existing files when extracting, treat them as errors\0"
                as *const u8 as *const libc::c_char,
            group: GRID_OVERWRITE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"skip-old-files\0" as *const u8 as *const libc::c_char,
            key: SKIP_OLD_FILES_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"don't replace existing files when extracting, silently skip over them\0"
                as *const u8 as *const libc::c_char,
            group: GRID_OVERWRITE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"keep-newer-files\0" as *const u8 as *const libc::c_char,
            key: KEEP_NEWER_FILES_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"don't replace existing files that are newer than their archive copies\0"
                as *const u8 as *const libc::c_char,
            group: GRID_OVERWRITE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"overwrite\0" as *const u8 as *const libc::c_char,
            key: OVERWRITE_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"overwrite existing files when extracting\0" as *const u8
                as *const libc::c_char,
            group: GRID_OVERWRITE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"unlink-first\0" as *const u8 as *const libc::c_char,
            key: 'U' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"remove each file prior to extracting over it\0" as *const u8
                as *const libc::c_char,
            group: GRID_OVERWRITE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"recursive-unlink\0" as *const u8 as *const libc::c_char,
            key: RECURSIVE_UNLINK_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"empty hierarchies prior to extracting directory\0" as *const u8
                as *const libc::c_char,
            group: GRID_OVERWRITE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-overwrite-dir\0" as *const u8 as *const libc::c_char,
            key: NO_OVERWRITE_DIR_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"preserve metadata of existing directories\0" as *const u8
                as *const libc::c_char,
            group: GRID_OVERWRITE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"overwrite-dir\0" as *const u8 as *const libc::c_char,
            key: OVERWRITE_DIR_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"overwrite metadata of existing directories when extracting (default)\0"
                as *const u8 as *const libc::c_char,
            group: GRID_OVERWRITE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"keep-directory-symlink\0" as *const u8 as *const libc::c_char,
            key: KEEP_DIRECTORY_SYMLINK_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"preserve existing symlinks to directories when extracting\0"
                as *const u8 as *const libc::c_char,
            group: GRID_OVERWRITE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"one-top-level\0" as *const u8 as *const libc::c_char,
            key: ONE_TOP_LEVEL_OPTION as libc::c_int,
            arg: b"DIR\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
            doc: b"create a subdirectory to avoid having loose files extracted\0"
                as *const u8 as *const libc::c_char,
            group: GRID_OVERWRITE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Select output stream:\0" as *const u8 as *const libc::c_char,
            group: GRH_OUTPUT as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"to-stdout\0" as *const u8 as *const libc::c_char,
            key: 'O' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"extract files to standard output\0" as *const u8
                as *const libc::c_char,
            group: GRID_OUTPUT as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"to-command\0" as *const u8 as *const libc::c_char,
            key: TO_COMMAND_OPTION as libc::c_int,
            arg: b"COMMAND\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"pipe extracted files to another program\0" as *const u8
                as *const libc::c_char,
            group: GRID_OUTPUT as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"ignore-command-error\0" as *const u8 as *const libc::c_char,
            key: IGNORE_COMMAND_ERROR_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"ignore exit codes of children\0" as *const u8 as *const libc::c_char,
            group: GRID_OUTPUT as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-ignore-command-error\0" as *const u8 as *const libc::c_char,
            key: NO_IGNORE_COMMAND_ERROR_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"treat non-zero exit codes of children as error\0" as *const u8
                as *const libc::c_char,
            group: GRID_OUTPUT as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Handling of file attributes:\0" as *const u8 as *const libc::c_char,
            group: GRH_FATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"owner\0" as *const u8 as *const libc::c_char,
            key: OWNER_OPTION as libc::c_int,
            arg: b"NAME\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"force NAME as owner for added files\0" as *const u8
                as *const libc::c_char,
            group: GRID_FATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"group\0" as *const u8 as *const libc::c_char,
            key: GROUP_OPTION as libc::c_int,
            arg: b"NAME\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"force NAME as group for added files\0" as *const u8
                as *const libc::c_char,
            group: GRID_FATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"owner-map\0" as *const u8 as *const libc::c_char,
            key: OWNER_MAP_OPTION as libc::c_int,
            arg: b"FILE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"use FILE to map file owner UIDs and names\0" as *const u8
                as *const libc::c_char,
            group: GRID_FATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"group-map\0" as *const u8 as *const libc::c_char,
            key: GROUP_MAP_OPTION as libc::c_int,
            arg: b"FILE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"use FILE to map file owner GIDs and names\0" as *const u8
                as *const libc::c_char,
            group: GRID_FATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"mtime\0" as *const u8 as *const libc::c_char,
            key: MTIME_OPTION as libc::c_int,
            arg: b"DATE-OR-FILE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"set mtime for added files from DATE-OR-FILE\0" as *const u8
                as *const libc::c_char,
            group: GRID_FATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"clamp-mtime\0" as *const u8 as *const libc::c_char,
            key: CLAMP_MTIME_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"only set time when the file is more recent than what was given with --mtime\0"
                as *const u8 as *const libc::c_char,
            group: GRID_FATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"mode\0" as *const u8 as *const libc::c_char,
            key: MODE_OPTION as libc::c_int,
            arg: b"CHANGES\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"force (symbolic) mode CHANGES for added files\0" as *const u8
                as *const libc::c_char,
            group: GRID_FATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"atime-preserve\0" as *const u8 as *const libc::c_char,
            key: ATIME_PRESERVE_OPTION as libc::c_int,
            arg: b"METHOD\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
            doc: b"preserve access times on dumped files, either by restoring the times after reading (METHOD='replace'; default) or by not setting the times in the first place (METHOD='system')\0"
                as *const u8 as *const libc::c_char,
            group: GRID_FATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"touch\0" as *const u8 as *const libc::c_char,
            key: 'm' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"don't extract file modified time\0" as *const u8
                as *const libc::c_char,
            group: GRID_FATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"same-owner\0" as *const u8 as *const libc::c_char,
            key: SAME_OWNER_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"try extracting files with the same ownership as exists in the archive (default for superuser)\0"
                as *const u8 as *const libc::c_char,
            group: GRID_FATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-same-owner\0" as *const u8 as *const libc::c_char,
            key: NO_SAME_OWNER_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"extract files as yourself (default for ordinary users)\0" as *const u8
                as *const libc::c_char,
            group: GRID_FATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"numeric-owner\0" as *const u8 as *const libc::c_char,
            key: NUMERIC_OWNER_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"always use numbers for user/group names\0" as *const u8
                as *const libc::c_char,
            group: GRID_FATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"preserve-permissions\0" as *const u8 as *const libc::c_char,
            key: 'p' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"extract information about file permissions (default for superuser)\0"
                as *const u8 as *const libc::c_char,
            group: GRID_FATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"same-permissions\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x4 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_FATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-same-permissions\0" as *const u8 as *const libc::c_char,
            key: NO_SAME_PERMISSIONS_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"apply the user's umask when extracting permissions from the archive (default for ordinary users)\0"
                as *const u8 as *const libc::c_char,
            group: GRID_FATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"preserve-order\0" as *const u8 as *const libc::c_char,
            key: 's' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"member arguments are listed in the same order as the files in the archive\0"
                as *const u8 as *const libc::c_char,
            group: GRID_FATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"same-order\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x4 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_FATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"delay-directory-restore\0" as *const u8 as *const libc::c_char,
            key: DELAY_DIRECTORY_RESTORE_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"delay setting modification times and permissions of extracted directories until the end of extraction\0"
                as *const u8 as *const libc::c_char,
            group: GRID_FATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-delay-directory-restore\0" as *const u8 as *const libc::c_char,
            key: NO_DELAY_DIRECTORY_RESTORE_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"cancel the effect of --delay-directory-restore option\0" as *const u8
                as *const libc::c_char,
            group: GRID_FATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"sort\0" as *const u8 as *const libc::c_char,
            key: SORT_OPTION as libc::c_int,
            arg: b"ORDER\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"directory sorting order: none (default), name or inode\0" as *const u8
                as *const libc::c_char,
            group: GRID_FATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Handling of extended file attributes:\0" as *const u8
                as *const libc::c_char,
            group: GRH_XATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"xattrs\0" as *const u8 as *const libc::c_char,
            key: XATTR_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Enable extended attributes support\0" as *const u8
                as *const libc::c_char,
            group: GRID_XATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-xattrs\0" as *const u8 as *const libc::c_char,
            key: NO_XATTR_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Disable extended attributes support\0" as *const u8
                as *const libc::c_char,
            group: GRID_XATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"xattrs-include\0" as *const u8 as *const libc::c_char,
            key: XATTR_INCLUDE as libc::c_int,
            arg: b"MASK\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"specify the include pattern for xattr keys\0" as *const u8
                as *const libc::c_char,
            group: GRID_XATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"xattrs-exclude\0" as *const u8 as *const libc::c_char,
            key: XATTR_EXCLUDE as libc::c_int,
            arg: b"MASK\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"specify the exclude pattern for xattr keys\0" as *const u8
                as *const libc::c_char,
            group: GRID_XATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"selinux\0" as *const u8 as *const libc::c_char,
            key: SELINUX_CONTEXT_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Enable the SELinux context support\0" as *const u8
                as *const libc::c_char,
            group: GRID_XATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-selinux\0" as *const u8 as *const libc::c_char,
            key: NO_SELINUX_CONTEXT_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Disable the SELinux context support\0" as *const u8
                as *const libc::c_char,
            group: GRID_XATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"acls\0" as *const u8 as *const libc::c_char,
            key: ACLS_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Enable the POSIX ACLs support\0" as *const u8 as *const libc::c_char,
            group: GRID_XATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-acls\0" as *const u8 as *const libc::c_char,
            key: NO_ACLS_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Disable the POSIX ACLs support\0" as *const u8 as *const libc::c_char,
            group: GRID_XATTR as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Device selection and switching:\0" as *const u8
                as *const libc::c_char,
            group: GRH_DEVICE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"file\0" as *const u8 as *const libc::c_char,
            key: 'f' as i32,
            arg: b"ARCHIVE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"use archive file or device ARCHIVE\0" as *const u8
                as *const libc::c_char,
            group: GRID_DEVICE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: '0' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_DEVICE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: '1' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_DEVICE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: '2' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_DEVICE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: '3' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_DEVICE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: '4' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_DEVICE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: '5' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_DEVICE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: '6' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_DEVICE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: '7' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_DEVICE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: '8' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_DEVICE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: '9' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_DEVICE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"force-local\0" as *const u8 as *const libc::c_char,
            key: FORCE_LOCAL_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"archive file is local even if it has a colon\0" as *const u8
                as *const libc::c_char,
            group: GRID_DEVICE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"rmt-command\0" as *const u8 as *const libc::c_char,
            key: RMT_COMMAND_OPTION as libc::c_int,
            arg: b"COMMAND\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"use given rmt COMMAND instead of rmt\0" as *const u8
                as *const libc::c_char,
            group: GRID_DEVICE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"rsh-command\0" as *const u8 as *const libc::c_char,
            key: RSH_COMMAND_OPTION as libc::c_int,
            arg: b"COMMAND\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"use remote COMMAND instead of rsh\0" as *const u8
                as *const libc::c_char,
            group: GRID_DEVICE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"multi-volume\0" as *const u8 as *const libc::c_char,
            key: 'M' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"create/list/extract multi-volume archive\0" as *const u8
                as *const libc::c_char,
            group: GRID_DEVICE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"tape-length\0" as *const u8 as *const libc::c_char,
            key: 'L' as i32,
            arg: b"NUMBER\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"change tape after writing NUMBER x 1024 bytes\0" as *const u8
                as *const libc::c_char,
            group: GRID_DEVICE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"info-script\0" as *const u8 as *const libc::c_char,
            key: 'F' as i32,
            arg: b"NAME\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"run script at end of each tape (implies -M)\0" as *const u8
                as *const libc::c_char,
            group: GRID_DEVICE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"new-volume-script\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x4 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_DEVICE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"volno-file\0" as *const u8 as *const libc::c_char,
            key: VOLNO_FILE_OPTION as libc::c_int,
            arg: b"FILE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"use/update the volume number in FILE\0" as *const u8
                as *const libc::c_char,
            group: GRID_DEVICE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Device blocking:\0" as *const u8 as *const libc::c_char,
            group: GRH_BLOCKING as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"blocking-factor\0" as *const u8 as *const libc::c_char,
            key: 'b' as i32,
            arg: b"BLOCKS\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"BLOCKS x 512 bytes per record\0" as *const u8 as *const libc::c_char,
            group: GRID_BLOCKING as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"record-size\0" as *const u8 as *const libc::c_char,
            key: RECORD_SIZE_OPTION as libc::c_int,
            arg: b"NUMBER\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"NUMBER of bytes per record, multiple of 512\0" as *const u8
                as *const libc::c_char,
            group: GRID_BLOCKING as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"ignore-zeros\0" as *const u8 as *const libc::c_char,
            key: 'i' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"ignore zeroed blocks in archive (means EOF)\0" as *const u8
                as *const libc::c_char,
            group: GRID_BLOCKING as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"read-full-records\0" as *const u8 as *const libc::c_char,
            key: 'B' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"reblock as we read (for 4.2BSD pipes)\0" as *const u8
                as *const libc::c_char,
            group: GRID_BLOCKING as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Archive format selection:\0" as *const u8 as *const libc::c_char,
            group: GRH_FORMAT as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"format\0" as *const u8 as *const libc::c_char,
            key: 'H' as i32,
            arg: b"FORMAT\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"create archive of the given format\0" as *const u8
                as *const libc::c_char,
            group: GRID_FORMAT as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"FORMAT is one of the following:\0" as *const u8
                as *const libc::c_char,
            group: GRDOC_FORMAT as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"  v7\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x8 as libc::c_int | 0x20 as libc::c_int,
            doc: b"old V7 tar format\0" as *const u8 as *const libc::c_char,
            group: GRDOC_FORMAT as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"  oldgnu\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x8 as libc::c_int | 0x20 as libc::c_int,
            doc: b"GNU format as per tar <= 1.12\0" as *const u8 as *const libc::c_char,
            group: GRDOC_FORMAT as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"  gnu\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x8 as libc::c_int | 0x20 as libc::c_int,
            doc: b"GNU tar 1.13.x format\0" as *const u8 as *const libc::c_char,
            group: GRDOC_FORMAT as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"  ustar\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x8 as libc::c_int | 0x20 as libc::c_int,
            doc: b"POSIX 1003.1-1988 (ustar) format\0" as *const u8
                as *const libc::c_char,
            group: GRDOC_FORMAT as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"  pax\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x8 as libc::c_int | 0x20 as libc::c_int,
            doc: b"POSIX 1003.1-2001 (pax) format\0" as *const u8 as *const libc::c_char,
            group: GRDOC_FORMAT as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"  posix\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x8 as libc::c_int | 0x20 as libc::c_int,
            doc: b"same as pax\0" as *const u8 as *const libc::c_char,
            group: GRDOC_FORMAT as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"old-archive\0" as *const u8 as *const libc::c_char,
            key: OLD_ARCHIVE_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"same as --format=v7\0" as *const u8 as *const libc::c_char,
            group: GRID_FORMAT_OPT as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"portability\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x4 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_FORMAT_OPT as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"posix\0" as *const u8 as *const libc::c_char,
            key: POSIX_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"same as --format=posix\0" as *const u8 as *const libc::c_char,
            group: GRID_FORMAT_OPT as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"pax-option\0" as *const u8 as *const libc::c_char,
            key: PAX_OPTION as libc::c_int,
            arg: b"keyword[[:]=value][,keyword[[:]=value]]...\0" as *const u8
                as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"control pax keywords\0" as *const u8 as *const libc::c_char,
            group: GRID_FORMAT_OPT as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"label\0" as *const u8 as *const libc::c_char,
            key: 'V' as i32,
            arg: b"TEXT\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"create archive with volume name TEXT; at list/extract time, use TEXT as a globbing pattern for volume name\0"
                as *const u8 as *const libc::c_char,
            group: GRID_FORMAT_OPT as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Compression options:\0" as *const u8 as *const libc::c_char,
            group: GRH_COMPRESS as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"auto-compress\0" as *const u8 as *const libc::c_char,
            key: 'a' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"use archive suffix to determine the compression program\0"
                as *const u8 as *const libc::c_char,
            group: GRID_COMPRESS as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-auto-compress\0" as *const u8 as *const libc::c_char,
            key: NO_AUTO_COMPRESS_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"do not use archive suffix to determine the compression program\0"
                as *const u8 as *const libc::c_char,
            group: GRID_COMPRESS as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"use-compress-program\0" as *const u8 as *const libc::c_char,
            key: 'I' as i32,
            arg: b"PROG\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"filter through PROG (must accept -d)\0" as *const u8
                as *const libc::c_char,
            group: GRID_COMPRESS as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"bzip2\0" as *const u8 as *const libc::c_char,
            key: 'j' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_COMPRESS as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"gzip\0" as *const u8 as *const libc::c_char,
            key: 'z' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_COMPRESS as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"gunzip\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x4 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_COMPRESS as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"ungzip\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x4 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_COMPRESS as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"compress\0" as *const u8 as *const libc::c_char,
            key: 'Z' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_COMPRESS as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"uncompress\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x4 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_COMPRESS as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"lzip\0" as *const u8 as *const libc::c_char,
            key: LZIP_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_COMPRESS as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"lzma\0" as *const u8 as *const libc::c_char,
            key: LZMA_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_COMPRESS as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"lzop\0" as *const u8 as *const libc::c_char,
            key: LZOP_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_COMPRESS as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"xz\0" as *const u8 as *const libc::c_char,
            key: 'J' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_COMPRESS as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"zstd\0" as *const u8 as *const libc::c_char,
            key: ZSTD_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_COMPRESS as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Local file selection:\0" as *const u8 as *const libc::c_char,
            group: GRH_FILE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"one-file-system\0" as *const u8 as *const libc::c_char,
            key: ONE_FILE_SYSTEM_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"stay in local file system when creating archive\0" as *const u8
                as *const libc::c_char,
            group: GRID_FILE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"absolute-names\0" as *const u8 as *const libc::c_char,
            key: 'P' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"don't strip leading '/'s from file names\0" as *const u8
                as *const libc::c_char,
            group: GRID_FILE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"dereference\0" as *const u8 as *const libc::c_char,
            key: 'h' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"follow symlinks; archive and dump the files they point to\0"
                as *const u8 as *const libc::c_char,
            group: GRID_FILE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"hard-dereference\0" as *const u8 as *const libc::c_char,
            key: HARD_DEREFERENCE_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"follow hard links; archive and dump the files they refer to\0"
                as *const u8 as *const libc::c_char,
            group: GRID_FILE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"starting-file\0" as *const u8 as *const libc::c_char,
            key: 'K' as i32,
            arg: b"MEMBER-NAME\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"begin at member MEMBER-NAME when reading the archive\0" as *const u8
                as *const libc::c_char,
            group: GRID_FILE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"newer\0" as *const u8 as *const libc::c_char,
            key: 'N' as i32,
            arg: b"DATE-OR-FILE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"only store files newer than DATE-OR-FILE\0" as *const u8
                as *const libc::c_char,
            group: GRID_FILE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"after-date\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x4 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_FILE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"newer-mtime\0" as *const u8 as *const libc::c_char,
            key: NEWER_MTIME_OPTION as libc::c_int,
            arg: b"DATE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"compare date and time when data changed only\0" as *const u8
                as *const libc::c_char,
            group: GRID_FILE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"backup\0" as *const u8 as *const libc::c_char,
            key: BACKUP_OPTION as libc::c_int,
            arg: b"CONTROL\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
            doc: b"backup before removal, choose version CONTROL\0" as *const u8
                as *const libc::c_char,
            group: GRID_FILE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"suffix\0" as *const u8 as *const libc::c_char,
            key: SUFFIX_OPTION as libc::c_int,
            arg: b"STRING\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"backup before removal, override usual suffix ('~' unless overridden by environment variable SIMPLE_BACKUP_SUFFIX)\0"
                as *const u8 as *const libc::c_char,
            group: GRID_FILE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"File name transformations:\0" as *const u8 as *const libc::c_char,
            group: GRH_NAME_XFORM as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"strip-components\0" as *const u8 as *const libc::c_char,
            key: STRIP_COMPONENTS_OPTION as libc::c_int,
            arg: b"NUMBER\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"strip NUMBER leading components from file names on extraction\0"
                as *const u8 as *const libc::c_char,
            group: GRID_NAME_XFORM as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"transform\0" as *const u8 as *const libc::c_char,
            key: TRANSFORM_OPTION as libc::c_int,
            arg: b"EXPRESSION\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"use sed replace EXPRESSION to transform file names\0" as *const u8
                as *const libc::c_char,
            group: GRID_NAME_XFORM as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"xform\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x4 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_NAME_XFORM as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Informative output:\0" as *const u8 as *const libc::c_char,
            group: GRH_INFORMATIVE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"checkpoint\0" as *const u8 as *const libc::c_char,
            key: CHECKPOINT_OPTION as libc::c_int,
            arg: b"NUMBER\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
            doc: b"display progress messages every NUMBERth record (default 10)\0"
                as *const u8 as *const libc::c_char,
            group: GRID_INFORMATIVE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"checkpoint-action\0" as *const u8 as *const libc::c_char,
            key: CHECKPOINT_ACTION_OPTION as libc::c_int,
            arg: b"ACTION\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"execute ACTION on each checkpoint\0" as *const u8
                as *const libc::c_char,
            group: GRID_INFORMATIVE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"check-links\0" as *const u8 as *const libc::c_char,
            key: 'l' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"print a message if not all links are dumped\0" as *const u8
                as *const libc::c_char,
            group: GRID_INFORMATIVE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"totals\0" as *const u8 as *const libc::c_char,
            key: TOTALS_OPTION as libc::c_int,
            arg: b"SIGNAL\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
            doc: b"print total bytes after processing the archive; with an argument - print total bytes when this SIGNAL is delivered; Allowed signals are: SIGHUP, SIGQUIT, SIGINT, SIGUSR1 and SIGUSR2; the names without SIG prefix are also accepted\0"
                as *const u8 as *const libc::c_char,
            group: GRID_INFORMATIVE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"utc\0" as *const u8 as *const libc::c_char,
            key: UTC_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"print file modification times in UTC\0" as *const u8
                as *const libc::c_char,
            group: GRID_INFORMATIVE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"full-time\0" as *const u8 as *const libc::c_char,
            key: FULL_TIME_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"print file time to its full resolution\0" as *const u8
                as *const libc::c_char,
            group: GRID_INFORMATIVE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"index-file\0" as *const u8 as *const libc::c_char,
            key: INDEX_FILE_OPTION as libc::c_int,
            arg: b"FILE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"send verbose output to FILE\0" as *const u8 as *const libc::c_char,
            group: GRID_INFORMATIVE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"block-number\0" as *const u8 as *const libc::c_char,
            key: 'R' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"show block number within archive with each message\0" as *const u8
                as *const libc::c_char,
            group: GRID_INFORMATIVE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"show-defaults\0" as *const u8 as *const libc::c_char,
            key: SHOW_DEFAULTS_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"show tar defaults\0" as *const u8 as *const libc::c_char,
            group: GRID_INFORMATIVE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"show-snapshot-field-ranges\0" as *const u8 as *const libc::c_char,
            key: SHOW_SNAPSHOT_FIELD_RANGES_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"show valid ranges for snapshot-file fields\0" as *const u8
                as *const libc::c_char,
            group: GRID_INFORMATIVE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"show-omitted-dirs\0" as *const u8 as *const libc::c_char,
            key: SHOW_OMITTED_DIRS_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"when listing or extracting, list each directory that does not match search criteria\0"
                as *const u8 as *const libc::c_char,
            group: GRID_INFORMATIVE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"show-transformed-names\0" as *const u8 as *const libc::c_char,
            key: SHOW_TRANSFORMED_NAMES_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"show file or archive names after transformation\0" as *const u8
                as *const libc::c_char,
            group: GRID_INFORMATIVE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"show-stored-names\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x4 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_INFORMATIVE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"quoting-style\0" as *const u8 as *const libc::c_char,
            key: QUOTING_STYLE_OPTION as libc::c_int,
            arg: b"STYLE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"set name quoting style; see below for valid STYLE values\0"
                as *const u8 as *const libc::c_char,
            group: GRID_INFORMATIVE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"quote-chars\0" as *const u8 as *const libc::c_char,
            key: QUOTE_CHARS_OPTION as libc::c_int,
            arg: b"STRING\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"additionally quote characters from STRING\0" as *const u8
                as *const libc::c_char,
            group: GRID_INFORMATIVE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-quote-chars\0" as *const u8 as *const libc::c_char,
            key: NO_QUOTE_CHARS_OPTION as libc::c_int,
            arg: b"STRING\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"disable quoting for characters from STRING\0" as *const u8
                as *const libc::c_char,
            group: GRID_INFORMATIVE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"interactive\0" as *const u8 as *const libc::c_char,
            key: 'w' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"ask for confirmation for every action\0" as *const u8
                as *const libc::c_char,
            group: GRID_INFORMATIVE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"confirmation\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x4 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: GRID_INFORMATIVE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            key: 'v' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"verbosely list files processed\0" as *const u8 as *const libc::c_char,
            group: GRID_INFORMATIVE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"warning\0" as *const u8 as *const libc::c_char,
            key: WARNING_OPTION as libc::c_int,
            arg: b"KEYWORD\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"warning control\0" as *const u8 as *const libc::c_char,
            group: GRID_INFORMATIVE as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Compatibility options:\0" as *const u8 as *const libc::c_char,
            group: GRH_COMPAT as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 'o' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"when creating, same as --old-archive; when extracting, same as --no-same-owner\0"
                as *const u8 as *const libc::c_char,
            group: GRID_COMPAT as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Other options:\0" as *const u8 as *const libc::c_char,
            group: GRH_OTHER as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"restrict\0" as *const u8 as *const libc::c_char,
            key: RESTRICT_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"disable use of some potentially harmful options\0" as *const u8
                as *const libc::c_char,
            group: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: 0 as libc::c_int,
        };
        init
    },
];
static mut atime_preserve_args: [*const libc::c_char; 3] = [
    b"replace\0" as *const u8 as *const libc::c_char,
    b"system\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut atime_preserve_types: [atime_preserve; 2] = [
    replace_atime_preserve,
    system_atime_preserve,
];
unsafe extern "C" fn format_default_settings() -> *mut libc::c_char {
    return xasprintf(
        b"--format=%s -f%s -b%d --quoting-style=%s --rmt-command=%s --rsh-command=%s\0"
            as *const u8 as *const libc::c_char,
        archive_format_string(GNU_FORMAT),
        b"-\0" as *const u8 as *const libc::c_char,
        20 as libc::c_int,
        *quoting_style_args
            .as_ptr()
            .offset(escape_quoting_style as libc::c_int as isize),
        b"/usr/local/libexec/rmt\0" as *const u8 as *const libc::c_char,
        b"/usr/bin/rsh\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn option_conflict_error(
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
) {
    if error_hook.is_some() {
        error_hook.unwrap()();
    }
    error(
        0 as libc::c_int,
        0 as libc::c_int,
        dcgettext(
            0 as *const libc::c_char,
            b"'%s' cannot be used with '%s'\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        a,
        b,
    );
    usage(2 as libc::c_int);
}
static mut option_class: [*mut option_locus; 10] = [0 as *const option_locus
    as *mut option_locus; 10];
unsafe extern "C" fn optloc_save(
    mut id: libc::c_uint,
    mut loc: *mut option_locus,
) -> *mut option_locus {
    let mut optloc: *mut option_locus = 0 as *mut option_locus;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: size_t = 0;
    if id as libc::c_ulong
        >= (::std::mem::size_of::<[*mut option_locus; 10]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut option_locus>() as libc::c_ulong)
    {
        abort();
    }
    s = ::std::mem::size_of::<option_locus>() as libc::c_ulong;
    if !((*loc).name).is_null() {
        s = (s as libc::c_ulong)
            .wrapping_add(
                (strlen((*loc).name)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
    }
    optloc = xmalloc(s) as *mut option_locus;
    if !((*loc).name).is_null() {
        p = (optloc as *mut libc::c_char)
            .offset(::std::mem::size_of::<option_locus>() as libc::c_ulong as isize);
        strcpy(p, (*loc).name);
        (*optloc).name = p;
    } else {
        (*optloc).name = 0 as *const libc::c_char;
    }
    (*optloc).source = (*loc).source;
    (*optloc).line = (*loc).line;
    (*optloc).prev = option_class[id as usize];
    option_class[id as usize] = optloc;
    return (*optloc).prev;
}
unsafe extern "C" fn optloc_lookup(mut id: libc::c_int) -> *mut option_locus {
    return option_class[id as usize];
}
unsafe extern "C" fn option_set_in_cl(mut id: libc::c_int) -> libc::c_int {
    let mut loc: *mut option_locus = optloc_lookup(id);
    if loc.is_null() {
        return 0 as libc::c_int;
    }
    return ((*loc).source as libc::c_uint
        == OPTS_COMMAND_LINE as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn optloc_eq(
    mut a: *mut option_locus,
    mut b: *mut option_locus,
) -> libc::c_int {
    if (*a).source as libc::c_uint != (*b).source as libc::c_uint {
        return 0 as libc::c_int;
    }
    if (*a).source as libc::c_uint == OPTS_COMMAND_LINE as libc::c_int as libc::c_uint {
        return 1 as libc::c_int;
    }
    return (strcmp((*a).name, (*b).name) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn set_subcommand_option(mut subcommand: subcommand) {
    if subcommand_option as libc::c_uint
        != UNKNOWN_SUBCOMMAND as libc::c_int as libc::c_uint
        && subcommand_option as libc::c_uint != subcommand as libc::c_uint
    {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"You may not specify more than one '-Acdtrux', '--delete' or  '--test-label' option\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(2 as libc::c_int);
    }
    subcommand_option = subcommand;
}
unsafe extern "C" fn set_use_compress_program_option(
    mut string: *const libc::c_char,
    mut loc: *mut option_locus,
) {
    let mut p: *mut option_locus = optloc_save(
        OC_COMPRESS as libc::c_int as libc::c_uint,
        loc,
    );
    if !use_compress_program_option.is_null()
        && strcmp(use_compress_program_option, string) != 0 as libc::c_int
        && (*p).source as libc::c_uint
            == OPTS_COMMAND_LINE as libc::c_int as libc::c_uint
    {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Conflicting compression options\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(2 as libc::c_int);
    }
    use_compress_program_option = string;
}
unsafe extern "C" fn sigstat(mut signo: libc::c_int) {
    compute_duration();
    print_total_stats();
    signal(signo, Some(sigstat as unsafe extern "C" fn(libc::c_int) -> ()));
}
unsafe extern "C" fn stat_on_signal(mut signo: libc::c_int) {
    signal(signo, Some(sigstat as unsafe extern "C" fn(libc::c_int) -> ()));
}
pub unsafe extern "C" fn decode_signal(mut name: *const libc::c_char) -> libc::c_int {
    static mut sigtab_0: [sigtab; 5] = [
        {
            let mut init = sigtab {
                name: b"USR1\0" as *const u8 as *const libc::c_char,
                signo: 10 as libc::c_int,
            };
            init
        },
        {
            let mut init = sigtab {
                name: b"USR2\0" as *const u8 as *const libc::c_char,
                signo: 12 as libc::c_int,
            };
            init
        },
        {
            let mut init = sigtab {
                name: b"HUP\0" as *const u8 as *const libc::c_char,
                signo: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = sigtab {
                name: b"INT\0" as *const u8 as *const libc::c_char,
                signo: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = sigtab {
                name: b"QUIT\0" as *const u8 as *const libc::c_char,
                signo: 3 as libc::c_int,
            };
            init
        },
    ];
    let mut p: *const sigtab = 0 as *const sigtab;
    let mut s: *const libc::c_char = name;
    if strncmp(
        s,
        b"SIG\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        s = s.offset(3 as libc::c_int as isize);
    }
    p = sigtab_0.as_ptr();
    while p
        < sigtab_0
            .as_ptr()
            .offset(
                (::std::mem::size_of::<[sigtab; 5]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<sigtab>() as libc::c_ulong)
                    as isize,
            )
    {
        if strcmp((*p).name, s) == 0 as libc::c_int {
            return (*p).signo;
        }
        p = p.offset(1);
        p;
    }
    if error_hook.is_some() {
        error_hook.unwrap()();
    }
    error(
        0 as libc::c_int,
        0 as libc::c_int,
        dcgettext(
            0 as *const libc::c_char,
            b"Unknown signal name: %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        name,
    );
    fatal_exit();
}
unsafe extern "C" fn set_stat_signal(mut name: *const libc::c_char) {
    stat_on_signal(decode_signal(name));
}
unsafe extern "C" fn get_date_or_file(
    mut args: *mut tar_args,
    mut rpl_option: *const libc::c_char,
    mut str: *const libc::c_char,
    mut ts: *mut timespec,
) -> libc::c_int {
    if 0 as libc::c_int != 0 as libc::c_int || *str as libc::c_int == '/' as i32
        || *str as libc::c_int == '.' as i32
    {
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
        if stat(str, &mut st) != 0 as libc::c_int {
            stat_error(str);
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Date sample file not found\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(2 as libc::c_int);
        }
        *ts = get_stat_mtime(&mut st);
    } else if !parse_datetime(ts, str, 0 as *const timespec) {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Substituting %s for unknown date format %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            tartime(*ts, 0 as libc::c_int != 0),
            quote(str),
        );
        (*ts).tv_nsec = 0 as libc::c_int as __syscall_slong_t;
        return 1 as libc::c_int;
    } else {
        let mut p: *mut textual_date = xmalloc(
            ::std::mem::size_of::<textual_date>() as libc::c_ulong,
        ) as *mut textual_date;
        (*p).ts = *ts;
        (*p).rpl_option = rpl_option;
        (*p).date = xstrdup(str);
        (*p).next = (*args).textual_date;
        (*args).textual_date = p;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn report_textual_dates(mut args: *mut tar_args) {
    let mut p: *mut textual_date = 0 as *mut textual_date;
    p = (*args).textual_date;
    while !p.is_null() {
        let mut next: *mut textual_date = (*p).next;
        if verbose_option != 0 {
            let mut treated_as: *const libc::c_char = tartime(
                (*p).ts,
                1 as libc::c_int != 0,
            );
            if strcmp((*p).date, treated_as) != 0 as libc::c_int {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Option %s: Treating date '%s' as %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*p).rpl_option,
                    (*p).date,
                    treated_as,
                );
            }
        }
        rpl_free((*p).date as *mut libc::c_void);
        rpl_free(p as *mut libc::c_void);
        p = next;
    }
}
unsafe extern "C" fn tar_help_filter(
    mut key: libc::c_int,
    mut text: *const libc::c_char,
    mut input: *mut libc::c_void,
) -> *mut libc::c_char {
    let mut stk: obstack = obstack {
        chunk_size: 0,
        chunk: 0 as *mut _obstack_chunk,
        object_base: 0 as *mut libc::c_char,
        next_free: 0 as *mut libc::c_char,
        chunk_limit: 0 as *mut libc::c_char,
        temp: C2RustUnnamed_2 { i: 0 },
        alignment_mask: 0,
        chunkfun: C2RustUnnamed_1 { plain: None },
        freefun: C2RustUnnamed_0 { plain: None },
        extra_arg: 0 as *mut libc::c_void,
        use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    match key {
        106 => {
            s = xasprintf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"filter the archive through %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"bzip2\0" as *const u8 as *const libc::c_char,
            );
        }
        122 => {
            s = xasprintf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"filter the archive through %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"gzip\0" as *const u8 as *const libc::c_char,
            );
        }
        90 => {
            s = xasprintf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"filter the archive through %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"compress\0" as *const u8 as *const libc::c_char,
            );
        }
        148 => {
            s = xasprintf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"filter the archive through %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"lzip\0" as *const u8 as *const libc::c_char,
            );
        }
        149 => {
            s = xasprintf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"filter the archive through %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"lzma\0" as *const u8 as *const libc::c_char,
            );
        }
        150 => {
            s = xasprintf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"filter the archive through %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"lzop\0" as *const u8 as *const libc::c_char,
            );
        }
        74 => {
            s = xasprintf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"filter the archive through %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"xz\0" as *const u8 as *const libc::c_char,
            );
        }
        207 => {
            s = xasprintf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"filter the archive through %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"zstd\0" as *const u8 as *const libc::c_char,
            );
        }
        33554436 => {
            let mut tstr: *const libc::c_char = 0 as *const libc::c_char;
            _obstack_begin(
                &mut stk,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
                Some(xmalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
                Some(rpl_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            );
            tstr = dcgettext(
                0 as *const libc::c_char,
                b"Valid arguments for the --quoting-style option are:\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            );
            let mut __o: *mut obstack = &mut stk;
            let mut __len: size_t = strlen(tstr);
            if ({
                let mut __o1: *const obstack = __o;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < __len
            {
                _obstack_newchunk(__o, __len);
            }
            memcpy(
                (*__o).next_free as *mut libc::c_void,
                tstr as *const libc::c_void,
                __len,
            );
            (*__o).next_free = ((*__o).next_free).offset(__len as isize);
            let mut __o_0: *mut obstack = &mut stk;
            let mut __len_0: size_t = 2 as libc::c_int as size_t;
            if ({
                let mut __o1: *const obstack = __o_0;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < __len_0
            {
                _obstack_newchunk(__o_0, __len_0);
            }
            memcpy(
                (*__o_0).next_free as *mut libc::c_void,
                b"\n\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                __len_0,
            );
            (*__o_0).next_free = ((*__o_0).next_free).offset(__len_0 as isize);
            tar_list_quoting_styles(
                &mut stk,
                b"  \0" as *const u8 as *const libc::c_char,
            );
            tstr = dcgettext(
                0 as *const libc::c_char,
                b"\n*This* tar defaults to:\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            let mut __o_1: *mut obstack = &mut stk;
            let mut __len_1: size_t = strlen(tstr);
            if ({
                let mut __o1: *const obstack = __o_1;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < __len_1
            {
                _obstack_newchunk(__o_1, __len_1);
            }
            memcpy(
                (*__o_1).next_free as *mut libc::c_void,
                tstr as *const libc::c_void,
                __len_1,
            );
            (*__o_1).next_free = ((*__o_1).next_free).offset(__len_1 as isize);
            s = format_default_settings();
            let mut __o_2: *mut obstack = &mut stk;
            let mut __len_2: size_t = strlen(s);
            if ({
                let mut __o1: *const obstack = __o_2;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < __len_2
            {
                _obstack_newchunk(__o_2, __len_2);
            }
            memcpy(
                (*__o_2).next_free as *mut libc::c_void,
                s as *const libc::c_void,
                __len_2,
            );
            (*__o_2).next_free = ((*__o_2).next_free).offset(__len_2 as isize);
            let mut __o_3: *mut obstack = &mut stk;
            if ({
                let mut __o1: *const obstack = __o_3;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < 1 as libc::c_int as libc::c_ulong
            {
                _obstack_newchunk(__o_3, 1 as libc::c_int as size_t);
            }
            let fresh2 = (*__o_3).next_free;
            (*__o_3).next_free = ((*__o_3).next_free).offset(1);
            *fresh2 = '\n' as i32 as libc::c_char;
            let mut __o_4: *mut obstack = &mut stk;
            if ({
                let mut __o1: *const obstack = __o_4;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < 1 as libc::c_int as libc::c_ulong
            {
                _obstack_newchunk(__o_4, 1 as libc::c_int as size_t);
            }
            let fresh3 = (*__o_4).next_free;
            (*__o_4).next_free = ((*__o_4).next_free).offset(1);
            *fresh3 = 0 as libc::c_int as libc::c_char;
            s = xstrdup(
                ({
                    let mut __o1: *mut obstack = &mut stk as *mut obstack;
                    let mut __value: *mut libc::c_void = (*__o1).object_base
                        as *mut libc::c_void;
                    if (*__o1).next_free == __value as *mut libc::c_char {
                        (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
                    }
                    (*__o1)
                        .next_free = (if (::std::mem::size_of::<ptrdiff_t>()
                        as libc::c_ulong)
                        < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    {
                        (*__o1).object_base
                    } else {
                        0 as *mut libc::c_char
                    })
                        .offset(
                            ((((*__o1).next_free)
                                .offset_from(
                                    if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                        < ::std::mem::size_of::<*mut libc::c_void>()
                                            as libc::c_ulong
                                    {
                                        (*__o1).object_base
                                    } else {
                                        0 as *mut libc::c_char
                                    },
                                ) as libc::c_long as libc::c_ulong)
                                .wrapping_add((*__o1).alignment_mask)
                                & !(*__o1).alignment_mask) as isize,
                        );
                    if ((*__o1).next_free)
                        .offset_from((*__o1).chunk as *mut libc::c_char) as libc::c_long
                        as size_t
                        > ((*__o1).chunk_limit)
                            .offset_from((*__o1).chunk as *mut libc::c_char)
                            as libc::c_long as size_t
                    {
                        (*__o1).next_free = (*__o1).chunk_limit;
                    }
                    (*__o1).object_base = (*__o1).next_free;
                    __value
                }) as *const libc::c_char,
            );
            let mut __o_5: *mut obstack = &mut stk;
            let mut __obj: *mut libc::c_void = 0 as *mut libc::c_void;
            if __obj > (*__o_5).chunk as *mut libc::c_void
                && __obj < (*__o_5).chunk_limit as *mut libc::c_void
            {
                (*__o_5).object_base = __obj as *mut libc::c_char;
                (*__o_5).next_free = (*__o_5).object_base;
            } else {
                _obstack_free(__o_5, __obj);
            }
        }
        _ => {
            s = text as *mut libc::c_char;
        }
    }
    return s;
}
unsafe extern "C" fn expand_pax_option(
    mut targs: *mut tar_args,
    mut arg: *const libc::c_char,
) -> *mut libc::c_char {
    let mut stk: obstack = obstack {
        chunk_size: 0,
        chunk: 0 as *mut _obstack_chunk,
        object_base: 0 as *mut libc::c_char,
        next_free: 0 as *mut libc::c_char,
        chunk_limit: 0 as *mut libc::c_char,
        temp: C2RustUnnamed_2 { i: 0 },
        alignment_mask: 0,
        chunkfun: C2RustUnnamed_1 { plain: None },
        freefun: C2RustUnnamed_0 { plain: None },
        extra_arg: 0 as *mut libc::c_void,
        use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    _obstack_begin(
        &mut stk,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        Some(xmalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        Some(rpl_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    while *arg != 0 {
        let mut seglen: size_t = strcspn(
            arg,
            b",\0" as *const u8 as *const libc::c_char,
        );
        let mut p: *mut libc::c_char = memchr(
            arg as *const libc::c_void,
            '=' as i32,
            seglen,
        ) as *mut libc::c_char;
        if !p.is_null() {
            let mut len: size_t = (p.offset_from(arg) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as size_t;
            let mut __o: *mut obstack = &mut stk;
            let mut __len: size_t = len;
            if ({
                let mut __o1: *const obstack = __o;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < __len
            {
                _obstack_newchunk(__o, __len);
            }
            memcpy(
                (*__o).next_free as *mut libc::c_void,
                arg as *const libc::c_void,
                __len,
            );
            (*__o).next_free = ((*__o).next_free).offset(__len as isize);
            len = seglen.wrapping_sub(len);
            p = p.offset(1);
            p;
            while *p as libc::c_int != 0
                && *(*__ctype_b_loc())
                    .offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                len = len.wrapping_sub(1);
                len;
                p = p.offset(1);
                p;
            }
            if *p as libc::c_int == '{' as i32
                && *p
                    .offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int == '}' as i32
            {
                let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
                let mut tmp: *mut libc::c_char = xmalloc(len) as *mut libc::c_char;
                memcpy(
                    tmp as *mut libc::c_void,
                    p.offset(1 as libc::c_int as isize) as *const libc::c_void,
                    len.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                );
                *tmp
                    .offset(
                        len.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
                    ) = 0 as libc::c_int as libc::c_char;
                if get_date_or_file(
                    targs,
                    b"--pax-option\0" as *const u8 as *const libc::c_char,
                    tmp,
                    &mut ts,
                ) == 0 as libc::c_int
                {
                    let mut buf: [libc::c_char; 32] = [0; 32];
                    let mut s: *const libc::c_char = code_timespec(ts, buf.as_mut_ptr());
                    let mut __o_0: *mut obstack = &mut stk;
                    let mut __len_0: size_t = strlen(s);
                    if ({
                        let mut __o1: *const obstack = __o_0;
                        ((*__o1).chunk_limit).offset_from((*__o1).next_free)
                            as libc::c_long as size_t
                    }) < __len_0
                    {
                        _obstack_newchunk(__o_0, __len_0);
                    }
                    memcpy(
                        (*__o_0).next_free as *mut libc::c_void,
                        s as *const libc::c_void,
                        __len_0,
                    );
                    (*__o_0).next_free = ((*__o_0).next_free).offset(__len_0 as isize);
                } else {
                    let mut __o_1: *mut obstack = &mut stk;
                    let mut __len_1: size_t = len;
                    if ({
                        let mut __o1: *const obstack = __o_1;
                        ((*__o1).chunk_limit).offset_from((*__o1).next_free)
                            as libc::c_long as size_t
                    }) < __len_1
                    {
                        _obstack_newchunk(__o_1, __len_1);
                    }
                    memcpy(
                        (*__o_1).next_free as *mut libc::c_void,
                        p as *const libc::c_void,
                        __len_1,
                    );
                    (*__o_1).next_free = ((*__o_1).next_free).offset(__len_1 as isize);
                }
                rpl_free(tmp as *mut libc::c_void);
            } else {
                let mut __o_2: *mut obstack = &mut stk;
                let mut __len_2: size_t = len;
                if ({
                    let mut __o1: *const obstack = __o_2;
                    ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                        as size_t
                }) < __len_2
                {
                    _obstack_newchunk(__o_2, __len_2);
                }
                memcpy(
                    (*__o_2).next_free as *mut libc::c_void,
                    p as *const libc::c_void,
                    __len_2,
                );
                (*__o_2).next_free = ((*__o_2).next_free).offset(__len_2 as isize);
            }
        } else {
            let mut __o_3: *mut obstack = &mut stk;
            let mut __len_3: size_t = seglen;
            if ({
                let mut __o1: *const obstack = __o_3;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < __len_3
            {
                _obstack_newchunk(__o_3, __len_3);
            }
            memcpy(
                (*__o_3).next_free as *mut libc::c_void,
                arg as *const libc::c_void,
                __len_3,
            );
            (*__o_3).next_free = ((*__o_3).next_free).offset(__len_3 as isize);
        }
        arg = arg.offset(seglen as isize);
        if *arg != 0 {
            let mut __o_4: *mut obstack = &mut stk;
            if ({
                let mut __o1: *const obstack = __o_4;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < 1 as libc::c_int as libc::c_ulong
            {
                _obstack_newchunk(__o_4, 1 as libc::c_int as size_t);
            }
            let fresh4 = (*__o_4).next_free;
            (*__o_4).next_free = ((*__o_4).next_free).offset(1);
            *fresh4 = *arg;
            arg = arg.offset(1);
            arg;
        }
    }
    let mut __o_5: *mut obstack = &mut stk;
    if ({
        let mut __o1: *const obstack = __o_5;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < 1 as libc::c_int as libc::c_ulong
    {
        _obstack_newchunk(__o_5, 1 as libc::c_int as size_t);
    }
    let fresh5 = (*__o_5).next_free;
    (*__o_5).next_free = ((*__o_5).next_free).offset(1);
    *fresh5 = 0 as libc::c_int as libc::c_char;
    res = xstrdup(
        ({
            let mut __o1: *mut obstack = &mut stk as *mut obstack;
            let mut __value: *mut libc::c_void = (*__o1).object_base
                as *mut libc::c_void;
            if (*__o1).next_free == __value as *mut libc::c_char {
                (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
            }
            (*__o1)
                .next_free = (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            {
                (*__o1).object_base
            } else {
                0 as *mut libc::c_char
            })
                .offset(
                    ((((*__o1).next_free)
                        .offset_from(
                            if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                < ::std::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong
                            {
                                (*__o1).object_base
                            } else {
                                0 as *mut libc::c_char
                            },
                        ) as libc::c_long as libc::c_ulong)
                        .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                        as isize,
                );
            if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
                as libc::c_long as size_t
                > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut libc::c_char)
                    as libc::c_long as size_t
            {
                (*__o1).next_free = (*__o1).chunk_limit;
            }
            (*__o1).object_base = (*__o1).next_free;
            __value
        }) as *const libc::c_char,
    );
    let mut __o_6: *mut obstack = &mut stk;
    let mut __obj: *mut libc::c_void = 0 as *mut libc::c_void;
    if __obj > (*__o_6).chunk as *mut libc::c_void
        && __obj < (*__o_6).chunk_limit as *mut libc::c_void
    {
        (*__o_6).object_base = __obj as *mut libc::c_char;
        (*__o_6).next_free = (*__o_6).object_base;
    } else {
        _obstack_free(__o_6, __obj);
    }
    return res;
}
unsafe extern "C" fn parse_owner_group(
    mut arg: *mut libc::c_char,
    mut field_max: uintmax_t,
    mut name_option: *mut *const libc::c_char,
) -> uintmax_t {
    let mut u: uintmax_t = 18446744073709551615 as libc::c_ulong;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut invalid_num: *const libc::c_char = 0 as *const libc::c_char;
    let mut colon: *mut libc::c_char = strchr(arg, ':' as i32);
    if !colon.is_null() {
        let mut num: *const libc::c_char = colon.offset(1 as libc::c_int as isize);
        *colon = '\0' as i32 as libc::c_char;
        if *arg != 0 {
            name = arg;
        }
        if !num.is_null()
            && !(xstrtoumax(
                num,
                &mut end,
                10 as libc::c_int,
                &mut u,
                b"\0" as *const u8 as *const libc::c_char,
            ) as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint
                && u <= field_max)
        {
            invalid_num = num;
        }
    } else {
        let mut u1: uintmax_t = 0;
        let mut current_block_9: u64;
        match if '0' as i32 <= *arg as libc::c_int && *arg as libc::c_int <= '9' as i32 {
            xstrtoumax(
                arg,
                &mut end,
                10 as libc::c_int,
                &mut u1,
                b"\0" as *const u8 as *const libc::c_char,
            ) as libc::c_uint
        } else {
            LONGINT_INVALID as libc::c_int as libc::c_uint
        } {
            0 => {
                if u1 <= field_max {
                    u = u1;
                    current_block_9 = 10048703153582371463;
                } else {
                    current_block_9 = 1408419459218748143;
                }
            }
            1 => {
                current_block_9 = 1408419459218748143;
            }
            _ => {
                name = arg;
                current_block_9 = 10048703153582371463;
            }
        }
        match current_block_9 {
            1408419459218748143 => {
                invalid_num = arg;
            }
            _ => {}
        }
    }
    if !invalid_num.is_null() {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            quotearg_colon(invalid_num),
            dcgettext(
                0 as *const libc::c_char,
                b"Invalid owner or group ID\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fatal_exit();
    }
    if !name.is_null() {
        *name_option = name;
    }
    return u;
}
static mut sort_mode_arg: [*const libc::c_char; 4] = [
    b"none\0" as *const u8 as *const libc::c_char,
    b"name\0" as *const u8 as *const libc::c_char,
    b"inode\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut sort_mode_flag: [libc::c_int; 3] = [
    SAVEDIR_SORT_NONE as libc::c_int,
    SAVEDIR_SORT_NAME as libc::c_int,
    SAVEDIR_SORT_INODE as libc::c_int,
];
static mut hole_detection_args: [*const libc::c_char; 3] = [
    b"raw\0" as *const u8 as *const libc::c_char,
    b"seek\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut hole_detection_types: [libc::c_int; 2] = [
    HOLE_DETECTION_RAW as libc::c_int,
    HOLE_DETECTION_SEEK as libc::c_int,
];
unsafe extern "C" fn set_old_files_option(
    mut code: libc::c_int,
    mut loc: *mut option_locus,
) {
    let mut prev: *mut option_locus = 0 as *mut option_locus;
    static mut code_to_opt: [*const libc::c_char; 7] = [
        b"--overwrite-dir\0" as *const u8 as *const libc::c_char,
        b"--no-overwrite-dir\0" as *const u8 as *const libc::c_char,
        b"--overwrite\0" as *const u8 as *const libc::c_char,
        b"--unlink-first\0" as *const u8 as *const libc::c_char,
        b"--keep-old-files\0" as *const u8 as *const libc::c_char,
        b"--skip-old-files\0" as *const u8 as *const libc::c_char,
        b"--keep-newer-files\0" as *const u8 as *const libc::c_char,
    ];
    prev = optloc_save(OC_OLD_FILES as libc::c_int as libc::c_uint, loc);
    if !prev.is_null() && optloc_eq(loc, prev) != 0
        && code as libc::c_uint != old_files_option as libc::c_uint
    {
        option_conflict_error(
            code_to_opt[code as usize],
            code_to_opt[old_files_option as usize],
        );
    }
    old_files_option = code as old_files;
}
unsafe extern "C" fn parse_opt(
    mut key: libc::c_int,
    mut arg: *mut libc::c_char,
    mut state: *mut argp_state,
) -> error_t {
    let mut args: *mut tar_args = (*state).input as *mut tar_args;
    let mut current_block_308: u64;
    match key {
        16777219 => {
            if !((*(*state).root_argp).children).is_null() {
                let mut i: libc::c_int = 0;
                i = 0 as libc::c_int;
                while !((*((*(*state).root_argp).children).offset(i as isize)).argp)
                    .is_null()
                {
                    let ref mut fresh6 = *((*state).child_inputs).offset(i as isize);
                    *fresh6 = (*state).input;
                    i += 1;
                    i;
                }
            }
            current_block_308 = 9657238515557273331;
        }
        0 => {
            name_add_name(arg);
            current_block_308 = 9657238515557273331;
        }
        65 => {
            set_subcommand_option(CAT_SUBCOMMAND);
            current_block_308 = 9657238515557273331;
        }
        97 => {
            (*args).compress_autodetect = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        155 => {
            (*args).compress_autodetect = 0 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        98 => {
            let mut u: uintmax_t = 0;
            if !(xstrtoumax(
                arg,
                0 as *mut *mut libc::c_char,
                10 as libc::c_int,
                &mut u,
                b"\0" as *const u8 as *const libc::c_char,
            ) as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint
                && {
                    blocking_factor = u as libc::c_int;
                    u == blocking_factor as libc::c_ulong
                } && (0 as libc::c_int) < blocking_factor
                && {
                    record_size = u.wrapping_mul(512 as libc::c_int as libc::c_ulong);
                    u == record_size.wrapping_div(512 as libc::c_int as libc::c_ulong)
                })
            {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    quotearg_colon(arg),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Invalid blocking factor\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                usage(2 as libc::c_int);
            }
            current_block_308 = 9657238515557273331;
        }
        66 => {
            read_full_records_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        99 => {
            set_subcommand_option(CREATE_SUBCOMMAND);
            current_block_308 = 9657238515557273331;
        }
        134 => {
            set_mtime_option = CLAMP_MTIME;
            current_block_308 = 9657238515557273331;
        }
        100 => {
            set_subcommand_option(DIFF_SUBCOMMAND);
            current_block_308 = 9657238515557273331;
        }
        70 => {
            info_script_option = arg;
            multi_volume_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        102 => {
            if archive_names == allocated_archive_names {
                archive_name_array = x2nrealloc(
                    archive_name_array as *mut libc::c_void,
                    &mut allocated_archive_names,
                    ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                ) as *mut *const libc::c_char;
            }
            let fresh7 = archive_names;
            archive_names = archive_names.wrapping_add(1);
            let ref mut fresh8 = *archive_name_array.offset(fresh7 as isize);
            *fresh8 = arg;
            current_block_308 = 9657238515557273331;
        }
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
            argp_error(
                state,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Options '-[0-7][lmh]' not supported by *this* tar\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            exit(64 as libc::c_int);
        }
        139 => {
            full_time_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        103 => {
            optloc_save(
                OC_LISTED_INCREMENTAL as libc::c_int as libc::c_uint,
                (*args).loc,
            );
            listed_incremental_option = arg;
            after_date_option = 1 as libc::c_int;
            current_block_308 = 1154520408629132897;
        }
        71 => {
            current_block_308 = 1154520408629132897;
        }
        104 => {
            dereference_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        136 => {
            hard_dereference_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        105 => {
            ignore_zeros_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        106 => {
            set_use_compress_program_option(
                b"bzip2\0" as *const u8 as *const libc::c_char,
                (*args).loc,
            );
            current_block_308 = 9657238515557273331;
        }
        74 => {
            set_use_compress_program_option(
                b"xz\0" as *const u8 as *const libc::c_char,
                (*args).loc,
            );
            current_block_308 = 9657238515557273331;
        }
        107 => {
            set_old_files_option(KEEP_OLD_FILES as libc::c_int, (*args).loc);
            current_block_308 = 9657238515557273331;
        }
        75 => {
            optloc_save(OC_STARTING_FILE as libc::c_int as libc::c_uint, (*args).loc);
            add_starting_file(arg);
            current_block_308 = 9657238515557273331;
        }
        169 => {
            one_file_system_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        170 => {
            optloc_save(OC_ONE_TOP_LEVEL as libc::c_int as libc::c_uint, (*args).loc);
            one_top_level_option = 1 as libc::c_int != 0;
            one_top_level_dir = arg;
            current_block_308 = 9657238515557273331;
        }
        108 => {
            check_links_option = 1 as libc::c_int;
            current_block_308 = 9657238515557273331;
        }
        76 => {
            let mut u_0: uintmax_t = 0;
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            if xstrtoumax(
                arg,
                &mut p,
                10 as libc::c_int,
                &mut u_0,
                b"bBcGgkKMmPTtw\0" as *const u8 as *const libc::c_char,
            ) as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint
            {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    quotearg_colon(arg),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Invalid tape length\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                usage(2 as libc::c_int);
            }
            if p > arg
                && (strchr(
                    b"bBcGgkKMmPTtw\0" as *const u8 as *const libc::c_char,
                    *p.offset(-(1 as libc::c_int) as isize) as libc::c_int,
                ))
                    .is_null()
            {
                tape_length_option = 1024 as libc::c_int as libc::c_double
                    * u_0 as tarlong;
            } else {
                tape_length_option = u_0 as tarlong;
            }
            multi_volume_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        147 => {
            let mut p_0: *mut libc::c_char = 0 as *mut libc::c_char;
            incremental_level = strtoul(arg, &mut p_0, 10 as libc::c_int) as libc::c_int;
            if *p_0 != 0 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Invalid incremental level value\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                usage(2 as libc::c_int);
            }
            current_block_308 = 9657238515557273331;
        }
        148 => {
            set_use_compress_program_option(
                b"lzip\0" as *const u8 as *const libc::c_char,
                (*args).loc,
            );
            current_block_308 = 9657238515557273331;
        }
        149 => {
            set_use_compress_program_option(
                b"lzma\0" as *const u8 as *const libc::c_char,
                (*args).loc,
            );
            current_block_308 = 9657238515557273331;
        }
        150 => {
            set_use_compress_program_option(
                b"lzop\0" as *const u8 as *const libc::c_char,
                (*args).loc,
            );
            current_block_308 = 9657238515557273331;
        }
        109 => {
            touch_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        77 => {
            multi_volume_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        152 => {
            get_date_or_file(
                args,
                b"--mtime\0" as *const u8 as *const libc::c_char,
                arg,
                &mut mtime_option,
            );
            if set_mtime_option as libc::c_uint
                == USE_FILE_MTIME as libc::c_int as libc::c_uint
            {
                set_mtime_option = FORCE_MTIME;
            }
            current_block_308 = 9657238515557273331;
        }
        110 => {
            seek_option = 1 as libc::c_int;
            current_block_308 = 9657238515557273331;
        }
        163 => {
            seek_option = 0 as libc::c_int;
            current_block_308 = 9657238515557273331;
        }
        78 => {
            after_date_option = 1 as libc::c_int;
            current_block_308 = 12792210781472814257;
        }
        153 => {
            current_block_308 = 12792210781472814257;
        }
        111 => {
            (*args).o_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        79 => {
            to_stdout_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        112 => {
            same_permissions_option = 1 as libc::c_int;
            current_block_308 = 9657238515557273331;
        }
        80 => {
            optloc_save(OC_ABSOLUTE_NAMES as libc::c_int as libc::c_uint, (*args).loc);
            absolute_names_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        114 => {
            set_subcommand_option(APPEND_SUBCOMMAND);
            current_block_308 = 9657238515557273331;
        }
        82 => {
            block_number_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        115 => {
            optloc_save(OC_SAME_ORDER as libc::c_int as libc::c_uint, (*args).loc);
            same_order_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        83 => {
            sparse_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        191 => {
            set_old_files_option(SKIP_OLD_FILES as libc::c_int, (*args).loc);
            current_block_308 = 9657238515557273331;
        }
        193 => {
            hole_detection = hole_detection_types[__xargmatch_internal(
                b"--hole-detection\0" as *const u8 as *const libc::c_char,
                arg,
                hole_detection_args.as_ptr(),
                hole_detection_types.as_ptr() as *const libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                argmatch_die,
            ) as usize] as hole_detection_method;
            sparse_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        194 => {
            sparse_option = 1 as libc::c_int != 0;
            let mut p_1: *mut libc::c_char = 0 as *mut libc::c_char;
            tar_sparse_major = strtoul(arg, &mut p_1, 10 as libc::c_int) as libc::c_uint;
            if *p_1 != 0 {
                if *p_1 as libc::c_int != '.' as i32 {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Invalid sparse version value\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    usage(2 as libc::c_int);
                }
                tar_sparse_minor = strtoul(
                    p_1.offset(1 as libc::c_int as isize),
                    &mut p_1,
                    10 as libc::c_int,
                ) as libc::c_uint;
                if *p_1 != 0 {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Invalid sparse version value\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    usage(2 as libc::c_int);
                }
            }
            current_block_308 = 9657238515557273331;
        }
        116 => {
            set_subcommand_option(LIST_SUBCOMMAND);
            verbose_option += 1;
            verbose_option;
            current_block_308 = 9657238515557273331;
        }
        197 => {
            set_subcommand_option(TEST_LABEL_SUBCOMMAND);
            current_block_308 = 9657238515557273331;
        }
        200 => {
            set_transform_expr(arg);
            current_block_308 = 9657238515557273331;
        }
        117 => {
            set_subcommand_option(UPDATE_SUBCOMMAND);
            current_block_308 = 9657238515557273331;
        }
        85 => {
            set_old_files_option(UNLINK_FIRST_OLD_FILES as libc::c_int, (*args).loc);
            current_block_308 = 9657238515557273331;
        }
        201 => {
            utc_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        86 => {
            volume_label_option = arg;
            current_block_308 = 9657238515557273331;
        }
        118 => {
            verbose_option += 1;
            verbose_option;
            warning_option
                |= 0x2000 as libc::c_int | 0x1000 as libc::c_int | 0x80000 as libc::c_int
                    | 0x100000 as libc::c_int | 0x400000 as libc::c_int;
            current_block_308 = 9657238515557273331;
        }
        119 => {
            interactive_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        87 => {
            optloc_save(OC_VERIFY as libc::c_int as libc::c_uint, (*args).loc);
            verify_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        203 => {
            set_warning_option(arg);
            current_block_308 = 9657238515557273331;
        }
        120 => {
            set_subcommand_option(EXTRACT_SUBCOMMAND);
            current_block_308 = 9657238515557273331;
        }
        122 => {
            set_use_compress_program_option(
                b"gzip\0" as *const u8 as *const libc::c_char,
                (*args).loc,
            );
            current_block_308 = 9657238515557273331;
        }
        90 => {
            set_use_compress_program_option(
                b"compress\0" as *const u8 as *const libc::c_char,
                (*args).loc,
            );
            current_block_308 = 9657238515557273331;
        }
        207 => {
            set_use_compress_program_option(
                b"zstd\0" as *const u8 as *const libc::c_char,
                (*args).loc,
            );
            current_block_308 = 9657238515557273331;
        }
        129 => {
            atime_preserve_option = (if !arg.is_null() {
                atime_preserve_types[__xargmatch_internal(
                    b"--atime-preserve\0" as *const u8 as *const libc::c_char,
                    arg,
                    atime_preserve_args.as_ptr(),
                    atime_preserve_types.as_ptr() as *const libc::c_void,
                    ::std::mem::size_of::<atime_preserve>() as libc::c_ulong,
                    argmatch_die,
                ) as usize] as libc::c_uint
            } else {
                replace_atime_preserve as libc::c_int as libc::c_uint
            }) as atime_preserve;
            if 0o1000000 as libc::c_int == 0
                && atime_preserve_option as libc::c_uint
                    == system_atime_preserve as libc::c_int as libc::c_uint
            {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"--atime-preserve='system' is not supported on this platform\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                fatal_exit();
            }
            current_block_308 = 9657238515557273331;
        }
        131 => {
            check_device_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        156 => {
            check_device_option = 0 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        132 => {
            if !arg.is_null() {
                let mut p_2: *mut libc::c_char = 0 as *mut libc::c_char;
                if *arg as libc::c_int == '.' as i32 {
                    checkpoint_compile_action(
                        b".\0" as *const u8 as *const libc::c_char,
                    );
                    arg = arg.offset(1);
                    arg;
                }
                checkpoint_option = strtoul(arg, &mut p_2, 0 as libc::c_int)
                    as libc::c_uint;
                if *p_2 != 0 {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"--checkpoint value is not an integer\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    fatal_exit();
                }
            } else {
                checkpoint_option = 10 as libc::c_int as libc::c_uint;
            }
            current_block_308 = 9657238515557273331;
        }
        133 => {
            checkpoint_compile_action(arg);
            current_block_308 = 9657238515557273331;
        }
        130 => {
            backup_option = 1 as libc::c_int != 0;
            if !arg.is_null() {
                (*args).version_control_string = arg;
            }
            current_block_308 = 9657238515557273331;
        }
        135 => {
            delay_directory_restore_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        157 => {
            delay_directory_restore_option = 0 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        137 => {
            set_subcommand_option(DELETE_SUBCOMMAND);
            current_block_308 = 9657238515557273331;
        }
        138 => {
            force_local_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        72 => {
            set_archive_format(arg);
            current_block_308 = 9657238515557273331;
        }
        144 => {
            index_file_name = arg;
            current_block_308 = 9657238515557273331;
        }
        142 => {
            ignore_command_error_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        143 => {
            ignore_failed_read_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        145 => {
            keep_directory_symlink_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        146 => {
            set_old_files_option(KEEP_NEWER_FILES as libc::c_int, (*args).loc);
            current_block_308 = 9657238515557273331;
        }
        140 => {
            let mut u_1: uintmax_t = parse_owner_group(
                arg,
                (if (0 as libc::c_int as gid_t) < -(1 as libc::c_int) as gid_t {
                    -(1 as libc::c_int) as gid_t
                } else {
                    ((1 as libc::c_int as gid_t)
                        << (::std::mem::size_of::<gid_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        .wrapping_mul(2 as libc::c_int as libc::c_uint)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }) as uintmax_t,
                &mut group_name_option,
            );
            if u_1 == 18446744073709551615 as libc::c_ulong {
                group_option = -(1 as libc::c_int) as gid_t;
                if !group_name_option.is_null() {
                    gname_to_gid(group_name_option, &mut group_option);
                }
            } else {
                group_option = u_1 as gid_t;
            }
            current_block_308 = 9657238515557273331;
        }
        141 => {
            group_map_read(arg);
            current_block_308 = 9657238515557273331;
        }
        151 => {
            mode_option = mode_compile(arg);
            if mode_option.is_null() {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Invalid mode given on option\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                fatal_exit();
            }
            initial_umask = umask(0 as libc::c_int as __mode_t);
            umask(initial_umask);
            current_block_308 = 9657238515557273331;
        }
        158 => {
            ignore_command_error_option = 0 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        159 => {
            set_old_files_option(NO_OVERWRITE_DIR_OLD_FILES as libc::c_int, (*args).loc);
            current_block_308 = 9657238515557273331;
        }
        160 => {
            while *arg != 0 {
                set_char_quoting(0 as *mut quoting_options, *arg, 0 as libc::c_int);
                arg = arg.offset(1);
                arg;
            }
            current_block_308 = 9657238515557273331;
        }
        166 => {
            numeric_owner_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        167 => {
            optloc_save(OC_OCCURRENCE as libc::c_int as libc::c_uint, (*args).loc);
            if arg.is_null() {
                occurrence_option = 1 as libc::c_int as uintmax_t;
            } else {
                let mut u_2: uintmax_t = 0;
                if xstrtoumax(
                    arg,
                    0 as *mut *mut libc::c_char,
                    10 as libc::c_int,
                    &mut u_2,
                    b"\0" as *const u8 as *const libc::c_char,
                ) as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint
                {
                    occurrence_option = u_2;
                } else {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"%s: %s\0" as *const u8 as *const libc::c_char,
                        quotearg_colon(arg),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Invalid number\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    fatal_exit();
                }
            }
            current_block_308 = 9657238515557273331;
        }
        168 => {
            set_archive_format(b"v7\0" as *const u8 as *const libc::c_char);
            current_block_308 = 9657238515557273331;
        }
        171 => {
            set_old_files_option(DEFAULT_OLD_FILES as libc::c_int, (*args).loc);
            current_block_308 = 9657238515557273331;
        }
        172 => {
            set_old_files_option(OVERWRITE_OLD_FILES as libc::c_int, (*args).loc);
            current_block_308 = 9657238515557273331;
        }
        173 => {
            let mut u_3: uintmax_t = parse_owner_group(
                arg,
                (if (0 as libc::c_int as uid_t) < -(1 as libc::c_int) as uid_t {
                    -(1 as libc::c_int) as uid_t
                } else {
                    ((1 as libc::c_int as uid_t)
                        << (::std::mem::size_of::<uid_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        .wrapping_mul(2 as libc::c_int as libc::c_uint)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                }) as uintmax_t,
                &mut owner_name_option,
            );
            if u_3 == 18446744073709551615 as libc::c_ulong {
                owner_option = -(1 as libc::c_int) as uid_t;
                if !owner_name_option.is_null() {
                    uname_to_uid(owner_name_option, &mut owner_option);
                }
            } else {
                owner_option = u_3 as uid_t;
            }
            current_block_308 = 9657238515557273331;
        }
        174 => {
            owner_map_read(arg);
            current_block_308 = 9657238515557273331;
        }
        177 => {
            while *arg != 0 {
                set_char_quoting(0 as *mut quoting_options, *arg, 1 as libc::c_int);
                arg = arg.offset(1);
                arg;
            }
            current_block_308 = 9657238515557273331;
        }
        178 => {
            tar_set_quoting_style(arg);
            current_block_308 = 9657238515557273331;
        }
        175 => {
            let mut tmp: *mut libc::c_char = expand_pax_option(args, arg);
            (*args).pax_option = 1 as libc::c_int != 0;
            xheader_set_option(tmp);
            rpl_free(tmp as *mut libc::c_void);
            current_block_308 = 9657238515557273331;
        }
        176 => {
            set_archive_format(b"posix\0" as *const u8 as *const libc::c_char);
            current_block_308 = 9657238515557273331;
        }
        179 => {
            let mut u_4: uintmax_t = 0;
            if !(xstrtoumax(
                arg,
                0 as *mut *mut libc::c_char,
                10 as libc::c_int,
                &mut u_4,
                b"bBcGgkKMmPTtw\0" as *const u8 as *const libc::c_char,
            ) as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint && u_4 == u_4)
            {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    quotearg_colon(arg),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Invalid record size\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                usage(2 as libc::c_int);
            }
            record_size = u_4;
            if record_size.wrapping_rem(512 as libc::c_int as libc::c_ulong)
                != 0 as libc::c_int as libc::c_ulong
            {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Record size must be a multiple of %d.\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    512 as libc::c_int,
                );
                usage(2 as libc::c_int);
            }
            blocking_factor = record_size
                .wrapping_div(512 as libc::c_int as libc::c_ulong) as libc::c_int;
            current_block_308 = 9657238515557273331;
        }
        180 => {
            recursive_unlink_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        181 => {
            remove_files_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        182 => {
            restrict_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        183 => {
            rmt_command = arg;
            current_block_308 = 9657238515557273331;
        }
        184 => {
            rsh_command_option = arg;
            current_block_308 = 9657238515557273331;
        }
        187 => {
            let mut s: *mut libc::c_char = format_default_settings();
            printf(b"%s\n\0" as *const u8 as *const libc::c_char, s);
            close_stdout();
            rpl_free(s as *mut libc::c_void);
            exit(0 as libc::c_int);
        }
        189 => {
            show_snapshot_field_ranges();
            close_stdout();
            exit(0 as libc::c_int);
        }
        195 => {
            let mut u_5: uintmax_t = 0;
            if !(xstrtoumax(
                arg,
                0 as *mut *mut libc::c_char,
                10 as libc::c_int,
                &mut u_5,
                b"\0" as *const u8 as *const libc::c_char,
            ) as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint && u_5 == u_5)
            {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    quotearg_colon(arg),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Invalid number of elements\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                usage(2 as libc::c_int);
            }
            strip_name_components = u_5;
            current_block_308 = 9657238515557273331;
        }
        188 => {
            show_omitted_dirs_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        190 => {
            show_transformed_names_option = 1 as libc::c_int != 0;
            current_block_308 = 9657238515557273331;
        }
        192 => {
            savedir_sort_order = sort_mode_flag[__xargmatch_internal(
                b"--sort\0" as *const u8 as *const libc::c_char,
                arg,
                sort_mode_arg.as_ptr(),
                sort_mode_flag.as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                argmatch_die,
            ) as usize];
            current_block_308 = 9657238515557273331;
        }
        196 => {
            backup_option = 1 as libc::c_int != 0;
            (*args).backup_suffix_string = arg;
            current_block_308 = 9657238515557273331;
        }
        199 => {
            if !to_command_option.is_null() {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Only one --to-command option allowed\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                usage(2 as libc::c_int);
            }
            to_command_option = arg;
            current_block_308 = 9657238515557273331;
        }
        198 => {
            if !arg.is_null() {
                set_stat_signal(arg);
            } else {
                totals_option = 1 as libc::c_int != 0;
            }
            current_block_308 = 9657238515557273331;
        }
        73 => {
            set_use_compress_program_option(arg, (*args).loc);
            current_block_308 = 9657238515557273331;
        }
        202 => {
            volno_file_option = arg;
            current_block_308 = 9657238515557273331;
        }
        161 => {
            same_owner_option = -(1 as libc::c_int);
            current_block_308 = 9657238515557273331;
        }
        162 => {
            same_permissions_option = -(1 as libc::c_int);
            current_block_308 = 9657238515557273331;
        }
        128 => {
            set_archive_format(b"posix\0" as *const u8 as *const libc::c_char);
            acls_option = 1 as libc::c_int;
            current_block_308 = 9657238515557273331;
        }
        154 => {
            acls_option = -(1 as libc::c_int);
            current_block_308 = 9657238515557273331;
        }
        186 => {
            set_archive_format(b"posix\0" as *const u8 as *const libc::c_char);
            selinux_context_option = 1 as libc::c_int;
            current_block_308 = 9657238515557273331;
        }
        164 => {
            selinux_context_option = -(1 as libc::c_int);
            current_block_308 = 9657238515557273331;
        }
        204 => {
            set_xattr_option(1 as libc::c_int);
            current_block_308 = 9657238515557273331;
        }
        165 => {
            set_xattr_option(-(1 as libc::c_int));
            current_block_308 = 9657238515557273331;
        }
        206 | 205 => {
            set_xattr_option(1 as libc::c_int);
            xattrs_mask_add(arg, key == XATTR_INCLUDE as libc::c_int);
            current_block_308 = 9657238515557273331;
        }
        185 => {
            same_owner_option = 1 as libc::c_int;
            current_block_308 = 9657238515557273331;
        }
        16777221 => {
            if (*(*args).loc).source as libc::c_uint
                == OPTS_FILE as libc::c_int as libc::c_uint
            {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s:%lu: location of the error\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*(*args).loc).name,
                    (*(*args).loc).line,
                );
            } else if (*(*args).loc).source as libc::c_uint
                == OPTS_ENVIRON as libc::c_int as libc::c_uint
            {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"error parsing %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*(*args).loc).name,
                );
            }
            exit(64 as libc::c_int);
        }
        _ => return 7 as libc::c_int,
    }
    match current_block_308 {
        12792210781472814257 => {
            if 0 as libc::c_int as libc::c_long <= newer_mtime_option.tv_nsec {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"More than one threshold date\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                usage(2 as libc::c_int);
            }
            get_date_or_file(
                args,
                if key == NEWER_MTIME_OPTION as libc::c_int {
                    b"--newer-mtime\0" as *const u8 as *const libc::c_char
                } else {
                    b"--after-date\0" as *const u8 as *const libc::c_char
                },
                arg,
                &mut newer_mtime_option,
            );
            optloc_save(OC_NEWER as libc::c_int as libc::c_uint, (*args).loc);
        }
        1154520408629132897 => {
            incremental_option = 1 as libc::c_int != 0;
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
static mut argp_children: [argp_child; 2] = unsafe {
    [
        {
            let mut init = argp_child {
                argp: &names_argp as *const argp as *mut argp,
                flags: 0 as libc::c_int,
                header: 0 as *const libc::c_char,
                group: GRID_FILE_NAME as libc::c_int,
            };
            init
        },
        {
            let mut init = argp_child {
                argp: 0 as *const argp,
                flags: 0,
                header: 0 as *const libc::c_char,
                group: 0,
            };
            init
        },
    ]
};
static mut argp: argp = unsafe {
    {
        let mut init = argp {
            options: options.as_ptr() as *mut _,
            parser: Some(
                parse_opt
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut libc::c_char,
                        *mut argp_state,
                    ) -> error_t,
            ),
            args_doc: b"[FILE]...\0" as *const u8 as *const libc::c_char,
            doc: doc.as_ptr(),
            children: argp_children.as_ptr() as *mut _,
            help_filter: Some(
                tar_help_filter
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *const libc::c_char,
                        *mut libc::c_void,
                    ) -> *mut libc::c_char,
            ),
            argp_domain: 0 as *const libc::c_char,
        };
        init
    }
};
pub unsafe extern "C" fn usage(mut status: libc::c_int) {
    argp_help(
        &mut argp,
        stderr,
        0x4 as libc::c_int as libc::c_uint,
        program_name as *mut libc::c_char,
    );
    close_stdout();
    exit(status);
}
unsafe extern "C" fn find_argp_option_key(
    mut o: *const argp_option,
    mut key: libc::c_int,
) -> *const argp_option {
    while !(((*o).name).is_null() && (*o).key == 0 as libc::c_int && ((*o).arg).is_null()
        && (*o).flags == 0 as libc::c_int && ((*o).doc).is_null())
    {
        if (*o).key == key {
            return o;
        }
        o = o.offset(1);
        o;
    }
    return 0 as *const argp_option;
}
unsafe extern "C" fn find_argp_option(
    mut ap: *mut argp,
    mut key: libc::c_int,
) -> *const argp_option {
    let mut p: *const argp_option = 0 as *const argp_option;
    let mut child: *const argp_child = 0 as *const argp_child;
    p = find_argp_option_key((*ap).options, key);
    if p.is_null() && !((*ap).children).is_null() {
        child = (*ap).children;
        while !((*child).argp).is_null() {
            p = find_argp_option_key((*(*child).argp).options, key);
            if !p.is_null() {
                break;
            }
            child = child.offset(1);
            child;
        }
    }
    return p;
}
static mut tar_authors: [*const libc::c_char; 3] = [
    b"John Gilmore\0" as *const u8 as *const libc::c_char,
    b"Jay Fenlason\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut subcommand_class: [libc::c_int; 10] = [
    0 as libc::c_int,
    0x2 as libc::c_int | 0x4 as libc::c_int,
    0x2 as libc::c_int,
    0x2 as libc::c_int,
    0x2 as libc::c_int | 0x4 as libc::c_int | 0x10 as libc::c_int,
    0x1 as libc::c_int | 0x10 as libc::c_int,
    0x1 as libc::c_int | 0x10 as libc::c_int,
    0x1 as libc::c_int | 0x10 as libc::c_int,
    0x2 as libc::c_int | 0x4 as libc::c_int,
    0x8 as libc::c_int,
];
pub unsafe extern "C" fn more_options(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut loc: *mut option_locus,
) {
    let mut args: tar_args = {
        let mut init = tar_args {
            loc: loc,
            textual_date: 0 as *mut textual_date,
            o_option: 0 as libc::c_int != 0,
            pax_option: 0 as libc::c_int != 0,
            compress_autodetect: 0 as libc::c_int != 0,
            backup_suffix_string: 0 as *const libc::c_char,
            version_control_string: 0 as *const libc::c_char,
        };
        init
    };
    argp_parse(
        &mut names_argp,
        argc,
        argv,
        (0x8 as libc::c_int | 0x20 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint,
        0 as *mut libc::c_int,
        &mut args as *mut tar_args as *mut libc::c_void,
    );
}
unsafe extern "C" fn parse_default_options(mut args: *mut tar_args) {
    let mut opts: *mut libc::c_char = getenv(
        b"TAR_OPTIONS\0" as *const u8 as *const libc::c_char,
    );
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
    let mut loc: option_locus = {
        let mut init = option_locus {
            source: OPTS_ENVIRON,
            name: b"TAR_OPTIONS\0" as *const u8 as *const libc::c_char,
            line: 0 as libc::c_int as size_t,
            prev: 0 as *mut option_locus,
        };
        init
    };
    let mut save_loc_ptr: *mut option_locus = 0 as *mut option_locus;
    if opts.is_null() {
        return;
    }
    ws.ws_offs = 1 as libc::c_int as size_t;
    if wordsplit(
        opts,
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
                b"cannot split TAR_OPTIONS: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            wordsplit_strerror(&mut ws),
        );
        fatal_exit();
    }
    if ws.ws_wordc != 0 {
        let mut idx: libc::c_int = 0;
        let ref mut fresh9 = *(ws.ws_wordv).offset(0 as libc::c_int as isize);
        *fresh9 = program_name as *mut libc::c_char;
        save_loc_ptr = (*args).loc;
        (*args).loc = &mut loc;
        if argp_parse(
            &mut argp,
            (ws.ws_offs).wrapping_add(ws.ws_wordc) as libc::c_int,
            ws.ws_wordv,
            (0x8 as libc::c_int | 0x20 as libc::c_int) as libc::c_uint,
            &mut idx,
            args as *mut libc::c_void,
        ) != 0
        {
            abort();
        }
        (*args).loc = save_loc_ptr;
        if name_more_files() {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"non-option arguments in %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                loc.name,
            );
            usage(2 as libc::c_int);
        }
        ws.ws_wordc = 0 as libc::c_int as size_t;
    }
    wordsplit_free(&mut ws);
}
unsafe extern "C" fn decode_options(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut idx: libc::c_int = 0;
    let mut loc: option_locus = {
        let mut init = option_locus {
            source: OPTS_COMMAND_LINE,
            name: 0 as *const libc::c_char,
            line: 0 as libc::c_int as size_t,
            prev: 0 as *mut option_locus,
        };
        init
    };
    let mut args: tar_args = {
        let mut init = tar_args {
            loc: &mut loc,
            textual_date: 0 as *mut textual_date,
            o_option: 0 as libc::c_int != 0,
            pax_option: 0 as libc::c_int != 0,
            compress_autodetect: 0 as libc::c_int != 0,
            backup_suffix_string: 0 as *const libc::c_char,
            version_control_string: 0 as *const libc::c_char,
        };
        init
    };
    argp_version_setup(
        b"tar\0" as *const u8 as *const libc::c_char,
        tar_authors.as_mut_ptr(),
    );
    args
        .backup_suffix_string = getenv(
        b"SIMPLE_BACKUP_SUFFIX\0" as *const u8 as *const libc::c_char,
    );
    posixly_correct = !(getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char))
        .is_null();
    subcommand_option = UNKNOWN_SUBCOMMAND;
    archive_format = DEFAULT_FORMAT;
    blocking_factor = 20 as libc::c_int;
    record_size = (20 as libc::c_int * 512 as libc::c_int) as size_t;
    excluded = new_exclude();
    hole_detection = HOLE_DETECTION_DEFAULT;
    newer_mtime_option
        .tv_sec = !if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
        -(1 as libc::c_int) as time_t
    } else {
        (((1 as libc::c_int as time_t)
            << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
            - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long
    };
    newer_mtime_option.tv_nsec = -(1 as libc::c_int) as __syscall_slong_t;
    mtime_option
        .tv_sec = !if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
        -(1 as libc::c_int) as time_t
    } else {
        (((1 as libc::c_int as time_t)
            << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
            - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long
    };
    mtime_option.tv_nsec = -(1 as libc::c_int) as __syscall_slong_t;
    recursion_option = (1 as libc::c_int) << 3 as libc::c_int;
    unquote_option = 1 as libc::c_int != 0;
    tar_sparse_major = 1 as libc::c_int as libc::c_uint;
    tar_sparse_minor = 0 as libc::c_int as libc::c_uint;
    savedir_sort_order = SAVEDIR_SORT_NONE as libc::c_int;
    owner_option = -(1 as libc::c_int) as uid_t;
    owner_name_option = 0 as *const libc::c_char;
    group_option = -(1 as libc::c_int) as gid_t;
    group_name_option = 0 as *const libc::c_char;
    check_device_option = 1 as libc::c_int != 0;
    incremental_level = -(1 as libc::c_int);
    seek_option = -(1 as libc::c_int);
    if argc > 1 as libc::c_int
        && *(*argv.offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int != '-' as i32
    {
        let mut new_argc: libc::c_int = 0;
        let mut new_argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut in_0: *const *mut libc::c_char = 0 as *const *mut libc::c_char;
        let mut out: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut letter: *const libc::c_char = 0 as *const libc::c_char;
        let mut buffer: [libc::c_char; 3] = [0; 3];
        buffer[0 as libc::c_int as usize] = '-' as i32 as libc::c_char;
        buffer[2 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        new_argc = ((argc - 1 as libc::c_int) as libc::c_ulong)
            .wrapping_add(strlen(*argv.offset(1 as libc::c_int as isize)))
            as libc::c_int;
        new_argv = xmalloc(
            ((new_argc + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
        in_0 = argv;
        out = new_argv;
        let fresh10 = in_0;
        in_0 = in_0.offset(1);
        let fresh11 = out;
        out = out.offset(1);
        *fresh11 = *fresh10;
        let fresh12 = in_0;
        in_0 = in_0.offset(1);
        letter = *fresh12;
        while *letter != 0 {
            let mut opt: *const argp_option = 0 as *const argp_option;
            buffer[1 as libc::c_int as usize] = *letter;
            let fresh13 = out;
            out = out.offset(1);
            *fresh13 = xstrdup(buffer.as_mut_ptr());
            opt = find_argp_option(&mut argp, *letter as libc::c_int);
            if !opt.is_null() && !((*opt).arg).is_null() {
                if in_0 < argv.offset(argc as isize) as *const *mut libc::c_char {
                    let fresh14 = in_0;
                    in_0 = in_0.offset(1);
                    let fresh15 = out;
                    out = out.offset(1);
                    *fresh15 = *fresh14;
                } else {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Old option '%c' requires an argument.\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *letter as libc::c_int,
                    );
                    usage(2 as libc::c_int);
                }
            }
            letter = letter.offset(1);
            letter;
        }
        while in_0 < argv.offset(argc as isize) as *const *mut libc::c_char {
            let fresh16 = in_0;
            in_0 = in_0.offset(1);
            let fresh17 = out;
            out = out.offset(1);
            *fresh17 = *fresh16;
        }
        *out = 0 as *mut libc::c_char;
        argc = new_argc;
        argv = new_argv;
    }
    parse_default_options(&mut args);
    if argp_parse(
        &mut argp,
        argc,
        argv,
        0x8 as libc::c_int as libc::c_uint,
        &mut idx,
        &mut args as *mut tar_args as *mut libc::c_void,
    ) != 0
    {
        exit(2 as libc::c_int);
    }
    if args.o_option {
        if subcommand_option as libc::c_uint
            == CREATE_SUBCOMMAND as libc::c_int as libc::c_uint
        {
            set_archive_format(b"v7\0" as *const u8 as *const libc::c_char);
        } else {
            same_owner_option = -(1 as libc::c_int);
        }
    }
    while idx < argc {
        name_add_name(*argv.offset(idx as isize));
        idx += 1;
        idx;
    }
    if archive_format as libc::c_uint == DEFAULT_FORMAT as libc::c_int as libc::c_uint {
        if args.pax_option {
            archive_format = POSIX_FORMAT;
        } else {
            archive_format = GNU_FORMAT;
        }
    }
    if !volume_label_option.is_null()
        && subcommand_option as libc::c_uint
            == CREATE_SUBCOMMAND as libc::c_int as libc::c_uint
        || incremental_option as libc::c_int != 0
        || multi_volume_option as libc::c_int != 0 || sparse_option as libc::c_int != 0
    {
        assert_format(
            ((1 as libc::c_int) << OLDGNU_FORMAT as libc::c_int
                | (1 as libc::c_int) << GNU_FORMAT as libc::c_int
                | (1 as libc::c_int) << POSIX_FORMAT as libc::c_int) as libc::c_uint,
        );
    }
    if occurrence_option != 0 {
        if !name_more_files() {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"--occurrence is meaningless without a file list\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(2 as libc::c_int);
        }
        if subcommand_class[subcommand_option as usize] & 0x10 as libc::c_int == 0 {
            if option_set_in_cl(OC_OCCURRENCE as libc::c_int) != 0 {
                option_conflict_error(
                    b"--occurrence\0" as *const u8 as *const libc::c_char,
                    subcommand_string(subcommand_option),
                );
            } else {
                occurrence_option = 0 as libc::c_int as uintmax_t;
            }
        }
    }
    if archive_names == 0 as libc::c_int as libc::c_ulong {
        archive_names = 1 as libc::c_int as size_t;
        let ref mut fresh18 = *archive_name_array.offset(0 as libc::c_int as isize);
        *fresh18 = getenv(b"TAPE\0" as *const u8 as *const libc::c_char);
        if (*archive_name_array.offset(0 as libc::c_int as isize)).is_null() {
            let ref mut fresh19 = *archive_name_array.offset(0 as libc::c_int as isize);
            *fresh19 = b"-\0" as *const u8 as *const libc::c_char;
        }
    }
    if archive_names > 1 as libc::c_int as libc::c_ulong && !multi_volume_option {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Multiple archive files require '-M' option\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(2 as libc::c_int);
    }
    if !listed_incremental_option.is_null()
        && 0 as libc::c_int as libc::c_long <= newer_mtime_option.tv_nsec
    {
        let mut listed_loc: *mut option_locus = optloc_lookup(
            OC_LISTED_INCREMENTAL as libc::c_int,
        );
        let mut newer_loc: *mut option_locus = optloc_lookup(OC_NEWER as libc::c_int);
        if optloc_eq(listed_loc, newer_loc) != 0 {
            option_conflict_error(
                b"--listed-incremental\0" as *const u8 as *const libc::c_char,
                b"--newer\0" as *const u8 as *const libc::c_char,
            );
        } else if (*listed_loc).source as libc::c_uint
            == OPTS_COMMAND_LINE as libc::c_int as libc::c_uint
        {
            listed_incremental_option = 0 as *const libc::c_char;
        } else {
            memset(
                &mut newer_mtime_option as *mut timespec as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<timespec>() as libc::c_ulong,
            );
        }
    }
    if incremental_level != -(1 as libc::c_int) && listed_incremental_option.is_null() {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"--level is meaningless without --listed-incremental\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if !volume_label_option.is_null() {
        if archive_format as libc::c_uint == GNU_FORMAT as libc::c_int as libc::c_uint
            || archive_format as libc::c_uint
                == OLDGNU_FORMAT as libc::c_int as libc::c_uint
        {
            let mut volume_label_max_len: size_t = (::std::mem::size_of::<
                [libc::c_char; 100],
            >() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(
                    if multi_volume_option as libc::c_int != 0 {
                        (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(
                                        !((0 as libc::c_int) < -(1 as libc::c_int)) as libc::c_int
                                            as libc::c_ulong,
                                    )
                                    .wrapping_mul(146 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(484 as libc::c_int as libc::c_ulong)
                                    .wrapping_div(485 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(
                                        !((0 as libc::c_int) < -(1 as libc::c_int)) as libc::c_int
                                            as libc::c_ulong,
                                    ),
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        0 as libc::c_int as libc::c_ulong
                    },
                );
            if volume_label_max_len < strlen(volume_label_option) {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcngettext(
                        0 as *const libc::c_char,
                        b"%s: Volume label is too long (limit is %lu byte)\0"
                            as *const u8 as *const libc::c_char,
                        b"%s: Volume label is too long (limit is %lu bytes)\0"
                            as *const u8 as *const libc::c_char,
                        volume_label_max_len,
                        5 as libc::c_int,
                    ),
                    quotearg_colon(volume_label_option),
                    volume_label_max_len,
                );
                usage(2 as libc::c_int);
            }
        }
    }
    if verify_option {
        if multi_volume_option {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cannot verify multi-volume archives\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(2 as libc::c_int);
        }
        if !use_compress_program_option.is_null() {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cannot verify compressed archives\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(2 as libc::c_int);
        }
        if subcommand_class[subcommand_option as usize] & 0x2 as libc::c_int == 0 {
            if option_set_in_cl(OC_VERIFY as libc::c_int) != 0 {
                option_conflict_error(
                    b"--verify\0" as *const u8 as *const libc::c_char,
                    subcommand_string(subcommand_option),
                );
            } else {
                verify_option = 0 as libc::c_int != 0;
            }
        }
    }
    if !use_compress_program_option.is_null() {
        if multi_volume_option {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cannot use multi-volume compressed archives\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(2 as libc::c_int);
        }
        if subcommand_class[subcommand_option as usize] & 0x4 as libc::c_int != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cannot update compressed archives\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(2 as libc::c_int);
        }
        if subcommand_option as libc::c_uint
            == CAT_SUBCOMMAND as libc::c_int as libc::c_uint
        {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cannot concatenate compressed archives\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(2 as libc::c_int);
        }
    }
    if set_mtime_option as libc::c_uint == CLAMP_MTIME as libc::c_int as libc::c_uint {
        if !(0 as libc::c_int as libc::c_long <= mtime_option.tv_nsec) {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"--clamp-mtime needs a date specified using --mtime\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(2 as libc::c_int);
        }
    }
    if args.pax_option as libc::c_int != 0
        && archive_format as libc::c_uint != POSIX_FORMAT as libc::c_int as libc::c_uint
        && subcommand_class[subcommand_option as usize] & 0x1 as libc::c_int == 0
    {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"--pax-option can be used only on POSIX archives\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(2 as libc::c_int);
    }
    if acls_option > 0 as libc::c_int
        && archive_format as libc::c_uint != POSIX_FORMAT as libc::c_int as libc::c_uint
        && subcommand_class[subcommand_option as usize] & 0x1 as libc::c_int == 0
    {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"--acls can be used only on POSIX archives\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(2 as libc::c_int);
    }
    if selinux_context_option > 0 as libc::c_int
        && archive_format as libc::c_uint != POSIX_FORMAT as libc::c_int as libc::c_uint
        && subcommand_class[subcommand_option as usize] & 0x1 as libc::c_int == 0
    {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"--selinux can be used only on POSIX archives\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(2 as libc::c_int);
    }
    if xattrs_option > 0 as libc::c_int
        && archive_format as libc::c_uint != POSIX_FORMAT as libc::c_int as libc::c_uint
        && subcommand_class[subcommand_option as usize] & 0x1 as libc::c_int == 0
    {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"--xattrs can be used only on POSIX archives\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(2 as libc::c_int);
    }
    if starting_file_option as libc::c_int != 0
        && subcommand_class[subcommand_option as usize] & 0x1 as libc::c_int == 0
    {
        if option_set_in_cl(OC_STARTING_FILE as libc::c_int) != 0 {
            option_conflict_error(
                b"--starting-file\0" as *const u8 as *const libc::c_char,
                subcommand_string(subcommand_option),
            );
        } else {
            starting_file_option = 0 as libc::c_int != 0;
        }
    }
    if same_order_option as libc::c_int != 0
        && subcommand_class[subcommand_option as usize] & 0x1 as libc::c_int == 0
    {
        if option_set_in_cl(OC_SAME_ORDER as libc::c_int) != 0 {
            option_conflict_error(
                b"--same-order\0" as *const u8 as *const libc::c_char,
                subcommand_string(subcommand_option),
            );
        } else {
            same_order_option = 0 as libc::c_int != 0;
        }
    }
    if one_top_level_option {
        let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
        if absolute_names_option {
            let mut one_top_level_loc: *mut option_locus = optloc_lookup(
                OC_ONE_TOP_LEVEL as libc::c_int,
            );
            let mut absolute_names_loc: *mut option_locus = optloc_lookup(
                OC_ABSOLUTE_NAMES as libc::c_int,
            );
            if optloc_eq(one_top_level_loc, absolute_names_loc) != 0 {
                option_conflict_error(
                    b"--one-top-level\0" as *const u8 as *const libc::c_char,
                    b"--absolute-names\0" as *const u8 as *const libc::c_char,
                );
            } else if (*one_top_level_loc).source as libc::c_uint
                == OPTS_COMMAND_LINE as libc::c_int as libc::c_uint
            {
                absolute_names_option = 0 as libc::c_int != 0;
            } else {
                one_top_level_option = 0 as libc::c_int != 0;
            }
        }
        if one_top_level_option as libc::c_int != 0 && one_top_level_dir.is_null() {
            base = base_name(*archive_name_array.offset(0 as libc::c_int as isize));
            one_top_level_dir = strip_compression_suffix(base);
            rpl_free(base as *mut libc::c_void);
            if one_top_level_dir.is_null() {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Cannot deduce top-level directory name; please set it explicitly with --one-top-level=DIR\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                usage(2 as libc::c_int);
            }
        }
    }
    if recursive_unlink_option {
        old_files_option = UNLINK_FIRST_OLD_FILES;
    }
    let mut base_open_flags: libc::c_int = 0 as libc::c_int | 0o2000000 as libc::c_int
        | 0o400 as libc::c_int | 0o4000 as libc::c_int
        | (if dereference_option as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            0o400000 as libc::c_int
        })
        | (if atime_preserve_option as libc::c_uint
            == system_atime_preserve as libc::c_int as libc::c_uint
        {
            0o1000000 as libc::c_int
        } else {
            0 as libc::c_int
        });
    open_read_flags = 0 as libc::c_int | base_open_flags;
    open_searchdir_flags = 0 as libc::c_int | 0o200000 as libc::c_int | base_open_flags;
    fstatat_flags = if dereference_option as libc::c_int != 0 {
        0 as libc::c_int
    } else {
        0x100 as libc::c_int
    };
    if subcommand_option as libc::c_uint
        == TEST_LABEL_SUBCOMMAND as libc::c_int as libc::c_uint
    {
        if !name_more_files() {
            verbose_option += 1;
            verbose_option;
        }
    } else if utc_option {
        verbose_option = 2 as libc::c_int;
    }
    if tape_length_option != 0. && tape_length_option < record_size as libc::c_double {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Volume length cannot be less than record size\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(2 as libc::c_int);
    }
    if same_order_option as libc::c_int != 0 && !listed_incremental_option.is_null() {
        let mut preserve_order_loc: *mut option_locus = optloc_lookup(
            OC_SAME_ORDER as libc::c_int,
        );
        let mut listed_incremental_loc: *mut option_locus = optloc_lookup(
            OC_LISTED_INCREMENTAL as libc::c_int,
        );
        if optloc_eq(preserve_order_loc, listed_incremental_loc) != 0 {
            option_conflict_error(
                b"--preserve-order\0" as *const u8 as *const libc::c_char,
                b"--listed-incremental\0" as *const u8 as *const libc::c_char,
            );
        } else if (*preserve_order_loc).source as libc::c_uint
            == OPTS_COMMAND_LINE as libc::c_int as libc::c_uint
        {
            listed_incremental_option = 0 as *const libc::c_char;
        } else {
            same_order_option = 0 as libc::c_int != 0;
        }
    }
    match subcommand_option as libc::c_uint {
        3 => {
            if !name_more_files() {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Cowardly refusing to create an empty archive\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                usage(2 as libc::c_int);
            }
            if args.compress_autodetect as libc::c_int != 0 && archive_names != 0
                && strcmp(
                    *archive_name_array.offset(0 as libc::c_int as isize),
                    b"-\0" as *const u8 as *const libc::c_char,
                ) != 0
            {
                set_compression_program_by_suffix(
                    *archive_name_array.offset(0 as libc::c_int as isize),
                    use_compress_program_option,
                );
            }
        }
        6 | 7 | 5 | 9 => {
            archive_name_cursor = archive_name_array;
            while archive_name_cursor < archive_name_array.offset(archive_names as isize)
            {
                if strcmp(
                    *archive_name_cursor,
                    b"-\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    request_stdin(b"-f\0" as *const u8 as *const libc::c_char);
                }
                archive_name_cursor = archive_name_cursor.offset(1);
                archive_name_cursor;
            }
        }
        2 | 8 | 1 => {
            archive_name_cursor = archive_name_array;
            while archive_name_cursor < archive_name_array.offset(archive_names as isize)
            {
                if strcmp(
                    *archive_name_cursor,
                    b"-\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Options '-Aru' are incompatible with '-f -'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    usage(2 as libc::c_int);
                }
                archive_name_cursor = archive_name_cursor.offset(1);
                archive_name_cursor;
            }
        }
        _ => {}
    }
    if !index_file_name.is_null() {
        stdlis = fopen(index_file_name, b"w\0" as *const u8 as *const libc::c_char);
        if stdlis.is_null() {
            open_fatal(index_file_name);
        }
    } else {
        stdlis = if to_stdout_option as libc::c_int != 0 { stderr } else { stdout };
    }
    archive_name_cursor = archive_name_array;
    if !(args.backup_suffix_string).is_null() {
        simple_backup_suffix = xstrdup(args.backup_suffix_string);
    }
    if backup_option {
        backup_type = xget_version(
            b"--backup\0" as *const u8 as *const libc::c_char,
            args.version_control_string,
        );
        if backup_type as libc::c_uint == no_backups as libc::c_int as libc::c_uint
            || (to_stdout_option as libc::c_int != 0 || !to_command_option.is_null())
        {
            backup_option = 0 as libc::c_int != 0;
        }
    }
    checkpoint_finish_compile();
    report_textual_dates(&mut args);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    set_start_time();
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"tar\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"tar\0" as *const u8 as *const libc::c_char);
    ::std::ptr::write_volatile(&mut exit_failure as *mut libc::c_int, 2 as libc::c_int);
    exit_status = 0 as libc::c_int;
    error_hook = Some(checkpoint_flush_actions as unsafe extern "C" fn() -> ());
    set_quoting_style(0 as *mut quoting_options, escape_quoting_style);
    close_stdout_set_file_name(
        dcgettext(
            0 as *const libc::c_char,
            b"stdout\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    if stdopen() != 0 {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"failed to assert availability of the standard file descriptors\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fatal_exit();
    }
    allocated_archive_names = 10 as libc::c_int as size_t;
    archive_name_array = xmalloc(
        (::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            .wrapping_mul(allocated_archive_names),
    ) as *mut *const libc::c_char;
    archive_names = 0 as libc::c_int as size_t;
    signal(17 as libc::c_int, None);
    priv_set_remove_linkdir();
    decode_options(argc, argv);
    name_init();
    if !volno_file_option.is_null() {
        init_volume_number();
    }
    match subcommand_option as libc::c_uint {
        0 => {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"You must specify one of the '-Acdtrux', '--delete' or '--test-label' options\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(2 as libc::c_int);
        }
        2 | 8 | 1 => {
            update_archive();
        }
        4 => {
            delete_archive_members();
        }
        3 => {
            create_archive();
        }
        6 => {
            extr_init();
            read_and(Some(extract_archive as unsafe extern "C" fn() -> ()));
            extract_finish();
        }
        7 => {
            read_and(Some(list_archive as unsafe extern "C" fn() -> ()));
        }
        5 => {
            diff_init();
            read_and(Some(diff_archive as unsafe extern "C" fn() -> ()));
        }
        9 => {
            test_archive_label();
        }
        _ => {}
    }
    checkpoint_finish();
    if totals_option {
        print_total_stats();
    }
    if check_links_option != 0 {
        check_links();
    }
    if !volno_file_option.is_null() {
        closeout_volume_number();
    }
    rpl_free(archive_name_array as *mut libc::c_void);
    xattrs_clear_setup();
    name_term();
    if exit_status == 2 as libc::c_int {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Exiting with failure status due to previous errors\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if stdlis == stdout {
        close_stdout();
    } else if ferror_unlocked(stderr) != 0 || fclose(stderr) != 0 as libc::c_int {
        set_exit_status(2 as libc::c_int);
    }
    return exit_status;
}
pub unsafe extern "C" fn tar_stat_init(mut st: *mut tar_stat_info) {
    memset(
        st as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<tar_stat_info>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn tar_stat_close(mut st: *mut tar_stat_info) -> bool {
    let mut status: libc::c_int = if !((*st).dirstream).is_null() {
        closedir((*st).dirstream)
    } else if (0 as libc::c_int) < (*st).fd {
        close((*st).fd)
    } else {
        0 as libc::c_int
    };
    (*st).dirstream = 0 as *mut DIR;
    (*st).fd = 0 as libc::c_int;
    if status == 0 as libc::c_int {
        return 1 as libc::c_int != 0
    } else {
        close_diag((*st).orig_file_name);
        return 0 as libc::c_int != 0;
    };
}
pub unsafe extern "C" fn tar_stat_destroy(mut st: *mut tar_stat_info) {
    tar_stat_close(st);
    xheader_xattr_free((*st).xattr_map, (*st).xattr_map_size);
    rpl_free((*st).orig_file_name as *mut libc::c_void);
    rpl_free((*st).file_name as *mut libc::c_void);
    rpl_free((*st).link_name as *mut libc::c_void);
    rpl_free((*st).uname as *mut libc::c_void);
    rpl_free((*st).gname as *mut libc::c_void);
    rpl_free((*st).cntx_name as *mut libc::c_void);
    rpl_free((*st).acls_a_ptr as *mut libc::c_void);
    rpl_free((*st).acls_d_ptr as *mut libc::c_void);
    rpl_free((*st).sparse_map as *mut libc::c_void);
    rpl_free((*st).dumpdir as *mut libc::c_void);
    xheader_destroy(&mut (*st).xhdr);
    info_free_exclist(st);
    memset(
        st as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<tar_stat_info>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn tar_timespec_cmp(
    mut a: timespec,
    mut b: timespec,
) -> libc::c_int {
    if (1 as libc::c_int) << current_format as libc::c_uint
        & (1 as libc::c_int) << POSIX_FORMAT as libc::c_int == 0
    {
        b.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
        a.tv_nsec = b.tv_nsec;
    }
    return timespec_cmp(a, b);
}
pub unsafe extern "C" fn set_exit_status(mut val: libc::c_int) {
    if val > exit_status {
        exit_status = val;
    }
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
