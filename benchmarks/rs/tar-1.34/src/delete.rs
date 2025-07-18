use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __dirstream;
    pub type exclist;
    pub type directory;
    fn abort() -> !;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn truncate_warn(_: *const libc::c_char);
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
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
    static mut error_hook: Option::<unsafe extern "C" fn() -> ()>;
    static mut exit_status: libc::c_int;
    fn seek_error_details(_: *const libc::c_char, _: off_t);
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    static mut blocking_factor: libc::c_int;
    static mut record_size: size_t;
    static mut ignore_zeros_option: bool;
    static mut occurrence_option: uintmax_t;
    static mut archive: libc::c_int;
    static mut current_stat_info: tar_stat_info;
    static mut archive_name_array: *mut *const libc::c_char;
    static mut write_archive_to_stdout: bool;
    static mut records_written: off_t;
    fn close_archive();
    fn flush_read();
    fn flush_write();
    fn flush_archive();
    fn open_archive(mode: access_mode);
    fn set_next_block_after(block: *mut block);
    static mut current_header: *mut block;
    static mut recent_long_name_blocks: size_t;
    static mut recent_long_link_blocks: size_t;
    fn read_header(
        return_block: *mut *mut block,
        info: *mut tar_stat_info,
        m: read_header_mode,
    ) -> read_header;
    fn skip_member();
    fn name_gather();
    fn names_notfound();
    fn name_scan(name: *const libc::c_char) -> *mut name;
    fn tar_stat_destroy(st: *mut tar_stat_info);
    fn xheader_decode(stat: *mut tar_stat_info);
    fn sys_truncate(fd: libc::c_int) -> libc::c_int;
    fn rmt_lseek__(_: libc::c_int, _: off_t, _: libc::c_int) -> off_t;
    fn rmt_ioctl__(_: libc::c_int, _: libc::c_int, _: *mut libc::c_char) -> libc::c_int;
    static mut record_start: *mut block;
    static mut record_end: *mut block;
    static mut current_block: *mut block;
    static mut recent_long_name: *mut block;
    static mut recent_long_link: *mut block;
    static mut records_read: off_t;
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
pub type __syscall_slong_t = libc::c_long;
pub type off_t = __off_t;
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
pub type access_mode = libc::c_uint;
pub const ACCESS_UPDATE: access_mode = 2;
pub const ACCESS_WRITE: access_mode = 1;
pub const ACCESS_READ: access_mode = 0;
pub const HEADER_END_OF_FILE: read_header = 4;
pub type read_header = libc::c_uint;
pub const HEADER_FAILURE: read_header = 5;
pub const HEADER_ZERO_BLOCK: read_header = 3;
pub const HEADER_SUCCESS_EXTENDED: read_header = 2;
pub const HEADER_SUCCESS: read_header = 1;
pub const HEADER_STILL_UNREAD: read_header = 0;
pub type read_header_mode = libc::c_uint;
pub const read_header_x_global: read_header_mode = 2;
pub const read_header_x_raw: read_header_mode = 1;
pub const read_header_auto: read_header_mode = 0;
static mut new_record: *mut block = 0 as *const block as *mut block;
static mut new_blocks: libc::c_int = 0;
static mut acting_as_filter: bool = false;
pub static mut records_skipped: off_t = 0;
unsafe extern "C" fn move_archive(mut count: off_t) {
    if count == 0 as libc::c_int as libc::c_long {
        return;
    }
    let mut operation: mtop = mtop { mt_op: 0, mt_count: 0 };
    if if count < 0 as libc::c_int as libc::c_long {
        operation.mt_op = 4 as libc::c_int as libc::c_short;
        operation.mt_count = -count as libc::c_int;
        (operation.mt_count as libc::c_long == -count) as libc::c_int
    } else {
        operation.mt_op = 3 as libc::c_int as libc::c_short;
        operation.mt_count = count as libc::c_int;
        (operation.mt_count as libc::c_long == count) as libc::c_int
    } != 0
    {
        if 0 as libc::c_int
            <= (if archive >= (1 as libc::c_int) << 30 as libc::c_int {
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
            })
        {
            return;
        }
        if *__errno_location() == 5 as libc::c_int
            && 0 as libc::c_int
                <= (if archive >= (1 as libc::c_int) << 30 as libc::c_int {
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
                })
        {
            return;
        }
    }
    let mut position0: off_t = if archive >= (1 as libc::c_int) << 30 as libc::c_int {
        rmt_lseek__(
            archive - ((1 as libc::c_int) << 30 as libc::c_int),
            0 as libc::c_int as off_t,
            1 as libc::c_int,
        )
    } else {
        lseek(archive, 0 as libc::c_int as off_t, 1 as libc::c_int)
    };
    let mut increment: off_t = record_size.wrapping_mul(count as libc::c_ulong) as off_t;
    let mut position: off_t = position0 + increment;
    if (increment / count) as libc::c_ulong != record_size
        || (position < position0) as libc::c_int
            != (increment < 0 as libc::c_int as libc::c_long) as libc::c_int
        || {
            position = if position < 0 as libc::c_int as libc::c_long {
                0 as libc::c_int as libc::c_long
            } else {
                position
            };
            (if archive >= (1 as libc::c_int) << 30 as libc::c_int {
                rmt_lseek__(
                    archive - ((1 as libc::c_int) << 30 as libc::c_int),
                    position,
                    0 as libc::c_int,
                )
            } else {
                lseek(archive, position, 0 as libc::c_int)
            }) != position
        }
    {
        seek_error_details(
            *archive_name_array.offset(0 as libc::c_int as isize),
            position,
        );
    }
}
unsafe extern "C" fn write_record(mut move_back_flag: libc::c_int) {
    let mut save_record: *mut block = record_start;
    record_start = new_record;
    if acting_as_filter {
        archive = 1 as libc::c_int;
        flush_write();
        archive = 0 as libc::c_int;
    } else {
        move_archive(records_written + records_skipped - records_read);
        flush_write();
    }
    record_start = save_record;
    if move_back_flag != 0 {
        if !acting_as_filter {
            move_archive(records_read - (records_written + records_skipped));
        }
    }
    new_blocks = 0 as libc::c_int;
}
unsafe extern "C" fn write_recent_blocks(mut h: *mut block, mut blocks: size_t) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < blocks {
        let fresh0 = new_blocks;
        new_blocks = new_blocks + 1;
        *new_record.offset(fresh0 as isize) = *h.offset(i as isize);
        if new_blocks == blocking_factor {
            write_record(1 as libc::c_int);
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn write_recent_bytes(mut data: *mut libc::c_char, mut bytes: size_t) {
    let mut blocks: size_t = bytes.wrapping_div(512 as libc::c_int as libc::c_ulong);
    let mut rest: size_t = bytes
        .wrapping_sub(blocks.wrapping_mul(512 as libc::c_int as libc::c_ulong));
    write_recent_blocks(data as *mut block, blocks);
    memcpy(
        ((*new_record.offset(new_blocks as isize)).buffer).as_mut_ptr()
            as *mut libc::c_void,
        data.offset(blocks.wrapping_mul(512 as libc::c_int as libc::c_ulong) as isize)
            as *const libc::c_void,
        rest,
    );
    if rest < 512 as libc::c_int as libc::c_ulong {
        memset(
            ((*new_record.offset(new_blocks as isize)).buffer)
                .as_mut_ptr()
                .offset(rest as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (512 as libc::c_int as libc::c_ulong).wrapping_sub(rest),
        );
    }
    new_blocks += 1;
    new_blocks;
    if new_blocks == blocking_factor {
        write_record(1 as libc::c_int);
    }
}
#[inline]
unsafe extern "C" fn flush_file() {
    let mut blocks_to_skip: off_t = 0;
    set_next_block_after(current_header);
    blocks_to_skip = (current_stat_info.stat.st_size + 512 as libc::c_int as libc::c_long
        - 1 as libc::c_int as libc::c_long) / 512 as libc::c_int as libc::c_long;
    while record_end.offset_from(current_block) as libc::c_long <= blocks_to_skip {
        blocks_to_skip -= record_end.offset_from(current_block) as libc::c_long;
        flush_archive();
    }
    current_block = current_block.offset(blocks_to_skip as isize);
}
pub unsafe extern "C" fn delete_archive_members() {
    let mut logical_status: read_header = HEADER_STILL_UNREAD;
    let mut previous_status: read_header = HEADER_STILL_UNREAD;
    let mut name: *mut name = 0 as *mut name;
    let mut blocks_to_keep: off_t = 0 as libc::c_int as off_t;
    let mut kept_blocks_in_record: libc::c_int = 0;
    name_gather();
    open_archive(ACCESS_UPDATE);
    acting_as_filter = strcmp(
        *archive_name_array.offset(0 as libc::c_int as isize),
        b"-\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int;
    loop {
        let mut status: read_header = read_header(
            &mut current_header,
            &mut current_stat_info,
            read_header_x_raw,
        );
        let mut current_block_28: u64;
        match status as libc::c_uint {
            0 => {
                abort();
            }
            1 => {
                name = name_scan(current_stat_info.file_name);
                if name.is_null() {
                    skip_member();
                    current_block_28 = 14136749492126903395;
                } else {
                    (*name).found_count = ((*name).found_count).wrapping_add(1);
                    (*name).found_count;
                    if if occurrence_option == 0 as libc::c_int as libc::c_ulong {
                        ((*name).found_count != 0 as libc::c_int as libc::c_ulong)
                            as libc::c_int
                    } else {
                        ((*name).found_count == occurrence_option) as libc::c_int
                    } == 0
                    {
                        skip_member();
                        current_block_28 = 14136749492126903395;
                    } else {
                        current_block_28 = 789247138018192599;
                    }
                }
            }
            2 => {
                current_block_28 = 789247138018192599;
            }
            3 => {
                if ignore_zeros_option {
                    set_next_block_after(current_header);
                    current_block_28 = 14136749492126903395;
                } else {
                    current_block_28 = 4668966650698264838;
                }
            }
            4 => {
                current_block_28 = 4668966650698264838;
            }
            5 => {
                set_next_block_after(current_header);
                let mut current_block_27: u64;
                match previous_status as libc::c_uint {
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
                        current_block_27 = 3292440606114030865;
                    }
                    1 | 2 | 3 => {
                        current_block_27 = 3292440606114030865;
                    }
                    4 => {
                        abort();
                    }
                    5 | _ => {
                        current_block_27 = 18377268871191777778;
                    }
                }
                match current_block_27 {
                    3292440606114030865 => {
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
                current_block_28 = 14136749492126903395;
            }
            _ => {
                current_block_28 = 14136749492126903395;
            }
        }
        match current_block_28 {
            789247138018192599 => {
                logical_status = status;
            }
            4668966650698264838 => {
                logical_status = HEADER_END_OF_FILE;
            }
            _ => {}
        }
        previous_status = status;
        if !(logical_status as libc::c_uint
            == HEADER_STILL_UNREAD as libc::c_int as libc::c_uint)
        {
            break;
        }
    }
    records_skipped = records_read - 1 as libc::c_int as libc::c_long;
    new_record = xmalloc(record_size) as *mut block;
    if logical_status as libc::c_uint == HEADER_SUCCESS as libc::c_int as libc::c_uint
        || logical_status as libc::c_uint
            == HEADER_SUCCESS_EXTENDED as libc::c_int as libc::c_uint
    {
        write_archive_to_stdout = 0 as libc::c_int != 0;
        new_blocks = current_block.offset_from(record_start) as libc::c_long
            as libc::c_int;
        if new_blocks != 0 {
            memcpy(
                new_record as *mut libc::c_void,
                record_start as *const libc::c_void,
                (new_blocks * 512 as libc::c_int) as libc::c_ulong,
            );
        }
        if logical_status as libc::c_uint
            == HEADER_SUCCESS as libc::c_int as libc::c_uint
        {
            logical_status = HEADER_STILL_UNREAD;
            flush_file();
        }
        while logical_status as libc::c_uint
            != HEADER_END_OF_FILE as libc::c_int as libc::c_uint
        {
            let mut status_0: read_header = HEADER_STILL_UNREAD;
            if current_block == record_end {
                flush_archive();
            }
            status_0 = read_header(
                &mut current_header,
                &mut current_stat_info,
                read_header_auto,
            );
            let mut current_block_97: u64;
            match status_0 as libc::c_uint {
                0 | 2 => {
                    abort();
                }
                1 => {
                    xheader_decode(&mut current_stat_info);
                    name = name_scan(current_stat_info.file_name);
                    if !name.is_null() {
                        (*name).found_count = ((*name).found_count).wrapping_add(1);
                        (*name).found_count;
                        if if occurrence_option == 0 as libc::c_int as libc::c_ulong {
                            ((*name).found_count != 0 as libc::c_int as libc::c_ulong)
                                as libc::c_int
                        } else {
                            ((*name).found_count == occurrence_option) as libc::c_int
                        } != 0
                        {
                            flush_file();
                            current_block_97 = 5687667889785024198;
                        } else {
                            current_block_97 = 8151474771948790331;
                        }
                    } else {
                        current_block_97 = 8151474771948790331;
                    }
                    match current_block_97 {
                        5687667889785024198 => {}
                        _ => {
                            if current_stat_info.xhdr.size != 0 {
                                write_recent_bytes(
                                    current_stat_info.xhdr.buffer,
                                    current_stat_info.xhdr.size,
                                );
                            } else {
                                write_recent_blocks(
                                    recent_long_name,
                                    recent_long_name_blocks,
                                );
                                write_recent_blocks(
                                    recent_long_link,
                                    recent_long_link_blocks,
                                );
                            }
                            *new_record.offset(new_blocks as isize) = *current_header;
                            new_blocks += 1;
                            new_blocks;
                            blocks_to_keep = (current_stat_info.stat.st_size
                                + 512 as libc::c_int as libc::c_long
                                - 1 as libc::c_int as libc::c_long)
                                / 512 as libc::c_int as libc::c_long;
                            set_next_block_after(current_header);
                            if new_blocks == blocking_factor {
                                write_record(1 as libc::c_int);
                            }
                            kept_blocks_in_record = record_end.offset_from(current_block)
                                as libc::c_long as libc::c_int;
                            if kept_blocks_in_record as libc::c_long > blocks_to_keep {
                                kept_blocks_in_record = blocks_to_keep as libc::c_int;
                            }
                            while blocks_to_keep != 0 {
                                let mut count: libc::c_int = 0;
                                if current_block == record_end {
                                    flush_read();
                                    current_block = record_start;
                                    kept_blocks_in_record = blocking_factor;
                                    if kept_blocks_in_record as libc::c_long > blocks_to_keep {
                                        kept_blocks_in_record = blocks_to_keep as libc::c_int;
                                    }
                                }
                                count = kept_blocks_in_record;
                                if blocking_factor - new_blocks < count {
                                    count = blocking_factor - new_blocks;
                                }
                                if count == 0 {
                                    abort();
                                }
                                memcpy(
                                    new_record.offset(new_blocks as isize) as *mut libc::c_void,
                                    current_block as *const libc::c_void,
                                    (count * 512 as libc::c_int) as libc::c_ulong,
                                );
                                new_blocks += count;
                                current_block = current_block.offset(count as isize);
                                blocks_to_keep -= count as libc::c_long;
                                kept_blocks_in_record -= count;
                                if new_blocks == blocking_factor {
                                    write_record(1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                3 => {
                    if ignore_zeros_option {
                        set_next_block_after(current_header);
                    } else {
                        logical_status = HEADER_END_OF_FILE;
                    }
                }
                4 => {
                    logical_status = HEADER_END_OF_FILE;
                }
                5 => {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Deleting non-header from archive\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    exit_status = 2 as libc::c_int;
                    set_next_block_after(current_header);
                }
                _ => {
                    abort();
                }
            }
            tar_stat_destroy(&mut current_stat_info);
        }
        if logical_status as libc::c_uint
            == HEADER_END_OF_FILE as libc::c_int as libc::c_uint
        {
            let mut total_zero_blocks: libc::c_int = 0 as libc::c_int;
            loop {
                let mut zero_blocks: libc::c_int = blocking_factor - new_blocks;
                memset(
                    new_record.offset(new_blocks as isize) as *mut libc::c_void,
                    0 as libc::c_int,
                    (512 as libc::c_int * zero_blocks) as libc::c_ulong,
                );
                total_zero_blocks += zero_blocks;
                write_record((total_zero_blocks < 2 as libc::c_int) as libc::c_int);
                if !(total_zero_blocks < 2 as libc::c_int) {
                    break;
                }
            }
        }
        if !acting_as_filter && !(archive >= (1 as libc::c_int) << 30 as libc::c_int) {
            if sys_truncate(archive) != 0 {
                truncate_warn(*archive_name_array.offset(0 as libc::c_int as isize));
            }
        }
    }
    rpl_free(new_record as *mut libc::c_void);
    close_archive();
    names_notfound();
}
