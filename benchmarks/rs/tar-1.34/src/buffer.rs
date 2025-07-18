use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type exclist;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn creat(__file: *const libc::c_char, __mode: mode_t) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fatal_exit() -> !;
    fn write_error_details(_: *const libc::c_char, _: size_t, _: size_t);
    fn write_error(_: *const libc::c_char);
    fn abort() -> !;
    fn rpl_free(ptr: *mut libc::c_void);
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn read_error(_: *const libc::c_char);
    fn strip_trailing_slashes(file: *mut libc::c_char) -> bool;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xmalloc(s: size_t) -> *mut libc::c_void;
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
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    static mut error_hook: Option::<unsafe extern "C" fn() -> ()>;
    static mut exit_status: libc::c_int;
    fn close_error(_: *const libc::c_char);
    fn open_error(_: *const libc::c_char);
    fn open_fatal(_: *const libc::c_char) -> !;
    fn open_warn(_: *const libc::c_char);
    fn getuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn human_readable(
        _: uintmax_t,
        _: *mut libc::c_char,
        _: libc::c_int,
        _: uintmax_t,
        _: uintmax_t,
    ) -> *mut libc::c_char;
    fn quotearg_colon(arg: *const libc::c_char) -> *mut libc::c_char;
    fn gettime(_: *mut timespec);
    static mut subcommand_option: subcommand;
    static mut archive_format: archive_format;
    static mut blocking_factor: libc::c_int;
    static mut record_size: size_t;
    static mut backup_option: bool;
    static mut use_compress_program_option: *const libc::c_char;
    static mut info_script_option: *const libc::c_char;
    static mut multi_volume_option: bool;
    static mut read_full_records_option: bool;
    static mut rsh_command_option: *const libc::c_char;
    static mut tape_length_option: tarlong;
    static mut totals_option: bool;
    static mut restrict_option: bool;
    static mut verbose_option: libc::c_int;
    static mut verify_option: bool;
    static mut volno_file_option: *const libc::c_char;
    static mut volume_label_option: *const libc::c_char;
    static mut archive: libc::c_int;
    static mut dev_null_output: bool;
    static mut start_time: timespec;
    static mut volume_start_time: timespec;
    static mut last_stat_time: timespec;
    static mut current_stat_info: tar_stat_info;
    static mut archive_name_array: *mut *const libc::c_char;
    static mut archive_names: size_t;
    static mut archive_name_cursor: *mut *const libc::c_char;
    static mut index_file_name: *const libc::c_char;
    static mut seek_option: libc::c_int;
    static mut seekable_archive: bool;
    fn verify_volume();
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn finish_header(st: *mut tar_stat_info, header: *mut block, block_ordinal: off_t);
    fn time_to_chars(t: time_t, buf: *mut libc::c_char, size: size_t) -> bool;
    fn assign_string(dest: *mut *mut libc::c_char, src: *const libc::c_char);
    fn off_from_header(buf: *const libc::c_char, size: size_t) -> off_t;
    fn assign_string_n(
        string: *mut *mut libc::c_char,
        value: *const libc::c_char,
        n: size_t,
    );
    fn undo_last_backup();
    fn tar_checksum(header: *mut block, silent: bool) -> read_header;
    fn maybe_backup_file(
        file_name: *const libc::c_char,
        this_is_the_archive: bool,
    ) -> bool;
    fn write_extended(
        global: bool,
        st: *mut tar_stat_info,
        old_header: *mut block,
    ) -> *mut block;
    fn simple_finish_header(header: *mut block);
    fn start_header(st: *mut tar_stat_info) -> *mut block;
    fn off_to_chars(off: off_t, buf: *mut libc::c_char, size: size_t) -> bool;
    static mut now_verifying: bool;
    fn read_header(
        return_block: *mut *mut block,
        info: *mut tar_stat_info,
        m: read_header_mode,
    ) -> read_header;
    fn uintmax_from_header(buf: *const libc::c_char, size: size_t) -> uintmax_t;
    static mut current_header: *mut block;
    fn tar_stat_destroy(st: *mut tar_stat_info);
    fn stat_diag(name: *const libc::c_char);
    fn sys_wait_for_child(_: pid_t, _: bool);
    fn xheader_write_global(xhdr: *mut xheader);
    static mut output_start: *mut libc::c_char;
    fn sys_write_archive_buffer() -> size_t;
    fn sys_spawn_shell();
    fn xheader_format_name(
        st: *mut tar_stat_info,
        fmt: *const libc::c_char,
        n: size_t,
    ) -> *mut libc::c_char;
    fn page_aligned_alloc(
        ptr: *mut *mut libc::c_void,
        size: size_t,
    ) -> *mut libc::c_void;
    fn sys_child_open_for_compress() -> pid_t;
    fn sys_child_open_for_uncompress() -> pid_t;
    fn xheader_store(
        keyword: *const libc::c_char,
        st: *mut tar_stat_info,
        data: *const libc::c_void,
    );
    fn sys_get_archive_stat() -> bool;
    fn sys_detect_dev_null_output();
    fn sys_save_archive_dev_ino();
    fn tar_stat_init(st: *mut tar_stat_info);
    fn xheader_read(xhdr: *mut xheader, header: *mut block, size: off_t);
    fn xheader_decode(stat: *mut tar_stat_info);
    fn checkpoint_run(do_write: bool);
    static mut warning_option: libc::c_int;
    fn sys_exec_info_script(
        archive_name: *mut *const libc::c_char,
        volume_number: libc::c_int,
    ) -> libc::c_int;
    fn set_compression_program_by_suffix(
        name: *const libc::c_char,
        defprog: *const libc::c_char,
    );
    static mut rmt_dev_name__: *const libc::c_char;
    fn rmt_open__(
        _: *const libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> libc::c_int;
    fn rmt_close__(_: libc::c_int) -> libc::c_int;
    fn rmt_read__(_: libc::c_int, _: *mut libc::c_char, _: size_t) -> size_t;
    fn rmt_lseek__(_: libc::c_int, _: off_t, _: libc::c_int) -> off_t;
    fn rmt_ioctl__(_: libc::c_int, _: libc::c_int, _: *mut libc::c_char) -> libc::c_int;
    static mut force_local_option: bool;
    static mut records_skipped: off_t;
    static mut time_to_start_writing: bool;
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
pub type mode_t = __mode_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
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
pub struct mtop {
    pub mt_op: libc::c_short,
    pub mt_count: libc::c_int,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const human_B: C2RustUnnamed_3 = 256;
pub const human_SI: C2RustUnnamed_3 = 128;
pub const human_space_before_unit: C2RustUnnamed_3 = 64;
pub const human_base_1024: C2RustUnnamed_3 = 32;
pub const human_autoscale: C2RustUnnamed_3 = 16;
pub const human_suppress_point_zero: C2RustUnnamed_3 = 8;
pub const human_group_digits: C2RustUnnamed_3 = 4;
pub const human_floor: C2RustUnnamed_3 = 2;
pub const human_round_to_nearest: C2RustUnnamed_3 = 1;
pub const human_ceiling: C2RustUnnamed_3 = 0;
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
pub type access_mode = libc::c_uint;
pub const ACCESS_UPDATE: access_mode = 2;
pub const ACCESS_WRITE: access_mode = 1;
pub const ACCESS_READ: access_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bufmap {
    pub next: *mut bufmap,
    pub start: size_t,
    pub file_name: *mut libc::c_char,
    pub sizetotal: off_t,
    pub sizeleft: off_t,
    pub nblocks: size_t,
}
pub const ct_tar: compress_type = 1;
pub const ct_none: compress_type = 0;
pub type compress_type = libc::c_uint;
pub const ct_zstd: compress_type = 9;
pub const ct_xz: compress_type = 8;
pub const ct_lzop: compress_type = 7;
pub const ct_lzma: compress_type = 6;
pub const ct_lzip: compress_type = 5;
pub const ct_bzip2: compress_type = 4;
pub const ct_gzip: compress_type = 3;
pub const ct_compress: compress_type = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zip_magic {
    pub type_0: compress_type,
    pub length: size_t,
    pub magic: *const libc::c_char,
}
pub const HEADER_SUCCESS: read_header = 1;
pub type read_header = libc::c_uint;
pub const HEADER_FAILURE: read_header = 5;
pub const HEADER_END_OF_FILE: read_header = 4;
pub const HEADER_ZERO_BLOCK: read_header = 3;
pub const HEADER_SUCCESS_EXTENDED: read_header = 2;
pub const HEADER_STILL_UNREAD: read_header = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zip_program {
    pub type_0: compress_type,
    pub program: *const libc::c_char,
    pub option: *const libc::c_char,
}
pub type read_header_mode = libc::c_uint;
pub const read_header_x_global: read_header_mode = 2;
pub const read_header_x_raw: read_header_mode = 1;
pub const read_header_auto: read_header_mode = 0;
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
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
static mut prev_written: tarlong = 0.;
static mut bytes_written: tarlong = 0.;
static mut record_buffer: [*mut libc::c_void; 2] = [0 as *const libc::c_void
    as *mut libc::c_void; 2];
static mut record_buffer_aligned: [*mut block; 2] = [0 as *const block as *mut block; 2];
static mut record_index: libc::c_int = 0;
pub static mut record_start: *mut block = 0 as *const block as *mut block;
pub static mut record_end: *mut block = 0 as *const block as *mut block;
pub static mut current_block: *mut block = 0 as *const block as *mut block;
pub static mut access_mode: access_mode = ACCESS_READ;
pub static mut records_read: off_t = 0;
pub static mut records_written: off_t = 0;
static mut record_start_block: off_t = 0;
pub static mut stdlis: *mut FILE = 0 as *const FILE as *mut FILE;
static mut child_pid: pid_t = 0;
static mut read_error_count: libc::c_int = 0;
static mut hit_eof: bool = false;
static mut read_full_records: bool = 0 as libc::c_int != 0;
pub static mut write_archive_to_stdout: bool = false;
static mut flush_write_ptr: Option::<unsafe extern "C" fn(size_t) -> ()> = None;
static mut flush_read_ptr: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut volume_label: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut continued_file_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut continued_file_size: uintmax_t = 0;
pub static mut continued_file_offset: uintmax_t = 0;
static mut volno: libc::c_int = 1 as libc::c_int;
static mut global_volno: libc::c_int = 1 as libc::c_int;
static mut bufmap_head: *mut bufmap = 0 as *const bufmap as *mut bufmap;
static mut bufmap_tail: *mut bufmap = 0 as *const bufmap as *mut bufmap;
static mut inhibit_map: libc::c_int = 0;
pub unsafe extern "C" fn mv_begin_write(
    mut file_name: *const libc::c_char,
    mut totsize: off_t,
    mut sizeleft: off_t,
) {
    if multi_volume_option {
        let mut bp: *mut bufmap = xmalloc(
            ::std::mem::size_of::<bufmap>() as libc::c_ulong,
        ) as *mut bufmap;
        if !bufmap_tail.is_null() {
            (*bufmap_tail).next = bp;
        } else {
            bufmap_head = bp;
        }
        bufmap_tail = bp;
        (*bp).next = 0 as *mut bufmap;
        (*bp).start = current_block.offset_from(record_start) as libc::c_long as size_t;
        (*bp).file_name = xstrdup(file_name);
        (*bp).sizetotal = totsize;
        (*bp).sizeleft = sizeleft;
        (*bp).nblocks = 0 as libc::c_int as size_t;
    }
}
unsafe extern "C" fn bufmap_locate(mut off: size_t) -> *mut bufmap {
    let mut map: *mut bufmap = 0 as *mut bufmap;
    map = bufmap_head;
    while !map.is_null() {
        if ((*map).next).is_null()
            || off
                < ((*(*map).next).start)
                    .wrapping_mul(512 as libc::c_int as libc::c_ulong)
        {
            break;
        }
        map = (*map).next;
    }
    return map;
}
unsafe extern "C" fn bufmap_free(mut mark: *mut bufmap) {
    let mut map: *mut bufmap = 0 as *mut bufmap;
    map = bufmap_head;
    while !map.is_null() && map != mark {
        let mut next: *mut bufmap = (*map).next;
        rpl_free((*map).file_name as *mut libc::c_void);
        rpl_free(map as *mut libc::c_void);
        map = next;
    }
    bufmap_head = map;
    if bufmap_head.is_null() {
        bufmap_tail = bufmap_head;
    }
}
unsafe extern "C" fn bufmap_reset(mut map: *mut bufmap, mut fixup: ssize_t) {
    bufmap_free(map);
    if !map.is_null() {
        while !map.is_null() {
            (*map)
                .start = ((*map).start as libc::c_ulong)
                .wrapping_add(fixup as libc::c_ulong) as size_t as size_t;
            (*map).nblocks = 0 as libc::c_int as size_t;
            map = (*map).next;
        }
    }
}
static mut dummy: tar_stat_info = tar_stat_info {
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
pub unsafe extern "C" fn buffer_write_global_xheader() {
    xheader_write_global(&mut dummy.xhdr);
}
pub unsafe extern "C" fn mv_begin_read(mut st: *mut tar_stat_info) {
    mv_begin_write((*st).orig_file_name, (*st).stat.st_size, (*st).stat.st_size);
}
pub unsafe extern "C" fn mv_end() {
    if multi_volume_option {
        bufmap_free(0 as *mut bufmap);
    }
}
pub unsafe extern "C" fn mv_size_left(mut size: off_t) {
    if !bufmap_head.is_null() {
        (*bufmap_head).sizeleft = size;
    }
}
pub unsafe extern "C" fn clear_read_error_count() {
    read_error_count = 0 as libc::c_int;
}
static mut duration: libc::c_double = 0.;
pub unsafe extern "C" fn set_start_time() {
    gettime(&mut start_time);
    volume_start_time = start_time;
    last_stat_time = start_time;
}
unsafe extern "C" fn set_volume_start_time() {
    gettime(&mut volume_start_time);
    last_stat_time = volume_start_time;
}
pub unsafe extern "C" fn compute_duration() -> libc::c_double {
    let mut now: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    gettime(&mut now);
    duration
        += (now.tv_sec - last_stat_time.tv_sec) as libc::c_double
            + (now.tv_nsec - last_stat_time.tv_nsec) as libc::c_double / 1e9f64;
    gettime(&mut last_stat_time);
    return duration;
}
static mut archive_compression_type: compress_type = ct_none;
static mut magic: [zip_magic; 10] = [
    {
        let mut init = zip_magic {
            type_0: ct_none,
            length: 0 as libc::c_int as size_t,
            magic: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = zip_magic {
            type_0: ct_tar,
            length: 0 as libc::c_int as size_t,
            magic: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = zip_magic {
            type_0: ct_compress,
            length: 2 as libc::c_int as size_t,
            magic: b"\x1F\x9D\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = zip_magic {
            type_0: ct_gzip,
            length: 2 as libc::c_int as size_t,
            magic: b"\x1F\x8B\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = zip_magic {
            type_0: ct_bzip2,
            length: 3 as libc::c_int as size_t,
            magic: b"BZh\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = zip_magic {
            type_0: ct_lzip,
            length: 4 as libc::c_int as size_t,
            magic: b"LZIP\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = zip_magic {
            type_0: ct_lzma,
            length: 6 as libc::c_int as size_t,
            magic: b"\xFFLZMA\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = zip_magic {
            type_0: ct_lzop,
            length: 4 as libc::c_int as size_t,
            magic: b"\x89LZO\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = zip_magic {
            type_0: ct_xz,
            length: 6 as libc::c_int as size_t,
            magic: b"\xFD7zXZ\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = zip_magic {
            type_0: ct_zstd,
            length: 4 as libc::c_int as size_t,
            magic: b"(\xB5/\xFD\0" as *const u8 as *const libc::c_char,
        };
        init
    },
];
static mut zip_program: [zip_program; 12] = [
    {
        let mut init = zip_program {
            type_0: ct_compress,
            program: b"compress\0" as *const u8 as *const libc::c_char,
            option: b"-Z\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = zip_program {
            type_0: ct_compress,
            program: b"gzip\0" as *const u8 as *const libc::c_char,
            option: b"-z\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = zip_program {
            type_0: ct_gzip,
            program: b"gzip\0" as *const u8 as *const libc::c_char,
            option: b"-z\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = zip_program {
            type_0: ct_bzip2,
            program: b"bzip2\0" as *const u8 as *const libc::c_char,
            option: b"-j\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = zip_program {
            type_0: ct_bzip2,
            program: b"lbzip2\0" as *const u8 as *const libc::c_char,
            option: b"-j\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = zip_program {
            type_0: ct_lzip,
            program: b"lzip\0" as *const u8 as *const libc::c_char,
            option: b"--lzip\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = zip_program {
            type_0: ct_lzma,
            program: b"lzma\0" as *const u8 as *const libc::c_char,
            option: b"--lzma\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = zip_program {
            type_0: ct_lzma,
            program: b"xz\0" as *const u8 as *const libc::c_char,
            option: b"-J\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = zip_program {
            type_0: ct_lzop,
            program: b"lzop\0" as *const u8 as *const libc::c_char,
            option: b"--lzop\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = zip_program {
            type_0: ct_xz,
            program: b"xz\0" as *const u8 as *const libc::c_char,
            option: b"-J\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = zip_program {
            type_0: ct_zstd,
            program: b"zstd\0" as *const u8 as *const libc::c_char,
            option: b"--zstd\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = zip_program {
            type_0: ct_none,
            program: 0 as *const libc::c_char,
            option: 0 as *const libc::c_char,
        };
        init
    },
];
unsafe extern "C" fn find_zip_program(
    mut type_0: compress_type,
    mut pstate: *mut libc::c_int,
) -> *const zip_program {
    let mut i: libc::c_int = 0;
    i = *pstate;
    while zip_program[i as usize].type_0 as libc::c_uint
        != ct_none as libc::c_int as libc::c_uint
    {
        if zip_program[i as usize].type_0 as libc::c_uint == type_0 as libc::c_uint {
            *pstate = i + 1 as libc::c_int;
            return zip_program.as_mut_ptr().offset(i as isize);
        }
        i += 1;
        i;
    }
    *pstate = i;
    return 0 as *const zip_program;
}
pub unsafe extern "C" fn first_decompress_program(
    mut pstate: *mut libc::c_int,
) -> *const libc::c_char {
    let mut zp: *const zip_program = 0 as *const zip_program;
    if !use_compress_program_option.is_null() {
        return use_compress_program_option;
    }
    if archive_compression_type as libc::c_uint == ct_none as libc::c_int as libc::c_uint
    {
        return 0 as *const libc::c_char;
    }
    *pstate = 0 as libc::c_int;
    zp = find_zip_program(archive_compression_type, pstate);
    return if !zp.is_null() { (*zp).program } else { 0 as *const libc::c_char };
}
pub unsafe extern "C" fn next_decompress_program(
    mut pstate: *mut libc::c_int,
) -> *const libc::c_char {
    let mut zp: *const zip_program = 0 as *const zip_program;
    if !use_compress_program_option.is_null() {
        return 0 as *const libc::c_char;
    }
    zp = find_zip_program(archive_compression_type, pstate);
    return if !zp.is_null() { (*zp).program } else { 0 as *const libc::c_char };
}
unsafe extern "C" fn compress_option(mut type_0: compress_type) -> *const libc::c_char {
    let mut zp: *const zip_program = 0 as *const zip_program;
    let mut i: libc::c_int = 0 as libc::c_int;
    zp = find_zip_program(type_0, &mut i);
    return if !zp.is_null() { (*zp).option } else { 0 as *const libc::c_char };
}
unsafe extern "C" fn check_compressed_archive(mut pshort: *mut bool) -> compress_type {
    let mut p: *const zip_magic = 0 as *const zip_magic;
    let mut sfr: bool = false;
    let mut temp: bool = false;
    if pshort.is_null() {
        pshort = &mut temp;
    }
    record_end = record_start;
    sfr = read_full_records;
    read_full_records = 1 as libc::c_int != 0;
    *pshort = (find_next_block()).is_null();
    read_full_records = sfr;
    if record_start != record_end
        && (strcmp(
            ((*record_start).header.magic).as_mut_ptr(),
            b"ustar\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                ((*record_start).buffer)
                    .as_mut_ptr()
                    .offset(257 as libc::c_ulong as isize),
                b"ustar  \0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int)
        && tar_checksum(record_start, 1 as libc::c_int != 0) as libc::c_uint
            == HEADER_SUCCESS as libc::c_int as libc::c_uint
    {
        return ct_tar;
    }
    p = magic.as_ptr().offset(2 as libc::c_int as isize);
    while p
        < magic
            .as_ptr()
            .offset(
                (::std::mem::size_of::<[zip_magic; 10]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<zip_magic>() as libc::c_ulong)
                    as isize,
            )
    {
        if memcmp(
            ((*record_start).buffer).as_mut_ptr() as *const libc::c_void,
            (*p).magic as *const libc::c_void,
            (*p).length,
        ) == 0 as libc::c_int
        {
            return (*p).type_0;
        }
        p = p.offset(1);
        p;
    }
    return ct_none;
}
unsafe extern "C" fn guess_seekable_archive() {
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
    if subcommand_option as libc::c_uint
        == DELETE_SUBCOMMAND as libc::c_int as libc::c_uint
    {
        seekable_archive = 0 as libc::c_int != 0;
    }
    if seek_option != -(1 as libc::c_int) {
        seekable_archive = seek_option != 0;
        return;
    }
    if !multi_volume_option && use_compress_program_option.is_null()
        && fstat(archive, &mut st) == 0 as libc::c_int
    {
        seekable_archive = st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint;
    } else {
        seekable_archive = 0 as libc::c_int != 0;
    };
}
unsafe extern "C" fn open_compressed_archive() -> libc::c_int {
    archive = if !force_local_option
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
    if archive == -(1 as libc::c_int) {
        return archive;
    }
    if !multi_volume_option {
        if use_compress_program_option.is_null() {
            let mut shortfile: bool = false;
            let mut type_0: compress_type = check_compressed_archive(&mut shortfile);
            match type_0 as libc::c_uint {
                1 => {
                    if shortfile {
                        if error_hook.is_some() {
                            error_hook.unwrap()();
                        }
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"This does not look like a tar archive\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        exit_status = 2 as libc::c_int;
                    }
                    return archive;
                }
                0 => {
                    if shortfile {
                        if error_hook.is_some() {
                            error_hook.unwrap()();
                        }
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"This does not look like a tar archive\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        exit_status = 2 as libc::c_int;
                    }
                    set_compression_program_by_suffix(
                        *archive_name_array.offset(0 as libc::c_int as isize),
                        0 as *const libc::c_char,
                    );
                    if use_compress_program_option.is_null() {
                        return archive;
                    }
                }
                _ => {
                    archive_compression_type = type_0;
                }
            }
        }
        if archive >= (1 as libc::c_int) << 30 as libc::c_int {
            rmt_close__(archive - ((1 as libc::c_int) << 30 as libc::c_int));
        } else {
            close(archive);
        };
        hit_eof = 0 as libc::c_int != 0;
        child_pid = sys_child_open_for_uncompress();
        read_full_records = 1 as libc::c_int != 0;
    }
    records_read = 0 as libc::c_int as off_t;
    record_end = record_start;
    return archive;
}
unsafe extern "C" fn print_stats(
    mut fp: *mut FILE,
    mut text: *const libc::c_char,
    mut numbytes: tarlong,
) -> libc::c_int {
    let mut abbr: [libc::c_char; 652] = [0; 652];
    let mut rate: [libc::c_char; 652] = [0; 652];
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut human_opts: libc::c_int = human_autoscale as libc::c_int
        | human_base_1024 as libc::c_int | human_SI as libc::c_int
        | human_B as libc::c_int;
    if !text.is_null() && *text.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        n
            += fprintf(
                fp,
                b"%s: \0" as *const u8 as *const libc::c_char,
                dcgettext(0 as *const libc::c_char, text, 5 as libc::c_int),
            );
    }
    return n
        + fprintf(
            fp,
            b"%.0f (%s, %s/s)\0" as *const u8 as *const libc::c_char,
            numbytes,
            human_readable(
                numbytes as uintmax_t,
                abbr.as_mut_ptr(),
                human_opts,
                1 as libc::c_int as uintmax_t,
                1 as libc::c_int as uintmax_t,
            ),
            if (0 as libc::c_int as libc::c_double) < duration
                && numbytes / duration
                    < -(1 as libc::c_int) as uintmax_t as libc::c_double
            {
                human_readable(
                    (numbytes / duration) as uintmax_t,
                    rate.as_mut_ptr(),
                    human_opts,
                    1 as libc::c_int as uintmax_t,
                    1 as libc::c_int as uintmax_t,
                ) as *const libc::c_char
            } else {
                b"?\0" as *const u8 as *const libc::c_char
            },
        );
}
pub unsafe extern "C" fn format_total_stats(
    mut fp: *mut FILE,
    mut formats: *const *const libc::c_char,
    mut eor: libc::c_int,
    mut eol: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    match subcommand_option as libc::c_uint {
        3 | 2 | 8 | 1 => {
            n = print_stats(
                fp,
                *formats.offset(1 as libc::c_int as isize),
                prev_written + bytes_written,
            );
        }
        4 => {
            let mut buf: [libc::c_char; 21] = [0; 21];
            n = print_stats(
                fp,
                *formats.offset(0 as libc::c_int as isize),
                (records_read as libc::c_ulong).wrapping_mul(record_size) as tarlong,
            );
            fputc_unlocked(eor, fp);
            n += 1;
            n;
            n
                += print_stats(
                    fp,
                    *formats.offset(1 as libc::c_int as isize),
                    prev_written + bytes_written,
                );
            fputc_unlocked(eor, fp);
            n += 1;
            n;
            if !(*formats.offset(2 as libc::c_int as isize)).is_null()
                && *(*formats.offset(2 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int != 0
            {
                n
                    += fprintf(
                        fp,
                        b"%s: \0" as *const u8 as *const libc::c_char,
                        dcgettext(
                            0 as *const libc::c_char,
                            *formats.offset(2 as libc::c_int as isize),
                            5 as libc::c_int,
                        ),
                    );
            }
            n
                += fprintf(
                    fp,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    umaxtostr(
                        (((records_read - records_skipped) as libc::c_ulong)
                            .wrapping_mul(record_size) as libc::c_double
                            - (prev_written + bytes_written)) as uintmax_t,
                        buf.as_mut_ptr(),
                    ),
                );
        }
        6 | 7 | 5 => {
            n = print_stats(
                fp,
                dcgettext(
                    0 as *const libc::c_char,
                    *formats.offset(0 as libc::c_int as isize),
                    5 as libc::c_int,
                ),
                (records_read as libc::c_ulong).wrapping_mul(record_size) as tarlong,
            );
        }
        _ => {
            abort();
        }
    }
    if eol != 0 {
        fputc_unlocked(eol, fp);
        n += 1;
        n;
    }
    return n;
}
static mut default_total_format: [*const libc::c_char; 3] = [
    b"Total bytes read\0" as *const u8 as *const libc::c_char,
    b"Total bytes written\0" as *const u8 as *const libc::c_char,
    b"Total bytes deleted\0" as *const u8 as *const libc::c_char,
];
pub unsafe extern "C" fn print_total_stats() {
    format_total_stats(stderr, default_total_format.as_ptr(), '\n' as i32, '\n' as i32);
}
pub unsafe extern "C" fn current_block_ordinal() -> off_t {
    return record_start_block + current_block.offset_from(record_start) as libc::c_long;
}
pub unsafe extern "C" fn reset_eof() {
    if hit_eof {
        hit_eof = 0 as libc::c_int != 0;
        current_block = record_start;
        record_end = record_start.offset(blocking_factor as isize);
        access_mode = ACCESS_WRITE;
    }
}
pub unsafe extern "C" fn find_next_block() -> *mut block {
    if current_block == record_end {
        if hit_eof {
            return 0 as *mut block;
        }
        flush_archive();
        if current_block == record_end {
            hit_eof = 1 as libc::c_int != 0;
            return 0 as *mut block;
        }
    }
    return current_block;
}
pub unsafe extern "C" fn set_next_block_after(mut block: *mut block) {
    while block >= current_block {
        current_block = current_block.offset(1);
        current_block;
    }
    if current_block > record_end {
        abort();
    }
}
pub unsafe extern "C" fn available_space_after(mut pointer: *mut block) -> size_t {
    return ((*record_end).buffer)
        .as_mut_ptr()
        .offset_from(((*pointer).buffer).as_mut_ptr()) as libc::c_long as size_t;
}
pub unsafe extern "C" fn xclose(mut fd: libc::c_int) {
    if close(fd) != 0 as libc::c_int {
        close_error(
            dcgettext(
                0 as *const libc::c_char,
                b"(pipe)\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
unsafe extern "C" fn init_buffer() {
    if (record_buffer_aligned[record_index as usize]).is_null() {
        record_buffer_aligned[record_index
            as usize] = page_aligned_alloc(
            &mut *record_buffer.as_mut_ptr().offset(record_index as isize),
            record_size,
        ) as *mut block;
    }
    record_start = record_buffer_aligned[record_index as usize];
    current_block = record_start;
    record_end = record_start.offset(blocking_factor as isize);
}
unsafe extern "C" fn check_tty(mut mode: access_mode) {
    if strcmp(
        *archive_name_array.offset(0 as libc::c_int as isize),
        b"-\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
        && isatty(
            if mode as libc::c_uint == ACCESS_READ as libc::c_int as libc::c_uint {
                0 as libc::c_int
            } else {
                1 as libc::c_int
            },
        ) != 0
    {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            if mode as libc::c_uint == ACCESS_READ as libc::c_int as libc::c_uint {
                dcgettext(
                    0 as *const libc::c_char,
                    b"Refusing to read archive contents from terminal (missing -f option?)\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            } else {
                dcgettext(
                    0 as *const libc::c_char,
                    b"Refusing to write archive contents to terminal (missing -f option?)\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                )
            },
        );
        fatal_exit();
    }
}
unsafe extern "C" fn _open_archive(mut wanted_access: access_mode) {
    let mut backed_up_flag: libc::c_int = 0 as libc::c_int;
    if record_size == 0 as libc::c_int as libc::c_ulong {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Invalid value for record_size\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fatal_exit();
    }
    if archive_names == 0 as libc::c_int as libc::c_ulong {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"No archive name given\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fatal_exit();
    }
    tar_stat_destroy(&mut current_stat_info);
    record_index = 0 as libc::c_int;
    init_buffer();
    access_mode = (if wanted_access as libc::c_uint
        == ACCESS_UPDATE as libc::c_int as libc::c_uint
    {
        ACCESS_READ as libc::c_int as libc::c_uint
    } else {
        wanted_access as libc::c_uint
    }) as access_mode;
    check_tty(access_mode);
    read_full_records = read_full_records_option;
    records_read = 0 as libc::c_int as off_t;
    if !use_compress_program_option.is_null() {
        match wanted_access as libc::c_uint {
            0 => {
                child_pid = sys_child_open_for_uncompress();
                read_full_records = 1 as libc::c_int != 0;
                record_end = record_start;
            }
            1 => {
                child_pid = sys_child_open_for_compress();
            }
            2 => {
                abort();
            }
            _ => {}
        }
        if index_file_name.is_null()
            && wanted_access as libc::c_uint
                == ACCESS_WRITE as libc::c_int as libc::c_uint
            && strcmp(
                *archive_name_array.offset(0 as libc::c_int as isize),
                b"-\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            stdlis = stderr;
        }
    } else if strcmp(
        *archive_name_array.offset(0 as libc::c_int as isize),
        b"-\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        read_full_records = 1 as libc::c_int != 0;
        if verify_option {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cannot verify stdin/stdout archive\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            fatal_exit();
        }
        match wanted_access as libc::c_uint {
            0 => {
                let mut shortfile: bool = false;
                let mut type_0: compress_type = ct_none;
                archive = 0 as libc::c_int;
                type_0 = check_compressed_archive(&mut shortfile);
                if type_0 as libc::c_uint != ct_tar as libc::c_int as libc::c_uint
                    && type_0 as libc::c_uint != ct_none as libc::c_int as libc::c_uint
                {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Archive is compressed. Use %s option\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        compress_option(type_0),
                    );
                    fatal_exit();
                }
                if shortfile {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"This does not look like a tar archive\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    exit_status = 2 as libc::c_int;
                }
            }
            1 => {
                archive = 1 as libc::c_int;
                if index_file_name.is_null() {
                    stdlis = stderr;
                }
            }
            2 => {
                archive = 0 as libc::c_int;
                write_archive_to_stdout = 1 as libc::c_int != 0;
                record_end = record_start;
                if index_file_name.is_null() {
                    stdlis = stderr;
                }
            }
            _ => {}
        }
    } else {
        match wanted_access as libc::c_uint {
            0 => {
                archive = open_compressed_archive();
                if archive >= 0 as libc::c_int {
                    guess_seekable_archive();
                }
            }
            1 => {
                if backup_option {
                    maybe_backup_file(
                        *archive_name_array.offset(0 as libc::c_int as isize),
                        1 as libc::c_int != 0,
                    );
                    backed_up_flag = 1 as libc::c_int;
                }
                if verify_option {
                    archive = if !force_local_option
                        && {
                            rmt_dev_name__ = strchr(
                                *archive_name_array.offset(0 as libc::c_int as isize),
                                ':' as i32,
                            );
                            !rmt_dev_name__.is_null()
                        }
                        && rmt_dev_name__
                            > *archive_name_array.offset(0 as libc::c_int as isize)
                        && (memchr(
                            *archive_name_array.offset(0 as libc::c_int as isize)
                                as *const libc::c_void,
                            '/' as i32,
                            rmt_dev_name__
                                .offset_from(
                                    *archive_name_array.offset(0 as libc::c_int as isize),
                                ) as libc::c_long as libc::c_ulong,
                        ))
                            .is_null()
                    {
                        rmt_open__(
                            *archive_name_array.offset(0 as libc::c_int as isize),
                            0o2 as libc::c_int | 0o100 as libc::c_int | 0 as libc::c_int,
                            (1 as libc::c_int) << 30 as libc::c_int,
                            rsh_command_option,
                        )
                    } else {
                        open(
                            *archive_name_array.offset(0 as libc::c_int as isize),
                            0o2 as libc::c_int | 0o100 as libc::c_int | 0 as libc::c_int,
                            0o200 as libc::c_int
                                | 0o200 as libc::c_int >> 3 as libc::c_int
                                | 0o200 as libc::c_int >> 3 as libc::c_int
                                    >> 3 as libc::c_int
                                | (0o400 as libc::c_int
                                    | 0o400 as libc::c_int >> 3 as libc::c_int
                                    | 0o400 as libc::c_int >> 3 as libc::c_int
                                        >> 3 as libc::c_int),
                        )
                    };
                } else {
                    archive = if !force_local_option
                        && {
                            rmt_dev_name__ = strchr(
                                *archive_name_array.offset(0 as libc::c_int as isize),
                                ':' as i32,
                            );
                            !rmt_dev_name__.is_null()
                        }
                        && rmt_dev_name__
                            > *archive_name_array.offset(0 as libc::c_int as isize)
                        && (memchr(
                            *archive_name_array.offset(0 as libc::c_int as isize)
                                as *const libc::c_void,
                            '/' as i32,
                            rmt_dev_name__
                                .offset_from(
                                    *archive_name_array.offset(0 as libc::c_int as isize),
                                ) as libc::c_long as libc::c_ulong,
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
                            (0o200 as libc::c_int
                                | 0o200 as libc::c_int >> 3 as libc::c_int
                                | 0o200 as libc::c_int >> 3 as libc::c_int
                                    >> 3 as libc::c_int
                                | (0o400 as libc::c_int
                                    | 0o400 as libc::c_int >> 3 as libc::c_int
                                    | 0o400 as libc::c_int >> 3 as libc::c_int
                                        >> 3 as libc::c_int)) as mode_t,
                        )
                    };
                }
            }
            2 => {
                archive = if !force_local_option
                    && {
                        rmt_dev_name__ = strchr(
                            *archive_name_array.offset(0 as libc::c_int as isize),
                            ':' as i32,
                        );
                        !rmt_dev_name__.is_null()
                    }
                    && rmt_dev_name__
                        > *archive_name_array.offset(0 as libc::c_int as isize)
                    && (memchr(
                        *archive_name_array.offset(0 as libc::c_int as isize)
                            as *const libc::c_void,
                        '/' as i32,
                        rmt_dev_name__
                            .offset_from(
                                *archive_name_array.offset(0 as libc::c_int as isize),
                            ) as libc::c_long as libc::c_ulong,
                    ))
                        .is_null()
                {
                    rmt_open__(
                        *archive_name_array.offset(0 as libc::c_int as isize),
                        0o2 as libc::c_int | 0o100 as libc::c_int | 0 as libc::c_int,
                        (1 as libc::c_int) << 30 as libc::c_int,
                        rsh_command_option,
                    )
                } else {
                    open(
                        *archive_name_array.offset(0 as libc::c_int as isize),
                        0o2 as libc::c_int | 0o100 as libc::c_int | 0 as libc::c_int,
                        0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                            | 0o200 as libc::c_int >> 3 as libc::c_int
                                >> 3 as libc::c_int
                            | (0o400 as libc::c_int
                                | 0o400 as libc::c_int >> 3 as libc::c_int
                                | 0o400 as libc::c_int >> 3 as libc::c_int
                                    >> 3 as libc::c_int),
                    )
                };
                match check_compressed_archive(0 as *mut bool) as libc::c_uint {
                    0 | 1 => {}
                    _ => {
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
                        fatal_exit();
                    }
                }
            }
            _ => {}
        }
    }
    if archive < 0 as libc::c_int
        || !(archive >= (1 as libc::c_int) << 30 as libc::c_int)
            && !sys_get_archive_stat()
    {
        let mut saved_errno: libc::c_int = *__errno_location();
        if backed_up_flag != 0 {
            undo_last_backup();
        }
        *__errno_location() = saved_errno;
        open_fatal(*archive_name_array.offset(0 as libc::c_int as isize));
    }
    sys_detect_dev_null_output();
    sys_save_archive_dev_ino();
    match wanted_access as libc::c_uint {
        0 => {
            find_next_block();
        }
        2 | 1 => {
            records_written = 0 as libc::c_int as off_t;
        }
        _ => {}
    };
}
unsafe extern "C" fn _flush_write() -> ssize_t {
    let mut status: ssize_t = 0;
    checkpoint_run(1 as libc::c_int != 0);
    if tape_length_option != 0. && tape_length_option <= bytes_written {
        *__errno_location() = 28 as libc::c_int;
        status = 0 as libc::c_int as ssize_t;
    } else if dev_null_output {
        status = record_size as ssize_t;
    } else {
        status = sys_write_archive_buffer() as ssize_t;
    }
    if status != 0 && multi_volume_option as libc::c_int != 0 && inhibit_map == 0 {
        let mut map: *mut bufmap = bufmap_locate(status as size_t);
        if !map.is_null() {
            let mut delta: size_t = (status as libc::c_ulong)
                .wrapping_sub(
                    ((*map).start).wrapping_mul(512 as libc::c_int as libc::c_ulong),
                );
            let mut diff: ssize_t = 0;
            (*map)
                .nblocks = ((*map).nblocks as libc::c_ulong)
                .wrapping_add(delta.wrapping_div(512 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
            if delta > (*map).sizeleft as libc::c_ulong {
                delta = (*map).sizeleft as size_t;
            }
            (*map)
                .sizeleft = ((*map).sizeleft as libc::c_ulong).wrapping_sub(delta)
                as off_t as off_t;
            if (*map).sizeleft == 0 as libc::c_int as libc::c_long {
                diff = ((*map).start).wrapping_add((*map).nblocks) as ssize_t;
                map = (*map).next;
            } else {
                diff = (*map).start as ssize_t;
            }
            bufmap_reset(map, -diff);
        }
    }
    return status;
}
pub unsafe extern "C" fn archive_write_error(mut status: ssize_t) -> ! {
    if totals_option {
        let mut e: libc::c_int = *__errno_location();
        print_total_stats();
        *__errno_location() = e;
    }
    write_fatal_details(*archive_name_cursor, status, record_size);
}
pub unsafe extern "C" fn archive_read_error() {
    read_error(*archive_name_cursor);
    if record_start_block == 0 as libc::c_int as libc::c_long {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"At beginning of tape, quitting now\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fatal_exit();
    }
    let fresh1 = read_error_count;
    read_error_count = read_error_count + 1;
    if fresh1 > 10 as libc::c_int {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Too many errors, quitting\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fatal_exit();
    }
}
unsafe extern "C" fn archive_is_dev() -> bool {
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
    if fstat(archive, &mut st) != 0 {
        stat_diag(*archive_name_cursor);
        return 0 as libc::c_int != 0;
    }
    return st.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o60000 as libc::c_int as libc::c_uint
        || st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o20000 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn short_read(mut status: size_t) {
    let mut left: size_t = 0;
    let mut more: *mut libc::c_char = 0 as *mut libc::c_char;
    more = ((*record_start).buffer).as_mut_ptr().offset(status as isize);
    left = record_size.wrapping_sub(status);
    if left != 0
        && left.wrapping_rem(512 as libc::c_int as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
        && warning_option & 0x400000 as libc::c_int != 0
        && record_start_block == 0 as libc::c_int as libc::c_long
        && status != 0 as libc::c_int as libc::c_ulong
        && archive_is_dev() as libc::c_int != 0
    {
        let mut rsize: libc::c_ulong = status
            .wrapping_div(512 as libc::c_int as libc::c_ulong);
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcngettext(
                0 as *const libc::c_char,
                b"Record size = %lu block\0" as *const u8 as *const libc::c_char,
                b"Record size = %lu blocks\0" as *const u8 as *const libc::c_char,
                rsize,
                5 as libc::c_int,
            ),
            rsize,
        );
    }
    while left.wrapping_rem(512 as libc::c_int as libc::c_ulong)
        != 0 as libc::c_int as libc::c_ulong
        || left != 0 && status != 0 && read_full_records as libc::c_int != 0
    {
        if status != 0 {
            loop {
                status = if archive >= (1 as libc::c_int) << 30 as libc::c_int {
                    rmt_read__(
                        archive - ((1 as libc::c_int) << 30 as libc::c_int),
                        more,
                        left,
                    )
                } else {
                    safe_read(archive, more as *mut libc::c_void, left)
                };
                if !(status == -(1 as libc::c_int) as size_t) {
                    break;
                }
                archive_read_error();
            }
        }
        if status == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        if !read_full_records {
            let mut rest: libc::c_ulong = record_size.wrapping_sub(left);
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcngettext(
                    0 as *const libc::c_char,
                    b"Unaligned block (%lu byte) in archive\0" as *const u8
                        as *const libc::c_char,
                    b"Unaligned block (%lu bytes) in archive\0" as *const u8
                        as *const libc::c_char,
                    rest,
                    5 as libc::c_int,
                ),
                rest,
            );
            fatal_exit();
        }
        left = (left as libc::c_ulong).wrapping_sub(status) as size_t as size_t;
        more = more.offset(status as isize);
    }
    record_end = record_start
        .offset(
            record_size
                .wrapping_sub(left)
                .wrapping_div(512 as libc::c_int as libc::c_ulong) as isize,
        );
    records_read += 1;
    records_read;
}
pub unsafe extern "C" fn flush_archive() {
    let mut buffer_level: size_t = 0;
    if access_mode as libc::c_uint == ACCESS_READ as libc::c_int as libc::c_uint
        && time_to_start_writing as libc::c_int != 0
    {
        access_mode = ACCESS_WRITE;
        time_to_start_writing = 0 as libc::c_int != 0;
        backspace_output();
        if (record_end.offset_from(record_start) as libc::c_long)
            < blocking_factor as libc::c_long
        {
            memset(
                record_end as *mut libc::c_void,
                0 as libc::c_int,
                ((blocking_factor as libc::c_long
                    - record_end.offset_from(record_start) as libc::c_long)
                    * 512 as libc::c_int as libc::c_long) as libc::c_ulong,
            );
            record_end = record_start.offset(blocking_factor as isize);
            return;
        }
    }
    buffer_level = ((*current_block).buffer)
        .as_mut_ptr()
        .offset_from(((*record_start).buffer).as_mut_ptr()) as libc::c_long as size_t;
    record_start_block += record_end.offset_from(record_start) as libc::c_long;
    current_block = record_start;
    record_end = record_start.offset(blocking_factor as isize);
    match access_mode as libc::c_uint {
        0 => {
            flush_read();
        }
        1 => {
            flush_write_ptr.unwrap()(buffer_level);
        }
        2 => {
            abort();
        }
        _ => {}
    };
}
unsafe extern "C" fn backspace_output() {
    let mut operation: mtop = mtop { mt_op: 0, mt_count: 0 };
    operation.mt_op = 4 as libc::c_int as libc::c_short;
    operation.mt_count = 1 as libc::c_int;
    if (if archive >= (1 as libc::c_int) << 30 as libc::c_int {
        rmt_ioctl__(
            archive - ((1 as libc::c_int) << 30 as libc::c_int),
            (((1 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int
                    + 14 as libc::c_int
                | (('m' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
                | ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                as libc::c_ulong
                | (::std::mem::size_of::<mtop>() as libc::c_ulong)
                    << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int)
                as libc::c_int,
            &mut operation as *mut mtop as *mut libc::c_char,
        )
    } else {
        ioctl(
            archive,
            ((1 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int
                    + 14 as libc::c_int
                | (('m' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
                | ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                as libc::c_ulong
                | (::std::mem::size_of::<mtop>() as libc::c_ulong)
                    << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
            &mut operation as *mut mtop as *mut libc::c_char,
        )
    }) >= 0 as libc::c_int
    {
        return;
    }
    if *__errno_location() == 5 as libc::c_int
        && (if archive >= (1 as libc::c_int) << 30 as libc::c_int {
            rmt_ioctl__(
                archive - ((1 as libc::c_int) << 30 as libc::c_int),
                (((1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int
                        + 14 as libc::c_int
                    | (('m' as i32) << 0 as libc::c_int + 8 as libc::c_int)
                        as libc::c_uint
                    | ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                    as libc::c_ulong
                    | (::std::mem::size_of::<mtop>() as libc::c_ulong)
                        << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int)
                    as libc::c_int,
                &mut operation as *mut mtop as *mut libc::c_char,
            )
        } else {
            ioctl(
                archive,
                ((1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int
                        + 14 as libc::c_int
                    | (('m' as i32) << 0 as libc::c_int + 8 as libc::c_int)
                        as libc::c_uint
                    | ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                    as libc::c_ulong
                    | (::std::mem::size_of::<mtop>() as libc::c_ulong)
                        << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
                &mut operation as *mut mtop as *mut libc::c_char,
            )
        }) >= 0 as libc::c_int
    {
        return;
    }
    let mut position: off_t = if archive >= (1 as libc::c_int) << 30 as libc::c_int {
        rmt_lseek__(
            archive - ((1 as libc::c_int) << 30 as libc::c_int),
            0 as libc::c_int as off_t,
            1 as libc::c_int,
        )
    } else {
        lseek(archive, 0 as libc::c_int as off_t, 1 as libc::c_int)
    };
    position
        -= ((*record_end).buffer)
            .as_mut_ptr()
            .offset_from(((*record_start).buffer).as_mut_ptr()) as libc::c_long;
    if position < 0 as libc::c_int as libc::c_long {
        position = 0 as libc::c_int as off_t;
    }
    if (if archive >= (1 as libc::c_int) << 30 as libc::c_int {
        rmt_lseek__(
            archive - ((1 as libc::c_int) << 30 as libc::c_int),
            position,
            0 as libc::c_int,
        )
    } else {
        lseek(archive, position, 0 as libc::c_int)
    }) != position
    {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Cannot backspace archive file; it may be unreadable without -i\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        if ((*record_start).buffer).as_mut_ptr() != output_start {
            memset(
                ((*record_start).buffer).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                output_start.offset_from(((*record_start).buffer).as_mut_ptr())
                    as libc::c_long as libc::c_ulong,
            );
        }
    }
}
pub unsafe extern "C" fn seek_archive(mut size: off_t) -> off_t {
    let mut start: off_t = current_block_ordinal();
    let mut offset: off_t = 0;
    let mut nrec: off_t = 0;
    let mut nblk: off_t = 0;
    let mut skipped: off_t = (blocking_factor as libc::c_long
        - current_block.offset_from(record_start) as libc::c_long)
        * 512 as libc::c_int as libc::c_long;
    if size <= skipped {
        return 0 as libc::c_int as off_t;
    }
    nrec = ((size - skipped) as libc::c_ulong).wrapping_div(record_size) as off_t;
    if nrec == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as off_t;
    }
    offset = if archive >= (1 as libc::c_int) << 30 as libc::c_int {
        rmt_lseek__(
            archive - ((1 as libc::c_int) << 30 as libc::c_int),
            (nrec as libc::c_ulong).wrapping_mul(record_size) as off_t,
            1 as libc::c_int,
        )
    } else {
        lseek(
            archive,
            (nrec as libc::c_ulong).wrapping_mul(record_size) as __off_t,
            1 as libc::c_int,
        )
    };
    if offset < 0 as libc::c_int as libc::c_long {
        return offset;
    }
    if (offset as libc::c_ulong).wrapping_rem(record_size) != 0 {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"rmtlseek not stopped at a record boundary\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fatal_exit();
    }
    offset /= 512 as libc::c_int as libc::c_long;
    nblk = offset - start;
    records_read += nblk / blocking_factor as libc::c_long;
    record_start_block = offset - blocking_factor as libc::c_long;
    current_block = record_end;
    return nblk;
}
pub unsafe extern "C" fn close_archive() {
    if time_to_start_writing as libc::c_int != 0
        || access_mode as libc::c_uint == ACCESS_WRITE as libc::c_int as libc::c_uint
    {
        loop {
            flush_archive();
            if !(current_block > record_start) {
                break;
            }
        }
    }
    compute_duration();
    if verify_option {
        verify_volume();
    }
    if (if archive >= (1 as libc::c_int) << 30 as libc::c_int {
        rmt_close__(archive - ((1 as libc::c_int) << 30 as libc::c_int))
    } else {
        close(archive)
    }) != 0 as libc::c_int
    {
        close_error(*archive_name_cursor);
    }
    sys_wait_for_child(child_pid, hit_eof);
    tar_stat_destroy(&mut current_stat_info);
    rpl_free(record_buffer[0 as libc::c_int as usize]);
    rpl_free(record_buffer[1 as libc::c_int as usize]);
    bufmap_free(0 as *mut bufmap);
}
pub unsafe extern "C" fn write_fatal_details(
    mut name: *const libc::c_char,
    mut status: ssize_t,
    mut size: size_t,
) -> ! {
    write_error_details(name, status as size_t, size);
    if (if archive >= (1 as libc::c_int) << 30 as libc::c_int {
        rmt_close__(archive - ((1 as libc::c_int) << 30 as libc::c_int))
    } else {
        close(archive)
    }) != 0 as libc::c_int
    {
        close_error(*archive_name_cursor);
    }
    sys_wait_for_child(child_pid, 0 as libc::c_int != 0);
    fatal_exit();
}
pub unsafe extern "C" fn init_volume_number() {
    let mut file: *mut FILE = fopen(
        volno_file_option,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if !file.is_null() {
        if fscanf(
            file,
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut global_volno as *mut libc::c_int,
        ) != 1 as libc::c_int || global_volno < 0 as libc::c_int
        {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: contains invalid volume number\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_colon(volno_file_option),
            );
            fatal_exit();
        }
        if ferror_unlocked(file) != 0 {
            read_error(volno_file_option);
        }
        if fclose(file) != 0 as libc::c_int {
            close_error(volno_file_option);
        }
    } else if *__errno_location() != 2 as libc::c_int {
        open_error(volno_file_option);
    }
}
pub unsafe extern "C" fn closeout_volume_number() {
    let mut file: *mut FILE = fopen(
        volno_file_option,
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if !file.is_null() {
        fprintf(file, b"%d\n\0" as *const u8 as *const libc::c_char, global_volno);
        if ferror_unlocked(file) != 0 {
            write_error(volno_file_option);
        }
        if fclose(file) != 0 as libc::c_int {
            close_error(volno_file_option);
        }
    } else {
        open_error(volno_file_option);
    };
}
unsafe extern "C" fn increase_volume_number() {
    global_volno += 1;
    global_volno;
    if global_volno < 0 as libc::c_int {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Volume number overflow\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fatal_exit();
    }
    volno += 1;
    volno;
}
unsafe extern "C" fn change_tape_menu(mut read_file: *mut FILE) {
    let mut input_buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut stop: bool = 0 as libc::c_int != 0;
    while !stop {
        fputc_unlocked('\u{7}' as i32, stderr);
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Prepare volume #%d for %s and hit return: \0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            global_volno + 1 as libc::c_int,
            quote(*archive_name_cursor),
        );
        fflush_unlocked(stderr);
        if getline(&mut input_buffer, &mut size, read_file)
            <= 0 as libc::c_int as libc::c_long
        {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"EOF where user reply was expected\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if subcommand_option as libc::c_uint
                != EXTRACT_SUBCOMMAND as libc::c_int as libc::c_uint
                && subcommand_option as libc::c_uint
                    != LIST_SUBCOMMAND as libc::c_int as libc::c_uint
                && subcommand_option as libc::c_uint
                    != DIFF_SUBCOMMAND as libc::c_int as libc::c_uint
            {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"WARNING: Archive is incomplete\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            fatal_exit();
        }
        if *input_buffer.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32
            || *input_buffer.offset(0 as libc::c_int as isize) as libc::c_int
                == 'y' as i32
            || *input_buffer.offset(0 as libc::c_int as isize) as libc::c_int
                == 'Y' as i32
        {
            break;
        }
        let mut current_block_50: u64;
        match *input_buffer.offset(0 as libc::c_int as isize) as libc::c_int {
            63 => {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b" n name        Give a new file name for the next (and subsequent) volume(s)\n q             Abort tar\n y or newline  Continue operation\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if !restrict_option {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b" !             Spawn a subshell\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b" ?             Print this list\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                current_block_50 = 2989495919056355252;
            }
            113 => {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"No new volume; exiting.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if subcommand_option as libc::c_uint
                    != EXTRACT_SUBCOMMAND as libc::c_int as libc::c_uint
                    && subcommand_option as libc::c_uint
                        != LIST_SUBCOMMAND as libc::c_int as libc::c_uint
                    && subcommand_option as libc::c_uint
                        != DIFF_SUBCOMMAND as libc::c_int as libc::c_uint
                {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"WARNING: Archive is incomplete\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                fatal_exit();
            }
            110 => {
                let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut cursor: *mut libc::c_char = 0 as *mut libc::c_char;
                name = input_buffer.offset(1 as libc::c_int as isize);
                while *name as libc::c_int == ' ' as i32
                    || *name as libc::c_int == '\t' as i32
                {
                    name = name.offset(1);
                    name;
                }
                cursor = name;
                while *cursor as libc::c_int != 0
                    && *cursor as libc::c_int != '\n' as i32
                {
                    cursor = cursor.offset(1);
                    cursor;
                }
                *cursor = '\0' as i32 as libc::c_char;
                if *name.offset(0 as libc::c_int as isize) != 0 {
                    *archive_name_cursor = xstrdup(name);
                    stop = 1 as libc::c_int != 0;
                } else {
                    fprintf(
                        stderr,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"File name not specified. Try again.\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                current_block_50 = 2989495919056355252;
            }
            33 => {
                if !restrict_option {
                    sys_spawn_shell();
                    current_block_50 = 2989495919056355252;
                } else {
                    current_block_50 = 14176679388036944966;
                }
            }
            _ => {
                current_block_50 = 14176679388036944966;
            }
        }
        match current_block_50 {
            14176679388036944966 => {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Invalid input. Type ? for help.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            _ => {}
        }
    }
    rpl_free(input_buffer as *mut libc::c_void);
}
unsafe extern "C" fn new_volume(mut mode: access_mode) -> bool {
    static mut read_file: *mut FILE = 0 as *const FILE as *mut FILE;
    static mut looped: libc::c_int = 0;
    let mut prompt: libc::c_int = 0;
    if read_file.is_null() && info_script_option.is_null() {
        read_file = if archive == 0 as libc::c_int {
            fopen(
                b"/dev/tty\0" as *const u8 as *const libc::c_char,
                b"r\0" as *const u8 as *const libc::c_char,
            )
        } else {
            stdin
        };
    }
    if now_verifying {
        return 0 as libc::c_int != 0;
    }
    if verify_option {
        verify_volume();
    }
    assign_string(&mut volume_label, 0 as *const libc::c_char);
    assign_string(&mut continued_file_name, 0 as *const libc::c_char);
    continued_file_offset = 0 as libc::c_int as uintmax_t;
    continued_file_size = continued_file_offset;
    current_block = record_start;
    if (if archive >= (1 as libc::c_int) << 30 as libc::c_int {
        rmt_close__(archive - ((1 as libc::c_int) << 30 as libc::c_int))
    } else {
        close(archive)
    }) != 0 as libc::c_int
    {
        close_error(*archive_name_cursor);
    }
    archive_name_cursor = archive_name_cursor.offset(1);
    archive_name_cursor;
    if archive_name_cursor == archive_name_array.offset(archive_names as isize) {
        archive_name_cursor = archive_name_array;
        looped = 1 as libc::c_int;
    }
    prompt = looped;
    loop {
        if prompt != 0 {
            if !info_script_option.is_null() {
                if !volno_file_option.is_null() {
                    closeout_volume_number();
                }
                if sys_exec_info_script(
                    archive_name_cursor,
                    global_volno + 1 as libc::c_int,
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
                            b"%s command failed\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quote(info_script_option),
                    );
                    fatal_exit();
                }
            } else {
                change_tape_menu(read_file);
            }
        }
        if strcmp(
            *archive_name_cursor.offset(0 as libc::c_int as isize),
            b"-\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            read_full_records = 1 as libc::c_int != 0;
            archive = 0 as libc::c_int;
        } else if verify_option {
            archive = if !force_local_option
                && {
                    rmt_dev_name__ = strchr(*archive_name_cursor, ':' as i32);
                    !rmt_dev_name__.is_null()
                } && rmt_dev_name__ > *archive_name_cursor
                && (memchr(
                    *archive_name_cursor as *const libc::c_void,
                    '/' as i32,
                    rmt_dev_name__.offset_from(*archive_name_cursor) as libc::c_long
                        as libc::c_ulong,
                ))
                    .is_null()
            {
                rmt_open__(
                    *archive_name_cursor,
                    0o2 as libc::c_int | 0o100 as libc::c_int,
                    (1 as libc::c_int) << 30 as libc::c_int,
                    rsh_command_option,
                )
            } else {
                open(
                    *archive_name_cursor,
                    0o2 as libc::c_int | 0o100 as libc::c_int,
                    0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                        | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                        | (0o400 as libc::c_int
                            | 0o400 as libc::c_int >> 3 as libc::c_int
                            | 0o400 as libc::c_int >> 3 as libc::c_int
                                >> 3 as libc::c_int),
                )
            };
        } else {
            match mode as libc::c_uint {
                0 => {
                    archive = if !force_local_option
                        && {
                            rmt_dev_name__ = strchr(*archive_name_cursor, ':' as i32);
                            !rmt_dev_name__.is_null()
                        } && rmt_dev_name__ > *archive_name_cursor
                        && (memchr(
                            *archive_name_cursor as *const libc::c_void,
                            '/' as i32,
                            rmt_dev_name__.offset_from(*archive_name_cursor)
                                as libc::c_long as libc::c_ulong,
                        ))
                            .is_null()
                    {
                        rmt_open__(
                            *archive_name_cursor,
                            0 as libc::c_int,
                            (1 as libc::c_int) << 30 as libc::c_int,
                            rsh_command_option,
                        )
                    } else {
                        open(
                            *archive_name_cursor,
                            0 as libc::c_int,
                            0o200 as libc::c_int
                                | 0o200 as libc::c_int >> 3 as libc::c_int
                                | 0o200 as libc::c_int >> 3 as libc::c_int
                                    >> 3 as libc::c_int
                                | (0o400 as libc::c_int
                                    | 0o400 as libc::c_int >> 3 as libc::c_int
                                    | 0o400 as libc::c_int >> 3 as libc::c_int
                                        >> 3 as libc::c_int),
                        )
                    };
                    guess_seekable_archive();
                }
                1 => {
                    if backup_option {
                        maybe_backup_file(*archive_name_cursor, 1 as libc::c_int != 0);
                    }
                    archive = if !force_local_option
                        && {
                            rmt_dev_name__ = strchr(*archive_name_cursor, ':' as i32);
                            !rmt_dev_name__.is_null()
                        } && rmt_dev_name__ > *archive_name_cursor
                        && (memchr(
                            *archive_name_cursor as *const libc::c_void,
                            '/' as i32,
                            rmt_dev_name__.offset_from(*archive_name_cursor)
                                as libc::c_long as libc::c_ulong,
                        ))
                            .is_null()
                    {
                        rmt_open__(
                            *archive_name_cursor,
                            0o100 as libc::c_int | 0o1 as libc::c_int,
                            (1 as libc::c_int) << 30 as libc::c_int,
                            rsh_command_option,
                        )
                    } else {
                        creat(
                            *archive_name_cursor,
                            (0o200 as libc::c_int
                                | 0o200 as libc::c_int >> 3 as libc::c_int
                                | 0o200 as libc::c_int >> 3 as libc::c_int
                                    >> 3 as libc::c_int
                                | (0o400 as libc::c_int
                                    | 0o400 as libc::c_int >> 3 as libc::c_int
                                    | 0o400 as libc::c_int >> 3 as libc::c_int
                                        >> 3 as libc::c_int)) as mode_t,
                        )
                    };
                }
                2 => {
                    archive = if !force_local_option
                        && {
                            rmt_dev_name__ = strchr(*archive_name_cursor, ':' as i32);
                            !rmt_dev_name__.is_null()
                        } && rmt_dev_name__ > *archive_name_cursor
                        && (memchr(
                            *archive_name_cursor as *const libc::c_void,
                            '/' as i32,
                            rmt_dev_name__.offset_from(*archive_name_cursor)
                                as libc::c_long as libc::c_ulong,
                        ))
                            .is_null()
                    {
                        rmt_open__(
                            *archive_name_cursor,
                            0o2 as libc::c_int | 0o100 as libc::c_int,
                            (1 as libc::c_int) << 30 as libc::c_int,
                            rsh_command_option,
                        )
                    } else {
                        open(
                            *archive_name_cursor,
                            0o2 as libc::c_int | 0o100 as libc::c_int,
                            0o200 as libc::c_int
                                | 0o200 as libc::c_int >> 3 as libc::c_int
                                | 0o200 as libc::c_int >> 3 as libc::c_int
                                    >> 3 as libc::c_int
                                | (0o400 as libc::c_int
                                    | 0o400 as libc::c_int >> 3 as libc::c_int
                                    | 0o400 as libc::c_int >> 3 as libc::c_int
                                        >> 3 as libc::c_int),
                        )
                    };
                }
                _ => {}
            }
        }
        if !(archive < 0 as libc::c_int) {
            break;
        }
        open_warn(*archive_name_cursor);
        if !verify_option
            && mode as libc::c_uint == ACCESS_WRITE as libc::c_int as libc::c_uint
            && backup_option as libc::c_int != 0
        {
            undo_last_backup();
        }
        prompt = 1 as libc::c_int;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn read_header0(mut info: *mut tar_stat_info) -> bool {
    let mut rc: read_header = HEADER_STILL_UNREAD;
    tar_stat_init(info);
    rc = read_header(&mut current_header, info, read_header_auto);
    if rc as libc::c_uint == HEADER_SUCCESS as libc::c_int as libc::c_uint {
        set_next_block_after(current_header);
        return 1 as libc::c_int != 0;
    }
    if error_hook.is_some() {
        error_hook.unwrap()();
    }
    error(
        0 as libc::c_int,
        0 as libc::c_int,
        dcgettext(
            0 as *const libc::c_char,
            b"This does not look like a tar archive\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    exit_status = 2 as libc::c_int;
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn try_new_volume() -> bool {
    let mut status: size_t = 0;
    let mut header: *mut block = 0 as *mut block;
    let mut acc: access_mode = ACCESS_READ;
    match subcommand_option as libc::c_uint {
        1 | 2 | 8 => {
            acc = ACCESS_UPDATE;
        }
        _ => {
            acc = ACCESS_READ;
        }
    }
    if !new_volume(acc) {
        return 1 as libc::c_int != 0;
    }
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
    if status != record_size {
        short_read(status);
    }
    header = find_next_block();
    if header.is_null() {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"This does not look like a tar archive\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    let mut current_block_50: u64;
    match (*header).header.typeflag as libc::c_int {
        103 => {
            tar_stat_init(&mut dummy);
            if read_header(&mut header, &mut dummy, read_header_x_global) as libc::c_uint
                != HEADER_SUCCESS_EXTENDED as libc::c_int as libc::c_uint
            {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"This does not look like a tar archive\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return 0 as libc::c_int != 0;
            }
            xheader_decode(&mut dummy);
            tar_stat_destroy(&mut dummy);
            match read_header(&mut header, &mut dummy, read_header_auto) as libc::c_uint
            {
                1 => {
                    set_next_block_after(header);
                }
                5 => {}
                _ => {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"This does not look like a tar archive\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    return 0 as libc::c_int != 0;
                }
            }
            current_block_50 = 1345366029464561491;
        }
        86 => {
            if !read_header0(&mut dummy) {
                return 0 as libc::c_int != 0;
            }
            tar_stat_destroy(&mut dummy);
            assign_string_n(
                &mut volume_label,
                ((*current_header).header.name).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
            );
            set_next_block_after(header);
            header = find_next_block();
            if (*header).header.typeflag as libc::c_int != 'M' as i32 {
                current_block_50 = 1345366029464561491;
            } else {
                current_block_50 = 12346440543440388163;
            }
        }
        77 => {
            current_block_50 = 12346440543440388163;
        }
        _ => {
            current_block_50 = 1345366029464561491;
        }
    }
    match current_block_50 {
        12346440543440388163 => {
            if !read_header0(&mut dummy) {
                return 0 as libc::c_int != 0;
            }
            tar_stat_destroy(&mut dummy);
            assign_string_n(
                &mut continued_file_name,
                ((*current_header).header.name).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
            );
            continued_file_size = uintmax_from_header(
                ((*current_header).header.size).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
            );
            continued_file_offset = uintmax_from_header(
                ((*current_header).oldgnu_header.offset).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
            );
        }
        _ => {}
    }
    if !bufmap_head.is_null() {
        let mut s: uintmax_t = 0;
        if continued_file_name.is_null() {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is not continued on this volume\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote((*bufmap_head).file_name),
            );
            return 0 as libc::c_int != 0;
        }
        if strcmp(continued_file_name, (*bufmap_head).file_name) != 0 {
            if (archive_format as libc::c_uint
                == GNU_FORMAT as libc::c_int as libc::c_uint
                || archive_format as libc::c_uint
                    == OLDGNU_FORMAT as libc::c_int as libc::c_uint)
                && strlen((*bufmap_head).file_name)
                    >= 100 as libc::c_int as libc::c_ulong
                && strncmp(
                    continued_file_name,
                    (*bufmap_head).file_name,
                    100 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s is possibly continued on this volume: header contains truncated name\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote((*bufmap_head).file_name),
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
                        b"%s is not continued on this volume\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote((*bufmap_head).file_name),
                );
                return 0 as libc::c_int != 0;
            }
        }
        s = continued_file_size.wrapping_add(continued_file_offset);
        if (*bufmap_head).sizetotal as libc::c_ulong != s || s < continued_file_offset {
            let mut totsizebuf: [libc::c_char; 21] = [0; 21];
            let mut s1buf: [libc::c_char; 21] = [0; 21];
            let mut s2buf: [libc::c_char; 21] = [0; 21];
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is the wrong size (%s != %s + %s)\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(continued_file_name),
                umaxtostr(
                    (*bufmap_head).sizetotal as uintmax_t,
                    totsizebuf.as_mut_ptr(),
                ),
                umaxtostr(continued_file_size, s1buf.as_mut_ptr()),
                umaxtostr(continued_file_offset, s2buf.as_mut_ptr()),
            );
            return 0 as libc::c_int != 0;
        }
        if ((*bufmap_head).sizetotal - (*bufmap_head).sizeleft) as libc::c_ulong
            != continued_file_offset
        {
            let mut totsizebuf_0: [libc::c_char; 21] = [0; 21];
            let mut s1buf_0: [libc::c_char; 21] = [0; 21];
            let mut s2buf_0: [libc::c_char; 21] = [0; 21];
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"This volume is out of sequence (%s - %s != %s)\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                umaxtostr(
                    (*bufmap_head).sizetotal as uintmax_t,
                    totsizebuf_0.as_mut_ptr(),
                ),
                umaxtostr((*bufmap_head).sizeleft as uintmax_t, s1buf_0.as_mut_ptr()),
                umaxtostr(continued_file_offset, s2buf_0.as_mut_ptr()),
            );
            return 0 as libc::c_int != 0;
        }
    }
    increase_volume_number();
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn drop_volume_label_suffix(
    mut label: *const libc::c_char,
) -> *mut libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = strlen(label);
    if len < 1 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_char;
    }
    p = label.offset(len as isize).offset(-(1 as libc::c_int as isize));
    while p > label
        && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        p = p.offset(-1);
        p;
    }
    if p > label
        && p
            .offset(
                -((::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize),
            ) > label
    {
        p = p
            .offset(
                -((::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize),
            );
        if memcmp(
            p as *const libc::c_void,
            b" Volume \0" as *const u8 as *const libc::c_char as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
        {
            len = p.offset_from(label) as libc::c_long as size_t;
            let mut s: *mut libc::c_char = xmalloc(
                len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            memcpy(s as *mut libc::c_void, label as *const libc::c_void, len);
            *s.offset(len as isize) = 0 as libc::c_int as libc::c_char;
            return s;
        }
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn check_label_pattern(mut label: *const libc::c_char) -> bool {
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: bool = 0 as libc::c_int != 0;
    if fnmatch(volume_label_option, label, 0 as libc::c_int) == 0 as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    if !multi_volume_option {
        return 0 as libc::c_int != 0;
    }
    string = drop_volume_label_suffix(label);
    if !string.is_null() {
        result = fnmatch(string, volume_label_option, 0 as libc::c_int)
            == 0 as libc::c_int;
        rpl_free(string as *mut libc::c_void);
    }
    return result;
}
unsafe extern "C" fn match_volume_label() {
    if volume_label.is_null() {
        let mut label: *mut block = find_next_block();
        if label.is_null() {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Archive not labeled to match %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(volume_label_option),
            );
            fatal_exit();
        }
        if (*label).header.typeflag as libc::c_int == 'V' as i32 {
            assign_string_n(
                &mut volume_label,
                ((*label).header.name).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
            );
        } else if (*label).header.typeflag as libc::c_int == 'g' as i32 {
            let mut st: tar_stat_info = tar_stat_info {
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
            tar_stat_init(&mut st);
            xheader_read(
                &mut st.xhdr,
                label,
                off_from_header(
                    ((*label).header.size).as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
                ),
            );
            xheader_decode(&mut st);
            tar_stat_destroy(&mut st);
        }
    }
    if volume_label.is_null() {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Archive not labeled to match %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(volume_label_option),
        );
        fatal_exit();
    }
    if !check_label_pattern(volume_label) {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Volume %s does not match %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote_n(0 as libc::c_int, volume_label),
            quote_n(1 as libc::c_int, volume_label_option),
        );
        fatal_exit();
    }
}
unsafe extern "C" fn _write_volume_label(mut str: *const libc::c_char) {
    if archive_format as libc::c_uint == POSIX_FORMAT as libc::c_int as libc::c_uint {
        xheader_store(
            b"GNU.volume.label\0" as *const u8 as *const libc::c_char,
            &mut dummy,
            str as *const libc::c_void,
        );
    } else {
        let mut label: *mut block = find_next_block();
        memset(
            label as *mut libc::c_void,
            0 as libc::c_int,
            512 as libc::c_int as libc::c_ulong,
        );
        strcpy(((*label).header.name).as_mut_ptr(), str);
        assign_string(
            &mut current_stat_info.file_name,
            ((*label).header.name).as_mut_ptr(),
        );
        current_stat_info
            .had_trailing_slash = strip_trailing_slashes(current_stat_info.file_name);
        (*label).header.typeflag = 'V' as i32 as libc::c_char;
        time_to_chars(
            start_time.tv_sec,
            ((*label).header.mtime).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
        );
        finish_header(&mut current_stat_info, label, -(1 as libc::c_int) as off_t);
        set_next_block_after(label);
    };
}
unsafe extern "C" fn add_volume_label() {
    let mut buf: [libc::c_char; 21] = [0; 21];
    let mut p: *mut libc::c_char = umaxtostr(volno as uintmax_t, buf.as_mut_ptr());
    let mut s: *mut libc::c_char = xmalloc(
        (strlen(volume_label_option))
            .wrapping_add(::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
            .wrapping_add(strlen(p))
            .wrapping_add(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    sprintf(
        s,
        b"%s %s %s\0" as *const u8 as *const libc::c_char,
        volume_label_option,
        b"Volume\0" as *const u8 as *const libc::c_char,
        p,
    );
    _write_volume_label(s);
    rpl_free(s as *mut libc::c_void);
}
unsafe extern "C" fn add_chunk_header(mut map: *mut bufmap) {
    if archive_format as libc::c_uint == POSIX_FORMAT as libc::c_int as libc::c_uint {
        let mut blk: *mut block = 0 as *mut block;
        let mut st: tar_stat_info = tar_stat_info {
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
        memset(
            &mut st as *mut tar_stat_info as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<tar_stat_info>() as libc::c_ulong,
        );
        st.file_name = (*map).file_name;
        st.orig_file_name = st.file_name;
        st
            .stat
            .st_mode = (0o100000 as libc::c_int | 0o400 as libc::c_int
            | 0o200 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as __mode_t;
        st.stat.st_uid = getuid();
        st.stat.st_gid = getgid();
        st
            .orig_file_name = xheader_format_name(
            &mut st,
            b"%d/GNUFileParts/%f.%n\0" as *const u8 as *const libc::c_char,
            volno as size_t,
        );
        st.file_name = st.orig_file_name;
        st.stat.st_size = (*map).sizeleft;
        st.archive_file_size = st.stat.st_size;
        blk = start_header(&mut st);
        if blk.is_null() {
            abort();
        }
        simple_finish_header(write_extended(0 as libc::c_int != 0, &mut st, blk));
        rpl_free(st.orig_file_name as *mut libc::c_void);
    }
}
unsafe extern "C" fn write_volume_label() {
    if multi_volume_option {
        add_volume_label();
    } else {
        _write_volume_label(volume_label_option);
    };
}
unsafe extern "C" fn gnu_add_multi_volume_header(mut map: *mut bufmap) {
    let mut tmp: libc::c_int = 0;
    let mut block: *mut block = find_next_block();
    let mut len: size_t = strlen((*map).file_name);
    if len > 100 as libc::c_int as libc::c_ulong {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: file name too long to be stored in a GNU multivolume header, truncated\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_colon((*map).file_name),
        );
        len = 100 as libc::c_int as size_t;
    }
    memset(
        block as *mut libc::c_void,
        0 as libc::c_int,
        512 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        ((*block).header.name).as_mut_ptr() as *mut libc::c_void,
        (*map).file_name as *const libc::c_void,
        len,
    );
    (*block).header.typeflag = 'M' as i32 as libc::c_char;
    off_to_chars(
        (*map).sizeleft,
        ((*block).header.size).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    );
    off_to_chars(
        (*map).sizetotal - (*map).sizeleft,
        ((*block).oldgnu_header.offset).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    );
    tmp = verbose_option;
    verbose_option = 0 as libc::c_int;
    finish_header(&mut current_stat_info, block, -(1 as libc::c_int) as off_t);
    verbose_option = tmp;
    set_next_block_after(block);
}
unsafe extern "C" fn add_multi_volume_header(mut map: *mut bufmap) {
    if archive_format as libc::c_uint == POSIX_FORMAT as libc::c_int as libc::c_uint {
        let mut d: off_t = (*map).sizetotal - (*map).sizeleft;
        xheader_store(
            b"GNU.volume.filename\0" as *const u8 as *const libc::c_char,
            &mut dummy,
            (*map).file_name as *const libc::c_void,
        );
        xheader_store(
            b"GNU.volume.size\0" as *const u8 as *const libc::c_char,
            &mut dummy,
            &mut (*map).sizeleft as *mut off_t as *const libc::c_void,
        );
        xheader_store(
            b"GNU.volume.offset\0" as *const u8 as *const libc::c_char,
            &mut dummy,
            &mut d as *mut off_t as *const libc::c_void,
        );
    } else {
        gnu_add_multi_volume_header(map);
    };
}
unsafe extern "C" fn simple_flush_read() {
    let mut status: size_t = 0;
    checkpoint_run(0 as libc::c_int != 0);
    read_error_count = 0 as libc::c_int;
    if write_archive_to_stdout as libc::c_int != 0
        && record_start_block != 0 as libc::c_int as libc::c_long
    {
        archive = 1 as libc::c_int;
        status = sys_write_archive_buffer();
        archive = 0 as libc::c_int;
        if status != record_size {
            archive_write_error(status as ssize_t);
        }
    }
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
        if status == record_size {
            records_read += 1;
            records_read;
            return;
        }
        if !(status == -(1 as libc::c_int) as size_t) {
            break;
        }
        archive_read_error();
    }
    short_read(status);
}
unsafe extern "C" fn simple_flush_write(mut level: size_t) {
    let mut status: ssize_t = 0;
    status = _flush_write();
    if status as libc::c_ulong != record_size {
        archive_write_error(status);
    } else {
        records_written += 1;
        records_written;
        bytes_written += status as libc::c_double;
    };
}
unsafe extern "C" fn _gnu_flush_read() {
    let mut status: size_t = 0;
    checkpoint_run(0 as libc::c_int != 0);
    read_error_count = 0 as libc::c_int;
    if write_archive_to_stdout as libc::c_int != 0
        && record_start_block != 0 as libc::c_int as libc::c_long
    {
        archive = 1 as libc::c_int;
        status = sys_write_archive_buffer();
        archive = 0 as libc::c_int;
        if status != record_size {
            archive_write_error(status as ssize_t);
        }
    }
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
        if status == record_size {
            records_read += 1;
            records_read;
            return;
        }
        if (status == 0 as libc::c_int as libc::c_ulong
            || status == -(1 as libc::c_int) as size_t
                && *__errno_location() == 28 as libc::c_int)
            && multi_volume_option as libc::c_int != 0
        {
            while !try_new_volume() {}
            if current_block == record_end {
                flush_archive();
            }
            return;
        } else {
            if !(status == -(1 as libc::c_int) as size_t) {
                break;
            }
            archive_read_error();
        }
    }
    short_read(status);
}
unsafe extern "C" fn gnu_flush_read() {
    flush_read_ptr = Some(simple_flush_read as unsafe extern "C" fn() -> ());
    _gnu_flush_read();
    flush_read_ptr = Some(gnu_flush_read as unsafe extern "C" fn() -> ());
}
unsafe extern "C" fn _gnu_flush_write(mut buffer_level: size_t) {
    let mut status: ssize_t = 0;
    let mut header: *mut block = 0 as *mut block;
    let mut copy_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut copy_size: size_t = 0;
    let mut bufsize: size_t = 0;
    let mut map: *mut bufmap = 0 as *mut bufmap;
    status = _flush_write();
    if status as libc::c_ulong != record_size && !multi_volume_option {
        archive_write_error(status);
    } else {
        if status != 0 {
            records_written += 1;
            records_written;
        }
        bytes_written += status as libc::c_double;
    }
    if status as libc::c_ulong == record_size {
        return;
    }
    map = bufmap_locate(status as size_t);
    if status % 512 as libc::c_int as libc::c_long != 0 {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"write did not end on a block boundary\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        exit_status = 2 as libc::c_int;
        archive_write_error(status);
    }
    if status < 0 as libc::c_int as libc::c_long
        && *__errno_location() != 28 as libc::c_int
        && *__errno_location() != 5 as libc::c_int
        && *__errno_location() != 6 as libc::c_int
    {
        archive_write_error(status);
    }
    if !new_volume(ACCESS_WRITE) {
        return;
    }
    tar_stat_destroy(&mut dummy);
    increase_volume_number();
    prev_written += bytes_written;
    bytes_written = 0 as libc::c_int as tarlong;
    copy_ptr = ((*record_start).buffer).as_mut_ptr().offset(status as isize);
    copy_size = buffer_level.wrapping_sub(status as libc::c_ulong);
    record_index = (record_index == 0) as libc::c_int;
    init_buffer();
    inhibit_map = 1 as libc::c_int;
    if !volume_label_option.is_null() {
        add_volume_label();
    }
    if !map.is_null() {
        add_multi_volume_header(map);
    }
    write_extended(1 as libc::c_int != 0, &mut dummy, find_next_block());
    tar_stat_destroy(&mut dummy);
    if !map.is_null() {
        add_chunk_header(map);
    }
    header = find_next_block();
    bufmap_reset(map, header.offset_from(record_start) as libc::c_long);
    bufsize = available_space_after(header);
    inhibit_map = 0 as libc::c_int;
    while bufsize < copy_size {
        memcpy(
            ((*header).buffer).as_mut_ptr() as *mut libc::c_void,
            copy_ptr as *const libc::c_void,
            bufsize,
        );
        copy_ptr = copy_ptr.offset(bufsize as isize);
        copy_size = (copy_size as libc::c_ulong).wrapping_sub(bufsize) as size_t
            as size_t;
        set_next_block_after(
            header
                .offset(
                    bufsize
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_div(512 as libc::c_int as libc::c_ulong) as isize,
                ),
        );
        header = find_next_block();
        bufsize = available_space_after(header);
    }
    memcpy(
        ((*header).buffer).as_mut_ptr() as *mut libc::c_void,
        copy_ptr as *const libc::c_void,
        copy_size,
    );
    memset(
        ((*header).buffer).as_mut_ptr().offset(copy_size as isize) as *mut libc::c_void,
        0 as libc::c_int,
        bufsize.wrapping_sub(copy_size),
    );
    set_next_block_after(
        header
            .offset(
                copy_size
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(512 as libc::c_int as libc::c_ulong) as isize,
            ),
    );
    find_next_block();
}
unsafe extern "C" fn gnu_flush_write(mut buffer_level: size_t) {
    flush_write_ptr = Some(simple_flush_write as unsafe extern "C" fn(size_t) -> ());
    _gnu_flush_write(buffer_level);
    flush_write_ptr = Some(gnu_flush_write as unsafe extern "C" fn(size_t) -> ());
}
pub unsafe extern "C" fn flush_read() {
    flush_read_ptr.unwrap()();
}
pub unsafe extern "C" fn flush_write() {
    flush_write_ptr.unwrap()(record_size);
}
pub unsafe extern "C" fn open_archive(mut wanted_access: access_mode) {
    flush_read_ptr = Some(gnu_flush_read as unsafe extern "C" fn() -> ());
    flush_write_ptr = Some(gnu_flush_write as unsafe extern "C" fn(size_t) -> ());
    _open_archive(wanted_access);
    match wanted_access as libc::c_uint {
        0 | 2 => {
            if !volume_label_option.is_null() {
                match_volume_label();
            }
        }
        1 => {
            records_written = 0 as libc::c_int as off_t;
            if !volume_label_option.is_null() {
                write_volume_label();
            }
        }
        _ => {}
    }
    set_volume_start_time();
}
