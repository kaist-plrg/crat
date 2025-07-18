use ::libc;
use ::c2rust_bitfields;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type gcry_cipher_handle;
    pub type ao_device;
    pub type AVOptionRanges;
    pub type AVOption;
    pub type AVBuffer;
    pub type AVDictionary;
    pub type AVCodecInternal;
    pub type MpegEncContext;
    pub type AVCodecHWConfigInternal;
    pub type AVCodecDefault;
    pub type AVFormatInternal;
    pub type AVStreamInternal;
    pub type AVDeviceCapabilitiesQuery;
    pub type AVDeviceInfoList;
    pub type AVCodecTag;
    pub type AVFilterCommand;
    pub type AVFilterInternal;
    pub type AVFilterChannelLayouts;
    pub type AVFilterFormats;
    pub type AVFilterPad;
    pub type AVFilterGraphInternal;
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    static mut stdout: *mut FILE;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execl(
        __path: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fork() -> __pid_t;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
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
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn gcry_check_version(req_version: *const libc::c_char) -> *const libc::c_char;
    fn gcry_control(CMD: gcry_ctl_cmds, _: ...) -> gcry_error_t;
    fn PianoInit(
        _: *mut PianoHandle_t,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *const libc::c_char,
    ) -> PianoReturn_t;
    fn PianoDestroy(_: *mut PianoHandle_t);
    fn PianoDestroyPlaylist(_: *mut PianoSong_t);
    fn PianoFindStationById(
        _: *mut PianoStation_t,
        _: *const libc::c_char,
    ) -> *mut PianoStation_t;
    fn PianoErrorToStr(_: PianoReturn_t) -> *const libc::c_char;
    fn curl_easy_init() -> *mut libc::c_void;
    fn curl_global_init(flags: libc::c_long) -> CURLcode;
    fn curl_global_cleanup();
    fn curl_easy_cleanup(curl: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn BarSettingsInit(_: *mut BarSettings_t);
    fn BarSettingsDestroy(_: *mut BarSettings_t);
    fn BarSettingsRead(_: *mut BarSettings_t);
    fn BarSettingsWrite(_: *mut PianoStation_t, _: *mut BarSettings_t);
    fn BarPlayerThread(data: *mut libc::c_void) -> *mut libc::c_void;
    fn BarPlayerInit(p: *mut player_t, settings: *const BarSettings_t);
    fn BarPlayerReset(p: *mut player_t);
    fn BarPlayerDestroy(p: *mut player_t);
    fn BarPlayerGetMode(player: *mut player_t) -> BarPlayerMode;
    fn BarReadline(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: *mut BarReadlineFds_t,
        _: BarReadlineFlags_t,
        _: libc::c_int,
    ) -> size_t;
    fn BarReadlineStr(
        _: *mut libc::c_char,
        _: size_t,
        _: *mut BarReadlineFds_t,
        _: BarReadlineFlags_t,
    ) -> size_t;
    fn BarTermInit();
    fn BarTermRestore();
    fn BarUiMsg(_: *const BarSettings_t, _: BarUiMsg_t, _: *const libc::c_char, _: ...);
    fn BarUiSelectStation(
        _: *mut BarApp_t,
        _: *mut PianoStation_t,
        _: *const libc::c_char,
        _: BarUiSelectStationCallback_t,
        _: bool,
    ) -> *mut PianoStation_t;
    fn BarUiPrintStation(_: *const BarSettings_t, _: *mut PianoStation_t);
    fn BarUiPrintSong(
        _: *const BarSettings_t,
        _: *const PianoSong_t,
        _: *const PianoStation_t,
    );
    fn BarUiStartEventCmd(
        _: *const BarSettings_t,
        _: *const libc::c_char,
        _: *const PianoStation_t,
        _: *const PianoSong_t,
        _: *mut player_t,
        _: *mut PianoStation_t,
        _: PianoReturn_t,
        _: CURLcode,
    );
    fn BarUiPianoCall(
        _: *mut BarApp_t,
        _: PianoRequestType_t,
        _: *mut libc::c_void,
        _: *mut PianoReturn_t,
        _: *mut CURLcode,
    ) -> bool;
    fn BarUiHistoryPrepend(app: *mut BarApp_t, song: *mut PianoSong_t);
    fn BarUiCustomFormat(
        dest: *mut libc::c_char,
        destSize: size_t,
        format: *const libc::c_char,
        formatChars: *const libc::c_char,
        formatVals: *mut *const libc::c_char,
    );
    fn BarUiDispatch(
        _: *mut BarApp_t,
        _: libc::c_char,
        _: *mut PianoStation_t,
        _: *mut PianoSong_t,
        _: bool,
        _: BarUiDispatchContext_t,
    ) -> BarKeyShortcutId_t;
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
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
pub type pid_t = __pid_t;
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
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type int8_t = __int8_t;
pub type int64_t = __int64_t;
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
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_12,
    pub _timer: C2RustUnnamed_11,
    pub _rt: C2RustUnnamed_10,
    pub _sigchld: C2RustUnnamed_9,
    pub _sigfault: C2RustUnnamed_6,
    pub _sigpoll: C2RustUnnamed_5,
    pub _sigsys: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub _addr_bnd: C2RustUnnamed_8,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_13,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
pub type gpg_error_t = libc::c_uint;
pub type gcry_error_t = gpg_error_t;
pub type gcry_ctl_cmds = libc::c_uint;
pub const GCRYCTL_REINIT_SYSCALL_CLAMP: gcry_ctl_cmds = 77;
pub const GCRYCTL_GET_TAGLEN: gcry_ctl_cmds = 76;
pub const GCRYCTL_SET_TAGLEN: gcry_ctl_cmds = 75;
pub const GCRYCTL_DRBG_REINIT: gcry_ctl_cmds = 74;
pub const GCRYCTL_SET_SBOX: gcry_ctl_cmds = 73;
pub const GCRYCTL_REACTIVATE_FIPS_FLAG: gcry_ctl_cmds = 72;
pub const GCRYCTL_INACTIVATE_FIPS_FLAG: gcry_ctl_cmds = 71;
pub const GCRYCTL_CLOSE_RANDOM_DEVICE: gcry_ctl_cmds = 70;
pub const GCRYCTL_SET_CCM_LENGTHS: gcry_ctl_cmds = 69;
pub const GCRYCTL_DISABLE_PRIV_DROP: gcry_ctl_cmds = 68;
pub const GCRYCTL_DISABLE_LOCKED_SECMEM: gcry_ctl_cmds = 67;
pub const GCRYCTL_GET_CURRENT_RNG_TYPE: gcry_ctl_cmds = 66;
pub const GCRYCTL_SET_PREFERRED_RNG_TYPE: gcry_ctl_cmds = 65;
pub const GCRYCTL_SET_ENFORCED_FIPS_FLAG: gcry_ctl_cmds = 64;
pub const GCRYCTL_DISABLE_HWF: gcry_ctl_cmds = 63;
pub const GCRYCTL_SELFTEST: gcry_ctl_cmds = 57;
pub const GCRYCTL_FORCE_FIPS_MODE: gcry_ctl_cmds = 56;
pub const GCRYCTL_FIPS_MODE_P: gcry_ctl_cmds = 55;
pub const GCRYCTL_OPERATIONAL_P: gcry_ctl_cmds = 54;
pub const GCRYCTL_PRINT_CONFIG: gcry_ctl_cmds = 53;
pub const GCRYCTL_SET_RNDEGD_SOCKET: gcry_ctl_cmds = 52;
pub const GCRYCTL_FAKED_RANDOM_P: gcry_ctl_cmds = 51;
pub const GCRYCTL_USE_RANDOM_DAEMON: gcry_ctl_cmds = 50;
pub const GCRYCTL_SET_RANDOM_DAEMON_SOCKET: gcry_ctl_cmds = 49;
pub const GCRYCTL_FAST_POLL: gcry_ctl_cmds = 48;
pub const GCRYCTL_SET_THREAD_CBS: gcry_ctl_cmds = 47;
pub const GCRYCTL_UPDATE_RANDOM_SEED_FILE: gcry_ctl_cmds = 46;
pub const GCRYCTL_SET_RANDOM_SEED_FILE: gcry_ctl_cmds = 45;
pub const GCRYCTL_ENABLE_QUICK_RANDOM: gcry_ctl_cmds = 44;
pub const GCRYCTL_SET_CBC_MAC: gcry_ctl_cmds = 42;
pub const GCRYCTL_SET_CBC_CTS: gcry_ctl_cmds = 41;
pub const GCRYCTL_ANY_INITIALIZATION_P: gcry_ctl_cmds = 40;
pub const GCRYCTL_INITIALIZATION_FINISHED_P: gcry_ctl_cmds = 39;
pub const GCRYCTL_INITIALIZATION_FINISHED: gcry_ctl_cmds = 38;
pub const GCRYCTL_DISABLE_SECMEM: gcry_ctl_cmds = 37;
pub const GCRYCTL_DISABLE_INTERNAL_LOCKING: gcry_ctl_cmds = 36;
pub const GCRYCTL_IS_ALGO_ENABLED: gcry_ctl_cmds = 35;
pub const GCRYCTL_GET_ALGO_USAGE: gcry_ctl_cmds = 34;
pub const GCRYCTL_STOP_DUMP: gcry_ctl_cmds = 33;
pub const GCRYCTL_START_DUMP: gcry_ctl_cmds = 32;
pub const GCRYCTL_ENABLE_M_GUARD: gcry_ctl_cmds = 31;
pub const GCRYCTL_DROP_PRIVS: gcry_ctl_cmds = 30;
pub const GCRYCTL_RESUME_SECMEM_WARN: gcry_ctl_cmds = 29;
pub const GCRYCTL_SUSPEND_SECMEM_WARN: gcry_ctl_cmds = 28;
pub const GCRYCTL_DISABLE_SECMEM_WARN: gcry_ctl_cmds = 27;
pub const GCRYCTL_TERM_SECMEM: gcry_ctl_cmds = 25;
pub const GCRYCTL_INIT_SECMEM: gcry_ctl_cmds = 24;
pub const GCRYCTL_DUMP_MEMORY_STATS: gcry_ctl_cmds = 23;
pub const GCRYCTL_USE_SECURE_RNDPOOL: gcry_ctl_cmds = 22;
pub const GCRYCTL_CLEAR_DEBUG_FLAGS: gcry_ctl_cmds = 21;
pub const GCRYCTL_SET_DEBUG_FLAGS: gcry_ctl_cmds = 20;
pub const GCRYCTL_SET_VERBOSITY: gcry_ctl_cmds = 19;
pub const GCRYCTL_GET_ALGO_NENCR: gcry_ctl_cmds = 18;
pub const GCRYCTL_GET_ALGO_NSIGN: gcry_ctl_cmds = 17;
pub const GCRYCTL_GET_ALGO_NSKEY: gcry_ctl_cmds = 16;
pub const GCRYCTL_GET_ALGO_NPKEY: gcry_ctl_cmds = 15;
pub const GCRYCTL_DUMP_SECMEM_STATS: gcry_ctl_cmds = 14;
pub const GCRYCTL_DUMP_RANDOM_STATS: gcry_ctl_cmds = 13;
pub const GCRYCTL_DISABLE_ALGO: gcry_ctl_cmds = 12;
pub const GCRYCTL_ENABLE_ALGO: gcry_ctl_cmds = 11;
pub const GCRYCTL_GET_ASNOID: gcry_ctl_cmds = 10;
pub const GCRYCTL_IS_SECURE: gcry_ctl_cmds = 9;
pub const GCRYCTL_TEST_ALGO: gcry_ctl_cmds = 8;
pub const GCRYCTL_GET_BLKLEN: gcry_ctl_cmds = 7;
pub const GCRYCTL_GET_KEYLEN: gcry_ctl_cmds = 6;
pub const GCRYCTL_FINALIZE: gcry_ctl_cmds = 5;
pub const GCRYCTL_RESET: gcry_ctl_cmds = 4;
pub const GCRYCTL_CFB_SYNC: gcry_ctl_cmds = 3;
pub type gcry_cipher_hd_t = *mut gcry_cipher_handle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoListHead {
    pub next: *mut PianoListHead,
}
pub type PianoListHead_t = PianoListHead;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoUserInfo {
    pub listenerId: *mut libc::c_char,
    pub authToken: *mut libc::c_char,
}
pub type PianoUserInfo_t = PianoUserInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoStation {
    pub head: PianoListHead_t,
    pub isCreator: libc::c_char,
    pub isQuickMix: libc::c_char,
    pub useQuickMix: libc::c_char,
    pub name: *mut libc::c_char,
    pub id: *mut libc::c_char,
    pub seedId: *mut libc::c_char,
}
pub type PianoStation_t = PianoStation;
pub type PianoSongRating_t = libc::c_uint;
pub const PIANO_RATE_TIRED: PianoSongRating_t = 3;
pub const PIANO_RATE_BAN: PianoSongRating_t = 2;
pub const PIANO_RATE_LOVE: PianoSongRating_t = 1;
pub const PIANO_RATE_NONE: PianoSongRating_t = 0;
pub type PianoAudioFormat_t = libc::c_uint;
pub const PIANO_AF_MP3: PianoAudioFormat_t = 2;
pub const PIANO_AF_AACPLUS: PianoAudioFormat_t = 1;
pub const PIANO_AF_UNKNOWN: PianoAudioFormat_t = 0;
pub type PianoAudioQuality_t = libc::c_uint;
pub const PIANO_AQ_HIGH: PianoAudioQuality_t = 3;
pub const PIANO_AQ_MEDIUM: PianoAudioQuality_t = 2;
pub const PIANO_AQ_LOW: PianoAudioQuality_t = 1;
pub const PIANO_AQ_UNKNOWN: PianoAudioQuality_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoSong {
    pub head: PianoListHead_t,
    pub artist: *mut libc::c_char,
    pub stationId: *mut libc::c_char,
    pub album: *mut libc::c_char,
    pub audioUrl: *mut libc::c_char,
    pub coverArt: *mut libc::c_char,
    pub musicId: *mut libc::c_char,
    pub title: *mut libc::c_char,
    pub seedId: *mut libc::c_char,
    pub feedbackId: *mut libc::c_char,
    pub detailUrl: *mut libc::c_char,
    pub trackToken: *mut libc::c_char,
    pub fileGain: libc::c_float,
    pub length: libc::c_uint,
    pub rating: PianoSongRating_t,
    pub audioFormat: PianoAudioFormat_t,
}
pub type PianoSong_t = PianoSong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoGenre {
    pub head: PianoListHead_t,
    pub name: *mut libc::c_char,
    pub musicId: *mut libc::c_char,
}
pub type PianoGenre_t = PianoGenre;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoGenreCategory {
    pub head: PianoListHead_t,
    pub name: *mut libc::c_char,
    pub genres: *mut PianoGenre_t,
}
pub type PianoGenreCategory_t = PianoGenreCategory;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoPartner {
    pub in_0: gcry_cipher_hd_t,
    pub out: gcry_cipher_hd_t,
    pub authToken: *mut libc::c_char,
    pub device: *mut libc::c_char,
    pub user: *mut libc::c_char,
    pub password: *mut libc::c_char,
    pub id: libc::c_uint,
}
pub type PianoPartner_t = PianoPartner;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoHandle {
    pub user: PianoUserInfo_t,
    pub stations: *mut PianoStation_t,
    pub genreStations: *mut PianoGenreCategory_t,
    pub partner: PianoPartner_t,
    pub timeOffset: libc::c_int,
}
pub type PianoHandle_t = PianoHandle;
pub type PianoRequestType_t = libc::c_uint;
pub const PIANO_REQUEST_SET_STATION_MODE: PianoRequestType_t = 26;
pub const PIANO_REQUEST_GET_STATION_MODES: PianoRequestType_t = 25;
pub const PIANO_REQUEST_CHANGE_SETTINGS: PianoRequestType_t = 24;
pub const PIANO_REQUEST_GET_SETTINGS: PianoRequestType_t = 23;
pub const PIANO_REQUEST_DELETE_SEED: PianoRequestType_t = 22;
pub const PIANO_REQUEST_DELETE_FEEDBACK: PianoRequestType_t = 21;
pub const PIANO_REQUEST_GET_STATION_INFO: PianoRequestType_t = 20;
pub const PIANO_REQUEST_BOOKMARK_ARTIST: PianoRequestType_t = 19;
pub const PIANO_REQUEST_BOOKMARK_SONG: PianoRequestType_t = 18;
pub const PIANO_REQUEST_EXPLAIN: PianoRequestType_t = 16;
pub const PIANO_REQUEST_TRANSFORM_STATION: PianoRequestType_t = 15;
pub const PIANO_REQUEST_GET_GENRE_STATIONS: PianoRequestType_t = 14;
pub const PIANO_REQUEST_SET_QUICKMIX: PianoRequestType_t = 13;
pub const PIANO_REQUEST_ADD_TIRED_SONG: PianoRequestType_t = 12;
pub const PIANO_REQUEST_ADD_SEED: PianoRequestType_t = 11;
pub const PIANO_REQUEST_CREATE_STATION: PianoRequestType_t = 10;
pub const PIANO_REQUEST_SEARCH: PianoRequestType_t = 9;
pub const PIANO_REQUEST_DELETE_STATION: PianoRequestType_t = 8;
pub const PIANO_REQUEST_RENAME_STATION: PianoRequestType_t = 7;
pub const PIANO_REQUEST_ADD_FEEDBACK: PianoRequestType_t = 5;
pub const PIANO_REQUEST_RATE_SONG: PianoRequestType_t = 4;
pub const PIANO_REQUEST_GET_PLAYLIST: PianoRequestType_t = 3;
pub const PIANO_REQUEST_GET_STATIONS: PianoRequestType_t = 2;
pub const PIANO_REQUEST_LOGIN: PianoRequestType_t = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoRequestDataLogin_t {
    pub user: *mut libc::c_char,
    pub password: *mut libc::c_char,
    pub step: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoRequestDataGetPlaylist_t {
    pub station: *mut PianoStation_t,
    pub quality: PianoAudioQuality_t,
    pub retPlaylist: *mut PianoSong_t,
}
pub type PianoReturn_t = libc::c_uint;
pub const PIANO_RET_P_RATE_LIMIT: PianoReturn_t = 2063;
pub const PIANO_RET_P_ZIP_CODE_INVALID: PianoReturn_t = 2048;
pub const PIANO_RET_P_USER_NOT_AUTHORIZED: PianoReturn_t = 2028;
pub const PIANO_RET_P_LISTENER_NOT_AUTHORIZED: PianoReturn_t = 2027;
pub const PIANO_RET_P_USER_ALREADY_USED_TRIAL: PianoReturn_t = 2061;
pub const PIANO_RET_P_USERNAME_ALREADY_EXISTS: PianoReturn_t = 2037;
pub const PIANO_RET_P_URL_PARAM_MISSING_USER_ID: PianoReturn_t = 1029;
pub const PIANO_RET_P_URL_PARAM_MISSING_PARTNER_ID: PianoReturn_t = 1028;
pub const PIANO_RET_P_URL_PARAM_MISSING_METHOD: PianoReturn_t = 1026;
pub const PIANO_RET_P_URL_PARAM_MISSING_AUTH_TOKEN: PianoReturn_t = 1027;
pub const PIANO_RET_P_UPGRADE_DEVICE_MODEL_INVALID: PianoReturn_t = 2039;
pub const PIANO_RET_P_STATION_DOES_NOT_EXIST: PianoReturn_t = 2030;
pub const PIANO_RET_P_SECURE_PROTOCOL_REQUIRED: PianoReturn_t = 1030;
pub const PIANO_RET_P_READ_ONLY_MODE: PianoReturn_t = 2024;
pub const PIANO_RET_P_PARTNER_NOT_AUTHORIZED: PianoReturn_t = 2034;
pub const PIANO_RET_P_PARAMETER_VALUE_INVALID: PianoReturn_t = 1034;
pub const PIANO_RET_P_PARAMETER_TYPE_MISMATCH: PianoReturn_t = 1032;
pub const PIANO_RET_P_PARAMETER_MISSING: PianoReturn_t = 1033;
pub const PIANO_RET_P_MAX_STATIONS_REACHED: PianoReturn_t = 2029;
pub const PIANO_RET_P_MAINTENANCE_MODE: PianoReturn_t = 1025;
pub const PIANO_RET_P_LICENSING_RESTRICTIONS: PianoReturn_t = 1036;
pub const PIANO_RET_P_INVALID_USERNAME: PianoReturn_t = 2035;
pub const PIANO_RET_P_INVALID_SPONSOR: PianoReturn_t = 2060;
pub const PIANO_RET_P_INVALID_PASSWORD: PianoReturn_t = 2036;
pub const PIANO_RET_P_INVALID_PARTNER_LOGIN: PianoReturn_t = 2026;
pub const PIANO_RET_P_INVALID_GENDER: PianoReturn_t = 2051;
pub const PIANO_RET_P_INVALID_COUNTRY_CODE: PianoReturn_t = 2051;
pub const PIANO_RET_P_INVALID_AUTH_TOKEN: PianoReturn_t = 2025;
pub const PIANO_RET_P_INSUFFICIENT_CONNECTIVITY: PianoReturn_t = 1037;
pub const PIANO_RET_P_EXPLICIT_PIN_MALFORMED: PianoReturn_t = 2044;
pub const PIANO_RET_P_EXPLICIT_PIN_INCORRECT: PianoReturn_t = 2042;
pub const PIANO_RET_P_DEVICE_NOT_FOUND: PianoReturn_t = 2033;
pub const PIANO_RET_P_DEVICE_MODEL_INVALID: PianoReturn_t = 2047;
pub const PIANO_RET_P_DEVICE_DISABLED: PianoReturn_t = 2058;
pub const PIANO_RET_P_DEVICE_ALREADY_ASSOCIATED_TO_ACCOUNT: PianoReturn_t = 2038;
pub const PIANO_RET_P_DAILY_TRIAL_LIMIT_REACHED: PianoReturn_t = 2059;
pub const PIANO_RET_P_COMPLIMENTARY_PERIOD_ALREADY_IN_USE: PianoReturn_t = 2031;
pub const PIANO_RET_P_CERTIFICATE_REQUIRED: PianoReturn_t = 1031;
pub const PIANO_RET_P_CALL_NOT_ALLOWED: PianoReturn_t = 2032;
pub const PIANO_RET_P_BIRTH_YEAR_TOO_YOUNG: PianoReturn_t = 2050;
pub const PIANO_RET_P_BIRTH_YEAR_INVALID: PianoReturn_t = 2049;
pub const PIANO_RET_P_API_VERSION_NOT_SUPPORTED: PianoReturn_t = 1035;
pub const PIANO_RET_P_INTERNAL: PianoReturn_t = 1024;
pub const PIANO_RET_GCRY_ERR: PianoReturn_t = 7;
pub const PIANO_RET_QUALITY_UNAVAILABLE: PianoReturn_t = 6;
pub const PIANO_RET_INVALID_LOGIN: PianoReturn_t = 5;
pub const PIANO_RET_OUT_OF_MEMORY: PianoReturn_t = 4;
pub const PIANO_RET_CONTINUE_REQUEST: PianoReturn_t = 3;
pub const PIANO_RET_INVALID_RESPONSE: PianoReturn_t = 2;
pub const PIANO_RET_OK: PianoReturn_t = 1;
pub const PIANO_RET_ERR: PianoReturn_t = 0;
pub type CURL = ();
pub type CURLcode = libc::c_uint;
pub const CURL_LAST: CURLcode = 96;
pub const CURLE_HTTP3: CURLcode = 95;
pub const CURLE_AUTH_ERROR: CURLcode = 94;
pub const CURLE_RECURSIVE_API_CALL: CURLcode = 93;
pub const CURLE_HTTP2_STREAM: CURLcode = 92;
pub const CURLE_SSL_INVALIDCERTSTATUS: CURLcode = 91;
pub const CURLE_SSL_PINNEDPUBKEYNOTMATCH: CURLcode = 90;
pub const CURLE_NO_CONNECTION_AVAILABLE: CURLcode = 89;
pub const CURLE_CHUNK_FAILED: CURLcode = 88;
pub const CURLE_FTP_BAD_FILE_LIST: CURLcode = 87;
pub const CURLE_RTSP_SESSION_ERROR: CURLcode = 86;
pub const CURLE_RTSP_CSEQ_ERROR: CURLcode = 85;
pub const CURLE_FTP_PRET_FAILED: CURLcode = 84;
pub const CURLE_SSL_ISSUER_ERROR: CURLcode = 83;
pub const CURLE_SSL_CRL_BADFILE: CURLcode = 82;
pub const CURLE_AGAIN: CURLcode = 81;
pub const CURLE_SSL_SHUTDOWN_FAILED: CURLcode = 80;
pub const CURLE_SSH: CURLcode = 79;
pub const CURLE_REMOTE_FILE_NOT_FOUND: CURLcode = 78;
pub const CURLE_SSL_CACERT_BADFILE: CURLcode = 77;
pub const CURLE_CONV_REQD: CURLcode = 76;
pub const CURLE_CONV_FAILED: CURLcode = 75;
pub const CURLE_TFTP_NOSUCHUSER: CURLcode = 74;
pub const CURLE_REMOTE_FILE_EXISTS: CURLcode = 73;
pub const CURLE_TFTP_UNKNOWNID: CURLcode = 72;
pub const CURLE_TFTP_ILLEGAL: CURLcode = 71;
pub const CURLE_REMOTE_DISK_FULL: CURLcode = 70;
pub const CURLE_TFTP_PERM: CURLcode = 69;
pub const CURLE_TFTP_NOTFOUND: CURLcode = 68;
pub const CURLE_LOGIN_DENIED: CURLcode = 67;
pub const CURLE_SSL_ENGINE_INITFAILED: CURLcode = 66;
pub const CURLE_SEND_FAIL_REWIND: CURLcode = 65;
pub const CURLE_USE_SSL_FAILED: CURLcode = 64;
pub const CURLE_FILESIZE_EXCEEDED: CURLcode = 63;
pub const CURLE_LDAP_INVALID_URL: CURLcode = 62;
pub const CURLE_BAD_CONTENT_ENCODING: CURLcode = 61;
pub const CURLE_PEER_FAILED_VERIFICATION: CURLcode = 60;
pub const CURLE_SSL_CIPHER: CURLcode = 59;
pub const CURLE_SSL_CERTPROBLEM: CURLcode = 58;
pub const CURLE_OBSOLETE57: CURLcode = 57;
pub const CURLE_RECV_ERROR: CURLcode = 56;
pub const CURLE_SEND_ERROR: CURLcode = 55;
pub const CURLE_SSL_ENGINE_SETFAILED: CURLcode = 54;
pub const CURLE_SSL_ENGINE_NOTFOUND: CURLcode = 53;
pub const CURLE_GOT_NOTHING: CURLcode = 52;
pub const CURLE_OBSOLETE51: CURLcode = 51;
pub const CURLE_OBSOLETE50: CURLcode = 50;
pub const CURLE_TELNET_OPTION_SYNTAX: CURLcode = 49;
pub const CURLE_UNKNOWN_OPTION: CURLcode = 48;
pub const CURLE_TOO_MANY_REDIRECTS: CURLcode = 47;
pub const CURLE_OBSOLETE46: CURLcode = 46;
pub const CURLE_INTERFACE_FAILED: CURLcode = 45;
pub const CURLE_OBSOLETE44: CURLcode = 44;
pub const CURLE_BAD_FUNCTION_ARGUMENT: CURLcode = 43;
pub const CURLE_ABORTED_BY_CALLBACK: CURLcode = 42;
pub const CURLE_FUNCTION_NOT_FOUND: CURLcode = 41;
pub const CURLE_OBSOLETE40: CURLcode = 40;
pub const CURLE_LDAP_SEARCH_FAILED: CURLcode = 39;
pub const CURLE_LDAP_CANNOT_BIND: CURLcode = 38;
pub const CURLE_FILE_COULDNT_READ_FILE: CURLcode = 37;
pub const CURLE_BAD_DOWNLOAD_RESUME: CURLcode = 36;
pub const CURLE_SSL_CONNECT_ERROR: CURLcode = 35;
pub const CURLE_HTTP_POST_ERROR: CURLcode = 34;
pub const CURLE_RANGE_ERROR: CURLcode = 33;
pub const CURLE_OBSOLETE32: CURLcode = 32;
pub const CURLE_FTP_COULDNT_USE_REST: CURLcode = 31;
pub const CURLE_FTP_PORT_FAILED: CURLcode = 30;
pub const CURLE_OBSOLETE29: CURLcode = 29;
pub const CURLE_OPERATION_TIMEDOUT: CURLcode = 28;
pub const CURLE_OUT_OF_MEMORY: CURLcode = 27;
pub const CURLE_READ_ERROR: CURLcode = 26;
pub const CURLE_UPLOAD_FAILED: CURLcode = 25;
pub const CURLE_OBSOLETE24: CURLcode = 24;
pub const CURLE_WRITE_ERROR: CURLcode = 23;
pub const CURLE_HTTP_RETURNED_ERROR: CURLcode = 22;
pub const CURLE_QUOTE_ERROR: CURLcode = 21;
pub const CURLE_OBSOLETE20: CURLcode = 20;
pub const CURLE_FTP_COULDNT_RETR_FILE: CURLcode = 19;
pub const CURLE_PARTIAL_FILE: CURLcode = 18;
pub const CURLE_FTP_COULDNT_SET_TYPE: CURLcode = 17;
pub const CURLE_HTTP2: CURLcode = 16;
pub const CURLE_FTP_CANT_GET_HOST: CURLcode = 15;
pub const CURLE_FTP_WEIRD_227_FORMAT: CURLcode = 14;
pub const CURLE_FTP_WEIRD_PASV_REPLY: CURLcode = 13;
pub const CURLE_FTP_ACCEPT_TIMEOUT: CURLcode = 12;
pub const CURLE_FTP_WEIRD_PASS_REPLY: CURLcode = 11;
pub const CURLE_FTP_ACCEPT_FAILED: CURLcode = 10;
pub const CURLE_REMOTE_ACCESS_DENIED: CURLcode = 9;
pub const CURLE_WEIRD_SERVER_REPLY: CURLcode = 8;
pub const CURLE_COULDNT_CONNECT: CURLcode = 7;
pub const CURLE_COULDNT_RESOLVE_HOST: CURLcode = 6;
pub const CURLE_COULDNT_RESOLVE_PROXY: CURLcode = 5;
pub const CURLE_NOT_BUILT_IN: CURLcode = 4;
pub const CURLE_URL_MALFORMAT: CURLcode = 3;
pub const CURLE_FAILED_INIT: CURLcode = 2;
pub const CURLE_UNSUPPORTED_PROTOCOL: CURLcode = 1;
pub const CURLE_OK: CURLcode = 0;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type AVMediaType = libc::c_int;
pub const AVMEDIA_TYPE_NB: AVMediaType = 5;
pub const AVMEDIA_TYPE_ATTACHMENT: AVMediaType = 4;
pub const AVMEDIA_TYPE_SUBTITLE: AVMediaType = 3;
pub const AVMEDIA_TYPE_DATA: AVMediaType = 2;
pub const AVMEDIA_TYPE_AUDIO: AVMediaType = 1;
pub const AVMEDIA_TYPE_VIDEO: AVMediaType = 0;
pub const AVMEDIA_TYPE_UNKNOWN: AVMediaType = -1;
pub type AVPictureType = libc::c_uint;
pub const AV_PICTURE_TYPE_BI: AVPictureType = 7;
pub const AV_PICTURE_TYPE_SP: AVPictureType = 6;
pub const AV_PICTURE_TYPE_SI: AVPictureType = 5;
pub const AV_PICTURE_TYPE_S: AVPictureType = 4;
pub const AV_PICTURE_TYPE_B: AVPictureType = 3;
pub const AV_PICTURE_TYPE_P: AVPictureType = 2;
pub const AV_PICTURE_TYPE_I: AVPictureType = 1;
pub const AV_PICTURE_TYPE_NONE: AVPictureType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: libc::c_int,
    pub den: libc::c_int,
}
pub type AVClassCategory = libc::c_uint;
pub const AV_CLASS_CATEGORY_NB: AVClassCategory = 46;
pub const AV_CLASS_CATEGORY_DEVICE_INPUT: AVClassCategory = 45;
pub const AV_CLASS_CATEGORY_DEVICE_OUTPUT: AVClassCategory = 44;
pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT: AVClassCategory = 43;
pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT: AVClassCategory = 42;
pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT: AVClassCategory = 41;
pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT: AVClassCategory = 40;
pub const AV_CLASS_CATEGORY_SWRESAMPLER: AVClassCategory = 10;
pub const AV_CLASS_CATEGORY_SWSCALER: AVClassCategory = 9;
pub const AV_CLASS_CATEGORY_BITSTREAM_FILTER: AVClassCategory = 8;
pub const AV_CLASS_CATEGORY_FILTER: AVClassCategory = 7;
pub const AV_CLASS_CATEGORY_DECODER: AVClassCategory = 6;
pub const AV_CLASS_CATEGORY_ENCODER: AVClassCategory = 5;
pub const AV_CLASS_CATEGORY_DEMUXER: AVClassCategory = 4;
pub const AV_CLASS_CATEGORY_MUXER: AVClassCategory = 3;
pub const AV_CLASS_CATEGORY_OUTPUT: AVClassCategory = 2;
pub const AV_CLASS_CATEGORY_INPUT: AVClassCategory = 1;
pub const AV_CLASS_CATEGORY_NA: AVClassCategory = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVClass {
    pub class_name: *const libc::c_char,
    pub item_name: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *const libc::c_char,
    >,
    pub option: *const AVOption,
    pub version: libc::c_int,
    pub log_level_offset_offset: libc::c_int,
    pub parent_log_context_offset: libc::c_int,
    pub child_next: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
    >,
    pub child_class_next: Option::<
        unsafe extern "C" fn(*const AVClass) -> *const AVClass,
    >,
    pub category: AVClassCategory,
    pub get_category: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> AVClassCategory,
    >,
    pub query_ranges: Option::<
        unsafe extern "C" fn(
            *mut *mut AVOptionRanges,
            *mut libc::c_void,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
}
pub type AVPixelFormat = libc::c_int;
pub const AV_PIX_FMT_NB: AVPixelFormat = 193;
pub const AV_PIX_FMT_NV42: AVPixelFormat = 192;
pub const AV_PIX_FMT_NV24: AVPixelFormat = 191;
pub const AV_PIX_FMT_YUVA444P12LE: AVPixelFormat = 190;
pub const AV_PIX_FMT_YUVA444P12BE: AVPixelFormat = 189;
pub const AV_PIX_FMT_YUVA422P12LE: AVPixelFormat = 188;
pub const AV_PIX_FMT_YUVA422P12BE: AVPixelFormat = 187;
pub const AV_PIX_FMT_GRAYF32LE: AVPixelFormat = 186;
pub const AV_PIX_FMT_GRAYF32BE: AVPixelFormat = 185;
pub const AV_PIX_FMT_GRAY14LE: AVPixelFormat = 184;
pub const AV_PIX_FMT_GRAY14BE: AVPixelFormat = 183;
pub const AV_PIX_FMT_OPENCL: AVPixelFormat = 182;
pub const AV_PIX_FMT_DRM_PRIME: AVPixelFormat = 181;
pub const AV_PIX_FMT_GBRAPF32LE: AVPixelFormat = 180;
pub const AV_PIX_FMT_GBRAPF32BE: AVPixelFormat = 179;
pub const AV_PIX_FMT_GBRPF32LE: AVPixelFormat = 178;
pub const AV_PIX_FMT_GBRPF32BE: AVPixelFormat = 177;
pub const AV_PIX_FMT_GRAY9LE: AVPixelFormat = 176;
pub const AV_PIX_FMT_GRAY9BE: AVPixelFormat = 175;
pub const AV_PIX_FMT_D3D11: AVPixelFormat = 174;
pub const AV_PIX_FMT_P016BE: AVPixelFormat = 173;
pub const AV_PIX_FMT_P016LE: AVPixelFormat = 172;
pub const AV_PIX_FMT_GRAY10LE: AVPixelFormat = 171;
pub const AV_PIX_FMT_GRAY10BE: AVPixelFormat = 170;
pub const AV_PIX_FMT_GRAY12LE: AVPixelFormat = 169;
pub const AV_PIX_FMT_GRAY12BE: AVPixelFormat = 168;
pub const AV_PIX_FMT_MEDIACODEC: AVPixelFormat = 167;
pub const AV_PIX_FMT_GBRAP10LE: AVPixelFormat = 166;
pub const AV_PIX_FMT_GBRAP10BE: AVPixelFormat = 165;
pub const AV_PIX_FMT_GBRAP12LE: AVPixelFormat = 164;
pub const AV_PIX_FMT_GBRAP12BE: AVPixelFormat = 163;
pub const AV_PIX_FMT_P010BE: AVPixelFormat = 162;
pub const AV_PIX_FMT_P010LE: AVPixelFormat = 161;
pub const AV_PIX_FMT_VIDEOTOOLBOX: AVPixelFormat = 160;
pub const AV_PIX_FMT_AYUV64BE: AVPixelFormat = 159;
pub const AV_PIX_FMT_AYUV64LE: AVPixelFormat = 158;
pub const AV_PIX_FMT_YUV440P12BE: AVPixelFormat = 157;
pub const AV_PIX_FMT_YUV440P12LE: AVPixelFormat = 156;
pub const AV_PIX_FMT_YUV440P10BE: AVPixelFormat = 155;
pub const AV_PIX_FMT_YUV440P10LE: AVPixelFormat = 154;
pub const AV_PIX_FMT_XVMC: AVPixelFormat = 153;
pub const AV_PIX_FMT_BAYER_GRBG16BE: AVPixelFormat = 152;
pub const AV_PIX_FMT_BAYER_GRBG16LE: AVPixelFormat = 151;
pub const AV_PIX_FMT_BAYER_GBRG16BE: AVPixelFormat = 150;
pub const AV_PIX_FMT_BAYER_GBRG16LE: AVPixelFormat = 149;
pub const AV_PIX_FMT_BAYER_RGGB16BE: AVPixelFormat = 148;
pub const AV_PIX_FMT_BAYER_RGGB16LE: AVPixelFormat = 147;
pub const AV_PIX_FMT_BAYER_BGGR16BE: AVPixelFormat = 146;
pub const AV_PIX_FMT_BAYER_BGGR16LE: AVPixelFormat = 145;
pub const AV_PIX_FMT_BAYER_GRBG8: AVPixelFormat = 144;
pub const AV_PIX_FMT_BAYER_GBRG8: AVPixelFormat = 143;
pub const AV_PIX_FMT_BAYER_RGGB8: AVPixelFormat = 142;
pub const AV_PIX_FMT_BAYER_BGGR8: AVPixelFormat = 141;
pub const AV_PIX_FMT_YUVJ411P: AVPixelFormat = 140;
pub const AV_PIX_FMT_GBRP14LE: AVPixelFormat = 139;
pub const AV_PIX_FMT_GBRP14BE: AVPixelFormat = 138;
pub const AV_PIX_FMT_GBRP12LE: AVPixelFormat = 137;
pub const AV_PIX_FMT_GBRP12BE: AVPixelFormat = 136;
pub const AV_PIX_FMT_YUV444P14LE: AVPixelFormat = 135;
pub const AV_PIX_FMT_YUV444P14BE: AVPixelFormat = 134;
pub const AV_PIX_FMT_YUV444P12LE: AVPixelFormat = 133;
pub const AV_PIX_FMT_YUV444P12BE: AVPixelFormat = 132;
pub const AV_PIX_FMT_YUV422P14LE: AVPixelFormat = 131;
pub const AV_PIX_FMT_YUV422P14BE: AVPixelFormat = 130;
pub const AV_PIX_FMT_YUV422P12LE: AVPixelFormat = 129;
pub const AV_PIX_FMT_YUV422P12BE: AVPixelFormat = 128;
pub const AV_PIX_FMT_YUV420P14LE: AVPixelFormat = 127;
pub const AV_PIX_FMT_YUV420P14BE: AVPixelFormat = 126;
pub const AV_PIX_FMT_YUV420P12LE: AVPixelFormat = 125;
pub const AV_PIX_FMT_YUV420P12BE: AVPixelFormat = 124;
pub const AV_PIX_FMT_BGR0: AVPixelFormat = 123;
pub const AV_PIX_FMT_0BGR: AVPixelFormat = 122;
pub const AV_PIX_FMT_RGB0: AVPixelFormat = 121;
pub const AV_PIX_FMT_0RGB: AVPixelFormat = 120;
pub const AV_PIX_FMT_CUDA: AVPixelFormat = 119;
pub const AV_PIX_FMT_D3D11VA_VLD: AVPixelFormat = 118;
pub const AV_PIX_FMT_MMAL: AVPixelFormat = 117;
pub const AV_PIX_FMT_QSV: AVPixelFormat = 116;
pub const AV_PIX_FMT_GBRAP16LE: AVPixelFormat = 115;
pub const AV_PIX_FMT_GBRAP16BE: AVPixelFormat = 114;
pub const AV_PIX_FMT_GBRAP: AVPixelFormat = 113;
pub const AV_PIX_FMT_YA16LE: AVPixelFormat = 112;
pub const AV_PIX_FMT_YA16BE: AVPixelFormat = 111;
pub const AV_PIX_FMT_YVYU422: AVPixelFormat = 110;
pub const AV_PIX_FMT_BGRA64LE: AVPixelFormat = 109;
pub const AV_PIX_FMT_BGRA64BE: AVPixelFormat = 108;
pub const AV_PIX_FMT_RGBA64LE: AVPixelFormat = 107;
pub const AV_PIX_FMT_RGBA64BE: AVPixelFormat = 106;
pub const AV_PIX_FMT_NV20BE: AVPixelFormat = 105;
pub const AV_PIX_FMT_NV20LE: AVPixelFormat = 104;
pub const AV_PIX_FMT_NV16: AVPixelFormat = 103;
pub const AV_PIX_FMT_XYZ12BE: AVPixelFormat = 102;
pub const AV_PIX_FMT_XYZ12LE: AVPixelFormat = 101;
pub const AV_PIX_FMT_VDPAU: AVPixelFormat = 100;
pub const AV_PIX_FMT_YUVA444P16LE: AVPixelFormat = 99;
pub const AV_PIX_FMT_YUVA444P16BE: AVPixelFormat = 98;
pub const AV_PIX_FMT_YUVA422P16LE: AVPixelFormat = 97;
pub const AV_PIX_FMT_YUVA422P16BE: AVPixelFormat = 96;
pub const AV_PIX_FMT_YUVA420P16LE: AVPixelFormat = 95;
pub const AV_PIX_FMT_YUVA420P16BE: AVPixelFormat = 94;
pub const AV_PIX_FMT_YUVA444P10LE: AVPixelFormat = 93;
pub const AV_PIX_FMT_YUVA444P10BE: AVPixelFormat = 92;
pub const AV_PIX_FMT_YUVA422P10LE: AVPixelFormat = 91;
pub const AV_PIX_FMT_YUVA422P10BE: AVPixelFormat = 90;
pub const AV_PIX_FMT_YUVA420P10LE: AVPixelFormat = 89;
pub const AV_PIX_FMT_YUVA420P10BE: AVPixelFormat = 88;
pub const AV_PIX_FMT_YUVA444P9LE: AVPixelFormat = 87;
pub const AV_PIX_FMT_YUVA444P9BE: AVPixelFormat = 86;
pub const AV_PIX_FMT_YUVA422P9LE: AVPixelFormat = 85;
pub const AV_PIX_FMT_YUVA422P9BE: AVPixelFormat = 84;
pub const AV_PIX_FMT_YUVA420P9LE: AVPixelFormat = 83;
pub const AV_PIX_FMT_YUVA420P9BE: AVPixelFormat = 82;
pub const AV_PIX_FMT_YUVA444P: AVPixelFormat = 81;
pub const AV_PIX_FMT_YUVA422P: AVPixelFormat = 80;
pub const AV_PIX_FMT_GBRP16LE: AVPixelFormat = 79;
pub const AV_PIX_FMT_GBRP16BE: AVPixelFormat = 78;
pub const AV_PIX_FMT_GBRP10LE: AVPixelFormat = 77;
pub const AV_PIX_FMT_GBRP10BE: AVPixelFormat = 76;
pub const AV_PIX_FMT_GBRP9LE: AVPixelFormat = 75;
pub const AV_PIX_FMT_GBRP9BE: AVPixelFormat = 74;
pub const AV_PIX_FMT_GBR24P: AVPixelFormat = 73;
pub const AV_PIX_FMT_GBRP: AVPixelFormat = 73;
pub const AV_PIX_FMT_YUV422P9LE: AVPixelFormat = 72;
pub const AV_PIX_FMT_YUV422P9BE: AVPixelFormat = 71;
pub const AV_PIX_FMT_YUV444P10LE: AVPixelFormat = 70;
pub const AV_PIX_FMT_YUV444P10BE: AVPixelFormat = 69;
pub const AV_PIX_FMT_YUV444P9LE: AVPixelFormat = 68;
pub const AV_PIX_FMT_YUV444P9BE: AVPixelFormat = 67;
pub const AV_PIX_FMT_YUV422P10LE: AVPixelFormat = 66;
pub const AV_PIX_FMT_YUV422P10BE: AVPixelFormat = 65;
pub const AV_PIX_FMT_YUV420P10LE: AVPixelFormat = 64;
pub const AV_PIX_FMT_YUV420P10BE: AVPixelFormat = 63;
pub const AV_PIX_FMT_YUV420P9LE: AVPixelFormat = 62;
pub const AV_PIX_FMT_YUV420P9BE: AVPixelFormat = 61;
pub const AV_PIX_FMT_BGR48LE: AVPixelFormat = 60;
pub const AV_PIX_FMT_BGR48BE: AVPixelFormat = 59;
pub const AV_PIX_FMT_GRAY8A: AVPixelFormat = 58;
pub const AV_PIX_FMT_Y400A: AVPixelFormat = 58;
pub const AV_PIX_FMT_YA8: AVPixelFormat = 58;
pub const AV_PIX_FMT_BGR444BE: AVPixelFormat = 57;
pub const AV_PIX_FMT_BGR444LE: AVPixelFormat = 56;
pub const AV_PIX_FMT_RGB444BE: AVPixelFormat = 55;
pub const AV_PIX_FMT_RGB444LE: AVPixelFormat = 54;
pub const AV_PIX_FMT_DXVA2_VLD: AVPixelFormat = 53;
pub const AV_PIX_FMT_YUV444P16BE: AVPixelFormat = 52;
pub const AV_PIX_FMT_YUV444P16LE: AVPixelFormat = 51;
pub const AV_PIX_FMT_YUV422P16BE: AVPixelFormat = 50;
pub const AV_PIX_FMT_YUV422P16LE: AVPixelFormat = 49;
pub const AV_PIX_FMT_YUV420P16BE: AVPixelFormat = 48;
pub const AV_PIX_FMT_YUV420P16LE: AVPixelFormat = 47;
pub const AV_PIX_FMT_VAAPI: AVPixelFormat = 46;
pub const AV_PIX_FMT_VAAPI_VLD: AVPixelFormat = 46;
pub const AV_PIX_FMT_VAAPI_IDCT: AVPixelFormat = 45;
pub const AV_PIX_FMT_VAAPI_MOCO: AVPixelFormat = 44;
pub const AV_PIX_FMT_BGR555LE: AVPixelFormat = 43;
pub const AV_PIX_FMT_BGR555BE: AVPixelFormat = 42;
pub const AV_PIX_FMT_BGR565LE: AVPixelFormat = 41;
pub const AV_PIX_FMT_BGR565BE: AVPixelFormat = 40;
pub const AV_PIX_FMT_RGB555LE: AVPixelFormat = 39;
pub const AV_PIX_FMT_RGB555BE: AVPixelFormat = 38;
pub const AV_PIX_FMT_RGB565LE: AVPixelFormat = 37;
pub const AV_PIX_FMT_RGB565BE: AVPixelFormat = 36;
pub const AV_PIX_FMT_RGB48LE: AVPixelFormat = 35;
pub const AV_PIX_FMT_RGB48BE: AVPixelFormat = 34;
pub const AV_PIX_FMT_YUVA420P: AVPixelFormat = 33;
pub const AV_PIX_FMT_YUVJ440P: AVPixelFormat = 32;
pub const AV_PIX_FMT_YUV440P: AVPixelFormat = 31;
pub const AV_PIX_FMT_GRAY16LE: AVPixelFormat = 30;
pub const AV_PIX_FMT_GRAY16BE: AVPixelFormat = 29;
pub const AV_PIX_FMT_BGRA: AVPixelFormat = 28;
pub const AV_PIX_FMT_ABGR: AVPixelFormat = 27;
pub const AV_PIX_FMT_RGBA: AVPixelFormat = 26;
pub const AV_PIX_FMT_ARGB: AVPixelFormat = 25;
pub const AV_PIX_FMT_NV21: AVPixelFormat = 24;
pub const AV_PIX_FMT_NV12: AVPixelFormat = 23;
pub const AV_PIX_FMT_RGB4_BYTE: AVPixelFormat = 22;
pub const AV_PIX_FMT_RGB4: AVPixelFormat = 21;
pub const AV_PIX_FMT_RGB8: AVPixelFormat = 20;
pub const AV_PIX_FMT_BGR4_BYTE: AVPixelFormat = 19;
pub const AV_PIX_FMT_BGR4: AVPixelFormat = 18;
pub const AV_PIX_FMT_BGR8: AVPixelFormat = 17;
pub const AV_PIX_FMT_UYYVYY411: AVPixelFormat = 16;
pub const AV_PIX_FMT_UYVY422: AVPixelFormat = 15;
pub const AV_PIX_FMT_YUVJ444P: AVPixelFormat = 14;
pub const AV_PIX_FMT_YUVJ422P: AVPixelFormat = 13;
pub const AV_PIX_FMT_YUVJ420P: AVPixelFormat = 12;
pub const AV_PIX_FMT_PAL8: AVPixelFormat = 11;
pub const AV_PIX_FMT_MONOBLACK: AVPixelFormat = 10;
pub const AV_PIX_FMT_MONOWHITE: AVPixelFormat = 9;
pub const AV_PIX_FMT_GRAY8: AVPixelFormat = 8;
pub const AV_PIX_FMT_YUV411P: AVPixelFormat = 7;
pub const AV_PIX_FMT_YUV410P: AVPixelFormat = 6;
pub const AV_PIX_FMT_YUV444P: AVPixelFormat = 5;
pub const AV_PIX_FMT_YUV422P: AVPixelFormat = 4;
pub const AV_PIX_FMT_BGR24: AVPixelFormat = 3;
pub const AV_PIX_FMT_RGB24: AVPixelFormat = 2;
pub const AV_PIX_FMT_YUYV422: AVPixelFormat = 1;
pub const AV_PIX_FMT_YUV420P: AVPixelFormat = 0;
pub const AV_PIX_FMT_NONE: AVPixelFormat = -1;
pub type AVColorPrimaries = libc::c_uint;
pub const AVCOL_PRI_NB: AVColorPrimaries = 23;
pub const AVCOL_PRI_JEDEC_P22: AVColorPrimaries = 22;
pub const AVCOL_PRI_SMPTE432: AVColorPrimaries = 12;
pub const AVCOL_PRI_SMPTE431: AVColorPrimaries = 11;
pub const AVCOL_PRI_SMPTEST428_1: AVColorPrimaries = 10;
pub const AVCOL_PRI_SMPTE428: AVColorPrimaries = 10;
pub const AVCOL_PRI_BT2020: AVColorPrimaries = 9;
pub const AVCOL_PRI_FILM: AVColorPrimaries = 8;
pub const AVCOL_PRI_SMPTE240M: AVColorPrimaries = 7;
pub const AVCOL_PRI_SMPTE170M: AVColorPrimaries = 6;
pub const AVCOL_PRI_BT470BG: AVColorPrimaries = 5;
pub const AVCOL_PRI_BT470M: AVColorPrimaries = 4;
pub const AVCOL_PRI_RESERVED: AVColorPrimaries = 3;
pub const AVCOL_PRI_UNSPECIFIED: AVColorPrimaries = 2;
pub const AVCOL_PRI_BT709: AVColorPrimaries = 1;
pub const AVCOL_PRI_RESERVED0: AVColorPrimaries = 0;
pub type AVColorTransferCharacteristic = libc::c_uint;
pub const AVCOL_TRC_NB: AVColorTransferCharacteristic = 19;
pub const AVCOL_TRC_ARIB_STD_B67: AVColorTransferCharacteristic = 18;
pub const AVCOL_TRC_SMPTEST428_1: AVColorTransferCharacteristic = 17;
pub const AVCOL_TRC_SMPTE428: AVColorTransferCharacteristic = 17;
pub const AVCOL_TRC_SMPTEST2084: AVColorTransferCharacteristic = 16;
pub const AVCOL_TRC_SMPTE2084: AVColorTransferCharacteristic = 16;
pub const AVCOL_TRC_BT2020_12: AVColorTransferCharacteristic = 15;
pub const AVCOL_TRC_BT2020_10: AVColorTransferCharacteristic = 14;
pub const AVCOL_TRC_IEC61966_2_1: AVColorTransferCharacteristic = 13;
pub const AVCOL_TRC_BT1361_ECG: AVColorTransferCharacteristic = 12;
pub const AVCOL_TRC_IEC61966_2_4: AVColorTransferCharacteristic = 11;
pub const AVCOL_TRC_LOG_SQRT: AVColorTransferCharacteristic = 10;
pub const AVCOL_TRC_LOG: AVColorTransferCharacteristic = 9;
pub const AVCOL_TRC_LINEAR: AVColorTransferCharacteristic = 8;
pub const AVCOL_TRC_SMPTE240M: AVColorTransferCharacteristic = 7;
pub const AVCOL_TRC_SMPTE170M: AVColorTransferCharacteristic = 6;
pub const AVCOL_TRC_GAMMA28: AVColorTransferCharacteristic = 5;
pub const AVCOL_TRC_GAMMA22: AVColorTransferCharacteristic = 4;
pub const AVCOL_TRC_RESERVED: AVColorTransferCharacteristic = 3;
pub const AVCOL_TRC_UNSPECIFIED: AVColorTransferCharacteristic = 2;
pub const AVCOL_TRC_BT709: AVColorTransferCharacteristic = 1;
pub const AVCOL_TRC_RESERVED0: AVColorTransferCharacteristic = 0;
pub type AVColorSpace = libc::c_uint;
pub const AVCOL_SPC_NB: AVColorSpace = 15;
pub const AVCOL_SPC_ICTCP: AVColorSpace = 14;
pub const AVCOL_SPC_CHROMA_DERIVED_CL: AVColorSpace = 13;
pub const AVCOL_SPC_CHROMA_DERIVED_NCL: AVColorSpace = 12;
pub const AVCOL_SPC_SMPTE2085: AVColorSpace = 11;
pub const AVCOL_SPC_BT2020_CL: AVColorSpace = 10;
pub const AVCOL_SPC_BT2020_NCL: AVColorSpace = 9;
pub const AVCOL_SPC_YCOCG: AVColorSpace = 8;
pub const AVCOL_SPC_YCGCO: AVColorSpace = 8;
pub const AVCOL_SPC_SMPTE240M: AVColorSpace = 7;
pub const AVCOL_SPC_SMPTE170M: AVColorSpace = 6;
pub const AVCOL_SPC_BT470BG: AVColorSpace = 5;
pub const AVCOL_SPC_FCC: AVColorSpace = 4;
pub const AVCOL_SPC_RESERVED: AVColorSpace = 3;
pub const AVCOL_SPC_UNSPECIFIED: AVColorSpace = 2;
pub const AVCOL_SPC_BT709: AVColorSpace = 1;
pub const AVCOL_SPC_RGB: AVColorSpace = 0;
pub type AVColorRange = libc::c_uint;
pub const AVCOL_RANGE_NB: AVColorRange = 3;
pub const AVCOL_RANGE_JPEG: AVColorRange = 2;
pub const AVCOL_RANGE_MPEG: AVColorRange = 1;
pub const AVCOL_RANGE_UNSPECIFIED: AVColorRange = 0;
pub type AVChromaLocation = libc::c_uint;
pub const AVCHROMA_LOC_NB: AVChromaLocation = 7;
pub const AVCHROMA_LOC_BOTTOM: AVChromaLocation = 6;
pub const AVCHROMA_LOC_BOTTOMLEFT: AVChromaLocation = 5;
pub const AVCHROMA_LOC_TOP: AVChromaLocation = 4;
pub const AVCHROMA_LOC_TOPLEFT: AVChromaLocation = 3;
pub const AVCHROMA_LOC_CENTER: AVChromaLocation = 2;
pub const AVCHROMA_LOC_LEFT: AVChromaLocation = 1;
pub const AVCHROMA_LOC_UNSPECIFIED: AVChromaLocation = 0;
pub type AVSampleFormat = libc::c_int;
pub const AV_SAMPLE_FMT_NB: AVSampleFormat = 12;
pub const AV_SAMPLE_FMT_S64P: AVSampleFormat = 11;
pub const AV_SAMPLE_FMT_S64: AVSampleFormat = 10;
pub const AV_SAMPLE_FMT_DBLP: AVSampleFormat = 9;
pub const AV_SAMPLE_FMT_FLTP: AVSampleFormat = 8;
pub const AV_SAMPLE_FMT_S32P: AVSampleFormat = 7;
pub const AV_SAMPLE_FMT_S16P: AVSampleFormat = 6;
pub const AV_SAMPLE_FMT_U8P: AVSampleFormat = 5;
pub const AV_SAMPLE_FMT_DBL: AVSampleFormat = 4;
pub const AV_SAMPLE_FMT_FLT: AVSampleFormat = 3;
pub const AV_SAMPLE_FMT_S32: AVSampleFormat = 2;
pub const AV_SAMPLE_FMT_S16: AVSampleFormat = 1;
pub const AV_SAMPLE_FMT_U8: AVSampleFormat = 0;
pub const AV_SAMPLE_FMT_NONE: AVSampleFormat = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut uint8_t,
    pub size: libc::c_int,
}
pub type AVFrameSideDataType = libc::c_uint;
pub const AV_FRAME_DATA_REGIONS_OF_INTEREST: AVFrameSideDataType = 20;
pub const AV_FRAME_DATA_DYNAMIC_HDR_PLUS: AVFrameSideDataType = 19;
pub const AV_FRAME_DATA_S12M_TIMECODE: AVFrameSideDataType = 18;
pub const AV_FRAME_DATA_QP_TABLE_DATA: AVFrameSideDataType = 17;
pub const AV_FRAME_DATA_QP_TABLE_PROPERTIES: AVFrameSideDataType = 16;
pub const AV_FRAME_DATA_ICC_PROFILE: AVFrameSideDataType = 15;
pub const AV_FRAME_DATA_CONTENT_LIGHT_LEVEL: AVFrameSideDataType = 14;
pub const AV_FRAME_DATA_SPHERICAL: AVFrameSideDataType = 13;
pub const AV_FRAME_DATA_GOP_TIMECODE: AVFrameSideDataType = 12;
pub const AV_FRAME_DATA_MASTERING_DISPLAY_METADATA: AVFrameSideDataType = 11;
pub const AV_FRAME_DATA_AUDIO_SERVICE_TYPE: AVFrameSideDataType = 10;
pub const AV_FRAME_DATA_SKIP_SAMPLES: AVFrameSideDataType = 9;
pub const AV_FRAME_DATA_MOTION_VECTORS: AVFrameSideDataType = 8;
pub const AV_FRAME_DATA_AFD: AVFrameSideDataType = 7;
pub const AV_FRAME_DATA_DISPLAYMATRIX: AVFrameSideDataType = 6;
pub const AV_FRAME_DATA_REPLAYGAIN: AVFrameSideDataType = 5;
pub const AV_FRAME_DATA_DOWNMIX_INFO: AVFrameSideDataType = 4;
pub const AV_FRAME_DATA_MATRIXENCODING: AVFrameSideDataType = 3;
pub const AV_FRAME_DATA_STEREO3D: AVFrameSideDataType = 2;
pub const AV_FRAME_DATA_A53_CC: AVFrameSideDataType = 1;
pub const AV_FRAME_DATA_PANSCAN: AVFrameSideDataType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFrameSideData {
    pub type_0: AVFrameSideDataType,
    pub data: *mut uint8_t,
    pub size: libc::c_int,
    pub metadata: *mut AVDictionary,
    pub buf: *mut AVBufferRef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFrame {
    pub data: [*mut uint8_t; 8],
    pub linesize: [libc::c_int; 8],
    pub extended_data: *mut *mut uint8_t,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub nb_samples: libc::c_int,
    pub format: libc::c_int,
    pub key_frame: libc::c_int,
    pub pict_type: AVPictureType,
    pub sample_aspect_ratio: AVRational,
    pub pts: int64_t,
    pub pkt_pts: int64_t,
    pub pkt_dts: int64_t,
    pub coded_picture_number: libc::c_int,
    pub display_picture_number: libc::c_int,
    pub quality: libc::c_int,
    pub opaque: *mut libc::c_void,
    pub error: [uint64_t; 8],
    pub repeat_pict: libc::c_int,
    pub interlaced_frame: libc::c_int,
    pub top_field_first: libc::c_int,
    pub palette_has_changed: libc::c_int,
    pub reordered_opaque: int64_t,
    pub sample_rate: libc::c_int,
    pub channel_layout: uint64_t,
    pub buf: [*mut AVBufferRef; 8],
    pub extended_buf: *mut *mut AVBufferRef,
    pub nb_extended_buf: libc::c_int,
    pub side_data: *mut *mut AVFrameSideData,
    pub nb_side_data: libc::c_int,
    pub flags: libc::c_int,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    pub best_effort_timestamp: int64_t,
    pub pkt_pos: int64_t,
    pub pkt_duration: int64_t,
    pub metadata: *mut AVDictionary,
    pub decode_error_flags: libc::c_int,
    pub channels: libc::c_int,
    pub pkt_size: libc::c_int,
    pub qscale_table: *mut int8_t,
    pub qstride: libc::c_int,
    pub qscale_type: libc::c_int,
    pub qp_table_buf: *mut AVBufferRef,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub opaque_ref: *mut AVBufferRef,
    pub crop_top: size_t,
    pub crop_bottom: size_t,
    pub crop_left: size_t,
    pub crop_right: size_t,
    pub private_ref: *mut AVBufferRef,
}
pub type AVCodecID = libc::c_uint;
pub const AV_CODEC_ID_WRAPPED_AVFRAME: AVCodecID = 135169;
pub const AV_CODEC_ID_FFMETADATA: AVCodecID = 135168;
pub const AV_CODEC_ID_MPEG4SYSTEMS: AVCodecID = 131073;
pub const AV_CODEC_ID_MPEG2TS: AVCodecID = 131072;
pub const AV_CODEC_ID_PROBE: AVCodecID = 102400;
pub const AV_CODEC_ID_BIN_DATA: AVCodecID = 100359;
pub const AV_CODEC_ID_TIMED_ID3: AVCodecID = 100358;
pub const AV_CODEC_ID_DVD_NAV: AVCodecID = 100357;
pub const AV_CODEC_ID_SMPTE_KLV: AVCodecID = 100356;
pub const AV_CODEC_ID_OTF: AVCodecID = 100355;
pub const AV_CODEC_ID_IDF: AVCodecID = 100354;
pub const AV_CODEC_ID_XBIN: AVCodecID = 100353;
pub const AV_CODEC_ID_BINTEXT: AVCodecID = 100352;
pub const AV_CODEC_ID_SCTE_35: AVCodecID = 98305;
pub const AV_CODEC_ID_TTF: AVCodecID = 98304;
pub const AV_CODEC_ID_FIRST_UNKNOWN: AVCodecID = 98304;
pub const AV_CODEC_ID_ARIB_CAPTION: AVCodecID = 96272;
pub const AV_CODEC_ID_TTML: AVCodecID = 96271;
pub const AV_CODEC_ID_HDMV_TEXT_SUBTITLE: AVCodecID = 96270;
pub const AV_CODEC_ID_ASS: AVCodecID = 96269;
pub const AV_CODEC_ID_PJS: AVCodecID = 96268;
pub const AV_CODEC_ID_VPLAYER: AVCodecID = 96267;
pub const AV_CODEC_ID_MPL2: AVCodecID = 96266;
pub const AV_CODEC_ID_WEBVTT: AVCodecID = 96265;
pub const AV_CODEC_ID_SUBRIP: AVCodecID = 96264;
pub const AV_CODEC_ID_SUBVIEWER: AVCodecID = 96263;
pub const AV_CODEC_ID_SUBVIEWER1: AVCodecID = 96262;
pub const AV_CODEC_ID_STL: AVCodecID = 96261;
pub const AV_CODEC_ID_REALTEXT: AVCodecID = 96260;
pub const AV_CODEC_ID_SAMI: AVCodecID = 96259;
pub const AV_CODEC_ID_JACOSUB: AVCodecID = 96258;
pub const AV_CODEC_ID_EIA_608: AVCodecID = 96257;
pub const AV_CODEC_ID_MICRODVD: AVCodecID = 96256;
pub const AV_CODEC_ID_SRT: AVCodecID = 94216;
pub const AV_CODEC_ID_DVB_TELETEXT: AVCodecID = 94215;
pub const AV_CODEC_ID_HDMV_PGS_SUBTITLE: AVCodecID = 94214;
pub const AV_CODEC_ID_MOV_TEXT: AVCodecID = 94213;
pub const AV_CODEC_ID_SSA: AVCodecID = 94212;
pub const AV_CODEC_ID_XSUB: AVCodecID = 94211;
pub const AV_CODEC_ID_TEXT: AVCodecID = 94210;
pub const AV_CODEC_ID_DVB_SUBTITLE: AVCodecID = 94209;
pub const AV_CODEC_ID_DVD_SUBTITLE: AVCodecID = 94208;
pub const AV_CODEC_ID_FIRST_SUBTITLE: AVCodecID = 94208;
pub const AV_CODEC_ID_HCOM: AVCodecID = 88085;
pub const AV_CODEC_ID_ATRAC9: AVCodecID = 88084;
pub const AV_CODEC_ID_SBC: AVCodecID = 88083;
pub const AV_CODEC_ID_APTX_HD: AVCodecID = 88082;
pub const AV_CODEC_ID_APTX: AVCodecID = 88081;
pub const AV_CODEC_ID_DOLBY_E: AVCodecID = 88080;
pub const AV_CODEC_ID_ATRAC3PAL: AVCodecID = 88079;
pub const AV_CODEC_ID_ATRAC3AL: AVCodecID = 88078;
pub const AV_CODEC_ID_DST: AVCodecID = 88077;
pub const AV_CODEC_ID_XMA2: AVCodecID = 88076;
pub const AV_CODEC_ID_XMA1: AVCodecID = 88075;
pub const AV_CODEC_ID_INTERPLAY_ACM: AVCodecID = 88074;
pub const AV_CODEC_ID_4GV: AVCodecID = 88073;
pub const AV_CODEC_ID_DSD_MSBF_PLANAR: AVCodecID = 88072;
pub const AV_CODEC_ID_DSD_LSBF_PLANAR: AVCodecID = 88071;
pub const AV_CODEC_ID_DSD_MSBF: AVCodecID = 88070;
pub const AV_CODEC_ID_DSD_LSBF: AVCodecID = 88069;
pub const AV_CODEC_ID_SMV: AVCodecID = 88068;
pub const AV_CODEC_ID_EVRC: AVCodecID = 88067;
pub const AV_CODEC_ID_SONIC_LS: AVCodecID = 88066;
pub const AV_CODEC_ID_SONIC: AVCodecID = 88065;
pub const AV_CODEC_ID_FFWAVESYNTH: AVCodecID = 88064;
pub const AV_CODEC_ID_CODEC2: AVCodecID = 86083;
pub const AV_CODEC_ID_DSS_SP: AVCodecID = 86082;
pub const AV_CODEC_ID_ON2AVC: AVCodecID = 86081;
pub const AV_CODEC_ID_PAF_AUDIO: AVCodecID = 86080;
pub const AV_CODEC_ID_METASOUND: AVCodecID = 86079;
pub const AV_CODEC_ID_TAK: AVCodecID = 86078;
pub const AV_CODEC_ID_COMFORT_NOISE: AVCodecID = 86077;
pub const AV_CODEC_ID_OPUS: AVCodecID = 86076;
pub const AV_CODEC_ID_ILBC: AVCodecID = 86075;
pub const AV_CODEC_ID_IAC: AVCodecID = 86074;
pub const AV_CODEC_ID_RALF: AVCodecID = 86073;
pub const AV_CODEC_ID_BMV_AUDIO: AVCodecID = 86072;
pub const AV_CODEC_ID_8SVX_FIB: AVCodecID = 86071;
pub const AV_CODEC_ID_8SVX_EXP: AVCodecID = 86070;
pub const AV_CODEC_ID_G729: AVCodecID = 86069;
pub const AV_CODEC_ID_G723_1: AVCodecID = 86068;
pub const AV_CODEC_ID_CELT: AVCodecID = 86067;
pub const AV_CODEC_ID_QDMC: AVCodecID = 86066;
pub const AV_CODEC_ID_AAC_LATM: AVCodecID = 86065;
pub const AV_CODEC_ID_BINKAUDIO_DCT: AVCodecID = 86064;
pub const AV_CODEC_ID_BINKAUDIO_RDFT: AVCodecID = 86063;
pub const AV_CODEC_ID_ATRAC1: AVCodecID = 86062;
pub const AV_CODEC_ID_MP4ALS: AVCodecID = 86061;
pub const AV_CODEC_ID_TRUEHD: AVCodecID = 86060;
pub const AV_CODEC_ID_TWINVQ: AVCodecID = 86059;
pub const AV_CODEC_ID_MP1: AVCodecID = 86058;
pub const AV_CODEC_ID_SIPR: AVCodecID = 86057;
pub const AV_CODEC_ID_EAC3: AVCodecID = 86056;
pub const AV_CODEC_ID_ATRAC3P: AVCodecID = 86055;
pub const AV_CODEC_ID_WMALOSSLESS: AVCodecID = 86054;
pub const AV_CODEC_ID_WMAPRO: AVCodecID = 86053;
pub const AV_CODEC_ID_WMAVOICE: AVCodecID = 86052;
pub const AV_CODEC_ID_SPEEX: AVCodecID = 86051;
pub const AV_CODEC_ID_MUSEPACK8: AVCodecID = 86050;
pub const AV_CODEC_ID_NELLYMOSER: AVCodecID = 86049;
pub const AV_CODEC_ID_APE: AVCodecID = 86048;
pub const AV_CODEC_ID_ATRAC3: AVCodecID = 86047;
pub const AV_CODEC_ID_GSM_MS: AVCodecID = 86046;
pub const AV_CODEC_ID_MLP: AVCodecID = 86045;
pub const AV_CODEC_ID_MUSEPACK7: AVCodecID = 86044;
pub const AV_CODEC_ID_IMC: AVCodecID = 86043;
pub const AV_CODEC_ID_DSICINAUDIO: AVCodecID = 86042;
pub const AV_CODEC_ID_WAVPACK: AVCodecID = 86041;
pub const AV_CODEC_ID_QCELP: AVCodecID = 86040;
pub const AV_CODEC_ID_SMACKAUDIO: AVCodecID = 86039;
pub const AV_CODEC_ID_TTA: AVCodecID = 86038;
pub const AV_CODEC_ID_TRUESPEECH: AVCodecID = 86037;
pub const AV_CODEC_ID_COOK: AVCodecID = 86036;
pub const AV_CODEC_ID_QDM2: AVCodecID = 86035;
pub const AV_CODEC_ID_GSM: AVCodecID = 86034;
pub const AV_CODEC_ID_WESTWOOD_SND1: AVCodecID = 86033;
pub const AV_CODEC_ID_ALAC: AVCodecID = 86032;
pub const AV_CODEC_ID_SHORTEN: AVCodecID = 86031;
pub const AV_CODEC_ID_MP3ON4: AVCodecID = 86030;
pub const AV_CODEC_ID_MP3ADU: AVCodecID = 86029;
pub const AV_CODEC_ID_FLAC: AVCodecID = 86028;
pub const AV_CODEC_ID_VMDAUDIO: AVCodecID = 86027;
pub const AV_CODEC_ID_MACE6: AVCodecID = 86026;
pub const AV_CODEC_ID_MACE3: AVCodecID = 86025;
pub const AV_CODEC_ID_WMAV2: AVCodecID = 86024;
pub const AV_CODEC_ID_WMAV1: AVCodecID = 86023;
pub const AV_CODEC_ID_DVAUDIO: AVCodecID = 86022;
pub const AV_CODEC_ID_VORBIS: AVCodecID = 86021;
pub const AV_CODEC_ID_DTS: AVCodecID = 86020;
pub const AV_CODEC_ID_AC3: AVCodecID = 86019;
pub const AV_CODEC_ID_AAC: AVCodecID = 86018;
pub const AV_CODEC_ID_MP3: AVCodecID = 86017;
pub const AV_CODEC_ID_MP2: AVCodecID = 86016;
pub const AV_CODEC_ID_GREMLIN_DPCM: AVCodecID = 83969;
pub const AV_CODEC_ID_SDX2_DPCM: AVCodecID = 83968;
pub const AV_CODEC_ID_SOL_DPCM: AVCodecID = 81923;
pub const AV_CODEC_ID_XAN_DPCM: AVCodecID = 81922;
pub const AV_CODEC_ID_INTERPLAY_DPCM: AVCodecID = 81921;
pub const AV_CODEC_ID_ROQ_DPCM: AVCodecID = 81920;
pub const AV_CODEC_ID_RA_288: AVCodecID = 77825;
pub const AV_CODEC_ID_RA_144: AVCodecID = 77824;
pub const AV_CODEC_ID_AMR_WB: AVCodecID = 73729;
pub const AV_CODEC_ID_AMR_NB: AVCodecID = 73728;
pub const AV_CODEC_ID_ADPCM_AGM: AVCodecID = 71690;
pub const AV_CODEC_ID_ADPCM_MTAF: AVCodecID = 71689;
pub const AV_CODEC_ID_ADPCM_IMA_DAT4: AVCodecID = 71688;
pub const AV_CODEC_ID_ADPCM_AICA: AVCodecID = 71687;
pub const AV_CODEC_ID_ADPCM_PSX: AVCodecID = 71686;
pub const AV_CODEC_ID_ADPCM_THP_LE: AVCodecID = 71685;
pub const AV_CODEC_ID_ADPCM_G726LE: AVCodecID = 71684;
pub const AV_CODEC_ID_ADPCM_IMA_RAD: AVCodecID = 71683;
pub const AV_CODEC_ID_ADPCM_DTK: AVCodecID = 71682;
pub const AV_CODEC_ID_ADPCM_IMA_OKI: AVCodecID = 71681;
pub const AV_CODEC_ID_ADPCM_AFC: AVCodecID = 71680;
pub const AV_CODEC_ID_ADPCM_VIMA: AVCodecID = 69662;
pub const AV_CODEC_ID_ADPCM_IMA_APC: AVCodecID = 69661;
pub const AV_CODEC_ID_ADPCM_G722: AVCodecID = 69660;
pub const AV_CODEC_ID_ADPCM_IMA_ISS: AVCodecID = 69659;
pub const AV_CODEC_ID_ADPCM_EA_MAXIS_XA: AVCodecID = 69658;
pub const AV_CODEC_ID_ADPCM_EA_XAS: AVCodecID = 69657;
pub const AV_CODEC_ID_ADPCM_IMA_EA_EACS: AVCodecID = 69656;
pub const AV_CODEC_ID_ADPCM_IMA_EA_SEAD: AVCodecID = 69655;
pub const AV_CODEC_ID_ADPCM_EA_R2: AVCodecID = 69654;
pub const AV_CODEC_ID_ADPCM_EA_R3: AVCodecID = 69653;
pub const AV_CODEC_ID_ADPCM_EA_R1: AVCodecID = 69652;
pub const AV_CODEC_ID_ADPCM_IMA_AMV: AVCodecID = 69651;
pub const AV_CODEC_ID_ADPCM_THP: AVCodecID = 69650;
pub const AV_CODEC_ID_ADPCM_SBPRO_2: AVCodecID = 69649;
pub const AV_CODEC_ID_ADPCM_SBPRO_3: AVCodecID = 69648;
pub const AV_CODEC_ID_ADPCM_SBPRO_4: AVCodecID = 69647;
pub const AV_CODEC_ID_ADPCM_YAMAHA: AVCodecID = 69646;
pub const AV_CODEC_ID_ADPCM_SWF: AVCodecID = 69645;
pub const AV_CODEC_ID_ADPCM_CT: AVCodecID = 69644;
pub const AV_CODEC_ID_ADPCM_G726: AVCodecID = 69643;
pub const AV_CODEC_ID_ADPCM_EA: AVCodecID = 69642;
pub const AV_CODEC_ID_ADPCM_ADX: AVCodecID = 69641;
pub const AV_CODEC_ID_ADPCM_XA: AVCodecID = 69640;
pub const AV_CODEC_ID_ADPCM_4XM: AVCodecID = 69639;
pub const AV_CODEC_ID_ADPCM_MS: AVCodecID = 69638;
pub const AV_CODEC_ID_ADPCM_IMA_SMJPEG: AVCodecID = 69637;
pub const AV_CODEC_ID_ADPCM_IMA_WS: AVCodecID = 69636;
pub const AV_CODEC_ID_ADPCM_IMA_DK4: AVCodecID = 69635;
pub const AV_CODEC_ID_ADPCM_IMA_DK3: AVCodecID = 69634;
pub const AV_CODEC_ID_ADPCM_IMA_WAV: AVCodecID = 69633;
pub const AV_CODEC_ID_ADPCM_IMA_QT: AVCodecID = 69632;
pub const AV_CODEC_ID_PCM_VIDC: AVCodecID = 67588;
pub const AV_CODEC_ID_PCM_F24LE: AVCodecID = 67587;
pub const AV_CODEC_ID_PCM_F16LE: AVCodecID = 67586;
pub const AV_CODEC_ID_PCM_S64BE: AVCodecID = 67585;
pub const AV_CODEC_ID_PCM_S64LE: AVCodecID = 67584;
pub const AV_CODEC_ID_PCM_S16BE_PLANAR: AVCodecID = 65566;
pub const AV_CODEC_ID_PCM_S32LE_PLANAR: AVCodecID = 65565;
pub const AV_CODEC_ID_PCM_S24LE_PLANAR: AVCodecID = 65564;
pub const AV_CODEC_ID_PCM_S8_PLANAR: AVCodecID = 65563;
pub const AV_CODEC_ID_S302M: AVCodecID = 65562;
pub const AV_CODEC_ID_PCM_LXF: AVCodecID = 65561;
pub const AV_CODEC_ID_PCM_BLURAY: AVCodecID = 65560;
pub const AV_CODEC_ID_PCM_F64LE: AVCodecID = 65559;
pub const AV_CODEC_ID_PCM_F64BE: AVCodecID = 65558;
pub const AV_CODEC_ID_PCM_F32LE: AVCodecID = 65557;
pub const AV_CODEC_ID_PCM_F32BE: AVCodecID = 65556;
pub const AV_CODEC_ID_PCM_DVD: AVCodecID = 65555;
pub const AV_CODEC_ID_PCM_S16LE_PLANAR: AVCodecID = 65554;
pub const AV_CODEC_ID_PCM_ZORK: AVCodecID = 65553;
pub const AV_CODEC_ID_PCM_S24DAUD: AVCodecID = 65552;
pub const AV_CODEC_ID_PCM_U24BE: AVCodecID = 65551;
pub const AV_CODEC_ID_PCM_U24LE: AVCodecID = 65550;
pub const AV_CODEC_ID_PCM_S24BE: AVCodecID = 65549;
pub const AV_CODEC_ID_PCM_S24LE: AVCodecID = 65548;
pub const AV_CODEC_ID_PCM_U32BE: AVCodecID = 65547;
pub const AV_CODEC_ID_PCM_U32LE: AVCodecID = 65546;
pub const AV_CODEC_ID_PCM_S32BE: AVCodecID = 65545;
pub const AV_CODEC_ID_PCM_S32LE: AVCodecID = 65544;
pub const AV_CODEC_ID_PCM_ALAW: AVCodecID = 65543;
pub const AV_CODEC_ID_PCM_MULAW: AVCodecID = 65542;
pub const AV_CODEC_ID_PCM_U8: AVCodecID = 65541;
pub const AV_CODEC_ID_PCM_S8: AVCodecID = 65540;
pub const AV_CODEC_ID_PCM_U16BE: AVCodecID = 65539;
pub const AV_CODEC_ID_PCM_U16LE: AVCodecID = 65538;
pub const AV_CODEC_ID_PCM_S16BE: AVCodecID = 65537;
pub const AV_CODEC_ID_PCM_S16LE: AVCodecID = 65536;
pub const AV_CODEC_ID_FIRST_AUDIO: AVCodecID = 65536;
pub const AV_CODEC_ID_VP4: AVCodecID = 32813;
pub const AV_CODEC_ID_LSCR: AVCodecID = 32812;
pub const AV_CODEC_ID_AGM: AVCodecID = 32811;
pub const AV_CODEC_ID_ARBC: AVCodecID = 32810;
pub const AV_CODEC_ID_HYMT: AVCodecID = 32809;
pub const AV_CODEC_ID_RASC: AVCodecID = 32808;
pub const AV_CODEC_ID_WCMV: AVCodecID = 32807;
pub const AV_CODEC_ID_MWSC: AVCodecID = 32806;
pub const AV_CODEC_ID_PROSUMER: AVCodecID = 32805;
pub const AV_CODEC_ID_IMM4: AVCodecID = 32804;
pub const AV_CODEC_ID_FITS: AVCodecID = 32803;
pub const AV_CODEC_ID_GDV: AVCodecID = 32802;
pub const AV_CODEC_ID_SVG: AVCodecID = 32801;
pub const AV_CODEC_ID_SRGC: AVCodecID = 32800;
pub const AV_CODEC_ID_MSCC: AVCodecID = 32799;
pub const AV_CODEC_ID_BITPACKED: AVCodecID = 32798;
pub const AV_CODEC_ID_AV1: AVCodecID = 32797;
pub const AV_CODEC_ID_XPM: AVCodecID = 32796;
pub const AV_CODEC_ID_CLEARVIDEO: AVCodecID = 32795;
pub const AV_CODEC_ID_SCPR: AVCodecID = 32794;
pub const AV_CODEC_ID_FMVC: AVCodecID = 32793;
pub const AV_CODEC_ID_SPEEDHQ: AVCodecID = 32792;
pub const AV_CODEC_ID_PIXLET: AVCodecID = 32791;
pub const AV_CODEC_ID_PSD: AVCodecID = 32790;
pub const AV_CODEC_ID_YLC: AVCodecID = 32789;
pub const AV_CODEC_ID_SHEERVIDEO: AVCodecID = 32788;
pub const AV_CODEC_ID_MAGICYUV: AVCodecID = 32787;
pub const AV_CODEC_ID_M101: AVCodecID = 32786;
pub const AV_CODEC_ID_TRUEMOTION2RT: AVCodecID = 32785;
pub const AV_CODEC_ID_CFHD: AVCodecID = 32784;
pub const AV_CODEC_ID_DAALA: AVCodecID = 32783;
pub const AV_CODEC_ID_APNG: AVCodecID = 32782;
pub const AV_CODEC_ID_SMVJPEG: AVCodecID = 32781;
pub const AV_CODEC_ID_SNOW: AVCodecID = 32780;
pub const AV_CODEC_ID_XFACE: AVCodecID = 32779;
pub const AV_CODEC_ID_CPIA: AVCodecID = 32778;
pub const AV_CODEC_ID_AVRN: AVCodecID = 32777;
pub const AV_CODEC_ID_YUV4: AVCodecID = 32776;
pub const AV_CODEC_ID_V408: AVCodecID = 32775;
pub const AV_CODEC_ID_V308: AVCodecID = 32774;
pub const AV_CODEC_ID_TARGA_Y216: AVCodecID = 32773;
pub const AV_CODEC_ID_AYUV: AVCodecID = 32772;
pub const AV_CODEC_ID_AVUI: AVCodecID = 32771;
pub const AV_CODEC_ID_012V: AVCodecID = 32770;
pub const AV_CODEC_ID_AVRP: AVCodecID = 32769;
pub const AV_CODEC_ID_Y41P: AVCodecID = 32768;
pub const AV_CODEC_ID_AVS2: AVCodecID = 192;
pub const AV_CODEC_ID_RSCC: AVCodecID = 191;
pub const AV_CODEC_ID_SCREENPRESSO: AVCodecID = 190;
pub const AV_CODEC_ID_DXV: AVCodecID = 189;
pub const AV_CODEC_ID_DDS: AVCodecID = 188;
pub const AV_CODEC_ID_HAP: AVCodecID = 187;
pub const AV_CODEC_ID_HQ_HQA: AVCodecID = 186;
pub const AV_CODEC_ID_TDSC: AVCodecID = 185;
pub const AV_CODEC_ID_HQX: AVCodecID = 184;
pub const AV_CODEC_ID_MVC2: AVCodecID = 183;
pub const AV_CODEC_ID_MVC1: AVCodecID = 182;
pub const AV_CODEC_ID_SGIRLE: AVCodecID = 181;
pub const AV_CODEC_ID_SANM: AVCodecID = 180;
pub const AV_CODEC_ID_VP7: AVCodecID = 179;
pub const AV_CODEC_ID_EXR: AVCodecID = 178;
pub const AV_CODEC_ID_PAF_VIDEO: AVCodecID = 177;
pub const AV_CODEC_ID_BRENDER_PIX: AVCodecID = 176;
pub const AV_CODEC_ID_ALIAS_PIX: AVCodecID = 175;
pub const AV_CODEC_ID_FIC: AVCodecID = 174;
pub const AV_CODEC_ID_HEVC: AVCodecID = 173;
pub const AV_CODEC_ID_HNM4_VIDEO: AVCodecID = 172;
pub const AV_CODEC_ID_WEBP: AVCodecID = 171;
pub const AV_CODEC_ID_G2M: AVCodecID = 170;
pub const AV_CODEC_ID_ESCAPE130: AVCodecID = 169;
pub const AV_CODEC_ID_AIC: AVCodecID = 168;
pub const AV_CODEC_ID_VP9: AVCodecID = 167;
pub const AV_CODEC_ID_MSS2: AVCodecID = 166;
pub const AV_CODEC_ID_CLLC: AVCodecID = 165;
pub const AV_CODEC_ID_MTS2: AVCodecID = 164;
pub const AV_CODEC_ID_TSCC2: AVCodecID = 163;
pub const AV_CODEC_ID_MSA1: AVCodecID = 162;
pub const AV_CODEC_ID_MSS1: AVCodecID = 161;
pub const AV_CODEC_ID_ZEROCODEC: AVCodecID = 160;
pub const AV_CODEC_ID_XBM: AVCodecID = 159;
pub const AV_CODEC_ID_CDXL: AVCodecID = 158;
pub const AV_CODEC_ID_XWD: AVCodecID = 157;
pub const AV_CODEC_ID_V410: AVCodecID = 156;
pub const AV_CODEC_ID_DXTORY: AVCodecID = 155;
pub const AV_CODEC_ID_VBLE: AVCodecID = 154;
pub const AV_CODEC_ID_BMV_VIDEO: AVCodecID = 153;
pub const AV_CODEC_ID_UTVIDEO: AVCodecID = 152;
pub const AV_CODEC_ID_VC1IMAGE: AVCodecID = 151;
pub const AV_CODEC_ID_WMV3IMAGE: AVCodecID = 150;
pub const AV_CODEC_ID_DFA: AVCodecID = 149;
pub const AV_CODEC_ID_JV: AVCodecID = 148;
pub const AV_CODEC_ID_PRORES: AVCodecID = 147;
pub const AV_CODEC_ID_LAGARITH: AVCodecID = 146;
pub const AV_CODEC_ID_MXPEG: AVCodecID = 145;
pub const AV_CODEC_ID_R10K: AVCodecID = 144;
pub const AV_CODEC_ID_A64_MULTI5: AVCodecID = 143;
pub const AV_CODEC_ID_A64_MULTI: AVCodecID = 142;
pub const AV_CODEC_ID_ANSI: AVCodecID = 141;
pub const AV_CODEC_ID_PICTOR: AVCodecID = 140;
pub const AV_CODEC_ID_VP8: AVCodecID = 139;
pub const AV_CODEC_ID_YOP: AVCodecID = 138;
pub const AV_CODEC_ID_KGV1: AVCodecID = 137;
pub const AV_CODEC_ID_IFF_ILBM: AVCodecID = 136;
pub const AV_CODEC_ID_BINKVIDEO: AVCodecID = 135;
pub const AV_CODEC_ID_ANM: AVCodecID = 134;
pub const AV_CODEC_ID_R210: AVCodecID = 133;
pub const AV_CODEC_ID_CDGRAPHICS: AVCodecID = 132;
pub const AV_CODEC_ID_FLASHSV2: AVCodecID = 131;
pub const AV_CODEC_ID_FRWU: AVCodecID = 130;
pub const AV_CODEC_ID_MAD: AVCodecID = 129;
pub const AV_CODEC_ID_DPX: AVCodecID = 128;
pub const AV_CODEC_ID_V210: AVCodecID = 127;
pub const AV_CODEC_ID_TMV: AVCodecID = 126;
pub const AV_CODEC_ID_V210X: AVCodecID = 125;
pub const AV_CODEC_ID_AURA2: AVCodecID = 124;
pub const AV_CODEC_ID_AURA: AVCodecID = 123;
pub const AV_CODEC_ID_TQI: AVCodecID = 122;
pub const AV_CODEC_ID_TGQ: AVCodecID = 121;
pub const AV_CODEC_ID_TGV: AVCodecID = 120;
pub const AV_CODEC_ID_MOTIONPIXELS: AVCodecID = 119;
pub const AV_CODEC_ID_CMV: AVCodecID = 118;
pub const AV_CODEC_ID_BFI: AVCodecID = 117;
pub const AV_CODEC_ID_DIRAC: AVCodecID = 116;
pub const AV_CODEC_ID_ESCAPE124: AVCodecID = 115;
pub const AV_CODEC_ID_RL2: AVCodecID = 114;
pub const AV_CODEC_ID_MIMIC: AVCodecID = 113;
pub const AV_CODEC_ID_INDEO5: AVCodecID = 112;
pub const AV_CODEC_ID_INDEO4: AVCodecID = 111;
pub const AV_CODEC_ID_SUNRAST: AVCodecID = 110;
pub const AV_CODEC_ID_PCX: AVCodecID = 109;
pub const AV_CODEC_ID_VB: AVCodecID = 108;
pub const AV_CODEC_ID_AMV: AVCodecID = 107;
pub const AV_CODEC_ID_VP6A: AVCodecID = 106;
pub const AV_CODEC_ID_TXD: AVCodecID = 105;
pub const AV_CODEC_ID_PTX: AVCodecID = 104;
pub const AV_CODEC_ID_BETHSOFTVID: AVCodecID = 103;
pub const AV_CODEC_ID_C93: AVCodecID = 102;
pub const AV_CODEC_ID_SGI: AVCodecID = 101;
pub const AV_CODEC_ID_THP: AVCodecID = 100;
pub const AV_CODEC_ID_DNXHD: AVCodecID = 99;
pub const AV_CODEC_ID_DXA: AVCodecID = 98;
pub const AV_CODEC_ID_GIF: AVCodecID = 97;
pub const AV_CODEC_ID_TIFF: AVCodecID = 96;
pub const AV_CODEC_ID_TIERTEXSEQVIDEO: AVCodecID = 95;
pub const AV_CODEC_ID_DSICINVIDEO: AVCodecID = 94;
pub const AV_CODEC_ID_TARGA: AVCodecID = 93;
pub const AV_CODEC_ID_VP6F: AVCodecID = 92;
pub const AV_CODEC_ID_VP6: AVCodecID = 91;
pub const AV_CODEC_ID_VP5: AVCodecID = 90;
pub const AV_CODEC_ID_VMNC: AVCodecID = 89;
pub const AV_CODEC_ID_JPEG2000: AVCodecID = 88;
pub const AV_CODEC_ID_CAVS: AVCodecID = 87;
pub const AV_CODEC_ID_FLASHSV: AVCodecID = 86;
pub const AV_CODEC_ID_KMVC: AVCodecID = 85;
pub const AV_CODEC_ID_NUV: AVCodecID = 84;
pub const AV_CODEC_ID_SMACKVIDEO: AVCodecID = 83;
pub const AV_CODEC_ID_AVS: AVCodecID = 82;
pub const AV_CODEC_ID_ZMBV: AVCodecID = 81;
pub const AV_CODEC_ID_MMVIDEO: AVCodecID = 80;
pub const AV_CODEC_ID_CSCD: AVCodecID = 79;
pub const AV_CODEC_ID_BMP: AVCodecID = 78;
pub const AV_CODEC_ID_TRUEMOTION2: AVCodecID = 77;
pub const AV_CODEC_ID_FRAPS: AVCodecID = 76;
pub const AV_CODEC_ID_INDEO2: AVCodecID = 75;
pub const AV_CODEC_ID_AASC: AVCodecID = 74;
pub const AV_CODEC_ID_WNV1: AVCodecID = 73;
pub const AV_CODEC_ID_LOCO: AVCodecID = 72;
pub const AV_CODEC_ID_WMV3: AVCodecID = 71;
pub const AV_CODEC_ID_VC1: AVCodecID = 70;
pub const AV_CODEC_ID_RV40: AVCodecID = 69;
pub const AV_CODEC_ID_RV30: AVCodecID = 68;
pub const AV_CODEC_ID_FFVHUFF: AVCodecID = 67;
pub const AV_CODEC_ID_PAM: AVCodecID = 66;
pub const AV_CODEC_ID_PGMYUV: AVCodecID = 65;
pub const AV_CODEC_ID_PGM: AVCodecID = 64;
pub const AV_CODEC_ID_PBM: AVCodecID = 63;
pub const AV_CODEC_ID_PPM: AVCodecID = 62;
pub const AV_CODEC_ID_PNG: AVCodecID = 61;
pub const AV_CODEC_ID_QPEG: AVCodecID = 60;
pub const AV_CODEC_ID_VIXL: AVCodecID = 59;
pub const AV_CODEC_ID_QDRAW: AVCodecID = 58;
pub const AV_CODEC_ID_ULTI: AVCodecID = 57;
pub const AV_CODEC_ID_TSCC: AVCodecID = 56;
pub const AV_CODEC_ID_QTRLE: AVCodecID = 55;
pub const AV_CODEC_ID_ZLIB: AVCodecID = 54;
pub const AV_CODEC_ID_MSZH: AVCodecID = 53;
pub const AV_CODEC_ID_VMDVIDEO: AVCodecID = 52;
pub const AV_CODEC_ID_TRUEMOTION1: AVCodecID = 51;
pub const AV_CODEC_ID_FLIC: AVCodecID = 50;
pub const AV_CODEC_ID_SMC: AVCodecID = 49;
pub const AV_CODEC_ID_8BPS: AVCodecID = 48;
pub const AV_CODEC_ID_IDCIN: AVCodecID = 47;
pub const AV_CODEC_ID_MSVIDEO1: AVCodecID = 46;
pub const AV_CODEC_ID_MSRLE: AVCodecID = 45;
pub const AV_CODEC_ID_WS_VQA: AVCodecID = 44;
pub const AV_CODEC_ID_CINEPAK: AVCodecID = 43;
pub const AV_CODEC_ID_RPZA: AVCodecID = 42;
pub const AV_CODEC_ID_XAN_WC4: AVCodecID = 41;
pub const AV_CODEC_ID_XAN_WC3: AVCodecID = 40;
pub const AV_CODEC_ID_INTERPLAY_VIDEO: AVCodecID = 39;
pub const AV_CODEC_ID_ROQ: AVCodecID = 38;
pub const AV_CODEC_ID_MDEC: AVCodecID = 37;
pub const AV_CODEC_ID_CLJR: AVCodecID = 36;
pub const AV_CODEC_ID_VCR1: AVCodecID = 35;
pub const AV_CODEC_ID_4XM: AVCodecID = 34;
pub const AV_CODEC_ID_FFV1: AVCodecID = 33;
pub const AV_CODEC_ID_ASV2: AVCodecID = 32;
pub const AV_CODEC_ID_ASV1: AVCodecID = 31;
pub const AV_CODEC_ID_THEORA: AVCodecID = 30;
pub const AV_CODEC_ID_VP3: AVCodecID = 29;
pub const AV_CODEC_ID_INDEO3: AVCodecID = 28;
pub const AV_CODEC_ID_H264: AVCodecID = 27;
pub const AV_CODEC_ID_CYUV: AVCodecID = 26;
pub const AV_CODEC_ID_HUFFYUV: AVCodecID = 25;
pub const AV_CODEC_ID_DVVIDEO: AVCodecID = 24;
pub const AV_CODEC_ID_SVQ3: AVCodecID = 23;
pub const AV_CODEC_ID_SVQ1: AVCodecID = 22;
pub const AV_CODEC_ID_FLV1: AVCodecID = 21;
pub const AV_CODEC_ID_H263I: AVCodecID = 20;
pub const AV_CODEC_ID_H263P: AVCodecID = 19;
pub const AV_CODEC_ID_WMV2: AVCodecID = 18;
pub const AV_CODEC_ID_WMV1: AVCodecID = 17;
pub const AV_CODEC_ID_MSMPEG4V3: AVCodecID = 16;
pub const AV_CODEC_ID_MSMPEG4V2: AVCodecID = 15;
pub const AV_CODEC_ID_MSMPEG4V1: AVCodecID = 14;
pub const AV_CODEC_ID_RAWVIDEO: AVCodecID = 13;
pub const AV_CODEC_ID_MPEG4: AVCodecID = 12;
pub const AV_CODEC_ID_JPEGLS: AVCodecID = 11;
pub const AV_CODEC_ID_SP5X: AVCodecID = 10;
pub const AV_CODEC_ID_LJPEG: AVCodecID = 9;
pub const AV_CODEC_ID_MJPEGB: AVCodecID = 8;
pub const AV_CODEC_ID_MJPEG: AVCodecID = 7;
pub const AV_CODEC_ID_RV20: AVCodecID = 6;
pub const AV_CODEC_ID_RV10: AVCodecID = 5;
pub const AV_CODEC_ID_H263: AVCodecID = 4;
pub const AV_CODEC_ID_H261: AVCodecID = 3;
pub const AV_CODEC_ID_MPEG2VIDEO: AVCodecID = 2;
pub const AV_CODEC_ID_MPEG1VIDEO: AVCodecID = 1;
pub const AV_CODEC_ID_NONE: AVCodecID = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecDescriptor {
    pub id: AVCodecID,
    pub type_0: AVMediaType,
    pub name: *const libc::c_char,
    pub long_name: *const libc::c_char,
    pub props: libc::c_int,
    pub mime_types: *const *const libc::c_char,
    pub profiles: *const AVProfile,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVProfile {
    pub profile: libc::c_int,
    pub name: *const libc::c_char,
}
pub type AVDiscard = libc::c_int;
pub const AVDISCARD_ALL: AVDiscard = 48;
pub const AVDISCARD_NONKEY: AVDiscard = 32;
pub const AVDISCARD_NONINTRA: AVDiscard = 24;
pub const AVDISCARD_BIDIR: AVDiscard = 16;
pub const AVDISCARD_NONREF: AVDiscard = 8;
pub const AVDISCARD_DEFAULT: AVDiscard = 0;
pub const AVDISCARD_NONE: AVDiscard = -16;
pub type AVAudioServiceType = libc::c_uint;
pub const AV_AUDIO_SERVICE_TYPE_NB: AVAudioServiceType = 9;
pub const AV_AUDIO_SERVICE_TYPE_KARAOKE: AVAudioServiceType = 8;
pub const AV_AUDIO_SERVICE_TYPE_VOICE_OVER: AVAudioServiceType = 7;
pub const AV_AUDIO_SERVICE_TYPE_EMERGENCY: AVAudioServiceType = 6;
pub const AV_AUDIO_SERVICE_TYPE_COMMENTARY: AVAudioServiceType = 5;
pub const AV_AUDIO_SERVICE_TYPE_DIALOGUE: AVAudioServiceType = 4;
pub const AV_AUDIO_SERVICE_TYPE_HEARING_IMPAIRED: AVAudioServiceType = 3;
pub const AV_AUDIO_SERVICE_TYPE_VISUALLY_IMPAIRED: AVAudioServiceType = 2;
pub const AV_AUDIO_SERVICE_TYPE_EFFECTS: AVAudioServiceType = 1;
pub const AV_AUDIO_SERVICE_TYPE_MAIN: AVAudioServiceType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RcOverride {
    pub start_frame: libc::c_int,
    pub end_frame: libc::c_int,
    pub qscale: libc::c_int,
    pub quality_factor: libc::c_float,
}
pub type AVPacketSideDataType = libc::c_uint;
pub const AV_PKT_DATA_NB: AVPacketSideDataType = 27;
pub const AV_PKT_DATA_AFD: AVPacketSideDataType = 26;
pub const AV_PKT_DATA_ENCRYPTION_INFO: AVPacketSideDataType = 25;
pub const AV_PKT_DATA_ENCRYPTION_INIT_INFO: AVPacketSideDataType = 24;
pub const AV_PKT_DATA_A53_CC: AVPacketSideDataType = 23;
pub const AV_PKT_DATA_CONTENT_LIGHT_LEVEL: AVPacketSideDataType = 22;
pub const AV_PKT_DATA_SPHERICAL: AVPacketSideDataType = 21;
pub const AV_PKT_DATA_MASTERING_DISPLAY_METADATA: AVPacketSideDataType = 20;
pub const AV_PKT_DATA_MPEGTS_STREAM_ID: AVPacketSideDataType = 19;
pub const AV_PKT_DATA_METADATA_UPDATE: AVPacketSideDataType = 18;
pub const AV_PKT_DATA_WEBVTT_SETTINGS: AVPacketSideDataType = 17;
pub const AV_PKT_DATA_WEBVTT_IDENTIFIER: AVPacketSideDataType = 16;
pub const AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL: AVPacketSideDataType = 15;
pub const AV_PKT_DATA_SUBTITLE_POSITION: AVPacketSideDataType = 14;
pub const AV_PKT_DATA_STRINGS_METADATA: AVPacketSideDataType = 13;
pub const AV_PKT_DATA_JP_DUALMONO: AVPacketSideDataType = 12;
pub const AV_PKT_DATA_SKIP_SAMPLES: AVPacketSideDataType = 11;
pub const AV_PKT_DATA_CPB_PROPERTIES: AVPacketSideDataType = 10;
pub const AV_PKT_DATA_FALLBACK_TRACK: AVPacketSideDataType = 9;
pub const AV_PKT_DATA_QUALITY_STATS: AVPacketSideDataType = 8;
pub const AV_PKT_DATA_AUDIO_SERVICE_TYPE: AVPacketSideDataType = 7;
pub const AV_PKT_DATA_STEREO3D: AVPacketSideDataType = 6;
pub const AV_PKT_DATA_DISPLAYMATRIX: AVPacketSideDataType = 5;
pub const AV_PKT_DATA_REPLAYGAIN: AVPacketSideDataType = 4;
pub const AV_PKT_DATA_H263_MB_INFO: AVPacketSideDataType = 3;
pub const AV_PKT_DATA_PARAM_CHANGE: AVPacketSideDataType = 2;
pub const AV_PKT_DATA_NEW_EXTRADATA: AVPacketSideDataType = 1;
pub const AV_PKT_DATA_PALETTE: AVPacketSideDataType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacketSideData {
    pub data: *mut uint8_t,
    pub size: libc::c_int,
    pub type_0: AVPacketSideDataType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacket {
    pub buf: *mut AVBufferRef,
    pub pts: int64_t,
    pub dts: int64_t,
    pub data: *mut uint8_t,
    pub size: libc::c_int,
    pub stream_index: libc::c_int,
    pub flags: libc::c_int,
    pub side_data: *mut AVPacketSideData,
    pub side_data_elems: libc::c_int,
    pub duration: int64_t,
    pub pos: int64_t,
    pub convergence_duration: int64_t,
}
pub type AVFieldOrder = libc::c_uint;
pub const AV_FIELD_BT: AVFieldOrder = 5;
pub const AV_FIELD_TB: AVFieldOrder = 4;
pub const AV_FIELD_BB: AVFieldOrder = 3;
pub const AV_FIELD_TT: AVFieldOrder = 2;
pub const AV_FIELD_PROGRESSIVE: AVFieldOrder = 1;
pub const AV_FIELD_UNKNOWN: AVFieldOrder = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecContext {
    pub av_class: *const AVClass,
    pub log_level_offset: libc::c_int,
    pub codec_type: AVMediaType,
    pub codec: *const AVCodec,
    pub codec_id: AVCodecID,
    pub codec_tag: libc::c_uint,
    pub priv_data: *mut libc::c_void,
    pub internal: *mut AVCodecInternal,
    pub opaque: *mut libc::c_void,
    pub bit_rate: int64_t,
    pub bit_rate_tolerance: libc::c_int,
    pub global_quality: libc::c_int,
    pub compression_level: libc::c_int,
    pub flags: libc::c_int,
    pub flags2: libc::c_int,
    pub extradata: *mut uint8_t,
    pub extradata_size: libc::c_int,
    pub time_base: AVRational,
    pub ticks_per_frame: libc::c_int,
    pub delay: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub coded_width: libc::c_int,
    pub coded_height: libc::c_int,
    pub gop_size: libc::c_int,
    pub pix_fmt: AVPixelFormat,
    pub draw_horiz_band: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const AVFrame,
            *mut libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub get_format: Option::<
        unsafe extern "C" fn(*mut AVCodecContext, *const AVPixelFormat) -> AVPixelFormat,
    >,
    pub max_b_frames: libc::c_int,
    pub b_quant_factor: libc::c_float,
    pub b_frame_strategy: libc::c_int,
    pub b_quant_offset: libc::c_float,
    pub has_b_frames: libc::c_int,
    pub mpeg_quant: libc::c_int,
    pub i_quant_factor: libc::c_float,
    pub i_quant_offset: libc::c_float,
    pub lumi_masking: libc::c_float,
    pub temporal_cplx_masking: libc::c_float,
    pub spatial_cplx_masking: libc::c_float,
    pub p_masking: libc::c_float,
    pub dark_masking: libc::c_float,
    pub slice_count: libc::c_int,
    pub prediction_method: libc::c_int,
    pub slice_offset: *mut libc::c_int,
    pub sample_aspect_ratio: AVRational,
    pub me_cmp: libc::c_int,
    pub me_sub_cmp: libc::c_int,
    pub mb_cmp: libc::c_int,
    pub ildct_cmp: libc::c_int,
    pub dia_size: libc::c_int,
    pub last_predictor_count: libc::c_int,
    pub pre_me: libc::c_int,
    pub me_pre_cmp: libc::c_int,
    pub pre_dia_size: libc::c_int,
    pub me_subpel_quality: libc::c_int,
    pub me_range: libc::c_int,
    pub slice_flags: libc::c_int,
    pub mb_decision: libc::c_int,
    pub intra_matrix: *mut uint16_t,
    pub inter_matrix: *mut uint16_t,
    pub scenechange_threshold: libc::c_int,
    pub noise_reduction: libc::c_int,
    pub intra_dc_precision: libc::c_int,
    pub skip_top: libc::c_int,
    pub skip_bottom: libc::c_int,
    pub mb_lmin: libc::c_int,
    pub mb_lmax: libc::c_int,
    pub me_penalty_compensation: libc::c_int,
    pub bidir_refine: libc::c_int,
    pub brd_scale: libc::c_int,
    pub keyint_min: libc::c_int,
    pub refs: libc::c_int,
    pub chromaoffset: libc::c_int,
    pub mv0_threshold: libc::c_int,
    pub b_sensitivity: libc::c_int,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub color_range: AVColorRange,
    pub chroma_sample_location: AVChromaLocation,
    pub slices: libc::c_int,
    pub field_order: AVFieldOrder,
    pub sample_rate: libc::c_int,
    pub channels: libc::c_int,
    pub sample_fmt: AVSampleFormat,
    pub frame_size: libc::c_int,
    pub frame_number: libc::c_int,
    pub block_align: libc::c_int,
    pub cutoff: libc::c_int,
    pub channel_layout: uint64_t,
    pub request_channel_layout: uint64_t,
    pub audio_service_type: AVAudioServiceType,
    pub request_sample_fmt: AVSampleFormat,
    pub get_buffer2: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVFrame,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub refcounted_frames: libc::c_int,
    pub qcompress: libc::c_float,
    pub qblur: libc::c_float,
    pub qmin: libc::c_int,
    pub qmax: libc::c_int,
    pub max_qdiff: libc::c_int,
    pub rc_buffer_size: libc::c_int,
    pub rc_override_count: libc::c_int,
    pub rc_override: *mut RcOverride,
    pub rc_max_rate: int64_t,
    pub rc_min_rate: int64_t,
    pub rc_max_available_vbv_use: libc::c_float,
    pub rc_min_vbv_overflow_use: libc::c_float,
    pub rc_initial_buffer_occupancy: libc::c_int,
    pub coder_type: libc::c_int,
    pub context_model: libc::c_int,
    pub frame_skip_threshold: libc::c_int,
    pub frame_skip_factor: libc::c_int,
    pub frame_skip_exp: libc::c_int,
    pub frame_skip_cmp: libc::c_int,
    pub trellis: libc::c_int,
    pub min_prediction_order: libc::c_int,
    pub max_prediction_order: libc::c_int,
    pub timecode_frame_start: int64_t,
    pub rtp_callback: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub rtp_payload_size: libc::c_int,
    pub mv_bits: libc::c_int,
    pub header_bits: libc::c_int,
    pub i_tex_bits: libc::c_int,
    pub p_tex_bits: libc::c_int,
    pub i_count: libc::c_int,
    pub p_count: libc::c_int,
    pub skip_count: libc::c_int,
    pub misc_bits: libc::c_int,
    pub frame_bits: libc::c_int,
    pub stats_out: *mut libc::c_char,
    pub stats_in: *mut libc::c_char,
    pub workaround_bugs: libc::c_int,
    pub strict_std_compliance: libc::c_int,
    pub error_concealment: libc::c_int,
    pub debug: libc::c_int,
    pub err_recognition: libc::c_int,
    pub reordered_opaque: int64_t,
    pub hwaccel: *const AVHWAccel,
    pub hwaccel_context: *mut libc::c_void,
    pub error: [uint64_t; 8],
    pub dct_algo: libc::c_int,
    pub idct_algo: libc::c_int,
    pub bits_per_coded_sample: libc::c_int,
    pub bits_per_raw_sample: libc::c_int,
    pub lowres: libc::c_int,
    pub coded_frame: *mut AVFrame,
    pub thread_count: libc::c_int,
    pub thread_type: libc::c_int,
    pub active_thread_type: libc::c_int,
    pub thread_safe_callbacks: libc::c_int,
    pub execute: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            Option::<
                unsafe extern "C" fn(
                    *mut AVCodecContext,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            *mut libc::c_void,
            *mut libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub execute2: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            Option::<
                unsafe extern "C" fn(
                    *mut AVCodecContext,
                    *mut libc::c_void,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
            >,
            *mut libc::c_void,
            *mut libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub nsse_weight: libc::c_int,
    pub profile: libc::c_int,
    pub level: libc::c_int,
    pub skip_loop_filter: AVDiscard,
    pub skip_idct: AVDiscard,
    pub skip_frame: AVDiscard,
    pub subtitle_header: *mut uint8_t,
    pub subtitle_header_size: libc::c_int,
    pub vbv_delay: uint64_t,
    pub side_data_only_packets: libc::c_int,
    pub initial_padding: libc::c_int,
    pub framerate: AVRational,
    pub sw_pix_fmt: AVPixelFormat,
    pub pkt_timebase: AVRational,
    pub codec_descriptor: *const AVCodecDescriptor,
    pub pts_correction_num_faulty_pts: int64_t,
    pub pts_correction_num_faulty_dts: int64_t,
    pub pts_correction_last_pts: int64_t,
    pub pts_correction_last_dts: int64_t,
    pub sub_charenc: *mut libc::c_char,
    pub sub_charenc_mode: libc::c_int,
    pub skip_alpha: libc::c_int,
    pub seek_preroll: libc::c_int,
    pub debug_mv: libc::c_int,
    pub chroma_intra_matrix: *mut uint16_t,
    pub dump_separator: *mut uint8_t,
    pub codec_whitelist: *mut libc::c_char,
    pub properties: libc::c_uint,
    pub coded_side_data: *mut AVPacketSideData,
    pub nb_coded_side_data: libc::c_int,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub sub_text_format: libc::c_int,
    pub trailing_padding: libc::c_int,
    pub max_pixels: int64_t,
    pub hw_device_ctx: *mut AVBufferRef,
    pub hwaccel_flags: libc::c_int,
    pub apply_cropping: libc::c_int,
    pub extra_hw_frames: libc::c_int,
    pub discard_damaged_percentage: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVHWAccel {
    pub name: *const libc::c_char,
    pub type_0: AVMediaType,
    pub id: AVCodecID,
    pub pix_fmt: AVPixelFormat,
    pub capabilities: libc::c_int,
    pub alloc_frame: Option::<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVFrame) -> libc::c_int,
    >,
    pub start_frame: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const uint8_t,
            uint32_t,
        ) -> libc::c_int,
    >,
    pub decode_params: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            libc::c_int,
            *const uint8_t,
            uint32_t,
        ) -> libc::c_int,
    >,
    pub decode_slice: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const uint8_t,
            uint32_t,
        ) -> libc::c_int,
    >,
    pub end_frame: Option::<unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int>,
    pub frame_priv_data_size: libc::c_int,
    pub decode_mb: Option::<unsafe extern "C" fn(*mut MpegEncContext) -> ()>,
    pub init: Option::<unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int>,
    pub uninit: Option::<unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int>,
    pub priv_data_size: libc::c_int,
    pub caps_internal: libc::c_int,
    pub frame_params: Option::<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVBufferRef) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodec {
    pub name: *const libc::c_char,
    pub long_name: *const libc::c_char,
    pub type_0: AVMediaType,
    pub id: AVCodecID,
    pub capabilities: libc::c_int,
    pub supported_framerates: *const AVRational,
    pub pix_fmts: *const AVPixelFormat,
    pub supported_samplerates: *const libc::c_int,
    pub sample_fmts: *const AVSampleFormat,
    pub channel_layouts: *const uint64_t,
    pub max_lowres: uint8_t,
    pub priv_class: *const AVClass,
    pub profiles: *const AVProfile,
    pub wrapper_name: *const libc::c_char,
    pub priv_data_size: libc::c_int,
    pub next: *mut AVCodec,
    pub init_thread_copy: Option::<
        unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int,
    >,
    pub update_thread_context: Option::<
        unsafe extern "C" fn(*mut AVCodecContext, *const AVCodecContext) -> libc::c_int,
    >,
    pub defaults: *const AVCodecDefault,
    pub init_static_data: Option::<unsafe extern "C" fn(*mut AVCodec) -> ()>,
    pub init: Option::<unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int>,
    pub encode_sub: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut uint8_t,
            libc::c_int,
            *const AVSubtitle,
        ) -> libc::c_int,
    >,
    pub encode2: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVPacket,
            *const AVFrame,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub decode: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut libc::c_void,
            *mut libc::c_int,
            *mut AVPacket,
        ) -> libc::c_int,
    >,
    pub close: Option::<unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int>,
    pub send_frame: Option::<
        unsafe extern "C" fn(*mut AVCodecContext, *const AVFrame) -> libc::c_int,
    >,
    pub receive_packet: Option::<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVPacket) -> libc::c_int,
    >,
    pub receive_frame: Option::<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVFrame) -> libc::c_int,
    >,
    pub flush: Option::<unsafe extern "C" fn(*mut AVCodecContext) -> ()>,
    pub caps_internal: libc::c_int,
    pub bsfs: *const libc::c_char,
    pub hw_configs: *mut *const AVCodecHWConfigInternal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVSubtitle {
    pub format: uint16_t,
    pub start_display_time: uint32_t,
    pub end_display_time: uint32_t,
    pub num_rects: libc::c_uint,
    pub rects: *mut *mut AVSubtitleRect,
    pub pts: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVSubtitleRect {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub nb_colors: libc::c_int,
    pub pict: AVPicture,
    pub data: [*mut uint8_t; 4],
    pub linesize: [libc::c_int; 4],
    pub type_0: AVSubtitleType,
    pub text: *mut libc::c_char,
    pub ass: *mut libc::c_char,
    pub flags: libc::c_int,
}
pub type AVSubtitleType = libc::c_uint;
pub const SUBTITLE_ASS: AVSubtitleType = 3;
pub const SUBTITLE_TEXT: AVSubtitleType = 2;
pub const SUBTITLE_BITMAP: AVSubtitleType = 1;
pub const SUBTITLE_NONE: AVSubtitleType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPicture {
    pub data: [*mut uint8_t; 8],
    pub linesize: [libc::c_int; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecParameters {
    pub codec_type: AVMediaType,
    pub codec_id: AVCodecID,
    pub codec_tag: uint32_t,
    pub extradata: *mut uint8_t,
    pub extradata_size: libc::c_int,
    pub format: libc::c_int,
    pub bit_rate: int64_t,
    pub bits_per_coded_sample: libc::c_int,
    pub bits_per_raw_sample: libc::c_int,
    pub profile: libc::c_int,
    pub level: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub sample_aspect_ratio: AVRational,
    pub field_order: AVFieldOrder,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub color_space: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    pub video_delay: libc::c_int,
    pub channel_layout: uint64_t,
    pub channels: libc::c_int,
    pub sample_rate: libc::c_int,
    pub block_align: libc::c_int,
    pub frame_size: libc::c_int,
    pub initial_padding: libc::c_int,
    pub trailing_padding: libc::c_int,
    pub seek_preroll: libc::c_int,
}
pub type AVPictureStructure = libc::c_uint;
pub const AV_PICTURE_STRUCTURE_FRAME: AVPictureStructure = 3;
pub const AV_PICTURE_STRUCTURE_BOTTOM_FIELD: AVPictureStructure = 2;
pub const AV_PICTURE_STRUCTURE_TOP_FIELD: AVPictureStructure = 1;
pub const AV_PICTURE_STRUCTURE_UNKNOWN: AVPictureStructure = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecParserContext {
    pub priv_data: *mut libc::c_void,
    pub parser: *mut AVCodecParser,
    pub frame_offset: int64_t,
    pub cur_offset: int64_t,
    pub next_frame_offset: int64_t,
    pub pict_type: libc::c_int,
    pub repeat_pict: libc::c_int,
    pub pts: int64_t,
    pub dts: int64_t,
    pub last_pts: int64_t,
    pub last_dts: int64_t,
    pub fetch_timestamp: libc::c_int,
    pub cur_frame_start_index: libc::c_int,
    pub cur_frame_offset: [int64_t; 4],
    pub cur_frame_pts: [int64_t; 4],
    pub cur_frame_dts: [int64_t; 4],
    pub flags: libc::c_int,
    pub offset: int64_t,
    pub cur_frame_end: [int64_t; 4],
    pub key_frame: libc::c_int,
    pub convergence_duration: int64_t,
    pub dts_sync_point: libc::c_int,
    pub dts_ref_dts_delta: libc::c_int,
    pub pts_dts_delta: libc::c_int,
    pub cur_frame_pos: [int64_t; 4],
    pub pos: int64_t,
    pub last_pos: int64_t,
    pub duration: libc::c_int,
    pub field_order: AVFieldOrder,
    pub picture_structure: AVPictureStructure,
    pub output_picture_number: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub coded_width: libc::c_int,
    pub coded_height: libc::c_int,
    pub format: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecParser {
    pub codec_ids: [libc::c_int; 5],
    pub priv_data_size: libc::c_int,
    pub parser_init: Option::<
        unsafe extern "C" fn(*mut AVCodecParserContext) -> libc::c_int,
    >,
    pub parser_parse: Option::<
        unsafe extern "C" fn(
            *mut AVCodecParserContext,
            *mut AVCodecContext,
            *mut *const uint8_t,
            *mut libc::c_int,
            *const uint8_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub parser_close: Option::<unsafe extern "C" fn(*mut AVCodecParserContext) -> ()>,
    pub split: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const uint8_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub next: *mut AVCodecParser,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVIOInterruptCB {
    pub callback: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub opaque: *mut libc::c_void,
}
pub type AVIODataMarkerType = libc::c_uint;
pub const AVIO_DATA_MARKER_FLUSH_POINT: AVIODataMarkerType = 5;
pub const AVIO_DATA_MARKER_TRAILER: AVIODataMarkerType = 4;
pub const AVIO_DATA_MARKER_UNKNOWN: AVIODataMarkerType = 3;
pub const AVIO_DATA_MARKER_BOUNDARY_POINT: AVIODataMarkerType = 2;
pub const AVIO_DATA_MARKER_SYNC_POINT: AVIODataMarkerType = 1;
pub const AVIO_DATA_MARKER_HEADER: AVIODataMarkerType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVIOContext {
    pub av_class: *const AVClass,
    pub buffer: *mut libc::c_uchar,
    pub buffer_size: libc::c_int,
    pub buf_ptr: *mut libc::c_uchar,
    pub buf_end: *mut libc::c_uchar,
    pub opaque: *mut libc::c_void,
    pub read_packet: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut uint8_t, libc::c_int) -> libc::c_int,
    >,
    pub write_packet: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut uint8_t, libc::c_int) -> libc::c_int,
    >,
    pub seek: Option::<
        unsafe extern "C" fn(*mut libc::c_void, int64_t, libc::c_int) -> int64_t,
    >,
    pub pos: int64_t,
    pub eof_reached: libc::c_int,
    pub write_flag: libc::c_int,
    pub max_packet_size: libc::c_int,
    pub checksum: libc::c_ulong,
    pub checksum_ptr: *mut libc::c_uchar,
    pub update_checksum: Option::<
        unsafe extern "C" fn(
            libc::c_ulong,
            *const uint8_t,
            libc::c_uint,
        ) -> libc::c_ulong,
    >,
    pub error: libc::c_int,
    pub read_pause: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> libc::c_int,
    >,
    pub read_seek: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            int64_t,
            libc::c_int,
        ) -> int64_t,
    >,
    pub seekable: libc::c_int,
    pub maxsize: int64_t,
    pub direct: libc::c_int,
    pub bytes_read: int64_t,
    pub seek_count: libc::c_int,
    pub writeout_count: libc::c_int,
    pub orig_buffer_size: libc::c_int,
    pub short_seek_threshold: libc::c_int,
    pub protocol_whitelist: *const libc::c_char,
    pub protocol_blacklist: *const libc::c_char,
    pub write_data_type: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut uint8_t,
            libc::c_int,
            AVIODataMarkerType,
            int64_t,
        ) -> libc::c_int,
    >,
    pub ignore_boundary_point: libc::c_int,
    pub current_type: AVIODataMarkerType,
    pub last_time: int64_t,
    pub short_seek_get: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub written: int64_t,
    pub buf_ptr_max: *mut libc::c_uchar,
    pub min_packet_size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFormatContext {
    pub av_class: *const AVClass,
    pub iformat: *mut AVInputFormat,
    pub oformat: *mut AVOutputFormat,
    pub priv_data: *mut libc::c_void,
    pub pb: *mut AVIOContext,
    pub ctx_flags: libc::c_int,
    pub nb_streams: libc::c_uint,
    pub streams: *mut *mut AVStream,
    pub filename: [libc::c_char; 1024],
    pub url: *mut libc::c_char,
    pub start_time: int64_t,
    pub duration: int64_t,
    pub bit_rate: int64_t,
    pub packet_size: libc::c_uint,
    pub max_delay: libc::c_int,
    pub flags: libc::c_int,
    pub probesize: int64_t,
    pub max_analyze_duration: int64_t,
    pub key: *const uint8_t,
    pub keylen: libc::c_int,
    pub nb_programs: libc::c_uint,
    pub programs: *mut *mut AVProgram,
    pub video_codec_id: AVCodecID,
    pub audio_codec_id: AVCodecID,
    pub subtitle_codec_id: AVCodecID,
    pub max_index_size: libc::c_uint,
    pub max_picture_buffer: libc::c_uint,
    pub nb_chapters: libc::c_uint,
    pub chapters: *mut *mut AVChapter,
    pub metadata: *mut AVDictionary,
    pub start_time_realtime: int64_t,
    pub fps_probe_size: libc::c_int,
    pub error_recognition: libc::c_int,
    pub interrupt_callback: AVIOInterruptCB,
    pub debug: libc::c_int,
    pub max_interleave_delta: int64_t,
    pub strict_std_compliance: libc::c_int,
    pub event_flags: libc::c_int,
    pub max_ts_probe: libc::c_int,
    pub avoid_negative_ts: libc::c_int,
    pub ts_id: libc::c_int,
    pub audio_preload: libc::c_int,
    pub max_chunk_duration: libc::c_int,
    pub max_chunk_size: libc::c_int,
    pub use_wallclock_as_timestamps: libc::c_int,
    pub avio_flags: libc::c_int,
    pub duration_estimation_method: AVDurationEstimationMethod,
    pub skip_initial_bytes: int64_t,
    pub correct_ts_overflow: libc::c_uint,
    pub seek2any: libc::c_int,
    pub flush_packets: libc::c_int,
    pub probe_score: libc::c_int,
    pub format_probesize: libc::c_int,
    pub codec_whitelist: *mut libc::c_char,
    pub format_whitelist: *mut libc::c_char,
    pub internal: *mut AVFormatInternal,
    pub io_repositioned: libc::c_int,
    pub video_codec: *mut AVCodec,
    pub audio_codec: *mut AVCodec,
    pub subtitle_codec: *mut AVCodec,
    pub data_codec: *mut AVCodec,
    pub metadata_header_padding: libc::c_int,
    pub opaque: *mut libc::c_void,
    pub control_message_cb: av_format_control_message,
    pub output_ts_offset: int64_t,
    pub dump_separator: *mut uint8_t,
    pub data_codec_id: AVCodecID,
    pub open_cb: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut *mut AVIOContext,
            *const libc::c_char,
            libc::c_int,
            *const AVIOInterruptCB,
            *mut *mut AVDictionary,
        ) -> libc::c_int,
    >,
    pub protocol_whitelist: *mut libc::c_char,
    pub io_open: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut *mut AVIOContext,
            *const libc::c_char,
            libc::c_int,
            *mut *mut AVDictionary,
        ) -> libc::c_int,
    >,
    pub io_close: Option::<
        unsafe extern "C" fn(*mut AVFormatContext, *mut AVIOContext) -> (),
    >,
    pub protocol_blacklist: *mut libc::c_char,
    pub max_streams: libc::c_int,
    pub skip_estimate_duration_from_pts: libc::c_int,
}
pub type av_format_control_message = Option::<
    unsafe extern "C" fn(
        *mut AVFormatContext,
        libc::c_int,
        *mut libc::c_void,
        size_t,
    ) -> libc::c_int,
>;
pub type AVDurationEstimationMethod = libc::c_uint;
pub const AVFMT_DURATION_FROM_BITRATE: AVDurationEstimationMethod = 2;
pub const AVFMT_DURATION_FROM_STREAM: AVDurationEstimationMethod = 1;
pub const AVFMT_DURATION_FROM_PTS: AVDurationEstimationMethod = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVChapter {
    pub id: libc::c_int,
    pub time_base: AVRational,
    pub start: int64_t,
    pub end: int64_t,
    pub metadata: *mut AVDictionary,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVProgram {
    pub id: libc::c_int,
    pub flags: libc::c_int,
    pub discard: AVDiscard,
    pub stream_index: *mut libc::c_uint,
    pub nb_stream_indexes: libc::c_uint,
    pub metadata: *mut AVDictionary,
    pub program_num: libc::c_int,
    pub pmt_pid: libc::c_int,
    pub pcr_pid: libc::c_int,
    pub pmt_version: libc::c_int,
    pub start_time: int64_t,
    pub end_time: int64_t,
    pub pts_wrap_reference: int64_t,
    pub pts_wrap_behavior: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVStream {
    pub index: libc::c_int,
    pub id: libc::c_int,
    pub codec: *mut AVCodecContext,
    pub priv_data: *mut libc::c_void,
    pub time_base: AVRational,
    pub start_time: int64_t,
    pub duration: int64_t,
    pub nb_frames: int64_t,
    pub disposition: libc::c_int,
    pub discard: AVDiscard,
    pub sample_aspect_ratio: AVRational,
    pub metadata: *mut AVDictionary,
    pub avg_frame_rate: AVRational,
    pub attached_pic: AVPacket,
    pub side_data: *mut AVPacketSideData,
    pub nb_side_data: libc::c_int,
    pub event_flags: libc::c_int,
    pub r_frame_rate: AVRational,
    pub recommended_encoder_configuration: *mut libc::c_char,
    pub codecpar: *mut AVCodecParameters,
    pub info: *mut C2RustUnnamed_14,
    pub pts_wrap_bits: libc::c_int,
    pub first_dts: int64_t,
    pub cur_dts: int64_t,
    pub last_IP_pts: int64_t,
    pub last_IP_duration: libc::c_int,
    pub probe_packets: libc::c_int,
    pub codec_info_nb_frames: libc::c_int,
    pub need_parsing: AVStreamParseType,
    pub parser: *mut AVCodecParserContext,
    pub last_in_packet_buffer: *mut AVPacketList,
    pub probe_data: AVProbeData,
    pub pts_buffer: [int64_t; 17],
    pub index_entries: *mut AVIndexEntry,
    pub nb_index_entries: libc::c_int,
    pub index_entries_allocated_size: libc::c_uint,
    pub stream_identifier: libc::c_int,
    pub program_num: libc::c_int,
    pub pmt_version: libc::c_int,
    pub pmt_stream_idx: libc::c_int,
    pub interleaver_chunk_size: int64_t,
    pub interleaver_chunk_duration: int64_t,
    pub request_probe: libc::c_int,
    pub skip_to_keyframe: libc::c_int,
    pub skip_samples: libc::c_int,
    pub start_skip_samples: int64_t,
    pub first_discard_sample: int64_t,
    pub last_discard_sample: int64_t,
    pub nb_decoded_frames: libc::c_int,
    pub mux_ts_offset: int64_t,
    pub pts_wrap_reference: int64_t,
    pub pts_wrap_behavior: libc::c_int,
    pub update_initial_durations_done: libc::c_int,
    pub pts_reorder_error: [int64_t; 17],
    pub pts_reorder_error_count: [uint8_t; 17],
    pub last_dts_for_order_check: int64_t,
    pub dts_ordered: uint8_t,
    pub dts_misordered: uint8_t,
    pub inject_global_side_data: libc::c_int,
    pub display_aspect_ratio: AVRational,
    pub internal: *mut AVStreamInternal,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct AVIndexEntry {
    pub pos: int64_t,
    pub timestamp: int64_t,
    #[bitfield(name = "flags", ty = "libc::c_int", bits = "0..=1")]
    #[bitfield(name = "size", ty = "libc::c_int", bits = "2..=31")]
    pub flags_size: [u8; 4],
    pub min_distance: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVProbeData {
    pub filename: *const libc::c_char,
    pub buf: *mut libc::c_uchar,
    pub buf_size: libc::c_int,
    pub mime_type: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacketList {
    pub pkt: AVPacket,
    pub next: *mut AVPacketList,
}
pub type AVStreamParseType = libc::c_uint;
pub const AVSTREAM_PARSE_FULL_RAW: AVStreamParseType = 5;
pub const AVSTREAM_PARSE_FULL_ONCE: AVStreamParseType = 4;
pub const AVSTREAM_PARSE_TIMESTAMPS: AVStreamParseType = 3;
pub const AVSTREAM_PARSE_HEADERS: AVStreamParseType = 2;
pub const AVSTREAM_PARSE_FULL: AVStreamParseType = 1;
pub const AVSTREAM_PARSE_NONE: AVStreamParseType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub last_dts: int64_t,
    pub duration_gcd: int64_t,
    pub duration_count: libc::c_int,
    pub rfps_duration_sum: int64_t,
    pub duration_error: *mut [[libc::c_double; 399]; 2],
    pub codec_info_duration: int64_t,
    pub codec_info_duration_fields: int64_t,
    pub frame_delay_evidence: libc::c_int,
    pub found_decoder: libc::c_int,
    pub last_duration: int64_t,
    pub fps_first_dts: int64_t,
    pub fps_first_dts_idx: libc::c_int,
    pub fps_last_dts: int64_t,
    pub fps_last_dts_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOutputFormat {
    pub name: *const libc::c_char,
    pub long_name: *const libc::c_char,
    pub mime_type: *const libc::c_char,
    pub extensions: *const libc::c_char,
    pub audio_codec: AVCodecID,
    pub video_codec: AVCodecID,
    pub subtitle_codec: AVCodecID,
    pub flags: libc::c_int,
    pub codec_tag: *const *const AVCodecTag,
    pub priv_class: *const AVClass,
    pub next: *mut AVOutputFormat,
    pub priv_data_size: libc::c_int,
    pub write_header: Option::<
        unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int,
    >,
    pub write_packet: Option::<
        unsafe extern "C" fn(*mut AVFormatContext, *mut AVPacket) -> libc::c_int,
    >,
    pub write_trailer: Option::<
        unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int,
    >,
    pub interleave_packet: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut AVPacket,
            *mut AVPacket,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub query_codec: Option::<
        unsafe extern "C" fn(AVCodecID, libc::c_int) -> libc::c_int,
    >,
    pub get_output_timestamp: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            libc::c_int,
            *mut int64_t,
            *mut int64_t,
        ) -> (),
    >,
    pub control_message: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            libc::c_int,
            *mut libc::c_void,
            size_t,
        ) -> libc::c_int,
    >,
    pub write_uncoded_frame: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            libc::c_int,
            *mut *mut AVFrame,
            libc::c_uint,
        ) -> libc::c_int,
    >,
    pub get_device_list: Option::<
        unsafe extern "C" fn(*mut AVFormatContext, *mut AVDeviceInfoList) -> libc::c_int,
    >,
    pub create_device_capabilities: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut AVDeviceCapabilitiesQuery,
        ) -> libc::c_int,
    >,
    pub free_device_capabilities: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut AVDeviceCapabilitiesQuery,
        ) -> libc::c_int,
    >,
    pub data_codec: AVCodecID,
    pub init: Option::<unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int>,
    pub deinit: Option::<unsafe extern "C" fn(*mut AVFormatContext) -> ()>,
    pub check_bitstream: Option::<
        unsafe extern "C" fn(*mut AVFormatContext, *const AVPacket) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVInputFormat {
    pub name: *const libc::c_char,
    pub long_name: *const libc::c_char,
    pub flags: libc::c_int,
    pub extensions: *const libc::c_char,
    pub codec_tag: *const *const AVCodecTag,
    pub priv_class: *const AVClass,
    pub mime_type: *const libc::c_char,
    pub next: *mut AVInputFormat,
    pub raw_codec_id: libc::c_int,
    pub priv_data_size: libc::c_int,
    pub read_probe: Option::<unsafe extern "C" fn(*const AVProbeData) -> libc::c_int>,
    pub read_header: Option::<unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int>,
    pub read_packet: Option::<
        unsafe extern "C" fn(*mut AVFormatContext, *mut AVPacket) -> libc::c_int,
    >,
    pub read_close: Option::<unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int>,
    pub read_seek: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            libc::c_int,
            int64_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub read_timestamp: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            libc::c_int,
            *mut int64_t,
            int64_t,
        ) -> int64_t,
    >,
    pub read_play: Option::<unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int>,
    pub read_pause: Option::<unsafe extern "C" fn(*mut AVFormatContext) -> libc::c_int>,
    pub read_seek2: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            libc::c_int,
            int64_t,
            int64_t,
            int64_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub get_device_list: Option::<
        unsafe extern "C" fn(*mut AVFormatContext, *mut AVDeviceInfoList) -> libc::c_int,
    >,
    pub create_device_capabilities: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut AVDeviceCapabilitiesQuery,
        ) -> libc::c_int,
    >,
    pub free_device_capabilities: Option::<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut AVDeviceCapabilitiesQuery,
        ) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFilterContext {
    pub av_class: *const AVClass,
    pub filter: *const AVFilter,
    pub name: *mut libc::c_char,
    pub input_pads: *mut AVFilterPad,
    pub inputs: *mut *mut AVFilterLink,
    pub nb_inputs: libc::c_uint,
    pub output_pads: *mut AVFilterPad,
    pub outputs: *mut *mut AVFilterLink,
    pub nb_outputs: libc::c_uint,
    pub priv_0: *mut libc::c_void,
    pub graph: *mut AVFilterGraph,
    pub thread_type: libc::c_int,
    pub internal: *mut AVFilterInternal,
    pub command_queue: *mut AVFilterCommand,
    pub enable_str: *mut libc::c_char,
    pub enable: *mut libc::c_void,
    pub var_values: *mut libc::c_double,
    pub is_disabled: libc::c_int,
    pub hw_device_ctx: *mut AVBufferRef,
    pub nb_threads: libc::c_int,
    pub ready: libc::c_uint,
    pub extra_hw_frames: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFilterGraph {
    pub av_class: *const AVClass,
    pub filters: *mut *mut AVFilterContext,
    pub nb_filters: libc::c_uint,
    pub scale_sws_opts: *mut libc::c_char,
    pub resample_lavr_opts: *mut libc::c_char,
    pub thread_type: libc::c_int,
    pub nb_threads: libc::c_int,
    pub internal: *mut AVFilterGraphInternal,
    pub opaque: *mut libc::c_void,
    pub execute: Option::<avfilter_execute_func>,
    pub aresample_swr_opts: *mut libc::c_char,
    pub sink_links: *mut *mut AVFilterLink,
    pub sink_links_count: libc::c_int,
    pub disable_auto_convert: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFilterLink {
    pub src: *mut AVFilterContext,
    pub srcpad: *mut AVFilterPad,
    pub dst: *mut AVFilterContext,
    pub dstpad: *mut AVFilterPad,
    pub type_0: AVMediaType,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub sample_aspect_ratio: AVRational,
    pub channel_layout: uint64_t,
    pub sample_rate: libc::c_int,
    pub format: libc::c_int,
    pub time_base: AVRational,
    pub in_formats: *mut AVFilterFormats,
    pub out_formats: *mut AVFilterFormats,
    pub in_samplerates: *mut AVFilterFormats,
    pub out_samplerates: *mut AVFilterFormats,
    pub in_channel_layouts: *mut AVFilterChannelLayouts,
    pub out_channel_layouts: *mut AVFilterChannelLayouts,
    pub request_samples: libc::c_int,
    pub init_state: C2RustUnnamed_15,
    pub graph: *mut AVFilterGraph,
    pub current_pts: int64_t,
    pub current_pts_us: int64_t,
    pub age_index: libc::c_int,
    pub frame_rate: AVRational,
    pub partial_buf: *mut AVFrame,
    pub partial_buf_size: libc::c_int,
    pub min_samples: libc::c_int,
    pub max_samples: libc::c_int,
    pub channels: libc::c_int,
    pub flags: libc::c_uint,
    pub frame_count_in: int64_t,
    pub frame_count_out: int64_t,
    pub frame_pool: *mut libc::c_void,
    pub frame_wanted_out: libc::c_int,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub reserved: [libc::c_char; 61440],
}
pub type C2RustUnnamed_15 = libc::c_uint;
pub const AVLINK_INIT: C2RustUnnamed_15 = 2;
pub const AVLINK_STARTINIT: C2RustUnnamed_15 = 1;
pub const AVLINK_UNINIT: C2RustUnnamed_15 = 0;
pub type avfilter_execute_func = unsafe extern "C" fn(
    *mut AVFilterContext,
    Option::<avfilter_action_func>,
    *mut libc::c_void,
    *mut libc::c_int,
    libc::c_int,
) -> libc::c_int;
pub type avfilter_action_func = unsafe extern "C" fn(
    *mut AVFilterContext,
    *mut libc::c_void,
    libc::c_int,
    libc::c_int,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFilter {
    pub name: *const libc::c_char,
    pub description: *const libc::c_char,
    pub inputs: *const AVFilterPad,
    pub outputs: *const AVFilterPad,
    pub priv_class: *const AVClass,
    pub flags: libc::c_int,
    pub preinit: Option::<unsafe extern "C" fn(*mut AVFilterContext) -> libc::c_int>,
    pub init: Option::<unsafe extern "C" fn(*mut AVFilterContext) -> libc::c_int>,
    pub init_dict: Option::<
        unsafe extern "C" fn(*mut AVFilterContext, *mut *mut AVDictionary) -> libc::c_int,
    >,
    pub uninit: Option::<unsafe extern "C" fn(*mut AVFilterContext) -> ()>,
    pub query_formats: Option::<
        unsafe extern "C" fn(*mut AVFilterContext) -> libc::c_int,
    >,
    pub priv_size: libc::c_int,
    pub flags_internal: libc::c_int,
    pub next: *mut AVFilter,
    pub process_command: Option::<
        unsafe extern "C" fn(
            *mut AVFilterContext,
            *const libc::c_char,
            *const libc::c_char,
            *mut libc::c_char,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub init_opaque: Option::<
        unsafe extern "C" fn(*mut AVFilterContext, *mut libc::c_void) -> libc::c_int,
    >,
    pub activate: Option::<unsafe extern "C" fn(*mut AVFilterContext) -> libc::c_int>,
}
pub type BarKeyShortcutId_t = libc::c_uint;
pub const BAR_KS_COUNT: BarKeyShortcutId_t = 30;
pub const BAR_KS_SETTINGS: BarKeyShortcutId_t = 29;
pub const BAR_KS_VOLRESET: BarKeyShortcutId_t = 28;
pub const BAR_KS_PAUSE: BarKeyShortcutId_t = 27;
pub const BAR_KS_PLAY: BarKeyShortcutId_t = 26;
pub const BAR_KS_CREATESTATIONFROMSONG: BarKeyShortcutId_t = 25;
pub const BAR_KS_PLAYPAUSE2: BarKeyShortcutId_t = 24;
pub const BAR_KS_MANAGESTATION: BarKeyShortcutId_t = 23;
pub const BAR_KS_VOLUP: BarKeyShortcutId_t = 22;
pub const BAR_KS_VOLDOWN: BarKeyShortcutId_t = 21;
pub const BAR_KS_BOOKMARK: BarKeyShortcutId_t = 20;
pub const BAR_KS_DEBUG: BarKeyShortcutId_t = 19;
pub const BAR_KS_SELECTQUICKMIX: BarKeyShortcutId_t = 18;
pub const BAR_KS_UPCOMING: BarKeyShortcutId_t = 17;
pub const BAR_KS_TIRED: BarKeyShortcutId_t = 16;
pub const BAR_KS_SELECTSTATION: BarKeyShortcutId_t = 15;
pub const BAR_KS_RENAMESTATION: BarKeyShortcutId_t = 14;
pub const BAR_KS_QUIT: BarKeyShortcutId_t = 13;
pub const BAR_KS_PLAYPAUSE: BarKeyShortcutId_t = 12;
pub const BAR_KS_SKIP: BarKeyShortcutId_t = 11;
pub const BAR_KS_ADDSHARED: BarKeyShortcutId_t = 10;
pub const BAR_KS_INFO: BarKeyShortcutId_t = 9;
pub const BAR_KS_HISTORY: BarKeyShortcutId_t = 8;
pub const BAR_KS_GENRESTATION: BarKeyShortcutId_t = 7;
pub const BAR_KS_EXPLAIN: BarKeyShortcutId_t = 6;
pub const BAR_KS_DELETESTATION: BarKeyShortcutId_t = 5;
pub const BAR_KS_CREATESTATION: BarKeyShortcutId_t = 4;
pub const BAR_KS_ADDMUSIC: BarKeyShortcutId_t = 3;
pub const BAR_KS_BAN: BarKeyShortcutId_t = 2;
pub const BAR_KS_LOVE: BarKeyShortcutId_t = 1;
pub const BAR_KS_HELP: BarKeyShortcutId_t = 0;
pub type BarStationSorting_t = libc::c_uint;
pub const BAR_SORT_COUNT: BarStationSorting_t = 6;
pub const BAR_SORT_QUICKMIX_10_NAME_ZA: BarStationSorting_t = 5;
pub const BAR_SORT_QUICKMIX_10_NAME_AZ: BarStationSorting_t = 4;
pub const BAR_SORT_QUICKMIX_01_NAME_ZA: BarStationSorting_t = 3;
pub const BAR_SORT_QUICKMIX_01_NAME_AZ: BarStationSorting_t = 2;
pub const BAR_SORT_NAME_ZA: BarStationSorting_t = 1;
pub const BAR_SORT_NAME_AZ: BarStationSorting_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BarMsgFormatStr_t {
    pub prefix: *mut libc::c_char,
    pub postfix: *mut libc::c_char,
}
pub type BarUiMsg_t = libc::c_uint;
pub const MSG_COUNT: BarUiMsg_t = 7;
pub const MSG_LIST: BarUiMsg_t = 6;
pub const MSG_QUESTION: BarUiMsg_t = 5;
pub const MSG_ERR: BarUiMsg_t = 4;
pub const MSG_TIME: BarUiMsg_t = 3;
pub const MSG_PLAYING: BarUiMsg_t = 2;
pub const MSG_INFO: BarUiMsg_t = 1;
pub const MSG_NONE: BarUiMsg_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BarSettings_t {
    pub autoselect: bool,
    pub history: libc::c_uint,
    pub maxRetry: libc::c_uint,
    pub timeout: libc::c_uint,
    pub bufferSecs: libc::c_uint,
    pub volume: libc::c_int,
    pub gainMul: libc::c_float,
    pub sortOrder: BarStationSorting_t,
    pub audioQuality: PianoAudioQuality_t,
    pub username: *mut libc::c_char,
    pub password: *mut libc::c_char,
    pub passwordCmd: *mut libc::c_char,
    pub controlProxy: *mut libc::c_char,
    pub proxy: *mut libc::c_char,
    pub bindTo: *mut libc::c_char,
    pub autostartStation: *mut libc::c_char,
    pub eventCmd: *mut libc::c_char,
    pub loveIcon: *mut libc::c_char,
    pub banIcon: *mut libc::c_char,
    pub tiredIcon: *mut libc::c_char,
    pub atIcon: *mut libc::c_char,
    pub npSongFormat: *mut libc::c_char,
    pub npStationFormat: *mut libc::c_char,
    pub listSongFormat: *mut libc::c_char,
    pub timeFormat: *mut libc::c_char,
    pub fifo: *mut libc::c_char,
    pub rpcHost: *mut libc::c_char,
    pub rpcTlsPort: *mut libc::c_char,
    pub partnerUser: *mut libc::c_char,
    pub partnerPassword: *mut libc::c_char,
    pub device: *mut libc::c_char,
    pub inkey: *mut libc::c_char,
    pub outkey: *mut libc::c_char,
    pub caBundle: *mut libc::c_char,
    pub audioPipe: *mut libc::c_char,
    pub keys: [libc::c_char; 30],
    pub sampleRate: libc::c_int,
    pub msgFormat: [BarMsgFormatStr_t; 7],
}
pub type BarPlayerMode = libc::c_uint;
pub const PLAYER_FINISHED: BarPlayerMode = 3;
pub const PLAYER_PLAYING: BarPlayerMode = 2;
pub const PLAYER_WAITING: BarPlayerMode = 1;
pub const PLAYER_DEAD: BarPlayerMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_t {
    pub lock: pthread_mutex_t,
    pub aoplayLock: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub aoplayCond: pthread_cond_t,
    pub doQuit: bool,
    pub doPause: bool,
    pub songDuration: libc::c_uint,
    pub songPlayed: libc::c_uint,
    pub mode: BarPlayerMode,
    pub fvolume: *mut AVFilterContext,
    pub fgraph: *mut AVFilterGraph,
    pub fctx: *mut AVFormatContext,
    pub st: *mut AVStream,
    pub cctx: *mut AVCodecContext,
    pub fbufsink: *mut AVFilterContext,
    pub fabuf: *mut AVFilterContext,
    pub streamIdx: libc::c_int,
    pub lastTimestamp: int64_t,
    pub interrupted: sig_atomic_t,
    pub aoDev: *mut ao_device,
    pub gain: libc::c_double,
    pub url: *mut libc::c_char,
    pub settings: *const BarSettings_t,
}
pub type C2RustUnnamed_16 = libc::c_uint;
pub const PLAYER_RET_SOFTFAIL: C2RustUnnamed_16 = 2;
pub const PLAYER_RET_HARDFAIL: C2RustUnnamed_16 = 1;
pub const PLAYER_RET_OK: C2RustUnnamed_16 = 0;
pub type BarReadlineFlags_t = libc::c_uint;
pub const BAR_RL_NOINT: BarReadlineFlags_t = 4;
pub const BAR_RL_NOECHO: BarReadlineFlags_t = 2;
pub const BAR_RL_FULLRETURN: BarReadlineFlags_t = 1;
pub const BAR_RL_DEFAULT: BarReadlineFlags_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BarReadlineFds_t {
    pub set: fd_set,
    pub maxfd: libc::c_int,
    pub fds: [libc::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BarApp_t {
    pub ph: PianoHandle_t,
    pub http: *mut libc::c_void,
    pub player: player_t,
    pub settings: BarSettings_t,
    pub playlist: *mut PianoSong_t,
    pub songHistory: *mut PianoSong_t,
    pub curStation: *mut PianoStation_t,
    pub nextStation: *mut PianoStation_t,
    pub doQuit: sig_atomic_t,
    pub input: BarReadlineFds_t,
    pub playerErrors: libc::c_uint,
}
pub type BarUiSelectStationCallback_t = Option::<
    unsafe extern "C" fn(*mut BarApp_t, *mut libc::c_char) -> (),
>;
pub type BarUiDispatchContext_t = libc::c_uint;
pub const BAR_DC_SONG: BarUiDispatchContext_t = 4;
pub const BAR_DC_STATION: BarUiDispatchContext_t = 2;
pub const BAR_DC_GLOBAL: BarUiDispatchContext_t = 1;
pub const BAR_DC_UNDEFINED: BarUiDispatchContext_t = 0;
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn debugEnable() -> bool {
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn BarMainLoginUser(mut app: *mut BarApp_t) -> bool {
    let mut pRet: PianoReturn_t = PIANO_RET_ERR;
    let mut wRet: CURLcode = CURLE_OK;
    let mut reqData: PianoRequestDataLogin_t = PianoRequestDataLogin_t {
        user: 0 as *mut libc::c_char,
        password: 0 as *mut libc::c_char,
        step: 0,
    };
    let mut ret: bool = false;
    reqData.user = (*app).settings.username;
    reqData.password = (*app).settings.password;
    reqData.step = 0 as libc::c_int as libc::c_uchar;
    BarUiMsg(
        &mut (*app).settings as *mut BarSettings_t,
        MSG_INFO,
        b"Login... \0" as *const u8 as *const libc::c_char,
    );
    ret = BarUiPianoCall(
        app,
        PIANO_REQUEST_LOGIN,
        &mut reqData as *mut PianoRequestDataLogin_t as *mut libc::c_void,
        &mut pRet,
        &mut wRet,
    );
    BarUiStartEventCmd(
        &mut (*app).settings,
        b"userlogin\0" as *const u8 as *const libc::c_char,
        0 as *const PianoStation_t,
        0 as *const PianoSong_t,
        &mut (*app).player,
        0 as *mut PianoStation_t,
        pRet,
        wRet,
    );
    return ret;
}
unsafe extern "C" fn BarMainGetLoginCredentials(
    mut settings: *mut BarSettings_t,
    mut input: *mut BarReadlineFds_t,
) -> bool {
    let mut usernameFromConfig: bool = 1 as libc::c_int != 0;
    if ((*settings).username).is_null() {
        let mut nameBuf: [libc::c_char; 100] = [0; 100];
        BarUiMsg(
            settings,
            MSG_QUESTION,
            b"Email: \0" as *const u8 as *const libc::c_char,
        );
        if BarReadlineStr(
            nameBuf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
            input,
            BAR_RL_DEFAULT,
        ) == 0 as libc::c_int as libc::c_ulong
        {
            return 0 as libc::c_int != 0;
        }
        (*settings).username = strdup(nameBuf.as_mut_ptr());
        usernameFromConfig = 0 as libc::c_int != 0;
    }
    if ((*settings).password).is_null() {
        let mut passBuf: [libc::c_char; 100] = [0; 100];
        if usernameFromConfig {
            BarUiMsg(
                settings,
                MSG_QUESTION,
                b"Email: %s\n\0" as *const u8 as *const libc::c_char,
                (*settings).username,
            );
        }
        if ((*settings).passwordCmd).is_null() {
            BarUiMsg(
                settings,
                MSG_QUESTION,
                b"Password: \0" as *const u8 as *const libc::c_char,
            );
            if BarReadlineStr(
                passBuf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
                input,
                BAR_RL_NOECHO,
            ) == 0 as libc::c_int as libc::c_ulong
            {
                puts(b"\0" as *const u8 as *const libc::c_char);
                return 0 as libc::c_int != 0;
            }
            puts(b"\0" as *const u8 as *const libc::c_char);
            (*settings).password = strdup(passBuf.as_mut_ptr());
        } else {
            let mut chld: pid_t = 0;
            let mut pipeFd: [libc::c_int; 2] = [0; 2];
            BarUiMsg(
                settings,
                MSG_INFO,
                b"Requesting password from external helper... \0" as *const u8
                    as *const libc::c_char,
            );
            if pipe(pipeFd.as_mut_ptr()) == -(1 as libc::c_int) {
                BarUiMsg(
                    settings,
                    MSG_NONE,
                    b"Error: %s\n\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                return 0 as libc::c_int != 0;
            }
            chld = fork();
            if chld == 0 as libc::c_int {
                close(pipeFd[0 as libc::c_int as usize]);
                dup2(pipeFd[1 as libc::c_int as usize], fileno(stdout));
                execl(
                    b"/bin/sh\0" as *const u8 as *const libc::c_char,
                    b"/bin/sh\0" as *const u8 as *const libc::c_char,
                    b"-c\0" as *const u8 as *const libc::c_char,
                    (*settings).passwordCmd,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                BarUiMsg(
                    settings,
                    MSG_NONE,
                    b"Error: %s\n\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                close(pipeFd[1 as libc::c_int as usize]);
                exit(1 as libc::c_int);
            } else if chld == -(1 as libc::c_int) {
                BarUiMsg(
                    settings,
                    MSG_NONE,
                    b"Error: %s\n\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                return 0 as libc::c_int != 0;
            } else {
                let mut status: libc::c_int = 0;
                close(pipeFd[1 as libc::c_int as usize]);
                memset(
                    passBuf.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
                );
                read(
                    pipeFd[0 as libc::c_int as usize],
                    passBuf.as_mut_ptr() as *mut libc::c_void,
                    (::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                close(pipeFd[0 as libc::c_int as usize]);
                let mut len: ssize_t = (strlen(passBuf.as_mut_ptr()))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as ssize_t;
                while len >= 0 as libc::c_int as libc::c_long
                    && passBuf[len as usize] as libc::c_int == '\n' as i32
                {
                    passBuf[len as usize] = '\0' as i32 as libc::c_char;
                    len -= 1;
                    len;
                }
                waitpid(chld, &mut status, 0 as libc::c_int);
                if (status & 0xff00 as libc::c_int) >> 8 as libc::c_int
                    == 0 as libc::c_int
                {
                    (*settings).password = strdup(passBuf.as_mut_ptr());
                    BarUiMsg(
                        settings,
                        MSG_NONE,
                        b"Ok.\n\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    BarUiMsg(
                        settings,
                        MSG_NONE,
                        b"Error: Exit status %i.\n\0" as *const u8
                            as *const libc::c_char,
                        (status & 0xff00 as libc::c_int) >> 8 as libc::c_int,
                    );
                    return 0 as libc::c_int != 0;
                }
            }
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn BarMainGetStations(mut app: *mut BarApp_t) -> bool {
    let mut pRet: PianoReturn_t = PIANO_RET_ERR;
    let mut wRet: CURLcode = CURLE_OK;
    let mut ret: bool = false;
    BarUiMsg(
        &mut (*app).settings as *mut BarSettings_t,
        MSG_INFO,
        b"Get stations... \0" as *const u8 as *const libc::c_char,
    );
    ret = BarUiPianoCall(
        app,
        PIANO_REQUEST_GET_STATIONS,
        0 as *mut libc::c_void,
        &mut pRet,
        &mut wRet,
    );
    BarUiStartEventCmd(
        &mut (*app).settings,
        b"usergetstations\0" as *const u8 as *const libc::c_char,
        0 as *const PianoStation_t,
        0 as *const PianoSong_t,
        &mut (*app).player,
        (*app).ph.stations,
        pRet,
        wRet,
    );
    return ret;
}
unsafe extern "C" fn BarMainGetInitialStation(mut app: *mut BarApp_t) {
    if !((*app).settings.autostartStation).is_null() {
        (*app)
            .nextStation = PianoFindStationById(
            (*app).ph.stations,
            (*app).settings.autostartStation,
        );
        if ((*app).nextStation).is_null() {
            BarUiMsg(
                &mut (*app).settings as *mut BarSettings_t,
                MSG_ERR,
                b"Error: Autostart station not found.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if ((*app).nextStation).is_null() {
        (*app)
            .nextStation = BarUiSelectStation(
            app,
            (*app).ph.stations,
            b"Select station: \0" as *const u8 as *const libc::c_char,
            None,
            (*app).settings.autoselect,
        );
    }
}
unsafe extern "C" fn BarMainHandleUserInput(mut app: *mut BarApp_t) {
    let mut buf: [libc::c_char; 2] = [0; 2];
    if BarReadline(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
        0 as *const libc::c_char,
        &mut (*app).input,
        (BAR_RL_FULLRETURN as libc::c_int | BAR_RL_NOECHO as libc::c_int
            | BAR_RL_NOINT as libc::c_int) as BarReadlineFlags_t,
        1 as libc::c_int,
    ) > 0 as libc::c_int as libc::c_ulong
    {
        BarUiDispatch(
            app,
            buf[0 as libc::c_int as usize],
            (*app).curStation,
            (*app).playlist,
            1 as libc::c_int != 0,
            BAR_DC_GLOBAL,
        );
    }
}
unsafe extern "C" fn BarMainGetPlaylist(mut app: *mut BarApp_t) {
    let mut pRet: PianoReturn_t = PIANO_RET_ERR;
    let mut wRet: CURLcode = CURLE_OK;
    let mut reqData: PianoRequestDataGetPlaylist_t = PianoRequestDataGetPlaylist_t {
        station: 0 as *mut PianoStation_t,
        quality: PIANO_AQ_UNKNOWN,
        retPlaylist: 0 as *mut PianoSong_t,
    };
    reqData.station = (*app).nextStation;
    reqData.quality = (*app).settings.audioQuality;
    BarUiMsg(
        &mut (*app).settings as *mut BarSettings_t,
        MSG_INFO,
        b"Receiving new playlist... \0" as *const u8 as *const libc::c_char,
    );
    if !BarUiPianoCall(
        app,
        PIANO_REQUEST_GET_PLAYLIST,
        &mut reqData as *mut PianoRequestDataGetPlaylist_t as *mut libc::c_void,
        &mut pRet,
        &mut wRet,
    ) {
        (*app).nextStation = 0 as *mut PianoStation_t;
    } else {
        (*app).playlist = reqData.retPlaylist;
        if ((*app).playlist).is_null() {
            BarUiMsg(
                &mut (*app).settings as *mut BarSettings_t,
                MSG_INFO,
                b"No tracks left.\n\0" as *const u8 as *const libc::c_char,
            );
            (*app).nextStation = 0 as *mut PianoStation_t;
        }
    }
    (*app).curStation = (*app).nextStation;
    BarUiStartEventCmd(
        &mut (*app).settings,
        b"stationfetchplaylist\0" as *const u8 as *const libc::c_char,
        (*app).curStation,
        (*app).playlist,
        &mut (*app).player,
        (*app).ph.stations,
        pRet,
        wRet,
    );
}
unsafe extern "C" fn BarMainStartPlayback(
    mut app: *mut BarApp_t,
    mut playerThread: *mut pthread_t,
) {
    let curSong: *const PianoSong_t = (*app).playlist;
    BarUiPrintSong(
        &mut (*app).settings,
        curSong,
        if (*(*app).curStation).isQuickMix as libc::c_int != 0 {
            PianoFindStationById((*app).ph.stations, (*curSong).stationId)
        } else {
            0 as *mut PianoStation_t
        },
    );
    static mut httpPrefix: [libc::c_char; 8] = unsafe {
        *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"http://\0")
    };
    if ((*curSong).audioUrl).is_null()
        || strncmp((*curSong).audioUrl, httpPrefix.as_ptr(), strlen(httpPrefix.as_ptr()))
            != 0 as libc::c_int
    {
        BarUiMsg(
            &mut (*app).settings as *mut BarSettings_t,
            MSG_ERR,
            b"Invalid song url.\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        let player: *mut player_t = &mut (*app).player;
        BarPlayerReset(player);
        (*app).player.url = (*curSong).audioUrl;
        (*app).player.gain = (*curSong).fileGain as libc::c_double;
        (*app).player.songDuration = (*curSong).length;
        interrupted = &mut (*app).player.interrupted;
        BarUiStartEventCmd(
            &mut (*app).settings,
            b"songstart\0" as *const u8 as *const libc::c_char,
            (*app).curStation,
            curSong,
            &mut (*app).player,
            (*app).ph.stations,
            PIANO_RET_OK,
            CURLE_OK,
        );
        (*app).player.mode = PLAYER_WAITING;
        pthread_create(
            playerThread,
            0 as *const pthread_attr_t,
            Some(
                BarPlayerThread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            &mut (*app).player as *mut player_t as *mut libc::c_void,
        );
    };
}
unsafe extern "C" fn BarMainPlayerCleanup(
    mut app: *mut BarApp_t,
    mut playerThread: *mut pthread_t,
) {
    let mut threadRet: *mut libc::c_void = 0 as *mut libc::c_void;
    BarUiStartEventCmd(
        &mut (*app).settings,
        b"songfinish\0" as *const u8 as *const libc::c_char,
        (*app).curStation,
        (*app).playlist,
        &mut (*app).player,
        (*app).ph.stations,
        PIANO_RET_OK,
        CURLE_OK,
    );
    pthread_join(*playerThread, &mut threadRet);
    if threadRet.is_null() {
        (*app).playerErrors = 0 as libc::c_int as libc::c_uint;
    } else if threadRet == PLAYER_RET_SOFTFAIL as libc::c_int as *mut libc::c_void {
        (*app).playerErrors = ((*app).playerErrors).wrapping_add(1);
        (*app).playerErrors;
        if (*app).playerErrors >= (*app).settings.maxRetry {
            (*app).nextStation = 0 as *mut PianoStation_t;
        }
    } else {
        (*app).nextStation = 0 as *mut PianoStation_t;
    }
    interrupted = &mut (*app).doQuit;
    (*app).player.mode = PLAYER_DEAD;
}
unsafe extern "C" fn BarMainPrintTime(mut app: *mut BarApp_t) {
    let mut songRemaining: libc::c_uint = 0;
    let mut sign: [libc::c_char; 2] = [
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
    ];
    let player: *mut player_t = &mut (*app).player;
    pthread_mutex_lock(&mut (*player).lock);
    let songDuration: libc::c_uint = (*player).songDuration;
    let songPlayed: libc::c_uint = (*player).songPlayed;
    pthread_mutex_unlock(&mut (*player).lock);
    if songPlayed <= songDuration {
        songRemaining = songDuration.wrapping_sub(songPlayed);
        sign[0 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    } else {
        songRemaining = songPlayed.wrapping_sub(songDuration);
        sign[0 as libc::c_int as usize] = '+' as i32 as libc::c_char;
    }
    let mut outstr: [libc::c_char; 512] = [0; 512];
    let mut totalFormatted: [libc::c_char; 16] = [0; 16];
    let mut remainingFormatted: [libc::c_char; 16] = [0; 16];
    let mut elapsedFormatted: [libc::c_char; 16] = [0; 16];
    let mut vals: [*const libc::c_char; 4] = [
        totalFormatted.as_mut_ptr() as *const libc::c_char,
        remainingFormatted.as_mut_ptr() as *const libc::c_char,
        elapsedFormatted.as_mut_ptr() as *const libc::c_char,
        sign.as_mut_ptr() as *const libc::c_char,
    ];
    snprintf(
        totalFormatted.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        b"%02u:%02u\0" as *const u8 as *const libc::c_char,
        songDuration.wrapping_div(60 as libc::c_int as libc::c_uint),
        songDuration.wrapping_rem(60 as libc::c_int as libc::c_uint),
    );
    snprintf(
        remainingFormatted.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        b"%02u:%02u\0" as *const u8 as *const libc::c_char,
        songRemaining.wrapping_div(60 as libc::c_int as libc::c_uint),
        songRemaining.wrapping_rem(60 as libc::c_int as libc::c_uint),
    );
    snprintf(
        elapsedFormatted.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        b"%02u:%02u\0" as *const u8 as *const libc::c_char,
        songPlayed.wrapping_div(60 as libc::c_int as libc::c_uint),
        songPlayed.wrapping_rem(60 as libc::c_int as libc::c_uint),
    );
    BarUiCustomFormat(
        outstr.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
        (*app).settings.timeFormat,
        b"tres\0" as *const u8 as *const libc::c_char,
        vals.as_mut_ptr(),
    );
    BarUiMsg(
        &mut (*app).settings as *mut BarSettings_t,
        MSG_TIME,
        b"%s\r\0" as *const u8 as *const libc::c_char,
        outstr.as_mut_ptr(),
    );
}
unsafe extern "C" fn BarMainLoop(mut app: *mut BarApp_t) {
    let mut playerThread: pthread_t = 0;
    if !BarMainGetLoginCredentials(&mut (*app).settings, &mut (*app).input) {
        return;
    }
    if !BarMainLoginUser(app) {
        return;
    }
    if !BarMainGetStations(app) {
        return;
    }
    BarMainGetInitialStation(app);
    let player: *mut player_t = &mut (*app).player;
    while (*app).doQuit == 0 {
        if BarPlayerGetMode(player) as libc::c_uint
            == PLAYER_FINISHED as libc::c_int as libc::c_uint
        {
            if (*player).interrupted != 0 as libc::c_int {
                (*app).doQuit = 1 as libc::c_int;
            }
            BarMainPlayerCleanup(app, &mut playerThread);
        }
        if BarPlayerGetMode(player) as libc::c_uint
            == PLAYER_DEAD as libc::c_int as libc::c_uint
        {
            if !((*app).playlist).is_null() {
                let mut histsong: *mut PianoSong_t = (*app).playlist;
                (*app)
                    .playlist = (if ((*app).playlist).is_null() {
                    0 as *mut libc::c_void
                } else {
                    (*(*app).playlist).head.next as *mut libc::c_void
                }) as *mut PianoSong_t;
                (*histsong).head.next = 0 as *mut PianoListHead;
                BarUiHistoryPrepend(app, histsong);
            }
            if ((*app).playlist).is_null() && !((*app).nextStation).is_null()
                && (*app).doQuit == 0
            {
                if (*app).nextStation != (*app).curStation {
                    BarUiPrintStation(&mut (*app).settings, (*app).nextStation);
                }
                BarMainGetPlaylist(app);
            }
            if !((*app).playlist).is_null() {
                BarMainStartPlayback(app, &mut playerThread);
            }
        }
        BarMainHandleUserInput(app);
        if BarPlayerGetMode(player) as libc::c_uint
            == PLAYER_PLAYING as libc::c_int as libc::c_uint
        {
            BarMainPrintTime(app);
        }
    }
    if BarPlayerGetMode(player) as libc::c_uint
        != PLAYER_DEAD as libc::c_int as libc::c_uint
    {
        pthread_join(playerThread, 0 as *mut *mut libc::c_void);
    }
}
pub static mut interrupted: *mut sig_atomic_t = 0 as *const sig_atomic_t
    as *mut sig_atomic_t;
unsafe extern "C" fn intHandler(mut signal_0: libc::c_int) {
    if !interrupted.is_null() {
        *interrupted += 1 as libc::c_int;
    }
}
unsafe extern "C" fn BarMainSetupSigaction() {
    let mut act: sigaction = {
        let mut init = sigaction {
            __sigaction_handler: C2RustUnnamed_13 {
                sa_handler: Some(intHandler as unsafe extern "C" fn(libc::c_int) -> ()),
            },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0 as libc::c_int,
            sa_restorer: None,
        };
        init
    };
    sigemptyset(&mut act.sa_mask);
    sigaction(2 as libc::c_int, &mut act, 0 as *mut sigaction);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    static mut app: BarApp_t = BarApp_t {
        ph: PianoHandle_t {
            user: PianoUserInfo_t {
                listenerId: 0 as *const libc::c_char as *mut libc::c_char,
                authToken: 0 as *const libc::c_char as *mut libc::c_char,
            },
            stations: 0 as *const PianoStation_t as *mut PianoStation_t,
            genreStations: 0 as *const PianoGenreCategory_t as *mut PianoGenreCategory_t,
            partner: PianoPartner_t {
                in_0: 0 as *const gcry_cipher_handle as *mut gcry_cipher_handle,
                out: 0 as *const gcry_cipher_handle as *mut gcry_cipher_handle,
                authToken: 0 as *const libc::c_char as *mut libc::c_char,
                device: 0 as *const libc::c_char as *mut libc::c_char,
                user: 0 as *const libc::c_char as *mut libc::c_char,
                password: 0 as *const libc::c_char as *mut libc::c_char,
                id: 0,
            },
            timeOffset: 0,
        },
        http: 0 as *const libc::c_void as *mut libc::c_void,
        player: player_t {
            lock: pthread_mutex_t {
                __data: __pthread_mutex_s {
                    __lock: 0,
                    __count: 0,
                    __owner: 0,
                    __nusers: 0,
                    __kind: 0,
                    __spins: 0,
                    __elision: 0,
                    __list: __pthread_list_t {
                        __prev: 0 as *const __pthread_internal_list
                            as *mut __pthread_internal_list,
                        __next: 0 as *const __pthread_internal_list
                            as *mut __pthread_internal_list,
                    },
                },
            },
            aoplayLock: pthread_mutex_t {
                __data: __pthread_mutex_s {
                    __lock: 0,
                    __count: 0,
                    __owner: 0,
                    __nusers: 0,
                    __kind: 0,
                    __spins: 0,
                    __elision: 0,
                    __list: __pthread_list_t {
                        __prev: 0 as *const __pthread_internal_list
                            as *mut __pthread_internal_list,
                        __next: 0 as *const __pthread_internal_list
                            as *mut __pthread_internal_list,
                    },
                },
            },
            cond: pthread_cond_t {
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
            aoplayCond: pthread_cond_t {
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
            doQuit: false,
            doPause: false,
            songDuration: 0,
            songPlayed: 0,
            mode: PLAYER_DEAD,
            fvolume: 0 as *const AVFilterContext as *mut AVFilterContext,
            fgraph: 0 as *const AVFilterGraph as *mut AVFilterGraph,
            fctx: 0 as *const AVFormatContext as *mut AVFormatContext,
            st: 0 as *const AVStream as *mut AVStream,
            cctx: 0 as *const AVCodecContext as *mut AVCodecContext,
            fbufsink: 0 as *const AVFilterContext as *mut AVFilterContext,
            fabuf: 0 as *const AVFilterContext as *mut AVFilterContext,
            streamIdx: 0,
            lastTimestamp: 0,
            interrupted: 0,
            aoDev: 0 as *const ao_device as *mut ao_device,
            gain: 0.,
            url: 0 as *const libc::c_char as *mut libc::c_char,
            settings: 0 as *const BarSettings_t,
        },
        settings: BarSettings_t {
            autoselect: false,
            history: 0,
            maxRetry: 0,
            timeout: 0,
            bufferSecs: 0,
            volume: 0,
            gainMul: 0.,
            sortOrder: BAR_SORT_NAME_AZ,
            audioQuality: PIANO_AQ_UNKNOWN,
            username: 0 as *const libc::c_char as *mut libc::c_char,
            password: 0 as *const libc::c_char as *mut libc::c_char,
            passwordCmd: 0 as *const libc::c_char as *mut libc::c_char,
            controlProxy: 0 as *const libc::c_char as *mut libc::c_char,
            proxy: 0 as *const libc::c_char as *mut libc::c_char,
            bindTo: 0 as *const libc::c_char as *mut libc::c_char,
            autostartStation: 0 as *const libc::c_char as *mut libc::c_char,
            eventCmd: 0 as *const libc::c_char as *mut libc::c_char,
            loveIcon: 0 as *const libc::c_char as *mut libc::c_char,
            banIcon: 0 as *const libc::c_char as *mut libc::c_char,
            tiredIcon: 0 as *const libc::c_char as *mut libc::c_char,
            atIcon: 0 as *const libc::c_char as *mut libc::c_char,
            npSongFormat: 0 as *const libc::c_char as *mut libc::c_char,
            npStationFormat: 0 as *const libc::c_char as *mut libc::c_char,
            listSongFormat: 0 as *const libc::c_char as *mut libc::c_char,
            timeFormat: 0 as *const libc::c_char as *mut libc::c_char,
            fifo: 0 as *const libc::c_char as *mut libc::c_char,
            rpcHost: 0 as *const libc::c_char as *mut libc::c_char,
            rpcTlsPort: 0 as *const libc::c_char as *mut libc::c_char,
            partnerUser: 0 as *const libc::c_char as *mut libc::c_char,
            partnerPassword: 0 as *const libc::c_char as *mut libc::c_char,
            device: 0 as *const libc::c_char as *mut libc::c_char,
            inkey: 0 as *const libc::c_char as *mut libc::c_char,
            outkey: 0 as *const libc::c_char as *mut libc::c_char,
            caBundle: 0 as *const libc::c_char as *mut libc::c_char,
            audioPipe: 0 as *const libc::c_char as *mut libc::c_char,
            keys: [0; 30],
            sampleRate: 0,
            msgFormat: [BarMsgFormatStr_t {
                prefix: 0 as *const libc::c_char as *mut libc::c_char,
                postfix: 0 as *const libc::c_char as *mut libc::c_char,
            }; 7],
        },
        playlist: 0 as *const PianoSong_t as *mut PianoSong_t,
        songHistory: 0 as *const PianoSong_t as *mut PianoSong_t,
        curStation: 0 as *const PianoStation_t as *mut PianoStation_t,
        nextStation: 0 as *const PianoStation_t as *mut PianoStation_t,
        doQuit: 0,
        input: BarReadlineFds_t {
            set: fd_set { __fds_bits: [0; 16] },
            maxfd: 0,
            fds: [0; 2],
        },
        playerErrors: 0,
    };
    debugEnable();
    memset(
        &mut app as *mut BarApp_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<BarApp_t>() as libc::c_ulong,
    );
    BarTermInit();
    signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    BarMainSetupSigaction();
    interrupted = &mut app.doQuit;
    gcry_check_version(0 as *const libc::c_char);
    gcry_control(GCRYCTL_DISABLE_SECMEM, 0 as libc::c_int);
    gcry_control(GCRYCTL_INITIALIZATION_FINISHED, 0 as libc::c_int);
    BarPlayerInit(&mut app.player, &mut app.settings);
    BarSettingsInit(&mut app.settings);
    BarSettingsRead(&mut app.settings);
    let mut pret: PianoReturn_t = PIANO_RET_ERR;
    pret = PianoInit(
        &mut app.ph,
        app.settings.partnerUser,
        app.settings.partnerPassword,
        app.settings.device,
        app.settings.inkey,
        app.settings.outkey,
    );
    if pret as libc::c_uint != PIANO_RET_OK as libc::c_int as libc::c_uint {
        BarUiMsg(
            &mut app.settings as *mut BarSettings_t,
            MSG_ERR,
            b"Initialization failed: %s\n\0" as *const u8 as *const libc::c_char,
            PianoErrorToStr(pret),
        );
        return 0 as libc::c_int;
    }
    BarUiMsg(
        &mut app.settings as *mut BarSettings_t,
        MSG_NONE,
        b"Welcome to pianobar (2022.04.01-dev)! \0" as *const u8 as *const libc::c_char,
    );
    if app.settings.keys[BAR_KS_HELP as libc::c_int as usize] as libc::c_int
        == '\0' as i32
    {
        BarUiMsg(
            &mut app.settings as *mut BarSettings_t,
            MSG_NONE,
            b"\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        BarUiMsg(
            &mut app.settings as *mut BarSettings_t,
            MSG_NONE,
            b"Press %c for a list of commands.\n\0" as *const u8 as *const libc::c_char,
            app.settings.keys[BAR_KS_HELP as libc::c_int as usize] as libc::c_int,
        );
    }
    curl_global_init(
        ((1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int)
            as libc::c_long,
    );
    app.http = curl_easy_init();
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh0 = &mut __d0;
    let fresh1;
    let fresh2 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh3 = &mut __d1;
    let fresh4;
    let fresh5 = &mut *(app.input.set.__fds_bits)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh0,
        fresh2) => fresh1, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh3,
        fresh5) => fresh4, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
    c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
    app.input.fds[0 as libc::c_int as usize] = 0 as libc::c_int;
    app
        .input
        .set
        .__fds_bits[(app.input.fds[0 as libc::c_int as usize]
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << app.input.fds[0 as libc::c_int as usize]
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    app
        .input
        .fds[1 as libc::c_int as usize] = open(app.settings.fifo, 0o2 as libc::c_int);
    if app.input.fds[1 as libc::c_int as usize] != -(1 as libc::c_int) {
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
        fstat(app.input.fds[1 as libc::c_int as usize], &mut s);
        if !(s.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o10000 as libc::c_int as libc::c_uint)
        {
            BarUiMsg(
                &mut app.settings as *mut BarSettings_t,
                MSG_ERR,
                b"File at %s is not a fifo\n\0" as *const u8 as *const libc::c_char,
                app.settings.fifo,
            );
            close(app.input.fds[1 as libc::c_int as usize]);
            app.input.fds[1 as libc::c_int as usize] = -(1 as libc::c_int);
        } else {
            app
                .input
                .set
                .__fds_bits[(app.input.fds[1 as libc::c_int as usize]
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                |= ((1 as libc::c_ulong)
                    << app.input.fds[1 as libc::c_int as usize]
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask;
            BarUiMsg(
                &mut app.settings as *mut BarSettings_t,
                MSG_INFO,
                b"Control fifo at %s opened\n\0" as *const u8 as *const libc::c_char,
                app.settings.fifo,
            );
        }
    }
    app
        .input
        .maxfd = if app.input.fds[0 as libc::c_int as usize]
        > app.input.fds[1 as libc::c_int as usize]
    {
        app.input.fds[0 as libc::c_int as usize]
    } else {
        app.input.fds[1 as libc::c_int as usize]
    };
    app.input.maxfd += 1;
    app.input.maxfd;
    BarMainLoop(&mut app);
    if app.input.fds[1 as libc::c_int as usize] != -(1 as libc::c_int) {
        close(app.input.fds[1 as libc::c_int as usize]);
    }
    BarSettingsWrite(app.curStation, &mut app.settings);
    PianoDestroy(&mut app.ph);
    PianoDestroyPlaylist(app.songHistory);
    PianoDestroyPlaylist(app.playlist);
    curl_easy_cleanup(app.http);
    curl_global_cleanup();
    BarPlayerDestroy(&mut app.player);
    BarSettingsDestroy(&mut app.settings);
    BarTermRestore();
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
