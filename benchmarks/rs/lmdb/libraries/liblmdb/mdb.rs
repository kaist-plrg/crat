use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn pthread_sigmask(
        __how: libc::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigwait(__set: *const sigset_t, __sig: *mut libc::c_int) -> libc::c_int;
    fn writev(__fd: libc::c_int, __iovec: *const iovec, __count: libc::c_int) -> ssize_t;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn msync(
        __addr: *mut libc::c_void,
        __len: size_t,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn madvise(
        __addr: *mut libc::c_void,
        __len: size_t,
        __advice: libc::c_int,
    ) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn posix_memalign(
        __memptr: *mut *mut libc::c_void,
        __alignment: size_t,
        __size: size_t,
    ) -> libc::c_int;
    fn abort() -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pread(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __nbytes: size_t,
        __offset: __off_t,
    ) -> ssize_t;
    fn pwrite(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __offset: __off_t,
    ) -> ssize_t;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn getpid() -> __pid_t;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn fdatasync(__fildes: libc::c_int) -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_consistent(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutexattr_init(__attr: *mut pthread_mutexattr_t) -> libc::c_int;
    fn pthread_mutexattr_destroy(__attr: *mut pthread_mutexattr_t) -> libc::c_int;
    fn pthread_mutexattr_setpshared(
        __attr: *mut pthread_mutexattr_t,
        __pshared: libc::c_int,
    ) -> libc::c_int;
    fn pthread_mutexattr_setrobust(
        __attr: *mut pthread_mutexattr_t,
        __robustness: libc::c_int,
    ) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn pthread_key_create(
        __key: *mut pthread_key_t,
        __destr_function: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    fn pthread_key_delete(__key: pthread_key_t) -> libc::c_int;
    fn pthread_getspecific(__key: pthread_key_t) -> *mut libc::c_void;
    fn pthread_setspecific(
        __key: pthread_key_t,
        __pointer: *const libc::c_void,
    ) -> libc::c_int;
    fn mdb_midl_search(ids: MDB_IDL, id: MDB_ID) -> libc::c_uint;
    fn mdb_midl_alloc(num: libc::c_int) -> MDB_IDL;
    fn mdb_midl_free(ids: MDB_IDL);
    fn mdb_midl_shrink(idp: *mut MDB_IDL);
    fn mdb_midl_need(idp: *mut MDB_IDL, num: libc::c_uint) -> libc::c_int;
    fn mdb_midl_append(idp: *mut MDB_IDL, id: MDB_ID) -> libc::c_int;
    fn mdb_midl_append_list(idp: *mut MDB_IDL, app: MDB_IDL) -> libc::c_int;
    fn mdb_midl_append_range(
        idp: *mut MDB_IDL,
        id: MDB_ID,
        n: libc::c_uint,
    ) -> libc::c_int;
    fn mdb_midl_xmerge(idl: MDB_IDL, merge: MDB_IDL);
    fn mdb_midl_sort(ids: MDB_IDL);
    fn mdb_mid2l_search(ids: MDB_ID2L, id: MDB_ID) -> libc::c_uint;
    fn mdb_mid2l_insert(ids: MDB_ID2L, id: *mut MDB_ID2) -> libc::c_int;
    fn mdb_mid2l_append(ids: MDB_ID2L, id: *mut MDB_ID2) -> libc::c_int;
    fn uname(__name: *mut utsname) -> libc::c_int;
    fn fstatfs(__fildes: libc::c_int, __buf: *mut statfs) -> libc::c_int;
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __fsid_t {
    pub __val: [libc::c_int; 2],
}
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __fsblkcnt_t = libc::c_ulong;
pub type __fsfilcnt_t = libc::c_ulong;
pub type __fsword_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type mode_t = __mode_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
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
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
pub type pthread_key_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
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
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
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
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flock {
    pub l_type: libc::c_short,
    pub l_whence: libc::c_short,
    pub l_start: __off_t,
    pub l_len: __off_t,
    pub l_pid: __pid_t,
}
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed_3 = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed_3 = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed_3 = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed_3 = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed_3 = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed_3 = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed_3 = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed_3 = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed_3 = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed_3 = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed_3 = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed_3 = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed_3 = 236;
pub const _SC_IPV6: C2RustUnnamed_3 = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed_3 = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed_3 = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed_3 = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed_3 = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed_3 = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed_3 = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed_3 = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed_3 = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed_3 = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed_3 = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed_3 = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed_3 = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed_3 = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed_3 = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed_3 = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed_3 = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed_3 = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed_3 = 182;
pub const _SC_TRACE: C2RustUnnamed_3 = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed_3 = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed_3 = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed_3 = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed_3 = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed_3 = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed_3 = 175;
pub const _SC_STREAMS: C2RustUnnamed_3 = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed_3 = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed_3 = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed_3 = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed_3 = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed_3 = 169;
pub const _SC_2_PBS: C2RustUnnamed_3 = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed_3 = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed_3 = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed_3 = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed_3 = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed_3 = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed_3 = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed_3 = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed_3 = 160;
pub const _SC_SPAWN: C2RustUnnamed_3 = 159;
pub const _SC_SIGNALS: C2RustUnnamed_3 = 158;
pub const _SC_SHELL: C2RustUnnamed_3 = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed_3 = 156;
pub const _SC_REGEXP: C2RustUnnamed_3 = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed_3 = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed_3 = 153;
pub const _SC_NETWORKING: C2RustUnnamed_3 = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed_3 = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed_3 = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed_3 = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed_3 = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed_3 = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed_3 = 146;
pub const _SC_PIPE: C2RustUnnamed_3 = 145;
pub const _SC_FIFO: C2RustUnnamed_3 = 144;
pub const _SC_FD_MGMT: C2RustUnnamed_3 = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed_3 = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed_3 = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed_3 = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed_3 = 139;
pub const _SC_CPUTIME: C2RustUnnamed_3 = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed_3 = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed_3 = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed_3 = 135;
pub const _SC_BASE: C2RustUnnamed_3 = 134;
pub const _SC_BARRIERS: C2RustUnnamed_3 = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed_3 = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed_3 = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed_3 = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed_3 = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed_3 = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed_3 = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed_3 = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed_3 = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed_3 = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed_3 = 123;
pub const _SC_NL_NMAX: C2RustUnnamed_3 = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed_3 = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed_3 = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed_3 = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed_3 = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed_3 = 117;
pub const _SC_UINT_MAX: C2RustUnnamed_3 = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed_3 = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed_3 = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed_3 = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed_3 = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed_3 = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed_3 = 110;
pub const _SC_NZERO: C2RustUnnamed_3 = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed_3 = 108;
pub const _SC_WORD_BIT: C2RustUnnamed_3 = 107;
pub const _SC_LONG_BIT: C2RustUnnamed_3 = 106;
pub const _SC_INT_MIN: C2RustUnnamed_3 = 105;
pub const _SC_INT_MAX: C2RustUnnamed_3 = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed_3 = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed_3 = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed_3 = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed_3 = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed_3 = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed_3 = 98;
pub const _SC_2_UPE: C2RustUnnamed_3 = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed_3 = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed_3 = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed_3 = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed_3 = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed_3 = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed_3 = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed_3 = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed_3 = 89;
pub const _SC_PASS_MAX: C2RustUnnamed_3 = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed_3 = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed_3 = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed_3 = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed_3 = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed_3 = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed_3 = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed_3 = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed_3 = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed_3 = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed_3 = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed_3 = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed_3 = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed_3 = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed_3 = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed_3 = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed_3 = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed_3 = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed_3 = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed_3 = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed_3 = 68;
pub const _SC_THREADS: C2RustUnnamed_3 = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed_3 = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed_3 = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed_3 = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed_3 = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed_3 = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed_3 = 61;
pub const _SC_IOV_MAX: C2RustUnnamed_3 = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed_3 = 60;
pub const _SC_SELECT: C2RustUnnamed_3 = 59;
pub const _SC_POLL: C2RustUnnamed_3 = 58;
pub const _SC_PII_OSI: C2RustUnnamed_3 = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed_3 = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed_3 = 55;
pub const _SC_PII_XTI: C2RustUnnamed_3 = 54;
pub const _SC_PII: C2RustUnnamed_3 = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed_3 = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed_3 = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed_3 = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed_3 = 49;
pub const _SC_2_C_DEV: C2RustUnnamed_3 = 48;
pub const _SC_2_C_BIND: C2RustUnnamed_3 = 47;
pub const _SC_2_VERSION: C2RustUnnamed_3 = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed_3 = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed_3 = 44;
pub const _SC_LINE_MAX: C2RustUnnamed_3 = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed_3 = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed_3 = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed_3 = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed_3 = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed_3 = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed_3 = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed_3 = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed_3 = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed_3 = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed_3 = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed_3 = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed_3 = 31;
pub const _SC_PAGESIZE: C2RustUnnamed_3 = 30;
pub const _SC_VERSION: C2RustUnnamed_3 = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed_3 = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed_3 = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed_3 = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed_3 = 25;
pub const _SC_AIO_MAX: C2RustUnnamed_3 = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed_3 = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed_3 = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed_3 = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed_3 = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed_3 = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed_3 = 18;
pub const _SC_MEMLOCK: C2RustUnnamed_3 = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed_3 = 16;
pub const _SC_FSYNC: C2RustUnnamed_3 = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed_3 = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed_3 = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed_3 = 12;
pub const _SC_TIMERS: C2RustUnnamed_3 = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed_3 = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed_3 = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed_3 = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed_3 = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed_3 = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed_3 = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed_3 = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed_3 = 3;
pub const _SC_CLK_TCK: C2RustUnnamed_3 = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed_3 = 1;
pub const _SC_ARG_MAX: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const PTHREAD_MUTEX_ROBUST_NP: C2RustUnnamed_4 = 1;
pub const PTHREAD_MUTEX_ROBUST: C2RustUnnamed_4 = 1;
pub const PTHREAD_MUTEX_STALLED_NP: C2RustUnnamed_4 = 0;
pub const PTHREAD_MUTEX_STALLED: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const PTHREAD_PROCESS_SHARED: C2RustUnnamed_5 = 1;
pub const PTHREAD_PROCESS_PRIVATE: C2RustUnnamed_5 = 0;
pub type mdb_mode_t = mode_t;
pub type mdb_size_t = size_t;
pub type mdb_filehandle_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_env {
    pub me_fd: libc::c_int,
    pub me_lfd: libc::c_int,
    pub me_mfd: libc::c_int,
    pub me_flags: uint32_t,
    pub me_psize: libc::c_uint,
    pub me_os_psize: libc::c_uint,
    pub me_maxreaders: libc::c_uint,
    pub me_close_readers: libc::c_int,
    pub me_numdbs: MDB_dbi,
    pub me_maxdbs: MDB_dbi,
    pub me_pid: pid_t,
    pub me_path: *mut libc::c_char,
    pub me_map: *mut libc::c_char,
    pub me_txns: *mut MDB_txninfo,
    pub me_metas: [*mut MDB_meta; 2],
    pub me_pbuf: *mut libc::c_void,
    pub me_txn: *mut MDB_txn,
    pub me_txn0: *mut MDB_txn,
    pub me_mapsize: mdb_size_t,
    pub me_size: off_t,
    pub me_maxpg: pgno_t,
    pub me_dbxs: *mut MDB_dbx,
    pub me_dbflags: *mut uint16_t,
    pub me_dbiseqs: *mut libc::c_uint,
    pub me_txkey: pthread_key_t,
    pub me_pgoldest: txnid_t,
    pub me_pgstate: MDB_pgstate,
    pub me_dpages: *mut MDB_page,
    pub me_free_pgs: MDB_IDL,
    pub me_dirty_list: MDB_ID2L,
    pub me_maxfree_1pg: libc::c_int,
    pub me_nodemax: libc::c_uint,
    pub me_live_reader: libc::c_int,
    pub me_userctx: *mut libc::c_void,
    pub me_assert_func: Option::<MDB_assert_func>,
}
pub type MDB_assert_func = unsafe extern "C" fn(*mut MDB_env, *const libc::c_char) -> ();
pub type MDB_ID2L = *mut MDB_ID2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_ID2 {
    pub mid: MDB_ID,
    pub mptr: *mut libc::c_void,
}
pub type MDB_ID = mdb_size_t;
pub type MDB_IDL = *mut MDB_ID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_page {
    pub mp_p: C2RustUnnamed_8,
    pub mp_pad: uint16_t,
    pub mp_flags: uint16_t,
    pub mp_pb: C2RustUnnamed_6,
    pub mp_ptrs: [indx_t; 1],
}
pub type indx_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub pb: C2RustUnnamed_7,
    pub pb_pages: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub pb_lower: indx_t,
    pub pb_upper: indx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub p_pgno: pgno_t,
    pub p_next: *mut MDB_page,
}
pub type pgno_t = MDB_ID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_pgstate {
    pub mf_pghead: *mut pgno_t,
    pub mf_pglast: txnid_t,
}
pub type txnid_t = MDB_ID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_dbx {
    pub md_name: MDB_val,
    pub md_cmp: Option::<MDB_cmp_func>,
    pub md_dcmp: Option::<MDB_cmp_func>,
    pub md_rel: Option::<MDB_rel_func>,
    pub md_relctx: *mut libc::c_void,
}
pub type MDB_rel_func = unsafe extern "C" fn(
    *mut MDB_val,
    *mut libc::c_void,
    *mut libc::c_void,
    *mut libc::c_void,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_val {
    pub mv_size: size_t,
    pub mv_data: *mut libc::c_void,
}
pub type MDB_cmp_func = unsafe extern "C" fn(
    *const MDB_val,
    *const MDB_val,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_txn {
    pub mt_parent: *mut MDB_txn,
    pub mt_child: *mut MDB_txn,
    pub mt_next_pgno: pgno_t,
    pub mt_txnid: txnid_t,
    pub mt_env: *mut MDB_env,
    pub mt_free_pgs: MDB_IDL,
    pub mt_loose_pgs: *mut MDB_page,
    pub mt_loose_count: libc::c_int,
    pub mt_spill_pgs: MDB_IDL,
    pub mt_u: C2RustUnnamed_9,
    pub mt_dbxs: *mut MDB_dbx,
    pub mt_dbs: *mut MDB_db,
    pub mt_dbiseqs: *mut libc::c_uint,
    pub mt_cursors: *mut *mut MDB_cursor,
    pub mt_dbflags: *mut libc::c_uchar,
    pub mt_numdbs: MDB_dbi,
    pub mt_flags: libc::c_uint,
    pub mt_dirty_room: libc::c_uint,
}
pub type MDB_dbi = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_cursor {
    pub mc_next: *mut MDB_cursor,
    pub mc_backup: *mut MDB_cursor,
    pub mc_xcursor: *mut MDB_xcursor,
    pub mc_txn: *mut MDB_txn,
    pub mc_dbi: MDB_dbi,
    pub mc_db: *mut MDB_db,
    pub mc_dbx: *mut MDB_dbx,
    pub mc_dbflag: *mut libc::c_uchar,
    pub mc_snum: libc::c_ushort,
    pub mc_top: libc::c_ushort,
    pub mc_flags: libc::c_uint,
    pub mc_pg: [*mut MDB_page; 32],
    pub mc_ki: [indx_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_db {
    pub md_pad: uint32_t,
    pub md_flags: uint16_t,
    pub md_depth: uint16_t,
    pub md_branch_pages: pgno_t,
    pub md_leaf_pages: pgno_t,
    pub md_overflow_pages: pgno_t,
    pub md_entries: mdb_size_t,
    pub md_root: pgno_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_xcursor {
    pub mx_cursor: MDB_cursor,
    pub mx_db: MDB_db,
    pub mx_dbx: MDB_dbx,
    pub mx_dbflag: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub dirty_list: MDB_ID2L,
    pub reader: *mut MDB_reader,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_reader {
    pub mru: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub mrx: MDB_rxbody,
    pub pad: [libc::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_rxbody {
    pub mrb_txnid: txnid_t,
    pub mrb_pid: pid_t,
    pub mrb_tid: pthread_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_meta {
    pub mm_magic: uint32_t,
    pub mm_version: uint32_t,
    pub mm_address: *mut libc::c_void,
    pub mm_mapsize: mdb_size_t,
    pub mm_dbs: [MDB_db; 2],
    pub mm_last_pg: pgno_t,
    pub mm_txnid: txnid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_txninfo {
    pub mt1: C2RustUnnamed_12,
    pub mt2: C2RustUnnamed_11,
    pub mti_readers: [MDB_reader; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub mt2_wmutex: mdb_mutex_t,
    pub pad: [libc::c_char; 64],
}
pub type mdb_mutex_t = [pthread_mutex_t; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub mtb: MDB_txbody,
    pub pad: [libc::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_txbody {
    pub mtb_magic: uint32_t,
    pub mtb_format: uint32_t,
    pub mtb_txnid: txnid_t,
    pub mtb_numreaders: libc::c_uint,
    pub mtb_rmutex: mdb_mutex_t,
}
pub type MDB_cursor_op = libc::c_uint;
pub const MDB_PREV_MULTIPLE: MDB_cursor_op = 18;
pub const MDB_SET_RANGE: MDB_cursor_op = 17;
pub const MDB_SET_KEY: MDB_cursor_op = 16;
pub const MDB_SET: MDB_cursor_op = 15;
pub const MDB_PREV_NODUP: MDB_cursor_op = 14;
pub const MDB_PREV_DUP: MDB_cursor_op = 13;
pub const MDB_PREV: MDB_cursor_op = 12;
pub const MDB_NEXT_NODUP: MDB_cursor_op = 11;
pub const MDB_NEXT_MULTIPLE: MDB_cursor_op = 10;
pub const MDB_NEXT_DUP: MDB_cursor_op = 9;
pub const MDB_NEXT: MDB_cursor_op = 8;
pub const MDB_LAST_DUP: MDB_cursor_op = 7;
pub const MDB_LAST: MDB_cursor_op = 6;
pub const MDB_GET_MULTIPLE: MDB_cursor_op = 5;
pub const MDB_GET_CURRENT: MDB_cursor_op = 4;
pub const MDB_GET_BOTH_RANGE: MDB_cursor_op = 3;
pub const MDB_GET_BOTH: MDB_cursor_op = 2;
pub const MDB_FIRST_DUP: MDB_cursor_op = 1;
pub const MDB_FIRST: MDB_cursor_op = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_stat {
    pub ms_psize: libc::c_uint,
    pub ms_depth: libc::c_uint,
    pub ms_branch_pages: mdb_size_t,
    pub ms_leaf_pages: mdb_size_t,
    pub ms_overflow_pages: mdb_size_t,
    pub ms_entries: mdb_size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_envinfo {
    pub me_mapaddr: *mut libc::c_void,
    pub me_mapsize: mdb_size_t,
    pub me_last_pgno: mdb_size_t,
    pub me_last_txnid: mdb_size_t,
    pub me_maxreaders: libc::c_uint,
    pub me_numreaders: libc::c_uint,
}
pub type mdb_nchar_t = libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_name {
    pub mn_len: libc::c_int,
    pub mn_alloced: libc::c_int,
    pub mn_val: *mut mdb_nchar_t,
}
pub type mdb_fopen_type = libc::c_uint;
pub const MDB_O_LOCKS: mdb_fopen_type = 524358;
pub const MDB_O_MASK: mdb_fopen_type = 528579;
pub const MDB_O_COPY: mdb_fopen_type = 524481;
pub const MDB_O_META: mdb_fopen_type = 528385;
pub const MDB_O_RDWR: mdb_fopen_type = 66;
pub const MDB_O_RDONLY: mdb_fopen_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union MDB_metabuf {
    pub mb_page: MDB_page,
    pub mb_metabuf: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub mm_pad: [libc::c_char; 16],
    pub mm_meta: MDB_meta,
}
pub const Size: C2RustUnnamed_14 = 152;
pub type C2RustUnnamed_14 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub domainname: [libc::c_char; 65],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statfs {
    pub f_type: __fsword_t,
    pub f_bsize: __fsword_t,
    pub f_blocks: __fsblkcnt_t,
    pub f_bfree: __fsblkcnt_t,
    pub f_bavail: __fsblkcnt_t,
    pub f_files: __fsfilcnt_t,
    pub f_ffree: __fsfilcnt_t,
    pub f_fsid: __fsid_t,
    pub f_namelen: __fsword_t,
    pub f_frsize: __fsword_t,
    pub f_flags: __fsword_t,
    pub f_spare: [__fsword_t; 4],
}
pub const MDB_lock_desc: C2RustUnnamed_18 = 180982;
pub const MDB_END_ABORT: C2RustUnnamed_19 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_ntxn {
    pub mnt_txn: MDB_txn,
    pub mnt_pgstate: MDB_pgstate,
}
pub type mdb_mutexref_t = *mut pthread_mutex_t;
pub const MDB_END_FAIL_BEGIN: C2RustUnnamed_19 = 5;
pub type Pidlock_op = libc::c_uint;
pub const Pidcheck: Pidlock_op = 5;
pub const Pidset: Pidlock_op = 6;
pub const MDB_END_RESET_TMP: C2RustUnnamed_19 = 4;
pub const MDB_END_FAIL_BEGINCHILD: C2RustUnnamed_19 = 6;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdb_copy {
    pub mc_env: *mut MDB_env,
    pub mc_txn: *mut MDB_txn,
    pub mc_mutex: pthread_mutex_t,
    pub mc_cond: pthread_cond_t,
    pub mc_wbuf: [*mut libc::c_char; 2],
    pub mc_over: [*mut libc::c_char; 2],
    pub mc_wlen: [libc::c_int; 2],
    pub mc_olen: [libc::c_int; 2],
    pub mc_next_pgno: pgno_t,
    pub mc_fd: libc::c_int,
    pub mc_toggle: libc::c_int,
    pub mc_new: libc::c_int,
    pub mc_error: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_node {
    pub mn_lo: libc::c_ushort,
    pub mn_hi: libc::c_ushort,
    pub mn_flags: libc::c_ushort,
    pub mn_ksize: libc::c_ushort,
    pub mn_data: [libc::c_char; 1],
}
pub const Align: C2RustUnnamed_15 = 8;
pub type C2RustUnnamed_15 = libc::c_uint;
pub const Paranoid: C2RustUnnamed_16 = 0;
pub type C2RustUnnamed_16 = libc::c_uint;
pub const Max_retries: C2RustUnnamed_16 = 2147483647;
pub const MDB_END_COMMITTED: C2RustUnnamed_19 = 0;
pub const Mask: C2RustUnnamed_17 = 49232;
pub type C2RustUnnamed_17 = libc::c_uint;
pub const MDB_END_EMPTY_COMMIT: C2RustUnnamed_19 = 1;
pub const MDB_END_RESET: C2RustUnnamed_19 = 3;
pub type MDB_msg_func = unsafe extern "C" fn(
    *const libc::c_char,
    *mut libc::c_void,
) -> libc::c_int;
pub type C2RustUnnamed_18 = libc::c_uint;
pub type C2RustUnnamed_19 = libc::c_uint;
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub unsafe extern "C" fn mdb_version(
    mut major: *mut libc::c_int,
    mut minor: *mut libc::c_int,
    mut patch: *mut libc::c_int,
) -> *mut libc::c_char {
    if !major.is_null() {
        *major = 0 as libc::c_int;
    }
    if !minor.is_null() {
        *minor = 9 as libc::c_int;
    }
    if !patch.is_null() {
        *patch = 70 as libc::c_int;
    }
    return b"LMDB 0.9.70: (December 19, 2015)\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
}
static mut mdb_errstr: [*mut libc::c_char; 21] = [
    b"MDB_KEYEXIST: Key/data pair already exists\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"MDB_NOTFOUND: No matching key/data pair found\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_PAGE_NOTFOUND: Requested page not found\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"MDB_CORRUPTED: Located page was wrong type\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"MDB_PANIC: Update of meta page failed or environment had fatal error\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MDB_VERSION_MISMATCH: Database environment version mismatch\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_INVALID: File is not an LMDB file\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"MDB_MAP_FULL: Environment mapsize limit reached\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_DBS_FULL: Environment maxdbs limit reached\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_READERS_FULL: Environment maxreaders limit reached\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_TLS_FULL: Thread-local storage keys full - too many environments open\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MDB_TXN_FULL: Transaction has too many dirty pages - transaction too big\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MDB_CURSOR_FULL: Internal error - cursor stack limit reached\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_PAGE_FULL: Internal error - page has no more space\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_MAP_RESIZED: Database contents grew beyond environment mapsize\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_INCOMPATIBLE: Operation and DB incompatible, or DB flags changed\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MDB_BAD_RSLOT: Invalid reuse of reader locktable slot\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_BAD_TXN: Transaction must abort, has a child, or is invalid\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"MDB_BAD_VALSIZE: Unsupported size of key/DB name/data, or wrong DUPFIXED size\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MDB_BAD_DBI: The specified DBI handle was closed/changed unexpectedly\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MDB_PROBLEM: Unexpected problem - txn should abort\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
];
pub unsafe extern "C" fn mdb_strerror(mut err: libc::c_int) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    if err == 0 {
        return b"Successful return: 0\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    if err >= -(30799 as libc::c_int) && err <= -(30779 as libc::c_int) {
        i = err - -(30799 as libc::c_int);
        return mdb_errstr[i as usize];
    }
    return strerror(err);
}
unsafe extern "C" fn mdb_assert_fail(
    mut env: *mut MDB_env,
    mut expr_txt: *const libc::c_char,
    mut func: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
) {
    let mut buf: [libc::c_char; 400] = [0; 400];
    sprintf(
        buf.as_mut_ptr(),
        b"%.100s:%d: Assertion '%.200s' failed in %.40s()\0" as *const u8
            as *const libc::c_char,
        file,
        line,
        expr_txt,
        func,
    );
    if ((*env).me_assert_func).is_some() {
        ((*env).me_assert_func).unwrap()(env, buf.as_mut_ptr());
    }
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
    abort();
}
pub unsafe extern "C" fn mdb_cmp(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut a: *const MDB_val,
    mut b: *const MDB_val,
) -> libc::c_int {
    return ((*((*txn).mt_dbxs).offset(dbi as isize)).md_cmp).unwrap()(a, b);
}
pub unsafe extern "C" fn mdb_dcmp(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut a: *const MDB_val,
    mut b: *const MDB_val,
) -> libc::c_int {
    let mut dcmp: Option::<MDB_cmp_func> = (*((*txn).mt_dbxs).offset(dbi as isize))
        .md_dcmp;
    if ((2147483647 as libc::c_int as libc::c_uint)
        .wrapping_mul(2 as libc::c_uint)
        .wrapping_add(1 as libc::c_uint) as libc::c_ulong)
        < 18446744073709551615 as libc::c_ulong
        && dcmp == Some(mdb_cmp_int as MDB_cmp_func)
        && (*a).mv_size == ::std::mem::size_of::<mdb_size_t>() as libc::c_ulong
    {
        dcmp = Some(mdb_cmp_long as MDB_cmp_func);
    }
    return dcmp.unwrap()(a, b);
}
unsafe extern "C" fn mdb_page_malloc(
    mut txn: *mut MDB_txn,
    mut num: libc::c_uint,
) -> *mut MDB_page {
    let mut env: *mut MDB_env = (*txn).mt_env;
    let mut ret: *mut MDB_page = (*env).me_dpages;
    let mut psize: size_t = (*env).me_psize as size_t;
    let mut sz: size_t = psize;
    let mut off: size_t = 0;
    if num == 1 as libc::c_int as libc::c_uint {
        if !ret.is_null() {
            (*env).me_dpages = (*ret).mp_p.p_next;
            return ret;
        }
        off = 16 as libc::c_ulong as libc::c_uint as size_t;
        psize = (psize as libc::c_ulong).wrapping_sub(off) as size_t as size_t;
    } else {
        sz = (sz as libc::c_ulong).wrapping_mul(num as libc::c_ulong) as size_t
            as size_t;
        off = sz.wrapping_sub(psize);
    }
    ret = malloc(sz) as *mut MDB_page;
    if !ret.is_null() {
        if (*env).me_flags & 0x1000000 as libc::c_int as libc::c_uint == 0 {
            memset(
                (ret as *mut libc::c_char).offset(off as isize) as *mut libc::c_void,
                0 as libc::c_int,
                psize,
            );
            (*ret).mp_pad = 0 as libc::c_int as uint16_t;
        }
    } else {
        (*txn).mt_flags |= 0x2 as libc::c_int as libc::c_uint;
    }
    return ret;
}
unsafe extern "C" fn mdb_page_free(mut env: *mut MDB_env, mut mp: *mut MDB_page) {
    (*mp).mp_p.p_next = (*env).me_dpages;
    (*env).me_dpages = mp;
}
unsafe extern "C" fn mdb_dpage_free(mut env: *mut MDB_env, mut dp: *mut MDB_page) {
    if !((*dp).mp_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int)
        || (*dp).mp_pb.pb_pages == 1 as libc::c_int as libc::c_uint
    {
        mdb_page_free(env, dp);
    } else {
        free(dp as *mut libc::c_void);
    };
}
unsafe extern "C" fn mdb_dlist_free(mut txn: *mut MDB_txn) {
    let mut env: *mut MDB_env = (*txn).mt_env;
    let mut dl: MDB_ID2L = (*txn).mt_u.dirty_list;
    let mut i: libc::c_uint = 0;
    let mut n: libc::c_uint = (*dl.offset(0 as libc::c_int as isize)).mid
        as libc::c_uint;
    i = 1 as libc::c_int as libc::c_uint;
    while i <= n {
        mdb_dpage_free(env, (*dl.offset(i as isize)).mptr as *mut MDB_page);
        i = i.wrapping_add(1);
        i;
    }
    (*dl.offset(0 as libc::c_int as isize)).mid = 0 as libc::c_int as MDB_ID;
}
unsafe extern "C" fn mdb_page_loose(
    mut mc: *mut MDB_cursor,
    mut mp: *mut MDB_page,
) -> libc::c_int {
    let mut loose: libc::c_int = 0 as libc::c_int;
    let mut pgno: pgno_t = (*mp).mp_p.p_pgno;
    let mut txn: *mut MDB_txn = (*mc).mc_txn;
    if (*mp).mp_flags as libc::c_int & 0x10 as libc::c_int != 0
        && (*mc).mc_dbi != 0 as libc::c_int as libc::c_uint
    {
        if !((*txn).mt_parent).is_null() {
            let mut dl: *mut MDB_ID2 = (*txn).mt_u.dirty_list;
            if (*dl.offset(0 as libc::c_int as isize)).mid != 0 {
                let mut x: libc::c_uint = mdb_mid2l_search(dl, pgno);
                if x as libc::c_ulong <= (*dl.offset(0 as libc::c_int as isize)).mid
                    && (*dl.offset(x as isize)).mid == pgno
                {
                    if mp != (*dl.offset(x as isize)).mptr as *mut MDB_page {
                        (*mc).mc_flags
                            &= !(0x1 as libc::c_int | 0x2 as libc::c_int)
                                as libc::c_uint;
                        (*txn).mt_flags |= 0x2 as libc::c_int as libc::c_uint;
                        return -(30779 as libc::c_int);
                    }
                    loose = 1 as libc::c_int;
                }
            }
        } else {
            loose = 1 as libc::c_int;
        }
    }
    if loose != 0 {
        let ref mut fresh0 = *(mp.offset(2 as libc::c_int as isize)
            as *mut *mut MDB_page);
        *fresh0 = (*txn).mt_loose_pgs;
        (*txn).mt_loose_pgs = mp;
        (*txn).mt_loose_count += 1;
        (*txn).mt_loose_count;
        (*mp)
            .mp_flags = ((*mp).mp_flags as libc::c_int | 0x4000 as libc::c_int)
            as uint16_t;
    } else {
        let mut rc: libc::c_int = mdb_midl_append(&mut (*txn).mt_free_pgs, pgno);
        if rc != 0 {
            return rc;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_pages_xkeep(
    mut mc: *mut MDB_cursor,
    mut pflags: libc::c_uint,
    mut all: libc::c_int,
) -> libc::c_int {
    let mut txn: *mut MDB_txn = (*mc).mc_txn;
    let mut m3: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut m0: *mut MDB_cursor = mc;
    let mut mx: *mut MDB_xcursor = 0 as *mut MDB_xcursor;
    let mut dp: *mut MDB_page = 0 as *mut MDB_page;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut leaf: *mut MDB_node = 0 as *mut MDB_node;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut level: libc::c_int = 0;
    i = (*txn).mt_numdbs;
    's_25: loop {
        if (*mc).mc_flags & 0x1 as libc::c_int as libc::c_uint != 0 {
            m3 = mc;
            loop {
                mp = 0 as *mut MDB_page;
                j = 0 as libc::c_int as libc::c_uint;
                while j < (*m3).mc_snum as libc::c_uint {
                    mp = (*m3).mc_pg[j as usize];
                    if ((*mp).mp_flags as libc::c_int & Mask as libc::c_int)
                        as libc::c_uint == pflags
                    {
                        (*mp)
                            .mp_flags = ((*mp).mp_flags as libc::c_int
                            ^ 0x8000 as libc::c_int) as uint16_t;
                    }
                    j = j.wrapping_add(1);
                    j;
                }
                mx = (*m3).mc_xcursor;
                if !(!mx.is_null()
                    && (*mx).mx_cursor.mc_flags & 0x1 as libc::c_int as libc::c_uint
                        != 0)
                {
                    break;
                }
                if !(!mp.is_null()
                    && (*mp).mp_flags as libc::c_int & 0x2 as libc::c_int != 0)
                {
                    break;
                }
                leaf = (mp as *mut libc::c_char)
                    .offset(
                        *((*mp).mp_ptrs)
                            .as_mut_ptr()
                            .offset(
                                (*m3)
                                    .mc_ki[j.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    as usize] as isize,
                            ) as libc::c_int as isize,
                    )
                    .offset(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                if (*leaf).mn_flags as libc::c_int & 0x2 as libc::c_int == 0 {
                    break;
                }
                m3 = &mut (*mx).mx_cursor;
            }
        }
        mc = (*mc).mc_next;
        while mc.is_null() || mc == m0 {
            if i == 0 as libc::c_int as libc::c_uint {
                break 's_25;
            }
            i = i.wrapping_sub(1);
            mc = *((*txn).mt_cursors).offset(i as isize);
        }
    }
    if all != 0 {
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*txn).mt_numdbs {
            if *((*txn).mt_dbflags).offset(i as isize) as libc::c_int
                & 0x1 as libc::c_int != 0
            {
                let mut pgno: pgno_t = (*((*txn).mt_dbs).offset(i as isize)).md_root;
                if !(pgno == !(0 as libc::c_int as pgno_t)) {
                    rc = mdb_page_get(m0, pgno, &mut dp, &mut level);
                    if rc != 0 as libc::c_int {
                        break;
                    }
                    if ((*dp).mp_flags as libc::c_int & Mask as libc::c_int)
                        as libc::c_uint == pflags && level <= 1 as libc::c_int
                    {
                        (*dp)
                            .mp_flags = ((*dp).mp_flags as libc::c_int
                            ^ 0x8000 as libc::c_int) as uint16_t;
                    }
                }
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return rc;
}
unsafe extern "C" fn mdb_page_spill(
    mut m0: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> libc::c_int {
    let mut current_block: u64;
    let mut txn: *mut MDB_txn = (*m0).mc_txn;
    let mut dp: *mut MDB_page = 0 as *mut MDB_page;
    let mut dl: MDB_ID2L = (*txn).mt_u.dirty_list;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut need: libc::c_uint = 0;
    let mut rc: libc::c_int = 0;
    if (*m0).mc_flags & 0x4 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int;
    }
    i = (*(*m0).mc_db).md_depth as libc::c_uint;
    if (*m0).mc_dbi >= 2 as libc::c_int as libc::c_uint {
        i = i
            .wrapping_add(
                (*((*txn).mt_dbs).offset(1 as libc::c_int as isize)).md_depth
                    as libc::c_uint,
            );
    }
    if !key.is_null() {
        i = (i as libc::c_ulong)
            .wrapping_add(
                (8 as libc::c_ulong)
                    .wrapping_add((*key).mv_size)
                    .wrapping_add((*data).mv_size)
                    .wrapping_add((*(*txn).mt_env).me_psize as libc::c_ulong)
                    .wrapping_div((*(*txn).mt_env).me_psize as libc::c_ulong),
            ) as libc::c_uint as libc::c_uint;
    }
    i = i.wrapping_add(i);
    need = i;
    if (*txn).mt_dirty_room > i {
        return 0 as libc::c_int;
    }
    if ((*txn).mt_spill_pgs).is_null() {
        (*txn)
            .mt_spill_pgs = mdb_midl_alloc(
            ((1 as libc::c_int) << 16 as libc::c_int + 1 as libc::c_int)
                - 1 as libc::c_int,
        );
        if ((*txn).mt_spill_pgs).is_null() {
            return 12 as libc::c_int;
        }
    } else {
        let mut sl: MDB_IDL = (*txn).mt_spill_pgs;
        let mut num: libc::c_uint = *sl.offset(0 as libc::c_int as isize)
            as libc::c_uint;
        j = 0 as libc::c_int as libc::c_uint;
        i = 1 as libc::c_int as libc::c_uint;
        while i <= num {
            if *sl.offset(i as isize) & 1 as libc::c_int as libc::c_ulong == 0 {
                j = j.wrapping_add(1);
                *sl.offset(j as isize) = *sl.offset(i as isize);
            }
            i = i.wrapping_add(1);
            i;
        }
        *sl.offset(0 as libc::c_int as isize) = j as MDB_ID;
    }
    rc = mdb_pages_xkeep(m0, 0x10 as libc::c_int as libc::c_uint, 1 as libc::c_int);
    if !(rc != 0 as libc::c_int) {
        if need
            < ((((1 as libc::c_int) << 16 as libc::c_int + 1 as libc::c_int)
                - 1 as libc::c_int) / 8 as libc::c_int) as libc::c_uint
        {
            need = ((((1 as libc::c_int) << 16 as libc::c_int + 1 as libc::c_int)
                - 1 as libc::c_int) / 8 as libc::c_int) as libc::c_uint;
        }
        i = (*dl.offset(0 as libc::c_int as isize)).mid as libc::c_uint;
        loop {
            if !(i != 0 && need != 0) {
                current_block = 980989089337379490;
                break;
            }
            let mut pn: MDB_ID = (*dl.offset(i as isize)).mid << 1 as libc::c_int;
            dp = (*dl.offset(i as isize)).mptr as *mut MDB_page;
            if !((*dp).mp_flags as libc::c_int
                & (0x4000 as libc::c_int | 0x8000 as libc::c_int) != 0)
            {
                if !((*txn).mt_parent).is_null() {
                    let mut tx2: *mut MDB_txn = 0 as *mut MDB_txn;
                    tx2 = (*txn).mt_parent;
                    while !tx2.is_null() {
                        if !((*tx2).mt_spill_pgs).is_null() {
                            j = mdb_midl_search((*tx2).mt_spill_pgs, pn);
                            if j as libc::c_ulong
                                <= *((*tx2).mt_spill_pgs).offset(0 as libc::c_int as isize)
                                && *((*tx2).mt_spill_pgs).offset(j as isize) == pn
                            {
                                (*dp)
                                    .mp_flags = ((*dp).mp_flags as libc::c_int
                                    | 0x8000 as libc::c_int) as uint16_t;
                                break;
                            }
                        }
                        tx2 = (*tx2).mt_parent;
                    }
                    if !tx2.is_null() {
                        current_block = 13472856163611868459;
                    } else {
                        current_block = 8180496224585318153;
                    }
                } else {
                    current_block = 8180496224585318153;
                }
                match current_block {
                    13472856163611868459 => {}
                    _ => {
                        rc = mdb_midl_append(&mut (*txn).mt_spill_pgs, pn);
                        if rc != 0 {
                            current_block = 15901060405358920724;
                            break;
                        }
                        need = need.wrapping_sub(1);
                        need;
                    }
                }
            }
            i = i.wrapping_sub(1);
            i;
        }
        match current_block {
            15901060405358920724 => {}
            _ => {
                mdb_midl_sort((*txn).mt_spill_pgs);
                rc = mdb_page_flush(txn, i as libc::c_int);
                if !(rc != 0 as libc::c_int) {
                    rc = mdb_pages_xkeep(
                        m0,
                        (0x10 as libc::c_int | 0x8000 as libc::c_int) as libc::c_uint,
                        i as libc::c_int,
                    );
                }
            }
        }
    }
    (*txn).mt_flags
        |= (if rc != 0 { 0x2 as libc::c_int } else { 0x8 as libc::c_int })
            as libc::c_uint;
    return rc;
}
unsafe extern "C" fn mdb_find_oldest(mut txn: *mut MDB_txn) -> txnid_t {
    let mut i: libc::c_int = 0;
    let mut mr: txnid_t = 0;
    let mut oldest: txnid_t = ((*txn).mt_txnid)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if !((*(*txn).mt_env).me_txns).is_null() {
        let mut r: *mut MDB_reader = ((*(*(*txn).mt_env).me_txns).mti_readers)
            .as_mut_ptr();
        i = (*(*(*txn).mt_env).me_txns).mt1.mtb.mtb_numreaders as libc::c_int;
        loop {
            i -= 1;
            if !(i >= 0 as libc::c_int) {
                break;
            }
            if (*r.offset(i as isize)).mru.mrx.mrb_pid != 0 {
                mr = (*r.offset(i as isize)).mru.mrx.mrb_txnid;
                if oldest > mr {
                    oldest = mr;
                }
            }
        }
    }
    return oldest;
}
unsafe extern "C" fn mdb_page_dirty(mut txn: *mut MDB_txn, mut mp: *mut MDB_page) {
    let mut mid: MDB_ID2 = MDB_ID2 {
        mid: 0,
        mptr: 0 as *mut libc::c_void,
    };
    let mut rc: libc::c_int = 0;
    let mut insert: Option::<
        unsafe extern "C" fn(MDB_ID2L, *mut MDB_ID2) -> libc::c_int,
    > = None;
    if (*txn).mt_flags & 0x80000 as libc::c_int as libc::c_uint != 0 {
        insert = Some(
            mdb_mid2l_append
                as unsafe extern "C" fn(MDB_ID2L, *mut MDB_ID2) -> libc::c_int,
        );
    } else {
        insert = Some(
            mdb_mid2l_insert
                as unsafe extern "C" fn(MDB_ID2L, *mut MDB_ID2) -> libc::c_int,
        );
    }
    mid.mid = (*mp).mp_p.p_pgno;
    mid.mptr = mp as *mut libc::c_void;
    rc = insert.unwrap()((*txn).mt_u.dirty_list, &mut mid);
    if rc == 0 as libc::c_int {} else {
        mdb_assert_fail(
            (*txn).mt_env,
            b"rc == 0\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"mdb_page_dirty\0"))
                .as_ptr(),
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            2412 as libc::c_int,
        );
    };
    (*txn).mt_dirty_room = ((*txn).mt_dirty_room).wrapping_sub(1);
    (*txn).mt_dirty_room;
}
unsafe extern "C" fn mdb_page_alloc(
    mut mc: *mut MDB_cursor,
    mut num: libc::c_int,
    mut mp: *mut *mut MDB_page,
) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut retry: libc::c_int = num * 60 as libc::c_int;
    let mut txn: *mut MDB_txn = (*mc).mc_txn;
    let mut env: *mut MDB_env = (*txn).mt_env;
    let mut pgno: pgno_t = 0;
    let mut mop: *mut pgno_t = (*env).me_pgstate.mf_pghead;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut mop_len: libc::c_uint = (if !mop.is_null() {
        *mop.offset(0 as libc::c_int as isize)
    } else {
        0 as libc::c_int as libc::c_ulong
    }) as libc::c_uint;
    let mut n2: libc::c_uint = (num - 1 as libc::c_int) as libc::c_uint;
    let mut np: *mut MDB_page = 0 as *mut MDB_page;
    let mut oldest: txnid_t = 0 as libc::c_int as txnid_t;
    let mut last: txnid_t = 0;
    let mut op: MDB_cursor_op = MDB_FIRST;
    let mut m2: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut found_old: libc::c_int = 0 as libc::c_int;
    if num == 1 as libc::c_int && !((*txn).mt_loose_pgs).is_null() {
        np = (*txn).mt_loose_pgs;
        (*txn)
            .mt_loose_pgs = *(np.offset(2 as libc::c_int as isize)
            as *mut *mut MDB_page);
        (*txn).mt_loose_count -= 1;
        (*txn).mt_loose_count;
        *mp = np;
        return 0 as libc::c_int;
    }
    *mp = 0 as *mut MDB_page;
    if (*txn).mt_dirty_room == 0 as libc::c_int as libc::c_uint {
        rc = -(30788 as libc::c_int);
    } else {
        op = MDB_FIRST;
        's_69: loop {
            let mut key: MDB_val = MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            };
            let mut data: MDB_val = MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            };
            let mut leaf: *mut MDB_node = 0 as *mut MDB_node;
            let mut idl: *mut pgno_t = 0 as *mut pgno_t;
            if mop_len > n2 {
                i = mop_len;
                loop {
                    pgno = *mop.offset(i as isize);
                    if *mop.offset(i.wrapping_sub(n2) as isize)
                        == pgno.wrapping_add(n2 as libc::c_ulong)
                    {
                        current_block = 15517219653170843842;
                        break 's_69;
                    }
                    i = i.wrapping_sub(1);
                    if !(i > n2) {
                        break;
                    }
                }
                retry -= 1;
                if retry < 0 as libc::c_int {
                    current_block = 6721012065216013753;
                    break;
                }
            }
            if op as libc::c_uint == MDB_FIRST as libc::c_int as libc::c_uint {
                last = (*env).me_pgstate.mf_pglast;
                oldest = (*env).me_pgoldest;
                mdb_cursor_init(
                    &mut m2,
                    txn,
                    0 as libc::c_int as MDB_dbi,
                    0 as *mut MDB_xcursor,
                );
                if last != 0 {
                    op = MDB_SET_RANGE;
                    key.mv_data = &mut last as *mut txnid_t as *mut libc::c_void;
                    key.mv_size = ::std::mem::size_of::<txnid_t>() as libc::c_ulong;
                }
                if Paranoid as libc::c_int != 0
                    && (*mc).mc_dbi == 0 as libc::c_int as libc::c_uint
                {
                    retry = -(1 as libc::c_int);
                }
            }
            if Paranoid as libc::c_int != 0 && retry < 0 as libc::c_int && mop_len != 0 {
                current_block = 6721012065216013753;
                break;
            }
            last = last.wrapping_add(1);
            last;
            if oldest <= last {
                if found_old == 0 {
                    oldest = mdb_find_oldest(txn);
                    (*env).me_pgoldest = oldest;
                    found_old = 1 as libc::c_int;
                }
                if oldest <= last {
                    current_block = 6721012065216013753;
                    break;
                }
            }
            rc = mdb_cursor_get(&mut m2, &mut key, 0 as *mut MDB_val, op);
            if rc != 0 {
                if rc == -(30798 as libc::c_int) {
                    current_block = 6721012065216013753;
                    break;
                } else {
                    current_block = 7542567549184679337;
                    break;
                }
            } else {
                last = *(key.mv_data as *mut txnid_t);
                if oldest <= last {
                    if found_old == 0 {
                        oldest = mdb_find_oldest(txn);
                        (*env).me_pgoldest = oldest;
                        found_old = 1 as libc::c_int;
                    }
                    if oldest <= last {
                        current_block = 6721012065216013753;
                        break;
                    }
                }
                np = m2.mc_pg[m2.mc_top as usize];
                leaf = (np as *mut libc::c_char)
                    .offset(
                        *((*np).mp_ptrs)
                            .as_mut_ptr()
                            .offset(m2.mc_ki[m2.mc_top as usize] as isize) as libc::c_int
                            as isize,
                    )
                    .offset(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                rc = mdb_node_read(&mut m2, leaf, &mut data);
                if rc != 0 as libc::c_int {
                    current_block = 7542567549184679337;
                    break;
                }
                idl = data.mv_data as *mut MDB_ID;
                i = *idl.offset(0 as libc::c_int as isize) as libc::c_uint;
                if mop.is_null() {
                    mop = mdb_midl_alloc(i as libc::c_int);
                    (*env).me_pgstate.mf_pghead = mop;
                    if ((*env).me_pgstate.mf_pghead).is_null() {
                        rc = 12 as libc::c_int;
                        current_block = 7542567549184679337;
                        break;
                    }
                } else {
                    rc = mdb_midl_need(&mut (*env).me_pgstate.mf_pghead, i);
                    if rc != 0 as libc::c_int {
                        current_block = 7542567549184679337;
                        break;
                    }
                    mop = (*env).me_pgstate.mf_pghead;
                }
                (*env).me_pgstate.mf_pglast = last;
                mdb_midl_xmerge(mop, idl);
                mop_len = *mop.offset(0 as libc::c_int as isize) as libc::c_uint;
                op = MDB_NEXT;
            }
        }
        match current_block {
            7542567549184679337 => {}
            _ => {
                match current_block {
                    6721012065216013753 => {
                        i = 0 as libc::c_int as libc::c_uint;
                        pgno = (*txn).mt_next_pgno;
                        if pgno.wrapping_add(num as libc::c_ulong) >= (*env).me_maxpg {
                            rc = -(30792 as libc::c_int);
                            current_block = 7542567549184679337;
                        } else {
                            current_block = 15517219653170843842;
                        }
                    }
                    _ => {}
                }
                match current_block {
                    7542567549184679337 => {}
                    _ => {
                        if (*env).me_flags & 0x80000 as libc::c_int as libc::c_uint != 0
                        {
                            np = ((*env).me_map)
                                .offset(
                                    ((*env).me_psize as libc::c_ulong).wrapping_mul(pgno)
                                        as isize,
                                ) as *mut MDB_page;
                            current_block = 16779030619667747692;
                        } else {
                            np = mdb_page_malloc(txn, num as libc::c_uint);
                            if np.is_null() {
                                rc = 12 as libc::c_int;
                                current_block = 7542567549184679337;
                            } else {
                                current_block = 16779030619667747692;
                            }
                        }
                        match current_block {
                            7542567549184679337 => {}
                            _ => {
                                if i != 0 {
                                    mop_len = mop_len.wrapping_sub(num as libc::c_uint);
                                    *mop.offset(0 as libc::c_int as isize) = mop_len as pgno_t;
                                    j = i.wrapping_sub(num as libc::c_uint);
                                    while j < mop_len {
                                        i = i.wrapping_add(1);
                                        j = j.wrapping_add(1);
                                        *mop.offset(j as isize) = *mop.offset(i as isize);
                                    }
                                } else {
                                    (*txn)
                                        .mt_next_pgno = pgno.wrapping_add(num as libc::c_ulong);
                                }
                                (*np).mp_p.p_pgno = pgno;
                                mdb_page_dirty(txn, np);
                                *mp = np;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    (*txn).mt_flags |= 0x2 as libc::c_int as libc::c_uint;
    return rc;
}
unsafe extern "C" fn mdb_page_copy(
    mut dst: *mut MDB_page,
    mut src: *mut MDB_page,
    mut psize: libc::c_uint,
) {
    let mut upper: indx_t = (*src).mp_pb.pb.pb_upper;
    let mut lower: indx_t = (*src).mp_pb.pb.pb_lower;
    let mut unused: indx_t = (upper as libc::c_int - lower as libc::c_int) as indx_t;
    unused = (unused as libc::c_int & -(Align as libc::c_int)) as indx_t;
    if unused as libc::c_int != 0
        && !((*src).mp_flags as libc::c_int & 0x20 as libc::c_int == 0x20 as libc::c_int)
    {
        upper = ((upper as libc::c_uint)
            .wrapping_add(
                (if 0 as libc::c_int != 0 {
                    16 as libc::c_ulong as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }),
            ) & -(Align as libc::c_int) as libc::c_uint) as indx_t;
        memcpy(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            ((lower as libc::c_uint)
                .wrapping_add(
                    (if 0 as libc::c_int != 0 {
                        16 as libc::c_ulong as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                )
                .wrapping_add((Align as libc::c_int - 1 as libc::c_int) as libc::c_uint)
                & -(Align as libc::c_int) as libc::c_uint) as libc::c_ulong,
        );
        memcpy(
            (dst as *mut libc::c_char).offset(upper as libc::c_int as isize)
                as *mut pgno_t as *mut libc::c_void,
            (src as *mut libc::c_char).offset(upper as libc::c_int as isize)
                as *mut pgno_t as *const libc::c_void,
            psize.wrapping_sub(upper as libc::c_uint) as libc::c_ulong,
        );
    } else {
        memcpy(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            psize.wrapping_sub(unused as libc::c_uint) as libc::c_ulong,
        );
    };
}
unsafe extern "C" fn mdb_page_unspill(
    mut txn: *mut MDB_txn,
    mut mp: *mut MDB_page,
    mut ret: *mut *mut MDB_page,
) -> libc::c_int {
    let mut env: *mut MDB_env = (*txn).mt_env;
    let mut tx2: *const MDB_txn = 0 as *const MDB_txn;
    let mut x: libc::c_uint = 0;
    let mut pgno: pgno_t = (*mp).mp_p.p_pgno;
    let mut pn: pgno_t = pgno << 1 as libc::c_int;
    tx2 = txn;
    while !tx2.is_null() {
        if !((*tx2).mt_spill_pgs).is_null() {
            x = mdb_midl_search((*tx2).mt_spill_pgs, pn);
            if x as libc::c_ulong
                <= *((*tx2).mt_spill_pgs).offset(0 as libc::c_int as isize)
                && *((*tx2).mt_spill_pgs).offset(x as isize) == pn
            {
                let mut np: *mut MDB_page = 0 as *mut MDB_page;
                let mut num: libc::c_int = 0;
                if (*txn).mt_dirty_room == 0 as libc::c_int as libc::c_uint {
                    return -(30788 as libc::c_int);
                }
                if (*mp).mp_flags as libc::c_int & 0x4 as libc::c_int
                    == 0x4 as libc::c_int
                {
                    num = (*mp).mp_pb.pb_pages as libc::c_int;
                } else {
                    num = 1 as libc::c_int;
                }
                if (*env).me_flags & 0x80000 as libc::c_int as libc::c_uint != 0 {
                    np = mp;
                } else {
                    np = mdb_page_malloc(txn, num as libc::c_uint);
                    if np.is_null() {
                        return 12 as libc::c_int;
                    }
                    if num > 1 as libc::c_int {
                        memcpy(
                            np as *mut libc::c_void,
                            mp as *const libc::c_void,
                            (num as libc::c_uint).wrapping_mul((*env).me_psize)
                                as libc::c_ulong,
                        );
                    } else {
                        mdb_page_copy(np, mp, (*env).me_psize);
                    }
                }
                if tx2 == txn as *const MDB_txn {
                    if x as libc::c_ulong
                        == *((*txn).mt_spill_pgs).offset(0 as libc::c_int as isize)
                    {
                        let ref mut fresh1 = *((*txn).mt_spill_pgs)
                            .offset(0 as libc::c_int as isize);
                        *fresh1 = (*fresh1).wrapping_sub(1);
                        *fresh1;
                    } else {
                        let ref mut fresh2 = *((*txn).mt_spill_pgs).offset(x as isize);
                        *fresh2 |= 1 as libc::c_int as libc::c_ulong;
                    }
                }
                mdb_page_dirty(txn, np);
                (*np)
                    .mp_flags = ((*np).mp_flags as libc::c_int | 0x10 as libc::c_int)
                    as uint16_t;
                *ret = np;
                break;
            }
        }
        tx2 = (*tx2).mt_parent;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_page_touch(mut mc: *mut MDB_cursor) -> libc::c_int {
    let mut current_block: u64;
    let mut mp: *mut MDB_page = (*mc).mc_pg[(*mc).mc_top as usize];
    let mut np: *mut MDB_page = 0 as *mut MDB_page;
    let mut txn: *mut MDB_txn = (*mc).mc_txn;
    let mut m2: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut m3: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut pgno: pgno_t = 0;
    let mut rc: libc::c_int = 0;
    if !((*mp).mp_flags as libc::c_int & 0x10 as libc::c_int == 0x10 as libc::c_int) {
        if (*txn).mt_flags & 0x8 as libc::c_int as libc::c_uint != 0 {
            np = 0 as *mut MDB_page;
            rc = mdb_page_unspill(txn, mp, &mut np);
            if rc != 0 {
                current_block = 17133093213338615676;
            } else if !np.is_null() {
                current_block = 5655102413987502650;
            } else {
                current_block = 11006700562992250127;
            }
        } else {
            current_block = 11006700562992250127;
        }
        match current_block {
            5655102413987502650 => {}
            _ => {
                match current_block {
                    11006700562992250127 => {
                        rc = mdb_midl_need(
                            &mut (*txn).mt_free_pgs,
                            1 as libc::c_int as libc::c_uint,
                        );
                        if rc != 0
                            || {
                                rc = mdb_page_alloc(mc, 1 as libc::c_int, &mut np);
                                rc != 0
                            }
                        {
                            current_block = 17133093213338615676;
                        } else {
                            pgno = (*np).mp_p.p_pgno;
                            if (*mp).mp_p.p_pgno != pgno {} else {
                                mdb_assert_fail(
                                    (*(*mc).mc_txn).mt_env,
                                    b"mp->mp_pgno != pgno\0" as *const u8
                                        as *const libc::c_char,
                                    (*::std::mem::transmute::<
                                        &[u8; 15],
                                        &[libc::c_char; 15],
                                    >(b"mdb_page_touch\0"))
                                        .as_ptr(),
                                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                                    2741 as libc::c_int,
                                );
                            };
                            let mut xidl: *mut MDB_ID = (*txn).mt_free_pgs;
                            let ref mut fresh3 = *xidl.offset(0 as libc::c_int as isize);
                            *fresh3 = (*fresh3).wrapping_add(1);
                            let mut xlen: MDB_ID = *fresh3;
                            *xidl.offset(xlen as isize) = (*mp).mp_p.p_pgno;
                            if (*mc).mc_top != 0 {
                                let mut parent: *mut MDB_page = (*mc)
                                    .mc_pg[((*mc).mc_top as libc::c_int - 1 as libc::c_int)
                                    as usize];
                                let mut node: *mut MDB_node = (parent as *mut libc::c_char)
                                    .offset(
                                        *((*parent).mp_ptrs)
                                            .as_mut_ptr()
                                            .offset(
                                                (*mc)
                                                    .mc_ki[((*mc).mc_top as libc::c_int - 1 as libc::c_int)
                                                    as usize] as isize,
                                            ) as libc::c_int as isize,
                                    )
                                    .offset(
                                        (if 0 as libc::c_int != 0 {
                                            16 as libc::c_ulong as libc::c_uint
                                        } else {
                                            0 as libc::c_int as libc::c_uint
                                        }) as isize,
                                    ) as *mut MDB_node;
                                (*node)
                                    .mn_lo = (pgno & 0xffff as libc::c_int as libc::c_ulong)
                                    as libc::c_ushort;
                                (*node)
                                    .mn_hi = (pgno >> 16 as libc::c_int) as libc::c_ushort;
                                if if -(1 as libc::c_int) as pgno_t
                                    > 0xffffffff as libc::c_uint as libc::c_ulong
                                {
                                    32 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                } != 0
                                {
                                    (*node)
                                        .mn_flags = (pgno
                                        >> (if -(1 as libc::c_int) as pgno_t
                                            > 0xffffffff as libc::c_uint as libc::c_ulong
                                        {
                                            32 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        })) as libc::c_ushort;
                                }
                            } else {
                                (*(*mc).mc_db).md_root = pgno;
                            }
                            current_block = 13131896068329595644;
                        }
                    }
                    _ => {}
                }
                match current_block {
                    13131896068329595644 => {}
                    _ => {
                        (*txn).mt_flags |= 0x2 as libc::c_int as libc::c_uint;
                        return rc;
                    }
                }
            }
        }
    } else {
        if !((*txn).mt_parent).is_null()
            && !((*mp).mp_flags as libc::c_int & 0x40 as libc::c_int
                == 0x40 as libc::c_int)
        {
            let mut mid: MDB_ID2 = MDB_ID2 {
                mid: 0,
                mptr: 0 as *mut libc::c_void,
            };
            let mut dl: *mut MDB_ID2 = (*txn).mt_u.dirty_list;
            pgno = (*mp).mp_p.p_pgno;
            if (*dl.offset(0 as libc::c_int as isize)).mid != 0 {
                let mut x: libc::c_uint = mdb_mid2l_search(dl, pgno);
                if x as libc::c_ulong <= (*dl.offset(0 as libc::c_int as isize)).mid
                    && (*dl.offset(x as isize)).mid == pgno
                {
                    if mp != (*dl.offset(x as isize)).mptr as *mut MDB_page {
                        (*mc).mc_flags
                            &= !(0x1 as libc::c_int | 0x2 as libc::c_int)
                                as libc::c_uint;
                        (*txn).mt_flags |= 0x2 as libc::c_int as libc::c_uint;
                        return -(30779 as libc::c_int);
                    }
                    return 0 as libc::c_int;
                }
            }
            if (*dl.offset(0 as libc::c_int as isize)).mid
                < (((1 as libc::c_int) << 16 as libc::c_int + 1 as libc::c_int)
                    - 1 as libc::c_int) as libc::c_ulong
            {} else {
                mdb_assert_fail(
                    (*(*mc).mc_txn).mt_env,
                    b"dl[0].mid < MDB_IDL_UM_MAX\0" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<
                        &[u8; 15],
                        &[libc::c_char; 15],
                    >(b"mdb_page_touch\0"))
                        .as_ptr(),
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    2768 as libc::c_int,
                );
            };
            np = mdb_page_malloc(txn, 1 as libc::c_int as libc::c_uint);
            if np.is_null() {
                return 12 as libc::c_int;
            }
            mid.mid = pgno;
            mid.mptr = np as *mut libc::c_void;
            rc = mdb_mid2l_insert(dl, &mut mid);
            if rc == 0 as libc::c_int {} else {
                mdb_assert_fail(
                    (*(*mc).mc_txn).mt_env,
                    b"rc == 0\0" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<
                        &[u8; 15],
                        &[libc::c_char; 15],
                    >(b"mdb_page_touch\0"))
                        .as_ptr(),
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    2776 as libc::c_int,
                );
            };
        } else {
            return 0 as libc::c_int
        }
        current_block = 13131896068329595644;
    }
    match current_block {
        13131896068329595644 => {
            mdb_page_copy(np, mp, (*(*txn).mt_env).me_psize);
            (*np).mp_p.p_pgno = pgno;
            (*np)
                .mp_flags = ((*np).mp_flags as libc::c_int | 0x10 as libc::c_int)
                as uint16_t;
        }
        _ => {}
    }
    (*mc).mc_pg[(*mc).mc_top as usize] = np;
    m2 = *((*txn).mt_cursors).offset((*mc).mc_dbi as isize);
    if (*mc).mc_flags & 0x4 as libc::c_int as libc::c_uint != 0 {
        while !m2.is_null() {
            m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
            if !(((*m3).mc_snum as libc::c_int) < (*mc).mc_snum as libc::c_int) {
                if (*m3).mc_pg[(*mc).mc_top as usize] == mp {
                    (*m3).mc_pg[(*mc).mc_top as usize] = np;
                }
            }
            m2 = (*m2).mc_next;
        }
    } else {
        while !m2.is_null() {
            if !(((*m2).mc_snum as libc::c_int) < (*mc).mc_snum as libc::c_int) {
                if !(m2 == mc) {
                    if (*m2).mc_pg[(*mc).mc_top as usize] == mp {
                        (*m2).mc_pg[(*mc).mc_top as usize] = np;
                        if (*np).mp_flags as libc::c_int & 0x2 as libc::c_int
                            == 0x2 as libc::c_int
                        {
                            let mut xr_pg: *mut MDB_page = np;
                            let mut xr_node: *mut MDB_node = 0 as *mut MDB_node;
                            if !(!(!((*m2).mc_xcursor).is_null()
                                && (*(*m2).mc_xcursor).mx_cursor.mc_flags
                                    & 0x1 as libc::c_int as libc::c_uint != 0)
                                || (*m2).mc_ki[(*mc).mc_top as usize] as libc::c_uint
                                    >= ((*xr_pg).mp_pb.pb.pb_lower as libc::c_uint)
                                        .wrapping_sub(
                                            (16 as libc::c_ulong as libc::c_uint)
                                                .wrapping_sub(
                                                    (if 0 as libc::c_int != 0 {
                                                        16 as libc::c_ulong as libc::c_uint
                                                    } else {
                                                        0 as libc::c_int as libc::c_uint
                                                    }),
                                                ),
                                        ) >> 1 as libc::c_int)
                            {
                                xr_node = (xr_pg as *mut libc::c_char)
                                    .offset(
                                        *((*xr_pg).mp_ptrs)
                                            .as_mut_ptr()
                                            .offset((*m2).mc_ki[(*mc).mc_top as usize] as isize)
                                            as libc::c_int as isize,
                                    )
                                    .offset(
                                        (if 0 as libc::c_int != 0 {
                                            16 as libc::c_ulong as libc::c_uint
                                        } else {
                                            0 as libc::c_int as libc::c_uint
                                        }) as isize,
                                    ) as *mut MDB_node;
                                if (*xr_node).mn_flags as libc::c_int
                                    & (0x4 as libc::c_int | 0x2 as libc::c_int)
                                    == 0x4 as libc::c_int
                                {
                                    (*(*m2).mc_xcursor)
                                        .mx_cursor
                                        .mc_pg[0 as libc::c_int
                                        as usize] = ((*xr_node).mn_data)
                                        .as_mut_ptr()
                                        .offset((*xr_node).mn_ksize as libc::c_int as isize)
                                        as *mut libc::c_void as *mut MDB_page;
                                }
                            }
                        }
                    }
                }
            }
            m2 = (*m2).mc_next;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_sync0(
    mut env: *mut MDB_env,
    mut force: libc::c_int,
    mut numpgs: pgno_t,
) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    if (*env).me_flags & 0x20000 as libc::c_int as libc::c_uint != 0 {
        return 13 as libc::c_int;
    }
    if force != 0 || (*env).me_flags & 0x10000 as libc::c_int as libc::c_uint == 0 {
        if (*env).me_flags & 0x80000 as libc::c_int as libc::c_uint != 0 {
            let mut flags: libc::c_int = if (*env).me_flags
                & 0x100000 as libc::c_int as libc::c_uint != 0 && force == 0
            {
                1 as libc::c_int
            } else {
                4 as libc::c_int
            };
            if msync(
                (*env).me_map as *mut libc::c_void,
                ((*env).me_psize as libc::c_ulong).wrapping_mul(numpgs),
                flags,
            ) != 0
            {
                rc = *__errno_location();
            }
        } else if (*env).me_flags & 0x8000000 as libc::c_uint != 0 {
            if fsync((*env).me_fd) != 0 {
                rc = *__errno_location();
            }
        } else if fdatasync((*env).me_fd) != 0 {
            rc = *__errno_location();
        }
    }
    return rc;
}
pub unsafe extern "C" fn mdb_env_sync(
    mut env: *mut MDB_env,
    mut force: libc::c_int,
) -> libc::c_int {
    let mut m: *mut MDB_meta = mdb_env_pick_meta(env);
    return mdb_env_sync0(
        env,
        force,
        ((*m).mm_last_pg).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
}
unsafe extern "C" fn mdb_cursor_shadow(
    mut src: *mut MDB_txn,
    mut dst: *mut MDB_txn,
) -> libc::c_int {
    let mut mc: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut bk: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut mx: *mut MDB_xcursor = 0 as *mut MDB_xcursor;
    let mut size: size_t = 0;
    let mut i: libc::c_int = 0;
    i = (*src).mt_numdbs as libc::c_int;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        mc = *((*src).mt_cursors).offset(i as isize);
        if !mc.is_null() {
            size = ::std::mem::size_of::<MDB_cursor>() as libc::c_ulong;
            if !((*mc).mc_xcursor).is_null() {
                size = (size as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<MDB_xcursor>() as libc::c_ulong)
                    as size_t as size_t;
            }
            while !mc.is_null() {
                bk = malloc(size) as *mut MDB_cursor;
                if bk.is_null() {
                    return 12 as libc::c_int;
                }
                *bk = *mc;
                (*mc).mc_backup = bk;
                (*mc).mc_db = &mut *((*dst).mt_dbs).offset(i as isize) as *mut MDB_db;
                (*mc).mc_txn = dst;
                (*mc)
                    .mc_dbflag = &mut *((*dst).mt_dbflags).offset(i as isize)
                    as *mut libc::c_uchar;
                mx = (*mc).mc_xcursor;
                if !mx.is_null() {
                    *(bk.offset(1 as libc::c_int as isize) as *mut MDB_xcursor) = *mx;
                    (*mx).mx_cursor.mc_txn = dst;
                }
                (*mc).mc_next = *((*dst).mt_cursors).offset(i as isize);
                let ref mut fresh4 = *((*dst).mt_cursors).offset(i as isize);
                *fresh4 = mc;
                mc = (*bk).mc_next;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_cursors_close(mut txn: *mut MDB_txn, mut merge: libc::c_uint) {
    let mut cursors: *mut *mut MDB_cursor = (*txn).mt_cursors;
    let mut mc: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut next: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut bk: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut mx: *mut MDB_xcursor = 0 as *mut MDB_xcursor;
    let mut i: libc::c_int = 0;
    i = (*txn).mt_numdbs as libc::c_int;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        mc = *cursors.offset(i as isize);
        while !mc.is_null() {
            next = (*mc).mc_next;
            bk = (*mc).mc_backup;
            if !bk.is_null() {
                if merge != 0 {
                    (*mc).mc_next = (*bk).mc_next;
                    (*mc).mc_backup = (*bk).mc_backup;
                    (*mc).mc_txn = (*bk).mc_txn;
                    (*mc).mc_db = (*bk).mc_db;
                    (*mc).mc_dbflag = (*bk).mc_dbflag;
                    mx = (*mc).mc_xcursor;
                    if !mx.is_null() {
                        (*mx).mx_cursor.mc_txn = (*bk).mc_txn;
                    }
                } else {
                    *mc = *bk;
                    mx = (*mc).mc_xcursor;
                    if !mx.is_null() {
                        *mx = *(bk.offset(1 as libc::c_int as isize)
                            as *mut MDB_xcursor);
                    }
                }
                mc = bk;
            }
            free(mc as *mut libc::c_void);
            mc = next;
        }
        let ref mut fresh5 = *cursors.offset(i as isize);
        *fresh5 = 0 as *mut MDB_cursor;
    };
}
unsafe extern "C" fn mdb_reader_pid(
    mut env: *mut MDB_env,
    mut op: Pidlock_op,
    mut pid: pid_t,
) -> libc::c_int {
    loop {
        let mut rc: libc::c_int = 0;
        let mut lock_info: flock = flock {
            l_type: 0,
            l_whence: 0,
            l_start: 0,
            l_len: 0,
            l_pid: 0,
        };
        memset(
            &mut lock_info as *mut flock as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<flock>() as libc::c_ulong,
        );
        lock_info.l_type = 1 as libc::c_int as libc::c_short;
        lock_info.l_whence = 0 as libc::c_int as libc::c_short;
        lock_info.l_start = pid as __off_t;
        lock_info.l_len = 1 as libc::c_int as __off_t;
        rc = fcntl((*env).me_lfd, op as libc::c_int, &mut lock_info as *mut flock);
        if rc == 0 as libc::c_int {
            if op as libc::c_uint == 5 as libc::c_int as libc::c_uint
                && lock_info.l_type as libc::c_int != 2 as libc::c_int
            {
                rc = -(1 as libc::c_int);
            }
        } else {
            rc = *__errno_location();
            if rc == 4 as libc::c_int {
                continue;
            }
        }
        return rc;
    };
}
unsafe extern "C" fn mdb_txn_renew0(mut txn: *mut MDB_txn) -> libc::c_int {
    let mut env: *mut MDB_env = (*txn).mt_env;
    let mut ti: *mut MDB_txninfo = (*env).me_txns;
    let mut meta: *mut MDB_meta = 0 as *mut MDB_meta;
    let mut i: libc::c_uint = 0;
    let mut nr: libc::c_uint = 0;
    let mut flags: libc::c_uint = (*txn).mt_flags;
    let mut x: uint16_t = 0;
    let mut rc: libc::c_int = 0;
    let mut new_notls: libc::c_int = 0 as libc::c_int;
    flags &= 0x20000 as libc::c_int as libc::c_uint;
    if flags != 0 as libc::c_int as libc::c_uint {
        if ti.is_null() {
            meta = mdb_env_pick_meta(env);
            (*txn).mt_txnid = (*meta).mm_txnid;
            (*txn).mt_u.reader = 0 as *mut MDB_reader;
        } else {
            let mut r: *mut MDB_reader = (if (*env).me_flags
                & 0x200000 as libc::c_int as libc::c_uint != 0
            {
                (*txn).mt_u.reader as *mut libc::c_void
            } else {
                pthread_getspecific((*env).me_txkey)
            }) as *mut MDB_reader;
            if !r.is_null() {
                if (*r).mru.mrx.mrb_pid != (*env).me_pid
                    || (*r).mru.mrx.mrb_txnid != -(1 as libc::c_int) as txnid_t
                {
                    return -(30783 as libc::c_int);
                }
            } else {
                let mut pid: pid_t = (*env).me_pid;
                let mut tid: pthread_t = pthread_self();
                let mut rmutex: mdb_mutexref_t = ((*(*env).me_txns).mt1.mtb.mtb_rmutex)
                    .as_mut_ptr();
                if (*env).me_live_reader == 0 {
                    rc = mdb_reader_pid(env, Pidset, pid);
                    if rc != 0 {
                        return rc;
                    }
                    (*env).me_live_reader = 1 as libc::c_int;
                }
                rc = pthread_mutex_lock(rmutex);
                if rc != 0
                    && {
                        rc = mdb_mutex_failed(env, rmutex, rc);
                        rc != 0
                    }
                {
                    return rc;
                }
                nr = (*ti).mt1.mtb.mtb_numreaders;
                i = 0 as libc::c_int as libc::c_uint;
                while i < nr {
                    if (*((*ti).mti_readers).as_mut_ptr().offset(i as isize))
                        .mru
                        .mrx
                        .mrb_pid == 0 as libc::c_int
                    {
                        break;
                    }
                    i = i.wrapping_add(1);
                    i;
                }
                if i == (*env).me_maxreaders {
                    pthread_mutex_unlock(rmutex);
                    return -(30790 as libc::c_int);
                }
                r = &mut *((*ti).mti_readers).as_mut_ptr().offset(i as isize)
                    as *mut MDB_reader;
                ::std::ptr::write_volatile(
                    &mut (*r).mru.mrx.mrb_pid as *mut pid_t,
                    0 as libc::c_int,
                );
                ::std::ptr::write_volatile(
                    &mut (*r).mru.mrx.mrb_txnid as *mut txnid_t,
                    -(1 as libc::c_int) as txnid_t,
                );
                ::std::ptr::write_volatile(
                    &mut (*r).mru.mrx.mrb_tid as *mut pthread_t,
                    tid,
                );
                if i == nr {
                    nr = nr.wrapping_add(1);
                    ::std::ptr::write_volatile(
                        &mut (*ti).mt1.mtb.mtb_numreaders as *mut libc::c_uint,
                        nr,
                    );
                }
                ::std::ptr::write_volatile(
                    &mut (*env).me_close_readers as *mut libc::c_int,
                    nr as libc::c_int,
                );
                ::std::ptr::write_volatile(&mut (*r).mru.mrx.mrb_pid as *mut pid_t, pid);
                pthread_mutex_unlock(rmutex);
                new_notls = ((*env).me_flags & 0x200000 as libc::c_int as libc::c_uint)
                    as libc::c_int;
                if new_notls == 0
                    && {
                        rc = pthread_setspecific(
                            (*env).me_txkey,
                            r as *const libc::c_void,
                        );
                        rc != 0
                    }
                {
                    ::std::ptr::write_volatile(
                        &mut (*r).mru.mrx.mrb_pid as *mut pid_t,
                        0 as libc::c_int,
                    );
                    return rc;
                }
            }
            loop {
                ::std::ptr::write_volatile(
                    &mut (*r).mru.mrx.mrb_txnid as *mut txnid_t,
                    (*ti).mt1.mtb.mtb_txnid,
                );
                if !((*r).mru.mrx.mrb_txnid != (*ti).mt1.mtb.mtb_txnid) {
                    break;
                }
            }
            (*txn).mt_txnid = (*r).mru.mrx.mrb_txnid;
            (*txn).mt_u.reader = r;
            meta = (*env)
                .me_metas[((*txn).mt_txnid & 1 as libc::c_int as libc::c_ulong)
                as usize];
        }
    } else {
        if !ti.is_null() {
            rc = pthread_mutex_lock(((*(*env).me_txns).mt2.mt2_wmutex).as_mut_ptr());
            if rc != 0
                && {
                    rc = mdb_mutex_failed(
                        env,
                        ((*(*env).me_txns).mt2.mt2_wmutex).as_mut_ptr(),
                        rc,
                    );
                    rc != 0
                }
            {
                return rc;
            }
            (*txn).mt_txnid = (*ti).mt1.mtb.mtb_txnid;
            meta = (*env)
                .me_metas[((*txn).mt_txnid & 1 as libc::c_int as libc::c_ulong)
                as usize];
        } else {
            meta = mdb_env_pick_meta(env);
            (*txn).mt_txnid = (*meta).mm_txnid;
        }
        (*txn).mt_txnid = ((*txn).mt_txnid).wrapping_add(1);
        (*txn).mt_txnid;
        (*txn).mt_child = 0 as *mut MDB_txn;
        (*txn).mt_loose_pgs = 0 as *mut MDB_page;
        (*txn).mt_loose_count = 0 as libc::c_int;
        (*txn)
            .mt_dirty_room = (((1 as libc::c_int)
            << 16 as libc::c_int + 1 as libc::c_int) - 1 as libc::c_int) as libc::c_uint;
        (*txn).mt_u.dirty_list = (*env).me_dirty_list;
        (*((*txn).mt_u.dirty_list).offset(0 as libc::c_int as isize))
            .mid = 0 as libc::c_int as MDB_ID;
        (*txn).mt_free_pgs = (*env).me_free_pgs;
        *((*txn).mt_free_pgs)
            .offset(0 as libc::c_int as isize) = 0 as libc::c_int as MDB_ID;
        (*txn).mt_spill_pgs = 0 as MDB_IDL;
        (*env).me_txn = txn;
        memcpy(
            (*txn).mt_dbiseqs as *mut libc::c_void,
            (*env).me_dbiseqs as *const libc::c_void,
            ((*env).me_maxdbs as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
        );
    }
    memcpy(
        (*txn).mt_dbs as *mut libc::c_void,
        ((*meta).mm_dbs).as_mut_ptr() as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<MDB_db>() as libc::c_ulong),
    );
    (*txn)
        .mt_next_pgno = ((*meta).mm_last_pg)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    (*txn).mt_flags = flags;
    (*txn).mt_numdbs = (*env).me_numdbs;
    i = 2 as libc::c_int as libc::c_uint;
    while i < (*txn).mt_numdbs {
        x = *((*env).me_dbflags).offset(i as isize);
        (*((*txn).mt_dbs).offset(i as isize))
            .md_flags = (x as libc::c_int
            & (0xffff as libc::c_int & !(0x8000 as libc::c_int))) as uint16_t;
        *((*txn).mt_dbflags)
            .offset(
                i as isize,
            ) = (if x as libc::c_int & 0x8000 as libc::c_int != 0 {
            0x8 as libc::c_int | 0x10 as libc::c_int | 0x2 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    *((*txn).mt_dbflags)
        .offset(
            1 as libc::c_int as isize,
        ) = (0x8 as libc::c_int | 0x10 as libc::c_int) as libc::c_uchar;
    *((*txn).mt_dbflags)
        .offset(0 as libc::c_int as isize) = 0x8 as libc::c_int as libc::c_uchar;
    if (*env).me_flags & 0x80000000 as libc::c_uint != 0 {
        rc = -(30795 as libc::c_int);
    } else if (*env).me_maxpg < (*txn).mt_next_pgno {
        rc = -(30785 as libc::c_int);
    } else {
        return 0 as libc::c_int
    }
    mdb_txn_end(txn, (new_notls | MDB_END_FAIL_BEGIN as libc::c_int) as libc::c_uint);
    return rc;
}
pub unsafe extern "C" fn mdb_txn_renew(mut txn: *mut MDB_txn) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    if txn.is_null()
        || !((*txn).mt_flags
            & (0x20000 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint
            == (0x20000 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint)
    {
        return 22 as libc::c_int;
    }
    rc = mdb_txn_renew0(txn);
    rc == 0 as libc::c_int;
    return rc;
}
pub unsafe extern "C" fn mdb_txn_begin(
    mut env: *mut MDB_env,
    mut parent: *mut MDB_txn,
    mut flags: libc::c_uint,
    mut ret: *mut *mut MDB_txn,
) -> libc::c_int {
    let mut current_block: u64;
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut ntxn: *mut MDB_ntxn = 0 as *mut MDB_ntxn;
    let mut rc: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut tsize: libc::c_int = 0;
    flags
        &= (0x40000 as libc::c_int | 0x10000 as libc::c_int | 0x20000 as libc::c_int)
            as libc::c_uint;
    flags |= (*env).me_flags & 0x80000 as libc::c_int as libc::c_uint;
    if (*env).me_flags & 0x20000 as libc::c_int as libc::c_uint & !flags != 0 {
        return 13 as libc::c_int;
    }
    if !parent.is_null() {
        flags |= (*parent).mt_flags;
        if flags
            & (0x20000 as libc::c_int | 0x80000 as libc::c_int
                | (0x1 as libc::c_int | 0x2 as libc::c_int | 0x10 as libc::c_int))
                as libc::c_uint != 0
        {
            return if (*parent).mt_flags & 0x20000 as libc::c_int as libc::c_uint != 0 {
                22 as libc::c_int
            } else {
                -(30782 as libc::c_int)
            };
        }
        size = ((*env).me_maxdbs as libc::c_ulong)
            .wrapping_mul(
                (::std::mem::size_of::<MDB_db>() as libc::c_ulong)
                    .wrapping_add(
                        ::std::mem::size_of::<*mut MDB_cursor>() as libc::c_ulong,
                    )
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_int;
        tsize = ::std::mem::size_of::<MDB_ntxn>() as libc::c_ulong as libc::c_int;
        size += tsize;
        current_block = 12800627514080957624;
    } else if flags & 0x20000 as libc::c_int as libc::c_uint != 0 {
        size = ((*env).me_maxdbs as libc::c_ulong)
            .wrapping_mul(
                (::std::mem::size_of::<MDB_db>() as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_int;
        tsize = ::std::mem::size_of::<MDB_txn>() as libc::c_ulong as libc::c_int;
        size += tsize;
        current_block = 12800627514080957624;
    } else {
        txn = (*env).me_txn0;
        current_block = 13766231232807252387;
    }
    match current_block {
        12800627514080957624 => {
            txn = calloc(1 as libc::c_int as libc::c_ulong, size as libc::c_ulong)
                as *mut MDB_txn;
            if txn.is_null() {
                return 12 as libc::c_int;
            }
            (*txn).mt_dbxs = (*env).me_dbxs;
            (*txn)
                .mt_dbs = (txn as *mut libc::c_char).offset(tsize as isize)
                as *mut MDB_db;
            (*txn)
                .mt_dbflags = (txn as *mut libc::c_uchar)
                .offset(size as isize)
                .offset(-((*env).me_maxdbs as isize));
            (*txn).mt_flags = flags;
            (*txn).mt_env = env;
            if !parent.is_null() {
                let mut i: libc::c_uint = 0;
                (*txn)
                    .mt_cursors = ((*txn).mt_dbs).offset((*env).me_maxdbs as isize)
                    as *mut *mut MDB_cursor;
                (*txn).mt_dbiseqs = (*parent).mt_dbiseqs;
                (*txn)
                    .mt_u
                    .dirty_list = malloc(
                    (::std::mem::size_of::<MDB_ID2>() as libc::c_ulong)
                        .wrapping_mul(
                            ((1 as libc::c_int) << 16 as libc::c_int + 1 as libc::c_int)
                                as libc::c_ulong,
                        ),
                ) as MDB_ID2L;
                if ((*txn).mt_u.dirty_list).is_null()
                    || {
                        (*txn)
                            .mt_free_pgs = mdb_midl_alloc(
                            ((1 as libc::c_int) << 16 as libc::c_int + 1 as libc::c_int)
                                - 1 as libc::c_int,
                        );
                        ((*txn).mt_free_pgs).is_null()
                    }
                {
                    free((*txn).mt_u.dirty_list as *mut libc::c_void);
                    free(txn as *mut libc::c_void);
                    return 12 as libc::c_int;
                }
                (*txn).mt_txnid = (*parent).mt_txnid;
                (*txn).mt_dirty_room = (*parent).mt_dirty_room;
                (*((*txn).mt_u.dirty_list).offset(0 as libc::c_int as isize))
                    .mid = 0 as libc::c_int as MDB_ID;
                (*txn).mt_spill_pgs = 0 as MDB_IDL;
                (*txn).mt_next_pgno = (*parent).mt_next_pgno;
                (*parent).mt_flags |= 0x10 as libc::c_int as libc::c_uint;
                (*parent).mt_child = txn;
                (*txn).mt_parent = parent;
                (*txn).mt_numdbs = (*parent).mt_numdbs;
                memcpy(
                    (*txn).mt_dbs as *mut libc::c_void,
                    (*parent).mt_dbs as *const libc::c_void,
                    ((*txn).mt_numdbs as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<MDB_db>() as libc::c_ulong),
                );
                i = 0 as libc::c_int as libc::c_uint;
                while i < (*txn).mt_numdbs {
                    *((*txn).mt_dbflags)
                        .offset(
                            i as isize,
                        ) = (*((*parent).mt_dbflags).offset(i as isize) as libc::c_int
                        & !(0x4 as libc::c_int)) as libc::c_uchar;
                    i = i.wrapping_add(1);
                    i;
                }
                rc = 0 as libc::c_int;
                ntxn = txn as *mut MDB_ntxn;
                (*ntxn).mnt_pgstate = (*env).me_pgstate;
                if !((*env).me_pgstate.mf_pghead).is_null() {
                    size = (*((*env).me_pgstate.mf_pghead)
                        .offset(0 as libc::c_int as isize))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<MDB_ID>() as libc::c_ulong)
                        as libc::c_int;
                    (*env)
                        .me_pgstate
                        .mf_pghead = mdb_midl_alloc(
                        *((*env).me_pgstate.mf_pghead).offset(0 as libc::c_int as isize)
                            as libc::c_int,
                    );
                    if !((*env).me_pgstate.mf_pghead).is_null() {
                        memcpy(
                            (*env).me_pgstate.mf_pghead as *mut libc::c_void,
                            (*ntxn).mnt_pgstate.mf_pghead as *const libc::c_void,
                            size as libc::c_ulong,
                        );
                    } else {
                        rc = 12 as libc::c_int;
                    }
                }
                if rc == 0 {
                    rc = mdb_cursor_shadow(parent, txn);
                }
                if rc != 0 {
                    mdb_txn_end(
                        txn,
                        MDB_END_FAIL_BEGINCHILD as libc::c_int as libc::c_uint,
                    );
                }
                current_block = 10150597327160359210;
            } else {
                (*txn).mt_dbiseqs = (*env).me_dbiseqs;
                current_block = 13766231232807252387;
            }
        }
        _ => {}
    }
    match current_block {
        13766231232807252387 => {
            rc = mdb_txn_renew0(txn);
        }
        _ => {}
    }
    if rc != 0 {
        if txn != (*env).me_txn0 {
            free(txn as *mut libc::c_void);
        }
    } else {
        (*txn).mt_flags |= flags;
        *ret = txn;
    }
    return rc;
}
pub unsafe extern "C" fn mdb_txn_env(mut txn: *mut MDB_txn) -> *mut MDB_env {
    if txn.is_null() {
        return 0 as *mut MDB_env;
    }
    return (*txn).mt_env;
}
pub unsafe extern "C" fn mdb_txn_id(mut txn: *mut MDB_txn) -> mdb_size_t {
    if txn.is_null() {
        return 0 as libc::c_int as mdb_size_t;
    }
    return (*txn).mt_txnid;
}
unsafe extern "C" fn mdb_dbis_update(mut txn: *mut MDB_txn, mut keep: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut n: MDB_dbi = (*txn).mt_numdbs;
    let mut env: *mut MDB_env = (*txn).mt_env;
    let mut tdbflags: *mut libc::c_uchar = (*txn).mt_dbflags;
    i = n as libc::c_int;
    loop {
        i -= 1;
        if !(i >= 2 as libc::c_int) {
            break;
        }
        if *tdbflags.offset(i as isize) as libc::c_int & 0x4 as libc::c_int != 0 {
            if keep != 0 {
                *((*env).me_dbflags)
                    .offset(
                        i as isize,
                    ) = ((*((*txn).mt_dbs).offset(i as isize)).md_flags as libc::c_int
                    | 0x8000 as libc::c_int) as uint16_t;
            } else {
                let mut ptr: *mut libc::c_char = (*((*env).me_dbxs).offset(i as isize))
                    .md_name
                    .mv_data as *mut libc::c_char;
                if !ptr.is_null() {
                    let ref mut fresh6 = (*((*env).me_dbxs).offset(i as isize))
                        .md_name
                        .mv_data;
                    *fresh6 = 0 as *mut libc::c_void;
                    (*((*env).me_dbxs).offset(i as isize))
                        .md_name
                        .mv_size = 0 as libc::c_int as size_t;
                    *((*env).me_dbflags)
                        .offset(i as isize) = 0 as libc::c_int as uint16_t;
                    let ref mut fresh7 = *((*env).me_dbiseqs).offset(i as isize);
                    *fresh7 = (*fresh7).wrapping_add(1);
                    *fresh7;
                    free(ptr as *mut libc::c_void);
                }
            }
        }
    }
    if keep != 0 && (*env).me_numdbs < n {
        (*env).me_numdbs = n;
    }
}
unsafe extern "C" fn mdb_txn_end(mut txn: *mut MDB_txn, mut mode: libc::c_uint) {
    let mut env: *mut MDB_env = (*txn).mt_env;
    mdb_dbis_update(txn, (mode & 0x10 as libc::c_int as libc::c_uint) as libc::c_int);
    if (*txn).mt_flags & 0x20000 as libc::c_int as libc::c_uint
        == 0x20000 as libc::c_int as libc::c_uint
    {
        if !((*txn).mt_u.reader).is_null() {
            ::std::ptr::write_volatile(
                &mut (*(*txn).mt_u.reader).mru.mrx.mrb_txnid as *mut txnid_t,
                -(1 as libc::c_int) as txnid_t,
            );
            if (*env).me_flags & 0x200000 as libc::c_int as libc::c_uint == 0 {
                (*txn).mt_u.reader = 0 as *mut MDB_reader;
            } else if mode & 0x200000 as libc::c_int as libc::c_uint != 0 {
                ::std::ptr::write_volatile(
                    &mut (*(*txn).mt_u.reader).mru.mrx.mrb_pid as *mut pid_t,
                    0 as libc::c_int,
                );
                (*txn).mt_u.reader = 0 as *mut MDB_reader;
            }
        }
        (*txn).mt_numdbs = 0 as libc::c_int as MDB_dbi;
        (*txn).mt_flags |= 0x1 as libc::c_int as libc::c_uint;
    } else if !((*txn).mt_flags & 0x1 as libc::c_int as libc::c_uint
        == 0x1 as libc::c_int as libc::c_uint)
    {
        let mut pghead: *mut pgno_t = (*env).me_pgstate.mf_pghead;
        if mode & 0x10 as libc::c_int as libc::c_uint == 0 {
            mdb_cursors_close(txn, 0 as libc::c_int as libc::c_uint);
        }
        if (*env).me_flags & 0x80000 as libc::c_int as libc::c_uint == 0 {
            mdb_dlist_free(txn);
        }
        (*txn).mt_numdbs = 0 as libc::c_int as MDB_dbi;
        (*txn).mt_flags = 0x1 as libc::c_int as libc::c_uint;
        if ((*txn).mt_parent).is_null() {
            mdb_midl_shrink(&mut (*txn).mt_free_pgs);
            (*env).me_free_pgs = (*txn).mt_free_pgs;
            (*env).me_pgstate.mf_pghead = 0 as *mut pgno_t;
            (*env).me_pgstate.mf_pglast = 0 as libc::c_int as txnid_t;
            (*env).me_txn = 0 as *mut MDB_txn;
            mode = 0 as libc::c_int as libc::c_uint;
            if !((*env).me_txns).is_null() {
                pthread_mutex_unlock(((*(*env).me_txns).mt2.mt2_wmutex).as_mut_ptr());
            }
        } else {
            (*(*txn).mt_parent).mt_child = 0 as *mut MDB_txn;
            (*(*txn).mt_parent).mt_flags &= !(0x10 as libc::c_int) as libc::c_uint;
            (*env).me_pgstate = (*(txn as *mut MDB_ntxn)).mnt_pgstate;
            mdb_midl_free((*txn).mt_free_pgs);
            free((*txn).mt_u.dirty_list as *mut libc::c_void);
        }
        mdb_midl_free((*txn).mt_spill_pgs);
        mdb_midl_free(pghead);
    }
    if mode & 0x20 as libc::c_int as libc::c_uint != 0 {
        free(txn as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn mdb_txn_reset(mut txn: *mut MDB_txn) {
    if txn.is_null() {
        return;
    }
    if (*txn).mt_flags & 0x20000 as libc::c_int as libc::c_uint == 0 {
        return;
    }
    mdb_txn_end(txn, MDB_END_RESET as libc::c_int as libc::c_uint);
}
pub unsafe extern "C" fn mdb_txn_abort(mut txn: *mut MDB_txn) {
    if txn.is_null() {
        return;
    }
    if !((*txn).mt_child).is_null() {
        mdb_txn_abort((*txn).mt_child);
    }
    mdb_txn_end(
        txn,
        (MDB_END_ABORT as libc::c_int | 0x200000 as libc::c_int | 0x20 as libc::c_int)
            as libc::c_uint,
    );
}
unsafe extern "C" fn mdb_freelist_save(mut txn: *mut MDB_txn) -> libc::c_int {
    let mut mc: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut env: *mut MDB_env = (*txn).mt_env;
    let mut rc: libc::c_int = 0;
    let mut maxfree_1pg: libc::c_int = (*env).me_maxfree_1pg;
    let mut more: libc::c_int = 1 as libc::c_int;
    let mut pglast: txnid_t = 0 as libc::c_int as txnid_t;
    let mut head_id: txnid_t = 0 as libc::c_int as txnid_t;
    let mut freecnt: pgno_t = 0 as libc::c_int as pgno_t;
    let mut free_pgs: *mut pgno_t = 0 as *mut pgno_t;
    let mut mop: *mut pgno_t = 0 as *mut pgno_t;
    let mut head_room: ssize_t = 0 as libc::c_int as ssize_t;
    let mut total_room: ssize_t = 0 as libc::c_int as ssize_t;
    let mut mop_len: ssize_t = 0;
    let mut clean_limit: ssize_t = 0;
    mdb_cursor_init(&mut mc, txn, 0 as libc::c_int as MDB_dbi, 0 as *mut MDB_xcursor);
    if !((*env).me_pgstate.mf_pghead).is_null() {
        rc = mdb_page_search(
            &mut mc,
            0 as *mut MDB_val,
            4 as libc::c_int | 1 as libc::c_int,
        );
        if rc != 0 && rc != -(30798 as libc::c_int) {
            return rc;
        }
    }
    if ((*env).me_pgstate.mf_pghead).is_null() && !((*txn).mt_loose_pgs).is_null() {
        let mut mp: *mut MDB_page = (*txn).mt_loose_pgs;
        let mut dl: *mut MDB_ID2 = (*txn).mt_u.dirty_list;
        let mut x: libc::c_uint = 0;
        rc = mdb_midl_need(
            &mut (*txn).mt_free_pgs,
            (*txn).mt_loose_count as libc::c_uint,
        );
        if rc != 0 as libc::c_int {
            return rc;
        }
        while !mp.is_null() {
            let mut xidl: *mut MDB_ID = (*txn).mt_free_pgs;
            let ref mut fresh8 = *xidl.offset(0 as libc::c_int as isize);
            *fresh8 = (*fresh8).wrapping_add(1);
            let mut xlen: MDB_ID = *fresh8;
            *xidl.offset(xlen as isize) = (*mp).mp_p.p_pgno;
            if (*txn).mt_flags & 0x80000 as libc::c_int as libc::c_uint != 0 {
                x = 1 as libc::c_int as libc::c_uint;
                while x as libc::c_ulong <= (*dl.offset(0 as libc::c_int as isize)).mid {
                    if (*dl.offset(x as isize)).mid == (*mp).mp_p.p_pgno {
                        break;
                    }
                    x = x.wrapping_add(1);
                    x;
                }
                if x as libc::c_ulong <= (*dl.offset(0 as libc::c_int as isize)).mid
                {} else {
                    mdb_assert_fail(
                        (*txn).mt_env,
                        b"x <= dl[0].mid\0" as *const u8 as *const libc::c_char,
                        (*::std::mem::transmute::<
                            &[u8; 18],
                            &[libc::c_char; 18],
                        >(b"mdb_freelist_save\0"))
                            .as_ptr(),
                        b"mdb.c\0" as *const u8 as *const libc::c_char,
                        3476 as libc::c_int,
                    );
                };
            } else {
                x = mdb_mid2l_search(dl, (*mp).mp_p.p_pgno);
                if (*dl.offset(x as isize)).mid == (*mp).mp_p.p_pgno {} else {
                    mdb_assert_fail(
                        (*txn).mt_env,
                        b"dl[x].mid == mp->mp_pgno\0" as *const u8
                            as *const libc::c_char,
                        (*::std::mem::transmute::<
                            &[u8; 18],
                            &[libc::c_char; 18],
                        >(b"mdb_freelist_save\0"))
                            .as_ptr(),
                        b"mdb.c\0" as *const u8 as *const libc::c_char,
                        3479 as libc::c_int,
                    );
                };
                mdb_dpage_free(env, mp);
            }
            let ref mut fresh9 = (*dl.offset(x as isize)).mptr;
            *fresh9 = 0 as *mut libc::c_void;
            mp = *(mp.offset(2 as libc::c_int as isize) as *mut *mut MDB_page);
        }
        let mut y: libc::c_uint = 0;
        y = 1 as libc::c_int as libc::c_uint;
        while !((*dl.offset(y as isize)).mptr).is_null()
            && y as libc::c_ulong <= (*dl.offset(0 as libc::c_int as isize)).mid
        {
            y = y.wrapping_add(1);
            y;
        }
        if y as libc::c_ulong <= (*dl.offset(0 as libc::c_int as isize)).mid {
            x = y;
            y = y.wrapping_add(1);
            y;
            loop {
                while ((*dl.offset(y as isize)).mptr).is_null()
                    && y as libc::c_ulong <= (*dl.offset(0 as libc::c_int as isize)).mid
                {
                    y = y.wrapping_add(1);
                    y;
                }
                if y as libc::c_ulong > (*dl.offset(0 as libc::c_int as isize)).mid {
                    break;
                }
                let fresh10 = y;
                y = y.wrapping_add(1);
                let fresh11 = x;
                x = x.wrapping_add(1);
                *dl.offset(fresh11 as isize) = *dl.offset(fresh10 as isize);
            }
            (*dl.offset(0 as libc::c_int as isize))
                .mid = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as MDB_ID;
        } else {
            (*dl.offset(0 as libc::c_int as isize)).mid = 0 as libc::c_int as MDB_ID;
        }
        (*txn).mt_loose_pgs = 0 as *mut MDB_page;
        (*txn).mt_loose_count = 0 as libc::c_int;
    }
    clean_limit = if (*env).me_flags
        & (0x1000000 as libc::c_int | 0x80000 as libc::c_int) as libc::c_uint != 0
    {
        9223372036854775807 as libc::c_long
    } else {
        maxfree_1pg as libc::c_long
    };
    loop {
        let mut key: MDB_val = MDB_val {
            mv_size: 0,
            mv_data: 0 as *mut libc::c_void,
        };
        let mut data: MDB_val = MDB_val {
            mv_size: 0,
            mv_data: 0 as *mut libc::c_void,
        };
        let mut pgs: *mut pgno_t = 0 as *mut pgno_t;
        let mut j: ssize_t = 0;
        while pglast < (*env).me_pgstate.mf_pglast {
            rc = mdb_cursor_first(&mut mc, &mut key, 0 as *mut MDB_val);
            if rc != 0 {
                return rc;
            }
            head_id = *(key.mv_data as *mut txnid_t);
            pglast = head_id;
            head_room = 0 as libc::c_int as ssize_t;
            total_room = head_room;
            if pglast <= (*env).me_pgstate.mf_pglast {} else {
                mdb_assert_fail(
                    (*txn).mt_env,
                    b"pglast <= env->me_pglast\0" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<
                        &[u8; 18],
                        &[libc::c_char; 18],
                    >(b"mdb_freelist_save\0"))
                        .as_ptr(),
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    3523 as libc::c_int,
                );
            };
            rc = mdb_cursor_del(&mut mc, 0 as libc::c_int as libc::c_uint);
            if rc != 0 {
                return rc;
            }
        }
        if freecnt < *((*txn).mt_free_pgs).offset(0 as libc::c_int as isize) {
            if freecnt == 0 {
                rc = mdb_page_search(
                    &mut mc,
                    0 as *mut MDB_val,
                    8 as libc::c_int | 1 as libc::c_int,
                );
                if rc != 0 && rc != -(30798 as libc::c_int) {
                    return rc;
                }
            }
            free_pgs = (*txn).mt_free_pgs;
            key.mv_size = ::std::mem::size_of::<txnid_t>() as libc::c_ulong;
            key.mv_data = &mut (*txn).mt_txnid as *mut txnid_t as *mut libc::c_void;
            loop {
                freecnt = *free_pgs.offset(0 as libc::c_int as isize);
                data
                    .mv_size = (*free_pgs.offset(0 as libc::c_int as isize))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<MDB_ID>() as libc::c_ulong);
                rc = mdb_cursor_put(
                    &mut mc,
                    &mut key,
                    &mut data,
                    0x10000 as libc::c_int as libc::c_uint,
                );
                if rc != 0 {
                    return rc;
                }
                free_pgs = (*txn).mt_free_pgs;
                if !(freecnt < *free_pgs.offset(0 as libc::c_int as isize)) {
                    break;
                }
            }
            mdb_midl_sort(free_pgs);
            memcpy(data.mv_data, free_pgs as *const libc::c_void, data.mv_size);
        } else {
            mop = (*env).me_pgstate.mf_pghead;
            mop_len = (if !mop.is_null() {
                *mop.offset(0 as libc::c_int as isize)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_add((*txn).mt_loose_count as libc::c_ulong) as ssize_t;
            if total_room >= mop_len {
                if total_room == mop_len
                    || {
                        more -= 1;
                        more < 0 as libc::c_int
                    }
                {
                    break;
                }
            } else if head_room >= maxfree_1pg as libc::c_long
                && head_id > 1 as libc::c_int as libc::c_ulong
            {
                head_id = head_id.wrapping_sub(1);
                head_id;
                head_room = 0 as libc::c_int as ssize_t;
            }
            total_room -= head_room;
            head_room = mop_len - total_room;
            if head_room > maxfree_1pg as libc::c_long
                && head_id > 1 as libc::c_int as libc::c_ulong
            {
                head_room = (head_room as libc::c_ulong).wrapping_div(head_id) as ssize_t
                    as ssize_t;
                head_room
                    += maxfree_1pg as libc::c_long
                        - head_room % (maxfree_1pg + 1 as libc::c_int) as libc::c_long;
            } else if head_room < 0 as libc::c_int as libc::c_long {
                head_room = 0 as libc::c_int as ssize_t;
            }
            key.mv_size = ::std::mem::size_of::<txnid_t>() as libc::c_ulong;
            key.mv_data = &mut head_id as *mut txnid_t as *mut libc::c_void;
            data
                .mv_size = ((head_room + 1 as libc::c_int as libc::c_long)
                as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<pgno_t>() as libc::c_ulong);
            rc = mdb_cursor_put(
                &mut mc,
                &mut key,
                &mut data,
                0x10000 as libc::c_int as libc::c_uint,
            );
            if rc != 0 {
                return rc;
            }
            pgs = data.mv_data as *mut pgno_t;
            j = if head_room > clean_limit {
                head_room
            } else {
                0 as libc::c_int as libc::c_long
            };
            loop {
                *pgs.offset(j as isize) = 0 as libc::c_int as pgno_t;
                j -= 1;
                if !(j >= 0 as libc::c_int as libc::c_long) {
                    break;
                }
            }
            total_room += head_room;
        }
    }
    if !((*txn).mt_loose_pgs).is_null() {
        let mut mp_0: *mut MDB_page = (*txn).mt_loose_pgs;
        let mut count: libc::c_uint = (*txn).mt_loose_count as libc::c_uint;
        let mut loose: MDB_IDL = 0 as *mut MDB_ID;
        rc = mdb_midl_need(
            &mut (*env).me_pgstate.mf_pghead,
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul(count)
                .wrapping_add(1 as libc::c_int as libc::c_uint),
        );
        if rc != 0 as libc::c_int {
            return rc;
        }
        mop = (*env).me_pgstate.mf_pghead;
        loose = mop
            .offset(*mop.offset(-(1 as libc::c_int) as isize) as isize)
            .offset(-(count as isize));
        count = 0 as libc::c_int as libc::c_uint;
        while !mp_0.is_null() {
            count = count.wrapping_add(1);
            *loose.offset(count as isize) = (*mp_0).mp_p.p_pgno;
            mp_0 = *(mp_0.offset(2 as libc::c_int as isize) as *mut *mut MDB_page);
        }
        *loose.offset(0 as libc::c_int as isize) = count as MDB_ID;
        mdb_midl_sort(loose);
        mdb_midl_xmerge(mop, loose);
        (*txn).mt_loose_pgs = 0 as *mut MDB_page;
        (*txn).mt_loose_count = 0 as libc::c_int;
        mop_len = *mop.offset(0 as libc::c_int as isize) as ssize_t;
    }
    rc = 0 as libc::c_int;
    if mop_len != 0 {
        let mut key_0: MDB_val = MDB_val {
            mv_size: 0,
            mv_data: 0 as *mut libc::c_void,
        };
        let mut data_0: MDB_val = MDB_val {
            mv_size: 0,
            mv_data: 0 as *mut libc::c_void,
        };
        mop = mop.offset(mop_len as isize);
        rc = mdb_cursor_first(&mut mc, &mut key_0, &mut data_0);
        while rc == 0 {
            let mut id: txnid_t = *(key_0.mv_data as *mut txnid_t);
            let mut len: ssize_t = (data_0.mv_size)
                .wrapping_div(::std::mem::size_of::<MDB_ID>() as libc::c_ulong)
                as ssize_t - 1 as libc::c_int as libc::c_long;
            let mut save: MDB_ID = 0;
            if len >= 0 as libc::c_int as libc::c_long
                && id <= (*env).me_pgstate.mf_pglast
            {} else {
                mdb_assert_fail(
                    (*txn).mt_env,
                    b"len >= 0 && id <= env->me_pglast\0" as *const u8
                        as *const libc::c_char,
                    (*::std::mem::transmute::<
                        &[u8; 18],
                        &[libc::c_char; 18],
                    >(b"mdb_freelist_save\0"))
                        .as_ptr(),
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    3639 as libc::c_int,
                );
            };
            key_0.mv_data = &mut id as *mut txnid_t as *mut libc::c_void;
            if len > mop_len {
                len = mop_len;
                data_0
                    .mv_size = ((len + 1 as libc::c_int as libc::c_long)
                    as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<MDB_ID>() as libc::c_ulong);
            }
            mop = mop.offset(-(len as isize));
            data_0.mv_data = mop as *mut libc::c_void;
            save = *mop.offset(0 as libc::c_int as isize);
            *mop.offset(0 as libc::c_int as isize) = len as pgno_t;
            rc = mdb_cursor_put(
                &mut mc,
                &mut key_0,
                &mut data_0,
                0x40 as libc::c_int as libc::c_uint,
            );
            *mop.offset(0 as libc::c_int as isize) = save;
            if rc != 0
                || {
                    mop_len -= len;
                    mop_len == 0
                }
            {
                break;
            }
            rc = mdb_cursor_next(&mut mc, &mut key_0, &mut data_0, MDB_NEXT);
        }
    }
    return rc;
}
unsafe extern "C" fn mdb_page_flush(
    mut txn: *mut MDB_txn,
    mut keep: libc::c_int,
) -> libc::c_int {
    let mut env: *mut MDB_env = (*txn).mt_env;
    let mut dl: MDB_ID2L = (*txn).mt_u.dirty_list;
    let mut psize: libc::c_uint = (*env).me_psize;
    let mut j: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut pagecount: libc::c_int = (*dl.offset(0 as libc::c_int as isize)).mid
        as libc::c_int;
    let mut rc: libc::c_int = 0;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut pos: off_t = 0 as libc::c_int as off_t;
    let mut pgno: pgno_t = 0 as libc::c_int as pgno_t;
    let mut dp: *mut MDB_page = 0 as *mut MDB_page;
    let mut iov: [iovec; 64] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 64];
    let mut fd: libc::c_int = (*env).me_fd;
    let mut wsize: ssize_t = 0 as libc::c_int as ssize_t;
    let mut wres: ssize_t = 0;
    let mut wpos: off_t = 0 as libc::c_int as off_t;
    let mut next_pos: off_t = 1 as libc::c_int as off_t;
    let mut n: libc::c_int = 0 as libc::c_int;
    i = keep;
    j = i as libc::c_uint;
    if (*env).me_flags & 0x80000 as libc::c_int as libc::c_uint != 0 {
        loop {
            i += 1;
            if !(i <= pagecount) {
                break;
            }
            dp = (*dl.offset(i as isize)).mptr as *mut MDB_page;
            if (*dp).mp_flags as libc::c_int
                & (0x4000 as libc::c_int | 0x8000 as libc::c_int) != 0
            {
                (*dp)
                    .mp_flags = ((*dp).mp_flags as libc::c_int
                    & !(0x8000 as libc::c_int)) as uint16_t;
                j = j.wrapping_add(1);
                *dl.offset(j as isize) = *dl.offset(i as isize);
            } else {
                (*dp)
                    .mp_flags = ((*dp).mp_flags as libc::c_int & !(0x10 as libc::c_int))
                    as uint16_t;
            }
        }
    } else {
        loop {
            i += 1;
            if i <= pagecount {
                dp = (*dl.offset(i as isize)).mptr as *mut MDB_page;
                if (*dp).mp_flags as libc::c_int
                    & (0x4000 as libc::c_int | 0x8000 as libc::c_int) != 0
                {
                    (*dp)
                        .mp_flags = ((*dp).mp_flags as libc::c_int
                        & !(0x8000 as libc::c_int)) as uint16_t;
                    (*dl.offset(i as isize)).mid = 0 as libc::c_int as MDB_ID;
                    continue;
                } else {
                    pgno = (*dl.offset(i as isize)).mid;
                    (*dp)
                        .mp_flags = ((*dp).mp_flags as libc::c_int
                        & !(0x10 as libc::c_int)) as uint16_t;
                    pos = pgno.wrapping_mul(psize as libc::c_ulong) as off_t;
                    size = psize as size_t;
                    if (*dp).mp_flags as libc::c_int & 0x4 as libc::c_int
                        == 0x4 as libc::c_int
                    {
                        size = (size as libc::c_ulong)
                            .wrapping_mul((*dp).mp_pb.pb_pages as libc::c_ulong)
                            as size_t as size_t;
                    }
                }
            }
            if pos != next_pos || n == 64 as libc::c_int
                || (wsize as libc::c_ulong).wrapping_add(size)
                    > (0x40000000 as libc::c_uint
                        >> (::std::mem::size_of::<ssize_t>() as libc::c_ulong
                            == 4 as libc::c_int as libc::c_ulong) as libc::c_int)
                        as libc::c_ulong
            {
                if n != 0 {
                    's_208: {
                        loop {
                            if n == 1 as libc::c_int {
                                wres = pwrite(
                                    fd,
                                    iov[0 as libc::c_int as usize].iov_base,
                                    wsize as size_t,
                                    wpos,
                                );
                            } else {
                                's_156: {
                                    loop {
                                        if lseek(fd, wpos, 0 as libc::c_int)
                                            == -(1 as libc::c_int) as libc::c_long
                                        {
                                            rc = *__errno_location();
                                            if rc == 4 as libc::c_int {
                                                continue;
                                            }
                                            return rc;
                                        } else {
                                            break 's_156;
                                        }
                                    }
                                }
                                wres = writev(fd, iov.as_mut_ptr(), n);
                            }
                            if wres != wsize {
                                if wres < 0 as libc::c_int as libc::c_long {
                                    rc = *__errno_location();
                                    if !(rc == 4 as libc::c_int) {
                                        break;
                                    }
                                } else {
                                    rc = 5 as libc::c_int;
                                    break;
                                }
                            } else {
                                n = 0 as libc::c_int;
                                break 's_208;
                            }
                        }
                        return rc;
                    }
                }
                if i > pagecount {
                    break;
                }
                wpos = pos;
                wsize = 0 as libc::c_int as ssize_t;
            }
            iov[n as usize].iov_len = size;
            iov[n as usize].iov_base = dp as *mut libc::c_char as *mut libc::c_void;
            next_pos = (pos as libc::c_ulong).wrapping_add(size) as off_t;
            wsize = (wsize as libc::c_ulong).wrapping_add(size) as ssize_t as ssize_t;
            n += 1;
            n;
        }
        if (*env).me_flags & 0x80000 as libc::c_int as libc::c_uint == 0 {
            i = keep;
            loop {
                i += 1;
                if !(i <= pagecount) {
                    break;
                }
                dp = (*dl.offset(i as isize)).mptr as *mut MDB_page;
                if (*dl.offset(i as isize)).mid == 0 {
                    j = j.wrapping_add(1);
                    *dl.offset(j as isize) = *dl.offset(i as isize);
                    (*dl.offset(j as isize)).mid = (*dp).mp_p.p_pgno;
                } else {
                    mdb_dpage_free(env, dp);
                }
            }
        }
    }
    i -= 1;
    i;
    (*txn)
        .mt_dirty_room = ((*txn).mt_dirty_room)
        .wrapping_add((i as libc::c_uint).wrapping_sub(j));
    (*dl.offset(0 as libc::c_int as isize)).mid = j as MDB_ID;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_txn_commit(mut txn: *mut MDB_txn) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut end_mode: libc::c_uint = 0;
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    if txn.is_null() {
        return 22 as libc::c_int;
    }
    end_mode = (MDB_END_EMPTY_COMMIT as libc::c_int | 0x10 as libc::c_int
        | 0x200000 as libc::c_int | 0x20 as libc::c_int) as libc::c_uint;
    if !((*txn).mt_child).is_null() {
        rc = mdb_txn_commit((*txn).mt_child);
        if rc != 0 {
            current_block = 11046667175926710646;
        } else {
            current_block = 11875828834189669668;
        }
    } else {
        current_block = 11875828834189669668;
    }
    match current_block {
        11875828834189669668 => {
            env = (*txn).mt_env;
            if (*txn).mt_flags & 0x20000 as libc::c_int as libc::c_uint
                == 0x20000 as libc::c_int as libc::c_uint
            {
                current_block = 15786631208078821863;
            } else if (*txn).mt_flags
                & (0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint != 0
            {
                if !((*txn).mt_parent).is_null() {
                    (*(*txn).mt_parent).mt_flags |= 0x2 as libc::c_int as libc::c_uint;
                }
                rc = -(30782 as libc::c_int);
                current_block = 11046667175926710646;
            } else if !((*txn).mt_parent).is_null() {
                let mut parent: *mut MDB_txn = (*txn).mt_parent;
                let mut lp: *mut *mut MDB_page = 0 as *mut *mut MDB_page;
                let mut dst: MDB_ID2L = 0 as *mut MDB_ID2;
                let mut src: MDB_ID2L = 0 as *mut MDB_ID2;
                let mut pspill: MDB_IDL = 0 as *mut MDB_ID;
                let mut x: libc::c_uint = 0;
                let mut y: libc::c_uint = 0;
                let mut len: libc::c_uint = 0;
                let mut ps_len: libc::c_uint = 0;
                rc = mdb_midl_append_list(
                    &mut (*parent).mt_free_pgs,
                    (*txn).mt_free_pgs,
                );
                if rc != 0 {
                    current_block = 11046667175926710646;
                } else {
                    mdb_midl_free((*txn).mt_free_pgs);
                    (*parent).mt_next_pgno = (*txn).mt_next_pgno;
                    (*parent).mt_flags = (*txn).mt_flags;
                    mdb_cursors_close(txn, 1 as libc::c_int as libc::c_uint);
                    memcpy(
                        (*parent).mt_dbs as *mut libc::c_void,
                        (*txn).mt_dbs as *const libc::c_void,
                        ((*txn).mt_numdbs as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<MDB_db>() as libc::c_ulong,
                            ),
                    );
                    (*parent).mt_numdbs = (*txn).mt_numdbs;
                    *((*parent).mt_dbflags)
                        .offset(
                            0 as libc::c_int as isize,
                        ) = *((*txn).mt_dbflags).offset(0 as libc::c_int as isize);
                    *((*parent).mt_dbflags)
                        .offset(
                            1 as libc::c_int as isize,
                        ) = *((*txn).mt_dbflags).offset(1 as libc::c_int as isize);
                    i = 2 as libc::c_int as libc::c_uint;
                    while i < (*txn).mt_numdbs {
                        x = (*((*parent).mt_dbflags).offset(i as isize) as libc::c_int
                            & 0x4 as libc::c_int) as libc::c_uint;
                        *((*parent).mt_dbflags)
                            .offset(
                                i as isize,
                            ) = (*((*txn).mt_dbflags).offset(i as isize) as libc::c_uint
                            | x) as libc::c_uchar;
                        i = i.wrapping_add(1);
                        i;
                    }
                    dst = (*parent).mt_u.dirty_list;
                    src = (*txn).mt_u.dirty_list;
                    pspill = (*parent).mt_spill_pgs;
                    if !pspill.is_null()
                        && {
                            ps_len = *pspill.offset(0 as libc::c_int as isize)
                                as libc::c_uint;
                            ps_len != 0
                        }
                    {
                        y = ps_len;
                        x = y;
                        *pspill
                            .offset(
                                0 as libc::c_int as isize,
                            ) = -(1 as libc::c_int) as pgno_t;
                        i = 0 as libc::c_int as libc::c_uint;
                        len = (*src.offset(0 as libc::c_int as isize)).mid
                            as libc::c_uint;
                        loop {
                            i = i.wrapping_add(1);
                            if !(i <= len) {
                                break;
                            }
                            let mut pn: MDB_ID = (*src.offset(i as isize)).mid
                                << 1 as libc::c_int;
                            while pn > *pspill.offset(x as isize) {
                                x = x.wrapping_sub(1);
                                x;
                            }
                            if pn == *pspill.offset(x as isize) {
                                *pspill.offset(x as isize) = 1 as libc::c_int as MDB_ID;
                                x = x.wrapping_sub(1);
                                y = x;
                            }
                        }
                        x = y;
                        loop {
                            x = x.wrapping_add(1);
                            if !(x <= ps_len) {
                                break;
                            }
                            if *pspill.offset(x as isize)
                                & 1 as libc::c_int as libc::c_ulong == 0
                            {
                                y = y.wrapping_add(1);
                                *pspill.offset(y as isize) = *pspill.offset(x as isize);
                            }
                        }
                        *pspill.offset(0 as libc::c_int as isize) = y as MDB_ID;
                    }
                    if !((*txn).mt_spill_pgs).is_null()
                        && *((*txn).mt_spill_pgs).offset(0 as libc::c_int as isize) != 0
                    {
                        i = 1 as libc::c_int as libc::c_uint;
                        while i as libc::c_ulong
                            <= *((*txn).mt_spill_pgs).offset(0 as libc::c_int as isize)
                        {
                            let mut pn_0: MDB_ID = *((*txn).mt_spill_pgs)
                                .offset(i as isize);
                            if !(pn_0 & 1 as libc::c_int as libc::c_ulong != 0) {
                                pn_0 >>= 1 as libc::c_int;
                                y = mdb_mid2l_search(dst, pn_0);
                                if y as libc::c_ulong
                                    <= (*dst.offset(0 as libc::c_int as isize)).mid
                                    && (*dst.offset(y as isize)).mid == pn_0
                                {
                                    free((*dst.offset(y as isize)).mptr);
                                    while (y as libc::c_ulong)
                                        < (*dst.offset(0 as libc::c_int as isize)).mid
                                    {
                                        *dst
                                            .offset(
                                                y as isize,
                                            ) = *dst
                                            .offset(
                                                y.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                                            );
                                        y = y.wrapping_add(1);
                                        y;
                                    }
                                    let ref mut fresh12 = (*dst
                                        .offset(0 as libc::c_int as isize))
                                        .mid;
                                    *fresh12 = (*fresh12).wrapping_sub(1);
                                    *fresh12;
                                }
                            }
                            i = i.wrapping_add(1);
                            i;
                        }
                    }
                    x = (*dst.offset(0 as libc::c_int as isize)).mid as libc::c_uint;
                    (*dst.offset(0 as libc::c_int as isize))
                        .mid = 0 as libc::c_int as MDB_ID;
                    if !((*parent).mt_parent).is_null() {
                        len = (x as libc::c_ulong)
                            .wrapping_add((*src.offset(0 as libc::c_int as isize)).mid)
                            as libc::c_uint;
                        y = (mdb_mid2l_search(
                            src,
                            ((*dst.offset(x as isize)).mid)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ))
                            .wrapping_sub(1 as libc::c_int as libc::c_uint);
                        i = x;
                        while y != 0 && i != 0 {
                            let mut yp: pgno_t = (*src.offset(y as isize)).mid;
                            while yp < (*dst.offset(i as isize)).mid {
                                i = i.wrapping_sub(1);
                                i;
                            }
                            if yp == (*dst.offset(i as isize)).mid {
                                i = i.wrapping_sub(1);
                                i;
                                len = len.wrapping_sub(1);
                                len;
                            }
                            y = y.wrapping_sub(1);
                            y;
                        }
                    } else {
                        len = ((((1 as libc::c_int)
                            << 16 as libc::c_int + 1 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint)
                            .wrapping_sub((*txn).mt_dirty_room);
                    }
                    y = (*src.offset(0 as libc::c_int as isize)).mid as libc::c_uint;
                    i = len;
                    while y != 0 {
                        let mut yp_0: pgno_t = (*src.offset(y as isize)).mid;
                        while yp_0 < (*dst.offset(x as isize)).mid {
                            let fresh13 = x;
                            x = x.wrapping_sub(1);
                            let fresh14 = i;
                            i = i.wrapping_sub(1);
                            *dst
                                .offset(fresh14 as isize) = *dst.offset(fresh13 as isize);
                        }
                        if yp_0 == (*dst.offset(x as isize)).mid {
                            let fresh15 = x;
                            x = x.wrapping_sub(1);
                            free((*dst.offset(fresh15 as isize)).mptr);
                        }
                        let fresh16 = y;
                        y = y.wrapping_sub(1);
                        let fresh17 = i;
                        i = i.wrapping_sub(1);
                        *dst.offset(fresh17 as isize) = *src.offset(fresh16 as isize);
                    }
                    if i == x {} else {
                        mdb_assert_fail(
                            (*txn).mt_env,
                            b"i == x\0" as *const u8 as *const libc::c_char,
                            (*::std::mem::transmute::<
                                &[u8; 15],
                                &[libc::c_char; 15],
                            >(b"mdb_txn_commit\0"))
                                .as_ptr(),
                            b"mdb.c\0" as *const u8 as *const libc::c_char,
                            4028 as libc::c_int,
                        );
                    };
                    (*dst.offset(0 as libc::c_int as isize)).mid = len as MDB_ID;
                    free((*txn).mt_u.dirty_list as *mut libc::c_void);
                    (*parent).mt_dirty_room = (*txn).mt_dirty_room;
                    if !((*txn).mt_spill_pgs).is_null() {
                        if !((*parent).mt_spill_pgs).is_null() {
                            rc = mdb_midl_append_list(
                                &mut (*parent).mt_spill_pgs,
                                (*txn).mt_spill_pgs,
                            );
                            if rc != 0 {
                                (*parent).mt_flags |= 0x2 as libc::c_int as libc::c_uint;
                            }
                            mdb_midl_free((*txn).mt_spill_pgs);
                            mdb_midl_sort((*parent).mt_spill_pgs);
                        } else {
                            (*parent).mt_spill_pgs = (*txn).mt_spill_pgs;
                        }
                    }
                    lp = &mut (*parent).mt_loose_pgs;
                    while !(*lp).is_null() {
                        lp = (*lp).offset(2 as libc::c_int as isize)
                            as *mut *mut MDB_page;
                    }
                    *lp = (*txn).mt_loose_pgs;
                    (*parent).mt_loose_count += (*txn).mt_loose_count;
                    (*parent).mt_child = 0 as *mut MDB_txn;
                    mdb_midl_free((*(txn as *mut MDB_ntxn)).mnt_pgstate.mf_pghead);
                    free(txn as *mut libc::c_void);
                    return rc;
                }
            } else if txn != (*env).me_txn {
                rc = 22 as libc::c_int;
                current_block = 11046667175926710646;
            } else {
                mdb_cursors_close(txn, 0 as libc::c_int as libc::c_uint);
                if (*((*txn).mt_u.dirty_list).offset(0 as libc::c_int as isize)).mid == 0
                    && (*txn).mt_flags
                        & (0x4 as libc::c_int | 0x8 as libc::c_int) as libc::c_uint == 0
                {
                    current_block = 15786631208078821863;
                } else {
                    if (*txn).mt_numdbs > 2 as libc::c_int as libc::c_uint {
                        let mut mc: MDB_cursor = MDB_cursor {
                            mc_next: 0 as *mut MDB_cursor,
                            mc_backup: 0 as *mut MDB_cursor,
                            mc_xcursor: 0 as *mut MDB_xcursor,
                            mc_txn: 0 as *mut MDB_txn,
                            mc_dbi: 0,
                            mc_db: 0 as *mut MDB_db,
                            mc_dbx: 0 as *mut MDB_dbx,
                            mc_dbflag: 0 as *mut libc::c_uchar,
                            mc_snum: 0,
                            mc_top: 0,
                            mc_flags: 0,
                            mc_pg: [0 as *mut MDB_page; 32],
                            mc_ki: [0; 32],
                        };
                        let mut i_0: MDB_dbi = 0;
                        let mut data: MDB_val = MDB_val {
                            mv_size: 0,
                            mv_data: 0 as *mut libc::c_void,
                        };
                        data.mv_size = ::std::mem::size_of::<MDB_db>() as libc::c_ulong;
                        mdb_cursor_init(
                            &mut mc,
                            txn,
                            1 as libc::c_int as MDB_dbi,
                            0 as *mut MDB_xcursor,
                        );
                        i_0 = 2 as libc::c_int as MDB_dbi;
                        loop {
                            if !(i_0 < (*txn).mt_numdbs) {
                                current_block = 8656139126282042408;
                                break;
                            }
                            if *((*txn).mt_dbflags).offset(i_0 as isize) as libc::c_int
                                & 0x1 as libc::c_int != 0
                            {
                                if *((*txn).mt_dbiseqs).offset(i_0 as isize)
                                    != *((*(*txn).mt_env).me_dbiseqs).offset(i_0 as isize)
                                {
                                    rc = -(30780 as libc::c_int);
                                    current_block = 11046667175926710646;
                                    break;
                                } else {
                                    data
                                        .mv_data = &mut *((*txn).mt_dbs).offset(i_0 as isize)
                                        as *mut MDB_db as *mut libc::c_void;
                                    rc = mdb_cursor_put(
                                        &mut mc,
                                        &mut (*((*txn).mt_dbxs).offset(i_0 as isize)).md_name,
                                        &mut data,
                                        0x2 as libc::c_int as libc::c_uint,
                                    );
                                    if rc != 0 {
                                        current_block = 11046667175926710646;
                                        break;
                                    }
                                }
                            }
                            i_0 = i_0.wrapping_add(1);
                            i_0;
                        }
                    } else {
                        current_block = 8656139126282042408;
                    }
                    match current_block {
                        11046667175926710646 => {}
                        _ => {
                            rc = mdb_freelist_save(txn);
                            if rc != 0 {
                                current_block = 11046667175926710646;
                            } else {
                                mdb_midl_free((*env).me_pgstate.mf_pghead);
                                (*env).me_pgstate.mf_pghead = 0 as *mut pgno_t;
                                mdb_midl_shrink(&mut (*txn).mt_free_pgs);
                                rc = mdb_page_flush(txn, 0 as libc::c_int);
                                if rc != 0 {
                                    current_block = 11046667175926710646;
                                } else if !((*txn).mt_flags
                                    & 0x10000 as libc::c_int as libc::c_uint
                                    == 0x10000 as libc::c_int as libc::c_uint)
                                    && {
                                        rc = mdb_env_sync0(
                                            env,
                                            0 as libc::c_int,
                                            (*txn).mt_next_pgno,
                                        );
                                        rc != 0
                                    }
                                {
                                    current_block = 11046667175926710646;
                                } else {
                                    rc = mdb_env_write_meta(txn);
                                    if rc != 0 {
                                        current_block = 11046667175926710646;
                                    } else {
                                        end_mode = (MDB_END_COMMITTED as libc::c_int
                                            | 0x10 as libc::c_int) as libc::c_uint;
                                        if (*env).me_flags
                                            & 0x2000000 as libc::c_int as libc::c_uint != 0
                                        {
                                            if (*env).me_flags & 0x400000 as libc::c_int as libc::c_uint
                                                == 0
                                            {
                                                let mut excl: libc::c_int = 0;
                                                rc = mdb_env_share_locks(env, &mut excl);
                                                if rc != 0 {
                                                    current_block = 11046667175926710646;
                                                } else {
                                                    current_block = 1604201581803946138;
                                                }
                                            } else {
                                                current_block = 1604201581803946138;
                                            }
                                            match current_block {
                                                11046667175926710646 => {}
                                                _ => {
                                                    (*env).me_flags ^= 0x2000000 as libc::c_int as libc::c_uint;
                                                    current_block = 15786631208078821863;
                                                }
                                            }
                                        } else {
                                            current_block = 15786631208078821863;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            match current_block {
                11046667175926710646 => {}
                _ => {
                    mdb_txn_end(txn, end_mode);
                    return 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    mdb_txn_abort(txn);
    return rc;
}
unsafe extern "C" fn mdb_env_read_header(
    mut env: *mut MDB_env,
    mut prev: libc::c_int,
    mut meta: *mut MDB_meta,
) -> libc::c_int {
    let mut pbuf: MDB_metabuf = MDB_metabuf {
        mb_page: MDB_page {
            mp_p: C2RustUnnamed_8 { p_pgno: 0 },
            mp_pad: 0,
            mp_flags: 0,
            mp_pb: C2RustUnnamed_6 {
                pb: C2RustUnnamed_7 {
                    pb_lower: 0,
                    pb_upper: 0,
                },
            },
            mp_ptrs: [0; 1],
        },
    };
    let mut p: *mut MDB_page = 0 as *mut MDB_page;
    let mut m: *mut MDB_meta = 0 as *mut MDB_meta;
    let mut i: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut off: libc::c_int = 0;
    off = 0 as libc::c_int;
    i = off;
    while i < 2 as libc::c_int {
        rc = pread(
            (*env).me_fd,
            &mut pbuf as *mut MDB_metabuf as *mut libc::c_void,
            Size as libc::c_int as size_t,
            off as __off_t,
        ) as libc::c_int;
        if rc != Size as libc::c_int {
            if rc == 0 as libc::c_int && off == 0 as libc::c_int {
                return 2 as libc::c_int;
            }
            rc = if rc < 0 as libc::c_int {
                *__errno_location()
            } else {
                -(30793 as libc::c_int)
            };
            return rc;
        }
        p = &mut pbuf as *mut MDB_metabuf as *mut MDB_page;
        if !((*p).mp_flags as libc::c_int & 0x8 as libc::c_int == 0x8 as libc::c_int) {
            return -(30793 as libc::c_int);
        }
        m = (p as *mut libc::c_char).offset(16 as libc::c_ulong as libc::c_uint as isize)
            as *mut libc::c_void as *mut MDB_meta;
        if (*m).mm_magic != 0xbeefc0de as libc::c_uint {
            return -(30793 as libc::c_int);
        }
        if (*m).mm_version
            != (if 0 as libc::c_int != 0 {
                999 as libc::c_int
            } else {
                1 as libc::c_int
            }) as libc::c_uint
        {
            return -(30794 as libc::c_int);
        }
        if off == 0 as libc::c_int
            || (if prev != 0 {
                ((*m).mm_txnid < (*meta).mm_txnid) as libc::c_int
            } else {
                ((*m).mm_txnid > (*meta).mm_txnid) as libc::c_int
            }) != 0
        {
            *meta = *m;
        }
        i += 1;
        i;
        off = (off as libc::c_uint)
            .wrapping_add((*meta).mm_dbs[0 as libc::c_int as usize].md_pad)
            as libc::c_int as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_env_init_meta0(mut env: *mut MDB_env, mut meta: *mut MDB_meta) {
    (*meta).mm_magic = 0xbeefc0de as libc::c_uint;
    (*meta)
        .mm_version = (if 0 as libc::c_int != 0 {
        999 as libc::c_int
    } else {
        1 as libc::c_int
    }) as uint32_t;
    (*meta).mm_mapsize = (*env).me_mapsize;
    (*meta).mm_dbs[0 as libc::c_int as usize].md_pad = (*env).me_psize;
    (*meta).mm_last_pg = (2 as libc::c_int - 1 as libc::c_int) as pgno_t;
    (*meta)
        .mm_dbs[0 as libc::c_int as usize]
        .md_flags = ((*env).me_flags & 0xffff as libc::c_int as libc::c_uint)
        as uint16_t;
    (*meta)
        .mm_dbs[0 as libc::c_int as usize]
        .md_flags = ((*meta).mm_dbs[0 as libc::c_int as usize].md_flags as libc::c_int
        | 0x8 as libc::c_int) as uint16_t;
    (*meta).mm_dbs[0 as libc::c_int as usize].md_root = !(0 as libc::c_int as pgno_t);
    (*meta).mm_dbs[1 as libc::c_int as usize].md_root = !(0 as libc::c_int as pgno_t);
}
unsafe extern "C" fn mdb_env_init_meta(
    mut env: *mut MDB_env,
    mut meta: *mut MDB_meta,
) -> libc::c_int {
    let mut p: *mut MDB_page = 0 as *mut MDB_page;
    let mut q: *mut MDB_page = 0 as *mut MDB_page;
    let mut rc: libc::c_int = 0;
    let mut psize: libc::c_uint = 0;
    let mut len: libc::c_int = 0;
    psize = (*env).me_psize;
    p = calloc(2 as libc::c_int as libc::c_ulong, psize as libc::c_ulong)
        as *mut MDB_page;
    if p.is_null() {
        return 12 as libc::c_int;
    }
    (*p).mp_p.p_pgno = 0 as libc::c_int as pgno_t;
    (*p).mp_flags = 0x8 as libc::c_int as uint16_t;
    *((p as *mut libc::c_char).offset(16 as libc::c_ulong as libc::c_uint as isize)
        as *mut libc::c_void as *mut MDB_meta) = *meta;
    q = (p as *mut libc::c_char).offset(psize as isize) as *mut MDB_page;
    (*q).mp_p.p_pgno = 1 as libc::c_int as pgno_t;
    (*q).mp_flags = 0x8 as libc::c_int as uint16_t;
    *((q as *mut libc::c_char).offset(16 as libc::c_ulong as libc::c_uint as isize)
        as *mut libc::c_void as *mut MDB_meta) = *meta;
    loop {
        len = pwrite(
            (*env).me_fd,
            p as *const libc::c_void,
            psize.wrapping_mul(2 as libc::c_int as libc::c_uint) as size_t,
            0 as libc::c_int as __off_t,
        ) as libc::c_int;
        if len == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int {
            continue;
        }
        rc = (len >= 0 as libc::c_int) as libc::c_int;
        break;
    }
    if rc == 0 {
        rc = *__errno_location();
    } else if len as libc::c_uint == psize.wrapping_mul(2 as libc::c_int as libc::c_uint)
    {
        rc = 0 as libc::c_int;
    } else {
        rc = 28 as libc::c_int;
    }
    free(p as *mut libc::c_void);
    return rc;
}
unsafe extern "C" fn mdb_env_write_meta(mut txn: *mut MDB_txn) -> libc::c_int {
    let mut current_block: u64;
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut meta: MDB_meta = MDB_meta {
        mm_magic: 0,
        mm_version: 0,
        mm_address: 0 as *mut libc::c_void,
        mm_mapsize: 0,
        mm_dbs: [MDB_db {
            md_pad: 0,
            md_flags: 0,
            md_depth: 0,
            md_branch_pages: 0,
            md_leaf_pages: 0,
            md_overflow_pages: 0,
            md_entries: 0,
            md_root: 0,
        }; 2],
        mm_last_pg: 0,
        mm_txnid: 0,
    };
    let mut metab: MDB_meta = MDB_meta {
        mm_magic: 0,
        mm_version: 0,
        mm_address: 0 as *mut libc::c_void,
        mm_mapsize: 0,
        mm_dbs: [MDB_db {
            md_pad: 0,
            md_flags: 0,
            md_depth: 0,
            md_branch_pages: 0,
            md_leaf_pages: 0,
            md_overflow_pages: 0,
            md_entries: 0,
            md_root: 0,
        }; 2],
        mm_last_pg: 0,
        mm_txnid: 0,
    };
    let mut mp: *mut MDB_meta = 0 as *mut MDB_meta;
    let mut flags: libc::c_uint = 0;
    let mut mapsize: mdb_size_t = 0;
    let mut off: off_t = 0;
    let mut rc: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut toggle: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mfd: libc::c_int = 0;
    let mut r2: libc::c_int = 0;
    toggle = ((*txn).mt_txnid & 1 as libc::c_int as libc::c_ulong) as libc::c_int;
    env = (*txn).mt_env;
    flags = (*txn).mt_flags | (*env).me_flags;
    mp = (*env).me_metas[toggle as usize];
    mapsize = (*(*env).me_metas[(toggle ^ 1 as libc::c_int) as usize]).mm_mapsize;
    if mapsize < (*env).me_mapsize {
        mapsize = (*env).me_mapsize;
    }
    if flags & 0x80000 as libc::c_int as libc::c_uint != 0 {
        (*mp).mm_mapsize = mapsize;
        (*mp)
            .mm_dbs[0 as libc::c_int
            as usize] = *((*txn).mt_dbs).offset(0 as libc::c_int as isize);
        (*mp)
            .mm_dbs[1 as libc::c_int
            as usize] = *((*txn).mt_dbs).offset(1 as libc::c_int as isize);
        (*mp)
            .mm_last_pg = ((*txn).mt_next_pgno)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        ::std::ptr::write_volatile(&mut (*mp).mm_txnid as *mut txnid_t, (*txn).mt_txnid);
        if flags & (0x40000 as libc::c_int | 0x10000 as libc::c_int) as libc::c_uint == 0
        {
            let mut meta_size: libc::c_uint = (*env).me_psize;
            rc = if (*env).me_flags & 0x100000 as libc::c_int as libc::c_uint != 0 {
                1 as libc::c_int
            } else {
                4 as libc::c_int
            };
            ptr = (mp as *mut libc::c_char)
                .offset(-(16 as libc::c_ulong as libc::c_uint as isize));
            r2 = (ptr.offset_from((*env).me_map) as libc::c_long
                & ((*env).me_os_psize).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as libc::c_long) as libc::c_int;
            ptr = ptr.offset(-(r2 as isize));
            meta_size = meta_size.wrapping_add(r2 as libc::c_uint);
            if msync(ptr as *mut libc::c_void, meta_size as size_t, rc) != 0 {
                rc = *__errno_location();
                current_block = 4207514812656518121;
            } else {
                current_block = 16676093276374978246;
            }
        } else {
            current_block = 16676093276374978246;
        }
    } else {
        ::std::ptr::write_volatile(&mut metab.mm_txnid as *mut txnid_t, (*mp).mm_txnid);
        metab.mm_last_pg = (*mp).mm_last_pg;
        meta.mm_mapsize = mapsize;
        meta
            .mm_dbs[0 as libc::c_int
            as usize] = *((*txn).mt_dbs).offset(0 as libc::c_int as isize);
        meta
            .mm_dbs[1 as libc::c_int
            as usize] = *((*txn).mt_dbs).offset(1 as libc::c_int as isize);
        meta
            .mm_last_pg = ((*txn).mt_next_pgno)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        ::std::ptr::write_volatile(&mut meta.mm_txnid as *mut txnid_t, (*txn).mt_txnid);
        off = 16 as libc::c_ulong as off_t;
        ptr = (&mut meta as *mut MDB_meta as *mut libc::c_char).offset(off as isize);
        len = (::std::mem::size_of::<MDB_meta>() as libc::c_ulong)
            .wrapping_sub(off as libc::c_ulong) as libc::c_int;
        off += (mp as *mut libc::c_char).offset_from((*env).me_map) as libc::c_long;
        mfd = if flags
            & (0x10000 as libc::c_int | 0x40000 as libc::c_int) as libc::c_uint != 0
        {
            (*env).me_fd
        } else {
            (*env).me_mfd
        };
        loop {
            rc = pwrite(mfd, ptr as *const libc::c_void, len as size_t, off)
                as libc::c_int;
            if !(rc != len) {
                current_block = 16676093276374978246;
                break;
            }
            rc = if rc < 0 as libc::c_int {
                *__errno_location()
            } else {
                5 as libc::c_int
            };
            if rc == 4 as libc::c_int {
                continue;
            }
            meta.mm_last_pg = metab.mm_last_pg;
            ::std::ptr::write_volatile(
                &mut meta.mm_txnid as *mut txnid_t,
                metab.mm_txnid,
            );
            r2 = pwrite((*env).me_fd, ptr as *const libc::c_void, len as size_t, off)
                as libc::c_int;
            current_block = 4207514812656518121;
            break;
        }
    }
    match current_block {
        16676093276374978246 => {
            if !((*env).me_txns).is_null() {
                ::std::ptr::write_volatile(
                    &mut (*(*env).me_txns).mt1.mtb.mtb_txnid as *mut txnid_t,
                    (*txn).mt_txnid,
                );
            }
            return 0 as libc::c_int;
        }
        _ => {
            (*env).me_flags |= 0x80000000 as libc::c_uint;
            return rc;
        }
    };
}
unsafe extern "C" fn mdb_env_pick_meta(mut env: *const MDB_env) -> *mut MDB_meta {
    let mut metas: *const *mut MDB_meta = ((*env).me_metas).as_ptr();
    return *metas
        .offset(
            (((**metas.offset(0 as libc::c_int as isize)).mm_txnid
                < (**metas.offset(1 as libc::c_int as isize)).mm_txnid) as libc::c_int
                ^ ((*env).me_flags & 0x2000000 as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint) as libc::c_int) as isize,
        );
}
pub unsafe extern "C" fn mdb_env_create(mut env: *mut *mut MDB_env) -> libc::c_int {
    let mut e: *mut MDB_env = 0 as *mut MDB_env;
    e = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<MDB_env>() as libc::c_ulong,
    ) as *mut MDB_env;
    if e.is_null() {
        return 12 as libc::c_int;
    }
    (*e).me_maxreaders = 126 as libc::c_int as libc::c_uint;
    (*e).me_numdbs = 2 as libc::c_int as MDB_dbi;
    (*e).me_maxdbs = (*e).me_numdbs;
    (*e).me_fd = -(1 as libc::c_int);
    (*e).me_lfd = -(1 as libc::c_int);
    (*e).me_mfd = -(1 as libc::c_int);
    (*e).me_pid = getpid();
    (*e).me_os_psize = sysconf(_SC_PAGESIZE as libc::c_int) as libc::c_uint;
    *env = e;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_env_map(
    mut env: *mut MDB_env,
    mut addr: *mut libc::c_void,
) -> libc::c_int {
    let mut p: *mut MDB_page = 0 as *mut MDB_page;
    let mut flags: libc::c_uint = (*env).me_flags;
    let mut mmap_flags: libc::c_int = 0x1 as libc::c_int;
    let mut prot: libc::c_int = 0x1 as libc::c_int;
    if flags & 0x80000 as libc::c_int as libc::c_uint != 0 {
        prot |= 0x2 as libc::c_int;
        if ftruncate((*env).me_fd, (*env).me_mapsize as __off_t) < 0 as libc::c_int {
            return *__errno_location();
        }
    }
    (*env)
        .me_map = mmap(
        addr,
        (*env).me_mapsize,
        prot,
        mmap_flags,
        (*env).me_fd,
        0 as libc::c_int as __off_t,
    ) as *mut libc::c_char;
    if (*env).me_map == -(1 as libc::c_int) as *mut libc::c_void as *mut libc::c_char {
        (*env).me_map = 0 as *mut libc::c_char;
        return *__errno_location();
    }
    if flags & 0x800000 as libc::c_int as libc::c_uint != 0 {
        madvise((*env).me_map as *mut libc::c_void, (*env).me_mapsize, 1 as libc::c_int);
    }
    if !addr.is_null() && (*env).me_map != addr as *mut libc::c_char {
        return 16 as libc::c_int;
    }
    p = (*env).me_map as *mut MDB_page;
    (*env)
        .me_metas[0 as libc::c_int
        as usize] = (p as *mut libc::c_char)
        .offset(16 as libc::c_ulong as libc::c_uint as isize) as *mut libc::c_void
        as *mut MDB_meta;
    (*env)
        .me_metas[1 as libc::c_int
        as usize] = ((*env).me_metas[0 as libc::c_int as usize] as *mut libc::c_char)
        .offset((*env).me_psize as isize) as *mut MDB_meta;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_set_mapsize(
    mut env: *mut MDB_env,
    mut size: mdb_size_t,
) -> libc::c_int {
    if !((*env).me_map).is_null() {
        let mut meta: *mut MDB_meta = 0 as *mut MDB_meta;
        let mut old: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut rc: libc::c_int = 0;
        if !((*env).me_txn).is_null() {
            return 22 as libc::c_int;
        }
        meta = mdb_env_pick_meta(env);
        if size == 0 {
            size = (*meta).mm_mapsize;
        }
        let mut minsize: mdb_size_t = ((*meta).mm_last_pg)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul((*env).me_psize as libc::c_ulong);
        if size < minsize {
            size = minsize;
        }
        munmap((*env).me_map as *mut libc::c_void, (*env).me_mapsize);
        (*env).me_mapsize = size;
        old = (if (*env).me_flags & 0x1 as libc::c_int as libc::c_uint != 0 {
            (*env).me_map
        } else {
            0 as *mut libc::c_char
        }) as *mut libc::c_void;
        rc = mdb_env_map(env, old);
        if rc != 0 {
            return rc;
        }
    }
    (*env).me_mapsize = size;
    if (*env).me_psize != 0 {
        (*env)
            .me_maxpg = ((*env).me_mapsize)
            .wrapping_div((*env).me_psize as libc::c_ulong);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_set_maxdbs(
    mut env: *mut MDB_env,
    mut dbs: MDB_dbi,
) -> libc::c_int {
    if !((*env).me_map).is_null() {
        return 22 as libc::c_int;
    }
    (*env).me_maxdbs = dbs.wrapping_add(2 as libc::c_int as libc::c_uint);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_set_maxreaders(
    mut env: *mut MDB_env,
    mut readers: libc::c_uint,
) -> libc::c_int {
    if !((*env).me_map).is_null() || readers < 1 as libc::c_int as libc::c_uint {
        return 22 as libc::c_int;
    }
    (*env).me_maxreaders = readers;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_get_maxreaders(
    mut env: *mut MDB_env,
    mut readers: *mut libc::c_uint,
) -> libc::c_int {
    if env.is_null() || readers.is_null() {
        return 22 as libc::c_int;
    }
    *readers = (*env).me_maxreaders;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_fsize(
    mut fd: libc::c_int,
    mut size: *mut mdb_size_t,
) -> libc::c_int {
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
    if fstat(fd, &mut st) != 0 {
        return *__errno_location();
    }
    *size = st.st_size as mdb_size_t;
    return 0 as libc::c_int;
}
static mut mdb_suffixes: [[*const mdb_nchar_t; 2]; 2] = [
    [
        b"/data.mdb\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"/lock.mdb\0" as *const u8 as *const libc::c_char,
        b"-lock\0" as *const u8 as *const libc::c_char,
    ],
];
unsafe extern "C" fn mdb_fname_init(
    mut path: *const libc::c_char,
    mut envflags: libc::c_uint,
    mut fname: *mut MDB_name,
) -> libc::c_int {
    let mut no_suffix: libc::c_int = (envflags
        & (0x4000 as libc::c_int | 0x400000 as libc::c_int) as libc::c_uint
        == (0x4000 as libc::c_int | 0x400000 as libc::c_int) as libc::c_uint)
        as libc::c_int;
    (*fname).mn_alloced = 0 as libc::c_int;
    (*fname).mn_len = strlen(path) as libc::c_int;
    if no_suffix != 0 {
        (*fname).mn_val = path as *mut libc::c_char;
    } else {
        (*fname)
            .mn_val = malloc(
            ((*fname).mn_len + 9 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
        ) as *mut mdb_nchar_t;
        if !((*fname).mn_val).is_null() {
            (*fname).mn_alloced = 1 as libc::c_int;
            strcpy((*fname).mn_val, path);
        } else {
            return 12 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_fopen(
    mut env: *const MDB_env,
    mut fname: *mut MDB_name,
    mut which: mdb_fopen_type,
    mut mode: mdb_mode_t,
    mut res: *mut libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut fd: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    if (*fname).mn_alloced != 0 {
        strcpy(
            ((*fname).mn_val).offset((*fname).mn_len as isize),
            mdb_suffixes[(which as libc::c_uint
                == MDB_O_LOCKS as libc::c_int as libc::c_uint) as libc::c_int
                as usize][((*env).me_flags & 0x4000 as libc::c_int as libc::c_uint
                == 0x4000 as libc::c_int as libc::c_uint) as libc::c_int as usize],
        );
    }
    fd = open(
        (*fname).mn_val,
        (which as libc::c_uint & MDB_O_MASK as libc::c_int as libc::c_uint)
            as libc::c_int,
        mode,
    );
    if fd == -(1 as libc::c_int) {
        rc = *__errno_location();
    } else {
        if which as libc::c_uint != MDB_O_RDONLY as libc::c_int as libc::c_uint
            && which as libc::c_uint != MDB_O_RDWR as libc::c_int as libc::c_uint
        {
            if 0o2000000 as libc::c_int == 0
                && {
                    flags = fcntl(fd, 1 as libc::c_int);
                    flags != -(1 as libc::c_int)
                }
            {
                fcntl(fd, 2 as libc::c_int, flags | 1 as libc::c_int);
            }
        }
        if which as libc::c_uint == MDB_O_COPY as libc::c_int as libc::c_uint
            && (*env).me_psize >= (*env).me_os_psize
        {
            flags = fcntl(fd, 3 as libc::c_int);
            if flags != -(1 as libc::c_int) {
                fcntl(fd, 4 as libc::c_int, flags | 0o40000 as libc::c_int);
            }
        }
    }
    *res = fd;
    return rc;
}
unsafe extern "C" fn mdb_env_open2(
    mut env: *mut MDB_env,
    mut prev: libc::c_int,
) -> libc::c_int {
    let mut flags: libc::c_uint = (*env).me_flags;
    let mut i: libc::c_int = 0;
    let mut newenv: libc::c_int = 0 as libc::c_int;
    let mut rc: libc::c_int = 0;
    let mut meta: MDB_meta = MDB_meta {
        mm_magic: 0,
        mm_version: 0,
        mm_address: 0 as *mut libc::c_void,
        mm_mapsize: 0,
        mm_dbs: [MDB_db {
            md_pad: 0,
            md_flags: 0,
            md_depth: 0,
            md_branch_pages: 0,
            md_leaf_pages: 0,
            md_overflow_pages: 0,
            md_entries: 0,
            md_root: 0,
        }; 2],
        mm_last_pg: 0,
        mm_txnid: 0,
    };
    let mut st: statfs = statfs {
        f_type: 0,
        f_bsize: 0,
        f_blocks: 0,
        f_bfree: 0,
        f_bavail: 0,
        f_files: 0,
        f_ffree: 0,
        f_fsid: __fsid_t { __val: [0; 2] },
        f_namelen: 0,
        f_frsize: 0,
        f_flags: 0,
        f_spare: [0; 4],
    };
    fstatfs((*env).me_fd, &mut st);
    let mut current_block_7: u64;
    if st.f_type == 0xef53 as libc::c_int as libc::c_long {
        let mut uts: utsname = utsname {
            sysname: [0; 65],
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
            domainname: [0; 65],
        };
        let mut i_0: libc::c_int = 0;
        uname(&mut uts);
        if (uts.release[0 as libc::c_int as usize] as libc::c_int) < '3' as i32 {
            if strncmp(
                (uts.release).as_mut_ptr(),
                b"2.6.32.\0" as *const u8 as *const libc::c_char,
                7 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                i_0 = atoi((uts.release).as_mut_ptr().offset(7 as libc::c_int as isize));
                if i_0 >= 60 as libc::c_int {
                    current_block_7 = 11307063007268554308;
                } else {
                    current_block_7 = 11042950489265723346;
                }
            } else if strncmp(
                (uts.release).as_mut_ptr(),
                b"2.6.34.\0" as *const u8 as *const libc::c_char,
                7 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                i_0 = atoi((uts.release).as_mut_ptr().offset(7 as libc::c_int as isize));
                if i_0 >= 15 as libc::c_int {
                    current_block_7 = 11307063007268554308;
                } else {
                    current_block_7 = 11042950489265723346;
                }
            } else {
                current_block_7 = 11042950489265723346;
            }
        } else if uts.release[0 as libc::c_int as usize] as libc::c_int == '3' as i32 {
            i_0 = atoi((uts.release).as_mut_ptr().offset(2 as libc::c_int as isize));
            if i_0 > 5 as libc::c_int {
                current_block_7 = 11307063007268554308;
            } else if i_0 == 5 as libc::c_int {
                i_0 = atoi((uts.release).as_mut_ptr().offset(4 as libc::c_int as isize));
                if i_0 >= 4 as libc::c_int {
                    current_block_7 = 11307063007268554308;
                } else {
                    current_block_7 = 11042950489265723346;
                }
            } else if i_0 == 2 as libc::c_int {
                i_0 = atoi((uts.release).as_mut_ptr().offset(4 as libc::c_int as isize));
                if i_0 >= 30 as libc::c_int {
                    current_block_7 = 11307063007268554308;
                } else {
                    current_block_7 = 11042950489265723346;
                }
            } else {
                current_block_7 = 11042950489265723346;
            }
        } else {
            current_block_7 = 11307063007268554308;
        }
        match current_block_7 {
            11307063007268554308 => {}
            _ => {
                (*env).me_flags |= 0x8000000 as libc::c_uint;
            }
        }
    }
    i = mdb_env_read_header(env, prev, &mut meta);
    if i != 0 as libc::c_int {
        if i != 2 as libc::c_int {
            return i;
        }
        newenv = 1 as libc::c_int;
        (*env).me_psize = (*env).me_os_psize;
        if (*env).me_psize
            > (if (if 0 as libc::c_int != 0 {
                16 as libc::c_ulong as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) != 0
            {
                0x10000 as libc::c_int
            } else {
                0x8000 as libc::c_int
            }) as libc::c_uint
        {
            (*env)
                .me_psize = (if if 0 as libc::c_int != 0 {
                16 as libc::c_ulong as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            } != 0
            {
                0x10000 as libc::c_int
            } else {
                0x8000 as libc::c_int
            }) as libc::c_uint;
        }
        memset(
            &mut meta as *mut MDB_meta as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<MDB_meta>() as libc::c_ulong,
        );
        mdb_env_init_meta0(env, &mut meta);
        meta.mm_mapsize = 1048576 as libc::c_int as mdb_size_t;
    } else {
        (*env).me_psize = meta.mm_dbs[0 as libc::c_int as usize].md_pad;
    }
    if (*env).me_mapsize == 0 {
        (*env).me_mapsize = meta.mm_mapsize;
    }
    let mut minsize: mdb_size_t = (meta.mm_last_pg)
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(meta.mm_dbs[0 as libc::c_int as usize].md_pad as libc::c_ulong);
    if (*env).me_mapsize < minsize {
        (*env).me_mapsize = minsize;
    }
    meta.mm_mapsize = (*env).me_mapsize;
    if newenv != 0 && flags & 0x1 as libc::c_int as libc::c_uint == 0 {
        rc = mdb_env_init_meta(env, &mut meta);
        if rc != 0 {
            return rc;
        }
        newenv = 0 as libc::c_int;
    }
    rc = mdb_env_map(
        env,
        if flags & 0x1 as libc::c_int as libc::c_uint != 0 {
            meta.mm_address
        } else {
            0 as *mut libc::c_void
        },
    );
    if rc != 0 {
        return rc;
    }
    if newenv != 0 {
        if flags & 0x1 as libc::c_int as libc::c_uint != 0 {
            meta.mm_address = (*env).me_map as *mut libc::c_void;
        }
        i = mdb_env_init_meta(env, &mut meta);
        if i != 0 as libc::c_int {
            return i;
        }
    }
    (*env)
        .me_maxfree_1pg = (((*env).me_psize)
        .wrapping_sub(16 as libc::c_ulong as libc::c_uint) as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<pgno_t>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    (*env)
        .me_nodemax = ((((*env).me_psize)
        .wrapping_sub(16 as libc::c_ulong as libc::c_uint)
        .wrapping_div(2 as libc::c_int as libc::c_uint)
        & -(2 as libc::c_int) as libc::c_uint) as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<indx_t>() as libc::c_ulong) as libc::c_uint;
    (*env).me_maxpg = ((*env).me_mapsize).wrapping_div((*env).me_psize as libc::c_ulong);
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_env_reader_dest(mut ptr: *mut libc::c_void) {
    let mut reader: *mut MDB_reader = ptr as *mut MDB_reader;
    if (*reader).mru.mrx.mrb_pid == getpid() {
        ::std::ptr::write_volatile(
            &mut (*reader).mru.mrx.mrb_pid as *mut pid_t,
            0 as libc::c_int,
        );
    }
}
unsafe extern "C" fn mdb_env_share_locks(
    mut env: *mut MDB_env,
    mut excl: *mut libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut meta: *mut MDB_meta = mdb_env_pick_meta(env);
    ::std::ptr::write_volatile(
        &mut (*(*env).me_txns).mt1.mtb.mtb_txnid as *mut txnid_t,
        (*meta).mm_txnid,
    );
    let mut lock_info: flock = flock {
        l_type: 0,
        l_whence: 0,
        l_start: 0,
        l_len: 0,
        l_pid: 0,
    };
    memset(
        &mut lock_info as *mut flock as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<flock>() as libc::c_ulong,
    );
    lock_info.l_type = 0 as libc::c_int as libc::c_short;
    lock_info.l_whence = 0 as libc::c_int as libc::c_short;
    lock_info.l_start = 0 as libc::c_int as __off_t;
    lock_info.l_len = 1 as libc::c_int as __off_t;
    loop {
        rc = fcntl((*env).me_lfd, 6 as libc::c_int, &mut lock_info as *mut flock);
        if !(rc != 0
            && {
                rc = *__errno_location();
                rc == 4 as libc::c_int
            })
        {
            break;
        }
    }
    *excl = if rc != 0 { -(1 as libc::c_int) } else { 0 as libc::c_int };
    return rc;
}
unsafe extern "C" fn mdb_env_excl_lock(
    mut env: *mut MDB_env,
    mut excl: *mut libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut lock_info: flock = flock {
        l_type: 0,
        l_whence: 0,
        l_start: 0,
        l_len: 0,
        l_pid: 0,
    };
    memset(
        &mut lock_info as *mut flock as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<flock>() as libc::c_ulong,
    );
    lock_info.l_type = 1 as libc::c_int as libc::c_short;
    lock_info.l_whence = 0 as libc::c_int as libc::c_short;
    lock_info.l_start = 0 as libc::c_int as __off_t;
    lock_info.l_len = 1 as libc::c_int as __off_t;
    loop {
        rc = fcntl((*env).me_lfd, 6 as libc::c_int, &mut lock_info as *mut flock);
        if !(rc != 0
            && {
                rc = *__errno_location();
                rc == 4 as libc::c_int
            })
        {
            break;
        }
    }
    if rc == 0 {
        *excl = 1 as libc::c_int;
    } else {
        lock_info.l_type = 0 as libc::c_int as libc::c_short;
        loop {
            rc = fcntl((*env).me_lfd, 7 as libc::c_int, &mut lock_info as *mut flock);
            if !(rc != 0
                && {
                    rc = *__errno_location();
                    rc == 4 as libc::c_int
                })
            {
                break;
            }
        }
        if rc == 0 as libc::c_int {
            *excl = 0 as libc::c_int;
        }
    }
    return rc;
}
unsafe extern "C" fn mdb_env_setup_locks(
    mut env: *mut MDB_env,
    mut fname: *mut MDB_name,
    mut mode: libc::c_int,
    mut excl: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut size: off_t = 0;
    let mut rsize: off_t = 0;
    rc = mdb_fopen(env, fname, MDB_O_LOCKS, mode as mdb_mode_t, &mut (*env).me_lfd);
    if rc != 0 {
        if rc == 30 as libc::c_int
            && (*env).me_flags & 0x20000 as libc::c_int as libc::c_uint != 0
        {
            return 0 as libc::c_int;
        }
    } else {
        if (*env).me_flags & 0x200000 as libc::c_int as libc::c_uint == 0 {
            rc = pthread_key_create(
                &mut (*env).me_txkey,
                Some(
                    mdb_env_reader_dest as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            );
            if rc != 0 {
                current_block = 12328564411458367264;
            } else {
                (*env).me_flags |= 0x10000000 as libc::c_uint;
                current_block = 14523784380283086299;
            }
        } else {
            current_block = 14523784380283086299;
        }
        match current_block {
            12328564411458367264 => {}
            _ => {
                rc = mdb_env_excl_lock(env, excl);
                if !(rc != 0) {
                    size = lseek(
                        (*env).me_lfd,
                        0 as libc::c_int as __off_t,
                        2 as libc::c_int,
                    );
                    if size == -(1 as libc::c_int) as libc::c_long {
                        current_block = 5964929934061908518;
                    } else {
                        rsize = (((*env).me_maxreaders)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<MDB_reader>() as libc::c_ulong,
                            )
                            .wrapping_add(
                                ::std::mem::size_of::<MDB_txninfo>() as libc::c_ulong,
                            ) as off_t;
                        if size < rsize && *excl > 0 as libc::c_int {
                            if ftruncate((*env).me_lfd, rsize) != 0 as libc::c_int {
                                current_block = 5964929934061908518;
                            } else {
                                current_block = 5143058163439228106;
                            }
                        } else {
                            rsize = size;
                            size = (rsize as libc::c_ulong)
                                .wrapping_sub(
                                    ::std::mem::size_of::<MDB_txninfo>() as libc::c_ulong,
                                ) as off_t;
                            (*env)
                                .me_maxreaders = (size as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<MDB_reader>() as libc::c_ulong,
                                )
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as libc::c_uint;
                            current_block = 5143058163439228106;
                        }
                        match current_block {
                            5964929934061908518 => {}
                            _ => {
                                let mut m: *mut libc::c_void = mmap(
                                    0 as *mut libc::c_void,
                                    rsize as size_t,
                                    0x1 as libc::c_int | 0x2 as libc::c_int,
                                    0x1 as libc::c_int,
                                    (*env).me_lfd,
                                    0 as libc::c_int as __off_t,
                                );
                                if m == -(1 as libc::c_int) as *mut libc::c_void {
                                    current_block = 5964929934061908518;
                                } else {
                                    (*env).me_txns = m as *mut MDB_txninfo;
                                    if *excl > 0 as libc::c_int {
                                        let mut mattr: pthread_mutexattr_t = pthread_mutexattr_t {
                                            __size: [0; 4],
                                        };
                                        memset(
                                            ((*(*env).me_txns).mt1.mtb.mtb_rmutex).as_mut_ptr()
                                                as *mut libc::c_void,
                                            0 as libc::c_int,
                                            ::std::mem::size_of::<pthread_mutex_t>() as libc::c_ulong,
                                        );
                                        memset(
                                            ((*(*env).me_txns).mt2.mt2_wmutex).as_mut_ptr()
                                                as *mut libc::c_void,
                                            0 as libc::c_int,
                                            ::std::mem::size_of::<pthread_mutex_t>() as libc::c_ulong,
                                        );
                                        rc = pthread_mutexattr_init(&mut mattr);
                                        if rc != 0 as libc::c_int {
                                            current_block = 12328564411458367264;
                                        } else {
                                            rc = pthread_mutexattr_setpshared(
                                                &mut mattr,
                                                PTHREAD_PROCESS_SHARED as libc::c_int,
                                            );
                                            if rc == 0 {
                                                rc = pthread_mutexattr_setrobust(
                                                    &mut mattr,
                                                    PTHREAD_MUTEX_ROBUST as libc::c_int,
                                                );
                                            }
                                            if rc == 0 {
                                                rc = pthread_mutex_init(
                                                    ((*(*env).me_txns).mt1.mtb.mtb_rmutex).as_mut_ptr(),
                                                    &mut mattr,
                                                );
                                            }
                                            if rc == 0 {
                                                rc = pthread_mutex_init(
                                                    ((*(*env).me_txns).mt2.mt2_wmutex).as_mut_ptr(),
                                                    &mut mattr,
                                                );
                                            }
                                            pthread_mutexattr_destroy(&mut mattr);
                                            if rc != 0 {
                                                current_block = 12328564411458367264;
                                            } else {
                                                (*(*env).me_txns)
                                                    .mt1
                                                    .mtb
                                                    .mtb_magic = 0xbeefc0de as libc::c_uint;
                                                (*(*env).me_txns)
                                                    .mt1
                                                    .mtb
                                                    .mtb_format = ((if 0 as libc::c_int != 0 {
                                                    999 as libc::c_int
                                                } else {
                                                    2 as libc::c_int
                                                }) as libc::c_uint)
                                                    .wrapping_rem((1 as libc::c_uint) << 12 as libc::c_int)
                                                    .wrapping_add(
                                                        (MDB_lock_desc as libc::c_int as libc::c_uint)
                                                            .wrapping_mul((1 as libc::c_uint) << 12 as libc::c_int),
                                                    );
                                                ::std::ptr::write_volatile(
                                                    &mut (*(*env).me_txns).mt1.mtb.mtb_txnid as *mut txnid_t,
                                                    0 as libc::c_int as txnid_t,
                                                );
                                                ::std::ptr::write_volatile(
                                                    &mut (*(*env).me_txns).mt1.mtb.mtb_numreaders
                                                        as *mut libc::c_uint,
                                                    0 as libc::c_int as libc::c_uint,
                                                );
                                                current_block = 3934796541983872331;
                                            }
                                        }
                                    } else if (*(*env).me_txns).mt1.mtb.mtb_magic
                                        != 0xbeefc0de as libc::c_uint
                                    {
                                        rc = -(30793 as libc::c_int);
                                        current_block = 12328564411458367264;
                                    } else if (*(*env).me_txns).mt1.mtb.mtb_format
                                        != ((if 0 as libc::c_int != 0 {
                                            999 as libc::c_int
                                        } else {
                                            2 as libc::c_int
                                        }) as libc::c_uint)
                                            .wrapping_rem((1 as libc::c_uint) << 12 as libc::c_int)
                                            .wrapping_add(
                                                (MDB_lock_desc as libc::c_int as libc::c_uint)
                                                    .wrapping_mul((1 as libc::c_uint) << 12 as libc::c_int),
                                            )
                                    {
                                        rc = -(30794 as libc::c_int);
                                        current_block = 12328564411458367264;
                                    } else {
                                        rc = *__errno_location();
                                        if rc != 0 && rc != 13 as libc::c_int
                                            && rc != 11 as libc::c_int
                                        {
                                            current_block = 12328564411458367264;
                                        } else {
                                            current_block = 3934796541983872331;
                                        }
                                    }
                                    match current_block {
                                        12328564411458367264 => {}
                                        _ => return 0 as libc::c_int,
                                    }
                                }
                            }
                        }
                    }
                    match current_block {
                        12328564411458367264 => {}
                        _ => {
                            rc = *__errno_location();
                        }
                    }
                }
            }
        }
    }
    return rc;
}
pub unsafe extern "C" fn mdb_env_open(
    mut env: *mut MDB_env,
    mut path: *const libc::c_char,
    mut flags: libc::c_uint,
    mut mode: mdb_mode_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut excl: libc::c_int = -(1 as libc::c_int);
    let mut fname: MDB_name = MDB_name {
        mn_len: 0,
        mn_alloced: 0,
        mn_val: 0 as *mut mdb_nchar_t,
    };
    if (*env).me_fd != -(1 as libc::c_int)
        || flags
            & !(0x10000 as libc::c_int | 0x40000 as libc::c_int | 0x100000 as libc::c_int
                | 0x1000000 as libc::c_int
                | (0x1 as libc::c_int | 0x4000 as libc::c_int | 0x20000 as libc::c_int
                    | 0x80000 as libc::c_int | 0x200000 as libc::c_int
                    | 0x400000 as libc::c_int | 0x800000 as libc::c_int
                    | 0x2000000 as libc::c_int)) as libc::c_uint != 0
    {
        return 22 as libc::c_int;
    }
    flags |= (*env).me_flags;
    rc = mdb_fname_init(path, flags, &mut fname);
    if rc != 0 {
        return rc;
    }
    flags |= 0x20000000 as libc::c_uint;
    if flags & 0x20000 as libc::c_int as libc::c_uint != 0 {
        flags &= !(0x80000 as libc::c_int) as libc::c_uint;
    } else {
        (*env)
            .me_free_pgs = mdb_midl_alloc(
            ((1 as libc::c_int) << 16 as libc::c_int + 1 as libc::c_int)
                - 1 as libc::c_int,
        );
        if !(!((*env).me_free_pgs).is_null()
            && {
                (*env)
                    .me_dirty_list = calloc(
                    ((1 as libc::c_int) << 16 as libc::c_int + 1 as libc::c_int)
                        as libc::c_ulong,
                    ::std::mem::size_of::<MDB_ID2>() as libc::c_ulong,
                ) as MDB_ID2L;
                !((*env).me_dirty_list).is_null()
            })
        {
            rc = 12 as libc::c_int;
        }
    }
    (*env).me_flags = flags;
    if !(rc != 0) {
        (*env).me_path = strdup(path);
        (*env)
            .me_dbxs = calloc(
            (*env).me_maxdbs as libc::c_ulong,
            ::std::mem::size_of::<MDB_dbx>() as libc::c_ulong,
        ) as *mut MDB_dbx;
        (*env)
            .me_dbflags = calloc(
            (*env).me_maxdbs as libc::c_ulong,
            ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
        ) as *mut uint16_t;
        (*env)
            .me_dbiseqs = calloc(
            (*env).me_maxdbs as libc::c_ulong,
            ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
        ) as *mut libc::c_uint;
        if !(!((*env).me_dbxs).is_null() && !((*env).me_path).is_null()
            && !((*env).me_dbflags).is_null() && !((*env).me_dbiseqs).is_null())
        {
            rc = 12 as libc::c_int;
        } else {
            let ref mut fresh18 = (*((*env).me_dbxs).offset(0 as libc::c_int as isize))
                .md_cmp;
            *fresh18 = Some(mdb_cmp_long as MDB_cmp_func);
            if flags & (0x20000 as libc::c_int | 0x400000 as libc::c_int) as libc::c_uint
                == 0
            {
                rc = mdb_env_setup_locks(
                    env,
                    &mut fname,
                    mode as libc::c_int,
                    &mut excl,
                );
                if rc != 0 {
                    current_block = 6425188171773001776;
                } else if flags & 0x2000000 as libc::c_int as libc::c_uint != 0
                    && excl == 0
                {
                    rc = 11 as libc::c_int;
                    current_block = 6425188171773001776;
                } else {
                    current_block = 5783071609795492627;
                }
            } else {
                current_block = 5783071609795492627;
            }
            match current_block {
                6425188171773001776 => {}
                _ => {
                    rc = mdb_fopen(
                        env,
                        &mut fname,
                        (if flags & 0x20000 as libc::c_int as libc::c_uint != 0 {
                            MDB_O_RDONLY as libc::c_int
                        } else {
                            MDB_O_RDWR as libc::c_int
                        }) as mdb_fopen_type,
                        mode,
                        &mut (*env).me_fd,
                    );
                    if !(rc != 0) {
                        if flags
                            & (0x20000 as libc::c_int | 0x400000 as libc::c_int)
                                as libc::c_uint == 0x20000 as libc::c_int as libc::c_uint
                        {
                            rc = mdb_env_setup_locks(
                                env,
                                &mut fname,
                                mode as libc::c_int,
                                &mut excl,
                            );
                            if rc != 0 {
                                current_block = 6425188171773001776;
                            } else {
                                current_block = 4068382217303356765;
                            }
                        } else {
                            current_block = 4068382217303356765;
                        }
                        match current_block {
                            6425188171773001776 => {}
                            _ => {
                                rc = mdb_env_open2(
                                    env,
                                    (flags & 0x2000000 as libc::c_int as libc::c_uint)
                                        as libc::c_int,
                                );
                                if rc == 0 as libc::c_int {
                                    if flags
                                        & (0x20000 as libc::c_int | 0x80000 as libc::c_int)
                                            as libc::c_uint == 0
                                    {
                                        rc = mdb_fopen(
                                            env,
                                            &mut fname,
                                            MDB_O_META,
                                            mode,
                                            &mut (*env).me_mfd,
                                        );
                                        if rc != 0 {
                                            current_block = 6425188171773001776;
                                        } else {
                                            current_block = 8693738493027456495;
                                        }
                                    } else {
                                        current_block = 8693738493027456495;
                                    }
                                    match current_block {
                                        6425188171773001776 => {}
                                        _ => {
                                            if excl > 0 as libc::c_int
                                                && flags & 0x2000000 as libc::c_int as libc::c_uint == 0
                                            {
                                                rc = mdb_env_share_locks(env, &mut excl);
                                                if rc != 0 {
                                                    current_block = 6425188171773001776;
                                                } else {
                                                    current_block = 14136749492126903395;
                                                }
                                            } else {
                                                current_block = 14136749492126903395;
                                            }
                                            match current_block {
                                                6425188171773001776 => {}
                                                _ => {
                                                    if flags & 0x20000 as libc::c_int as libc::c_uint == 0 {
                                                        let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
                                                        let mut tsize: libc::c_int = ::std::mem::size_of::<
                                                            MDB_txn,
                                                        >() as libc::c_ulong as libc::c_int;
                                                        let mut size: libc::c_int = (tsize as libc::c_ulong)
                                                            .wrapping_add(
                                                                ((*env).me_maxdbs as libc::c_ulong)
                                                                    .wrapping_mul(
                                                                        (::std::mem::size_of::<MDB_db>() as libc::c_ulong)
                                                                            .wrapping_add(
                                                                                ::std::mem::size_of::<*mut MDB_cursor>() as libc::c_ulong,
                                                                            )
                                                                            .wrapping_add(
                                                                                ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                                                                            )
                                                                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                                                    ),
                                                            ) as libc::c_int;
                                                        (*env)
                                                            .me_pbuf = calloc(
                                                            1 as libc::c_int as libc::c_ulong,
                                                            (*env).me_psize as libc::c_ulong,
                                                        );
                                                        if !((*env).me_pbuf).is_null()
                                                            && {
                                                                txn = calloc(
                                                                    1 as libc::c_int as libc::c_ulong,
                                                                    size as libc::c_ulong,
                                                                ) as *mut MDB_txn;
                                                                !txn.is_null()
                                                            }
                                                        {
                                                            (*txn)
                                                                .mt_dbs = (txn as *mut libc::c_char).offset(tsize as isize)
                                                                as *mut MDB_db;
                                                            (*txn)
                                                                .mt_cursors = ((*txn).mt_dbs)
                                                                .offset((*env).me_maxdbs as isize) as *mut *mut MDB_cursor;
                                                            (*txn)
                                                                .mt_dbiseqs = ((*txn).mt_cursors)
                                                                .offset((*env).me_maxdbs as isize) as *mut libc::c_uint;
                                                            (*txn)
                                                                .mt_dbflags = ((*txn).mt_dbiseqs)
                                                                .offset((*env).me_maxdbs as isize) as *mut libc::c_uchar;
                                                            (*txn).mt_env = env;
                                                            (*txn).mt_dbxs = (*env).me_dbxs;
                                                            (*txn).mt_flags = 0x1 as libc::c_int as libc::c_uint;
                                                            (*env).me_txn0 = txn;
                                                        } else {
                                                            rc = 12 as libc::c_int;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if rc != 0 {
        mdb_env_close0(env, excl);
    }
    if fname.mn_alloced != 0 {
        free(fname.mn_val as *mut libc::c_void);
    }
    return rc;
}
unsafe extern "C" fn mdb_env_close0(mut env: *mut MDB_env, mut excl: libc::c_int) {
    let mut i: libc::c_int = 0;
    if (*env).me_flags & 0x20000000 as libc::c_uint == 0 {
        return;
    }
    if !((*env).me_dbxs).is_null() {
        i = (*env).me_maxdbs as libc::c_int;
        loop {
            i -= 1;
            if !(i >= 2 as libc::c_int) {
                break;
            }
            free((*((*env).me_dbxs).offset(i as isize)).md_name.mv_data);
        }
        free((*env).me_dbxs as *mut libc::c_void);
    }
    free((*env).me_pbuf);
    free((*env).me_dbiseqs as *mut libc::c_void);
    free((*env).me_dbflags as *mut libc::c_void);
    free((*env).me_path as *mut libc::c_void);
    free((*env).me_dirty_list as *mut libc::c_void);
    free((*env).me_txn0 as *mut libc::c_void);
    mdb_midl_free((*env).me_free_pgs);
    if (*env).me_flags & 0x10000000 as libc::c_uint != 0 {
        pthread_key_delete((*env).me_txkey);
    }
    if !((*env).me_map).is_null() {
        munmap((*env).me_map as *mut libc::c_void, (*env).me_mapsize);
    }
    if (*env).me_mfd != -(1 as libc::c_int) {
        close((*env).me_mfd);
    }
    if (*env).me_fd != -(1 as libc::c_int) {
        close((*env).me_fd);
    }
    if !((*env).me_txns).is_null() {
        let mut pid: pid_t = getpid();
        i = (*env).me_close_readers;
        loop {
            i -= 1;
            if !(i >= 0 as libc::c_int) {
                break;
            }
            if (*((*(*env).me_txns).mti_readers).as_mut_ptr().offset(i as isize))
                .mru
                .mrx
                .mrb_pid == pid
            {
                ::std::ptr::write_volatile(
                    &mut (*((*(*env).me_txns).mti_readers)
                        .as_mut_ptr()
                        .offset(i as isize))
                        .mru
                        .mrx
                        .mrb_pid as *mut pid_t,
                    0 as libc::c_int,
                );
            }
        }
        if excl == 0 as libc::c_int {
            mdb_env_excl_lock(env, &mut excl);
        }
        if excl > 0 as libc::c_int {
            pthread_mutex_destroy(((*(*env).me_txns).mt1.mtb.mtb_rmutex).as_mut_ptr());
            pthread_mutex_destroy(((*(*env).me_txns).mt2.mt2_wmutex).as_mut_ptr());
        }
        munmap(
            (*env).me_txns as *mut libc::c_void,
            (((*env).me_maxreaders).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<MDB_reader>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<MDB_txninfo>() as libc::c_ulong),
        );
    }
    if (*env).me_lfd != -(1 as libc::c_int) {
        close((*env).me_lfd);
    }
    (*env).me_flags &= !(0x20000000 as libc::c_uint | 0x10000000 as libc::c_uint);
}
pub unsafe extern "C" fn mdb_env_close(mut env: *mut MDB_env) {
    let mut dp: *mut MDB_page = 0 as *mut MDB_page;
    if env.is_null() {
        return;
    }
    loop {
        dp = (*env).me_dpages;
        if dp.is_null() {
            break;
        }
        (*env).me_dpages = (*dp).mp_p.p_next;
        free(dp as *mut libc::c_void);
    }
    mdb_env_close0(env, 0 as libc::c_int);
    free(env as *mut libc::c_void);
}
unsafe extern "C" fn mdb_cmp_long(
    mut a: *const MDB_val,
    mut b: *const MDB_val,
) -> libc::c_int {
    return if *((*a).mv_data as *mut mdb_size_t) < *((*b).mv_data as *mut mdb_size_t) {
        -(1 as libc::c_int)
    } else {
        (*((*a).mv_data as *mut mdb_size_t) > *((*b).mv_data as *mut mdb_size_t))
            as libc::c_int
    };
}
unsafe extern "C" fn mdb_cmp_int(
    mut a: *const MDB_val,
    mut b: *const MDB_val,
) -> libc::c_int {
    return if *((*a).mv_data as *mut libc::c_uint) < *((*b).mv_data as *mut libc::c_uint)
    {
        -(1 as libc::c_int)
    } else {
        (*((*a).mv_data as *mut libc::c_uint) > *((*b).mv_data as *mut libc::c_uint))
            as libc::c_int
    };
}
unsafe extern "C" fn mdb_cmp_cint(
    mut a: *const MDB_val,
    mut b: *const MDB_val,
) -> libc::c_int {
    let mut u: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut c: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut x: libc::c_int = 0;
    u = ((*a).mv_data as *mut libc::c_char).offset((*a).mv_size as isize)
        as *mut libc::c_ushort;
    c = ((*b).mv_data as *mut libc::c_char).offset((*a).mv_size as isize)
        as *mut libc::c_ushort;
    loop {
        u = u.offset(-1);
        c = c.offset(-1);
        x = *u as libc::c_int - *c as libc::c_int;
        if !(x == 0 && u > (*a).mv_data as *mut libc::c_ushort) {
            break;
        }
    }
    return x;
}
unsafe extern "C" fn mdb_cmp_memn(
    mut a: *const MDB_val,
    mut b: *const MDB_val,
) -> libc::c_int {
    let mut diff: libc::c_int = 0;
    let mut len_diff: ssize_t = 0;
    let mut len: libc::c_uint = 0;
    len = (*a).mv_size as libc::c_uint;
    len_diff = (*a).mv_size as ssize_t - (*b).mv_size as ssize_t;
    if len_diff > 0 as libc::c_int as libc::c_long {
        len = (*b).mv_size as libc::c_uint;
        len_diff = 1 as libc::c_int as ssize_t;
    }
    diff = memcmp((*a).mv_data, (*b).mv_data, len as libc::c_ulong);
    return (if diff != 0 {
        diff as libc::c_long
    } else if len_diff < 0 as libc::c_int as libc::c_long {
        -(1 as libc::c_int) as libc::c_long
    } else {
        len_diff
    }) as libc::c_int;
}
unsafe extern "C" fn mdb_cmp_memnr(
    mut a: *const MDB_val,
    mut b: *const MDB_val,
) -> libc::c_int {
    let mut p1: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut p2: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut p1_lim: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut len_diff: ssize_t = 0;
    let mut diff: libc::c_int = 0;
    p1_lim = (*a).mv_data as *const libc::c_uchar;
    p1 = ((*a).mv_data as *const libc::c_uchar).offset((*a).mv_size as isize);
    p2 = ((*b).mv_data as *const libc::c_uchar).offset((*b).mv_size as isize);
    len_diff = (*a).mv_size as ssize_t - (*b).mv_size as ssize_t;
    if len_diff > 0 as libc::c_int as libc::c_long {
        p1_lim = p1_lim.offset(len_diff as isize);
        len_diff = 1 as libc::c_int as ssize_t;
    }
    while p1 > p1_lim {
        p1 = p1.offset(-1);
        p2 = p2.offset(-1);
        diff = *p1 as libc::c_int - *p2 as libc::c_int;
        if diff != 0 {
            return diff;
        }
    }
    return (if len_diff < 0 as libc::c_int as libc::c_long {
        -(1 as libc::c_int) as libc::c_long
    } else {
        len_diff
    }) as libc::c_int;
}
unsafe extern "C" fn mdb_node_search(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut exactp: *mut libc::c_int,
) -> *mut MDB_node {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut nkeys: libc::c_uint = 0;
    let mut low: libc::c_int = 0;
    let mut high: libc::c_int = 0;
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut mp: *mut MDB_page = (*mc).mc_pg[(*mc).mc_top as usize];
    let mut node: *mut MDB_node = 0 as *mut MDB_node;
    let mut nodekey: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut cmp: Option::<MDB_cmp_func> = None;
    nkeys = ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
        .wrapping_sub(
            (16 as libc::c_ulong as libc::c_uint)
                .wrapping_sub(
                    (if 0 as libc::c_int != 0 {
                        16 as libc::c_ulong as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                ),
        ) >> 1 as libc::c_int;
    low = if (*mp).mp_flags as libc::c_int & 0x2 as libc::c_int == 0x2 as libc::c_int {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    high = nkeys.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
    cmp = (*(*mc).mc_dbx).md_cmp;
    if cmp == Some(mdb_cmp_cint as MDB_cmp_func)
        && (*mp).mp_flags as libc::c_int & 0x1 as libc::c_int == 0x1 as libc::c_int
    {
        if (*((mp as *mut libc::c_char)
            .offset(
                *((*mp).mp_ptrs).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as libc::c_int as isize,
            )
            .offset(
                (if 0 as libc::c_int != 0 {
                    16 as libc::c_ulong as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }) as isize,
            ) as *mut MDB_node))
            .mn_ksize as libc::c_ulong
            == ::std::mem::size_of::<mdb_size_t>() as libc::c_ulong
        {
            cmp = Some(mdb_cmp_long as MDB_cmp_func);
        } else {
            cmp = Some(mdb_cmp_int as MDB_cmp_func);
        }
    }
    if (*mp).mp_flags as libc::c_int & 0x20 as libc::c_int == 0x20 as libc::c_int {
        nodekey.mv_size = (*(*mc).mc_db).md_pad as size_t;
        node = (mp as *mut libc::c_char)
            .offset(
                *((*mp).mp_ptrs).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as libc::c_int as isize,
            )
            .offset(
                (if 0 as libc::c_int != 0 {
                    16 as libc::c_ulong as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }) as isize,
            ) as *mut MDB_node;
        while low <= high {
            i = (low + high >> 1 as libc::c_int) as libc::c_uint;
            nodekey
                .mv_data = (mp as *mut libc::c_char)
                .offset(16 as libc::c_ulong as libc::c_uint as isize)
                .offset((i as libc::c_ulong).wrapping_mul(nodekey.mv_size) as isize)
                as *mut libc::c_void;
            rc = cmp.unwrap()(key, &mut nodekey);
            if rc == 0 as libc::c_int {
                break;
            }
            if rc > 0 as libc::c_int {
                low = i.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
            } else {
                high = i.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
            }
        }
    } else {
        while low <= high {
            i = (low + high >> 1 as libc::c_int) as libc::c_uint;
            node = (mp as *mut libc::c_char)
                .offset(
                    *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize) as libc::c_int
                        as isize,
                )
                .offset(
                    (if 0 as libc::c_int != 0 {
                        16 as libc::c_ulong as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }) as isize,
                ) as *mut MDB_node;
            nodekey.mv_size = (*node).mn_ksize as size_t;
            nodekey.mv_data = ((*node).mn_data).as_mut_ptr() as *mut libc::c_void;
            rc = cmp.unwrap()(key, &mut nodekey);
            if rc == 0 as libc::c_int {
                break;
            }
            if rc > 0 as libc::c_int {
                low = i.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
            } else {
                high = i.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
            }
        }
    }
    if rc > 0 as libc::c_int {
        i = i.wrapping_add(1);
        i;
        if !((*mp).mp_flags as libc::c_int & 0x20 as libc::c_int == 0x20 as libc::c_int)
        {
            node = (mp as *mut libc::c_char)
                .offset(
                    *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize) as libc::c_int
                        as isize,
                )
                .offset(
                    (if 0 as libc::c_int != 0 {
                        16 as libc::c_ulong as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }) as isize,
                ) as *mut MDB_node;
        }
    }
    if !exactp.is_null() {
        *exactp = (rc == 0 as libc::c_int && nkeys > 0 as libc::c_int as libc::c_uint)
            as libc::c_int;
    }
    (*mc).mc_ki[(*mc).mc_top as usize] = i as indx_t;
    if i >= nkeys {
        return 0 as *mut MDB_node;
    }
    return node;
}
unsafe extern "C" fn mdb_cursor_pop(mut mc: *mut MDB_cursor) {
    if (*mc).mc_snum != 0 {
        (*mc).mc_snum = ((*mc).mc_snum).wrapping_sub(1);
        (*mc).mc_snum;
        if (*mc).mc_snum != 0 {
            (*mc).mc_top = ((*mc).mc_top).wrapping_sub(1);
            (*mc).mc_top;
        } else {
            (*mc).mc_flags &= !(0x1 as libc::c_int) as libc::c_uint;
        }
    }
}
unsafe extern "C" fn mdb_cursor_push(
    mut mc: *mut MDB_cursor,
    mut mp: *mut MDB_page,
) -> libc::c_int {
    if (*mc).mc_snum as libc::c_int >= 32 as libc::c_int {
        (*(*mc).mc_txn).mt_flags |= 0x2 as libc::c_int as libc::c_uint;
        return -(30787 as libc::c_int);
    }
    let fresh19 = (*mc).mc_snum;
    (*mc).mc_snum = ((*mc).mc_snum).wrapping_add(1);
    (*mc).mc_top = fresh19;
    (*mc).mc_pg[(*mc).mc_top as usize] = mp;
    (*mc).mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_page_get(
    mut mc: *mut MDB_cursor,
    mut pgno: pgno_t,
    mut ret: *mut *mut MDB_page,
    mut lvl: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut txn: *mut MDB_txn = (*mc).mc_txn;
    let mut p: *mut MDB_page = 0 as *mut MDB_page;
    let mut level: libc::c_int = 0;
    if (*mc).mc_flags & (0x20000 as libc::c_int | 0x80000 as libc::c_int) as libc::c_uint
        == 0
    {
        let mut tx2: *mut MDB_txn = txn;
        level = 1 as libc::c_int;
        loop {
            let mut dl: MDB_ID2L = (*tx2).mt_u.dirty_list;
            let mut x: libc::c_uint = 0;
            if !((*tx2).mt_spill_pgs).is_null() {
                let mut pn: MDB_ID = pgno << 1 as libc::c_int;
                x = mdb_midl_search((*tx2).mt_spill_pgs, pn);
                if x as libc::c_ulong
                    <= *((*tx2).mt_spill_pgs).offset(0 as libc::c_int as isize)
                    && *((*tx2).mt_spill_pgs).offset(x as isize) == pn
                {
                    current_block = 11723105132364514939;
                    break;
                }
            }
            if (*dl.offset(0 as libc::c_int as isize)).mid != 0 {
                let mut x_0: libc::c_uint = mdb_mid2l_search(dl, pgno);
                if x_0 as libc::c_ulong <= (*dl.offset(0 as libc::c_int as isize)).mid
                    && (*dl.offset(x_0 as isize)).mid == pgno
                {
                    p = (*dl.offset(x_0 as isize)).mptr as *mut MDB_page;
                    current_block = 2892667683710788131;
                    break;
                }
            }
            level += 1;
            level;
            tx2 = (*tx2).mt_parent;
            if tx2.is_null() {
                current_block = 3512920355445576850;
                break;
            }
        }
    } else {
        current_block = 3512920355445576850;
    }
    match current_block {
        3512920355445576850 => {
            if pgno >= (*txn).mt_next_pgno {
                (*txn).mt_flags |= 0x2 as libc::c_int as libc::c_uint;
                return -(30797 as libc::c_int);
            }
            level = 0 as libc::c_int;
            current_block = 11723105132364514939;
        }
        _ => {}
    }
    match current_block {
        11723105132364514939 => {
            let mut env: *mut MDB_env = (*txn).mt_env;
            p = ((*env).me_map)
                .offset(((*env).me_psize as libc::c_ulong).wrapping_mul(pgno) as isize)
                as *mut MDB_page;
        }
        _ => {}
    }
    *ret = p;
    if !lvl.is_null() {
        *lvl = level;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_page_search_root(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut mp: *mut MDB_page = (*mc).mc_pg[(*mc).mc_top as usize];
    let mut rc: libc::c_int = 0;
    while (*mp).mp_flags as libc::c_int & 0x1 as libc::c_int == 0x1 as libc::c_int {
        let mut current_block_30: u64;
        let mut node: *mut MDB_node = 0 as *mut MDB_node;
        let mut i: indx_t = 0;
        if (*mc).mc_dbi == 0
            || ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                .wrapping_sub(
                    (16 as libc::c_ulong as libc::c_uint)
                        .wrapping_sub(
                            (if 0 as libc::c_int != 0 {
                                16 as libc::c_ulong as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }),
                        ),
                ) >> 1 as libc::c_int > 1 as libc::c_int as libc::c_uint
        {} else {
            mdb_assert_fail(
                (*(*mc).mc_txn).mt_env,
                b"!mc->mc_dbi || NUMKEYS(mp) > 1\0" as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"mdb_page_search_root\0"))
                    .as_ptr(),
                b"mdb.c\0" as *const u8 as *const libc::c_char,
                6482 as libc::c_int,
            );
        };
        if flags & (4 as libc::c_int | 8 as libc::c_int) != 0 {
            i = 0 as libc::c_int as indx_t;
            if flags & 8 as libc::c_int != 0 {
                i = (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                    .wrapping_sub(
                        (16 as libc::c_ulong as libc::c_uint)
                            .wrapping_sub(
                                (if 0 as libc::c_int != 0 {
                                    16 as libc::c_ulong as libc::c_uint
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }),
                            ),
                    ) >> 1 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as indx_t;
                if (*mc).mc_flags & 0x1 as libc::c_int as libc::c_uint != 0 {
                    if (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int
                        == i as libc::c_int
                    {
                        let fresh20 = (*mc).mc_snum;
                        (*mc).mc_snum = ((*mc).mc_snum).wrapping_add(1);
                        (*mc).mc_top = fresh20;
                        mp = (*mc).mc_pg[(*mc).mc_top as usize];
                        current_block_30 = 10271957417922996484;
                    } else {
                        current_block_30 = 4495394744059808450;
                    }
                } else {
                    current_block_30 = 4495394744059808450;
                }
            } else {
                current_block_30 = 4495394744059808450;
            }
        } else {
            let mut exact: libc::c_int = 0;
            node = mdb_node_search(mc, key, &mut exact);
            if node.is_null() {
                i = (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                    .wrapping_sub(
                        (16 as libc::c_ulong as libc::c_uint)
                            .wrapping_sub(
                                (if 0 as libc::c_int != 0 {
                                    16 as libc::c_ulong as libc::c_uint
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }),
                            ),
                    ) >> 1 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as indx_t;
            } else {
                i = (*mc).mc_ki[(*mc).mc_top as usize];
                if exact == 0 {
                    if i as libc::c_int > 0 as libc::c_int {} else {
                        mdb_assert_fail(
                            (*(*mc).mc_txn).mt_env,
                            b"i > 0\0" as *const u8 as *const libc::c_char,
                            (*::std::mem::transmute::<
                                &[u8; 21],
                                &[libc::c_char; 21],
                            >(b"mdb_page_search_root\0"))
                                .as_ptr(),
                            b"mdb.c\0" as *const u8 as *const libc::c_char,
                            6506 as libc::c_int,
                        );
                    };
                    i = i.wrapping_sub(1);
                    i;
                }
            }
            current_block_30 = 4495394744059808450;
        }
        match current_block_30 {
            4495394744059808450 => {
                if (i as libc::c_uint)
                    < ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                        .wrapping_sub(
                            (16 as libc::c_ulong as libc::c_uint)
                                .wrapping_sub(
                                    (if 0 as libc::c_int != 0 {
                                        16 as libc::c_ulong as libc::c_uint
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    }),
                                ),
                        ) >> 1 as libc::c_int
                {} else {
                    mdb_assert_fail(
                        (*(*mc).mc_txn).mt_env,
                        b"i < NUMKEYS(mp)\0" as *const u8 as *const libc::c_char,
                        (*::std::mem::transmute::<
                            &[u8; 21],
                            &[libc::c_char; 21],
                        >(b"mdb_page_search_root\0"))
                            .as_ptr(),
                        b"mdb.c\0" as *const u8 as *const libc::c_char,
                        6513 as libc::c_int,
                    );
                };
                node = (mp as *mut libc::c_char)
                    .offset(
                        *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize) as libc::c_int
                            as isize,
                    )
                    .offset(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                rc = mdb_page_get(
                    mc,
                    (*node).mn_lo as libc::c_ulong
                        | ((*node).mn_hi as pgno_t) << 16 as libc::c_int
                        | (if (if -(1 as libc::c_int) as pgno_t
                            > 0xffffffff as libc::c_uint as libc::c_ulong
                        {
                            32 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) != 0
                        {
                            ((*node).mn_flags as pgno_t)
                                << (if -(1 as libc::c_int) as pgno_t
                                    > 0xffffffff as libc::c_uint as libc::c_ulong
                                {
                                    32 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                        } else {
                            0 as libc::c_int as libc::c_ulong
                        }),
                    &mut mp,
                    0 as *mut libc::c_int,
                );
                if rc != 0 as libc::c_int {
                    return rc;
                }
                (*mc).mc_ki[(*mc).mc_top as usize] = i;
                rc = mdb_cursor_push(mc, mp);
                if rc != 0 {
                    return rc;
                }
            }
            _ => {}
        }
        if flags & 1 as libc::c_int != 0 {
            rc = mdb_page_touch(mc);
            if rc != 0 as libc::c_int {
                return rc;
            }
            mp = (*mc).mc_pg[(*mc).mc_top as usize];
        }
    }
    if !((*mp).mp_flags as libc::c_int & 0x2 as libc::c_int == 0x2 as libc::c_int) {
        (*(*mc).mc_txn).mt_flags |= 0x2 as libc::c_int as libc::c_uint;
        return -(30796 as libc::c_int);
    }
    (*mc).mc_flags |= 0x1 as libc::c_int as libc::c_uint;
    (*mc).mc_flags &= !(0x2 as libc::c_int) as libc::c_uint;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_page_search_lowest(mut mc: *mut MDB_cursor) -> libc::c_int {
    let mut mp: *mut MDB_page = (*mc).mc_pg[(*mc).mc_top as usize];
    let mut node: *mut MDB_node = (mp as *mut libc::c_char)
        .offset(
            *((*mp).mp_ptrs).as_mut_ptr().offset(0 as libc::c_int as isize)
                as libc::c_int as isize,
        )
        .offset(
            (if 0 as libc::c_int != 0 {
                16 as libc::c_ulong as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        ) as *mut MDB_node;
    let mut rc: libc::c_int = 0;
    rc = mdb_page_get(
        mc,
        (*node).mn_lo as libc::c_ulong | ((*node).mn_hi as pgno_t) << 16 as libc::c_int
            | (if (if -(1 as libc::c_int) as pgno_t
                > 0xffffffff as libc::c_uint as libc::c_ulong
            {
                32 as libc::c_int
            } else {
                0 as libc::c_int
            }) != 0
            {
                ((*node).mn_flags as pgno_t)
                    << (if -(1 as libc::c_int) as pgno_t
                        > 0xffffffff as libc::c_uint as libc::c_ulong
                    {
                        32 as libc::c_int
                    } else {
                        0 as libc::c_int
                    })
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        &mut mp,
        0 as *mut libc::c_int,
    );
    if rc != 0 as libc::c_int {
        return rc;
    }
    (*mc).mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
    rc = mdb_cursor_push(mc, mp);
    if rc != 0 {
        return rc;
    }
    return mdb_page_search_root(mc, 0 as *mut MDB_val, 4 as libc::c_int);
}
unsafe extern "C" fn mdb_page_search(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut root: pgno_t = 0;
    if (*(*mc).mc_txn).mt_flags
        & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
        != 0
    {
        return -(30782 as libc::c_int)
    } else {
        if *(*mc).mc_dbflag as libc::c_int & 0x2 as libc::c_int != 0 {
            let mut mc2: MDB_cursor = MDB_cursor {
                mc_next: 0 as *mut MDB_cursor,
                mc_backup: 0 as *mut MDB_cursor,
                mc_xcursor: 0 as *mut MDB_xcursor,
                mc_txn: 0 as *mut MDB_txn,
                mc_dbi: 0,
                mc_db: 0 as *mut MDB_db,
                mc_dbx: 0 as *mut MDB_dbx,
                mc_dbflag: 0 as *mut libc::c_uchar,
                mc_snum: 0,
                mc_top: 0,
                mc_flags: 0,
                mc_pg: [0 as *mut MDB_page; 32],
                mc_ki: [0; 32],
            };
            if *((*(*mc).mc_txn).mt_dbiseqs).offset((*mc).mc_dbi as isize)
                != *((*(*(*mc).mc_txn).mt_env).me_dbiseqs).offset((*mc).mc_dbi as isize)
            {
                return -(30780 as libc::c_int);
            }
            mdb_cursor_init(
                &mut mc2,
                (*mc).mc_txn,
                1 as libc::c_int as MDB_dbi,
                0 as *mut MDB_xcursor,
            );
            rc = mdb_page_search(
                &mut mc2,
                &mut (*(*mc).mc_dbx).md_name,
                0 as libc::c_int,
            );
            if rc != 0 {
                return rc;
            }
            let mut data: MDB_val = MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            };
            let mut exact: libc::c_int = 0 as libc::c_int;
            let mut flags_0: uint16_t = 0;
            let mut leaf: *mut MDB_node = mdb_node_search(
                &mut mc2,
                &mut (*(*mc).mc_dbx).md_name,
                &mut exact,
            );
            if exact == 0 {
                return -(30798 as libc::c_int);
            }
            if (*leaf).mn_flags as libc::c_int
                & (0x4 as libc::c_int | 0x2 as libc::c_int) != 0x2 as libc::c_int
            {
                return -(30784 as libc::c_int);
            }
            rc = mdb_node_read(&mut mc2, leaf, &mut data);
            if rc != 0 {
                return rc;
            }
            memcpy(
                &mut flags_0 as *mut uint16_t as *mut libc::c_void,
                (data.mv_data as *mut libc::c_char).offset(4 as libc::c_ulong as isize)
                    as *const libc::c_void,
                ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
            );
            if (*(*mc).mc_db).md_flags as libc::c_int
                & (0xffff as libc::c_int & !(0x8000 as libc::c_int))
                != flags_0 as libc::c_int
            {
                return -(30784 as libc::c_int);
            }
            memcpy(
                (*mc).mc_db as *mut libc::c_void,
                data.mv_data,
                ::std::mem::size_of::<MDB_db>() as libc::c_ulong,
            );
            *(*mc)
                .mc_dbflag = (*(*mc).mc_dbflag as libc::c_int & !(0x2 as libc::c_int))
                as libc::c_uchar;
        }
        root = (*(*mc).mc_db).md_root;
        if root == !(0 as libc::c_int as pgno_t) {
            return -(30798 as libc::c_int);
        }
    }
    if root > 1 as libc::c_int as libc::c_ulong {} else {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"root > 1\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"mdb_page_search\0"))
                .as_ptr(),
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            6633 as libc::c_int,
        );
    };
    if ((*mc).mc_pg[0 as libc::c_int as usize]).is_null()
        || (*(*mc).mc_pg[0 as libc::c_int as usize]).mp_p.p_pgno != root
    {
        rc = mdb_page_get(
            mc,
            root,
            &mut *((*mc).mc_pg).as_mut_ptr().offset(0 as libc::c_int as isize),
            0 as *mut libc::c_int,
        );
        if rc != 0 as libc::c_int {
            return rc;
        }
    }
    (*mc).mc_snum = 1 as libc::c_int as libc::c_ushort;
    (*mc).mc_top = 0 as libc::c_int as libc::c_ushort;
    if flags & 1 as libc::c_int != 0 {
        rc = mdb_page_touch(mc);
        if rc != 0 {
            return rc;
        }
    }
    if flags & 2 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    return mdb_page_search_root(mc, key, flags);
}
unsafe extern "C" fn mdb_ovpage_free(
    mut mc: *mut MDB_cursor,
    mut mp: *mut MDB_page,
) -> libc::c_int {
    let mut txn: *mut MDB_txn = (*mc).mc_txn;
    let mut pg: pgno_t = (*mp).mp_p.p_pgno;
    let mut x: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut ovpages: libc::c_uint = (*mp).mp_pb.pb_pages;
    let mut env: *mut MDB_env = (*txn).mt_env;
    let mut sl: MDB_IDL = (*txn).mt_spill_pgs;
    let mut pn: MDB_ID = pg << 1 as libc::c_int;
    let mut rc: libc::c_int = 0;
    if !((*env).me_pgstate.mf_pghead).is_null() && ((*txn).mt_parent).is_null()
        && ((*mp).mp_flags as libc::c_int & 0x10 as libc::c_int != 0
            || !sl.is_null()
                && {
                    x = mdb_midl_search(sl, pn);
                    x as libc::c_ulong <= *sl.offset(0 as libc::c_int as isize)
                } && *sl.offset(x as isize) == pn)
    {
        let mut i: libc::c_uint = 0;
        let mut j: libc::c_uint = 0;
        let mut mop: *mut pgno_t = 0 as *mut pgno_t;
        let mut dl: *mut MDB_ID2 = 0 as *mut MDB_ID2;
        let mut ix: MDB_ID2 = MDB_ID2 {
            mid: 0,
            mptr: 0 as *mut libc::c_void,
        };
        let mut iy: MDB_ID2 = MDB_ID2 {
            mid: 0,
            mptr: 0 as *mut libc::c_void,
        };
        rc = mdb_midl_need(&mut (*env).me_pgstate.mf_pghead, ovpages);
        if rc != 0 {
            return rc;
        }
        if (*mp).mp_flags as libc::c_int & 0x10 as libc::c_int == 0 {
            if x as libc::c_ulong == *sl.offset(0 as libc::c_int as isize) {
                let ref mut fresh21 = *sl.offset(0 as libc::c_int as isize);
                *fresh21 = (*fresh21).wrapping_sub(1);
                *fresh21;
            } else {
                let ref mut fresh22 = *sl.offset(x as isize);
                *fresh22 |= 1 as libc::c_int as libc::c_ulong;
            }
        } else {
            dl = (*txn).mt_u.dirty_list;
            let ref mut fresh23 = (*dl.offset(0 as libc::c_int as isize)).mid;
            let fresh24 = *fresh23;
            *fresh23 = (*fresh23).wrapping_sub(1);
            x = fresh24 as libc::c_uint;
            ix = *dl.offset(x as isize);
            while ix.mptr != mp as *mut libc::c_void {
                if x > 1 as libc::c_int as libc::c_uint {
                    x = x.wrapping_sub(1);
                    x;
                    iy = *dl.offset(x as isize);
                    *dl.offset(x as isize) = ix;
                } else {
                    if x > 1 as libc::c_int as libc::c_uint {} else {
                        mdb_assert_fail(
                            (*(*mc).mc_txn).mt_env,
                            b"x > 1\0" as *const u8 as *const libc::c_char,
                            (*::std::mem::transmute::<
                                &[u8; 16],
                                &[libc::c_char; 16],
                            >(b"mdb_ovpage_free\0"))
                                .as_ptr(),
                            b"mdb.c\0" as *const u8 as *const libc::c_char,
                            6715 as libc::c_int,
                        );
                    };
                    let ref mut fresh25 = (*dl.offset(0 as libc::c_int as isize)).mid;
                    *fresh25 = (*fresh25).wrapping_add(1);
                    j = *fresh25 as libc::c_uint;
                    *dl.offset(j as isize) = ix;
                    (*txn).mt_flags |= 0x2 as libc::c_int as libc::c_uint;
                    return -(30779 as libc::c_int);
                }
                ix = iy;
            }
            (*txn).mt_dirty_room = ((*txn).mt_dirty_room).wrapping_add(1);
            (*txn).mt_dirty_room;
            if (*env).me_flags & 0x80000 as libc::c_int as libc::c_uint == 0 {
                mdb_dpage_free(env, mp);
            }
        }
        mop = (*env).me_pgstate.mf_pghead;
        j = (*mop.offset(0 as libc::c_int as isize))
            .wrapping_add(ovpages as libc::c_ulong) as libc::c_uint;
        i = *mop.offset(0 as libc::c_int as isize) as libc::c_uint;
        while i != 0 && *mop.offset(i as isize) < pg {
            let fresh26 = j;
            j = j.wrapping_sub(1);
            *mop.offset(fresh26 as isize) = *mop.offset(i as isize);
            i = i.wrapping_sub(1);
            i;
        }
        while j > i {
            let fresh27 = pg;
            pg = pg.wrapping_add(1);
            let fresh28 = j;
            j = j.wrapping_sub(1);
            *mop.offset(fresh28 as isize) = fresh27;
        }
        let ref mut fresh29 = *mop.offset(0 as libc::c_int as isize);
        *fresh29 = (*fresh29 as libc::c_ulong).wrapping_add(ovpages as libc::c_ulong)
            as pgno_t as pgno_t;
    } else {
        rc = mdb_midl_append_range(&mut (*txn).mt_free_pgs, pg, ovpages);
        if rc != 0 {
            return rc;
        }
    }
    (*(*mc).mc_db)
        .md_overflow_pages = ((*(*mc).mc_db).md_overflow_pages as libc::c_ulong)
        .wrapping_sub(ovpages as libc::c_ulong) as pgno_t as pgno_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_node_read(
    mut mc: *mut MDB_cursor,
    mut leaf: *mut MDB_node,
    mut data: *mut MDB_val,
) -> libc::c_int {
    let mut omp: *mut MDB_page = 0 as *mut MDB_page;
    let mut pgno: pgno_t = 0;
    let mut rc: libc::c_int = 0;
    !(0 as *mut MDB_page).is_null();
    if !((*leaf).mn_flags as libc::c_int & 0x1 as libc::c_int == 0x1 as libc::c_int) {
        (*data)
            .mv_size = ((*leaf).mn_lo as libc::c_uint
            | ((*leaf).mn_hi as libc::c_uint) << 16 as libc::c_int) as size_t;
        (*data)
            .mv_data = ((*leaf).mn_data)
            .as_mut_ptr()
            .offset((*leaf).mn_ksize as libc::c_int as isize) as *mut libc::c_void;
        return 0 as libc::c_int;
    }
    (*data)
        .mv_size = ((*leaf).mn_lo as libc::c_uint
        | ((*leaf).mn_hi as libc::c_uint) << 16 as libc::c_int) as size_t;
    memcpy(
        &mut pgno as *mut pgno_t as *mut libc::c_void,
        ((*leaf).mn_data).as_mut_ptr().offset((*leaf).mn_ksize as libc::c_int as isize)
            as *mut libc::c_void,
        ::std::mem::size_of::<pgno_t>() as libc::c_ulong,
    );
    rc = mdb_page_get(mc, pgno, &mut omp, 0 as *mut libc::c_int);
    if rc != 0 as libc::c_int {
        return rc;
    }
    (*data)
        .mv_data = (omp as *mut libc::c_char)
        .offset(16 as libc::c_ulong as libc::c_uint as isize) as *mut libc::c_void;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_get(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> libc::c_int {
    let mut mc: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut mx: MDB_xcursor = MDB_xcursor {
        mx_cursor: MDB_cursor {
            mc_next: 0 as *mut MDB_cursor,
            mc_backup: 0 as *mut MDB_cursor,
            mc_xcursor: 0 as *mut MDB_xcursor,
            mc_txn: 0 as *mut MDB_txn,
            mc_dbi: 0,
            mc_db: 0 as *mut MDB_db,
            mc_dbx: 0 as *mut MDB_dbx,
            mc_dbflag: 0 as *mut libc::c_uchar,
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [0 as *mut MDB_page; 32],
            mc_ki: [0; 32],
        },
        mx_db: MDB_db {
            md_pad: 0,
            md_flags: 0,
            md_depth: 0,
            md_branch_pages: 0,
            md_leaf_pages: 0,
            md_overflow_pages: 0,
            md_entries: 0,
            md_root: 0,
        },
        mx_dbx: MDB_dbx {
            md_name: MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            },
            md_cmp: None,
            md_dcmp: None,
            md_rel: None,
            md_relctx: 0 as *mut libc::c_void,
        },
        mx_dbflag: 0,
    };
    let mut exact: libc::c_int = 0 as libc::c_int;
    let mut rc: libc::c_int = 0;
    if key.is_null() || data.is_null()
        || !(!txn.is_null() && dbi < (*txn).mt_numdbs
            && *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int
                & 0x10 as libc::c_int != 0)
    {
        return 22 as libc::c_int;
    }
    if (*txn).mt_flags
        & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
        != 0
    {
        return -(30782 as libc::c_int);
    }
    mdb_cursor_init(&mut mc, txn, dbi, &mut mx);
    rc = mdb_cursor_set(&mut mc, key, data, MDB_SET, &mut exact);
    return rc;
}
unsafe extern "C" fn mdb_cursor_sibling(
    mut mc: *mut MDB_cursor,
    mut move_right: libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut indx: *mut MDB_node = 0 as *mut MDB_node;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    if ((*mc).mc_snum as libc::c_int) < 2 as libc::c_int {
        return -(30798 as libc::c_int);
    }
    mdb_cursor_pop(mc);
    if if move_right != 0 {
        (((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint)
            .wrapping_add(1 as libc::c_uint)
            >= ((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_lower as libc::c_uint)
                .wrapping_sub(
                    (16 as libc::c_ulong as libc::c_uint)
                        .wrapping_sub(
                            (if 0 as libc::c_int != 0 {
                                16 as libc::c_ulong as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }),
                        ),
                ) >> 1 as libc::c_int) as libc::c_int
    } else {
        ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int == 0 as libc::c_int)
            as libc::c_int
    } != 0
    {
        rc = mdb_cursor_sibling(mc, move_right);
        if rc != 0 as libc::c_int {
            (*mc).mc_top = ((*mc).mc_top).wrapping_add(1);
            (*mc).mc_top;
            (*mc).mc_snum = ((*mc).mc_snum).wrapping_add(1);
            (*mc).mc_snum;
            return rc;
        }
    } else if move_right != 0 {
        (*mc)
            .mc_ki[(*mc).mc_top
            as usize] = ((*mc).mc_ki[(*mc).mc_top as usize]).wrapping_add(1);
        (*mc).mc_ki[(*mc).mc_top as usize];
    } else {
        (*mc)
            .mc_ki[(*mc).mc_top
            as usize] = ((*mc).mc_ki[(*mc).mc_top as usize]).wrapping_sub(1);
        (*mc).mc_ki[(*mc).mc_top as usize];
    }
    if (*(*mc).mc_pg[(*mc).mc_top as usize]).mp_flags as libc::c_int & 0x1 as libc::c_int
        == 0x1 as libc::c_int
    {} else {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"IS_BRANCH(mc->mc_pg[mc->mc_top])\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"mdb_cursor_sibling\0"))
                .as_ptr(),
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            6857 as libc::c_int,
        );
    };
    indx = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
        .offset(
            *((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_ptrs)
                .as_mut_ptr()
                .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize) as libc::c_int
                as isize,
        )
        .offset(
            (if 0 as libc::c_int != 0 {
                16 as libc::c_ulong as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        ) as *mut MDB_node;
    rc = mdb_page_get(
        mc,
        (*indx).mn_lo as libc::c_ulong | ((*indx).mn_hi as pgno_t) << 16 as libc::c_int
            | (if (if -(1 as libc::c_int) as pgno_t
                > 0xffffffff as libc::c_uint as libc::c_ulong
            {
                32 as libc::c_int
            } else {
                0 as libc::c_int
            }) != 0
            {
                ((*indx).mn_flags as pgno_t)
                    << (if -(1 as libc::c_int) as pgno_t
                        > 0xffffffff as libc::c_uint as libc::c_ulong
                    {
                        32 as libc::c_int
                    } else {
                        0 as libc::c_int
                    })
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        &mut mp,
        0 as *mut libc::c_int,
    );
    if rc != 0 as libc::c_int {
        (*mc).mc_flags &= !(0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint;
        return rc;
    }
    mdb_cursor_push(mc, mp);
    if move_right == 0 {
        (*mc)
            .mc_ki[(*mc).mc_top
            as usize] = (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                (16 as libc::c_ulong as libc::c_uint)
                    .wrapping_sub(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }),
                    ),
            ) >> 1 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as indx_t;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_cursor_next(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut op: MDB_cursor_op,
) -> libc::c_int {
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut leaf: *mut MDB_node = 0 as *mut MDB_node;
    let mut rc: libc::c_int = 0;
    if (*mc).mc_flags & 0x8 as libc::c_int as libc::c_uint != 0
        && op as libc::c_uint == MDB_NEXT_DUP as libc::c_int as libc::c_uint
    {
        return -(30798 as libc::c_int);
    }
    if (*mc).mc_flags & 0x1 as libc::c_int as libc::c_uint == 0 {
        return mdb_cursor_first(mc, key, data);
    }
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    if (*mc).mc_flags & 0x2 as libc::c_int as libc::c_uint != 0 {
        if (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint
            >= (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                .wrapping_sub(
                    (16 as libc::c_ulong as libc::c_uint)
                        .wrapping_sub(
                            (if 0 as libc::c_int != 0 {
                                16 as libc::c_ulong as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }),
                        ),
                ) >> 1 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
        {
            return -(30798 as libc::c_int);
        }
        (*mc).mc_flags ^= 0x2 as libc::c_int as libc::c_uint;
    }
    if (*(*mc).mc_db).md_flags as libc::c_int & 0x4 as libc::c_int != 0 {
        leaf = (mp as *mut libc::c_char)
            .offset(
                *((*mp).mp_ptrs)
                    .as_mut_ptr()
                    .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize) as libc::c_int
                    as isize,
            )
            .offset(
                (if 0 as libc::c_int != 0 {
                    16 as libc::c_ulong as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }) as isize,
            ) as *mut MDB_node;
        if (*leaf).mn_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int {
            if op as libc::c_uint == MDB_NEXT as libc::c_int as libc::c_uint
                || op as libc::c_uint == MDB_NEXT_DUP as libc::c_int as libc::c_uint
            {
                rc = mdb_cursor_next(
                    &mut (*(*mc).mc_xcursor).mx_cursor,
                    data,
                    0 as *mut MDB_val,
                    MDB_NEXT,
                );
                if op as libc::c_uint != MDB_NEXT as libc::c_int as libc::c_uint
                    || rc != -(30798 as libc::c_int)
                {
                    if rc == 0 as libc::c_int {
                        if !key.is_null() {
                            (*key).mv_size = (*leaf).mn_ksize as size_t;
                            (*key)
                                .mv_data = ((*leaf).mn_data).as_mut_ptr()
                                as *mut libc::c_void;
                        }
                    }
                    return rc;
                }
            }
        } else {
            (*(*mc).mc_xcursor).mx_cursor.mc_flags
                &= !(0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint;
            if op as libc::c_uint == MDB_NEXT_DUP as libc::c_int as libc::c_uint {
                return -(30798 as libc::c_int);
            }
        }
    }
    if (*mc).mc_flags & 0x8 as libc::c_int as libc::c_uint != 0 {
        (*mc).mc_flags ^= 0x8 as libc::c_int as libc::c_uint;
    } else if ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint)
        .wrapping_add(1 as libc::c_uint)
        >= ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                (16 as libc::c_ulong as libc::c_uint)
                    .wrapping_sub(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }),
                    ),
            ) >> 1 as libc::c_int
    {
        rc = mdb_cursor_sibling(mc, 1 as libc::c_int);
        if rc != 0 as libc::c_int {
            (*mc).mc_flags |= 0x2 as libc::c_int as libc::c_uint;
            return rc;
        }
        mp = (*mc).mc_pg[(*mc).mc_top as usize];
    } else {
        (*mc)
            .mc_ki[(*mc).mc_top
            as usize] = ((*mc).mc_ki[(*mc).mc_top as usize]).wrapping_add(1);
        (*mc).mc_ki[(*mc).mc_top as usize];
    }
    if (*mp).mp_flags as libc::c_int & 0x20 as libc::c_int == 0x20 as libc::c_int {
        (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
        (*key)
            .mv_data = (mp as *mut libc::c_char)
            .offset(16 as libc::c_ulong as libc::c_uint as isize)
            .offset(
                ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_ulong)
                    .wrapping_mul((*key).mv_size) as isize,
            ) as *mut libc::c_void;
        return 0 as libc::c_int;
    }
    if (*mp).mp_flags as libc::c_int & 0x2 as libc::c_int == 0x2 as libc::c_int {} else {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"IS_LEAF(mp)\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"mdb_cursor_next\0"))
                .as_ptr(),
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            6946 as libc::c_int,
        );
    };
    leaf = (mp as *mut libc::c_char)
        .offset(
            *((*mp).mp_ptrs)
                .as_mut_ptr()
                .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize) as libc::c_int
                as isize,
        )
        .offset(
            (if 0 as libc::c_int != 0 {
                16 as libc::c_ulong as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        ) as *mut MDB_node;
    if (*leaf).mn_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int {
        mdb_xcursor_init1(mc, leaf);
        rc = mdb_cursor_first(
            &mut (*(*mc).mc_xcursor).mx_cursor,
            data,
            0 as *mut MDB_val,
        );
        if rc != 0 as libc::c_int {
            return rc;
        }
    } else if !data.is_null() {
        rc = mdb_node_read(mc, leaf, data);
        if rc != 0 as libc::c_int {
            return rc;
        }
    }
    if !key.is_null() {
        (*key).mv_size = (*leaf).mn_ksize as size_t;
        (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_cursor_prev(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut op: MDB_cursor_op,
) -> libc::c_int {
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut leaf: *mut MDB_node = 0 as *mut MDB_node;
    let mut rc: libc::c_int = 0;
    if (*mc).mc_flags & 0x1 as libc::c_int as libc::c_uint == 0 {
        rc = mdb_cursor_last(mc, key, data);
        if rc != 0 {
            return rc;
        }
        (*mc)
            .mc_ki[(*mc).mc_top
            as usize] = ((*mc).mc_ki[(*mc).mc_top as usize]).wrapping_add(1);
        (*mc).mc_ki[(*mc).mc_top as usize];
    }
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    if (*(*mc).mc_db).md_flags as libc::c_int & 0x4 as libc::c_int != 0
        && ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint)
            < ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                .wrapping_sub(
                    (16 as libc::c_ulong as libc::c_uint)
                        .wrapping_sub(
                            (if 0 as libc::c_int != 0 {
                                16 as libc::c_ulong as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }),
                        ),
                ) >> 1 as libc::c_int
    {
        leaf = (mp as *mut libc::c_char)
            .offset(
                *((*mp).mp_ptrs)
                    .as_mut_ptr()
                    .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize) as libc::c_int
                    as isize,
            )
            .offset(
                (if 0 as libc::c_int != 0 {
                    16 as libc::c_ulong as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }) as isize,
            ) as *mut MDB_node;
        if (*leaf).mn_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int {
            if op as libc::c_uint == MDB_PREV as libc::c_int as libc::c_uint
                || op as libc::c_uint == MDB_PREV_DUP as libc::c_int as libc::c_uint
            {
                rc = mdb_cursor_prev(
                    &mut (*(*mc).mc_xcursor).mx_cursor,
                    data,
                    0 as *mut MDB_val,
                    MDB_PREV,
                );
                if op as libc::c_uint != MDB_PREV as libc::c_int as libc::c_uint
                    || rc != -(30798 as libc::c_int)
                {
                    if rc == 0 as libc::c_int {
                        if !key.is_null() {
                            (*key).mv_size = (*leaf).mn_ksize as size_t;
                            (*key)
                                .mv_data = ((*leaf).mn_data).as_mut_ptr()
                                as *mut libc::c_void;
                        }
                        (*mc).mc_flags &= !(0x2 as libc::c_int) as libc::c_uint;
                    }
                    return rc;
                }
            }
        } else {
            (*(*mc).mc_xcursor).mx_cursor.mc_flags
                &= !(0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint;
            if op as libc::c_uint == MDB_PREV_DUP as libc::c_int as libc::c_uint {
                return -(30798 as libc::c_int);
            }
        }
    }
    (*mc).mc_flags &= !(0x2 as libc::c_int | 0x8 as libc::c_int) as libc::c_uint;
    if (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int == 0 as libc::c_int {
        rc = mdb_cursor_sibling(mc, 0 as libc::c_int);
        if rc != 0 as libc::c_int {
            return rc;
        }
        mp = (*mc).mc_pg[(*mc).mc_top as usize];
        (*mc)
            .mc_ki[(*mc).mc_top
            as usize] = (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                (16 as libc::c_ulong as libc::c_uint)
                    .wrapping_sub(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }),
                    ),
            ) >> 1 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as indx_t;
    } else {
        (*mc)
            .mc_ki[(*mc).mc_top
            as usize] = ((*mc).mc_ki[(*mc).mc_top as usize]).wrapping_sub(1);
        (*mc).mc_ki[(*mc).mc_top as usize];
    }
    if !((*mp).mp_flags as libc::c_int & 0x2 as libc::c_int == 0x2 as libc::c_int) {
        return -(30796 as libc::c_int);
    }
    if (*mp).mp_flags as libc::c_int & 0x20 as libc::c_int == 0x20 as libc::c_int {
        (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
        (*key)
            .mv_data = (mp as *mut libc::c_char)
            .offset(16 as libc::c_ulong as libc::c_uint as isize)
            .offset(
                ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_ulong)
                    .wrapping_mul((*key).mv_size) as isize,
            ) as *mut libc::c_void;
        return 0 as libc::c_int;
    }
    leaf = (mp as *mut libc::c_char)
        .offset(
            *((*mp).mp_ptrs)
                .as_mut_ptr()
                .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize) as libc::c_int
                as isize,
        )
        .offset(
            (if 0 as libc::c_int != 0 {
                16 as libc::c_ulong as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        ) as *mut MDB_node;
    if (*leaf).mn_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int {
        mdb_xcursor_init1(mc, leaf);
        rc = mdb_cursor_last(
            &mut (*(*mc).mc_xcursor).mx_cursor,
            data,
            0 as *mut MDB_val,
        );
        if rc != 0 as libc::c_int {
            return rc;
        }
    } else if !data.is_null() {
        rc = mdb_node_read(mc, leaf, data);
        if rc != 0 as libc::c_int {
            return rc;
        }
    }
    if !key.is_null() {
        (*key).mv_size = (*leaf).mn_ksize as size_t;
        (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_cursor_set(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut op: MDB_cursor_op,
    mut exactp: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut leaf: *mut MDB_node = 0 as *mut MDB_node;
    if (*key).mv_size == 0 as libc::c_int as libc::c_ulong {
        return -(30781 as libc::c_int);
    }
    if !((*mc).mc_xcursor).is_null() {
        (*(*mc).mc_xcursor).mx_cursor.mc_flags
            &= !(0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint;
    }
    if (*mc).mc_flags & 0x1 as libc::c_int as libc::c_uint != 0 {
        let mut nodekey: MDB_val = MDB_val {
            mv_size: 0,
            mv_data: 0 as *mut libc::c_void,
        };
        mp = (*mc).mc_pg[(*mc).mc_top as usize];
        if ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                (16 as libc::c_ulong as libc::c_uint)
                    .wrapping_sub(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }),
                    ),
            ) >> 1 as libc::c_int == 0
        {
            (*mc).mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
            return -(30798 as libc::c_int);
        }
        if (*mp).mp_flags as libc::c_int & 0x20 as libc::c_int != 0 {
            nodekey.mv_size = (*(*mc).mc_db).md_pad as size_t;
            nodekey
                .mv_data = (mp as *mut libc::c_char)
                .offset(16 as libc::c_ulong as libc::c_uint as isize)
                .offset(
                    (0 as libc::c_int as libc::c_ulong).wrapping_mul(nodekey.mv_size)
                        as isize,
                ) as *mut libc::c_void;
        } else {
            leaf = (mp as *mut libc::c_char)
                .offset(
                    *((*mp).mp_ptrs).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as libc::c_int as isize,
                )
                .offset(
                    (if 0 as libc::c_int != 0 {
                        16 as libc::c_ulong as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }) as isize,
                ) as *mut MDB_node;
            nodekey.mv_size = (*leaf).mn_ksize as size_t;
            nodekey.mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
        }
        rc = ((*(*mc).mc_dbx).md_cmp).unwrap()(key, &mut nodekey);
        if rc == 0 as libc::c_int {
            (*mc).mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
            if !exactp.is_null() {
                *exactp = 1 as libc::c_int;
            }
            current_block = 17739119538417469436;
        } else {
            if rc > 0 as libc::c_int {
                let mut i: libc::c_uint = 0;
                let mut nkeys: libc::c_uint = ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                    .wrapping_sub(
                        (16 as libc::c_ulong as libc::c_uint)
                            .wrapping_sub(
                                (if 0 as libc::c_int != 0 {
                                    16 as libc::c_ulong as libc::c_uint
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }),
                            ),
                    ) >> 1 as libc::c_int;
                if nkeys > 1 as libc::c_int as libc::c_uint {
                    if (*mp).mp_flags as libc::c_int & 0x20 as libc::c_int != 0 {
                        nodekey
                            .mv_data = (mp as *mut libc::c_char)
                            .offset(16 as libc::c_ulong as libc::c_uint as isize)
                            .offset(
                                (nkeys.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    as libc::c_ulong)
                                    .wrapping_mul(nodekey.mv_size) as isize,
                            ) as *mut libc::c_void;
                    } else {
                        leaf = (mp as *mut libc::c_char)
                            .offset(
                                *((*mp).mp_ptrs)
                                    .as_mut_ptr()
                                    .offset(
                                        nkeys.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                            as isize,
                                    ) as libc::c_int as isize,
                            )
                            .offset(
                                (if 0 as libc::c_int != 0 {
                                    16 as libc::c_ulong as libc::c_uint
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }) as isize,
                            ) as *mut MDB_node;
                        nodekey.mv_size = (*leaf).mn_ksize as size_t;
                        nodekey
                            .mv_data = ((*leaf).mn_data).as_mut_ptr()
                            as *mut libc::c_void;
                    }
                    rc = ((*(*mc).mc_dbx).md_cmp).unwrap()(key, &mut nodekey);
                    if rc == 0 as libc::c_int {
                        (*mc)
                            .mc_ki[(*mc).mc_top
                            as usize] = nkeys
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as indx_t;
                        if !exactp.is_null() {
                            *exactp = 1 as libc::c_int;
                        }
                        current_block = 17739119538417469436;
                    } else if rc < 0 as libc::c_int {
                        if ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint)
                            < ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                                .wrapping_sub(
                                    (16 as libc::c_ulong as libc::c_uint)
                                        .wrapping_sub(
                                            (if 0 as libc::c_int != 0 {
                                                16 as libc::c_ulong as libc::c_uint
                                            } else {
                                                0 as libc::c_int as libc::c_uint
                                            }),
                                        ),
                                ) >> 1 as libc::c_int
                        {
                            if (*mp).mp_flags as libc::c_int & 0x20 as libc::c_int != 0 {
                                nodekey
                                    .mv_data = (mp as *mut libc::c_char)
                                    .offset(16 as libc::c_ulong as libc::c_uint as isize)
                                    .offset(
                                        ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_ulong)
                                            .wrapping_mul(nodekey.mv_size) as isize,
                                    ) as *mut libc::c_void;
                            } else {
                                leaf = (mp as *mut libc::c_char)
                                    .offset(
                                        *((*mp).mp_ptrs)
                                            .as_mut_ptr()
                                            .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                                            as libc::c_int as isize,
                                    )
                                    .offset(
                                        (if 0 as libc::c_int != 0 {
                                            16 as libc::c_ulong as libc::c_uint
                                        } else {
                                            0 as libc::c_int as libc::c_uint
                                        }) as isize,
                                    ) as *mut MDB_node;
                                nodekey.mv_size = (*leaf).mn_ksize as size_t;
                                nodekey
                                    .mv_data = ((*leaf).mn_data).as_mut_ptr()
                                    as *mut libc::c_void;
                            }
                            rc = ((*(*mc).mc_dbx).md_cmp).unwrap()(key, &mut nodekey);
                            if rc == 0 as libc::c_int {
                                if !exactp.is_null() {
                                    *exactp = 1 as libc::c_int;
                                }
                                current_block = 17739119538417469436;
                            } else {
                                current_block = 3160140712158701372;
                            }
                        } else {
                            current_block = 3160140712158701372;
                        }
                        match current_block {
                            17739119538417469436 => {}
                            _ => {
                                rc = 0 as libc::c_int;
                                (*mc).mc_flags &= !(0x2 as libc::c_int) as libc::c_uint;
                                current_block = 1556262993922609740;
                            }
                        }
                    } else {
                        current_block = 13619784596304402172;
                    }
                } else {
                    current_block = 13619784596304402172;
                }
                match current_block {
                    1556262993922609740 => {}
                    17739119538417469436 => {}
                    _ => {
                        i = 0 as libc::c_int as libc::c_uint;
                        while i < (*mc).mc_top as libc::c_uint {
                            if ((*mc).mc_ki[i as usize] as libc::c_uint)
                                < (((*(*mc).mc_pg[i as usize]).mp_pb.pb.pb_lower
                                    as libc::c_uint)
                                    .wrapping_sub(
                                        (16 as libc::c_ulong as libc::c_uint)
                                            .wrapping_sub(
                                                (if 0 as libc::c_int != 0 {
                                                    16 as libc::c_ulong as libc::c_uint
                                                } else {
                                                    0 as libc::c_int as libc::c_uint
                                                }),
                                            ),
                                    ) >> 1 as libc::c_int)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            {
                                break;
                            }
                            i = i.wrapping_add(1);
                            i;
                        }
                        if i == (*mc).mc_top as libc::c_uint {
                            (*mc).mc_ki[(*mc).mc_top as usize] = nkeys as indx_t;
                            return -(30798 as libc::c_int);
                        }
                        current_block = 8347882395825654554;
                    }
                }
            } else {
                current_block = 8347882395825654554;
            }
            match current_block {
                17739119538417469436 => {}
                1556262993922609740 => {}
                _ => {
                    if (*mc).mc_top == 0 {
                        (*mc).mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
                        if op as libc::c_uint
                            == MDB_SET_RANGE as libc::c_int as libc::c_uint
                            && exactp.is_null()
                        {
                            rc = 0 as libc::c_int;
                        } else {
                            return -(30798 as libc::c_int)
                        }
                        current_block = 17739119538417469436;
                    } else {
                        current_block = 15594603006322722090;
                    }
                }
            }
        }
    } else {
        (*mc).mc_pg[0 as libc::c_int as usize] = 0 as *mut MDB_page;
        current_block = 15594603006322722090;
    }
    match current_block {
        15594603006322722090 => {
            rc = mdb_page_search(mc, key, 0 as libc::c_int);
            if rc != 0 as libc::c_int {
                return rc;
            }
            mp = (*mc).mc_pg[(*mc).mc_top as usize];
            if (*mp).mp_flags as libc::c_int & 0x2 as libc::c_int == 0x2 as libc::c_int
            {} else {
                mdb_assert_fail(
                    (*(*mc).mc_txn).mt_env,
                    b"IS_LEAF(mp)\0" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<
                        &[u8; 15],
                        &[libc::c_char; 15],
                    >(b"mdb_cursor_set\0"))
                        .as_ptr(),
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    7165 as libc::c_int,
                );
            };
            current_block = 1556262993922609740;
        }
        _ => {}
    }
    match current_block {
        1556262993922609740 => {
            leaf = mdb_node_search(mc, key, exactp);
            if !exactp.is_null() && *exactp == 0 {
                return -(30798 as libc::c_int);
            }
            if leaf.is_null() {
                rc = mdb_cursor_sibling(mc, 1 as libc::c_int);
                if rc != 0 as libc::c_int {
                    (*mc).mc_flags |= 0x2 as libc::c_int as libc::c_uint;
                    return rc;
                }
                mp = (*mc).mc_pg[(*mc).mc_top as usize];
                if (*mp).mp_flags as libc::c_int & 0x2 as libc::c_int
                    == 0x2 as libc::c_int
                {} else {
                    mdb_assert_fail(
                        (*(*mc).mc_txn).mt_env,
                        b"IS_LEAF(mp)\0" as *const u8 as *const libc::c_char,
                        (*::std::mem::transmute::<
                            &[u8; 15],
                            &[libc::c_char; 15],
                        >(b"mdb_cursor_set\0"))
                            .as_ptr(),
                        b"mdb.c\0" as *const u8 as *const libc::c_char,
                        7181 as libc::c_int,
                    );
                };
                leaf = (mp as *mut libc::c_char)
                    .offset(
                        *((*mp).mp_ptrs).as_mut_ptr().offset(0 as libc::c_int as isize)
                            as libc::c_int as isize,
                    )
                    .offset(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
            }
        }
        _ => {}
    }
    (*mc).mc_flags |= 0x1 as libc::c_int as libc::c_uint;
    (*mc).mc_flags &= !(0x2 as libc::c_int) as libc::c_uint;
    if (*mp).mp_flags as libc::c_int & 0x20 as libc::c_int == 0x20 as libc::c_int {
        if op as libc::c_uint == MDB_SET_RANGE as libc::c_int as libc::c_uint
            || op as libc::c_uint == MDB_SET_KEY as libc::c_int as libc::c_uint
        {
            (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
            (*key)
                .mv_data = (mp as *mut libc::c_char)
                .offset(16 as libc::c_ulong as libc::c_uint as isize)
                .offset(
                    ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_ulong)
                        .wrapping_mul((*key).mv_size) as isize,
                ) as *mut libc::c_void;
        }
        return 0 as libc::c_int;
    }
    if (*leaf).mn_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int {
        mdb_xcursor_init1(mc, leaf);
        if op as libc::c_uint == MDB_SET as libc::c_int as libc::c_uint
            || op as libc::c_uint == MDB_SET_KEY as libc::c_int as libc::c_uint
            || op as libc::c_uint == MDB_SET_RANGE as libc::c_int as libc::c_uint
        {
            rc = mdb_cursor_first(
                &mut (*(*mc).mc_xcursor).mx_cursor,
                data,
                0 as *mut MDB_val,
            );
        } else {
            let mut ex2: libc::c_int = 0;
            let mut ex2p: *mut libc::c_int = 0 as *mut libc::c_int;
            if op as libc::c_uint == MDB_GET_BOTH as libc::c_int as libc::c_uint {
                ex2p = &mut ex2;
                ex2 = 0 as libc::c_int;
            } else {
                ex2p = 0 as *mut libc::c_int;
            }
            rc = mdb_cursor_set(
                &mut (*(*mc).mc_xcursor).mx_cursor,
                data,
                0 as *mut MDB_val,
                MDB_SET_RANGE,
                ex2p,
            );
            if rc != 0 as libc::c_int {
                return rc;
            }
        }
    } else if !data.is_null() {
        if op as libc::c_uint == MDB_GET_BOTH as libc::c_int as libc::c_uint
            || op as libc::c_uint == MDB_GET_BOTH_RANGE as libc::c_int as libc::c_uint
        {
            let mut olddata: MDB_val = MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            };
            let mut dcmp: Option::<MDB_cmp_func> = None;
            rc = mdb_node_read(mc, leaf, &mut olddata);
            if rc != 0 as libc::c_int {
                return rc;
            }
            dcmp = (*(*mc).mc_dbx).md_dcmp;
            if ((2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint) as libc::c_ulong)
                < 18446744073709551615 as libc::c_ulong
                && dcmp == Some(mdb_cmp_int as MDB_cmp_func)
                && olddata.mv_size
                    == ::std::mem::size_of::<mdb_size_t>() as libc::c_ulong
            {
                dcmp = Some(mdb_cmp_long as MDB_cmp_func);
            }
            rc = dcmp.unwrap()(data, &mut olddata);
            if rc != 0 {
                if op as libc::c_uint == MDB_GET_BOTH as libc::c_int as libc::c_uint
                    || rc > 0 as libc::c_int
                {
                    return -(30798 as libc::c_int);
                }
                rc = 0 as libc::c_int;
            }
            *data = olddata;
        } else {
            if !((*mc).mc_xcursor).is_null() {
                (*(*mc).mc_xcursor).mx_cursor.mc_flags
                    &= !(0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint;
            }
            rc = mdb_node_read(mc, leaf, data);
            if rc != 0 as libc::c_int {
                return rc;
            }
        }
    }
    if op as libc::c_uint == MDB_SET_RANGE as libc::c_int as libc::c_uint
        || op as libc::c_uint == MDB_SET_KEY as libc::c_int as libc::c_uint
    {
        if !key.is_null() {
            (*key).mv_size = (*leaf).mn_ksize as size_t;
            (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
        }
    }
    return rc;
}
unsafe extern "C" fn mdb_cursor_first(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut leaf: *mut MDB_node = 0 as *mut MDB_node;
    if !((*mc).mc_xcursor).is_null() {
        (*(*mc).mc_xcursor).mx_cursor.mc_flags
            &= !(0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint;
    }
    if (*mc).mc_flags & 0x1 as libc::c_int as libc::c_uint == 0
        || (*mc).mc_top as libc::c_int != 0
    {
        rc = mdb_page_search(mc, 0 as *mut MDB_val, 4 as libc::c_int);
        if rc != 0 as libc::c_int {
            return rc;
        }
    }
    if (*(*mc).mc_pg[(*mc).mc_top as usize]).mp_flags as libc::c_int & 0x2 as libc::c_int
        == 0x2 as libc::c_int
    {} else {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"IS_LEAF(mc->mc_pg[mc->mc_top])\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"mdb_cursor_first\0"))
                .as_ptr(),
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            7263 as libc::c_int,
        );
    };
    leaf = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
        .offset(
            *((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_ptrs)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as libc::c_int as isize,
        )
        .offset(
            (if 0 as libc::c_int != 0 {
                16 as libc::c_ulong as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        ) as *mut MDB_node;
    (*mc).mc_flags |= 0x1 as libc::c_int as libc::c_uint;
    (*mc).mc_flags &= !(0x2 as libc::c_int) as libc::c_uint;
    (*mc).mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
    if (*(*mc).mc_pg[(*mc).mc_top as usize]).mp_flags as libc::c_int
        & 0x20 as libc::c_int == 0x20 as libc::c_int
    {
        if !key.is_null() {
            (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
            (*key)
                .mv_data = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
                .offset(16 as libc::c_ulong as libc::c_uint as isize)
                .offset(
                    (0 as libc::c_int as libc::c_ulong).wrapping_mul((*key).mv_size)
                        as isize,
                ) as *mut libc::c_void;
        }
        return 0 as libc::c_int;
    }
    if (*leaf).mn_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int {
        mdb_xcursor_init1(mc, leaf);
        rc = mdb_cursor_first(
            &mut (*(*mc).mc_xcursor).mx_cursor,
            data,
            0 as *mut MDB_val,
        );
        if rc != 0 {
            return rc;
        }
    } else if !data.is_null() {
        rc = mdb_node_read(mc, leaf, data);
        if rc != 0 as libc::c_int {
            return rc;
        }
    }
    if !key.is_null() {
        (*key).mv_size = (*leaf).mn_ksize as size_t;
        (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_cursor_last(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut leaf: *mut MDB_node = 0 as *mut MDB_node;
    if !((*mc).mc_xcursor).is_null() {
        (*(*mc).mc_xcursor).mx_cursor.mc_flags
            &= !(0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint;
    }
    if (*mc).mc_flags & 0x1 as libc::c_int as libc::c_uint == 0
        || (*mc).mc_top as libc::c_int != 0
    {
        rc = mdb_page_search(mc, 0 as *mut MDB_val, 8 as libc::c_int);
        if rc != 0 as libc::c_int {
            return rc;
        }
    }
    if (*(*mc).mc_pg[(*mc).mc_top as usize]).mp_flags as libc::c_int & 0x2 as libc::c_int
        == 0x2 as libc::c_int
    {} else {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"IS_LEAF(mc->mc_pg[mc->mc_top])\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"mdb_cursor_last\0"))
                .as_ptr(),
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            7310 as libc::c_int,
        );
    };
    (*mc)
        .mc_ki[(*mc).mc_top
        as usize] = (((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_lower
        as libc::c_uint)
        .wrapping_sub(
            (16 as libc::c_ulong as libc::c_uint)
                .wrapping_sub(
                    (if 0 as libc::c_int != 0 {
                        16 as libc::c_ulong as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                ),
        ) >> 1 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as indx_t;
    (*mc).mc_flags |= (0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint;
    leaf = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
        .offset(
            *((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_ptrs)
                .as_mut_ptr()
                .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize) as libc::c_int
                as isize,
        )
        .offset(
            (if 0 as libc::c_int != 0 {
                16 as libc::c_ulong as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        ) as *mut MDB_node;
    if (*(*mc).mc_pg[(*mc).mc_top as usize]).mp_flags as libc::c_int
        & 0x20 as libc::c_int == 0x20 as libc::c_int
    {
        if !key.is_null() {
            (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
            (*key)
                .mv_data = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
                .offset(16 as libc::c_ulong as libc::c_uint as isize)
                .offset(
                    ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_ulong)
                        .wrapping_mul((*key).mv_size) as isize,
                ) as *mut libc::c_void;
        }
        return 0 as libc::c_int;
    }
    if (*leaf).mn_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int {
        mdb_xcursor_init1(mc, leaf);
        rc = mdb_cursor_last(
            &mut (*(*mc).mc_xcursor).mx_cursor,
            data,
            0 as *mut MDB_val,
        );
        if rc != 0 {
            return rc;
        }
    } else if !data.is_null() {
        rc = mdb_node_read(mc, leaf, data);
        if rc != 0 as libc::c_int {
            return rc;
        }
    }
    if !key.is_null() {
        (*key).mv_size = (*leaf).mn_ksize as size_t;
        (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_cursor_get(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut op: MDB_cursor_op,
) -> libc::c_int {
    let mut mx: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut exact: libc::c_int = 0 as libc::c_int;
    let mut mfunc: Option::<
        unsafe extern "C" fn(*mut MDB_cursor, *mut MDB_val, *mut MDB_val) -> libc::c_int,
    > = None;
    if mc.is_null() {
        return 22 as libc::c_int;
    }
    if (*(*mc).mc_txn).mt_flags
        & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
        != 0
    {
        return -(30782 as libc::c_int);
    }
    match op as libc::c_uint {
        4 => {
            if (*mc).mc_flags & 0x1 as libc::c_int as libc::c_uint == 0 {
                rc = 22 as libc::c_int;
            } else {
                let mut mp: *mut MDB_page = (*mc).mc_pg[(*mc).mc_top as usize];
                let mut nkeys: libc::c_int = (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                    .wrapping_sub(
                        (16 as libc::c_ulong as libc::c_uint)
                            .wrapping_sub(
                                (if 0 as libc::c_int != 0 {
                                    16 as libc::c_ulong as libc::c_uint
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }),
                            ),
                    ) >> 1 as libc::c_int) as libc::c_int;
                if nkeys == 0
                    || (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int >= nkeys
                {
                    (*mc).mc_ki[(*mc).mc_top as usize] = nkeys as indx_t;
                    rc = -(30798 as libc::c_int);
                } else {
                    rc = 0 as libc::c_int;
                    if (*mp).mp_flags as libc::c_int & 0x20 as libc::c_int
                        == 0x20 as libc::c_int
                    {
                        (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
                        (*key)
                            .mv_data = (mp as *mut libc::c_char)
                            .offset(16 as libc::c_ulong as libc::c_uint as isize)
                            .offset(
                                ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_ulong)
                                    .wrapping_mul((*key).mv_size) as isize,
                            ) as *mut libc::c_void;
                    } else {
                        let mut leaf: *mut MDB_node = (mp as *mut libc::c_char)
                            .offset(
                                *((*mp).mp_ptrs)
                                    .as_mut_ptr()
                                    .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                                    as libc::c_int as isize,
                            )
                            .offset(
                                (if 0 as libc::c_int != 0 {
                                    16 as libc::c_ulong as libc::c_uint
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }) as isize,
                            ) as *mut MDB_node;
                        if !key.is_null() {
                            (*key).mv_size = (*leaf).mn_ksize as size_t;
                            (*key)
                                .mv_data = ((*leaf).mn_data).as_mut_ptr()
                                as *mut libc::c_void;
                        }
                        if !data.is_null() {
                            if (*leaf).mn_flags as libc::c_int & 0x4 as libc::c_int
                                == 0x4 as libc::c_int
                            {
                                rc = mdb_cursor_get(
                                    &mut (*(*mc).mc_xcursor).mx_cursor,
                                    data,
                                    0 as *mut MDB_val,
                                    MDB_GET_CURRENT,
                                );
                            } else {
                                rc = mdb_node_read(mc, leaf, data);
                            }
                        }
                    }
                }
            }
            current_block = 1765866445182206997;
        }
        2 | 3 => {
            if data.is_null() {
                rc = 22 as libc::c_int;
                current_block = 1765866445182206997;
            } else if ((*mc).mc_xcursor).is_null() {
                rc = -(30784 as libc::c_int);
                current_block = 1765866445182206997;
            } else {
                current_block = 2829306932918634116;
            }
        }
        15 | 16 | 17 => {
            current_block = 2829306932918634116;
        }
        5 => {
            if data.is_null() || (*mc).mc_flags & 0x1 as libc::c_int as libc::c_uint == 0
            {
                rc = 22 as libc::c_int;
                current_block = 1765866445182206997;
            } else if (*(*mc).mc_db).md_flags as libc::c_int & 0x10 as libc::c_int == 0 {
                rc = -(30784 as libc::c_int);
                current_block = 1765866445182206997;
            } else {
                rc = 0 as libc::c_int;
                if (*(*mc).mc_xcursor).mx_cursor.mc_flags
                    & 0x1 as libc::c_int as libc::c_uint == 0
                    || (*(*mc).mc_xcursor).mx_cursor.mc_flags
                        & 0x2 as libc::c_int as libc::c_uint != 0
                {
                    current_block = 1765866445182206997;
                } else {
                    current_block = 8084019061100922670;
                }
            }
        }
        10 => {
            if data.is_null() {
                rc = 22 as libc::c_int;
                current_block = 1765866445182206997;
            } else if (*(*mc).mc_db).md_flags as libc::c_int & 0x10 as libc::c_int == 0 {
                rc = -(30784 as libc::c_int);
                current_block = 1765866445182206997;
            } else {
                rc = mdb_cursor_next(mc, key, data, MDB_NEXT_DUP);
                if rc == 0 as libc::c_int {
                    if (*(*mc).mc_xcursor).mx_cursor.mc_flags
                        & 0x1 as libc::c_int as libc::c_uint != 0
                    {
                        mx = 0 as *mut MDB_cursor;
                        current_block = 8084019061100922670;
                    } else {
                        rc = -(30798 as libc::c_int);
                        current_block = 1765866445182206997;
                    }
                } else {
                    current_block = 1765866445182206997;
                }
            }
        }
        18 => {
            if data.is_null() {
                rc = 22 as libc::c_int;
                current_block = 1765866445182206997;
            } else if (*(*mc).mc_db).md_flags as libc::c_int & 0x10 as libc::c_int == 0 {
                rc = -(30784 as libc::c_int);
                current_block = 1765866445182206997;
            } else {
                if (*mc).mc_flags & 0x1 as libc::c_int as libc::c_uint == 0 {
                    rc = mdb_cursor_last(mc, key, data);
                } else {
                    rc = 0 as libc::c_int;
                }
                if rc == 0 as libc::c_int {
                    let mut mx_0: *mut MDB_cursor = &mut (*(*mc).mc_xcursor).mx_cursor;
                    if (*mx_0).mc_flags & 0x1 as libc::c_int as libc::c_uint != 0 {
                        rc = mdb_cursor_sibling(mx_0, 0 as libc::c_int);
                        if rc == 0 as libc::c_int {
                            current_block = 8084019061100922670;
                        } else {
                            current_block = 1765866445182206997;
                        }
                    } else {
                        rc = -(30798 as libc::c_int);
                        current_block = 1765866445182206997;
                    }
                } else {
                    current_block = 1765866445182206997;
                }
            }
        }
        8 | 9 | 11 => {
            rc = mdb_cursor_next(mc, key, data, op);
            current_block = 1765866445182206997;
        }
        12 | 13 | 14 => {
            rc = mdb_cursor_prev(mc, key, data, op);
            current_block = 1765866445182206997;
        }
        0 => {
            rc = mdb_cursor_first(mc, key, data);
            current_block = 1765866445182206997;
        }
        1 => {
            mfunc = Some(
                mdb_cursor_first
                    as unsafe extern "C" fn(
                        *mut MDB_cursor,
                        *mut MDB_val,
                        *mut MDB_val,
                    ) -> libc::c_int,
            );
            current_block = 16343666387556838955;
        }
        6 => {
            rc = mdb_cursor_last(mc, key, data);
            current_block = 1765866445182206997;
        }
        7 => {
            mfunc = Some(
                mdb_cursor_last
                    as unsafe extern "C" fn(
                        *mut MDB_cursor,
                        *mut MDB_val,
                        *mut MDB_val,
                    ) -> libc::c_int,
            );
            current_block = 16343666387556838955;
        }
        _ => {
            rc = 22 as libc::c_int;
            current_block = 1765866445182206997;
        }
    }
    match current_block {
        16343666387556838955 => {
            if data.is_null() || (*mc).mc_flags & 0x1 as libc::c_int as libc::c_uint == 0
            {
                rc = 22 as libc::c_int;
            } else if ((*mc).mc_xcursor).is_null() {
                rc = -(30784 as libc::c_int);
            } else if (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint
                >= ((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_lower
                    as libc::c_uint)
                    .wrapping_sub(
                        (16 as libc::c_ulong as libc::c_uint)
                            .wrapping_sub(
                                (if 0 as libc::c_int != 0 {
                                    16 as libc::c_ulong as libc::c_uint
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }),
                            ),
                    ) >> 1 as libc::c_int
            {
                (*mc)
                    .mc_ki[(*mc).mc_top
                    as usize] = (((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_lower
                    as libc::c_uint)
                    .wrapping_sub(
                        (16 as libc::c_ulong as libc::c_uint)
                            .wrapping_sub(
                                (if 0 as libc::c_int != 0 {
                                    16 as libc::c_ulong as libc::c_uint
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }),
                            ),
                    ) >> 1 as libc::c_int) as indx_t;
                rc = -(30798 as libc::c_int);
            } else {
                (*mc).mc_flags &= !(0x2 as libc::c_int) as libc::c_uint;
                let mut leaf_0: *mut MDB_node = ((*mc).mc_pg[(*mc).mc_top as usize]
                    as *mut libc::c_char)
                    .offset(
                        *((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_ptrs)
                            .as_mut_ptr()
                            .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                            as libc::c_int as isize,
                    )
                    .offset(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                if !((*leaf_0).mn_flags as libc::c_int & 0x4 as libc::c_int
                    == 0x4 as libc::c_int)
                {
                    if !key.is_null() {
                        (*key).mv_size = (*leaf_0).mn_ksize as size_t;
                        (*key)
                            .mv_data = ((*leaf_0).mn_data).as_mut_ptr()
                            as *mut libc::c_void;
                    }
                    rc = mdb_node_read(mc, leaf_0, data);
                } else if (*(*mc).mc_xcursor).mx_cursor.mc_flags
                    & 0x1 as libc::c_int as libc::c_uint == 0
                {
                    rc = 22 as libc::c_int;
                } else {
                    rc = mfunc
                        .unwrap()(
                        &mut (*(*mc).mc_xcursor).mx_cursor,
                        data,
                        0 as *mut MDB_val,
                    );
                }
            }
        }
        8084019061100922670 => {
            mx = &mut (*(*mc).mc_xcursor).mx_cursor;
            (*data)
                .mv_size = (((*(*mx).mc_pg[(*mx).mc_top as usize]).mp_pb.pb.pb_lower
                as libc::c_uint)
                .wrapping_sub(
                    (16 as libc::c_ulong as libc::c_uint)
                        .wrapping_sub(
                            (if 0 as libc::c_int != 0 {
                                16 as libc::c_ulong as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }),
                        ),
                ) >> 1 as libc::c_int)
                .wrapping_mul((*(*mx).mc_db).md_pad) as size_t;
            (*data)
                .mv_data = ((*mx).mc_pg[(*mx).mc_top as usize] as *mut libc::c_char)
                .offset(16 as libc::c_ulong as libc::c_uint as isize)
                as *mut libc::c_void;
            (*mx)
                .mc_ki[(*mx).mc_top
                as usize] = (((*(*mx).mc_pg[(*mx).mc_top as usize]).mp_pb.pb.pb_lower
                as libc::c_uint)
                .wrapping_sub(
                    (16 as libc::c_ulong as libc::c_uint)
                        .wrapping_sub(
                            (if 0 as libc::c_int != 0 {
                                16 as libc::c_ulong as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }),
                        ),
                ) >> 1 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as indx_t;
        }
        2829306932918634116 => {
            if key.is_null() {
                rc = 22 as libc::c_int;
            } else {
                rc = mdb_cursor_set(
                    mc,
                    key,
                    data,
                    op,
                    if op as libc::c_uint == MDB_SET_RANGE as libc::c_int as libc::c_uint
                    {
                        0 as *mut libc::c_int
                    } else {
                        &mut exact
                    },
                );
            }
        }
        _ => {}
    }
    if (*mc).mc_flags & 0x8 as libc::c_int as libc::c_uint != 0 {
        (*mc).mc_flags ^= 0x8 as libc::c_int as libc::c_uint;
    }
    return rc;
}
unsafe extern "C" fn mdb_cursor_touch(mut mc: *mut MDB_cursor) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    if (*mc).mc_dbi >= 2 as libc::c_int as libc::c_uint
        && *(*mc).mc_dbflag as libc::c_int & (0x1 as libc::c_int | 0x20 as libc::c_int)
            == 0
    {
        let mut mc2: MDB_cursor = MDB_cursor {
            mc_next: 0 as *mut MDB_cursor,
            mc_backup: 0 as *mut MDB_cursor,
            mc_xcursor: 0 as *mut MDB_xcursor,
            mc_txn: 0 as *mut MDB_txn,
            mc_dbi: 0,
            mc_db: 0 as *mut MDB_db,
            mc_dbx: 0 as *mut MDB_dbx,
            mc_dbflag: 0 as *mut libc::c_uchar,
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [0 as *mut MDB_page; 32],
            mc_ki: [0; 32],
        };
        let mut mcx: MDB_xcursor = MDB_xcursor {
            mx_cursor: MDB_cursor {
                mc_next: 0 as *mut MDB_cursor,
                mc_backup: 0 as *mut MDB_cursor,
                mc_xcursor: 0 as *mut MDB_xcursor,
                mc_txn: 0 as *mut MDB_txn,
                mc_dbi: 0,
                mc_db: 0 as *mut MDB_db,
                mc_dbx: 0 as *mut MDB_dbx,
                mc_dbflag: 0 as *mut libc::c_uchar,
                mc_snum: 0,
                mc_top: 0,
                mc_flags: 0,
                mc_pg: [0 as *mut MDB_page; 32],
                mc_ki: [0; 32],
            },
            mx_db: MDB_db {
                md_pad: 0,
                md_flags: 0,
                md_depth: 0,
                md_branch_pages: 0,
                md_leaf_pages: 0,
                md_overflow_pages: 0,
                md_entries: 0,
                md_root: 0,
            },
            mx_dbx: MDB_dbx {
                md_name: MDB_val {
                    mv_size: 0,
                    mv_data: 0 as *mut libc::c_void,
                },
                md_cmp: None,
                md_dcmp: None,
                md_rel: None,
                md_relctx: 0 as *mut libc::c_void,
            },
            mx_dbflag: 0,
        };
        if *((*(*mc).mc_txn).mt_dbiseqs).offset((*mc).mc_dbi as isize)
            != *((*(*(*mc).mc_txn).mt_env).me_dbiseqs).offset((*mc).mc_dbi as isize)
        {
            return -(30780 as libc::c_int);
        }
        mdb_cursor_init(&mut mc2, (*mc).mc_txn, 1 as libc::c_int as MDB_dbi, &mut mcx);
        rc = mdb_page_search(&mut mc2, &mut (*(*mc).mc_dbx).md_name, 1 as libc::c_int);
        if rc != 0 {
            return rc;
        }
        *(*mc)
            .mc_dbflag = (*(*mc).mc_dbflag as libc::c_int | 0x1 as libc::c_int)
            as libc::c_uchar;
    }
    (*mc).mc_top = 0 as libc::c_int as libc::c_ushort;
    if (*mc).mc_snum != 0 {
        loop {
            rc = mdb_page_touch(mc);
            if !(rc == 0
                && {
                    (*mc).mc_top = ((*mc).mc_top).wrapping_add(1);
                    ((*mc).mc_top as libc::c_int) < (*mc).mc_snum as libc::c_int
                })
            {
                break;
            }
        }
        (*mc)
            .mc_top = ((*mc).mc_snum as libc::c_int - 1 as libc::c_int)
            as libc::c_ushort;
    }
    return rc;
}
pub unsafe extern "C" fn mdb_cursor_put(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut offset: libc::c_uint = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ksize: libc::c_uint = 0;
    let mut xflags: libc::c_int = 0;
    let mut new_dupdata: libc::c_int = 0;
    let mut ecount: mdb_size_t = 0;
    let mut current_block: u64;
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut leaf: *mut MDB_node = 0 as *mut MDB_node;
    let mut fp: *mut MDB_page = 0 as *mut MDB_page;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut sub_root: *mut MDB_page = 0 as *mut MDB_page;
    let mut fp_flags: uint16_t = 0;
    let mut xdata: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut rdata: *mut MDB_val = 0 as *mut MDB_val;
    let mut dkey: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut olddata: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut dummy: MDB_db = MDB_db {
        md_pad: 0,
        md_flags: 0,
        md_depth: 0,
        md_branch_pages: 0,
        md_leaf_pages: 0,
        md_overflow_pages: 0,
        md_entries: 0,
        md_root: 0,
    };
    let mut do_sub: libc::c_int = 0 as libc::c_int;
    let mut insert_key: libc::c_int = 0;
    let mut insert_data: libc::c_int = 0;
    let mut mcount: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut dcount: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut nospill: libc::c_uint = 0;
    let mut nsize: size_t = 0;
    let mut rc: libc::c_int = 0;
    let mut rc2: libc::c_int = 0;
    let mut nflags: libc::c_uint = 0;
    if mc.is_null() || key.is_null() {
        return 22 as libc::c_int;
    }
    env = (*(*mc).mc_txn).mt_env;
    if flags & 0x80000 as libc::c_int as libc::c_uint != 0 {
        dcount = (*data.offset(1 as libc::c_int as isize)).mv_size as libc::c_uint;
        (*data.offset(1 as libc::c_int as isize)).mv_size = 0 as libc::c_int as size_t;
        if !((*(*mc).mc_db).md_flags as libc::c_int & 0x10 as libc::c_int
            == 0x10 as libc::c_int)
        {
            return -(30784 as libc::c_int);
        }
    }
    nospill = flags & 0x8000 as libc::c_int as libc::c_uint;
    flags &= !(0x8000 as libc::c_int) as libc::c_uint;
    if (*(*mc).mc_txn).mt_flags
        & (0x20000 as libc::c_int
            | (0x1 as libc::c_int | 0x2 as libc::c_int | 0x10 as libc::c_int))
            as libc::c_uint != 0
    {
        return if (*(*mc).mc_txn).mt_flags & 0x20000 as libc::c_int as libc::c_uint != 0
        {
            13 as libc::c_int
        } else {
            -(30782 as libc::c_int)
        };
    }
    if ((*key).mv_size).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        >= (if 0 as libc::c_int != 0 { 0 as libc::c_int } else { 511 as libc::c_int })
            as libc::c_ulong
    {
        return -(30781 as libc::c_int);
    }
    if (*data).mv_size
        > (if (*(*mc).mc_db).md_flags as libc::c_int & 0x4 as libc::c_int != 0 {
            (if 0 as libc::c_int != 0 { 0 as libc::c_int } else { 511 as libc::c_int })
                as libc::c_ulong
        } else {
            0xffffffff as libc::c_ulong
        })
    {
        return -(30781 as libc::c_int);
    }
    dkey.mv_size = 0 as libc::c_int as size_t;
    if flags & 0x40 as libc::c_int as libc::c_uint != 0 {
        if (*mc).mc_flags & 0x1 as libc::c_int as libc::c_uint == 0 {
            return 22 as libc::c_int;
        }
        rc = 0 as libc::c_int;
    } else if (*(*mc).mc_db).md_root == !(0 as libc::c_int as pgno_t) {
        (*mc).mc_snum = 0 as libc::c_int as libc::c_ushort;
        (*mc).mc_top = 0 as libc::c_int as libc::c_ushort;
        (*mc).mc_flags &= !(0x1 as libc::c_int) as libc::c_uint;
        rc = -(30779 as libc::c_int) + 10 as libc::c_int;
    } else {
        let mut exact: libc::c_int = 0 as libc::c_int;
        let mut d2: MDB_val = MDB_val {
            mv_size: 0,
            mv_data: 0 as *mut libc::c_void,
        };
        if flags & 0x20000 as libc::c_int as libc::c_uint != 0 {
            let mut k2: MDB_val = MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            };
            rc = mdb_cursor_last(mc, &mut k2, &mut d2);
            if rc == 0 as libc::c_int {
                rc = ((*(*mc).mc_dbx).md_cmp).unwrap()(key, &mut k2);
                if rc > 0 as libc::c_int {
                    rc = -(30798 as libc::c_int);
                    (*mc)
                        .mc_ki[(*mc).mc_top
                        as usize] = ((*mc).mc_ki[(*mc).mc_top as usize]).wrapping_add(1);
                    (*mc).mc_ki[(*mc).mc_top as usize];
                } else {
                    rc = -(30799 as libc::c_int);
                }
            }
        } else {
            rc = mdb_cursor_set(mc, key, &mut d2, MDB_SET, &mut exact);
        }
        if flags & 0x10 as libc::c_int as libc::c_uint != 0 && rc == 0 as libc::c_int {
            *data = d2;
            return -(30799 as libc::c_int);
        }
        if rc != 0 && rc != -(30798 as libc::c_int) {
            return rc;
        }
    }
    if (*mc).mc_flags & 0x8 as libc::c_int as libc::c_uint != 0 {
        (*mc).mc_flags ^= 0x8 as libc::c_int as libc::c_uint;
    }
    if nospill == 0 {
        if flags & 0x80000 as libc::c_int as libc::c_uint != 0 {
            rdata = &mut xdata;
            xdata.mv_size = ((*data).mv_size).wrapping_mul(dcount as libc::c_ulong);
        } else {
            rdata = data;
        }
        rc2 = mdb_page_spill(mc, key, rdata);
        if rc2 != 0 {
            return rc2;
        }
    }
    if rc == -(30779 as libc::c_int) + 10 as libc::c_int {
        let mut np: *mut MDB_page = 0 as *mut MDB_page;
        rc2 = mdb_page_new(
            mc,
            0x2 as libc::c_int as uint32_t,
            1 as libc::c_int,
            &mut np,
        );
        if rc2 != 0 {
            return rc2;
        }
        mdb_cursor_push(mc, np);
        (*(*mc).mc_db).md_root = (*np).mp_p.p_pgno;
        (*(*mc).mc_db).md_depth = ((*(*mc).mc_db).md_depth).wrapping_add(1);
        (*(*mc).mc_db).md_depth;
        *(*mc)
            .mc_dbflag = (*(*mc).mc_dbflag as libc::c_int | 0x1 as libc::c_int)
            as libc::c_uchar;
        if (*(*mc).mc_db).md_flags as libc::c_int
            & (0x4 as libc::c_int | 0x10 as libc::c_int) == 0x10 as libc::c_int
        {
            (*np)
                .mp_flags = ((*np).mp_flags as libc::c_int | 0x20 as libc::c_int)
                as uint16_t;
        }
        (*mc).mc_flags |= 0x1 as libc::c_int as libc::c_uint;
    } else {
        rc2 = mdb_cursor_touch(mc);
        if rc2 != 0 {
            return rc2;
        }
    }
    insert_data = rc;
    insert_key = insert_data;
    if insert_key != 0 {
        if (*(*mc).mc_db).md_flags as libc::c_int & 0x4 as libc::c_int != 0
            && (8 as libc::c_ulong)
                .wrapping_add((*key).mv_size)
                .wrapping_add((*data).mv_size) > (*env).me_nodemax as libc::c_ulong
        {
            fp_flags = (0x2 as libc::c_int | 0x10 as libc::c_int) as uint16_t;
            fp = (*env).me_pbuf as *mut MDB_page;
            (*fp).mp_pad = (*data).mv_size as uint16_t;
            (*fp)
                .mp_pb
                .pb
                .pb_upper = (16 as libc::c_ulong as libc::c_uint)
                .wrapping_sub(
                    (if 0 as libc::c_int != 0 {
                        16 as libc::c_ulong as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                ) as indx_t;
            (*fp).mp_pb.pb.pb_lower = (*fp).mp_pb.pb.pb_upper;
            olddata.mv_size = 16 as libc::c_ulong as libc::c_uint as size_t;
            current_block = 17424024178509782500;
        } else {
            current_block = 2661260690706066674;
        }
    } else if (*(*mc).mc_pg[(*mc).mc_top as usize]).mp_flags as libc::c_int
        & 0x20 as libc::c_int == 0x20 as libc::c_int
    {
        ptr = 0 as *mut libc::c_char;
        ksize = (*(*mc).mc_db).md_pad;
        if (*key).mv_size != ksize as libc::c_ulong {
            return -(30781 as libc::c_int);
        }
        ptr = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
            .offset(16 as libc::c_ulong as libc::c_uint as isize)
            .offset(
                ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint).wrapping_mul(ksize)
                    as isize,
            );
        memcpy(ptr as *mut libc::c_void, (*key).mv_data, ksize as libc::c_ulong);
        current_block = 5300594046240281167;
    } else {
        current_block = 14635294375004343297;
    }
    loop {
        match current_block {
            2661260690706066674 => {
                rdata = data;
                current_block = 11632160926666140969;
            }
            17424024178509782500 => {
                if (*(*mc).mc_db).md_flags as libc::c_int & 0x10 as libc::c_int != 0 {
                    fp_flags = (fp_flags as libc::c_int | 0x20 as libc::c_int)
                        as uint16_t;
                    dummy.md_pad = (*fp).mp_pad as uint32_t;
                    dummy.md_flags = 0x10 as libc::c_int as uint16_t;
                    if (*(*mc).mc_db).md_flags as libc::c_int & 0x20 as libc::c_int != 0
                    {
                        dummy
                            .md_flags = (dummy.md_flags as libc::c_int
                            | 0x8 as libc::c_int) as uint16_t;
                    }
                } else {
                    dummy.md_pad = 0 as libc::c_int as uint32_t;
                    dummy.md_flags = 0 as libc::c_int as uint16_t;
                }
                dummy.md_depth = 1 as libc::c_int as uint16_t;
                dummy.md_branch_pages = 0 as libc::c_int as pgno_t;
                dummy.md_leaf_pages = 1 as libc::c_int as pgno_t;
                dummy.md_overflow_pages = 0 as libc::c_int as pgno_t;
                dummy
                    .md_entries = (((*fp).mp_pb.pb.pb_lower as libc::c_uint)
                    .wrapping_sub(
                        (16 as libc::c_ulong as libc::c_uint)
                            .wrapping_sub(
                                (if 0 as libc::c_int != 0 {
                                    16 as libc::c_ulong as libc::c_uint
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }),
                            ),
                    ) >> 1 as libc::c_int) as mdb_size_t;
                xdata.mv_size = ::std::mem::size_of::<MDB_db>() as libc::c_ulong;
                xdata.mv_data = &mut dummy as *mut MDB_db as *mut libc::c_void;
                rc = mdb_page_alloc(mc, 1 as libc::c_int, &mut mp);
                if rc != 0 {
                    return rc;
                }
                offset = ((*env).me_psize as libc::c_ulong).wrapping_sub(olddata.mv_size)
                    as libc::c_uint;
                flags |= (0x4 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint;
                dummy.md_root = (*mp).mp_p.p_pgno;
                sub_root = mp;
                current_block = 7315983924538012637;
            }
            5300594046240281167 => {
                if (*mc).mc_top as libc::c_int != 0
                    && (*mc).mc_ki[(*mc).mc_top as usize] == 0
                {
                    let mut dtop: libc::c_ushort = 1 as libc::c_int as libc::c_ushort;
                    (*mc).mc_top = ((*mc).mc_top).wrapping_sub(1);
                    (*mc).mc_top;
                    while (*mc).mc_top as libc::c_int != 0
                        && (*mc).mc_ki[(*mc).mc_top as usize] == 0
                    {
                        (*mc).mc_top = ((*mc).mc_top).wrapping_sub(1);
                        (*mc).mc_top;
                        dtop = dtop.wrapping_add(1);
                        dtop;
                    }
                    if (*mc).mc_ki[(*mc).mc_top as usize] != 0 {
                        rc2 = mdb_update_key(mc, key);
                    } else {
                        rc2 = 0 as libc::c_int;
                    }
                    (*mc)
                        .mc_top = ((*mc).mc_top as libc::c_int + dtop as libc::c_int)
                        as libc::c_ushort;
                    if rc2 != 0 {
                        return rc2;
                    }
                }
                return 0 as libc::c_int;
            }
            _ => {
                leaf = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
                    .offset(
                        *((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_ptrs)
                            .as_mut_ptr()
                            .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                            as libc::c_int as isize,
                    )
                    .offset(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                olddata
                    .mv_size = ((*leaf).mn_lo as libc::c_uint
                    | ((*leaf).mn_hi as libc::c_uint) << 16 as libc::c_int) as size_t;
                olddata
                    .mv_data = ((*leaf).mn_data)
                    .as_mut_ptr()
                    .offset((*leaf).mn_ksize as libc::c_int as isize)
                    as *mut libc::c_void;
                if (*(*mc).mc_db).md_flags as libc::c_int & 0x4 as libc::c_int
                    == 0x4 as libc::c_int
                {
                    i = 0;
                    offset = 0 as libc::c_int as libc::c_uint;
                    xdata.mv_data = (*env).me_pbuf;
                    fp = xdata.mv_data as *mut MDB_page;
                    mp = fp;
                    (*mp)
                        .mp_p
                        .p_pgno = (*(*mc).mc_pg[(*mc).mc_top as usize]).mp_p.p_pgno;
                    if !((*leaf).mn_flags as libc::c_int & 0x4 as libc::c_int
                        == 0x4 as libc::c_int)
                    {
                        let mut dcmp: Option::<MDB_cmp_func> = None;
                        if flags == 0x40 as libc::c_int as libc::c_uint {
                            current_block = 16896160091404590989;
                        } else {
                            dcmp = (*(*mc).mc_dbx).md_dcmp;
                            if ((2147483647 as libc::c_int as libc::c_uint)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_add(1 as libc::c_uint) as libc::c_ulong)
                                < 18446744073709551615 as libc::c_ulong
                                && dcmp == Some(mdb_cmp_int as MDB_cmp_func)
                                && olddata.mv_size
                                    == ::std::mem::size_of::<mdb_size_t>() as libc::c_ulong
                            {
                                dcmp = Some(mdb_cmp_long as MDB_cmp_func);
                            }
                            if dcmp.unwrap()(data, &mut olddata) == 0 {
                                if flags
                                    & (0x20 as libc::c_int | 0x40000 as libc::c_int)
                                        as libc::c_uint != 0
                                {
                                    return -(30799 as libc::c_int);
                                }
                                current_block = 16896160091404590989;
                            } else {
                                dkey.mv_size = olddata.mv_size;
                                dkey
                                    .mv_data = memcpy(
                                    fp.offset(1 as libc::c_int as isize) as *mut libc::c_void,
                                    olddata.mv_data,
                                    olddata.mv_size,
                                );
                                (*fp)
                                    .mp_flags = (0x2 as libc::c_int | 0x10 as libc::c_int
                                    | 0x40 as libc::c_int) as uint16_t;
                                (*fp)
                                    .mp_pb
                                    .pb
                                    .pb_lower = (16 as libc::c_ulong as libc::c_uint)
                                    .wrapping_sub(
                                        (if 0 as libc::c_int != 0 {
                                            16 as libc::c_ulong as libc::c_uint
                                        } else {
                                            0 as libc::c_int as libc::c_uint
                                        }),
                                    ) as indx_t;
                                xdata
                                    .mv_size = (16 as libc::c_ulong as libc::c_uint
                                    as libc::c_ulong)
                                    .wrapping_add(dkey.mv_size)
                                    .wrapping_add((*data).mv_size);
                                if (*(*mc).mc_db).md_flags as libc::c_int
                                    & 0x10 as libc::c_int != 0
                                {
                                    (*fp)
                                        .mp_flags = ((*fp).mp_flags as libc::c_int
                                        | 0x20 as libc::c_int) as uint16_t;
                                    (*fp).mp_pad = (*data).mv_size as uint16_t;
                                    xdata
                                        .mv_size = (xdata.mv_size as libc::c_ulong)
                                        .wrapping_add(
                                            (2 as libc::c_int as libc::c_ulong)
                                                .wrapping_mul((*data).mv_size),
                                        ) as size_t as size_t;
                                } else {
                                    xdata
                                        .mv_size = (xdata.mv_size as libc::c_ulong)
                                        .wrapping_add(
                                            (2 as libc::c_int as libc::c_ulong)
                                                .wrapping_mul(
                                                    (::std::mem::size_of::<indx_t>() as libc::c_ulong)
                                                        .wrapping_add(8 as libc::c_ulong),
                                                )
                                                .wrapping_add(
                                                    dkey.mv_size & 1 as libc::c_int as libc::c_ulong,
                                                )
                                                .wrapping_add(
                                                    (*data).mv_size & 1 as libc::c_int as libc::c_ulong,
                                                ),
                                        ) as size_t as size_t;
                                }
                                (*fp)
                                    .mp_pb
                                    .pb
                                    .pb_upper = (xdata.mv_size)
                                    .wrapping_sub(
                                        (if 0 as libc::c_int != 0 {
                                            16 as libc::c_ulong as libc::c_uint
                                        } else {
                                            0 as libc::c_int as libc::c_uint
                                        }) as libc::c_ulong,
                                    ) as indx_t;
                                olddata.mv_size = xdata.mv_size;
                                current_block = 1930794479672247912;
                            }
                        }
                    } else if (*leaf).mn_flags as libc::c_int & 0x2 as libc::c_int != 0 {
                        flags
                            |= (0x4 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint;
                        current_block = 4091900800743210317;
                    } else {
                        fp = olddata.mv_data as *mut MDB_page;
                        match flags {
                            64 => {
                                current_block = 96122713595493893;
                            }
                            _ => {
                                if (*(*mc).mc_db).md_flags as libc::c_int
                                    & 0x10 as libc::c_int == 0
                                {
                                    offset = ((8 as libc::c_ulong)
                                        .wrapping_add(
                                            ::std::mem::size_of::<indx_t>() as libc::c_ulong,
                                        )
                                        .wrapping_add((*data).mv_size)
                                        .wrapping_add(1 as libc::c_uint as libc::c_ulong)
                                        & -(2 as libc::c_int) as libc::c_ulong) as libc::c_uint;
                                    current_block = 11162283542402356847;
                                } else {
                                    offset = (*fp).mp_pad as libc::c_uint;
                                    if (((*fp).mp_pb.pb.pb_upper as libc::c_int
                                        - (*fp).mp_pb.pb.pb_lower as libc::c_int) as indx_t
                                        as libc::c_uint) < offset
                                    {
                                        offset = offset
                                            .wrapping_mul(4 as libc::c_int as libc::c_uint);
                                        current_block = 11162283542402356847;
                                    } else {
                                        current_block = 96122713595493893;
                                    }
                                }
                                match current_block {
                                    96122713595493893 => {}
                                    _ => {
                                        xdata
                                            .mv_size = (olddata.mv_size)
                                            .wrapping_add(offset as libc::c_ulong);
                                        current_block = 1930794479672247912;
                                    }
                                }
                            }
                        }
                        match current_block {
                            1930794479672247912 => {}
                            _ => {
                                (*fp)
                                    .mp_flags = ((*fp).mp_flags as libc::c_int
                                    | 0x10 as libc::c_int) as uint16_t;
                                (*fp).mp_p.p_pgno = (*mp).mp_p.p_pgno;
                                (*(*mc).mc_xcursor)
                                    .mx_cursor
                                    .mc_pg[0 as libc::c_int as usize] = fp;
                                flags |= 0x4 as libc::c_int as libc::c_uint;
                                current_block = 4091900800743210317;
                            }
                        }
                    }
                    match current_block {
                        4091900800743210317 => {}
                        16896160091404590989 => {}
                        _ => {
                            fp_flags = (*fp).mp_flags;
                            if (8 as libc::c_ulong)
                                .wrapping_add((*leaf).mn_ksize as libc::c_ulong)
                                .wrapping_add(xdata.mv_size)
                                > (*env).me_nodemax as libc::c_ulong
                            {
                                fp_flags = (fp_flags as libc::c_int
                                    & !(0x40 as libc::c_int)) as uint16_t;
                                current_block = 17424024178509782500;
                                continue;
                            } else {
                                current_block = 7315983924538012637;
                            }
                        }
                    }
                } else {
                    current_block = 16896160091404590989;
                }
                match current_block {
                    4091900800743210317 => {}
                    7315983924538012637 => {}
                    _ => {
                        if ((*leaf).mn_flags as libc::c_uint ^ flags)
                            & 0x2 as libc::c_int as libc::c_uint != 0
                        {
                            return -(30784 as libc::c_int);
                        }
                        if (*leaf).mn_flags as libc::c_int & 0x1 as libc::c_int
                            == 0x1 as libc::c_int
                        {
                            let mut omp: *mut MDB_page = 0 as *mut MDB_page;
                            let mut pg: pgno_t = 0;
                            let mut level: libc::c_int = 0;
                            let mut ovpages: libc::c_int = 0;
                            let mut dpages: libc::c_int = ((16 as libc::c_ulong
                                as libc::c_uint)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                as libc::c_ulong)
                                .wrapping_add((*data).mv_size)
                                .wrapping_div((*env).me_psize as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as libc::c_int;
                            memcpy(
                                &mut pg as *mut pgno_t as *mut libc::c_void,
                                olddata.mv_data,
                                ::std::mem::size_of::<pgno_t>() as libc::c_ulong,
                            );
                            rc2 = mdb_page_get(mc, pg, &mut omp, &mut level);
                            if rc2 != 0 as libc::c_int {
                                return rc2;
                            }
                            ovpages = (*omp).mp_pb.pb_pages as libc::c_int;
                            if ovpages >= dpages {
                                if (*omp).mp_flags as libc::c_int & 0x10 as libc::c_int == 0
                                    && (level != 0
                                        || (*env).me_flags & 0x80000 as libc::c_int as libc::c_uint
                                            != 0)
                                {
                                    rc = mdb_page_unspill((*mc).mc_txn, omp, &mut omp);
                                    if rc != 0 {
                                        return rc;
                                    }
                                    level = 0 as libc::c_int;
                                }
                                if (*omp).mp_flags as libc::c_int & 0x10 as libc::c_int != 0
                                {
                                    if level > 1 as libc::c_int {
                                        let mut sz: size_t = ((*env).me_psize as size_t)
                                            .wrapping_mul(ovpages as libc::c_ulong);
                                        let mut off: size_t = 0;
                                        let mut np_0: *mut MDB_page = mdb_page_malloc(
                                            (*mc).mc_txn,
                                            ovpages as libc::c_uint,
                                        );
                                        let mut id2: MDB_ID2 = MDB_ID2 {
                                            mid: 0,
                                            mptr: 0 as *mut libc::c_void,
                                        };
                                        if np_0.is_null() {
                                            return 12 as libc::c_int;
                                        }
                                        id2.mid = pg;
                                        id2.mptr = np_0 as *mut libc::c_void;
                                        rc2 = mdb_mid2l_insert(
                                            (*(*mc).mc_txn).mt_u.dirty_list,
                                            &mut id2,
                                        );
                                        if rc2 == 0 as libc::c_int {} else {
                                            mdb_assert_fail(
                                                (*(*mc).mc_txn).mt_env,
                                                b"rc2 == 0\0" as *const u8 as *const libc::c_char,
                                                (*::std::mem::transmute::<
                                                    &[u8; 15],
                                                    &[libc::c_char; 15],
                                                >(b"mdb_cursor_put\0"))
                                                    .as_ptr(),
                                                b"mdb.c\0" as *const u8 as *const libc::c_char,
                                                7913 as libc::c_int,
                                            );
                                        };
                                        if flags & 0x10000 as libc::c_int as libc::c_uint == 0 {
                                            off = (16 as libc::c_ulong as libc::c_uint as libc::c_ulong)
                                                .wrapping_add((*data).mv_size)
                                                & (::std::mem::size_of::<size_t>() as libc::c_ulong)
                                                    .wrapping_neg();
                                            memcpy(
                                                (np_0 as *mut libc::c_char).offset(off as isize)
                                                    as *mut size_t as *mut libc::c_void,
                                                (omp as *mut libc::c_char).offset(off as isize)
                                                    as *mut size_t as *const libc::c_void,
                                                sz.wrapping_sub(off),
                                            );
                                            sz = 16 as libc::c_ulong as libc::c_uint as size_t;
                                        }
                                        memcpy(
                                            np_0 as *mut libc::c_void,
                                            omp as *const libc::c_void,
                                            sz,
                                        );
                                        omp = np_0;
                                    }
                                    (*leaf)
                                        .mn_lo = ((*data).mv_size
                                        & 0xffff as libc::c_int as libc::c_ulong) as libc::c_ushort;
                                    (*leaf)
                                        .mn_hi = ((*data).mv_size >> 16 as libc::c_int)
                                        as libc::c_ushort;
                                    if flags & 0x10000 as libc::c_int as libc::c_uint
                                        == 0x10000 as libc::c_int as libc::c_uint
                                    {
                                        (*data)
                                            .mv_data = (omp as *mut libc::c_char)
                                            .offset(16 as libc::c_ulong as libc::c_uint as isize)
                                            as *mut libc::c_void;
                                    } else {
                                        memcpy(
                                            (omp as *mut libc::c_char)
                                                .offset(16 as libc::c_ulong as libc::c_uint as isize)
                                                as *mut libc::c_void,
                                            (*data).mv_data,
                                            (*data).mv_size,
                                        );
                                    }
                                    return 0 as libc::c_int;
                                }
                            }
                            rc2 = mdb_ovpage_free(mc, omp);
                            if rc2 != 0 as libc::c_int {
                                return rc2;
                            }
                        } else if (*data).mv_size == olddata.mv_size {
                            if flags & 0x10000 as libc::c_int as libc::c_uint
                                == 0x10000 as libc::c_int as libc::c_uint
                            {
                                (*data).mv_data = olddata.mv_data;
                                current_block = 7142959534011586167;
                                break;
                            } else if (*mc).mc_flags & 0x4 as libc::c_int as libc::c_uint
                                == 0
                            {
                                memcpy(olddata.mv_data, (*data).mv_data, (*data).mv_size);
                                current_block = 7142959534011586167;
                                break;
                            } else {
                                memcpy(
                                    ((*leaf).mn_data).as_mut_ptr() as *mut libc::c_void,
                                    (*key).mv_data,
                                    (*key).mv_size,
                                );
                                current_block = 5300594046240281167;
                                continue;
                            }
                        }
                        mdb_node_del(mc, 0 as libc::c_int);
                        current_block = 2661260690706066674;
                        continue;
                    }
                }
            }
        }
        match current_block {
            7315983924538012637 => {
                if mp != fp {
                    (*mp)
                        .mp_flags = (fp_flags as libc::c_int | 0x10 as libc::c_int)
                        as uint16_t;
                    (*mp).mp_pad = (*fp).mp_pad;
                    (*mp).mp_pb.pb.pb_lower = (*fp).mp_pb.pb.pb_lower;
                    (*mp)
                        .mp_pb
                        .pb
                        .pb_upper = ((*fp).mp_pb.pb.pb_upper as libc::c_uint)
                        .wrapping_add(offset) as indx_t;
                    if fp_flags as libc::c_int & 0x20 as libc::c_int != 0 {
                        memcpy(
                            (mp as *mut libc::c_char)
                                .offset(16 as libc::c_ulong as libc::c_uint as isize)
                                as *mut libc::c_void,
                            (fp as *mut libc::c_char)
                                .offset(16 as libc::c_ulong as libc::c_uint as isize)
                                as *mut libc::c_void,
                            (((*fp).mp_pb.pb.pb_lower as libc::c_uint)
                                .wrapping_sub(
                                    (16 as libc::c_ulong as libc::c_uint)
                                        .wrapping_sub(
                                            (if 0 as libc::c_int != 0 {
                                                16 as libc::c_ulong as libc::c_uint
                                            } else {
                                                0 as libc::c_int as libc::c_uint
                                            }),
                                        ),
                                ) >> 1 as libc::c_int)
                                .wrapping_mul((*fp).mp_pad as libc::c_uint) as libc::c_ulong,
                        );
                    } else {
                        memcpy(
                            (mp as *mut libc::c_char)
                                .offset((*mp).mp_pb.pb.pb_upper as libc::c_int as isize)
                                .offset(
                                    (if 0 as libc::c_int != 0 {
                                        16 as libc::c_ulong as libc::c_uint
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    }) as isize,
                                ) as *mut libc::c_void,
                            (fp as *mut libc::c_char)
                                .offset((*fp).mp_pb.pb.pb_upper as libc::c_int as isize)
                                .offset(
                                    (if 0 as libc::c_int != 0 {
                                        16 as libc::c_ulong as libc::c_uint
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    }) as isize,
                                ) as *const libc::c_void,
                            (olddata.mv_size)
                                .wrapping_sub((*fp).mp_pb.pb.pb_upper as libc::c_ulong)
                                .wrapping_sub(
                                    (if 0 as libc::c_int != 0 {
                                        16 as libc::c_ulong as libc::c_uint
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    }) as libc::c_ulong,
                                ),
                        );
                        memcpy(
                            &mut (*mp).mp_ptrs as *mut [indx_t; 1] as *mut libc::c_char
                                as *mut libc::c_void,
                            &mut (*fp).mp_ptrs as *mut [indx_t; 1] as *mut libc::c_char
                                as *const libc::c_void,
                            ((((*fp).mp_pb.pb.pb_lower as libc::c_uint)
                                .wrapping_sub(
                                    (16 as libc::c_ulong as libc::c_uint)
                                        .wrapping_sub(
                                            (if 0 as libc::c_int != 0 {
                                                16 as libc::c_ulong as libc::c_uint
                                            } else {
                                                0 as libc::c_int as libc::c_uint
                                            }),
                                        ),
                                ) >> 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<indx_t>() as libc::c_ulong,
                                ),
                        );
                        i = 0 as libc::c_int as libc::c_uint;
                        while i
                            < ((*fp).mp_pb.pb.pb_lower as libc::c_uint)
                                .wrapping_sub(
                                    (16 as libc::c_ulong as libc::c_uint)
                                        .wrapping_sub(
                                            (if 0 as libc::c_int != 0 {
                                                16 as libc::c_ulong as libc::c_uint
                                            } else {
                                                0 as libc::c_int as libc::c_uint
                                            }),
                                        ),
                                ) >> 1 as libc::c_int
                        {
                            let ref mut fresh30 = *((*mp).mp_ptrs)
                                .as_mut_ptr()
                                .offset(i as isize);
                            *fresh30 = (*fresh30 as libc::c_uint).wrapping_add(offset)
                                as indx_t as indx_t;
                            i = i.wrapping_add(1);
                            i;
                        }
                    }
                }
                rdata = &mut xdata;
                flags |= 0x4 as libc::c_int as libc::c_uint;
                do_sub = 1 as libc::c_int;
                if insert_key == 0 {
                    mdb_node_del(mc, 0 as libc::c_int);
                }
                current_block = 11632160926666140969;
            }
            _ => {}
        }
        match current_block {
            11632160926666140969 => {
                nflags = flags
                    & (0x4 as libc::c_int | 0x2 as libc::c_int | 0x10000 as libc::c_int
                        | 0x20000 as libc::c_int) as libc::c_uint;
                nsize = if (*(*mc).mc_pg[(*mc).mc_top as usize]).mp_flags as libc::c_int
                    & 0x20 as libc::c_int == 0x20 as libc::c_int
                {
                    (*key).mv_size
                } else {
                    mdb_leaf_size(env, key, rdata)
                };
                if (((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_upper
                    as libc::c_int
                    - (*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_lower
                        as libc::c_int) as indx_t as libc::c_ulong) < nsize
                {
                    if flags & (0x4 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint
                        == 0x4 as libc::c_int as libc::c_uint
                    {
                        nflags &= !(0x20000 as libc::c_int) as libc::c_uint;
                    }
                    if insert_key == 0 {
                        nflags |= 0x40000 as libc::c_int as libc::c_uint;
                    }
                    rc = mdb_page_split(
                        mc,
                        key,
                        rdata,
                        !(0 as libc::c_int as pgno_t),
                        nflags,
                    );
                } else {
                    rc = mdb_node_add(
                        mc,
                        (*mc).mc_ki[(*mc).mc_top as usize],
                        key,
                        rdata,
                        0 as libc::c_int as pgno_t,
                        nflags,
                    );
                    if rc == 0 as libc::c_int {
                        let mut m2: *mut MDB_cursor = 0 as *mut MDB_cursor;
                        let mut m3: *mut MDB_cursor = 0 as *mut MDB_cursor;
                        let mut dbi: MDB_dbi = (*mc).mc_dbi;
                        let mut i_0: libc::c_uint = (*mc).mc_top as libc::c_uint;
                        let mut mp_0: *mut MDB_page = (*mc).mc_pg[i_0 as usize];
                        m2 = *((*(*mc).mc_txn).mt_cursors).offset(dbi as isize);
                        while !m2.is_null() {
                            if (*mc).mc_flags & 0x4 as libc::c_int as libc::c_uint != 0 {
                                m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
                            } else {
                                m3 = m2;
                            }
                            if !(m3 == mc
                                || ((*m3).mc_snum as libc::c_int)
                                    < (*mc).mc_snum as libc::c_int
                                || (*m3).mc_pg[i_0 as usize] != mp_0)
                            {
                                if (*m3).mc_ki[i_0 as usize] as libc::c_int
                                    >= (*mc).mc_ki[i_0 as usize] as libc::c_int
                                    && insert_key != 0
                                {
                                    (*m3)
                                        .mc_ki[i_0
                                        as usize] = ((*m3).mc_ki[i_0 as usize]).wrapping_add(1);
                                    (*m3).mc_ki[i_0 as usize];
                                }
                                let mut xr_pg: *mut MDB_page = mp_0;
                                let mut xr_node: *mut MDB_node = 0 as *mut MDB_node;
                                if !(!(!((*m3).mc_xcursor).is_null()
                                    && (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                        & 0x1 as libc::c_int as libc::c_uint != 0)
                                    || (*m3).mc_ki[i_0 as usize] as libc::c_uint
                                        >= ((*xr_pg).mp_pb.pb.pb_lower as libc::c_uint)
                                            .wrapping_sub(
                                                (16 as libc::c_ulong as libc::c_uint)
                                                    .wrapping_sub(
                                                        (if 0 as libc::c_int != 0 {
                                                            16 as libc::c_ulong as libc::c_uint
                                                        } else {
                                                            0 as libc::c_int as libc::c_uint
                                                        }),
                                                    ),
                                            ) >> 1 as libc::c_int)
                                {
                                    xr_node = (xr_pg as *mut libc::c_char)
                                        .offset(
                                            *((*xr_pg).mp_ptrs)
                                                .as_mut_ptr()
                                                .offset((*m3).mc_ki[i_0 as usize] as isize) as libc::c_int
                                                as isize,
                                        )
                                        .offset(
                                            (if 0 as libc::c_int != 0 {
                                                16 as libc::c_ulong as libc::c_uint
                                            } else {
                                                0 as libc::c_int as libc::c_uint
                                            }) as isize,
                                        ) as *mut MDB_node;
                                    if (*xr_node).mn_flags as libc::c_int
                                        & (0x4 as libc::c_int | 0x2 as libc::c_int)
                                        == 0x4 as libc::c_int
                                    {
                                        (*(*m3).mc_xcursor)
                                            .mx_cursor
                                            .mc_pg[0 as libc::c_int
                                            as usize] = ((*xr_node).mn_data)
                                            .as_mut_ptr()
                                            .offset((*xr_node).mn_ksize as libc::c_int as isize)
                                            as *mut libc::c_void as *mut MDB_page;
                                    }
                                }
                            }
                            m2 = (*m2).mc_next;
                        }
                    }
                }
                if !(rc == 0 as libc::c_int) {
                    current_block = 7991679940794782184;
                    break;
                }
                if do_sub != 0 {
                    xflags = 0;
                    new_dupdata = 0;
                    ecount = 0;
                    current_block = 4091900800743210317;
                } else {
                    current_block = 11535013887061877469;
                }
            }
            _ => {}
        }
        match current_block {
            4091900800743210317 => {
                xdata.mv_size = 0 as libc::c_int as size_t;
                xdata
                    .mv_data = b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_void;
                leaf = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
                    .offset(
                        *((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_ptrs)
                            .as_mut_ptr()
                            .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                            as libc::c_int as isize,
                    )
                    .offset(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                if flags & (0x40 as libc::c_int | 0x40000 as libc::c_int) as libc::c_uint
                    == 0x40 as libc::c_int as libc::c_uint
                {
                    xflags = 0x40 as libc::c_int | 0x8000 as libc::c_int;
                } else {
                    mdb_xcursor_init1(mc, leaf);
                    xflags = if flags & 0x20 as libc::c_int as libc::c_uint != 0 {
                        0x10 as libc::c_int | 0x8000 as libc::c_int
                    } else {
                        0x8000 as libc::c_int
                    };
                }
                if !sub_root.is_null() {
                    (*(*mc).mc_xcursor)
                        .mx_cursor
                        .mc_pg[0 as libc::c_int as usize] = sub_root;
                }
                new_dupdata = dkey.mv_size as libc::c_int;
                if dkey.mv_size != 0 {
                    rc = mdb_cursor_put(
                        &mut (*(*mc).mc_xcursor).mx_cursor,
                        &mut dkey,
                        &mut xdata,
                        xflags as libc::c_uint,
                    );
                    if rc != 0 {
                        current_block = 4443411524525352111;
                        break;
                    }
                    dkey.mv_size = 0 as libc::c_int as size_t;
                }
                if (*leaf).mn_flags as libc::c_int & 0x2 as libc::c_int == 0
                    || !sub_root.is_null()
                {
                    let mut m2_0: *mut MDB_cursor = 0 as *mut MDB_cursor;
                    let mut mx: *mut MDB_xcursor = (*mc).mc_xcursor;
                    let mut i_1: libc::c_uint = (*mc).mc_top as libc::c_uint;
                    let mut mp_1: *mut MDB_page = (*mc).mc_pg[i_1 as usize];
                    m2_0 = *((*(*mc).mc_txn).mt_cursors).offset((*mc).mc_dbi as isize);
                    while !m2_0.is_null() {
                        if !(m2_0 == mc
                            || ((*m2_0).mc_snum as libc::c_int)
                                < (*mc).mc_snum as libc::c_int)
                        {
                            if !((*m2_0).mc_flags & 0x1 as libc::c_int as libc::c_uint
                                == 0)
                            {
                                if (*m2_0).mc_pg[i_1 as usize] == mp_1 {
                                    if (*m2_0).mc_ki[i_1 as usize] as libc::c_int
                                        == (*mc).mc_ki[i_1 as usize] as libc::c_int
                                    {
                                        mdb_xcursor_init2(m2_0, mx, new_dupdata);
                                    } else if insert_key == 0 {
                                        let mut xr_pg_0: *mut MDB_page = mp_1;
                                        let mut xr_node_0: *mut MDB_node = 0 as *mut MDB_node;
                                        if !(!(!((*m2_0).mc_xcursor).is_null()
                                            && (*(*m2_0).mc_xcursor).mx_cursor.mc_flags
                                                & 0x1 as libc::c_int as libc::c_uint != 0)
                                            || (*m2_0).mc_ki[i_1 as usize] as libc::c_uint
                                                >= ((*xr_pg_0).mp_pb.pb.pb_lower as libc::c_uint)
                                                    .wrapping_sub(
                                                        (16 as libc::c_ulong as libc::c_uint)
                                                            .wrapping_sub(
                                                                (if 0 as libc::c_int != 0 {
                                                                    16 as libc::c_ulong as libc::c_uint
                                                                } else {
                                                                    0 as libc::c_int as libc::c_uint
                                                                }),
                                                            ),
                                                    ) >> 1 as libc::c_int)
                                        {
                                            xr_node_0 = (xr_pg_0 as *mut libc::c_char)
                                                .offset(
                                                    *((*xr_pg_0).mp_ptrs)
                                                        .as_mut_ptr()
                                                        .offset((*m2_0).mc_ki[i_1 as usize] as isize) as libc::c_int
                                                        as isize,
                                                )
                                                .offset(
                                                    (if 0 as libc::c_int != 0 {
                                                        16 as libc::c_ulong as libc::c_uint
                                                    } else {
                                                        0 as libc::c_int as libc::c_uint
                                                    }) as isize,
                                                ) as *mut MDB_node;
                                            if (*xr_node_0).mn_flags as libc::c_int
                                                & (0x4 as libc::c_int | 0x2 as libc::c_int)
                                                == 0x4 as libc::c_int
                                            {
                                                (*(*m2_0).mc_xcursor)
                                                    .mx_cursor
                                                    .mc_pg[0 as libc::c_int
                                                    as usize] = ((*xr_node_0).mn_data)
                                                    .as_mut_ptr()
                                                    .offset((*xr_node_0).mn_ksize as libc::c_int as isize)
                                                    as *mut libc::c_void as *mut MDB_page;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        m2_0 = (*m2_0).mc_next;
                    }
                }
                ecount = (*(*mc).mc_xcursor).mx_db.md_entries;
                if flags & 0x40000 as libc::c_int as libc::c_uint != 0 {
                    xflags |= 0x20000 as libc::c_int;
                }
                rc = mdb_cursor_put(
                    &mut (*(*mc).mc_xcursor).mx_cursor,
                    data,
                    &mut xdata,
                    xflags as libc::c_uint,
                );
                if flags & 0x2 as libc::c_int as libc::c_uint != 0 {
                    let mut db: *mut libc::c_void = ((*leaf).mn_data)
                        .as_mut_ptr()
                        .offset((*leaf).mn_ksize as libc::c_int as isize)
                        as *mut libc::c_void;
                    memcpy(
                        db,
                        &mut (*(*mc).mc_xcursor).mx_db as *mut MDB_db
                            as *const libc::c_void,
                        ::std::mem::size_of::<MDB_db>() as libc::c_ulong,
                    );
                }
                insert_data = ((*(*mc).mc_xcursor).mx_db.md_entries).wrapping_sub(ecount)
                    as libc::c_int;
            }
            _ => {}
        }
        if insert_data != 0 {
            (*(*mc).mc_db).md_entries = ((*(*mc).mc_db).md_entries).wrapping_add(1);
            (*(*mc).mc_db).md_entries;
        }
        if insert_key != 0 {
            if rc != 0 {
                current_block = 4443411524525352111;
                break;
            }
            (*mc).mc_flags |= 0x1 as libc::c_int as libc::c_uint;
        }
        if !(flags & 0x80000 as libc::c_int as libc::c_uint != 0) {
            current_block = 4839309778395429725;
            break;
        }
        if !(rc == 0) {
            current_block = 4839309778395429725;
            break;
        }
        mcount = mcount.wrapping_add(1);
        mcount;
        (*data.offset(1 as libc::c_int as isize)).mv_size = mcount as size_t;
        if !(mcount < dcount) {
            current_block = 4839309778395429725;
            break;
        }
        let ref mut fresh31 = (*data.offset(0 as libc::c_int as isize)).mv_data;
        *fresh31 = ((*data.offset(0 as libc::c_int as isize)).mv_data
            as *mut libc::c_char)
            .offset((*data.offset(0 as libc::c_int as isize)).mv_size as isize)
            as *mut libc::c_void;
        insert_data = 0 as libc::c_int;
        insert_key = insert_data;
        current_block = 14635294375004343297;
    }
    match current_block {
        4839309778395429725 => return rc,
        4443411524525352111 => {
            if rc == -(30799 as libc::c_int) {
                rc = -(30779 as libc::c_int);
            }
        }
        7142959534011586167 => return 0 as libc::c_int,
        _ => {}
    }
    (*(*mc).mc_txn).mt_flags |= 0x2 as libc::c_int as libc::c_uint;
    return rc;
}
pub unsafe extern "C" fn mdb_cursor_del(
    mut mc: *mut MDB_cursor,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut current_block: u64;
    let mut leaf: *mut MDB_node = 0 as *mut MDB_node;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut rc: libc::c_int = 0;
    if (*(*mc).mc_txn).mt_flags
        & (0x20000 as libc::c_int
            | (0x1 as libc::c_int | 0x2 as libc::c_int | 0x10 as libc::c_int))
            as libc::c_uint != 0
    {
        return if (*(*mc).mc_txn).mt_flags & 0x20000 as libc::c_int as libc::c_uint != 0
        {
            13 as libc::c_int
        } else {
            -(30782 as libc::c_int)
        };
    }
    if (*mc).mc_flags & 0x1 as libc::c_int as libc::c_uint == 0 {
        return 22 as libc::c_int;
    }
    if (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint
        >= ((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                (16 as libc::c_ulong as libc::c_uint)
                    .wrapping_sub(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }),
                    ),
            ) >> 1 as libc::c_int
    {
        return -(30798 as libc::c_int);
    }
    if flags & 0x8000 as libc::c_int as libc::c_uint == 0
        && {
            rc = mdb_page_spill(mc, 0 as *mut MDB_val, 0 as *mut MDB_val);
            rc != 0
        }
    {
        return rc;
    }
    rc = mdb_cursor_touch(mc);
    if rc != 0 {
        return rc;
    }
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    if !((*mp).mp_flags as libc::c_int & 0x2 as libc::c_int == 0x2 as libc::c_int) {
        return -(30796 as libc::c_int);
    }
    if !((*mp).mp_flags as libc::c_int & 0x20 as libc::c_int == 0x20 as libc::c_int) {
        leaf = (mp as *mut libc::c_char)
            .offset(
                *((*mp).mp_ptrs)
                    .as_mut_ptr()
                    .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize) as libc::c_int
                    as isize,
            )
            .offset(
                (if 0 as libc::c_int != 0 {
                    16 as libc::c_ulong as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }) as isize,
            ) as *mut MDB_node;
        if (*leaf).mn_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int {
            if flags & 0x20 as libc::c_int as libc::c_uint != 0 {
                (*(*mc).mc_db)
                    .md_entries = ((*(*mc).mc_db).md_entries as libc::c_ulong)
                    .wrapping_sub(
                        ((*(*mc).mc_xcursor).mx_db.md_entries)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) as mdb_size_t as mdb_size_t;
                (*(*mc).mc_xcursor).mx_cursor.mc_flags
                    &= !(0x1 as libc::c_int) as libc::c_uint;
            } else {
                if !((*leaf).mn_flags as libc::c_int & 0x2 as libc::c_int
                    == 0x2 as libc::c_int)
                {
                    (*(*mc).mc_xcursor)
                        .mx_cursor
                        .mc_pg[0 as libc::c_int
                        as usize] = ((*leaf).mn_data)
                        .as_mut_ptr()
                        .offset((*leaf).mn_ksize as libc::c_int as isize)
                        as *mut libc::c_void as *mut MDB_page;
                }
                rc = mdb_cursor_del(
                    &mut (*(*mc).mc_xcursor).mx_cursor,
                    0x8000 as libc::c_int as libc::c_uint,
                );
                if rc != 0 {
                    return rc;
                }
                if (*(*mc).mc_xcursor).mx_db.md_entries != 0 {
                    if (*leaf).mn_flags as libc::c_int & 0x2 as libc::c_int != 0 {
                        let mut db: *mut libc::c_void = ((*leaf).mn_data)
                            .as_mut_ptr()
                            .offset((*leaf).mn_ksize as libc::c_int as isize)
                            as *mut libc::c_void;
                        memcpy(
                            db,
                            &mut (*(*mc).mc_xcursor).mx_db as *mut MDB_db
                                as *const libc::c_void,
                            ::std::mem::size_of::<MDB_db>() as libc::c_ulong,
                        );
                    } else {
                        let mut m2: *mut MDB_cursor = 0 as *mut MDB_cursor;
                        mdb_node_shrink(mp, (*mc).mc_ki[(*mc).mc_top as usize]);
                        leaf = (mp as *mut libc::c_char)
                            .offset(
                                *((*mp).mp_ptrs)
                                    .as_mut_ptr()
                                    .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                                    as libc::c_int as isize,
                            )
                            .offset(
                                (if 0 as libc::c_int != 0 {
                                    16 as libc::c_ulong as libc::c_uint
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }) as isize,
                            ) as *mut MDB_node;
                        (*(*mc).mc_xcursor)
                            .mx_cursor
                            .mc_pg[0 as libc::c_int
                            as usize] = ((*leaf).mn_data)
                            .as_mut_ptr()
                            .offset((*leaf).mn_ksize as libc::c_int as isize)
                            as *mut libc::c_void as *mut MDB_page;
                        m2 = *((*(*mc).mc_txn).mt_cursors).offset((*mc).mc_dbi as isize);
                        while !m2.is_null() {
                            if !(m2 == mc
                                || ((*m2).mc_snum as libc::c_int)
                                    < (*mc).mc_snum as libc::c_int)
                            {
                                if !((*m2).mc_flags & 0x1 as libc::c_int as libc::c_uint
                                    == 0)
                                {
                                    if (*m2).mc_pg[(*mc).mc_top as usize] == mp {
                                        let mut xr_pg: *mut MDB_page = mp;
                                        let mut xr_node: *mut MDB_node = 0 as *mut MDB_node;
                                        if !(!(!((*m2).mc_xcursor).is_null()
                                            && (*(*m2).mc_xcursor).mx_cursor.mc_flags
                                                & 0x1 as libc::c_int as libc::c_uint != 0)
                                            || (*m2).mc_ki[(*mc).mc_top as usize] as libc::c_uint
                                                >= ((*xr_pg).mp_pb.pb.pb_lower as libc::c_uint)
                                                    .wrapping_sub(
                                                        (16 as libc::c_ulong as libc::c_uint)
                                                            .wrapping_sub(
                                                                (if 0 as libc::c_int != 0 {
                                                                    16 as libc::c_ulong as libc::c_uint
                                                                } else {
                                                                    0 as libc::c_int as libc::c_uint
                                                                }),
                                                            ),
                                                    ) >> 1 as libc::c_int)
                                        {
                                            xr_node = (xr_pg as *mut libc::c_char)
                                                .offset(
                                                    *((*xr_pg).mp_ptrs)
                                                        .as_mut_ptr()
                                                        .offset((*m2).mc_ki[(*mc).mc_top as usize] as isize)
                                                        as libc::c_int as isize,
                                                )
                                                .offset(
                                                    (if 0 as libc::c_int != 0 {
                                                        16 as libc::c_ulong as libc::c_uint
                                                    } else {
                                                        0 as libc::c_int as libc::c_uint
                                                    }) as isize,
                                                ) as *mut MDB_node;
                                            if (*xr_node).mn_flags as libc::c_int
                                                & (0x4 as libc::c_int | 0x2 as libc::c_int)
                                                == 0x4 as libc::c_int
                                            {
                                                (*(*m2).mc_xcursor)
                                                    .mx_cursor
                                                    .mc_pg[0 as libc::c_int
                                                    as usize] = ((*xr_node).mn_data)
                                                    .as_mut_ptr()
                                                    .offset((*xr_node).mn_ksize as libc::c_int as isize)
                                                    as *mut libc::c_void as *mut MDB_page;
                                            }
                                        }
                                    }
                                }
                            }
                            m2 = (*m2).mc_next;
                        }
                    }
                    (*(*mc).mc_db)
                        .md_entries = ((*(*mc).mc_db).md_entries).wrapping_sub(1);
                    (*(*mc).mc_db).md_entries;
                    return rc;
                } else {
                    (*(*mc).mc_xcursor).mx_cursor.mc_flags
                        &= !(0x1 as libc::c_int) as libc::c_uint;
                }
            }
            if (*leaf).mn_flags as libc::c_int & 0x2 as libc::c_int != 0 {
                rc = mdb_drop0(&mut (*(*mc).mc_xcursor).mx_cursor, 0 as libc::c_int);
                if rc != 0 {
                    current_block = 1177862984040527687;
                } else {
                    current_block = 3689906465960840878;
                }
            } else {
                current_block = 3689906465960840878;
            }
        } else if ((*leaf).mn_flags as libc::c_uint ^ flags)
            & 0x2 as libc::c_int as libc::c_uint != 0
        {
            rc = -(30784 as libc::c_int);
            current_block = 1177862984040527687;
        } else {
            current_block = 3689906465960840878;
        }
        match current_block {
            3689906465960840878 => {
                if (*leaf).mn_flags as libc::c_int & 0x1 as libc::c_int
                    == 0x1 as libc::c_int
                {
                    let mut omp: *mut MDB_page = 0 as *mut MDB_page;
                    let mut pg: pgno_t = 0;
                    memcpy(
                        &mut pg as *mut pgno_t as *mut libc::c_void,
                        ((*leaf).mn_data)
                            .as_mut_ptr()
                            .offset((*leaf).mn_ksize as libc::c_int as isize)
                            as *mut libc::c_void,
                        ::std::mem::size_of::<pgno_t>() as libc::c_ulong,
                    );
                    rc = mdb_page_get(mc, pg, &mut omp, 0 as *mut libc::c_int);
                    if rc != 0
                        || {
                            rc = mdb_ovpage_free(mc, omp);
                            rc != 0
                        }
                    {
                        current_block = 1177862984040527687;
                    } else {
                        current_block = 414833563694728819;
                    }
                } else {
                    current_block = 414833563694728819;
                }
            }
            _ => {}
        }
        match current_block {
            414833563694728819 => {}
            _ => {
                (*(*mc).mc_txn).mt_flags |= 0x2 as libc::c_int as libc::c_uint;
                return rc;
            }
        }
    }
    return mdb_cursor_del0(mc);
}
unsafe extern "C" fn mdb_page_new(
    mut mc: *mut MDB_cursor,
    mut flags: uint32_t,
    mut num: libc::c_int,
    mut mp: *mut *mut MDB_page,
) -> libc::c_int {
    let mut np: *mut MDB_page = 0 as *mut MDB_page;
    let mut rc: libc::c_int = 0;
    rc = mdb_page_alloc(mc, num, &mut np);
    if rc != 0 {
        return rc;
    }
    (*np).mp_flags = (flags | 0x10 as libc::c_int as libc::c_uint) as uint16_t;
    (*np)
        .mp_pb
        .pb
        .pb_lower = (16 as libc::c_ulong as libc::c_uint)
        .wrapping_sub(
            (if 0 as libc::c_int != 0 {
                16 as libc::c_ulong as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }),
        ) as indx_t;
    (*np)
        .mp_pb
        .pb
        .pb_upper = ((*(*(*mc).mc_txn).mt_env).me_psize)
        .wrapping_sub(
            (if 0 as libc::c_int != 0 {
                16 as libc::c_ulong as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }),
        ) as indx_t;
    if (*np).mp_flags as libc::c_int & 0x1 as libc::c_int == 0x1 as libc::c_int {
        (*(*mc).mc_db)
            .md_branch_pages = ((*(*mc).mc_db).md_branch_pages).wrapping_add(1);
        (*(*mc).mc_db).md_branch_pages;
    } else if (*np).mp_flags as libc::c_int & 0x2 as libc::c_int == 0x2 as libc::c_int {
        (*(*mc).mc_db).md_leaf_pages = ((*(*mc).mc_db).md_leaf_pages).wrapping_add(1);
        (*(*mc).mc_db).md_leaf_pages;
    } else if (*np).mp_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int {
        (*(*mc).mc_db)
            .md_overflow_pages = ((*(*mc).mc_db).md_overflow_pages as libc::c_ulong)
            .wrapping_add(num as libc::c_ulong) as pgno_t as pgno_t;
        (*np).mp_pb.pb_pages = num as uint32_t;
    }
    *mp = np;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_leaf_size(
    mut env: *mut MDB_env,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> size_t {
    let mut sz: size_t = 0;
    sz = (8 as libc::c_ulong).wrapping_add((*key).mv_size).wrapping_add((*data).mv_size);
    if sz > (*env).me_nodemax as libc::c_ulong {
        sz = (sz as libc::c_ulong)
            .wrapping_sub(
                ((*data).mv_size)
                    .wrapping_sub(::std::mem::size_of::<pgno_t>() as libc::c_ulong),
            ) as size_t as size_t;
    }
    return sz
        .wrapping_add(::std::mem::size_of::<indx_t>() as libc::c_ulong)
        .wrapping_add(1 as libc::c_uint as libc::c_ulong)
        & -(2 as libc::c_int) as libc::c_ulong;
}
unsafe extern "C" fn mdb_branch_size(
    mut env: *mut MDB_env,
    mut key: *mut MDB_val,
) -> size_t {
    let mut sz: size_t = 0;
    sz = (8 as libc::c_ulong)
        .wrapping_add(
            (if key.is_null() {
                0 as libc::c_int as libc::c_ulong
            } else {
                (*key).mv_size
            }),
        );
    sz > (*env).me_nodemax as libc::c_ulong;
    return sz.wrapping_add(::std::mem::size_of::<indx_t>() as libc::c_ulong);
}
unsafe extern "C" fn mdb_node_add(
    mut mc: *mut MDB_cursor,
    mut indx: indx_t,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut pgno: pgno_t,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_uint = 0;
    let mut node_size: size_t = 8 as libc::c_ulong;
    let mut room: ssize_t = 0;
    let mut ofs: indx_t = 0;
    let mut node: *mut MDB_node = 0 as *mut MDB_node;
    let mut mp: *mut MDB_page = (*mc).mc_pg[(*mc).mc_top as usize];
    let mut ofp: *mut MDB_page = 0 as *mut MDB_page;
    let mut ndata: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*mp).mp_pb.pb.pb_upper as libc::c_int >= (*mp).mp_pb.pb.pb_lower as libc::c_int
    {} else {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"mp->mp_upper >= mp->mp_lower\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"mdb_node_add\0"))
                .as_ptr(),
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            8306 as libc::c_int,
        );
    };
    if (*mp).mp_flags as libc::c_int & 0x20 as libc::c_int == 0x20 as libc::c_int {
        let mut ksize: libc::c_int = (*(*mc).mc_db).md_pad as libc::c_int;
        let mut dif: libc::c_int = 0;
        let mut ptr: *mut libc::c_char = (mp as *mut libc::c_char)
            .offset(16 as libc::c_ulong as libc::c_uint as isize)
            .offset((indx as libc::c_int * ksize) as isize);
        dif = (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                (16 as libc::c_ulong as libc::c_uint)
                    .wrapping_sub(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }),
                    ),
            ) >> 1 as libc::c_int)
            .wrapping_sub(indx as libc::c_uint) as libc::c_int;
        if dif > 0 as libc::c_int {
            memmove(
                ptr.offset(ksize as isize) as *mut libc::c_void,
                ptr as *const libc::c_void,
                (dif * ksize) as libc::c_ulong,
            );
        }
        memcpy(ptr as *mut libc::c_void, (*key).mv_data, ksize as libc::c_ulong);
        (*mp)
            .mp_pb
            .pb
            .pb_lower = ((*mp).mp_pb.pb.pb_lower as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<indx_t>() as libc::c_ulong) as indx_t
            as indx_t;
        (*mp)
            .mp_pb
            .pb
            .pb_upper = ((*mp).mp_pb.pb.pb_upper as libc::c_ulong)
            .wrapping_sub(
                (ksize as libc::c_ulong)
                    .wrapping_sub(::std::mem::size_of::<indx_t>() as libc::c_ulong),
            ) as indx_t as indx_t;
        return 0 as libc::c_int;
    }
    room = ((*mp).mp_pb.pb.pb_upper as libc::c_int
        - (*mp).mp_pb.pb.pb_lower as libc::c_int) as indx_t as ssize_t
        - ::std::mem::size_of::<indx_t>() as libc::c_ulong as ssize_t;
    if !key.is_null() {
        node_size = (node_size as libc::c_ulong).wrapping_add((*key).mv_size) as size_t
            as size_t;
    }
    if (*mp).mp_flags as libc::c_int & 0x2 as libc::c_int == 0x2 as libc::c_int {
        if !key.is_null() && !data.is_null() {} else {
            mdb_assert_fail(
                (*(*mc).mc_txn).mt_env,
                b"key && data\0" as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"mdb_node_add\0"))
                    .as_ptr(),
                b"mdb.c\0" as *const u8 as *const libc::c_char,
                8334 as libc::c_int,
            );
        };
        if flags & 0x1 as libc::c_int as libc::c_uint
            == 0x1 as libc::c_int as libc::c_uint
        {
            node_size = (node_size as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<pgno_t>() as libc::c_ulong) as size_t
                as size_t;
            current_block = 11057878835866523405;
        } else if node_size.wrapping_add((*data).mv_size)
            > (*(*(*mc).mc_txn).mt_env).me_nodemax as libc::c_ulong
        {
            let mut ovpages: libc::c_int = ((16 as libc::c_ulong as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
                .wrapping_add((*data).mv_size)
                .wrapping_div((*(*(*mc).mc_txn).mt_env).me_psize as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
            let mut rc: libc::c_int = 0;
            node_size = node_size
                .wrapping_add(::std::mem::size_of::<pgno_t>() as libc::c_ulong)
                .wrapping_add(1 as libc::c_uint as libc::c_ulong)
                & -(2 as libc::c_int) as libc::c_ulong;
            if node_size as ssize_t > room {
                current_block = 15594603006322722090;
            } else {
                rc = mdb_page_new(mc, 0x4 as libc::c_int as uint32_t, ovpages, &mut ofp);
                if rc != 0 {
                    return rc;
                }
                flags |= 0x1 as libc::c_int as libc::c_uint;
                current_block = 17289349619643943057;
            }
        } else {
            node_size = (node_size as libc::c_ulong).wrapping_add((*data).mv_size)
                as size_t as size_t;
            current_block = 11057878835866523405;
        }
    } else {
        current_block = 11057878835866523405;
    }
    match current_block {
        11057878835866523405 => {
            node_size = node_size.wrapping_add(1 as libc::c_uint as libc::c_ulong)
                & -(2 as libc::c_int) as libc::c_ulong;
            if node_size as ssize_t > room {
                current_block = 15594603006322722090;
            } else {
                current_block = 17289349619643943057;
            }
        }
        _ => {}
    }
    match current_block {
        15594603006322722090 => {
            (*(*mc).mc_txn).mt_flags |= 0x2 as libc::c_int as libc::c_uint;
            return -(30786 as libc::c_int);
        }
        _ => {
            i = ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                .wrapping_sub(
                    (16 as libc::c_ulong as libc::c_uint)
                        .wrapping_sub(
                            (if 0 as libc::c_int != 0 {
                                16 as libc::c_ulong as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }),
                        ),
                ) >> 1 as libc::c_int;
            while i > indx as libc::c_uint {
                *((*mp).mp_ptrs)
                    .as_mut_ptr()
                    .offset(
                        i as isize,
                    ) = *((*mp).mp_ptrs)
                    .as_mut_ptr()
                    .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
                i = i.wrapping_sub(1);
                i;
            }
            ofs = ((*mp).mp_pb.pb.pb_upper as libc::c_ulong).wrapping_sub(node_size)
                as indx_t;
            if ofs as libc::c_ulong
                >= ((*mp).mp_pb.pb.pb_lower as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<indx_t>() as libc::c_ulong)
            {} else {
                mdb_assert_fail(
                    (*(*mc).mc_txn).mt_env,
                    b"ofs >= mp->mp_lower + sizeof(indx_t)\0" as *const u8
                        as *const libc::c_char,
                    (*::std::mem::transmute::<
                        &[u8; 13],
                        &[libc::c_char; 13],
                    >(b"mdb_node_add\0"))
                        .as_ptr(),
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    8367 as libc::c_int,
                );
            };
            *((*mp).mp_ptrs).as_mut_ptr().offset(indx as isize) = ofs;
            (*mp).mp_pb.pb.pb_upper = ofs;
            (*mp)
                .mp_pb
                .pb
                .pb_lower = ((*mp).mp_pb.pb.pb_lower as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<indx_t>() as libc::c_ulong) as indx_t
                as indx_t;
            node = (mp as *mut libc::c_char)
                .offset(
                    *((*mp).mp_ptrs).as_mut_ptr().offset(indx as isize) as libc::c_int
                        as isize,
                )
                .offset(
                    (if 0 as libc::c_int != 0 {
                        16 as libc::c_ulong as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }) as isize,
                ) as *mut MDB_node;
            (*node)
                .mn_ksize = (if key.is_null() {
                0 as libc::c_int as libc::c_ulong
            } else {
                (*key).mv_size
            }) as libc::c_ushort;
            (*node).mn_flags = flags as libc::c_ushort;
            if (*mp).mp_flags as libc::c_int & 0x2 as libc::c_int == 0x2 as libc::c_int {
                (*node)
                    .mn_lo = ((*data).mv_size & 0xffff as libc::c_int as libc::c_ulong)
                    as libc::c_ushort;
                (*node).mn_hi = ((*data).mv_size >> 16 as libc::c_int) as libc::c_ushort;
            } else {
                (*node)
                    .mn_lo = (pgno & 0xffff as libc::c_int as libc::c_ulong)
                    as libc::c_ushort;
                (*node).mn_hi = (pgno >> 16 as libc::c_int) as libc::c_ushort;
                if if -(1 as libc::c_int) as pgno_t
                    > 0xffffffff as libc::c_uint as libc::c_ulong
                {
                    32 as libc::c_int
                } else {
                    0 as libc::c_int
                } != 0
                {
                    (*node)
                        .mn_flags = (pgno
                        >> (if -(1 as libc::c_int) as pgno_t
                            > 0xffffffff as libc::c_uint as libc::c_ulong
                        {
                            32 as libc::c_int
                        } else {
                            0 as libc::c_int
                        })) as libc::c_ushort;
                }
            }
            if !key.is_null() {
                memcpy(
                    ((*node).mn_data).as_mut_ptr() as *mut libc::c_void,
                    (*key).mv_data,
                    (*key).mv_size,
                );
            }
            if (*mp).mp_flags as libc::c_int & 0x2 as libc::c_int == 0x2 as libc::c_int {
                ndata = ((*node).mn_data)
                    .as_mut_ptr()
                    .offset((*node).mn_ksize as libc::c_int as isize)
                    as *mut libc::c_void;
                if ofp.is_null() {
                    if flags & 0x1 as libc::c_int as libc::c_uint
                        == 0x1 as libc::c_int as libc::c_uint
                    {
                        memcpy(
                            ndata,
                            (*data).mv_data,
                            ::std::mem::size_of::<pgno_t>() as libc::c_ulong,
                        );
                    } else if flags & 0x10000 as libc::c_int as libc::c_uint
                        == 0x10000 as libc::c_int as libc::c_uint
                    {
                        (*data).mv_data = ndata;
                    } else {
                        memcpy(ndata, (*data).mv_data, (*data).mv_size);
                    }
                } else {
                    memcpy(
                        ndata,
                        &mut (*ofp).mp_p.p_pgno as *mut pgno_t as *const libc::c_void,
                        ::std::mem::size_of::<pgno_t>() as libc::c_ulong,
                    );
                    ndata = (ofp as *mut libc::c_char)
                        .offset(16 as libc::c_ulong as libc::c_uint as isize)
                        as *mut libc::c_void;
                    if flags & 0x10000 as libc::c_int as libc::c_uint
                        == 0x10000 as libc::c_int as libc::c_uint
                    {
                        (*data).mv_data = ndata;
                    } else {
                        memcpy(ndata, (*data).mv_data, (*data).mv_size);
                    }
                }
            }
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn mdb_node_del(mut mc: *mut MDB_cursor, mut ksize: libc::c_int) {
    let mut mp: *mut MDB_page = (*mc).mc_pg[(*mc).mc_top as usize];
    let mut indx: indx_t = (*mc).mc_ki[(*mc).mc_top as usize];
    let mut sz: libc::c_uint = 0;
    let mut i: indx_t = 0;
    let mut j: indx_t = 0;
    let mut numkeys: indx_t = 0;
    let mut ptr: indx_t = 0;
    let mut node: *mut MDB_node = 0 as *mut MDB_node;
    let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
    numkeys = (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
        .wrapping_sub(
            (16 as libc::c_ulong as libc::c_uint)
                .wrapping_sub(
                    (if 0 as libc::c_int != 0 {
                        16 as libc::c_ulong as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                ),
        ) >> 1 as libc::c_int) as indx_t;
    if (indx as libc::c_int) < numkeys as libc::c_int {} else {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"indx < numkeys\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"mdb_node_del\0"))
                .as_ptr(),
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            8432 as libc::c_int,
        );
    };
    if (*mp).mp_flags as libc::c_int & 0x20 as libc::c_int == 0x20 as libc::c_int {
        let mut x: libc::c_int = numkeys as libc::c_int - 1 as libc::c_int
            - indx as libc::c_int;
        base = (mp as *mut libc::c_char)
            .offset(16 as libc::c_ulong as libc::c_uint as isize)
            .offset((indx as libc::c_int * ksize) as isize);
        if x != 0 {
            memmove(
                base as *mut libc::c_void,
                base.offset(ksize as isize) as *const libc::c_void,
                (x * ksize) as libc::c_ulong,
            );
        }
        (*mp)
            .mp_pb
            .pb
            .pb_lower = ((*mp).mp_pb.pb.pb_lower as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<indx_t>() as libc::c_ulong) as indx_t
            as indx_t;
        (*mp)
            .mp_pb
            .pb
            .pb_upper = ((*mp).mp_pb.pb.pb_upper as libc::c_ulong)
            .wrapping_add(
                (ksize as libc::c_ulong)
                    .wrapping_sub(::std::mem::size_of::<indx_t>() as libc::c_ulong),
            ) as indx_t as indx_t;
        return;
    }
    node = (mp as *mut libc::c_char)
        .offset(
            *((*mp).mp_ptrs).as_mut_ptr().offset(indx as isize) as libc::c_int as isize,
        )
        .offset(
            (if 0 as libc::c_int != 0 {
                16 as libc::c_ulong as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        ) as *mut MDB_node;
    sz = (8 as libc::c_ulong).wrapping_add((*node).mn_ksize as libc::c_ulong)
        as libc::c_uint;
    if (*mp).mp_flags as libc::c_int & 0x2 as libc::c_int == 0x2 as libc::c_int {
        if (*node).mn_flags as libc::c_int & 0x1 as libc::c_int == 0x1 as libc::c_int {
            sz = (sz as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<pgno_t>() as libc::c_ulong)
                as libc::c_uint as libc::c_uint;
        } else {
            sz = sz
                .wrapping_add(
                    (*node).mn_lo as libc::c_uint
                        | ((*node).mn_hi as libc::c_uint) << 16 as libc::c_int,
                );
        }
    }
    sz = sz.wrapping_add(1 as libc::c_uint) & -(2 as libc::c_int) as libc::c_uint;
    ptr = *((*mp).mp_ptrs).as_mut_ptr().offset(indx as isize);
    j = 0 as libc::c_int as indx_t;
    i = j;
    while (i as libc::c_int) < numkeys as libc::c_int {
        if i as libc::c_int != indx as libc::c_int {
            *((*mp).mp_ptrs)
                .as_mut_ptr()
                .offset(j as isize) = *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize);
            if (*((*mp).mp_ptrs).as_mut_ptr().offset(i as isize) as libc::c_int)
                < ptr as libc::c_int
            {
                let ref mut fresh32 = *((*mp).mp_ptrs).as_mut_ptr().offset(j as isize);
                *fresh32 = (*fresh32 as libc::c_uint).wrapping_add(sz) as indx_t
                    as indx_t;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    base = (mp as *mut libc::c_char)
        .offset((*mp).mp_pb.pb.pb_upper as libc::c_int as isize)
        .offset(
            (if 0 as libc::c_int != 0 {
                16 as libc::c_ulong as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        );
    memmove(
        base.offset(sz as isize) as *mut libc::c_void,
        base as *const libc::c_void,
        (ptr as libc::c_int - (*mp).mp_pb.pb.pb_upper as libc::c_int) as libc::c_ulong,
    );
    (*mp)
        .mp_pb
        .pb
        .pb_lower = ((*mp).mp_pb.pb.pb_lower as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<indx_t>() as libc::c_ulong) as indx_t
        as indx_t;
    (*mp)
        .mp_pb
        .pb
        .pb_upper = ((*mp).mp_pb.pb.pb_upper as libc::c_uint).wrapping_add(sz) as indx_t
        as indx_t;
}
unsafe extern "C" fn mdb_node_shrink(mut mp: *mut MDB_page, mut indx: indx_t) {
    let mut node: *mut MDB_node = 0 as *mut MDB_node;
    let mut sp: *mut MDB_page = 0 as *mut MDB_page;
    let mut xp: *mut MDB_page = 0 as *mut MDB_page;
    let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut delta: indx_t = 0;
    let mut nsize: indx_t = 0;
    let mut len: indx_t = 0;
    let mut ptr: indx_t = 0;
    let mut i: libc::c_int = 0;
    node = (mp as *mut libc::c_char)
        .offset(
            *((*mp).mp_ptrs).as_mut_ptr().offset(indx as isize) as libc::c_int as isize,
        )
        .offset(
            (if 0 as libc::c_int != 0 {
                16 as libc::c_ulong as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        ) as *mut MDB_node;
    sp = ((*node).mn_data).as_mut_ptr().offset((*node).mn_ksize as libc::c_int as isize)
        as *mut libc::c_void as *mut MDB_page;
    delta = ((*sp).mp_pb.pb.pb_upper as libc::c_int
        - (*sp).mp_pb.pb.pb_lower as libc::c_int) as indx_t;
    nsize = ((*node).mn_lo as libc::c_uint
        | ((*node).mn_hi as libc::c_uint) << 16 as libc::c_int)
        .wrapping_sub(delta as libc::c_uint) as indx_t;
    if (*sp).mp_flags as libc::c_int & 0x20 as libc::c_int == 0x20 as libc::c_int {
        len = nsize;
        if nsize as libc::c_int & 1 as libc::c_int != 0 {
            return;
        }
    } else {
        xp = (sp as *mut libc::c_char).offset(delta as libc::c_int as isize)
            as *mut MDB_page;
        i = (((*sp).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                (16 as libc::c_ulong as libc::c_uint)
                    .wrapping_sub(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }),
                    ),
            ) >> 1 as libc::c_int) as libc::c_int;
        loop {
            i -= 1;
            if !(i >= 0 as libc::c_int) {
                break;
            }
            *((*xp).mp_ptrs)
                .as_mut_ptr()
                .offset(
                    i as isize,
                ) = (*((*sp).mp_ptrs).as_mut_ptr().offset(i as isize) as libc::c_int
                - delta as libc::c_int) as indx_t;
        }
        len = 16 as libc::c_ulong as libc::c_uint as indx_t;
    }
    (*sp).mp_pb.pb.pb_upper = (*sp).mp_pb.pb.pb_lower;
    (*sp).mp_p.p_pgno = (*mp).mp_p.p_pgno;
    (*node).mn_lo = (nsize as libc::c_int & 0xffff as libc::c_int) as libc::c_ushort;
    (*node).mn_hi = (nsize as libc::c_int >> 16 as libc::c_int) as libc::c_ushort;
    base = (mp as *mut libc::c_char)
        .offset((*mp).mp_pb.pb.pb_upper as libc::c_int as isize)
        .offset(
            (if 0 as libc::c_int != 0 {
                16 as libc::c_ulong as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        );
    memmove(
        base.offset(delta as libc::c_int as isize) as *mut libc::c_void,
        base as *const libc::c_void,
        (sp as *mut libc::c_char).offset(len as libc::c_int as isize).offset_from(base)
            as libc::c_long as libc::c_ulong,
    );
    ptr = *((*mp).mp_ptrs).as_mut_ptr().offset(indx as isize);
    i = (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
        .wrapping_sub(
            (16 as libc::c_ulong as libc::c_uint)
                .wrapping_sub(
                    (if 0 as libc::c_int != 0 {
                        16 as libc::c_ulong as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                ),
        ) >> 1 as libc::c_int) as libc::c_int;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        if *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize) as libc::c_int
            <= ptr as libc::c_int
        {
            let ref mut fresh33 = *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize);
            *fresh33 = (*fresh33 as libc::c_int + delta as libc::c_int) as indx_t;
        }
    }
    (*mp)
        .mp_pb
        .pb
        .pb_upper = ((*mp).mp_pb.pb.pb_upper as libc::c_int + delta as libc::c_int)
        as indx_t;
}
unsafe extern "C" fn mdb_xcursor_init0(mut mc: *mut MDB_cursor) {
    let mut mx: *mut MDB_xcursor = (*mc).mc_xcursor;
    (*mx).mx_cursor.mc_xcursor = 0 as *mut MDB_xcursor;
    (*mx).mx_cursor.mc_txn = (*mc).mc_txn;
    (*mx).mx_cursor.mc_db = &mut (*mx).mx_db;
    (*mx).mx_cursor.mc_dbx = &mut (*mx).mx_dbx;
    (*mx).mx_cursor.mc_dbi = (*mc).mc_dbi;
    (*mx).mx_cursor.mc_dbflag = &mut (*mx).mx_dbflag;
    (*mx).mx_cursor.mc_snum = 0 as libc::c_int as libc::c_ushort;
    (*mx).mx_cursor.mc_top = 0 as libc::c_int as libc::c_ushort;
    (*mx)
        .mx_cursor
        .mc_flags = 0x4 as libc::c_int as libc::c_uint
        | (*mc).mc_flags
            & (0x20000 as libc::c_int | 0x80000 as libc::c_int) as libc::c_uint;
    (*mx).mx_dbx.md_name.mv_size = 0 as libc::c_int as size_t;
    (*mx).mx_dbx.md_name.mv_data = 0 as *mut libc::c_void;
    (*mx).mx_dbx.md_cmp = (*(*mc).mc_dbx).md_dcmp;
    (*mx).mx_dbx.md_dcmp = None;
    (*mx).mx_dbx.md_rel = (*(*mc).mc_dbx).md_rel;
}
unsafe extern "C" fn mdb_xcursor_init1(
    mut mc: *mut MDB_cursor,
    mut node: *mut MDB_node,
) {
    let mut mx: *mut MDB_xcursor = (*mc).mc_xcursor;
    (*mx).mx_cursor.mc_flags
        &= (0x4 as libc::c_int | 0x20000 as libc::c_int | 0x80000 as libc::c_int)
            as libc::c_uint;
    if (*node).mn_flags as libc::c_int & 0x2 as libc::c_int != 0 {
        memcpy(
            &mut (*mx).mx_db as *mut MDB_db as *mut libc::c_void,
            ((*node).mn_data)
                .as_mut_ptr()
                .offset((*node).mn_ksize as libc::c_int as isize) as *mut libc::c_void,
            ::std::mem::size_of::<MDB_db>() as libc::c_ulong,
        );
        (*mx).mx_cursor.mc_pg[0 as libc::c_int as usize] = 0 as *mut MDB_page;
        (*mx).mx_cursor.mc_snum = 0 as libc::c_int as libc::c_ushort;
        (*mx).mx_cursor.mc_top = 0 as libc::c_int as libc::c_ushort;
    } else {
        let mut fp: *mut MDB_page = ((*node).mn_data)
            .as_mut_ptr()
            .offset((*node).mn_ksize as libc::c_int as isize) as *mut libc::c_void
            as *mut MDB_page;
        (*mx).mx_db.md_pad = 0 as libc::c_int as uint32_t;
        (*mx).mx_db.md_flags = 0 as libc::c_int as uint16_t;
        (*mx).mx_db.md_depth = 1 as libc::c_int as uint16_t;
        (*mx).mx_db.md_branch_pages = 0 as libc::c_int as pgno_t;
        (*mx).mx_db.md_leaf_pages = 1 as libc::c_int as pgno_t;
        (*mx).mx_db.md_overflow_pages = 0 as libc::c_int as pgno_t;
        (*mx)
            .mx_db
            .md_entries = (((*fp).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                (16 as libc::c_ulong as libc::c_uint)
                    .wrapping_sub(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }),
                    ),
            ) >> 1 as libc::c_int) as mdb_size_t;
        (*mx).mx_db.md_root = (*fp).mp_p.p_pgno;
        (*mx).mx_cursor.mc_snum = 1 as libc::c_int as libc::c_ushort;
        (*mx).mx_cursor.mc_top = 0 as libc::c_int as libc::c_ushort;
        (*mx).mx_cursor.mc_flags |= 0x1 as libc::c_int as libc::c_uint;
        (*mx).mx_cursor.mc_pg[0 as libc::c_int as usize] = fp;
        (*mx).mx_cursor.mc_ki[0 as libc::c_int as usize] = 0 as libc::c_int as indx_t;
        if (*(*mc).mc_db).md_flags as libc::c_int & 0x10 as libc::c_int != 0 {
            (*mx).mx_db.md_flags = 0x10 as libc::c_int as uint16_t;
            (*mx).mx_db.md_pad = (*fp).mp_pad as uint32_t;
            if (*(*mc).mc_db).md_flags as libc::c_int & 0x20 as libc::c_int != 0 {
                (*mx)
                    .mx_db
                    .md_flags = ((*mx).mx_db.md_flags as libc::c_int
                    | 0x8 as libc::c_int) as uint16_t;
            }
        }
    }
    (*mx)
        .mx_dbflag = (0x8 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int)
        as libc::c_uchar;
    if ((2147483647 as libc::c_int as libc::c_uint)
        .wrapping_mul(2 as libc::c_uint)
        .wrapping_add(1 as libc::c_uint) as libc::c_ulong)
        < 18446744073709551615 as libc::c_ulong
        && (*mx).mx_dbx.md_cmp == Some(mdb_cmp_int as MDB_cmp_func)
        && (*mx).mx_db.md_pad as libc::c_ulong
            == ::std::mem::size_of::<mdb_size_t>() as libc::c_ulong
    {
        (*mx).mx_dbx.md_cmp = Some(mdb_cmp_long as MDB_cmp_func);
    }
}
unsafe extern "C" fn mdb_xcursor_init2(
    mut mc: *mut MDB_cursor,
    mut src_mx: *mut MDB_xcursor,
    mut new_dupdata: libc::c_int,
) {
    let mut mx: *mut MDB_xcursor = (*mc).mc_xcursor;
    if new_dupdata != 0 {
        (*mx).mx_cursor.mc_snum = 1 as libc::c_int as libc::c_ushort;
        (*mx).mx_cursor.mc_top = 0 as libc::c_int as libc::c_ushort;
        (*mx).mx_cursor.mc_flags |= 0x1 as libc::c_int as libc::c_uint;
        (*mx).mx_cursor.mc_ki[0 as libc::c_int as usize] = 0 as libc::c_int as indx_t;
        (*mx)
            .mx_dbflag = (0x8 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int)
            as libc::c_uchar;
        (*mx).mx_dbx.md_cmp = (*src_mx).mx_dbx.md_cmp;
    } else if (*mx).mx_cursor.mc_flags & 0x1 as libc::c_int as libc::c_uint == 0 {
        return
    }
    (*mx).mx_db = (*src_mx).mx_db;
    (*mx)
        .mx_cursor
        .mc_pg[0 as libc::c_int
        as usize] = (*src_mx).mx_cursor.mc_pg[0 as libc::c_int as usize];
}
unsafe extern "C" fn mdb_cursor_init(
    mut mc: *mut MDB_cursor,
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut mx: *mut MDB_xcursor,
) {
    (*mc).mc_next = 0 as *mut MDB_cursor;
    (*mc).mc_backup = 0 as *mut MDB_cursor;
    (*mc).mc_dbi = dbi;
    (*mc).mc_txn = txn;
    (*mc).mc_db = &mut *((*txn).mt_dbs).offset(dbi as isize) as *mut MDB_db;
    (*mc).mc_dbx = &mut *((*txn).mt_dbxs).offset(dbi as isize) as *mut MDB_dbx;
    (*mc)
        .mc_dbflag = &mut *((*txn).mt_dbflags).offset(dbi as isize)
        as *mut libc::c_uchar;
    (*mc).mc_snum = 0 as libc::c_int as libc::c_ushort;
    (*mc).mc_top = 0 as libc::c_int as libc::c_ushort;
    (*mc).mc_pg[0 as libc::c_int as usize] = 0 as *mut MDB_page;
    (*mc).mc_ki[0 as libc::c_int as usize] = 0 as libc::c_int as indx_t;
    (*mc)
        .mc_flags = (*txn).mt_flags
        & (0x20000 as libc::c_int | 0x80000 as libc::c_int) as libc::c_uint;
    if (*((*txn).mt_dbs).offset(dbi as isize)).md_flags as libc::c_int
        & 0x4 as libc::c_int != 0
    {
        if !mx.is_null() {} else {
            mdb_assert_fail(
                (*txn).mt_env,
                b"mx != NULL\0" as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"mdb_cursor_init\0"))
                    .as_ptr(),
                b"mdb.c\0" as *const u8 as *const libc::c_char,
                8643 as libc::c_int,
            );
        };
        (*mc).mc_xcursor = mx;
        mdb_xcursor_init0(mc);
    } else {
        (*mc).mc_xcursor = 0 as *mut MDB_xcursor;
    }
    if *(*mc).mc_dbflag as libc::c_int & 0x2 as libc::c_int != 0 {
        mdb_page_search(mc, 0 as *mut MDB_val, 2 as libc::c_int);
    }
}
pub unsafe extern "C" fn mdb_cursor_open(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut ret: *mut *mut MDB_cursor,
) -> libc::c_int {
    let mut mc: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut size: size_t = ::std::mem::size_of::<MDB_cursor>() as libc::c_ulong;
    if ret.is_null()
        || !(!txn.is_null() && dbi < (*txn).mt_numdbs
            && *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int
                & 0x8 as libc::c_int != 0)
    {
        return 22 as libc::c_int;
    }
    if (*txn).mt_flags
        & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
        != 0
    {
        return -(30782 as libc::c_int);
    }
    if dbi == 0 as libc::c_int as libc::c_uint
        && !((*txn).mt_flags & 0x20000 as libc::c_int as libc::c_uint
            == 0x20000 as libc::c_int as libc::c_uint)
    {
        return 22 as libc::c_int;
    }
    if (*((*txn).mt_dbs).offset(dbi as isize)).md_flags as libc::c_int
        & 0x4 as libc::c_int != 0
    {
        size = (size as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<MDB_xcursor>() as libc::c_ulong)
            as size_t as size_t;
    }
    mc = malloc(size) as *mut MDB_cursor;
    if !mc.is_null() {
        mdb_cursor_init(
            mc,
            txn,
            dbi,
            mc.offset(1 as libc::c_int as isize) as *mut MDB_xcursor,
        );
        if !((*txn).mt_cursors).is_null() {
            (*mc).mc_next = *((*txn).mt_cursors).offset(dbi as isize);
            let ref mut fresh34 = *((*txn).mt_cursors).offset(dbi as isize);
            *fresh34 = mc;
            (*mc).mc_flags |= 0x40 as libc::c_int as libc::c_uint;
        }
    } else {
        return 12 as libc::c_int
    }
    *ret = mc;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_cursor_renew(
    mut txn: *mut MDB_txn,
    mut mc: *mut MDB_cursor,
) -> libc::c_int {
    if mc.is_null()
        || !(!txn.is_null() && (*mc).mc_dbi < (*txn).mt_numdbs
            && *((*txn).mt_dbflags).offset((*mc).mc_dbi as isize) as libc::c_int
                & 0x8 as libc::c_int != 0)
    {
        return 22 as libc::c_int;
    }
    if (*mc).mc_flags & 0x40 as libc::c_int as libc::c_uint != 0
        || !((*txn).mt_cursors).is_null()
    {
        return 22 as libc::c_int;
    }
    if (*txn).mt_flags
        & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
        != 0
    {
        return -(30782 as libc::c_int);
    }
    mdb_cursor_init(mc, txn, (*mc).mc_dbi, (*mc).mc_xcursor);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_cursor_count(
    mut mc: *mut MDB_cursor,
    mut countp: *mut mdb_size_t,
) -> libc::c_int {
    let mut leaf: *mut MDB_node = 0 as *mut MDB_node;
    if mc.is_null() || countp.is_null() {
        return 22 as libc::c_int;
    }
    if ((*mc).mc_xcursor).is_null() {
        return -(30784 as libc::c_int);
    }
    if (*(*mc).mc_txn).mt_flags
        & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
        != 0
    {
        return -(30782 as libc::c_int);
    }
    if (*mc).mc_flags & 0x1 as libc::c_int as libc::c_uint == 0 {
        return 22 as libc::c_int;
    }
    if (*mc).mc_snum == 0 {
        return -(30798 as libc::c_int);
    }
    if (*mc).mc_flags & 0x2 as libc::c_int as libc::c_uint != 0 {
        if (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint
            >= ((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_lower as libc::c_uint)
                .wrapping_sub(
                    (16 as libc::c_ulong as libc::c_uint)
                        .wrapping_sub(
                            (if 0 as libc::c_int != 0 {
                                16 as libc::c_ulong as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }),
                        ),
                ) >> 1 as libc::c_int
        {
            return -(30798 as libc::c_int);
        }
        (*mc).mc_flags ^= 0x2 as libc::c_int as libc::c_uint;
    }
    leaf = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut libc::c_char)
        .offset(
            *((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_ptrs)
                .as_mut_ptr()
                .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize) as libc::c_int
                as isize,
        )
        .offset(
            (if 0 as libc::c_int != 0 {
                16 as libc::c_ulong as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        ) as *mut MDB_node;
    if !((*leaf).mn_flags as libc::c_int & 0x4 as libc::c_int == 0x4 as libc::c_int) {
        *countp = 1 as libc::c_int as mdb_size_t;
    } else {
        if (*(*mc).mc_xcursor).mx_cursor.mc_flags & 0x1 as libc::c_int as libc::c_uint
            == 0
        {
            return 22 as libc::c_int;
        }
        *countp = (*(*mc).mc_xcursor).mx_db.md_entries;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_cursor_close(mut mc: *mut MDB_cursor) {
    !mc.is_null();
    if !mc.is_null() && ((*mc).mc_backup).is_null() {
        if (*mc).mc_flags & 0x40 as libc::c_int as libc::c_uint != 0
            && !((*(*mc).mc_txn).mt_cursors).is_null()
        {
            let mut prev: *mut *mut MDB_cursor = &mut *((*(*mc).mc_txn).mt_cursors)
                .offset((*mc).mc_dbi as isize) as *mut *mut MDB_cursor;
            while !(*prev).is_null() && *prev != mc {
                prev = &mut (**prev).mc_next;
            }
            if *prev == mc {
                *prev = (*mc).mc_next;
            }
        }
        free(mc as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn mdb_cursor_txn(mut mc: *mut MDB_cursor) -> *mut MDB_txn {
    if mc.is_null() {
        return 0 as *mut MDB_txn;
    }
    return (*mc).mc_txn;
}
pub unsafe extern "C" fn mdb_cursor_dbi(mut mc: *mut MDB_cursor) -> MDB_dbi {
    return (*mc).mc_dbi;
}
unsafe extern "C" fn mdb_update_key(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
) -> libc::c_int {
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut node: *mut MDB_node = 0 as *mut MDB_node;
    let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut delta: libc::c_int = 0;
    let mut ksize: libc::c_int = 0;
    let mut oksize: libc::c_int = 0;
    let mut ptr: indx_t = 0;
    let mut i: indx_t = 0;
    let mut numkeys: indx_t = 0;
    let mut indx: indx_t = 0;
    indx = (*mc).mc_ki[(*mc).mc_top as usize];
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    node = (mp as *mut libc::c_char)
        .offset(
            *((*mp).mp_ptrs).as_mut_ptr().offset(indx as isize) as libc::c_int as isize,
        )
        .offset(
            (if 0 as libc::c_int != 0 {
                16 as libc::c_ulong as libc::c_uint
            } else {
                0 as libc::c_int as libc::c_uint
            }) as isize,
        ) as *mut MDB_node;
    ptr = *((*mp).mp_ptrs).as_mut_ptr().offset(indx as isize);
    ksize = (((*key).mv_size).wrapping_add(1 as libc::c_uint as libc::c_ulong)
        & -(2 as libc::c_int) as libc::c_ulong) as libc::c_int;
    oksize = (((*node).mn_ksize as libc::c_uint).wrapping_add(1 as libc::c_uint)
        & -(2 as libc::c_int) as libc::c_uint) as libc::c_int;
    delta = ksize - oksize;
    if delta != 0 {
        if delta > 0 as libc::c_int
            && (((*mp).mp_pb.pb.pb_upper as libc::c_int
                - (*mp).mp_pb.pb.pb_lower as libc::c_int) as indx_t as libc::c_int)
                < delta
        {
            let mut pgno: pgno_t = 0;
            pgno = (*node).mn_lo as libc::c_ulong
                | ((*node).mn_hi as pgno_t) << 16 as libc::c_int
                | (if (if -(1 as libc::c_int) as pgno_t
                    > 0xffffffff as libc::c_uint as libc::c_ulong
                {
                    32 as libc::c_int
                } else {
                    0 as libc::c_int
                }) != 0
                {
                    ((*node).mn_flags as pgno_t)
                        << (if -(1 as libc::c_int) as pgno_t
                            > 0xffffffff as libc::c_uint as libc::c_ulong
                        {
                            32 as libc::c_int
                        } else {
                            0 as libc::c_int
                        })
                } else {
                    0 as libc::c_int as libc::c_ulong
                });
            mdb_node_del(mc, 0 as libc::c_int);
            return mdb_page_split(
                mc,
                key,
                0 as *mut MDB_val,
                pgno,
                0x40000 as libc::c_int as libc::c_uint,
            );
        }
        numkeys = (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                (16 as libc::c_ulong as libc::c_uint)
                    .wrapping_sub(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }),
                    ),
            ) >> 1 as libc::c_int) as indx_t;
        i = 0 as libc::c_int as indx_t;
        while (i as libc::c_int) < numkeys as libc::c_int {
            if *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize) as libc::c_int
                <= ptr as libc::c_int
            {
                let ref mut fresh35 = *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize);
                *fresh35 = (*fresh35 as libc::c_int - delta) as indx_t;
            }
            i = i.wrapping_add(1);
            i;
        }
        base = (mp as *mut libc::c_char)
            .offset((*mp).mp_pb.pb.pb_upper as libc::c_int as isize)
            .offset(
                (if 0 as libc::c_int != 0 {
                    16 as libc::c_ulong as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }) as isize,
            );
        len = ((ptr as libc::c_int - (*mp).mp_pb.pb.pb_upper as libc::c_int)
            as libc::c_ulong)
            .wrapping_add(8 as libc::c_ulong);
        memmove(
            base.offset(-(delta as isize)) as *mut libc::c_void,
            base as *const libc::c_void,
            len,
        );
        (*mp)
            .mp_pb
            .pb
            .pb_upper = ((*mp).mp_pb.pb.pb_upper as libc::c_int - delta) as indx_t;
        node = (mp as *mut libc::c_char)
            .offset(
                *((*mp).mp_ptrs).as_mut_ptr().offset(indx as isize) as libc::c_int
                    as isize,
            )
            .offset(
                (if 0 as libc::c_int != 0 {
                    16 as libc::c_ulong as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }) as isize,
            ) as *mut MDB_node;
    }
    if (*node).mn_ksize as libc::c_ulong != (*key).mv_size {
        (*node).mn_ksize = (*key).mv_size as libc::c_ushort;
    }
    if (*key).mv_size != 0 {
        memcpy(
            ((*node).mn_data).as_mut_ptr() as *mut libc::c_void,
            (*key).mv_data,
            (*key).mv_size,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_node_move(
    mut csrc: *mut MDB_cursor,
    mut cdst: *mut MDB_cursor,
    mut fromleft: libc::c_int,
) -> libc::c_int {
    let mut srcnode: *mut MDB_node = 0 as *mut MDB_node;
    let mut key: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut data: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut srcpg: pgno_t = 0;
    let mut mn: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut rc: libc::c_int = 0;
    let mut flags: libc::c_ushort = 0;
    rc = mdb_page_touch(csrc);
    if rc != 0
        || {
            rc = mdb_page_touch(cdst);
            rc != 0
        }
    {
        return rc;
    }
    if (*(*csrc).mc_pg[(*csrc).mc_top as usize]).mp_flags as libc::c_int
        & 0x20 as libc::c_int == 0x20 as libc::c_int
    {
        key.mv_size = (*(*csrc).mc_db).md_pad as size_t;
        key
            .mv_data = ((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_char)
            .offset(16 as libc::c_ulong as libc::c_uint as isize)
            .offset(
                ((*csrc).mc_ki[(*csrc).mc_top as usize] as libc::c_ulong)
                    .wrapping_mul(key.mv_size) as isize,
            ) as *mut libc::c_void;
        data.mv_size = 0 as libc::c_int as size_t;
        data.mv_data = 0 as *mut libc::c_void;
        srcpg = 0 as libc::c_int as pgno_t;
        flags = 0 as libc::c_int as libc::c_ushort;
    } else {
        srcnode = ((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_char)
            .offset(
                *((*(*csrc).mc_pg[(*csrc).mc_top as usize]).mp_ptrs)
                    .as_mut_ptr()
                    .offset((*csrc).mc_ki[(*csrc).mc_top as usize] as isize)
                    as libc::c_int as isize,
            )
            .offset(
                (if 0 as libc::c_int != 0 {
                    16 as libc::c_ulong as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }) as isize,
            ) as *mut MDB_node;
        if srcnode as size_t & 1 as libc::c_int as libc::c_ulong == 0 {} else {
            mdb_assert_fail(
                (*(*csrc).mc_txn).mt_env,
                b"!((size_t)srcnode & 1)\0" as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"mdb_node_move\0"))
                    .as_ptr(),
                b"mdb.c\0" as *const u8 as *const libc::c_char,
                8899 as libc::c_int,
            );
        };
        srcpg = (*srcnode).mn_lo as libc::c_ulong
            | ((*srcnode).mn_hi as pgno_t) << 16 as libc::c_int
            | (if (if -(1 as libc::c_int) as pgno_t
                > 0xffffffff as libc::c_uint as libc::c_ulong
            {
                32 as libc::c_int
            } else {
                0 as libc::c_int
            }) != 0
            {
                ((*srcnode).mn_flags as pgno_t)
                    << (if -(1 as libc::c_int) as pgno_t
                        > 0xffffffff as libc::c_uint as libc::c_ulong
                    {
                        32 as libc::c_int
                    } else {
                        0 as libc::c_int
                    })
            } else {
                0 as libc::c_int as libc::c_ulong
            });
        flags = (*srcnode).mn_flags;
        if (*csrc).mc_ki[(*csrc).mc_top as usize] as libc::c_int == 0 as libc::c_int
            && (*(*csrc).mc_pg[(*csrc).mc_top as usize]).mp_flags as libc::c_int
                & 0x1 as libc::c_int == 0x1 as libc::c_int
        {
            let mut snum: libc::c_uint = (*csrc).mc_snum as libc::c_uint;
            let mut s2: *mut MDB_node = 0 as *mut MDB_node;
            rc = mdb_page_search_lowest(csrc);
            if rc != 0 {
                return rc;
            }
            if (*(*csrc).mc_pg[(*csrc).mc_top as usize]).mp_flags as libc::c_int
                & 0x20 as libc::c_int == 0x20 as libc::c_int
            {
                key.mv_size = (*(*csrc).mc_db).md_pad as size_t;
                key
                    .mv_data = ((*csrc).mc_pg[(*csrc).mc_top as usize]
                    as *mut libc::c_char)
                    .offset(16 as libc::c_ulong as libc::c_uint as isize)
                    .offset(
                        (0 as libc::c_int as libc::c_ulong).wrapping_mul(key.mv_size)
                            as isize,
                    ) as *mut libc::c_void;
            } else {
                s2 = ((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_char)
                    .offset(
                        *((*(*csrc).mc_pg[(*csrc).mc_top as usize]).mp_ptrs)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as libc::c_int as isize,
                    )
                    .offset(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                key.mv_size = (*s2).mn_ksize as size_t;
                key.mv_data = ((*s2).mn_data).as_mut_ptr() as *mut libc::c_void;
            }
            let fresh36 = snum;
            snum = snum.wrapping_sub(1);
            (*csrc).mc_snum = fresh36 as libc::c_ushort;
            (*csrc).mc_top = snum as libc::c_ushort;
        } else {
            key.mv_size = (*srcnode).mn_ksize as size_t;
            key.mv_data = ((*srcnode).mn_data).as_mut_ptr() as *mut libc::c_void;
        }
        data
            .mv_size = ((*srcnode).mn_lo as libc::c_uint
            | ((*srcnode).mn_hi as libc::c_uint) << 16 as libc::c_int) as size_t;
        data
            .mv_data = ((*srcnode).mn_data)
            .as_mut_ptr()
            .offset((*srcnode).mn_ksize as libc::c_int as isize) as *mut libc::c_void;
    }
    mn.mc_xcursor = 0 as *mut MDB_xcursor;
    if (*(*cdst).mc_pg[(*cdst).mc_top as usize]).mp_flags as libc::c_int
        & 0x1 as libc::c_int == 0x1 as libc::c_int
        && (*cdst).mc_ki[(*cdst).mc_top as usize] as libc::c_int == 0 as libc::c_int
    {
        let mut snum_0: libc::c_uint = (*cdst).mc_snum as libc::c_uint;
        let mut s2_0: *mut MDB_node = 0 as *mut MDB_node;
        let mut bkey: MDB_val = MDB_val {
            mv_size: 0,
            mv_data: 0 as *mut libc::c_void,
        };
        mdb_cursor_copy(cdst, &mut mn);
        rc = mdb_page_search_lowest(&mut mn);
        if rc != 0 {
            return rc;
        }
        if (*mn.mc_pg[mn.mc_top as usize]).mp_flags as libc::c_int & 0x20 as libc::c_int
            == 0x20 as libc::c_int
        {
            bkey.mv_size = (*mn.mc_db).md_pad as size_t;
            bkey
                .mv_data = (mn.mc_pg[mn.mc_top as usize] as *mut libc::c_char)
                .offset(16 as libc::c_ulong as libc::c_uint as isize)
                .offset(
                    (0 as libc::c_int as libc::c_ulong).wrapping_mul(bkey.mv_size)
                        as isize,
                ) as *mut libc::c_void;
        } else {
            s2_0 = (mn.mc_pg[mn.mc_top as usize] as *mut libc::c_char)
                .offset(
                    *((*mn.mc_pg[mn.mc_top as usize]).mp_ptrs)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as libc::c_int as isize,
                )
                .offset(
                    (if 0 as libc::c_int != 0 {
                        16 as libc::c_ulong as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }) as isize,
                ) as *mut MDB_node;
            bkey.mv_size = (*s2_0).mn_ksize as size_t;
            bkey.mv_data = ((*s2_0).mn_data).as_mut_ptr() as *mut libc::c_void;
        }
        let fresh37 = snum_0;
        snum_0 = snum_0.wrapping_sub(1);
        mn.mc_snum = fresh37 as libc::c_ushort;
        mn.mc_top = snum_0 as libc::c_ushort;
        mn.mc_ki[snum_0 as usize] = 0 as libc::c_int as indx_t;
        rc = mdb_update_key(&mut mn, &mut bkey);
        if rc != 0 {
            return rc;
        }
    }
    rc = mdb_node_add(
        cdst,
        (*cdst).mc_ki[(*cdst).mc_top as usize],
        &mut key,
        &mut data,
        srcpg,
        flags as libc::c_uint,
    );
    if rc != 0 as libc::c_int {
        return rc;
    }
    mdb_node_del(csrc, key.mv_size as libc::c_int);
    let mut m2: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut m3: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut dbi: MDB_dbi = (*csrc).mc_dbi;
    let mut mpd: *mut MDB_page = 0 as *mut MDB_page;
    let mut mps: *mut MDB_page = 0 as *mut MDB_page;
    mps = (*csrc).mc_pg[(*csrc).mc_top as usize];
    if fromleft != 0 {
        mpd = (*cdst).mc_pg[(*csrc).mc_top as usize];
        m2 = *((*(*csrc).mc_txn).mt_cursors).offset(dbi as isize);
        while !m2.is_null() {
            if (*csrc).mc_flags & 0x4 as libc::c_int as libc::c_uint != 0 {
                m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
            } else {
                m3 = m2;
            }
            if !((*m3).mc_flags & 0x1 as libc::c_int as libc::c_uint == 0
                || ((*m3).mc_top as libc::c_int) < (*csrc).mc_top as libc::c_int)
            {
                if m3 != cdst && (*m3).mc_pg[(*csrc).mc_top as usize] == mpd
                    && (*m3).mc_ki[(*csrc).mc_top as usize] as libc::c_int
                        >= (*cdst).mc_ki[(*csrc).mc_top as usize] as libc::c_int
                {
                    (*m3)
                        .mc_ki[(*csrc).mc_top
                        as usize] = ((*m3).mc_ki[(*csrc).mc_top as usize])
                        .wrapping_add(1);
                    (*m3).mc_ki[(*csrc).mc_top as usize];
                }
                if m3 != csrc && (*m3).mc_pg[(*csrc).mc_top as usize] == mps
                    && (*m3).mc_ki[(*csrc).mc_top as usize] as libc::c_int
                        == (*csrc).mc_ki[(*csrc).mc_top as usize] as libc::c_int
                {
                    (*m3)
                        .mc_pg[(*csrc).mc_top
                        as usize] = (*cdst).mc_pg[(*cdst).mc_top as usize];
                    (*m3)
                        .mc_ki[(*csrc).mc_top
                        as usize] = (*cdst).mc_ki[(*cdst).mc_top as usize];
                    (*m3)
                        .mc_ki[((*csrc).mc_top as libc::c_int - 1 as libc::c_int)
                        as usize] = ((*m3)
                        .mc_ki[((*csrc).mc_top as libc::c_int - 1 as libc::c_int)
                        as usize])
                        .wrapping_add(1);
                    (*m3)
                        .mc_ki[((*csrc).mc_top as libc::c_int - 1 as libc::c_int)
                        as usize];
                }
                if (*mps).mp_flags as libc::c_int & 0x2 as libc::c_int
                    == 0x2 as libc::c_int
                {
                    let mut xr_pg: *mut MDB_page = (*m3).mc_pg[(*csrc).mc_top as usize];
                    let mut xr_node: *mut MDB_node = 0 as *mut MDB_node;
                    if !(!(!((*m3).mc_xcursor).is_null()
                        && (*(*m3).mc_xcursor).mx_cursor.mc_flags
                            & 0x1 as libc::c_int as libc::c_uint != 0)
                        || (*m3).mc_ki[(*csrc).mc_top as usize] as libc::c_uint
                            >= ((*xr_pg).mp_pb.pb.pb_lower as libc::c_uint)
                                .wrapping_sub(
                                    (16 as libc::c_ulong as libc::c_uint)
                                        .wrapping_sub(
                                            (if 0 as libc::c_int != 0 {
                                                16 as libc::c_ulong as libc::c_uint
                                            } else {
                                                0 as libc::c_int as libc::c_uint
                                            }),
                                        ),
                                ) >> 1 as libc::c_int)
                    {
                        xr_node = (xr_pg as *mut libc::c_char)
                            .offset(
                                *((*xr_pg).mp_ptrs)
                                    .as_mut_ptr()
                                    .offset((*m3).mc_ki[(*csrc).mc_top as usize] as isize)
                                    as libc::c_int as isize,
                            )
                            .offset(
                                (if 0 as libc::c_int != 0 {
                                    16 as libc::c_ulong as libc::c_uint
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }) as isize,
                            ) as *mut MDB_node;
                        if (*xr_node).mn_flags as libc::c_int
                            & (0x4 as libc::c_int | 0x2 as libc::c_int)
                            == 0x4 as libc::c_int
                        {
                            (*(*m3).mc_xcursor)
                                .mx_cursor
                                .mc_pg[0 as libc::c_int
                                as usize] = ((*xr_node).mn_data)
                                .as_mut_ptr()
                                .offset((*xr_node).mn_ksize as libc::c_int as isize)
                                as *mut libc::c_void as *mut MDB_page;
                        }
                    }
                }
            }
            m2 = (*m2).mc_next;
        }
    } else {
        m2 = *((*(*csrc).mc_txn).mt_cursors).offset(dbi as isize);
        while !m2.is_null() {
            if (*csrc).mc_flags & 0x4 as libc::c_int as libc::c_uint != 0 {
                m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
            } else {
                m3 = m2;
            }
            if !(m3 == csrc) {
                if !((*m3).mc_flags & 0x1 as libc::c_int as libc::c_uint == 0
                    || ((*m3).mc_top as libc::c_int) < (*csrc).mc_top as libc::c_int)
                {
                    if (*m3).mc_pg[(*csrc).mc_top as usize] == mps {
                        if (*m3).mc_ki[(*csrc).mc_top as usize] == 0 {
                            (*m3)
                                .mc_pg[(*csrc).mc_top
                                as usize] = (*cdst).mc_pg[(*cdst).mc_top as usize];
                            (*m3)
                                .mc_ki[(*csrc).mc_top
                                as usize] = (*cdst).mc_ki[(*cdst).mc_top as usize];
                            (*m3)
                                .mc_ki[((*csrc).mc_top as libc::c_int - 1 as libc::c_int)
                                as usize] = ((*m3)
                                .mc_ki[((*csrc).mc_top as libc::c_int - 1 as libc::c_int)
                                as usize])
                                .wrapping_sub(1);
                            (*m3)
                                .mc_ki[((*csrc).mc_top as libc::c_int - 1 as libc::c_int)
                                as usize];
                        } else {
                            (*m3)
                                .mc_ki[(*csrc).mc_top
                                as usize] = ((*m3).mc_ki[(*csrc).mc_top as usize])
                                .wrapping_sub(1);
                            (*m3).mc_ki[(*csrc).mc_top as usize];
                        }
                        if (*mps).mp_flags as libc::c_int & 0x2 as libc::c_int
                            == 0x2 as libc::c_int
                        {
                            let mut xr_pg_0: *mut MDB_page = (*m3)
                                .mc_pg[(*csrc).mc_top as usize];
                            let mut xr_node_0: *mut MDB_node = 0 as *mut MDB_node;
                            if !(!(!((*m3).mc_xcursor).is_null()
                                && (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                    & 0x1 as libc::c_int as libc::c_uint != 0)
                                || (*m3).mc_ki[(*csrc).mc_top as usize] as libc::c_uint
                                    >= ((*xr_pg_0).mp_pb.pb.pb_lower as libc::c_uint)
                                        .wrapping_sub(
                                            (16 as libc::c_ulong as libc::c_uint)
                                                .wrapping_sub(
                                                    (if 0 as libc::c_int != 0 {
                                                        16 as libc::c_ulong as libc::c_uint
                                                    } else {
                                                        0 as libc::c_int as libc::c_uint
                                                    }),
                                                ),
                                        ) >> 1 as libc::c_int)
                            {
                                xr_node_0 = (xr_pg_0 as *mut libc::c_char)
                                    .offset(
                                        *((*xr_pg_0).mp_ptrs)
                                            .as_mut_ptr()
                                            .offset((*m3).mc_ki[(*csrc).mc_top as usize] as isize)
                                            as libc::c_int as isize,
                                    )
                                    .offset(
                                        (if 0 as libc::c_int != 0 {
                                            16 as libc::c_ulong as libc::c_uint
                                        } else {
                                            0 as libc::c_int as libc::c_uint
                                        }) as isize,
                                    ) as *mut MDB_node;
                                if (*xr_node_0).mn_flags as libc::c_int
                                    & (0x4 as libc::c_int | 0x2 as libc::c_int)
                                    == 0x4 as libc::c_int
                                {
                                    (*(*m3).mc_xcursor)
                                        .mx_cursor
                                        .mc_pg[0 as libc::c_int
                                        as usize] = ((*xr_node_0).mn_data)
                                        .as_mut_ptr()
                                        .offset((*xr_node_0).mn_ksize as libc::c_int as isize)
                                        as *mut libc::c_void as *mut MDB_page;
                                }
                            }
                        }
                    }
                }
            }
            m2 = (*m2).mc_next;
        }
    }
    if (*csrc).mc_ki[(*csrc).mc_top as usize] as libc::c_int == 0 as libc::c_int {
        if (*csrc).mc_ki[((*csrc).mc_top as libc::c_int - 1 as libc::c_int) as usize]
            as libc::c_int != 0 as libc::c_int
        {
            if (*(*csrc).mc_pg[(*csrc).mc_top as usize]).mp_flags as libc::c_int
                & 0x20 as libc::c_int == 0x20 as libc::c_int
            {
                key
                    .mv_data = ((*csrc).mc_pg[(*csrc).mc_top as usize]
                    as *mut libc::c_char)
                    .offset(16 as libc::c_ulong as libc::c_uint as isize)
                    .offset(
                        (0 as libc::c_int as libc::c_ulong).wrapping_mul(key.mv_size)
                            as isize,
                    ) as *mut libc::c_void;
            } else {
                srcnode = ((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut libc::c_char)
                    .offset(
                        *((*(*csrc).mc_pg[(*csrc).mc_top as usize]).mp_ptrs)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as libc::c_int as isize,
                    )
                    .offset(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                key.mv_size = (*srcnode).mn_ksize as size_t;
                key.mv_data = ((*srcnode).mn_data).as_mut_ptr() as *mut libc::c_void;
            }
            mdb_cursor_copy(csrc, &mut mn);
            mn.mc_snum = (mn.mc_snum).wrapping_sub(1);
            mn.mc_snum;
            mn.mc_top = (mn.mc_top).wrapping_sub(1);
            mn.mc_top;
            let mut dummy: MDB_cursor = MDB_cursor {
                mc_next: 0 as *mut MDB_cursor,
                mc_backup: 0 as *mut MDB_cursor,
                mc_xcursor: 0 as *mut MDB_xcursor,
                mc_txn: 0 as *mut MDB_txn,
                mc_dbi: 0,
                mc_db: 0 as *mut MDB_db,
                mc_dbx: 0 as *mut MDB_dbx,
                mc_dbflag: 0 as *mut libc::c_uchar,
                mc_snum: 0,
                mc_top: 0,
                mc_flags: 0,
                mc_pg: [0 as *mut MDB_page; 32],
                mc_ki: [0; 32],
            };
            let mut tracked: *mut MDB_cursor = 0 as *mut MDB_cursor;
            let mut tp: *mut *mut MDB_cursor = &mut *((*mn.mc_txn).mt_cursors)
                .offset(mn.mc_dbi as isize) as *mut *mut MDB_cursor;
            if mn.mc_flags & 0x4 as libc::c_int as libc::c_uint != 0 {
                dummy.mc_flags = 0x1 as libc::c_int as libc::c_uint;
                dummy.mc_xcursor = &mut mn as *mut MDB_cursor as *mut MDB_xcursor;
                tracked = &mut dummy;
            } else {
                tracked = &mut mn;
            }
            (*tracked).mc_next = *tp;
            *tp = tracked;
            rc = mdb_update_key(&mut mn, &mut key);
            *tp = (*tracked).mc_next;
            if rc != 0 {
                return rc;
            }
        }
        if (*(*csrc).mc_pg[(*csrc).mc_top as usize]).mp_flags as libc::c_int
            & 0x1 as libc::c_int == 0x1 as libc::c_int
        {
            let mut nullkey: MDB_val = MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            };
            let mut ix: indx_t = (*csrc).mc_ki[(*csrc).mc_top as usize];
            nullkey.mv_size = 0 as libc::c_int as size_t;
            (*csrc).mc_ki[(*csrc).mc_top as usize] = 0 as libc::c_int as indx_t;
            rc = mdb_update_key(csrc, &mut nullkey);
            (*csrc).mc_ki[(*csrc).mc_top as usize] = ix;
            if rc == 0 as libc::c_int {} else {
                mdb_assert_fail(
                    (*(*csrc).mc_txn).mt_env,
                    b"rc == MDB_SUCCESS\0" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<
                        &[u8; 14],
                        &[libc::c_char; 14],
                    >(b"mdb_node_move\0"))
                        .as_ptr(),
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    9056 as libc::c_int,
                );
            };
        }
    }
    if (*cdst).mc_ki[(*cdst).mc_top as usize] as libc::c_int == 0 as libc::c_int {
        if (*cdst).mc_ki[((*cdst).mc_top as libc::c_int - 1 as libc::c_int) as usize]
            as libc::c_int != 0 as libc::c_int
        {
            if (*(*csrc).mc_pg[(*csrc).mc_top as usize]).mp_flags as libc::c_int
                & 0x20 as libc::c_int == 0x20 as libc::c_int
            {
                key
                    .mv_data = ((*cdst).mc_pg[(*cdst).mc_top as usize]
                    as *mut libc::c_char)
                    .offset(16 as libc::c_ulong as libc::c_uint as isize)
                    .offset(
                        (0 as libc::c_int as libc::c_ulong).wrapping_mul(key.mv_size)
                            as isize,
                    ) as *mut libc::c_void;
            } else {
                srcnode = ((*cdst).mc_pg[(*cdst).mc_top as usize] as *mut libc::c_char)
                    .offset(
                        *((*(*cdst).mc_pg[(*cdst).mc_top as usize]).mp_ptrs)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as libc::c_int as isize,
                    )
                    .offset(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                key.mv_size = (*srcnode).mn_ksize as size_t;
                key.mv_data = ((*srcnode).mn_data).as_mut_ptr() as *mut libc::c_void;
            }
            mdb_cursor_copy(cdst, &mut mn);
            mn.mc_snum = (mn.mc_snum).wrapping_sub(1);
            mn.mc_snum;
            mn.mc_top = (mn.mc_top).wrapping_sub(1);
            mn.mc_top;
            let mut dummy_0: MDB_cursor = MDB_cursor {
                mc_next: 0 as *mut MDB_cursor,
                mc_backup: 0 as *mut MDB_cursor,
                mc_xcursor: 0 as *mut MDB_xcursor,
                mc_txn: 0 as *mut MDB_txn,
                mc_dbi: 0,
                mc_db: 0 as *mut MDB_db,
                mc_dbx: 0 as *mut MDB_dbx,
                mc_dbflag: 0 as *mut libc::c_uchar,
                mc_snum: 0,
                mc_top: 0,
                mc_flags: 0,
                mc_pg: [0 as *mut MDB_page; 32],
                mc_ki: [0; 32],
            };
            let mut tracked_0: *mut MDB_cursor = 0 as *mut MDB_cursor;
            let mut tp_0: *mut *mut MDB_cursor = &mut *((*mn.mc_txn).mt_cursors)
                .offset(mn.mc_dbi as isize) as *mut *mut MDB_cursor;
            if mn.mc_flags & 0x4 as libc::c_int as libc::c_uint != 0 {
                dummy_0.mc_flags = 0x1 as libc::c_int as libc::c_uint;
                dummy_0.mc_xcursor = &mut mn as *mut MDB_cursor as *mut MDB_xcursor;
                tracked_0 = &mut dummy_0;
            } else {
                tracked_0 = &mut mn;
            }
            (*tracked_0).mc_next = *tp_0;
            *tp_0 = tracked_0;
            rc = mdb_update_key(&mut mn, &mut key);
            *tp_0 = (*tracked_0).mc_next;
            if rc != 0 {
                return rc;
            }
        }
        if (*(*cdst).mc_pg[(*cdst).mc_top as usize]).mp_flags as libc::c_int
            & 0x1 as libc::c_int == 0x1 as libc::c_int
        {
            let mut nullkey_0: MDB_val = MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            };
            let mut ix_0: indx_t = (*cdst).mc_ki[(*cdst).mc_top as usize];
            nullkey_0.mv_size = 0 as libc::c_int as size_t;
            (*cdst).mc_ki[(*cdst).mc_top as usize] = 0 as libc::c_int as indx_t;
            rc = mdb_update_key(cdst, &mut nullkey_0);
            (*cdst).mc_ki[(*cdst).mc_top as usize] = ix_0;
            if rc == 0 as libc::c_int {} else {
                mdb_assert_fail(
                    (*(*cdst).mc_txn).mt_env,
                    b"rc == MDB_SUCCESS\0" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<
                        &[u8; 14],
                        &[libc::c_char; 14],
                    >(b"mdb_node_move\0"))
                        .as_ptr(),
                    b"mdb.c\0" as *const u8 as *const libc::c_char,
                    9087 as libc::c_int,
                );
            };
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_page_merge(
    mut csrc: *mut MDB_cursor,
    mut cdst: *mut MDB_cursor,
) -> libc::c_int {
    let mut psrc: *mut MDB_page = 0 as *mut MDB_page;
    let mut pdst: *mut MDB_page = 0 as *mut MDB_page;
    let mut srcnode: *mut MDB_node = 0 as *mut MDB_node;
    let mut key: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut data: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut nkeys: libc::c_uint = 0;
    let mut rc: libc::c_int = 0;
    let mut i: indx_t = 0;
    let mut j: indx_t = 0;
    psrc = (*csrc).mc_pg[(*csrc).mc_top as usize];
    pdst = (*cdst).mc_pg[(*cdst).mc_top as usize];
    if (*csrc).mc_snum as libc::c_int > 1 as libc::c_int {} else {
        mdb_assert_fail(
            (*(*csrc).mc_txn).mt_env,
            b"csrc->mc_snum > 1\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"mdb_page_merge\0"))
                .as_ptr(),
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            9117 as libc::c_int,
        );
    };
    if (*cdst).mc_snum as libc::c_int > 1 as libc::c_int {} else {
        mdb_assert_fail(
            (*(*csrc).mc_txn).mt_env,
            b"cdst->mc_snum > 1\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"mdb_page_merge\0"))
                .as_ptr(),
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            9118 as libc::c_int,
        );
    };
    rc = mdb_page_touch(cdst);
    if rc != 0 {
        return rc;
    }
    pdst = (*cdst).mc_pg[(*cdst).mc_top as usize];
    nkeys = ((*pdst).mp_pb.pb.pb_lower as libc::c_uint)
        .wrapping_sub(
            (16 as libc::c_ulong as libc::c_uint)
                .wrapping_sub(
                    (if 0 as libc::c_int != 0 {
                        16 as libc::c_ulong as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                ),
        ) >> 1 as libc::c_int;
    j = nkeys as indx_t;
    if (*psrc).mp_flags as libc::c_int & 0x20 as libc::c_int == 0x20 as libc::c_int {
        key.mv_size = (*(*csrc).mc_db).md_pad as size_t;
        key
            .mv_data = (psrc as *mut libc::c_char)
            .offset(16 as libc::c_ulong as libc::c_uint as isize) as *mut libc::c_void;
        i = 0 as libc::c_int as indx_t;
        while (i as libc::c_uint)
            < ((*psrc).mp_pb.pb.pb_lower as libc::c_uint)
                .wrapping_sub(
                    (16 as libc::c_ulong as libc::c_uint)
                        .wrapping_sub(
                            (if 0 as libc::c_int != 0 {
                                16 as libc::c_ulong as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }),
                        ),
                ) >> 1 as libc::c_int
        {
            rc = mdb_node_add(
                cdst,
                j,
                &mut key,
                0 as *mut MDB_val,
                0 as libc::c_int as pgno_t,
                0 as libc::c_int as libc::c_uint,
            );
            if rc != 0 as libc::c_int {
                return rc;
            }
            key
                .mv_data = (key.mv_data as *mut libc::c_char)
                .offset(key.mv_size as isize) as *mut libc::c_void;
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_add(1);
            j;
        }
    } else {
        i = 0 as libc::c_int as indx_t;
        while (i as libc::c_uint)
            < ((*psrc).mp_pb.pb.pb_lower as libc::c_uint)
                .wrapping_sub(
                    (16 as libc::c_ulong as libc::c_uint)
                        .wrapping_sub(
                            (if 0 as libc::c_int != 0 {
                                16 as libc::c_ulong as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }),
                        ),
                ) >> 1 as libc::c_int
        {
            srcnode = (psrc as *mut libc::c_char)
                .offset(
                    *((*psrc).mp_ptrs).as_mut_ptr().offset(i as isize) as libc::c_int
                        as isize,
                )
                .offset(
                    (if 0 as libc::c_int != 0 {
                        16 as libc::c_ulong as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }) as isize,
                ) as *mut MDB_node;
            if i as libc::c_int == 0 as libc::c_int
                && (*psrc).mp_flags as libc::c_int & 0x1 as libc::c_int
                    == 0x1 as libc::c_int
            {
                let mut mn: MDB_cursor = MDB_cursor {
                    mc_next: 0 as *mut MDB_cursor,
                    mc_backup: 0 as *mut MDB_cursor,
                    mc_xcursor: 0 as *mut MDB_xcursor,
                    mc_txn: 0 as *mut MDB_txn,
                    mc_dbi: 0,
                    mc_db: 0 as *mut MDB_db,
                    mc_dbx: 0 as *mut MDB_dbx,
                    mc_dbflag: 0 as *mut libc::c_uchar,
                    mc_snum: 0,
                    mc_top: 0,
                    mc_flags: 0,
                    mc_pg: [0 as *mut MDB_page; 32],
                    mc_ki: [0; 32],
                };
                let mut s2: *mut MDB_node = 0 as *mut MDB_node;
                mdb_cursor_copy(csrc, &mut mn);
                mn.mc_xcursor = 0 as *mut MDB_xcursor;
                rc = mdb_page_search_lowest(&mut mn);
                if rc != 0 {
                    return rc;
                }
                if (*mn.mc_pg[mn.mc_top as usize]).mp_flags as libc::c_int
                    & 0x20 as libc::c_int == 0x20 as libc::c_int
                {
                    key.mv_size = (*mn.mc_db).md_pad as size_t;
                    key
                        .mv_data = (mn.mc_pg[mn.mc_top as usize] as *mut libc::c_char)
                        .offset(16 as libc::c_ulong as libc::c_uint as isize)
                        .offset(
                            (0 as libc::c_int as libc::c_ulong).wrapping_mul(key.mv_size)
                                as isize,
                        ) as *mut libc::c_void;
                } else {
                    s2 = (mn.mc_pg[mn.mc_top as usize] as *mut libc::c_char)
                        .offset(
                            *((*mn.mc_pg[mn.mc_top as usize]).mp_ptrs)
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize) as libc::c_int as isize,
                        )
                        .offset(
                            (if 0 as libc::c_int != 0 {
                                16 as libc::c_ulong as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) as isize,
                        ) as *mut MDB_node;
                    key.mv_size = (*s2).mn_ksize as size_t;
                    key.mv_data = ((*s2).mn_data).as_mut_ptr() as *mut libc::c_void;
                }
            } else {
                key.mv_size = (*srcnode).mn_ksize as size_t;
                key.mv_data = ((*srcnode).mn_data).as_mut_ptr() as *mut libc::c_void;
            }
            data
                .mv_size = ((*srcnode).mn_lo as libc::c_uint
                | ((*srcnode).mn_hi as libc::c_uint) << 16 as libc::c_int) as size_t;
            data
                .mv_data = ((*srcnode).mn_data)
                .as_mut_ptr()
                .offset((*srcnode).mn_ksize as libc::c_int as isize)
                as *mut libc::c_void;
            rc = mdb_node_add(
                cdst,
                j,
                &mut key,
                &mut data,
                (*srcnode).mn_lo as libc::c_ulong
                    | ((*srcnode).mn_hi as pgno_t) << 16 as libc::c_int
                    | (if (if -(1 as libc::c_int) as pgno_t
                        > 0xffffffff as libc::c_uint as libc::c_ulong
                    {
                        32 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) != 0
                    {
                        ((*srcnode).mn_flags as pgno_t)
                            << (if -(1 as libc::c_int) as pgno_t
                                > 0xffffffff as libc::c_uint as libc::c_ulong
                            {
                                32 as libc::c_int
                            } else {
                                0 as libc::c_int
                            })
                    } else {
                        0 as libc::c_int as libc::c_ulong
                    }),
                (*srcnode).mn_flags as libc::c_uint,
            );
            if rc != 0 as libc::c_int {
                return rc;
            }
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_add(1);
            j;
        }
    }
    (*csrc).mc_top = ((*csrc).mc_top).wrapping_sub(1);
    (*csrc).mc_top;
    mdb_node_del(csrc, 0 as libc::c_int);
    if (*csrc).mc_ki[(*csrc).mc_top as usize] as libc::c_int == 0 as libc::c_int {
        key.mv_size = 0 as libc::c_int as size_t;
        rc = mdb_update_key(csrc, &mut key);
        if rc != 0 {
            (*csrc).mc_top = ((*csrc).mc_top).wrapping_add(1);
            (*csrc).mc_top;
            return rc;
        }
    }
    (*csrc).mc_top = ((*csrc).mc_top).wrapping_add(1);
    (*csrc).mc_top;
    psrc = (*csrc).mc_pg[(*csrc).mc_top as usize];
    rc = mdb_page_loose(csrc, psrc);
    if rc != 0 {
        return rc;
    }
    if (*psrc).mp_flags as libc::c_int & 0x2 as libc::c_int == 0x2 as libc::c_int {
        (*(*csrc).mc_db)
            .md_leaf_pages = ((*(*csrc).mc_db).md_leaf_pages).wrapping_sub(1);
        (*(*csrc).mc_db).md_leaf_pages;
    } else {
        (*(*csrc).mc_db)
            .md_branch_pages = ((*(*csrc).mc_db).md_branch_pages).wrapping_sub(1);
        (*(*csrc).mc_db).md_branch_pages;
    }
    let mut m2: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut m3: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut dbi: MDB_dbi = (*csrc).mc_dbi;
    let mut top: libc::c_uint = (*csrc).mc_top as libc::c_uint;
    m2 = *((*(*csrc).mc_txn).mt_cursors).offset(dbi as isize);
    while !m2.is_null() {
        if (*csrc).mc_flags & 0x4 as libc::c_int as libc::c_uint != 0 {
            m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
        } else {
            m3 = m2;
        }
        if !(m3 == csrc) {
            if !(((*m3).mc_snum as libc::c_int) < (*csrc).mc_snum as libc::c_int) {
                if (*m3).mc_pg[top as usize] == psrc {
                    (*m3).mc_pg[top as usize] = pdst;
                    (*m3)
                        .mc_ki[top
                        as usize] = ((*m3).mc_ki[top as usize] as libc::c_uint)
                        .wrapping_add(nkeys) as indx_t as indx_t;
                    (*m3)
                        .mc_ki[top.wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as usize] = (*cdst)
                        .mc_ki[top.wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as usize];
                } else if (*m3)
                    .mc_pg[top.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize]
                    == (*csrc)
                        .mc_pg[top.wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as usize]
                    && (*m3)
                        .mc_ki[top.wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as usize] as libc::c_int
                        > (*csrc)
                            .mc_ki[top.wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as usize] as libc::c_int
                {
                    (*m3)
                        .mc_ki[top.wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as usize] = ((*m3)
                        .mc_ki[top.wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as usize])
                        .wrapping_sub(1);
                    (*m3)
                        .mc_ki[top.wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as usize];
                }
                if (*psrc).mp_flags as libc::c_int & 0x2 as libc::c_int
                    == 0x2 as libc::c_int
                {
                    let mut xr_pg: *mut MDB_page = (*m3).mc_pg[top as usize];
                    let mut xr_node: *mut MDB_node = 0 as *mut MDB_node;
                    if !(!(!((*m3).mc_xcursor).is_null()
                        && (*(*m3).mc_xcursor).mx_cursor.mc_flags
                            & 0x1 as libc::c_int as libc::c_uint != 0)
                        || (*m3).mc_ki[top as usize] as libc::c_uint
                            >= ((*xr_pg).mp_pb.pb.pb_lower as libc::c_uint)
                                .wrapping_sub(
                                    (16 as libc::c_ulong as libc::c_uint)
                                        .wrapping_sub(
                                            (if 0 as libc::c_int != 0 {
                                                16 as libc::c_ulong as libc::c_uint
                                            } else {
                                                0 as libc::c_int as libc::c_uint
                                            }),
                                        ),
                                ) >> 1 as libc::c_int)
                    {
                        xr_node = (xr_pg as *mut libc::c_char)
                            .offset(
                                *((*xr_pg).mp_ptrs)
                                    .as_mut_ptr()
                                    .offset((*m3).mc_ki[top as usize] as isize) as libc::c_int
                                    as isize,
                            )
                            .offset(
                                (if 0 as libc::c_int != 0 {
                                    16 as libc::c_ulong as libc::c_uint
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }) as isize,
                            ) as *mut MDB_node;
                        if (*xr_node).mn_flags as libc::c_int
                            & (0x4 as libc::c_int | 0x2 as libc::c_int)
                            == 0x4 as libc::c_int
                        {
                            (*(*m3).mc_xcursor)
                                .mx_cursor
                                .mc_pg[0 as libc::c_int
                                as usize] = ((*xr_node).mn_data)
                                .as_mut_ptr()
                                .offset((*xr_node).mn_ksize as libc::c_int as isize)
                                as *mut libc::c_void as *mut MDB_page;
                        }
                    }
                }
            }
        }
        m2 = (*m2).mc_next;
    }
    let mut snum: libc::c_uint = (*cdst).mc_snum as libc::c_uint;
    let mut depth: uint16_t = (*(*cdst).mc_db).md_depth;
    mdb_cursor_pop(cdst);
    rc = mdb_rebalance(cdst);
    if depth as libc::c_int != (*(*cdst).mc_db).md_depth as libc::c_int {
        snum = snum
            .wrapping_add(
                ((*(*cdst).mc_db).md_depth as libc::c_int - depth as libc::c_int)
                    as libc::c_uint,
            );
    }
    (*cdst).mc_snum = snum as libc::c_ushort;
    (*cdst)
        .mc_top = snum.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ushort;
    return rc;
}
unsafe extern "C" fn mdb_cursor_copy(
    mut csrc: *const MDB_cursor,
    mut cdst: *mut MDB_cursor,
) {
    let mut i: libc::c_uint = 0;
    (*cdst).mc_txn = (*csrc).mc_txn;
    (*cdst).mc_dbi = (*csrc).mc_dbi;
    (*cdst).mc_db = (*csrc).mc_db;
    (*cdst).mc_dbx = (*csrc).mc_dbx;
    (*cdst).mc_snum = (*csrc).mc_snum;
    (*cdst).mc_top = (*csrc).mc_top;
    (*cdst).mc_flags = (*csrc).mc_flags;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*csrc).mc_snum as libc::c_uint {
        (*cdst).mc_pg[i as usize] = (*csrc).mc_pg[i as usize];
        (*cdst).mc_ki[i as usize] = (*csrc).mc_ki[i as usize];
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn mdb_rebalance(mut mc: *mut MDB_cursor) -> libc::c_int {
    let mut node: *mut MDB_node = 0 as *mut MDB_node;
    let mut rc: libc::c_int = 0;
    let mut fromleft: libc::c_int = 0;
    let mut ptop: libc::c_uint = 0;
    let mut minkeys: libc::c_uint = 0;
    let mut thresh: libc::c_uint = 0;
    let mut mn: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut oldki: indx_t = 0;
    if (*(*mc).mc_pg[(*mc).mc_top as usize]).mp_flags as libc::c_int & 0x1 as libc::c_int
        == 0x1 as libc::c_int
    {
        minkeys = 2 as libc::c_int as libc::c_uint;
        thresh = 1 as libc::c_int as libc::c_uint;
    } else {
        minkeys = 1 as libc::c_int as libc::c_uint;
        thresh = 250 as libc::c_int as libc::c_uint;
    }
    if 1000 as libc::c_long
        * ((*(*(*mc).mc_txn).mt_env).me_psize)
            .wrapping_sub(16 as libc::c_ulong as libc::c_uint)
            .wrapping_sub(
                ((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_upper as libc::c_int
                    - (*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_lower
                        as libc::c_int) as indx_t as libc::c_uint,
            ) as libc::c_long
        / ((*(*(*mc).mc_txn).mt_env).me_psize)
            .wrapping_sub(16 as libc::c_ulong as libc::c_uint) as libc::c_long
        >= thresh as libc::c_long
        && ((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                (16 as libc::c_ulong as libc::c_uint)
                    .wrapping_sub(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }),
                    ),
            ) >> 1 as libc::c_int >= minkeys
    {
        return 0 as libc::c_int;
    }
    if ((*mc).mc_snum as libc::c_int) < 2 as libc::c_int {
        let mut mp: *mut MDB_page = (*mc).mc_pg[0 as libc::c_int as usize];
        if (*mp).mp_flags as libc::c_int & 0x40 as libc::c_int == 0x40 as libc::c_int {
            return 0 as libc::c_int;
        }
        if ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                (16 as libc::c_ulong as libc::c_uint)
                    .wrapping_sub(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }),
                    ),
            ) >> 1 as libc::c_int == 0 as libc::c_int as libc::c_uint
        {
            (*(*mc).mc_db).md_root = !(0 as libc::c_int as pgno_t);
            (*(*mc).mc_db).md_depth = 0 as libc::c_int as uint16_t;
            (*(*mc).mc_db).md_leaf_pages = 0 as libc::c_int as pgno_t;
            rc = mdb_midl_append(&mut (*(*mc).mc_txn).mt_free_pgs, (*mp).mp_p.p_pgno);
            if rc != 0 {
                return rc;
            }
            (*mc).mc_snum = 0 as libc::c_int as libc::c_ushort;
            (*mc).mc_top = 0 as libc::c_int as libc::c_ushort;
            (*mc).mc_flags &= !(0x1 as libc::c_int) as libc::c_uint;
            let mut m2: *mut MDB_cursor = 0 as *mut MDB_cursor;
            let mut m3: *mut MDB_cursor = 0 as *mut MDB_cursor;
            let mut dbi: MDB_dbi = (*mc).mc_dbi;
            m2 = *((*(*mc).mc_txn).mt_cursors).offset(dbi as isize);
            while !m2.is_null() {
                if (*mc).mc_flags & 0x4 as libc::c_int as libc::c_uint != 0 {
                    m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
                } else {
                    m3 = m2;
                }
                if !((*m3).mc_flags & 0x1 as libc::c_int as libc::c_uint == 0
                    || ((*m3).mc_snum as libc::c_int) < (*mc).mc_snum as libc::c_int)
                {
                    if (*m3).mc_pg[0 as libc::c_int as usize] == mp {
                        (*m3).mc_snum = 0 as libc::c_int as libc::c_ushort;
                        (*m3).mc_top = 0 as libc::c_int as libc::c_ushort;
                        (*m3).mc_flags &= !(0x1 as libc::c_int) as libc::c_uint;
                    }
                }
                m2 = (*m2).mc_next;
            }
        } else if (*mp).mp_flags as libc::c_int & 0x1 as libc::c_int
            == 0x1 as libc::c_int
            && ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                .wrapping_sub(
                    (16 as libc::c_ulong as libc::c_uint)
                        .wrapping_sub(
                            (if 0 as libc::c_int != 0 {
                                16 as libc::c_ulong as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }),
                        ),
                ) >> 1 as libc::c_int == 1 as libc::c_int as libc::c_uint
        {
            let mut i: libc::c_int = 0;
            rc = mdb_midl_append(&mut (*(*mc).mc_txn).mt_free_pgs, (*mp).mp_p.p_pgno);
            if rc != 0 {
                return rc;
            }
            (*(*mc).mc_db)
                .md_root = (*((mp as *mut libc::c_char)
                .offset(
                    *((*mp).mp_ptrs).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as libc::c_int as isize,
                )
                .offset(
                    (if 0 as libc::c_int != 0 {
                        16 as libc::c_ulong as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }) as isize,
                ) as *mut MDB_node))
                .mn_lo as libc::c_ulong
                | ((*((mp as *mut libc::c_char)
                    .offset(
                        *((*mp).mp_ptrs).as_mut_ptr().offset(0 as libc::c_int as isize)
                            as libc::c_int as isize,
                    )
                    .offset(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }) as isize,
                    ) as *mut MDB_node))
                    .mn_hi as pgno_t) << 16 as libc::c_int
                | (if (if -(1 as libc::c_int) as pgno_t
                    > 0xffffffff as libc::c_uint as libc::c_ulong
                {
                    32 as libc::c_int
                } else {
                    0 as libc::c_int
                }) != 0
                {
                    ((*((mp as *mut libc::c_char)
                        .offset(
                            *((*mp).mp_ptrs)
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize) as libc::c_int as isize,
                        )
                        .offset(
                            (if 0 as libc::c_int != 0 {
                                16 as libc::c_ulong as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) as isize,
                        ) as *mut MDB_node))
                        .mn_flags as pgno_t)
                        << (if -(1 as libc::c_int) as pgno_t
                            > 0xffffffff as libc::c_uint as libc::c_ulong
                        {
                            32 as libc::c_int
                        } else {
                            0 as libc::c_int
                        })
                } else {
                    0 as libc::c_int as libc::c_ulong
                });
            rc = mdb_page_get(
                mc,
                (*(*mc).mc_db).md_root,
                &mut *((*mc).mc_pg).as_mut_ptr().offset(0 as libc::c_int as isize),
                0 as *mut libc::c_int,
            );
            if rc != 0 {
                return rc;
            }
            (*(*mc).mc_db).md_depth = ((*(*mc).mc_db).md_depth).wrapping_sub(1);
            (*(*mc).mc_db).md_depth;
            (*(*mc).mc_db)
                .md_branch_pages = ((*(*mc).mc_db).md_branch_pages).wrapping_sub(1);
            (*(*mc).mc_db).md_branch_pages;
            (*mc)
                .mc_ki[0 as libc::c_int
                as usize] = (*mc).mc_ki[1 as libc::c_int as usize];
            i = 1 as libc::c_int;
            while i < (*(*mc).mc_db).md_depth as libc::c_int {
                (*mc).mc_pg[i as usize] = (*mc).mc_pg[(i + 1 as libc::c_int) as usize];
                (*mc).mc_ki[i as usize] = (*mc).mc_ki[(i + 1 as libc::c_int) as usize];
                i += 1;
                i;
            }
            let mut m2_0: *mut MDB_cursor = 0 as *mut MDB_cursor;
            let mut m3_0: *mut MDB_cursor = 0 as *mut MDB_cursor;
            let mut dbi_0: MDB_dbi = (*mc).mc_dbi;
            m2_0 = *((*(*mc).mc_txn).mt_cursors).offset(dbi_0 as isize);
            while !m2_0.is_null() {
                if (*mc).mc_flags & 0x4 as libc::c_int as libc::c_uint != 0 {
                    m3_0 = &mut (*(*m2_0).mc_xcursor).mx_cursor;
                } else {
                    m3_0 = m2_0;
                }
                if !(m3_0 == mc) {
                    if !((*m3_0).mc_flags & 0x1 as libc::c_int as libc::c_uint == 0) {
                        if (*m3_0).mc_pg[0 as libc::c_int as usize] == mp {
                            i = 0 as libc::c_int;
                            while i < (*(*mc).mc_db).md_depth as libc::c_int {
                                (*m3_0)
                                    .mc_pg[i
                                    as usize] = (*m3_0).mc_pg[(i + 1 as libc::c_int) as usize];
                                (*m3_0)
                                    .mc_ki[i
                                    as usize] = (*m3_0).mc_ki[(i + 1 as libc::c_int) as usize];
                                i += 1;
                                i;
                            }
                            (*m3_0).mc_snum = ((*m3_0).mc_snum).wrapping_sub(1);
                            (*m3_0).mc_snum;
                            (*m3_0).mc_top = ((*m3_0).mc_top).wrapping_sub(1);
                            (*m3_0).mc_top;
                        }
                    }
                }
                m2_0 = (*m2_0).mc_next;
            }
        }
        return 0 as libc::c_int;
    }
    ptop = ((*mc).mc_top as libc::c_int - 1 as libc::c_int) as libc::c_uint;
    if ((*(*mc).mc_pg[ptop as usize]).mp_pb.pb.pb_lower as libc::c_uint)
        .wrapping_sub(
            (16 as libc::c_ulong as libc::c_uint)
                .wrapping_sub(
                    (if 0 as libc::c_int != 0 {
                        16 as libc::c_ulong as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                ),
        ) >> 1 as libc::c_int > 1 as libc::c_int as libc::c_uint
    {} else {
        mdb_assert_fail(
            (*(*mc).mc_txn).mt_env,
            b"NUMKEYS(mc->mc_pg[ptop]) > 1\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"mdb_rebalance\0"))
                .as_ptr(),
            b"mdb.c\0" as *const u8 as *const libc::c_char,
            9382 as libc::c_int,
        );
    };
    mdb_cursor_copy(mc, &mut mn);
    mn.mc_xcursor = 0 as *mut MDB_xcursor;
    oldki = (*mc).mc_ki[(*mc).mc_top as usize];
    if (*mc).mc_ki[ptop as usize] as libc::c_int == 0 as libc::c_int {
        mn.mc_ki[ptop as usize] = (mn.mc_ki[ptop as usize]).wrapping_add(1);
        mn.mc_ki[ptop as usize];
        node = ((*mc).mc_pg[ptop as usize] as *mut libc::c_char)
            .offset(
                *((*(*mc).mc_pg[ptop as usize]).mp_ptrs)
                    .as_mut_ptr()
                    .offset(mn.mc_ki[ptop as usize] as isize) as libc::c_int as isize,
            )
            .offset(
                (if 0 as libc::c_int != 0 {
                    16 as libc::c_ulong as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }) as isize,
            ) as *mut MDB_node;
        rc = mdb_page_get(
            mc,
            (*node).mn_lo as libc::c_ulong
                | ((*node).mn_hi as pgno_t) << 16 as libc::c_int
                | (if (if -(1 as libc::c_int) as pgno_t
                    > 0xffffffff as libc::c_uint as libc::c_ulong
                {
                    32 as libc::c_int
                } else {
                    0 as libc::c_int
                }) != 0
                {
                    ((*node).mn_flags as pgno_t)
                        << (if -(1 as libc::c_int) as pgno_t
                            > 0xffffffff as libc::c_uint as libc::c_ulong
                        {
                            32 as libc::c_int
                        } else {
                            0 as libc::c_int
                        })
                } else {
                    0 as libc::c_int as libc::c_ulong
                }),
            &mut *(mn.mc_pg).as_mut_ptr().offset(mn.mc_top as isize),
            0 as *mut libc::c_int,
        );
        if rc != 0 {
            return rc;
        }
        mn.mc_ki[mn.mc_top as usize] = 0 as libc::c_int as indx_t;
        (*mc)
            .mc_ki[(*mc).mc_top
            as usize] = (((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_pb.pb.pb_lower
            as libc::c_uint)
            .wrapping_sub(
                (16 as libc::c_ulong as libc::c_uint)
                    .wrapping_sub(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }),
                    ),
            ) >> 1 as libc::c_int) as indx_t;
        fromleft = 0 as libc::c_int;
    } else {
        mn.mc_ki[ptop as usize] = (mn.mc_ki[ptop as usize]).wrapping_sub(1);
        mn.mc_ki[ptop as usize];
        node = ((*mc).mc_pg[ptop as usize] as *mut libc::c_char)
            .offset(
                *((*(*mc).mc_pg[ptop as usize]).mp_ptrs)
                    .as_mut_ptr()
                    .offset(mn.mc_ki[ptop as usize] as isize) as libc::c_int as isize,
            )
            .offset(
                (if 0 as libc::c_int != 0 {
                    16 as libc::c_ulong as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }) as isize,
            ) as *mut MDB_node;
        rc = mdb_page_get(
            mc,
            (*node).mn_lo as libc::c_ulong
                | ((*node).mn_hi as pgno_t) << 16 as libc::c_int
                | (if (if -(1 as libc::c_int) as pgno_t
                    > 0xffffffff as libc::c_uint as libc::c_ulong
                {
                    32 as libc::c_int
                } else {
                    0 as libc::c_int
                }) != 0
                {
                    ((*node).mn_flags as pgno_t)
                        << (if -(1 as libc::c_int) as pgno_t
                            > 0xffffffff as libc::c_uint as libc::c_ulong
                        {
                            32 as libc::c_int
                        } else {
                            0 as libc::c_int
                        })
                } else {
                    0 as libc::c_int as libc::c_ulong
                }),
            &mut *(mn.mc_pg).as_mut_ptr().offset(mn.mc_top as isize),
            0 as *mut libc::c_int,
        );
        if rc != 0 {
            return rc;
        }
        mn
            .mc_ki[mn.mc_top
            as usize] = (((*mn.mc_pg[mn.mc_top as usize]).mp_pb.pb.pb_lower
            as libc::c_uint)
            .wrapping_sub(
                (16 as libc::c_ulong as libc::c_uint)
                    .wrapping_sub(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }),
                    ),
            ) >> 1 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as indx_t;
        (*mc).mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
        fromleft = 1 as libc::c_int;
    }
    if 1000 as libc::c_long
        * ((*(*(*mc).mc_txn).mt_env).me_psize)
            .wrapping_sub(16 as libc::c_ulong as libc::c_uint)
            .wrapping_sub(
                ((*mn.mc_pg[mn.mc_top as usize]).mp_pb.pb.pb_upper as libc::c_int
                    - (*mn.mc_pg[mn.mc_top as usize]).mp_pb.pb.pb_lower as libc::c_int)
                    as indx_t as libc::c_uint,
            ) as libc::c_long
        / ((*(*(*mc).mc_txn).mt_env).me_psize)
            .wrapping_sub(16 as libc::c_ulong as libc::c_uint) as libc::c_long
        >= thresh as libc::c_long
        && ((*mn.mc_pg[mn.mc_top as usize]).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                (16 as libc::c_ulong as libc::c_uint)
                    .wrapping_sub(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }),
                    ),
            ) >> 1 as libc::c_int > minkeys
    {
        rc = mdb_node_move(&mut mn, mc, fromleft);
        if fromleft != 0 {
            oldki = oldki.wrapping_add(1);
            oldki;
        }
    } else {
        if fromleft == 0 {
            rc = mdb_page_merge(&mut mn, mc);
        } else {
            oldki = (oldki as libc::c_uint)
                .wrapping_add(
                    ((*mn.mc_pg[mn.mc_top as usize]).mp_pb.pb.pb_lower as libc::c_uint)
                        .wrapping_sub(
                            (16 as libc::c_ulong as libc::c_uint)
                                .wrapping_sub(
                                    (if 0 as libc::c_int != 0 {
                                        16 as libc::c_ulong as libc::c_uint
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    }),
                                ),
                        ) >> 1 as libc::c_int,
                ) as indx_t as indx_t;
            mn
                .mc_ki[mn.mc_top
                as usize] = (mn.mc_ki[mn.mc_top as usize] as libc::c_int
                + ((*mc).mc_ki[mn.mc_top as usize] as libc::c_int + 1 as libc::c_int))
                as indx_t;
            let mut dummy: MDB_cursor = MDB_cursor {
                mc_next: 0 as *mut MDB_cursor,
                mc_backup: 0 as *mut MDB_cursor,
                mc_xcursor: 0 as *mut MDB_xcursor,
                mc_txn: 0 as *mut MDB_txn,
                mc_dbi: 0,
                mc_db: 0 as *mut MDB_db,
                mc_dbx: 0 as *mut MDB_dbx,
                mc_dbflag: 0 as *mut libc::c_uchar,
                mc_snum: 0,
                mc_top: 0,
                mc_flags: 0,
                mc_pg: [0 as *mut MDB_page; 32],
                mc_ki: [0; 32],
            };
            let mut tracked: *mut MDB_cursor = 0 as *mut MDB_cursor;
            let mut tp: *mut *mut MDB_cursor = &mut *((*mn.mc_txn).mt_cursors)
                .offset(mn.mc_dbi as isize) as *mut *mut MDB_cursor;
            if mn.mc_flags & 0x4 as libc::c_int as libc::c_uint != 0 {
                dummy.mc_flags = 0x1 as libc::c_int as libc::c_uint;
                dummy.mc_xcursor = &mut mn as *mut MDB_cursor as *mut MDB_xcursor;
                tracked = &mut dummy;
            } else {
                tracked = &mut mn;
            }
            (*tracked).mc_next = *tp;
            *tp = tracked;
            rc = mdb_page_merge(mc, &mut mn);
            *tp = (*tracked).mc_next;
            mdb_cursor_copy(&mut mn, mc);
        }
        (*mc).mc_flags &= !(0x2 as libc::c_int) as libc::c_uint;
    }
    (*mc).mc_ki[(*mc).mc_top as usize] = oldki;
    return rc;
}
unsafe extern "C" fn mdb_cursor_del0(mut mc: *mut MDB_cursor) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut ki: indx_t = 0;
    let mut nkeys: libc::c_uint = 0;
    let mut m2: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut m3: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut dbi: MDB_dbi = (*mc).mc_dbi;
    ki = (*mc).mc_ki[(*mc).mc_top as usize];
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    mdb_node_del(mc, (*(*mc).mc_db).md_pad as libc::c_int);
    (*(*mc).mc_db).md_entries = ((*(*mc).mc_db).md_entries).wrapping_sub(1);
    (*(*mc).mc_db).md_entries;
    m2 = *((*(*mc).mc_txn).mt_cursors).offset(dbi as isize);
    while !m2.is_null() {
        m3 = if (*mc).mc_flags & 0x4 as libc::c_int as libc::c_uint != 0 {
            &mut (*(*m2).mc_xcursor).mx_cursor
        } else {
            m2
        };
        if !((*m2).mc_flags & (*m3).mc_flags & 0x1 as libc::c_int as libc::c_uint == 0) {
            if !(m3 == mc
                || ((*m3).mc_snum as libc::c_int) < (*mc).mc_snum as libc::c_int)
            {
                if (*m3).mc_pg[(*mc).mc_top as usize] == mp {
                    if (*m3).mc_ki[(*mc).mc_top as usize] as libc::c_int
                        == ki as libc::c_int
                    {
                        (*m3).mc_flags |= 0x8 as libc::c_int as libc::c_uint;
                        if (*(*mc).mc_db).md_flags as libc::c_int & 0x4 as libc::c_int
                            != 0
                        {
                            (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                &= !(0x1 as libc::c_int | 0x2 as libc::c_int)
                                    as libc::c_uint;
                        }
                    } else {
                        if (*m3).mc_ki[(*mc).mc_top as usize] as libc::c_int
                            > ki as libc::c_int
                        {
                            (*m3)
                                .mc_ki[(*mc).mc_top
                                as usize] = ((*m3).mc_ki[(*mc).mc_top as usize])
                                .wrapping_sub(1);
                            (*m3).mc_ki[(*mc).mc_top as usize];
                        }
                        let mut xr_pg: *mut MDB_page = mp;
                        let mut xr_node: *mut MDB_node = 0 as *mut MDB_node;
                        if !(!(!((*m3).mc_xcursor).is_null()
                            && (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                & 0x1 as libc::c_int as libc::c_uint != 0)
                            || (*m3).mc_ki[(*mc).mc_top as usize] as libc::c_uint
                                >= ((*xr_pg).mp_pb.pb.pb_lower as libc::c_uint)
                                    .wrapping_sub(
                                        (16 as libc::c_ulong as libc::c_uint)
                                            .wrapping_sub(
                                                (if 0 as libc::c_int != 0 {
                                                    16 as libc::c_ulong as libc::c_uint
                                                } else {
                                                    0 as libc::c_int as libc::c_uint
                                                }),
                                            ),
                                    ) >> 1 as libc::c_int)
                        {
                            xr_node = (xr_pg as *mut libc::c_char)
                                .offset(
                                    *((*xr_pg).mp_ptrs)
                                        .as_mut_ptr()
                                        .offset((*m3).mc_ki[(*mc).mc_top as usize] as isize)
                                        as libc::c_int as isize,
                                )
                                .offset(
                                    (if 0 as libc::c_int != 0 {
                                        16 as libc::c_ulong as libc::c_uint
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    }) as isize,
                                ) as *mut MDB_node;
                            if (*xr_node).mn_flags as libc::c_int
                                & (0x4 as libc::c_int | 0x2 as libc::c_int)
                                == 0x4 as libc::c_int
                            {
                                (*(*m3).mc_xcursor)
                                    .mx_cursor
                                    .mc_pg[0 as libc::c_int
                                    as usize] = ((*xr_node).mn_data)
                                    .as_mut_ptr()
                                    .offset((*xr_node).mn_ksize as libc::c_int as isize)
                                    as *mut libc::c_void as *mut MDB_page;
                            }
                        }
                    }
                }
            }
        }
        m2 = (*m2).mc_next;
    }
    rc = mdb_rebalance(mc);
    if !(rc != 0) {
        if (*mc).mc_snum == 0 {
            (*mc).mc_flags |= 0x2 as libc::c_int as libc::c_uint;
            return rc;
        }
        mp = (*mc).mc_pg[(*mc).mc_top as usize];
        nkeys = ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                (16 as libc::c_ulong as libc::c_uint)
                    .wrapping_sub(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }),
                    ),
            ) >> 1 as libc::c_int;
        m2 = *((*(*mc).mc_txn).mt_cursors).offset(dbi as isize);
        loop {
            if !(rc == 0 && !m2.is_null()) {
                current_block = 13281731871476506071;
                break;
            }
            m3 = if (*mc).mc_flags & 0x4 as libc::c_int as libc::c_uint != 0 {
                &mut (*(*m2).mc_xcursor).mx_cursor
            } else {
                m2
            };
            if !((*m2).mc_flags & (*m3).mc_flags & 0x1 as libc::c_int as libc::c_uint
                == 0)
            {
                if !(((*m3).mc_snum as libc::c_int) < (*mc).mc_snum as libc::c_int) {
                    if (*m3).mc_pg[(*mc).mc_top as usize] == mp {
                        if (*m3).mc_ki[(*mc).mc_top as usize] as libc::c_int
                            >= (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int
                        {
                            if (*m3).mc_ki[(*mc).mc_top as usize] as libc::c_uint
                                >= nkeys
                            {
                                rc = mdb_cursor_sibling(m3, 1 as libc::c_int);
                                if rc == -(30798 as libc::c_int) {
                                    (*m3).mc_flags |= 0x2 as libc::c_int as libc::c_uint;
                                    rc = 0 as libc::c_int;
                                    current_block = 2569451025026770673;
                                } else {
                                    if rc != 0 {
                                        current_block = 257654509287698822;
                                        break;
                                    }
                                    current_block = 18435049525520518667;
                                }
                            } else {
                                current_block = 18435049525520518667;
                            }
                            match current_block {
                                2569451025026770673 => {}
                                _ => {
                                    if !((*m3).mc_xcursor).is_null()
                                        && (*m3).mc_flags & 0x2 as libc::c_int as libc::c_uint == 0
                                    {
                                        let mut node: *mut MDB_node = ((*m3)
                                            .mc_pg[(*m3).mc_top as usize] as *mut libc::c_char)
                                            .offset(
                                                *((*(*m3).mc_pg[(*m3).mc_top as usize]).mp_ptrs)
                                                    .as_mut_ptr()
                                                    .offset((*m3).mc_ki[(*m3).mc_top as usize] as isize)
                                                    as libc::c_int as isize,
                                            )
                                            .offset(
                                                (if 0 as libc::c_int != 0 {
                                                    16 as libc::c_ulong as libc::c_uint
                                                } else {
                                                    0 as libc::c_int as libc::c_uint
                                                }) as isize,
                                            ) as *mut MDB_node;
                                        if (*node).mn_flags as libc::c_int & 0x4 as libc::c_int != 0
                                        {
                                            if (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                                & 0x1 as libc::c_int as libc::c_uint != 0
                                            {
                                                if (*node).mn_flags as libc::c_int & 0x2 as libc::c_int == 0
                                                {
                                                    (*(*m3).mc_xcursor)
                                                        .mx_cursor
                                                        .mc_pg[0 as libc::c_int
                                                        as usize] = ((*node).mn_data)
                                                        .as_mut_ptr()
                                                        .offset((*node).mn_ksize as libc::c_int as isize)
                                                        as *mut libc::c_void as *mut MDB_page;
                                                }
                                            } else {
                                                mdb_xcursor_init1(m3, node);
                                                rc = mdb_cursor_first(
                                                    &mut (*(*m3).mc_xcursor).mx_cursor,
                                                    0 as *mut MDB_val,
                                                    0 as *mut MDB_val,
                                                );
                                                if rc != 0 {
                                                    current_block = 257654509287698822;
                                                    break;
                                                }
                                            }
                                        }
                                        (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                            |= 0x8 as libc::c_int as libc::c_uint;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            m2 = (*m2).mc_next;
        }
        match current_block {
            257654509287698822 => {}
            _ => {
                (*mc).mc_flags |= 0x8 as libc::c_int as libc::c_uint;
            }
        }
    }
    if rc != 0 {
        (*(*mc).mc_txn).mt_flags |= 0x2 as libc::c_int as libc::c_uint;
    }
    return rc;
}
pub unsafe extern "C" fn mdb_del(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> libc::c_int {
    if key.is_null()
        || !(!txn.is_null() && dbi < (*txn).mt_numdbs
            && *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int
                & 0x10 as libc::c_int != 0)
    {
        return 22 as libc::c_int;
    }
    if (*txn).mt_flags
        & (0x20000 as libc::c_int
            | (0x1 as libc::c_int | 0x2 as libc::c_int | 0x10 as libc::c_int))
            as libc::c_uint != 0
    {
        return if (*txn).mt_flags & 0x20000 as libc::c_int as libc::c_uint != 0 {
            13 as libc::c_int
        } else {
            -(30782 as libc::c_int)
        };
    }
    if !((*((*txn).mt_dbs).offset(dbi as isize)).md_flags as libc::c_int
        & 0x4 as libc::c_int == 0x4 as libc::c_int)
    {
        data = 0 as *mut MDB_val;
    }
    return mdb_del0(txn, dbi, key, data, 0 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn mdb_del0(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut mc: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut mx: MDB_xcursor = MDB_xcursor {
        mx_cursor: MDB_cursor {
            mc_next: 0 as *mut MDB_cursor,
            mc_backup: 0 as *mut MDB_cursor,
            mc_xcursor: 0 as *mut MDB_xcursor,
            mc_txn: 0 as *mut MDB_txn,
            mc_dbi: 0,
            mc_db: 0 as *mut MDB_db,
            mc_dbx: 0 as *mut MDB_dbx,
            mc_dbflag: 0 as *mut libc::c_uchar,
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [0 as *mut MDB_page; 32],
            mc_ki: [0; 32],
        },
        mx_db: MDB_db {
            md_pad: 0,
            md_flags: 0,
            md_depth: 0,
            md_branch_pages: 0,
            md_leaf_pages: 0,
            md_overflow_pages: 0,
            md_entries: 0,
            md_root: 0,
        },
        mx_dbx: MDB_dbx {
            md_name: MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            },
            md_cmp: None,
            md_dcmp: None,
            md_rel: None,
            md_relctx: 0 as *mut libc::c_void,
        },
        mx_dbflag: 0,
    };
    let mut op: MDB_cursor_op = MDB_FIRST;
    let mut rdata: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut xdata: *mut MDB_val = 0 as *mut MDB_val;
    let mut rc: libc::c_int = 0;
    let mut exact: libc::c_int = 0 as libc::c_int;
    mdb_cursor_init(&mut mc, txn, dbi, &mut mx);
    if !data.is_null() {
        op = MDB_GET_BOTH;
        rdata = *data;
        xdata = &mut rdata;
    } else {
        op = MDB_SET;
        xdata = 0 as *mut MDB_val;
        flags |= 0x20 as libc::c_int as libc::c_uint;
    }
    rc = mdb_cursor_set(&mut mc, key, xdata, op, &mut exact);
    if rc == 0 as libc::c_int {
        mc.mc_next = *((*txn).mt_cursors).offset(dbi as isize);
        let ref mut fresh38 = *((*txn).mt_cursors).offset(dbi as isize);
        *fresh38 = &mut mc;
        rc = mdb_cursor_del(&mut mc, flags);
        let ref mut fresh39 = *((*txn).mt_cursors).offset(dbi as isize);
        *fresh39 = mc.mc_next;
    }
    return rc;
}
unsafe extern "C" fn mdb_page_split(
    mut mc: *mut MDB_cursor,
    mut newkey: *mut MDB_val,
    mut newdata: *mut MDB_val,
    mut newpgno: pgno_t,
    mut nflags: libc::c_uint,
) -> libc::c_int {
    let mut current_block: u64;
    let mut flags: libc::c_uint = 0;
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut new_root: libc::c_int = 0 as libc::c_int;
    let mut did_split: libc::c_int = 0 as libc::c_int;
    let mut newindx: indx_t = 0;
    let mut pgno: pgno_t = 0 as libc::c_int as pgno_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut split_indx: libc::c_int = 0;
    let mut nkeys: libc::c_int = 0;
    let mut pmax: libc::c_int = 0;
    let mut env: *mut MDB_env = (*(*mc).mc_txn).mt_env;
    let mut node: *mut MDB_node = 0 as *mut MDB_node;
    let mut sepkey: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut rkey: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut xdata: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut rdata: *mut MDB_val = &mut xdata;
    let mut copy: *mut MDB_page = 0 as *mut MDB_page;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut rp: *mut MDB_page = 0 as *mut MDB_page;
    let mut pp: *mut MDB_page = 0 as *mut MDB_page;
    let mut ptop: libc::c_int = 0;
    let mut mn: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    mp = (*mc).mc_pg[(*mc).mc_top as usize];
    newindx = (*mc).mc_ki[(*mc).mc_top as usize];
    nkeys = (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
        .wrapping_sub(
            (16 as libc::c_ulong as libc::c_uint)
                .wrapping_sub(
                    (if 0 as libc::c_int != 0 {
                        16 as libc::c_ulong as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }),
                ),
        ) >> 1 as libc::c_int) as libc::c_int;
    rc = mdb_page_new(mc, (*mp).mp_flags as uint32_t, 1 as libc::c_int, &mut rp);
    if rc != 0 {
        return rc;
    }
    (*rp).mp_pad = (*mp).mp_pad;
    if ((*mc).mc_top as libc::c_int) < 1 as libc::c_int {
        rc = mdb_page_new(mc, 0x1 as libc::c_int as uint32_t, 1 as libc::c_int, &mut pp);
        if rc != 0 {
            current_block = 679948005759432958;
        } else {
            i = (*mc).mc_snum as libc::c_int;
            while i > 0 as libc::c_int {
                (*mc).mc_pg[i as usize] = (*mc).mc_pg[(i - 1 as libc::c_int) as usize];
                (*mc).mc_ki[i as usize] = (*mc).mc_ki[(i - 1 as libc::c_int) as usize];
                i -= 1;
                i;
            }
            (*mc).mc_pg[0 as libc::c_int as usize] = pp;
            (*mc).mc_ki[0 as libc::c_int as usize] = 0 as libc::c_int as indx_t;
            (*(*mc).mc_db).md_root = (*pp).mp_p.p_pgno;
            let fresh40 = (*(*mc).mc_db).md_depth;
            (*(*mc).mc_db).md_depth = ((*(*mc).mc_db).md_depth).wrapping_add(1);
            new_root = fresh40 as libc::c_int;
            rc = mdb_node_add(
                mc,
                0 as libc::c_int as indx_t,
                0 as *mut MDB_val,
                0 as *mut MDB_val,
                (*mp).mp_p.p_pgno,
                0 as libc::c_int as libc::c_uint,
            );
            if rc != 0 as libc::c_int {
                (*mc)
                    .mc_pg[0 as libc::c_int
                    as usize] = (*mc).mc_pg[1 as libc::c_int as usize];
                (*mc)
                    .mc_ki[0 as libc::c_int
                    as usize] = (*mc).mc_ki[1 as libc::c_int as usize];
                (*(*mc).mc_db).md_root = (*mp).mp_p.p_pgno;
                (*(*mc).mc_db).md_depth = ((*(*mc).mc_db).md_depth).wrapping_sub(1);
                (*(*mc).mc_db).md_depth;
                current_block = 679948005759432958;
            } else {
                (*mc).mc_snum = ((*mc).mc_snum).wrapping_add(1);
                (*mc).mc_snum;
                (*mc).mc_top = ((*mc).mc_top).wrapping_add(1);
                (*mc).mc_top;
                ptop = 0 as libc::c_int;
                current_block = 15925075030174552612;
            }
        }
    } else {
        ptop = (*mc).mc_top as libc::c_int - 1 as libc::c_int;
        current_block = 15925075030174552612;
    }
    match current_block {
        15925075030174552612 => {
            mdb_cursor_copy(mc, &mut mn);
            mn.mc_xcursor = 0 as *mut MDB_xcursor;
            mn.mc_pg[mn.mc_top as usize] = rp;
            mn
                .mc_ki[ptop
                as usize] = ((*mc).mc_ki[ptop as usize] as libc::c_int
                + 1 as libc::c_int) as indx_t;
            if nflags & 0x20000 as libc::c_int as libc::c_uint != 0 {
                mn.mc_ki[mn.mc_top as usize] = 0 as libc::c_int as indx_t;
                sepkey = *newkey;
                split_indx = newindx as libc::c_int;
                nkeys = 0 as libc::c_int;
                current_block = 14155412868135599428;
            } else {
                split_indx = (nkeys + 1 as libc::c_int) / 2 as libc::c_int;
                if (*rp).mp_flags as libc::c_int & 0x20 as libc::c_int
                    == 0x20 as libc::c_int
                {
                    let mut split: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut ins: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut x: libc::c_int = 0;
                    let mut lsize: libc::c_uint = 0;
                    let mut rsize: libc::c_uint = 0;
                    let mut ksize: libc::c_uint = 0;
                    x = (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int - split_indx;
                    ksize = (*(*mc).mc_db).md_pad;
                    split = (mp as *mut libc::c_char)
                        .offset(16 as libc::c_ulong as libc::c_uint as isize)
                        .offset(
                            (split_indx as libc::c_uint).wrapping_mul(ksize) as isize,
                        );
                    rsize = ((nkeys - split_indx) as libc::c_uint).wrapping_mul(ksize);
                    lsize = ((nkeys - split_indx) as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<indx_t>() as libc::c_ulong)
                        as libc::c_uint;
                    (*mp)
                        .mp_pb
                        .pb
                        .pb_lower = ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                        .wrapping_sub(lsize) as indx_t as indx_t;
                    (*rp)
                        .mp_pb
                        .pb
                        .pb_lower = ((*rp).mp_pb.pb.pb_lower as libc::c_uint)
                        .wrapping_add(lsize) as indx_t as indx_t;
                    (*mp)
                        .mp_pb
                        .pb
                        .pb_upper = ((*mp).mp_pb.pb.pb_upper as libc::c_uint)
                        .wrapping_add(rsize.wrapping_sub(lsize)) as indx_t as indx_t;
                    (*rp)
                        .mp_pb
                        .pb
                        .pb_upper = ((*rp).mp_pb.pb.pb_upper as libc::c_uint)
                        .wrapping_sub(rsize.wrapping_sub(lsize)) as indx_t as indx_t;
                    sepkey.mv_size = ksize as size_t;
                    if newindx as libc::c_int == split_indx {
                        sepkey.mv_data = (*newkey).mv_data;
                    } else {
                        sepkey.mv_data = split as *mut libc::c_void;
                    }
                    if x < 0 as libc::c_int {
                        ins = (mp as *mut libc::c_char)
                            .offset(16 as libc::c_ulong as libc::c_uint as isize)
                            .offset(
                                ((*mc).mc_ki[(*mc).mc_top as usize] as libc::c_uint)
                                    .wrapping_mul(ksize) as isize,
                            );
                        memcpy(
                            ((*rp).mp_ptrs).as_mut_ptr() as *mut libc::c_void,
                            split as *const libc::c_void,
                            rsize as libc::c_ulong,
                        );
                        sepkey
                            .mv_data = ((*rp).mp_ptrs).as_mut_ptr() as *mut libc::c_void;
                        memmove(
                            ins.offset(ksize as isize) as *mut libc::c_void,
                            ins as *const libc::c_void,
                            ((split_indx
                                - (*mc).mc_ki[(*mc).mc_top as usize] as libc::c_int)
                                as libc::c_uint)
                                .wrapping_mul(ksize) as libc::c_ulong,
                        );
                        memcpy(
                            ins as *mut libc::c_void,
                            (*newkey).mv_data,
                            ksize as libc::c_ulong,
                        );
                        (*mp)
                            .mp_pb
                            .pb
                            .pb_lower = ((*mp).mp_pb.pb.pb_lower as libc::c_ulong)
                            .wrapping_add(
                                ::std::mem::size_of::<indx_t>() as libc::c_ulong,
                            ) as indx_t as indx_t;
                        (*mp)
                            .mp_pb
                            .pb
                            .pb_upper = ((*mp).mp_pb.pb.pb_upper as libc::c_ulong)
                            .wrapping_sub(
                                (ksize as libc::c_ulong)
                                    .wrapping_sub(
                                        ::std::mem::size_of::<indx_t>() as libc::c_ulong,
                                    ),
                            ) as indx_t as indx_t;
                    } else {
                        if x != 0 {
                            memcpy(
                                ((*rp).mp_ptrs).as_mut_ptr() as *mut libc::c_void,
                                split as *const libc::c_void,
                                (x as libc::c_uint).wrapping_mul(ksize) as libc::c_ulong,
                            );
                        }
                        ins = (rp as *mut libc::c_char)
                            .offset(16 as libc::c_ulong as libc::c_uint as isize)
                            .offset((x as libc::c_uint).wrapping_mul(ksize) as isize);
                        memcpy(
                            ins as *mut libc::c_void,
                            (*newkey).mv_data,
                            ksize as libc::c_ulong,
                        );
                        memcpy(
                            ins.offset(ksize as isize) as *mut libc::c_void,
                            split
                                .offset((x as libc::c_uint).wrapping_mul(ksize) as isize)
                                as *const libc::c_void,
                            rsize.wrapping_sub((x as libc::c_uint).wrapping_mul(ksize))
                                as libc::c_ulong,
                        );
                        (*rp)
                            .mp_pb
                            .pb
                            .pb_lower = ((*rp).mp_pb.pb.pb_lower as libc::c_ulong)
                            .wrapping_add(
                                ::std::mem::size_of::<indx_t>() as libc::c_ulong,
                            ) as indx_t as indx_t;
                        (*rp)
                            .mp_pb
                            .pb
                            .pb_upper = ((*rp).mp_pb.pb.pb_upper as libc::c_ulong)
                            .wrapping_sub(
                                (ksize as libc::c_ulong)
                                    .wrapping_sub(
                                        ::std::mem::size_of::<indx_t>() as libc::c_ulong,
                                    ),
                            ) as indx_t as indx_t;
                        (*mc).mc_ki[(*mc).mc_top as usize] = x as indx_t;
                    }
                    current_block = 14155412868135599428;
                } else {
                    let mut psize: libc::c_int = 0;
                    let mut nsize: libc::c_int = 0;
                    let mut k: libc::c_int = 0;
                    pmax = ((*env).me_psize)
                        .wrapping_sub(16 as libc::c_ulong as libc::c_uint)
                        as libc::c_int;
                    if (*mp).mp_flags as libc::c_int & 0x2 as libc::c_int
                        == 0x2 as libc::c_int
                    {
                        nsize = mdb_leaf_size(env, newkey, newdata) as libc::c_int;
                    } else {
                        nsize = mdb_branch_size(env, newkey) as libc::c_int;
                    }
                    nsize = ((nsize as libc::c_uint).wrapping_add(1 as libc::c_uint)
                        & -(2 as libc::c_int) as libc::c_uint) as libc::c_int;
                    copy = mdb_page_malloc(
                        (*mc).mc_txn,
                        1 as libc::c_int as libc::c_uint,
                    );
                    if copy.is_null() {
                        rc = 12 as libc::c_int;
                        current_block = 679948005759432958;
                    } else {
                        (*copy).mp_p.p_pgno = (*mp).mp_p.p_pgno;
                        (*copy).mp_flags = (*mp).mp_flags;
                        (*copy)
                            .mp_pb
                            .pb
                            .pb_lower = (16 as libc::c_ulong as libc::c_uint)
                            .wrapping_sub(
                                (if 0 as libc::c_int != 0 {
                                    16 as libc::c_ulong as libc::c_uint
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }),
                            ) as indx_t;
                        (*copy)
                            .mp_pb
                            .pb
                            .pb_upper = ((*env).me_psize)
                            .wrapping_sub(
                                (if 0 as libc::c_int != 0 {
                                    16 as libc::c_ulong as libc::c_uint
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }),
                            ) as indx_t;
                        i = 0 as libc::c_int;
                        j = 0 as libc::c_int;
                        while i < nkeys {
                            if i == newindx as libc::c_int {
                                let fresh41 = j;
                                j = j + 1;
                                *((*copy).mp_ptrs)
                                    .as_mut_ptr()
                                    .offset(fresh41 as isize) = 0 as libc::c_int as indx_t;
                            }
                            let fresh42 = j;
                            j = j + 1;
                            *((*copy).mp_ptrs)
                                .as_mut_ptr()
                                .offset(
                                    fresh42 as isize,
                                ) = *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize);
                            i += 1;
                            i;
                        }
                        if nkeys < 32 as libc::c_int || nsize > pmax / 16 as libc::c_int
                            || newindx as libc::c_int >= nkeys
                        {
                            psize = 0 as libc::c_int;
                            if newindx as libc::c_int <= split_indx
                                || newindx as libc::c_int >= nkeys
                            {
                                i = 0 as libc::c_int;
                                j = 1 as libc::c_int;
                                k = if newindx as libc::c_int >= nkeys {
                                    nkeys
                                } else {
                                    split_indx + 1 as libc::c_int
                                        + ((*mp).mp_flags as libc::c_int & 0x2 as libc::c_int
                                            == 0x2 as libc::c_int) as libc::c_int
                                };
                            } else {
                                i = nkeys;
                                j = -(1 as libc::c_int);
                                k = split_indx - 1 as libc::c_int;
                            }
                            while i != k {
                                if i == newindx as libc::c_int {
                                    psize += nsize;
                                    node = 0 as *mut MDB_node;
                                } else {
                                    node = (mp as *mut libc::c_char)
                                        .offset(
                                            *((*copy).mp_ptrs).as_mut_ptr().offset(i as isize)
                                                as libc::c_int as isize,
                                        )
                                        .offset(
                                            (if 0 as libc::c_int != 0 {
                                                16 as libc::c_ulong as libc::c_uint
                                            } else {
                                                0 as libc::c_int as libc::c_uint
                                            }) as isize,
                                        ) as *mut MDB_node;
                                    psize = (psize as libc::c_ulong)
                                        .wrapping_add(
                                            (8 as libc::c_ulong)
                                                .wrapping_add((*node).mn_ksize as libc::c_ulong)
                                                .wrapping_add(
                                                    ::std::mem::size_of::<indx_t>() as libc::c_ulong,
                                                ),
                                        ) as libc::c_int as libc::c_int;
                                    if (*mp).mp_flags as libc::c_int & 0x2 as libc::c_int
                                        == 0x2 as libc::c_int
                                    {
                                        if (*node).mn_flags as libc::c_int & 0x1 as libc::c_int
                                            == 0x1 as libc::c_int
                                        {
                                            psize = (psize as libc::c_ulong)
                                                .wrapping_add(
                                                    ::std::mem::size_of::<pgno_t>() as libc::c_ulong,
                                                ) as libc::c_int as libc::c_int;
                                        } else {
                                            psize = (psize as libc::c_uint)
                                                .wrapping_add(
                                                    (*node).mn_lo as libc::c_uint
                                                        | ((*node).mn_hi as libc::c_uint) << 16 as libc::c_int,
                                                ) as libc::c_int as libc::c_int;
                                        }
                                    }
                                    psize = ((psize as libc::c_uint)
                                        .wrapping_add(1 as libc::c_uint)
                                        & -(2 as libc::c_int) as libc::c_uint) as libc::c_int;
                                }
                                if psize > pmax || i == k - j {
                                    split_indx = i + (j < 0 as libc::c_int) as libc::c_int;
                                    break;
                                } else {
                                    i += j;
                                }
                            }
                        }
                        if split_indx == newindx as libc::c_int {
                            sepkey.mv_size = (*newkey).mv_size;
                            sepkey.mv_data = (*newkey).mv_data;
                        } else {
                            node = (mp as *mut libc::c_char)
                                .offset(
                                    *((*copy).mp_ptrs).as_mut_ptr().offset(split_indx as isize)
                                        as libc::c_int as isize,
                                )
                                .offset(
                                    (if 0 as libc::c_int != 0 {
                                        16 as libc::c_ulong as libc::c_uint
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    }) as isize,
                                ) as *mut MDB_node;
                            sepkey.mv_size = (*node).mn_ksize as size_t;
                            sepkey
                                .mv_data = ((*node).mn_data).as_mut_ptr()
                                as *mut libc::c_void;
                        }
                        current_block = 14155412868135599428;
                    }
                }
            }
            match current_block {
                679948005759432958 => {}
                _ => {
                    if (((*mn.mc_pg[ptop as usize]).mp_pb.pb.pb_upper as libc::c_int
                        - (*mn.mc_pg[ptop as usize]).mp_pb.pb.pb_lower as libc::c_int)
                        as indx_t as libc::c_ulong) < mdb_branch_size(env, &mut sepkey)
                    {
                        let mut snum: libc::c_int = (*mc).mc_snum as libc::c_int;
                        mn.mc_snum = (mn.mc_snum).wrapping_sub(1);
                        mn.mc_snum;
                        mn.mc_top = (mn.mc_top).wrapping_sub(1);
                        mn.mc_top;
                        did_split = 1 as libc::c_int;
                        let mut dummy: MDB_cursor = MDB_cursor {
                            mc_next: 0 as *mut MDB_cursor,
                            mc_backup: 0 as *mut MDB_cursor,
                            mc_xcursor: 0 as *mut MDB_xcursor,
                            mc_txn: 0 as *mut MDB_txn,
                            mc_dbi: 0,
                            mc_db: 0 as *mut MDB_db,
                            mc_dbx: 0 as *mut MDB_dbx,
                            mc_dbflag: 0 as *mut libc::c_uchar,
                            mc_snum: 0,
                            mc_top: 0,
                            mc_flags: 0,
                            mc_pg: [0 as *mut MDB_page; 32],
                            mc_ki: [0; 32],
                        };
                        let mut tracked: *mut MDB_cursor = 0 as *mut MDB_cursor;
                        let mut tp: *mut *mut MDB_cursor = &mut *((*mn.mc_txn)
                            .mt_cursors)
                            .offset(mn.mc_dbi as isize) as *mut *mut MDB_cursor;
                        if mn.mc_flags & 0x4 as libc::c_int as libc::c_uint != 0 {
                            dummy.mc_flags = 0x1 as libc::c_int as libc::c_uint;
                            dummy
                                .mc_xcursor = &mut mn as *mut MDB_cursor
                                as *mut MDB_xcursor;
                            tracked = &mut dummy;
                        } else {
                            tracked = &mut mn;
                        }
                        (*tracked).mc_next = *tp;
                        *tp = tracked;
                        rc = mdb_page_split(
                            &mut mn,
                            &mut sepkey,
                            0 as *mut MDB_val,
                            (*rp).mp_p.p_pgno,
                            0 as libc::c_int as libc::c_uint,
                        );
                        *tp = (*tracked).mc_next;
                        if rc != 0 {
                            current_block = 679948005759432958;
                        } else {
                            if (*mc).mc_snum as libc::c_int > snum {
                                ptop += 1;
                                ptop;
                            }
                            if mn.mc_pg[ptop as usize] != (*mc).mc_pg[ptop as usize]
                                && (*mc).mc_ki[ptop as usize] as libc::c_uint
                                    >= ((*(*mc).mc_pg[ptop as usize]).mp_pb.pb.pb_lower
                                        as libc::c_uint)
                                        .wrapping_sub(
                                            (16 as libc::c_ulong as libc::c_uint)
                                                .wrapping_sub(
                                                    (if 0 as libc::c_int != 0 {
                                                        16 as libc::c_ulong as libc::c_uint
                                                    } else {
                                                        0 as libc::c_int as libc::c_uint
                                                    }),
                                                ),
                                        ) >> 1 as libc::c_int
                            {
                                i = 0 as libc::c_int;
                                while i < ptop {
                                    (*mc).mc_pg[i as usize] = mn.mc_pg[i as usize];
                                    (*mc).mc_ki[i as usize] = mn.mc_ki[i as usize];
                                    i += 1;
                                    i;
                                }
                                (*mc).mc_pg[ptop as usize] = mn.mc_pg[ptop as usize];
                                if mn.mc_ki[ptop as usize] != 0 {
                                    (*mc)
                                        .mc_ki[ptop
                                        as usize] = (mn.mc_ki[ptop as usize] as libc::c_int
                                        - 1 as libc::c_int) as indx_t;
                                } else {
                                    (*mc).mc_ki[ptop as usize] = mn.mc_ki[ptop as usize];
                                    rc = mdb_cursor_sibling(mc, 0 as libc::c_int);
                                }
                            }
                            current_block = 7293850626974290116;
                        }
                    } else {
                        mn.mc_top = (mn.mc_top).wrapping_sub(1);
                        mn.mc_top;
                        rc = mdb_node_add(
                            &mut mn,
                            mn.mc_ki[ptop as usize],
                            &mut sepkey,
                            0 as *mut MDB_val,
                            (*rp).mp_p.p_pgno,
                            0 as libc::c_int as libc::c_uint,
                        );
                        mn.mc_top = (mn.mc_top).wrapping_add(1);
                        mn.mc_top;
                        current_block = 7293850626974290116;
                    }
                    match current_block {
                        679948005759432958 => {}
                        _ => {
                            if rc != 0 as libc::c_int {
                                if rc == -(30798 as libc::c_int) {
                                    rc = -(30779 as libc::c_int);
                                }
                            } else {
                                if nflags & 0x20000 as libc::c_int as libc::c_uint != 0 {
                                    (*mc).mc_pg[(*mc).mc_top as usize] = rp;
                                    (*mc)
                                        .mc_ki[(*mc).mc_top as usize] = 0 as libc::c_int as indx_t;
                                    rc = mdb_node_add(
                                        mc,
                                        0 as libc::c_int as indx_t,
                                        newkey,
                                        newdata,
                                        newpgno,
                                        nflags,
                                    );
                                    if rc != 0 {
                                        current_block = 679948005759432958;
                                    } else {
                                        i = 0 as libc::c_int;
                                        while i < (*mc).mc_top as libc::c_int {
                                            (*mc).mc_ki[i as usize] = mn.mc_ki[i as usize];
                                            i += 1;
                                            i;
                                        }
                                        current_block = 12703207391024092602;
                                    }
                                } else if !((*mp).mp_flags as libc::c_int
                                    & 0x20 as libc::c_int == 0x20 as libc::c_int)
                                {
                                    (*mc).mc_pg[(*mc).mc_top as usize] = rp;
                                    i = split_indx;
                                    j = 0 as libc::c_int;
                                    loop {
                                        if i == newindx as libc::c_int {
                                            rkey.mv_data = (*newkey).mv_data;
                                            rkey.mv_size = (*newkey).mv_size;
                                            if (*mp).mp_flags as libc::c_int & 0x2 as libc::c_int
                                                == 0x2 as libc::c_int
                                            {
                                                rdata = newdata;
                                            } else {
                                                pgno = newpgno;
                                            }
                                            flags = nflags;
                                            (*mc).mc_ki[(*mc).mc_top as usize] = j as indx_t;
                                        } else {
                                            node = (mp as *mut libc::c_char)
                                                .offset(
                                                    *((*copy).mp_ptrs).as_mut_ptr().offset(i as isize)
                                                        as libc::c_int as isize,
                                                )
                                                .offset(
                                                    (if 0 as libc::c_int != 0 {
                                                        16 as libc::c_ulong as libc::c_uint
                                                    } else {
                                                        0 as libc::c_int as libc::c_uint
                                                    }) as isize,
                                                ) as *mut MDB_node;
                                            rkey
                                                .mv_data = ((*node).mn_data).as_mut_ptr()
                                                as *mut libc::c_void;
                                            rkey.mv_size = (*node).mn_ksize as size_t;
                                            if (*mp).mp_flags as libc::c_int & 0x2 as libc::c_int
                                                == 0x2 as libc::c_int
                                            {
                                                xdata
                                                    .mv_data = ((*node).mn_data)
                                                    .as_mut_ptr()
                                                    .offset((*node).mn_ksize as libc::c_int as isize)
                                                    as *mut libc::c_void;
                                                xdata
                                                    .mv_size = ((*node).mn_lo as libc::c_uint
                                                    | ((*node).mn_hi as libc::c_uint) << 16 as libc::c_int)
                                                    as size_t;
                                                rdata = &mut xdata;
                                            } else {
                                                pgno = (*node).mn_lo as libc::c_ulong
                                                    | ((*node).mn_hi as pgno_t) << 16 as libc::c_int
                                                    | (if (if -(1 as libc::c_int) as pgno_t
                                                        > 0xffffffff as libc::c_uint as libc::c_ulong
                                                    {
                                                        32 as libc::c_int
                                                    } else {
                                                        0 as libc::c_int
                                                    }) != 0
                                                    {
                                                        ((*node).mn_flags as pgno_t)
                                                            << (if -(1 as libc::c_int) as pgno_t
                                                                > 0xffffffff as libc::c_uint as libc::c_ulong
                                                            {
                                                                32 as libc::c_int
                                                            } else {
                                                                0 as libc::c_int
                                                            })
                                                    } else {
                                                        0 as libc::c_int as libc::c_ulong
                                                    });
                                            }
                                            flags = (*node).mn_flags as libc::c_uint;
                                        }
                                        if !((*mp).mp_flags as libc::c_int & 0x2 as libc::c_int
                                            == 0x2 as libc::c_int) && j == 0 as libc::c_int
                                        {
                                            rkey.mv_size = 0 as libc::c_int as size_t;
                                        }
                                        rc = mdb_node_add(
                                            mc,
                                            j as indx_t,
                                            &mut rkey,
                                            rdata,
                                            pgno,
                                            flags,
                                        );
                                        if rc != 0 {
                                            current_block = 679948005759432958;
                                            break;
                                        }
                                        if i == nkeys {
                                            i = 0 as libc::c_int;
                                            j = 0 as libc::c_int;
                                            (*mc).mc_pg[(*mc).mc_top as usize] = copy;
                                        } else {
                                            i += 1;
                                            i;
                                            j += 1;
                                            j;
                                        }
                                        if !(i != split_indx) {
                                            current_block = 6957654774345280688;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        679948005759432958 => {}
                                        _ => {
                                            nkeys = (((*copy).mp_pb.pb.pb_lower as libc::c_uint)
                                                .wrapping_sub(
                                                    (16 as libc::c_ulong as libc::c_uint)
                                                        .wrapping_sub(
                                                            (if 0 as libc::c_int != 0 {
                                                                16 as libc::c_ulong as libc::c_uint
                                                            } else {
                                                                0 as libc::c_int as libc::c_uint
                                                            }),
                                                        ),
                                                ) >> 1 as libc::c_int) as libc::c_int;
                                            i = 0 as libc::c_int;
                                            while i < nkeys {
                                                *((*mp).mp_ptrs)
                                                    .as_mut_ptr()
                                                    .offset(
                                                        i as isize,
                                                    ) = *((*copy).mp_ptrs).as_mut_ptr().offset(i as isize);
                                                i += 1;
                                                i;
                                            }
                                            (*mp).mp_pb.pb.pb_lower = (*copy).mp_pb.pb.pb_lower;
                                            (*mp).mp_pb.pb.pb_upper = (*copy).mp_pb.pb.pb_upper;
                                            memcpy(
                                                (mp as *mut libc::c_char)
                                                    .offset(
                                                        *((*mp).mp_ptrs)
                                                            .as_mut_ptr()
                                                            .offset((nkeys - 1 as libc::c_int) as isize) as libc::c_int
                                                            as isize,
                                                    )
                                                    .offset(
                                                        (if 0 as libc::c_int != 0 {
                                                            16 as libc::c_ulong as libc::c_uint
                                                        } else {
                                                            0 as libc::c_int as libc::c_uint
                                                        }) as isize,
                                                    ) as *mut MDB_node as *mut libc::c_void,
                                                (copy as *mut libc::c_char)
                                                    .offset(
                                                        *((*copy).mp_ptrs)
                                                            .as_mut_ptr()
                                                            .offset((nkeys - 1 as libc::c_int) as isize) as libc::c_int
                                                            as isize,
                                                    )
                                                    .offset(
                                                        (if 0 as libc::c_int != 0 {
                                                            16 as libc::c_ulong as libc::c_uint
                                                        } else {
                                                            0 as libc::c_int as libc::c_uint
                                                        }) as isize,
                                                    ) as *mut MDB_node as *const libc::c_void,
                                                ((*env).me_psize)
                                                    .wrapping_sub((*copy).mp_pb.pb.pb_upper as libc::c_uint)
                                                    .wrapping_sub(
                                                        (if 0 as libc::c_int != 0 {
                                                            16 as libc::c_ulong as libc::c_uint
                                                        } else {
                                                            0 as libc::c_int as libc::c_uint
                                                        }),
                                                    ) as libc::c_ulong,
                                            );
                                            if (newindx as libc::c_int) < split_indx {
                                                (*mc).mc_pg[(*mc).mc_top as usize] = mp;
                                            } else {
                                                (*mc).mc_pg[(*mc).mc_top as usize] = rp;
                                                (*mc)
                                                    .mc_ki[ptop
                                                    as usize] = ((*mc).mc_ki[ptop as usize]).wrapping_add(1);
                                                (*mc).mc_ki[ptop as usize];
                                                if mn.mc_pg[ptop as usize] != (*mc).mc_pg[ptop as usize]
                                                    && (*mc).mc_ki[ptop as usize] as libc::c_uint
                                                        >= ((*(*mc).mc_pg[ptop as usize]).mp_pb.pb.pb_lower
                                                            as libc::c_uint)
                                                            .wrapping_sub(
                                                                (16 as libc::c_ulong as libc::c_uint)
                                                                    .wrapping_sub(
                                                                        (if 0 as libc::c_int != 0 {
                                                                            16 as libc::c_ulong as libc::c_uint
                                                                        } else {
                                                                            0 as libc::c_int as libc::c_uint
                                                                        }),
                                                                    ),
                                                            ) >> 1 as libc::c_int
                                                {
                                                    i = 0 as libc::c_int;
                                                    while i <= ptop {
                                                        (*mc).mc_pg[i as usize] = mn.mc_pg[i as usize];
                                                        (*mc).mc_ki[i as usize] = mn.mc_ki[i as usize];
                                                        i += 1;
                                                        i;
                                                    }
                                                }
                                            }
                                            if nflags & 0x10000 as libc::c_int as libc::c_uint != 0 {
                                                node = ((*mc).mc_pg[(*mc).mc_top as usize]
                                                    as *mut libc::c_char)
                                                    .offset(
                                                        *((*(*mc).mc_pg[(*mc).mc_top as usize]).mp_ptrs)
                                                            .as_mut_ptr()
                                                            .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                                                            as libc::c_int as isize,
                                                    )
                                                    .offset(
                                                        (if 0 as libc::c_int != 0 {
                                                            16 as libc::c_ulong as libc::c_uint
                                                        } else {
                                                            0 as libc::c_int as libc::c_uint
                                                        }) as isize,
                                                    ) as *mut MDB_node;
                                                if (*node).mn_flags as libc::c_int & 0x1 as libc::c_int == 0
                                                {
                                                    (*newdata)
                                                        .mv_data = ((*node).mn_data)
                                                        .as_mut_ptr()
                                                        .offset((*node).mn_ksize as libc::c_int as isize)
                                                        as *mut libc::c_void;
                                                }
                                            }
                                            current_block = 12703207391024092602;
                                        }
                                    }
                                } else {
                                    if newindx as libc::c_int >= split_indx {
                                        (*mc).mc_pg[(*mc).mc_top as usize] = rp;
                                        (*mc)
                                            .mc_ki[ptop
                                            as usize] = ((*mc).mc_ki[ptop as usize]).wrapping_add(1);
                                        (*mc).mc_ki[ptop as usize];
                                        if mn.mc_pg[ptop as usize] != (*mc).mc_pg[ptop as usize]
                                            && (*mc).mc_ki[ptop as usize] as libc::c_uint
                                                >= ((*(*mc).mc_pg[ptop as usize]).mp_pb.pb.pb_lower
                                                    as libc::c_uint)
                                                    .wrapping_sub(
                                                        (16 as libc::c_ulong as libc::c_uint)
                                                            .wrapping_sub(
                                                                (if 0 as libc::c_int != 0 {
                                                                    16 as libc::c_ulong as libc::c_uint
                                                                } else {
                                                                    0 as libc::c_int as libc::c_uint
                                                                }),
                                                            ),
                                                    ) >> 1 as libc::c_int
                                        {
                                            i = 0 as libc::c_int;
                                            while i <= ptop {
                                                (*mc).mc_pg[i as usize] = mn.mc_pg[i as usize];
                                                (*mc).mc_ki[i as usize] = mn.mc_ki[i as usize];
                                                i += 1;
                                                i;
                                            }
                                        }
                                    }
                                    current_block = 12703207391024092602;
                                }
                                match current_block {
                                    679948005759432958 => {}
                                    _ => {
                                        let mut m2: *mut MDB_cursor = 0 as *mut MDB_cursor;
                                        let mut m3: *mut MDB_cursor = 0 as *mut MDB_cursor;
                                        let mut dbi: MDB_dbi = (*mc).mc_dbi;
                                        nkeys = (((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                                            .wrapping_sub(
                                                (16 as libc::c_ulong as libc::c_uint)
                                                    .wrapping_sub(
                                                        (if 0 as libc::c_int != 0 {
                                                            16 as libc::c_ulong as libc::c_uint
                                                        } else {
                                                            0 as libc::c_int as libc::c_uint
                                                        }),
                                                    ),
                                            ) >> 1 as libc::c_int) as libc::c_int;
                                        let mut current_block_284: u64;
                                        m2 = *((*(*mc).mc_txn).mt_cursors).offset(dbi as isize);
                                        while !m2.is_null() {
                                            if (*mc).mc_flags & 0x4 as libc::c_int as libc::c_uint != 0
                                            {
                                                m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
                                            } else {
                                                m3 = m2;
                                            }
                                            if !(m3 == mc) {
                                                if !((*m2).mc_flags & (*m3).mc_flags
                                                    & 0x1 as libc::c_int as libc::c_uint == 0)
                                                {
                                                    if new_root != 0 {
                                                        let mut k_0: libc::c_int = 0;
                                                        if (*m3).mc_pg[0 as libc::c_int as usize] != mp {
                                                            current_block_284 = 870304920170074103;
                                                        } else {
                                                            k_0 = new_root;
                                                            while k_0 >= 0 as libc::c_int {
                                                                (*m3)
                                                                    .mc_ki[(k_0 + 1 as libc::c_int)
                                                                    as usize] = (*m3).mc_ki[k_0 as usize];
                                                                (*m3)
                                                                    .mc_pg[(k_0 + 1 as libc::c_int)
                                                                    as usize] = (*m3).mc_pg[k_0 as usize];
                                                                k_0 -= 1;
                                                                k_0;
                                                            }
                                                            if (*m3).mc_ki[0 as libc::c_int as usize] as libc::c_int
                                                                >= nkeys
                                                            {
                                                                (*m3)
                                                                    .mc_ki[0 as libc::c_int
                                                                    as usize] = 1 as libc::c_int as indx_t;
                                                            } else {
                                                                (*m3)
                                                                    .mc_ki[0 as libc::c_int
                                                                    as usize] = 0 as libc::c_int as indx_t;
                                                            }
                                                            (*m3)
                                                                .mc_pg[0 as libc::c_int
                                                                as usize] = (*mc).mc_pg[0 as libc::c_int as usize];
                                                            (*m3).mc_snum = ((*m3).mc_snum).wrapping_add(1);
                                                            (*m3).mc_snum;
                                                            (*m3).mc_top = ((*m3).mc_top).wrapping_add(1);
                                                            (*m3).mc_top;
                                                            current_block_284 = 8193737063574930042;
                                                        }
                                                    } else {
                                                        current_block_284 = 8193737063574930042;
                                                    }
                                                    match current_block_284 {
                                                        870304920170074103 => {}
                                                        _ => {
                                                            if (*m3).mc_top as libc::c_int
                                                                >= (*mc).mc_top as libc::c_int
                                                                && (*m3).mc_pg[(*mc).mc_top as usize] == mp
                                                            {
                                                                if (*m3).mc_ki[(*mc).mc_top as usize] as libc::c_int
                                                                    >= newindx as libc::c_int
                                                                    && nflags & 0x40000 as libc::c_int as libc::c_uint == 0
                                                                {
                                                                    (*m3)
                                                                        .mc_ki[(*mc).mc_top
                                                                        as usize] = ((*m3).mc_ki[(*mc).mc_top as usize])
                                                                        .wrapping_add(1);
                                                                    (*m3).mc_ki[(*mc).mc_top as usize];
                                                                }
                                                                if (*m3).mc_ki[(*mc).mc_top as usize] as libc::c_int
                                                                    >= nkeys
                                                                {
                                                                    (*m3).mc_pg[(*mc).mc_top as usize] = rp;
                                                                    (*m3)
                                                                        .mc_ki[(*mc).mc_top
                                                                        as usize] = ((*m3).mc_ki[(*mc).mc_top as usize]
                                                                        as libc::c_int - nkeys) as indx_t;
                                                                    i = 0 as libc::c_int;
                                                                    while i < (*mc).mc_top as libc::c_int {
                                                                        (*m3).mc_ki[i as usize] = mn.mc_ki[i as usize];
                                                                        (*m3).mc_pg[i as usize] = mn.mc_pg[i as usize];
                                                                        i += 1;
                                                                        i;
                                                                    }
                                                                }
                                                            } else if did_split == 0
                                                                && (*m3).mc_top as libc::c_int >= ptop
                                                                && (*m3).mc_pg[ptop as usize] == (*mc).mc_pg[ptop as usize]
                                                                && (*m3).mc_ki[ptop as usize] as libc::c_int
                                                                    >= (*mc).mc_ki[ptop as usize] as libc::c_int
                                                            {
                                                                (*m3)
                                                                    .mc_ki[ptop
                                                                    as usize] = ((*m3).mc_ki[ptop as usize]).wrapping_add(1);
                                                                (*m3).mc_ki[ptop as usize];
                                                            }
                                                            if (*mp).mp_flags as libc::c_int & 0x2 as libc::c_int
                                                                == 0x2 as libc::c_int
                                                            {
                                                                let mut xr_pg: *mut MDB_page = (*m3)
                                                                    .mc_pg[(*mc).mc_top as usize];
                                                                let mut xr_node: *mut MDB_node = 0 as *mut MDB_node;
                                                                if !(!(!((*m3).mc_xcursor).is_null()
                                                                    && (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                                                        & 0x1 as libc::c_int as libc::c_uint != 0)
                                                                    || (*m3).mc_ki[(*mc).mc_top as usize] as libc::c_uint
                                                                        >= ((*xr_pg).mp_pb.pb.pb_lower as libc::c_uint)
                                                                            .wrapping_sub(
                                                                                (16 as libc::c_ulong as libc::c_uint)
                                                                                    .wrapping_sub(
                                                                                        (if 0 as libc::c_int != 0 {
                                                                                            16 as libc::c_ulong as libc::c_uint
                                                                                        } else {
                                                                                            0 as libc::c_int as libc::c_uint
                                                                                        }),
                                                                                    ),
                                                                            ) >> 1 as libc::c_int)
                                                                {
                                                                    xr_node = (xr_pg as *mut libc::c_char)
                                                                        .offset(
                                                                            *((*xr_pg).mp_ptrs)
                                                                                .as_mut_ptr()
                                                                                .offset((*m3).mc_ki[(*mc).mc_top as usize] as isize)
                                                                                as libc::c_int as isize,
                                                                        )
                                                                        .offset(
                                                                            (if 0 as libc::c_int != 0 {
                                                                                16 as libc::c_ulong as libc::c_uint
                                                                            } else {
                                                                                0 as libc::c_int as libc::c_uint
                                                                            }) as isize,
                                                                        ) as *mut MDB_node;
                                                                    if (*xr_node).mn_flags as libc::c_int
                                                                        & (0x4 as libc::c_int | 0x2 as libc::c_int)
                                                                        == 0x4 as libc::c_int
                                                                    {
                                                                        (*(*m3).mc_xcursor)
                                                                            .mx_cursor
                                                                            .mc_pg[0 as libc::c_int
                                                                            as usize] = ((*xr_node).mn_data)
                                                                            .as_mut_ptr()
                                                                            .offset((*xr_node).mn_ksize as libc::c_int as isize)
                                                                            as *mut libc::c_void as *mut MDB_page;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            m2 = (*m2).mc_next;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if !copy.is_null() {
        mdb_page_free(env, copy);
    }
    if rc != 0 {
        (*(*mc).mc_txn).mt_flags |= 0x2 as libc::c_int as libc::c_uint;
    }
    return rc;
}
pub unsafe extern "C" fn mdb_put(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut mc: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut mx: MDB_xcursor = MDB_xcursor {
        mx_cursor: MDB_cursor {
            mc_next: 0 as *mut MDB_cursor,
            mc_backup: 0 as *mut MDB_cursor,
            mc_xcursor: 0 as *mut MDB_xcursor,
            mc_txn: 0 as *mut MDB_txn,
            mc_dbi: 0,
            mc_db: 0 as *mut MDB_db,
            mc_dbx: 0 as *mut MDB_dbx,
            mc_dbflag: 0 as *mut libc::c_uchar,
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [0 as *mut MDB_page; 32],
            mc_ki: [0; 32],
        },
        mx_db: MDB_db {
            md_pad: 0,
            md_flags: 0,
            md_depth: 0,
            md_branch_pages: 0,
            md_leaf_pages: 0,
            md_overflow_pages: 0,
            md_entries: 0,
            md_root: 0,
        },
        mx_dbx: MDB_dbx {
            md_name: MDB_val {
                mv_size: 0,
                mv_data: 0 as *mut libc::c_void,
            },
            md_cmp: None,
            md_dcmp: None,
            md_rel: None,
            md_relctx: 0 as *mut libc::c_void,
        },
        mx_dbflag: 0,
    };
    let mut rc: libc::c_int = 0;
    if key.is_null() || data.is_null()
        || !(!txn.is_null() && dbi < (*txn).mt_numdbs
            && *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int
                & 0x10 as libc::c_int != 0)
    {
        return 22 as libc::c_int;
    }
    if flags
        & !(0x10 as libc::c_int | 0x20 as libc::c_int | 0x10000 as libc::c_int
            | 0x20000 as libc::c_int | 0x40000 as libc::c_int) as libc::c_uint != 0
    {
        return 22 as libc::c_int;
    }
    if (*txn).mt_flags
        & (0x20000 as libc::c_int
            | (0x1 as libc::c_int | 0x2 as libc::c_int | 0x10 as libc::c_int))
            as libc::c_uint != 0
    {
        return if (*txn).mt_flags & 0x20000 as libc::c_int as libc::c_uint != 0 {
            13 as libc::c_int
        } else {
            -(30782 as libc::c_int)
        };
    }
    mdb_cursor_init(&mut mc, txn, dbi, &mut mx);
    mc.mc_next = *((*txn).mt_cursors).offset(dbi as isize);
    let ref mut fresh43 = *((*txn).mt_cursors).offset(dbi as isize);
    *fresh43 = &mut mc;
    rc = mdb_cursor_put(&mut mc, key, data, flags);
    let ref mut fresh44 = *((*txn).mt_cursors).offset(dbi as isize);
    *fresh44 = mc.mc_next;
    return rc;
}
unsafe extern "C" fn mdb_env_copythr(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut my: *mut mdb_copy = arg as *mut mdb_copy;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut toggle: libc::c_int = 0 as libc::c_int;
    let mut wsize: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    sigemptyset(&mut set);
    sigaddset(&mut set, 13 as libc::c_int);
    rc = pthread_sigmask(0 as libc::c_int, &mut set, 0 as *mut __sigset_t);
    if rc != 0 as libc::c_int {
        ::std::ptr::write_volatile(&mut (*my).mc_error as *mut libc::c_int, rc);
    }
    pthread_mutex_lock(&mut (*my).mc_mutex);
    loop {
        while (*my).mc_new == 0 {
            pthread_cond_wait(&mut (*my).mc_cond, &mut (*my).mc_mutex);
        }
        if (*my).mc_new == 0 as libc::c_int + 0x10 as libc::c_int {
            break;
        }
        wsize = (*my).mc_wlen[toggle as usize];
        ptr = (*my).mc_wbuf[toggle as usize];
        loop {
            rc = 0 as libc::c_int;
            while wsize > 0 as libc::c_int && (*my).mc_error == 0 {
                len = write((*my).mc_fd, ptr as *const libc::c_void, wsize as size_t)
                    as libc::c_int;
                rc = (len >= 0 as libc::c_int) as libc::c_int;
                if rc == 0 {
                    rc = *__errno_location();
                    if rc == 32 as libc::c_int {
                        let mut tmp: libc::c_int = 0;
                        sigwait(&mut set, &mut tmp);
                    }
                    break;
                } else if len > 0 as libc::c_int {
                    rc = 0 as libc::c_int;
                    ptr = ptr.offset(len as isize);
                    wsize -= len;
                } else {
                    rc = 5 as libc::c_int;
                    break;
                }
            }
            if rc != 0 {
                ::std::ptr::write_volatile(&mut (*my).mc_error as *mut libc::c_int, rc);
            }
            if !((*my).mc_olen[toggle as usize] != 0) {
                break;
            }
            wsize = (*my).mc_olen[toggle as usize];
            ptr = (*my).mc_over[toggle as usize];
            (*my).mc_olen[toggle as usize] = 0 as libc::c_int;
        }
        (*my).mc_wlen[toggle as usize] = 0 as libc::c_int;
        toggle ^= 1 as libc::c_int;
        (*my).mc_new -= 1;
        (*my).mc_new;
        pthread_cond_signal(&mut (*my).mc_cond);
    }
    pthread_mutex_unlock(&mut (*my).mc_mutex);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn mdb_env_cthr_toggle(
    mut my: *mut mdb_copy,
    mut adjust: libc::c_int,
) -> libc::c_int {
    pthread_mutex_lock(&mut (*my).mc_mutex);
    (*my).mc_new += adjust;
    pthread_cond_signal(&mut (*my).mc_cond);
    while (*my).mc_new & 2 as libc::c_int != 0 {
        pthread_cond_wait(&mut (*my).mc_cond, &mut (*my).mc_mutex);
    }
    pthread_mutex_unlock(&mut (*my).mc_mutex);
    (*my).mc_toggle ^= adjust & 1 as libc::c_int;
    (*my).mc_wlen[(*my).mc_toggle as usize] = 0 as libc::c_int;
    return (*my).mc_error;
}
unsafe extern "C" fn mdb_env_cwalk(
    mut my: *mut mdb_copy,
    mut pg: *mut pgno_t,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut mc: MDB_cursor = {
        let mut init = MDB_cursor {
            mc_next: 0 as *mut MDB_cursor,
            mc_backup: 0 as *mut MDB_cursor,
            mc_xcursor: 0 as *mut MDB_xcursor,
            mc_txn: 0 as *mut MDB_txn,
            mc_dbi: 0,
            mc_db: 0 as *mut MDB_db,
            mc_dbx: 0 as *mut MDB_dbx,
            mc_dbflag: 0 as *mut libc::c_uchar,
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [0 as *mut MDB_page; 32],
            mc_ki: [0; 32],
        };
        init
    };
    let mut ni: *mut MDB_node = 0 as *mut MDB_node;
    let mut mo: *mut MDB_page = 0 as *mut MDB_page;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut leaf: *mut MDB_page = 0 as *mut MDB_page;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rc: libc::c_int = 0;
    let mut toggle: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    if *pg == !(0 as libc::c_int as pgno_t) {
        return 0 as libc::c_int;
    }
    mc.mc_snum = 1 as libc::c_int as libc::c_ushort;
    mc.mc_txn = (*my).mc_txn;
    mc
        .mc_flags = (*(*my).mc_txn).mt_flags
        & (0x20000 as libc::c_int | 0x80000 as libc::c_int) as libc::c_uint;
    rc = mdb_page_get(
        &mut mc,
        *pg,
        &mut *(mc.mc_pg).as_mut_ptr().offset(0 as libc::c_int as isize),
        0 as *mut libc::c_int,
    );
    if rc != 0 {
        return rc;
    }
    rc = mdb_page_search_root(&mut mc, 0 as *mut MDB_val, 4 as libc::c_int);
    if rc != 0 {
        return rc;
    }
    ptr = malloc(
        ((*(*my).mc_env).me_psize).wrapping_mul(mc.mc_snum as libc::c_uint)
            as libc::c_ulong,
    ) as *mut libc::c_char;
    buf = ptr;
    if buf.is_null() {
        return 12 as libc::c_int;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < mc.mc_top as libc::c_uint {
        mdb_page_copy(
            ptr as *mut MDB_page,
            mc.mc_pg[i as usize],
            (*(*my).mc_env).me_psize,
        );
        mc.mc_pg[i as usize] = ptr as *mut MDB_page;
        ptr = ptr.offset((*(*my).mc_env).me_psize as isize);
        i = i.wrapping_add(1);
        i;
    }
    leaf = ptr as *mut MDB_page;
    toggle = (*my).mc_toggle;
    's_89: while mc.mc_snum as libc::c_int > 0 as libc::c_int {
        let mut n: libc::c_uint = 0;
        mp = mc.mc_pg[mc.mc_top as usize];
        n = ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
            .wrapping_sub(
                (16 as libc::c_ulong as libc::c_uint)
                    .wrapping_sub(
                        (if 0 as libc::c_int != 0 {
                            16 as libc::c_ulong as libc::c_uint
                        } else {
                            0 as libc::c_int as libc::c_uint
                        }),
                    ),
            ) >> 1 as libc::c_int;
        if (*mp).mp_flags as libc::c_int & 0x2 as libc::c_int == 0x2 as libc::c_int {
            if !((*mp).mp_flags as libc::c_int & 0x20 as libc::c_int
                == 0x20 as libc::c_int) && flags & 0x4 as libc::c_int == 0
            {
                i = 0 as libc::c_int as libc::c_uint;
                while i < n {
                    ni = (mp as *mut libc::c_char)
                        .offset(
                            *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize)
                                as libc::c_int as isize,
                        )
                        .offset(
                            (if 0 as libc::c_int != 0 {
                                16 as libc::c_ulong as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) as isize,
                        ) as *mut MDB_node;
                    if (*ni).mn_flags as libc::c_int & 0x1 as libc::c_int != 0 {
                        let mut omp: *mut MDB_page = 0 as *mut MDB_page;
                        let mut pg_0: pgno_t = 0;
                        if mp != leaf {
                            mc.mc_pg[mc.mc_top as usize] = leaf;
                            mdb_page_copy(leaf, mp, (*(*my).mc_env).me_psize);
                            mp = leaf;
                            ni = (mp as *mut libc::c_char)
                                .offset(
                                    *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize)
                                        as libc::c_int as isize,
                                )
                                .offset(
                                    (if 0 as libc::c_int != 0 {
                                        16 as libc::c_ulong as libc::c_uint
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    }) as isize,
                                ) as *mut MDB_node;
                        }
                        memcpy(
                            &mut pg_0 as *mut pgno_t as *mut libc::c_void,
                            ((*ni).mn_data)
                                .as_mut_ptr()
                                .offset((*ni).mn_ksize as libc::c_int as isize)
                                as *mut libc::c_void,
                            ::std::mem::size_of::<pgno_t>() as libc::c_ulong,
                        );
                        memcpy(
                            ((*ni).mn_data)
                                .as_mut_ptr()
                                .offset((*ni).mn_ksize as libc::c_int as isize)
                                as *mut libc::c_void,
                            &mut (*my).mc_next_pgno as *mut pgno_t
                                as *const libc::c_void,
                            ::std::mem::size_of::<pgno_t>() as libc::c_ulong,
                        );
                        rc = mdb_page_get(
                            &mut mc,
                            pg_0,
                            &mut omp,
                            0 as *mut libc::c_int,
                        );
                        if rc != 0 {
                            break 's_89;
                        }
                        if (*my).mc_wlen[toggle as usize]
                            >= 1024 as libc::c_int * 1024 as libc::c_int
                        {
                            rc = mdb_env_cthr_toggle(my, 1 as libc::c_int);
                            if rc != 0 {
                                break 's_89;
                            }
                            toggle = (*my).mc_toggle;
                        }
                        mo = ((*my).mc_wbuf[toggle as usize])
                            .offset((*my).mc_wlen[toggle as usize] as isize)
                            as *mut MDB_page;
                        memcpy(
                            mo as *mut libc::c_void,
                            omp as *const libc::c_void,
                            (*(*my).mc_env).me_psize as libc::c_ulong,
                        );
                        (*mo).mp_p.p_pgno = (*my).mc_next_pgno;
                        (*my)
                            .mc_next_pgno = ((*my).mc_next_pgno as libc::c_ulong)
                            .wrapping_add((*omp).mp_pb.pb_pages as libc::c_ulong)
                            as pgno_t as pgno_t;
                        (*my)
                            .mc_wlen[toggle
                            as usize] = ((*my).mc_wlen[toggle as usize] as libc::c_uint)
                            .wrapping_add((*(*my).mc_env).me_psize) as libc::c_int
                            as libc::c_int;
                        if (*omp).mp_pb.pb_pages > 1 as libc::c_int as libc::c_uint {
                            (*my)
                                .mc_olen[toggle
                                as usize] = ((*(*my).mc_env).me_psize)
                                .wrapping_mul(
                                    ((*omp).mp_pb.pb_pages)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                ) as libc::c_int;
                            (*my)
                                .mc_over[toggle
                                as usize] = (omp as *mut libc::c_char)
                                .offset((*(*my).mc_env).me_psize as isize);
                            rc = mdb_env_cthr_toggle(my, 1 as libc::c_int);
                            if rc != 0 {
                                break 's_89;
                            }
                            toggle = (*my).mc_toggle;
                        }
                    } else if (*ni).mn_flags as libc::c_int & 0x2 as libc::c_int != 0 {
                        let mut db: MDB_db = MDB_db {
                            md_pad: 0,
                            md_flags: 0,
                            md_depth: 0,
                            md_branch_pages: 0,
                            md_leaf_pages: 0,
                            md_overflow_pages: 0,
                            md_entries: 0,
                            md_root: 0,
                        };
                        if mp != leaf {
                            mc.mc_pg[mc.mc_top as usize] = leaf;
                            mdb_page_copy(leaf, mp, (*(*my).mc_env).me_psize);
                            mp = leaf;
                            ni = (mp as *mut libc::c_char)
                                .offset(
                                    *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize)
                                        as libc::c_int as isize,
                                )
                                .offset(
                                    (if 0 as libc::c_int != 0 {
                                        16 as libc::c_ulong as libc::c_uint
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    }) as isize,
                                ) as *mut MDB_node;
                        }
                        memcpy(
                            &mut db as *mut MDB_db as *mut libc::c_void,
                            ((*ni).mn_data)
                                .as_mut_ptr()
                                .offset((*ni).mn_ksize as libc::c_int as isize)
                                as *mut libc::c_void,
                            ::std::mem::size_of::<MDB_db>() as libc::c_ulong,
                        );
                        (*my).mc_toggle = toggle;
                        rc = mdb_env_cwalk(
                            my,
                            &mut db.md_root,
                            (*ni).mn_flags as libc::c_int & 0x4 as libc::c_int,
                        );
                        if rc != 0 {
                            break 's_89;
                        }
                        toggle = (*my).mc_toggle;
                        memcpy(
                            ((*ni).mn_data)
                                .as_mut_ptr()
                                .offset((*ni).mn_ksize as libc::c_int as isize)
                                as *mut libc::c_void,
                            &mut db as *mut MDB_db as *const libc::c_void,
                            ::std::mem::size_of::<MDB_db>() as libc::c_ulong,
                        );
                    }
                    i = i.wrapping_add(1);
                    i;
                }
            }
        } else {
            mc
                .mc_ki[mc.mc_top
                as usize] = (mc.mc_ki[mc.mc_top as usize]).wrapping_add(1);
            mc.mc_ki[mc.mc_top as usize];
            if (mc.mc_ki[mc.mc_top as usize] as libc::c_uint) < n {
                let mut pg_1: pgno_t = 0;
                loop {
                    ni = (mp as *mut libc::c_char)
                        .offset(
                            *((*mp).mp_ptrs)
                                .as_mut_ptr()
                                .offset(mc.mc_ki[mc.mc_top as usize] as isize)
                                as libc::c_int as isize,
                        )
                        .offset(
                            (if 0 as libc::c_int != 0 {
                                16 as libc::c_ulong as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) as isize,
                        ) as *mut MDB_node;
                    pg_1 = (*ni).mn_lo as libc::c_ulong
                        | ((*ni).mn_hi as pgno_t) << 16 as libc::c_int
                        | (if (if -(1 as libc::c_int) as pgno_t
                            > 0xffffffff as libc::c_uint as libc::c_ulong
                        {
                            32 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) != 0
                        {
                            ((*ni).mn_flags as pgno_t)
                                << (if -(1 as libc::c_int) as pgno_t
                                    > 0xffffffff as libc::c_uint as libc::c_ulong
                                {
                                    32 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                        } else {
                            0 as libc::c_int as libc::c_ulong
                        });
                    rc = mdb_page_get(&mut mc, pg_1, &mut mp, 0 as *mut libc::c_int);
                    if rc != 0 {
                        break 's_89;
                    }
                    mc.mc_top = (mc.mc_top).wrapping_add(1);
                    mc.mc_top;
                    mc.mc_snum = (mc.mc_snum).wrapping_add(1);
                    mc.mc_snum;
                    mc.mc_ki[mc.mc_top as usize] = 0 as libc::c_int as indx_t;
                    if !((*mp).mp_flags as libc::c_int & 0x1 as libc::c_int
                        == 0x1 as libc::c_int)
                    {
                        break;
                    }
                    mdb_page_copy(
                        mc.mc_pg[mc.mc_top as usize],
                        mp,
                        (*(*my).mc_env).me_psize,
                    );
                }
                mc.mc_pg[mc.mc_top as usize] = mp;
                continue;
            }
        }
        if (*my).mc_wlen[toggle as usize] >= 1024 as libc::c_int * 1024 as libc::c_int {
            rc = mdb_env_cthr_toggle(my, 1 as libc::c_int);
            if rc != 0 {
                break;
            }
            toggle = (*my).mc_toggle;
        }
        mo = ((*my).mc_wbuf[toggle as usize])
            .offset((*my).mc_wlen[toggle as usize] as isize) as *mut MDB_page;
        mdb_page_copy(mo, mp, (*(*my).mc_env).me_psize);
        let fresh45 = (*my).mc_next_pgno;
        (*my).mc_next_pgno = ((*my).mc_next_pgno).wrapping_add(1);
        (*mo).mp_p.p_pgno = fresh45;
        (*my)
            .mc_wlen[toggle
            as usize] = ((*my).mc_wlen[toggle as usize] as libc::c_uint)
            .wrapping_add((*(*my).mc_env).me_psize) as libc::c_int as libc::c_int;
        if mc.mc_top != 0 {
            ni = (mc.mc_pg[(mc.mc_top as libc::c_int - 1 as libc::c_int) as usize]
                as *mut libc::c_char)
                .offset(
                    *((*mc.mc_pg[(mc.mc_top as libc::c_int - 1 as libc::c_int) as usize])
                        .mp_ptrs)
                        .as_mut_ptr()
                        .offset(
                            mc
                                .mc_ki[(mc.mc_top as libc::c_int - 1 as libc::c_int)
                                as usize] as isize,
                        ) as libc::c_int as isize,
                )
                .offset(
                    (if 0 as libc::c_int != 0 {
                        16 as libc::c_ulong as libc::c_uint
                    } else {
                        0 as libc::c_int as libc::c_uint
                    }) as isize,
                ) as *mut MDB_node;
            (*ni)
                .mn_lo = ((*mo).mp_p.p_pgno & 0xffff as libc::c_int as libc::c_ulong)
                as libc::c_ushort;
            (*ni).mn_hi = ((*mo).mp_p.p_pgno >> 16 as libc::c_int) as libc::c_ushort;
            if if -(1 as libc::c_int) as pgno_t
                > 0xffffffff as libc::c_uint as libc::c_ulong
            {
                32 as libc::c_int
            } else {
                0 as libc::c_int
            } != 0
            {
                (*ni)
                    .mn_flags = ((*mo).mp_p.p_pgno
                    >> (if -(1 as libc::c_int) as pgno_t
                        > 0xffffffff as libc::c_uint as libc::c_ulong
                    {
                        32 as libc::c_int
                    } else {
                        0 as libc::c_int
                    })) as libc::c_ushort;
            }
            mdb_cursor_pop(&mut mc);
        } else {
            *pg = (*mo).mp_p.p_pgno;
            break;
        }
    }
    free(buf as *mut libc::c_void);
    return rc;
}
unsafe extern "C" fn mdb_env_copyfd1(
    mut env: *mut MDB_env,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut mm: *mut MDB_meta = 0 as *mut MDB_meta;
    let mut mp: *mut MDB_page = 0 as *mut MDB_page;
    let mut my: mdb_copy = {
        let mut init = mdb_copy {
            mc_env: 0 as *mut MDB_env,
            mc_txn: 0 as *mut MDB_txn,
            mc_mutex: pthread_mutex_t {
                __data: __pthread_mutex_s {
                    __lock: 0,
                    __count: 0,
                    __owner: 0,
                    __nusers: 0,
                    __kind: 0,
                    __spins: 0,
                    __elision: 0,
                    __list: __pthread_list_t {
                        __prev: 0 as *mut __pthread_internal_list,
                        __next: 0 as *mut __pthread_internal_list,
                    },
                },
            },
            mc_cond: pthread_cond_t {
                __data: __pthread_cond_s {
                    c2rust_unnamed: C2RustUnnamed_1 { __wseq: 0 },
                    c2rust_unnamed_0: C2RustUnnamed { __g1_start: 0 },
                    __g_refs: [0; 2],
                    __g_size: [0; 2],
                    __g1_orig_size: 0,
                    __wrefs: 0,
                    __g_signals: [0; 2],
                },
            },
            mc_wbuf: [0 as *mut libc::c_char; 2],
            mc_over: [0 as *mut libc::c_char; 2],
            mc_wlen: [0; 2],
            mc_olen: [0; 2],
            mc_next_pgno: 0,
            mc_fd: 0,
            mc_toggle: 0,
            mc_new: 0,
            mc_error: 0,
        };
        init
    };
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut thr: pthread_t = 0;
    let mut root: pgno_t = 0;
    let mut new_root: pgno_t = 0;
    let mut rc: libc::c_int = 0 as libc::c_int;
    rc = pthread_mutex_init(&mut my.mc_mutex, 0 as *const pthread_mutexattr_t);
    if rc != 0 as libc::c_int {
        return rc;
    }
    rc = pthread_cond_init(&mut my.mc_cond, 0 as *const pthread_condattr_t);
    if !(rc != 0 as libc::c_int) {
        let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
        rc = posix_memalign(
            &mut p,
            (*env).me_os_psize as size_t,
            (1024 as libc::c_int * 1024 as libc::c_int * 2 as libc::c_int) as size_t,
        );
        if !(rc != 0 as libc::c_int) {
            my.mc_wbuf[0 as libc::c_int as usize] = p as *mut libc::c_char;
            memset(
                my.mc_wbuf[0 as libc::c_int as usize] as *mut libc::c_void,
                0 as libc::c_int,
                (1024 as libc::c_int * 1024 as libc::c_int * 2 as libc::c_int)
                    as libc::c_ulong,
            );
            my
                .mc_wbuf[1 as libc::c_int
                as usize] = (my.mc_wbuf[0 as libc::c_int as usize])
                .offset((1024 as libc::c_int * 1024 as libc::c_int) as isize);
            my.mc_next_pgno = 2 as libc::c_int as pgno_t;
            my.mc_env = env;
            my.mc_fd = fd;
            rc = pthread_create(
                &mut thr,
                0 as *const pthread_attr_t,
                Some(
                    mdb_env_copythr
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
                &mut my as *mut mdb_copy as *mut libc::c_void,
            );
            if !(rc != 0) {
                rc = mdb_txn_begin(
                    env,
                    0 as *mut MDB_txn,
                    0x20000 as libc::c_int as libc::c_uint,
                    &mut txn,
                );
                if !(rc != 0) {
                    mp = my.mc_wbuf[0 as libc::c_int as usize] as *mut MDB_page;
                    memset(
                        mp as *mut libc::c_void,
                        0 as libc::c_int,
                        (2 as libc::c_int as libc::c_uint).wrapping_mul((*env).me_psize)
                            as libc::c_ulong,
                    );
                    (*mp).mp_p.p_pgno = 0 as libc::c_int as pgno_t;
                    (*mp).mp_flags = 0x8 as libc::c_int as uint16_t;
                    mm = (mp as *mut libc::c_char)
                        .offset(16 as libc::c_ulong as libc::c_uint as isize)
                        as *mut libc::c_void as *mut MDB_meta;
                    mdb_env_init_meta0(env, mm);
                    (*mm)
                        .mm_address = (*(*env).me_metas[0 as libc::c_int as usize])
                        .mm_address;
                    mp = (my.mc_wbuf[0 as libc::c_int as usize])
                        .offset((*env).me_psize as isize) as *mut MDB_page;
                    (*mp).mp_p.p_pgno = 1 as libc::c_int as pgno_t;
                    (*mp).mp_flags = 0x8 as libc::c_int as uint16_t;
                    *((mp as *mut libc::c_char)
                        .offset(16 as libc::c_ulong as libc::c_uint as isize)
                        as *mut libc::c_void as *mut MDB_meta) = *mm;
                    mm = (mp as *mut libc::c_char)
                        .offset(16 as libc::c_ulong as libc::c_uint as isize)
                        as *mut libc::c_void as *mut MDB_meta;
                    new_root = (*((*txn).mt_dbs).offset(1 as libc::c_int as isize))
                        .md_root;
                    root = new_root;
                    if root != !(0 as libc::c_int as pgno_t) {
                        let mut freecount: MDB_ID = 0 as libc::c_int as MDB_ID;
                        let mut mc: MDB_cursor = MDB_cursor {
                            mc_next: 0 as *mut MDB_cursor,
                            mc_backup: 0 as *mut MDB_cursor,
                            mc_xcursor: 0 as *mut MDB_xcursor,
                            mc_txn: 0 as *mut MDB_txn,
                            mc_dbi: 0,
                            mc_db: 0 as *mut MDB_db,
                            mc_dbx: 0 as *mut MDB_dbx,
                            mc_dbflag: 0 as *mut libc::c_uchar,
                            mc_snum: 0,
                            mc_top: 0,
                            mc_flags: 0,
                            mc_pg: [0 as *mut MDB_page; 32],
                            mc_ki: [0; 32],
                        };
                        let mut key: MDB_val = MDB_val {
                            mv_size: 0,
                            mv_data: 0 as *mut libc::c_void,
                        };
                        let mut data: MDB_val = MDB_val {
                            mv_size: 0,
                            mv_data: 0 as *mut libc::c_void,
                        };
                        mdb_cursor_init(
                            &mut mc,
                            txn,
                            0 as libc::c_int as MDB_dbi,
                            0 as *mut MDB_xcursor,
                        );
                        loop {
                            rc = mdb_cursor_get(&mut mc, &mut key, &mut data, MDB_NEXT);
                            if !(rc == 0 as libc::c_int) {
                                break;
                            }
                            freecount = (freecount as libc::c_ulong)
                                .wrapping_add(*(data.mv_data as *mut MDB_ID)) as MDB_ID
                                as MDB_ID;
                        }
                        if rc != -(30798 as libc::c_int) {
                            current_block = 13675073358212236199;
                        } else {
                            freecount = (freecount as libc::c_ulong)
                                .wrapping_add(
                                    ((*((*txn).mt_dbs).offset(0 as libc::c_int as isize))
                                        .md_branch_pages)
                                        .wrapping_add(
                                            (*((*txn).mt_dbs).offset(0 as libc::c_int as isize))
                                                .md_leaf_pages,
                                        )
                                        .wrapping_add(
                                            (*((*txn).mt_dbs).offset(0 as libc::c_int as isize))
                                                .md_overflow_pages,
                                        ),
                                ) as MDB_ID as MDB_ID;
                            new_root = ((*txn).mt_next_pgno)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(freecount);
                            (*mm).mm_last_pg = new_root;
                            (*mm)
                                .mm_dbs[1 as libc::c_int
                                as usize] = *((*txn).mt_dbs)
                                .offset(1 as libc::c_int as isize);
                            (*mm).mm_dbs[1 as libc::c_int as usize].md_root = new_root;
                            current_block = 652864300344834934;
                        }
                    } else {
                        (*mm)
                            .mm_dbs[1 as libc::c_int as usize]
                            .md_flags = (*((*txn).mt_dbs)
                            .offset(1 as libc::c_int as isize))
                            .md_flags;
                        current_block = 652864300344834934;
                    }
                    match current_block {
                        13675073358212236199 => {}
                        _ => {
                            if root != !(0 as libc::c_int as pgno_t)
                                || (*mm).mm_dbs[1 as libc::c_int as usize].md_flags
                                    as libc::c_int != 0
                            {
                                ::std::ptr::write_volatile(
                                    &mut (*mm).mm_txnid as *mut txnid_t,
                                    1 as libc::c_int as txnid_t,
                                );
                            }
                            my
                                .mc_wlen[0 as libc::c_int
                                as usize] = ((*env).me_psize)
                                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                                as libc::c_int;
                            my.mc_txn = txn;
                            rc = mdb_env_cwalk(&mut my, &mut root, 0 as libc::c_int);
                            if rc == 0 as libc::c_int && root != new_root {
                                rc = -(30784 as libc::c_int);
                            }
                        }
                    }
                }
                if rc != 0 {
                    ::std::ptr::write_volatile(&mut my.mc_error as *mut libc::c_int, rc);
                }
                mdb_env_cthr_toggle(&mut my, 1 as libc::c_int | 0x10 as libc::c_int);
                rc = pthread_join(thr, 0 as *mut *mut libc::c_void);
                mdb_txn_abort(txn);
            }
        }
        free(my.mc_wbuf[0 as libc::c_int as usize] as *mut libc::c_void);
        pthread_cond_destroy(&mut my.mc_cond);
    }
    pthread_mutex_destroy(&mut my.mc_mutex);
    return if rc != 0 { rc } else { my.mc_error };
}
unsafe extern "C" fn mdb_env_copyfd0(
    mut env: *mut MDB_env,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut wmutex: mdb_mutexref_t = 0 as mdb_mutexref_t;
    let mut rc: libc::c_int = 0;
    let mut wsize: mdb_size_t = 0;
    let mut w3: mdb_size_t = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: ssize_t = 0;
    let mut w2: size_t = 0;
    rc = mdb_txn_begin(
        env,
        0 as *mut MDB_txn,
        0x20000 as libc::c_int as libc::c_uint,
        &mut txn,
    );
    if rc != 0 {
        return rc;
    }
    if !((*env).me_txns).is_null() {
        mdb_txn_end(txn, MDB_END_RESET_TMP as libc::c_int as libc::c_uint);
        wmutex = ((*(*env).me_txns).mt2.mt2_wmutex).as_mut_ptr();
        rc = pthread_mutex_lock(wmutex);
        if rc != 0
            && {
                rc = mdb_mutex_failed(env, wmutex, rc);
                rc != 0
            }
        {
            current_block = 1527066814328054980;
        } else {
            rc = mdb_txn_renew0(txn);
            if rc != 0 {
                pthread_mutex_unlock(wmutex);
                current_block = 1527066814328054980;
            } else {
                current_block = 11650488183268122163;
            }
        }
    } else {
        current_block = 11650488183268122163;
    }
    match current_block {
        11650488183268122163 => {
            wsize = ((*env).me_psize).wrapping_mul(2 as libc::c_int as libc::c_uint)
                as mdb_size_t;
            ptr = (*env).me_map;
            w2 = wsize;
            while w2 > 0 as libc::c_int as libc::c_ulong {
                len = write(fd, ptr as *const libc::c_void, w2);
                rc = (len >= 0 as libc::c_int as libc::c_long) as libc::c_int;
                if rc == 0 {
                    rc = *__errno_location();
                    break;
                } else if len > 0 as libc::c_int as libc::c_long {
                    rc = 0 as libc::c_int;
                    ptr = ptr.offset(len as isize);
                    w2 = (w2 as libc::c_ulong).wrapping_sub(len as libc::c_ulong)
                        as size_t as size_t;
                } else {
                    rc = 5 as libc::c_int;
                    break;
                }
            }
            if !wmutex.is_null() {
                pthread_mutex_unlock(wmutex);
            }
            if !(rc != 0) {
                w3 = ((*txn).mt_next_pgno)
                    .wrapping_mul((*env).me_psize as libc::c_ulong);
                let mut fsize: mdb_size_t = 0 as libc::c_int as mdb_size_t;
                rc = mdb_fsize((*env).me_fd, &mut fsize);
                if !(rc != 0) {
                    if w3 > fsize {
                        w3 = fsize;
                    }
                    wsize = w3.wrapping_sub(wsize);
                    while wsize > 0 as libc::c_int as libc::c_ulong {
                        if wsize
                            > (0x40000000 as libc::c_uint
                                >> (::std::mem::size_of::<ssize_t>() as libc::c_ulong
                                    == 4 as libc::c_int as libc::c_ulong) as libc::c_int)
                                as libc::c_ulong
                        {
                            w2 = (0x40000000 as libc::c_uint
                                >> (::std::mem::size_of::<ssize_t>() as libc::c_ulong
                                    == 4 as libc::c_int as libc::c_ulong) as libc::c_int)
                                as size_t;
                        } else {
                            w2 = wsize;
                        }
                        len = write(fd, ptr as *const libc::c_void, w2);
                        rc = (len >= 0 as libc::c_int as libc::c_long) as libc::c_int;
                        if rc == 0 {
                            rc = *__errno_location();
                            break;
                        } else if len > 0 as libc::c_int as libc::c_long {
                            rc = 0 as libc::c_int;
                            ptr = ptr.offset(len as isize);
                            wsize = (wsize as libc::c_ulong)
                                .wrapping_sub(len as libc::c_ulong) as mdb_size_t
                                as mdb_size_t;
                        } else {
                            rc = 5 as libc::c_int;
                            break;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    mdb_txn_abort(txn);
    return rc;
}
pub unsafe extern "C" fn mdb_env_copyfd2(
    mut env: *mut MDB_env,
    mut fd: libc::c_int,
    mut flags: libc::c_uint,
) -> libc::c_int {
    if flags & 0x1 as libc::c_int as libc::c_uint != 0 {
        return mdb_env_copyfd1(env, fd)
    } else {
        return mdb_env_copyfd0(env, fd)
    };
}
pub unsafe extern "C" fn mdb_env_copyfd(
    mut env: *mut MDB_env,
    mut fd: libc::c_int,
) -> libc::c_int {
    return mdb_env_copyfd2(env, fd, 0 as libc::c_int as libc::c_uint);
}
pub unsafe extern "C" fn mdb_env_copy2(
    mut env: *mut MDB_env,
    mut path: *const libc::c_char,
    mut flags: libc::c_uint,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut fname: MDB_name = MDB_name {
        mn_len: 0,
        mn_alloced: 0,
        mn_val: 0 as *mut mdb_nchar_t,
    };
    let mut newfd: libc::c_int = -(1 as libc::c_int);
    rc = mdb_fname_init(
        path,
        (*env).me_flags | 0x400000 as libc::c_int as libc::c_uint,
        &mut fname,
    );
    if rc == 0 as libc::c_int {
        rc = mdb_fopen(
            env,
            &mut fname,
            MDB_O_COPY,
            0o666 as libc::c_int as mdb_mode_t,
            &mut newfd,
        );
        if fname.mn_alloced != 0 {
            free(fname.mn_val as *mut libc::c_void);
        }
    }
    if rc == 0 as libc::c_int {
        rc = mdb_env_copyfd2(env, newfd, flags);
        if close(newfd) < 0 as libc::c_int && rc == 0 as libc::c_int {
            rc = *__errno_location();
        }
    }
    return rc;
}
pub unsafe extern "C" fn mdb_env_copy(
    mut env: *mut MDB_env,
    mut path: *const libc::c_char,
) -> libc::c_int {
    return mdb_env_copy2(env, path, 0 as libc::c_int as libc::c_uint);
}
pub unsafe extern "C" fn mdb_env_set_flags(
    mut env: *mut MDB_env,
    mut flag: libc::c_uint,
    mut onoff: libc::c_int,
) -> libc::c_int {
    if flag
        & !(0x10000 as libc::c_int | 0x40000 as libc::c_int | 0x100000 as libc::c_int
            | 0x1000000 as libc::c_int) as libc::c_uint != 0
    {
        return 22 as libc::c_int;
    }
    if onoff != 0 {
        (*env).me_flags |= flag;
    } else {
        (*env).me_flags &= !flag;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_get_flags(
    mut env: *mut MDB_env,
    mut arg: *mut libc::c_uint,
) -> libc::c_int {
    if env.is_null() || arg.is_null() {
        return 22 as libc::c_int;
    }
    *arg = (*env).me_flags
        & (0x10000 as libc::c_int | 0x40000 as libc::c_int | 0x100000 as libc::c_int
            | 0x1000000 as libc::c_int
            | (0x1 as libc::c_int | 0x4000 as libc::c_int | 0x20000 as libc::c_int
                | 0x80000 as libc::c_int | 0x200000 as libc::c_int
                | 0x400000 as libc::c_int | 0x800000 as libc::c_int
                | 0x2000000 as libc::c_int)) as libc::c_uint;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_set_userctx(
    mut env: *mut MDB_env,
    mut ctx: *mut libc::c_void,
) -> libc::c_int {
    if env.is_null() {
        return 22 as libc::c_int;
    }
    (*env).me_userctx = ctx;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_get_userctx(
    mut env: *mut MDB_env,
) -> *mut libc::c_void {
    return if !env.is_null() { (*env).me_userctx } else { 0 as *mut libc::c_void };
}
pub unsafe extern "C" fn mdb_env_set_assert(
    mut env: *mut MDB_env,
    mut func: Option::<MDB_assert_func>,
) -> libc::c_int {
    if env.is_null() {
        return 22 as libc::c_int;
    }
    (*env).me_assert_func = func;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_get_path(
    mut env: *mut MDB_env,
    mut arg: *mut *const libc::c_char,
) -> libc::c_int {
    if env.is_null() || arg.is_null() {
        return 22 as libc::c_int;
    }
    *arg = (*env).me_path;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_get_fd(
    mut env: *mut MDB_env,
    mut arg: *mut mdb_filehandle_t,
) -> libc::c_int {
    if env.is_null() || arg.is_null() {
        return 22 as libc::c_int;
    }
    *arg = (*env).me_fd;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_stat0(
    mut env: *mut MDB_env,
    mut db: *mut MDB_db,
    mut arg: *mut MDB_stat,
) -> libc::c_int {
    (*arg).ms_psize = (*env).me_psize;
    (*arg).ms_depth = (*db).md_depth as libc::c_uint;
    (*arg).ms_branch_pages = (*db).md_branch_pages;
    (*arg).ms_leaf_pages = (*db).md_leaf_pages;
    (*arg).ms_overflow_pages = (*db).md_overflow_pages;
    (*arg).ms_entries = (*db).md_entries;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_stat(
    mut env: *mut MDB_env,
    mut arg: *mut MDB_stat,
) -> libc::c_int {
    let mut meta: *mut MDB_meta = 0 as *mut MDB_meta;
    if env.is_null() || arg.is_null() {
        return 22 as libc::c_int;
    }
    meta = mdb_env_pick_meta(env);
    return mdb_stat0(
        env,
        &mut *((*meta).mm_dbs).as_mut_ptr().offset(1 as libc::c_int as isize),
        arg,
    );
}
pub unsafe extern "C" fn mdb_env_info(
    mut env: *mut MDB_env,
    mut arg: *mut MDB_envinfo,
) -> libc::c_int {
    let mut meta: *mut MDB_meta = 0 as *mut MDB_meta;
    if env.is_null() || arg.is_null() {
        return 22 as libc::c_int;
    }
    meta = mdb_env_pick_meta(env);
    (*arg).me_mapaddr = (*meta).mm_address;
    (*arg).me_last_pgno = (*meta).mm_last_pg;
    (*arg).me_last_txnid = (*meta).mm_txnid;
    (*arg).me_mapsize = (*env).me_mapsize;
    (*arg).me_maxreaders = (*env).me_maxreaders;
    (*arg)
        .me_numreaders = if !((*env).me_txns).is_null() {
        (*(*env).me_txns).mt1.mtb.mtb_numreaders
    } else {
        0 as libc::c_int as libc::c_uint
    };
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_default_cmp(mut txn: *mut MDB_txn, mut dbi: MDB_dbi) {
    let mut f: uint16_t = (*((*txn).mt_dbs).offset(dbi as isize)).md_flags;
    let ref mut fresh46 = (*((*txn).mt_dbxs).offset(dbi as isize)).md_cmp;
    *fresh46 = if f as libc::c_int & 0x2 as libc::c_int != 0 {
        Some(mdb_cmp_memnr as MDB_cmp_func)
    } else if f as libc::c_int & 0x8 as libc::c_int != 0 {
        Some(mdb_cmp_cint as MDB_cmp_func)
    } else {
        Some(mdb_cmp_memn as MDB_cmp_func)
    };
    let ref mut fresh47 = (*((*txn).mt_dbxs).offset(dbi as isize)).md_dcmp;
    *fresh47 = if f as libc::c_int & 0x4 as libc::c_int == 0 {
        None
    } else if f as libc::c_int & 0x20 as libc::c_int != 0 {
        if f as libc::c_int & 0x10 as libc::c_int != 0 {
            Some(mdb_cmp_int as MDB_cmp_func)
        } else {
            Some(mdb_cmp_cint as MDB_cmp_func)
        }
    } else if f as libc::c_int & 0x40 as libc::c_int != 0 {
        Some(mdb_cmp_memnr as MDB_cmp_func)
    } else {
        Some(mdb_cmp_memn as MDB_cmp_func)
    };
}
pub unsafe extern "C" fn mdb_dbi_open(
    mut txn: *mut MDB_txn,
    mut name: *const libc::c_char,
    mut flags: libc::c_uint,
    mut dbi: *mut MDB_dbi,
) -> libc::c_int {
    let mut key: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut data: MDB_val = MDB_val {
        mv_size: 0,
        mv_data: 0 as *mut libc::c_void,
    };
    let mut i: MDB_dbi = 0;
    let mut mc: MDB_cursor = MDB_cursor {
        mc_next: 0 as *mut MDB_cursor,
        mc_backup: 0 as *mut MDB_cursor,
        mc_xcursor: 0 as *mut MDB_xcursor,
        mc_txn: 0 as *mut MDB_txn,
        mc_dbi: 0,
        mc_db: 0 as *mut MDB_db,
        mc_dbx: 0 as *mut MDB_dbx,
        mc_dbflag: 0 as *mut libc::c_uchar,
        mc_snum: 0,
        mc_top: 0,
        mc_flags: 0,
        mc_pg: [0 as *mut MDB_page; 32],
        mc_ki: [0; 32],
    };
    let mut dummy: MDB_db = MDB_db {
        md_pad: 0,
        md_flags: 0,
        md_depth: 0,
        md_branch_pages: 0,
        md_leaf_pages: 0,
        md_overflow_pages: 0,
        md_entries: 0,
        md_root: 0,
    };
    let mut rc: libc::c_int = 0;
    let mut dbflag: libc::c_int = 0;
    let mut exact: libc::c_int = 0;
    let mut unused: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut seq: libc::c_uint = 0;
    let mut namedup: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    if flags
        & !(0x2 as libc::c_int | 0x4 as libc::c_int | 0x8 as libc::c_int
            | 0x10 as libc::c_int | 0x20 as libc::c_int | 0x40 as libc::c_int
            | 0x40000 as libc::c_int) as libc::c_uint != 0
    {
        return 22 as libc::c_int;
    }
    if (*txn).mt_flags
        & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
        != 0
    {
        return -(30782 as libc::c_int);
    }
    if name.is_null() {
        *dbi = 1 as libc::c_int as MDB_dbi;
        if flags & (0xffff as libc::c_int & !(0x8000 as libc::c_int)) as libc::c_uint
            != 0
        {
            let mut f2: uint16_t = (flags
                & (0xffff as libc::c_int & !(0x8000 as libc::c_int)) as libc::c_uint)
                as uint16_t;
            if (*((*txn).mt_dbs).offset(1 as libc::c_int as isize)).md_flags
                as libc::c_int | f2 as libc::c_int
                != (*((*txn).mt_dbs).offset(1 as libc::c_int as isize)).md_flags
                    as libc::c_int
            {
                let ref mut fresh48 = (*((*txn).mt_dbs)
                    .offset(1 as libc::c_int as isize))
                    .md_flags;
                *fresh48 = (*fresh48 as libc::c_int | f2 as libc::c_int) as uint16_t;
                (*txn).mt_flags |= 0x4 as libc::c_int as libc::c_uint;
            }
        }
        mdb_default_cmp(txn, 1 as libc::c_int as MDB_dbi);
        return 0 as libc::c_int;
    }
    if ((*((*txn).mt_dbxs).offset(1 as libc::c_int as isize)).md_cmp).is_none() {
        mdb_default_cmp(txn, 1 as libc::c_int as MDB_dbi);
    }
    len = strlen(name);
    i = 2 as libc::c_int as MDB_dbi;
    while i < (*txn).mt_numdbs {
        if (*((*txn).mt_dbxs).offset(i as isize)).md_name.mv_size == 0 {
            if unused == 0 {
                unused = i;
            }
        } else if len == (*((*txn).mt_dbxs).offset(i as isize)).md_name.mv_size
            && strncmp(
                name,
                (*((*txn).mt_dbxs).offset(i as isize)).md_name.mv_data
                    as *const libc::c_char,
                len,
            ) == 0
        {
            *dbi = i;
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    if unused == 0 && (*txn).mt_numdbs >= (*(*txn).mt_env).me_maxdbs {
        return -(30791 as libc::c_int);
    }
    if (*((*txn).mt_dbs).offset(1 as libc::c_int as isize)).md_flags as libc::c_int
        & (0x4 as libc::c_int | 0x8 as libc::c_int) != 0
    {
        return if flags & 0x40000 as libc::c_int as libc::c_uint != 0 {
            -(30784 as libc::c_int)
        } else {
            -(30798 as libc::c_int)
        };
    }
    dbflag = 0x4 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int;
    exact = 0 as libc::c_int;
    key.mv_size = len;
    key.mv_data = name as *mut libc::c_void;
    mdb_cursor_init(&mut mc, txn, 1 as libc::c_int as MDB_dbi, 0 as *mut MDB_xcursor);
    rc = mdb_cursor_set(&mut mc, &mut key, &mut data, MDB_SET, &mut exact);
    if rc == 0 as libc::c_int {
        let mut node: *mut MDB_node = (mc.mc_pg[mc.mc_top as usize] as *mut libc::c_char)
            .offset(
                *((*mc.mc_pg[mc.mc_top as usize]).mp_ptrs)
                    .as_mut_ptr()
                    .offset(mc.mc_ki[mc.mc_top as usize] as isize) as libc::c_int
                    as isize,
            )
            .offset(
                (if 0 as libc::c_int != 0 {
                    16 as libc::c_ulong as libc::c_uint
                } else {
                    0 as libc::c_int as libc::c_uint
                }) as isize,
            ) as *mut MDB_node;
        if (*node).mn_flags as libc::c_int & (0x4 as libc::c_int | 0x2 as libc::c_int)
            != 0x2 as libc::c_int
        {
            return -(30784 as libc::c_int);
        }
    } else {
        if rc != -(30798 as libc::c_int)
            || flags & 0x40000 as libc::c_int as libc::c_uint == 0
        {
            return rc;
        }
        if (*txn).mt_flags & 0x20000 as libc::c_int as libc::c_uint
            == 0x20000 as libc::c_int as libc::c_uint
        {
            return 13 as libc::c_int;
        }
    }
    namedup = strdup(name);
    if namedup.is_null() {
        return 12 as libc::c_int;
    }
    if rc != 0 {
        data.mv_size = ::std::mem::size_of::<MDB_db>() as libc::c_ulong;
        data.mv_data = &mut dummy as *mut MDB_db as *mut libc::c_void;
        memset(
            &mut dummy as *mut MDB_db as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<MDB_db>() as libc::c_ulong,
        );
        dummy.md_root = !(0 as libc::c_int as pgno_t);
        dummy
            .md_flags = (flags
            & (0xffff as libc::c_int & !(0x8000 as libc::c_int)) as libc::c_uint)
            as uint16_t;
        let mut dummy_0: MDB_cursor = MDB_cursor {
            mc_next: 0 as *mut MDB_cursor,
            mc_backup: 0 as *mut MDB_cursor,
            mc_xcursor: 0 as *mut MDB_xcursor,
            mc_txn: 0 as *mut MDB_txn,
            mc_dbi: 0,
            mc_db: 0 as *mut MDB_db,
            mc_dbx: 0 as *mut MDB_dbx,
            mc_dbflag: 0 as *mut libc::c_uchar,
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [0 as *mut MDB_page; 32],
            mc_ki: [0; 32],
        };
        let mut tracked: *mut MDB_cursor = 0 as *mut MDB_cursor;
        let mut tp: *mut *mut MDB_cursor = &mut *((*mc.mc_txn).mt_cursors)
            .offset(mc.mc_dbi as isize) as *mut *mut MDB_cursor;
        if mc.mc_flags & 0x4 as libc::c_int as libc::c_uint != 0 {
            dummy_0.mc_flags = 0x1 as libc::c_int as libc::c_uint;
            dummy_0.mc_xcursor = &mut mc as *mut MDB_cursor as *mut MDB_xcursor;
            tracked = &mut dummy_0;
        } else {
            tracked = &mut mc;
        }
        (*tracked).mc_next = *tp;
        *tp = tracked;
        rc = mdb_cursor_put(
            &mut mc,
            &mut key,
            &mut data,
            0x2 as libc::c_int as libc::c_uint,
        );
        *tp = (*tracked).mc_next;
        dbflag |= 0x1 as libc::c_int;
    }
    if rc != 0 {
        free(namedup as *mut libc::c_void);
    } else {
        let mut slot: libc::c_uint = if unused != 0 { unused } else { (*txn).mt_numdbs };
        let ref mut fresh49 = (*((*txn).mt_dbxs).offset(slot as isize)).md_name.mv_data;
        *fresh49 = namedup as *mut libc::c_void;
        (*((*txn).mt_dbxs).offset(slot as isize)).md_name.mv_size = len;
        let ref mut fresh50 = (*((*txn).mt_dbxs).offset(slot as isize)).md_rel;
        *fresh50 = None;
        *((*txn).mt_dbflags).offset(slot as isize) = dbflag as libc::c_uchar;
        let ref mut fresh51 = *((*(*txn).mt_env).me_dbiseqs).offset(slot as isize);
        *fresh51 = (*fresh51).wrapping_add(1);
        seq = *fresh51;
        *((*txn).mt_dbiseqs).offset(slot as isize) = seq;
        memcpy(
            &mut *((*txn).mt_dbs).offset(slot as isize) as *mut MDB_db
                as *mut libc::c_void,
            data.mv_data,
            ::std::mem::size_of::<MDB_db>() as libc::c_ulong,
        );
        *dbi = slot;
        mdb_default_cmp(txn, slot);
        if unused == 0 {
            (*txn).mt_numdbs = ((*txn).mt_numdbs).wrapping_add(1);
            (*txn).mt_numdbs;
        }
    }
    return rc;
}
pub unsafe extern "C" fn mdb_stat(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut arg: *mut MDB_stat,
) -> libc::c_int {
    if arg.is_null()
        || !(!txn.is_null() && dbi < (*txn).mt_numdbs
            && *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int
                & 0x8 as libc::c_int != 0)
    {
        return 22 as libc::c_int;
    }
    if (*txn).mt_flags
        & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
        != 0
    {
        return -(30782 as libc::c_int);
    }
    if *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int & 0x2 as libc::c_int != 0
    {
        let mut mc: MDB_cursor = MDB_cursor {
            mc_next: 0 as *mut MDB_cursor,
            mc_backup: 0 as *mut MDB_cursor,
            mc_xcursor: 0 as *mut MDB_xcursor,
            mc_txn: 0 as *mut MDB_txn,
            mc_dbi: 0,
            mc_db: 0 as *mut MDB_db,
            mc_dbx: 0 as *mut MDB_dbx,
            mc_dbflag: 0 as *mut libc::c_uchar,
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [0 as *mut MDB_page; 32],
            mc_ki: [0; 32],
        };
        let mut mx: MDB_xcursor = MDB_xcursor {
            mx_cursor: MDB_cursor {
                mc_next: 0 as *mut MDB_cursor,
                mc_backup: 0 as *mut MDB_cursor,
                mc_xcursor: 0 as *mut MDB_xcursor,
                mc_txn: 0 as *mut MDB_txn,
                mc_dbi: 0,
                mc_db: 0 as *mut MDB_db,
                mc_dbx: 0 as *mut MDB_dbx,
                mc_dbflag: 0 as *mut libc::c_uchar,
                mc_snum: 0,
                mc_top: 0,
                mc_flags: 0,
                mc_pg: [0 as *mut MDB_page; 32],
                mc_ki: [0; 32],
            },
            mx_db: MDB_db {
                md_pad: 0,
                md_flags: 0,
                md_depth: 0,
                md_branch_pages: 0,
                md_leaf_pages: 0,
                md_overflow_pages: 0,
                md_entries: 0,
                md_root: 0,
            },
            mx_dbx: MDB_dbx {
                md_name: MDB_val {
                    mv_size: 0,
                    mv_data: 0 as *mut libc::c_void,
                },
                md_cmp: None,
                md_dcmp: None,
                md_rel: None,
                md_relctx: 0 as *mut libc::c_void,
            },
            mx_dbflag: 0,
        };
        mdb_cursor_init(&mut mc, txn, dbi, &mut mx);
    }
    return mdb_stat0((*txn).mt_env, &mut *((*txn).mt_dbs).offset(dbi as isize), arg);
}
pub unsafe extern "C" fn mdb_dbi_close(mut env: *mut MDB_env, mut dbi: MDB_dbi) {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if dbi < 2 as libc::c_int as libc::c_uint || dbi >= (*env).me_maxdbs {
        return;
    }
    ptr = (*((*env).me_dbxs).offset(dbi as isize)).md_name.mv_data as *mut libc::c_char;
    if !ptr.is_null() {
        let ref mut fresh52 = (*((*env).me_dbxs).offset(dbi as isize)).md_name.mv_data;
        *fresh52 = 0 as *mut libc::c_void;
        (*((*env).me_dbxs).offset(dbi as isize))
            .md_name
            .mv_size = 0 as libc::c_int as size_t;
        *((*env).me_dbflags).offset(dbi as isize) = 0 as libc::c_int as uint16_t;
        let ref mut fresh53 = *((*env).me_dbiseqs).offset(dbi as isize);
        *fresh53 = (*fresh53).wrapping_add(1);
        *fresh53;
        free(ptr as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn mdb_dbi_flags(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut flags: *mut libc::c_uint,
) -> libc::c_int {
    if !(!txn.is_null() && dbi < (*txn).mt_numdbs
        && *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int & 0x10 as libc::c_int
            != 0)
    {
        return 22 as libc::c_int;
    }
    *flags = ((*((*txn).mt_dbs).offset(dbi as isize)).md_flags as libc::c_int
        & (0xffff as libc::c_int & !(0x8000 as libc::c_int))) as libc::c_uint;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdb_drop0(
    mut mc: *mut MDB_cursor,
    mut subs: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut rc: libc::c_int = 0;
    rc = mdb_page_search(mc, 0 as *mut MDB_val, 4 as libc::c_int);
    if rc == 0 as libc::c_int {
        let mut txn: *mut MDB_txn = (*mc).mc_txn;
        let mut ni: *mut MDB_node = 0 as *mut MDB_node;
        let mut mx: MDB_cursor = MDB_cursor {
            mc_next: 0 as *mut MDB_cursor,
            mc_backup: 0 as *mut MDB_cursor,
            mc_xcursor: 0 as *mut MDB_xcursor,
            mc_txn: 0 as *mut MDB_txn,
            mc_dbi: 0,
            mc_db: 0 as *mut MDB_db,
            mc_dbx: 0 as *mut MDB_dbx,
            mc_dbflag: 0 as *mut libc::c_uchar,
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [0 as *mut MDB_page; 32],
            mc_ki: [0; 32],
        };
        let mut i: libc::c_uint = 0;
        if (*mc).mc_flags & 0x4 as libc::c_int as libc::c_uint != 0
            || subs == 0 && (*(*mc).mc_db).md_overflow_pages == 0
        {
            mdb_cursor_pop(mc);
        }
        mdb_cursor_copy(mc, &mut mx);
        's_29: loop {
            if !((*mc).mc_snum as libc::c_int > 0 as libc::c_int) {
                current_block = 14447253356787937536;
                break;
            }
            let mut mp: *mut MDB_page = (*mc).mc_pg[(*mc).mc_top as usize];
            let mut n: libc::c_uint = ((*mp).mp_pb.pb.pb_lower as libc::c_uint)
                .wrapping_sub(
                    (16 as libc::c_ulong as libc::c_uint)
                        .wrapping_sub(
                            (if 0 as libc::c_int != 0 {
                                16 as libc::c_ulong as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }),
                        ),
                ) >> 1 as libc::c_int;
            if (*mp).mp_flags as libc::c_int & 0x2 as libc::c_int == 0x2 as libc::c_int {
                i = 0 as libc::c_int as libc::c_uint;
                while i < n {
                    ni = (mp as *mut libc::c_char)
                        .offset(
                            *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize)
                                as libc::c_int as isize,
                        )
                        .offset(
                            (if 0 as libc::c_int != 0 {
                                16 as libc::c_ulong as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) as isize,
                        ) as *mut MDB_node;
                    if (*ni).mn_flags as libc::c_int & 0x1 as libc::c_int != 0 {
                        let mut omp: *mut MDB_page = 0 as *mut MDB_page;
                        let mut pg: pgno_t = 0;
                        memcpy(
                            &mut pg as *mut pgno_t as *mut libc::c_void,
                            ((*ni).mn_data)
                                .as_mut_ptr()
                                .offset((*ni).mn_ksize as libc::c_int as isize)
                                as *mut libc::c_void,
                            ::std::mem::size_of::<pgno_t>() as libc::c_ulong,
                        );
                        rc = mdb_page_get(mc, pg, &mut omp, 0 as *mut libc::c_int);
                        if rc != 0 as libc::c_int {
                            current_block = 17640278311867083205;
                            break 's_29;
                        }
                        if (*omp).mp_flags as libc::c_int & 0x4 as libc::c_int
                            == 0x4 as libc::c_int
                        {} else {
                            mdb_assert_fail(
                                (*(*mc).mc_txn).mt_env,
                                b"IS_OVERFLOW(omp)\0" as *const u8 as *const libc::c_char,
                                (*::std::mem::transmute::<
                                    &[u8; 10],
                                    &[libc::c_char; 10],
                                >(b"mdb_drop0\0"))
                                    .as_ptr(),
                                b"mdb.c\0" as *const u8 as *const libc::c_char,
                                10975 as libc::c_int,
                            );
                        };
                        rc = mdb_midl_append_range(
                            &mut (*txn).mt_free_pgs,
                            pg,
                            (*omp).mp_pb.pb_pages,
                        );
                        if rc != 0 {
                            current_block = 17640278311867083205;
                            break 's_29;
                        }
                        (*(*mc).mc_db)
                            .md_overflow_pages = ((*(*mc).mc_db).md_overflow_pages
                            as libc::c_ulong)
                            .wrapping_sub((*omp).mp_pb.pb_pages as libc::c_ulong)
                            as pgno_t as pgno_t;
                        if (*(*mc).mc_db).md_overflow_pages == 0 && subs == 0 {
                            break;
                        }
                    } else if subs != 0
                        && (*ni).mn_flags as libc::c_int & 0x2 as libc::c_int != 0
                    {
                        mdb_xcursor_init1(mc, ni);
                        rc = mdb_drop0(
                            &mut (*(*mc).mc_xcursor).mx_cursor,
                            0 as libc::c_int,
                        );
                        if rc != 0 {
                            current_block = 17640278311867083205;
                            break 's_29;
                        }
                    }
                    i = i.wrapping_add(1);
                    i;
                }
                if subs == 0 && (*(*mc).mc_db).md_overflow_pages == 0 {
                    current_block = 12804655452731335028;
                } else {
                    current_block = 14434620278749266018;
                }
            } else {
                rc = mdb_midl_need(&mut (*txn).mt_free_pgs, n);
                if rc != 0 as libc::c_int {
                    current_block = 17640278311867083205;
                    break;
                }
                i = 0 as libc::c_int as libc::c_uint;
                while i < n {
                    let mut pg_0: pgno_t = 0;
                    ni = (mp as *mut libc::c_char)
                        .offset(
                            *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize)
                                as libc::c_int as isize,
                        )
                        .offset(
                            (if 0 as libc::c_int != 0 {
                                16 as libc::c_ulong as libc::c_uint
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) as isize,
                        ) as *mut MDB_node;
                    pg_0 = (*ni).mn_lo as libc::c_ulong
                        | ((*ni).mn_hi as pgno_t) << 16 as libc::c_int
                        | (if (if -(1 as libc::c_int) as pgno_t
                            > 0xffffffff as libc::c_uint as libc::c_ulong
                        {
                            32 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) != 0
                        {
                            ((*ni).mn_flags as pgno_t)
                                << (if -(1 as libc::c_int) as pgno_t
                                    > 0xffffffff as libc::c_uint as libc::c_ulong
                                {
                                    32 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                        } else {
                            0 as libc::c_int as libc::c_ulong
                        });
                    let mut xidl: *mut MDB_ID = (*txn).mt_free_pgs;
                    let ref mut fresh54 = *xidl.offset(0 as libc::c_int as isize);
                    *fresh54 = (*fresh54).wrapping_add(1);
                    let mut xlen: MDB_ID = *fresh54;
                    *xidl.offset(xlen as isize) = pg_0;
                    i = i.wrapping_add(1);
                    i;
                }
                current_block = 14434620278749266018;
            }
            match current_block {
                14434620278749266018 => {
                    if (*mc).mc_top == 0 {
                        current_block = 14447253356787937536;
                        break;
                    }
                    (*mc).mc_ki[(*mc).mc_top as usize] = i as indx_t;
                    rc = mdb_cursor_sibling(mc, 1 as libc::c_int);
                    if !(rc != 0) {
                        continue;
                    }
                    if rc != -(30798 as libc::c_int) {
                        current_block = 17640278311867083205;
                        break;
                    }
                }
                _ => {}
            }
            mdb_cursor_pop(mc);
            (*mc).mc_ki[0 as libc::c_int as usize] = 0 as libc::c_int as indx_t;
            i = 1 as libc::c_int as libc::c_uint;
            while i < (*mc).mc_snum as libc::c_uint {
                (*mc).mc_ki[i as usize] = 0 as libc::c_int as indx_t;
                (*mc).mc_pg[i as usize] = mx.mc_pg[i as usize];
                i = i.wrapping_add(1);
                i;
            }
        }
        match current_block {
            14447253356787937536 => {
                rc = mdb_midl_append(&mut (*txn).mt_free_pgs, (*(*mc).mc_db).md_root);
            }
            _ => {}
        }
        if rc != 0 {
            (*txn).mt_flags |= 0x2 as libc::c_int as libc::c_uint;
        }
    } else if rc == -(30798 as libc::c_int) {
        rc = 0 as libc::c_int;
    }
    (*mc).mc_flags &= !(0x1 as libc::c_int) as libc::c_uint;
    return rc;
}
pub unsafe extern "C" fn mdb_drop(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut del: libc::c_int,
) -> libc::c_int {
    let mut mc: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut m2: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut rc: libc::c_int = 0;
    if del as libc::c_uint > 1 as libc::c_int as libc::c_uint
        || !(!txn.is_null() && dbi < (*txn).mt_numdbs
            && *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int
                & 0x10 as libc::c_int != 0)
    {
        return 22 as libc::c_int;
    }
    if (*txn).mt_flags & 0x20000 as libc::c_int as libc::c_uint
        == 0x20000 as libc::c_int as libc::c_uint
    {
        return 13 as libc::c_int;
    }
    if *((*txn).mt_dbiseqs).offset(dbi as isize)
        != *((*(*txn).mt_env).me_dbiseqs).offset(dbi as isize)
    {
        return -(30780 as libc::c_int);
    }
    rc = mdb_cursor_open(txn, dbi, &mut mc);
    if rc != 0 {
        return rc;
    }
    rc = mdb_drop0(mc, (*(*mc).mc_db).md_flags as libc::c_int & 0x4 as libc::c_int);
    m2 = *((*txn).mt_cursors).offset(dbi as isize);
    while !m2.is_null() {
        (*m2).mc_flags &= !(0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint;
        m2 = (*m2).mc_next;
    }
    if !(rc != 0) {
        if del != 0 && dbi >= 2 as libc::c_int as libc::c_uint {
            rc = mdb_del0(
                txn,
                1 as libc::c_int as MDB_dbi,
                &mut (*(*mc).mc_dbx).md_name,
                0 as *mut MDB_val,
                0x2 as libc::c_int as libc::c_uint,
            );
            if rc == 0 {
                *((*txn).mt_dbflags)
                    .offset(dbi as isize) = 0x2 as libc::c_int as libc::c_uchar;
                mdb_dbi_close((*txn).mt_env, dbi);
            } else {
                (*txn).mt_flags |= 0x2 as libc::c_int as libc::c_uint;
            }
        } else {
            let ref mut fresh55 = *((*txn).mt_dbflags).offset(dbi as isize);
            *fresh55 = (*fresh55 as libc::c_int | 0x1 as libc::c_int) as libc::c_uchar;
            (*((*txn).mt_dbs).offset(dbi as isize))
                .md_depth = 0 as libc::c_int as uint16_t;
            (*((*txn).mt_dbs).offset(dbi as isize))
                .md_branch_pages = 0 as libc::c_int as pgno_t;
            (*((*txn).mt_dbs).offset(dbi as isize))
                .md_leaf_pages = 0 as libc::c_int as pgno_t;
            (*((*txn).mt_dbs).offset(dbi as isize))
                .md_overflow_pages = 0 as libc::c_int as pgno_t;
            (*((*txn).mt_dbs).offset(dbi as isize))
                .md_entries = 0 as libc::c_int as mdb_size_t;
            (*((*txn).mt_dbs).offset(dbi as isize))
                .md_root = !(0 as libc::c_int as pgno_t);
            (*txn).mt_flags |= 0x4 as libc::c_int as libc::c_uint;
        }
    }
    mdb_cursor_close(mc);
    return rc;
}
pub unsafe extern "C" fn mdb_set_compare(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut cmp: Option::<MDB_cmp_func>,
) -> libc::c_int {
    if !(!txn.is_null() && dbi < (*txn).mt_numdbs
        && *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int & 0x10 as libc::c_int
            != 0)
    {
        return 22 as libc::c_int;
    }
    let ref mut fresh56 = (*((*txn).mt_dbxs).offset(dbi as isize)).md_cmp;
    *fresh56 = cmp;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_set_dupsort(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut cmp: Option::<MDB_cmp_func>,
) -> libc::c_int {
    if !(!txn.is_null() && dbi < (*txn).mt_numdbs
        && *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int & 0x10 as libc::c_int
            != 0)
    {
        return 22 as libc::c_int;
    }
    let ref mut fresh57 = (*((*txn).mt_dbxs).offset(dbi as isize)).md_dcmp;
    *fresh57 = cmp;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_set_relfunc(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut rel: Option::<MDB_rel_func>,
) -> libc::c_int {
    if !(!txn.is_null() && dbi < (*txn).mt_numdbs
        && *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int & 0x10 as libc::c_int
            != 0)
    {
        return 22 as libc::c_int;
    }
    let ref mut fresh58 = (*((*txn).mt_dbxs).offset(dbi as isize)).md_rel;
    *fresh58 = rel;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_set_relctx(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut ctx: *mut libc::c_void,
) -> libc::c_int {
    if !(!txn.is_null() && dbi < (*txn).mt_numdbs
        && *((*txn).mt_dbflags).offset(dbi as isize) as libc::c_int & 0x10 as libc::c_int
            != 0)
    {
        return 22 as libc::c_int;
    }
    let ref mut fresh59 = (*((*txn).mt_dbxs).offset(dbi as isize)).md_relctx;
    *fresh59 = ctx;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_env_get_maxkeysize(mut env: *mut MDB_env) -> libc::c_int {
    return if 0 as libc::c_int != 0 { 0 as libc::c_int } else { 511 as libc::c_int };
}
pub unsafe extern "C" fn mdb_reader_list(
    mut env: *mut MDB_env,
    mut func: Option::<MDB_msg_func>,
    mut ctx: *mut libc::c_void,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut rdrs: libc::c_uint = 0;
    let mut mr: *mut MDB_reader = 0 as *mut MDB_reader;
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut first: libc::c_int = 1 as libc::c_int;
    if env.is_null() || func.is_none() {
        return -(1 as libc::c_int);
    }
    if ((*env).me_txns).is_null() {
        return func
            .unwrap()(b"(no reader locks)\n\0" as *const u8 as *const libc::c_char, ctx);
    }
    rdrs = (*(*env).me_txns).mt1.mtb.mtb_numreaders;
    mr = ((*(*env).me_txns).mti_readers).as_mut_ptr();
    i = 0 as libc::c_int as libc::c_uint;
    while i < rdrs {
        if (*mr.offset(i as isize)).mru.mrx.mrb_pid != 0 {
            let mut txnid: txnid_t = (*mr.offset(i as isize)).mru.mrx.mrb_txnid;
            sprintf(
                buf.as_mut_ptr(),
                if txnid == -(1 as libc::c_int) as txnid_t {
                    b"%10d %zx -\n\0" as *const u8 as *const libc::c_char
                } else {
                    b"%10d %zx %zu\n\0" as *const u8 as *const libc::c_char
                },
                (*mr.offset(i as isize)).mru.mrx.mrb_pid,
                (*mr.offset(i as isize)).mru.mrx.mrb_tid,
                txnid,
            );
            if first != 0 {
                first = 0 as libc::c_int;
                rc = func
                    .unwrap()(
                    b"    pid     thread     txnid\n\0" as *const u8
                        as *const libc::c_char,
                    ctx,
                );
                if rc < 0 as libc::c_int {
                    break;
                }
            }
            rc = func.unwrap()(buf.as_mut_ptr(), ctx);
            if rc < 0 as libc::c_int {
                break;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if first != 0 {
        rc = func
            .unwrap()(
            b"(no active readers)\n\0" as *const u8 as *const libc::c_char,
            ctx,
        );
    }
    return rc;
}
unsafe extern "C" fn mdb_pid_insert(mut ids: *mut pid_t, mut pid: pid_t) -> libc::c_int {
    let mut base: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut cursor: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    let mut val: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_uint = *ids.offset(0 as libc::c_int as isize) as libc::c_uint;
    while (0 as libc::c_int as libc::c_uint) < n {
        let mut pivot: libc::c_uint = n >> 1 as libc::c_int;
        cursor = base.wrapping_add(pivot).wrapping_add(1 as libc::c_int as libc::c_uint);
        val = pid - *ids.offset(cursor as isize);
        if val < 0 as libc::c_int {
            n = pivot;
        } else if val > 0 as libc::c_int {
            base = cursor;
            n = n.wrapping_sub(pivot.wrapping_add(1 as libc::c_int as libc::c_uint));
        } else {
            return -(1 as libc::c_int)
        }
    }
    if val > 0 as libc::c_int {
        cursor = cursor.wrapping_add(1);
        cursor;
    }
    let ref mut fresh60 = *ids.offset(0 as libc::c_int as isize);
    *fresh60 += 1;
    *fresh60;
    n = *ids.offset(0 as libc::c_int as isize) as libc::c_uint;
    while n > cursor {
        *ids
            .offset(
                n as isize,
            ) = *ids.offset(n.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
        n = n.wrapping_sub(1);
        n;
    }
    *ids.offset(n as isize) = pid;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_reader_check(
    mut env: *mut MDB_env,
    mut dead: *mut libc::c_int,
) -> libc::c_int {
    if env.is_null() {
        return 22 as libc::c_int;
    }
    if !dead.is_null() {
        *dead = 0 as libc::c_int;
    }
    return if !((*env).me_txns).is_null() {
        mdb_reader_check0(env, 0 as libc::c_int, dead)
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn mdb_reader_check0(
    mut env: *mut MDB_env,
    mut rlocked: libc::c_int,
    mut dead: *mut libc::c_int,
) -> libc::c_int {
    let mut rmutex: mdb_mutexref_t = if rlocked != 0 {
        0 as *mut pthread_mutex_t
    } else {
        ((*(*env).me_txns).mt1.mtb.mtb_rmutex).as_mut_ptr()
    };
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut rdrs: libc::c_uint = 0;
    let mut mr: *mut MDB_reader = 0 as *mut MDB_reader;
    let mut pids: *mut pid_t = 0 as *mut pid_t;
    let mut pid: pid_t = 0;
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = 0 as libc::c_int;
    rdrs = (*(*env).me_txns).mt1.mtb.mtb_numreaders;
    pids = malloc(
        (rdrs.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<pid_t>() as libc::c_ulong),
    ) as *mut pid_t;
    if pids.is_null() {
        return 12 as libc::c_int;
    }
    *pids.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    mr = ((*(*env).me_txns).mti_readers).as_mut_ptr();
    i = 0 as libc::c_int as libc::c_uint;
    while i < rdrs {
        pid = (*mr.offset(i as isize)).mru.mrx.mrb_pid;
        if pid != 0 && pid != (*env).me_pid {
            if mdb_pid_insert(pids, pid) == 0 as libc::c_int {
                if mdb_reader_pid(env, Pidcheck, pid) == 0 {
                    j = i;
                    if !rmutex.is_null() {
                        rc = pthread_mutex_lock(rmutex);
                        if rc != 0 as libc::c_int {
                            rc = mdb_mutex_failed(env, rmutex, rc);
                            if rc != 0 {
                                break;
                            }
                            rdrs = 0 as libc::c_int as libc::c_uint;
                        } else if mdb_reader_pid(env, Pidcheck, pid) != 0 {
                            j = rdrs;
                        }
                    }
                    while j < rdrs {
                        if (*mr.offset(j as isize)).mru.mrx.mrb_pid == pid {
                            ::std::ptr::write_volatile(
                                &mut (*mr.offset(j as isize)).mru.mrx.mrb_pid as *mut pid_t,
                                0 as libc::c_int,
                            );
                            count += 1;
                            count;
                        }
                        j = j.wrapping_add(1);
                        j;
                    }
                    if !rmutex.is_null() {
                        pthread_mutex_unlock(rmutex);
                    }
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    free(pids as *mut libc::c_void);
    if !dead.is_null() {
        *dead = count;
    }
    return rc;
}
unsafe extern "C" fn mdb_mutex_failed(
    mut env: *mut MDB_env,
    mut mutex: mdb_mutexref_t,
    mut rc: libc::c_int,
) -> libc::c_int {
    let mut rlocked: libc::c_int = 0;
    let mut rc2: libc::c_int = 0;
    let mut meta: *mut MDB_meta = 0 as *mut MDB_meta;
    if rc == 130 as libc::c_int {
        rc = 0 as libc::c_int;
        rlocked = (mutex == ((*(*env).me_txns).mt1.mtb.mtb_rmutex).as_mut_ptr())
            as libc::c_int;
        if rlocked == 0 {
            meta = mdb_env_pick_meta(env);
            ::std::ptr::write_volatile(
                &mut (*(*env).me_txns).mt1.mtb.mtb_txnid as *mut txnid_t,
                (*meta).mm_txnid,
            );
            if !((*env).me_txn).is_null() {
                (*env).me_flags |= 0x80000000 as libc::c_uint;
                (*env).me_txn = 0 as *mut MDB_txn;
                rc = -(30795 as libc::c_int);
            }
        }
        rc2 = mdb_reader_check0(env, rlocked, 0 as *mut libc::c_int);
        if rc2 == 0 as libc::c_int {
            rc2 = pthread_mutex_consistent(mutex);
        }
        if rc != 0
            || {
                rc = rc2;
                rc != 0
            }
        {
            pthread_mutex_unlock(mutex);
        }
    }
    return rc;
}
