use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
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
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn atof(__nptr: *const libc::c_char) -> libc::c_double;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn atol(__nptr: *const libc::c_char) -> libc::c_long;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn getpid() -> __pid_t;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn gnu_get_libc_version() -> *const libc::c_char;
    fn mktime(__tp: *mut tm) -> time_t;
    fn strptime(
        __s: *const libc::c_char,
        __fmt: *const libc::c_char,
        __tp: *mut tm,
    ) -> *mut libc::c_char;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn __errno_location() -> *mut libc::c_int;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn sem_open(__name: *const libc::c_char, __oflag: libc::c_int, _: ...) -> *mut sem_t;
    fn sem_close(__sem: *mut sem_t) -> libc::c_int;
    fn sem_wait(__sem: *mut sem_t) -> libc::c_int;
    fn sem_post(__sem: *mut sem_t) -> libc::c_int;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn shm_open(
        __name: *const libc::c_char,
        __oflag: libc::c_int,
        __mode: mode_t,
    ) -> libc::c_int;
    fn pthread_rwlock_init(
        __rwlock: *mut pthread_rwlock_t,
        __attr: *const pthread_rwlockattr_t,
    ) -> libc::c_int;
    fn pthread_rwlock_rdlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
    fn pthread_rwlock_wrlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
    fn pthread_rwlock_unlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
    fn pthread_condattr_getclock(
        __attr: *const pthread_condattr_t,
        __clock_id: *mut __clockid_t,
    ) -> libc::c_int;
    fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn dlvsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
        __version: *const libc::c_char,
    ) -> *mut libc::c_void;
    static mut __progname: *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
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
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __timer_t = *mut libc::c_void;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __blkcnt64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
pub type mode_t = __mode_t;
pub type pid_t = __pid_t;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type timer_t = __timer_t;
pub type useconds_t = __useconds_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_time_s {
    pub real: timespec,
    pub mon: timespec,
    pub mon_raw: timespec,
    pub boot: timespec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sem_t {
    pub __size: [libc::c_char; 32],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ft_shared_s {
    pub ticks: uint64_t,
    pub file_idx: uint64_t,
    pub start_time: system_time_s,
}
pub type uint64_t = __uint64_t;
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
pub struct saved_timestamp {
    pub sec: int64_t,
    pub nsec: uint64_t,
}
pub const FT_START_AT: ft_mode_t = 1;
pub const FT_FREEZE: ft_mode_t = 0;
pub type ft_mode_t = libc::c_uint;
pub const FT_NOOP: ft_mode_t = 2;
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
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlockattr_t {
    pub __size: [libc::c_char; 8],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: libc::c_uint,
    pub __writers: libc::c_uint,
    pub __wrphase_futex: libc::c_uint,
    pub __writers_futex: libc::c_uint,
    pub __pad3: libc::c_uint,
    pub __pad4: libc::c_uint,
    pub __cur_writer: libc::c_int,
    pub __shared: libc::c_int,
    pub __rwelision: libc::c_schar,
    pub __pad1: [libc::c_uchar; 7],
    pub __pad2: libc::c_ulong,
    pub __flags: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeb {
    pub time: time_t,
    pub millitm: libc::c_ushort,
    pub timezone: libc::c_short,
    pub dstflag: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct epoll_event {
    pub events: uint32_t,
    pub data: epoll_data_t,
}
pub type epoll_data_t = epoll_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub union epoll_data {
    pub ptr: *mut libc::c_void,
    pub fd: libc::c_int,
    pub u32_0: uint32_t,
    pub u64_0: uint64_t,
}
pub type uint32_t = __uint32_t;
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utimbuf {
    pub actime: time_t,
    pub modtime: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat64 {
    pub st_dev: __dev_t,
    pub st_ino: __ino64_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt64_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const TFD_TIMER_CANCEL_ON_SET: C2RustUnnamed_3 = 2;
pub const TFD_TIMER_ABSTIME: C2RustUnnamed_3 = 1;
pub type ft_lib_compat_timer = libc::c_uint;
pub const FT_FD: ft_lib_compat_timer = 2;
pub const FT_COMPAT_GLIBC_2_3_3: ft_lib_compat_timer = 1;
pub const FT_COMPAT_GLIBC_2_2: ft_lib_compat_timer = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union timer_t_or_int {
    pub int_member: libc::c_int,
    pub timer_t_member: timer_t,
}
pub type ptrdiff_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_bucket {
    pub hh_head: *mut UT_hash_handle,
    pub count: libc::c_uint,
    pub expand_mult: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_handle {
    pub tbl: *mut UT_hash_table,
    pub prev: *mut libc::c_void,
    pub next: *mut libc::c_void,
    pub hh_prev: *mut UT_hash_handle,
    pub hh_next: *mut UT_hash_handle,
    pub key: *mut libc::c_void,
    pub keylen: libc::c_uint,
    pub hashv: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_table {
    pub buckets: *mut UT_hash_bucket,
    pub num_buckets: libc::c_uint,
    pub log2_num_buckets: libc::c_uint,
    pub num_items: libc::c_uint,
    pub tail: *mut UT_hash_handle,
    pub hho: ptrdiff_t,
    pub ideal_chain_maxlen: libc::c_uint,
    pub nonideal_items: libc::c_uint,
    pub ineff_expands: libc::c_uint,
    pub noexpand: libc::c_uint,
    pub signature: uint32_t,
}
pub type ft_lib_compat_pthread = libc::c_uint;
pub const FT_COMPAT_GLIBC_2_3_2: ft_lib_compat_pthread = 1;
pub const FT_COMPAT_GLIBC_2_2_5: ft_lib_compat_pthread = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_cond_monotonic {
    pub ptr: *mut pthread_cond_t,
    pub hh: UT_hash_handle,
}
#[inline]
unsafe extern "C" fn __bswap_64(mut __bsx: __uint64_t) -> __uint64_t {
    return ((__bsx as libc::c_ulonglong & 0xff00000000000000 as libc::c_ulonglong)
        >> 56 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff000000000000 as libc::c_ulonglong)
            >> 40 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff0000000000 as libc::c_ulonglong)
            >> 24 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff00000000 as libc::c_ulonglong)
            >> 8 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff000000 as libc::c_ulonglong)
            << 8 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff0000 as libc::c_ulonglong)
            << 24 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff00 as libc::c_ulonglong) << 40 as libc::c_int
        | (__bsx as libc::c_ulonglong & 0xff as libc::c_ulonglong) << 56 as libc::c_int)
        as __uint64_t;
}
#[thread_local]
static mut dont_fake: bool = 0 as libc::c_int != 0;
static mut real_stat: Option::<
    unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
> = None;
static mut real_fstat: Option::<
    unsafe extern "C" fn(libc::c_int, *mut stat) -> libc::c_int,
> = None;
static mut real_lstat: Option::<
    unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
> = None;
static mut real_xstat: Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_char, *mut stat) -> libc::c_int,
> = None;
static mut real_fxstat: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut stat) -> libc::c_int,
> = None;
static mut real_fxstatat: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        *const libc::c_char,
        *mut stat,
        libc::c_int,
    ) -> libc::c_int,
> = None;
static mut real_lxstat: Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_char, *mut stat) -> libc::c_int,
> = None;
static mut real_xstat64: Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_char, *mut stat64) -> libc::c_int,
> = None;
static mut real_fxstat64: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut stat64) -> libc::c_int,
> = None;
static mut real_fxstatat64: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        *const libc::c_char,
        *mut stat64,
        libc::c_int,
    ) -> libc::c_int,
> = None;
static mut real_lxstat64: Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_char, *mut stat64) -> libc::c_int,
> = None;
static mut real_time: Option::<unsafe extern "C" fn(*mut time_t) -> time_t> = None;
static mut real_ftime: Option::<unsafe extern "C" fn(*mut timeb) -> libc::c_int> = None;
static mut real_gettimeofday: Option::<
    unsafe extern "C" fn(*mut timeval, *mut libc::c_void) -> libc::c_int,
> = None;
static mut real_clock_gettime: Option::<
    unsafe extern "C" fn(clockid_t, *mut timespec) -> libc::c_int,
> = None;
static mut real_timespec_get: Option::<
    unsafe extern "C" fn(*mut timespec, libc::c_int) -> libc::c_int,
> = None;
static mut real___ftime: Option::<unsafe extern "C" fn(*mut timeb) -> libc::c_int> = None;
static mut real___gettimeofday: Option::<
    unsafe extern "C" fn(*mut timeval, *mut libc::c_void) -> libc::c_int,
> = None;
static mut real___clock_gettime: Option::<
    unsafe extern "C" fn(clockid_t, *mut timespec) -> libc::c_int,
> = None;
static mut real_pthread_cond_timedwait_225: Option::<
    unsafe extern "C" fn(
        *mut pthread_cond_t,
        *mut pthread_mutex_t,
        *mut timespec,
    ) -> libc::c_int,
> = None;
static mut real_pthread_cond_timedwait_232: Option::<
    unsafe extern "C" fn(
        *mut pthread_cond_t,
        *mut pthread_mutex_t,
        *mut timespec,
    ) -> libc::c_int,
> = None;
static mut real_pthread_cond_init_232: Option::<
    unsafe extern "C" fn(*mut pthread_cond_t, *const pthread_condattr_t) -> libc::c_int,
> = None;
static mut real_pthread_cond_destroy_232: Option::<
    unsafe extern "C" fn(*mut pthread_cond_t) -> libc::c_int,
> = None;
static mut monotonic_conds_lock: pthread_rwlock_t = pthread_rwlock_t {
    __data: __pthread_rwlock_arch_t {
        __readers: 0,
        __writers: 0,
        __wrphase_futex: 0,
        __writers_futex: 0,
        __pad3: 0,
        __pad4: 0,
        __cur_writer: 0,
        __shared: 0,
        __rwelision: 0,
        __pad1: [0; 7],
        __pad2: 0,
        __flags: 0,
    },
};
static mut real_timer_settime_22: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        *const itimerspec,
        *mut itimerspec,
    ) -> libc::c_int,
> = None;
static mut real_timer_settime_233: Option::<
    unsafe extern "C" fn(
        timer_t,
        libc::c_int,
        *const itimerspec,
        *mut itimerspec,
    ) -> libc::c_int,
> = None;
static mut real_timer_gettime_22: Option::<
    unsafe extern "C" fn(libc::c_int, *mut itimerspec) -> libc::c_int,
> = None;
static mut real_timer_gettime_233: Option::<
    unsafe extern "C" fn(timer_t, *mut itimerspec) -> libc::c_int,
> = None;
static mut real_timerfd_settime: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        *const itimerspec,
        *mut itimerspec,
    ) -> libc::c_int,
> = None;
static mut real_timerfd_gettime: Option::<
    unsafe extern "C" fn(libc::c_int, *mut itimerspec) -> libc::c_int,
> = None;
static mut real_nanosleep: Option::<
    unsafe extern "C" fn(*const timespec, *mut timespec) -> libc::c_int,
> = None;
static mut real_clock_nanosleep: Option::<
    unsafe extern "C" fn(
        clockid_t,
        libc::c_int,
        *const timespec,
        *mut timespec,
    ) -> libc::c_int,
> = None;
static mut real_usleep: Option::<unsafe extern "C" fn(useconds_t) -> libc::c_int> = None;
static mut real_sleep: Option::<unsafe extern "C" fn(libc::c_uint) -> libc::c_uint> = None;
static mut real_alarm: Option::<unsafe extern "C" fn(libc::c_uint) -> libc::c_uint> = None;
static mut real_poll: Option::<
    unsafe extern "C" fn(*mut pollfd, nfds_t, libc::c_int) -> libc::c_int,
> = None;
static mut real_ppoll: Option::<
    unsafe extern "C" fn(
        *mut pollfd,
        nfds_t,
        *const timespec,
        *const sigset_t,
    ) -> libc::c_int,
> = None;
static mut real_epoll_wait: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        *mut epoll_event,
        libc::c_int,
        libc::c_int,
    ) -> libc::c_int,
> = None;
static mut real_epoll_pwait: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        *mut epoll_event,
        libc::c_int,
        libc::c_int,
        *const sigset_t,
    ) -> libc::c_int,
> = None;
static mut real_select: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        *mut fd_set,
        *mut fd_set,
        *mut fd_set,
        *mut timeval,
    ) -> libc::c_int,
> = None;
static mut real_pselect: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        *mut fd_set,
        *mut fd_set,
        *mut fd_set,
        *const timespec,
        *const sigset_t,
    ) -> libc::c_int,
> = None;
static mut real_sem_timedwait: Option::<
    unsafe extern "C" fn(*mut sem_t, *const timespec) -> libc::c_int,
> = None;
static mut real_sem_clockwait: Option::<
    unsafe extern "C" fn(*mut sem_t, clockid_t, *const timespec) -> libc::c_int,
> = None;
static mut real_utimes: Option::<
    unsafe extern "C" fn(*const libc::c_char, *const timeval) -> libc::c_int,
> = None;
static mut real_utime: Option::<
    unsafe extern "C" fn(*const libc::c_char, *const utimbuf) -> libc::c_int,
> = None;
static mut real_utimensat: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        *const libc::c_char,
        *const timespec,
        libc::c_int,
    ) -> libc::c_int,
> = None;
static mut real_futimens: Option::<
    unsafe extern "C" fn(libc::c_int, *const timespec) -> libc::c_int,
