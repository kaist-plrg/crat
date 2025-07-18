use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type exclist;
    pub type wordsplit_node;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
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
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn creat(__file: *const libc::c_char, __mode: mode_t) -> libc::c_int;
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
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    static mut environ: *mut *mut libc::c_char;
    fn execv(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn execlp(
        __file: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xalloc_die();
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    static mut error_hook: Option::<unsafe extern "C" fn() -> ()>;
    static mut exit_status: libc::c_int;
    fn exec_fatal(_: *const libc::c_char) -> !;
    fn open_fatal(_: *const libc::c_char) -> !;
    fn read_fatal(_: *const libc::c_char) -> !;
    fn waitpid_error(_: *const libc::c_char);
    fn write_error(_: *const libc::c_char);
    fn fatal_exit() -> !;
    fn set_program_name(argv0: *const libc::c_char);
    static mut subcommand_option: subcommand;
    static mut archive_format: archive_format;
    static mut blocking_factor: libc::c_int;
    static mut record_size: size_t;
    static mut backup_option: bool;
    static mut use_compress_program_option: *const libc::c_char;
    static mut info_script_option: *const libc::c_char;
    static mut rsh_command_option: *const libc::c_char;
    static mut to_command_option: *mut libc::c_char;
    static mut ignore_command_error_option: bool;
    fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> size_t;
    static mut archive: libc::c_int;
    static mut dev_null_output: bool;
    static mut archive_name_array: *mut *const libc::c_char;
    static mut archive_name_cursor: *mut *const libc::c_char;
    static mut ar_dev: dev_t;
    static mut ar_ino: ino_t;
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn xpipe(fd: *mut libc::c_int);
    fn undo_last_backup();
    static mut current_format: archive_format;
    fn code_timespec(ts: timespec, sbuf: *mut libc::c_char) -> *const libc::c_char;
    fn xfork() -> pid_t;
    fn maybe_backup_file(
        file_name: *const libc::c_char,
        this_is_the_archive: bool,
    ) -> bool;
    fn clear_read_error_count();
    fn xclose(fd: libc::c_int);
    fn archive_write_error(status: ssize_t) -> !;
    fn archive_read_error();
    fn first_decompress_program(pstate: *mut libc::c_int) -> *const libc::c_char;
    fn next_decompress_program(pstate: *mut libc::c_int) -> *const libc::c_char;
    fn subcommand_string(c: subcommand) -> *const libc::c_char;
    fn archive_format_string(fmt: archive_format) -> *const libc::c_char;
    static mut warning_option: libc::c_int;
    static mut rmt_dev_name__: *const libc::c_char;
    fn rmt_open__(
        _: *const libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> libc::c_int;
    fn rmt_read__(_: libc::c_int, _: *mut libc::c_char, _: size_t) -> size_t;
    fn rmt_write__(_: libc::c_int, _: *mut libc::c_char, _: size_t) -> size_t;
    static mut force_local_option: bool;
    fn wordsplit(
        s: *const libc::c_char,
        ws: *mut wordsplit_t,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn wordsplit_strerror(ws: *mut wordsplit_t) -> *const libc::c_char;
    static mut record_start: *mut block;
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
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut libc::c_char,
    pub next_free: *mut libc::c_char,
    pub chunk_limit: *mut libc::c_char,
    pub temp: C2RustUnnamed_1,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_0,
    pub freefun: C2RustUnnamed,
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
pub union C2RustUnnamed {
    pub plain: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub plain: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
unsafe extern "C" fn gnu_dev_major(mut __dev: __dev_t) -> libc::c_uint {
    let mut __major: libc::c_uint = 0;
    __major = ((__dev & 0xfff00 as libc::c_uint as __dev_t) >> 8 as libc::c_int)
        as libc::c_uint;
    __major = (__major as libc::c_ulong
        | (__dev & 0xfffff00000000000 as libc::c_ulong) >> 32 as libc::c_int)
        as libc::c_uint;
    return __major;
}
#[inline]
unsafe extern "C" fn gnu_dev_minor(mut __dev: __dev_t) -> libc::c_uint {
    let mut __minor: libc::c_uint = 0;
    __minor = ((__dev & 0xff as libc::c_uint as __dev_t) >> 0 as libc::c_int)
        as libc::c_uint;
    __minor = (__minor as libc::c_ulong
        | (__dev & 0xffffff00000 as libc::c_ulong) >> 12 as libc::c_int) as libc::c_uint;
    return __minor;
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
unsafe extern "C" fn priv_set_restore_linkdir() -> libc::c_int {
    return -(1 as libc::c_int);
}
unsafe extern "C" fn xexec(mut cmd: *const libc::c_char) {
    let mut argv: [*mut libc::c_char; 4] = [0 as *mut libc::c_char; 4];
    argv[0 as libc::c_int
        as usize] = b"/bin/sh\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    argv[1 as libc::c_int
        as usize] = b"-c\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[2 as libc::c_int as usize] = cmd as *mut libc::c_char;
    argv[3 as libc::c_int as usize] = 0 as *mut libc::c_char;
    execv(
        b"/bin/sh\0" as *const u8 as *const libc::c_char,
        argv.as_mut_ptr() as *const *mut libc::c_char,
    );
    exec_fatal(cmd);
}
static mut archive_stat: stat = stat {
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
pub unsafe extern "C" fn sys_get_archive_stat() -> bool {
    return fstat(archive, &mut archive_stat) == 0 as libc::c_int;
}
pub unsafe extern "C" fn sys_file_is_archive(mut p: *mut tar_stat_info) -> bool {
    return ar_dev != 0 && (*p).stat.st_dev == ar_dev && (*p).stat.st_ino == ar_ino;
}
pub unsafe extern "C" fn sys_save_archive_dev_ino() {
    if !(archive >= (1 as libc::c_int) << 30 as libc::c_int)
        && archive_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint
    {
        ar_dev = archive_stat.st_dev;
        ar_ino = archive_stat.st_ino;
    } else {
        ar_dev = 0 as libc::c_int as dev_t;
    };
}
pub unsafe extern "C" fn sys_detect_dev_null_output() {
    static mut dev_null: [libc::c_char; 10] = unsafe {
        *::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"/dev/null\0")
    };
    let mut dev_null_stat: stat = stat {
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
    dev_null_output = strcmp(
        *archive_name_array.offset(0 as libc::c_int as isize),
        dev_null.as_ptr(),
    ) == 0 as libc::c_int
        || !(archive >= (1 as libc::c_int) << 30 as libc::c_int)
            && archive_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o20000 as libc::c_int as libc::c_uint
            && stat(dev_null.as_ptr(), &mut dev_null_stat) == 0 as libc::c_int
            && archive_stat.st_dev == dev_null_stat.st_dev
            && archive_stat.st_ino == dev_null_stat.st_ino;
}
pub unsafe extern "C" fn sys_wait_for_child(mut child_pid: pid_t, mut eof: bool) {
    if child_pid != 0 {
        let mut wait_status: libc::c_int = 0;
        while waitpid(child_pid, &mut wait_status, 0 as libc::c_int)
            == -(1 as libc::c_int)
        {
            if !(*__errno_location() != 4 as libc::c_int) {
                continue;
            }
            waitpid_error(use_compress_program_option);
            break;
        }
        if ((wait_status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
            as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
        {
            let mut sig: libc::c_int = wait_status & 0x7f as libc::c_int;
            if !(!eof && sig == 13 as libc::c_int) {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Child died with signal %d\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    sig,
                );
                fatal_exit();
            }
        } else if (wait_status & 0xff00 as libc::c_int) >> 8 as libc::c_int
            != 0 as libc::c_int
        {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Child returned status %d\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (wait_status & 0xff00 as libc::c_int) >> 8 as libc::c_int,
            );
            fatal_exit();
        }
    }
}
pub unsafe extern "C" fn sys_spawn_shell() {
    let mut child: pid_t = 0;
    let mut shell: *const libc::c_char = getenv(
        b"SHELL\0" as *const u8 as *const libc::c_char,
    );
    if shell.is_null() {
        shell = b"/bin/sh\0" as *const u8 as *const libc::c_char;
    }
    child = xfork();
    if child == 0 as libc::c_int {
        priv_set_restore_linkdir();
        execlp(
            shell,
            b"-sh\0" as *const u8 as *const libc::c_char,
            b"-i\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        exec_fatal(shell);
    } else {
        let mut wait_status: libc::c_int = 0;
        while waitpid(child, &mut wait_status, 0 as libc::c_int) == -(1 as libc::c_int) {
            if !(*__errno_location() != 4 as libc::c_int) {
                continue;
            }
            waitpid_error(shell);
            break;
        }
    };
}
pub unsafe extern "C" fn sys_compare_uid(mut a: *mut stat, mut b: *mut stat) -> bool {
    return (*a).st_uid == (*b).st_uid;
}
pub unsafe extern "C" fn sys_compare_gid(mut a: *mut stat, mut b: *mut stat) -> bool {
    return (*a).st_gid == (*b).st_gid;
}
pub unsafe extern "C" fn sys_compare_links(
    mut link_data: *mut stat,
    mut stat_data: *mut stat,
) -> bool {
    return (*stat_data).st_dev == (*link_data).st_dev
        && (*stat_data).st_ino == (*link_data).st_ino;
}
pub unsafe extern "C" fn sys_truncate(mut fd: libc::c_int) -> libc::c_int {
    let mut pos: off_t = lseek(fd, 0 as libc::c_int as off_t, 1 as libc::c_int);
    return if pos < 0 as libc::c_int as libc::c_long {
        -(1 as libc::c_int)
    } else {
        ftruncate(fd, pos)
    };
}
unsafe extern "C" fn is_regular_file(mut name: *const libc::c_char) -> libc::c_int {
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
    if stat(name, &mut stbuf) == 0 as libc::c_int {
        return (stbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint) as libc::c_int
    } else {
        return (*__errno_location() == 2 as libc::c_int) as libc::c_int
    };
}
pub unsafe extern "C" fn sys_write_archive_buffer() -> size_t {
    return if archive >= (1 as libc::c_int) << 30 as libc::c_int {
        rmt_write__(
            archive - ((1 as libc::c_int) << 30 as libc::c_int),
            ((*record_start).buffer).as_mut_ptr(),
            record_size,
        )
    } else {
        full_write(
            archive,
            ((*record_start).buffer).as_mut_ptr() as *const libc::c_void,
            record_size,
        )
    };
}
unsafe extern "C" fn xdup2(mut from: libc::c_int, mut into: libc::c_int) {
    if from != into {
        let mut status: libc::c_int = close(into);
        if status != 0 as libc::c_int && *__errno_location() != 9 as libc::c_int {
            let mut e: libc::c_int = *__errno_location();
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                e,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cannot close\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            fatal_exit();
        }
        status = dup(from);
        if status != into {
            if status < 0 as libc::c_int {
                let mut e_0: libc::c_int = *__errno_location();
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    e_0,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Cannot dup\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                fatal_exit();
            }
            abort();
        }
        xclose(from);
    }
}
unsafe extern "C" fn wait_for_grandchild(mut pid: pid_t) -> ! {
    let mut wait_status: libc::c_int = 0;
    let mut exit_code: libc::c_int = 0 as libc::c_int;
    while waitpid(pid, &mut wait_status, 0 as libc::c_int) == -(1 as libc::c_int) {
        if !(*__errno_location() != 4 as libc::c_int) {
            continue;
        }
        waitpid_error(use_compress_program_option);
        break;
    }
    if ((wait_status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
        as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
    {
        raise(wait_status & 0x7f as libc::c_int);
    } else if (wait_status & 0xff00 as libc::c_int) >> 8 as libc::c_int
        != 0 as libc::c_int
    {
        exit_code = (wait_status & 0xff00 as libc::c_int) >> 8 as libc::c_int;
    }
    exit(exit_code);
}
pub unsafe extern "C" fn sys_child_open_for_compress() -> pid_t {
    let mut parent_pipe: [libc::c_int; 2] = [0; 2];
    let mut child_pipe: [libc::c_int; 2] = [0; 2];
    let mut grandchild_pid: pid_t = 0;
    let mut child_pid: pid_t = 0;
    signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    xpipe(parent_pipe.as_mut_ptr());
    child_pid = xfork();
    if child_pid > 0 as libc::c_int {
        archive = parent_pipe[1 as libc::c_int as usize];
        xclose(parent_pipe[0 as libc::c_int as usize]);
        return child_pid;
    }
    set_program_name(
        dcgettext(
            0 as *const libc::c_char,
            b"tar (child)\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    signal(13 as libc::c_int, None);
    xdup2(parent_pipe[0 as libc::c_int as usize], 0 as libc::c_int);
    xclose(parent_pipe[1 as libc::c_int as usize]);
    if !(!force_local_option
        && {
            rmt_dev_name__ = strchr(
                *archive_name_array.offset(0 as libc::c_int as isize),
                ':' as i32,
            );
            !rmt_dev_name__.is_null()
        } && rmt_dev_name__ > *archive_name_array.offset(0 as libc::c_int as isize)
        && (memchr(
            *archive_name_array.offset(0 as libc::c_int as isize) as *const libc::c_void,
            '/' as i32,
            rmt_dev_name__
                .offset_from(*archive_name_array.offset(0 as libc::c_int as isize))
                as libc::c_long as libc::c_ulong,
        ))
            .is_null())
        && is_regular_file(*archive_name_array.offset(0 as libc::c_int as isize)) != 0
    {
        if backup_option {
            maybe_backup_file(
                *archive_name_array.offset(0 as libc::c_int as isize),
                1 as libc::c_int != 0,
            );
        }
        if strcmp(
            *archive_name_array.offset(0 as libc::c_int as isize),
            b"-\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            archive = creat(
                *archive_name_array.offset(0 as libc::c_int as isize),
                (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                    | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                        | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int))
                    as mode_t,
            );
            if archive < 0 as libc::c_int {
                let mut saved_errno: libc::c_int = *__errno_location();
                if backup_option {
                    undo_last_backup();
                }
                *__errno_location() = saved_errno;
                open_fatal(*archive_name_array.offset(0 as libc::c_int as isize));
            }
            xdup2(archive, 1 as libc::c_int);
        }
        priv_set_restore_linkdir();
        xexec(use_compress_program_option);
    }
    xpipe(child_pipe.as_mut_ptr());
    grandchild_pid = xfork();
    if grandchild_pid == 0 as libc::c_int {
        set_program_name(
            dcgettext(
                0 as *const libc::c_char,
                b"tar (grandchild)\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        xdup2(child_pipe[1 as libc::c_int as usize], 1 as libc::c_int);
        xclose(child_pipe[0 as libc::c_int as usize]);
        priv_set_restore_linkdir();
        xexec(use_compress_program_option);
    }
    xdup2(child_pipe[0 as libc::c_int as usize], 0 as libc::c_int);
    xclose(child_pipe[1 as libc::c_int as usize]);
    if strcmp(
        *archive_name_array.offset(0 as libc::c_int as isize),
        b"-\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        archive = 1 as libc::c_int;
    } else {
        archive = if !force_local_option
            && {
                rmt_dev_name__ = strchr(
                    *archive_name_array.offset(0 as libc::c_int as isize),
                    ':' as i32,
                );
                !rmt_dev_name__.is_null()
            } && rmt_dev_name__ > *archive_name_array.offset(0 as libc::c_int as isize)
            && (memchr(
                *archive_name_array.offset(0 as libc::c_int as isize)
                    as *const libc::c_void,
                '/' as i32,
                rmt_dev_name__
                    .offset_from(*archive_name_array.offset(0 as libc::c_int as isize))
                    as libc::c_long as libc::c_ulong,
            ))
                .is_null()
        {
            rmt_open__(
                *archive_name_array.offset(0 as libc::c_int as isize),
                0o100 as libc::c_int | 0o1 as libc::c_int,
                (1 as libc::c_int) << 30 as libc::c_int,
                rsh_command_option,
            )
        } else {
            creat(
                *archive_name_array.offset(0 as libc::c_int as isize),
                (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                    | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                        | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int))
                    as mode_t,
            )
        };
        if archive < 0 as libc::c_int {
            open_fatal(*archive_name_array.offset(0 as libc::c_int as isize));
        }
    }
    loop {
        let mut status: size_t = 0 as libc::c_int as size_t;
        let mut cursor: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut length: size_t = 0;
        length = 0 as libc::c_int as size_t;
        cursor = ((*record_start).buffer).as_mut_ptr();
        while length < record_size {
            let mut size: size_t = record_size.wrapping_sub(length);
            status = safe_read(0 as libc::c_int, cursor as *mut libc::c_void, size);
            if status == -(1 as libc::c_int) as size_t {
                read_fatal(use_compress_program_option);
            }
            if status == 0 as libc::c_int as libc::c_ulong {
                break;
            }
            length = (length as libc::c_ulong).wrapping_add(status) as size_t as size_t;
            cursor = cursor.offset(status as isize);
        }
        if status == 0 as libc::c_int as libc::c_ulong {
            if length > 0 as libc::c_int as libc::c_ulong {
                memset(
                    ((*record_start).buffer).as_mut_ptr().offset(length as isize)
                        as *mut libc::c_void,
                    0 as libc::c_int,
                    record_size.wrapping_sub(length),
                );
                status = sys_write_archive_buffer();
                if status != record_size {
                    archive_write_error(status as ssize_t);
                }
            }
            break;
        } else {
            status = sys_write_archive_buffer();
            if status != record_size {
                archive_write_error(status as ssize_t);
            }
        }
    }
    wait_for_grandchild(grandchild_pid);
}
unsafe extern "C" fn run_decompress_program() {
    let mut i: libc::c_int = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut prog: *const libc::c_char = 0 as *const libc::c_char;
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
    let mut wsflags: libc::c_int = (0x40 as libc::c_int | 0x4 as libc::c_int
        | (0x200 as libc::c_int | 0x400 as libc::c_int) | 0x800 as libc::c_int
        | 0x2000000 as libc::c_int | 0x80000 as libc::c_int | 0x2 as libc::c_int)
        & !(0x40 as libc::c_int);
    ws.ws_env = environ as *mut *const libc::c_char;
    ws.ws_offs = 1 as libc::c_int as size_t;
    p = first_decompress_program(&mut i);
    while !p.is_null() {
        if !prog.is_null() {
            if warning_option & 0x80000 as libc::c_int != 0 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot run %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    prog,
                );
            }
            if warning_option & 0x80000 as libc::c_int != 0 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"trying %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    p,
                );
            }
        }
        if wordsplit(p, &mut ws, wsflags as libc::c_uint) != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot split string '%s': %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                p,
                wordsplit_strerror(&mut ws),
            );
            fatal_exit();
        }
        wsflags |= 0x8 as libc::c_int;
        memmove(
            ws.ws_wordv as *mut libc::c_void,
            (ws.ws_wordv).offset(ws.ws_offs as isize) as *const libc::c_void,
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul(ws.ws_wordc),
        );
        let ref mut fresh0 = *(ws.ws_wordv).offset(ws.ws_wordc as isize);
        *fresh0 = b"-d\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        prog = p;
        execvp(
            *(ws.ws_wordv).offset(0 as libc::c_int as isize),
            ws.ws_wordv as *const *mut libc::c_char,
        );
        let ref mut fresh1 = *(ws.ws_wordv).offset(ws.ws_wordc as isize);
        *fresh1 = 0 as *mut libc::c_char;
        p = next_decompress_program(&mut i);
    }
    if prog.is_null() {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"unable to run decompression program\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fatal_exit();
    }
    exec_fatal(prog);
}
pub unsafe extern "C" fn sys_child_open_for_uncompress() -> pid_t {
    let mut parent_pipe: [libc::c_int; 2] = [0; 2];
    let mut child_pipe: [libc::c_int; 2] = [0; 2];
    let mut grandchild_pid: pid_t = 0;
    let mut child_pid: pid_t = 0;
    xpipe(parent_pipe.as_mut_ptr());
    child_pid = xfork();
    if child_pid > 0 as libc::c_int {
        archive = parent_pipe[0 as libc::c_int as usize];
        xclose(parent_pipe[1 as libc::c_int as usize]);
        return child_pid;
    }
    set_program_name(
        dcgettext(
            0 as *const libc::c_char,
            b"tar (child)\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    signal(13 as libc::c_int, None);
    xdup2(parent_pipe[1 as libc::c_int as usize], 1 as libc::c_int);
    xclose(parent_pipe[0 as libc::c_int as usize]);
    if strcmp(
        *archive_name_array.offset(0 as libc::c_int as isize),
        b"-\0" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int
        && !(!force_local_option
            && {
                rmt_dev_name__ = strchr(
                    *archive_name_array.offset(0 as libc::c_int as isize),
                    ':' as i32,
                );
                !rmt_dev_name__.is_null()
            } && rmt_dev_name__ > *archive_name_array.offset(0 as libc::c_int as isize)
            && (memchr(
                *archive_name_array.offset(0 as libc::c_int as isize)
                    as *const libc::c_void,
                '/' as i32,
                rmt_dev_name__
                    .offset_from(*archive_name_array.offset(0 as libc::c_int as isize))
                    as libc::c_long as libc::c_ulong,
            ))
                .is_null())
        && is_regular_file(*archive_name_array.offset(0 as libc::c_int as isize)) != 0
    {
        archive = open(
            *archive_name_array.offset(0 as libc::c_int as isize),
            0 as libc::c_int | 0 as libc::c_int,
            0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int),
        );
        if archive < 0 as libc::c_int {
            open_fatal(*archive_name_array.offset(0 as libc::c_int as isize));
        }
        xdup2(archive, 0 as libc::c_int);
        priv_set_restore_linkdir();
        run_decompress_program();
    }
    xpipe(child_pipe.as_mut_ptr());
    grandchild_pid = xfork();
    if grandchild_pid == 0 as libc::c_int {
        set_program_name(
            dcgettext(
                0 as *const libc::c_char,
                b"tar (grandchild)\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        xdup2(child_pipe[0 as libc::c_int as usize], 0 as libc::c_int);
        xclose(child_pipe[1 as libc::c_int as usize]);
        priv_set_restore_linkdir();
        run_decompress_program();
    }
    xdup2(child_pipe[1 as libc::c_int as usize], 1 as libc::c_int);
    xclose(child_pipe[0 as libc::c_int as usize]);
    if strcmp(
        *archive_name_array.offset(0 as libc::c_int as isize),
        b"-\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        archive = 0 as libc::c_int;
    } else {
        archive = if !force_local_option
            && {
                rmt_dev_name__ = strchr(
                    *archive_name_array.offset(0 as libc::c_int as isize),
                    ':' as i32,
                );
                !rmt_dev_name__.is_null()
            } && rmt_dev_name__ > *archive_name_array.offset(0 as libc::c_int as isize)
            && (memchr(
                *archive_name_array.offset(0 as libc::c_int as isize)
                    as *const libc::c_void,
                '/' as i32,
                rmt_dev_name__
                    .offset_from(*archive_name_array.offset(0 as libc::c_int as isize))
                    as libc::c_long as libc::c_ulong,
            ))
                .is_null()
        {
            rmt_open__(
                *archive_name_array.offset(0 as libc::c_int as isize),
                0 as libc::c_int | 0 as libc::c_int,
                (1 as libc::c_int) << 30 as libc::c_int,
                rsh_command_option,
            )
        } else {
            open(
                *archive_name_array.offset(0 as libc::c_int as isize),
                0 as libc::c_int | 0 as libc::c_int,
                0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                    | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                        | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int),
            )
        };
    }
    if archive < 0 as libc::c_int {
        open_fatal(*archive_name_array.offset(0 as libc::c_int as isize));
    }
    loop {
        let mut cursor: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut maximum: size_t = 0;
        let mut count: size_t = 0;
        let mut status: size_t = 0;
        clear_read_error_count();
        loop {
            status = if archive >= (1 as libc::c_int) << 30 as libc::c_int {
                rmt_read__(
                    archive - ((1 as libc::c_int) << 30 as libc::c_int),
                    ((*record_start).buffer).as_mut_ptr(),
                    record_size,
                )
            } else {
                safe_read(
                    archive,
                    ((*record_start).buffer).as_mut_ptr() as *mut libc::c_void,
                    record_size,
                )
            };
            if !(status == -(1 as libc::c_int) as size_t) {
                break;
            }
            archive_read_error();
        }
        if status == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        cursor = ((*record_start).buffer).as_mut_ptr();
        maximum = status;
        while maximum != 0 {
            count = if maximum < 512 as libc::c_int as libc::c_ulong {
                maximum
            } else {
                512 as libc::c_int as libc::c_ulong
            };
            if full_write(1 as libc::c_int, cursor as *const libc::c_void, count)
                != count
            {
                write_error(use_compress_program_option);
            }
            cursor = cursor.offset(count as isize);
            maximum = (maximum as libc::c_ulong).wrapping_sub(count) as size_t as size_t;
        }
    }
    xclose(1 as libc::c_int);
    wait_for_grandchild(grandchild_pid);
}
unsafe extern "C" fn dec_to_env(mut envar: *const libc::c_char, mut num: uintmax_t) {
    let mut buf: [libc::c_char; 21] = [0; 21];
    let mut numstr: *mut libc::c_char = 0 as *mut libc::c_char;
    numstr = umaxtostr(num, buf.as_mut_ptr());
    if setenv(envar, numstr, 1 as libc::c_int) != 0 as libc::c_int {
        xalloc_die();
    }
}
unsafe extern "C" fn time_to_env(mut envar: *const libc::c_char, mut t: timespec) {
    let mut buf: [libc::c_char; 32] = [0; 32];
    if setenv(envar, code_timespec(t, buf.as_mut_ptr()), 1 as libc::c_int)
        != 0 as libc::c_int
    {
        xalloc_die();
    }
}
unsafe extern "C" fn oct_to_env(mut envar: *const libc::c_char, mut num: libc::c_ulong) {
    let mut buf: [libc::c_char; 24] = [0; 24];
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong,
        b"0%lo\0" as *const u8 as *const libc::c_char,
        num,
    );
    if setenv(envar, buf.as_mut_ptr(), 1 as libc::c_int) != 0 as libc::c_int {
        xalloc_die();
    }
}
unsafe extern "C" fn str_to_env(
    mut envar: *const libc::c_char,
    mut str: *const libc::c_char,
) {
    if !str.is_null() {
        if setenv(envar, str, 1 as libc::c_int) != 0 as libc::c_int {
            xalloc_die();
        }
    } else {
        unsetenv(envar);
    };
}
unsafe extern "C" fn chr_to_env(mut envar: *const libc::c_char, mut c: libc::c_char) {
    let mut buf: [libc::c_char; 2] = [0; 2];
    buf[0 as libc::c_int as usize] = c;
    buf[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if setenv(envar, buf.as_mut_ptr(), 1 as libc::c_int) != 0 as libc::c_int {
        xalloc_die();
    }
}
unsafe extern "C" fn stat_to_env(
    mut name: *mut libc::c_char,
    mut type_0: libc::c_char,
    mut st: *mut tar_stat_info,
) {
    str_to_env(
        b"TAR_VERSION\0" as *const u8 as *const libc::c_char,
        b"1.34\0" as *const u8 as *const libc::c_char,
    );
    str_to_env(
        b"TAR_ARCHIVE\0" as *const u8 as *const libc::c_char,
        *archive_name_cursor,
    );
    dec_to_env(
        b"TAR_VOLUME\0" as *const u8 as *const libc::c_char,
        (archive_name_cursor.offset_from(archive_name_array) as libc::c_long
            + 1 as libc::c_int as libc::c_long) as uintmax_t,
    );
    dec_to_env(
        b"TAR_BLOCKING_FACTOR\0" as *const u8 as *const libc::c_char,
        blocking_factor as uintmax_t,
    );
    str_to_env(
        b"TAR_FORMAT\0" as *const u8 as *const libc::c_char,
        archive_format_string(
            (if current_format as libc::c_uint
                == DEFAULT_FORMAT as libc::c_int as libc::c_uint
            {
                archive_format as libc::c_uint
            } else {
                current_format as libc::c_uint
            }) as archive_format,
        ),
    );
    chr_to_env(b"TAR_FILETYPE\0" as *const u8 as *const libc::c_char, type_0);
    oct_to_env(
        b"TAR_MODE\0" as *const u8 as *const libc::c_char,
        (*st).stat.st_mode as libc::c_ulong,
    );
    str_to_env(b"TAR_FILENAME\0" as *const u8 as *const libc::c_char, name);
    str_to_env(b"TAR_REALNAME\0" as *const u8 as *const libc::c_char, (*st).file_name);
    str_to_env(b"TAR_UNAME\0" as *const u8 as *const libc::c_char, (*st).uname);
    str_to_env(b"TAR_GNAME\0" as *const u8 as *const libc::c_char, (*st).gname);
    time_to_env(b"TAR_ATIME\0" as *const u8 as *const libc::c_char, (*st).atime);
    time_to_env(b"TAR_MTIME\0" as *const u8 as *const libc::c_char, (*st).mtime);
    time_to_env(b"TAR_CTIME\0" as *const u8 as *const libc::c_char, (*st).ctime);
    dec_to_env(
        b"TAR_SIZE\0" as *const u8 as *const libc::c_char,
        (*st).stat.st_size as uintmax_t,
    );
    dec_to_env(
        b"TAR_UID\0" as *const u8 as *const libc::c_char,
        (*st).stat.st_uid as uintmax_t,
    );
    dec_to_env(
        b"TAR_GID\0" as *const u8 as *const libc::c_char,
        (*st).stat.st_gid as uintmax_t,
    );
    match type_0 as libc::c_int {
        98 | 99 => {
            dec_to_env(
                b"TAR_MINOR\0" as *const u8 as *const libc::c_char,
                gnu_dev_minor((*st).stat.st_rdev) as uintmax_t,
            );
            dec_to_env(
                b"TAR_MAJOR\0" as *const u8 as *const libc::c_char,
                gnu_dev_major((*st).stat.st_rdev) as uintmax_t,
            );
            unsetenv(b"TAR_LINKNAME\0" as *const u8 as *const libc::c_char);
        }
        108 | 104 => {
            unsetenv(b"TAR_MINOR\0" as *const u8 as *const libc::c_char);
            unsetenv(b"TAR_MAJOR\0" as *const u8 as *const libc::c_char);
            str_to_env(
                b"TAR_LINKNAME\0" as *const u8 as *const libc::c_char,
                (*st).link_name,
            );
        }
        _ => {
            unsetenv(b"TAR_MINOR\0" as *const u8 as *const libc::c_char);
            unsetenv(b"TAR_MAJOR\0" as *const u8 as *const libc::c_char);
            unsetenv(b"TAR_LINKNAME\0" as *const u8 as *const libc::c_char);
        }
    };
}
static mut global_pid: pid_t = 0;
static mut pipe_handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
pub unsafe extern "C" fn sys_exec_command(
    mut file_name: *mut libc::c_char,
    mut typechar: libc::c_int,
    mut st: *mut tar_stat_info,
) -> libc::c_int {
    let mut p: [libc::c_int; 2] = [0; 2];
    xpipe(p.as_mut_ptr());
    pipe_handler = signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    global_pid = xfork();
    if global_pid != 0 as libc::c_int {
        xclose(p[0 as libc::c_int as usize]);
        return p[1 as libc::c_int as usize];
    }
    xdup2(p[0 as libc::c_int as usize], 0 as libc::c_int);
    xclose(p[1 as libc::c_int as usize]);
    stat_to_env(file_name, typechar as libc::c_char, st);
    priv_set_restore_linkdir();
    xexec(to_command_option);
    panic!();
}
pub unsafe extern "C" fn sys_wait_command() {
    let mut status: libc::c_int = 0;
    if global_pid < 0 as libc::c_int {
        return;
    }
    signal(13 as libc::c_int, pipe_handler);
    while waitpid(global_pid, &mut status, 0 as libc::c_int) == -(1 as libc::c_int) {
        if *__errno_location() != 4 as libc::c_int {
            global_pid = -(1 as libc::c_int);
            waitpid_error(to_command_option);
            return;
        }
    }
    if status & 0x7f as libc::c_int == 0 as libc::c_int {
        if !ignore_command_error_option
            && (status & 0xff00 as libc::c_int) >> 8 as libc::c_int != 0
        {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%lu: Child returned status %d\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                global_pid as libc::c_ulong,
                (status & 0xff00 as libc::c_int) >> 8 as libc::c_int,
            );
            exit_status = 2 as libc::c_int;
        }
    } else if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
        as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
    {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%lu: Child terminated on signal %d\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            global_pid as libc::c_ulong,
            status & 0x7f as libc::c_int,
        );
    } else {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%lu: Child terminated on unknown reason\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            global_pid as libc::c_ulong,
        );
        exit_status = 2 as libc::c_int;
    }
    global_pid = -(1 as libc::c_int);
}
pub unsafe extern "C" fn sys_exec_info_script(
    mut archive_name: *mut *const libc::c_char,
    mut volume_number: libc::c_int,
) -> libc::c_int {
    let mut pid: pid_t = 0;
    let mut uintbuf: [libc::c_char; 21] = [0; 21];
    let mut p: [libc::c_int; 2] = [0; 2];
    static mut saved_handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
    xpipe(p.as_mut_ptr());
    saved_handler = signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    pid = xfork();
    if pid != 0 as libc::c_int {
        let mut rc: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut size: size_t = 0 as libc::c_int as size_t;
        let mut fp: *mut FILE = 0 as *mut FILE;
        xclose(p[1 as libc::c_int as usize]);
        fp = fdopen(
            p[0 as libc::c_int as usize],
            b"r\0" as *const u8 as *const libc::c_char,
        );
        rc = getline(&mut buf, &mut size, fp) as libc::c_int;
        fclose(fp);
        if rc > 0 as libc::c_int
            && *buf.offset((rc - 1 as libc::c_int) as isize) as libc::c_int
                == '\n' as i32
        {
            rc -= 1;
            *buf.offset(rc as isize) = 0 as libc::c_int as libc::c_char;
        }
        while waitpid(pid, &mut status, 0 as libc::c_int) == -(1 as libc::c_int) {
            if *__errno_location() != 4 as libc::c_int {
                signal(13 as libc::c_int, saved_handler);
                waitpid_error(info_script_option);
                return -(1 as libc::c_int);
            }
        }
        signal(13 as libc::c_int, saved_handler);
        if status & 0x7f as libc::c_int == 0 as libc::c_int {
            if (status & 0xff00 as libc::c_int) >> 8 as libc::c_int == 0 as libc::c_int
                && rc > 0 as libc::c_int
            {
                *archive_name = buf;
            } else {
                rpl_free(buf as *mut libc::c_void);
            }
            return (status & 0xff00 as libc::c_int) >> 8 as libc::c_int;
        }
        rpl_free(buf as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    setenv(
        b"TAR_VERSION\0" as *const u8 as *const libc::c_char,
        b"1.34\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    setenv(
        b"TAR_ARCHIVE\0" as *const u8 as *const libc::c_char,
        *archive_name,
        1 as libc::c_int,
    );
    setenv(
        b"TAR_VOLUME\0" as *const u8 as *const libc::c_char,
        umaxtostr(volume_number as uintmax_t, uintbuf.as_mut_ptr()),
        1 as libc::c_int,
    );
    setenv(
        b"TAR_BLOCKING_FACTOR\0" as *const u8 as *const libc::c_char,
        umaxtostr(blocking_factor as uintmax_t, uintbuf.as_mut_ptr()),
        1 as libc::c_int,
    );
    setenv(
        b"TAR_SUBCOMMAND\0" as *const u8 as *const libc::c_char,
        subcommand_string(subcommand_option),
        1 as libc::c_int,
    );
    setenv(
        b"TAR_FORMAT\0" as *const u8 as *const libc::c_char,
        archive_format_string(
            (if current_format as libc::c_uint
                == DEFAULT_FORMAT as libc::c_int as libc::c_uint
            {
                archive_format as libc::c_uint
            } else {
                current_format as libc::c_uint
            }) as archive_format,
        ),
        1 as libc::c_int,
    );
    setenv(
        b"TAR_FD\0" as *const u8 as *const libc::c_char,
        umaxtostr(p[1 as libc::c_int as usize] as uintmax_t, uintbuf.as_mut_ptr()),
        1 as libc::c_int,
    );
    xclose(p[0 as libc::c_int as usize]);
    priv_set_restore_linkdir();
    xexec(info_script_option);
    panic!();
}
pub unsafe extern "C" fn sys_exec_checkpoint_script(
    mut script_name: *const libc::c_char,
    mut archive_name: *const libc::c_char,
    mut checkpoint_number: libc::c_int,
) {
    let mut pid: pid_t = 0;
    let mut uintbuf: [libc::c_char; 21] = [0; 21];
    pid = xfork();
    if pid != 0 as libc::c_int {
        let mut status: libc::c_int = 0;
        while waitpid(pid, &mut status, 0 as libc::c_int) == -(1 as libc::c_int) {
            if !(*__errno_location() != 4 as libc::c_int) {
                continue;
            }
            waitpid_error(script_name);
            break;
        }
        return;
    }
    setenv(
        b"TAR_VERSION\0" as *const u8 as *const libc::c_char,
        b"1.34\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    setenv(
        b"TAR_ARCHIVE\0" as *const u8 as *const libc::c_char,
        archive_name,
        1 as libc::c_int,
    );
    setenv(
        b"TAR_CHECKPOINT\0" as *const u8 as *const libc::c_char,
        umaxtostr(checkpoint_number as uintmax_t, uintbuf.as_mut_ptr()),
        1 as libc::c_int,
    );
    setenv(
        b"TAR_BLOCKING_FACTOR\0" as *const u8 as *const libc::c_char,
        umaxtostr(blocking_factor as uintmax_t, uintbuf.as_mut_ptr()),
        1 as libc::c_int,
    );
    setenv(
        b"TAR_SUBCOMMAND\0" as *const u8 as *const libc::c_char,
        subcommand_string(subcommand_option),
        1 as libc::c_int,
    );
    setenv(
        b"TAR_FORMAT\0" as *const u8 as *const libc::c_char,
        archive_format_string(
            (if current_format as libc::c_uint
                == DEFAULT_FORMAT as libc::c_int as libc::c_uint
            {
                archive_format as libc::c_uint
            } else {
                current_format as libc::c_uint
            }) as archive_format,
        ),
        1 as libc::c_int,
    );
    priv_set_restore_linkdir();
    xexec(script_name);
}
