use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type fuse_pollhandle;
    pub type fuse_dirhandle;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn getaddrinfo_a(
        __mode: libc::c_int,
        __list: *mut *mut gaicb,
        __ent: libc::c_int,
        __sig: *mut sigevent,
    ) -> libc::c_int;
    fn gai_error(__req: *mut gaicb) -> libc::c_int;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn host_make_resolvlist(
        hostfile: *mut FILE,
        list: *mut *mut *mut gaicb,
    ) -> libc::c_int;
    fn host_free_resolvlist(list: *mut *mut gaicb, length: libc::c_int);
    fn host_create(list: *mut *mut gaicb, listlength: libc::c_int) -> *mut host;
    fn host_evaluate(
        hosts: *mut *mut host,
        length: libc::c_int,
        timeout: libc::c_int,
    ) -> libc::c_int;
    fn host_use(hosts: *mut host);
    fn fuse_opt_parse(
        args: *mut fuse_args,
        data: *mut libc::c_void,
        opts: *const fuse_opt,
        proc_0: fuse_opt_proc_t,
    ) -> libc::c_int;
    fn fuse_opt_add_arg(args: *mut fuse_args, arg: *const libc::c_char) -> libc::c_int;
    fn fuse_opt_free_args(args: *mut fuse_args);
    fn fuse_main_real(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        op: *const fuse_operations,
        op_size: size_t,
        user_data: *mut libc::c_void,
    ) -> libc::c_int;
    static fs_ops: fuse_operations;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn net_open_sockets() -> libc::c_int;
    fn chunk_set_timeout(t: libc::c_int);
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
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
pub type uint64_t = __uint64_t;
pub type socklen_t = __socklen_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigevent {
    pub sigev_value: __sigval_t,
    pub sigev_signo: libc::c_int,
    pub sigev_notify: libc::c_int,
    pub _sigev_un: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 12],
    pub _tid: __pid_t,
    pub _sigev_thread: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _function: Option::<unsafe extern "C" fn(__sigval_t) -> ()>,
    pub _attribute: *mut pthread_attr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gaicb {
    pub ar_name: *const libc::c_char,
    pub ar_service: *const libc::c_char,
    pub ar_request: *const addrinfo,
    pub ar_result: *mut addrinfo,
    pub __return: libc::c_int,
    pub __glibc_reserved: [libc::c_int; 5],
}
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
pub struct host {
    pub next: *mut host,
    pub sockaddr: sockaddr_storage,
    pub sockaddr_len: socklen_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_opt {
    pub templ: *const libc::c_char,
    pub offset: libc::c_ulong,
    pub value: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_args {
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub allocated: libc::c_int,
}
pub type fuse_opt_proc_t = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_char,
        libc::c_int,
        *mut fuse_args,
    ) -> libc::c_int,
>;
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
pub struct arginfo {
    pub hostfile: *mut libc::c_char,
    pub mountpoint: *mut libc::c_char,
    pub num_args: libc::c_int,
    pub timeout: libc::c_int,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const KEY_TIMEOUT: C2RustUnnamed_1 = 2;
pub const KEY_ASUSER: C2RustUnnamed_1 = 1;
pub const KEY_HELP: C2RustUnnamed_1 = 0;
static mut pingfs_opts: [fuse_opt; 4] = [fuse_opt {
    templ: 0 as *const libc::c_char,
    offset: 0,
    value: 0,
}; 4];
unsafe extern "C" fn read_hostnames(
    mut hfile: *const libc::c_char,
    mut list: *mut *mut *mut gaicb,
) -> libc::c_int {
    let mut h: libc::c_int = 0 as libc::c_int;
    let mut file: *mut FILE = 0 as *mut FILE;
    if strcmp(b"-\0" as *const u8 as *const libc::c_char, hfile) == 0 as libc::c_int {
        file = stdin;
    } else {
        file = fopen(hfile, b"r\0" as *const u8 as *const libc::c_char);
        if file.is_null() {
            perror(b"Failed to read file\0" as *const u8 as *const libc::c_char);
            return h;
        }
    }
    h = host_make_resolvlist(file, list);
    fclose(file);
    return h;
}
unsafe extern "C" fn resolve_names(
    mut list: *mut *mut gaicb,
    mut names: libc::c_int,
    mut hosts: *mut *mut host,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut hostcount: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    fprintf(
        stderr,
        b"Resolving %d hostnames... \0" as *const u8 as *const libc::c_char,
        names,
    );
    fflush(stderr);
    ret = getaddrinfo_a(0 as libc::c_int, list, names, 0 as *mut sigevent);
    if ret != 0 as libc::c_int {
        fprintf(
            stderr,
            b"Resolving failed: %s\n\0" as *const u8 as *const libc::c_char,
            gai_strerror(ret),
        );
        return -(1 as libc::c_int);
    }
    fprintf(stderr, b"done.\n\0" as *const u8 as *const libc::c_char);
    hostcount = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < names {
        ret = gai_error(*list.offset(i as isize));
        if ret != 0 {
            fprintf(
                stderr,
                b"Skipping %s: %s\n\0" as *const u8 as *const libc::c_char,
                (**list.offset(i as isize)).ar_name,
                gai_strerror(ret),
            );
        } else {
            let mut result: *mut addrinfo = (**list.offset(i as isize)).ar_result;
            loop {
                hostcount += 1;
                hostcount;
                result = (*result).ai_next;
                if result.is_null() {
                    break;
                }
            }
        }
        i += 1;
        i;
    }
    if hostcount == 0 {
        fprintf(
            stderr,
            b"No hosts found! Exiting\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    *hosts = host_create(list, names);
    host_free_resolvlist(list, names);
    if (*hosts).is_null() {
        fprintf(
            stderr,
            b"Failed creating list list, exiting\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return hostcount;
}
unsafe extern "C" fn print_usage(mut progname: *mut libc::c_char) {
    fprintf(
        stderr,
        b"Usage: %s [options] hostfile mountpoint\nOptions:\n -h           : Print this help and exit\n -u username  : Mount the filesystem as this user\n -t timeout   : Max time to wait for icmp reply (seconds, default 1)\n\0"
            as *const u8 as *const libc::c_char,
        progname,
    );
}
unsafe extern "C" fn pingfs_opt_proc(
    mut data: *mut libc::c_void,
    mut arg: *const libc::c_char,
    mut key: libc::c_int,
    mut outargs: *mut fuse_args,
) -> libc::c_int {
    let mut arginfo: *mut arginfo = data as *mut arginfo;
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut res: libc::c_int = 0;
    match key {
        -2 => {
            (*arginfo).num_args += 1;
            (*arginfo).num_args;
            if ((*arginfo).hostfile).is_null() {
                (*arginfo).hostfile = strdup(arg);
                return 0 as libc::c_int;
            } else if ((*arginfo).mountpoint).is_null() {
                (*arginfo).mountpoint = strdup(arg);
            }
        }
        0 => {
            print_usage(*((*outargs).argv).offset(0 as libc::c_int as isize));
            exit(0 as libc::c_int);
        }
        1 => {
            pw = getpwnam(&*arg.offset(2 as libc::c_int as isize));
            if !pw.is_null() {
                let mut userarg: [libc::c_char; 64] = [0; 64];
                snprintf(
                    userarg.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                    b"-ouid=%d,gid=%d\0" as *const u8 as *const libc::c_char,
                    (*pw).pw_uid,
                    (*pw).pw_gid,
                );
                fuse_opt_add_arg(outargs, userarg.as_mut_ptr());
                return 0 as libc::c_int;
            } else {
                fprintf(
                    stderr,
                    b"Bad username given! Exiting\n\0" as *const u8
                        as *const libc::c_char,
                );
                print_usage(*((*outargs).argv).offset(0 as libc::c_int as isize));
                exit(1 as libc::c_int);
            }
        }
        2 => {
            res = sscanf(
                arg,
                b"-t%d\0" as *const u8 as *const libc::c_char,
                &mut (*arginfo).timeout as *mut libc::c_int,
            );
            if res == 1 as libc::c_int && (*arginfo).timeout > 0 as libc::c_int
                && (*arginfo).timeout < 60 as libc::c_int
            {
                return 0 as libc::c_int
            } else {
                fprintf(
                    stderr,
                    b"Bad timeout given! Exiting\n\0" as *const u8 as *const libc::c_char,
                );
                print_usage(*((*outargs).argv).offset(0 as libc::c_int as isize));
                exit(1 as libc::c_int);
            }
        }
        _ => {}
    }
    return 1 as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut list: *mut *mut gaicb = 0 as *mut *mut gaicb;
    let mut hosts: *mut host = 0 as *mut host;
    let mut h: *mut host = 0 as *mut host;
    let mut hostnames: libc::c_int = 0;
    let mut host_count: libc::c_int = 0;
    let mut args: fuse_args = {
        let mut init = fuse_args {
            argc: argc,
            argv: argv,
            allocated: 0 as libc::c_int,
        };
        init
    };
    let mut arginfo: arginfo = arginfo {
        hostfile: 0 as *mut libc::c_char,
        mountpoint: 0 as *mut libc::c_char,
        num_args: 0,
        timeout: 0,
    };
    let mut mountdir: stat = stat {
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
    let mut res: libc::c_int = 0;
    memset(
        &mut arginfo as *mut arginfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<arginfo>() as libc::c_ulong,
    );
    arginfo.timeout = 1 as libc::c_int;
    if fuse_opt_parse(
        &mut args,
        &mut arginfo as *mut arginfo as *mut libc::c_void,
        pingfs_opts.as_ptr(),
        Some(
            pingfs_opt_proc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    libc::c_int,
                    *mut fuse_args,
                ) -> libc::c_int,
        ),
    ) == -(1 as libc::c_int)
    {
        fprintf(
            stderr,
            b"Error parsing options!\n\0" as *const u8 as *const libc::c_char,
        );
        print_usage(*argv.offset(0 as libc::c_int as isize));
        return 1 as libc::c_int;
    }
    if arginfo.num_args != 2 as libc::c_int {
        fprintf(stderr, b"Need two arguments!\n\0" as *const u8 as *const libc::c_char);
        print_usage(*argv.offset(0 as libc::c_int as isize));
        return 1 as libc::c_int;
    }
    res = stat(arginfo.mountpoint, &mut mountdir);
    if res != 0 {
        perror(b"Failed to check mountpoint\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    if !(mountdir.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint)
    {
        fprintf(
            stderr,
            b"Mountpoint must be a directory! Exiting\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    free(arginfo.mountpoint as *mut libc::c_void);
    hostnames = read_hostnames(arginfo.hostfile, &mut list);
    free(arginfo.hostfile as *mut libc::c_void);
    if hostnames == 0 {
        fprintf(
            stderr,
            b"No hosts configured! Exiting\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if net_open_sockets() != 0 {
        fprintf(
            stderr,
            b"No raw sockets opened. Got root?\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    host_count = resolve_names(list, hostnames, &mut hosts);
    if host_count < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    host_count = host_evaluate(&mut hosts, host_count, arginfo.timeout);
    if host_count == 0 {
        fprintf(
            stderr,
            b"No host passed the test\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    chunk_set_timeout(arginfo.timeout);
    host_use(hosts);
    fuse_opt_add_arg(&mut args, b"-f\0" as *const u8 as *const libc::c_char);
    fuse_opt_add_arg(&mut args, b"-s\0" as *const u8 as *const libc::c_char);
    fuse_opt_add_arg(
        &mut args,
        b"-odefault_permissions,allow_other\0" as *const u8 as *const libc::c_char,
    );
    fuse_opt_add_arg(&mut args, b"-odirect_io\0" as *const u8 as *const libc::c_char);
    printf(b"Mounting filesystem\n\0" as *const u8 as *const libc::c_char);
    fuse_main_real(
        args.argc,
        args.argv,
        &fs_ops,
        ::std::mem::size_of::<fuse_operations>() as libc::c_ulong,
        0 as *mut libc::c_void,
    );
    fuse_opt_free_args(&mut args);
    h = hosts;
    while !h.is_null() {
        let mut host: *mut host = h;
        h = (*h).next;
        free(host as *mut libc::c_void);
    }
    return 0 as libc::c_int;
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
unsafe extern "C" fn run_static_initializers() {
    pingfs_opts = [
        {
            let mut init = fuse_opt {
                templ: b"-h\0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: KEY_HELP as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"-u \0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: KEY_ASUSER as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: b"-t \0" as *const u8 as *const libc::c_char,
                offset: (1 as libc::c_uint).wrapping_neg() as libc::c_ulong,
                value: KEY_TIMEOUT as libc::c_int,
            };
            init
        },
        {
            let mut init = fuse_opt {
                templ: 0 as *const libc::c_char,
                offset: 0 as libc::c_int as libc::c_ulong,
                value: 0 as libc::c_int,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
