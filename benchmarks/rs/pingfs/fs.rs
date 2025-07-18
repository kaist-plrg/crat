use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type fuse_pollhandle;
    pub type fuse_dirhandle;
    pub type io;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn host_get_next() -> *mut host;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn net_send(
        host: *mut host,
        id: uint16_t,
        seqno: uint16_t,
        data: *const uint8_t,
        len: size_t,
    );
    fn net_start();
    fn net_stop();
    fn chunk_create() -> *mut chunk;
    fn chunk_free(c: *mut chunk);
    fn chunk_add(c: *mut chunk);
    fn chunk_remove(c: *mut chunk);
    fn chunk_wait_for(c: *mut chunk, data: *mut *mut uint8_t) -> libc::c_int;
    fn chunk_done(c: *mut chunk, data: *mut uint8_t, len: size_t);
    fn __errno_location() -> *mut libc::c_int;
    fn getuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __ino64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __fsblkcnt64_t = libc::c_ulong;
pub type __fsfilcnt64_t = libc::c_ulong;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ino_t = __ino64_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct fuse_file_info {
    pub flags: libc::c_int,
    pub fh_old: libc::c_ulong,
    pub writepage: libc::c_int,
    #[bitfield(name = "direct_io", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "keep_cache", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "flush", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "nonseekable", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "flock_release", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "padding", ty = "libc::c_uint", bits = "5..=31")]
    pub direct_io_keep_cache_flush_nonseekable_flock_release_padding: [u8; 4],
    pub fh: uint64_t,
    pub lock_owner: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_conn_info {
    pub proto_major: libc::c_uint,
    pub proto_minor: libc::c_uint,
    pub async_read: libc::c_uint,
    pub max_write: libc::c_uint,
    pub max_readahead: libc::c_uint,
    pub capable: libc::c_uint,
    pub want: libc::c_uint,
    pub max_background: libc::c_uint,
    pub congestion_threshold: libc::c_uint,
    pub reserved: [libc::c_uint; 23],
}
pub type fuse_buf_flags = libc::c_uint;
pub const FUSE_BUF_FD_RETRY: fuse_buf_flags = 8;
pub const FUSE_BUF_FD_SEEK: fuse_buf_flags = 4;
pub const FUSE_BUF_IS_FD: fuse_buf_flags = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_buf {
    pub size: size_t,
    pub flags: fuse_buf_flags,
    pub mem: *mut libc::c_void,
    pub fd: libc::c_int,
    pub pos: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_bufvec {
    pub count: size_t,
    pub idx: size_t,
    pub off: size_t,
    pub buf: [fuse_buf; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flock {
    pub l_type: libc::c_short,
    pub l_whence: libc::c_short,
    pub l_start: __off64_t,
    pub l_len: __off64_t,
    pub l_pid: __pid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utimbuf {
    pub actime: __time_t,
    pub modtime: __time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statvfs {
    pub f_bsize: libc::c_ulong,
    pub f_frsize: libc::c_ulong,
    pub f_blocks: __fsblkcnt64_t,
    pub f_bfree: __fsblkcnt64_t,
    pub f_bavail: __fsblkcnt64_t,
    pub f_files: __fsfilcnt64_t,
    pub f_ffree: __fsfilcnt64_t,
    pub f_favail: __fsfilcnt64_t,
    pub f_fsid: libc::c_ulong,
    pub f_flag: libc::c_ulong,
    pub f_namemax: libc::c_ulong,
    pub __f_spare: [libc::c_int; 6],
}
pub type fuse_fill_dir_t = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_char,
        *const stat,
        off_t,
    ) -> libc::c_int,
>;
pub type fuse_dirh_t = *mut fuse_dirhandle;
pub type fuse_dirfil_t = Option::<
    unsafe extern "C" fn(
        fuse_dirh_t,
        *const libc::c_char,
        libc::c_int,
        ino_t,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct fuse_operations {
    pub getattr: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
    pub readlink: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut libc::c_char,
            size_t,
        ) -> libc::c_int,
    >,
    pub getdir: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            fuse_dirh_t,
            fuse_dirfil_t,
        ) -> libc::c_int,
    >,
    pub mknod: Option::<
        unsafe extern "C" fn(*const libc::c_char, mode_t, dev_t) -> libc::c_int,
    >,
    pub mkdir: Option::<
        unsafe extern "C" fn(*const libc::c_char, mode_t) -> libc::c_int,
    >,
    pub unlink: Option::<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>,
    pub rmdir: Option::<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>,
    pub symlink: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
    pub rename: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
    pub link: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
    pub chmod: Option::<
        unsafe extern "C" fn(*const libc::c_char, mode_t) -> libc::c_int,
    >,
    pub chown: Option::<
        unsafe extern "C" fn(*const libc::c_char, uid_t, gid_t) -> libc::c_int,
    >,
    pub truncate: Option::<
        unsafe extern "C" fn(*const libc::c_char, off_t) -> libc::c_int,
    >,
    pub utime: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut utimbuf) -> libc::c_int,
    >,
    pub open: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut fuse_file_info) -> libc::c_int,
    >,
    pub read: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut libc::c_char,
            size_t,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub write: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            size_t,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub statfs: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut statvfs) -> libc::c_int,
    >,
    pub flush: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut fuse_file_info) -> libc::c_int,
    >,
    pub release: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut fuse_file_info) -> libc::c_int,
    >,
    pub fsync: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub setxattr: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            *const libc::c_char,
            size_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub getxattr: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            *mut libc::c_char,
            size_t,
        ) -> libc::c_int,
    >,
    pub listxattr: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut libc::c_char,
            size_t,
        ) -> libc::c_int,
    >,
    pub removexattr: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    >,
    pub opendir: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut fuse_file_info) -> libc::c_int,
    >,
    pub readdir: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut libc::c_void,
            fuse_fill_dir_t,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub releasedir: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut fuse_file_info) -> libc::c_int,
    >,
    pub fsyncdir: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub init: Option::<unsafe extern "C" fn(*mut fuse_conn_info) -> *mut libc::c_void>,
    pub destroy: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub access: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub create: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            mode_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub ftruncate: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub fgetattr: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut stat,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub lock: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut fuse_file_info,
            libc::c_int,
            *mut flock,
        ) -> libc::c_int,
    >,
    pub utimens: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const timespec) -> libc::c_int,
    >,
    pub bmap: Option::<
        unsafe extern "C" fn(*const libc::c_char, size_t, *mut uint64_t) -> libc::c_int,
    >,
    #[bitfield(name = "flag_nullpath_ok", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "flag_nopath", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "flag_utime_omit_ok", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "flag_reserved", ty = "libc::c_uint", bits = "3..=31")]
    pub flag_nullpath_ok_flag_nopath_flag_utime_omit_ok_flag_reserved: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub ioctl: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
            *mut libc::c_void,
            *mut fuse_file_info,
            libc::c_uint,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub poll: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut fuse_file_info,
            *mut fuse_pollhandle,
            *mut libc::c_uint,
        ) -> libc::c_int,
    >,
    pub write_buf: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut fuse_bufvec,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub read_buf: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut *mut fuse_bufvec,
            size_t,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
    pub flock: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut fuse_file_info,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub fallocate: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
            off_t,
            off_t,
            *mut fuse_file_info,
        ) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file {
    pub next: *mut file,
    pub name: *const libc::c_char,
    pub chunks: *mut chunk,
    pub mode: mode_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chunk {
    pub next_active: *mut chunk,
    pub next_file: *mut chunk,
    pub host: *mut host,
    pub io: *mut io,
    pub id: uint16_t,
    pub seqno: uint16_t,
    pub len: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct host {
    pub next: *mut host,
    pub sockaddr: sockaddr_storage,
    pub sockaddr_len: socklen_t,
}
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
}
pub type sa_family_t = libc::c_ushort;
pub static mut files: *mut file = 0 as *const file as *mut file;
unsafe extern "C" fn fs_free(mut f: *mut file) {
    let mut c: *mut chunk = 0 as *mut chunk;
    let mut next: *mut chunk = 0 as *mut chunk;
    c = (*f).chunks;
    while !c.is_null() {
        next = (*c).next_file;
        chunk_remove(c);
        chunk_free(c);
        c = next;
    }
    free((*f).name as *mut libc::c_void);
    free(f as *mut libc::c_void);
}
unsafe extern "C" fn file_size(mut f: *mut file) -> size_t {
    let mut c: *mut chunk = 0 as *mut chunk;
    let mut size: size_t = 0 as libc::c_int as size_t;
    c = (*f).chunks;
    while !c.is_null() {
        size = (size as libc::c_ulong).wrapping_add((*c).len as libc::c_ulong) as size_t
            as size_t;
        c = (*c).next_file;
    }
    return size;
}
unsafe extern "C" fn fs_init(mut conn: *mut fuse_conn_info) -> *mut libc::c_void {
    net_start();
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn fs_destroy(mut data: *mut libc::c_void) {
    let mut f: *mut file = 0 as *mut file;
    net_stop();
    f = files;
    while !f.is_null() {
        let mut next: *mut file = (*f).next;
        fs_free(f);
        f = next;
    }
}
unsafe extern "C" fn find_file(mut name: *const libc::c_char) -> *mut file {
    let mut f: *mut file = 0 as *mut file;
    f = files;
    while !f.is_null() {
        if strcmp(name, (*f).name) == 0 as libc::c_int {
            return f;
        }
        f = (*f).next;
    }
    return 0 as *mut file;
}
unsafe extern "C" fn fs_mkdir(
    mut name: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    return -(95 as libc::c_int);
}
unsafe extern "C" fn fs_mknod(
    mut name: *const libc::c_char,
    mut mode: mode_t,
    mut device: dev_t,
) -> libc::c_int {
    let mut f: *mut file = 0 as *mut file;
    if !(mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint)
    {
        return -(95 as libc::c_int);
    }
    f = find_file(name);
    if !f.is_null() {
        return -(17 as libc::c_int);
    }
    f = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<file>() as libc::c_ulong,
    ) as *mut file;
    if f.is_null() {
        return -*__errno_location();
    }
    (*f).name = strdup(name);
    (*f).mode = mode;
    (*f).next = files;
    files = f;
    return 0 as libc::c_int;
}
unsafe extern "C" fn fs_chmod(
    mut name: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    let mut f: *mut file = 0 as *mut file;
    f = find_file(name);
    if f.is_null() {
        return -(2 as libc::c_int);
    }
    (*f).mode = mode;
    return 0 as libc::c_int;
}
unsafe extern "C" fn fs_utime(
    mut name: *const libc::c_char,
    mut utim: *mut utimbuf,
) -> libc::c_int {
    let mut f: *mut file = 0 as *mut file;
    f = find_file(name);
    if f.is_null() {
        return -(2 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fs_getattr(
    mut name: *const libc::c_char,
    mut stat: *mut stat,
) -> libc::c_int {
    let mut f: *mut file = 0 as *mut file;
    (*stat).st_nlink = 1 as libc::c_int as __nlink_t;
    (*stat).st_uid = getuid();
    (*stat).st_gid = getgid();
    (*stat).st_atim.tv_sec = 0 as libc::c_int as __time_t;
    (*stat).st_mtim.tv_sec = 0 as libc::c_int as __time_t;
    (*stat).st_ctim.tv_sec = 0 as libc::c_int as __time_t;
    if strcmp(b"/\0" as *const u8 as *const libc::c_char, name) == 0 as libc::c_int {
        (*stat).st_mode = (0o40000 as libc::c_int | 0o775 as libc::c_int) as __mode_t;
        (*stat).st_size = 0 as libc::c_int as __off_t;
        (*stat).st_blksize = 0 as libc::c_int as __blksize_t;
        (*stat).st_blocks = 0 as libc::c_int as __blkcnt_t;
        return 0 as libc::c_int;
    }
    f = find_file(name);
    if f.is_null() {
        return -(2 as libc::c_int);
    }
    (*stat).st_mode = (*f).mode;
    (*stat).st_size = file_size(f) as __off_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn fs_unlink(mut name: *const libc::c_char) -> libc::c_int {
    let mut f: *mut file = files;
    let mut last: *mut file = 0 as *mut file;
    while !f.is_null() {
        if strcmp(name, (*f).name) == 0 as libc::c_int {
            if !last.is_null() {
                (*last).next = (*f).next;
            } else {
                files = (*f).next;
            }
            fs_free(f);
            return 0 as libc::c_int;
        }
        last = f;
        f = (*f).next;
    }
    return -(2 as libc::c_int);
}
unsafe extern "C" fn fs_readdir(
    mut path: *const libc::c_char,
    mut buf: *mut libc::c_void,
    mut filler: fuse_fill_dir_t,
    mut offset: off_t,
    mut fi: *mut fuse_file_info,
) -> libc::c_int {
    let mut f: *mut file = 0 as *mut file;
    if strcmp(b"/\0" as *const u8 as *const libc::c_char, path) != 0 {
        return -(2 as libc::c_int);
    }
    filler
        .unwrap()(
        buf,
        b".\0" as *const u8 as *const libc::c_char,
        0 as *const stat,
        0 as libc::c_int as off_t,
    );
    filler
        .unwrap()(
        buf,
        b"..\0" as *const u8 as *const libc::c_char,
        0 as *const stat,
        0 as libc::c_int as off_t,
    );
    f = files;
    while !f.is_null() {
        filler
            .unwrap()(
            buf,
            &*((*f).name).offset(1 as libc::c_int as isize),
            0 as *const stat,
            0 as libc::c_int as off_t,
        );
        f = (*f).next;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fs_open(
    mut name: *const libc::c_char,
    mut fileinfo: *mut fuse_file_info,
) -> libc::c_int {
    let mut f: *mut file = 0 as *mut file;
    f = find_file(name);
    if f.is_null() {
        return -(2 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fs_inner_write(
    mut f: *mut file,
    mut buf: *const libc::c_char,
    mut size: size_t,
    mut offset: off_t,
) -> libc::c_int {
    let mut c: *mut chunk = 0 as *mut chunk;
    let mut last: *mut chunk = 0 as *mut chunk;
    let mut chunkdata: *mut uint8_t = 0 as *mut uint8_t;
    let mut len: libc::c_int = 0;
    let mut clen: libc::c_int = 0;
    c = (*f).chunks;
    while !c.is_null() && offset >= (*c).len as libc::c_long {
        if (*c).len as libc::c_int != 1024 as libc::c_int {
            break;
        }
        offset -= (*c).len as libc::c_long;
        last = c;
        c = (*c).next_file;
    }
    if c.is_null() {
        c = chunk_create();
        (*c)
            .len = (if size < 1024 as libc::c_int as libc::c_ulong {
            size
        } else {
            1024 as libc::c_int as libc::c_ulong
        }) as uint16_t;
        chunk_add(c);
        if !last.is_null() {
            (*last).next_file = c;
        } else {
            (*f).chunks = c;
        }
        (*c).host = host_get_next();
        net_send(
            (*c).host,
            (*c).id,
            (*c).seqno,
            buf as *const uint8_t,
            (*c).len as size_t,
        );
        return (*c).len as libc::c_int;
    }
    chunkdata = 0 as *mut uint8_t;
    clen = chunk_wait_for(c, &mut chunkdata);
    if clen <= 0 as libc::c_int {
        return clen;
    }
    clen = (if (1024 as libc::c_int as libc::c_ulong)
        < size.wrapping_add(offset as libc::c_ulong)
    {
        1024 as libc::c_int as libc::c_ulong
    } else {
        size.wrapping_add(offset as libc::c_ulong)
    }) as libc::c_int;
    len = (if ((clen as libc::c_long - offset) as libc::c_ulong) < size {
        (clen as libc::c_long - offset) as libc::c_ulong
    } else {
        size
    }) as libc::c_int;
    chunkdata = realloc(chunkdata as *mut libc::c_void, clen as libc::c_ulong)
        as *mut uint8_t;
    memcpy(
        &mut *chunkdata.offset(offset as isize) as *mut uint8_t as *mut libc::c_void,
        buf as *const libc::c_void,
        len as libc::c_ulong,
    );
    chunk_done(c, chunkdata, clen as size_t);
    return len;
}
unsafe extern "C" fn fs_write(
    mut name: *const libc::c_char,
    mut buf: *const libc::c_char,
    mut size: size_t,
    mut offset: off_t,
    mut fileinfo: *mut fuse_file_info,
) -> libc::c_int {
    let mut f: *mut file = 0 as *mut file;
    f = find_file(name);
    if f.is_null() {
        return -(2 as libc::c_int);
    }
    return fs_inner_write(f, buf, size, offset);
}
unsafe extern "C" fn fs_read(
    mut name: *const libc::c_char,
    mut buf: *mut libc::c_char,
    mut size: size_t,
    mut offset: off_t,
    mut fileinfo: *mut fuse_file_info,
) -> libc::c_int {
    let mut f: *mut file = 0 as *mut file;
    let mut c: *mut chunk = 0 as *mut chunk;
    let mut chunkdata: *mut uint8_t = 0 as *mut uint8_t;
    let mut len: libc::c_int = 0;
    let mut clen: libc::c_int = 0;
    f = find_file(name);
    if f.is_null() {
        return -(2 as libc::c_int);
    }
    c = (*f).chunks;
    while !c.is_null() && offset >= (*c).len as libc::c_long {
        offset -= (*c).len as libc::c_long;
        c = (*c).next_file;
    }
    if c.is_null() {
        return 0 as libc::c_int;
    }
    len = (if (((*c).len as libc::c_long - offset) as libc::c_ulong) < size {
        ((*c).len as libc::c_long - offset) as libc::c_ulong
    } else {
        size
    }) as libc::c_int;
    chunkdata = 0 as *mut uint8_t;
    clen = chunk_wait_for(c, &mut chunkdata);
    if clen == 0 {
        return -(5 as libc::c_int);
    }
    memcpy(
        buf as *mut libc::c_void,
        &mut *chunkdata.offset(offset as isize) as *mut uint8_t as *const libc::c_void,
        len as libc::c_ulong,
    );
    chunk_done(c, chunkdata, clen as size_t);
    return len;
}
unsafe extern "C" fn shrink_file(mut f: *mut file, mut length: off_t) -> libc::c_int {
    let mut c: *mut chunk = (*f).chunks;
    let mut prev: *mut chunk = 0 as *mut chunk;
    while (*c).len as libc::c_long <= length {
        length -= (*c).len as libc::c_long;
        prev = c;
        c = (*c).next_file;
    }
    if length == 0 {
        if !prev.is_null() {
            (*prev).next_file = 0 as *mut chunk;
        } else {
            (*f).chunks = 0 as *mut chunk;
        }
    }
    while !c.is_null() {
        let mut next: *mut chunk = (*c).next_file;
        if length != 0 {
            let mut cdata: *mut uint8_t = 0 as *mut uint8_t;
            let mut clen: libc::c_int = 0;
            clen = chunk_wait_for(c, &mut cdata);
            if clen == 0 {
                return -(5 as libc::c_int);
            }
            chunk_done(c, cdata, length as size_t);
            (*c).next_file = 0 as *mut chunk;
            length = 0 as libc::c_int as off_t;
        } else {
            chunk_remove(c);
            chunk_free(c);
        }
        c = next;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn grow_file(mut f: *mut file, mut length: off_t) -> libc::c_int {
    let mut offset: libc::c_int = file_size(f) as libc::c_int;
    let mut to_grow: libc::c_int = (length - offset as libc::c_long) as libc::c_int;
    let mut zerobuf: [libc::c_char; 1024] = [0; 1024];
    memset(
        zerobuf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    while to_grow != 0 {
        let mut res: libc::c_int = fs_inner_write(
            f,
            zerobuf.as_mut_ptr(),
            (if to_grow < 1024 as libc::c_int { to_grow } else { 1024 as libc::c_int })
                as size_t,
            offset as off_t,
        );
        if res < 0 as libc::c_int {
            return res;
        }
        if res != 0 {} else {
            __assert_fail(
                b"res\0" as *const u8 as *const libc::c_char,
                b"fs.c\0" as *const u8 as *const libc::c_char,
                379 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"int grow_file(struct file *, off_t)\0"))
                    .as_ptr(),
            );
        };
        offset += res;
        to_grow -= res;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fs_truncate(
    mut name: *const libc::c_char,
    mut length: off_t,
) -> libc::c_int {
    let mut f: *mut file = 0 as *mut file;
    let mut cur_size: libc::c_int = 0;
    f = find_file(name);
    if f.is_null() {
        return -(2 as libc::c_int);
    }
    cur_size = file_size(f) as libc::c_int;
    if length > cur_size as libc::c_long {
        return grow_file(f, length);
    }
    if length < cur_size as libc::c_long {
        return shrink_file(f, length);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fs_rename(
    mut name: *const libc::c_char,
    mut newname: *const libc::c_char,
) -> libc::c_int {
    let mut f: *mut file = 0 as *mut file;
    f = find_file(name);
    if f.is_null() {
        return -(2 as libc::c_int);
    }
    free((*f).name as *mut libc::c_void);
    (*f).name = strdup(newname);
    return 0 as libc::c_int;
}
pub static mut fs_ops: fuse_operations = fuse_operations {
    getattr: None,
    readlink: None,
    getdir: None,
    mknod: None,
    mkdir: None,
    unlink: None,
    rmdir: None,
    symlink: None,
    rename: None,
    link: None,
    chmod: None,
    chown: None,
    truncate: None,
    utime: None,
    open: None,
    read: None,
    write: None,
    statfs: None,
    flush: None,
    release: None,
    fsync: None,
    setxattr: None,
    getxattr: None,
    listxattr: None,
    removexattr: None,
    opendir: None,
    readdir: None,
    releasedir: None,
    fsyncdir: None,
    init: None,
    destroy: None,
    access: None,
    create: None,
    ftruncate: None,
    fgetattr: None,
    lock: None,
    utimens: None,
    bmap: None,
    flag_nullpath_ok_flag_nopath_flag_utime_omit_ok_flag_reserved: [0; 4],
    c2rust_padding: [0; 4],
    ioctl: None,
    poll: None,
    write_buf: None,
    read_buf: None,
    flock: None,
    fallocate: None,
};
unsafe extern "C" fn run_static_initializers() {
    fs_ops = {
        let mut init = fuse_operations {
            flag_nullpath_ok_flag_nopath_flag_utime_omit_ok_flag_reserved: [0; 4],
            c2rust_padding: [0; 4],
            getattr: Some(
                fs_getattr
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut stat,
                    ) -> libc::c_int,
            ),
            readlink: None,
            getdir: None,
            mknod: Some(
                fs_mknod
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        mode_t,
                        dev_t,
                    ) -> libc::c_int,
            ),
            mkdir: Some(
                fs_mkdir
                    as unsafe extern "C" fn(*const libc::c_char, mode_t) -> libc::c_int,
            ),
            unlink: Some(
                fs_unlink as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
            ),
            rmdir: None,
            symlink: None,
            rename: Some(
                fs_rename
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
            link: None,
            chmod: Some(
                fs_chmod
                    as unsafe extern "C" fn(*const libc::c_char, mode_t) -> libc::c_int,
            ),
            chown: None,
            truncate: Some(
                fs_truncate
                    as unsafe extern "C" fn(*const libc::c_char, off_t) -> libc::c_int,
            ),
            utime: Some(
                fs_utime
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut utimbuf,
                    ) -> libc::c_int,
            ),
            open: Some(
                fs_open
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut fuse_file_info,
                    ) -> libc::c_int,
            ),
            read: Some(
                fs_read
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut libc::c_char,
                        size_t,
                        off_t,
                        *mut fuse_file_info,
                    ) -> libc::c_int,
            ),
            write: Some(
                fs_write
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                        size_t,
                        off_t,
                        *mut fuse_file_info,
                    ) -> libc::c_int,
            ),
            statfs: None,
            flush: None,
            release: None,
            fsync: None,
            setxattr: None,
            getxattr: None,
            listxattr: None,
            removexattr: None,
            opendir: None,
            readdir: Some(
                fs_readdir
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *mut libc::c_void,
                        fuse_fill_dir_t,
                        off_t,
                        *mut fuse_file_info,
                    ) -> libc::c_int,
            ),
            releasedir: None,
            fsyncdir: None,
            init: Some(
                fs_init as unsafe extern "C" fn(*mut fuse_conn_info) -> *mut libc::c_void,
            ),
            destroy: Some(fs_destroy as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            access: None,
            create: None,
            ftruncate: None,
            fgetattr: None,
            lock: None,
            utimens: None,
            bmap: None,
            ioctl: None,
            poll: None,
            write_buf: None,
            read_buf: None,
            flock: None,
            fallocate: None,
        };
        init.set_flag_nullpath_ok(0);
        init.set_flag_nopath(0);
        init.set_flag_utime_omit_ok(0);
        init.set_flag_reserved(0);
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
