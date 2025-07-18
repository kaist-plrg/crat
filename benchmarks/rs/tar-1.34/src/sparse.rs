use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __dirstream;
    pub type exclist;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn rpl_free(ptr: *mut libc::c_void);
    fn __strtoul_internal(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
        __group: libc::c_int,
    ) -> libc::c_ulong;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xalloc_die();
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
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
    fn offtostr(_: off_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    static mut error_hook: Option::<unsafe extern "C" fn() -> ()>;
    static mut exit_status: libc::c_int;
    fn truncate_warn(_: *const libc::c_char);
    fn write_error_details(_: *const libc::c_char, _: size_t, _: size_t);
    fn fatal_exit() -> !;
    fn quotearg_colon(arg: *const libc::c_char) -> *mut libc::c_char;
    static mut archive_format: archive_format;
    static mut ignore_failed_read_option: bool;
    static mut tar_sparse_major: libc::c_uint;
    static mut tar_sparse_minor: libc::c_uint;
    static mut hole_detection: hole_detection_method;
    static mut current_stat_info: tar_stat_info;
    fn current_block_ordinal() -> off_t;
    fn find_next_block() -> *mut block;
    fn set_next_block_after(block: *mut block);
    fn mv_begin_write(file_name: *const libc::c_char, totsize: off_t, sizeleft: off_t);
    fn mv_begin_read(st: *mut tar_stat_info);
    fn mv_end();
    fn mv_size_left(size: off_t);
    fn pad_archive(size_left: off_t);
    fn start_header(st: *mut tar_stat_info) -> *mut block;
    fn finish_header(st: *mut tar_stat_info, header: *mut block, block_ordinal: off_t);
    fn off_to_chars(off: off_t, buf: *mut libc::c_char, size: size_t) -> bool;
    static mut current_header: *mut block;
    static mut current_format: archive_format;
    fn off_from_header(buf: *const libc::c_char, size: size_t) -> off_t;
    fn skip_file(size: off_t);
    fn blocking_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn blocking_write(
        fd: libc::c_int,
        buf: *const libc::c_void,
        count: size_t,
    ) -> size_t;
    fn read_diag_details(name: *const libc::c_char, offset: off_t, size: size_t);
    fn seek_diag_details(name: *const libc::c_char, offset: off_t);
    fn set_exit_status(val: libc::c_int);
    fn xheader_store(
        keyword: *const libc::c_char,
        st: *mut tar_stat_info,
        data: *const libc::c_void,
    );
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn xheader_string_begin(xhdr: *mut xheader);
    fn xheader_string_add(xhdr: *mut xheader, s: *const libc::c_char);
    fn xheader_string_end(xhdr: *mut xheader, keyword: *const libc::c_char) -> bool;
    fn xheader_keyword_deleted_p(kw: *const libc::c_char) -> bool;
    fn xheader_format_name(
        st: *mut tar_stat_info,
        fmt: *const libc::c_char,
        n: size_t,
    ) -> *mut libc::c_char;
    fn sys_truncate(fd: libc::c_int) -> libc::c_int;
    fn report_difference(st: *mut tar_stat_info, message: *const libc::c_char, _: ...);
    static mut warning_option: libc::c_int;
}
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
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
pub const DEFAULT_MXFAST: C2RustUnnamed_2 = 128;
pub type C2RustUnnamed_2 = libc::c_uint;
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
pub type hole_detection_method = libc::c_uint;
pub const HOLE_DETECTION_SEEK: hole_detection_method = 2;
pub const HOLE_DETECTION_RAW: hole_detection_method = 1;
pub const HOLE_DETECTION_DEFAULT: hole_detection_method = 0;
pub type dump_status = libc::c_uint;
pub const dump_status_not_implemented: dump_status = 3;
pub const dump_status_fail: dump_status = 2;
pub const dump_status_short: dump_status = 1;
pub const dump_status_ok: dump_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tar_sparse_file {
    pub fd: libc::c_int,
    pub seekable: bool,
    pub offset: off_t,
    pub dumped_size: off_t,
    pub stat_info: *mut tar_stat_info,
    pub optab: *const tar_sparse_optab,
    pub closure: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tar_sparse_optab {
    pub init: Option::<unsafe extern "C" fn(*mut tar_sparse_file) -> bool>,
    pub done: Option::<unsafe extern "C" fn(*mut tar_sparse_file) -> bool>,
    pub sparse_member_p: Option::<unsafe extern "C" fn(*mut tar_sparse_file) -> bool>,
    pub dump_header: Option::<unsafe extern "C" fn(*mut tar_sparse_file) -> bool>,
    pub fixup_header: Option::<unsafe extern "C" fn(*mut tar_sparse_file) -> bool>,
    pub decode_header: Option::<unsafe extern "C" fn(*mut tar_sparse_file) -> bool>,
    pub scan_block: Option::<
        unsafe extern "C" fn(
            *mut tar_sparse_file,
            sparse_scan_state,
            *mut libc::c_void,
        ) -> bool,
    >,
    pub dump_region: Option::<
        unsafe extern "C" fn(*mut tar_sparse_file, size_t) -> bool,
    >,
    pub extract_region: Option::<
        unsafe extern "C" fn(*mut tar_sparse_file, size_t) -> bool,
    >,
}
pub type sparse_scan_state = libc::c_uint;
pub const scan_end: sparse_scan_state = 2;
pub const scan_block: sparse_scan_state = 1;
pub const scan_begin: sparse_scan_state = 0;
pub const add_fail: oldgnu_add_status = 2;
pub type oldgnu_add_status = libc::c_uint;
pub const add_finish: oldgnu_add_status = 1;
pub const add_ok: oldgnu_add_status = 0;
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn strtoumax(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut base: libc::c_int,
) -> uintmax_t {
    return __strtoul_internal(nptr, endptr, base, 0 as libc::c_int);
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
unsafe extern "C" fn dump_zeros(
    mut file: *mut tar_sparse_file,
    mut offset: off_t,
) -> bool {
    static mut zero_buf: [libc::c_char; 512] = [0; 512];
    if offset < (*file).offset {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int != 0;
    }
    while (*file).offset < offset {
        let mut size: size_t = (if (512 as libc::c_int as libc::c_long)
            < offset - (*file).offset
        {
            512 as libc::c_int as libc::c_long
        } else {
            offset - (*file).offset
        }) as size_t;
        let mut wrbytes: ssize_t = 0;
        wrbytes = write((*file).fd, zero_buf.as_ptr() as *const libc::c_void, size);
        if wrbytes <= 0 as libc::c_int as libc::c_long {
            if wrbytes == 0 as libc::c_int as libc::c_long {
                *__errno_location() = 22 as libc::c_int;
            }
            return 0 as libc::c_int != 0;
        }
        (*file).offset += wrbytes;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tar_sparse_member_p(mut file: *mut tar_sparse_file) -> bool {
    if ((*(*file).optab).sparse_member_p).is_some() {
        return ((*(*file).optab).sparse_member_p).unwrap()(file);
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn tar_sparse_init(mut file: *mut tar_sparse_file) -> bool {
    memset(
        file as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<tar_sparse_file>() as libc::c_ulong,
    );
    if !sparse_select_optab(file) {
        return 0 as libc::c_int != 0;
    }
    if ((*(*file).optab).init).is_some() {
        return ((*(*file).optab).init).unwrap()(file);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tar_sparse_done(mut file: *mut tar_sparse_file) -> bool {
    if ((*(*file).optab).done).is_some() {
        return ((*(*file).optab).done).unwrap()(file);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tar_sparse_scan(
    mut file: *mut tar_sparse_file,
    mut state: sparse_scan_state,
    mut block: *mut libc::c_void,
) -> bool {
    if ((*(*file).optab).scan_block).is_some() {
        return ((*(*file).optab).scan_block).unwrap()(file, state, block);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tar_sparse_dump_region(
    mut file: *mut tar_sparse_file,
    mut i: size_t,
) -> bool {
    if ((*(*file).optab).dump_region).is_some() {
        return ((*(*file).optab).dump_region).unwrap()(file, i);
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn tar_sparse_extract_region(
    mut file: *mut tar_sparse_file,
    mut i: size_t,
) -> bool {
    if ((*(*file).optab).extract_region).is_some() {
        return ((*(*file).optab).extract_region).unwrap()(file, i);
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn tar_sparse_dump_header(mut file: *mut tar_sparse_file) -> bool {
    if ((*(*file).optab).dump_header).is_some() {
        return ((*(*file).optab).dump_header).unwrap()(file);
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn tar_sparse_decode_header(mut file: *mut tar_sparse_file) -> bool {
    if ((*(*file).optab).decode_header).is_some() {
        return ((*(*file).optab).decode_header).unwrap()(file);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tar_sparse_fixup_header(mut file: *mut tar_sparse_file) -> bool {
    if ((*(*file).optab).fixup_header).is_some() {
        return ((*(*file).optab).fixup_header).unwrap()(file);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn lseek_or_error(
    mut file: *mut tar_sparse_file,
    mut offset: off_t,
) -> bool {
    if if (*file).seekable as libc::c_int != 0 {
        (lseek((*file).fd, offset, 0 as libc::c_int) < 0 as libc::c_int as libc::c_long)
            as libc::c_int
    } else {
        !dump_zeros(file, offset) as libc::c_int
    } != 0
    {
        seek_diag_details((*(*file).stat_info).orig_file_name, offset);
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn zero_block_p(
    mut buffer: *const libc::c_char,
    mut size: size_t,
) -> bool {
    loop {
        let fresh0 = size;
        size = size.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        let fresh1 = buffer;
        buffer = buffer.offset(1);
        if *fresh1 != 0 {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn sparse_add_map(
    mut st: *mut tar_stat_info,
    mut sp: *const sp_array,
) {
    let mut sparse_map: *mut sp_array = (*st).sparse_map;
    let mut avail: size_t = (*st).sparse_map_avail;
    if avail == (*st).sparse_map_size {
        sparse_map = x2nrealloc(
            sparse_map as *mut libc::c_void,
            &mut (*st).sparse_map_size,
            ::std::mem::size_of::<sp_array>() as libc::c_ulong,
        ) as *mut sp_array;
        (*st).sparse_map = sparse_map;
    }
    *sparse_map.offset(avail as isize) = *sp;
    (*st).sparse_map_avail = avail.wrapping_add(1 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn sparse_scan_file_raw(mut file: *mut tar_sparse_file) -> bool {
    let mut st: *mut tar_stat_info = (*file).stat_info;
    let mut fd: libc::c_int = (*file).fd;
    let mut buffer: [libc::c_char; 512] = [0; 512];
    let mut count: size_t = 0 as libc::c_int as size_t;
    let mut offset: off_t = 0 as libc::c_int as off_t;
    let mut sp: sp_array = {
        let mut init = sp_array {
            offset: 0 as libc::c_int as off_t,
            numbytes: 0 as libc::c_int as off_t,
        };
        init
    };
    (*st).archive_file_size = 0 as libc::c_int as off_t;
    if !tar_sparse_scan(file, scan_begin, 0 as *mut libc::c_void) {
        return 0 as libc::c_int != 0;
    }
    loop {
        count = blocking_read(
            fd,
            buffer.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
        );
        if !(count != 0 as libc::c_int as libc::c_ulong
            && count != -(1 as libc::c_int) as size_t)
        {
            break;
        }
        if zero_block_p(buffer.as_mut_ptr(), count) {
            if sp.numbytes != 0 {
                sparse_add_map(st, &mut sp);
                sp.numbytes = 0 as libc::c_int as off_t;
                if !tar_sparse_scan(file, scan_block, 0 as *mut libc::c_void) {
                    return 0 as libc::c_int != 0;
                }
            }
        } else {
            if sp.numbytes == 0 as libc::c_int as libc::c_long {
                sp.offset = offset;
            }
            sp
                .numbytes = (sp.numbytes as libc::c_ulong).wrapping_add(count) as off_t
                as off_t;
            (*st)
                .archive_file_size = ((*st).archive_file_size as libc::c_ulong)
                .wrapping_add(count) as off_t as off_t;
            if !tar_sparse_scan(
                file,
                scan_block,
                buffer.as_mut_ptr() as *mut libc::c_void,
            ) {
                return 0 as libc::c_int != 0;
            }
        }
        offset = (offset as libc::c_ulong).wrapping_add(count) as off_t as off_t;
    }
    if sp.numbytes == 0 as libc::c_int as libc::c_long {
        sp.offset = offset;
    }
    sparse_add_map(st, &mut sp);
    (*st)
        .archive_file_size = ((*st).archive_file_size as libc::c_ulong)
        .wrapping_add(count) as off_t as off_t;
    return tar_sparse_scan(file, scan_end, 0 as *mut libc::c_void);
}
unsafe extern "C" fn sparse_scan_file_wholesparse(
    mut file: *mut tar_sparse_file,
) -> bool {
    let mut st: *mut tar_stat_info = (*file).stat_info;
    let mut sp: sp_array = {
        let mut init = sp_array {
            offset: 0 as libc::c_int as off_t,
            numbytes: 0 as libc::c_int as off_t,
        };
        init
    };
    if (*st).stat.st_blocks == 0 as libc::c_int as libc::c_long {
        (*st).archive_file_size = 0 as libc::c_int as off_t;
        sp.offset = (*st).stat.st_size;
        sparse_add_map(st, &mut sp);
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn sparse_scan_file_seek(mut file: *mut tar_sparse_file) -> bool {
    let mut st: *mut tar_stat_info = (*file).stat_info;
    let mut fd: libc::c_int = (*file).fd;
    let mut sp: sp_array = {
        let mut init = sp_array {
            offset: 0 as libc::c_int as off_t,
            numbytes: 0 as libc::c_int as off_t,
        };
        init
    };
    let mut offset: off_t = 0 as libc::c_int as off_t;
    let mut data_offset: off_t = 0;
    let mut hole_offset: off_t = 0;
    (*st).archive_file_size = 0 as libc::c_int as off_t;
    loop {
        data_offset = lseek(fd, offset, 3 as libc::c_int);
        if data_offset == -(1 as libc::c_int) as off_t {
            if *__errno_location() == 6 as libc::c_int {
                sp.numbytes = 0 as libc::c_int as off_t;
                sp.offset = (*st).stat.st_size;
                sparse_add_map(st, &mut sp);
                return 1 as libc::c_int != 0;
            }
            return 0 as libc::c_int != 0;
        }
        hole_offset = lseek(fd, data_offset, 4 as libc::c_int);
        if offset == 0 as libc::c_int as libc::c_long
            && data_offset == 0 as libc::c_int as libc::c_long
            && hole_offset == (*st).stat.st_size
        {
            lseek(fd, 0 as libc::c_int as __off_t, 0 as libc::c_int);
            return 0 as libc::c_int != 0;
        }
        sp.offset = data_offset;
        sp.numbytes = hole_offset - data_offset;
        sparse_add_map(st, &mut sp);
        (*st).archive_file_size += sp.numbytes;
        offset = hole_offset;
    };
}
unsafe extern "C" fn sparse_scan_file(mut file: *mut tar_sparse_file) -> bool {
    if sparse_scan_file_wholesparse(file) {
        return 1 as libc::c_int != 0;
    }
    let mut current_block_6: u64;
    match hole_detection as libc::c_uint {
        0 | 2 => {
            if sparse_scan_file_seek(file) {
                return 1 as libc::c_int != 0;
            }
            current_block_6 = 4589443810046320981;
        }
        1 => {
            current_block_6 = 4589443810046320981;
        }
        _ => {
            current_block_6 = 10879442775620481940;
        }
    }
    match current_block_6 {
        4589443810046320981 => {
            if sparse_scan_file_raw(file) {
                return 1 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn sparse_select_optab(mut file: *mut tar_sparse_file) -> bool {
    match if current_format as libc::c_uint
        == DEFAULT_FORMAT as libc::c_int as libc::c_uint
    {
        archive_format as libc::c_uint
    } else {
        current_format as libc::c_uint
    } {
        1 | 3 => return 0 as libc::c_int != 0,
        2 | 6 => {
            (*file).optab = &oldgnu_optab;
        }
        4 => {
            (*file).optab = &pax_optab;
        }
        5 => {
            (*file).optab = &star_optab;
        }
        _ => return 0 as libc::c_int != 0,
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn sparse_dump_region(
    mut file: *mut tar_sparse_file,
    mut i: size_t,
) -> bool {
    let mut blk: *mut block = 0 as *mut block;
    let mut bytes_left: off_t = (*((*(*file).stat_info).sparse_map).offset(i as isize))
        .numbytes;
    if !lseek_or_error(
        file,
        (*((*(*file).stat_info).sparse_map).offset(i as isize)).offset,
    ) {
        return 0 as libc::c_int != 0;
    }
    while bytes_left > 0 as libc::c_int as libc::c_long {
        let mut bufsize: size_t = (if bytes_left > 512 as libc::c_int as libc::c_long {
            512 as libc::c_int as libc::c_long
        } else {
            bytes_left
        }) as size_t;
        let mut bytes_read: size_t = 0;
        blk = find_next_block();
        bytes_read = safe_read(
            (*file).fd,
            ((*blk).buffer).as_mut_ptr() as *mut libc::c_void,
            bufsize,
        );
        if bytes_read == -(1 as libc::c_int) as size_t {
            read_diag_details(
                (*(*file).stat_info).orig_file_name,
                (*((*(*file).stat_info).sparse_map).offset(i as isize)).offset
                    + (*((*(*file).stat_info).sparse_map).offset(i as isize)).numbytes
                    - bytes_left,
                bufsize,
            );
            return 0 as libc::c_int != 0;
        } else if bytes_read == 0 as libc::c_int as libc::c_ulong {
            let mut buf: [libc::c_char; 21] = [0; 21];
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
            let mut n: size_t = 0;
            if fstat((*file).fd, &mut st) == 0 as libc::c_int {
                n = ((*(*file).stat_info).stat.st_size - st.st_size) as size_t;
            } else {
                n = ((*(*file).stat_info).stat.st_size
                    - ((*((*(*file).stat_info).sparse_map).offset(i as isize)).offset
                        + (*((*(*file).stat_info).sparse_map).offset(i as isize))
                            .numbytes - bytes_left)) as size_t;
            }
            if warning_option & 0x80 as libc::c_int != 0 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcngettext(
                        0 as *const libc::c_char,
                        b"%s: File shrank by %s byte; padding with zeros\0" as *const u8
                            as *const libc::c_char,
                        b"%s: File shrank by %s bytes; padding with zeros\0" as *const u8
                            as *const libc::c_char,
                        n,
                        5 as libc::c_int,
                    ),
                    quotearg_colon((*(*file).stat_info).orig_file_name),
                    umaxtostr(n, buf.as_mut_ptr()),
                );
            }
            if !ignore_failed_read_option {
                set_exit_status(1 as libc::c_int);
            }
            return 0 as libc::c_int != 0;
        }
        memset(
            ((*blk).buffer).as_mut_ptr().offset(bytes_read as isize)
                as *mut libc::c_void,
            0 as libc::c_int,
            (512 as libc::c_int as libc::c_ulong).wrapping_sub(bytes_read),
        );
        bytes_left = (bytes_left as libc::c_ulong).wrapping_sub(bytes_read) as off_t
            as off_t;
        (*file)
            .dumped_size = ((*file).dumped_size as libc::c_ulong)
            .wrapping_add(bytes_read) as off_t as off_t;
        set_next_block_after(blk);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn sparse_extract_region(
    mut file: *mut tar_sparse_file,
    mut i: size_t,
) -> bool {
    let mut write_size: off_t = 0;
    if !lseek_or_error(
        file,
        (*((*(*file).stat_info).sparse_map).offset(i as isize)).offset,
    ) {
        return 0 as libc::c_int != 0;
    }
    write_size = (*((*(*file).stat_info).sparse_map).offset(i as isize)).numbytes;
    if write_size == 0 as libc::c_int as libc::c_long {
        if (*file).seekable as libc::c_int != 0 && sys_truncate((*file).fd) != 0 {
            truncate_warn((*(*file).stat_info).orig_file_name);
        }
    } else {
        while write_size > 0 as libc::c_int as libc::c_long {
            let mut count: size_t = 0;
            let mut wrbytes: size_t = (if write_size > 512 as libc::c_int as libc::c_long
            {
                512 as libc::c_int as libc::c_long
            } else {
                write_size
            }) as size_t;
            let mut blk: *mut block = find_next_block();
            if blk.is_null() {
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
                return 0 as libc::c_int != 0;
            }
            set_next_block_after(blk);
            (*file).dumped_size += 512 as libc::c_int as libc::c_long;
            count = blocking_write(
                (*file).fd,
                ((*blk).buffer).as_mut_ptr() as *const libc::c_void,
                wrbytes,
            );
            write_size = (write_size as libc::c_ulong).wrapping_sub(count) as off_t
                as off_t;
            mv_size_left((*(*file).stat_info).archive_file_size - (*file).dumped_size);
            (*file)
                .offset = ((*file).offset as libc::c_ulong).wrapping_add(count) as off_t
                as off_t;
            if count != wrbytes {
                write_error_details((*(*file).stat_info).orig_file_name, count, wrbytes);
                return 0 as libc::c_int != 0;
            }
        }
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn sparse_dump_file(
    mut fd: libc::c_int,
    mut st: *mut tar_stat_info,
) -> dump_status {
    let mut rc: bool = false;
    let mut file: tar_sparse_file = tar_sparse_file {
        fd: 0,
        seekable: false,
        offset: 0,
        dumped_size: 0,
        stat_info: 0 as *mut tar_stat_info,
        optab: 0 as *const tar_sparse_optab,
        closure: 0 as *mut libc::c_void,
    };
    if !tar_sparse_init(&mut file) {
        return dump_status_not_implemented;
    }
    file.stat_info = st;
    file.fd = fd;
    file.seekable = 1 as libc::c_int != 0;
    rc = sparse_scan_file(&mut file);
    if rc as libc::c_int != 0 && ((*file.optab).dump_region).is_some() {
        tar_sparse_dump_header(&mut file);
        if fd >= 0 as libc::c_int {
            let mut i: size_t = 0;
            mv_begin_write(
                (*file.stat_info).file_name,
                (*file.stat_info).stat.st_size,
                (*file.stat_info).archive_file_size - file.dumped_size,
            );
            i = 0 as libc::c_int as size_t;
            while rc as libc::c_int != 0 && i < (*file.stat_info).sparse_map_avail {
                rc = tar_sparse_dump_region(&mut file, i);
                i = i.wrapping_add(1);
                i;
            }
        }
    }
    pad_archive((*file.stat_info).archive_file_size - file.dumped_size);
    return (if tar_sparse_done(&mut file) as libc::c_int != 0 && rc as libc::c_int != 0 {
        dump_status_ok as libc::c_int
    } else {
        dump_status_short as libc::c_int
    }) as dump_status;
}
pub unsafe extern "C" fn sparse_member_p(mut st: *mut tar_stat_info) -> bool {
    let mut file: tar_sparse_file = tar_sparse_file {
        fd: 0,
        seekable: false,
        offset: 0,
        dumped_size: 0,
        stat_info: 0 as *mut tar_stat_info,
        optab: 0 as *const tar_sparse_optab,
        closure: 0 as *mut libc::c_void,
    };
    if !tar_sparse_init(&mut file) {
        return 0 as libc::c_int != 0;
    }
    file.stat_info = st;
    return tar_sparse_member_p(&mut file);
}
pub unsafe extern "C" fn sparse_fixup_header(mut st: *mut tar_stat_info) -> bool {
    let mut file: tar_sparse_file = tar_sparse_file {
        fd: 0,
        seekable: false,
        offset: 0,
        dumped_size: 0,
        stat_info: 0 as *mut tar_stat_info,
        optab: 0 as *const tar_sparse_optab,
        closure: 0 as *mut libc::c_void,
    };
    if !tar_sparse_init(&mut file) {
        return 0 as libc::c_int != 0;
    }
    file.stat_info = st;
    return tar_sparse_fixup_header(&mut file);
}
pub unsafe extern "C" fn sparse_extract_file(
    mut fd: libc::c_int,
    mut st: *mut tar_stat_info,
    mut size: *mut off_t,
) -> dump_status {
    let mut rc: bool = 1 as libc::c_int != 0;
    let mut file: tar_sparse_file = tar_sparse_file {
        fd: 0,
        seekable: false,
        offset: 0,
        dumped_size: 0,
        stat_info: 0 as *mut tar_stat_info,
        optab: 0 as *const tar_sparse_optab,
        closure: 0 as *mut libc::c_void,
    };
    let mut i: size_t = 0;
    if !tar_sparse_init(&mut file) {
        return dump_status_not_implemented;
    }
    file.stat_info = st;
    file.fd = fd;
    file
        .seekable = lseek(fd, 0 as libc::c_int as __off_t, 0 as libc::c_int)
        == 0 as libc::c_int as libc::c_long;
    file.offset = 0 as libc::c_int as off_t;
    rc = tar_sparse_decode_header(&mut file);
    i = 0 as libc::c_int as size_t;
    while rc as libc::c_int != 0 && i < (*file.stat_info).sparse_map_avail {
        rc = tar_sparse_extract_region(&mut file, i);
        i = i.wrapping_add(1);
        i;
    }
    *size = (*file.stat_info).archive_file_size - file.dumped_size;
    return (if tar_sparse_done(&mut file) as libc::c_int != 0 && rc as libc::c_int != 0 {
        dump_status_ok as libc::c_int
    } else {
        dump_status_short as libc::c_int
    }) as dump_status;
}
pub unsafe extern "C" fn sparse_skip_file(mut st: *mut tar_stat_info) -> dump_status {
    let mut rc: bool = 1 as libc::c_int != 0;
    let mut file: tar_sparse_file = tar_sparse_file {
        fd: 0,
        seekable: false,
        offset: 0,
        dumped_size: 0,
        stat_info: 0 as *mut tar_stat_info,
        optab: 0 as *const tar_sparse_optab,
        closure: 0 as *mut libc::c_void,
    };
    if !tar_sparse_init(&mut file) {
        return dump_status_not_implemented;
    }
    file.stat_info = st;
    file.fd = -(1 as libc::c_int);
    rc = tar_sparse_decode_header(&mut file);
    skip_file((*file.stat_info).archive_file_size - file.dumped_size);
    return (if tar_sparse_done(&mut file) as libc::c_int != 0 && rc as libc::c_int != 0 {
        dump_status_ok as libc::c_int
    } else {
        dump_status_short as libc::c_int
    }) as dump_status;
}
unsafe extern "C" fn check_sparse_region(
    mut file: *mut tar_sparse_file,
    mut beg: off_t,
    mut end: off_t,
) -> bool {
    if !lseek_or_error(file, beg) {
        return 0 as libc::c_int != 0;
    }
    while beg < end {
        let mut bytes_read: size_t = 0;
        let mut rdsize: size_t = (if (512 as libc::c_int as libc::c_long) < end - beg {
            512 as libc::c_int as libc::c_long
        } else {
            end - beg
        }) as size_t;
        let mut diff_buffer: [libc::c_char; 512] = [0; 512];
        bytes_read = safe_read(
            (*file).fd,
            diff_buffer.as_mut_ptr() as *mut libc::c_void,
            rdsize,
        );
        if bytes_read == -(1 as libc::c_int) as size_t {
            read_diag_details((*(*file).stat_info).orig_file_name, beg, rdsize);
            return 0 as libc::c_int != 0;
        } else if bytes_read == 0 as libc::c_int as libc::c_ulong {
            report_difference(
                (*file).stat_info,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Size differs\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as libc::c_int != 0;
        }
        if !zero_block_p(diff_buffer.as_mut_ptr(), bytes_read) {
            let mut begbuf: [libc::c_char; 21] = [0; 21];
            report_difference(
                (*file).stat_info,
                dcgettext(
                    0 as *const libc::c_char,
                    b"File fragment at %s is not a hole\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                offtostr(beg, begbuf.as_mut_ptr()),
            );
            return 0 as libc::c_int != 0;
        }
        beg = (beg as libc::c_ulong).wrapping_add(bytes_read) as off_t as off_t;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn check_data_region(
    mut file: *mut tar_sparse_file,
    mut i: size_t,
) -> bool {
    let mut size_left: off_t = 0;
    if !lseek_or_error(
        file,
        (*((*(*file).stat_info).sparse_map).offset(i as isize)).offset,
    ) {
        return 0 as libc::c_int != 0;
    }
    size_left = (*((*(*file).stat_info).sparse_map).offset(i as isize)).numbytes;
    mv_size_left((*(*file).stat_info).archive_file_size - (*file).dumped_size);
    while size_left > 0 as libc::c_int as libc::c_long {
        let mut bytes_read: size_t = 0;
        let mut rdsize: size_t = (if size_left > 512 as libc::c_int as libc::c_long {
            512 as libc::c_int as libc::c_long
        } else {
            size_left
        }) as size_t;
        let mut diff_buffer: [libc::c_char; 512] = [0; 512];
        let mut blk: *mut block = find_next_block();
        if blk.is_null() {
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
            return 0 as libc::c_int != 0;
        }
        set_next_block_after(blk);
        (*file).dumped_size += 512 as libc::c_int as libc::c_long;
        bytes_read = safe_read(
            (*file).fd,
            diff_buffer.as_mut_ptr() as *mut libc::c_void,
            rdsize,
        );
        if bytes_read == -(1 as libc::c_int) as size_t {
            read_diag_details(
                (*(*file).stat_info).orig_file_name,
                (*((*(*file).stat_info).sparse_map).offset(i as isize)).offset
                    + (*((*(*file).stat_info).sparse_map).offset(i as isize)).numbytes
                    - size_left,
                rdsize,
            );
            return 0 as libc::c_int != 0;
        } else if bytes_read == 0 as libc::c_int as libc::c_ulong {
            report_difference(
                &mut current_stat_info as *mut tar_stat_info,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Size differs\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as libc::c_int != 0;
        }
        size_left = (size_left as libc::c_ulong).wrapping_sub(bytes_read) as off_t
            as off_t;
        mv_size_left((*(*file).stat_info).archive_file_size - (*file).dumped_size);
        if memcmp(
            ((*blk).buffer).as_mut_ptr() as *const libc::c_void,
            diff_buffer.as_mut_ptr() as *const libc::c_void,
            bytes_read,
        ) != 0
        {
            report_difference(
                (*file).stat_info,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Contents differ\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn sparse_diff_file(
    mut fd: libc::c_int,
    mut st: *mut tar_stat_info,
) -> bool {
    let mut rc: bool = 1 as libc::c_int != 0;
    let mut file: tar_sparse_file = tar_sparse_file {
        fd: 0,
        seekable: false,
        offset: 0,
        dumped_size: 0,
        stat_info: 0 as *mut tar_stat_info,
        optab: 0 as *const tar_sparse_optab,
        closure: 0 as *mut libc::c_void,
    };
    let mut i: size_t = 0;
    let mut offset: off_t = 0 as libc::c_int as off_t;
    if !tar_sparse_init(&mut file) {
        return 0 as libc::c_int != 0;
    }
    file.stat_info = st;
    file.fd = fd;
    file.seekable = 1 as libc::c_int != 0;
    rc = tar_sparse_decode_header(&mut file);
    mv_begin_read(st);
    i = 0 as libc::c_int as size_t;
    while rc as libc::c_int != 0 && i < (*file.stat_info).sparse_map_avail {
        rc = check_sparse_region(
            &mut file,
            offset,
            (*((*file.stat_info).sparse_map).offset(i as isize)).offset,
        ) as libc::c_int != 0 && check_data_region(&mut file, i) as libc::c_int != 0;
        offset = (*((*file.stat_info).sparse_map).offset(i as isize)).offset
            + (*((*file.stat_info).sparse_map).offset(i as isize)).numbytes;
        i = i.wrapping_add(1);
        i;
    }
    if !rc {
        skip_file((*file.stat_info).archive_file_size - file.dumped_size);
    }
    mv_end();
    tar_sparse_done(&mut file);
    return rc;
}
unsafe extern "C" fn oldgnu_sparse_member_p(mut file: *mut tar_sparse_file) -> bool {
    return (*current_header).header.typeflag as libc::c_int == 'S' as i32;
}
unsafe extern "C" fn oldgnu_add_sparse(
    mut file: *mut tar_sparse_file,
    mut s: *mut sparse,
) -> oldgnu_add_status {
    let mut sp: sp_array = sp_array { offset: 0, numbytes: 0 };
    if (*s).numbytes[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
        return add_finish;
    }
    sp
        .offset = off_from_header(
        ((*s).offset).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    );
    sp
        .numbytes = off_from_header(
        ((*s).numbytes).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    );
    if sp.offset < 0 as libc::c_int as libc::c_long
        || sp.numbytes < 0 as libc::c_int as libc::c_long
        || (if (if ((if 1 as libc::c_int != 0 {
            0 as libc::c_int as libc::c_long
        } else {
            (if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                sp.offset
            }) + sp.numbytes
        }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
        {
            !(((((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    sp.offset
                }) + sp.numbytes
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
                    sp.offset
                }) + sp.numbytes
            }) + 0 as libc::c_int as libc::c_long
        }) < 0 as libc::c_int as libc::c_long
        {
            if sp.numbytes < 0 as libc::c_int as libc::c_long {
                (sp.offset
                    < (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            sp.offset
                        }) + sp.numbytes
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                sp.offset
                            }) + sp.numbytes
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
                                sp.offset
                            }) + sp.numbytes
                        }) + 0 as libc::c_int as libc::c_long
                    }) - sp.numbytes) as libc::c_int
            } else {
                ((if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        sp.offset
                    }) + sp.numbytes
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    ((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            sp.offset
                        }) + sp.numbytes
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
                            sp.offset
                        }) + sp.numbytes
                    }) - 1 as libc::c_int as libc::c_long
                }) - sp.numbytes < sp.offset) as libc::c_int
            }
        } else {
            if sp.offset < 0 as libc::c_int as libc::c_long {
                (sp.numbytes <= sp.offset + sp.numbytes) as libc::c_int
            } else {
                if sp.numbytes < 0 as libc::c_int as libc::c_long {
                    (sp.offset <= sp.offset + sp.numbytes) as libc::c_int
                } else {
                    (sp.offset + sp.numbytes < sp.numbytes) as libc::c_int
                }
            }
        }) != 0 || (*(*file).stat_info).stat.st_size < sp.offset + sp.numbytes
        || (*(*file).stat_info).archive_file_size < 0 as libc::c_int as libc::c_long
    {
        return add_fail;
    }
    sparse_add_map((*file).stat_info, &mut sp);
    return add_ok;
}
unsafe extern "C" fn oldgnu_fixup_header(mut file: *mut tar_sparse_file) -> bool {
    let mut realsize: off_t = off_from_header(
        ((*current_header).oldgnu_header.realsize).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    );
    (*(*file).stat_info).archive_file_size = (*(*file).stat_info).stat.st_size;
    (*(*file).stat_info)
        .stat
        .st_size = if (0 as libc::c_int as libc::c_long) < realsize {
        realsize
    } else {
        0 as libc::c_int as libc::c_long
    };
    return 0 as libc::c_int as libc::c_long <= realsize;
}
unsafe extern "C" fn oldgnu_get_sparse_info(mut file: *mut tar_sparse_file) -> bool {
    let mut i: size_t = 0;
    let mut h: *mut block = current_header;
    let mut ext_p: libc::c_int = 0;
    let mut rc: oldgnu_add_status = add_ok;
    (*(*file).stat_info).sparse_map_avail = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < 4 as libc::c_int as libc::c_ulong {
        rc = oldgnu_add_sparse(
            file,
            &mut *((*h).oldgnu_header.sp).as_mut_ptr().offset(i as isize),
        );
        if rc as libc::c_uint != add_ok as libc::c_int as libc::c_uint {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    ext_p = (*h).oldgnu_header.isextended as libc::c_int;
    while rc as libc::c_uint == add_ok as libc::c_int as libc::c_uint && ext_p != 0 {
        h = find_next_block();
        if h.is_null() {
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
            return 0 as libc::c_int != 0;
        }
        set_next_block_after(h);
        i = 0 as libc::c_int as size_t;
        while i < 21 as libc::c_int as libc::c_ulong
            && rc as libc::c_uint == add_ok as libc::c_int as libc::c_uint
        {
            rc = oldgnu_add_sparse(
                file,
                &mut *((*h).sparse_header.sp).as_mut_ptr().offset(i as isize),
            );
            i = i.wrapping_add(1);
            i;
        }
        ext_p = (*h).sparse_header.isextended as libc::c_int;
    }
    if rc as libc::c_uint == add_fail as libc::c_int as libc::c_uint {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: invalid sparse archive member\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*(*file).stat_info).orig_file_name,
        );
        exit_status = 2 as libc::c_int;
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn oldgnu_store_sparse_info(
    mut file: *mut tar_sparse_file,
    mut pindex: *mut size_t,
    mut sp: *mut sparse,
    mut sparse_size: size_t,
) {
    while *pindex < (*(*file).stat_info).sparse_map_avail
        && sparse_size > 0 as libc::c_int as libc::c_ulong
    {
        off_to_chars(
            (*((*(*file).stat_info).sparse_map).offset(*pindex as isize)).offset,
            ((*sp).offset).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
        );
        off_to_chars(
            (*((*(*file).stat_info).sparse_map).offset(*pindex as isize)).numbytes,
            ((*sp).numbytes).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
        );
        sparse_size = sparse_size.wrapping_sub(1);
        sparse_size;
        sp = sp.offset(1);
        sp;
        *pindex = (*pindex).wrapping_add(1);
        *pindex;
    }
}
unsafe extern "C" fn oldgnu_dump_header(mut file: *mut tar_sparse_file) -> bool {
    let mut block_ordinal: off_t = current_block_ordinal();
    let mut blk: *mut block = 0 as *mut block;
    let mut i: size_t = 0;
    blk = start_header((*file).stat_info);
    (*blk).header.typeflag = 'S' as i32 as libc::c_char;
    if (*(*file).stat_info).sparse_map_avail > 4 as libc::c_int as libc::c_ulong {
        (*blk).oldgnu_header.isextended = 1 as libc::c_int as libc::c_char;
    }
    off_to_chars(
        (*(*file).stat_info).stat.st_size,
        ((*blk).oldgnu_header.realsize).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    );
    off_to_chars(
        (*(*file).stat_info).archive_file_size,
        ((*blk).header.size).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    );
    i = 0 as libc::c_int as size_t;
    oldgnu_store_sparse_info(
        file,
        &mut i,
        ((*blk).oldgnu_header.sp).as_mut_ptr(),
        4 as libc::c_int as size_t,
    );
    (*blk)
        .oldgnu_header
        .isextended = (i < (*(*file).stat_info).sparse_map_avail) as libc::c_int
        as libc::c_char;
    finish_header((*file).stat_info, blk, block_ordinal);
    while i < (*(*file).stat_info).sparse_map_avail {
        blk = find_next_block();
        memset(
            ((*blk).buffer).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            512 as libc::c_int as libc::c_ulong,
        );
        oldgnu_store_sparse_info(
            file,
            &mut i,
            ((*blk).sparse_header.sp).as_mut_ptr(),
            21 as libc::c_int as size_t,
        );
        if i < (*(*file).stat_info).sparse_map_avail {
            (*blk).sparse_header.isextended = 1 as libc::c_int as libc::c_char;
        }
        set_next_block_after(blk);
    }
    return 1 as libc::c_int != 0;
}
static mut oldgnu_optab: tar_sparse_optab = unsafe {
    {
        let mut init = tar_sparse_optab {
            init: None,
            done: None,
            sparse_member_p: Some(
                oldgnu_sparse_member_p
                    as unsafe extern "C" fn(*mut tar_sparse_file) -> bool,
            ),
            dump_header: Some(
                oldgnu_dump_header as unsafe extern "C" fn(*mut tar_sparse_file) -> bool,
            ),
            fixup_header: Some(
                oldgnu_fixup_header as unsafe extern "C" fn(*mut tar_sparse_file) -> bool,
            ),
            decode_header: Some(
                oldgnu_get_sparse_info
                    as unsafe extern "C" fn(*mut tar_sparse_file) -> bool,
            ),
            scan_block: None,
            dump_region: Some(
                sparse_dump_region
                    as unsafe extern "C" fn(*mut tar_sparse_file, size_t) -> bool,
            ),
            extract_region: Some(
                sparse_extract_region
                    as unsafe extern "C" fn(*mut tar_sparse_file, size_t) -> bool,
            ),
        };
        init
    }
};
unsafe extern "C" fn star_sparse_member_p(mut file: *mut tar_sparse_file) -> bool {
    return (*current_header).header.typeflag as libc::c_int == 'S' as i32;
}
unsafe extern "C" fn star_fixup_header(mut file: *mut tar_sparse_file) -> bool {
    let mut realsize: off_t = off_from_header(
        ((*current_header).star_in_header.realsize).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    );
    (*(*file).stat_info).archive_file_size = (*(*file).stat_info).stat.st_size;
    (*(*file).stat_info)
        .stat
        .st_size = if (0 as libc::c_int as libc::c_long) < realsize {
        realsize
    } else {
        0 as libc::c_int as libc::c_long
    };
    return 0 as libc::c_int as libc::c_long <= realsize;
}
unsafe extern "C" fn star_get_sparse_info(mut file: *mut tar_sparse_file) -> bool {
    let mut i: size_t = 0;
    let mut h: *mut block = current_header;
    let mut ext_p: libc::c_int = 0;
    let mut rc: oldgnu_add_status = add_ok;
    (*(*file).stat_info).sparse_map_avail = 0 as libc::c_int as size_t;
    if (*h).star_in_header.prefix[0 as libc::c_int as usize] as libc::c_int
        == '\0' as i32
        && (*h)
            .star_in_header
            .sp[0 as libc::c_int as usize]
            .offset[10 as libc::c_int as usize] as libc::c_int != '\0' as i32
    {
        i = 0 as libc::c_int as size_t;
        while i < 4 as libc::c_int as libc::c_ulong {
            rc = oldgnu_add_sparse(
                file,
                &mut *((*h).star_in_header.sp).as_mut_ptr().offset(i as isize),
            );
            if rc as libc::c_uint != add_ok as libc::c_int as libc::c_uint {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
        ext_p = (*h).star_in_header.isextended as libc::c_int;
    } else {
        ext_p = 1 as libc::c_int;
    }
    while rc as libc::c_uint == add_ok as libc::c_int as libc::c_uint && ext_p != 0 {
        h = find_next_block();
        if h.is_null() {
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
            return 0 as libc::c_int != 0;
        }
        set_next_block_after(h);
        i = 0 as libc::c_int as size_t;
        while i < 21 as libc::c_int as libc::c_ulong
            && rc as libc::c_uint == add_ok as libc::c_int as libc::c_uint
        {
            rc = oldgnu_add_sparse(
                file,
                &mut *((*h).star_ext_header.sp).as_mut_ptr().offset(i as isize),
            );
            i = i.wrapping_add(1);
            i;
        }
        (*file).dumped_size += 512 as libc::c_int as libc::c_long;
        ext_p = (*h).star_ext_header.isextended as libc::c_int;
    }
    if rc as libc::c_uint == add_fail as libc::c_int as libc::c_uint {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: invalid sparse archive member\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*(*file).stat_info).orig_file_name,
        );
        exit_status = 2 as libc::c_int;
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
static mut star_optab: tar_sparse_optab = unsafe {
    {
        let mut init = tar_sparse_optab {
            init: None,
            done: None,
            sparse_member_p: Some(
                star_sparse_member_p
                    as unsafe extern "C" fn(*mut tar_sparse_file) -> bool,
            ),
            dump_header: None,
            fixup_header: Some(
                star_fixup_header as unsafe extern "C" fn(*mut tar_sparse_file) -> bool,
            ),
            decode_header: Some(
                star_get_sparse_info
                    as unsafe extern "C" fn(*mut tar_sparse_file) -> bool,
            ),
            scan_block: None,
            dump_region: None,
            extract_region: Some(
                sparse_extract_region
                    as unsafe extern "C" fn(*mut tar_sparse_file, size_t) -> bool,
            ),
        };
        init
    }
};
unsafe extern "C" fn pax_sparse_member_p(mut file: *mut tar_sparse_file) -> bool {
    return (*(*file).stat_info).sparse_map_avail > 0 as libc::c_int as libc::c_ulong
        || (*(*file).stat_info).sparse_major > 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn pax_start_header(mut st: *mut tar_stat_info) -> *mut block {
    let mut realsize: off_t = (*st).stat.st_size;
    let mut blk: *mut block = 0 as *mut block;
    (*st).stat.st_size = (*st).archive_file_size;
    blk = start_header(st);
    (*st).stat.st_size = realsize;
    return blk;
}
unsafe extern "C" fn pax_dump_header_0(mut file: *mut tar_sparse_file) -> bool {
    let mut block_ordinal: off_t = current_block_ordinal();
    let mut blk: *mut block = 0 as *mut block;
    let mut i: size_t = 0;
    let mut nbuf: [libc::c_char; 21] = [0; 21];
    let mut map: *mut sp_array = (*(*file).stat_info).sparse_map;
    let mut save_file_name: *mut libc::c_char = 0 as *mut libc::c_char;
    xheader_store(
        b"GNU.sparse.size\0" as *const u8 as *const libc::c_char,
        (*file).stat_info,
        0 as *const libc::c_void,
    );
    xheader_store(
        b"GNU.sparse.numblocks\0" as *const u8 as *const libc::c_char,
        (*file).stat_info,
        0 as *const libc::c_void,
    );
    if xheader_keyword_deleted_p(b"GNU.sparse.map\0" as *const u8 as *const libc::c_char)
        as libc::c_int != 0 || tar_sparse_minor == 0 as libc::c_int as libc::c_uint
    {
        i = 0 as libc::c_int as size_t;
        while i < (*(*file).stat_info).sparse_map_avail {
            xheader_store(
                b"GNU.sparse.offset\0" as *const u8 as *const libc::c_char,
                (*file).stat_info,
                &mut i as *mut size_t as *const libc::c_void,
            );
            xheader_store(
                b"GNU.sparse.numbytes\0" as *const u8 as *const libc::c_char,
                (*file).stat_info,
                &mut i as *mut size_t as *const libc::c_void,
            );
            i = i.wrapping_add(1);
            i;
        }
    } else {
        xheader_store(
            b"GNU.sparse.name\0" as *const u8 as *const libc::c_char,
            (*file).stat_info,
            0 as *const libc::c_void,
        );
        save_file_name = (*(*file).stat_info).file_name;
        (*(*file).stat_info)
            .file_name = xheader_format_name(
            (*file).stat_info,
            b"%d/GNUSparseFile.%p/%f\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        xheader_string_begin(&mut (*(*file).stat_info).xhdr);
        i = 0 as libc::c_int as size_t;
        while i < (*(*file).stat_info).sparse_map_avail {
            if i != 0 {
                xheader_string_add(
                    &mut (*(*file).stat_info).xhdr,
                    b",\0" as *const u8 as *const libc::c_char,
                );
            }
            xheader_string_add(
                &mut (*(*file).stat_info).xhdr,
                umaxtostr(
                    (*map.offset(i as isize)).offset as uintmax_t,
                    nbuf.as_mut_ptr(),
                ),
            );
            xheader_string_add(
                &mut (*(*file).stat_info).xhdr,
                b",\0" as *const u8 as *const libc::c_char,
            );
            xheader_string_add(
                &mut (*(*file).stat_info).xhdr,
                umaxtostr(
                    (*map.offset(i as isize)).numbytes as uintmax_t,
                    nbuf.as_mut_ptr(),
                ),
            );
            i = i.wrapping_add(1);
            i;
        }
        if !xheader_string_end(
            &mut (*(*file).stat_info).xhdr,
            b"GNU.sparse.map\0" as *const u8 as *const libc::c_char,
        ) {
            rpl_free((*(*file).stat_info).file_name as *mut libc::c_void);
            (*(*file).stat_info).file_name = save_file_name;
            return 0 as libc::c_int != 0;
        }
    }
    blk = pax_start_header((*file).stat_info);
    finish_header((*file).stat_info, blk, block_ordinal);
    if !save_file_name.is_null() {
        rpl_free((*(*file).stat_info).file_name as *mut libc::c_void);
        (*(*file).stat_info).file_name = save_file_name;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pax_dump_header_1(mut file: *mut tar_sparse_file) -> bool {
    let mut block_ordinal: off_t = current_block_ordinal();
    let mut blk: *mut block = 0 as *mut block;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: size_t = 0;
    let mut nbuf: [libc::c_char; 21] = [0; 21];
    let mut size: off_t = 0 as libc::c_int as off_t;
    let mut map: *mut sp_array = (*(*file).stat_info).sparse_map;
    let mut save_file_name: *mut libc::c_char = (*(*file).stat_info).file_name;
    p = umaxtostr((*(*file).stat_info).sparse_map_avail, nbuf.as_mut_ptr());
    size = (size as libc::c_ulong)
        .wrapping_add((strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as off_t as off_t;
    i = 0 as libc::c_int as size_t;
    while i < (*(*file).stat_info).sparse_map_avail {
        p = umaxtostr((*map.offset(i as isize)).offset as uintmax_t, nbuf.as_mut_ptr());
        size = (size as libc::c_ulong)
            .wrapping_add((strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong))
            as off_t as off_t;
        p = umaxtostr(
            (*map.offset(i as isize)).numbytes as uintmax_t,
            nbuf.as_mut_ptr(),
        );
        size = (size as libc::c_ulong)
            .wrapping_add((strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong))
            as off_t as off_t;
        i = i.wrapping_add(1);
        i;
    }
    size = (size + 512 as libc::c_int as libc::c_long - 1 as libc::c_int as libc::c_long)
        / 512 as libc::c_int as libc::c_long;
    (*(*file).stat_info).archive_file_size += size * 512 as libc::c_int as libc::c_long;
    (*file).dumped_size += size * 512 as libc::c_int as libc::c_long;
    xheader_store(
        b"GNU.sparse.major\0" as *const u8 as *const libc::c_char,
        (*file).stat_info,
        0 as *const libc::c_void,
    );
    xheader_store(
        b"GNU.sparse.minor\0" as *const u8 as *const libc::c_char,
        (*file).stat_info,
        0 as *const libc::c_void,
    );
    xheader_store(
        b"GNU.sparse.name\0" as *const u8 as *const libc::c_char,
        (*file).stat_info,
        0 as *const libc::c_void,
    );
    xheader_store(
        b"GNU.sparse.realsize\0" as *const u8 as *const libc::c_char,
        (*file).stat_info,
        0 as *const libc::c_void,
    );
    (*(*file).stat_info)
        .file_name = xheader_format_name(
        (*file).stat_info,
        b"%d/GNUSparseFile.%p/%f\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as size_t,
    );
    if strlen((*(*file).stat_info).file_name) > 100 as libc::c_int as libc::c_ulong {
        *((*(*file).stat_info).file_name)
            .offset(100 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    }
    blk = pax_start_header((*file).stat_info);
    finish_header((*file).stat_info, blk, block_ordinal);
    rpl_free((*(*file).stat_info).file_name as *mut libc::c_void);
    (*(*file).stat_info).file_name = save_file_name;
    blk = find_next_block();
    q = ((*blk).buffer).as_mut_ptr();
    p = umaxtostr((*(*file).stat_info).sparse_map_avail, nbuf.as_mut_ptr());
    let mut endp: *mut libc::c_char = ((*blk).buffer)
        .as_mut_ptr()
        .offset(512 as libc::c_int as isize);
    let mut srcp: *const libc::c_char = p;
    while *srcp != 0 {
        if q == endp {
            set_next_block_after(blk);
            blk = find_next_block();
            q = ((*blk).buffer).as_mut_ptr();
            endp = ((*blk).buffer).as_mut_ptr().offset(512 as libc::c_int as isize);
        }
        let fresh2 = srcp;
        srcp = srcp.offset(1);
        let fresh3 = q;
        q = q.offset(1);
        *fresh3 = *fresh2;
    }
    let mut endp_0: *mut libc::c_char = ((*blk).buffer)
        .as_mut_ptr()
        .offset(512 as libc::c_int as isize);
    let mut srcp_0: *const libc::c_char = b"\n\0" as *const u8 as *const libc::c_char;
    while *srcp_0 != 0 {
        if q == endp_0 {
            set_next_block_after(blk);
            blk = find_next_block();
            q = ((*blk).buffer).as_mut_ptr();
            endp_0 = ((*blk).buffer).as_mut_ptr().offset(512 as libc::c_int as isize);
        }
        let fresh4 = srcp_0;
        srcp_0 = srcp_0.offset(1);
        let fresh5 = q;
        q = q.offset(1);
        *fresh5 = *fresh4;
    }
    i = 0 as libc::c_int as size_t;
    while i < (*(*file).stat_info).sparse_map_avail {
        p = umaxtostr((*map.offset(i as isize)).offset as uintmax_t, nbuf.as_mut_ptr());
        let mut endp_1: *mut libc::c_char = ((*blk).buffer)
            .as_mut_ptr()
            .offset(512 as libc::c_int as isize);
        let mut srcp_1: *const libc::c_char = p;
        while *srcp_1 != 0 {
            if q == endp_1 {
                set_next_block_after(blk);
                blk = find_next_block();
                q = ((*blk).buffer).as_mut_ptr();
                endp_1 = ((*blk).buffer)
                    .as_mut_ptr()
                    .offset(512 as libc::c_int as isize);
            }
            let fresh6 = srcp_1;
            srcp_1 = srcp_1.offset(1);
            let fresh7 = q;
            q = q.offset(1);
            *fresh7 = *fresh6;
        }
        let mut endp_2: *mut libc::c_char = ((*blk).buffer)
            .as_mut_ptr()
            .offset(512 as libc::c_int as isize);
        let mut srcp_2: *const libc::c_char = b"\n\0" as *const u8
            as *const libc::c_char;
        while *srcp_2 != 0 {
            if q == endp_2 {
                set_next_block_after(blk);
                blk = find_next_block();
                q = ((*blk).buffer).as_mut_ptr();
                endp_2 = ((*blk).buffer)
                    .as_mut_ptr()
                    .offset(512 as libc::c_int as isize);
            }
            let fresh8 = srcp_2;
            srcp_2 = srcp_2.offset(1);
            let fresh9 = q;
            q = q.offset(1);
            *fresh9 = *fresh8;
        }
        p = umaxtostr(
            (*map.offset(i as isize)).numbytes as uintmax_t,
            nbuf.as_mut_ptr(),
        );
        let mut endp_3: *mut libc::c_char = ((*blk).buffer)
            .as_mut_ptr()
            .offset(512 as libc::c_int as isize);
        let mut srcp_3: *const libc::c_char = p;
        while *srcp_3 != 0 {
            if q == endp_3 {
                set_next_block_after(blk);
                blk = find_next_block();
                q = ((*blk).buffer).as_mut_ptr();
                endp_3 = ((*blk).buffer)
                    .as_mut_ptr()
                    .offset(512 as libc::c_int as isize);
            }
            let fresh10 = srcp_3;
            srcp_3 = srcp_3.offset(1);
            let fresh11 = q;
            q = q.offset(1);
            *fresh11 = *fresh10;
        }
        let mut endp_4: *mut libc::c_char = ((*blk).buffer)
            .as_mut_ptr()
            .offset(512 as libc::c_int as isize);
        let mut srcp_4: *const libc::c_char = b"\n\0" as *const u8
            as *const libc::c_char;
        while *srcp_4 != 0 {
            if q == endp_4 {
                set_next_block_after(blk);
                blk = find_next_block();
                q = ((*blk).buffer).as_mut_ptr();
                endp_4 = ((*blk).buffer)
                    .as_mut_ptr()
                    .offset(512 as libc::c_int as isize);
            }
            let fresh12 = srcp_4;
            srcp_4 = srcp_4.offset(1);
            let fresh13 = q;
            q = q.offset(1);
            *fresh13 = *fresh12;
        }
        i = i.wrapping_add(1);
        i;
    }
    memset(
        q as *mut libc::c_void,
        0 as libc::c_int,
        (512 as libc::c_int as libc::c_long
            - q.offset_from(((*blk).buffer).as_mut_ptr()) as libc::c_long)
            as libc::c_ulong,
    );
    set_next_block_after(blk);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pax_dump_header(mut file: *mut tar_sparse_file) -> bool {
    (*(*file).stat_info).sparse_major = tar_sparse_major;
    (*(*file).stat_info).sparse_minor = tar_sparse_minor;
    return if (*(*file).stat_info).sparse_major == 0 as libc::c_int as libc::c_uint {
        pax_dump_header_0(file) as libc::c_int
    } else {
        pax_dump_header_1(file) as libc::c_int
    } != 0;
}
unsafe extern "C" fn decode_num(
    mut num: *mut uintmax_t,
    mut arg: *const libc::c_char,
    mut maxval: uintmax_t,
) -> bool {
    let mut u: uintmax_t = 0;
    let mut arg_lim: *mut libc::c_char = 0 as *mut libc::c_char;
    if !((*arg as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
        <= 9 as libc::c_int as libc::c_uint)
    {
        return 0 as libc::c_int != 0;
    }
    *__errno_location() = 0 as libc::c_int;
    u = strtoumax(arg, &mut arg_lim, 10 as libc::c_int);
    if !(u <= maxval && *__errno_location() != 34 as libc::c_int)
        || *arg_lim as libc::c_int != 0
    {
        return 0 as libc::c_int != 0;
    }
    *num = u;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pax_decode_header(mut file: *mut tar_sparse_file) -> bool {
    if (*(*file).stat_info).sparse_major > 0 as libc::c_int as libc::c_uint {
        let mut u: uintmax_t = 0;
        let mut nbuf: [libc::c_char; 21] = [0; 21];
        let mut blk: *mut block = 0 as *mut block;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut i: size_t = 0;
        let mut start: off_t = 0;
        start = current_block_ordinal();
        set_next_block_after(current_header);
        blk = find_next_block();
        if blk.is_null() {
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
        p = ((*blk).buffer).as_mut_ptr();
        let mut endp: *mut libc::c_char = ((*blk).buffer)
            .as_mut_ptr()
            .offset(512 as libc::c_int as isize);
        let mut dst: *mut libc::c_char = nbuf.as_mut_ptr();
        loop {
            if dst
                == nbuf
                    .as_mut_ptr()
                    .offset(
                        (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
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
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    )
                    .offset(-(1 as libc::c_int as isize))
            {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: numeric overflow in sparse archive member\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*(*file).stat_info).orig_file_name,
                );
                exit_status = 2 as libc::c_int;
                return 0 as libc::c_int != 0;
            }
            if p == endp {
                set_next_block_after(blk);
                blk = find_next_block();
                if blk.is_null() {
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
                    fatal_exit();
                }
                p = ((*blk).buffer).as_mut_ptr();
                endp = ((*blk).buffer).as_mut_ptr().offset(512 as libc::c_int as isize);
            }
            let fresh14 = p;
            p = p.offset(1);
            *dst = *fresh14;
            let fresh15 = dst;
            dst = dst.offset(1);
            if !(*fresh15 as libc::c_int != '\n' as i32) {
                break;
            }
        }
        *dst.offset(-(1 as libc::c_int) as isize) = 0 as libc::c_int as libc::c_char;
        if !decode_num(
            &mut u,
            nbuf.as_mut_ptr(),
            if (0 as libc::c_int as size_t) < -(1 as libc::c_int) as size_t {
                -(1 as libc::c_int) as size_t
            } else {
                ((1 as libc::c_int as size_t)
                    << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            },
        ) {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: malformed sparse archive member\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*(*file).stat_info).orig_file_name,
            );
            exit_status = 2 as libc::c_int;
            return 0 as libc::c_int != 0;
        }
        (*(*file).stat_info).sparse_map_size = u;
        (*(*file).stat_info)
            .sparse_map = xcalloc(
            (*(*file).stat_info).sparse_map_size,
            ::std::mem::size_of::<sp_array>() as libc::c_ulong,
        ) as *mut sp_array;
        (*(*file).stat_info).sparse_map_avail = 0 as libc::c_int as size_t;
        i = 0 as libc::c_int as size_t;
        while i < (*(*file).stat_info).sparse_map_size {
            let mut sp: sp_array = sp_array { offset: 0, numbytes: 0 };
            let mut endp_0: *mut libc::c_char = ((*blk).buffer)
                .as_mut_ptr()
                .offset(512 as libc::c_int as isize);
            let mut dst_0: *mut libc::c_char = nbuf.as_mut_ptr();
            loop {
                if dst_0
                    == nbuf
                        .as_mut_ptr()
                        .offset(
                            (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
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
                                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                        .offset(-(1 as libc::c_int as isize))
                {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: numeric overflow in sparse archive member\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*(*file).stat_info).orig_file_name,
                    );
                    exit_status = 2 as libc::c_int;
                    return 0 as libc::c_int != 0;
                }
                if p == endp_0 {
                    set_next_block_after(blk);
                    blk = find_next_block();
                    if blk.is_null() {
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
                        fatal_exit();
                    }
                    p = ((*blk).buffer).as_mut_ptr();
                    endp_0 = ((*blk).buffer)
                        .as_mut_ptr()
                        .offset(512 as libc::c_int as isize);
                }
                let fresh16 = p;
                p = p.offset(1);
                *dst_0 = *fresh16;
                let fresh17 = dst_0;
                dst_0 = dst_0.offset(1);
                if !(*fresh17 as libc::c_int != '\n' as i32) {
                    break;
                }
            }
            *dst_0
                .offset(-(1 as libc::c_int) as isize) = 0 as libc::c_int as libc::c_char;
            if !decode_num(
                &mut u,
                nbuf.as_mut_ptr(),
                (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                    -(1 as libc::c_int) as off_t
                } else {
                    (((1 as libc::c_int as off_t)
                        << (::std::mem::size_of::<off_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                }) as uintmax_t,
            ) {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: malformed sparse archive member\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*(*file).stat_info).orig_file_name,
                );
                exit_status = 2 as libc::c_int;
                return 0 as libc::c_int != 0;
            }
            sp.offset = u as off_t;
            let mut endp_1: *mut libc::c_char = ((*blk).buffer)
                .as_mut_ptr()
                .offset(512 as libc::c_int as isize);
            let mut dst_1: *mut libc::c_char = nbuf.as_mut_ptr();
            loop {
                if dst_1
                    == nbuf
                        .as_mut_ptr()
                        .offset(
                            (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
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
                                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                        .offset(-(1 as libc::c_int as isize))
                {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: numeric overflow in sparse archive member\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*(*file).stat_info).orig_file_name,
                    );
                    exit_status = 2 as libc::c_int;
                    return 0 as libc::c_int != 0;
                }
                if p == endp_1 {
                    set_next_block_after(blk);
                    blk = find_next_block();
                    if blk.is_null() {
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
                        fatal_exit();
                    }
                    p = ((*blk).buffer).as_mut_ptr();
                    endp_1 = ((*blk).buffer)
                        .as_mut_ptr()
                        .offset(512 as libc::c_int as isize);
                }
                let fresh18 = p;
                p = p.offset(1);
                *dst_1 = *fresh18;
                let fresh19 = dst_1;
                dst_1 = dst_1.offset(1);
                if !(*fresh19 as libc::c_int != '\n' as i32) {
                    break;
                }
            }
            *dst_1
                .offset(-(1 as libc::c_int) as isize) = 0 as libc::c_int as libc::c_char;
            if !decode_num(
                &mut u,
                nbuf.as_mut_ptr(),
                (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                    -(1 as libc::c_int) as off_t
                } else {
                    (((1 as libc::c_int as off_t)
                        << (::std::mem::size_of::<off_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                }) as uintmax_t,
            ) {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: malformed sparse archive member\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*(*file).stat_info).orig_file_name,
                );
                exit_status = 2 as libc::c_int;
                return 0 as libc::c_int != 0;
            }
            sp.numbytes = u as off_t;
            sparse_add_map((*file).stat_info, &mut sp);
            i = i.wrapping_add(1);
            i;
        }
        set_next_block_after(blk);
        (*file).dumped_size
            += 512 as libc::c_int as libc::c_long * (current_block_ordinal() - start);
    }
    return 1 as libc::c_int != 0;
}
static mut pax_optab: tar_sparse_optab = unsafe {
    {
        let mut init = tar_sparse_optab {
            init: None,
            done: None,
            sparse_member_p: Some(
                pax_sparse_member_p as unsafe extern "C" fn(*mut tar_sparse_file) -> bool,
            ),
            dump_header: Some(
                pax_dump_header as unsafe extern "C" fn(*mut tar_sparse_file) -> bool,
            ),
            fixup_header: None,
            decode_header: Some(
                pax_decode_header as unsafe extern "C" fn(*mut tar_sparse_file) -> bool,
            ),
            scan_block: None,
            dump_region: Some(
                sparse_dump_region
                    as unsafe extern "C" fn(*mut tar_sparse_file, size_t) -> bool,
            ),
            extract_region: Some(
                sparse_extract_region
                    as unsafe extern "C" fn(*mut tar_sparse_file, size_t) -> bool,
            ),
        };
        init
    }
};
