use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type quoting_options;
    pub type exclist;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn abort() -> !;
    fn rpl_free(ptr: *mut libc::c_void);
    fn safer_name_suffix(
        file_name: *const libc::c_char,
        link_target: bool,
        absolute_names: bool,
    ) -> *mut libc::c_char;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn fatal_exit() -> !;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn strip_trailing_slashes(file: *mut libc::c_char) -> bool;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    static mut error_hook: Option::<unsafe extern "C" fn() -> ()>;
    static mut exit_status: libc::c_int;
    fn pax_decode_mode(mode: mode_t, string: *mut libc::c_char);
    fn clone_quoting_options(o: *mut quoting_options) -> *mut quoting_options;
    fn set_quoting_style(o: *mut quoting_options, s: quoting_style);
    fn quotearg_buffer(
        buffer: *mut libc::c_char,
        buffersize: size_t,
        arg: *const libc::c_char,
        argsize: size_t,
        o: *const quoting_options,
    ) -> size_t;
    fn quotearg(arg: *const libc::c_char) -> *mut libc::c_char;
    fn quotearg_colon(arg: *const libc::c_char) -> *mut libc::c_char;
    static mut absolute_names_option: bool;
    static mut utc_option: bool;
    static mut full_time_option: bool;
    static mut block_number_option: bool;
    static mut ignore_zeros_option: bool;
    static mut incremental_option: bool;
    static mut multi_volume_option: bool;
    static mut newer_mtime_option: timespec;
    static mut numeric_owner_option: bool;
    static mut one_top_level_option: bool;
    static mut one_top_level_dir: *mut libc::c_char;
    static mut strip_name_components: size_t;
    static mut show_omitted_dirs_option: bool;
    static mut verbose_option: libc::c_int;
    static mut start_time: timespec;
    static mut current_stat_info: tar_stat_info;
    static mut seekable_archive: bool;
    static mut show_transformed_names_option: bool;
    static mut stdlis: *mut FILE;
    static mut volume_label: *mut libc::c_char;
    fn drop_volume_label_suffix(label: *const libc::c_char) -> *mut libc::c_char;
    fn available_space_after(pointer: *mut block) -> size_t;
    fn current_block_ordinal() -> off_t;
    fn close_archive();
    fn find_next_block() -> *mut block;
    fn open_archive(mode: access_mode);
    fn set_next_block_after(block: *mut block);
    fn seek_archive(size: off_t) -> off_t;
    fn mv_begin_read(st: *mut tar_stat_info);
    fn mv_end();
    fn mv_size_left(size: off_t);
    fn list_dumpdir(buffer: *mut libc::c_char, size: size_t);
    fn dumpdir_size(p: *const libc::c_char) -> size_t;
    fn is_dumpdir(stat_info: *mut tar_stat_info) -> bool;
    fn sparse_fixup_header(st: *mut tar_stat_info) -> bool;
    fn sparse_member_p(st: *mut tar_stat_info) -> bool;
    fn xheader_decode(stat: *mut tar_stat_info);
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn gname_to_gid(gname: *const libc::c_char, pgid: *mut gid_t) -> libc::c_int;
    fn uname_to_uid(uname: *const libc::c_char, puid: *mut uid_t) -> libc::c_int;
    fn xheader_xattr_init(st: *mut tar_stat_info);
    fn assign_string_n(
        string: *mut *mut libc::c_char,
        value: *const libc::c_char,
        n: size_t,
    );
    fn normalize_filename_x(name: *mut libc::c_char);
    fn make_file_name(
        dir_name: *const libc::c_char,
        name: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn stripped_prefix_len(file_name: *const libc::c_char, num: size_t) -> size_t;
    fn transform_name_fp(
        pinput: *mut *mut libc::c_char,
        type_0: libc::c_int,
        fun: Option::<
            unsafe extern "C" fn(
                *mut libc::c_char,
                *mut libc::c_void,
            ) -> *mut libc::c_char,
        >,
        _: *mut libc::c_void,
    ) -> bool;
    fn code_ns_fraction(ns: libc::c_int, p: *mut libc::c_char);
    fn sparse_skip_file(st: *mut tar_stat_info) -> dump_status;
    fn xattrs_print(st: *const tar_stat_info);
    fn xattrs_print_char(st: *const tar_stat_info, output: *mut libc::c_char);
    fn tar_stat_destroy(st: *mut tar_stat_info);
    fn assign_string(dest: *mut *mut libc::c_char, src: *const libc::c_char);
    fn tar_stat_init(st: *mut tar_stat_info);
    fn label_notfound();
    fn name_match(name: *const libc::c_char) -> bool;
    fn xheader_destroy(hdr: *mut xheader);
    fn xheader_decode_global(xhdr: *mut xheader);
    fn xheader_read(xhdr: *mut xheader, header: *mut block, size: off_t);
    fn name_gather();
    fn names_notfound();
    fn all_names_found(st: *mut tar_stat_info) -> bool;
    static mut warning_option: libc::c_int;
    fn excluded_name(name: *const libc::c_char, st: *mut tar_stat_info) -> bool;
}
pub type __intmax_t = libc::c_long;
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
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
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
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type DIR = __dirstream;
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
pub type access_mode = libc::c_uint;
pub const ACCESS_UPDATE: access_mode = 2;
pub const ACCESS_WRITE: access_mode = 1;
pub const ACCESS_READ: access_mode = 0;
pub type dump_status = libc::c_uint;
pub const dump_status_not_implemented: dump_status = 3;
pub const dump_status_fail: dump_status = 2;
pub const dump_status_short: dump_status = 1;
pub const dump_status_ok: dump_status = 0;
pub type read_header = libc::c_uint;
pub const HEADER_FAILURE: read_header = 5;
pub const HEADER_END_OF_FILE: read_header = 4;
pub const HEADER_ZERO_BLOCK: read_header = 3;
pub const HEADER_SUCCESS_EXTENDED: read_header = 2;
pub const HEADER_SUCCESS: read_header = 1;
pub const HEADER_STILL_UNREAD: read_header = 0;
pub type read_header_mode = libc::c_uint;
pub const read_header_x_global: read_header_mode = 2;
pub const read_header_x_raw: read_header_mode = 1;
pub const read_header_auto: read_header_mode = 0;
pub const fraclen: C2RustUnnamed_3 = 10;
pub type C2RustUnnamed_3 = libc::c_uint;
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
unsafe extern "C" fn gnu_dev_makedev(
    mut __major: libc::c_uint,
    mut __minor: libc::c_uint,
) -> __dev_t {
    let mut __dev: __dev_t = 0;
    __dev = ((__major & 0xfff as libc::c_uint) as __dev_t) << 8 as libc::c_int;
    __dev |= ((__major & 0xfffff000 as libc::c_uint) as __dev_t) << 32 as libc::c_int;
    __dev |= ((__minor & 0xff as libc::c_uint) as __dev_t) << 0 as libc::c_int;
    __dev |= ((__minor & 0xffffff00 as libc::c_uint) as __dev_t) << 12 as libc::c_int;
    return __dev;
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
unsafe extern "C" fn putc_unlocked(
    mut __c: libc::c_int,
    mut __stream: *mut FILE,
) -> libc::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(__stream, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh1 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh1 = __c as libc::c_char;
        *fresh1 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn timespec_cmp(mut a: timespec, mut b: timespec) -> libc::c_int {
    return 2 as libc::c_int
        * ((a.tv_sec > b.tv_sec) as libc::c_int - (a.tv_sec < b.tv_sec) as libc::c_int)
        + ((a.tv_nsec > b.tv_nsec) as libc::c_int
            - (a.tv_nsec < b.tv_nsec) as libc::c_int);
}
#[inline]
unsafe extern "C" fn represent_uintmax(mut n: uintmax_t) -> intmax_t {
    if n <= 9223372036854775807 as libc::c_long as libc::c_ulong {
        return n as intmax_t
    } else {
        let mut nd: intmax_t = n
            .wrapping_sub(
                (-(9223372036854775807 as libc::c_long)
                    - 1 as libc::c_int as libc::c_long) as libc::c_ulong,
            ) as intmax_t;
        return nd
            + (-(9223372036854775807 as libc::c_long)
                - 1 as libc::c_int as libc::c_long);
    };
}
pub static mut current_header: *mut block = 0 as *const block as *mut block;
pub static mut current_format: archive_format = DEFAULT_FORMAT;
pub static mut recent_long_name: *mut block = 0 as *const block as *mut block;
pub static mut recent_long_link: *mut block = 0 as *const block as *mut block;
pub static mut recent_long_name_blocks: size_t = 0;
pub static mut recent_long_link_blocks: size_t = 0;
static mut recent_global_header: *mut block = 0 as *const block as *mut block;
static mut base_64_digits: [libc::c_char; 64] = [
    'A' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
];
static mut base64_map: [libc::c_char; 256] = [0; 256];
unsafe extern "C" fn base64_init() {
    let mut i: libc::c_int = 0;
    memset(
        base64_map.as_mut_ptr() as *mut libc::c_void,
        64 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        base64_map[base_64_digits[i as usize] as libc::c_int
            as usize] = i as libc::c_char;
        i += 1;
        i;
    }
}
unsafe extern "C" fn decode_xform(
    mut file_name: *mut libc::c_char,
    mut data: *mut libc::c_void,
) -> *mut libc::c_char {
    let mut type_0: libc::c_int = *(data as *mut libc::c_int);
    match type_0 {
        4 => return file_name,
        2 => {
            file_name = safer_name_suffix(
                file_name,
                1 as libc::c_int != 0,
                absolute_names_option,
            );
        }
        1 => {
            file_name = safer_name_suffix(
                file_name,
                0 as libc::c_int != 0,
                absolute_names_option,
            );
        }
        _ => {}
    }
    if strip_name_components != 0 {
        let mut prefix_len: size_t = stripped_prefix_len(
            file_name,
            strip_name_components,
        );
        if prefix_len == -(1 as libc::c_int) as size_t {
            prefix_len = strlen(file_name);
        }
        file_name = file_name.offset(prefix_len as isize);
    }
    return file_name;
}
unsafe extern "C" fn transform_member_name(
    mut pinput: *mut *mut libc::c_char,
    mut type_0: libc::c_int,
) -> bool {
    return transform_name_fp(
        pinput,
        type_0,
        Some(
            decode_xform
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *mut libc::c_void,
                ) -> *mut libc::c_char,
        ),
        &mut type_0 as *mut libc::c_int as *mut libc::c_void,
    );
}
unsafe extern "C" fn enforce_one_top_level(mut pfile_name: *mut *mut libc::c_char) {
    let mut file_name: *mut libc::c_char = *pfile_name;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = file_name;
    while *p as libc::c_int != 0
        && (*p as libc::c_int == '/' as i32 || *p as libc::c_int == '.' as i32)
    {
        p = p.offset(1);
        p;
    }
    if *p != 0 {
        let mut pos: libc::c_int = strlen(one_top_level_dir) as libc::c_int;
        if strncmp(p, one_top_level_dir, pos as libc::c_ulong) == 0 as libc::c_int {
            if *p.offset(pos as isize) as libc::c_int == '/' as i32
                || *p.offset(pos as isize) as libc::c_int == 0 as libc::c_int
            {
                return;
            }
        }
        *pfile_name = make_file_name(one_top_level_dir, file_name);
        normalize_filename_x(*pfile_name);
    } else {
        *pfile_name = xstrdup(one_top_level_dir);
    }
    rpl_free(file_name as *mut libc::c_void);
}
pub unsafe extern "C" fn transform_stat_info(
    mut typeflag: libc::c_int,
    mut stat_info: *mut tar_stat_info,
) {
    if typeflag == 'V' as i32 {
        return;
    }
    transform_member_name(&mut (*stat_info).file_name, 0x1 as libc::c_int);
    match typeflag {
        50 => {
            transform_member_name(&mut (*stat_info).link_name, 0x4 as libc::c_int);
        }
        49 => {
            transform_member_name(&mut (*stat_info).link_name, 0x2 as libc::c_int);
        }
        _ => {}
    }
    if one_top_level_option {
        enforce_one_top_level(&mut current_stat_info.file_name);
    }
}
pub unsafe extern "C" fn read_and(
    mut do_something: Option::<unsafe extern "C" fn() -> ()>,
) {
    let mut status: read_header = HEADER_STILL_UNREAD;
    let mut prev_status: read_header = HEADER_STILL_UNREAD;
    let mut mtime: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    base64_init();
    name_gather();
    open_archive(ACCESS_READ);
    let mut current_block_55: u64;
    loop {
        prev_status = status;
        tar_stat_destroy(&mut current_stat_info);
        status = read_header(
            &mut current_header,
            &mut current_stat_info,
            read_header_auto,
        );
        match status as libc::c_uint {
            0 | 2 => {
                abort();
            }
            1 => {
                decode_header(
                    current_header,
                    &mut current_stat_info,
                    &mut current_format,
                    1 as libc::c_int,
                );
                if !name_match(current_stat_info.file_name)
                    || 0 as libc::c_int as libc::c_long <= newer_mtime_option.tv_nsec
                        && {
                            mtime
                                .tv_sec = time_from_header(
                                ((*current_header).header.mtime).as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
                            );
                            mtime.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
                            current_stat_info.mtime = mtime;
                            timespec_cmp(current_stat_info.mtime, newer_mtime_option)
                                < 0 as libc::c_int
                        }
                    || excluded_name(
                        current_stat_info.file_name,
                        current_stat_info.parent,
                    ) as libc::c_int != 0
                {
                    match (*current_header).header.typeflag as libc::c_int {
                        86 | 77 => {
                            current_block_55 = 6057473163062296781;
                        }
                        53 => {
                            current_block_55 = 10029153119333114254;
                            match current_block_55 {
                                10029153119333114254 => {
                                    if show_omitted_dirs_option {
                                        if error_hook.is_some() {
                                            error_hook.unwrap()();
                                        }
                                        error(
                                            0 as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"%s: Omitting\0" as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            quotearg_colon(current_stat_info.file_name),
                                        );
                                    }
                                }
                                _ => {}
                            }
                            skip_member();
                            current_block_55 = 12675440807659640239;
                        }
                        _ => {
                            current_block_55 = 17110730540101275301;
                            match current_block_55 {
                                10029153119333114254 => {
                                    if show_omitted_dirs_option {
                                        if error_hook.is_some() {
                                            error_hook.unwrap()();
                                        }
                                        error(
                                            0 as libc::c_int,
                                            0 as libc::c_int,
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"%s: Omitting\0" as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            quotearg_colon(current_stat_info.file_name),
                                        );
                                    }
                                }
                                _ => {}
                            }
                            skip_member();
                            current_block_55 = 12675440807659640239;
                        }
                    }
                } else {
                    current_block_55 = 6057473163062296781;
                }
                match current_block_55 {
                    12675440807659640239 => {}
                    _ => {
                        transform_stat_info(
                            (*current_header).header.typeflag as libc::c_int,
                            &mut current_stat_info,
                        );
                        (Some(do_something.unwrap())).unwrap()();
                    }
                }
            }
            3 => {
                if block_number_option {
                    let mut buf: [libc::c_char; 21] = [0; 21];
                    fprintf(
                        stdlis,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"block %s: ** Block of NULs **\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        umaxtostr(current_block_ordinal() as uintmax_t, buf.as_mut_ptr()),
                    );
                }
                set_next_block_after(current_header);
                if !ignore_zeros_option {
                    let mut buf_0: [libc::c_char; 21] = [0; 21];
                    status = read_header(
                        &mut current_header,
                        &mut current_stat_info,
                        read_header_auto,
                    );
                    if status as libc::c_uint
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
                                b"A lone zero block at %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            umaxtostr(
                                current_block_ordinal() as uintmax_t,
                                buf_0.as_mut_ptr(),
                            ),
                        );
                    }
                    break;
                } else {
                    status = prev_status;
                }
            }
            4 => {
                if block_number_option {
                    let mut buf_1: [libc::c_char; 21] = [0; 21];
                    fprintf(
                        stdlis,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"block %s: ** End of File **\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        umaxtostr(
                            current_block_ordinal() as uintmax_t,
                            buf_1.as_mut_ptr(),
                        ),
                    );
                }
                break;
            }
            5 => {
                set_next_block_after(current_header);
                let mut current_block_54: u64;
                match prev_status as libc::c_uint {
                    0 => {
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
                        current_block_54 = 15646275005637401879;
                    }
                    3 | 1 => {
                        current_block_54 = 15646275005637401879;
                    }
                    2 => {
                        abort();
                    }
                    4 | 5 | _ => {
                        current_block_54 = 168769493162332264;
                    }
                }
                match current_block_54 {
                    15646275005637401879 => {
                        if block_number_option {
                            let mut buf_2: [libc::c_char; 21] = [0; 21];
                            let mut block_ordinal: off_t = current_block_ordinal();
                            block_ordinal = (block_ordinal as libc::c_ulong)
                                .wrapping_sub(recent_long_name_blocks) as off_t as off_t;
                            block_ordinal = (block_ordinal as libc::c_ulong)
                                .wrapping_sub(recent_long_link_blocks) as off_t as off_t;
                            fprintf(
                                stdlis,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"block %s: \0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                umaxtostr(block_ordinal as uintmax_t, buf_2.as_mut_ptr()),
                            );
                        }
                        if error_hook.is_some() {
                            error_hook.unwrap()();
                        }
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Skipping to next header\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        exit_status = 2 as libc::c_int;
                    }
                    _ => {}
                }
            }
            _ => {
                break;
            }
        }
        if all_names_found(&mut current_stat_info) {
            break;
        }
    }
    close_archive();
    names_notfound();
}
pub unsafe extern "C" fn list_archive() {
    let mut block_ordinal: off_t = current_block_ordinal();
    if verbose_option != 0 {
        print_header(&mut current_stat_info, current_header, block_ordinal);
    }
    if incremental_option {
        if verbose_option > 2 as libc::c_int {
            if is_dumpdir(&mut current_stat_info) {
                list_dumpdir(
                    current_stat_info.dumpdir,
                    dumpdir_size(current_stat_info.dumpdir),
                );
            }
        }
    }
    skip_member();
}
pub unsafe extern "C" fn tar_checksum(
    mut header: *mut block,
    mut silent: bool,
) -> read_header {
    let mut i: size_t = 0;
    let mut unsigned_sum: libc::c_int = 0 as libc::c_int;
    let mut signed_sum: libc::c_int = 0 as libc::c_int;
    let mut recorded_sum: libc::c_int = 0;
    let mut parsed_sum: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = ((*header).buffer).as_mut_ptr();
    i = ::std::mem::size_of::<block>() as libc::c_ulong;
    loop {
        let fresh2 = i;
        i = i.wrapping_sub(1);
        if !(fresh2 != 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        unsigned_sum += *p as libc::c_uchar as libc::c_int;
        let fresh3 = p;
        p = p.offset(1);
        signed_sum += *fresh3 as libc::c_schar as libc::c_int;
    }
    if unsigned_sum == 0 as libc::c_int {
        return HEADER_ZERO_BLOCK;
    }
    i = ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong;
    loop {
        let fresh4 = i;
        i = i.wrapping_sub(1);
        if !(fresh4 != 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        unsigned_sum
            -= (*header).header.chksum[i as usize] as libc::c_uchar as libc::c_int;
        signed_sum
            -= (*header).header.chksum[i as usize] as libc::c_schar as libc::c_int;
    }
    unsigned_sum = (unsigned_sum as libc::c_ulong)
        .wrapping_add(
            (' ' as i32 as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                ),
        ) as libc::c_int as libc::c_int;
    signed_sum = (signed_sum as libc::c_ulong)
        .wrapping_add(
            (' ' as i32 as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                ),
        ) as libc::c_int as libc::c_int;
    parsed_sum = from_header(
        ((*header).header.chksum).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        0 as *const libc::c_char,
        0 as libc::c_int as intmax_t,
        2147483647 as libc::c_int as uintmax_t,
        1 as libc::c_int != 0,
        silent,
    ) as libc::c_int;
    if parsed_sum < 0 as libc::c_int {
        return HEADER_FAILURE;
    }
    recorded_sum = parsed_sum;
    if unsigned_sum != recorded_sum && signed_sum != recorded_sum {
        return HEADER_FAILURE;
    }
    return HEADER_SUCCESS;
}
pub unsafe extern "C" fn read_header(
    mut return_block: *mut *mut block,
    mut info: *mut tar_stat_info,
    mut mode: read_header_mode,
) -> read_header {
    let mut header: *mut block = 0 as *mut block;
    let mut bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut data_block: *mut block = 0 as *mut block;
    let mut size: size_t = 0;
    let mut written: size_t = 0;
    let mut next_long_name: *mut block = 0 as *mut block;
    let mut next_long_link: *mut block = 0 as *mut block;
    let mut next_long_name_blocks: size_t = 0 as libc::c_int as size_t;
    let mut next_long_link_blocks: size_t = 0 as libc::c_int as size_t;
    let mut status: read_header = HEADER_SUCCESS;
    loop {
        header = find_next_block();
        *return_block = header;
        if header.is_null() {
            status = HEADER_END_OF_FILE;
            break;
        } else {
            status = tar_checksum(header, 0 as libc::c_int != 0);
            if status as libc::c_uint != HEADER_SUCCESS as libc::c_int as libc::c_uint {
                break;
            }
            if (*header).header.typeflag as libc::c_int == '1' as i32 {
                (*info).stat.st_size = 0 as libc::c_int as __off_t;
            } else {
                (*info)
                    .stat
                    .st_size = off_from_header(
                    ((*header).header.size).as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
                );
                if (*info).stat.st_size < 0 as libc::c_int as libc::c_long {
                    status = HEADER_FAILURE;
                    break;
                }
            }
            if (*header).header.typeflag as libc::c_int == 'L' as i32
                || (*header).header.typeflag as libc::c_int == 'K' as i32
                || (*header).header.typeflag as libc::c_int == 'x' as i32
                || (*header).header.typeflag as libc::c_int == 'g' as i32
                || (*header).header.typeflag as libc::c_int == 'X' as i32
            {
                if mode as libc::c_uint
                    == read_header_x_raw as libc::c_int as libc::c_uint
                {
                    status = HEADER_SUCCESS_EXTENDED;
                    break;
                } else if (*header).header.typeflag as libc::c_int == 'L' as i32
                    || (*header).header.typeflag as libc::c_int == 'K' as i32
                {
                    let mut header_copy: *mut block = 0 as *mut block;
                    let mut name_size: size_t = (*info).stat.st_size as size_t;
                    let mut n: size_t = name_size
                        .wrapping_rem(512 as libc::c_int as libc::c_ulong);
                    size = name_size.wrapping_add(512 as libc::c_int as libc::c_ulong);
                    if n != 0 {
                        size = (size as libc::c_ulong)
                            .wrapping_add(
                                (512 as libc::c_int as libc::c_ulong).wrapping_sub(n),
                            ) as size_t as size_t;
                    }
                    if name_size != (*info).stat.st_size as libc::c_ulong
                        || size < name_size
                    {
                        xalloc_die();
                    }
                    header_copy = xmalloc(
                        size.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as *mut block;
                    if (*header).header.typeflag as libc::c_int == 'L' as i32 {
                        rpl_free(next_long_name as *mut libc::c_void);
                        next_long_name = header_copy;
                        next_long_name_blocks = size
                            .wrapping_div(512 as libc::c_int as libc::c_ulong);
                    } else {
                        rpl_free(next_long_link as *mut libc::c_void);
                        next_long_link = header_copy;
                        next_long_link_blocks = size
                            .wrapping_div(512 as libc::c_int as libc::c_ulong);
                    }
                    set_next_block_after(header);
                    *header_copy = *header;
                    bp = ((*header_copy).buffer)
                        .as_mut_ptr()
                        .offset(512 as libc::c_int as isize);
                    size = (size as libc::c_ulong)
                        .wrapping_sub(512 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                    while size > 0 as libc::c_int as libc::c_ulong {
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
                                    b"Unexpected EOF in archive\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            exit_status = 2 as libc::c_int;
                            break;
                        } else {
                            written = available_space_after(data_block);
                            if written > size {
                                written = size;
                            }
                            memcpy(
                                bp as *mut libc::c_void,
                                ((*data_block).buffer).as_mut_ptr() as *const libc::c_void,
                                written,
                            );
                            bp = bp.offset(written as isize);
                            set_next_block_after(
                                ((*data_block).buffer)
                                    .as_mut_ptr()
                                    .offset(written as isize)
                                    .offset(-(1 as libc::c_int as isize)) as *mut block,
                            );
                            size = (size as libc::c_ulong).wrapping_sub(written)
                                as size_t as size_t;
                        }
                    }
                    *bp = '\0' as i32 as libc::c_char;
                } else if (*header).header.typeflag as libc::c_int == 'x' as i32
                    || (*header).header.typeflag as libc::c_int == 'X' as i32
                {
                    xheader_read(
                        &mut (*info).xhdr,
                        header,
                        off_from_header(
                            ((*header).header.size).as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
                        ),
                    );
                } else {
                    if !((*header).header.typeflag as libc::c_int == 'g' as i32) {
                        continue;
                    }
                    let mut xhdr: xheader = xheader {
                        stk: 0 as *const obstack as *mut obstack,
                        size: 0,
                        buffer: 0 as *const libc::c_char as *mut libc::c_char,
                        string_length: 0,
                    };
                    if recent_global_header.is_null() {
                        recent_global_header = xmalloc(
                            ::std::mem::size_of::<block>() as libc::c_ulong,
                        ) as *mut block;
                    }
                    memcpy(
                        recent_global_header as *mut libc::c_void,
                        header as *const libc::c_void,
                        ::std::mem::size_of::<block>() as libc::c_ulong,
                    );
                    memset(
                        &mut xhdr as *mut xheader as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<xheader>() as libc::c_ulong,
                    );
                    xheader_read(
                        &mut xhdr,
                        header,
                        off_from_header(
                            ((*header).header.size).as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
                        ),
                    );
                    xheader_decode_global(&mut xhdr);
                    xheader_destroy(&mut xhdr);
                    if !(mode as libc::c_uint
                        == read_header_x_global as libc::c_int as libc::c_uint)
                    {
                        continue;
                    }
                    status = HEADER_SUCCESS_EXTENDED;
                    break;
                }
            } else {
                let mut name: *const libc::c_char = 0 as *const libc::c_char;
                let mut h: *const posix_header = &mut (*header).header;
                let mut namebuf: [libc::c_char; 257] = [0; 257];
                rpl_free(recent_long_name as *mut libc::c_void);
                if !next_long_name.is_null() {
                    name = ((*next_long_name).buffer)
                        .as_mut_ptr()
                        .offset(512 as libc::c_int as isize);
                    recent_long_name = next_long_name;
                    recent_long_name_blocks = next_long_name_blocks;
                    next_long_name = 0 as *mut block;
                } else {
                    let mut np: *mut libc::c_char = namebuf.as_mut_ptr();
                    if (*h).prefix[0 as libc::c_int as usize] as libc::c_int != 0
                        && strcmp(
                            ((*h).magic).as_ptr(),
                            b"ustar\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                    {
                        memcpy(
                            np as *mut libc::c_void,
                            ((*h).prefix).as_ptr() as *const libc::c_void,
                            ::std::mem::size_of::<[libc::c_char; 155]>() as libc::c_ulong,
                        );
                        *np
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 155]>()
                                    as libc::c_ulong as isize,
                            ) = '\0' as i32 as libc::c_char;
                        np = np.offset(strlen(np) as isize);
                        let fresh5 = np;
                        np = np.offset(1);
                        *fresh5 = '/' as i32 as libc::c_char;
                    }
                    memcpy(
                        np as *mut libc::c_void,
                        ((*h).name).as_ptr() as *const libc::c_void,
                        ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
                    );
                    *np
                        .offset(
                            ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong
                                as isize,
                        ) = '\0' as i32 as libc::c_char;
                    name = namebuf.as_mut_ptr();
                    recent_long_name = 0 as *mut block;
                    recent_long_name_blocks = 0 as libc::c_int as size_t;
                }
                assign_string(&mut (*info).orig_file_name, name);
                assign_string(&mut (*info).file_name, name);
                (*info).had_trailing_slash = strip_trailing_slashes((*info).file_name);
                rpl_free(recent_long_link as *mut libc::c_void);
                if !next_long_link.is_null() {
                    name = ((*next_long_link).buffer)
                        .as_mut_ptr()
                        .offset(512 as libc::c_int as isize);
                    recent_long_link = next_long_link;
                    recent_long_link_blocks = next_long_link_blocks;
                    next_long_link = 0 as *mut block;
                } else {
                    memcpy(
                        namebuf.as_mut_ptr() as *mut libc::c_void,
                        ((*h).linkname).as_ptr() as *const libc::c_void,
                        ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
                    );
                    namebuf[::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong
                        as usize] = '\0' as i32 as libc::c_char;
                    name = namebuf.as_mut_ptr();
                    recent_long_link = 0 as *mut block;
                    recent_long_link_blocks = 0 as libc::c_int as size_t;
                }
                assign_string(&mut (*info).link_name, name);
                break;
            }
        }
    }
    rpl_free(next_long_name as *mut libc::c_void);
    rpl_free(next_long_link as *mut libc::c_void);
    return status;
}
pub unsafe extern "C" fn decode_header(
    mut header: *mut block,
    mut stat_info: *mut tar_stat_info,
    mut format_pointer: *mut archive_format,
    mut do_user_group: libc::c_int,
) {
    let mut format: archive_format = DEFAULT_FORMAT;
    let mut hbits: bool = false;
    let mut mode: mode_t = mode_from_header(
        ((*header).header.mode).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        &mut hbits,
    );
    if strcmp(
        ((*header).header.magic).as_mut_ptr(),
        b"ustar\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        if (*header).star_header.prefix[130 as libc::c_int as usize] as libc::c_int
            == 0 as libc::c_int
            && ((*header).star_header.atime[0 as libc::c_int as usize] as libc::c_int
                >= '0' as i32
                && (*header).star_header.atime[0 as libc::c_int as usize] as libc::c_int
                    <= '7' as i32)
            && (*header).star_header.atime[11 as libc::c_int as usize] as libc::c_int
                == ' ' as i32
            && ((*header).star_header.ctime[0 as libc::c_int as usize] as libc::c_int
                >= '0' as i32
                && (*header).star_header.ctime[0 as libc::c_int as usize] as libc::c_int
                    <= '7' as i32)
            && (*header).star_header.ctime[11 as libc::c_int as usize] as libc::c_int
                == ' ' as i32
        {
            format = STAR_FORMAT;
        } else if (*stat_info).xhdr.size != 0 {
            format = POSIX_FORMAT;
        } else {
            format = USTAR_FORMAT;
        }
    } else if strcmp(
        ((*header).buffer).as_mut_ptr().offset(257 as libc::c_ulong as isize),
        b"ustar  \0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        format = (if hbits as libc::c_int != 0 {
            OLDGNU_FORMAT as libc::c_int
        } else {
            GNU_FORMAT as libc::c_int
        }) as archive_format;
    } else {
        format = V7_FORMAT;
    }
    *format_pointer = format;
    (*stat_info).stat.st_mode = mode;
    (*stat_info)
        .mtime
        .tv_sec = time_from_header(
        ((*header).header.mtime).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    );
    (*stat_info).mtime.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
    assign_string_n(
        &mut (*stat_info).uname,
        if (*header).header.uname[0 as libc::c_int as usize] as libc::c_int != 0 {
            ((*header).header.uname).as_mut_ptr()
        } else {
            0 as *mut libc::c_char
        },
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
    assign_string_n(
        &mut (*stat_info).gname,
        if (*header).header.gname[0 as libc::c_int as usize] as libc::c_int != 0 {
            ((*header).header.gname).as_mut_ptr()
        } else {
            0 as *mut libc::c_char
        },
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
    xheader_xattr_init(stat_info);
    if format as libc::c_uint == OLDGNU_FORMAT as libc::c_int as libc::c_uint
        && incremental_option as libc::c_int != 0
    {
        (*stat_info)
            .atime
            .tv_sec = time_from_header(
            ((*header).oldgnu_header.atime).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
        );
        (*stat_info)
            .ctime
            .tv_sec = time_from_header(
            ((*header).oldgnu_header.ctime).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
        );
        (*stat_info).ctime.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
        (*stat_info).atime.tv_nsec = (*stat_info).ctime.tv_nsec;
    } else if format as libc::c_uint == STAR_FORMAT as libc::c_int as libc::c_uint {
        (*stat_info)
            .atime
            .tv_sec = time_from_header(
            ((*header).star_header.atime).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
        );
        (*stat_info)
            .ctime
            .tv_sec = time_from_header(
            ((*header).star_header.ctime).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
        );
        (*stat_info).ctime.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
        (*stat_info).atime.tv_nsec = (*stat_info).ctime.tv_nsec;
    } else {
        (*stat_info).ctime = start_time;
        (*stat_info).atime = (*stat_info).ctime;
    }
    if format as libc::c_uint == V7_FORMAT as libc::c_int as libc::c_uint {
        (*stat_info)
            .stat
            .st_uid = uid_from_header(
            ((*header).header.uid).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        );
        (*stat_info)
            .stat
            .st_gid = gid_from_header(
            ((*header).header.gid).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        );
        (*stat_info).stat.st_rdev = 0 as libc::c_int as __dev_t;
    } else {
        if do_user_group != 0 {
            if numeric_owner_option as libc::c_int != 0
                || *((*header).header.uname).as_mut_ptr() == 0
                || uname_to_uid(
                    ((*header).header.uname).as_mut_ptr(),
                    &mut (*stat_info).stat.st_uid,
                ) == 0
            {
                (*stat_info)
                    .stat
                    .st_uid = uid_from_header(
                    ((*header).header.uid).as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                );
            }
            if numeric_owner_option as libc::c_int != 0
                || *((*header).header.gname).as_mut_ptr() == 0
                || gname_to_gid(
                    ((*header).header.gname).as_mut_ptr(),
                    &mut (*stat_info).stat.st_gid,
                ) == 0
            {
                (*stat_info)
                    .stat
                    .st_gid = gid_from_header(
                    ((*header).header.gid).as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                );
            }
        }
        match (*header).header.typeflag as libc::c_int {
            52 | 51 => {
                (*stat_info)
                    .stat
                    .st_rdev = gnu_dev_makedev(
                    major_from_header(
                        ((*header).header.devmajor).as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                    ) as libc::c_uint,
                    minor_from_header(
                        ((*header).header.devminor).as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                    ) as libc::c_uint,
                );
            }
            _ => {
                (*stat_info).stat.st_rdev = 0 as libc::c_int as __dev_t;
            }
        }
    }
    xheader_decode(stat_info);
    if sparse_member_p(stat_info) {
        sparse_fixup_header(stat_info);
        (*stat_info).is_sparse = 1 as libc::c_int != 0;
    } else {
        (*stat_info).is_sparse = 0 as libc::c_int != 0;
        if (current_format as libc::c_uint == GNU_FORMAT as libc::c_int as libc::c_uint
            || current_format as libc::c_uint
                == OLDGNU_FORMAT as libc::c_int as libc::c_uint)
            && (*current_header).header.typeflag as libc::c_int == 'D' as i32
            || !((*stat_info).dumpdir).is_null()
        {
            (*stat_info).is_dumpdir = 1 as libc::c_int != 0;
        }
    };
}
unsafe extern "C" fn from_header(
    mut where0: *const libc::c_char,
    mut digs: size_t,
    mut type_0: *const libc::c_char,
    mut minval: intmax_t,
    mut maxval: uintmax_t,
    mut octal_only: bool,
    mut silent: bool,
) -> intmax_t {
    let mut value: uintmax_t = 0;
    let mut uminval: uintmax_t = minval as uintmax_t;
    let mut minus_minval: uintmax_t = uminval.wrapping_neg();
    let mut where_0: *const libc::c_char = where0;
    let mut lim: *const libc::c_char = where_0.offset(digs as isize);
    let mut negative: bool = 0 as libc::c_int != 0;
    where_0 = where_0.offset((*where_0 == 0) as libc::c_int as isize);
    loop {
        if where_0 == lim {
            if !type_0.is_null() && !silent {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Blanks in header where numeric %s value expected\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    type_0,
                );
                exit_status = 2 as libc::c_int;
            }
            return -(1 as libc::c_int) as intmax_t;
        }
        if *(*__ctype_b_loc()).offset(*where_0 as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            break;
        }
        where_0 = where_0.offset(1);
        where_0;
    }
    value = 0 as libc::c_int as uintmax_t;
    if (*where_0 as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
        <= 7 as libc::c_int as libc::c_uint
    {
        let mut where1: *const libc::c_char = where_0;
        let mut overflow: bool = 0 as libc::c_int != 0;
        loop {
            let fresh6 = where_0;
            where_0 = where_0.offset(1);
            value = (value as libc::c_ulong)
                .wrapping_add((*fresh6 as libc::c_int - '0' as i32) as libc::c_ulong)
                as uintmax_t as uintmax_t;
            if where_0 == lim
                || !((*where_0 as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                    <= 7 as libc::c_int as libc::c_uint)
            {
                break;
            }
            overflow = (overflow as libc::c_int
                | (value != value << 3 as libc::c_int >> 3 as libc::c_int)
                    as libc::c_int) != 0;
            value <<= 3 as libc::c_int;
        }
        if (overflow as libc::c_int != 0 || maxval < value)
            && '2' as i32 <= *where1 as libc::c_int && !type_0.is_null()
        {
            let mut digit: libc::c_int = *where1 as libc::c_int - '0' as i32
                | 4 as libc::c_int;
            overflow = 0 as libc::c_int != 0;
            value = 0 as libc::c_int as uintmax_t;
            where_0 = where1;
            loop {
                value = (value as libc::c_ulong)
                    .wrapping_add((7 as libc::c_int - digit) as libc::c_ulong)
                    as uintmax_t as uintmax_t;
                where_0 = where_0.offset(1);
                where_0;
                if where_0 == lim
                    || !((*where_0 as libc::c_uint)
                        .wrapping_sub('0' as i32 as libc::c_uint)
                        <= 7 as libc::c_int as libc::c_uint)
                {
                    break;
                }
                digit = *where_0 as libc::c_int - '0' as i32;
                overflow = (overflow as libc::c_int
                    | (value != value << 3 as libc::c_int >> 3 as libc::c_int)
                        as libc::c_int) != 0;
                value <<= 3 as libc::c_int;
            }
            value = value.wrapping_add(1);
            value;
            overflow = (overflow as libc::c_int | (value == 0) as libc::c_int) != 0;
            if !overflow && value <= minus_minval {
                if !silent {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Archive octal value %.*s is out of %s range; assuming two's complement\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        where_0.offset_from(where1) as libc::c_long as libc::c_int,
                        where1,
                        type_0,
                    );
                }
                negative = 1 as libc::c_int != 0;
            }
        }
        if overflow {
            if !type_0.is_null() && !silent {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Archive octal value %.*s is out of %s range\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    where_0.offset_from(where1) as libc::c_long as libc::c_int,
                    where1,
                    type_0,
                );
                exit_status = 2 as libc::c_int;
            }
            return -(1 as libc::c_int) as intmax_t;
        }
    } else if !octal_only {
        if *where_0 as libc::c_int == '-' as i32 || *where_0 as libc::c_int == '+' as i32
        {
            let mut dig: libc::c_int = 0;
            if !silent {
                static mut warned_once: bool = false;
                if !warned_once {
                    warned_once = 1 as libc::c_int != 0;
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Archive contains obsolescent base-64 headers\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            }
            let fresh8 = where_0;
            where_0 = where_0.offset(1);
            negative = *fresh8 as libc::c_int == '-' as i32;
            while where_0 != lim
                && {
                    dig = base64_map[*where_0 as libc::c_uchar as usize] as libc::c_int;
                    dig < 64 as libc::c_int
                }
            {
                if value << 6 as libc::c_int >> 6 as libc::c_int != value {
                    let mut fresh9 = ::std::vec::from_elem(
                        0,
                        digs.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
                    );
                    let mut string: *mut libc::c_char = fresh9.leak().as_mut_ptr()
                        as *mut libc::c_char;
                    memcpy(
                        string as *mut libc::c_void,
                        where0 as *const libc::c_void,
                        digs,
                    );
                    *string.offset(digs as isize) = '\0' as i32 as libc::c_char;
                    if !type_0.is_null() && !silent {
                        if error_hook.is_some() {
                            error_hook.unwrap()();
                        }
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Archive signed base-64 string %s is out of %s range\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quote(string),
                            type_0,
                        );
                        exit_status = 2 as libc::c_int;
                    }
                    return -(1 as libc::c_int) as intmax_t;
                }
                value = value << 6 as libc::c_int | dig as libc::c_ulong;
                where_0 = where_0.offset(1);
                where_0;
            }
        } else if *where_0 as libc::c_int == -128i32 || *where_0 as libc::c_int == -1i32
        {
            let mut signbit: libc::c_int = *where_0 as libc::c_int
                & (1 as libc::c_int) << 8 as libc::c_int - 2 as libc::c_int;
            let mut topbits: uintmax_t = (-signbit as uintmax_t)
                << (8 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                    .wrapping_sub(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(
                        (8 as libc::c_int - 2 as libc::c_int) as libc::c_ulong,
                    );
            let fresh10 = where_0;
            where_0 = where_0.offset(1);
            value = ((*fresh10 as libc::c_int
                & ((1 as libc::c_int) << 8 as libc::c_int - 2 as libc::c_int)
                    - 1 as libc::c_int) - signbit) as uintmax_t;
            loop {
                let fresh11 = where_0;
                where_0 = where_0.offset(1);
                value = (value << 8 as libc::c_int)
                    .wrapping_add(*fresh11 as libc::c_uchar as libc::c_ulong);
                if where_0 == lim {
                    break;
                }
                if value << 8 as libc::c_int >> 8 as libc::c_int | topbits != value {
                    if !type_0.is_null() && !silent {
                        if error_hook.is_some() {
                            error_hook.unwrap()();
                        }
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Archive base-256 value is out of %s range\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            type_0,
                        );
                        exit_status = 2 as libc::c_int;
                    }
                    return -(1 as libc::c_int) as intmax_t;
                }
            }
            negative = signbit != 0 as libc::c_int;
            if negative {
                value = value.wrapping_neg();
            }
        }
    }
    if where_0 != lim && *where_0 as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*where_0 as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        if !type_0.is_null() {
            let mut buf: [libc::c_char; 1000] = [0; 1000];
            static mut o: *mut quoting_options = 0 as *const quoting_options
                as *mut quoting_options;
            if o.is_null() {
                o = clone_quoting_options(0 as *mut quoting_options);
                set_quoting_style(o, locale_quoting_style);
            }
            while where0 != lim && *lim.offset(-(1 as libc::c_int) as isize) == 0 {
                lim = lim.offset(-1);
                lim;
            }
            quotearg_buffer(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1000]>() as libc::c_ulong,
                where0,
                lim.offset_from(where0) as libc::c_long as size_t,
                o,
            );
            if !silent {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Archive contains %.*s where numeric %s value expected\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    ::std::mem::size_of::<[libc::c_char; 1000]>() as libc::c_ulong
                        as libc::c_int,
                    buf.as_mut_ptr(),
                    type_0,
                );
                exit_status = 2 as libc::c_int;
            }
        }
        return -(1 as libc::c_int) as intmax_t;
    }
    if value <= (if negative as libc::c_int != 0 { minus_minval } else { maxval }) {
        return represent_uintmax(
            if negative as libc::c_int != 0 { value.wrapping_neg() } else { value },
        );
    }
    if !type_0.is_null() && !silent {
        let mut minval_buf: [libc::c_char; 22] = [0; 22];
        let mut maxval_buf: [libc::c_char; 21] = [0; 21];
        let mut value_buf: [libc::c_char; 22] = [0; 22];
        let mut minval_string: *mut libc::c_char = umaxtostr(
            minus_minval,
            minval_buf.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        let mut value_string: *mut libc::c_char = umaxtostr(
            value,
            value_buf.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        if negative {
            value_string = value_string.offset(-1);
            *value_string = '-' as i32 as libc::c_char;
        }
        if minus_minval != 0 {
            minval_string = minval_string.offset(-1);
            *minval_string = '-' as i32 as libc::c_char;
        }
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Archive value %s is out of %s range %s..%s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            value_string,
            type_0,
            minval_string,
            umaxtostr(maxval, maxval_buf.as_mut_ptr()),
        );
        exit_status = 2 as libc::c_int;
    }
    return -(1 as libc::c_int) as intmax_t;
}
unsafe extern "C" fn gid_from_header(
    mut p: *const libc::c_char,
    mut s: size_t,
) -> gid_t {
    return from_header(
        p,
        s,
        b"gid_t\0" as *const u8 as *const libc::c_char,
        !if (0 as libc::c_int as gid_t) < -(1 as libc::c_int) as gid_t {
            -(1 as libc::c_int) as gid_t
        } else {
            ((1 as libc::c_int as gid_t)
                << (::std::mem::size_of::<gid_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        } as intmax_t,
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
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
    ) as gid_t;
}
unsafe extern "C" fn major_from_header(
    mut p: *const libc::c_char,
    mut s: size_t,
) -> libc::c_int {
    return from_header(
        p,
        s,
        b"major_t\0" as *const u8 as *const libc::c_char,
        !if (0 as libc::c_int) < -(1 as libc::c_int) {
            -(1 as libc::c_int)
        } else {
            (((1 as libc::c_int)
                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong)) - 1 as libc::c_int)
                * 2 as libc::c_int + 1 as libc::c_int
        } as intmax_t,
        (if (0 as libc::c_int) < -(1 as libc::c_int) {
            -(1 as libc::c_int)
        } else {
            (((1 as libc::c_int)
                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong)) - 1 as libc::c_int)
                * 2 as libc::c_int + 1 as libc::c_int
        }) as uintmax_t,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
    ) as libc::c_int;
}
unsafe extern "C" fn minor_from_header(
    mut p: *const libc::c_char,
    mut s: size_t,
) -> libc::c_int {
    return from_header(
        p,
        s,
        b"minor_t\0" as *const u8 as *const libc::c_char,
        !if (0 as libc::c_int) < -(1 as libc::c_int) {
            -(1 as libc::c_int)
        } else {
            (((1 as libc::c_int)
                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong)) - 1 as libc::c_int)
                * 2 as libc::c_int + 1 as libc::c_int
        } as intmax_t,
        (if (0 as libc::c_int) < -(1 as libc::c_int) {
            -(1 as libc::c_int)
        } else {
            (((1 as libc::c_int)
                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong)) - 1 as libc::c_int)
                * 2 as libc::c_int + 1 as libc::c_int
        }) as uintmax_t,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
    ) as libc::c_int;
}
unsafe extern "C" fn mode_from_header(
    mut p: *const libc::c_char,
    mut s: size_t,
    mut hbits: *mut bool,
) -> mode_t {
    let mut u: intmax_t = from_header(
        p,
        s,
        b"mode_t\0" as *const u8 as *const libc::c_char,
        -(9223372036854775807 as libc::c_long) - 1 as libc::c_int as libc::c_long,
        18446744073709551615 as libc::c_ulong,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
    );
    let mut mode: mode_t = ((if u & 0o4000 as libc::c_int as libc::c_long != 0 {
        0o4000 as libc::c_int
    } else {
        0 as libc::c_int
    })
        | (if u & 0o2000 as libc::c_int as libc::c_long != 0 {
            0o2000 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if u & 0o1000 as libc::c_int as libc::c_long != 0 {
            0o1000 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if u & 0o400 as libc::c_int as libc::c_long != 0 {
            0o400 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if u & 0o200 as libc::c_int as libc::c_long != 0 {
            0o200 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if u & 0o100 as libc::c_int as libc::c_long != 0 {
            0o100 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if u & 0o40 as libc::c_int as libc::c_long != 0 {
            0o400 as libc::c_int >> 3 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if u & 0o20 as libc::c_int as libc::c_long != 0 {
            0o200 as libc::c_int >> 3 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if u & 0o10 as libc::c_int as libc::c_long != 0 {
            0o100 as libc::c_int >> 3 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if u & 0o4 as libc::c_int as libc::c_long != 0 {
            0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if u & 0o2 as libc::c_int as libc::c_long != 0 {
            0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if u & 0o1 as libc::c_int as libc::c_long != 0 {
            0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
        } else {
            0 as libc::c_int
        })) as mode_t;
    *hbits = u & !(0o7777 as libc::c_int) as libc::c_long
        != 0 as libc::c_int as libc::c_long;
    return mode;
}
pub unsafe extern "C" fn off_from_header(
    mut p: *const libc::c_char,
    mut s: size_t,
) -> off_t {
    return from_header(
        p,
        s,
        b"off_t\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as intmax_t,
        (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
            -(1 as libc::c_int) as off_t
        } else {
            (((1 as libc::c_int as off_t)
                << (::std::mem::size_of::<off_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        }) as uintmax_t,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
    );
}
unsafe extern "C" fn time_from_header(
    mut p: *const libc::c_char,
    mut s: size_t,
) -> time_t {
    return from_header(
        p,
        s,
        b"time_t\0" as *const u8 as *const libc::c_char,
        !if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
            -(1 as libc::c_int) as time_t
        } else {
            (((1 as libc::c_int as time_t)
                << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        },
        (if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
            -(1 as libc::c_int) as time_t
        } else {
            (((1 as libc::c_int as time_t)
                << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        }) as uintmax_t,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
    );
}
unsafe extern "C" fn uid_from_header(
    mut p: *const libc::c_char,
    mut s: size_t,
) -> uid_t {
    return from_header(
        p,
        s,
        b"uid_t\0" as *const u8 as *const libc::c_char,
        !if (0 as libc::c_int as uid_t) < -(1 as libc::c_int) as uid_t {
            -(1 as libc::c_int) as uid_t
        } else {
            ((1 as libc::c_int as uid_t)
                << (::std::mem::size_of::<uid_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        } as intmax_t,
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
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
    ) as uid_t;
}
pub unsafe extern "C" fn uintmax_from_header(
    mut p: *const libc::c_char,
    mut s: size_t,
) -> uintmax_t {
    return from_header(
        p,
        s,
        b"uintmax_t\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as intmax_t,
        18446744073709551615 as libc::c_ulong,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
    ) as uintmax_t;
}
pub unsafe extern "C" fn tartime(
    mut t: timespec,
    mut full_time: bool,
) -> *const libc::c_char {
    static mut buffer: [libc::c_char; 37] = [0; 37];
    let mut tm: *mut tm = 0 as *mut tm;
    let mut s: time_t = t.tv_sec;
    let mut ns: libc::c_int = t.tv_nsec as libc::c_int;
    let mut negative: bool = s < 0 as libc::c_int as libc::c_long;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if negative as libc::c_int != 0 && ns != 0 as libc::c_int {
        s += 1;
        s;
        ns = 1000000000 as libc::c_int - ns;
    }
    tm = if utc_option as libc::c_int != 0 { gmtime(&mut s) } else { localtime(&mut s) };
    if !tm.is_null() {
        if full_time {
            strftime(
                buffer.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong,
                b"%Y-%m-%d %H:%M:%S\0" as *const u8 as *const libc::c_char,
                tm,
            );
            code_ns_fraction(
                ns,
                buffer.as_mut_ptr().offset(strlen(buffer.as_mut_ptr()) as isize),
            );
        } else {
            strftime(
                buffer.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong,
                b"%Y-%m-%d %H:%M\0" as *const u8 as *const libc::c_char,
                tm,
            );
        }
        return buffer.as_mut_ptr();
    }
    p = umaxtostr(
        if negative as libc::c_int != 0 {
            (s as uintmax_t).wrapping_neg()
        } else {
            s as libc::c_ulong
        },
        buffer
            .as_mut_ptr()
            .offset(
                ::std::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong as isize,
            )
            .offset(
                -((::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(
                        !((0 as libc::c_int as uintmax_t)
                            < -(1 as libc::c_int) as uintmax_t) as libc::c_int
                            as libc::c_ulong,
                    )
                    .wrapping_mul(146 as libc::c_int as libc::c_ulong)
                    .wrapping_add(484 as libc::c_int as libc::c_ulong)
                    .wrapping_div(485 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        !((0 as libc::c_int as uintmax_t)
                            < -(1 as libc::c_int) as uintmax_t) as libc::c_int
                            as libc::c_ulong,
                    )
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize),
            )
            .offset(-(fraclen as libc::c_int as isize)),
    );
    if negative {
        p = p.offset(-1);
        *p = '-' as i32 as libc::c_char;
    }
    while buffer
        .as_mut_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong as isize)
        .offset(-(::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as isize))
        .offset(
            (if full_time as libc::c_int != 0 {
                (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            }) as isize,
        ) < p
    {
        p = p.offset(-1);
        *p = ' ' as i32 as libc::c_char;
    }
    if full_time {
        code_ns_fraction(
            ns,
            buffer
                .as_mut_ptr()
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong as isize,
                )
                .offset(-(1 as libc::c_int as isize))
                .offset(-(fraclen as libc::c_int as isize)),
        );
    }
    return p;
}
static mut ugswidth: libc::c_int = 19 as libc::c_int;
static mut datewidth: libc::c_int = 0;
static mut volume_label_printed: bool = 0 as libc::c_int != 0;
unsafe extern "C" fn simple_print_header(
    mut st: *mut tar_stat_info,
    mut blk: *mut block,
    mut block_ordinal: off_t,
) {
    let mut modes: [libc::c_char; 12] = [0; 12];
    let mut time_stamp: *const libc::c_char = 0 as *const libc::c_char;
    let mut time_stamp_len: libc::c_int = 0;
    let mut temp_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut uform: [libc::c_char; 21] = [0; 21];
    let mut gform: [libc::c_char; 21] = [0; 21];
    let mut user: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut group: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: [libc::c_char; 42] = [0; 42];
    let mut uintbuf: [libc::c_char; 21] = [0; 21];
    let mut pad: libc::c_int = 0;
    let mut sizelen: libc::c_int = 0;
    if show_transformed_names_option {
        temp_name = if !((*st).file_name).is_null() {
            (*st).file_name
        } else {
            (*st).orig_file_name
        };
    } else {
        temp_name = if !((*st).orig_file_name).is_null() {
            (*st).orig_file_name
        } else {
            (*st).file_name
        };
    }
    if block_number_option {
        let mut buf: [libc::c_char; 21] = [0; 21];
        if block_ordinal < 0 as libc::c_int as libc::c_long {
            block_ordinal = current_block_ordinal();
        }
        block_ordinal = (block_ordinal as libc::c_ulong)
            .wrapping_sub(recent_long_name_blocks) as off_t as off_t;
        block_ordinal = (block_ordinal as libc::c_ulong)
            .wrapping_sub(recent_long_link_blocks) as off_t as off_t;
        fprintf(
            stdlis,
            dcgettext(
                0 as *const libc::c_char,
                b"block %s: \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            umaxtostr(block_ordinal as uintmax_t, buf.as_mut_ptr()),
        );
    }
    if verbose_option <= 1 as libc::c_int {
        fputs_unlocked(quotearg(temp_name), stdlis);
        if show_transformed_names_option as libc::c_int != 0
            && (*st).had_trailing_slash as libc::c_int != 0
        {
            fputc_unlocked('/' as i32, stdlis);
        }
        fputc_unlocked('\n' as i32, stdlis);
    } else {
        modes[0 as libc::c_int as usize] = '?' as i32 as libc::c_char;
        match (*blk).header.typeflag as libc::c_int {
            86 => {
                volume_label_printed = 1 as libc::c_int != 0;
                modes[0 as libc::c_int as usize] = 'V' as i32 as libc::c_char;
            }
            77 => {
                modes[0 as libc::c_int as usize] = 'M' as i32 as libc::c_char;
            }
            76 | 75 => {
                modes[0 as libc::c_int as usize] = 'L' as i32 as libc::c_char;
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Unexpected long name header\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                exit_status = 2 as libc::c_int;
            }
            83 | 48 | 0 => {
                modes[0 as libc::c_int
                    as usize] = (if (*st).had_trailing_slash as libc::c_int != 0 {
                    'd' as i32
                } else {
                    '-' as i32
                }) as libc::c_char;
            }
            49 => {
                modes[0 as libc::c_int as usize] = 'h' as i32 as libc::c_char;
            }
            68 => {
                modes[0 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
            }
            53 => {
                modes[0 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
            }
            50 => {
                modes[0 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
            }
            52 => {
                modes[0 as libc::c_int as usize] = 'b' as i32 as libc::c_char;
            }
            51 => {
                modes[0 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
            }
            54 => {
                modes[0 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
            }
            55 => {
                modes[0 as libc::c_int as usize] = 'C' as i32 as libc::c_char;
            }
            _ => {}
        }
        pax_decode_mode(
            (*st).stat.st_mode,
            modes.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        xattrs_print_char(st, modes.as_mut_ptr().offset(10 as libc::c_int as isize));
        time_stamp = tartime((*st).mtime, full_time_option);
        time_stamp_len = strlen(time_stamp) as libc::c_int;
        if datewidth < time_stamp_len {
            datewidth = time_stamp_len;
        }
        if !((*st).uname).is_null()
            && *((*st).uname).offset(0 as libc::c_int as isize) as libc::c_int != 0
            && current_format as libc::c_uint != V7_FORMAT as libc::c_int as libc::c_uint
            && !numeric_owner_option
        {
            user = (*st).uname;
        } else {
            user = umaxtostr((*st).stat.st_uid as uintmax_t, uform.as_mut_ptr());
        }
        if !((*st).gname).is_null()
            && *((*st).gname).offset(0 as libc::c_int as isize) as libc::c_int != 0
            && current_format as libc::c_uint != V7_FORMAT as libc::c_int as libc::c_uint
            && !numeric_owner_option
        {
            group = (*st).gname;
        } else {
            group = umaxtostr((*st).stat.st_gid as uintmax_t, gform.as_mut_ptr());
        }
        match (*blk).header.typeflag as libc::c_int {
            51 | 52 => {
                strcpy(
                    size.as_mut_ptr(),
                    umaxtostr(
                        gnu_dev_major((*st).stat.st_rdev) as uintmax_t,
                        uintbuf.as_mut_ptr(),
                    ),
                );
                strcat(size.as_mut_ptr(), b",\0" as *const u8 as *const libc::c_char);
                strcat(
                    size.as_mut_ptr(),
                    umaxtostr(
                        gnu_dev_minor((*st).stat.st_rdev) as uintmax_t,
                        uintbuf.as_mut_ptr(),
                    ),
                );
            }
            _ => {
                strcpy(
                    size.as_mut_ptr(),
                    umaxtostr((*st).stat.st_size as uintmax_t, uintbuf.as_mut_ptr()),
                );
            }
        }
        sizelen = strlen(size.as_mut_ptr()) as libc::c_int;
        pad = (strlen(user))
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen(group))
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(sizelen as libc::c_ulong) as libc::c_int;
        if pad > ugswidth {
            ugswidth = pad;
        }
        fprintf(
            stdlis,
            b"%s %s/%s %*s %-*s\0" as *const u8 as *const libc::c_char,
            modes.as_mut_ptr(),
            user,
            group,
            ugswidth - pad + sizelen,
            size.as_mut_ptr(),
            datewidth,
            time_stamp,
        );
        fprintf(
            stdlis,
            b" %s\0" as *const u8 as *const libc::c_char,
            quotearg(temp_name),
        );
        if show_transformed_names_option as libc::c_int != 0
            && (*st).had_trailing_slash as libc::c_int != 0
        {
            fputc_unlocked('/' as i32, stdlis);
        }
        match (*blk).header.typeflag as libc::c_int {
            50 => {
                fprintf(
                    stdlis,
                    b" -> %s\n\0" as *const u8 as *const libc::c_char,
                    quotearg((*st).link_name),
                );
            }
            49 => {
                fprintf(
                    stdlis,
                    dcgettext(
                        0 as *const libc::c_char,
                        b" link to %s\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg((*st).link_name),
                );
            }
            0 | 48 | 83 | 51 | 52 | 53 | 54 | 55 | 68 => {
                putc_unlocked('\n' as i32, stdlis);
            }
            75 => {
                fprintf(
                    stdlis,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"--Long Link--\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            76 => {
                fprintf(
                    stdlis,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"--Long Name--\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            86 => {
                fprintf(
                    stdlis,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"--Volume Header--\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            77 => {
                strcpy(
                    size.as_mut_ptr(),
                    umaxtostr(
                        uintmax_from_header(
                            ((*blk).oldgnu_header.offset).as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
                        ),
                        uintbuf.as_mut_ptr(),
                    ),
                );
                fprintf(
                    stdlis,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"--Continued at byte %s--\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    size.as_mut_ptr(),
                );
            }
            _ => {
                let mut type_string: [libc::c_char; 2] = [0; 2];
                type_string[0 as libc::c_int as usize] = (*blk).header.typeflag;
                type_string[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                fprintf(
                    stdlis,
                    dcgettext(
                        0 as *const libc::c_char,
                        b" unknown file type %s\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(type_string.as_mut_ptr()),
                );
            }
        }
    }
    fflush_unlocked(stdlis);
    xattrs_print(st);
}
unsafe extern "C" fn print_volume_label() {
    let mut vstat: tar_stat_info = tar_stat_info {
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
    let mut vblk: block = block { buffer: [0; 512] };
    let mut dummy: archive_format = DEFAULT_FORMAT;
    memset(
        &mut vblk as *mut block as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<block>() as libc::c_ulong,
    );
    vblk.header.typeflag = 'V' as i32 as libc::c_char;
    if !recent_global_header.is_null() {
        memcpy(
            (vblk.header.mtime).as_mut_ptr() as *mut libc::c_void,
            ((*recent_global_header).header.mtime).as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
        );
    }
    tar_stat_init(&mut vstat);
    assign_string(&mut vstat.file_name, b".\0" as *const u8 as *const libc::c_char);
    decode_header(&mut vblk, &mut vstat, &mut dummy, 0 as libc::c_int);
    assign_string(&mut vstat.file_name, volume_label);
    simple_print_header(&mut vstat, &mut vblk, 0 as libc::c_int as off_t);
    tar_stat_destroy(&mut vstat);
}
pub unsafe extern "C" fn print_header(
    mut st: *mut tar_stat_info,
    mut blk: *mut block,
    mut block_ordinal: off_t,
) {
    if current_format as libc::c_uint == POSIX_FORMAT as libc::c_int as libc::c_uint
        && !volume_label_printed && !volume_label.is_null()
    {
        print_volume_label();
        volume_label_printed = 1 as libc::c_int != 0;
    }
    simple_print_header(st, blk, block_ordinal);
}
pub unsafe extern "C" fn print_for_mkdir(
    mut dirname: *mut libc::c_char,
    mut length: libc::c_int,
    mut mode: mode_t,
) {
    let mut modes: [libc::c_char; 11] = [0; 11];
    if verbose_option > 1 as libc::c_int {
        modes[0 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
        pax_decode_mode(mode, modes.as_mut_ptr().offset(1 as libc::c_int as isize));
        if block_number_option {
            let mut buf: [libc::c_char; 21] = [0; 21];
            fprintf(
                stdlis,
                dcgettext(
                    0 as *const libc::c_char,
                    b"block %s: \0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                umaxtostr(current_block_ordinal() as uintmax_t, buf.as_mut_ptr()),
            );
        }
        fprintf(
            stdlis,
            b"%s %*s %s\n\0" as *const u8 as *const libc::c_char,
            modes.as_mut_ptr(),
            ugswidth + 1 as libc::c_int + datewidth,
            dcgettext(
                0 as *const libc::c_char,
                b"Creating directory:\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg(dirname),
        );
    }
}
pub unsafe extern "C" fn skip_file(mut size: off_t) {
    let mut x: *mut block = 0 as *mut block;
    if seekable_archive {
        let mut nblk: off_t = seek_archive(size);
        if nblk >= 0 as libc::c_int as libc::c_long {
            size -= nblk * 512 as libc::c_int as libc::c_long;
        } else {
            seekable_archive = 0 as libc::c_int != 0;
        }
    }
    mv_size_left(size);
    while size > 0 as libc::c_int as libc::c_long {
        x = find_next_block();
        if x.is_null() {
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
            fatal_exit();
        }
        set_next_block_after(x);
        size -= 512 as libc::c_int as libc::c_long;
        mv_size_left(size);
    }
}
pub unsafe extern "C" fn skip_member() {
    if !current_stat_info.skipped {
        let mut save_typeflag: libc::c_char = (*current_header).header.typeflag;
        set_next_block_after(current_header);
        mv_begin_read(&mut current_stat_info);
        if current_stat_info.is_sparse {
            sparse_skip_file(&mut current_stat_info);
        } else if save_typeflag as libc::c_int != '5' as i32 {
            skip_file(current_stat_info.stat.st_size);
        }
        mv_end();
    }
}
pub unsafe extern "C" fn test_archive_label() {
    base64_init();
    name_gather();
    open_archive(ACCESS_READ);
    if read_header(&mut current_header, &mut current_stat_info, read_header_auto)
        as libc::c_uint == HEADER_SUCCESS as libc::c_int as libc::c_uint
    {
        decode_header(
            current_header,
            &mut current_stat_info,
            &mut current_format,
            0 as libc::c_int,
        );
        if (*current_header).header.typeflag as libc::c_int == 'V' as i32 {
            assign_string_n(
                &mut volume_label,
                ((*current_header).header.name).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
            );
        }
        if !volume_label.is_null() {
            if verbose_option != 0 {
                print_volume_label();
            }
            if !name_match(volume_label) && multi_volume_option as libc::c_int != 0 {
                let mut s: *mut libc::c_char = drop_volume_label_suffix(volume_label);
                name_match(s);
                rpl_free(s as *mut libc::c_void);
            }
        }
    }
    close_archive();
    label_notfound();
}
unsafe extern "C" fn run_static_initializers() {
    datewidth = (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
