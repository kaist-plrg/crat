use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type exclist;
    pub type directory;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn openat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __oflag: libc::c_int,
        _: ...
    ) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn readlinkat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn rpl_free(ptr: *mut libc::c_void);
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xmalloc(s: size_t) -> *mut libc::c_void;
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
    fn read_error(_: *const libc::c_char);
    fn readlink_error(_: *const libc::c_char);
    fn readlink_warn(_: *const libc::c_char);
    fn seek_error_details(_: *const libc::c_char, _: off_t);
    fn seek_warn(_: *const libc::c_char);
    fn stat_error(_: *const libc::c_char);
    fn stat_warn(_: *const libc::c_char);
    fn utime_error(_: *const libc::c_char);
    fn removed_prefixes_p() -> bool;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    static mut record_size: size_t;
    static mut atime_preserve_option: atime_preserve;
    static mut ignore_zeros_option: bool;
    static mut listed_incremental_option: *const libc::c_char;
    static mut verbose_option: libc::c_int;
    static mut archive: libc::c_int;
    static mut current_stat_info: tar_stat_info;
    static mut archive_name_array: *mut *const libc::c_char;
    static mut open_read_flags: libc::c_int;
    static mut access_mode: access_mode;
    static mut stdlis: *mut FILE;
    fn current_block_ordinal() -> off_t;
    fn find_next_block() -> *mut block;
    fn flush_read();
    fn available_space_after(pointer: *mut block) -> size_t;
    fn off_from_header(buf: *const libc::c_char, size: size_t) -> off_t;
    static mut current_header: *mut block;
    fn set_next_block_after(block: *mut block);
    fn mv_begin_read(st: *mut tar_stat_info);
    fn mv_end();
    fn scan_directory(st: *mut tar_stat_info) -> *mut directory;
    fn directory_contents(dir: *mut directory) -> *const libc::c_char;
    fn mv_size_left(size: off_t);
    fn is_dumpdir(stat_info: *mut tar_stat_info) -> bool;
    fn subfile_open(
        dir: *const tar_stat_info,
        file: *const libc::c_char,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn read_directory_file();
    fn clear_directory_table();
    static mut current_format: archive_format;
    fn decode_header(
        header: *mut block,
        stat_info: *mut tar_stat_info,
        format_pointer: *mut archive_format,
        do_user_group: libc::c_int,
    );
    fn deref_stat(name: *const libc::c_char, buf: *mut stat) -> libc::c_int;
    static mut chdir_fd: libc::c_int;
    fn skip_member();
    fn blocking_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn quote_n_colon(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn read_header(
        return_block: *mut *mut block,
        info: *mut tar_stat_info,
        m: read_header_mode,
    ) -> read_header;
    fn page_aligned_alloc(
        ptr: *mut *mut libc::c_void,
        size: size_t,
    ) -> *mut libc::c_void;
    fn print_header(st: *mut tar_stat_info, blk: *mut block, block_ordinal: off_t);
    fn set_file_atime(
        fd: libc::c_int,
        parentfd: libc::c_int,
        file: *const libc::c_char,
        atime: timespec,
    ) -> libc::c_int;
    fn open_diag(name: *const libc::c_char);
    fn stat_diag(name: *const libc::c_char);
    fn file_removed_diag(
        name: *const libc::c_char,
        top_level: bool,
        diagfn: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
    );
    fn sys_compare_links(link_data: *mut stat, stat_data: *mut stat) -> bool;
    fn tar_timespec_cmp(a: timespec, b: timespec) -> libc::c_int;
    fn sys_compare_gid(a: *mut stat, b: *mut stat) -> bool;
    fn sys_compare_uid(a: *mut stat, b: *mut stat) -> bool;
    fn tar_stat_destroy(st: *mut tar_stat_info);
    fn set_exit_status(val: libc::c_int);
    static mut warning_option: libc::c_int;
    fn transform_program_p() -> bool;
    fn sparse_diff_file(_: libc::c_int, st: *mut tar_stat_info) -> bool;
    fn quotearg_colon(arg: *const libc::c_char) -> *mut libc::c_char;
    fn rmt_lseek__(_: libc::c_int, _: off_t, _: libc::c_int) -> off_t;
    fn rmt_ioctl__(_: libc::c_int, _: libc::c_int, _: *mut libc::c_char) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
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
pub type off_t = __off_t;
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
pub type va_list = __builtin_va_list;
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
pub struct mtop {
    pub mt_op: libc::c_short,
    pub mt_count: libc::c_int,
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
pub type atime_preserve = libc::c_uint;
pub const system_atime_preserve: atime_preserve = 2;
pub const replace_atime_preserve: atime_preserve = 1;
pub const no_atime_preserve: atime_preserve = 0;
pub type access_mode = libc::c_uint;
pub const ACCESS_UPDATE: access_mode = 2;
pub const ACCESS_WRITE: access_mode = 1;
pub const ACCESS_READ: access_mode = 0;
pub const QUOTE_NAME: C2RustUnnamed_2 = 1;
pub const QUOTE_ARG: C2RustUnnamed_2 = 0;
pub const HEADER_ZERO_BLOCK: read_header = 3;
pub type read_header = libc::c_uint;
pub const HEADER_FAILURE: read_header = 5;
pub const HEADER_END_OF_FILE: read_header = 4;
pub const HEADER_SUCCESS_EXTENDED: read_header = 2;
pub const HEADER_SUCCESS: read_header = 1;
pub const HEADER_STILL_UNREAD: read_header = 0;
pub type read_header_mode = libc::c_uint;
pub const read_header_x_global: read_header_mode = 2;
pub const read_header_x_raw: read_header_mode = 1;
pub const read_header_auto: read_header_mode = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn get_stat_atime(mut st: *const stat) -> timespec {
    return (*st).st_atim;
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
pub static mut now_verifying: bool = false;
static mut diff_handle: libc::c_int = 0;
static mut diff_buffer: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub unsafe extern "C" fn diff_init() {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    diff_buffer = page_aligned_alloc(&mut ptr, record_size) as *mut libc::c_char;
    if !listed_incremental_option.is_null() {
        read_directory_file();
    }
}
pub unsafe extern "C" fn report_difference(
    mut st: *mut tar_stat_info,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    if !fmt.is_null() {
        let mut ap: ::std::ffi::VaListImpl;
        fprintf(
            stdlis,
            b"%s: \0" as *const u8 as *const libc::c_char,
            quote_n_colon(QUOTE_NAME as libc::c_int, (*st).file_name),
        );
        ap = args.clone();
        vfprintf(stdlis, fmt, ap.as_va_list());
        fprintf(stdlis, b"\n\0" as *const u8 as *const libc::c_char);
    }
    set_exit_status(1 as libc::c_int);
}
unsafe extern "C" fn process_noop(
    mut size: size_t,
    mut data: *mut libc::c_char,
) -> libc::c_int {
    return 1 as libc::c_int;
}
unsafe extern "C" fn process_rawdata(
    mut bytes: size_t,
    mut buffer: *mut libc::c_char,
) -> libc::c_int {
    let mut status: size_t = blocking_read(
        diff_handle,
        diff_buffer as *mut libc::c_void,
        bytes,
    );
    if status != bytes {
        if status == -(1 as libc::c_int) as size_t {
            read_error(current_stat_info.file_name);
            report_difference(
                &mut current_stat_info as *mut tar_stat_info,
                0 as *const libc::c_char,
            );
        } else {
            report_difference(
                &mut current_stat_info as *mut tar_stat_info,
                dcngettext(
                    0 as *const libc::c_char,
                    b"Could only read %lu of %lu byte\0" as *const u8
                        as *const libc::c_char,
                    b"Could only read %lu of %lu bytes\0" as *const u8
                        as *const libc::c_char,
                    bytes,
                    5 as libc::c_int,
                ),
                status,
                bytes,
            );
        }
        return 0 as libc::c_int;
    }
    if memcmp(buffer as *const libc::c_void, diff_buffer as *const libc::c_void, bytes)
        != 0
    {
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            dcgettext(
                0 as *const libc::c_char,
                b"Contents differ\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn read_and_process(
    mut st: *mut tar_stat_info,
    mut processor: Option::<
        unsafe extern "C" fn(size_t, *mut libc::c_char) -> libc::c_int,
    >,
) {
    let mut data_block: *mut block = 0 as *mut block;
    let mut data_size: size_t = 0;
    let mut size: off_t = (*st).stat.st_size;
    mv_begin_read(st);
    while size != 0 {
        data_block = find_next_block();
        if data_block.is_null() {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unexpected EOF in archive\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            exit_status = 2 as libc::c_int;
            return;
        }
        data_size = available_space_after(data_block);
        if data_size > size as libc::c_ulong {
            data_size = size as size_t;
        }
        if (Some(processor.unwrap()))
            .unwrap()(data_size, ((*data_block).buffer).as_mut_ptr()) == 0
        {
            processor = Some(
                process_noop
                    as unsafe extern "C" fn(size_t, *mut libc::c_char) -> libc::c_int,
            );
        }
        set_next_block_after(
            ((*data_block).buffer)
                .as_mut_ptr()
                .offset(data_size as isize)
                .offset(-(1 as libc::c_int as isize)) as *mut block,
        );
        size = (size as libc::c_ulong).wrapping_sub(data_size) as off_t as off_t;
        mv_size_left(size);
    }
    mv_end();
}
unsafe extern "C" fn get_stat_data(
    mut file_name: *const libc::c_char,
    mut stat_data: *mut stat,
) -> libc::c_int {
    let mut status: libc::c_int = deref_stat(file_name, stat_data);
    if status != 0 as libc::c_int {
        if *__errno_location() == 2 as libc::c_int {
            stat_warn(file_name);
        } else {
            stat_error(file_name);
        }
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            0 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn diff_dir() {
    let mut stat_data: stat = stat {
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
    if get_stat_data(current_stat_info.file_name, &mut stat_data) == 0 {
        return;
    }
    if !(stat_data.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint)
    {
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            dcgettext(
                0 as *const libc::c_char,
                b"File type differs\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else if current_stat_info.stat.st_mode
        & (0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int
            | (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                    | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                        | 0o400 as libc::c_int >> 3 as libc::c_int
                            >> 3 as libc::c_int)))) as libc::c_uint
        != stat_data.st_mode
            & (0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int
                | (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                    | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                        | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                        | (0o400 as libc::c_int
                            | 0o400 as libc::c_int >> 3 as libc::c_int
                            | 0o400 as libc::c_int >> 3 as libc::c_int
                                >> 3 as libc::c_int)))) as libc::c_uint
    {
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            dcgettext(
                0 as *const libc::c_char,
                b"Mode differs\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
unsafe extern "C" fn diff_file() {
    let mut file_name: *const libc::c_char = current_stat_info.file_name;
    let mut stat_data: stat = stat {
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
    if get_stat_data(file_name, &mut stat_data) == 0 {
        skip_member();
    } else if !(stat_data.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint)
    {
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            dcgettext(
                0 as *const libc::c_char,
                b"File type differs\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        skip_member();
    } else {
        if current_stat_info.stat.st_mode
            & (0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int
                | (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                    | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                        | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                        | (0o400 as libc::c_int
                            | 0o400 as libc::c_int >> 3 as libc::c_int
                            | 0o400 as libc::c_int >> 3 as libc::c_int
                                >> 3 as libc::c_int)))) as libc::c_uint
            != stat_data.st_mode
                & (0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int
                    | (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                        | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                        | (0o200 as libc::c_int
                            | 0o200 as libc::c_int >> 3 as libc::c_int
                            | 0o200 as libc::c_int >> 3 as libc::c_int
                                >> 3 as libc::c_int
                            | (0o400 as libc::c_int
                                | 0o400 as libc::c_int >> 3 as libc::c_int
                                | 0o400 as libc::c_int >> 3 as libc::c_int
                                    >> 3 as libc::c_int)))) as libc::c_uint
        {
            report_difference(
                &mut current_stat_info as *mut tar_stat_info,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Mode differs\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if !sys_compare_uid(&mut stat_data, &mut current_stat_info.stat) {
            report_difference(
                &mut current_stat_info as *mut tar_stat_info,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Uid differs\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if !sys_compare_gid(&mut stat_data, &mut current_stat_info.stat) {
            report_difference(
                &mut current_stat_info as *mut tar_stat_info,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Gid differs\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if tar_timespec_cmp(get_stat_mtime(&mut stat_data), current_stat_info.mtime) != 0
        {
            report_difference(
                &mut current_stat_info as *mut tar_stat_info,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Mod time differs\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        if (*current_header).header.typeflag as libc::c_int != 'S' as i32
            && stat_data.st_size != current_stat_info.stat.st_size
        {
            report_difference(
                &mut current_stat_info as *mut tar_stat_info,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Size differs\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            skip_member();
        } else {
            diff_handle = openat(chdir_fd, file_name, open_read_flags);
            if diff_handle < 0 as libc::c_int {
                open_error(file_name);
                skip_member();
                report_difference(
                    &mut current_stat_info as *mut tar_stat_info,
                    0 as *const libc::c_char,
                );
            } else {
                let mut status: libc::c_int = 0;
                if current_stat_info.is_sparse {
                    sparse_diff_file(diff_handle, &mut current_stat_info);
                } else {
                    read_and_process(
                        &mut current_stat_info,
                        Some(
                            process_rawdata
                                as unsafe extern "C" fn(
                                    size_t,
                                    *mut libc::c_char,
                                ) -> libc::c_int,
                        ),
                    );
                }
                if atime_preserve_option as libc::c_uint
                    == replace_atime_preserve as libc::c_int as libc::c_uint
                    && stat_data.st_size != 0 as libc::c_int as libc::c_long
                {
                    let mut atime: timespec = get_stat_atime(&mut stat_data);
                    if set_file_atime(diff_handle, chdir_fd, file_name, atime)
                        != 0 as libc::c_int
                    {
                        utime_error(file_name);
                    }
                }
                status = close(diff_handle);
                if status != 0 as libc::c_int {
                    close_error(file_name);
                }
            }
        }
    };
}
unsafe extern "C" fn diff_link() {
    let mut file_data: stat = stat {
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
    let mut link_data: stat = stat {
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
    if get_stat_data(current_stat_info.file_name, &mut file_data) != 0
        && get_stat_data(current_stat_info.link_name, &mut link_data) != 0
        && !sys_compare_links(&mut file_data, &mut link_data)
    {
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            dcgettext(
                0 as *const libc::c_char,
                b"Not linked to %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote_n_colon(QUOTE_ARG as libc::c_int, current_stat_info.link_name),
        );
    }
}
unsafe extern "C" fn diff_symlink() {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut len: size_t = strlen(current_stat_info.link_name);
    let mut linkbuf: *mut libc::c_char = (if len
        < ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
    {
        buf.as_mut_ptr() as *mut libc::c_void
    } else {
        xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
    }) as *mut libc::c_char;
    let mut status: ssize_t = readlinkat(
        chdir_fd,
        current_stat_info.file_name,
        linkbuf,
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    if status < 0 as libc::c_int as libc::c_long {
        if *__errno_location() == 2 as libc::c_int {
            readlink_warn(current_stat_info.file_name);
        } else {
            readlink_error(current_stat_info.file_name);
        }
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            0 as *const libc::c_char,
        );
    } else if status as libc::c_ulong != len
        || memcmp(
            current_stat_info.link_name as *const libc::c_void,
            linkbuf as *const libc::c_void,
            len,
        ) != 0 as libc::c_int
    {
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            dcgettext(
                0 as *const libc::c_char,
                b"Symlink differs\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if linkbuf != buf.as_mut_ptr() {
        rpl_free(linkbuf as *mut libc::c_void);
    }
}
unsafe extern "C" fn diff_special() {
    let mut stat_data: stat = stat {
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
    if get_stat_data(current_stat_info.file_name, &mut stat_data) == 0 {
        return;
    }
    if if (*current_header).header.typeflag as libc::c_int == '3' as i32 {
        !(stat_data.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o20000 as libc::c_int as libc::c_uint) as libc::c_int
    } else if (*current_header).header.typeflag as libc::c_int == '4' as i32 {
        !(stat_data.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o60000 as libc::c_int as libc::c_uint) as libc::c_int
    } else {
        !(stat_data.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o10000 as libc::c_int as libc::c_uint) as libc::c_int
    } != 0
    {
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            dcgettext(
                0 as *const libc::c_char,
                b"File type differs\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    if ((*current_header).header.typeflag as libc::c_int == '3' as i32
        || (*current_header).header.typeflag as libc::c_int == '4' as i32)
        && current_stat_info.stat.st_rdev != stat_data.st_rdev
    {
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            dcgettext(
                0 as *const libc::c_char,
                b"Device number differs\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    if current_stat_info.stat.st_mode
        & (0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int
            | (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                    | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                        | 0o400 as libc::c_int >> 3 as libc::c_int
                            >> 3 as libc::c_int)))) as libc::c_uint
        != stat_data.st_mode
            & (0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int
                | (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                    | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                        | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                        | (0o400 as libc::c_int
                            | 0o400 as libc::c_int >> 3 as libc::c_int
                            | 0o400 as libc::c_int >> 3 as libc::c_int
                                >> 3 as libc::c_int)))) as libc::c_uint
    {
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            dcgettext(
                0 as *const libc::c_char,
                b"Mode differs\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
unsafe extern "C" fn dumpdir_cmp(
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
) -> libc::c_int {
    let mut len: size_t = 0;
    while *a != 0 {
        match *a as libc::c_int {
            89 | 78 => {
                if (strchr(
                    b"YN\0" as *const u8 as *const libc::c_char,
                    *b as libc::c_int,
                ))
                    .is_null()
                {
                    return 1 as libc::c_int;
                }
                if strcmp(
                    a.offset(1 as libc::c_int as isize),
                    b.offset(1 as libc::c_int as isize),
                ) != 0
                {
                    return 1 as libc::c_int;
                }
                len = (strlen(a)).wrapping_add(1 as libc::c_int as libc::c_ulong);
                a = a.offset(len as isize);
                b = b.offset(len as isize);
            }
            68 => {
                if strcmp(a, b) != 0 {
                    return 1 as libc::c_int;
                }
                len = (strlen(a)).wrapping_add(1 as libc::c_int as libc::c_ulong);
                a = a.offset(len as isize);
                b = b.offset(len as isize);
            }
            82 | 84 | 88 => return *b as libc::c_int,
            _ => {}
        }
    }
    return *b as libc::c_int;
}
unsafe extern "C" fn diff_dumpdir(mut dir: *mut tar_stat_info) {
    let mut dumpdir_buffer: *const libc::c_char = 0 as *const libc::c_char;
    if (*dir).fd == 0 as libc::c_int {
        let mut diag: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()> = None;
        let mut fd: libc::c_int = subfile_open(
            (*dir).parent,
            (*dir).orig_file_name,
            open_read_flags,
        );
        if fd < 0 as libc::c_int {
            diag = Some(open_diag as unsafe extern "C" fn(*const libc::c_char) -> ());
        } else if fstat(fd, &mut (*dir).stat) != 0 {
            diag = Some(stat_diag as unsafe extern "C" fn(*const libc::c_char) -> ());
            close(fd);
        } else {
            (*dir).fd = fd;
        }
        if diag.is_some() {
            file_removed_diag((*dir).orig_file_name, 0 as libc::c_int != 0, diag);
            return;
        }
    }
    dumpdir_buffer = directory_contents(scan_directory(dir));
    if !dumpdir_buffer.is_null() {
        if dumpdir_cmp((*dir).dumpdir, dumpdir_buffer) != 0 {
            report_difference(
                dir,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Contents differ\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    } else {
        read_and_process(
            dir,
            Some(
                process_noop
                    as unsafe extern "C" fn(size_t, *mut libc::c_char) -> libc::c_int,
            ),
        );
    };
}
unsafe extern "C" fn diff_multivol() {
    let mut stat_data: stat = stat {
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
    let mut status: libc::c_int = 0;
    let mut offset: off_t = 0;
    if current_stat_info.had_trailing_slash {
        diff_dir();
        return;
    }
    if get_stat_data(current_stat_info.file_name, &mut stat_data) == 0 {
        return;
    }
    if !(stat_data.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint)
    {
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            dcgettext(
                0 as *const libc::c_char,
                b"File type differs\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        skip_member();
        return;
    }
    offset = off_from_header(
        ((*current_header).oldgnu_header.offset).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    );
    if offset < 0 as libc::c_int as libc::c_long
        || (if (if ((if 1 as libc::c_int != 0 {
            0 as libc::c_int as libc::c_long
        } else {
            (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                current_stat_info.stat.st_size
            }) + offset
        }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
        {
            !(((((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    current_stat_info.stat.st_size
                }) + offset
            }) + 1 as libc::c_int as libc::c_long)
                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long)
        } else {
            (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    current_stat_info.stat.st_size
                }) + offset
            }) + 0 as libc::c_int as libc::c_long
        }) < 0 as libc::c_int as libc::c_long
        {
            if offset < 0 as libc::c_int as libc::c_long {
                (current_stat_info.stat.st_size
                    < (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            current_stat_info.stat.st_size
                        }) + offset
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                current_stat_info.stat.st_size
                            }) + offset
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                current_stat_info.stat.st_size
                            }) + offset
                        }) + 0 as libc::c_int as libc::c_long
                    }) - offset) as libc::c_int
            } else {
                ((if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        current_stat_info.stat.st_size
                    }) + offset
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    ((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            current_stat_info.stat.st_size
                        }) + offset
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            current_stat_info.stat.st_size
                        }) + offset
                    }) - 1 as libc::c_int as libc::c_long
                }) - offset < current_stat_info.stat.st_size) as libc::c_int
            }
        } else {
            if current_stat_info.stat.st_size < 0 as libc::c_int as libc::c_long {
                (offset <= current_stat_info.stat.st_size + offset) as libc::c_int
            } else {
                if offset < 0 as libc::c_int as libc::c_long {
                    (current_stat_info.stat.st_size
                        <= current_stat_info.stat.st_size + offset) as libc::c_int
                } else {
                    (current_stat_info.stat.st_size + offset < offset) as libc::c_int
                }
            }
        }) != 0 || stat_data.st_size != current_stat_info.stat.st_size + offset
    {
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            dcgettext(
                0 as *const libc::c_char,
                b"Size differs\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        skip_member();
        return;
    }
    fd = openat(chdir_fd, current_stat_info.file_name, open_read_flags);
    if fd < 0 as libc::c_int {
        open_error(current_stat_info.file_name);
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            0 as *const libc::c_char,
        );
        skip_member();
        return;
    }
    if lseek(fd, offset, 0 as libc::c_int) < 0 as libc::c_int as libc::c_long {
        seek_error_details(current_stat_info.file_name, offset);
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            0 as *const libc::c_char,
        );
    } else {
        read_and_process(
            &mut current_stat_info,
            Some(
                process_rawdata
                    as unsafe extern "C" fn(size_t, *mut libc::c_char) -> libc::c_int,
            ),
        );
    }
    status = close(fd);
    if status != 0 as libc::c_int {
        close_error(current_stat_info.file_name);
    }
}
pub unsafe extern "C" fn diff_archive() {
    set_next_block_after(current_header);
    if verbose_option != 0 {
        if now_verifying {
            fprintf(
                stdlis,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Verify \0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        print_header(
            &mut current_stat_info,
            current_header,
            -(1 as libc::c_int) as off_t,
        );
    }
    let mut current_block_22: u64;
    match (*current_header).header.typeflag as libc::c_int {
        0 | 48 | 83 | 55 => {
            current_block_22 = 10304134390680427967;
        }
        49 => {
            diff_link();
            current_block_22 = 18386322304582297246;
        }
        50 => {
            diff_symlink();
            current_block_22 = 18386322304582297246;
        }
        51 | 52 | 54 => {
            diff_special();
            current_block_22 = 18386322304582297246;
        }
        68 | 53 => {
            if is_dumpdir(&mut current_stat_info) {
                diff_dumpdir(&mut current_stat_info);
            }
            diff_dir();
            current_block_22 = 18386322304582297246;
        }
        86 => {
            current_block_22 = 18386322304582297246;
        }
        77 => {
            diff_multivol();
            current_block_22 = 18386322304582297246;
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
                    b"%s: Unknown file type '%c', diffed as normal file\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_colon(current_stat_info.file_name),
                (*current_header).header.typeflag as libc::c_int,
            );
            exit_status = 2 as libc::c_int;
            current_block_22 = 10304134390680427967;
        }
    }
    match current_block_22 {
        10304134390680427967 => {
            if current_stat_info.had_trailing_slash {
                diff_dir();
            } else {
                diff_file();
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn verify_volume() {
    let mut may_fail: libc::c_int = 0 as libc::c_int;
    if removed_prefixes_p() {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Archive contains file names with leading prefixes removed.\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        may_fail = 1 as libc::c_int;
    }
    if transform_program_p() {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Archive contains transformed file names.\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        may_fail = 1 as libc::c_int;
    }
    if may_fail != 0 {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Verification may fail to locate original files.\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    clear_directory_table();
    if diff_buffer.is_null() {
        diff_init();
    }
    fsync(archive);
    ioctl(
        archive,
        ((0 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | ((2 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((0x4b as libc::c_int) << 0 as libc::c_int) as libc::c_uint
            | ((0 as libc::c_int)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int)
                as libc::c_uint) as libc::c_ulong,
    );
    let mut operation: mtop = mtop { mt_op: 0, mt_count: 0 };
    let mut status: libc::c_int = 0;
    operation.mt_op = 2 as libc::c_int as libc::c_short;
    operation.mt_count = 1 as libc::c_int;
    status = if archive >= (1 as libc::c_int) << 30 as libc::c_int {
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
    };
    if status < 0 as libc::c_int {
        if *__errno_location() != 5 as libc::c_int
            || {
                status = if archive >= (1 as libc::c_int) << 30 as libc::c_int {
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
                };
                status < 0 as libc::c_int
            }
        {
            if (if archive >= (1 as libc::c_int) << 30 as libc::c_int {
                rmt_lseek__(
                    archive - ((1 as libc::c_int) << 30 as libc::c_int),
                    0 as libc::c_int as off_t,
                    0 as libc::c_int,
                )
            } else {
                lseek(archive, 0 as libc::c_int as off_t, 0 as libc::c_int)
            }) != 0 as libc::c_int as libc::c_long
            {
                seek_warn(*archive_name_array.offset(0 as libc::c_int as isize));
                return;
            }
        }
    }
    access_mode = ACCESS_READ;
    now_verifying = 1 as libc::c_int != 0;
    flush_read();
    loop {
        let mut status_0: read_header = read_header(
            &mut current_header,
            &mut current_stat_info,
            read_header_auto,
        );
        if status_0 as libc::c_uint == HEADER_FAILURE as libc::c_int as libc::c_uint {
            let mut counter: libc::c_int = 0 as libc::c_int;
            loop {
                counter += 1;
                counter;
                set_next_block_after(current_header);
                status_0 = read_header(
                    &mut current_header,
                    &mut current_stat_info,
                    read_header_auto,
                );
                if !(status_0 as libc::c_uint
                    == HEADER_FAILURE as libc::c_int as libc::c_uint)
                {
                    break;
                }
            }
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcngettext(
                    0 as *const libc::c_char,
                    b"VERIFY FAILURE: %d invalid header detected\0" as *const u8
                        as *const libc::c_char,
                    b"VERIFY FAILURE: %d invalid headers detected\0" as *const u8
                        as *const libc::c_char,
                    counter as libc::c_ulong,
                    5 as libc::c_int,
                ),
                counter,
            );
            exit_status = 2 as libc::c_int;
        }
        if status_0 as libc::c_uint == HEADER_END_OF_FILE as libc::c_int as libc::c_uint
        {
            break;
        }
        if status_0 as libc::c_uint == HEADER_ZERO_BLOCK as libc::c_int as libc::c_uint {
            set_next_block_after(current_header);
            if ignore_zeros_option {
                continue;
            }
            let mut buf: [libc::c_char; 21] = [0; 21];
            status_0 = read_header(
                &mut current_header,
                &mut current_stat_info,
                read_header_auto,
            );
            if status_0 as libc::c_uint
                == HEADER_ZERO_BLOCK as libc::c_int as libc::c_uint
            {
                break;
            }
            if warning_option & 0x1 as libc::c_int != 0 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"A lone zero block at %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    umaxtostr(current_block_ordinal() as uintmax_t, buf.as_mut_ptr()),
                );
            }
        } else {
            decode_header(
                current_header,
                &mut current_stat_info,
                &mut current_format,
                1 as libc::c_int,
            );
            diff_archive();
            tar_stat_destroy(&mut current_stat_info);
        }
    }
    access_mode = ACCESS_WRITE;
    now_verifying = 0 as libc::c_int != 0;
}
