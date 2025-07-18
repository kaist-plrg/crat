use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __dirstream;
    pub type exclist;
    pub type directory;
    pub type namebuf;
    fn abort() -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
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
    fn read_fatal_details(_: *const libc::c_char, _: off_t, _: size_t) -> !;
    fn stat_error(_: *const libc::c_char);
    fn fatal_exit() -> !;
    fn quotearg_colon(arg: *const libc::c_char) -> *mut libc::c_char;
    static mut subcommand_option: subcommand;
    static mut archive_format: archive_format;
    static mut interactive_option: bool;
    static mut current_stat_info: tar_stat_info;
    fn available_space_after(pointer: *mut block) -> size_t;
    fn close_archive();
    fn find_next_block() -> *mut block;
    fn open_archive(mode: access_mode);
    fn reset_eof();
    fn set_next_block_after(block: *mut block);
    fn dump_file(
        parent: *mut tar_stat_info,
        name: *const libc::c_char,
        fullname: *const libc::c_char,
    );
    fn write_eot();
    static mut current_header: *mut block;
    static mut current_format: archive_format;
    fn decode_header(
        header: *mut block,
        stat_info: *mut tar_stat_info,
        format_pointer: *mut archive_format,
        do_user_group: libc::c_int,
    );
    fn transform_stat_info(typeflag: libc::c_int, stat_info: *mut tar_stat_info);
    fn read_header(
        return_block: *mut *mut block,
        info: *mut tar_stat_info,
        m: read_header_mode,
    ) -> read_header;
    fn skip_member();
    fn tar_savedir(
        name: *const libc::c_char,
        must_exist: libc::c_int,
    ) -> *mut libc::c_char;
    fn namebuf_create(dir: *const libc::c_char) -> namebuf_t;
    fn namebuf_free(buf: namebuf_t);
    fn namebuf_name(buf: namebuf_t, name: *const libc::c_char) -> *mut libc::c_char;
    fn deref_stat(name: *const libc::c_char, buf: *mut stat) -> libc::c_int;
    static mut chdir_fd: libc::c_int;
    fn chdir_do(dir: libc::c_int);
    fn name_gather();
    fn addname(
        string: *const libc::c_char,
        change_dir: libc::c_int,
        cmdline: bool,
        parent: *mut name,
    ) -> *mut name;
    fn remname(name: *mut name);
    fn names_notfound();
    fn name_scan(name: *const libc::c_char) -> *mut name;
    fn name_from_list() -> *const name;
    fn confirm(
        message_action: *const libc::c_char,
        name: *const libc::c_char,
    ) -> libc::c_int;
    fn tar_stat_destroy(st: *mut tar_stat_info);
    fn tar_timespec_cmp(a: timespec, b: timespec) -> libc::c_int;
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn finish_deferred_unlinks();
    fn excluded_name(name: *const libc::c_char, st: *mut tar_stat_info) -> bool;
    fn xheader_forbid_global();
    static mut current_block: *mut block;
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
pub type namebuf_t = *mut namebuf;
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
pub static mut time_to_start_writing: bool = false;
pub static mut output_start: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
unsafe extern "C" fn append_file(mut file_name: *mut libc::c_char) {
    let mut handle: libc::c_int = openat(
        chdir_fd,
        file_name,
        0 as libc::c_int | 0 as libc::c_int,
    );
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
    if handle < 0 as libc::c_int {
        open_error(file_name);
        return;
    }
    if fstat(handle, &mut stat_data) != 0 as libc::c_int {
        stat_error(file_name);
    } else {
        let mut bytes_left: off_t = stat_data.st_size;
        while bytes_left > 0 as libc::c_int as libc::c_long {
            let mut start: *mut block = find_next_block();
            let mut buffer_size: size_t = available_space_after(start);
            let mut status: size_t = 0;
            let mut buf: [libc::c_char; 21] = [0; 21];
            if (bytes_left as libc::c_ulong) < buffer_size {
                buffer_size = bytes_left as size_t;
                status = buffer_size.wrapping_rem(512 as libc::c_int as libc::c_ulong);
                if status != 0 {
                    memset(
                        ((*start).buffer).as_mut_ptr().offset(bytes_left as isize)
                            as *mut libc::c_void,
                        0 as libc::c_int,
                        (512 as libc::c_int as libc::c_ulong).wrapping_sub(status),
                    );
                }
            }
            status = safe_read(
                handle,
                ((*start).buffer).as_mut_ptr() as *mut libc::c_void,
                buffer_size,
            );
            if status == -(1 as libc::c_int) as size_t {
                read_fatal_details(
                    file_name,
                    stat_data.st_size - bytes_left,
                    buffer_size,
                );
            }
            if status == 0 as libc::c_int as libc::c_ulong {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcngettext(
                        0 as *const libc::c_char,
                        b"%s: File shrank by %s byte\0" as *const u8
                            as *const libc::c_char,
                        b"%s: File shrank by %s bytes\0" as *const u8
                            as *const libc::c_char,
                        bytes_left as libc::c_ulong,
                        5 as libc::c_int,
                    ),
                    quotearg_colon(file_name),
                    umaxtostr(bytes_left as uintmax_t, buf.as_mut_ptr()),
                );
                fatal_exit();
            }
            bytes_left = (bytes_left as libc::c_ulong).wrapping_sub(status) as off_t
                as off_t;
            set_next_block_after(
                start
                    .offset(
                        status
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_div(512 as libc::c_int as libc::c_ulong) as isize,
                    ),
            );
        }
    }
    if close(handle) != 0 as libc::c_int {
        close_error(file_name);
    }
}
pub unsafe extern "C" fn update_archive() {
    let mut previous_status: read_header = HEADER_STILL_UNREAD;
    let mut found_end: bool = 0 as libc::c_int != 0;
    name_gather();
    open_archive(ACCESS_UPDATE);
    xheader_forbid_global();
    while !found_end {
        let mut status: read_header = read_header(
            &mut current_header,
            &mut current_stat_info,
            read_header_auto,
        );
        match status as libc::c_uint {
            0 | 2 => {
                abort();
            }
            1 => {
                let mut name: *mut name = 0 as *mut name;
                decode_header(
                    current_header,
                    &mut current_stat_info,
                    &mut current_format,
                    0 as libc::c_int,
                );
                transform_stat_info(
                    (*current_header).header.typeflag as libc::c_int,
                    &mut current_stat_info,
                );
                archive_format = current_format;
                if subcommand_option as libc::c_uint
                    == UPDATE_SUBCOMMAND as libc::c_int as libc::c_uint
                    && {
                        name = name_scan(current_stat_info.file_name);
                        !name.is_null()
                    }
                {
                    let mut s: stat = stat {
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
                    chdir_do((*name).change_dir);
                    if deref_stat(current_stat_info.file_name, &mut s)
                        == 0 as libc::c_int
                    {
                        if s.st_mode & 0o170000 as libc::c_int as libc::c_uint
                            == 0o40000 as libc::c_int as libc::c_uint
                        {
                            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
                            let mut dirp: *mut libc::c_char = tar_savedir(
                                (*name).name,
                                1 as libc::c_int,
                            );
                            if !dirp.is_null() {
                                let mut nbuf: namebuf_t = namebuf_create((*name).name);
                                p = dirp;
                                while *p != 0 {
                                    addname(
                                        namebuf_name(nbuf, p),
                                        (*name).change_dir,
                                        0 as libc::c_int != 0,
                                        0 as *mut name,
                                    );
                                    p = p
                                        .offset(
                                            (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        );
                                }
                                namebuf_free(nbuf);
                                rpl_free(dirp as *mut libc::c_void);
                                remname(name);
                            }
                        } else if tar_timespec_cmp(
                            get_stat_mtime(&mut s),
                            current_stat_info.mtime,
                        ) <= 0 as libc::c_int
                        {
                            remname(name);
                        }
                    }
                }
                skip_member();
            }
            3 => {
                current_block = current_header;
                found_end = 1 as libc::c_int != 0;
            }
            4 => {
                found_end = 1 as libc::c_int != 0;
            }
            5 => {
                set_next_block_after(current_header);
                let mut current_block_43: u64;
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
                        current_block_43 = 16134383897737664421;
                    }
                    1 | 3 => {
                        current_block_43 = 16134383897737664421;
                    }
                    4 | 2 => {
                        abort();
                    }
                    5 | _ => {
                        current_block_43 = 15512526488502093901;
                    }
                }
                match current_block_43 {
                    16134383897737664421 => {
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
            _ => {}
        }
        tar_stat_destroy(&mut current_stat_info);
        previous_status = status;
    }
    reset_eof();
    time_to_start_writing = 1 as libc::c_int != 0;
    output_start = ((*current_block).buffer).as_mut_ptr();
    let mut p_0: *const name = 0 as *const name;
    loop {
        p_0 = name_from_list();
        if p_0.is_null() {
            break;
        }
        let mut file_name: *mut libc::c_char = (*p_0).name;
        if excluded_name(file_name, 0 as *mut tar_stat_info) {
            continue;
        }
        if interactive_option as libc::c_int != 0
            && confirm(b"add\0" as *const u8 as *const libc::c_char, file_name) == 0
        {
            continue;
        }
        if subcommand_option as libc::c_uint
            == CAT_SUBCOMMAND as libc::c_int as libc::c_uint
        {
            append_file(file_name);
        } else {
            dump_file(0 as *mut tar_stat_info, file_name, file_name);
        }
    }
    write_eot();
    close_archive();
    finish_deferred_unlinks();
    names_notfound();
}