> = None;
unsafe extern "C" fn check_missing_real(
    mut name: *const libc::c_char,
    mut missing: bool,
) -> bool {
    if missing {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
static mut initialized: libc::c_int = 0 as libc::c_int;
static mut shared_sem: *mut sem_t = 0 as *const sem_t as *mut sem_t;
static mut ft_shared: *mut ft_shared_s = 0 as *const ft_shared_s as *mut ft_shared_s;
#[inline]
unsafe extern "C" fn timespec_from_saved(
    mut tp: *mut timespec,
    mut saved: *mut saved_timestamp,
) {
    (*tp).tv_sec = __bswap_64((*saved).sec as __uint64_t) as __time_t;
    (*tp).tv_nsec = __bswap_64((*saved).nsec) as __syscall_slong_t;
}
static mut stss: *mut saved_timestamp = 0 as *const saved_timestamp
    as *mut saved_timestamp;
static mut infile_size: size_t = 0;
static mut infile_set: bool = 0 as libc::c_int != 0;
static mut outfile: libc::c_int = -(1 as libc::c_int);
static mut limited_faking: bool = 0 as libc::c_int != 0;
static mut callcounter: libc::c_long = 0 as libc::c_int as libc::c_long;
static mut ft_start_after_secs: libc::c_long = -(1 as libc::c_int) as libc::c_long;
static mut ft_stop_after_secs: libc::c_long = -(1 as libc::c_int) as libc::c_long;
static mut ft_start_after_ncalls: libc::c_long = -(1 as libc::c_int) as libc::c_long;
static mut ft_stop_after_ncalls: libc::c_long = -(1 as libc::c_int) as libc::c_long;
static mut spawnsupport: bool = 0 as libc::c_int != 0;
static mut spawned: libc::c_int = 0 as libc::c_int;
static mut ft_spawn_target: [libc::c_char; 1024] = [0; 1024];
static mut ft_spawn_secs: libc::c_long = -(1 as libc::c_int) as libc::c_long;
static mut ft_spawn_ncalls: libc::c_long = -(1 as libc::c_int) as libc::c_long;
static mut fake_monotonic_clock: libc::c_int = 1 as libc::c_int;
static mut cache_enabled: libc::c_int = 1 as libc::c_int;
static mut cache_duration: libc::c_int = 10 as libc::c_int;
static mut force_cache_expiration: libc::c_int = 0 as libc::c_int;
static mut ftpl_starttime: system_time_s = {
    let mut init = system_time_s {
        real: {
            let mut init = timespec {
                tv_sec: 0 as libc::c_int as __time_t,
                tv_nsec: -(1 as libc::c_int) as __syscall_slong_t,
            };
            init
        },
        mon: {
            let mut init = timespec {
                tv_sec: 0 as libc::c_int as __time_t,
                tv_nsec: -(1 as libc::c_int) as __syscall_slong_t,
            };
            init
        },
        mon_raw: {
            let mut init = timespec {
                tv_sec: 0 as libc::c_int as __time_t,
                tv_nsec: -(1 as libc::c_int) as __syscall_slong_t,
            };
            init
        },
        boot: {
            let mut init = timespec {
                tv_sec: 0 as libc::c_int as __time_t,
                tv_nsec: -(1 as libc::c_int) as __syscall_slong_t,
            };
            init
        },
    };
    init
};
static mut ftpl_timecache: system_time_s = {
    let mut init = system_time_s {
        real: {
            let mut init = timespec {
                tv_sec: 0 as libc::c_int as __time_t,
                tv_nsec: -(1 as libc::c_int) as __syscall_slong_t,
            };
            init
        },
        mon: {
            let mut init = timespec {
                tv_sec: 0 as libc::c_int as __time_t,
                tv_nsec: -(1 as libc::c_int) as __syscall_slong_t,
            };
            init
        },
        mon_raw: {
            let mut init = timespec {
                tv_sec: 0 as libc::c_int as __time_t,
                tv_nsec: -(1 as libc::c_int) as __syscall_slong_t,
            };
            init
        },
        boot: {
            let mut init = timespec {
                tv_sec: 0 as libc::c_int as __time_t,
                tv_nsec: -(1 as libc::c_int) as __syscall_slong_t,
            };
            init
        },
    };
    init
};
static mut ftpl_faketimecache: system_time_s = {
    let mut init = system_time_s {
        real: {
            let mut init = timespec {
                tv_sec: 0 as libc::c_int as __time_t,
                tv_nsec: -(1 as libc::c_int) as __syscall_slong_t,
            };
            init
        },
        mon: {
            let mut init = timespec {
                tv_sec: 0 as libc::c_int as __time_t,
                tv_nsec: -(1 as libc::c_int) as __syscall_slong_t,
            };
            init
        },
        mon_raw: {
            let mut init = timespec {
                tv_sec: 0 as libc::c_int as __time_t,
                tv_nsec: -(1 as libc::c_int) as __syscall_slong_t,
            };
            init
        },
        boot: {
            let mut init = timespec {
                tv_sec: 0 as libc::c_int as __time_t,
                tv_nsec: -(1 as libc::c_int) as __syscall_slong_t,
            };
            init
        },
    };
    init
};
static mut user_faked_time_fmt: [libc::c_char; 8192] = [
    0 as libc::c_int as libc::c_char,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
static mut user_faked_time_timespec: timespec = {
    let mut init = timespec {
        tv_sec: 0 as libc::c_int as __time_t,
        tv_nsec: -(1 as libc::c_int) as __syscall_slong_t,
    };
    init
};
static mut user_faked_time_set: bool = 0 as libc::c_int != 0;
static mut user_faked_time_saved: [libc::c_char; 256] = [
    0 as libc::c_int as libc::c_char,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
static mut user_offset: timespec = {
    let mut init = timespec {
        tv_sec: 0 as libc::c_int as __time_t,
        tv_nsec: -(1 as libc::c_int) as __syscall_slong_t,
    };
    init
};
static mut user_rate: libc::c_double = 1.0f64;
static mut user_rate_set: bool = 0 as libc::c_int != 0;
static mut user_per_tick_inc: timespec = {
    let mut init = timespec {
        tv_sec: 0 as libc::c_int as __time_t,
        tv_nsec: -(1 as libc::c_int) as __syscall_slong_t,
    };
    init
};
static mut user_per_tick_inc_set: bool = 0 as libc::c_int != 0;
pub static mut ft_mode: ft_mode_t = FT_FREEZE;
static mut parse_config_file: bool = 1 as libc::c_int != 0;
static mut shmCreator: bool = 0 as libc::c_int != 0;
unsafe extern "C" fn ft_shm_create() {
    let mut sem_name: [libc::c_char; 256] = [0; 256];
    let mut shm_name: [libc::c_char; 256] = [0; 256];
    let mut sem_nameT: [libc::c_char; 256] = [0; 256];
    let mut shm_nameT: [libc::c_char; 256] = [0; 256];
    let mut shm_fdN: libc::c_int = 0;
    let mut semN: *mut sem_t = 0 as *mut sem_t;
    let mut ft_sharedN: *mut ft_shared_s = 0 as *mut ft_shared_s;
    let mut shared_objsN: [libc::c_char; 513] = [0; 513];
    let mut shared_semT: *mut sem_t = 0 as *mut sem_t;
    let mut pid: pid_t = 0;
    pid = getpid();
    snprintf(
        sem_name.as_mut_ptr(),
        255 as libc::c_int as libc::c_ulong,
        b"/faketime_sem_%ld\0" as *const u8 as *const libc::c_char,
        pid as libc::c_long,
    );
    snprintf(
        shm_name.as_mut_ptr(),
        255 as libc::c_int as libc::c_ulong,
        b"/faketime_shm_%ld\0" as *const u8 as *const libc::c_char,
        pid as libc::c_long,
    );
    semN = sem_open(
        sem_name.as_mut_ptr(),
        0o100 as libc::c_int | 0o200 as libc::c_int,
        0o200 as libc::c_int | 0o400 as libc::c_int,
        1 as libc::c_int,
    );
    if semN.is_null() {
        return;
    }
    shm_fdN = shm_open(
        shm_name.as_mut_ptr(),
        0o100 as libc::c_int | 0o200 as libc::c_int | 0o2 as libc::c_int,
        (0o200 as libc::c_int | 0o400 as libc::c_int) as mode_t,
    );
    if -(1 as libc::c_int) == shm_fdN {
        perror(
            b"libfaketime: In ft_shm_create(), shm_open failed\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if -(1 as libc::c_int)
        == ftruncate(
            shm_fdN,
            ::std::mem::size_of::<uint64_t>() as libc::c_ulong as __off_t,
        )
    {
        perror(
            b"libfaketime: In ft_shm_create(), ftruncate failed\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    ft_sharedN = mmap(
        0 as *mut libc::c_void,
        ::std::mem::size_of::<ft_shared_s>() as libc::c_ulong,
        0x1 as libc::c_int | 0x2 as libc::c_int,
        0x1 as libc::c_int,
        shm_fdN,
        0 as libc::c_int as __off_t,
    ) as *mut ft_shared_s;
    if -(1 as libc::c_int) as *mut libc::c_void == ft_sharedN as *mut libc::c_void {
        perror(
            b"libfaketime: In ft_shm_create(), mmap failed\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if sem_wait(semN) == -(1 as libc::c_int) {
        perror(
            b"libfaketime: In ft_shm_create(), sem_wait failed\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    (*ft_sharedN).ticks = 0 as libc::c_int as uint64_t;
    (*ft_sharedN).file_idx = 0 as libc::c_int as uint64_t;
    (*ft_sharedN).start_time.real.tv_sec = 0 as libc::c_int as __time_t;
    (*ft_sharedN).start_time.real.tv_nsec = -(1 as libc::c_int) as __syscall_slong_t;
    (*ft_sharedN).start_time.mon.tv_sec = 0 as libc::c_int as __time_t;
    (*ft_sharedN).start_time.mon.tv_nsec = -(1 as libc::c_int) as __syscall_slong_t;
    (*ft_sharedN).start_time.mon_raw.tv_sec = 0 as libc::c_int as __time_t;
    (*ft_sharedN).start_time.mon_raw.tv_nsec = -(1 as libc::c_int) as __syscall_slong_t;
    if -(1 as libc::c_int)
        == munmap(
            ft_sharedN as *mut libc::c_void,
            ::std::mem::size_of::<ft_shared_s>() as libc::c_ulong,
        )
    {
        perror(
            b"libfaketime: In ft_shm_create(), munmap failed\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if sem_post(semN) == -(1 as libc::c_int) {
        perror(
            b"libfaketime: In ft_shm_create(), sem_post failed\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    snprintf(
        shared_objsN.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 513]>() as libc::c_ulong,
        b"%s %s\0" as *const u8 as *const libc::c_char,
        sem_name.as_mut_ptr(),
        shm_name.as_mut_ptr(),
    );
    let mut semSafetyCheckPassed: libc::c_int = 0 as libc::c_int;
    sem_close(semN);
    sscanf(
        shared_objsN.as_mut_ptr(),
        b"%255s %255s\0" as *const u8 as *const libc::c_char,
        sem_nameT.as_mut_ptr(),
        shm_nameT.as_mut_ptr(),
    );
    shared_semT = sem_open(sem_nameT.as_mut_ptr(), 0 as libc::c_int);
    if shared_semT.is_null() {
        fprintf(
            stderr,
            b"libfaketime: In ft_shm_create(), non-fatal sem_open issue with %s\0"
                as *const u8 as *const libc::c_char,
            sem_nameT.as_mut_ptr(),
        );
    } else {
        semSafetyCheckPassed = 1 as libc::c_int;
        sem_close(shared_semT);
    }
    if semSafetyCheckPassed == 1 as libc::c_int {
        setenv(
            b"FAKETIME_SHARED\0" as *const u8 as *const libc::c_char,
            shared_objsN.as_mut_ptr(),
            1 as libc::c_int,
        );
        shmCreator = 1 as libc::c_int != 0;
    }
}
unsafe extern "C" fn ft_shm_init() {
    let mut ticks_shm_fd: libc::c_int = 0;
    let mut sem_name: [libc::c_char; 256] = [0; 256];
    let mut shm_name: [libc::c_char; 256] = [0; 256];
    let mut ft_shared_env: *mut libc::c_char = getenv(
        b"FAKETIME_SHARED\0" as *const u8 as *const libc::c_char,
    );
    let mut shared_semR: *mut sem_t = 0 as *mut sem_t;
    static mut nt: libc::c_int = 1 as libc::c_int;
    if ft_shared_env.is_null() {
        ft_shm_create();
        ft_shared_env = getenv(b"FAKETIME_SHARED\0" as *const u8 as *const libc::c_char);
    }
    if !ft_shared_env.is_null() {
        if sscanf(
            ft_shared_env,
            b"%255s %255s\0" as *const u8 as *const libc::c_char,
            sem_name.as_mut_ptr(),
            shm_name.as_mut_ptr(),
        ) < 2 as libc::c_int
        {
            printf(
                b"libfaketime: In ft_shm_init(), error parsing semaphore name and shared memory id from string: %s\0"
                    as *const u8 as *const libc::c_char,
                ft_shared_env,
            );
            exit(1 as libc::c_int);
        }
        shared_semR = sem_open(sem_name.as_mut_ptr(), 0 as libc::c_int);
        if shared_semR.is_null() {
            ft_shm_create();
            ft_shared_env = getenv(
                b"FAKETIME_SHARED\0" as *const u8 as *const libc::c_char,
            );
        } else {
            sem_close(shared_semR);
        }
    }
    if !ft_shared_env.is_null() {
        if sscanf(
            ft_shared_env,
            b"%255s %255s\0" as *const u8 as *const libc::c_char,
            sem_name.as_mut_ptr(),
            shm_name.as_mut_ptr(),
        ) < 2 as libc::c_int
        {
            printf(
                b"libfaketime: In ft_shm_init(), error parsing semaphore name and shared memory id from string: %s\0"
                    as *const u8 as *const libc::c_char,
                ft_shared_env,
            );
            exit(1 as libc::c_int);
        }
        shared_sem = sem_open(sem_name.as_mut_ptr(), 0 as libc::c_int);
        if shared_sem.is_null() {
            if shmCreator {
                perror(
                    b"libfaketime: In ft_shm_init(), sem_open failed\0" as *const u8
                        as *const libc::c_char,
                );
                fprintf(
                    stderr,
                    b"libfaketime: sem_name was %s, created locally: %s\n\0" as *const u8
                        as *const libc::c_char,
                    sem_name.as_mut_ptr(),
                    if shmCreator as libc::c_int != 0 {
                        b"true\0" as *const u8 as *const libc::c_char
                    } else {
                        b"false\0" as *const u8 as *const libc::c_char
                    },
                );
                fprintf(
                    stderr,
                    b"libfaketime: parsed from env: %s\n\0" as *const u8
                        as *const libc::c_char,
                    ft_shared_env,
                );
                exit(1 as libc::c_int);
            } else {
                nt += 1;
                nt;
                if nt > 3 as libc::c_int {
                    perror(
                        b"libfaketime: In ft_shm_init(), sem_open failed and recreation attempts failed\0"
                            as *const u8 as *const libc::c_char,
                    );
                    fprintf(
                        stderr,
                        b"libfaketime: sem_name was %s, created locally: %s\n\0"
                            as *const u8 as *const libc::c_char,
                        sem_name.as_mut_ptr(),
                        if shmCreator as libc::c_int != 0 {
                            b"true\0" as *const u8 as *const libc::c_char
                        } else {
                            b"false\0" as *const u8 as *const libc::c_char
                        },
                    );
                    exit(1 as libc::c_int);
                } else {
                    ft_shm_init();
                    return;
                }
            }
        }
        ticks_shm_fd = shm_open(
            shm_name.as_mut_ptr(),
            0o100 as libc::c_int | 0o2 as libc::c_int,
            (0o200 as libc::c_int | 0o400 as libc::c_int) as mode_t,
        );
        if -(1 as libc::c_int) == ticks_shm_fd {
            perror(
                b"libfaketime: In ft_shm_init(), shm_open failed\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        ft_shared = mmap(
            0 as *mut libc::c_void,
            ::std::mem::size_of::<ft_shared_s>() as libc::c_ulong,
            0x1 as libc::c_int | 0x2 as libc::c_int,
            0x1 as libc::c_int,
            ticks_shm_fd,
            0 as libc::c_int as __off_t,
        ) as *mut ft_shared_s;
        if -(1 as libc::c_int) as *mut libc::c_void == ft_shared as *mut libc::c_void {
            perror(
                b"libfaketime: In ft_shm_init(), mmap failed\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
}
unsafe extern "C" fn get_fake_monotonic_setting(mut current_value: *mut libc::c_int) {
    let mut tmp_env: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp_env = getenv(
        b"FAKETIME_DONT_FAKE_MONOTONIC\0" as *const u8 as *const libc::c_char,
    );
    if !tmp_env.is_null()
        || {
            tmp_env = getenv(
                b"DONT_FAKE_MONOTONIC\0" as *const u8 as *const libc::c_char,
            );
            !tmp_env.is_null()
        }
    {
        if 0 as libc::c_int
            == strcmp(tmp_env, b"1\0" as *const u8 as *const libc::c_char)
        {
            *current_value = 0 as libc::c_int;
        } else {
            *current_value = 1 as libc::c_int;
        }
    }
}
unsafe extern "C" fn system_time_from_system(mut systime: *mut system_time_s) {
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    (Some(real_clock_gettime.unwrap())).unwrap()(0 as libc::c_int, &mut (*systime).real);
    dont_fake = dont_fake_orig;
    let mut dont_fake_orig_0: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    (Some(real_clock_gettime.unwrap())).unwrap()(1 as libc::c_int, &mut (*systime).mon);
    dont_fake = dont_fake_orig_0;
    let mut dont_fake_orig_1: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    (Some(real_clock_gettime.unwrap()))
        .unwrap()(4 as libc::c_int, &mut (*systime).mon_raw);
    dont_fake = dont_fake_orig_1;
    let mut dont_fake_orig_2: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    (Some(real_clock_gettime.unwrap())).unwrap()(7 as libc::c_int, &mut (*systime).boot);
    dont_fake = dont_fake_orig_2;
}
unsafe extern "C" fn next_time(mut tp: *mut timespec, mut ticklen: *mut timespec) {
    if !shared_sem.is_null() {
        let mut inc: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        if sem_wait(shared_sem) == -(1 as libc::c_int) {
            if *__errno_location() == 4 as libc::c_int {
                next_time(tp, ticklen);
                return;
            } else {
                perror(
                    b"libfaketime: In next_time(), sem_wait failed\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        }
        let mut tmp_time: int64_t = 0;
        tmp_time = ((*ft_shared).ticks)
            .wrapping_mul(
                ((*ticklen).tv_sec * 1000000000 as libc::c_int as libc::c_long
                    + (*ticklen).tv_nsec) as libc::c_ulong,
            ) as int64_t;
        inc.tv_nsec = tmp_time % 1000000000 as libc::c_int as libc::c_long;
        inc
            .tv_sec = (tmp_time - inc.tv_nsec)
            / 1000000000 as libc::c_int as libc::c_long;
        if inc.tv_nsec < 0 as libc::c_int as libc::c_long {
            inc.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
            inc.tv_sec -= 1 as libc::c_int as libc::c_long;
        }
        (*tp).tv_sec = user_faked_time_timespec.tv_sec + inc.tv_sec;
        (*tp).tv_nsec = user_faked_time_timespec.tv_nsec + inc.tv_nsec;
        if (*tp).tv_nsec >= 1000000000 as libc::c_int as libc::c_long {
            (*tp).tv_sec += 1;
            (*tp).tv_sec;
            (*tp).tv_nsec -= 1000000000 as libc::c_int as libc::c_long;
        }
        (*ft_shared).ticks = ((*ft_shared).ticks).wrapping_add(1);
        (*ft_shared).ticks;
        if sem_post(shared_sem) == -(1 as libc::c_int) {
            perror(
                b"libfaketime: In next_time(), sem_post failed\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
}
unsafe extern "C" fn reset_time() {
    system_time_from_system(&mut ftpl_starttime);
    if !shared_sem.is_null() {
        if sem_wait(shared_sem) == -(1 as libc::c_int) {
            perror(
                b"libfaketime: In reset_time(), sem_wait failed\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        (*ft_shared).start_time = ftpl_starttime;
        if sem_post(shared_sem) == -(1 as libc::c_int) {
            perror(
                b"libfaketime: In reset_time(), sem_post failed\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
}
unsafe extern "C" fn save_time(mut tp: *mut timespec) {
    if !shared_sem.is_null() && outfile != -(1 as libc::c_int) {
        let mut time_write: saved_timestamp = saved_timestamp { sec: 0, nsec: 0 };
        let mut written: ssize_t = 0;
        let mut n: size_t = 0 as libc::c_int as size_t;
        time_write.sec = __bswap_64((*tp).tv_sec as __uint64_t) as int64_t;
        time_write.nsec = __bswap_64((*tp).tv_nsec as __uint64_t);
        if sem_wait(shared_sem) == -(1 as libc::c_int) {
            if *__errno_location() == 4 as libc::c_int {
                save_time(tp);
                return;
            } else {
                perror(
                    b"libfaketime: In save_time(), sem_wait failed\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        }
        lseek(outfile, 0 as libc::c_int as __off_t, 2 as libc::c_int);
        loop {
            written = write(
                outfile,
                &mut *(&mut time_write as *mut saved_timestamp as *mut libc::c_char)
                    .offset(n as isize) as *mut libc::c_char as *const libc::c_void,
                (::std::mem::size_of::<saved_timestamp>() as libc::c_ulong)
                    .wrapping_sub(n),
            );
            if !(written == -(1 as libc::c_int) as libc::c_long
                && *__errno_location() == 4 as libc::c_int
                || {
                    n = (n as libc::c_ulong).wrapping_add(written as libc::c_ulong)
                        as size_t as size_t;
                    (::std::mem::size_of::<saved_timestamp>() as libc::c_ulong) < n
                })
            {
                break;
            }
        }
        if written == -(1 as libc::c_int) as libc::c_long
            || n < ::std::mem::size_of::<saved_timestamp>() as libc::c_ulong
        {
            perror(
                b"libfaketime: In save_time(), saving timestamp to file failed\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if sem_post(shared_sem) == -(1 as libc::c_int) {
            perror(
                b"libfaketime: In save_time(), sem_post failed\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
}
unsafe extern "C" fn load_time(mut tp: *mut timespec) -> bool {
    let mut ret: bool = 0 as libc::c_int != 0;
    if !shared_sem.is_null() && infile_set as libc::c_int != 0 {
        if sem_wait(shared_sem) == -(1 as libc::c_int) {
            if *__errno_location() == 4 as libc::c_int {
                return load_time(tp)
            } else {
                perror(
                    b"libfaketime: In load_time(), sem_wait failed\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        }
        if (::std::mem::size_of::<saved_timestamp>() as libc::c_ulong)
            .wrapping_mul(
                ((*ft_shared).file_idx).wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) > infile_size
        {
            timespec_from_saved(
                &mut user_faked_time_timespec,
                &mut *stss
                    .offset(
                        infile_size
                            .wrapping_div(
                                ::std::mem::size_of::<saved_timestamp>() as libc::c_ulong,
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ),
            );
            if (*ft_shared).ticks == 0 as libc::c_int as libc::c_ulong {
                (*ft_shared).ticks = 1 as libc::c_int as uint64_t;
                system_time_from_system(&mut ftpl_starttime);
                (*ft_shared).start_time = ftpl_starttime;
            } else {
                ftpl_starttime = (*ft_shared).start_time;
            }
            munmap(stss as *mut libc::c_void, infile_size);
            infile_set = 0 as libc::c_int != 0;
        } else {
            timespec_from_saved(tp, &mut *stss.offset((*ft_shared).file_idx as isize));
            (*ft_shared).file_idx = ((*ft_shared).file_idx).wrapping_add(1);
            (*ft_shared).file_idx;
            ret = 1 as libc::c_int != 0;
        }
        if sem_post(shared_sem) == -(1 as libc::c_int) {
            perror(
                b"libfaketime: In load_time(), sem_post failed\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    return ret;
}
static mut fake_utime_disabled: libc::c_int = 0 as libc::c_int;
static mut fake_stat_disabled: libc::c_int = 0 as libc::c_int;
static mut user_per_tick_inc_set_backup: bool = 0 as libc::c_int != 0;
#[no_mangle] pub unsafe extern "C" fn lock_for_stat() {
    if !shared_sem.is_null() {
        if sem_wait(shared_sem) == -(1 as libc::c_int) {
            if *__errno_location() == 4 as libc::c_int {
                lock_for_stat();
                return;
            } else {
                perror(
                    b"libfaketime: In lock_for_stat(), sem_wait failed\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        }
    }
    user_per_tick_inc_set_backup = user_per_tick_inc_set;
    user_per_tick_inc_set = 0 as libc::c_int != 0;
}
#[no_mangle] pub unsafe extern "C" fn unlock_for_stat() {
    user_per_tick_inc_set = user_per_tick_inc_set_backup;
    if !shared_sem.is_null() {
        if sem_post(shared_sem) == -(1 as libc::c_int) {
            perror(
                b"libfaketime: In unlock_for_stat(), sem_post failed\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
}
#[inline]
unsafe extern "C" fn fake_statbuf(mut buf: *mut stat) {
    lock_for_stat();
    fake_clock_gettime(0 as libc::c_int, &mut (*buf).st_ctim);
    fake_clock_gettime(0 as libc::c_int, &mut (*buf).st_atim);
    fake_clock_gettime(0 as libc::c_int, &mut (*buf).st_mtim);
    unlock_for_stat();
}
#[inline]
unsafe extern "C" fn fake_stat64buf(mut buf: *mut stat64) {
    lock_for_stat();
    fake_clock_gettime(0 as libc::c_int, &mut (*buf).st_ctim);
    fake_clock_gettime(0 as libc::c_int, &mut (*buf).st_atim);
    fake_clock_gettime(0 as libc::c_int, &mut (*buf).st_mtim);
    unlock_for_stat();
}
#[no_mangle] pub unsafe extern "C" fn stat(
    mut path: *const libc::c_char,
    mut buf: *mut stat,
) -> libc::c_int {
    if initialized == 0 {
        ftpl_init();
    }
    if !check_missing_real(
        b"stat\0" as *const u8 as *const libc::c_char,
        real_stat.is_none(),
    ) {
        return -(1 as libc::c_int);
    }
    let mut result: libc::c_int = 0;
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = real_stat.unwrap()(path, buf);
    dont_fake = dont_fake_orig;
    if result == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if !buf.is_null() {
        if fake_stat_disabled == 0 {
            if !dont_fake {
                fake_statbuf(buf);
            }
        }
    }
    return result;
}
#[no_mangle] pub unsafe extern "C" fn fstat(
    mut fildes: libc::c_int,
    mut buf: *mut stat,
) -> libc::c_int {
    if initialized == 0 {
        ftpl_init();
    }
    if !check_missing_real(
        b"fstat\0" as *const u8 as *const libc::c_char,
        real_fstat.is_none(),
    ) {
        return -(1 as libc::c_int);
    }
    let mut result: libc::c_int = 0;
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = real_fstat.unwrap()(fildes, buf);
    dont_fake = dont_fake_orig;
    if result == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if !buf.is_null() {
        if fake_stat_disabled == 0 {
            if !dont_fake {
                fake_statbuf(buf);
            }
        }
    }
    return result;
}
#[no_mangle] pub unsafe extern "C" fn lstat(
    mut path: *const libc::c_char,
    mut buf: *mut stat,
) -> libc::c_int {
    if initialized == 0 {
        ftpl_init();
    }
    if !check_missing_real(
        b"lstat\0" as *const u8 as *const libc::c_char,
        real_lstat.is_none(),
    ) {
        return -(1 as libc::c_int);
    }
    let mut result: libc::c_int = 0;
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = real_lstat.unwrap()(path, buf);
    dont_fake = dont_fake_orig;
    if result == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if !buf.is_null() {
        if fake_stat_disabled == 0 {
            if !dont_fake {
                fake_statbuf(buf);
            }
        }
    }
    return result;
}
#[no_mangle] pub unsafe extern "C" fn __xstat(
    mut ver: libc::c_int,
    mut path: *const libc::c_char,
    mut buf: *mut stat,
) -> libc::c_int {
    if initialized == 0 {
        ftpl_init();
    }
    if !check_missing_real(
        b"xstat\0" as *const u8 as *const libc::c_char,
        real_xstat.is_none(),
    ) {
        return -(1 as libc::c_int);
    }
    let mut result: libc::c_int = 0;
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = real_xstat.unwrap()(ver, path, buf);
    dont_fake = dont_fake_orig;
    if result == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if !buf.is_null() {
        if fake_stat_disabled == 0 {
            if !dont_fake {
                fake_statbuf(buf);
            }
        }
    }
    return result;
}
#[no_mangle] pub unsafe extern "C" fn __fxstat(
    mut ver: libc::c_int,
    mut fildes: libc::c_int,
    mut buf: *mut stat,
) -> libc::c_int {
    if initialized == 0 {
        ftpl_init();
    }
    if !check_missing_real(
        b"fxstat\0" as *const u8 as *const libc::c_char,
        real_fxstat.is_none(),
    ) {
        return -(1 as libc::c_int);
    }
    let mut result: libc::c_int = 0;
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = real_fxstat.unwrap()(ver, fildes, buf);
    dont_fake = dont_fake_orig;
    if result == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if !buf.is_null() {
        if fake_stat_disabled == 0 {
            if !dont_fake {
                fake_statbuf(buf);
            }
        }
    }
    return result;
}
#[no_mangle] pub unsafe extern "C" fn __fxstatat(
    mut ver: libc::c_int,
    mut fildes: libc::c_int,
    mut filename: *const libc::c_char,
    mut buf: *mut stat,
    mut flag: libc::c_int,
) -> libc::c_int {
    if initialized == 0 {
        ftpl_init();
    }
    if !check_missing_real(
        b"fxstatat\0" as *const u8 as *const libc::c_char,
        real_fxstatat.is_none(),
    ) {
        return -(1 as libc::c_int);
    }
    let mut result: libc::c_int = 0;
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = real_fxstatat.unwrap()(ver, fildes, filename, buf, flag);
    dont_fake = dont_fake_orig;
    if result == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if !buf.is_null() {
        if fake_stat_disabled == 0 {
            if !dont_fake {
                fake_statbuf(buf);
            }
        }
    }
    return result;
}
#[no_mangle] pub unsafe extern "C" fn __lxstat(
    mut ver: libc::c_int,
    mut path: *const libc::c_char,
    mut buf: *mut stat,
) -> libc::c_int {
    if initialized == 0 {
        ftpl_init();
    }
    if !check_missing_real(
        b"lxstat\0" as *const u8 as *const libc::c_char,
        real_lxstat.is_none(),
    ) {
        return -(1 as libc::c_int);
    }
    let mut result: libc::c_int = 0;
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = real_lxstat.unwrap()(ver, path, buf);
    dont_fake = dont_fake_orig;
    if result == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if !buf.is_null() {
        if fake_stat_disabled == 0 {
            if !dont_fake {
                fake_statbuf(buf);
            }
        }
    }
    return result;
}
#[no_mangle] pub unsafe extern "C" fn __xstat64(
    mut ver: libc::c_int,
    mut path: *const libc::c_char,
    mut buf: *mut stat64,
) -> libc::c_int {
    if initialized == 0 {
        ftpl_init();
    }
    if !check_missing_real(
        b"xstat64\0" as *const u8 as *const libc::c_char,
        real_xstat64.is_none(),
    ) {
        return -(1 as libc::c_int);
    }
    let mut result: libc::c_int = 0;
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = real_xstat64.unwrap()(ver, path, buf);
    dont_fake = dont_fake_orig;
    if result == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if !buf.is_null() {
        if fake_stat_disabled == 0 {
            if !dont_fake {
                fake_stat64buf(buf);
            }
        }
    }
    return result;
}
#[no_mangle] pub unsafe extern "C" fn __fxstat64(
    mut ver: libc::c_int,
    mut fildes: libc::c_int,
    mut buf: *mut stat64,
) -> libc::c_int {
    if initialized == 0 {
        ftpl_init();
    }
    if !check_missing_real(
        b"fxstat64\0" as *const u8 as *const libc::c_char,
        real_fxstat64.is_none(),
    ) {
        return -(1 as libc::c_int);
    }
    let mut result: libc::c_int = 0;
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = real_fxstat64.unwrap()(ver, fildes, buf);
    dont_fake = dont_fake_orig;
    if result == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if !buf.is_null() {
        if fake_stat_disabled == 0 {
            if !dont_fake {
                fake_stat64buf(buf);
            }
        }
    }
    return result;
}
#[no_mangle] pub unsafe extern "C" fn __fxstatat64(
    mut ver: libc::c_int,
    mut fildes: libc::c_int,
    mut filename: *const libc::c_char,
    mut buf: *mut stat64,
    mut flag: libc::c_int,
) -> libc::c_int {
    if initialized == 0 {
        ftpl_init();
    }
    if !check_missing_real(
        b"fxstatat64\0" as *const u8 as *const libc::c_char,
        real_fxstatat64.is_none(),
    ) {
        return -(1 as libc::c_int);
    }
    let mut result: libc::c_int = 0;
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = real_fxstatat64.unwrap()(ver, fildes, filename, buf, flag);
    dont_fake = dont_fake_orig;
    if result == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if !buf.is_null() {
        if fake_stat_disabled == 0 {
            if !dont_fake {
                fake_stat64buf(buf);
            }
        }
    }
    return result;
}
#[no_mangle] pub unsafe extern "C" fn __lxstat64(
    mut ver: libc::c_int,
    mut path: *const libc::c_char,
    mut buf: *mut stat64,
) -> libc::c_int {
    if initialized == 0 {
        ftpl_init();
    }
    if !check_missing_real(
        b"lxstat64\0" as *const u8 as *const libc::c_char,
        real_lxstat64.is_none(),
    ) {
        return -(1 as libc::c_int);
    }
    let mut result: libc::c_int = 0;
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = real_lxstat64.unwrap()(ver, path, buf);
    dont_fake = dont_fake_orig;
    if result == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if !buf.is_null() {
        if fake_stat_disabled == 0 {
            if !dont_fake {
                fake_stat64buf(buf);
            }
        }
    }
    return result;
}
#[no_mangle] pub unsafe extern "C" fn utime(
    mut filename: *const libc::c_char,
    mut times: *const utimbuf,
) -> libc::c_int {
    if initialized == 0 {
        ftpl_init();
    }
    if !check_missing_real(
        b"utime\0" as *const u8 as *const libc::c_char,
        real_utime.is_none(),
    ) {
        return -(1 as libc::c_int);
    }
    let mut result: libc::c_int = 0;
    let mut ntbuf: utimbuf = utimbuf { actime: 0, modtime: 0 };
    if fake_utime_disabled != 0 {
        if times.is_null() {
            ntbuf.modtime = time(0 as *mut time_t);
            ntbuf.actime = ntbuf.modtime;
            times = &mut ntbuf;
        }
    } else if !times.is_null() {
        ntbuf.actime = (*times).actime - user_offset.tv_sec;
        ntbuf.modtime = (*times).modtime - user_offset.tv_sec;
        times = &mut ntbuf;
    }
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = real_utime.unwrap()(filename, times);
    dont_fake = dont_fake_orig;
    return result;
}
#[no_mangle] pub unsafe extern "C" fn utimes(
    mut filename: *const libc::c_char,
    mut times: *const timeval,
) -> libc::c_int {
    if initialized == 0 {
        ftpl_init();
    }
    if !check_missing_real(
        b"utimes\0" as *const u8 as *const libc::c_char,
        real_utimes.is_none(),
    ) {
        return -(1 as libc::c_int);
    }
    let mut result: libc::c_int = 0;
    let mut tn: [timeval; 2] = [timeval { tv_sec: 0, tv_usec: 0 }; 2];
    if fake_utime_disabled != 0 {
        if times.is_null() {
            fake_gettimeofday(&mut *tn.as_mut_ptr().offset(0 as libc::c_int as isize));
            tn[1 as libc::c_int as usize] = tn[0 as libc::c_int as usize];
            times = tn.as_mut_ptr() as *const timeval;
        }
    } else if !times.is_null() {
        let mut user_offset2: timeval = timeval { tv_sec: 0, tv_usec: 0 };
        user_offset2.tv_sec = user_offset.tv_sec;
        user_offset2.tv_usec = user_offset.tv_nsec / 1000 as libc::c_int as libc::c_long;
        tn[0 as libc::c_int as usize]
            .tv_sec = (*times.offset(0 as libc::c_int as isize)).tv_sec
            - user_offset2.tv_sec;
        tn[0 as libc::c_int as usize]
            .tv_usec = (*times.offset(0 as libc::c_int as isize)).tv_usec
            - user_offset2.tv_usec;
        if tn[0 as libc::c_int as usize].tv_usec < 0 as libc::c_int as libc::c_long {
            tn[0 as libc::c_int as usize].tv_sec -= 1;
            tn[0 as libc::c_int as usize].tv_sec;
            tn[0 as libc::c_int as usize].tv_usec
                += 1000000 as libc::c_int as libc::c_long;
        }
        tn[1 as libc::c_int as usize]
            .tv_sec = (*times.offset(1 as libc::c_int as isize)).tv_sec
            - user_offset2.tv_sec;
        tn[1 as libc::c_int as usize]
            .tv_usec = (*times.offset(1 as libc::c_int as isize)).tv_usec
            - user_offset2.tv_usec;
        if tn[1 as libc::c_int as usize].tv_usec < 0 as libc::c_int as libc::c_long {
            tn[1 as libc::c_int as usize].tv_sec -= 1;
            tn[1 as libc::c_int as usize].tv_sec;
            tn[1 as libc::c_int as usize].tv_usec
                += 1000000 as libc::c_int as libc::c_long;
        }
        times = tn.as_mut_ptr() as *const timeval;
    }
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = real_utimes.unwrap()(filename, times);
    dont_fake = dont_fake_orig;
    return result;
}
unsafe extern "C" fn fake_two_timespec(
    mut in_times: *const timespec,
    mut out_times: *mut timespec,
) {
    if in_times.is_null() {
        let ref mut fresh0 = (*out_times.offset(1 as libc::c_int as isize)).tv_sec;
        *fresh0 = 0 as libc::c_int as __time_t;
        (*out_times.offset(0 as libc::c_int as isize)).tv_sec = *fresh0;
        let ref mut fresh1 = (*out_times.offset(1 as libc::c_int as isize)).tv_nsec;
        *fresh1 = ((1 as libc::c_long) << 30 as libc::c_int) - 1 as libc::c_long;
        (*out_times.offset(0 as libc::c_int as isize)).tv_nsec = *fresh1;
        in_times = out_times as *const timespec;
    }
    let mut now: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    now.tv_nsec = ((1 as libc::c_long) << 30 as libc::c_int) - 2 as libc::c_long;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j <= 1 as libc::c_int {
        if fake_utime_disabled != 0
            || (*in_times.offset(j as isize)).tv_nsec
                == ((1 as libc::c_long) << 30 as libc::c_int) - 2 as libc::c_long
            || (*in_times.offset(j as isize)).tv_nsec
                == ((1 as libc::c_long) << 30 as libc::c_int) - 1 as libc::c_long
        {
            if fake_utime_disabled != 0
                && (*in_times.offset(j as isize)).tv_nsec
                    == ((1 as libc::c_long) << 30 as libc::c_int) - 1 as libc::c_long
            {
                if now.tv_nsec
                    == ((1 as libc::c_long) << 30 as libc::c_int) - 2 as libc::c_long
                {
                    let mut dont_fake_orig: bool = dont_fake;
                    if !dont_fake {
                        dont_fake = 1 as libc::c_int != 0;
                    }
                    real_clock_gettime.unwrap()(0 as libc::c_int, &mut now);
                    dont_fake = dont_fake_orig;
                }
                (*out_times.offset(j as isize)).tv_sec = now.tv_sec + user_offset.tv_sec;
                (*out_times.offset(j as isize))
                    .tv_nsec = now.tv_nsec + user_offset.tv_nsec;
                if (*out_times.offset(j as isize)).tv_nsec
                    >= 1000000000 as libc::c_int as libc::c_long
                {
                    let ref mut fresh2 = (*out_times.offset(j as isize)).tv_sec;
                    *fresh2 += 1;
                    *fresh2;
                    let ref mut fresh3 = (*out_times.offset(j as isize)).tv_nsec;
                    *fresh3 -= 1000000000 as libc::c_int as libc::c_long;
                }
            } else if out_times != in_times as *mut timespec {
                *out_times.offset(j as isize) = *in_times.offset(j as isize);
            }
        } else {
            (*out_times.offset(j as isize))
                .tv_sec = (*in_times.offset(j as isize)).tv_sec - user_offset.tv_sec;
            (*out_times.offset(j as isize))
                .tv_nsec = (*in_times.offset(j as isize)).tv_nsec - user_offset.tv_nsec;
            if (*out_times.offset(j as isize)).tv_nsec < 0 as libc::c_int as libc::c_long
            {
                let ref mut fresh4 = (*out_times.offset(j as isize)).tv_sec;
                *fresh4 -= 1;
                *fresh4;
                let ref mut fresh5 = (*out_times.offset(j as isize)).tv_nsec;
                *fresh5 += 1000000000 as libc::c_int as libc::c_long;
            }
        }
        j += 1;
        j;
    }
}
#[no_mangle] pub unsafe extern "C" fn utimensat(
    mut dirfd: libc::c_int,
    mut filename: *const libc::c_char,
    mut times: *const timespec,
    mut flags: libc::c_int,
) -> libc::c_int {
    if initialized == 0 {
        ftpl_init();
    }
    if !check_missing_real(
        b"utimensat\0" as *const u8 as *const libc::c_char,
        real_utimensat.is_none(),
    ) {
        return -(1 as libc::c_int);
    }
    let mut result: libc::c_int = 0;
    let mut tn: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    fake_two_timespec(times, tn.as_mut_ptr());
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = real_utimensat
        .unwrap()(dirfd, filename, tn.as_mut_ptr() as *const timespec, flags);
    dont_fake = dont_fake_orig;
    return result;
}
#[no_mangle] pub unsafe extern "C" fn futimens(
    mut fd: libc::c_int,
    mut times: *const timespec,
) -> libc::c_int {
    if initialized == 0 {
        ftpl_init();
    }
    if !check_missing_real(
        b"futimens\0" as *const u8 as *const libc::c_char,
        real_futimens.is_none(),
    ) {
        return -(1 as libc::c_int);
    }
    let mut result: libc::c_int = 0;
    let mut tn: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    fake_two_timespec(times, tn.as_mut_ptr());
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = real_futimens.unwrap()(fd, tn.as_mut_ptr() as *const timespec);
    dont_fake = dont_fake_orig;
    return result;
}
#[no_mangle] pub unsafe extern "C" fn nanosleep(
    mut req: *const timespec,
    mut rem: *mut timespec,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut real_req: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    if initialized == 0 {
        ftpl_init();
    }
    if real_nanosleep.is_none() {
        return -(1 as libc::c_int);
    }
    if !req.is_null() {
        if user_rate_set as libc::c_int != 0 && !dont_fake {
            let mut tmp_time: int64_t = 0;
            tmp_time = (1.0f64 / user_rate
                * ((*req).tv_sec * 1000000000 as libc::c_int as libc::c_long
                    + (*req).tv_nsec) as libc::c_double) as int64_t;
            real_req.tv_nsec = tmp_time % 1000000000 as libc::c_int as libc::c_long;
            real_req
                .tv_sec = (tmp_time - real_req.tv_nsec)
                / 1000000000 as libc::c_int as libc::c_long;
            if real_req.tv_nsec < 0 as libc::c_int as libc::c_long {
                real_req.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                real_req.tv_sec -= 1 as libc::c_int as libc::c_long;
            }
        } else {
            real_req = *req;
        }
    } else {
        return -(1 as libc::c_int)
    }
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = (Some(real_nanosleep.unwrap())).unwrap()(&mut real_req, rem);
    dont_fake = dont_fake_orig;
    if result == -(1 as libc::c_int) {
        return result;
    }
    if !rem.is_null()
        && ((*rem).tv_sec != 0 as libc::c_int as libc::c_long
            || (*rem).tv_nsec != 0 as libc::c_int as libc::c_long)
    {
        if user_rate_set as libc::c_int != 0 && !dont_fake {
            let mut tmp_time_0: int64_t = 0;
            tmp_time_0 = (user_rate
                * ((*rem).tv_sec * 1000000000 as libc::c_int as libc::c_long
                    + (*rem).tv_nsec) as libc::c_double) as int64_t;
            (*rem).tv_nsec = tmp_time_0 % 1000000000 as libc::c_int as libc::c_long;
            (*rem)
                .tv_sec = (tmp_time_0 - (*rem).tv_nsec)
                / 1000000000 as libc::c_int as libc::c_long;
            if (*rem).tv_nsec < 0 as libc::c_int as libc::c_long {
                (*rem).tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                (*rem).tv_sec -= 1 as libc::c_int as libc::c_long;
            }
        }
    }
    return result;
}
#[no_mangle] pub unsafe extern "C" fn clock_nanosleep(
    mut clock_id: clockid_t,
    mut flags: libc::c_int,
    mut req: *const timespec,
    mut rem: *mut timespec,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut real_req: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    if initialized == 0 {
        ftpl_init();
    }
    if real_clock_nanosleep.is_none() {
        return -(1 as libc::c_int);
    }
    if !req.is_null() {
        if flags & 1 as libc::c_int != 0 {
            let mut tdiff: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
            let mut timeadj: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
            timeadj.tv_sec = (*req).tv_sec - user_faked_time_timespec.tv_sec;
            timeadj.tv_nsec = (*req).tv_nsec - user_faked_time_timespec.tv_nsec;
            if timeadj.tv_nsec < 0 as libc::c_int as libc::c_long {
                timeadj.tv_sec -= 1;
                timeadj.tv_sec;
                timeadj.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
            }
            if user_rate_set {
                let mut tmp_time: int64_t = 0;
                tmp_time = (1.0f64 / user_rate
                    * (timeadj.tv_sec * 1000000000 as libc::c_int as libc::c_long
                        + timeadj.tv_nsec) as libc::c_double) as int64_t;
                tdiff.tv_nsec = tmp_time % 1000000000 as libc::c_int as libc::c_long;
                tdiff
                    .tv_sec = (tmp_time - tdiff.tv_nsec)
                    / 1000000000 as libc::c_int as libc::c_long;
                if tdiff.tv_nsec < 0 as libc::c_int as libc::c_long {
                    tdiff.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                    tdiff.tv_sec -= 1 as libc::c_int as libc::c_long;
                }
            } else {
                tdiff = timeadj;
            }
            if clock_id == 0 as libc::c_int {
                real_req.tv_sec = ftpl_starttime.real.tv_sec + tdiff.tv_sec;
                real_req.tv_nsec = ftpl_starttime.real.tv_nsec + tdiff.tv_nsec;
                if real_req.tv_nsec >= 1000000000 as libc::c_int as libc::c_long {
                    real_req.tv_sec += 1;
                    real_req.tv_sec;
                    real_req.tv_nsec -= 1000000000 as libc::c_int as libc::c_long;
                }
            } else if clock_id == 1 as libc::c_int {
                real_req.tv_sec = ftpl_starttime.mon.tv_sec + tdiff.tv_sec;
                real_req.tv_nsec = ftpl_starttime.mon.tv_nsec + tdiff.tv_nsec;
                if real_req.tv_nsec >= 1000000000 as libc::c_int as libc::c_long {
                    real_req.tv_sec += 1;
                    real_req.tv_sec;
                    real_req.tv_nsec -= 1000000000 as libc::c_int as libc::c_long;
                }
            } else {
                real_req = *req;
            }
        } else if user_rate_set as libc::c_int != 0 && !dont_fake
            && (clock_id == 0 as libc::c_int || clock_id == 1 as libc::c_int)
        {
            let mut tmp_time_0: int64_t = 0;
            tmp_time_0 = (1.0f64 / user_rate
                * ((*req).tv_sec * 1000000000 as libc::c_int as libc::c_long
                    + (*req).tv_nsec) as libc::c_double) as int64_t;
            real_req.tv_nsec = tmp_time_0 % 1000000000 as libc::c_int as libc::c_long;
            real_req
                .tv_sec = (tmp_time_0 - real_req.tv_nsec)
                / 1000000000 as libc::c_int as libc::c_long;
            if real_req.tv_nsec < 0 as libc::c_int as libc::c_long {
                real_req.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                real_req.tv_sec -= 1 as libc::c_int as libc::c_long;
            }
        } else {
            real_req = *req;
        }
    } else {
        return -(1 as libc::c_int)
    }
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = (Some(real_clock_nanosleep.unwrap()))
        .unwrap()(clock_id, flags, &mut real_req, rem);
    dont_fake = dont_fake_orig;
    if result == -(1 as libc::c_int) {
        return result;
    }
    if !rem.is_null()
        && ((*rem).tv_sec != 0 as libc::c_int as libc::c_long
            || (*rem).tv_nsec != 0 as libc::c_int as libc::c_long)
    {
        if user_rate_set as libc::c_int != 0 && !dont_fake {
            let mut tmp_time_1: int64_t = 0;
            tmp_time_1 = (user_rate
                * ((*rem).tv_sec * 1000000000 as libc::c_int as libc::c_long
                    + (*rem).tv_nsec) as libc::c_double) as int64_t;
            (*rem).tv_nsec = tmp_time_1 % 1000000000 as libc::c_int as libc::c_long;
            (*rem)
                .tv_sec = (tmp_time_1 - (*rem).tv_nsec)
                / 1000000000 as libc::c_int as libc::c_long;
            if (*rem).tv_nsec < 0 as libc::c_int as libc::c_long {
                (*rem).tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                (*rem).tv_sec -= 1 as libc::c_int as libc::c_long;
            }
        }
    }
    return result;
}
#[no_mangle] pub unsafe extern "C" fn usleep(mut usec: useconds_t) -> libc::c_int {
    let mut result: libc::c_int = 0;
    if initialized == 0 {
        ftpl_init();
    }
    if user_rate_set as libc::c_int != 0 && !dont_fake {
        let mut real_req: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        if real_nanosleep.is_none() {
            if real_usleep.is_none() {
                return -(1 as libc::c_int);
            }
            let mut dont_fake_orig: bool = dont_fake;
            if !dont_fake {
                dont_fake = 1 as libc::c_int != 0;
            }
            result = (Some(real_usleep.unwrap()))
                .unwrap()((1.0f64 / user_rate * usec as libc::c_double) as useconds_t);
            dont_fake = dont_fake_orig;
            return result;
        }
        real_req
            .tv_sec = usec.wrapping_div(1000000 as libc::c_int as libc::c_uint)
            as __time_t;
        real_req
            .tv_nsec = usec
            .wrapping_rem(1000000 as libc::c_int as libc::c_uint)
            .wrapping_mul(1000 as libc::c_int as libc::c_uint) as __syscall_slong_t;
        let mut tmp_time: int64_t = 0;
        tmp_time = (1.0f64 / user_rate
            * (real_req.tv_sec * 1000000000 as libc::c_int as libc::c_long
                + real_req.tv_nsec) as libc::c_double) as int64_t;
        real_req.tv_nsec = tmp_time % 1000000000 as libc::c_int as libc::c_long;
        real_req
            .tv_sec = (tmp_time - real_req.tv_nsec)
            / 1000000000 as libc::c_int as libc::c_long;
        if real_req.tv_nsec < 0 as libc::c_int as libc::c_long {
            real_req.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
            real_req.tv_sec -= 1 as libc::c_int as libc::c_long;
        }
        let mut dont_fake_orig_0: bool = dont_fake;
        if !dont_fake {
            dont_fake = 1 as libc::c_int != 0;
        }
        result = (Some(real_nanosleep.unwrap()))
            .unwrap()(&mut real_req, 0 as *mut timespec);
        dont_fake = dont_fake_orig_0;
    } else {
        let mut dont_fake_orig_1: bool = dont_fake;
        if !dont_fake {
            dont_fake = 1 as libc::c_int != 0;
        }
        result = (Some(real_usleep.unwrap())).unwrap()(usec);
        dont_fake = dont_fake_orig_1;
    }
    return result;
}
#[no_mangle] pub unsafe extern "C" fn sleep(mut seconds: libc::c_uint) -> libc::c_uint {
    if initialized == 0 {
        ftpl_init();
    }
    if user_rate_set as libc::c_int != 0 && !dont_fake {
        if real_nanosleep.is_none() {
            let mut ret: libc::c_uint = 0;
            if real_sleep.is_none() {
                return 0 as libc::c_int as libc::c_uint;
            }
            let mut dont_fake_orig: bool = dont_fake;
            if !dont_fake {
                dont_fake = 1 as libc::c_int != 0;
            }
            ret = (Some(real_sleep.unwrap()))
                .unwrap()(
                (1.0f64 / user_rate * seconds as libc::c_double) as libc::c_uint,
            );
            dont_fake = dont_fake_orig;
            return (if user_rate_set as libc::c_int != 0 && !dont_fake {
                user_rate * ret as libc::c_double
            } else {
                ret as libc::c_double
            }) as libc::c_uint;
        } else {
            let mut result: libc::c_int = 0;
            let mut real_req: timespec = {
                let mut init = timespec {
                    tv_sec: seconds as __time_t,
                    tv_nsec: 0 as libc::c_int as __syscall_slong_t,
                };
                init
            };
            let mut rem: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
            let mut tmp_time: int64_t = 0;
            tmp_time = (1.0f64 / user_rate
                * (real_req.tv_sec * 1000000000 as libc::c_int as libc::c_long
                    + real_req.tv_nsec) as libc::c_double) as int64_t;
            real_req.tv_nsec = tmp_time % 1000000000 as libc::c_int as libc::c_long;
            real_req
                .tv_sec = (tmp_time - real_req.tv_nsec)
                / 1000000000 as libc::c_int as libc::c_long;
            if real_req.tv_nsec < 0 as libc::c_int as libc::c_long {
                real_req.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                real_req.tv_sec -= 1 as libc::c_int as libc::c_long;
            }
            let mut dont_fake_orig_0: bool = dont_fake;
            if !dont_fake {
                dont_fake = 1 as libc::c_int != 0;
            }
            result = (Some(real_nanosleep.unwrap())).unwrap()(&mut real_req, &mut rem);
            dont_fake = dont_fake_orig_0;
            if result == -(1 as libc::c_int) {
                return 0 as libc::c_int as libc::c_uint;
            }
            if rem.tv_sec != 0 as libc::c_int as libc::c_long
                || rem.tv_nsec != 0 as libc::c_int as libc::c_long
            {
                let mut tmp_time_0: int64_t = 0;
                tmp_time_0 = (user_rate
                    * (rem.tv_sec * 1000000000 as libc::c_int as libc::c_long
                        + rem.tv_nsec) as libc::c_double) as int64_t;
                rem.tv_nsec = tmp_time_0 % 1000000000 as libc::c_int as libc::c_long;
                rem
                    .tv_sec = (tmp_time_0 - rem.tv_nsec)
                    / 1000000000 as libc::c_int as libc::c_long;
                if rem.tv_nsec < 0 as libc::c_int as libc::c_long {
                    rem.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                    rem.tv_sec -= 1 as libc::c_int as libc::c_long;
                }
            }
            return rem.tv_sec as libc::c_uint;
        }
    } else {
        let mut ret_0: libc::c_uint = 0;
        let mut dont_fake_orig_1: bool = dont_fake;
        if !dont_fake {
            dont_fake = 1 as libc::c_int != 0;
        }
        ret_0 = (Some(real_sleep.unwrap())).unwrap()(seconds);
        dont_fake = dont_fake_orig_1;
        return ret_0;
    };
}
#[no_mangle] pub unsafe extern "C" fn alarm(mut seconds: libc::c_uint) -> libc::c_uint {
    let mut ret: libc::c_uint = 0;
    let mut seconds_real: libc::c_uint = (if user_rate_set as libc::c_int != 0
        && !dont_fake
    {
        1.0f64 / user_rate * seconds as libc::c_double
    } else {
        seconds as libc::c_double
    }) as libc::c_uint;
    if initialized == 0 {
        ftpl_init();
    }
    if real_alarm.is_none() {
        return -(1 as libc::c_int) as libc::c_uint;
    }
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    ret = (Some(real_alarm.unwrap())).unwrap()(seconds_real);
    dont_fake = dont_fake_orig;
    return (if user_rate_set as libc::c_int != 0 && !dont_fake {
        user_rate * ret as libc::c_double
    } else {
        ret as libc::c_double
    }) as libc::c_uint;
}
#[no_mangle] pub unsafe extern "C" fn ppoll(
    mut fds: *mut pollfd,
    mut nfds: nfds_t,
    mut timeout_ts: *const timespec,
    mut sigmask: *const sigset_t,
) -> libc::c_int {
    let mut real_timeout: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut real_timeout_pt: *mut timespec = 0 as *mut timespec;
    let mut ret: libc::c_int = 0;
    if initialized == 0 {
        ftpl_init();
    }
    if real_ppoll.is_none() {
        return -(1 as libc::c_int);
    }
    if !timeout_ts.is_null() {
        if user_rate_set as libc::c_int != 0 && !dont_fake
            && ((*timeout_ts).tv_sec > 0 as libc::c_int as libc::c_long
                || (*timeout_ts).tv_nsec > 0 as libc::c_int as libc::c_long)
        {
            let mut tmp_time: int64_t = 0;
            tmp_time = (1.0f64 / user_rate
                * ((*timeout_ts).tv_sec * 1000000000 as libc::c_int as libc::c_long
                    + (*timeout_ts).tv_nsec) as libc::c_double) as int64_t;
            real_timeout.tv_nsec = tmp_time % 1000000000 as libc::c_int as libc::c_long;
            real_timeout
                .tv_sec = (tmp_time - real_timeout.tv_nsec)
                / 1000000000 as libc::c_int as libc::c_long;
            if real_timeout.tv_nsec < 0 as libc::c_int as libc::c_long {
                real_timeout.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                real_timeout.tv_sec -= 1 as libc::c_int as libc::c_long;
            }
            real_timeout_pt = &mut real_timeout;
        } else {
            real_timeout_pt = timeout_ts as *mut timespec;
        }
    } else {
        real_timeout_pt = 0 as *mut timespec;
    }
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    ret = (Some(real_ppoll.unwrap())).unwrap()(fds, nfds, real_timeout_pt, sigmask);
    dont_fake = dont_fake_orig;
    return ret;
}
#[no_mangle] pub unsafe extern "C" fn epoll_wait(
    mut epfd: libc::c_int,
    mut events: *mut epoll_event,
    mut maxevents: libc::c_int,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut real_timeout: libc::c_int = 0;
    if initialized == 0 {
        ftpl_init();
    }
    if real_epoll_wait.is_none() {
        return -(1 as libc::c_int);
    }
    if user_rate_set as libc::c_int != 0 && !dont_fake && timeout > 0 as libc::c_int {
        real_timeout = (timeout as libc::c_double * 1.0f64 / user_rate) as libc::c_int;
    } else {
        real_timeout = timeout;
    }
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    ret = (Some(real_epoll_wait.unwrap()))
        .unwrap()(epfd, events, maxevents, real_timeout);
    dont_fake = dont_fake_orig;
    return ret;
}
#[no_mangle] pub unsafe extern "C" fn epoll_pwait(
    mut epfd: libc::c_int,
    mut events: *mut epoll_event,
    mut maxevents: libc::c_int,
    mut timeout: libc::c_int,
    mut sigmask: *const sigset_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut real_timeout: libc::c_int = 0;
    if initialized == 0 {
        ftpl_init();
    }
    if real_epoll_pwait.is_none() {
        return -(1 as libc::c_int);
    }
    if user_rate_set as libc::c_int != 0 && !dont_fake && timeout > 0 as libc::c_int {
        real_timeout = (timeout as libc::c_double * 1.0f64 / user_rate) as libc::c_int;
    } else {
        real_timeout = timeout;
    }
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    ret = (Some(real_epoll_pwait.unwrap()))
        .unwrap()(epfd, events, maxevents, real_timeout, sigmask);
    dont_fake = dont_fake_orig;
    return ret;
}
#[no_mangle] pub unsafe extern "C" fn poll(
    mut fds: *mut pollfd,
    mut nfds: nfds_t,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut timeout_real: libc::c_int = (if user_rate_set as libc::c_int != 0
        && !dont_fake && timeout > 0 as libc::c_int
    {
        timeout as libc::c_double / user_rate
    } else {
        timeout as libc::c_double
    }) as libc::c_int;
    if initialized == 0 {
        ftpl_init();
    }
    if real_poll.is_none() {
        return -(1 as libc::c_int);
    }
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    ret = (Some(real_poll.unwrap())).unwrap()(fds, nfds, timeout_real);
    dont_fake = dont_fake_orig;
    return ret;
}
#[no_mangle] pub unsafe extern "C" fn select(
    mut nfds: libc::c_int,
    mut readfds: *mut fd_set,
    mut writefds: *mut fd_set,
    mut errorfds: *mut fd_set,
    mut timeout: *mut timeval,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut timeout_real: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    if initialized == 0 {
        ftpl_init();
    }
    if real_select.is_none() {
        return -(1 as libc::c_int);
    }
    if !timeout.is_null() {
        if user_rate_set as libc::c_int != 0 && !dont_fake
            && ((*timeout).tv_sec > 0 as libc::c_int as libc::c_long
                || (*timeout).tv_usec > 0 as libc::c_int as libc::c_long)
        {
            let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
            ts.tv_sec = (*timeout).tv_sec;
            ts.tv_nsec = (*timeout).tv_usec * 1000 as libc::c_int as libc::c_long;
            let mut tmp_time: int64_t = 0;
            tmp_time = (1.0f64 / user_rate
                * (ts.tv_sec * 1000000000 as libc::c_int as libc::c_long + ts.tv_nsec)
                    as libc::c_double) as int64_t;
            ts.tv_nsec = tmp_time % 1000000000 as libc::c_int as libc::c_long;
            ts
                .tv_sec = (tmp_time - ts.tv_nsec)
                / 1000000000 as libc::c_int as libc::c_long;
            if ts.tv_nsec < 0 as libc::c_int as libc::c_long {
                ts.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                ts.tv_sec -= 1 as libc::c_int as libc::c_long;
            }
            timeout_real.tv_sec = ts.tv_sec;
            timeout_real.tv_usec = ts.tv_nsec / 1000 as libc::c_int as libc::c_long;
        } else {
            timeout_real.tv_sec = (*timeout).tv_sec;
            timeout_real.tv_usec = (*timeout).tv_usec;
        }
    }
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    ret = (Some(real_select.unwrap()))
        .unwrap()(
        nfds,
        readfds,
        writefds,
        errorfds,
        if timeout.is_null() { timeout } else { &mut timeout_real },
    );
    dont_fake = dont_fake_orig;
    if user_rate_set as libc::c_int != 0 && !timeout.is_null() {
        let mut ts_0: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        ts_0.tv_sec = timeout_real.tv_sec;
        ts_0.tv_nsec = timeout_real.tv_usec * 1000 as libc::c_int as libc::c_long;
        let mut tmp_time_0: int64_t = 0;
        tmp_time_0 = (user_rate
            * (ts_0.tv_sec * 1000000000 as libc::c_int as libc::c_long + ts_0.tv_nsec)
                as libc::c_double) as int64_t;
        ts_0.tv_nsec = tmp_time_0 % 1000000000 as libc::c_int as libc::c_long;
        ts_0
            .tv_sec = (tmp_time_0 - ts_0.tv_nsec)
            / 1000000000 as libc::c_int as libc::c_long;
        if ts_0.tv_nsec < 0 as libc::c_int as libc::c_long {
            ts_0.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
            ts_0.tv_sec -= 1 as libc::c_int as libc::c_long;
        }
        (*timeout).tv_sec = ts_0.tv_sec;
        (*timeout).tv_usec = ts_0.tv_nsec / 1000 as libc::c_int as libc::c_long;
    }
    return ret;
}
#[no_mangle] pub unsafe extern "C" fn pselect(
    mut nfds: libc::c_int,
    mut readfds: *mut fd_set,
    mut writefds: *mut fd_set,
    mut errorfds: *mut fd_set,
    mut timeout: *const timespec,
    mut sigmask: *const sigset_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut timeout_real: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    if initialized == 0 {
        ftpl_init();
    }
    if real_pselect.is_none() {
        return -(1 as libc::c_int);
    }
    if !timeout.is_null() {
        if user_rate_set as libc::c_int != 0 && !dont_fake
            && ((*timeout).tv_sec > 0 as libc::c_int as libc::c_long
                || (*timeout).tv_nsec > 0 as libc::c_int as libc::c_long)
        {
            let mut tmp_time: int64_t = 0;
            tmp_time = (1.0f64 / user_rate
                * ((*timeout).tv_sec * 1000000000 as libc::c_int as libc::c_long
                    + (*timeout).tv_nsec) as libc::c_double) as int64_t;
            timeout_real.tv_nsec = tmp_time % 1000000000 as libc::c_int as libc::c_long;
            timeout_real
                .tv_sec = (tmp_time - timeout_real.tv_nsec)
                / 1000000000 as libc::c_int as libc::c_long;
            if timeout_real.tv_nsec < 0 as libc::c_int as libc::c_long {
                timeout_real.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                timeout_real.tv_sec -= 1 as libc::c_int as libc::c_long;
            }
        } else {
            timeout_real.tv_sec = (*timeout).tv_sec;
            timeout_real.tv_nsec = (*timeout).tv_nsec;
        }
    }
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    ret = (Some(real_pselect.unwrap()))
        .unwrap()(
        nfds,
        readfds,
        writefds,
        errorfds,
        if timeout.is_null() {
            timeout
        } else {
            &mut timeout_real as *mut timespec as *const timespec
        },
        sigmask,
    );
    dont_fake = dont_fake_orig;
    return ret;
}
#[no_mangle] pub unsafe extern "C" fn sem_timedwait(
    mut sem: *mut sem_t,
    mut abs_timeout: *const timespec,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut real_abs_timeout: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut real_abs_timeout_pt: *mut timespec = 0 as *mut timespec;
    if abs_timeout.is_null() {
        return -(1 as libc::c_int);
    }
    if !check_missing_real(
        b"sem_timedwait\0" as *const u8 as *const libc::c_char,
        real_sem_timedwait.is_none(),
    ) {
        return -(1 as libc::c_int);
    }
    if !dont_fake {
        let mut tdiff: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        let mut timeadj: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        tdiff.tv_sec = (*abs_timeout).tv_sec - user_faked_time_timespec.tv_sec;
        tdiff.tv_nsec = (*abs_timeout).tv_nsec - user_faked_time_timespec.tv_nsec;
        if tdiff.tv_nsec < 0 as libc::c_int as libc::c_long {
            tdiff.tv_sec -= 1;
            tdiff.tv_sec;
            tdiff.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
        }
        if user_rate_set {
            let mut tmp_time: int64_t = 0;
            tmp_time = (user_rate
                * (tdiff.tv_sec * 1000000000 as libc::c_int as libc::c_long
                    + tdiff.tv_nsec) as libc::c_double) as int64_t;
            timeadj.tv_nsec = tmp_time % 1000000000 as libc::c_int as libc::c_long;
            timeadj
                .tv_sec = (tmp_time - timeadj.tv_nsec)
                / 1000000000 as libc::c_int as libc::c_long;
            if timeadj.tv_nsec < 0 as libc::c_int as libc::c_long {
                timeadj.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                timeadj.tv_sec -= 1 as libc::c_int as libc::c_long;
            }
        } else {
            timeadj = tdiff;
        }
        real_abs_timeout.tv_sec = ftpl_starttime.real.tv_sec + timeadj.tv_sec;
        real_abs_timeout.tv_nsec = ftpl_starttime.real.tv_nsec + timeadj.tv_nsec;
        if real_abs_timeout.tv_nsec >= 1000000000 as libc::c_int as libc::c_long {
            real_abs_timeout.tv_sec += 1;
            real_abs_timeout.tv_sec;
            real_abs_timeout.tv_nsec -= 1000000000 as libc::c_int as libc::c_long;
        }
        real_abs_timeout_pt = &mut real_abs_timeout;
    } else {
        real_abs_timeout_pt = abs_timeout as *mut timespec;
    }
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = (Some(real_sem_timedwait.unwrap())).unwrap()(sem, real_abs_timeout_pt);
    dont_fake = dont_fake_orig;
    return result;
}
#[no_mangle] pub unsafe extern "C" fn sem_clockwait(
    mut sem: *mut sem_t,
    mut clockid: clockid_t,
    mut abstime: *const timespec,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut real_abstime: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut real_abstime_pt: *mut timespec = 0 as *mut timespec;
    if fake_monotonic_clock == 0 && clockid == 1 as libc::c_int {
        let mut dont_fake_orig: bool = dont_fake;
        if !dont_fake {
            dont_fake = 1 as libc::c_int != 0;
        }
        result = (Some(real_sem_clockwait.unwrap())).unwrap()(sem, clockid, abstime);
        dont_fake = dont_fake_orig;
        return result;
    }
    if abstime.is_null() {
        return -(1 as libc::c_int);
    }
    if !check_missing_real(
        b"sem_clockwait\0" as *const u8 as *const libc::c_char,
        real_sem_clockwait.is_none(),
    ) {
        return -(1 as libc::c_int);
    }
    if !dont_fake {
        let mut tdiff: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        let mut timeadj: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        tdiff.tv_sec = (*abstime).tv_sec - user_faked_time_timespec.tv_sec;
        tdiff.tv_nsec = (*abstime).tv_nsec - user_faked_time_timespec.tv_nsec;
        if tdiff.tv_nsec < 0 as libc::c_int as libc::c_long {
            tdiff.tv_sec -= 1;
            tdiff.tv_sec;
            tdiff.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
        }
        if user_rate_set {
            let mut tmp_time: int64_t = 0;
            tmp_time = (1.0f64 / user_rate
                * (tdiff.tv_sec * 1000000000 as libc::c_int as libc::c_long
                    + tdiff.tv_nsec) as libc::c_double) as int64_t;
            timeadj.tv_nsec = tmp_time % 1000000000 as libc::c_int as libc::c_long;
            timeadj
                .tv_sec = (tmp_time - timeadj.tv_nsec)
                / 1000000000 as libc::c_int as libc::c_long;
            if timeadj.tv_nsec < 0 as libc::c_int as libc::c_long {
                timeadj.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                timeadj.tv_sec -= 1 as libc::c_int as libc::c_long;
            }
        } else {
            timeadj = tdiff;
        }
        if clockid == 0 as libc::c_int {
            real_abstime.tv_sec = ftpl_starttime.real.tv_sec + timeadj.tv_sec;
            real_abstime.tv_nsec = ftpl_starttime.real.tv_nsec + timeadj.tv_nsec;
            if real_abstime.tv_nsec >= 1000000000 as libc::c_int as libc::c_long {
                real_abstime.tv_sec += 1;
                real_abstime.tv_sec;
                real_abstime.tv_nsec -= 1000000000 as libc::c_int as libc::c_long;
            }
        }
        if clockid == 1 as libc::c_int {
            real_abstime.tv_sec = ftpl_starttime.mon.tv_sec + timeadj.tv_sec;
            real_abstime.tv_nsec = ftpl_starttime.mon.tv_nsec + timeadj.tv_nsec;
            if real_abstime.tv_nsec >= 1000000000 as libc::c_int as libc::c_long {
                real_abstime.tv_sec += 1;
                real_abstime.tv_sec;
                real_abstime.tv_nsec -= 1000000000 as libc::c_int as libc::c_long;
            }
        }
        real_abstime_pt = &mut real_abstime;
    } else {
        real_abstime_pt = abstime as *mut timespec;
    }
    let mut dont_fake_orig_0: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = (Some(real_sem_clockwait.unwrap())).unwrap()(sem, clockid, real_abstime_pt);
    dont_fake = dont_fake_orig_0;
    return result;
}
unsafe extern "C" fn timer_settime_common(
    mut timerid: timer_t_or_int,
    mut flags: libc::c_int,
    mut new_value: *const itimerspec,
    mut old_value: *mut itimerspec,
    mut compat: ft_lib_compat_timer,
    mut abstime_flag: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut new_real: itimerspec = itimerspec {
        it_interval: timespec { tv_sec: 0, tv_nsec: 0 },
        it_value: timespec { tv_sec: 0, tv_nsec: 0 },
    };
    let mut new_real_pt: *mut itimerspec = &mut new_real;
    if initialized == 0 {
        ftpl_init();
    }
    if new_value.is_null() {
        new_real_pt = 0 as *mut itimerspec;
    } else if dont_fake {
        new_real_pt = new_value as *mut itimerspec;
    } else {
        if (*new_value).it_value.tv_sec != 0 as libc::c_int as libc::c_long
            || (*new_value).it_value.tv_nsec != 0 as libc::c_int as libc::c_long
        {
            if flags & abstime_flag != 0 {
                let mut tdiff: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
                let mut timeadj: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
                timeadj
                    .tv_sec = (*new_value).it_value.tv_sec
                    - user_faked_time_timespec.tv_sec;
                timeadj
                    .tv_nsec = (*new_value).it_value.tv_nsec
                    - user_faked_time_timespec.tv_nsec;
                if timeadj.tv_nsec < 0 as libc::c_int as libc::c_long {
                    timeadj.tv_sec -= 1;
                    timeadj.tv_sec;
                    timeadj.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                }
                if user_rate_set {
                    let mut tmp_time: int64_t = 0;
                    tmp_time = (1.0f64 / user_rate
                        * (timeadj.tv_sec * 1000000000 as libc::c_int as libc::c_long
                            + timeadj.tv_nsec) as libc::c_double) as int64_t;
                    tdiff.tv_nsec = tmp_time % 1000000000 as libc::c_int as libc::c_long;
                    tdiff
                        .tv_sec = (tmp_time - tdiff.tv_nsec)
                        / 1000000000 as libc::c_int as libc::c_long;
                    if tdiff.tv_nsec < 0 as libc::c_int as libc::c_long {
                        tdiff.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                        tdiff.tv_sec -= 1 as libc::c_int as libc::c_long;
                    }
                } else {
                    tdiff = timeadj;
                }
                new_real.it_value.tv_sec = ftpl_starttime.real.tv_sec + tdiff.tv_sec;
                new_real.it_value.tv_nsec = ftpl_starttime.real.tv_nsec + tdiff.tv_nsec;
                if new_real.it_value.tv_nsec >= 1000000000 as libc::c_int as libc::c_long
                {
                    new_real.it_value.tv_sec += 1;
                    new_real.it_value.tv_sec;
                    new_real.it_value.tv_nsec
                        -= 1000000000 as libc::c_int as libc::c_long;
                }
            } else if user_rate_set {
                let mut tmp_time_0: int64_t = 0;
                tmp_time_0 = (1.0f64 / user_rate
                    * ((*new_value).it_value.tv_sec
                        * 1000000000 as libc::c_int as libc::c_long
                        + (*new_value).it_value.tv_nsec) as libc::c_double) as int64_t;
                new_real
                    .it_value
                    .tv_nsec = tmp_time_0 % 1000000000 as libc::c_int as libc::c_long;
                new_real
                    .it_value
                    .tv_sec = (tmp_time_0 - new_real.it_value.tv_nsec)
                    / 1000000000 as libc::c_int as libc::c_long;
                if new_real.it_value.tv_nsec < 0 as libc::c_int as libc::c_long {
                    new_real.it_value.tv_nsec
                        += 1000000000 as libc::c_int as libc::c_long;
                    new_real.it_value.tv_sec -= 1 as libc::c_int as libc::c_long;
                }
            } else {
                new_real.it_value = (*new_value).it_value;
            }
        } else {
            new_real.it_value = (*new_value).it_value;
        }
        if user_rate_set as libc::c_int != 0
            && ((*new_value).it_interval.tv_sec != 0 as libc::c_int as libc::c_long
                || (*new_value).it_interval.tv_nsec != 0 as libc::c_int as libc::c_long)
        {
            let mut tmp_time_1: int64_t = 0;
            tmp_time_1 = (1.0f64 / user_rate
                * ((*new_value).it_interval.tv_sec
                    * 1000000000 as libc::c_int as libc::c_long
                    + (*new_value).it_interval.tv_nsec) as libc::c_double) as int64_t;
            new_real
                .it_interval
                .tv_nsec = tmp_time_1 % 1000000000 as libc::c_int as libc::c_long;
            new_real
                .it_interval
                .tv_sec = (tmp_time_1 - new_real.it_interval.tv_nsec)
                / 1000000000 as libc::c_int as libc::c_long;
            if new_real.it_interval.tv_nsec < 0 as libc::c_int as libc::c_long {
                new_real.it_interval.tv_nsec
                    += 1000000000 as libc::c_int as libc::c_long;
                new_real.it_interval.tv_sec -= 1 as libc::c_int as libc::c_long;
            }
        } else {
            new_real.it_interval = (*new_value).it_interval;
        }
    }
    match compat as libc::c_uint {
        0 => {
            let mut dont_fake_orig: bool = dont_fake;
            if !dont_fake {
                dont_fake = 1 as libc::c_int != 0;
            }
            result = (Some(real_timer_settime_22.unwrap()))
                .unwrap()(timerid.int_member, flags, new_real_pt, old_value);
            dont_fake = dont_fake_orig;
        }
        1 => {
            let mut dont_fake_orig_0: bool = dont_fake;
            if !dont_fake {
                dont_fake = 1 as libc::c_int != 0;
            }
            result = (Some(real_timer_settime_233.unwrap()))
                .unwrap()(timerid.timer_t_member, flags, new_real_pt, old_value);
            dont_fake = dont_fake_orig_0;
        }
        2 => {
            let mut dont_fake_orig_1: bool = dont_fake;
            if !dont_fake {
                dont_fake = 1 as libc::c_int != 0;
            }
            result = (Some(real_timerfd_settime.unwrap()))
                .unwrap()(timerid.int_member, flags, new_real_pt, old_value);
            dont_fake = dont_fake_orig_1;
        }
        _ => {
            result = -(1 as libc::c_int);
        }
    }
    if result == -(1 as libc::c_int) {
        return result;
    }
    if !old_value.is_null() && !dont_fake {
        if (*old_value).it_value.tv_sec != 0 as libc::c_int as libc::c_long
            || (*old_value).it_value.tv_nsec != 0 as libc::c_int as libc::c_long
        {
            result = fake_clock_gettime(0 as libc::c_int, &mut (*old_value).it_value);
        }
        if user_rate_set as libc::c_int != 0
            && ((*old_value).it_interval.tv_sec != 0 as libc::c_int as libc::c_long
                || (*old_value).it_interval.tv_nsec != 0 as libc::c_int as libc::c_long)
        {
            let mut tmp_time_2: int64_t = 0;
            tmp_time_2 = (user_rate
                * ((*old_value).it_interval.tv_sec
                    * 1000000000 as libc::c_int as libc::c_long
                    + (*old_value).it_interval.tv_nsec) as libc::c_double) as int64_t;
            (*old_value)
                .it_interval
                .tv_nsec = tmp_time_2 % 1000000000 as libc::c_int as libc::c_long;
            (*old_value)
                .it_interval
                .tv_sec = (tmp_time_2 - (*old_value).it_interval.tv_nsec)
                / 1000000000 as libc::c_int as libc::c_long;
            if (*old_value).it_interval.tv_nsec < 0 as libc::c_int as libc::c_long {
                (*old_value).it_interval.tv_nsec
                    += 1000000000 as libc::c_int as libc::c_long;
                (*old_value).it_interval.tv_sec -= 1 as libc::c_int as libc::c_long;
            }
        }
    }
    return result;
}
#[no_mangle] pub unsafe extern "C" fn timer_settime_22(
    mut timerid: libc::c_int,
    mut flags: libc::c_int,
    mut new_value: *const itimerspec,
    mut old_value: *mut itimerspec,
) -> libc::c_int {
    if initialized == 0 {
        ftpl_init();
    }
    if real_timer_settime_22.is_none() {
        return -(1 as libc::c_int)
    } else {
        let mut temp: timer_t_or_int = timer_t_or_int { int_member: 0 };
        temp.int_member = timerid;
        return timer_settime_common(
            temp,
            flags,
            new_value,
            old_value,
            FT_COMPAT_GLIBC_2_2,
            1 as libc::c_int,
        );
    };
}
#[no_mangle] pub unsafe extern "C" fn timer_settime_233(
    mut timerid: timer_t,
    mut flags: libc::c_int,
    mut new_value: *const itimerspec,
    mut old_value: *mut itimerspec,
) -> libc::c_int {
    if initialized == 0 {
        ftpl_init();
    }
    if real_timer_settime_233.is_none() {
        return -(1 as libc::c_int)
    } else {
        let mut temp: timer_t_or_int = timer_t_or_int { int_member: 0 };
        temp.timer_t_member = timerid;
        return timer_settime_common(
            temp,
            flags,
            new_value,
            old_value,
            FT_COMPAT_GLIBC_2_3_3,
            1 as libc::c_int,
        );
    };
}
#[no_mangle] pub unsafe extern "C" fn timer_gettime_common(
    mut timerid: timer_t_or_int,
    mut curr_value: *mut itimerspec,
    mut compat: ft_lib_compat_timer,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    if initialized == 0 {
        ftpl_init();
    }
    if real_timer_gettime_233.is_none() {
        return -(1 as libc::c_int);
    }
    match compat as libc::c_uint {
        0 => {
            let mut dont_fake_orig: bool = dont_fake;
            if !dont_fake {
                dont_fake = 1 as libc::c_int != 0;
            }
            result = (Some(real_timer_gettime_22.unwrap()))
                .unwrap()(timerid.int_member, curr_value);
            dont_fake = dont_fake_orig;
        }
        1 => {
            let mut dont_fake_orig_0: bool = dont_fake;
            if !dont_fake {
                dont_fake = 1 as libc::c_int != 0;
            }
            result = (Some(real_timer_gettime_233.unwrap()))
                .unwrap()(timerid.timer_t_member, curr_value);
            dont_fake = dont_fake_orig_0;
        }
        2 => {
            let mut dont_fake_orig_1: bool = dont_fake;
            if !dont_fake {
                dont_fake = 1 as libc::c_int != 0;
            }
            result = (Some(real_timerfd_gettime.unwrap()))
                .unwrap()(timerid.int_member, curr_value);
            dont_fake = dont_fake_orig_1;
        }
        _ => {
            result = -(1 as libc::c_int);
        }
    }
    if result == -(1 as libc::c_int) {
        return result;
    }
    if !curr_value.is_null() {
        if user_rate_set as libc::c_int != 0 && !dont_fake {
            let mut tmp_time: int64_t = 0;
            tmp_time = (user_rate
                * ((*curr_value).it_interval.tv_sec
                    * 1000000000 as libc::c_int as libc::c_long
                    + (*curr_value).it_interval.tv_nsec) as libc::c_double) as int64_t;
            (*curr_value)
                .it_interval
                .tv_nsec = tmp_time % 1000000000 as libc::c_int as libc::c_long;
            (*curr_value)
                .it_interval
                .tv_sec = (tmp_time - (*curr_value).it_interval.tv_nsec)
                / 1000000000 as libc::c_int as libc::c_long;
            if (*curr_value).it_interval.tv_nsec < 0 as libc::c_int as libc::c_long {
                (*curr_value).it_interval.tv_nsec
                    += 1000000000 as libc::c_int as libc::c_long;
                (*curr_value).it_interval.tv_sec -= 1 as libc::c_int as libc::c_long;
            }
            let mut tmp_time_0: int64_t = 0;
            tmp_time_0 = (user_rate
                * ((*curr_value).it_value.tv_sec
                    * 1000000000 as libc::c_int as libc::c_long
                    + (*curr_value).it_value.tv_nsec) as libc::c_double) as int64_t;
            (*curr_value)
                .it_value
                .tv_nsec = tmp_time_0 % 1000000000 as libc::c_int as libc::c_long;
            (*curr_value)
                .it_value
                .tv_sec = (tmp_time_0 - (*curr_value).it_value.tv_nsec)
                / 1000000000 as libc::c_int as libc::c_long;
            if (*curr_value).it_value.tv_nsec < 0 as libc::c_int as libc::c_long {
                (*curr_value).it_value.tv_nsec
                    += 1000000000 as libc::c_int as libc::c_long;
                (*curr_value).it_value.tv_sec -= 1 as libc::c_int as libc::c_long;
            }
        }
    }
    return result;
}
#[no_mangle] pub unsafe extern "C" fn timer_gettime_22(
    mut timerid: timer_t,
    mut curr_value: *mut itimerspec,
) -> libc::c_int {
    if initialized == 0 {
        ftpl_init();
    }
    if real_timer_gettime_22.is_none() {
        return -(1 as libc::c_int)
    } else {
        let mut temp: timer_t_or_int = timer_t_or_int { int_member: 0 };
        temp.timer_t_member = timerid;
        return timer_gettime_common(temp, curr_value, FT_COMPAT_GLIBC_2_2);
    };
}
#[no_mangle] pub unsafe extern "C" fn timer_gettime_233(
    mut timerid: timer_t,
    mut curr_value: *mut itimerspec,
) -> libc::c_int {
    if initialized == 0 {
        ftpl_init();
    }
    if real_timer_gettime_233.is_none() {
        return -(1 as libc::c_int)
    } else {
        let mut temp: timer_t_or_int = timer_t_or_int { int_member: 0 };
        temp.timer_t_member = timerid;
        return timer_gettime_common(temp, curr_value, FT_COMPAT_GLIBC_2_3_3);
    };
}
#[no_mangle] pub unsafe extern "C" fn timerfd_settime(
    mut fd: libc::c_int,
    mut flags: libc::c_int,
    mut new_value: *const itimerspec,
    mut old_value: *mut itimerspec,
) -> libc::c_int {
    if initialized == 0 {
        ftpl_init();
    }
    if real_timerfd_settime.is_none() {
        return -(1 as libc::c_int)
    } else {
        let mut temp: timer_t_or_int = timer_t_or_int { int_member: 0 };
        temp.int_member = fd;
        return timer_settime_common(
            temp,
            flags,
            new_value,
            old_value,
            FT_FD,
            TFD_TIMER_ABSTIME as libc::c_int,
        );
    };
}
#[no_mangle] pub unsafe extern "C" fn timerfd_gettime(
    mut fd: libc::c_int,
    mut curr_value: *mut itimerspec,
) -> libc::c_int {
    if initialized == 0 {
        ftpl_init();
    }
    if real_timerfd_gettime.is_none() {
        return -(1 as libc::c_int)
    } else {
        let mut temp: timer_t_or_int = timer_t_or_int { int_member: 0 };
        temp.int_member = fd;
        return timer_gettime_common(temp, curr_value, FT_FD);
    };
}
#[no_mangle] pub unsafe extern "C" fn time(mut time_tptr: *mut time_t) -> time_t {
    let mut tp: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut result: time_t = 0;
    if initialized == 0 {
        ftpl_init();
    }
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = (Some(real_clock_gettime.unwrap())).unwrap()(0 as libc::c_int, &mut tp)
        as time_t;
    dont_fake = dont_fake_orig;
    if result == -(1 as libc::c_int) as libc::c_long {
        return -(1 as libc::c_int) as time_t;
    }
    fake_clock_gettime(0 as libc::c_int, &mut tp);
    if !time_tptr.is_null() {
        *time_tptr = tp.tv_sec;
    }
    return tp.tv_sec;
}
#[no_mangle] pub unsafe extern "C" fn ftime(mut tb: *mut timeb) -> libc::c_int {
    let mut tp: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut result: libc::c_int = 0;
    if initialized == 0 {
        ftpl_init();
    }
    if tb.is_null() {
        return 0 as libc::c_int;
    }
    if !check_missing_real(
        b"ftime\0" as *const u8 as *const libc::c_char,
        real_ftime.is_none(),
    ) {
        return 0 as libc::c_int;
    }
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = (Some(real_ftime.unwrap())).unwrap()(tb);
    dont_fake = dont_fake_orig;
    if result == -(1 as libc::c_int) {
        return result;
    }
    let mut dont_fake_orig_0: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = (Some(real_clock_gettime.unwrap())).unwrap()(0 as libc::c_int, &mut tp);
    dont_fake = dont_fake_orig_0;
    if result == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    fake_clock_gettime(0 as libc::c_int, &mut tp);
    (*tb).time = tp.tv_sec;
    (*tb)
        .millitm = (tp.tv_nsec / 1000000 as libc::c_int as libc::c_long)
        as libc::c_ushort;
    return result;
}
#[no_mangle] pub unsafe extern "C" fn gettimeofday(
    mut tv: *mut timeval,
    mut tz: *mut libc::c_void,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    if initialized == 0 {
        ftpl_init();
    }
    if tv.is_null() {
        return -(1 as libc::c_int);
    }
    if !check_missing_real(
        b"gettimeofday\0" as *const u8 as *const libc::c_char,
        real_gettimeofday.is_none(),
    ) {
        return -(1 as libc::c_int);
    }
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = (Some(real_gettimeofday.unwrap())).unwrap()(tv, tz);
    dont_fake = dont_fake_orig;
    if result == -(1 as libc::c_int) {
        return result;
    }
    result = fake_gettimeofday(tv);
    return result;
}
#[no_mangle] pub unsafe extern "C" fn clock_gettime(
    mut clk_id: clockid_t,
    mut tp: *mut timespec,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    static mut recursion_depth: libc::c_int = 0 as libc::c_int;
    if initialized == 0 {
        recursion_depth += 1;
        recursion_depth;
        if recursion_depth == 2 as libc::c_int {
            fprintf(
                stderr,
                b"libfaketime: Unexpected recursive calls to clock_gettime() without proper initialization. Trying alternative.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            let mut dont_fake_orig: bool = dont_fake;
            if !dont_fake {
                dont_fake = 1 as libc::c_int != 0;
            }
            ftpl_init();
            dont_fake = dont_fake_orig;
        } else if recursion_depth == 3 as libc::c_int {
            fprintf(
                stderr,
                b"libfaketime: Cannot recover from unexpected recursive calls to clock_gettime().\n\0"
                    as *const u8 as *const libc::c_char,
            );
            fprintf(
                stderr,
                b"libfaketime:  Please check whether any other libraries are in use that clash with libfaketime.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            fprintf(
                stderr,
                b"libfaketime:  Returning -1 on clock_gettime() to break recursion now... if that does not work, please check other libraries' error handling.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            if !tp.is_null() {
                (*tp).tv_sec = 0 as libc::c_int as __time_t;
                (*tp).tv_nsec = 0 as libc::c_int as __syscall_slong_t;
            }
            return -(1 as libc::c_int);
        } else {
            ftpl_init();
        }
        recursion_depth -= 1;
        recursion_depth;
    }
    if tp.is_null() {
        return -(1 as libc::c_int);
    }
    if !check_missing_real(
        b"clock_gettime\0" as *const u8 as *const libc::c_char,
        real_clock_gettime.is_none(),
    ) {
        return -(1 as libc::c_int);
    }
    let mut dont_fake_orig_0: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = (Some(real_clock_gettime.unwrap())).unwrap()(clk_id, tp);
    dont_fake = dont_fake_orig_0;
    if result == -(1 as libc::c_int) {
        return result;
    }
    if fake_monotonic_clock != 0
        || clk_id != 1 as libc::c_int && clk_id != 4 as libc::c_int
            && clk_id != 6 as libc::c_int && clk_id != 7 as libc::c_int
    {
        result = fake_clock_gettime(clk_id, tp);
    }
    return result;
}
#[no_mangle] pub unsafe extern "C" fn timespec_get(
    mut ts: *mut timespec,
    mut base: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    if initialized == 0 {
        ftpl_init();
    }
    if ts.is_null() {
        return 0 as libc::c_int;
    }
    if !check_missing_real(
        b"timespec_get\0" as *const u8 as *const libc::c_char,
        real_timespec_get.is_none(),
    ) {
        return 0 as libc::c_int;
    }
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = (Some(real_timespec_get.unwrap())).unwrap()(ts, base);
    dont_fake = dont_fake_orig;
    if result == 0 as libc::c_int {
        return result;
    }
    fake_clock_gettime(0 as libc::c_int, ts);
    return result;
}
unsafe extern "C" fn parse_ft_string(mut user_faked_time: *const libc::c_char) {
    let mut frac_offset: libc::c_double = 0.;
    let mut master_file_stats: stat = stat {
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
    let mut ret: libc::c_int = 0;
    let mut user_faked_time_tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut tmp_time_fmt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nstime_str: *mut libc::c_char = 0 as *mut libc::c_char;
    if strncmp(
        user_faked_time,
        user_faked_time_saved.as_mut_ptr(),
        256 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        if *user_faked_time.offset(0 as libc::c_int as isize) as libc::c_int
            != '%' as i32
        {
            return;
        }
    }
    match *user_faked_time.offset(0 as libc::c_int as isize) as libc::c_int {
        43 | 45 => {
            if ft_mode as libc::c_uint != FT_NOOP as libc::c_int as libc::c_uint {
                ft_mode = FT_START_AT;
            }
            frac_offset = atof(user_faked_time);
            if !(strchr(user_faked_time, 'm' as i32)).is_null() {
                frac_offset *= 60 as libc::c_int as libc::c_double;
            } else if !(strchr(user_faked_time, 'h' as i32)).is_null() {
                frac_offset *= (60 as libc::c_int * 60 as libc::c_int) as libc::c_double;
            } else if !(strchr(user_faked_time, 'd' as i32)).is_null() {
                frac_offset
                    *= (60 as libc::c_int * 60 as libc::c_int * 24 as libc::c_int)
                        as libc::c_double;
            } else if !(strchr(user_faked_time, 'y' as i32)).is_null() {
                frac_offset
                    *= (60 as libc::c_int * 60 as libc::c_int * 24 as libc::c_int
                        * 365 as libc::c_int) as libc::c_double;
            }
            user_offset.tv_sec = floor(frac_offset) as __time_t;
            user_offset
                .tv_nsec = ((frac_offset - user_offset.tv_sec as libc::c_double)
                * 1000000000 as libc::c_int as libc::c_double) as __syscall_slong_t;
            user_faked_time_timespec
                .tv_sec = ftpl_starttime.real.tv_sec + user_offset.tv_sec;
            user_faked_time_timespec
                .tv_nsec = ftpl_starttime.real.tv_nsec + user_offset.tv_nsec;
            if user_faked_time_timespec.tv_nsec
                >= 1000000000 as libc::c_int as libc::c_long
            {
                user_faked_time_timespec.tv_sec += 1;
                user_faked_time_timespec.tv_sec;
                user_faked_time_timespec.tv_nsec
                    -= 1000000000 as libc::c_int as libc::c_long;
            }
        }
        64 => {
            ft_mode = FT_START_AT;
            user_faked_time_tm.tm_isdst = -(1 as libc::c_int);
            nstime_str = strptime(
                &*user_faked_time.offset(1 as libc::c_int as isize),
                user_faked_time_fmt.as_mut_ptr(),
                &mut user_faked_time_tm,
            );
            if !nstime_str.is_null() {
                user_faked_time_timespec.tv_sec = mktime(&mut user_faked_time_tm);
                user_faked_time_timespec.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
                if *nstime_str.offset(0 as libc::c_int as isize) as libc::c_int
                    == '.' as i32
                {
                    nstime_str = nstime_str.offset(-1);
                    let mut nstime_0: libc::c_double = atof(nstime_str);
                    user_faked_time_timespec
                        .tv_nsec = ((nstime_0 - floor(nstime_0))
                        * 1000000000 as libc::c_int as libc::c_double)
                        as __syscall_slong_t;
                }
            } else {
                fprintf(
                    stderr,
                    b"libfaketime: In parse_ft_string(), failed to parse FAKETIME timestamp.\n\0"
                        as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            if (getenv(b"FAKETIME_DONT_RESET\0" as *const u8 as *const libc::c_char))
                .is_null()
            {
                reset_time();
            }
        }
        37 => {
            ft_mode = FT_START_AT;
            master_file_stats = stat {
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
            ret = 0;
            if (getenv(b"FAKETIME_FOLLOW_FILE\0" as *const u8 as *const libc::c_char))
                .is_null()
            {
                fprintf(
                    stderr,
                    b"libfaketime: %% operator in FAKETIME setting requires environment variable FAKETIME_FOLLOW_FILE set.\n\0"
                        as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            } else {
                let mut dont_fake_orig: bool = dont_fake;
                if !dont_fake {
                    dont_fake = 1 as libc::c_int != 0;
                }
                ret = stat(
                    getenv(
                        b"FAKETIME_FOLLOW_FILE\0" as *const u8 as *const libc::c_char,
                    ),
                    &mut master_file_stats,
                );
                dont_fake = dont_fake_orig;
                if ret == -(1 as libc::c_int) {
                    fprintf(
                        stderr,
                        b"libfaketime: Cannot get timestamp of file %s as requested by %% operator.\n\0"
                            as *const u8 as *const libc::c_char,
                        getenv(
                            b"FAKETIME_FOLLOW_FILE\0" as *const u8 as *const libc::c_char,
                        ),
                    );
                    exit(1 as libc::c_int);
                } else {
                    user_faked_time_timespec.tv_sec = master_file_stats.st_mtim.tv_sec;
                    user_faked_time_timespec
                        .tv_nsec = 0 as libc::c_int as __syscall_slong_t;
                }
            }
            if (getenv(b"FAKETIME_DONT_RESET\0" as *const u8 as *const libc::c_char))
                .is_null()
            {
                reset_time();
            }
        }
        105 | 120 => {}
        _ => {
            if ft_mode as libc::c_uint != FT_NOOP as libc::c_int as libc::c_uint {
                ft_mode = FT_FREEZE;
            }
            user_faked_time_tm.tm_isdst = -(1 as libc::c_int);
            nstime_str = strptime(
                user_faked_time,
                user_faked_time_fmt.as_mut_ptr(),
                &mut user_faked_time_tm,
            );
            if !nstime_str.is_null() {
                user_faked_time_timespec.tv_sec = mktime(&mut user_faked_time_tm);
                user_faked_time_timespec.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
                if *nstime_str.offset(0 as libc::c_int as isize) as libc::c_int
                    == '.' as i32
                {
                    nstime_str = nstime_str.offset(-1);
                    let mut nstime: libc::c_double = atof(nstime_str);
                    user_faked_time_timespec
                        .tv_nsec = ((nstime - floor(nstime))
                        * 1000000000 as libc::c_int as libc::c_double)
                        as __syscall_slong_t;
                }
                user_faked_time_set = 1 as libc::c_int != 0;
            } else {
                fprintf(
                    stderr,
                    b"libfaketime: In parse_ft_string(), failed to parse FAKETIME timestamp.\nPlease check specification %s with format %s\n\0"
                        as *const u8 as *const libc::c_char,
                    user_faked_time,
                    user_faked_time_fmt.as_mut_ptr(),
                );
                exit(1 as libc::c_int);
            }
        }
    }
    if !(strchr(user_faked_time, 'x' as i32)).is_null() {
        user_rate = atof(
            (strchr(user_faked_time, 'x' as i32)).offset(1 as libc::c_int as isize),
        );
        user_rate_set = 1 as libc::c_int != 0;
        if !(getenv(b"FAKETIME_XRESET\0" as *const u8 as *const libc::c_char)).is_null()
        {
            if ftpl_timecache.real.tv_nsec >= 0 as libc::c_int as libc::c_long {
                user_faked_time_timespec.tv_sec = ftpl_faketimecache.real.tv_sec;
                user_faked_time_timespec.tv_nsec = ftpl_faketimecache.real.tv_nsec;
                ftpl_starttime.real.tv_sec = ftpl_timecache.real.tv_sec;
                ftpl_starttime.real.tv_nsec = ftpl_timecache.real.tv_nsec;
                ftpl_starttime.mon.tv_sec = ftpl_timecache.mon.tv_sec;
                ftpl_starttime.mon.tv_nsec = ftpl_timecache.mon.tv_nsec;
                ftpl_starttime.mon_raw.tv_sec = ftpl_timecache.mon_raw.tv_sec;
                ftpl_starttime.mon_raw.tv_nsec = ftpl_timecache.mon_raw.tv_nsec;
                ftpl_starttime.boot.tv_sec = ftpl_timecache.boot.tv_sec;
                ftpl_starttime.boot.tv_nsec = ftpl_timecache.boot.tv_nsec;
            }
        }
    } else {
        tmp_time_fmt = strchr(user_faked_time, 'i' as i32);
        if !tmp_time_fmt.is_null() {
            let mut tick_inc: libc::c_double = atof(
                tmp_time_fmt.offset(1 as libc::c_int as isize),
            );
            user_per_tick_inc.tv_sec = floor(tick_inc) as __time_t;
            user_per_tick_inc
                .tv_nsec = ((tick_inc - user_per_tick_inc.tv_sec as libc::c_double)
                * 1000000000 as libc::c_int as libc::c_double) as __syscall_slong_t;
            user_per_tick_inc_set = 1 as libc::c_int != 0;
        }
    }
    strncpy(
        user_faked_time_saved.as_mut_ptr(),
        user_faked_time,
        (256 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
    );
    user_faked_time_saved[(256 as libc::c_int - 1 as libc::c_int)
        as usize] = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn ftpl_init() {
    let mut tmp_env: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dont_fake_final: bool = false;
    dont_fake = 1 as libc::c_int != 0;
    dont_fake_final = 0 as libc::c_int != 0;
    let mut progname: *const libc::c_char = __progname;
    real_stat = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int>,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"stat\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_lstat = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int>,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"lstat\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_fstat = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(libc::c_int, *mut stat) -> libc::c_int>,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"fstat\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_xstat = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                libc::c_int,
                *const libc::c_char,
                *mut stat,
            ) -> libc::c_int,
        >,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"__xstat\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_fxstat = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(libc::c_int, libc::c_int, *mut stat) -> libc::c_int,
        >,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"__fxstat\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_fxstatat = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const libc::c_char,
                *mut stat,
                libc::c_int,
            ) -> libc::c_int,
        >,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"__fxstatat\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_lxstat = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                libc::c_int,
                *const libc::c_char,
                *mut stat,
            ) -> libc::c_int,
        >,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"__lxstat\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_xstat64 = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                libc::c_int,
                *const libc::c_char,
                *mut stat64,
            ) -> libc::c_int,
        >,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"__xstat64\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_fxstat64 = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(libc::c_int, libc::c_int, *mut stat64) -> libc::c_int,
        >,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"__fxstat64\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_fxstatat64 = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const libc::c_char,
                *mut stat64,
                libc::c_int,
            ) -> libc::c_int,
        >,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"__fxstatat64\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_lxstat64 = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                libc::c_int,
                *const libc::c_char,
                *mut stat64,
            ) -> libc::c_int,
        >,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"__lxstat64\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_time = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*mut time_t) -> time_t>,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"time\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_ftime = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*mut timeb) -> libc::c_int>,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"ftime\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_timespec_get = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*mut timespec, libc::c_int) -> libc::c_int>,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"timespec_get\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_utimes = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(*const libc::c_char, *const timeval) -> libc::c_int,
        >,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"utimes\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_utime = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(*const libc::c_char, *const utimbuf) -> libc::c_int,
        >,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"utime\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_utimensat = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                libc::c_int,
                *const libc::c_char,
                *const timespec,
                libc::c_int,
            ) -> libc::c_int,
        >,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"utimensat\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_futimens = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(libc::c_int, *const timespec) -> libc::c_int>,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"futimens\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_gettimeofday = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*mut timeval, *mut libc::c_void) -> libc::c_int>,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"gettimeofday\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_nanosleep = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*const timespec, *mut timespec) -> libc::c_int>,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"nanosleep\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_clock_nanosleep = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                clockid_t,
                libc::c_int,
                *const timespec,
                *mut timespec,
            ) -> libc::c_int,
        >,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"clock_nanosleep\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_usleep = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(useconds_t) -> libc::c_int>,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"usleep\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_sleep = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(libc::c_uint) -> libc::c_uint>,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"sleep\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_alarm = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(libc::c_uint) -> libc::c_uint>,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"alarm\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_poll = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*mut pollfd, nfds_t, libc::c_int) -> libc::c_int>,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"poll\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_ppoll = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                *mut pollfd,
                nfds_t,
                *const timespec,
                *const sigset_t,
            ) -> libc::c_int,
        >,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"ppoll\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_epoll_wait = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                libc::c_int,
                *mut epoll_event,
                libc::c_int,
                libc::c_int,
            ) -> libc::c_int,
        >,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"epoll_wait\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_epoll_pwait = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                libc::c_int,
                *mut epoll_event,
                libc::c_int,
                libc::c_int,
                *const sigset_t,
            ) -> libc::c_int,
        >,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"epoll_pwait\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_select = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                libc::c_int,
                *mut fd_set,
                *mut fd_set,
                *mut fd_set,
                *mut timeval,
            ) -> libc::c_int,
        >,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"select\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_pselect = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                libc::c_int,
                *mut fd_set,
                *mut fd_set,
                *mut fd_set,
                *const timespec,
                *const sigset_t,
            ) -> libc::c_int,
        >,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"pselect\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_sem_timedwait = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*mut sem_t, *const timespec) -> libc::c_int>,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"sem_timedwait\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_sem_clockwait = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(*mut sem_t, clockid_t, *const timespec) -> libc::c_int,
        >,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"sem_clockwait\0" as *const u8 as *const libc::c_char,
        ),
    );
    real___ftime = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*mut timeb) -> libc::c_int>,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"__ftime\0" as *const u8 as *const libc::c_char,
        ),
    );
    real___gettimeofday = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*mut timeval, *mut libc::c_void) -> libc::c_int>,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"__gettimeofday\0" as *const u8 as *const libc::c_char,
        ),
    );
    real___clock_gettime = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(clockid_t, *mut timespec) -> libc::c_int>,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"__clock_gettime\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_pthread_cond_timedwait_225 = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                *mut pthread_cond_t,
                *mut pthread_mutex_t,
                *mut timespec,
            ) -> libc::c_int,
        >,
    >(
        dlvsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"pthread_cond_timedwait\0" as *const u8 as *const libc::c_char,
            b"GLIBC_2.2.5\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_pthread_cond_timedwait_232 = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                *mut pthread_cond_t,
                *mut pthread_mutex_t,
                *mut timespec,
            ) -> libc::c_int,
        >,
    >(
        dlvsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"pthread_cond_timedwait\0" as *const u8 as *const libc::c_char,
            b"GLIBC_2.3.2\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_pthread_cond_init_232 = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                *mut pthread_cond_t,
                *const pthread_condattr_t,
            ) -> libc::c_int,
        >,
    >(
        dlvsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"pthread_cond_init\0" as *const u8 as *const libc::c_char,
            b"GLIBC_2.3.2\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_pthread_cond_destroy_232 = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*mut pthread_cond_t) -> libc::c_int>,
    >(
        dlvsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"pthread_cond_destroy\0" as *const u8 as *const libc::c_char,
            b"GLIBC_2.3.2\0" as *const u8 as *const libc::c_char,
        ),
    );
    if real_pthread_cond_timedwait_232.is_none() {
        real_pthread_cond_timedwait_232 = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *mut pthread_cond_t,
                    *mut pthread_mutex_t,
                    *mut timespec,
                ) -> libc::c_int,
            >,
        >(
            dlsym(
                -(1 as libc::c_long) as *mut libc::c_void,
                b"pthread_cond_timedwait\0" as *const u8 as *const libc::c_char,
            ),
        );
    }
    if real_pthread_cond_init_232.is_none() {
        real_pthread_cond_init_232 = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    *mut pthread_cond_t,
                    *const pthread_condattr_t,
                ) -> libc::c_int,
            >,
        >(
            dlsym(
                -(1 as libc::c_long) as *mut libc::c_void,
                b"pthread_cond_init\0" as *const u8 as *const libc::c_char,
            ),
        );
    }
    if real_pthread_cond_destroy_232.is_none() {
        real_pthread_cond_destroy_232 = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*mut pthread_cond_t) -> libc::c_int>,
        >(
            dlsym(
                -(1 as libc::c_long) as *mut libc::c_void,
                b"pthread_cond_destroy\0" as *const u8 as *const libc::c_char,
            ),
        );
    }
    if pthread_rwlock_init(&mut monotonic_conds_lock, 0 as *const pthread_rwlockattr_t)
        != 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"monotonic_conds_lock init failed\n\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    real_clock_gettime = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(clockid_t, *mut timespec) -> libc::c_int>,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"__clock_gettime\0" as *const u8 as *const libc::c_char,
        ),
    );
    if real_clock_gettime.is_none() {
        real_clock_gettime = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(clockid_t, *mut timespec) -> libc::c_int>,
        >(
            dlsym(
                -(1 as libc::c_long) as *mut libc::c_void,
                b"clock_gettime\0" as *const u8 as *const libc::c_char,
            ),
        );
    }
    real_timer_settime_22 = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const itimerspec,
                *mut itimerspec,
            ) -> libc::c_int,
        >,
    >(
        dlvsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"timer_settime\0" as *const u8 as *const libc::c_char,
            b"GLIBC_2.2\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_timer_settime_233 = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                timer_t,
                libc::c_int,
                *const itimerspec,
                *mut itimerspec,
            ) -> libc::c_int,
        >,
    >(
        dlvsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"timer_settime\0" as *const u8 as *const libc::c_char,
            b"GLIBC_2.3.3\0" as *const u8 as *const libc::c_char,
        ),
    );
    if real_timer_settime_233.is_none() {
        real_timer_settime_233 = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(
                    timer_t,
                    libc::c_int,
                    *const itimerspec,
                    *mut itimerspec,
                ) -> libc::c_int,
            >,
        >(
            dlsym(
                -(1 as libc::c_long) as *mut libc::c_void,
                b"timer_settime\0" as *const u8 as *const libc::c_char,
            ),
        );
    }
    real_timer_gettime_22 = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(libc::c_int, *mut itimerspec) -> libc::c_int>,
    >(
        dlvsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"timer_gettime\0" as *const u8 as *const libc::c_char,
            b"GLIBC_2.2\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_timer_gettime_233 = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(timer_t, *mut itimerspec) -> libc::c_int>,
    >(
        dlvsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"timer_gettime\0" as *const u8 as *const libc::c_char,
            b"GLIBC_2.3.3\0" as *const u8 as *const libc::c_char,
        ),
    );
    if real_timer_gettime_233.is_none() {
        real_timer_gettime_233 = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(timer_t, *mut itimerspec) -> libc::c_int>,
        >(
            dlsym(
                -(1 as libc::c_long) as *mut libc::c_void,
                b"timer_gettime\0" as *const u8 as *const libc::c_char,
            ),
        );
    }
    real_timerfd_gettime = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(libc::c_int, *mut itimerspec) -> libc::c_int>,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"timerfd_gettime\0" as *const u8 as *const libc::c_char,
        ),
    );
    real_timerfd_settime = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const itimerspec,
                *mut itimerspec,
            ) -> libc::c_int,
        >,
    >(
        dlsym(
            -(1 as libc::c_long) as *mut libc::c_void,
            b"timerfd_settime\0" as *const u8 as *const libc::c_char,
        ),
    );
    initialized = 1 as libc::c_int;
    ft_shm_init();
    if !(getenv(b"NO_FAKE_STAT\0" as *const u8 as *const libc::c_char)).is_null() {
        fake_stat_disabled = 1 as libc::c_int;
    }
    tmp_env = getenv(b"FAKE_UTIME\0" as *const u8 as *const libc::c_char);
    if !tmp_env.is_null() {
        if *tmp_env == 0 || *tmp_env as libc::c_int == 'y' as i32
            || *tmp_env as libc::c_int == 'Y' as i32
            || *tmp_env as libc::c_int == 't' as i32
            || *tmp_env as libc::c_int == 'T' as i32
        {
            fake_utime_disabled = 0 as libc::c_int;
        } else {
            fake_utime_disabled = (atoi(tmp_env) == 0) as libc::c_int;
        }
    }
    tmp_env = getenv(b"FAKETIME_CACHE_DURATION\0" as *const u8 as *const libc::c_char);
    if !tmp_env.is_null() {
        cache_duration = atoi(tmp_env);
    }
    tmp_env = getenv(b"FAKETIME_NO_CACHE\0" as *const u8 as *const libc::c_char);
    if !tmp_env.is_null() {
        if 0 as libc::c_int
            == strcmp(tmp_env, b"1\0" as *const u8 as *const libc::c_char)
        {
            cache_enabled = 0 as libc::c_int;
        }
    }
    get_fake_monotonic_setting(&mut fake_monotonic_clock);
    tmp_env = getenv(b"FAKETIME_SKIP_CMDS\0" as *const u8 as *const libc::c_char);
    if !tmp_env.is_null() {
        let mut skip_cmd: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut saveptr: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut tmpvar: *mut libc::c_char = 0 as *mut libc::c_char;
        tmpvar = strdup(tmp_env);
        if !tmpvar.is_null() {
            skip_cmd = strtok_r(
                tmpvar,
                b",\0" as *const u8 as *const libc::c_char,
                &mut saveptr,
            );
            while !skip_cmd.is_null() {
                if 0 as libc::c_int == strcmp(progname, skip_cmd) {
                    ft_mode = FT_NOOP;
                    dont_fake_final = 1 as libc::c_int != 0;
                    break;
                } else {
                    skip_cmd = strtok_r(
                        0 as *mut libc::c_char,
                        b",\0" as *const u8 as *const libc::c_char,
                        &mut saveptr,
                    );
                }
            }
            free(tmpvar as *mut libc::c_void);
            tmpvar = 0 as *mut libc::c_char;
        } else {
            fprintf(
                stderr,
                b"Error: Could not copy the environment variable value.\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    tmp_env = getenv(b"FAKETIME_ONLY_CMDS\0" as *const u8 as *const libc::c_char);
    if !tmp_env.is_null() {
        let mut only_cmd: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut saveptr_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut tmpvar_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut cmd_matched: bool = 0 as libc::c_int != 0;
        if !(getenv(b"FAKETIME_SKIP_CMDS\0" as *const u8 as *const libc::c_char))
            .is_null()
        {
            fprintf(
                stderr,
                b"Error: Both FAKETIME_SKIP_CMDS and FAKETIME_ONLY_CMDS can't be set.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        tmpvar_0 = strdup(tmp_env);
        if !tmpvar_0.is_null() {
            only_cmd = strtok_r(
                tmpvar_0,
                b",\0" as *const u8 as *const libc::c_char,
                &mut saveptr_0,
            );
            while !only_cmd.is_null() {
                if 0 as libc::c_int == strcmp(progname, only_cmd) {
                    cmd_matched = 1 as libc::c_int != 0;
                    break;
                } else {
                    only_cmd = strtok_r(
                        0 as *mut libc::c_char,
                        b",\0" as *const u8 as *const libc::c_char,
                        &mut saveptr_0,
                    );
                }
            }
            if !cmd_matched {
                ft_mode = FT_NOOP;
                dont_fake_final = 1 as libc::c_int != 0;
            }
            free(tmpvar_0 as *mut libc::c_void);
        } else {
            fprintf(
                stderr,
                b"Error: Could not copy the environment variable value.\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    tmp_env = getenv(
        b"FAKETIME_START_AFTER_SECONDS\0" as *const u8 as *const libc::c_char,
    );
    if !tmp_env.is_null() {
        ft_start_after_secs = atol(tmp_env);
        limited_faking = 1 as libc::c_int != 0;
    }
    tmp_env = getenv(
        b"FAKETIME_STOP_AFTER_SECONDS\0" as *const u8 as *const libc::c_char,
    );
    if !tmp_env.is_null() {
        ft_stop_after_secs = atol(tmp_env);
        limited_faking = 1 as libc::c_int != 0;
    }
    tmp_env = getenv(
        b"FAKETIME_START_AFTER_NUMCALLS\0" as *const u8 as *const libc::c_char,
    );
    if !tmp_env.is_null() {
        ft_start_after_ncalls = atol(tmp_env);
        limited_faking = 1 as libc::c_int != 0;
    }
    tmp_env = getenv(
        b"FAKETIME_STOP_AFTER_NUMCALLS\0" as *const u8 as *const libc::c_char,
    );
    if !tmp_env.is_null() {
        ft_stop_after_ncalls = atol(tmp_env);
        limited_faking = 1 as libc::c_int != 0;
    }
    tmp_env = getenv(b"FAKETIME_SPAWN_TARGET\0" as *const u8 as *const libc::c_char);
    if !tmp_env.is_null() {
        spawnsupport = 1 as libc::c_int != 0;
        strncpy(
            ft_spawn_target.as_mut_ptr(),
            getenv(b"FAKETIME_SPAWN_TARGET\0" as *const u8 as *const libc::c_char),
            (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        ft_spawn_target[(::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = 0 as libc::c_int as libc::c_char;
        tmp_env = getenv(
            b"FAKETIME_SPAWN_SECONDS\0" as *const u8 as *const libc::c_char,
        );
        if !tmp_env.is_null() {
            ft_spawn_secs = atol(tmp_env);
        }
        tmp_env = getenv(
            b"FAKETIME_SPAWN_NUMCALLS\0" as *const u8 as *const libc::c_char,
        );
        if !tmp_env.is_null() {
            ft_spawn_ncalls = atol(tmp_env);
        }
    }
    tmp_env = getenv(b"FAKETIME_SAVE_FILE\0" as *const u8 as *const libc::c_char);
    if !tmp_env.is_null() {
        outfile = open(
            tmp_env,
            0o2 as libc::c_int | 0o2000 as libc::c_int | 0o2000000 as libc::c_int
                | 0o100 as libc::c_int,
            0o200 as libc::c_int | 0o400 as libc::c_int,
        );
        if -(1 as libc::c_int) == outfile {
            perror(
                b"libfaketime: In ftpl_init(), opening file for saving timestamps failed\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    tmp_env = getenv(b"FAKETIME_LOAD_FILE\0" as *const u8 as *const libc::c_char);
    if !tmp_env.is_null() {
        let mut infile: libc::c_int = -(1 as libc::c_int);
        let mut sb: stat = stat {
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
        infile = open(tmp_env, 0 as libc::c_int | 0o2000000 as libc::c_int);
        if -(1 as libc::c_int) == infile {
            perror(
                b"libfaketime: In ftpl_init(), opening file for loading timestamps failed\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        fstat(infile, &mut sb);
        infile_size = sb.st_size as size_t;
        if ::std::mem::size_of::<saved_timestamp>() as libc::c_ulong > infile_size {
            printf(
                b"There are no timestamps in the provided file to load timestamps from\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        if infile_size
            .wrapping_rem(::std::mem::size_of::<saved_timestamp>() as libc::c_ulong)
            != 0 as libc::c_int as libc::c_ulong
        {
            printf(
                b"File size is not multiple of timestamp size. It is probably damaged.\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        stss = mmap(
            0 as *mut libc::c_void,
            infile_size,
            0x1 as libc::c_int,
            0x1 as libc::c_int,
            infile,
            0 as libc::c_int as __off_t,
        ) as *mut saved_timestamp;
        if stss == -(1 as libc::c_int) as *mut libc::c_void as *mut saved_timestamp {
            perror(
                b"libfaketime: In ftpl_init(), mapping file for loading timestamps failed\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        infile_set = 1 as libc::c_int != 0;
    }
    tmp_env = getenv(b"FAKETIME_FMT\0" as *const u8 as *const libc::c_char);
    if tmp_env.is_null() {
        strcpy(
            user_faked_time_fmt.as_mut_ptr(),
            b"%Y-%m-%d %T\0" as *const u8 as *const libc::c_char,
        );
    } else {
        strncpy(
            user_faked_time_fmt.as_mut_ptr(),
            tmp_env,
            (8192 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
        );
        user_faked_time_fmt[(8192 as libc::c_int - 1 as libc::c_int)
            as usize] = 0 as libc::c_int as libc::c_char;
    }
    if !shared_sem.is_null() {
        if sem_wait(shared_sem) == -(1 as libc::c_int) {
            perror(
                b"libfaketime: In ftpl_init(), sem_wait failed\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        if (*ft_shared).start_time.real.tv_nsec == -(1 as libc::c_int) as libc::c_long {
            system_time_from_system(&mut ftpl_starttime);
            (*ft_shared).start_time = ftpl_starttime;
        } else {
            ftpl_starttime = (*ft_shared).start_time;
        }
        if sem_post(shared_sem) == -(1 as libc::c_int) {
            perror(
                b"libfaketime: In ftpl_init(), sem_post failed\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    } else {
        system_time_from_system(&mut ftpl_starttime);
    }
    tmp_env = getenv(b"FAKETIME\0" as *const u8 as *const libc::c_char);
    if !tmp_env.is_null() {
        parse_config_file = 0 as libc::c_int != 0;
        parse_ft_string(tmp_env);
    } else {
        read_config_file();
    }
    dont_fake = dont_fake_final;
}
unsafe extern "C" fn prepare_config_contents(mut contents: *mut libc::c_char) {
    let mut read_position: *mut libc::c_char = contents;
    let mut write_position: *mut libc::c_char = contents;
    let mut in_comment: bool = 0 as libc::c_int != 0;
    let mut beginning_of_line: bool = 1 as libc::c_int != 0;
    while *read_position as libc::c_int != '\0' as i32 {
        if beginning_of_line as libc::c_int != 0
            && (*read_position as libc::c_int == '#' as i32
                || *read_position as libc::c_int == ';' as i32)
        {
            in_comment = 1 as libc::c_int != 0;
        }
        beginning_of_line = 0 as libc::c_int != 0;
        if *read_position as libc::c_int == '\n' as i32 {
            in_comment = 0 as libc::c_int != 0;
            beginning_of_line = 1 as libc::c_int != 0;
        }
        if !in_comment && *read_position as libc::c_int != '\r' as i32
            && *write_position as libc::c_int != '\n' as i32
        {
            *write_position = *read_position;
            write_position = write_position.offset(1);
            write_position;
        }
        read_position = read_position.offset(1);
        read_position;
    }
    *write_position = '\0' as i32 as libc::c_char;
}
#[no_mangle] pub unsafe extern "C" fn read_config_file() -> libc::c_int {
    static mut user_faked_time: [libc::c_char; 256] = [0; 256];
    static mut custom_filename: [libc::c_char; 8192] = [0; 8192];
    static mut filename: [libc::c_char; 8192] = [0; 8192];
    let mut faketimerc: libc::c_int = 0;
    snprintf(
        custom_filename.as_mut_ptr(),
        8192 as libc::c_int as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        getenv(b"FAKETIME_TIMESTAMP_FILE\0" as *const u8 as *const libc::c_char),
    );
    snprintf(
        filename.as_mut_ptr(),
        8192 as libc::c_int as libc::c_ulong,
        b"%s/.faketimerc\0" as *const u8 as *const libc::c_char,
        getenv(b"HOME\0" as *const u8 as *const libc::c_char),
    );
    faketimerc = open(custom_filename.as_mut_ptr(), 0 as libc::c_int);
    if faketimerc != -(1 as libc::c_int)
        || {
            faketimerc = open(filename.as_mut_ptr(), 0 as libc::c_int);
            faketimerc != -(1 as libc::c_int)
        }
        || {
            faketimerc = open(
                b"/etc/faketimerc\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
            faketimerc != -(1 as libc::c_int)
        }
    {
        let mut length: ssize_t = read(
            faketimerc,
            user_faked_time.as_mut_ptr() as *mut libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        close(faketimerc);
        if length < 0 as libc::c_int as libc::c_long {
            length = 0 as libc::c_int as ssize_t;
        }
        user_faked_time[length as usize] = 0 as libc::c_int as libc::c_char;
        prepare_config_contents(user_faked_time.as_mut_ptr());
        parse_ft_string(user_faked_time.as_mut_ptr());
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fake_clock_gettime(
    mut clk_id: clockid_t,
    mut tp: *mut timespec,
) -> libc::c_int {
    let mut current_ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut current_block: u64;
    static mut last_data_fetch: time_t = 0 as libc::c_int as time_t;
    static mut cache_expired: libc::c_int = 1 as libc::c_int;
    if tp.is_null() {
        return -(1 as libc::c_int);
    }
    let mut tp_save: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    tp_save.tv_sec = (*tp).tv_sec;
    tp_save.tv_nsec = (*tp).tv_nsec;
    if dont_fake {
        return 0 as libc::c_int;
    }
    if clk_id == 2 as libc::c_int || clk_id == 3 as libc::c_int {
        if user_rate_set {
            let mut tmp_time: int64_t = 0;
            tmp_time = (user_rate
                * ((*tp).tv_sec * 1000000000 as libc::c_int as libc::c_long
                    + (*tp).tv_nsec) as libc::c_double) as int64_t;
            (*tp).tv_nsec = tmp_time % 1000000000 as libc::c_int as libc::c_long;
            (*tp)
                .tv_sec = (tmp_time - (*tp).tv_nsec)
                / 1000000000 as libc::c_int as libc::c_long;
            if (*tp).tv_nsec < 0 as libc::c_int as libc::c_long {
                (*tp).tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                (*tp).tv_sec -= 1 as libc::c_int as libc::c_long;
            }
        }
        return 0 as libc::c_int;
    }
    let mut ret: libc::c_int = 2147483647 as libc::c_int;
    if limited_faking as libc::c_int != 0
        && (ft_start_after_ncalls != -(1 as libc::c_int) as libc::c_long
            || ft_stop_after_ncalls != -(1 as libc::c_int) as libc::c_long)
        || spawnsupport as libc::c_int != 0 && ft_spawn_ncalls != 0
    {
        if callcounter < 9223372036854775807 as libc::c_long {
            callcounter += 1;
            callcounter;
        }
    }
    if limited_faking as libc::c_int != 0 || spawnsupport as libc::c_int != 0 {
        let mut tmp_ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        match clk_id {
            0 | 5 => {
                tmp_ts.tv_sec = (*tp).tv_sec - ftpl_starttime.real.tv_sec;
                tmp_ts.tv_nsec = (*tp).tv_nsec - ftpl_starttime.real.tv_nsec;
                if tmp_ts.tv_nsec < 0 as libc::c_int as libc::c_long {
                    tmp_ts.tv_sec -= 1;
                    tmp_ts.tv_sec;
                    tmp_ts.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                }
            }
            1 | 6 => {
                tmp_ts.tv_sec = (*tp).tv_sec - ftpl_starttime.mon.tv_sec;
                tmp_ts.tv_nsec = (*tp).tv_nsec - ftpl_starttime.mon.tv_nsec;
                if tmp_ts.tv_nsec < 0 as libc::c_int as libc::c_long {
                    tmp_ts.tv_sec -= 1;
                    tmp_ts.tv_sec;
                    tmp_ts.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                }
            }
            4 => {
                tmp_ts.tv_sec = (*tp).tv_sec - ftpl_starttime.mon_raw.tv_sec;
                tmp_ts.tv_nsec = (*tp).tv_nsec - ftpl_starttime.mon_raw.tv_nsec;
                if tmp_ts.tv_nsec < 0 as libc::c_int as libc::c_long {
                    tmp_ts.tv_sec -= 1;
                    tmp_ts.tv_sec;
                    tmp_ts.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                }
            }
            7 => {
                tmp_ts.tv_sec = (*tp).tv_sec - ftpl_starttime.boot.tv_sec;
                tmp_ts.tv_nsec = (*tp).tv_nsec - ftpl_starttime.boot.tv_nsec;
                if tmp_ts.tv_nsec < 0 as libc::c_int as libc::c_long {
                    tmp_ts.tv_sec -= 1;
                    tmp_ts.tv_sec;
                    tmp_ts.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                }
            }
            _ => {
                tmp_ts.tv_sec = (*tp).tv_sec - ftpl_starttime.real.tv_sec;
                tmp_ts.tv_nsec = (*tp).tv_nsec - ftpl_starttime.real.tv_nsec;
                if tmp_ts.tv_nsec < 0 as libc::c_int as libc::c_long {
                    tmp_ts.tv_sec -= 1;
                    tmp_ts.tv_sec;
                    tmp_ts.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                }
            }
        }
        if limited_faking {
            if ft_start_after_secs != -(1 as libc::c_int) as libc::c_long
                && tmp_ts.tv_sec < ft_start_after_secs
                || ft_stop_after_secs != -(1 as libc::c_int) as libc::c_long
                    && tmp_ts.tv_sec >= ft_stop_after_secs
                || ft_start_after_ncalls != -(1 as libc::c_int) as libc::c_long
                    && callcounter < ft_start_after_ncalls
                || ft_stop_after_ncalls != -(1 as libc::c_int) as libc::c_long
                    && callcounter >= ft_stop_after_ncalls
            {
                ::std::ptr::write_volatile(
                    &mut ret as *mut libc::c_int,
                    0 as libc::c_int,
                );
                current_block = 4501044008137847122;
            } else {
                current_block = 3546145585875536353;
            }
        } else {
            current_block = 3546145585875536353;
        }
        match current_block {
            4501044008137847122 => {}
            _ => {
                if spawnsupport {
                    if spawned == 0 as libc::c_int {
                        if (tmp_ts.tv_sec == ft_spawn_secs
                            || callcounter == ft_spawn_ncalls)
                            && spawned == 0 as libc::c_int
                        {
                            spawned = 1 as libc::c_int;
                            system(ft_spawn_target.as_mut_ptr());
                        }
                    }
                }
                current_block = 6281126495347172768;
            }
        }
    } else {
        current_block = 6281126495347172768;
    }
    match current_block {
        6281126495347172768 => {
            current_ts = timespec { tv_sec: 0, tv_nsec: 0 };
            let mut dont_fake_orig: bool = dont_fake;
            if !dont_fake {
                dont_fake = 1 as libc::c_int != 0;
            }
            (Some(real_clock_gettime.unwrap()))
                .unwrap()(0 as libc::c_int, &mut current_ts);
            dont_fake = dont_fake_orig;
            if last_data_fetch > 0 as libc::c_int as libc::c_long {
                if current_ts.tv_sec - last_data_fetch > cache_duration as libc::c_long {
                    cache_expired = 1 as libc::c_int;
                } else {
                    cache_expired = 0 as libc::c_int;
                }
            }
            if cache_enabled == 0 as libc::c_int {
                cache_expired = 1 as libc::c_int;
            }
            if force_cache_expiration != 0 as libc::c_int {
                cache_expired = 1 as libc::c_int;
                force_cache_expiration = 0 as libc::c_int;
            }
            if cache_expired == 1 as libc::c_int {
                static mut user_faked_time: [libc::c_char; 256] = [0; 256];
                let mut tmp_env: *mut libc::c_char = 0 as *mut libc::c_char;
                tmp_env = getenv(b"FAKETIME\0" as *const u8 as *const libc::c_char);
                if !tmp_env.is_null() {
                    strncpy(
                        user_faked_time.as_mut_ptr(),
                        tmp_env,
                        (256 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
                    );
                    user_faked_time[(256 as libc::c_int - 1 as libc::c_int)
                        as usize] = 0 as libc::c_int as libc::c_char;
                } else {
                    snprintf(
                        user_faked_time.as_mut_ptr(),
                        256 as libc::c_int as libc::c_ulong,
                        b"+0\0" as *const u8 as *const libc::c_char,
                    );
                }
                last_data_fetch = current_ts.tv_sec;
                if parse_config_file {
                    if read_config_file() == 0 as libc::c_int {
                        parse_ft_string(user_faked_time.as_mut_ptr());
                    }
                } else {
                    parse_ft_string(user_faked_time.as_mut_ptr());
                }
                get_fake_monotonic_setting(&mut fake_monotonic_clock);
            }
            if infile_set {
                if load_time(tp) {
                    ::std::ptr::write_volatile(
                        &mut ret as *mut libc::c_int,
                        0 as libc::c_int,
                    );
                    current_block = 4501044008137847122;
                } else {
                    current_block = 2705889988320590074;
                }
            } else {
                current_block = 2705889988320590074;
            }
            match current_block {
                4501044008137847122 => {}
                _ => {
                    match ft_mode as libc::c_uint {
                        0 => {
                            if user_faked_time_set {
                                *tp = user_faked_time_timespec;
                            }
                        }
                        1 => {
                            if user_per_tick_inc_set {
                                next_time(tp, &mut user_per_tick_inc);
                            } else {
                                let mut tdiff: timespec = timespec {
                                    tv_sec: 0,
                                    tv_nsec: 0,
                                };
                                let mut timeadj: timespec = timespec {
                                    tv_sec: 0,
                                    tv_nsec: 0,
                                };
                                match clk_id {
                                    0 | 5 => {
                                        tdiff.tv_sec = (*tp).tv_sec - ftpl_starttime.real.tv_sec;
                                        tdiff.tv_nsec = (*tp).tv_nsec - ftpl_starttime.real.tv_nsec;
                                        if tdiff.tv_nsec < 0 as libc::c_int as libc::c_long {
                                            tdiff.tv_sec -= 1;
                                            tdiff.tv_sec;
                                            tdiff.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                                        }
                                    }
                                    1 | 6 => {
                                        tdiff.tv_sec = (*tp).tv_sec - ftpl_starttime.mon.tv_sec;
                                        tdiff.tv_nsec = (*tp).tv_nsec - ftpl_starttime.mon.tv_nsec;
                                        if tdiff.tv_nsec < 0 as libc::c_int as libc::c_long {
                                            tdiff.tv_sec -= 1;
                                            tdiff.tv_sec;
                                            tdiff.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                                        }
                                    }
                                    4 => {
                                        tdiff.tv_sec = (*tp).tv_sec - ftpl_starttime.mon_raw.tv_sec;
                                        tdiff
                                            .tv_nsec = (*tp).tv_nsec - ftpl_starttime.mon_raw.tv_nsec;
                                        if tdiff.tv_nsec < 0 as libc::c_int as libc::c_long {
                                            tdiff.tv_sec -= 1;
                                            tdiff.tv_sec;
                                            tdiff.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                                        }
                                    }
                                    7 => {
                                        tdiff.tv_sec = (*tp).tv_sec - ftpl_starttime.boot.tv_sec;
                                        tdiff.tv_nsec = (*tp).tv_nsec - ftpl_starttime.boot.tv_nsec;
                                        if tdiff.tv_nsec < 0 as libc::c_int as libc::c_long {
                                            tdiff.tv_sec -= 1;
                                            tdiff.tv_sec;
                                            tdiff.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                                        }
                                    }
                                    _ => {
                                        tdiff.tv_sec = (*tp).tv_sec - ftpl_starttime.real.tv_sec;
                                        tdiff.tv_nsec = (*tp).tv_nsec - ftpl_starttime.real.tv_nsec;
                                        if tdiff.tv_nsec < 0 as libc::c_int as libc::c_long {
                                            tdiff.tv_sec -= 1;
                                            tdiff.tv_sec;
                                            tdiff.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                                        }
                                    }
                                }
                                if user_rate_set {
                                    let mut tmp_time_0: int64_t = 0;
                                    tmp_time_0 = (user_rate
                                        * (tdiff.tv_sec * 1000000000 as libc::c_int as libc::c_long
                                            + tdiff.tv_nsec) as libc::c_double) as int64_t;
                                    timeadj
                                        .tv_nsec = tmp_time_0
                                        % 1000000000 as libc::c_int as libc::c_long;
                                    timeadj
                                        .tv_sec = (tmp_time_0 - timeadj.tv_nsec)
                                        / 1000000000 as libc::c_int as libc::c_long;
                                    if timeadj.tv_nsec < 0 as libc::c_int as libc::c_long {
                                        timeadj.tv_nsec
                                            += 1000000000 as libc::c_int as libc::c_long;
                                        timeadj.tv_sec -= 1 as libc::c_int as libc::c_long;
                                    }
                                } else {
                                    timeadj = tdiff;
                                }
                                (*tp)
                                    .tv_sec = user_faked_time_timespec.tv_sec + timeadj.tv_sec;
                                (*tp)
                                    .tv_nsec = user_faked_time_timespec.tv_nsec
                                    + timeadj.tv_nsec;
                                if (*tp).tv_nsec
                                    >= 1000000000 as libc::c_int as libc::c_long
                                {
                                    (*tp).tv_sec += 1;
                                    (*tp).tv_sec;
                                    (*tp).tv_nsec -= 1000000000 as libc::c_int as libc::c_long;
                                }
                            }
                        }
                        _ => {
                            ::std::ptr::write_volatile(
                                &mut ret as *mut libc::c_int,
                                -(1 as libc::c_int),
                            );
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if ret != 2147483647 as libc::c_int {
        return ret;
    }
    save_time(tp);
    if clk_id == 0 as libc::c_int {
        ftpl_timecache.real.tv_sec = tp_save.tv_sec;
        ftpl_timecache.real.tv_nsec = tp_save.tv_nsec;
        ftpl_faketimecache.real.tv_sec = (*tp).tv_sec;
        ftpl_faketimecache.real.tv_nsec = (*tp).tv_nsec;
    } else if clk_id == 1 as libc::c_int {
        ftpl_timecache.mon.tv_sec = tp_save.tv_sec;
        ftpl_timecache.mon.tv_nsec = tp_save.tv_nsec;
        ftpl_faketimecache.mon.tv_sec = (*tp).tv_sec;
        ftpl_faketimecache.mon.tv_nsec = (*tp).tv_nsec;
    } else if clk_id == 4 as libc::c_int {
        ftpl_timecache.mon_raw.tv_sec = tp_save.tv_sec;
        ftpl_timecache.mon_raw.tv_nsec = tp_save.tv_nsec;
        ftpl_faketimecache.mon_raw.tv_sec = (*tp).tv_sec;
        ftpl_faketimecache.mon_raw.tv_nsec = (*tp).tv_nsec;
    } else if clk_id == 7 as libc::c_int {
        ftpl_timecache.boot.tv_sec = tp_save.tv_sec;
        ftpl_timecache.boot.tv_nsec = tp_save.tv_nsec;
        ftpl_faketimecache.boot.tv_sec = (*tp).tv_sec;
        ftpl_faketimecache.boot.tv_nsec = (*tp).tv_nsec;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fake_gettimeofday(mut tv: *mut timeval) -> libc::c_int {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut ret: libc::c_int = 0;
    ts.tv_sec = (*tv).tv_sec;
    ts
        .tv_nsec = (*tv).tv_usec * 1000 as libc::c_int as libc::c_long
        + ftpl_starttime.real.tv_nsec % 1000 as libc::c_int as libc::c_long;
    ret = fake_clock_gettime(0 as libc::c_int, &mut ts);
    (*tv).tv_sec = ts.tv_sec;
    (*tv).tv_usec = ts.tv_nsec / 1000 as libc::c_int as libc::c_long;
    return ret;
}
#[no_mangle] pub unsafe extern "C" fn __gettimeofday(
    mut tv: *mut timeval,
    mut tz: *mut libc::c_void,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    if tv.is_null() {
        return -(1 as libc::c_int);
    }
    if !check_missing_real(
        b"__gettimeofday\0" as *const u8 as *const libc::c_char,
        real___gettimeofday.is_none(),
    ) {
        return -(1 as libc::c_int);
    }
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = (Some(real___gettimeofday.unwrap())).unwrap()(tv, tz);
    dont_fake = dont_fake_orig;
    if result == -(1 as libc::c_int) {
        return result;
    }
    result = fake_gettimeofday(tv);
    return result;
}
#[no_mangle] pub unsafe extern "C" fn __clock_gettime(
    mut clk_id: clockid_t,
    mut tp: *mut timespec,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    if tp.is_null() {
        return -(1 as libc::c_int);
    }
    if !check_missing_real(
        b"__clock_gettime\0" as *const u8 as *const libc::c_char,
        real___clock_gettime.is_none(),
    ) {
        return -(1 as libc::c_int);
    }
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = (Some(real___clock_gettime.unwrap())).unwrap()(clk_id, tp);
    dont_fake = dont_fake_orig;
    if result == -(1 as libc::c_int) {
        return result;
    }
    if fake_monotonic_clock != 0
        || clk_id != 1 as libc::c_int && clk_id != 4 as libc::c_int
            && clk_id != 6 as libc::c_int && clk_id != 7 as libc::c_int
    {
        result = fake_clock_gettime(clk_id, tp);
    }
    return result;
}
#[no_mangle] pub unsafe extern "C" fn __time(mut time_tptr: *mut time_t) -> time_t {
    let mut tp: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut result: time_t = 0;
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = (Some(real_clock_gettime.unwrap())).unwrap()(0 as libc::c_int, &mut tp)
        as time_t;
    dont_fake = dont_fake_orig;
    if result == -(1 as libc::c_int) as libc::c_long {
        return -(1 as libc::c_int) as time_t;
    }
    fake_clock_gettime(0 as libc::c_int, &mut tp);
    if !time_tptr.is_null() {
        *time_tptr = tp.tv_sec;
    }
    return tp.tv_sec;
}
#[no_mangle] pub unsafe extern "C" fn __ftime(mut tb: *mut timeb) -> libc::c_int {
    let mut tp: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut result: libc::c_int = 0;
    if tb.is_null() {
        return 0 as libc::c_int;
    }
    if !check_missing_real(
        b"__ftime\0" as *const u8 as *const libc::c_char,
        real___ftime.is_none(),
    ) {
        return 0 as libc::c_int;
    }
    let mut dont_fake_orig: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = (Some(real___ftime.unwrap())).unwrap()(tb);
    dont_fake = dont_fake_orig;
    if result == -(1 as libc::c_int) {
        return result;
    }
    let mut dont_fake_orig_0: bool = dont_fake;
    if !dont_fake {
        dont_fake = 1 as libc::c_int != 0;
    }
    result = (Some(real_clock_gettime.unwrap())).unwrap()(0 as libc::c_int, &mut tp);
    dont_fake = dont_fake_orig_0;
    if result == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    fake_clock_gettime(0 as libc::c_int, &mut tp);
    (*tb).time = tp.tv_sec;
    (*tb)
        .millitm = (tp.tv_nsec / 1000000 as libc::c_int as libc::c_long)
        as libc::c_ushort;
    return result;
}
static mut monotonic_conds: *mut pthread_cond_monotonic = 0
    as *const pthread_cond_monotonic as *mut pthread_cond_monotonic;
#[no_mangle] pub unsafe extern "C" fn pthread_cond_init_232(
    mut cond: *mut pthread_cond_t,
    mut attr: *const pthread_condattr_t,
) -> libc::c_int {
    let mut clock_id: clockid_t = 0;
    let mut result: libc::c_int = 0;
    if initialized == 0 {
        ftpl_init();
    }
    if !check_missing_real(
        b"pthread_cond_init_232\0" as *const u8 as *const libc::c_char,
        real_pthread_cond_init_232.is_none(),
    ) {
        return -(1 as libc::c_int);
    }
    result = real_pthread_cond_init_232.unwrap()(cond, attr);
    if result != 0 as libc::c_int || attr.is_null() {
        return result;
    }
    pthread_condattr_getclock(attr, &mut clock_id);
    if clock_id == 1 as libc::c_int {
        let mut e: *mut pthread_cond_monotonic = malloc(
            ::std::mem::size_of::<pthread_cond_monotonic>() as libc::c_ulong,
        ) as *mut pthread_cond_monotonic;
        (*e).ptr = cond;
        if pthread_rwlock_wrlock(&mut monotonic_conds_lock) != 0 as libc::c_int {
            fprintf(
                stderr,
                b"can't acquire write monotonic_conds_lock\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(-(1 as libc::c_int));
        }
        let mut _ha_hashv: libc::c_uint = 0;
        let mut _hj_i: libc::c_uint = 0;
        let mut _hj_j: libc::c_uint = 0;
        let mut _hj_k: libc::c_uint = 0;
        let mut _hj_key: *const libc::c_uchar = &mut (*e).ptr as *mut *mut pthread_cond_t
            as *const libc::c_uchar;
        _ha_hashv = 0xfeedbeef as libc::c_uint;
        _hj_j = 0x9e3779b9 as libc::c_uint;
        _hj_i = _hj_j;
        _hj_k = ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            as libc::c_uint;
        while _hj_k >= 12 as libc::c_uint {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        ),
                );
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        ),
                );
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(11 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        ),
                );
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_ha_hashv);
            _hj_i ^= _ha_hashv >> 13 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_ha_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 8 as libc::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
            _ha_hashv ^= _hj_j >> 13 as libc::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_ha_hashv);
            _hj_i ^= _ha_hashv >> 12 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_ha_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 16 as libc::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
            _ha_hashv ^= _hj_j >> 5 as libc::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_ha_hashv);
            _hj_i ^= _ha_hashv >> 3 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_ha_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 10 as libc::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
            _ha_hashv ^= _hj_j >> 15 as libc::c_int;
            _hj_key = _hj_key.offset(12 as libc::c_int as isize);
            _hj_k = _hj_k.wrapping_sub(12 as libc::c_uint);
        }
        _ha_hashv = _ha_hashv
            .wrapping_add(
                ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    as libc::c_uint,
            );
        let mut current_block_64: u64;
        match _hj_k {
            11 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_64 = 15133938232545819288;
            }
            10 => {
                current_block_64 = 15133938232545819288;
            }
            9 => {
                current_block_64 = 14161415853371003312;
            }
            8 => {
                current_block_64 = 6101387614174675360;
            }
            7 => {
                current_block_64 = 13578152789532068866;
            }
            6 => {
                current_block_64 = 6290448941148516191;
            }
            5 => {
                current_block_64 = 8520576854503709470;
            }
            4 => {
                current_block_64 = 3412936076835973157;
            }
            3 => {
                current_block_64 = 9281738827997295735;
            }
            2 => {
                current_block_64 = 9530150790249184898;
            }
            1 => {
                current_block_64 = 17804070343020517427;
            }
            _ => {
                current_block_64 = 5892776923941496671;
            }
        }
        match current_block_64 {
            15133938232545819288 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_64 = 14161415853371003312;
            }
            _ => {}
        }
        match current_block_64 {
            14161415853371003312 => {
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_64 = 6101387614174675360;
            }
            _ => {}
        }
        match current_block_64 {
            6101387614174675360 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_64 = 13578152789532068866;
            }
            _ => {}
        }
        match current_block_64 {
            13578152789532068866 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_64 = 6290448941148516191;
            }
            _ => {}
        }
        match current_block_64 {
            6290448941148516191 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_64 = 8520576854503709470;
            }
            _ => {}
        }
        match current_block_64 {
            8520576854503709470 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_64 = 3412936076835973157;
            }
            _ => {}
        }
        match current_block_64 {
            3412936076835973157 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_64 = 9281738827997295735;
            }
            _ => {}
        }
        match current_block_64 {
            9281738827997295735 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_64 = 9530150790249184898;
            }
            _ => {}
        }
        match current_block_64 {
            9530150790249184898 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_64 = 17804070343020517427;
            }
            _ => {}
        }
        match current_block_64 {
            17804070343020517427 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        *_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint,
                    );
            }
            _ => {}
        }
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_ha_hashv);
        _hj_i ^= _ha_hashv >> 13 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_ha_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as libc::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
        _ha_hashv ^= _hj_j >> 13 as libc::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_ha_hashv);
        _hj_i ^= _ha_hashv >> 12 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_ha_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as libc::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
        _ha_hashv ^= _hj_j >> 5 as libc::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_ha_hashv);
        _hj_i ^= _ha_hashv >> 3 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_ha_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as libc::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j);
        _ha_hashv ^= _hj_j >> 15 as libc::c_int;
        (*e).hh.hashv = _ha_hashv;
        (*e)
            .hh
            .key = &mut (*e).ptr as *mut *mut pthread_cond_t as *mut libc::c_char
            as *mut libc::c_void;
        (*e)
            .hh
            .keylen = ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            as libc::c_uint;
        if monotonic_conds.is_null() {
            (*e).hh.next = 0 as *mut libc::c_void;
            (*e).hh.prev = 0 as *mut libc::c_void;
            (*e)
                .hh
                .tbl = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong)
                as *mut UT_hash_table;
            if ((*e).hh.tbl).is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    (*e).hh.tbl as *mut libc::c_void,
                    '\0' as i32,
                    ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                );
                (*(*e).hh.tbl).tail = &mut (*e).hh;
                (*(*e).hh.tbl).num_buckets = 32 as libc::c_uint;
                (*(*e).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
                (*(*e).hh.tbl)
                    .hho = (&mut (*e).hh as *mut UT_hash_handle as *mut libc::c_char)
                    .offset_from(e as *mut libc::c_char) as libc::c_long;
                (*(*e).hh.tbl)
                    .buckets = malloc(
                    (32 as libc::c_uint as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                ) as *mut UT_hash_bucket;
                (*(*e).hh.tbl).signature = 0xa0111fe1 as libc::c_uint;
                if ((*(*e).hh.tbl).buckets).is_null() {
                    exit(-(1 as libc::c_int));
                } else {
                    memset(
                        (*(*e).hh.tbl).buckets as *mut libc::c_void,
                        '\0' as i32,
                        (32 as libc::c_uint as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                            ),
                    );
                }
            }
            monotonic_conds = e;
        } else {
            (*e).hh.tbl = (*monotonic_conds).hh.tbl;
            (*e).hh.next = 0 as *mut libc::c_void;
            (*e)
                .hh
                .prev = ((*(*monotonic_conds).hh.tbl).tail as *mut libc::c_char)
                .offset(-((*(*monotonic_conds).hh.tbl).hho as isize))
                as *mut libc::c_void;
            (*(*(*monotonic_conds).hh.tbl).tail).next = e as *mut libc::c_void;
            (*(*monotonic_conds).hh.tbl).tail = &mut (*e).hh;
        }
        let mut _ha_bkt: libc::c_uint = 0;
        (*(*monotonic_conds).hh.tbl)
            .num_items = ((*(*monotonic_conds).hh.tbl).num_items).wrapping_add(1);
        (*(*monotonic_conds).hh.tbl).num_items;
        _ha_bkt = _ha_hashv
            & ((*(*monotonic_conds).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        let mut _ha_head: *mut UT_hash_bucket = &mut *((*(*monotonic_conds).hh.tbl)
            .buckets)
            .offset(_ha_bkt as isize) as *mut UT_hash_bucket;
        (*_ha_head).count = ((*_ha_head).count).wrapping_add(1);
        (*_ha_head).count;
        (*e).hh.hh_next = (*_ha_head).hh_head;
        (*e).hh.hh_prev = 0 as *mut UT_hash_handle;
        if !((*_ha_head).hh_head).is_null() {
            (*(*_ha_head).hh_head).hh_prev = &mut (*e).hh;
        }
        (*_ha_head).hh_head = &mut (*e).hh;
        if (*_ha_head).count
            >= ((*_ha_head).expand_mult)
                .wrapping_add(1 as libc::c_uint)
                .wrapping_mul(10 as libc::c_uint) && (*(*e).hh.tbl).noexpand == 0
        {
            let mut _he_bkt: libc::c_uint = 0;
            let mut _he_bkt_i: libc::c_uint = 0;
            let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
            let mut _he_new_buckets: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
            _he_new_buckets = malloc(
                (2 as libc::c_ulong)
                    .wrapping_mul((*(*e).hh.tbl).num_buckets as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as libc::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut libc::c_void,
                    '\0' as i32,
                    (2 as libc::c_ulong)
                        .wrapping_mul((*(*e).hh.tbl).num_buckets as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                        ),
                );
                (*(*e).hh.tbl)
                    .ideal_chain_maxlen = ((*(*e).hh.tbl).num_items
                    >> ((*(*e).hh.tbl).log2_num_buckets).wrapping_add(1 as libc::c_uint))
                    .wrapping_add(
                        (if (*(*e).hh.tbl).num_items
                            & ((*(*e).hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                        {
                            1 as libc::c_uint
                        } else {
                            0 as libc::c_uint
                        }),
                    );
                (*(*e).hh.tbl).nonideal_items = 0 as libc::c_int as libc::c_uint;
                _he_bkt_i = 0 as libc::c_int as libc::c_uint;
                while _he_bkt_i < (*(*e).hh.tbl).num_buckets {
                    _he_thh = (*((*(*e).hh.tbl).buckets).offset(_he_bkt_i as isize))
                        .hh_head;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & ((*(*e).hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint);
                        _he_newbkt = &mut *_he_new_buckets.offset(_he_bkt as isize)
                            as *mut UT_hash_bucket;
                        (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                        if (*_he_newbkt).count > (*(*e).hh.tbl).ideal_chain_maxlen {
                            (*(*e).hh.tbl)
                                .nonideal_items = ((*(*e).hh.tbl).nonideal_items)
                                .wrapping_add(1);
                            (*(*e).hh.tbl).nonideal_items;
                            (*_he_newbkt)
                                .expand_mult = ((*_he_newbkt).count)
                                .wrapping_div((*(*e).hh.tbl).ideal_chain_maxlen);
                        }
                        (*_he_thh).hh_prev = 0 as *mut UT_hash_handle;
                        (*_he_thh).hh_next = (*_he_newbkt).hh_head;
                        if !((*_he_newbkt).hh_head).is_null() {
                            (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                        }
                        (*_he_newbkt).hh_head = _he_thh;
                        _he_thh = _he_hh_nxt;
                    }
                    _he_bkt_i = _he_bkt_i.wrapping_add(1);
                    _he_bkt_i;
                }
                free((*(*e).hh.tbl).buckets as *mut libc::c_void);
                (*(*e).hh.tbl)
                    .num_buckets = ((*(*e).hh.tbl).num_buckets)
                    .wrapping_mul(2 as libc::c_uint);
                (*(*e).hh.tbl)
                    .log2_num_buckets = ((*(*e).hh.tbl).log2_num_buckets)
                    .wrapping_add(1);
                (*(*e).hh.tbl).log2_num_buckets;
                (*(*e).hh.tbl).buckets = _he_new_buckets;
                (*(*e).hh.tbl)
                    .ineff_expands = if (*(*e).hh.tbl).nonideal_items
                    > (*(*e).hh.tbl).num_items >> 1 as libc::c_int
                {
                    ((*(*e).hh.tbl).ineff_expands).wrapping_add(1 as libc::c_uint)
                } else {
                    0 as libc::c_uint
                };
                if (*(*e).hh.tbl).ineff_expands > 1 as libc::c_uint {
                    (*(*e).hh.tbl).noexpand = 1 as libc::c_int as libc::c_uint;
                }
            }
        }
        pthread_rwlock_unlock(&mut monotonic_conds_lock);
    }
    return result;
}
#[no_mangle] pub unsafe extern "C" fn pthread_cond_destroy_232(
    mut cond: *mut pthread_cond_t,
) -> libc::c_int {
    let mut e: *mut pthread_cond_monotonic = 0 as *mut pthread_cond_monotonic;
    if pthread_rwlock_wrlock(&mut monotonic_conds_lock) != 0 as libc::c_int {
        fprintf(
            stderr,
            b"can't acquire write monotonic_conds_lock\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    let mut _hf_hashv: libc::c_uint = 0;
    let mut _hj_i: libc::c_uint = 0;
    let mut _hj_j: libc::c_uint = 0;
    let mut _hj_k: libc::c_uint = 0;
    let mut _hj_key: *const libc::c_uchar = &mut cond as *mut *mut pthread_cond_t
        as *const libc::c_uchar;
    _hf_hashv = 0xfeedbeef as libc::c_uint;
    _hj_j = 0x9e3779b9 as libc::c_uint;
    _hj_i = _hj_j;
    _hj_k = ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as libc::c_uint;
    while _hj_k >= 12 as libc::c_uint {
        _hj_i = _hj_i
            .wrapping_add(
                (*_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _hj_j = _hj_j
            .wrapping_add(
                (*_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _hf_hashv = _hf_hashv
            .wrapping_add(
                (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(11 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 13 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 13 as libc::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 12 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 5 as libc::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 3 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 15 as libc::c_int;
        _hj_key = _hj_key.offset(12 as libc::c_int as isize);
        _hj_k = _hj_k.wrapping_sub(12 as libc::c_uint);
    }
    _hf_hashv = _hf_hashv
        .wrapping_add(
            ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as libc::c_uint,
        );
    let mut current_block_54: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv
                .wrapping_add(
                    (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_54 = 7949765165458945995;
        }
        10 => {
            current_block_54 = 7949765165458945995;
        }
        9 => {
            current_block_54 = 16157078313657245745;
        }
        8 => {
            current_block_54 = 18435508912408360646;
        }
        7 => {
            current_block_54 = 4843484421140599062;
        }
        6 => {
            current_block_54 = 205748028837621170;
        }
        5 => {
            current_block_54 = 3077332263009337644;
        }
        4 => {
            current_block_54 = 11712453467413961974;
        }
        3 => {
            current_block_54 = 5670779790647566325;
        }
        2 => {
            current_block_54 = 10052401752756963827;
        }
        1 => {
            current_block_54 = 10324068642467916555;
        }
        _ => {
            current_block_54 = 3938820862080741272;
        }
    }
    match current_block_54 {
        7949765165458945995 => {
            _hf_hashv = _hf_hashv
                .wrapping_add(
                    (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_54 = 16157078313657245745;
        }
        _ => {}
    }
    match current_block_54 {
        16157078313657245745 => {
            _hf_hashv = _hf_hashv
                .wrapping_add(
                    (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_54 = 18435508912408360646;
        }
        _ => {}
    }
    match current_block_54 {
        18435508912408360646 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_54 = 4843484421140599062;
        }
        _ => {}
    }
    match current_block_54 {
        4843484421140599062 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_54 = 205748028837621170;
        }
        _ => {}
    }
    match current_block_54 {
        205748028837621170 => {
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_54 = 3077332263009337644;
        }
        _ => {}
    }
    match current_block_54 {
        3077332263009337644 => {
            _hj_j = _hj_j
                .wrapping_add(
                    *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_54 = 11712453467413961974;
        }
        _ => {}
    }
    match current_block_54 {
        11712453467413961974 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_54 = 5670779790647566325;
        }
        _ => {}
    }
    match current_block_54 {
        5670779790647566325 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_54 = 10052401752756963827;
        }
        _ => {}
    }
    match current_block_54 {
        10052401752756963827 => {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_54 = 10324068642467916555;
        }
        _ => {}
    }
    match current_block_54 {
        10324068642467916555 => {
            _hj_i = _hj_i
                .wrapping_add(
                    *_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint,
                );
        }
        _ => {}
    }
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
    _hj_i ^= _hf_hashv >> 13 as libc::c_int;
    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 8 as libc::c_int;
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
    _hf_hashv ^= _hj_j >> 13 as libc::c_int;
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
    _hj_i ^= _hf_hashv >> 12 as libc::c_int;
    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 16 as libc::c_int;
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
    _hf_hashv ^= _hj_j >> 5 as libc::c_int;
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
    _hj_i ^= _hf_hashv >> 3 as libc::c_int;
    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 10 as libc::c_int;
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
    _hf_hashv ^= _hj_j >> 15 as libc::c_int;
    e = 0 as *mut pthread_cond_monotonic;
    if !monotonic_conds.is_null() {
        let mut _hf_bkt: libc::c_uint = 0;
        _hf_bkt = _hf_hashv
            & ((*(*monotonic_conds).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        if 1 as libc::c_int != 0 as libc::c_int {
            if !((*((*(*monotonic_conds).hh.tbl).buckets).offset(_hf_bkt as isize))
                .hh_head)
                .is_null()
            {
                e = ((*((*(*monotonic_conds).hh.tbl).buckets).offset(_hf_bkt as isize))
                    .hh_head as *mut libc::c_char)
                    .offset(-((*(*monotonic_conds).hh.tbl).hho as isize))
                    as *mut libc::c_void as *mut pthread_cond_monotonic;
            } else {
                e = 0 as *mut pthread_cond_monotonic;
            }
            while !e.is_null() {
                if (*e).hh.hashv == _hf_hashv
                    && (*e).hh.keylen as libc::c_ulong
                        == ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                {
                    if memcmp(
                        (*e).hh.key,
                        &mut cond as *mut *mut pthread_cond_t as *const libc::c_void,
                        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        break;
                    }
                }
                if !((*e).hh.hh_next).is_null() {
                    e = ((*e).hh.hh_next as *mut libc::c_char)
                        .offset(-((*(*monotonic_conds).hh.tbl).hho as isize))
                        as *mut libc::c_void as *mut pthread_cond_monotonic;
                } else {
                    e = 0 as *mut pthread_cond_monotonic;
                }
            }
        }
    }
    if !e.is_null() {
        let mut _hd_hh_del: *mut UT_hash_handle = &mut (*e).hh;
        if ((*_hd_hh_del).prev).is_null() && ((*_hd_hh_del).next).is_null() {
            free((*(*monotonic_conds).hh.tbl).buckets as *mut libc::c_void);
            free((*monotonic_conds).hh.tbl as *mut libc::c_void);
            monotonic_conds = 0 as *mut pthread_cond_monotonic;
        } else {
            let mut _hd_bkt: libc::c_uint = 0;
            if _hd_hh_del == (*(*monotonic_conds).hh.tbl).tail {
                (*(*monotonic_conds).hh.tbl)
                    .tail = ((*_hd_hh_del).prev as *mut libc::c_char)
                    .offset((*(*monotonic_conds).hh.tbl).hho as isize)
                    as *mut UT_hash_handle;
            }
            if !((*_hd_hh_del).prev).is_null() {
                let ref mut fresh6 = (*(((*_hd_hh_del).prev as *mut libc::c_char)
                    .offset((*(*monotonic_conds).hh.tbl).hho as isize)
                    as *mut UT_hash_handle))
                    .next;
                *fresh6 = (*_hd_hh_del).next;
            } else {
                monotonic_conds = (*_hd_hh_del).next as *mut pthread_cond_monotonic;
            }
            if !((*_hd_hh_del).next).is_null() {
                let ref mut fresh7 = (*(((*_hd_hh_del).next as *mut libc::c_char)
                    .offset((*(*monotonic_conds).hh.tbl).hho as isize)
                    as *mut UT_hash_handle))
                    .prev;
                *fresh7 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & ((*(*monotonic_conds).hh.tbl).num_buckets)
                    .wrapping_sub(1 as libc::c_uint);
            let mut _hd_head: *mut UT_hash_bucket = &mut *((*(*monotonic_conds).hh.tbl)
                .buckets)
                .offset(_hd_bkt as isize) as *mut UT_hash_bucket;
            (*_hd_head).count = ((*_hd_head).count).wrapping_sub(1);
            (*_hd_head).count;
            if (*_hd_head).hh_head == _hd_hh_del {
                (*_hd_head).hh_head = (*_hd_hh_del).hh_next;
            }
            if !((*_hd_hh_del).hh_prev).is_null() {
                (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
            }
            if !((*_hd_hh_del).hh_next).is_null() {
                (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
            }
            (*(*monotonic_conds).hh.tbl)
                .num_items = ((*(*monotonic_conds).hh.tbl).num_items).wrapping_sub(1);
            (*(*monotonic_conds).hh.tbl).num_items;
        }
        free(e as *mut libc::c_void);
    }
    pthread_rwlock_unlock(&mut monotonic_conds_lock);
    return real_pthread_cond_destroy_232.unwrap()(cond);
}
#[no_mangle] pub unsafe extern "C" fn needs_forced_monotonic_fix(
    mut function_name: *mut libc::c_char,
) -> bool {
    let mut result: bool = 0 as libc::c_int != 0;
    let mut env_var: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut glibc_version_string: *const libc::c_char = gnu_get_libc_version();
    if function_name.is_null() {
        return 0 as libc::c_int != 0;
    }
    env_var = getenv(
        b"FAKETIME_FORCE_MONOTONIC_FIX\0" as *const u8 as *const libc::c_char,
    );
    if !env_var.is_null() {
        if *env_var.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32 {
            result = 0 as libc::c_int != 0;
        } else {
            result = 1 as libc::c_int != 0;
        }
    } else {
        let mut glibc_major: libc::c_int = 0;
        let mut glibc_minor: libc::c_int = 0;
        sscanf(
            glibc_version_string,
            b"%d.%d\0" as *const u8 as *const libc::c_char,
            &mut glibc_major as *mut libc::c_int,
            &mut glibc_minor as *mut libc::c_int,
        );
        if glibc_major == 2 as libc::c_int
            && (glibc_minor <= 24 as libc::c_int || glibc_minor >= 30 as libc::c_int)
        {
            result = 1 as libc::c_int != 0;
        } else {
            result = 0 as libc::c_int != 0;
        }
    }
    if !(getenv(b"FAKETIME_DEBUG\0" as *const u8 as *const libc::c_char)).is_null() {
        fprintf(
            stderr,
            b"libfaketime: forced monotonic fix for %s = %s (glibc version %s)\n\0"
                as *const u8 as *const libc::c_char,
            function_name,
            if result as libc::c_int != 0 {
                b"yes\0" as *const u8 as *const libc::c_char
            } else {
                b"no\0" as *const u8 as *const libc::c_char
            },
            glibc_version_string,
        );
    }
    return result;
}
#[no_mangle] pub unsafe extern "C" fn pthread_cond_timedwait_common(
    mut cond: *mut pthread_cond_t,
    mut mutex: *mut pthread_mutex_t,
    mut abstime: *const timespec,
    mut compat: ft_lib_compat_pthread,
) -> libc::c_int {
    let mut tp: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut tdiff_actual: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut realtime: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut faketime: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut tf: *mut timespec = 0 as *mut timespec;
    let mut e: *mut pthread_cond_monotonic = 0 as *mut pthread_cond_monotonic;
    let mut tmp_env: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wait_ms: libc::c_int = 0;
    let mut clk_id: clockid_t = 0;
    let mut result: libc::c_int = 0 as libc::c_int;
    if !abstime.is_null() {
        if pthread_rwlock_rdlock(&mut monotonic_conds_lock) != 0 as libc::c_int {
            fprintf(
                stderr,
                b"can't acquire read monotonic_conds_lock\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(-(1 as libc::c_int));
        }
        let mut _hf_hashv: libc::c_uint = 0;
        let mut _hj_i: libc::c_uint = 0;
        let mut _hj_j: libc::c_uint = 0;
        let mut _hj_k: libc::c_uint = 0;
        let mut _hj_key: *const libc::c_uchar = &mut cond as *mut *mut pthread_cond_t
            as *const libc::c_uchar;
        _hf_hashv = 0xfeedbeef as libc::c_uint;
        _hj_j = 0x9e3779b9 as libc::c_uint;
        _hj_i = _hj_j;
        _hj_k = ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            as libc::c_uint;
        while _hj_k >= 12 as libc::c_uint {
            _hj_i = _hj_i
                .wrapping_add(
                    (*_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        ),
                );
            _hj_j = _hj_j
                .wrapping_add(
                    (*_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        ),
                );
            _hf_hashv = _hf_hashv
                .wrapping_add(
                    (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_add(
                            (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(11 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        ),
                );
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 13 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 8 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 13 as libc::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 12 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 16 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 5 as libc::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 3 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 10 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 15 as libc::c_int;
            _hj_key = _hj_key.offset(12 as libc::c_int as isize);
            _hj_k = _hj_k.wrapping_sub(12 as libc::c_uint);
        }
        _hf_hashv = _hf_hashv
            .wrapping_add(
                ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    as libc::c_uint,
            );
        let mut current_block_54: u64;
        match _hj_k {
            11 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_54 = 18148774039771602297;
            }
            10 => {
                current_block_54 = 18148774039771602297;
            }
            9 => {
                current_block_54 = 10036371062546261272;
            }
            8 => {
                current_block_54 = 14399816886812208938;
            }
            7 => {
                current_block_54 = 1080332908278044257;
            }
            6 => {
                current_block_54 = 18072476671732897849;
            }
            5 => {
                current_block_54 = 3892783650257935544;
            }
            4 => {
                current_block_54 = 5206991630100269981;
            }
            3 => {
                current_block_54 = 2431160423376578768;
            }
            2 => {
                current_block_54 = 9021970710478294852;
            }
            1 => {
                current_block_54 = 11620144116472646757;
            }
            _ => {
                current_block_54 = 10399321362245223758;
            }
        }
        match current_block_54 {
            18148774039771602297 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_54 = 10036371062546261272;
            }
            _ => {}
        }
        match current_block_54 {
            10036371062546261272 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_54 = 14399816886812208938;
            }
            _ => {}
        }
        match current_block_54 {
            14399816886812208938 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_54 = 1080332908278044257;
            }
            _ => {}
        }
        match current_block_54 {
            1080332908278044257 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_54 = 18072476671732897849;
            }
            _ => {}
        }
        match current_block_54 {
            18072476671732897849 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_54 = 3892783650257935544;
            }
            _ => {}
        }
        match current_block_54 {
            3892783650257935544 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_54 = 5206991630100269981;
            }
            _ => {}
        }
        match current_block_54 {
            5206991630100269981 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_54 = 2431160423376578768;
            }
            _ => {}
        }
        match current_block_54 {
            2431160423376578768 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_54 = 9021970710478294852;
            }
            _ => {}
        }
        match current_block_54 {
            9021970710478294852 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_54 = 11620144116472646757;
            }
            _ => {}
        }
        match current_block_54 {
            11620144116472646757 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        *_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint,
                    );
            }
            _ => {}
        }
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 13 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 13 as libc::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 12 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 5 as libc::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 3 as libc::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as libc::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 15 as libc::c_int;
        e = 0 as *mut pthread_cond_monotonic;
        if !monotonic_conds.is_null() {
            let mut _hf_bkt: libc::c_uint = 0;
            _hf_bkt = _hf_hashv
                & ((*(*monotonic_conds).hh.tbl).num_buckets)
                    .wrapping_sub(1 as libc::c_uint);
            if 1 as libc::c_int != 0 as libc::c_int {
                if !((*((*(*monotonic_conds).hh.tbl).buckets).offset(_hf_bkt as isize))
                    .hh_head)
                    .is_null()
                {
                    e = ((*((*(*monotonic_conds).hh.tbl).buckets)
                        .offset(_hf_bkt as isize))
                        .hh_head as *mut libc::c_char)
                        .offset(-((*(*monotonic_conds).hh.tbl).hho as isize))
                        as *mut libc::c_void as *mut pthread_cond_monotonic;
                } else {
                    e = 0 as *mut pthread_cond_monotonic;
                }
                while !e.is_null() {
                    if (*e).hh.hashv == _hf_hashv
                        && (*e).hh.keylen as libc::c_ulong
                            == ::std::mem::size_of::<*mut libc::c_void>()
                                as libc::c_ulong
                    {
                        if memcmp(
                            (*e).hh.key,
                            &mut cond as *mut *mut pthread_cond_t as *const libc::c_void,
                            ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            break;
                        }
                    }
                    if !((*e).hh.hh_next).is_null() {
                        e = ((*e).hh.hh_next as *mut libc::c_char)
                            .offset(-((*(*monotonic_conds).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut pthread_cond_monotonic;
                    } else {
                        e = 0 as *mut pthread_cond_monotonic;
                    }
                }
            }
        }
        pthread_rwlock_unlock(&mut monotonic_conds_lock);
        if !e.is_null() {
            clk_id = 1 as libc::c_int;
        } else {
            clk_id = 0 as libc::c_int;
        }
        let mut dont_fake_orig: bool = dont_fake;
        if !dont_fake {
            dont_fake = 1 as libc::c_int != 0;
        }
        result = (Some(real_clock_gettime.unwrap())).unwrap()(clk_id, &mut realtime);
        dont_fake = dont_fake_orig;
        if result == -(1 as libc::c_int) {
            return 22 as libc::c_int;
        }
        faketime = realtime;
        fake_clock_gettime(clk_id, &mut faketime);
        tmp_env = getenv(b"FAKETIME_WAIT_MS\0" as *const u8 as *const libc::c_char);
        if !tmp_env.is_null() {
            wait_ms = atol(tmp_env) as libc::c_int;
            let mut dont_fake_orig_0: bool = dont_fake;
            if !dont_fake {
                dont_fake = 1 as libc::c_int != 0;
            }
            result = (Some(real_clock_gettime.unwrap())).unwrap()(clk_id, &mut realtime);
            dont_fake = dont_fake_orig_0;
            if result == -(1 as libc::c_int) {
                return 22 as libc::c_int;
            }
            tdiff_actual.tv_sec = (wait_ms / 1000 as libc::c_int) as __time_t;
            tdiff_actual
                .tv_nsec = (wait_ms % 1000 as libc::c_int * 1000000 as libc::c_int)
                as __syscall_slong_t;
            tp.tv_sec = realtime.tv_sec + tdiff_actual.tv_sec;
            tp.tv_nsec = realtime.tv_nsec + tdiff_actual.tv_nsec;
            if tp.tv_nsec >= 1000000000 as libc::c_int as libc::c_long {
                tp.tv_sec += 1;
                tp.tv_sec;
                tp.tv_nsec -= 1000000000 as libc::c_int as libc::c_long;
            }
            tf = &mut tp;
        } else {
            tp.tv_sec = (*abstime).tv_sec - faketime.tv_sec;
            tp.tv_nsec = (*abstime).tv_nsec - faketime.tv_nsec;
            if tp.tv_nsec < 0 as libc::c_int as libc::c_long {
                tp.tv_sec -= 1;
                tp.tv_sec;
                tp.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
            }
            if user_rate_set {
                let mut tmp_time: int64_t = 0;
                tmp_time = (1.0f64 / user_rate
                    * (tp.tv_sec * 1000000000 as libc::c_int as libc::c_long
                        + tp.tv_nsec) as libc::c_double) as int64_t;
                tdiff_actual
                    .tv_nsec = tmp_time % 1000000000 as libc::c_int as libc::c_long;
                tdiff_actual
                    .tv_sec = (tmp_time - tdiff_actual.tv_nsec)
                    / 1000000000 as libc::c_int as libc::c_long;
                if tdiff_actual.tv_nsec < 0 as libc::c_int as libc::c_long {
                    tdiff_actual.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
                    tdiff_actual.tv_sec -= 1 as libc::c_int as libc::c_long;
                }
            } else {
                tdiff_actual = tp;
            }
        }
        if clk_id == 1 as libc::c_int {
            if needs_forced_monotonic_fix(
                b"pthread_cond_timedwait\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ) as libc::c_int == 1 as libc::c_int
            {
                tp.tv_sec = realtime.tv_sec + tdiff_actual.tv_sec;
                tp.tv_nsec = realtime.tv_nsec + tdiff_actual.tv_nsec;
                if tp.tv_nsec >= 1000000000 as libc::c_int as libc::c_long {
                    tp.tv_sec += 1;
                    tp.tv_sec;
                    tp.tv_nsec -= 1000000000 as libc::c_int as libc::c_long;
                }
            } else {
                tp.tv_sec = faketime.tv_sec + tdiff_actual.tv_sec;
                tp.tv_nsec = faketime.tv_nsec + tdiff_actual.tv_nsec;
                if tp.tv_nsec >= 1000000000 as libc::c_int as libc::c_long {
                    tp.tv_sec += 1;
                    tp.tv_sec;
                    tp.tv_nsec -= 1000000000 as libc::c_int as libc::c_long;
                }
            }
        } else {
            tp.tv_sec = realtime.tv_sec + tdiff_actual.tv_sec;
            tp.tv_nsec = realtime.tv_nsec + tdiff_actual.tv_nsec;
            if tp.tv_nsec >= 1000000000 as libc::c_int as libc::c_long {
                tp.tv_sec += 1;
                tp.tv_sec;
                tp.tv_nsec -= 1000000000 as libc::c_int as libc::c_long;
            }
        }
        tf = &mut tp;
    }
    match compat as libc::c_uint {
        1 => {
            result = real_pthread_cond_timedwait_232.unwrap()(cond, mutex, tf);
        }
        0 => {
            result = real_pthread_cond_timedwait_225.unwrap()(cond, mutex, tf);
        }
        _ => {}
    }
    return result;
}
#[no_mangle] pub unsafe extern "C" fn pthread_cond_timedwait_225(
    mut cond: *mut pthread_cond_t,
    mut mutex: *mut pthread_mutex_t,
    mut abstime: *const timespec,
) -> libc::c_int {
    return pthread_cond_timedwait_common(cond, mutex, abstime, FT_COMPAT_GLIBC_2_2_5);
}
#[no_mangle] pub unsafe extern "C" fn pthread_cond_timedwait_232(
    mut cond: *mut pthread_cond_t,
    mut mutex: *mut pthread_mutex_t,
    mut abstime: *const timespec,
) -> libc::c_int {
    return pthread_cond_timedwait_common(cond, mutex, abstime, FT_COMPAT_GLIBC_2_3_2);
}
