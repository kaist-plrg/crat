use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __dirstream;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    pub type screen;
    pub type ldat;
    fn inotify_init1(__flags: libc::c_int) -> libc::c_int;
    fn inotify_add_watch(
        __fd: libc::c_int,
        __name: *const libc::c_char,
        __mask: uint32_t,
    ) -> libc::c_int;
    fn inotify_rm_watch(__fd: libc::c_int, __wd: libc::c_int) -> libc::c_int;
    fn setrlimit(
        __resource: __rlimit_resource_t,
        __rlimits: *const rlimit,
    ) -> libc::c_int;
    fn getrlimit(__resource: __rlimit_resource_t, __rlimits: *mut rlimit) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn mkfifo(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __fxstatat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn statvfs(__file: *const libc::c_char, __buf: *mut statvfs) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn dirfd(__dirp: *mut DIR) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn posix_fadvise(
        __fd: libc::c_int,
        __offset: off_t,
        __len: off_t,
        __advise: libc::c_int,
    ) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn fts_read(_: *mut FTS) -> *mut FTSENT;
    fn fts_open(
        _: *const *mut libc::c_char,
        _: libc::c_int,
        _: Option::<
            unsafe extern "C" fn(*mut *const FTSENT, *mut *const FTSENT) -> libc::c_int,
        >,
    ) -> *mut FTS;
    fn fts_close(_: *mut FTS) -> libc::c_int;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn renameat(
        __oldfd: libc::c_int,
        __old: *const libc::c_char,
        __newfd: libc::c_int,
        __new: *const libc::c_char,
    ) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn dprintf(__fd: libc::c_int, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn ctermid(__s: *mut libc::c_char) -> *mut libc::c_char;
    fn perror(__s: *const libc::c_char);
    fn add_history(_: *const libc::c_char);
    fn read_history(_: *const libc::c_char) -> libc::c_int;
    fn write_history(_: *const libc::c_char) -> libc::c_int;
    fn rl_menu_complete(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn readline(_: *const libc::c_char) -> *mut libc::c_char;
    fn rl_bind_key(_: libc::c_int, _: Option::<rl_command_func_t>) -> libc::c_int;
    fn rl_variable_bind(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut rl_change_environment: libc::c_int;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn ffs(__i: libc::c_int) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strnlen(__string: *const libc::c_char, __maxlen: size_t) -> size_t;
    fn memmem(
        __haystack: *const libc::c_void,
        __haystacklen: size_t,
        __needle: *const libc::c_void,
        __needlelen: size_t,
    ) -> *mut libc::c_void;
    fn strcasestr(
        __haystack: *const libc::c_char,
        __needle: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcoll(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn memrchr(
        __s: *const libc::c_void,
        __c: libc::c_int,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn rawmemchr(__s: *const libc::c_void, __c: libc::c_int) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memccpy(
        __dest: *mut libc::c_void,
        __src: *const libc::c_void,
        __c: libc::c_int,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn regcomp(
        __preg: *mut regex_t,
        __pattern: *const libc::c_char,
        __cflags: libc::c_int,
    ) -> libc::c_int;
    fn regexec(
        __preg: *const regex_t,
        __String: *const libc::c_char,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: libc::c_int,
    ) -> libc::c_int;
    fn regfree(__preg: *mut regex_t);
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    fn mkdtemp(__template: *mut libc::c_char) -> *mut libc::c_char;
    fn realpath(
        __name: *const libc::c_char,
        __resolved: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn mbstowcs(__pwcs: *mut wchar_t, __s: *const libc::c_char, __n: size_t) -> size_t;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn wcstombs(__s: *mut libc::c_char, __pwcs: *const wchar_t, __n: size_t) -> size_t;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn _exit(__status: libc::c_int) -> !;
    fn getpid() -> __pid_t;
    fn getppid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn fork() -> __pid_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn link(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
    fn symlink(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
    fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn rmdir(__path: *const libc::c_char) -> libc::c_int;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    static mut optind: libc::c_int;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getgrgid(__gid: __gid_t) -> *mut group;
    fn cbreak() -> libc::c_int;
    fn curs_set(_: libc::c_int) -> libc::c_int;
    fn endwin() -> libc::c_int;
    fn initscr() -> *mut WINDOW;
    fn init_pair(_: libc::c_short, _: libc::c_short, _: libc::c_short) -> libc::c_int;
    fn keyname(_: libc::c_int) -> *const libc::c_char;
    fn keypad(_: *mut WINDOW, _: bool) -> libc::c_int;
    fn newterm(_: *const libc::c_char, _: *mut FILE, _: *mut FILE) -> *mut SCREEN;
    fn noecho() -> libc::c_int;
    fn nonl() -> libc::c_int;
    fn printw(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn start_color() -> libc::c_int;
    fn waddch(_: *mut WINDOW, _: chtype) -> libc::c_int;
    fn waddnstr(_: *mut WINDOW, _: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn wattr_on(_: *mut WINDOW, _: attr_t, _: *mut libc::c_void) -> libc::c_int;
    fn wattr_off(_: *mut WINDOW, _: attr_t, _: *mut libc::c_void) -> libc::c_int;
    fn wclrtoeol(_: *mut WINDOW) -> libc::c_int;
    fn werase(_: *mut WINDOW) -> libc::c_int;
    fn wmove(_: *mut WINDOW, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn wrefresh(_: *mut WINDOW) -> libc::c_int;
    fn wtimeout(_: *mut WINDOW, _: libc::c_int);
    fn set_escdelay(_: libc::c_int) -> libc::c_int;
    fn use_default_colors() -> libc::c_int;
    static mut stdscr: *mut WINDOW;
    static mut COLORS: libc::c_int;
    static mut COLS: libc::c_int;
    static mut LINES: libc::c_int;
    fn unget_wch(_: wchar_t) -> libc::c_int;
    fn waddnwstr(_: *mut WINDOW, _: *const wchar_t, _: libc::c_int) -> libc::c_int;
    fn wget_wch(_: *mut WINDOW, _: *mut wint_t) -> libc::c_int;
    fn getmouse(_: *mut MEVENT) -> libc::c_int;
    fn mousemask(_: mmask_t, _: *mut mmask_t) -> mmask_t;
    fn mouseinterval(_: libc::c_int) -> libc::c_int;
    fn wcswidth(__s: *const wchar_t, __n: size_t) -> libc::c_int;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
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
pub type __clock_t = libc::c_long;
pub type __rlim_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __clockid_t = libc::c_int;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __fsblkcnt_t = libc::c_ulong;
pub type __fsfilcnt_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type int8_t = __int8_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const IN_NONBLOCK: C2RustUnnamed = 2048;
pub const IN_CLOEXEC: C2RustUnnamed = 524288;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inotify_event {
    pub wd: libc::c_int,
    pub mask: uint32_t,
    pub cookie: uint32_t,
    pub len: uint32_t,
    pub name: [libc::c_char; 0],
}
pub type __rlimit_resource = libc::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type rlim_t = __rlim_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = __rlimit_resource;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type time_t = __time_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type ino_t = __ino_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type off_t = __off_t;
pub type uid_t = __uid_t;
pub type blkcnt_t = __blkcnt_t;
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
pub struct statvfs {
    pub f_bsize: libc::c_ulong,
    pub f_frsize: libc::c_ulong,
    pub f_blocks: __fsblkcnt_t,
    pub f_bfree: __fsblkcnt_t,
    pub f_bavail: __fsblkcnt_t,
    pub f_files: __fsfilcnt_t,
    pub f_ffree: __fsfilcnt_t,
    pub f_favail: __fsfilcnt_t,
    pub f_fsid: libc::c_ulong,
    pub f_flag: libc::c_ulong,
    pub f_namemax: libc::c_ulong,
    pub __f_spare: [libc::c_int; 6],
}
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
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
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_9,
    pub _timer: C2RustUnnamed_8,
    pub _rt: C2RustUnnamed_7,
    pub _sigchld: C2RustUnnamed_6,
    pub _sigfault: C2RustUnnamed_3,
    pub _sigpoll: C2RustUnnamed_2,
    pub _sigsys: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub _addr_bnd: C2RustUnnamed_5,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_10,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
pub type size_t = libc::c_ulong;
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
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type C2RustUnnamed_11 = libc::c_uint;
pub const DT_WHT: C2RustUnnamed_11 = 14;
pub const DT_SOCK: C2RustUnnamed_11 = 12;
pub const DT_LNK: C2RustUnnamed_11 = 10;
pub const DT_REG: C2RustUnnamed_11 = 8;
pub const DT_BLK: C2RustUnnamed_11 = 6;
pub const DT_DIR: C2RustUnnamed_11 = 4;
pub const DT_CHR: C2RustUnnamed_11 = 2;
pub const DT_FIFO: C2RustUnnamed_11 = 1;
pub const DT_UNKNOWN: C2RustUnnamed_11 = 0;
pub type DIR = __dirstream;
pub type ssize_t = __ssize_t;
pub type clockid_t = __clockid_t;
pub type useconds_t = __useconds_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FTS {
    pub fts_cur: *mut _ftsent,
    pub fts_child: *mut _ftsent,
    pub fts_array: *mut *mut _ftsent,
    pub fts_dev: dev_t,
    pub fts_path: *mut libc::c_char,
    pub fts_rfd: libc::c_int,
    pub fts_pathlen: libc::c_int,
    pub fts_nitems: libc::c_int,
    pub fts_compar: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
    pub fts_options: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ftsent {
    pub fts_cycle: *mut _ftsent,
    pub fts_parent: *mut _ftsent,
    pub fts_link: *mut _ftsent,
    pub fts_number: libc::c_long,
    pub fts_pointer: *mut libc::c_void,
    pub fts_accpath: *mut libc::c_char,
    pub fts_path: *mut libc::c_char,
    pub fts_errno: libc::c_int,
    pub fts_symfd: libc::c_int,
    pub fts_pathlen: libc::c_ushort,
    pub fts_namelen: libc::c_ushort,
    pub fts_ino: ino_t,
    pub fts_dev: dev_t,
    pub fts_nlink: nlink_t,
    pub fts_level: libc::c_short,
    pub fts_info: libc::c_ushort,
    pub fts_flags: libc::c_ushort,
    pub fts_instr: libc::c_ushort,
    pub fts_statp: *mut stat,
    pub fts_name: [libc::c_char; 1],
}
pub type FTSENT = _ftsent;
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
pub type C2RustUnnamed_12 = libc::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed_12 = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_12 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_12 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_12 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_12 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_12 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_12 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_12 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_12 = 0;
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
pub type rl_command_func_t = unsafe extern "C" fn(
    libc::c_int,
    libc::c_int,
) -> libc::c_int;
pub type __re_long_size_t = libc::c_ulong;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
pub type wchar_t = libc::c_int;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
pub type chtype = libc::c_uint;
pub type mmask_t = libc::c_uint;
pub type SCREEN = screen;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _win_st {
    pub _cury: libc::c_short,
    pub _curx: libc::c_short,
    pub _maxy: libc::c_short,
    pub _maxx: libc::c_short,
    pub _begy: libc::c_short,
    pub _begx: libc::c_short,
    pub _flags: libc::c_short,
    pub _attrs: attr_t,
    pub _bkgd: chtype,
    pub _notimeout: bool,
    pub _clear: bool,
    pub _leaveok: bool,
    pub _scroll: bool,
    pub _idlok: bool,
    pub _idcok: bool,
    pub _immed: bool,
    pub _sync: bool,
    pub _use_keypad: bool,
    pub _delay: libc::c_int,
    pub _line: *mut ldat,
    pub _regtop: libc::c_short,
    pub _regbottom: libc::c_short,
    pub _parx: libc::c_int,
    pub _pary: libc::c_int,
    pub _parent: *mut WINDOW,
    pub _pad: pdat,
    pub _yoffset: libc::c_short,
    pub _bkgrnd: cchar_t,
    pub _color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cchar_t {
    pub attr: attr_t,
    pub chars: [wchar_t; 5],
    pub ext_color: libc::c_int,
}
pub type attr_t = chtype;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdat {
    pub _pad_y: libc::c_short,
    pub _pad_x: libc::c_short,
    pub _pad_top: libc::c_short,
    pub _pad_left: libc::c_short,
    pub _pad_bottom: libc::c_short,
    pub _pad_right: libc::c_short,
}
pub type WINDOW = _win_st;
pub type wint_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MEVENT {
    pub id: libc::c_short,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub z: libc::c_int,
    pub bstate: mmask_t,
}
pub type action = libc::c_uint;
pub const SEL_CLICK: action = 66;
pub const SEL_QUITERR: action = 65;
pub const SEL_QUIT: action = 64;
pub const SEL_QUITCD: action = 63;
pub const SEL_QUITCTX: action = 62;
pub const SEL_TIMETYPE: action = 61;
pub const SEL_EXPORT: action = 60;
pub const SEL_SESSIONS: action = 59;
pub const SEL_LOCK: action = 58;
pub const SEL_PROMPT: action = 57;
pub const SEL_LAUNCH: action = 56;
pub const SEL_SHELL: action = 55;
pub const SEL_PLUGIN: action = 54;
pub const SEL_EDIT: action = 53;
pub const SEL_AUTONEXT: action = 52;
pub const SEL_HELP: action = 51;
pub const SEL_UMOUNT: action = 50;
pub const SEL_RENAMEMUL: action = 49;
pub const SEL_RENAME: action = 48;
pub const SEL_NEW: action = 47;
pub const SEL_OPENWITH: action = 46;
pub const SEL_RM: action = 45;
pub const SEL_CPMVAS: action = 44;
pub const SEL_MV: action = 43;
pub const SEL_CP: action = 42;
pub const SEL_SELEDIT: action = 41;
pub const SEL_SELINV: action = 40;
pub const SEL_SELALL: action = 39;
pub const SEL_SELMUL: action = 38;
pub const SEL_SEL: action = 37;
pub const SEL_REDRAW: action = 36;
pub const SEL_SORT: action = 35;
pub const SEL_ARCHIVE: action = 34;
pub const SEL_CHMODX: action = 33;
pub const SEL_STATS: action = 32;
pub const SEL_DETAIL: action = 31;
pub const SEL_HIDDEN: action = 30;
pub const SEL_MFLTR: action = 29;
pub const SEL_FLTR: action = 28;
pub const SEL_BMARK: action = 27;
pub const SEL_MARK: action = 26;
pub const SEL_CTX4: action = 25;
pub const SEL_CTX3: action = 24;
pub const SEL_CTX2: action = 23;
pub const SEL_CTX1: action = 22;
pub const SEL_CYCLER: action = 21;
pub const SEL_CYCLE: action = 20;
pub const SEL_REMOTE: action = 19;
pub const SEL_BMOPEN: action = 18;
pub const SEL_CDROOT: action = 17;
pub const SEL_CDLAST: action = 16;
pub const SEL_CDBEGIN: action = 15;
pub const SEL_CDHOME: action = 14;
pub const SEL_JUMP: action = 13;
pub const SEL_FIRST: action = 12;
pub const SEL_END: action = 11;
pub const SEL_HOME: action = 10;
pub const SEL_CTRL_U: action = 9;
pub const SEL_CTRL_D: action = 8;
pub const SEL_PGUP: action = 7;
pub const SEL_PGDN: action = 6;
pub const SEL_PREV: action = 5;
pub const SEL_NEXT: action = 4;
pub const SEL_NAV_IN: action = 3;
pub const SEL_OPEN: action = 2;
pub const SEL_BACK: action = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key {
    pub sym: wint_t,
    pub act: action,
}
pub type uint_t = libc::c_uint;
pub type uchar_t = libc::c_uchar;
pub type ushort_t = libc::c_ushort;
pub type ullong_t = libc::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct entry {
    pub name: *mut libc::c_char,
    pub sec: time_t,
    pub nsec: uint_t,
    pub mode: mode_t,
    pub size: off_t,
    pub c2rust_unnamed: C2RustUnnamed_13,
    pub uid: uid_t,
    pub gid: gid_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    #[bitfield(name = "blocks", ty = "ullong_t", bits = "0..=39")]
    #[bitfield(name = "nlen", ty = "ullong_t", bits = "40..=55")]
    #[bitfield(name = "flags", ty = "ullong_t", bits = "56..=63")]
    pub blocks_nlen_flags: [u8; 8],
}
pub type pEntry = *mut entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct selmark {
    pub startpos: *mut libc::c_char,
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kv {
    pub key: libc::c_int,
    pub off: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fltrexp_t {
    pub regex: *const regex_t,
    pub str_0: *const libc::c_char,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct settings {
    #[bitfield(name = "filtermode", ty = "uint_t", bits = "0..=0")]
    #[bitfield(name = "timeorder", ty = "uint_t", bits = "1..=1")]
    #[bitfield(name = "sizeorder", ty = "uint_t", bits = "2..=2")]
    #[bitfield(name = "apparentsz", ty = "uint_t", bits = "3..=3")]
    #[bitfield(name = "blkorder", ty = "uint_t", bits = "4..=4")]
    #[bitfield(name = "extnorder", ty = "uint_t", bits = "5..=5")]
    #[bitfield(name = "showhidden", ty = "uint_t", bits = "6..=6")]
    #[bitfield(name = "reserved0", ty = "uint_t", bits = "7..=7")]
    #[bitfield(name = "showdetail", ty = "uint_t", bits = "8..=8")]
    #[bitfield(name = "ctxactive", ty = "uint_t", bits = "9..=9")]
    #[bitfield(name = "reverse", ty = "uint_t", bits = "10..=10")]
    #[bitfield(name = "version", ty = "uint_t", bits = "11..=11")]
    #[bitfield(name = "reserved1", ty = "uint_t", bits = "12..=12")]
    #[bitfield(name = "curctx", ty = "uint_t", bits = "13..=15")]
    #[bitfield(name = "prefersel", ty = "uint_t", bits = "16..=16")]
    #[bitfield(name = "fileinfo", ty = "uint_t", bits = "17..=17")]
    #[bitfield(name = "nonavopen", ty = "uint_t", bits = "18..=18")]
    #[bitfield(name = "autoenter", ty = "uint_t", bits = "19..=19")]
    #[bitfield(name = "reserved2", ty = "uint_t", bits = "20..=20")]
    #[bitfield(name = "useeditor", ty = "uint_t", bits = "21..=21")]
    #[bitfield(name = "reserved3", ty = "uint_t", bits = "22..=24")]
    #[bitfield(name = "regex", ty = "uint_t", bits = "25..=25")]
    #[bitfield(name = "x11", ty = "uint_t", bits = "26..=26")]
    #[bitfield(name = "timetype", ty = "uint_t", bits = "27..=28")]
    #[bitfield(name = "cliopener", ty = "uint_t", bits = "29..=29")]
    #[bitfield(name = "waitedit", ty = "uint_t", bits = "30..=30")]
    #[bitfield(name = "rollover", ty = "uint_t", bits = "31..=31")]
    pub filtermode_timeorder_sizeorder_apparentsz_blkorder_extnorder_showhidden_reserved0_showdetail_ctxactive_reverse_version_reserved1_curctx_prefersel_fileinfo_nonavopen_autoenter_reserved2_useeditor_reserved3_regex_x11_timetype_cliopener_waitedit_rollover: [u8; 4],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct runstate {
    #[bitfield(name = "autofifo", ty = "uint_t", bits = "0..=0")]
    #[bitfield(name = "autonext", ty = "uint_t", bits = "1..=1")]
    #[bitfield(name = "dircolor", ty = "uint_t", bits = "2..=2")]
    #[bitfield(name = "dirctx", ty = "uint_t", bits = "3..=3")]
    #[bitfield(name = "duinit", ty = "uint_t", bits = "4..=4")]
    #[bitfield(name = "fifomode", ty = "uint_t", bits = "5..=5")]
    #[bitfield(name = "forcequit", ty = "uint_t", bits = "6..=6")]
    #[bitfield(name = "initfile", ty = "uint_t", bits = "7..=7")]
    #[bitfield(name = "interrupt", ty = "uint_t", bits = "8..=8")]
    #[bitfield(name = "move_0", ty = "uint_t", bits = "9..=9")]
    #[bitfield(name = "oldcolor", ty = "uint_t", bits = "10..=10")]
    #[bitfield(name = "picked", ty = "uint_t", bits = "11..=11")]
    #[bitfield(name = "picker", ty = "uint_t", bits = "12..=12")]
    #[bitfield(name = "pluginit", ty = "uint_t", bits = "13..=13")]
    #[bitfield(name = "prstssn", ty = "uint_t", bits = "14..=14")]
    #[bitfield(name = "rangesel", ty = "uint_t", bits = "15..=15")]
    #[bitfield(name = "runctx", ty = "uint_t", bits = "16..=18")]
    #[bitfield(name = "runplugin", ty = "uint_t", bits = "19..=19")]
    #[bitfield(name = "selbm", ty = "uint_t", bits = "20..=20")]
    #[bitfield(name = "selmode", ty = "uint_t", bits = "21..=21")]
    #[bitfield(name = "stayonsel", ty = "uint_t", bits = "22..=22")]
    #[bitfield(name = "trash", ty = "uint_t", bits = "23..=24")]
    #[bitfield(name = "uidgid", ty = "uint_t", bits = "25..=25")]
    #[bitfield(name = "usebsdtar", ty = "uint_t", bits = "26..=26")]
    #[bitfield(name = "reserved", ty = "uint_t", bits = "27..=31")]
    pub autofifo_autonext_dircolor_dirctx_duinit_fifomode_forcequit_initfile_interrupt_move_0_oldcolor_picked_picker_pluginit_prstssn_rangesel_runctx_runplugin_selbm_selmode_stayonsel_trash_uidgid_usebsdtar_reserved: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context {
    pub c_path: [libc::c_char; 4096],
    pub c_last: [libc::c_char; 4096],
    pub c_name: [libc::c_char; 256],
    pub c_fltr: [libc::c_char; 48],
    pub c_cfg: settings,
    pub color: uint_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct session_header_t {
    pub ver: size_t,
    pub pathln: [size_t; 4],
    pub lastln: [size_t; 4],
    pub nameln: [size_t; 4],
    pub fltrln: [size_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread_data {
    pub path: [libc::c_char; 4096],
    pub entnum: libc::c_int,
    pub core: ushort_t,
    pub mntpoint: bool,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn fstatat(
    mut __fd: libc::c_int,
    mut __filename: *const libc::c_char,
    mut __statbuf: *mut stat,
    mut __flag: libc::c_int,
) -> libc::c_int {
    return __fxstatat(1 as libc::c_int, __fd, __filename, __statbuf, __flag);
}
#[inline]
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut bindings: [key; 85] = [
    {
        let mut init = key {
            sym: 0o404 as libc::c_int as wint_t,
            act: SEL_BACK,
        };
        init
    },
    {
        let mut init = key {
            sym: 'h' as i32 as wint_t,
            act: SEL_BACK,
        };
        init
    },
    {
        let mut init = key {
            sym: 0o527 as libc::c_int as wint_t,
            act: SEL_OPEN,
        };
        init
    },
    {
        let mut init = key {
            sym: '\r' as i32 as wint_t,
            act: SEL_OPEN,
        };
        init
    },
    {
        let mut init = key {
            sym: 0o405 as libc::c_int as wint_t,
            act: SEL_NAV_IN,
        };
        init
    },
    {
        let mut init = key {
            sym: 'l' as i32 as wint_t,
            act: SEL_NAV_IN,
        };
        init
    },
    {
        let mut init = key {
            sym: 'j' as i32 as wint_t,
            act: SEL_NEXT,
        };
        init
    },
    {
        let mut init = key {
            sym: 0o402 as libc::c_int as wint_t,
            act: SEL_NEXT,
        };
        init
    },
    {
        let mut init = key {
            sym: 'k' as i32 as wint_t,
            act: SEL_PREV,
        };
        init
    },
    {
        let mut init = key {
            sym: 0o403 as libc::c_int as wint_t,
            act: SEL_PREV,
        };
        init
    },
    {
        let mut init = key {
            sym: 0o522 as libc::c_int as wint_t,
            act: SEL_PGDN,
        };
        init
    },
    {
        let mut init = key {
            sym: 0o523 as libc::c_int as wint_t,
            act: SEL_PGUP,
        };
        init
    },
    {
        let mut init = key {
            sym: ('D' as i32 & 0x1f as libc::c_int) as wint_t,
            act: SEL_CTRL_D,
        };
        init
    },
    {
        let mut init = key {
            sym: ('U' as i32 & 0x1f as libc::c_int) as wint_t,
            act: SEL_CTRL_U,
        };
        init
    },
    {
        let mut init = key {
            sym: 0o406 as libc::c_int as wint_t,
            act: SEL_HOME,
        };
        init
    },
    {
        let mut init = key {
            sym: 'g' as i32 as wint_t,
            act: SEL_HOME,
        };
        init
    },
    {
        let mut init = key {
            sym: ('A' as i32 & 0x1f as libc::c_int) as wint_t,
            act: SEL_HOME,
        };
        init
    },
    {
        let mut init = key {
            sym: 0o550 as libc::c_int as wint_t,
            act: SEL_END,
        };
        init
    },
    {
        let mut init = key {
            sym: 'G' as i32 as wint_t,
            act: SEL_END,
        };
        init
    },
    {
        let mut init = key {
            sym: ('E' as i32 & 0x1f as libc::c_int) as wint_t,
            act: SEL_END,
        };
        init
    },
    {
        let mut init = key {
            sym: '\'' as i32 as wint_t,
            act: SEL_FIRST,
        };
        init
    },
    {
        let mut init = key {
            sym: 'J' as i32 as wint_t,
            act: SEL_JUMP,
        };
        init
    },
    {
        let mut init = key {
            sym: '~' as i32 as wint_t,
            act: SEL_CDHOME,
        };
        init
    },
    {
        let mut init = key {
            sym: '@' as i32 as wint_t,
            act: SEL_CDBEGIN,
        };
        init
    },
    {
        let mut init = key {
            sym: '-' as i32 as wint_t,
            act: SEL_CDLAST,
        };
        init
    },
    {
        let mut init = key {
            sym: '`' as i32 as wint_t,
            act: SEL_CDROOT,
        };
        init
    },
    {
        let mut init = key {
            sym: 'b' as i32 as wint_t,
            act: SEL_BMOPEN,
        };
        init
    },
    {
        let mut init = key {
            sym: ('_' as i32 & 0x1f as libc::c_int) as wint_t,
            act: SEL_BMOPEN,
        };
        init
    },
    {
        let mut init = key {
            sym: 'c' as i32 as wint_t,
            act: SEL_REMOTE,
        };
        init
    },
    {
        let mut init = key {
            sym: '\t' as i32 as wint_t,
            act: SEL_CYCLE,
        };
        init
    },
    {
        let mut init = key {
            sym: 0o541 as libc::c_int as wint_t,
            act: SEL_CYCLER,
        };
        init
    },
    {
        let mut init = key {
            sym: '1' as i32 as wint_t,
            act: SEL_CTX1,
        };
        init
    },
    {
        let mut init = key {
            sym: '2' as i32 as wint_t,
            act: SEL_CTX2,
        };
        init
    },
    {
        let mut init = key {
            sym: '3' as i32 as wint_t,
            act: SEL_CTX3,
        };
        init
    },
    {
        let mut init = key {
            sym: '4' as i32 as wint_t,
            act: SEL_CTX4,
        };
        init
    },
    {
        let mut init = key {
            sym: ',' as i32 as wint_t,
            act: SEL_MARK,
        };
        init
    },
    {
        let mut init = key {
            sym: 'B' as i32 as wint_t,
            act: SEL_BMARK,
        };
        init
    },
    {
        let mut init = key {
            sym: '/' as i32 as wint_t,
            act: SEL_FLTR,
        };
        init
    },
    {
        let mut init = key {
            sym: ('N' as i32 & 0x1f as libc::c_int) as wint_t,
            act: SEL_MFLTR,
        };
        init
    },
    {
        let mut init = key {
            sym: '.' as i32 as wint_t,
            act: SEL_HIDDEN,
        };
        init
    },
    {
        let mut init = key {
            sym: 'd' as i32 as wint_t,
            act: SEL_DETAIL,
        };
        init
    },
    {
        let mut init = key {
            sym: 'f' as i32 as wint_t,
            act: SEL_STATS,
        };
        init
    },
    {
        let mut init = key {
            sym: ('F' as i32 & 0x1f as libc::c_int) as wint_t,
            act: SEL_STATS,
        };
        init
    },
    {
        let mut init = key {
            sym: '*' as i32 as wint_t,
            act: SEL_CHMODX,
        };
        init
    },
    {
        let mut init = key {
            sym: 'z' as i32 as wint_t,
            act: SEL_ARCHIVE,
        };
        init
    },
    {
        let mut init = key {
            sym: 't' as i32 as wint_t,
            act: SEL_SORT,
        };
        init
    },
    {
        let mut init = key {
            sym: ('T' as i32 & 0x1f as libc::c_int) as wint_t,
            act: SEL_SORT,
        };
        init
    },
    {
        let mut init = key {
            sym: ('L' as i32 & 0x1f as libc::c_int) as wint_t,
            act: SEL_REDRAW,
        };
        init
    },
    {
        let mut init = key {
            sym: ' ' as i32 as wint_t,
            act: SEL_SEL,
        };
        init
    },
    {
        let mut init = key {
            sym: '+' as i32 as wint_t,
            act: SEL_SEL,
        };
        init
    },
    {
        let mut init = key {
            sym: 'm' as i32 as wint_t,
            act: SEL_SELMUL,
        };
        init
    },
    {
        let mut init = key {
            sym: 'a' as i32 as wint_t,
            act: SEL_SELALL,
        };
        init
    },
    {
        let mut init = key {
            sym: 'A' as i32 as wint_t,
            act: SEL_SELINV,
        };
        init
    },
    {
        let mut init = key {
            sym: 'E' as i32 as wint_t,
            act: SEL_SELEDIT,
        };
        init
    },
    {
        let mut init = key {
            sym: 'p' as i32 as wint_t,
            act: SEL_CP,
        };
        init
    },
    {
        let mut init = key {
            sym: ('P' as i32 & 0x1f as libc::c_int) as wint_t,
            act: SEL_CP,
        };
        init
    },
    {
        let mut init = key {
            sym: 'v' as i32 as wint_t,
            act: SEL_MV,
        };
        init
    },
    {
        let mut init = key {
            sym: ('V' as i32 & 0x1f as libc::c_int) as wint_t,
            act: SEL_MV,
        };
        init
    },
    {
        let mut init = key {
            sym: 'w' as i32 as wint_t,
            act: SEL_CPMVAS,
        };
        init
    },
    {
        let mut init = key {
            sym: ('W' as i32 & 0x1f as libc::c_int) as wint_t,
            act: SEL_CPMVAS,
        };
        init
    },
    {
        let mut init = key {
            sym: 'x' as i32 as wint_t,
            act: SEL_RM,
        };
        init
    },
    {
        let mut init = key {
            sym: ('X' as i32 & 0x1f as libc::c_int) as wint_t,
            act: SEL_RM,
        };
        init
    },
    {
        let mut init = key {
            sym: 'o' as i32 as wint_t,
            act: SEL_OPENWITH,
        };
        init
    },
    {
        let mut init = key {
            sym: ('O' as i32 & 0x1f as libc::c_int) as wint_t,
            act: SEL_OPENWITH,
        };
        init
    },
    {
        let mut init = key {
            sym: 'n' as i32 as wint_t,
            act: SEL_NEW,
        };
        init
    },
    {
        let mut init = key {
            sym: ('R' as i32 & 0x1f as libc::c_int) as wint_t,
            act: SEL_RENAME,
        };
        init
    },
    {
        let mut init = key {
            sym: 'r' as i32 as wint_t,
            act: SEL_RENAMEMUL,
        };
        init
    },
    {
        let mut init = key {
            sym: 'u' as i32 as wint_t,
            act: SEL_UMOUNT,
        };
        init
    },
    {
        let mut init = key {
            sym: '?' as i32 as wint_t,
            act: SEL_HELP,
        };
        init
    },
    {
        let mut init = key {
            sym: ('J' as i32 & 0x1f as libc::c_int) as wint_t,
            act: SEL_AUTONEXT,
        };
        init
    },
    {
        let mut init = key {
            sym: 'e' as i32 as wint_t,
            act: SEL_EDIT,
        };
        init
    },
    {
        let mut init = key {
            sym: ';' as i32 as wint_t,
            act: SEL_PLUGIN,
        };
        init
    },
    {
        let mut init = key {
            sym: '!' as i32 as wint_t,
            act: SEL_SHELL,
        };
        init
    },
    {
        let mut init = key {
            sym: (']' as i32 & 0x1f as libc::c_int) as wint_t,
            act: SEL_SHELL,
        };
        init
    },
    {
        let mut init = key {
            sym: '=' as i32 as wint_t,
            act: SEL_LAUNCH,
        };
        init
    },
    {
        let mut init = key {
            sym: ']' as i32 as wint_t,
            act: SEL_PROMPT,
        };
        init
    },
    {
        let mut init = key {
            sym: '0' as i32 as wint_t,
            act: SEL_LOCK,
        };
        init
    },
    {
        let mut init = key {
            sym: 's' as i32 as wint_t,
            act: SEL_SESSIONS,
        };
        init
    },
    {
        let mut init = key {
            sym: '>' as i32 as wint_t,
            act: SEL_EXPORT,
        };
        init
    },
    {
        let mut init = key {
            sym: 'T' as i32 as wint_t,
            act: SEL_TIMETYPE,
        };
        init
    },
    {
        let mut init = key {
            sym: 'q' as i32 as wint_t,
            act: SEL_QUITCTX,
        };
        init
    },
    {
        let mut init = key {
            sym: ('G' as i32 & 0x1f as libc::c_int) as wint_t,
            act: SEL_QUITCD,
        };
        init
    },
    {
        let mut init = key {
            sym: ('Q' as i32 & 0x1f as libc::c_int) as wint_t,
            act: SEL_QUIT,
        };
        init
    },
    {
        let mut init = key {
            sym: 'Q' as i32 as wint_t,
            act: SEL_QUITERR,
        };
        init
    },
    {
        let mut init = key {
            sym: 0o631 as libc::c_int as wint_t,
            act: SEL_CLICK,
        };
        init
    },
];
static mut cfg: settings = settings {
    filtermode_timeorder_sizeorder_apparentsz_blkorder_extnorder_showhidden_reserved0_showdetail_ctxactive_reverse_version_reserved1_curctx_prefersel_fileinfo_nonavopen_autoenter_reserved2_useeditor_reserved3_regex_x11_timetype_cliopener_waitedit_rollover: [0; 4],
};
static mut g_ctx: [context; 4] = [context {
    c_path: [0; 4096],
    c_last: [0; 4096],
    c_name: [0; 256],
    c_fltr: [0; 48],
    c_cfg: settings {
        filtermode_timeorder_sizeorder_apparentsz_blkorder_extnorder_showhidden_reserved0_showdetail_ctxactive_reverse_version_reserved1_curctx_prefersel_fileinfo_nonavopen_autoenter_reserved2_useeditor_reserved3_regex_x11_timetype_cliopener_waitedit_rollover: [0; 4],
    },
    color: 0,
}; 4];
static mut ndents: libc::c_int = 0;
static mut cur: libc::c_int = 0;
static mut last: libc::c_int = 0;
static mut curscroll: libc::c_int = 0;
static mut last_curscroll: libc::c_int = 0;
static mut total_dents: libc::c_int = 64 as libc::c_int;
static mut scroll_lines: libc::c_int = 1 as libc::c_int;
static mut nselected: libc::c_int = 0;
static mut fifofd: libc::c_int = -(1 as libc::c_int);
static mut gtimesecs: time_t = 0;
static mut idletimeout: uint_t = 0;
static mut selbufpos: uint_t = 0;
static mut selbuflen: uint_t = 0;
static mut xlines: ushort_t = 0;
static mut xcols: ushort_t = 0;
static mut idle: ushort_t = 0;
static mut maxbm: uchar_t = 0;
static mut maxplug: uchar_t = 0;
static mut maxorder: uchar_t = 0;
static mut cfgsort: [uchar_t; 5] = [0; 5];
static mut bmstr: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut pluginstr: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut orderstr: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut opener: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut editor: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut enveditor: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut pager: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut shell: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut home: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut initpath: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut cfgpath: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut selpath: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut listpath: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut listroot: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut plgpath: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut pnamebuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut pselbuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut findselpos: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut mark: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut hostname: [libc::c_char; 256] = [0; 256];
static mut fifopath: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut lastcmd: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut ihashbmp: *mut ullong_t = 0 as *const ullong_t as *mut ullong_t;
static mut pdents: *mut entry = 0 as *const entry as *mut entry;
static mut dir_blocks: blkcnt_t = 0;
static mut bookmark: *mut kv = 0 as *const kv as *mut kv;
static mut plug: *mut kv = 0 as *const kv as *mut kv;
static mut order: *mut kv = 0 as *const kv as *mut kv;
static mut tmpfplen: uchar_t = 0;
static mut homelen: uchar_t = 0;
static mut blk_shift: uchar_t = 9 as libc::c_int as uchar_t;
static mut middle_click_key: libc::c_int = 0;
static mut archive_re: regex_t = regex_t {
    buffer: 0 as *const re_dfa_t as *mut re_dfa_t,
    allocated: 0,
    used: 0,
    syntax: 0,
    fastmap: 0 as *const libc::c_char as *mut libc::c_char,
    translate: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    re_nsub: 0,
    can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
    c2rust_padding: [0; 7],
};
static mut threadbmp: libc::c_int = -(1 as libc::c_int);
static mut active_threads: libc::c_int = 0;
static mut running_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut hardlink_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut core_files: *mut ullong_t = 0 as *const ullong_t as *mut ullong_t;
static mut core_blocks: *mut blkcnt_t = 0 as *const blkcnt_t as *mut blkcnt_t;
static mut num_files: ullong_t = 0;
static mut core_data: *mut thread_data = 0 as *const thread_data as *mut thread_data;
static mut oldsighup: sigaction = sigaction {
    __sigaction_handler: C2RustUnnamed_10 {
        sa_handler: None,
    },
    sa_mask: __sigset_t { __val: [0; 16] },
    sa_flags: 0,
    sa_restorer: None,
};
static mut oldsigtstp: sigaction = sigaction {
    __sigaction_handler: C2RustUnnamed_10 {
        sa_handler: None,
    },
    sa_mask: __sigset_t { __val: [0; 16] },
    sa_flags: 0,
    sa_restorer: None,
};
static mut oldsigwinch: sigaction = sigaction {
    __sigaction_handler: C2RustUnnamed_10 {
        sa_handler: None,
    },
    sa_mask: __sigset_t { __val: [0; 16] },
    sa_flags: 0,
    sa_restorer: None,
};
static mut g_buf: [libc::c_char; 4608] = [0; 4608];
static mut g_sel: [libc::c_char; 4096] = [0; 4096];
static mut g_tmpfpath: [libc::c_char; 64] = [0; 64];
static mut g_pipepath: [libc::c_char; 64] = [0; 64];
static mut g_state: runstate = runstate {
    autofifo_autonext_dircolor_dirctx_duinit_fifomode_forcequit_initfile_interrupt_move_0_oldcolor_picked_picker_pluginit_prstssn_rangesel_runctx_runplugin_selbm_selmode_stayonsel_trash_uidgid_usebsdtar_reserved: [0; 4],
};
static mut utils: [*mut libc::c_char; 21] = [
    b"xdg-open\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"atool\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"bsdtar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"unzip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"vlock\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"launch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sh -c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"bash\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sshfs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"rclone\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"vi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"less\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"fzf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b".ntfy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b".cbcp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b".nmv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"trash-put\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gio trash\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"rm -rf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
static mut messages: [*const libc::c_char; 45] = [
    b"\0" as *const u8 as *const libc::c_char,
    b"0 entries\0" as *const u8 as *const libc::c_char,
    b"/.nnnXXXXXX\0" as *const u8 as *const libc::c_char,
    b"0 selected\0" as *const u8 as *const libc::c_char,
    b"cancelled\0" as *const u8 as *const libc::c_char,
    b"failed!\0" as *const u8 as *const libc::c_char,
    b"session name: \0" as *const u8 as *const libc::c_char,
    b"'c'p/'m'v as?\0" as *const u8 as *const libc::c_char,
    b"'c'urrent/'s'el?\0" as *const u8 as *const libc::c_char,
    b"%s %s? [Esc cancels]\0" as *const u8 as *const libc::c_char,
    b"size limit exceeded\0" as *const u8 as *const libc::c_char,
    b"'f'ile/'d'ir/'s'ym/'h'ard?\0" as *const u8 as *const libc::c_char,
    b"'c'li/'g'ui?\0" as *const u8 as *const libc::c_char,
    b"overwrite?\0" as *const u8 as *const libc::c_char,
    b"'s'ave/'l'oad/'r'estore?\0" as *const u8 as *const libc::c_char,
    b"Quit all contexts?\0" as *const u8 as *const libc::c_char,
    b"remote name (- for hovered): \0" as *const u8 as *const libc::c_char,
    b"archive [path/]name: \0" as *const u8 as *const libc::c_char,
    b"open with: \0" as *const u8 as *const libc::c_char,
    b"[path/]name: \0" as *const u8 as *const libc::c_char,
    b"link prefix [@ for none]: \0" as *const u8 as *const libc::c_char,
    b"copy [path/]name: \0" as *const u8 as *const libc::c_char,
    b"\n'Enter' to continue\0" as *const u8 as *const libc::c_char,
    b"open failed\0" as *const u8 as *const libc::c_char,
    b"dir inaccessible\0" as *const u8 as *const libc::c_char,
    b"empty! edit/open with\0" as *const u8 as *const libc::c_char,
    b"?\0" as *const u8 as *const libc::c_char,
    b"not set\0" as *const u8 as *const libc::c_char,
    b"entry exists\0" as *const u8 as *const libc::c_char,
    b"too few cols!\0" as *const u8 as *const libc::c_char,
    b"'s'shfs/'r'clone?\0" as *const u8 as *const libc::c_char,
    b"refresh if slow\0" as *const u8 as *const libc::c_char,
    b"app: \0" as *const u8 as *const libc::c_char,
    b"'o'pen/e'x'tract/'l's/'m'nt?\0" as *const u8 as *const libc::c_char,
    b"keys:\0" as *const u8 as *const libc::c_char,
    b"invalid regex\0" as *const u8 as *const libc::c_char,
    b"'a'u/'d'u/'e'xt/'r'ev/'s'z/'t'm/'v'er/'c'lr/'^T'?\0" as *const u8
        as *const libc::c_char,
    b"unmount failed! try lazy?\0" as *const u8 as *const libc::c_char,
    b"first file (')/char?\0" as *const u8 as *const libc::c_char,
    b"remove tmp file?\0" as *const u8 as *const libc::c_char,
    b"invalid key\0" as *const u8 as *const libc::c_char,
    b"unchanged\0" as *const u8 as *const libc::c_char,
    b"dir changed, range sel off\0" as *const u8 as *const libc::c_char,
    b"name: \0" as *const u8 as *const libc::c_char,
    b"file limit exceeded\0" as *const u8 as *const libc::c_char,
];
static mut env_cfg: [*const libc::c_char; 14] = [
    b"NNN_OPTS\0" as *const u8 as *const libc::c_char,
    b"NNN_BMS\0" as *const u8 as *const libc::c_char,
    b"NNN_PLUG\0" as *const u8 as *const libc::c_char,
    b"NNN_OPENER\0" as *const u8 as *const libc::c_char,
    b"NNN_COLORS\0" as *const u8 as *const libc::c_char,
    b"NNN_FCOLORS\0" as *const u8 as *const libc::c_char,
    b"NNNLVL\0" as *const u8 as *const libc::c_char,
    b"NNN_PIPE\0" as *const u8 as *const libc::c_char,
    b"NNN_MCLICK\0" as *const u8 as *const libc::c_char,
    b"NNN_SEL\0" as *const u8 as *const libc::c_char,
    b"NNN_ARCHIVE\0" as *const u8 as *const libc::c_char,
    b"NNN_ORDER\0" as *const u8 as *const libc::c_char,
    b"NNN_HELP\0" as *const u8 as *const libc::c_char,
    b"NNN_TRASH\0" as *const u8 as *const libc::c_char,
];
static mut envs: [*const libc::c_char; 5] = [
    b"SHELL\0" as *const u8 as *const libc::c_char,
    b"VISUAL\0" as *const u8 as *const libc::c_char,
    b"EDITOR\0" as *const u8 as *const libc::c_char,
    b"PAGER\0" as *const u8 as *const libc::c_char,
    b"nnn\0" as *const u8 as *const libc::c_char,
];
static mut cp: [libc::c_char; 10] = unsafe {
    *::std::mem::transmute::<&[u8; 10], &mut [libc::c_char; 10]>(b"cp   -iRp\0")
};
static mut mv: [libc::c_char; 8] = unsafe {
    *::std::mem::transmute::<&[u8; 8], &mut [libc::c_char; 8]>(b"mv   -i\0")
};
static mut archive_cmd: [*const libc::c_char; 4] = [
    b"atool -a\0" as *const u8 as *const libc::c_char,
    b"bsdtar -acvf\0" as *const u8 as *const libc::c_char,
    b"zip -r\0" as *const u8 as *const libc::c_char,
    b"tar -acvf\0" as *const u8 as *const libc::c_char,
];
static mut toks: [*const libc::c_char; 4] = [
    b"bookmarks\0" as *const u8 as *const libc::c_char,
    b"sessions\0" as *const u8 as *const libc::c_char,
    b"mounts\0" as *const u8 as *const libc::c_char,
    b"plugins\0" as *const u8 as *const libc::c_char,
];
static mut patterns: [*const libc::c_char; 5] = [
    b"sed -i 's|^\\(\\(.*/\\)\\(.*\\)$\\)|#\\1\\n\\3|' %s\0" as *const u8
        as *const libc::c_char,
    b"sed 's|^\\([^#/][^/]\\?.*\\)$|%s/\\1|;s|^#\\(/.*\\)$|\\1|' %s | tr '\\n' '\\0' | xargs -0 -n2 sh -c '%s \"$0\" \"$@\" < /dev/tty'\0"
        as *const u8 as *const libc::c_char,
    b"\\.(bz|bz2|gz|tar|taz|tbz|tbz2|tgz|z|zip)$\0" as *const u8 as *const libc::c_char,
    b"sed -i 's|^%s\\(.*\\)$|%s\\1|' %s\0" as *const u8 as *const libc::c_char,
    b"sed -ze 's|^%s/||' '%s' | xargs -0 %s %s\0" as *const u8 as *const libc::c_char,
];
static mut gcolors: [libc::c_char; 25] = unsafe {
    *::std::mem::transmute::<
        &[u8; 25],
        &mut [libc::c_char; 25],
    >(b"c1e2272e006033f7c6d6abc4\0")
};
static mut fcolors: [uint_t; 17] = [
    0 as libc::c_int as uint_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
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
static mut inotify_fd: libc::c_int = 0;
static mut inotify_wd: libc::c_int = -(1 as libc::c_int);
static mut INOTIFY_MASK: uint_t = (0x100 as libc::c_int | 0x200 as libc::c_int
    | 0x400 as libc::c_int | 0x2 as libc::c_int | 0x800 as libc::c_int
    | 0x40 as libc::c_int | 0x80 as libc::c_int) as uint_t;
unsafe extern "C" fn sigint_handler(mut sig: libc::c_int) {
    g_state.set_interrupt(1 as libc::c_int as uint_t);
}
unsafe extern "C" fn clean_exit_sighandler(mut sig: libc::c_int) {
    endwin();
    exit(0 as libc::c_int);
}
unsafe extern "C" fn xitoa(mut val: uint_t) -> *mut libc::c_char {
    static mut dst: [libc::c_char; 32] = [
        '\0' as i32 as libc::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    static mut digits: [libc::c_char; 201] = unsafe {
        *::std::mem::transmute::<
            &[u8; 201],
            &[libc::c_char; 201],
        >(
            b"00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899\0",
        )
    };
    let mut next: uint_t = 30 as libc::c_int as uint_t;
    let mut quo: uint_t = 0;
    let mut i: uint_t = 0;
    while val >= 100 as libc::c_int as libc::c_uint {
        quo = val.wrapping_div(100 as libc::c_int as libc::c_uint);
        i = val
            .wrapping_sub(quo.wrapping_mul(100 as libc::c_int as libc::c_uint))
            .wrapping_mul(2 as libc::c_int as libc::c_uint);
        val = quo;
        dst[next
            as usize] = digits[i.wrapping_add(1 as libc::c_int as libc::c_uint)
            as usize];
        next = next.wrapping_sub(1);
        dst[next as usize] = digits[i as usize];
        next = next.wrapping_sub(1);
        next;
    }
    if val < 10 as libc::c_int as libc::c_uint {
        dst[next
            as usize] = ('0' as i32 as libc::c_uint).wrapping_add(val) as libc::c_char;
    } else {
        i = val.wrapping_mul(2 as libc::c_int as libc::c_uint);
        dst[next
            as usize] = digits[i.wrapping_add(1 as libc::c_int as libc::c_uint)
            as usize];
        next = next.wrapping_sub(1);
        dst[next as usize] = digits[i as usize];
    }
    return &mut *dst.as_mut_ptr().offset(next as isize) as *mut libc::c_char;
}
unsafe extern "C" fn xchartohex(mut c: uchar_t) -> uchar_t {
    if (c as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
        <= 9 as libc::c_int as libc::c_uint
    {
        return (c as libc::c_int - '0' as i32) as uchar_t;
    }
    if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'f' as i32 {
        return (c as libc::c_int - 'a' as i32 + 10 as libc::c_int) as uchar_t;
    }
    if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'F' as i32 {
        return (c as libc::c_int - 'A' as i32 + 10 as libc::c_int) as uchar_t;
    }
    return c;
}
unsafe extern "C" fn test_set_bit(mut nr: uint_t) -> bool {
    nr &= 0xffffff as libc::c_int as libc::c_uint;
    pthread_mutex_lock(&mut hardlink_mutex);
    let mut m: *mut ullong_t = ihashbmp.offset((nr >> 6 as libc::c_int) as isize);
    if *m
        & ((1 as libc::c_int) << (nr & 63 as libc::c_int as libc::c_uint))
            as libc::c_ulonglong != 0
    {
        pthread_mutex_unlock(&mut hardlink_mutex);
        return 0 as libc::c_int != 0;
    }
    *m
        |= ((1 as libc::c_int) << (nr & 63 as libc::c_int as libc::c_uint))
            as libc::c_ulonglong;
    pthread_mutex_unlock(&mut hardlink_mutex);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn max_openfds() {
    let mut rl: rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };
    if getrlimit(RLIMIT_NOFILE, &mut rl) == 0 {
        if rl.rlim_cur < rl.rlim_max {
            rl.rlim_cur = rl.rlim_max;
            setrlimit(RLIMIT_NOFILE, &mut rl);
        }
    }
}
unsafe extern "C" fn xrealloc(
    mut pcur: *mut libc::c_void,
    mut len: size_t,
) -> *mut libc::c_void {
    let mut pmem: *mut libc::c_void = realloc(pcur, len);
    if pmem.is_null() {
        free(pcur);
    }
    return pmem;
}
unsafe extern "C" fn xstrsncpy(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut n: size_t,
) -> size_t {
    let mut end: *mut libc::c_char = memccpy(
        dst as *mut libc::c_void,
        src as *const libc::c_void,
        '\0' as i32,
        n,
    ) as *mut libc::c_char;
    if end.is_null() {
        *dst
            .offset(
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
        end = dst.offset(n as isize);
    }
    return end.offset_from(dst) as libc::c_long as size_t;
}
#[inline]
unsafe extern "C" fn xstrlen(mut s: *const libc::c_char) -> size_t {
    return (rawmemchr(s as *const libc::c_void, '\0' as i32) as *mut libc::c_char)
        .offset_from(s) as libc::c_long as size_t;
}
unsafe extern "C" fn xstrdup(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut len: size_t = (xstrlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut ptr: *mut libc::c_char = malloc(len) as *mut libc::c_char;
    if !ptr.is_null() {
        xstrsncpy(ptr, s, len);
    }
    return ptr;
}
unsafe extern "C" fn is_suffix(
    mut str: *const libc::c_char,
    mut suffix: *const libc::c_char,
) -> bool {
    if str.is_null() || suffix.is_null() {
        return 0 as libc::c_int != 0;
    }
    let mut lenstr: size_t = xstrlen(str);
    let mut lensuffix: size_t = xstrlen(suffix);
    if lensuffix > lenstr {
        return 0 as libc::c_int != 0;
    }
    return (if *str.offset(lenstr.wrapping_sub(lensuffix) as isize) as libc::c_int
        != *suffix as libc::c_int
    {
        -(1 as libc::c_int)
    } else {
        strcmp(str.offset(lenstr.wrapping_sub(lensuffix) as isize), suffix)
    }) == 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn is_prefix(
    mut str: *const libc::c_char,
    mut prefix: *const libc::c_char,
    mut len: size_t,
) -> bool {
    return strncmp(str, prefix, len) == 0;
}
unsafe extern "C" fn xmemrchr(
    mut s: *mut uchar_t,
    mut ch: uchar_t,
    mut n: size_t,
) -> *mut libc::c_void {
    return memrchr(s as *const libc::c_void, ch as libc::c_int, n);
}
unsafe extern "C" fn xdirname(mut path: *mut libc::c_char) -> *mut libc::c_char {
    let mut base: *mut libc::c_char = xmemrchr(
        path as *mut uchar_t,
        '/' as i32 as uchar_t,
        xstrlen(path),
    ) as *mut libc::c_char;
    if base == path {
        *path.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    } else {
        *base = '\0' as i32 as libc::c_char;
    }
    return path;
}
unsafe extern "C" fn xbasename(mut path: *mut libc::c_char) -> *mut libc::c_char {
    let mut base: *mut libc::c_char = xmemrchr(
        path as *mut uchar_t,
        '/' as i32 as uchar_t,
        xstrlen(path),
    ) as *mut libc::c_char;
    return if !base.is_null() { base.offset(1 as libc::c_int as isize) } else { path };
}
#[inline]
unsafe extern "C" fn xextension(
    mut fname: *const libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    return xmemrchr(fname as *mut uchar_t, '.' as i32 as uchar_t, len)
        as *mut libc::c_char;
}
static mut uidcache: uint_t = 0;
unsafe extern "C" fn getpwname(mut uid: uid_t) -> *mut libc::c_char {
    static mut namecache: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    if uidcache != uid {
        let mut pw: *mut passwd = getpwuid(uid);
        uidcache = uid;
        namecache = if !pw.is_null() { (*pw).pw_name } else { 0 as *mut libc::c_char };
    }
    return if !namecache.is_null() { namecache } else { xitoa(uid) };
}
static mut gidcache: uint_t = 0;
unsafe extern "C" fn getgrname(mut gid: gid_t) -> *mut libc::c_char {
    static mut grpcache: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    if gidcache != gid {
        let mut gr: *mut group = getgrgid(gid);
        gidcache = gid;
        grpcache = if !gr.is_null() { (*gr).gr_name } else { 0 as *mut libc::c_char };
    }
    return if !grpcache.is_null() { grpcache } else { xitoa(gid) };
}
#[inline]
unsafe extern "C" fn getutil(mut util: *mut libc::c_char) -> bool {
    return spawn(
        b"which\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        util,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        (0x8 as libc::c_int | 0x4 as libc::c_int) as ushort_t,
    ) == 0 as libc::c_int;
}
unsafe extern "C" fn mkpath(
    mut dir: *const libc::c_char,
    mut name: *const libc::c_char,
    mut out: *mut libc::c_char,
) -> size_t {
    let mut len: size_t = 0 as libc::c_int as size_t;
    if *name.offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32 {
        len = if *dir.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
            && *dir.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            1 as libc::c_int as libc::c_ulong
        } else {
            xstrsncpy(out, dir, 4096 as libc::c_int as size_t)
        };
        *out
            .offset(
                len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '/' as i32 as libc::c_char;
    }
    return (xstrsncpy(
        out.offset(len as isize),
        name,
        (4096 as libc::c_int as libc::c_ulong).wrapping_sub(len),
    ))
        .wrapping_add(len);
}
unsafe extern "C" fn common_prefix(
    mut path: *const libc::c_char,
    mut prefix: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut x: *const libc::c_char = path;
    let mut y: *const libc::c_char = prefix;
    let mut sep: *mut libc::c_char = 0 as *mut libc::c_char;
    if path.is_null() || *path == 0 || prefix.is_null() {
        return 0 as *mut libc::c_char;
    }
    if *prefix == 0 {
        xstrsncpy(prefix, path, 4096 as libc::c_int as size_t);
        return prefix;
    }
    while *x as libc::c_int != 0 && *y as libc::c_int != 0
        && *x as libc::c_int == *y as libc::c_int
    {
        x = x.offset(1);
        x;
        y = y.offset(1);
        y;
    }
    if *x == 0 && *y == 0 {
        return prefix;
    }
    if *x == 0 && *y as libc::c_int == '/' as i32 {
        xstrsncpy(prefix, path, y.offset_from(path) as libc::c_long as size_t);
        return prefix;
    }
    if *y == 0 && *x as libc::c_int == '/' as i32 {
        return prefix;
    }
    *prefix
        .offset(
            y.offset_from(prefix) as libc::c_long as isize,
        ) = '\0' as i32 as libc::c_char;
    sep = xmemrchr(
        prefix as *mut uchar_t,
        '/' as i32 as uchar_t,
        y.offset_from(prefix) as libc::c_long as size_t,
    ) as *mut libc::c_char;
    if sep != prefix {
        *sep = '\0' as i32 as libc::c_char;
    } else {
        *prefix.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
    return prefix;
}
unsafe extern "C" fn abspath(
    mut path: *const libc::c_char,
    mut cwd: *const libc::c_char,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    if path.is_null() {
        return 0 as *mut libc::c_char;
    }
    if *path.offset(0 as libc::c_int as isize) as libc::c_int == '~' as i32 {
        cwd = home;
    } else if *path.offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32
        && cwd.is_null()
    {
        cwd = getcwd(0 as *mut libc::c_char, 0 as libc::c_int as size_t);
    }
    let mut dst_size: size_t = 0 as libc::c_int as size_t;
    let mut src_size: size_t = xstrlen(path);
    let mut cwd_size: size_t = if !cwd.is_null() {
        xstrlen(cwd)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut len: size_t = src_size;
    let mut src: *const libc::c_char = 0 as *const libc::c_char;
    let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut resolved_path: *mut libc::c_char = (if !buf.is_null() {
        buf as *mut libc::c_void
    } else {
        malloc(
            src_size
                .wrapping_add(cwd_size)
                .wrapping_add(2 as libc::c_int as libc::c_ulong),
        )
    }) as *mut libc::c_char;
    if resolved_path.is_null() {
        return 0 as *mut libc::c_char;
    }
    if *path.offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32 {
        if cwd.is_null() {
            if buf.is_null() {
                free(resolved_path as *mut libc::c_void);
            }
            return 0 as *mut libc::c_char;
        }
        dst_size = (xstrsncpy(
            resolved_path,
            cwd,
            cwd_size.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    } else {
        *resolved_path.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
    src = path;
    dst = resolved_path.offset(dst_size as isize);
    let mut next: *const libc::c_char = 0 as *const libc::c_char;
    while next != path.offset(src_size as isize) {
        next = memchr(src as *const libc::c_void, '/' as i32, len)
            as *const libc::c_char;
        if next.is_null() {
            next = path.offset(src_size as isize);
        }
        if next.offset_from(src) as libc::c_long == 2 as libc::c_int as libc::c_long
            && *src.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && *src.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
        {
            if dst.offset_from(resolved_path) as libc::c_long != 0 {
                dst = xmemrchr(
                    resolved_path as *mut uchar_t,
                    '/' as i32 as uchar_t,
                    dst.offset_from(resolved_path) as libc::c_long as size_t,
                ) as *mut libc::c_char;
                *dst = '\0' as i32 as libc::c_char;
            }
        } else if !(next.offset_from(src) as libc::c_long
            == 1 as libc::c_int as libc::c_long
            && *src.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32)
        {
            if next.offset_from(src) as libc::c_long != 0 {
                let fresh0 = dst;
                dst = dst.offset(1);
                *fresh0 = '/' as i32 as libc::c_char;
                xstrsncpy(
                    dst,
                    src,
                    (next.offset_from(src) as libc::c_long
                        + 1 as libc::c_int as libc::c_long) as size_t,
                );
                dst = dst.offset(next.offset_from(src) as libc::c_long as isize);
            }
        }
        src = next.offset(1 as libc::c_int as isize);
        len = src_size
            .wrapping_sub(src.offset_from(path) as libc::c_long as libc::c_ulong);
    }
    if *resolved_path as libc::c_int == '\0' as i32 {
        *resolved_path.offset(0 as libc::c_int as isize) = '/' as i32 as libc::c_char;
        *resolved_path.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
    return resolved_path;
}
unsafe extern "C" fn set_tilde_in_path(mut path: *mut libc::c_char) -> bool {
    if is_prefix(path, home, homelen as size_t) {
        *home
            .offset(
                homelen as isize,
            ) = *path.offset((homelen as libc::c_int - 1 as libc::c_int) as isize);
        *path
            .offset(
                (homelen as libc::c_int - 1 as libc::c_int) as isize,
            ) = '~' as i32 as libc::c_char;
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn reset_tilde_in_path(mut path: *mut libc::c_char) {
    *path
        .offset(
            (homelen as libc::c_int - 1 as libc::c_int) as isize,
        ) = *home.offset(homelen as isize);
    *home.offset(homelen as isize) = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn xterm_cfg(mut path: *mut libc::c_char) {
    if cfg.x11() as libc::c_int != 0 && g_state.picker() == 0 {
        printf(
            b"\x1B]7;file://%s%s\x1B\\\0" as *const u8 as *const libc::c_char,
            hostname.as_mut_ptr(),
            path,
        );
        let mut r: bool = set_tilde_in_path(path);
        printf(
            b"\x1B]2;%s\x07\0" as *const u8 as *const libc::c_char,
            if r as libc::c_int != 0 {
                &mut *path.offset((homelen as libc::c_int - 1 as libc::c_int) as isize)
                    as *mut libc::c_char
            } else {
                path
            },
        );
        fflush(stdout);
        if r {
            reset_tilde_in_path(path);
        }
    }
}
unsafe extern "C" fn convert_tilde(
    mut path: *const libc::c_char,
    mut buf: *mut libc::c_char,
) {
    if *path.offset(0 as libc::c_int as isize) as libc::c_int == '~' as i32 {
        let mut len: ssize_t = xstrlen(home) as ssize_t;
        let mut loclen: ssize_t = xstrlen(path) as ssize_t;
        xstrsncpy(buf, home, (len + 1 as libc::c_int as libc::c_long) as size_t);
        xstrsncpy(
            buf.offset(len as isize),
            path.offset(1 as libc::c_int as isize),
            loclen as size_t,
        );
    }
}
unsafe extern "C" fn create_tmp_file() -> libc::c_int {
    xstrsncpy(
        g_tmpfpath
            .as_mut_ptr()
            .offset(tmpfplen as libc::c_int as isize)
            .offset(-(1 as libc::c_int as isize)),
        messages[2 as libc::c_int as usize],
        (64 as libc::c_int - tmpfplen as libc::c_int) as size_t,
    );
    let mut fd: libc::c_int = mkstemp(g_tmpfpath.as_mut_ptr());
    fd == -(1 as libc::c_int);
    return fd;
}
unsafe extern "C" fn msg(mut message: *const libc::c_char) {
    dprintf(2 as libc::c_int, b"%s\n\0" as *const u8 as *const libc::c_char, message);
}
unsafe extern "C" fn handle_key_resize() {
    endwin();
    wrefresh(stdscr);
}
unsafe extern "C" fn clearoldprompt() {
    wmove(stdscr, xlines as libc::c_int - 2 as libc::c_int, 0 as libc::c_int);
    wclrtoeol(stdscr);
    wmove(stdscr, xlines as libc::c_int - 1 as libc::c_int, 0 as libc::c_int);
    wclrtoeol(stdscr);
    handle_key_resize();
}
#[inline]
unsafe extern "C" fn printmsg_nc(mut msg_0: *const libc::c_char) {
    wmove(stdscr, xlines as libc::c_int - 1 as libc::c_int, 0 as libc::c_int);
    waddnstr(stdscr, msg_0, -(1 as libc::c_int));
    wclrtoeol(stdscr);
}
unsafe extern "C" fn printmsg(mut msg_0: *const libc::c_char) {
    wattr_on(
        stdscr,
        ((cfg.curctx() as libc::c_int + 1 as libc::c_int) as chtype)
            << 0 as libc::c_int + 8 as libc::c_int
            & ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int,
        0 as *mut libc::c_void,
    );
    printmsg_nc(msg_0);
    wattr_off(
        stdscr,
        ((cfg.curctx() as libc::c_int + 1 as libc::c_int) as chtype)
            << 0 as libc::c_int + 8 as libc::c_int
            & ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int,
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn printwait(
    mut msg_0: *const libc::c_char,
    mut presel: *mut libc::c_int,
) {
    printmsg(msg_0);
    if !presel.is_null() {
        *presel = '$' as i32;
        if ndents != 0 {
            xstrsncpy(
                (g_ctx[cfg.curctx() as usize].c_name).as_mut_ptr(),
                (*pdents.offset(cur as isize)).name,
                (255 as libc::c_int + 1 as libc::c_int) as size_t,
            );
        }
    }
}
unsafe extern "C" fn printerr(mut linenum: libc::c_int) {
    endwin();
    perror(xitoa(linenum as uint_t));
    if g_state.picker() == 0 && !selpath.is_null() {
        unlink(selpath);
    }
    free(pselbuf as *mut libc::c_void);
    exit(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn xconfirm(mut c: libc::c_int) -> bool {
    return c == 'y' as i32 || c == 'Y' as i32;
}
unsafe extern "C" fn get_input(mut prompt: *const libc::c_char) -> libc::c_int {
    let mut ch: [wint_t; 1] = [0; 1];
    if !prompt.is_null() {
        printmsg(prompt);
    }
    wtimeout(stdscr, -(1 as libc::c_int));
    wget_wch(stdscr, ch.as_mut_ptr());
    while *ch.as_mut_ptr() == 0o632 as libc::c_int as libc::c_uint {
        if !prompt.is_null() {
            clearoldprompt();
            xlines = LINES as ushort_t;
            printmsg(prompt);
        }
        wget_wch(stdscr, ch.as_mut_ptr());
    }
    wtimeout(stdscr, 1000 as libc::c_int);
    return *ch.as_mut_ptr() as libc::c_int;
}
unsafe extern "C" fn isselfileempty() -> bool {
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
    return stat(selpath, &mut sb) == -(1 as libc::c_int) || sb.st_size == 0;
}
unsafe extern "C" fn get_cur_or_sel() -> libc::c_int {
    let mut sel: bool = selbufpos != 0 || !isselfileempty();
    if sel as libc::c_int != 0 && ndents != 0 {
        if cfg.prefersel() as libc::c_int != 0 && selbufpos != 0 {
            return 's' as i32;
        }
        let mut choice: libc::c_int = get_input(messages[8 as libc::c_int as usize]);
        return if choice == 'c' as i32 || choice == 's' as i32 {
            choice
        } else {
            0 as libc::c_int
        };
    }
    if sel {
        return 's' as i32;
    }
    if ndents != 0 {
        return 'c' as i32;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xdelay(mut delay: useconds_t) {
    wrefresh(stdscr);
    usleep(delay);
}
unsafe extern "C" fn confirm_force(mut selection: bool) -> libc::c_char {
    let mut str: [libc::c_char; 64] = [0; 64];
    snprintf(
        str.as_mut_ptr(),
        64 as libc::c_int as libc::c_ulong,
        messages[9 as libc::c_int as usize],
        if g_state.trash() as libc::c_int != 0 {
            (utils[19 as libc::c_int as usize]).offset(4 as libc::c_int as isize)
        } else {
            utils[20 as libc::c_int as usize]
        },
        if selection as libc::c_int != 0 {
            b"selection\0" as *const u8 as *const libc::c_char
        } else {
            b"hovered\0" as *const u8 as *const libc::c_char
        },
    );
    let mut r: libc::c_int = get_input(str.as_mut_ptr());
    if r == '[' as i32 & 0x1f as libc::c_int {
        return '\0' as i32 as libc::c_char;
    }
    if r == 'y' as i32 || r == 'Y' as i32 {
        return 'f' as i32 as libc::c_char;
    }
    return (if g_state.trash() as libc::c_int != 0 { '\0' as i32 } else { 'i' as i32 })
        as libc::c_char;
}
unsafe extern "C" fn writesel(mut buf: *const libc::c_char, buflen: size_t) {
    if selpath.is_null() {
        return;
    }
    let mut fd: libc::c_int = open(
        selpath,
        0o100 as libc::c_int | 0o1 as libc::c_int | 0o1000 as libc::c_int,
        0o666 as libc::c_int,
    );
    if fd != -(1 as libc::c_int) {
        if write(fd, buf as *const libc::c_void, buflen) != buflen as ssize_t {
            printwait(strerror(*__errno_location()), 0 as *mut libc::c_int);
        }
        close(fd);
    } else {
        printwait(strerror(*__errno_location()), 0 as *mut libc::c_int);
    };
}
unsafe extern "C" fn appendfpath(mut path: *const libc::c_char, len: size_t) {
    if selbufpos >= selbuflen
        || len.wrapping_add(3 as libc::c_int as libc::c_ulong)
            > selbuflen.wrapping_sub(selbufpos) as libc::c_ulong
    {
        selbuflen = (selbuflen as libc::c_uint)
            .wrapping_add(4096 as libc::c_int as libc::c_uint) as uint_t as uint_t;
        pselbuf = xrealloc(pselbuf as *mut libc::c_void, selbuflen as size_t)
            as *mut libc::c_char;
        if pselbuf.is_null() {
            printerr(1465 as libc::c_int);
        }
    }
    selbufpos = (selbufpos as libc::c_ulong)
        .wrapping_add(xstrsncpy(pselbuf.offset(selbufpos as isize), path, len)) as uint_t
        as uint_t;
}
unsafe extern "C" fn selbufrealloc(alloclen: size_t) {
    if (selbufpos as libc::c_ulong).wrapping_add(alloclen) > selbuflen as libc::c_ulong {
        selbuflen = (selbufpos as libc::c_ulong)
            .wrapping_add(alloclen)
            .wrapping_add(4096 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(4096 as libc::c_int as libc::c_ulong)
            .wrapping_mul(4096 as libc::c_int as libc::c_ulong) as uint_t;
        pselbuf = xrealloc(pselbuf as *mut libc::c_void, selbuflen as size_t)
            as *mut libc::c_char;
        if pselbuf.is_null() {
            printerr(1477 as libc::c_int);
        }
    }
}
unsafe extern "C" fn seltofile(mut fd: libc::c_int, mut pcount: *mut uint_t) -> size_t {
    let mut lastpos: uint_t = 0;
    let mut count: uint_t = 0 as libc::c_int as uint_t;
    let mut pbuf: *mut libc::c_char = pselbuf;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    let mut len: ssize_t = 0;
    let mut prefixlen: ssize_t = 0 as libc::c_int as ssize_t;
    let mut initlen: ssize_t = 0 as libc::c_int as ssize_t;
    if !pcount.is_null() {
        *pcount = 0 as libc::c_int as uint_t;
    }
    if selbufpos == 0 {
        return 0 as libc::c_int as size_t;
    }
    lastpos = selbufpos.wrapping_sub(1 as libc::c_int as libc::c_uint);
    if !listpath.is_null() {
        prefixlen = xstrlen(listroot) as ssize_t;
        initlen = xstrlen(listpath) as ssize_t;
    }
    while pos <= lastpos as libc::c_ulong {
        len = xstrlen(pbuf) as ssize_t;
        if listpath.is_null() || !is_prefix(pbuf, listpath, initlen as size_t) {
            if write(fd, pbuf as *const libc::c_void, len as size_t) != len {
                return pos;
            }
        } else {
            if write(fd, listroot as *const libc::c_void, prefixlen as size_t)
                != prefixlen
            {
                return pos;
            }
            if write(
                fd,
                pbuf.offset(initlen as isize) as *const libc::c_void,
                (len - initlen) as size_t,
            ) != len - initlen
            {
                return pos;
            }
        }
        pos = (pos as libc::c_ulong).wrapping_add(len as libc::c_ulong) as size_t
            as size_t;
        if pos <= lastpos as libc::c_ulong {
            if write(
                fd,
                b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            ) != 1 as libc::c_int as libc::c_long
            {
                return pos;
            }
            pbuf = pbuf.offset((len + 1 as libc::c_int as libc::c_long) as isize);
        }
        pos = pos.wrapping_add(1);
        pos;
        count = count.wrapping_add(1);
        count;
    }
    if !pcount.is_null() {
        *pcount = count;
    }
    return pos;
}
unsafe extern "C" fn listselfile() -> bool {
    if isselfileempty() {
        return 0 as libc::c_int != 0;
    }
    snprintf(
        g_buf.as_mut_ptr(),
        (4096 as libc::c_int
            + ((255 as libc::c_int + 1 as libc::c_int) << 1 as libc::c_int))
            as libc::c_ulong,
        b"tr '\\0' '\\n' < %s\0" as *const u8 as *const libc::c_char,
        selpath,
    );
    spawn(
        utils[7 as libc::c_int as usize],
        g_buf.as_mut_ptr(),
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        (0x8 as libc::c_int | 0x1 as libc::c_int | 0x10 as libc::c_int) as ushort_t,
    );
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn resetselind() {
    let mut r: libc::c_int = 0 as libc::c_int;
    while r < ndents {
        if ((*pdents.offset(r as isize)).c2rust_unnamed).flags() as libc::c_int
            & 0x10 as libc::c_int != 0
        {
            let ref mut fresh1 = (*pdents.offset(r as isize)).c2rust_unnamed;
            (*fresh1).set_flags((*fresh1).flags() & !(0x10 as libc::c_int) as ullong_t);
        }
        r += 1;
        r;
    }
}
unsafe extern "C" fn startselection() {
    if g_state.selmode() == 0 {
        g_state.set_selmode(1 as libc::c_int as uint_t);
        nselected = 0 as libc::c_int;
        if selbufpos != 0 {
            resetselind();
            writesel(0 as *const libc::c_char, 0 as libc::c_int as size_t);
            selbufpos = 0 as libc::c_int as uint_t;
        }
    }
}
unsafe extern "C" fn clearselection() {
    nselected = 0 as libc::c_int;
    selbufpos = 0 as libc::c_int as uint_t;
    g_state.set_selmode(0 as libc::c_int as uint_t);
    writesel(0 as *const libc::c_char, 0 as libc::c_int as size_t);
}
unsafe extern "C" fn findinsel(
    mut startpos: *mut libc::c_char,
    mut len: libc::c_int,
) -> *mut libc::c_char {
    if selbufpos == 0 {
        return 0 as *mut libc::c_char;
    }
    if startpos.is_null() {
        startpos = pselbuf;
    }
    let mut found: *mut libc::c_char = startpos;
    let mut buflen: size_t = (selbufpos as libc::c_long
        - startpos.offset_from(pselbuf) as libc::c_long) as size_t;
    loop {
        found = memmem(
            found as *const libc::c_void,
            buflen
                .wrapping_sub(
                    found.offset_from(startpos) as libc::c_long as libc::c_ulong,
                ),
            g_sel.as_mut_ptr() as *const libc::c_void,
            len as size_t,
        ) as *mut libc::c_char;
        if found.is_null() {
            return 0 as *mut libc::c_char;
        }
        if found == startpos
            || *found.offset(-(1 as libc::c_int as isize)) as libc::c_int == '\0' as i32
        {
            return found;
        }
        found = found.offset(len as isize);
        if found >= startpos.offset(buflen as isize) {
            return 0 as *mut libc::c_char;
        }
    };
}
unsafe extern "C" fn markcmp(
    mut va: *const libc::c_void,
    mut vb: *const libc::c_void,
) -> libc::c_int {
    let mut ma: *const selmark = va as *mut selmark;
    let mut mb: *const selmark = vb as *mut selmark;
    return ((*ma).startpos).offset_from((*mb).startpos) as libc::c_long as libc::c_int;
}
#[inline]
unsafe extern "C" fn findmarkentry(mut len: size_t, mut dentp: *mut entry) {
    if ((*dentp).c2rust_unnamed).flags() as libc::c_int & 0x20 as libc::c_int == 0 {
        if !(findinsel(
            findselpos,
            len
                .wrapping_add(
                    xstrsncpy(
                        g_sel.as_mut_ptr().offset(len as isize),
                        (*dentp).name,
                        ((*dentp).c2rust_unnamed).nlen() as size_t,
                    ),
                ) as libc::c_int,
        ))
            .is_null()
        {
            ((*dentp).c2rust_unnamed)
                .set_flags(
                    ((*dentp).c2rust_unnamed).flags() | 0x10 as libc::c_int as ullong_t,
                );
        }
        ((*dentp).c2rust_unnamed)
            .set_flags(
                ((*dentp).c2rust_unnamed).flags() | 0x20 as libc::c_int as ullong_t,
            );
    }
}
unsafe extern "C" fn invertselbuf(pathlen: libc::c_int) {
    let mut len: size_t = 0;
    let mut endpos: size_t = 0;
    let mut shrinklen: size_t = 0 as libc::c_int as size_t;
    let mut alloclen: size_t = 0 as libc::c_int as size_t;
    let pbuf: *mut libc::c_char = g_sel.as_mut_ptr().offset(pathlen as isize);
    let mut found: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut nmarked: libc::c_int = 0 as libc::c_int;
    let mut prev: libc::c_int = 0 as libc::c_int;
    let mut dentp: *mut entry = 0 as *mut entry;
    let mut scan: bool = 0 as libc::c_int != 0;
    let mut marked: *mut selmark = malloc(
        (nselected as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<selmark>() as libc::c_ulong),
    ) as *mut selmark;
    if marked.is_null() {
        printwait(strerror(*__errno_location()), 0 as *mut libc::c_int);
        return;
    }
    i = 0 as libc::c_int;
    while i < ndents {
        dentp = &mut *pdents.offset(i as isize) as *mut entry;
        if ((*dentp).c2rust_unnamed).flags() as libc::c_int & 0x20 as libc::c_int != 0 {
            if ((*dentp).c2rust_unnamed).flags() as libc::c_int & 0x10 as libc::c_int
                != 0
            {
                ((*dentp).c2rust_unnamed)
                    .set_flags(
                        ((*dentp).c2rust_unnamed).flags()
                            ^ 0x10 as libc::c_int as ullong_t,
                    );
                scan = 1 as libc::c_int != 0;
            } else {
                ((*dentp).c2rust_unnamed)
                    .set_flags(
                        ((*dentp).c2rust_unnamed).flags()
                            | 0x10 as libc::c_int as ullong_t,
                    );
                alloclen = (alloclen as libc::c_ulong)
                    .wrapping_add(
                        (pathlen + ((*dentp).c2rust_unnamed).nlen() as libc::c_int)
                            as libc::c_ulong,
                    ) as size_t as size_t;
            }
        } else {
            ((*dentp).c2rust_unnamed)
                .set_flags(
                    ((*dentp).c2rust_unnamed).flags() | 0x20 as libc::c_int as ullong_t,
                );
            scan = 1 as libc::c_int != 0;
        }
        if scan {
            len = (pathlen as libc::c_ulong)
                .wrapping_add(
                    xstrsncpy(pbuf, (*dentp).name, 255 as libc::c_int as size_t),
                );
            found = findinsel(findselpos, len as libc::c_int);
            if !found.is_null() {
                if findselpos == found {
                    findselpos = findselpos.offset(len as isize);
                }
                if nmarked != 0
                    && found
                        == ((*marked.offset((nmarked - 1 as libc::c_int) as isize))
                            .startpos)
                            .offset(
                                (*marked.offset((nmarked - 1 as libc::c_int) as isize)).len
                                    as isize,
                            )
                {
                    let ref mut fresh2 = (*marked
                        .offset((nmarked - 1 as libc::c_int) as isize))
                        .len;
                    *fresh2 = (*fresh2 as libc::c_ulong).wrapping_add(len) as size_t
                        as size_t;
                } else {
                    let ref mut fresh3 = (*marked.offset(nmarked as isize)).startpos;
                    *fresh3 = found;
                    (*marked.offset(nmarked as isize)).len = len;
                    nmarked += 1;
                    nmarked;
                }
                nselected -= 1;
                nselected;
                shrinklen = (shrinklen as libc::c_ulong).wrapping_add(len) as size_t
                    as size_t;
            } else {
                ((*dentp).c2rust_unnamed)
                    .set_flags(
                        ((*dentp).c2rust_unnamed).flags()
                            | 0x10 as libc::c_int as ullong_t,
                    );
                alloclen = (alloclen as libc::c_ulong)
                    .wrapping_add(
                        (pathlen + ((*dentp).c2rust_unnamed).nlen() as libc::c_int)
                            as libc::c_ulong,
                    ) as size_t as size_t;
            }
            scan = 0 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    qsort(
        marked as *mut libc::c_void,
        nmarked as size_t,
        ::std::mem::size_of::<selmark>() as libc::c_ulong,
        Some(
            markcmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 1 as libc::c_int;
    while i < nmarked {
        if (*marked.offset(i as isize)).startpos
            == ((*marked.offset(prev as isize)).startpos)
                .offset((*marked.offset(prev as isize)).len as isize)
        {
            let ref mut fresh4 = (*marked.offset(prev as isize)).len;
            *fresh4 = (*fresh4 as libc::c_ulong)
                .wrapping_add((*marked.offset(i as isize)).len) as size_t as size_t;
        } else {
            prev += 1;
            prev;
            let ref mut fresh5 = (*marked.offset(prev as isize)).startpos;
            *fresh5 = (*marked.offset(i as isize)).startpos;
            (*marked.offset(prev as isize)).len = (*marked.offset(i as isize)).len;
        }
        i += 1;
        i;
    }
    if nmarked != 0 {
        nmarked = prev + 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < nmarked {
        found = (*marked.offset(i as isize)).startpos;
        endpos = (if i + 1 as libc::c_int == nmarked {
            selbufpos as libc::c_long
        } else {
            ((*marked.offset((i + 1 as libc::c_int) as isize)).startpos)
                .offset_from(pselbuf) as libc::c_long
        }) as size_t;
        len = (*marked.offset(i as isize)).len;
        memmove(
            found as *mut libc::c_void,
            found.offset(len as isize) as *const libc::c_void,
            endpos
                .wrapping_sub(
                    found.offset(len as isize).offset_from(pselbuf) as libc::c_long
                        as libc::c_ulong,
                ),
        );
        i += 1;
        i;
    }
    free(marked as *mut libc::c_void);
    selbufpos = (selbufpos as libc::c_ulong).wrapping_sub(shrinklen) as uint_t as uint_t;
    selbufrealloc(alloclen);
    i = 0 as libc::c_int;
    while i < ndents {
        if ((*pdents.offset(i as isize)).c2rust_unnamed).flags() as libc::c_int
            & 0x10 as libc::c_int != 0
        {
            len = (pathlen as libc::c_ulong)
                .wrapping_add(
                    xstrsncpy(
                        pbuf,
                        (*pdents.offset(i as isize)).name,
                        255 as libc::c_int as size_t,
                    ),
                );
            appendfpath(g_sel.as_mut_ptr(), len);
            nselected += 1;
            nselected;
        }
        i += 1;
        i;
    }
    if nselected != 0 {
        writesel(
            pselbuf,
            selbufpos.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    } else {
        clearselection();
    };
}
unsafe extern "C" fn addtoselbuf(
    pathlen: libc::c_int,
    mut startid: libc::c_int,
    mut endid: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut alloclen: size_t = 0 as libc::c_int as size_t;
    let mut dentp: *mut entry = 0 as *mut entry;
    let mut found: *mut libc::c_char = 0 as *mut libc::c_char;
    let pbuf: *mut libc::c_char = g_sel.as_mut_ptr().offset(pathlen as isize);
    i = startid;
    while i <= endid {
        dentp = &mut *pdents.offset(i as isize) as *mut entry;
        if !findselpos.is_null() {
            len = (pathlen as libc::c_ulong)
                .wrapping_add(
                    xstrsncpy(pbuf, (*dentp).name, 255 as libc::c_int as size_t),
                );
            found = findinsel(findselpos, len as libc::c_int);
            if !found.is_null() {
                ((*dentp).c2rust_unnamed)
                    .set_flags(
                        ((*dentp).c2rust_unnamed).flags()
                            | (0x20 as libc::c_int | 0x10 as libc::c_int) as ullong_t,
                    );
                if found == findselpos {
                    findselpos = findselpos.offset(len as isize);
                    if findselpos == pselbuf.offset(selbufpos as isize) {
                        findselpos = 0 as *mut libc::c_char;
                    }
                }
            } else {
                alloclen = (alloclen as libc::c_ulong)
                    .wrapping_add(
                        (pathlen + ((*dentp).c2rust_unnamed).nlen() as libc::c_int)
                            as libc::c_ulong,
                    ) as size_t as size_t;
            }
        } else {
            alloclen = (alloclen as libc::c_ulong)
                .wrapping_add(
                    (pathlen + ((*dentp).c2rust_unnamed).nlen() as libc::c_int)
                        as libc::c_ulong,
                ) as size_t as size_t;
        }
        i += 1;
        i;
    }
    selbufrealloc(alloclen);
    i = startid;
    while i <= endid {
        if ((*pdents.offset(i as isize)).c2rust_unnamed).flags() as libc::c_int
            & 0x10 as libc::c_int == 0
        {
            len = (pathlen as libc::c_ulong)
                .wrapping_add(
                    xstrsncpy(
                        pbuf,
                        (*pdents.offset(i as isize)).name,
                        255 as libc::c_int as size_t,
                    ),
                );
            appendfpath(g_sel.as_mut_ptr(), len);
            nselected += 1;
            nselected;
            let ref mut fresh6 = (*pdents.offset(i as isize)).c2rust_unnamed;
            (*fresh6)
                .set_flags(
                    (*fresh6).flags()
                        | (0x20 as libc::c_int | 0x10 as libc::c_int) as ullong_t,
                );
        }
        i += 1;
        i;
    }
    writesel(
        pselbuf,
        selbufpos.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
}
unsafe extern "C" fn rmfromselbuf(mut len: size_t) {
    let mut found: *mut libc::c_char = findinsel(findselpos, len as libc::c_int);
    if found.is_null() {
        return;
    }
    memmove(
        found as *mut libc::c_void,
        found.offset(len as isize) as *const libc::c_void,
        (selbufpos as libc::c_long
            - found.offset(len as isize).offset_from(pselbuf) as libc::c_long)
            as libc::c_ulong,
    );
    selbufpos = (selbufpos as libc::c_ulong).wrapping_sub(len) as uint_t as uint_t;
    if nselected != 0 {
        writesel(
            pselbuf,
            selbufpos.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    } else {
        clearselection();
    };
}
unsafe extern "C" fn scanselforpath(
    mut path: *const libc::c_char,
    mut getsize: bool,
) -> libc::c_int {
    if *path.offset(1 as libc::c_int as isize) == 0 {
        g_sel[0 as libc::c_int as usize] = '/' as i32 as libc::c_char;
        findselpos = pselbuf;
        return 1 as libc::c_int;
    }
    let mut off: size_t = xstrsncpy(
        g_sel.as_mut_ptr(),
        path,
        4096 as libc::c_int as size_t,
    );
    g_sel[off.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = '/' as i32 as libc::c_char;
    findselpos = findinsel(0 as *mut libc::c_char, off as libc::c_int);
    if getsize {
        return off as libc::c_int;
    }
    return (if !findselpos.is_null() { off } else { 0 as libc::c_int as libc::c_ulong })
        as libc::c_int;
}
unsafe extern "C" fn endselection(mut endselmode: bool) {
    let mut fd: libc::c_int = 0;
    let mut count: ssize_t = 0;
    let mut buf: [libc::c_char; 4232] = [0; 4232];
    if endselmode as libc::c_int != 0 && g_state.selmode() as libc::c_int != 0 {
        g_state.set_selmode(0 as libc::c_int as uint_t);
    }
    if listpath.is_null() || selbufpos == 0 {
        return;
    }
    fd = create_tmp_file();
    if fd == -(1 as libc::c_int) {
        return;
    }
    seltofile(fd, 0 as *mut uint_t);
    if close(fd) != 0 {
        printwait(strerror(*__errno_location()), 0 as *mut libc::c_int);
        return;
    }
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4232]>() as libc::c_ulong,
        patterns[3 as libc::c_int as usize],
        listpath,
        listroot,
        g_tmpfpath.as_mut_ptr(),
    );
    spawn(
        utils[7 as libc::c_int as usize],
        buf.as_mut_ptr(),
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        (0x8 as libc::c_int | 0x1 as libc::c_int) as ushort_t,
    );
    fd = open(g_tmpfpath.as_mut_ptr(), 0 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        printwait(strerror(*__errno_location()), 0 as *mut libc::c_int);
        if unlink(g_tmpfpath.as_mut_ptr()) != 0 {
            printwait(strerror(*__errno_location()), 0 as *mut libc::c_int);
        }
        return;
    }
    count = read(fd, pselbuf as *mut libc::c_void, selbuflen as size_t);
    if count < 0 as libc::c_int as libc::c_long {
        printwait(strerror(*__errno_location()), 0 as *mut libc::c_int);
        close(fd) != 0 || unlink(g_tmpfpath.as_mut_ptr()) != 0;
        return;
    }
    if close(fd) != 0 || unlink(g_tmpfpath.as_mut_ptr()) != 0 {
        printwait(strerror(*__errno_location()), 0 as *mut libc::c_int);
        return;
    }
    selbufpos = count as uint_t;
    count -= 1;
    *pselbuf.offset(count as isize) = '\0' as i32 as libc::c_char;
    count -= 1;
    count;
    while count > 0 as libc::c_int as libc::c_long {
        if *pselbuf.offset(count as isize) as libc::c_int == '\n' as i32
            && *pselbuf.offset((count + 1 as libc::c_int as libc::c_long) as isize)
                as libc::c_int == '/' as i32
        {
            *pselbuf.offset(count as isize) = '\0' as i32 as libc::c_char;
        }
        count -= 1;
        count;
    }
    writesel(
        pselbuf,
        selbufpos.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
}
unsafe extern "C" fn editselection() -> libc::c_int {
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut fd: libc::c_int = 0;
    let mut lines: libc::c_int = 0 as libc::c_int;
    let mut count: ssize_t = 0;
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
    let mut mtime: time_t = 0;
    if selbufpos == 0 {
        return listselfile() as libc::c_int;
    }
    fd = create_tmp_file();
    if fd == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    seltofile(fd, 0 as *mut uint_t);
    if close(fd) != 0 {
        return -(1 as libc::c_int);
    }
    if stat(g_tmpfpath.as_mut_ptr(), &mut sb) != 0 {
        unlink(g_tmpfpath.as_mut_ptr());
        return -(1 as libc::c_int);
    }
    mtime = sb.st_mtim.tv_sec;
    spawn(
        if cfg.waitedit() as libc::c_int != 0 { enveditor } else { editor },
        g_tmpfpath.as_mut_ptr(),
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        (0x8 as libc::c_int | 0x1 as libc::c_int) as ushort_t,
    );
    fd = open(g_tmpfpath.as_mut_ptr(), 0 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        unlink(g_tmpfpath.as_mut_ptr());
        return -(1 as libc::c_int);
    }
    fstat(fd, &mut sb);
    if mtime == sb.st_mtim.tv_sec {
        unlink(g_tmpfpath.as_mut_ptr());
        return 1 as libc::c_int;
    }
    if sb.st_size > selbufpos as libc::c_long {
        unlink(g_tmpfpath.as_mut_ptr());
    } else {
        count = read(fd, pselbuf as *mut libc::c_void, selbuflen as size_t);
        if count < 0 as libc::c_int as libc::c_long {
            printwait(strerror(*__errno_location()), 0 as *mut libc::c_int);
            if close(fd) != 0 || unlink(g_tmpfpath.as_mut_ptr()) != 0 {
                printwait(strerror(*__errno_location()), 0 as *mut libc::c_int);
            }
        } else if close(fd) != 0 || unlink(g_tmpfpath.as_mut_ptr()) != 0 {
            printwait(strerror(*__errno_location()), 0 as *mut libc::c_int);
        } else if count == 0 {
            ret = 1 as libc::c_int;
        } else {
            resetselind();
            selbufpos = count as uint_t;
            count -= 1;
            *pselbuf.offset(count as isize) = '\0' as i32 as libc::c_char;
            count -= 1;
            count;
            while count > 0 as libc::c_int as libc::c_long {
                if *pselbuf.offset(count as isize) as libc::c_int == '\n' as i32
                    && *pselbuf
                        .offset((count + 1 as libc::c_int as libc::c_long) as isize)
                        as libc::c_int == '/' as i32
                {
                    lines += 1;
                    lines;
                    *pselbuf.offset(count as isize) = '\0' as i32 as libc::c_char;
                }
                count -= 1;
                count;
            }
            lines += 1;
            lines;
            if !(lines > nselected) {
                nselected = lines;
                writesel(
                    pselbuf,
                    selbufpos.wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
                return 1 as libc::c_int;
            }
        }
    }
    resetselind();
    clearselection();
    return ret;
}
unsafe extern "C" fn selsafe() -> bool {
    if selpath.is_null() {
        printmsg(messages[23 as libc::c_int as usize]);
        return 0 as libc::c_int != 0;
    }
    if access(selpath, 4 as libc::c_int | 2 as libc::c_int) == -(1 as libc::c_int) {
        if *__errno_location() == 2 as libc::c_int {
            printmsg(messages[3 as libc::c_int as usize]);
        } else {
            printwait(strerror(*__errno_location()), 0 as *mut libc::c_int);
        };
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn export_file_list() {
    if ndents == 0 {
        return;
    }
    let mut pdent: *mut entry = pdents;
    let mut fd: libc::c_int = create_tmp_file();
    if fd == -(1 as libc::c_int) {
        return;
    }
    let mut r: libc::c_int = 0 as libc::c_int;
    while r < ndents {
        if write(
            fd,
            (*pdent).name as *const libc::c_void,
            (((*pdent).c2rust_unnamed).nlen() as libc::c_int - 1 as libc::c_int)
                as size_t,
        )
            != (((*pdent).c2rust_unnamed).nlen() as libc::c_int - 1 as libc::c_int)
                as libc::c_long
        {
            break;
        }
        if r != ndents - 1 as libc::c_int
            && write(
                fd,
                b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            ) != 1 as libc::c_int as libc::c_long
        {
            break;
        }
        pdent = pdent.offset(1);
        pdent;
        r += 1;
        r;
    }
    close(fd) != 0;
    spawn(
        editor,
        g_tmpfpath.as_mut_ptr(),
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        (0x8 as libc::c_int | 0x1 as libc::c_int) as ushort_t,
    );
    if xconfirm(get_input(messages[39 as libc::c_int as usize])) {
        unlink(g_tmpfpath.as_mut_ptr());
    }
}
unsafe extern "C" fn init_fcolors() -> bool {
    let mut f_colors: *mut libc::c_char = getenv(env_cfg[5 as libc::c_int as usize]);
    if f_colors.is_null() || *f_colors == 0 {
        f_colors = gcolors.as_mut_ptr();
    }
    let mut id: uchar_t = (4 as libc::c_int + 1 as libc::c_int) as uchar_t;
    while *f_colors as libc::c_int != 0
        && id as libc::c_int
            <= 4 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
    {
        fcolors[id
            as usize] = ((xchartohex(*f_colors as uchar_t) as libc::c_int)
            << 4 as libc::c_int) as uint_t;
        f_colors = f_colors.offset(1);
        if *f_colors != 0 {
            fcolors[id
                as usize] = (fcolors[id as usize] as libc::c_uint)
                .wrapping_add(xchartohex(*f_colors as uchar_t) as libc::c_uint) as uint_t
                as uint_t;
            if fcolors[id as usize] != 0 {
                init_pair(
                    id as libc::c_short,
                    fcolors[id as usize] as libc::c_short,
                    -(1 as libc::c_int) as libc::c_short,
                );
            }
        } else {
            return 0 as libc::c_int != 0
        }
        f_colors = f_colors.offset(1);
        f_colors;
        id = id.wrapping_add(1);
        id;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn initcurses(mut oldmask: *mut libc::c_void) -> bool {
    if g_state.picker() != 0 {
        if (newterm(0 as *const libc::c_char, stderr, stdin)).is_null() {
            msg(b"newterm!\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int != 0;
        }
    } else if (initscr()).is_null() {
        msg(b"initscr!\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int != 0;
    }
    cbreak();
    noecho();
    nonl();
    keypad(stdscr, 1 as libc::c_int != 0);
    mousemask(
        ((0o2 as libc::c_long)
            << (1 as libc::c_int - 1 as libc::c_int) * 5 as libc::c_int
            | (0o2 as libc::c_long)
                << (2 as libc::c_int - 1 as libc::c_int) * 5 as libc::c_int
            | (0o2 as libc::c_long)
                << (3 as libc::c_int - 1 as libc::c_int) * 5 as libc::c_int
            | (0o2 as libc::c_long)
                << (4 as libc::c_int - 1 as libc::c_int) * 5 as libc::c_int
            | (0o2 as libc::c_long)
                << (5 as libc::c_int - 1 as libc::c_int) * 5 as libc::c_int) as mmask_t,
        oldmask as *mut mmask_t,
    );
    mouseinterval(0 as libc::c_int);
    curs_set(0 as libc::c_int);
    let mut colors: *mut libc::c_char = getenv(env_cfg[4 as libc::c_int as usize]);
    if !colors.is_null()
        || (getenv(b"NO_COLOR\0" as *const u8 as *const libc::c_char)).is_null()
    {
        let mut pcode: *mut uint_t = 0 as *mut uint_t;
        let mut ext: bool = 0 as libc::c_int != 0;
        start_color();
        use_default_colors();
        if COLORS >= 256 as libc::c_int {
            if !(g_state.oldcolor() as libc::c_int != 0
                || init_fcolors() as libc::c_int != 0)
            {
                endwin();
                msg(env_cfg[5 as libc::c_int as usize]);
                return 0 as libc::c_int != 0;
            }
        } else {
            g_state.set_oldcolor(1 as libc::c_int as uint_t);
        }
        if !colors.is_null() && *colors as libc::c_int == '#' as i32 {
            let mut sep: *mut libc::c_char = strchr(colors, ';' as i32);
            if g_state.oldcolor() == 0 && COLORS >= 256 as libc::c_int {
                colors = colors.offset(1);
                colors;
                ext = 1 as libc::c_int != 0;
                if !sep.is_null() {
                    *sep = '\0' as i32 as libc::c_char;
                }
            } else {
                colors = sep;
                if !colors.is_null() {
                    colors = colors.offset(1);
                    colors;
                }
            }
        }
        let mut i: uchar_t = 0 as libc::c_int as uchar_t;
        while (i as libc::c_int) < 4 as libc::c_int {
            pcode = &mut (*g_ctx.as_mut_ptr().offset(i as isize)).color;
            if !colors.is_null() && *colors as libc::c_int != 0 {
                if ext {
                    *pcode = ((xchartohex(*colors as uchar_t) as libc::c_int)
                        << 4 as libc::c_int) as uint_t;
                    colors = colors.offset(1);
                    if *colors != 0 {
                        *pcode = (*pcode as libc::c_uint)
                            .wrapping_add(xchartohex(*colors as uchar_t) as libc::c_uint)
                            as uint_t as uint_t;
                        fcolors[(i as libc::c_int + 1 as libc::c_int) as usize] = *pcode;
                    } else {
                        endwin();
                        msg(env_cfg[4 as libc::c_int as usize]);
                        return 0 as libc::c_int != 0;
                    }
                } else {
                    *pcode = (if (*colors as libc::c_int) < '0' as i32
                        || *colors as libc::c_int > '7' as i32
                    {
                        4 as libc::c_int
                    } else {
                        *colors as libc::c_int - '0' as i32
                    }) as uint_t;
                }
                colors = colors.offset(1);
                colors;
            } else {
                *pcode = 4 as libc::c_int as uint_t;
            }
            init_pair(
                (i as libc::c_int + 1 as libc::c_int) as libc::c_short,
                *pcode as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
            );
            i = i.wrapping_add(1);
            i;
        }
    }
    wtimeout(stdscr, 1000 as libc::c_int);
    set_escdelay(25 as libc::c_int);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parseargs(
    mut cmd: *mut libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut pindex: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut len: size_t = (xstrlen(cmd)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut line: *mut libc::c_char = malloc(len) as *mut libc::c_char;
    if line.is_null() {
        return 0 as *mut libc::c_char;
    }
    xstrsncpy(line, cmd, len);
    let fresh7 = count;
    count = count + 1;
    let ref mut fresh8 = *argv.offset(fresh7 as isize);
    *fresh8 = line;
    cmd = line;
    while *line != 0 {
        if *line as libc::c_int == ' ' as i32 || *line as libc::c_int == '\t' as i32 {
            let fresh9 = line;
            line = line.offset(1);
            *fresh9 = '\0' as i32 as libc::c_char;
            if *line == 0 {
                break;
            }
            let fresh10 = count;
            count = count + 1;
            let ref mut fresh11 = *argv.offset(fresh10 as isize);
            *fresh11 = line;
            if count == 10 as libc::c_int {
                count = -(1 as libc::c_int);
                break;
            }
        }
        line = line.offset(1);
        line;
    }
    if count == -(1 as libc::c_int) || count > 10 as libc::c_int - 4 as libc::c_int {
        free(cmd as *mut libc::c_void);
        cmd = 0 as *mut libc::c_char;
    }
    *pindex = count;
    return cmd;
}
unsafe extern "C" fn enable_signals() {
    let mut dfl_act: sigaction = {
        let mut init = sigaction {
            __sigaction_handler: C2RustUnnamed_10 {
                sa_handler: None,
            },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        init
    };
    sigaction(1 as libc::c_int, &mut dfl_act, 0 as *mut sigaction);
    sigaction(2 as libc::c_int, &mut dfl_act, 0 as *mut sigaction);
    sigaction(3 as libc::c_int, &mut dfl_act, 0 as *mut sigaction);
    sigaction(20 as libc::c_int, &mut dfl_act, 0 as *mut sigaction);
    sigaction(28 as libc::c_int, &mut dfl_act, 0 as *mut sigaction);
}
unsafe extern "C" fn xfork(mut flag: uchar_t) -> pid_t {
    let mut p: pid_t = fork();
    if p > 0 as libc::c_int {
        sigaction(
            1 as libc::c_int,
            &mut {
                let mut init = sigaction {
                    __sigaction_handler: C2RustUnnamed_10 {
                        sa_handler: ::std::mem::transmute::<
                            libc::intptr_t,
                            __sighandler_t,
                        >(1 as libc::c_int as libc::intptr_t),
                    },
                    sa_mask: __sigset_t { __val: [0; 16] },
                    sa_flags: 0,
                    sa_restorer: None,
                };
                init
            },
            &mut oldsighup,
        );
        sigaction(
            20 as libc::c_int,
            &mut {
                let mut init = sigaction {
                    __sigaction_handler: C2RustUnnamed_10 {
                        sa_handler: None,
                    },
                    sa_mask: __sigset_t { __val: [0; 16] },
                    sa_flags: 0,
                    sa_restorer: None,
                };
                init
            },
            &mut oldsigtstp,
        );
        sigaction(
            28 as libc::c_int,
            &mut {
                let mut init = sigaction {
                    __sigaction_handler: C2RustUnnamed_10 {
                        sa_handler: ::std::mem::transmute::<
                            libc::intptr_t,
                            __sighandler_t,
                        >(1 as libc::c_int as libc::intptr_t),
                    },
                    sa_mask: __sigset_t { __val: [0; 16] },
                    sa_flags: 0,
                    sa_restorer: None,
                };
                init
            },
            &mut oldsigwinch,
        );
    } else if p == 0 as libc::c_int {
        if flag as libc::c_int & 0x2 as libc::c_int != 0 {
            p = fork();
            if p > 0 as libc::c_int {
                _exit(0 as libc::c_int);
            } else if p == 0 as libc::c_int {
                enable_signals();
                setsid();
                return p;
            }
            perror(b"fork\0" as *const u8 as *const libc::c_char);
            _exit(1 as libc::c_int);
        }
        enable_signals();
    }
    if flag as libc::c_int & 0x2 as libc::c_int != 0 {
        waitpid(p, 0 as *mut libc::c_int, 0 as libc::c_int);
    }
    if p == -(1 as libc::c_int) {
        perror(b"fork\0" as *const u8 as *const libc::c_char);
    }
    return p;
}
unsafe extern "C" fn join(mut p: pid_t, mut flag: uchar_t) -> libc::c_int {
    let mut status: libc::c_int = 0xffff as libc::c_int;
    if flag as libc::c_int & 0x2 as libc::c_int == 0 {
        while waitpid(p, &mut status, 0 as libc::c_int) == -(1 as libc::c_int) {}
        if status & 0x7f as libc::c_int == 0 as libc::c_int {
            status = (status & 0xff00 as libc::c_int) >> 8 as libc::c_int;
        }
    }
    sigaction(1 as libc::c_int, &mut oldsighup, 0 as *mut sigaction);
    sigaction(20 as libc::c_int, &mut oldsigtstp, 0 as *mut sigaction);
    sigaction(28 as libc::c_int, &mut oldsigwinch, 0 as *mut sigaction);
    return status;
}
unsafe extern "C" fn spawn(
    mut file: *mut libc::c_char,
    mut arg1: *mut libc::c_char,
    mut arg2: *mut libc::c_char,
    mut arg3: *mut libc::c_char,
    mut flag: ushort_t,
) -> libc::c_int {
    let mut pid: pid_t = 0;
    let mut status: libc::c_int = 0 as libc::c_int;
    let mut retstatus: libc::c_int = 0xffff as libc::c_int;
    let mut argv: [*mut libc::c_char; 10] = [
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
    ];
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    if file.is_null() || *file == 0 {
        return retstatus;
    }
    if arg1.is_null() && !arg2.is_null() {
        arg1 = arg2;
        if !arg3.is_null() {
            arg2 = arg3;
            arg3 = 0 as *mut libc::c_char;
        } else {
            arg2 = 0 as *mut libc::c_char;
        }
    }
    if flag as libc::c_int & 0x1 as libc::c_int != 0 {
        cmd = parseargs(file, argv.as_mut_ptr(), &mut status);
        if cmd.is_null() {
            return -(1 as libc::c_int);
        }
    } else {
        let fresh12 = status;
        status = status + 1;
        argv[fresh12 as usize] = file;
    }
    argv[status as usize] = arg1;
    status += 1;
    argv[status as usize] = arg2;
    status += 1;
    argv[status as usize] = arg3;
    if flag as libc::c_int & 0x8 as libc::c_int != 0 {
        endwin();
    }
    pid = xfork(flag as uchar_t);
    if pid == 0 as libc::c_int {
        if flag as libc::c_int & 0x4 as libc::c_int != 0 {
            let mut fd: libc::c_int = open(
                b"/dev/null\0" as *const u8 as *const libc::c_char,
                0o1 as libc::c_int,
                0o200 as libc::c_int,
            );
            if flag as libc::c_int & 0x40 as libc::c_int != 0 {
                dup2(fd, 0 as libc::c_int);
            }
            dup2(fd, 1 as libc::c_int);
            dup2(fd, 2 as libc::c_int);
            close(fd);
        } else if flag as libc::c_int & 0x100 as libc::c_int != 0 {
            if isatty(1 as libc::c_int) == 0 {
                let mut fd_0: libc::c_int = open(
                    ctermid(0 as *mut libc::c_char),
                    0o1 as libc::c_int,
                    0o200 as libc::c_int,
                );
                dup2(fd_0, 1 as libc::c_int);
                close(fd_0);
            }
        }
        execvp(*argv.as_mut_ptr(), argv.as_mut_ptr() as *const *mut libc::c_char);
        _exit(0 as libc::c_int);
    } else {
        retstatus = join(pid, flag as uchar_t);
        if flag as libc::c_int & 0x10 as libc::c_int != 0
            || flag as libc::c_int & 0x20 as libc::c_int != 0 && retstatus != 0
        {
            status = write(
                1 as libc::c_int,
                messages[22 as libc::c_int as usize] as *const libc::c_void,
                xstrlen(messages[22 as libc::c_int as usize]),
            ) as libc::c_int;
            while read(
                0 as libc::c_int,
                &mut status as *mut libc::c_int as *mut libc::c_void,
                1 as libc::c_int as size_t,
            ) > 0 as libc::c_int as libc::c_long && status != '\n' as i32
            {}
        }
        if flag as libc::c_int & 0x8 as libc::c_int != 0 {
            wrefresh(stdscr);
        }
        free(cmd as *mut libc::c_void);
    }
    return retstatus;
}
unsafe extern "C" fn xgetenv(
    name: *const libc::c_char,
    mut fallback: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut value: *mut libc::c_char = getenv(name);
    return if !value.is_null()
        && *value.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        value
    } else {
        fallback
    };
}
#[inline]
unsafe extern "C" fn xgetenv_val(mut name: *const libc::c_char) -> uint_t {
    let mut str: *mut libc::c_char = getenv(name);
    if !str.is_null() && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        return atoi(str) as uint_t;
    }
    return 0 as libc::c_int as uint_t;
}
unsafe extern "C" fn xdiraccess(mut path: *const libc::c_char) -> bool {
    let mut dirp: *mut DIR = opendir(path);
    if dirp.is_null() {
        printwait(strerror(*__errno_location()), 0 as *mut libc::c_int);
        return 0 as libc::c_int != 0;
    }
    closedir(dirp);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn plugscript(
    mut plugin: *const libc::c_char,
    mut flags: uchar_t,
) -> bool {
    mkpath(plgpath, plugin, g_buf.as_mut_ptr());
    if access(g_buf.as_mut_ptr(), 1 as libc::c_int) == 0 {
        spawn(
            g_buf.as_mut_ptr(),
            0 as *mut libc::c_char,
            0 as *mut libc::c_char,
            0 as *mut libc::c_char,
            flags as ushort_t,
        );
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn opstr(mut buf: *mut libc::c_char, mut op: *mut libc::c_char) {
    snprintf(
        buf,
        (4096 as libc::c_int
            + ((255 as libc::c_int + 1 as libc::c_int) << 1 as libc::c_int))
            as libc::c_ulong,
        b"xargs -0 sh -c '%s \"$0\" \"$@\" . < /dev/tty' < %s\0" as *const u8
            as *const libc::c_char,
        op,
        selpath,
    );
}
unsafe extern "C" fn rmmulstr(mut buf: *mut libc::c_char) -> bool {
    let mut r: libc::c_char = confirm_force(1 as libc::c_int != 0);
    if r == 0 {
        return 0 as libc::c_int != 0;
    }
    if g_state.trash() == 0 {
        snprintf(
            buf,
            (4096 as libc::c_int
                + ((255 as libc::c_int + 1 as libc::c_int) << 1 as libc::c_int))
                as libc::c_ulong,
            b"xargs -0 sh -c 'rm -%cr \"$0\" \"$@\" < /dev/tty' < %s\0" as *const u8
                as *const libc::c_char,
            r as libc::c_int,
            selpath,
        );
    } else {
        snprintf(
            buf,
            (4096 as libc::c_int
                + ((255 as libc::c_int + 1 as libc::c_int) << 1 as libc::c_int))
                as libc::c_ulong,
            b"xargs -0 %s < %s\0" as *const u8 as *const libc::c_char,
            utils[(if g_state.trash() as libc::c_int == 1 as libc::c_int {
                18 as libc::c_int
            } else {
                19 as libc::c_int
            }) as usize],
            selpath,
        );
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn xrm(fpath: *mut libc::c_char) -> bool {
    let mut r: libc::c_char = confirm_force(0 as libc::c_int != 0);
    if r == 0 {
        return 0 as libc::c_int != 0;
    }
    if g_state.trash() == 0 {
        let mut rm_opts: [libc::c_char; 4] = *::std::mem::transmute::<
            &[u8; 4],
            &mut [libc::c_char; 4],
        >(b"-ir\0");
        rm_opts[1 as libc::c_int as usize] = r;
        spawn(
            b"rm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            rm_opts.as_mut_ptr(),
            fpath,
            0 as *mut libc::c_char,
            (0x8 as libc::c_int | 0x20 as libc::c_int) as ushort_t,
        );
    } else {
        spawn(
            utils[(if g_state.trash() as libc::c_int == 1 as libc::c_int {
                18 as libc::c_int
            } else {
                19 as libc::c_int
            }) as usize],
            fpath,
            0 as *mut libc::c_char,
            0 as *mut libc::c_char,
            (0x8 as libc::c_int | 0x1 as libc::c_int) as ushort_t,
        );
    }
    return access(fpath, 0 as libc::c_int) == -(1 as libc::c_int);
}
unsafe extern "C" fn xrmfromsel(
    mut path: *mut libc::c_char,
    mut fpath: *mut libc::c_char,
) {
    let mut selected: bool = 1 as libc::c_int != 0;
    if ((*pdents.offset(cur as isize)).c2rust_unnamed).flags() as libc::c_int
        & 0x1 as libc::c_int != 0 && scanselforpath(fpath, 0 as libc::c_int != 0) != 0
    {
        clearselection();
    } else if ((*pdents.offset(cur as isize)).c2rust_unnamed).flags() as libc::c_int
        & 0x10 as libc::c_int != 0
    {
        nselected -= 1;
        nselected;
        rmfromselbuf(
            mkpath(path, (*pdents.offset(cur as isize)).name, g_sel.as_mut_ptr()),
        );
    } else {
        selected = 0 as libc::c_int != 0;
    }
    if selected as libc::c_int != 0 && cfg.x11() as libc::c_int != 0 {
        plugscript(
            utils[16 as libc::c_int as usize],
            (0x2 as libc::c_int | 0x4 as libc::c_int) as uchar_t,
        );
    }
}
unsafe extern "C" fn lines_in_file(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut buflen: size_t,
) -> uint_t {
    let mut len: ssize_t = 0;
    let mut count: uint_t = 0 as libc::c_int as uint_t;
    loop {
        len = read(fd, buf as *mut libc::c_void, buflen);
        if !(len > 0 as libc::c_int as libc::c_long) {
            break;
        }
        while len != 0 {
            len -= 1;
            count = (count as libc::c_uint)
                .wrapping_add(
                    (*buf.offset(len as isize) as libc::c_int == '\n' as i32)
                        as libc::c_int as libc::c_uint,
                ) as uint_t as uint_t;
        }
    }
    return if len < 0 as libc::c_int as libc::c_long {
        0 as libc::c_int as libc::c_uint
    } else {
        count
    };
}
unsafe extern "C" fn cpmv_rename(
    mut choice: libc::c_int,
    mut path: *const libc::c_char,
) -> bool {
    let mut current_block: u64;
    let mut fd: libc::c_int = 0;
    let mut count: uint_t = 0 as libc::c_int as uint_t;
    let mut lines: uint_t = 0 as libc::c_int as uint_t;
    let mut ret: bool = 0 as libc::c_int != 0;
    let mut cmd: *mut libc::c_char = if choice == 'c' as i32 {
        cp.as_mut_ptr()
    } else {
        mv.as_mut_ptr()
    };
    let mut buf: [libc::c_char; 8210] = [0; 8210];
    fd = create_tmp_file();
    if fd == -(1 as libc::c_int) {
        return ret;
    }
    if selbufpos == 0 {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8210]>() as libc::c_ulong,
            b"tr '\\0' '\\n' < %s > %s\0" as *const u8 as *const libc::c_char,
            selpath,
            g_tmpfpath.as_mut_ptr(),
        );
        spawn(
            utils[7 as libc::c_int as usize],
            buf.as_mut_ptr(),
            0 as *mut libc::c_char,
            0 as *mut libc::c_char,
            (0x8 as libc::c_int | 0x1 as libc::c_int) as ushort_t,
        );
        count = lines_in_file(
            fd,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8210]>() as libc::c_ulong,
        );
        if count == 0 {
            current_block = 8056199966157947041;
        } else {
            current_block = 13513818773234778473;
        }
    } else {
        seltofile(fd, &mut count);
        current_block = 13513818773234778473;
    }
    match current_block {
        13513818773234778473 => {
            close(fd);
            snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8210]>() as libc::c_ulong,
                patterns[0 as libc::c_int as usize],
                g_tmpfpath.as_mut_ptr(),
            );
            spawn(
                utils[7 as libc::c_int as usize],
                buf.as_mut_ptr(),
                0 as *mut libc::c_char,
                0 as *mut libc::c_char,
                (0x8 as libc::c_int | 0x1 as libc::c_int) as ushort_t,
            );
            spawn(
                if cfg.waitedit() as libc::c_int != 0 { enveditor } else { editor },
                g_tmpfpath.as_mut_ptr(),
                0 as *mut libc::c_char,
                0 as *mut libc::c_char,
                (0x8 as libc::c_int | 0x1 as libc::c_int) as ushort_t,
            );
            fd = open(g_tmpfpath.as_mut_ptr(), 0 as libc::c_int);
            if !(fd == -(1 as libc::c_int)) {
                lines = lines_in_file(
                    fd,
                    buf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 8210]>() as libc::c_ulong,
                );
                if !(lines == 0
                    || (2 as libc::c_int as libc::c_uint).wrapping_mul(count) != lines)
                {
                    snprintf(
                        buf.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 8210]>() as libc::c_ulong,
                        patterns[1 as libc::c_int as usize],
                        path,
                        g_tmpfpath.as_mut_ptr(),
                        cmd,
                    );
                    if spawn(
                        utils[7 as libc::c_int as usize],
                        buf.as_mut_ptr(),
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        (0x8 as libc::c_int | 0x1 as libc::c_int | 0x20 as libc::c_int)
                            as ushort_t,
                    ) == 0
                    {
                        ret = 1 as libc::c_int != 0;
                    }
                }
            }
        }
        _ => {}
    }
    if fd >= 0 as libc::c_int {
        close(fd);
    }
    return ret;
}
unsafe extern "C" fn cpmvrm_selection(
    mut sel: action,
    mut path: *mut libc::c_char,
) -> bool {
    let mut r: libc::c_int = 0;
    if isselfileempty() {
        if nselected != 0 {
            clearselection();
        }
        printmsg(messages[3 as libc::c_int as usize]);
        return 0 as libc::c_int != 0;
    }
    if !selsafe() {
        return 0 as libc::c_int != 0;
    }
    match sel as libc::c_uint {
        42 => {
            opstr(g_buf.as_mut_ptr(), cp.as_mut_ptr());
        }
        43 => {
            opstr(g_buf.as_mut_ptr(), mv.as_mut_ptr());
        }
        44 => {
            r = get_input(messages[7 as libc::c_int as usize]);
            if r != 'c' as i32 && r != 'm' as i32 {
                printmsg(messages[40 as libc::c_int as usize]);
                return 0 as libc::c_int != 0;
            }
            if !cpmv_rename(r, path) {
                printmsg(messages[5 as libc::c_int as usize]);
                return 0 as libc::c_int != 0;
            }
        }
        _ => {
            if !rmmulstr(g_buf.as_mut_ptr()) {
                printmsg(messages[4 as libc::c_int as usize]);
                return 0 as libc::c_int != 0;
            }
        }
    }
    if sel as libc::c_uint != SEL_CPMVAS as libc::c_int as libc::c_uint
        && spawn(
            utils[7 as libc::c_int as usize],
            g_buf.as_mut_ptr(),
            0 as *mut libc::c_char,
            0 as *mut libc::c_char,
            (0x8 as libc::c_int | 0x1 as libc::c_int | 0x20 as libc::c_int) as ushort_t,
        ) != 0
    {
        printmsg(messages[5 as libc::c_int as usize]);
        return 0 as libc::c_int != 0;
    }
    clearselection();
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn batch_rename() -> bool {
    let mut fd1: libc::c_int = 0;
    let mut fd2: libc::c_int = 0;
    let mut count: uint_t = 0 as libc::c_int as uint_t;
    let mut lines: uint_t = 0 as libc::c_int as uint_t;
    let mut dir: bool = 0 as libc::c_int != 0;
    let mut ret: bool = 0 as libc::c_int != 0;
    let mut foriginal: [libc::c_char; 64] = [
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
    ];
    static mut batchrenamecmd: [libc::c_char; 112] = unsafe {
        *::std::mem::transmute::<
            &[u8; 112],
            &[libc::c_char; 112],
        >(
            b"paste -d'\n' %s %s | sed 'N; /^\\(.*\\)\\n\\1$/!p;d' | tr '\n' '\\0' | xargs -0 -n2 sh -c 'mv -i \"$0\" \"$@\" < /dev/tty'\0",
        )
    };
    let mut buf: [libc::c_char; 8304] = [0; 8304];
    let mut i: libc::c_int = get_cur_or_sel();
    if i == 0 {
        return ret;
    }
    if i == 'c' as i32 {
        selbufpos = 0 as libc::c_int as uint_t;
        dir = 1 as libc::c_int != 0;
    }
    fd1 = create_tmp_file();
    if fd1 == -(1 as libc::c_int) {
        return ret;
    }
    xstrsncpy(
        foriginal.as_mut_ptr(),
        g_tmpfpath.as_mut_ptr(),
        (xstrlen(g_tmpfpath.as_mut_ptr()))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    fd2 = create_tmp_file();
    if fd2 == -(1 as libc::c_int) {
        unlink(foriginal.as_mut_ptr());
        close(fd1);
        return ret;
    }
    if dir {
        i = 0 as libc::c_int;
        while i < ndents {
            appendfpath((*pdents.offset(i as isize)).name, 255 as libc::c_int as size_t);
            i += 1;
            i;
        }
    }
    seltofile(fd1, &mut count);
    seltofile(fd2, 0 as *mut uint_t);
    close(fd2);
    if dir {
        selbufpos = 0 as libc::c_int as uint_t;
    }
    spawn(
        if cfg.waitedit() as libc::c_int != 0 { enveditor } else { editor },
        g_tmpfpath.as_mut_ptr(),
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        (0x8 as libc::c_int | 0x1 as libc::c_int) as ushort_t,
    );
    fd2 = open(g_tmpfpath.as_mut_ptr(), 0 as libc::c_int);
    if !(fd2 == -(1 as libc::c_int)) {
        lines = lines_in_file(
            fd2,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8304]>() as libc::c_ulong,
        );
        if !(lines == 0 || count != lines) {
            snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8304]>() as libc::c_ulong,
                batchrenamecmd.as_ptr(),
                foriginal.as_mut_ptr(),
                g_tmpfpath.as_mut_ptr(),
            );
            spawn(
                utils[7 as libc::c_int as usize],
                buf.as_mut_ptr(),
                0 as *mut libc::c_char,
                0 as *mut libc::c_char,
                (0x8 as libc::c_int | 0x1 as libc::c_int) as ushort_t,
            );
            ret = 1 as libc::c_int != 0;
        }
    }
    if fd1 >= 0 as libc::c_int {
        close(fd1);
    }
    unlink(foriginal.as_mut_ptr());
    if fd2 >= 0 as libc::c_int {
        close(fd2);
    }
    unlink(g_tmpfpath.as_mut_ptr());
    return ret;
}
unsafe extern "C" fn get_archive_cmd(
    mut cmd: *mut libc::c_char,
    mut archive: *const libc::c_char,
) {
    let mut i: uchar_t = 3 as libc::c_int as uchar_t;
    if g_state.usebsdtar() == 0
        && getutil(utils[1 as libc::c_int as usize]) as libc::c_int != 0
    {
        i = 0 as libc::c_int as uchar_t;
    } else if getutil(utils[2 as libc::c_int as usize]) {
        i = 1 as libc::c_int as uchar_t;
    } else if is_suffix(archive, b".zip\0" as *const u8 as *const libc::c_char) {
        i = 2 as libc::c_int as uchar_t;
    }
    xstrsncpy(cmd, archive_cmd[i as usize], 16 as libc::c_int as size_t);
}
unsafe extern "C" fn archive_selection(
    mut cmd: *const libc::c_char,
    mut archive: *const libc::c_char,
    mut curpath: *const libc::c_char,
) {
    let mut buf: *mut libc::c_char = malloc(
        (xstrlen(patterns[4 as libc::c_int as usize]))
            .wrapping_add(xstrlen(cmd))
            .wrapping_add(xstrlen(archive))
            .wrapping_add(xstrlen(curpath))
            .wrapping_add(xstrlen(selpath))
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    if buf.is_null() {
        printwait(strerror(*__errno_location()), 0 as *mut libc::c_int);
        return;
    }
    snprintf(
        buf,
        (4096 as libc::c_int
            + ((255 as libc::c_int + 1 as libc::c_int) << 1 as libc::c_int))
            as libc::c_ulong,
        patterns[4 as libc::c_int as usize],
        curpath,
        selpath,
        cmd,
        archive,
    );
    spawn(
        utils[7 as libc::c_int as usize],
        buf,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        (0x8 as libc::c_int | 0x1 as libc::c_int | 0x10 as libc::c_int) as ushort_t,
    );
    free(buf as *mut libc::c_void);
}
unsafe extern "C" fn write_lastdir(
    mut curpath: *const libc::c_char,
    mut outfile: *const libc::c_char,
) {
    if outfile.is_null() {
        xstrsncpy(
            cfgpath.offset(xstrlen(cfgpath) as isize),
            b"/.lastd\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as size_t,
        );
    } else {
        convert_tilde(outfile, g_buf.as_mut_ptr());
    }
    let mut fd: libc::c_int = open(
        if !outfile.is_null() {
            if *outfile.offset(0 as libc::c_int as isize) as libc::c_int == '~' as i32 {
                g_buf.as_mut_ptr() as *const libc::c_char
            } else {
                outfile
            }
        } else {
            cfgpath as *const libc::c_char
        },
        0o100 as libc::c_int | 0o1 as libc::c_int | 0o1000 as libc::c_int,
        0o666 as libc::c_int,
    );
    if fd != -(1 as libc::c_int) {
        dprintf(fd, b"cd \"%s\"\0" as *const u8 as *const libc::c_char, curpath);
        close(fd);
    }
}
unsafe extern "C" fn xstricmp(
    s1: *const libc::c_char,
    s2: *const libc::c_char,
) -> libc::c_int {
    let mut p1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v1: libc::c_longlong = strtoll(s1, &mut p1, 10 as libc::c_int);
    let mut v2: libc::c_longlong = strtoll(s2, &mut p2, 10 as libc::c_int);
    if s1 != p1 as *const libc::c_char || s2 != p2 as *const libc::c_char {
        if s1 != p1 as *const libc::c_char && s2 != p2 as *const libc::c_char {
            if v2 > v1 {
                return -(1 as libc::c_int);
            }
            if v1 > v2 {
                return 1 as libc::c_int;
            }
        }
        if s1 == p1 as *const libc::c_char {
            return 1 as libc::c_int;
        }
        if s2 == p2 as *const libc::c_char {
            return -(1 as libc::c_int);
        }
    }
    return strcoll(s1, s2);
}
unsafe extern "C" fn xstrverscasecmp(
    s1: *const libc::c_char,
    s2: *const libc::c_char,
) -> libc::c_int {
    let mut p1: *const uchar_t = s1 as *const uchar_t;
    let mut p2: *const uchar_t = s2 as *const uchar_t;
    let mut state: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut c1: uchar_t = 0;
    let mut c2: uchar_t = 0;
    static mut next_state: [uint8_t; 12] = [
        0 as libc::c_int as uint8_t,
        0x3 as libc::c_int as uint8_t,
        0x9 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0x3 as libc::c_int as uint8_t,
        0x3 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0x6 as libc::c_int as uint8_t,
        0x6 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0x6 as libc::c_int as uint8_t,
        0x9 as libc::c_int as uint8_t,
    ];
    static mut result_type: [int8_t; 36] = [
        2 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        3 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        -(1 as libc::c_int) as int8_t,
        -(1 as libc::c_int) as int8_t,
        1 as libc::c_int as int8_t,
        3 as libc::c_int as int8_t,
        3 as libc::c_int as int8_t,
        1 as libc::c_int as int8_t,
        3 as libc::c_int as int8_t,
        3 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        1 as libc::c_int as int8_t,
        1 as libc::c_int as int8_t,
        -(1 as libc::c_int) as int8_t,
        2 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        -(1 as libc::c_int) as int8_t,
        2 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
    ];
    if p1 == p2 {
        return 0 as libc::c_int;
    }
    c1 = (if *p1 as libc::c_int >= 'a' as i32 && *p1 as libc::c_int <= 'z' as i32 {
        *p1 as libc::c_int - 'a' as i32 + 'A' as i32
    } else {
        *p1 as libc::c_int
    }) as uchar_t;
    p1 = p1.offset(1);
    p1;
    c2 = (if *p2 as libc::c_int >= 'a' as i32 && *p2 as libc::c_int <= 'z' as i32 {
        *p2 as libc::c_int - 'a' as i32 + 'A' as i32
    } else {
        *p2 as libc::c_int
    }) as uchar_t;
    p2 = p2.offset(1);
    p2;
    state = 0 as libc::c_int
        + ((c1 as libc::c_int == '0' as i32) as libc::c_int
            + (((c1 as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint) as libc::c_int != 0 as libc::c_int)
                as libc::c_int);
    loop {
        diff = c1 as libc::c_int - c2 as libc::c_int;
        if !(diff == 0 as libc::c_int) {
            break;
        }
        if c1 as libc::c_int == '\0' as i32 {
            return diff;
        }
        state = next_state[state as usize] as libc::c_int;
        c1 = (if *p1 as libc::c_int >= 'a' as i32 && *p1 as libc::c_int <= 'z' as i32 {
            *p1 as libc::c_int - 'a' as i32 + 'A' as i32
        } else {
            *p1 as libc::c_int
        }) as uchar_t;
        p1 = p1.offset(1);
        p1;
        c2 = (if *p2 as libc::c_int >= 'a' as i32 && *p2 as libc::c_int <= 'z' as i32 {
            *p2 as libc::c_int - 'a' as i32 + 'A' as i32
        } else {
            *p2 as libc::c_int
        }) as uchar_t;
        p2 = p2.offset(1);
        p2;
        state
            += (c1 as libc::c_int == '0' as i32) as libc::c_int
                + (((c1 as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                    <= 9 as libc::c_int as libc::c_uint) as libc::c_int
                    != 0 as libc::c_int) as libc::c_int;
    }
    state = result_type[(state * 3 as libc::c_int
        + ((c2 as libc::c_int == '0' as i32) as libc::c_int
            + (((c2 as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint) as libc::c_int != 0 as libc::c_int)
                as libc::c_int)) as usize] as libc::c_int;
    match state {
        2 => return diff,
        3 => {
            loop {
                let fresh13 = p1;
                p1 = p1.offset(1);
                if !((*fresh13 as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                    <= 9 as libc::c_int as libc::c_uint)
                {
                    break;
                }
                let fresh14 = p2;
                p2 = p2.offset(1);
                if !((*fresh14 as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                    <= 9 as libc::c_int as libc::c_uint)
                {
                    return 1 as libc::c_int;
                }
            }
            return if (*p2 as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint
            {
                -(1 as libc::c_int)
            } else {
                diff
            };
        }
        _ => return state,
    };
}
static mut namecmpfn: Option::<
    unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
> = Some(
    xstricmp
        as unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
);
static mut fnstrstr: Option::<
    unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> *mut libc::c_char,
> = Some(
    strcasestr
        as unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
        ) -> *mut libc::c_char,
);
static mut regflags: libc::c_int = (1 as libc::c_int) << 3 as libc::c_int
    | 1 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int;
unsafe extern "C" fn setfilter(
    mut regex: *mut regex_t,
    mut filter: *const libc::c_char,
) -> libc::c_int {
    return regcomp(regex, filter, regflags);
}
unsafe extern "C" fn visible_re(
    mut fltrexp: *const fltrexp_t,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    return (regexec(
        (*fltrexp).regex,
        fname,
        0 as libc::c_int as size_t,
        0 as *mut regmatch_t,
        0 as libc::c_int,
    ) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn visible_str(
    mut fltrexp: *const fltrexp_t,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    return (fnstrstr.unwrap()(fname, (*fltrexp).str_0)
        != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
}
static mut filterfn: Option::<
    unsafe extern "C" fn(*const fltrexp_t, *const libc::c_char) -> libc::c_int,
> = Some(
    visible_str
        as unsafe extern "C" fn(*const fltrexp_t, *const libc::c_char) -> libc::c_int,
);
unsafe extern "C" fn clearfilter() {
    let mut fltr: *mut libc::c_char = (g_ctx[cfg.curctx() as usize].c_fltr).as_mut_ptr();
    if *fltr.offset(1 as libc::c_int as isize) != 0 {
        *fltr
            .offset(
                (48 as libc::c_int - 1 as libc::c_int) as isize,
            ) = *fltr.offset(1 as libc::c_int as isize);
        *fltr.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
}
unsafe extern "C" fn entrycmp(
    mut va: *const libc::c_void,
    mut vb: *const libc::c_void,
) -> libc::c_int {
    let mut pa: *const entry = va as pEntry as *const entry;
    let mut pb: *const entry = vb as pEntry as *const entry;
    if ((*pb).c2rust_unnamed).flags() as libc::c_int & 0x1 as libc::c_int
        != ((*pa).c2rust_unnamed).flags() as libc::c_int & 0x1 as libc::c_int
    {
        if ((*pb).c2rust_unnamed).flags() as libc::c_int & 0x1 as libc::c_int != 0 {
            return 1 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    if cfg.timeorder() != 0 {
        if (*pb).sec > (*pa).sec {
            return 1 as libc::c_int;
        }
        if (*pb).sec < (*pa).sec {
            return -(1 as libc::c_int);
        }
        if (*pb).nsec > (*pa).nsec {
            return 1 as libc::c_int;
        }
        if (*pb).nsec < (*pa).nsec {
            return -(1 as libc::c_int);
        }
    } else if cfg.sizeorder() != 0 {
        if (*pb).size > (*pa).size {
            return 1 as libc::c_int;
        }
        if (*pb).size < (*pa).size {
            return -(1 as libc::c_int);
        }
    } else if cfg.blkorder() != 0 {
        if ((*pb).c2rust_unnamed).blocks() > ((*pa).c2rust_unnamed).blocks() {
            return 1 as libc::c_int;
        }
        if ((*pb).c2rust_unnamed).blocks() < ((*pa).c2rust_unnamed).blocks() {
            return -(1 as libc::c_int);
        }
    } else if cfg.extnorder() as libc::c_int != 0
        && ((*pb).c2rust_unnamed).flags() as libc::c_int & 0x1 as libc::c_int == 0
    {
        let mut extna: *mut libc::c_char = xextension(
            (*pa).name,
            (((*pa).c2rust_unnamed).nlen() as libc::c_int - 1 as libc::c_int) as size_t,
        );
        let mut extnb: *mut libc::c_char = xextension(
            (*pb).name,
            (((*pb).c2rust_unnamed).nlen() as libc::c_int - 1 as libc::c_int) as size_t,
        );
        if !extna.is_null() || !extnb.is_null() {
            if extna.is_null() {
                return -(1 as libc::c_int);
            }
            if extnb.is_null() {
                return 1 as libc::c_int;
            }
            let mut ret: libc::c_int = strcasecmp(extna, extnb);
            if ret != 0 {
                return ret;
            }
        }
    }
    return namecmpfn.unwrap()((*pa).name, (*pb).name);
}
unsafe extern "C" fn reventrycmp(
    mut va: *const libc::c_void,
    mut vb: *const libc::c_void,
) -> libc::c_int {
    if ((*(vb as pEntry)).c2rust_unnamed).flags() as libc::c_int & 0x1 as libc::c_int
        != ((*(va as pEntry)).c2rust_unnamed).flags() as libc::c_int & 0x1 as libc::c_int
    {
        if ((*(vb as pEntry)).c2rust_unnamed).flags() as libc::c_int & 0x1 as libc::c_int
            != 0
        {
            return 1 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    return -entrycmp(va, vb);
}
static mut entrycmpfn: Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
> = Some(
    entrycmp
        as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
);
unsafe extern "C" fn handle_alt_key(mut wch: *mut wint_t) -> libc::c_int {
    wtimeout(stdscr, 0 as libc::c_int);
    let mut r: libc::c_int = wget_wch(stdscr, wch);
    if r == -(1 as libc::c_int) {
        *wch = ('[' as i32 & 0x1f as libc::c_int) as wint_t;
    }
    wtimeout(stdscr, -(1 as libc::c_int));
    return r;
}
#[inline]
unsafe extern "C" fn handle_event() -> libc::c_int {
    if nselected != 0 && isselfileempty() as libc::c_int != 0 {
        clearselection();
    }
    return 'L' as i32 & 0x1f as libc::c_int;
}
unsafe extern "C" fn nextsel(mut presel: libc::c_int) -> libc::c_int {
    let mut c: wint_t = presel as wint_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut escaped: bool = 0 as libc::c_int != 0;
    if c == 0 as libc::c_int as libc::c_uint || c == '$' as i32 as libc::c_uint {
        loop {
            i = wget_wch(stdscr, &mut c);
            if c == 0o632 as libc::c_int as libc::c_uint {
                handle_key_resize();
            }
            if !(c == ('[' as i32 & 0x1f as libc::c_int) as libc::c_uint) {
                break;
            }
            wtimeout(stdscr, 0 as libc::c_int);
            i = wget_wch(stdscr, &mut c);
            if i != -(1 as libc::c_int) {
                if c == ('[' as i32 & 0x1f as libc::c_int) as libc::c_uint {
                    c = ('L' as i32 & 0x1f as libc::c_int) as wint_t;
                } else {
                    unget_wch(c as wchar_t);
                    c = ';' as i32 as wint_t;
                }
                wtimeout(stdscr, 1000 as libc::c_int);
                break;
            } else if escaped {
                wtimeout(stdscr, 1000 as libc::c_int);
                c = ('Q' as i32 & 0x1f as libc::c_int) as wint_t;
                break;
            } else {
                if g_state.fifomode() == 0 {
                    notify_fifo(1 as libc::c_int != 0);
                }
                escaped = 1 as libc::c_int != 0;
                wtimeout(stdscr, 1000 as libc::c_int);
            }
        }
        if i == -(1 as libc::c_int) && presel == '$' as i32 {
            c = (if cfg.filtermode() as libc::c_int != 0
                || g_ctx[cfg.curctx() as usize].c_fltr[1 as libc::c_int as usize]
                    as libc::c_int != 0
            {
                '/' as i32
            } else {
                'L' as i32 & 0x1f as libc::c_int
            }) as wint_t;
        } else if c == '/' as i32 as libc::c_uint
            || c == ('L' as i32 & 0x1f as libc::c_int) as libc::c_uint
        {
            clearfilter();
        }
    }
    if i == -(1 as libc::c_int) {
        idle = idle.wrapping_add(1);
        idle;
        if cfg.blkorder() == 0 && inotify_wd >= 0 as libc::c_int
            && idle as libc::c_int & 1 as libc::c_int != 0
        {
            let mut event: *mut inotify_event = 0 as *mut inotify_event;
            let mut inotify_buf: [libc::c_char; 512] = [
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
            ];
            i = read(
                inotify_fd,
                inotify_buf.as_mut_ptr() as *mut libc::c_void,
                (::std::mem::size_of::<inotify_event>() as libc::c_ulong)
                    .wrapping_mul(32 as libc::c_int as libc::c_ulong),
            ) as libc::c_int;
            if i > 0 as libc::c_int {
                let mut ptr: *mut libc::c_char = inotify_buf.as_mut_ptr();
                while ptr.offset((*(ptr as *mut inotify_event)).len as isize)
                    < inotify_buf.as_mut_ptr().offset(i as isize)
                {
                    event = ptr as *mut inotify_event;
                    if (*event).wd == 0 {
                        break;
                    }
                    if (*event).mask & INOTIFY_MASK != 0 {
                        c = handle_event() as wint_t;
                        break;
                    } else {
                        ptr = ptr
                            .offset(
                                (::std::mem::size_of::<inotify_event>() as libc::c_ulong)
                                    .wrapping_add((*event).len as libc::c_ulong) as isize,
                            );
                    }
                }
            }
        }
    } else {
        idle = 0 as libc::c_int as ushort_t;
    }
    i = 0 as libc::c_int;
    while i
        < (::std::mem::size_of::<[key; 85]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<key>() as libc::c_ulong) as libc::c_int
    {
        if c == bindings[i as usize].sym {
            return bindings[i as usize].act as libc::c_int;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn getorderstr(mut sort: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    if cfg.showhidden() != 0 {
        let fresh15 = i;
        i = i + 1;
        *sort.offset(fresh15 as isize) = 'H' as i32 as libc::c_char;
    }
    if cfg.timeorder() != 0 {
        let fresh16 = i;
        i = i + 1;
        *sort
            .offset(
                fresh16 as isize,
            ) = (if cfg.timetype() as libc::c_int == 2 as libc::c_int {
            'M' as i32
        } else if cfg.timetype() as libc::c_int == 0 as libc::c_int {
            'A' as i32
        } else {
            'C' as i32
        }) as libc::c_char;
    } else if cfg.sizeorder() != 0 {
        let fresh17 = i;
        i = i + 1;
        *sort.offset(fresh17 as isize) = 'S' as i32 as libc::c_char;
    } else if cfg.extnorder() != 0 {
        let fresh18 = i;
        i = i + 1;
        *sort.offset(fresh18 as isize) = 'E' as i32 as libc::c_char;
    }
    if entrycmpfn
        == Some(
            reventrycmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        )
    {
        let fresh19 = i;
        i = i + 1;
        *sort.offset(fresh19 as isize) = 'R' as i32 as libc::c_char;
    }
    if namecmpfn
        == Some(
            xstrverscasecmp
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        )
    {
        let fresh20 = i;
        i = i + 1;
        *sort.offset(fresh20 as isize) = 'V' as i32 as libc::c_char;
    }
    if i != 0 {
        *sort.offset(i as isize) = ' ' as i32 as libc::c_char;
    }
    return i;
}
unsafe extern "C" fn showfilterinfo() {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut info: [libc::c_char; 48] = *::std::mem::transmute::<
        &[u8; 48],
        &mut [libc::c_char; 48],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    i = getorderstr(info.as_mut_ptr());
    if cfg.fileinfo() as libc::c_int != 0 && ndents != 0
        && get_output(
            b"file\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"-b\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*pdents.offset(cur as isize)).name,
            -(1 as libc::c_int),
            0 as libc::c_int != 0,
            0 as libc::c_int != 0,
        ) as libc::c_int != 0
    {
        if wmove(stdscr, xlines as libc::c_int - 2 as libc::c_int, 2 as libc::c_int)
            == -(1 as libc::c_int)
        {
            -(1 as libc::c_int);
        } else {
            waddnstr(stdscr, g_buf.as_mut_ptr(), -(1 as libc::c_int));
        };
    } else {
        snprintf(
            info.as_mut_ptr().offset(i as isize),
            (48 as libc::c_int - i - 1 as libc::c_int) as libc::c_ulong,
            b"  %s [/], %4s [:]\0" as *const u8 as *const libc::c_char,
            if cfg.regex() as libc::c_int != 0 {
                b"reg\0" as *const u8 as *const libc::c_char
            } else {
                b"str\0" as *const u8 as *const libc::c_char
            },
            if fnstrstr
                == Some(
                    strcasestr
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                        ) -> *mut libc::c_char,
                )
            {
                b"ic\0" as *const u8 as *const libc::c_char
            } else {
                b"noic\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if wmove(
        stdscr,
        xlines as libc::c_int - 2 as libc::c_int,
        (xcols as libc::c_ulong).wrapping_sub(xstrlen(info.as_mut_ptr())) as libc::c_int,
    ) == -(1 as libc::c_int)
    {
        -(1 as libc::c_int);
    } else {
        waddnstr(stdscr, info.as_mut_ptr(), -(1 as libc::c_int));
    };
}
unsafe extern "C" fn showfilter(mut str: *mut libc::c_char) {
    wattr_on(
        stdscr,
        ((cfg.curctx() as libc::c_int + 1 as libc::c_int) as chtype)
            << 0 as libc::c_int + 8 as libc::c_int
            & ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int,
        0 as *mut libc::c_void,
    );
    showfilterinfo();
    printmsg(str);
}
#[inline]
unsafe extern "C" fn swap_ent(mut id1: libc::c_int, mut id2: libc::c_int) {
    let mut _dent: entry = entry {
        name: 0 as *mut libc::c_char,
        sec: 0,
        nsec: 0,
        mode: 0,
        size: 0,
        c2rust_unnamed: C2RustUnnamed_13 {
            blocks_nlen_flags: [0; 8],
        },
        uid: 0,
        gid: 0,
    };
    let mut pdent1: *mut entry = &mut *pdents.offset(id1 as isize) as *mut entry;
    let mut pdent2: *mut entry = &mut *pdents.offset(id2 as isize) as *mut entry;
    _dent = *pdent1;
    *pdent1 = *pdent2;
    *pdent2 = _dent;
}
unsafe extern "C" fn fill(
    mut fltr: *const libc::c_char,
    mut re: *mut regex_t,
) -> libc::c_int {
    let mut fltrexp: fltrexp_t = {
        let mut init = fltrexp_t {
            regex: re,
            str_0: fltr,
        };
        init
    };
    let mut count: libc::c_int = 0 as libc::c_int;
    while count < ndents {
        if filterfn.unwrap()(&mut fltrexp, (*pdents.offset(count as isize)).name)
            == 0 as libc::c_int
        {
            ndents -= 1;
            if count != ndents {
                swap_ent(count, ndents);
                count -= 1;
                count;
            }
        }
        count += 1;
        count;
    }
    return ndents;
}
unsafe extern "C" fn matches(mut fltr: *const libc::c_char) -> libc::c_int {
    let mut re: regex_t = regex_t {
        buffer: 0 as *const re_dfa_t as *mut re_dfa_t,
        allocated: 0,
        used: 0,
        syntax: 0,
        fastmap: 0 as *const libc::c_char as *mut libc::c_char,
        translate: 0 as *const libc::c_uchar as *mut libc::c_uchar,
        re_nsub: 0,
        can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
        c2rust_padding: [0; 7],
    };
    if cfg.regex() as libc::c_int != 0 && setfilter(&mut re, fltr) != 0 {
        return -(1 as libc::c_int);
    }
    ndents = fill(fltr, &mut re);
    if cfg.regex() != 0 {
        regfree(&mut re);
    }
    qsort(
        pdents as *mut libc::c_void,
        ndents as size_t,
        ::std::mem::size_of::<entry>() as libc::c_ulong,
        entrycmpfn,
    );
    return ndents;
}
unsafe extern "C" fn dentfind(
    mut fname: *const libc::c_char,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        if (if *fname as libc::c_int != *(*pdents.offset(i as isize)).name as libc::c_int
        {
            -(1 as libc::c_int)
        } else {
            strcmp(fname, (*pdents.offset(i as isize)).name)
        }) == 0 as libc::c_int
        {
            return i;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn filterentries(
    mut path: *mut libc::c_char,
    mut lastname: *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut fresh21 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<wchar_t>() as libc::c_ulong)
            .wrapping_mul(48 as libc::c_int as libc::c_ulong) as usize,
    );
    let mut wln: *mut wchar_t = fresh21.leak().as_mut_ptr() as *mut wchar_t;
    let mut ln: *mut libc::c_char = (g_ctx[cfg.curctx() as usize].c_fltr).as_mut_ptr();
    let mut ch: [wint_t; 1] = [0; 1];
    let mut r: libc::c_int = 0;
    let mut total: libc::c_int = ndents;
    let mut len: libc::c_int = 0;
    let mut pln: *mut libc::c_char = (g_ctx[cfg.curctx() as usize].c_fltr)
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize);
    if ndents != 0
        && (*ln.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
            || *ln.offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32)
        && *pln as libc::c_int != 0
    {
        if matches(pln) != -(1 as libc::c_int) {
            move_cursor(dentfind(lastname, ndents), 0 as libc::c_int);
            redraw(path);
        }
        if cfg.filtermode() == 0 {
            statusbar(path);
            return 0 as libc::c_int;
        }
        len = mbstowcs(wln, ln, 48 as libc::c_int as size_t) as libc::c_int;
    } else {
        let ref mut fresh22 = *wln.offset(0 as libc::c_int as isize);
        *fresh22 = if cfg.regex() as libc::c_int != 0 {
            '\\' as i32
        } else {
            '/' as i32
        };
        *ln.offset(0 as libc::c_int as isize) = *fresh22 as libc::c_char;
        let ref mut fresh23 = *wln.offset(1 as libc::c_int as isize);
        *fresh23 = '\0' as i32;
        *ln.offset(1 as libc::c_int as isize) = *fresh23 as libc::c_char;
        len = 1 as libc::c_int;
    }
    wtimeout(stdscr, -(1 as libc::c_int));
    curs_set(1 as libc::c_int);
    showfilter(ln);
    loop {
        r = wget_wch(stdscr, ch.as_mut_ptr());
        if !(r != -(1 as libc::c_int)) {
            break;
        }
        match *ch.as_mut_ptr() {
            0 | 410 => {
                clearoldprompt();
                redraw(path);
                showfilter(ln);
                continue;
            }
            330 => {
                current_block = 17540844955045440384;
            }
            263 => {
                current_block = 17540844955045440384;
            }
            8 => {
                current_block = 17427440078520747492;
            }
            127 => {
                current_block = 4594597675263786568;
            }
            12 => {
                current_block = 11079902710555032349;
            }
            409 => {
                break;
            }
            27 => {
                if handle_alt_key(ch.as_mut_ptr()) != -(1 as libc::c_int) {
                    if *ch.as_mut_ptr()
                        == ('[' as i32 & 0x1f as libc::c_int) as libc::c_uint
                    {
                        *ch.as_mut_ptr() = 'q' as i32 as wint_t;
                    } else {
                        unget_wch(*ch.as_mut_ptr() as wchar_t);
                        *ch.as_mut_ptr() = ';' as i32 as wint_t;
                    }
                }
                break;
            }
            _ => {
                if r != 0 as libc::c_int {
                    break;
                } else {
                    if *ch.as_mut_ptr() < 128 as libc::c_int as libc::c_uint
                        && *(keyname(*ch.as_mut_ptr() as libc::c_int))
                            .offset(0 as libc::c_int as isize) as libc::c_int
                            == '^' as i32
                        && *ch.as_mut_ptr() != '^' as i32 as libc::c_uint
                    {
                        break;
                    }
                    if len == 1 as libc::c_int {
                        if *ch.as_mut_ptr() == '?' as i32 as libc::c_uint {
                            break;
                        } else {
                            if cfg.filtermode() != 0 {
                                match *ch.as_mut_ptr() {
                                    39 => {
                                        current_block = 6019675103915610540;
                                        match current_block {
                                            6019675103915610540 => {
                                                current_block = 10722413751011434832;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            10722413751011434832 => {
                                                current_block = 17078545039806120709;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            17078545039806120709 => {
                                                current_block = 7736478293654569422;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7736478293654569422 => {
                                                current_block = 7638681725831278579;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7638681725831278579 => {
                                                current_block = 13059714787291680576;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            13059714787291680576 => {
                                                current_block = 3925304876064047349;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            3925304876064047349 => {
                                                current_block = 7773413567138364438;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7773413567138364438 => {
                                                current_block = 2102695986033157800;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            2102695986033157800 => {
                                                current_block = 5526597595903895085;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            5526597595903895085 => {}
                                            _ => {}
                                        }
                                        break;
                                    }
                                    43 => {
                                        current_block = 10722413751011434832;
                                        match current_block {
                                            6019675103915610540 => {
                                                current_block = 10722413751011434832;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            10722413751011434832 => {
                                                current_block = 17078545039806120709;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            17078545039806120709 => {
                                                current_block = 7736478293654569422;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7736478293654569422 => {
                                                current_block = 7638681725831278579;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7638681725831278579 => {
                                                current_block = 13059714787291680576;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            13059714787291680576 => {
                                                current_block = 3925304876064047349;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            3925304876064047349 => {
                                                current_block = 7773413567138364438;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7773413567138364438 => {
                                                current_block = 2102695986033157800;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            2102695986033157800 => {
                                                current_block = 5526597595903895085;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            5526597595903895085 => {}
                                            _ => {}
                                        }
                                        break;
                                    }
                                    44 => {
                                        current_block = 17078545039806120709;
                                        match current_block {
                                            6019675103915610540 => {
                                                current_block = 10722413751011434832;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            10722413751011434832 => {
                                                current_block = 17078545039806120709;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            17078545039806120709 => {
                                                current_block = 7736478293654569422;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7736478293654569422 => {
                                                current_block = 7638681725831278579;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7638681725831278579 => {
                                                current_block = 13059714787291680576;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            13059714787291680576 => {
                                                current_block = 3925304876064047349;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            3925304876064047349 => {
                                                current_block = 7773413567138364438;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7773413567138364438 => {
                                                current_block = 2102695986033157800;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            2102695986033157800 => {
                                                current_block = 5526597595903895085;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            5526597595903895085 => {}
                                            _ => {}
                                        }
                                        break;
                                    }
                                    45 => {
                                        current_block = 7736478293654569422;
                                        match current_block {
                                            6019675103915610540 => {
                                                current_block = 10722413751011434832;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            10722413751011434832 => {
                                                current_block = 17078545039806120709;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            17078545039806120709 => {
                                                current_block = 7736478293654569422;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7736478293654569422 => {
                                                current_block = 7638681725831278579;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7638681725831278579 => {
                                                current_block = 13059714787291680576;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            13059714787291680576 => {
                                                current_block = 3925304876064047349;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            3925304876064047349 => {
                                                current_block = 7773413567138364438;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7773413567138364438 => {
                                                current_block = 2102695986033157800;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            2102695986033157800 => {
                                                current_block = 5526597595903895085;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            5526597595903895085 => {}
                                            _ => {}
                                        }
                                        break;
                                    }
                                    46 => {
                                        current_block = 7638681725831278579;
                                        match current_block {
                                            6019675103915610540 => {
                                                current_block = 10722413751011434832;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            10722413751011434832 => {
                                                current_block = 17078545039806120709;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            17078545039806120709 => {
                                                current_block = 7736478293654569422;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7736478293654569422 => {
                                                current_block = 7638681725831278579;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7638681725831278579 => {
                                                current_block = 13059714787291680576;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            13059714787291680576 => {
                                                current_block = 3925304876064047349;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            3925304876064047349 => {
                                                current_block = 7773413567138364438;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7773413567138364438 => {
                                                current_block = 2102695986033157800;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            2102695986033157800 => {
                                                current_block = 5526597595903895085;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            5526597595903895085 => {}
                                            _ => {}
                                        }
                                        break;
                                    }
                                    59 => {
                                        current_block = 13059714787291680576;
                                        match current_block {
                                            6019675103915610540 => {
                                                current_block = 10722413751011434832;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            10722413751011434832 => {
                                                current_block = 17078545039806120709;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            17078545039806120709 => {
                                                current_block = 7736478293654569422;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7736478293654569422 => {
                                                current_block = 7638681725831278579;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7638681725831278579 => {
                                                current_block = 13059714787291680576;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            13059714787291680576 => {
                                                current_block = 3925304876064047349;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            3925304876064047349 => {
                                                current_block = 7773413567138364438;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7773413567138364438 => {
                                                current_block = 2102695986033157800;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            2102695986033157800 => {
                                                current_block = 5526597595903895085;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            5526597595903895085 => {}
                                            _ => {}
                                        }
                                        break;
                                    }
                                    61 => {
                                        current_block = 3925304876064047349;
                                        match current_block {
                                            6019675103915610540 => {
                                                current_block = 10722413751011434832;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            10722413751011434832 => {
                                                current_block = 17078545039806120709;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            17078545039806120709 => {
                                                current_block = 7736478293654569422;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7736478293654569422 => {
                                                current_block = 7638681725831278579;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7638681725831278579 => {
                                                current_block = 13059714787291680576;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            13059714787291680576 => {
                                                current_block = 3925304876064047349;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            3925304876064047349 => {
                                                current_block = 7773413567138364438;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7773413567138364438 => {
                                                current_block = 2102695986033157800;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            2102695986033157800 => {
                                                current_block = 5526597595903895085;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            5526597595903895085 => {}
                                            _ => {}
                                        }
                                        break;
                                    }
                                    62 => {
                                        current_block = 7773413567138364438;
                                        match current_block {
                                            6019675103915610540 => {
                                                current_block = 10722413751011434832;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            10722413751011434832 => {
                                                current_block = 17078545039806120709;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            17078545039806120709 => {
                                                current_block = 7736478293654569422;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7736478293654569422 => {
                                                current_block = 7638681725831278579;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7638681725831278579 => {
                                                current_block = 13059714787291680576;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            13059714787291680576 => {
                                                current_block = 3925304876064047349;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            3925304876064047349 => {
                                                current_block = 7773413567138364438;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7773413567138364438 => {
                                                current_block = 2102695986033157800;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            2102695986033157800 => {
                                                current_block = 5526597595903895085;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            5526597595903895085 => {}
                                            _ => {}
                                        }
                                        break;
                                    }
                                    64 => {
                                        current_block = 2102695986033157800;
                                        match current_block {
                                            6019675103915610540 => {
                                                current_block = 10722413751011434832;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            10722413751011434832 => {
                                                current_block = 17078545039806120709;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            17078545039806120709 => {
                                                current_block = 7736478293654569422;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7736478293654569422 => {
                                                current_block = 7638681725831278579;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7638681725831278579 => {
                                                current_block = 13059714787291680576;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            13059714787291680576 => {
                                                current_block = 3925304876064047349;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            3925304876064047349 => {
                                                current_block = 7773413567138364438;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7773413567138364438 => {
                                                current_block = 2102695986033157800;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            2102695986033157800 => {
                                                current_block = 5526597595903895085;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            5526597595903895085 => {}
                                            _ => {}
                                        }
                                        break;
                                    }
                                    93 => {
                                        current_block = 5526597595903895085;
                                        match current_block {
                                            6019675103915610540 => {
                                                current_block = 10722413751011434832;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            10722413751011434832 => {
                                                current_block = 17078545039806120709;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            17078545039806120709 => {
                                                current_block = 7736478293654569422;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7736478293654569422 => {
                                                current_block = 7638681725831278579;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7638681725831278579 => {
                                                current_block = 13059714787291680576;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            13059714787291680576 => {
                                                current_block = 3925304876064047349;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            3925304876064047349 => {
                                                current_block = 7773413567138364438;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7773413567138364438 => {
                                                current_block = 2102695986033157800;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            2102695986033157800 => {
                                                current_block = 5526597595903895085;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            5526597595903895085 => {}
                                            _ => {}
                                        }
                                        break;
                                    }
                                    96 => {
                                        current_block = 3012350749724007084;
                                        match current_block {
                                            6019675103915610540 => {
                                                current_block = 10722413751011434832;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            10722413751011434832 => {
                                                current_block = 17078545039806120709;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            17078545039806120709 => {
                                                current_block = 7736478293654569422;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7736478293654569422 => {
                                                current_block = 7638681725831278579;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7638681725831278579 => {
                                                current_block = 13059714787291680576;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            13059714787291680576 => {
                                                current_block = 3925304876064047349;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            3925304876064047349 => {
                                                current_block = 7773413567138364438;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            7773413567138364438 => {
                                                current_block = 2102695986033157800;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            2102695986033157800 => {
                                                current_block = 5526597595903895085;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            5526597595903895085 => {}
                                            _ => {}
                                        }
                                        break;
                                    }
                                    126 => {
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                            if *ch.as_mut_ptr() == ':' as i32 as libc::c_uint {
                                fnstrstr = if fnstrstr
                                    == Some(
                                        strcasestr
                                            as unsafe extern "C" fn(
                                                *const libc::c_char,
                                                *const libc::c_char,
                                            ) -> *mut libc::c_char,
                                    )
                                {
                                    Some(
                                        strstr
                                            as unsafe extern "C" fn(
                                                *const libc::c_char,
                                                *const libc::c_char,
                                            ) -> *mut libc::c_char,
                                    )
                                } else {
                                    Some(
                                        strcasestr
                                            as unsafe extern "C" fn(
                                                *const libc::c_char,
                                                *const libc::c_char,
                                            ) -> *mut libc::c_char,
                                    )
                                };
                                regflags ^= (1 as libc::c_int) << 1 as libc::c_int;
                                showfilter(ln);
                                continue;
                            } else if *ch.as_mut_ptr() == '/' as i32 as libc::c_uint {
                                *ln
                                    .offset(
                                        0 as libc::c_int as isize,
                                    ) = (if *ln.offset(0 as libc::c_int as isize) as libc::c_int
                                    == '/' as i32
                                {
                                    '\\' as i32
                                } else {
                                    '/' as i32
                                }) as libc::c_char;
                                *wln
                                    .offset(
                                        0 as libc::c_int as isize,
                                    ) = *ln.offset(0 as libc::c_int as isize) as uchar_t
                                    as wchar_t;
                                cfg.set_regex(cfg.regex() ^ 1 as libc::c_int as uint_t);
                                filterfn = if cfg.regex() as libc::c_int != 0 {
                                    Some(
                                        visible_re
                                            as unsafe extern "C" fn(
                                                *const fltrexp_t,
                                                *const libc::c_char,
                                            ) -> libc::c_int,
                                    )
                                } else {
                                    Some(
                                        visible_str
                                            as unsafe extern "C" fn(
                                                *const fltrexp_t,
                                                *const libc::c_char,
                                            ) -> libc::c_int,
                                    )
                                };
                                showfilter(ln);
                                continue;
                            } else {
                                cur = 0 as libc::c_int;
                            }
                        }
                    } else if len == 48 as libc::c_int - 1 as libc::c_int {
                        continue;
                    }
                    *wln.offset(len as isize) = *ch.as_mut_ptr() as wchar_t;
                    len += 1;
                    *wln.offset(len as isize) = '\0' as i32;
                    wcstombs(ln, wln, 48 as libc::c_int as size_t);
                    if matches(pln) == -(1 as libc::c_int) {
                        showfilter(ln);
                        continue;
                    } else if ndents == 1 as libc::c_int
                        && cfg.autoenter() as libc::c_int != 0
                        && ((*pdents.offset(0 as libc::c_int as isize)).c2rust_unnamed)
                            .flags() as libc::c_int & 0x1 as libc::c_int != 0
                    {
                        *ch.as_mut_ptr() = 0o527 as libc::c_int as wint_t;
                        cur = 0 as libc::c_int;
                        break;
                    } else {
                        redraw(path);
                        showfilter(ln);
                        continue;
                    }
                }
            }
        }
        match current_block {
            17540844955045440384 => {
                current_block = 17427440078520747492;
            }
            _ => {}
        }
        match current_block {
            17427440078520747492 => {
                current_block = 4594597675263786568;
            }
            _ => {}
        }
        match current_block {
            4594597675263786568 => {
                if len != 1 as libc::c_int {
                    len -= 1;
                    *wln.offset(len as isize) = '\0' as i32;
                    wcstombs(ln, wln, 48 as libc::c_int as size_t);
                    ndents = total;
                } else {
                    *ch.as_mut_ptr() = '/' as i32 as wint_t;
                    break;
                }
            }
            _ => {}
        }
        if *ch.as_mut_ptr() == ('L' as i32 & 0x1f as libc::c_int) as libc::c_uint {
            if *wln.offset(1 as libc::c_int as isize) != 0 {
                *ln
                    .offset(
                        (48 as libc::c_int - 1 as libc::c_int) as isize,
                    ) = *ln.offset(1 as libc::c_int as isize);
                let ref mut fresh24 = *wln.offset(1 as libc::c_int as isize);
                *fresh24 = '\0' as i32;
                *ln.offset(1 as libc::c_int as isize) = *fresh24 as libc::c_char;
                len = 1 as libc::c_int;
                ndents = total;
            } else {
                if !(*ln.offset((48 as libc::c_int - 1 as libc::c_int) as isize) != 0) {
                    break;
                }
                *ln
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *ln.offset((48 as libc::c_int - 1 as libc::c_int) as isize);
                *ln
                    .offset(
                        (48 as libc::c_int - 1 as libc::c_int) as isize,
                    ) = '\0' as i32 as libc::c_char;
                len = mbstowcs(wln, ln, 48 as libc::c_int as size_t) as libc::c_int;
            }
        }
        cur = 0 as libc::c_int;
        if matches(pln) != -(1 as libc::c_int) {
            redraw(path);
        }
        showfilter(ln);
    }
    if *ln.offset(1 as libc::c_int as isize) != 0 {
        *ln
            .offset(
                (48 as libc::c_int - 1 as libc::c_int) as isize,
            ) = *ln.offset(1 as libc::c_int as isize);
    }
    xstrsncpy(
        lastname,
        if ndents != 0 {
            (*pdents.offset(cur as isize)).name as *const libc::c_char
        } else {
            b"\0\0" as *const u8 as *const libc::c_char
        },
        (255 as libc::c_int + 1 as libc::c_int) as size_t,
    );
    curs_set(0 as libc::c_int);
    wtimeout(stdscr, 1000 as libc::c_int);
    return *ch.as_mut_ptr() as libc::c_int;
}
unsafe extern "C" fn xreadline(
    mut prefill: *const libc::c_char,
    mut prompt: *const libc::c_char,
) -> *mut libc::c_char {
    let mut len: size_t = 0;
    let mut pos: size_t = 0;
    let mut x: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let WCHAR_T_WIDTH: libc::c_int = ::std::mem::size_of::<wchar_t>() as libc::c_ulong
        as libc::c_int;
    let mut ch: [wint_t; 1] = [0; 1];
    let buf: *mut wchar_t = malloc(
        (::std::mem::size_of::<wchar_t>() as libc::c_ulong)
            .wrapping_mul(256 as libc::c_int as libc::c_ulong),
    ) as *mut wchar_t;
    if buf.is_null() {
        printerr(3442 as libc::c_int);
    }
    wtimeout(stdscr, -(1 as libc::c_int));
    printmsg(prompt);
    if !prefill.is_null() {
        pos = mbstowcs(buf, prefill, 256 as libc::c_int as size_t);
        len = pos;
    } else {
        len = -(1 as libc::c_int) as size_t;
    }
    if len == -(1 as libc::c_int) as size_t {
        *buf.offset(0 as libc::c_int as isize) = '\0' as i32;
        pos = 0 as libc::c_int as size_t;
        len = pos;
    }
    x = if !(stdscr as *const libc::c_void).is_null() {
        (*stdscr)._curx as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    curs_set(1 as libc::c_int);
    loop {
        *buf.offset(len as isize) = ' ' as i32;
        wattr_on(
            stdscr,
            ((cfg.curctx() as libc::c_int + 1 as libc::c_int) as chtype)
                << 0 as libc::c_int + 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int,
            0 as *mut libc::c_void,
        );
        if pos > (xcols as libc::c_int - x) as size_t {
            if wmove(stdscr, xlines as libc::c_int - 1 as libc::c_int, x)
                == -(1 as libc::c_int)
            {
                -(1 as libc::c_int);
            } else {
                waddnwstr(
                    stdscr,
                    buf
                        .offset(
                            pos
                                .wrapping_sub((xcols as libc::c_int - x) as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        ),
                    xcols as libc::c_int - x,
                );
            };
            wmove(
                stdscr,
                xlines as libc::c_int - 1 as libc::c_int,
                xcols as libc::c_int - 1 as libc::c_int,
            );
        } else {
            if wmove(stdscr, xlines as libc::c_int - 1 as libc::c_int, x)
                == -(1 as libc::c_int)
            {
                -(1 as libc::c_int);
            } else {
                waddnwstr(
                    stdscr,
                    buf,
                    len.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                );
            };
            wmove(
                stdscr,
                xlines as libc::c_int - 1 as libc::c_int,
                x + wcswidth(buf, pos),
            );
        }
        wattr_off(
            stdscr,
            ((cfg.curctx() as libc::c_int + 1 as libc::c_int) as chtype)
                << 0 as libc::c_int + 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int,
            0 as *mut libc::c_void,
        );
        r = wget_wch(stdscr, ch.as_mut_ptr());
        if r == -(1 as libc::c_int) {
            continue;
        }
        if r == 0 as libc::c_int {
            match *ch.as_mut_ptr() {
                343 => {
                    break;
                }
                10 | 13 => {
                    break;
                }
                4 => {
                    if pos < len {
                        pos = pos.wrapping_add(1);
                        pos;
                    } else {
                        if pos != 0 || len != 0 {
                            continue;
                        }
                        len = 0 as libc::c_int as size_t;
                        break;
                    }
                }
                127 | 8 => {}
                9 => {
                    if !(len != 0 || pos != 0) && ndents != 0 {
                        pos = mbstowcs(
                            buf,
                            (*pdents.offset(cur as isize)).name,
                            256 as libc::c_int as size_t,
                        );
                        len = pos;
                    }
                    continue;
                }
                6 => {
                    if pos < len {
                        pos = pos.wrapping_add(1);
                        pos;
                    }
                    continue;
                }
                2 => {
                    if pos > 0 as libc::c_int as libc::c_ulong {
                        pos = pos.wrapping_sub(1);
                        pos;
                    }
                    continue;
                }
                23 => {
                    printmsg(prompt);
                    while !(pos == 0 as libc::c_int as libc::c_ulong) {
                        memmove(
                            buf.offset(pos as isize).offset(-(1 as libc::c_int as isize))
                                as *mut libc::c_void,
                            buf.offset(pos as isize) as *const libc::c_void,
                            len
                                .wrapping_sub(pos)
                                .wrapping_mul(WCHAR_T_WIDTH as libc::c_ulong),
                        );
                        pos = pos.wrapping_sub(1);
                        pos;
                        len = len.wrapping_sub(1);
                        len;
                        if !(*buf
                            .offset(
                                pos.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) != ' ' as i32
                            && *buf
                                .offset(
                                    pos.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                ) != '/' as i32)
                        {
                            break;
                        }
                    }
                    continue;
                }
                11 => {
                    printmsg(prompt);
                    len = pos;
                    continue;
                }
                12 => {
                    printmsg(prompt);
                    pos = 0 as libc::c_int as size_t;
                    len = pos;
                    continue;
                }
                1 => {
                    pos = 0 as libc::c_int as size_t;
                    continue;
                }
                5 => {
                    pos = len;
                    continue;
                }
                21 => {
                    printmsg(prompt);
                    memmove(
                        buf as *mut libc::c_void,
                        buf.offset(pos as isize) as *const libc::c_void,
                        len
                            .wrapping_sub(pos)
                            .wrapping_mul(WCHAR_T_WIDTH as libc::c_ulong),
                    );
                    len = (len as libc::c_ulong).wrapping_sub(pos) as size_t as size_t;
                    pos = 0 as libc::c_int as size_t;
                    continue;
                }
                27 => {
                    if handle_alt_key(ch.as_mut_ptr()) != -(1 as libc::c_int) {
                        continue;
                    }
                    len = 0 as libc::c_int as size_t;
                    break;
                }
                _ => {
                    if *ch.as_mut_ptr() < 128 as libc::c_int as libc::c_uint
                        && *(keyname(*ch.as_mut_ptr() as libc::c_int))
                            .offset(0 as libc::c_int as isize) as libc::c_int
                            == '^' as i32
                    {
                        continue;
                    }
                    if !(pos < (256 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                    {
                        continue;
                    }
                    memmove(
                        buf.offset(pos as isize).offset(1 as libc::c_int as isize)
                            as *mut libc::c_void,
                        buf.offset(pos as isize) as *const libc::c_void,
                        len
                            .wrapping_sub(pos)
                            .wrapping_mul(WCHAR_T_WIDTH as libc::c_ulong),
                    );
                    *buf.offset(pos as isize) = *ch.as_mut_ptr() as wchar_t;
                    len = len.wrapping_add(1);
                    len;
                    pos = pos.wrapping_add(1);
                    pos;
                    continue;
                }
            }
            if pos > 0 as libc::c_int as libc::c_ulong {
                memmove(
                    buf.offset(pos as isize).offset(-(1 as libc::c_int as isize))
                        as *mut libc::c_void,
                    buf.offset(pos as isize) as *const libc::c_void,
                    len.wrapping_sub(pos).wrapping_mul(WCHAR_T_WIDTH as libc::c_ulong),
                );
                len = len.wrapping_sub(1);
                len;
                pos = pos.wrapping_sub(1);
                pos;
            }
        } else {
            match *ch.as_mut_ptr() {
                410 => {
                    clearoldprompt();
                    xlines = LINES as ushort_t;
                    printmsg(prompt);
                }
                260 => {
                    if pos > 0 as libc::c_int as libc::c_ulong {
                        pos = pos.wrapping_sub(1);
                        pos;
                    }
                }
                261 => {
                    if pos < len {
                        pos = pos.wrapping_add(1);
                        pos;
                    }
                }
                263 => {
                    if pos > 0 as libc::c_int as libc::c_ulong {
                        memmove(
                            buf.offset(pos as isize).offset(-(1 as libc::c_int as isize))
                                as *mut libc::c_void,
                            buf.offset(pos as isize) as *const libc::c_void,
                            len
                                .wrapping_sub(pos)
                                .wrapping_mul(WCHAR_T_WIDTH as libc::c_ulong),
                        );
                        len = len.wrapping_sub(1);
                        len;
                        pos = pos.wrapping_sub(1);
                        pos;
                    }
                }
                330 => {
                    if pos < len {
                        memmove(
                            buf.offset(pos as isize) as *mut libc::c_void,
                            buf.offset(pos as isize).offset(1 as libc::c_int as isize)
                                as *const libc::c_void,
                            len
                                .wrapping_sub(pos)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(WCHAR_T_WIDTH as libc::c_ulong),
                        );
                        len = len.wrapping_sub(1);
                        len;
                    }
                }
                360 => {
                    pos = len;
                }
                262 => {
                    pos = 0 as libc::c_int as size_t;
                }
                259 | 258 => {
                    if !prompt.is_null() && !lastcmd.is_null()
                        && (if *prompt as libc::c_int
                            != *(b">>> \0" as *const u8 as *const libc::c_char)
                                as libc::c_int
                        {
                            -(1 as libc::c_int)
                        } else {
                            strcmp(prompt, b">>> \0" as *const u8 as *const libc::c_char)
                        }) == 0 as libc::c_int
                    {
                        printmsg(prompt);
                        pos = mbstowcs(buf, lastcmd, 256 as libc::c_int as size_t);
                        len = pos;
                    }
                }
                _ => {}
            }
        }
    }
    curs_set(0 as libc::c_int);
    wtimeout(stdscr, 1000 as libc::c_int);
    printmsg(b"\0" as *const u8 as *const libc::c_char);
    *buf.offset(len as isize) = '\0' as i32;
    pos = wcstombs(
        g_buf.as_mut_ptr(),
        buf,
        (256 as libc::c_int - 1 as libc::c_int) as size_t,
    );
    if pos >= (256 as libc::c_int - 1 as libc::c_int) as libc::c_ulong {
        g_buf[(256 as libc::c_int - 1 as libc::c_int)
            as usize] = '\0' as i32 as libc::c_char;
    }
    free(buf as *mut libc::c_void);
    return g_buf.as_mut_ptr();
}
unsafe extern "C" fn getreadline(mut prompt: *const libc::c_char) -> *mut libc::c_char {
    endwin();
    let mut input: *mut libc::c_char = readline(prompt);
    wrefresh(stdscr);
    if !input.is_null() && *input.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        add_history(input);
        xstrsncpy(
            g_buf.as_mut_ptr(),
            input,
            (4096 as libc::c_int
                + ((255 as libc::c_int + 1 as libc::c_int) << 1 as libc::c_int))
                as size_t,
        );
        free(input as *mut libc::c_void);
        return g_buf.as_mut_ptr();
    }
    free(input as *mut libc::c_void);
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn xlink(
    mut prefix: *mut libc::c_char,
    mut path: *mut libc::c_char,
    mut curfname: *mut libc::c_char,
    mut buf: *mut libc::c_char,
    mut presel: *mut libc::c_int,
    mut type_0: libc::c_int,
) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut choice: libc::c_int = 0;
    let mut psel: *mut libc::c_char = pselbuf;
    let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    let mut len: size_t = 0;
    let mut r: size_t = 0;
    let mut link_fn: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> libc::c_int,
    > = None;
    let mut lnpath: [libc::c_char; 4096] = [0; 4096];
    choice = get_cur_or_sel();
    if choice == 0 {
        return -(1 as libc::c_int);
    }
    if type_0 == 's' as i32 {
        link_fn = Some(
            symlink
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        );
    } else {
        link_fn = Some(
            link
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        );
    }
    if choice == 'c' as i32 || nselected == 1 as libc::c_int {
        mkpath(path, prefix, lnpath.as_mut_ptr());
        mkpath(path, if choice == 'c' as i32 { curfname } else { pselbuf }, buf);
        if link_fn.unwrap()(buf, lnpath.as_mut_ptr()) == 0 {
            if choice == 's' as i32 {
                clearselection();
            }
            return 1 as libc::c_int;
        }
        printwait(strerror(*__errno_location()), presel);
        return -(1 as libc::c_int);
    }
    while pos < selbufpos as libc::c_ulong {
        len = xstrlen(psel);
        fname = xbasename(psel);
        r = xstrsncpy(buf, prefix, (255 as libc::c_int + 1 as libc::c_int) as size_t);
        xstrsncpy(
            buf.offset(r as isize).offset(-(1 as libc::c_int as isize)),
            fname,
            (255 as libc::c_int as libc::c_ulong).wrapping_sub(r),
        );
        mkpath(path, buf, lnpath.as_mut_ptr());
        if link_fn.unwrap()(psel, lnpath.as_mut_ptr()) == 0 {
            count += 1;
            count;
        }
        pos = (pos as libc::c_ulong)
            .wrapping_add(len.wrapping_add(1 as libc::c_int as libc::c_ulong)) as size_t
            as size_t;
        psel = psel.offset(len.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    }
    clearselection();
    return count;
}
unsafe extern "C" fn parsekvpair(
    mut arr: *mut *mut kv,
    mut envcpy: *mut *mut libc::c_char,
    id: uchar_t,
    mut items: *mut uchar_t,
) -> bool {
    let mut new: bool = 1 as libc::c_int != 0;
    let INCR: uchar_t = 8 as libc::c_int as uchar_t;
    let mut i: uint_t = 0 as libc::c_int as uint_t;
    let mut kvarr: *mut kv = 0 as *mut kv;
    let mut ptr: *mut libc::c_char = getenv(env_cfg[id as usize]);
    if ptr.is_null() || *ptr == 0 {
        return 1 as libc::c_int != 0;
    }
    *envcpy = xstrdup(ptr);
    if (*envcpy).is_null() {
        perror(xitoa(3716 as libc::c_int as uint_t));
        return 0 as libc::c_int != 0;
    }
    ptr = *envcpy;
    while *ptr as libc::c_int != 0 && i < 100 as libc::c_int as libc::c_uint {
        if new {
            if i & (INCR as libc::c_int - 1 as libc::c_int) as libc::c_uint == 0 {
                kvarr = xrealloc(
                    kvarr as *mut libc::c_void,
                    (::std::mem::size_of::<kv>() as libc::c_ulong)
                        .wrapping_mul(
                            i.wrapping_add(INCR as libc::c_uint) as libc::c_ulong,
                        ),
                ) as *mut kv;
                *arr = kvarr;
                if kvarr.is_null() {
                    perror(xitoa(3728 as libc::c_int as uint_t));
                    return 0 as libc::c_int != 0;
                }
                memset(
                    kvarr.offset(i as isize) as *mut libc::c_void,
                    0 as libc::c_int,
                    (::std::mem::size_of::<kv>() as libc::c_ulong)
                        .wrapping_mul(INCR as libc::c_ulong),
                );
            }
            (*kvarr.offset(i as isize)).key = *ptr as uchar_t as libc::c_int;
            ptr = ptr.offset(1);
            if *ptr as libc::c_int != ':' as i32
                || {
                    ptr = ptr.offset(1);
                    *ptr as libc::c_int == '\0' as i32
                } || *ptr as libc::c_int == ';' as i32
            {
                return 0 as libc::c_int != 0;
            }
            (*kvarr.offset(i as isize))
                .off = ptr.offset_from(*envcpy) as libc::c_long as libc::c_int;
            i = i.wrapping_add(1);
            i;
            new = 0 as libc::c_int != 0;
        }
        if *ptr as libc::c_int == ';' as i32 {
            *ptr = '\0' as i32 as libc::c_char;
            new = 1 as libc::c_int != 0;
        }
        ptr = ptr.offset(1);
        ptr;
    }
    *items = i as uchar_t;
    return i != 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn get_kv_val(
    mut kvarr: *mut kv,
    mut buf: *mut libc::c_char,
    mut key: libc::c_int,
    mut max: uchar_t,
    mut id: uchar_t,
) -> *mut libc::c_char {
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    if kvarr.is_null() {
        return 0 as *mut libc::c_char;
    }
    let mut r: libc::c_int = 0 as libc::c_int;
    while r < max as libc::c_int && (*kvarr.offset(r as isize)).key != 0 {
        if (*kvarr.offset(r as isize)).key == key {
            if id as libc::c_int == 2 as libc::c_int {
                return pluginstr.offset((*kvarr.offset(r as isize)).off as isize);
            }
            val = bmstr.offset((*kvarr.offset(r as isize)).off as isize);
            convert_tilde(val, g_buf.as_mut_ptr());
            return abspath(
                if *val.offset(0 as libc::c_int as isize) as libc::c_int == '~' as i32 {
                    g_buf.as_mut_ptr()
                } else {
                    val
                },
                0 as *const libc::c_char,
                buf,
            );
        }
        r += 1;
        r;
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn get_kv_key(
    mut kvarr: *mut kv,
    mut val: *mut libc::c_char,
    mut max: uchar_t,
    mut id: uchar_t,
) -> libc::c_int {
    if kvarr.is_null() {
        return -(1 as libc::c_int);
    }
    if id as libc::c_int != 11 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let mut r: libc::c_int = 0 as libc::c_int;
    while r < max as libc::c_int && (*kvarr.offset(r as isize)).key != 0 {
        if (if *orderstr.offset((*kvarr.offset(r as isize)).off as isize) as libc::c_int
            != *val as libc::c_int
        {
            -(1 as libc::c_int)
        } else {
            strcmp(orderstr.offset((*kvarr.offset(r as isize)).off as isize), val)
        }) == 0 as libc::c_int
        {
            return (*kvarr.offset(r as isize)).key;
        }
        r += 1;
        r;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn resetdircolor(mut flags: libc::c_int) {
    if g_state.dircolor() as libc::c_int != 0 && flags & 0x1 as libc::c_int == 0 {
        wattr_off(
            stdscr,
            ((cfg.curctx() as libc::c_int + 1 as libc::c_int) as chtype)
                << 0 as libc::c_int + 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int
                | (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int,
            0 as *mut libc::c_void,
        );
        g_state.set_dircolor(0 as libc::c_int as uint_t);
    }
}
unsafe extern "C" fn unescape(
    mut str: *const libc::c_char,
    mut maxcols: uint_t,
) -> *mut wchar_t {
    let wbuf: *mut wchar_t = g_buf.as_mut_ptr() as *mut wchar_t;
    let mut buf: *mut wchar_t = wbuf;
    let mut len: size_t = mbstowcs(wbuf, str, maxcols as size_t);
    len = wcswidth(wbuf, len) as size_t;
    if len >= maxcols as libc::c_ulong {
        let mut lencount: size_t = maxcols as size_t;
        while len > maxcols as libc::c_ulong {
            lencount = lencount.wrapping_sub(1);
            len = wcswidth(wbuf, lencount) as size_t;
        }
        *wbuf.offset(lencount as isize) = '\0' as i32;
    }
    while *buf != 0 {
        if *buf <= '\u{1f}' as i32 || *buf == '\u{7f}' as i32 {
            *buf = '?' as i32;
        }
        buf = buf.offset(1);
        buf;
    }
    return wbuf;
}
unsafe extern "C" fn get_size(
    mut size: off_t,
    mut pval: *mut off_t,
    mut comp: libc::c_int,
) -> off_t {
    let mut rem: off_t = *pval;
    let mut quo: off_t = rem / 10 as libc::c_int as libc::c_long;
    if rem - quo * 10 as libc::c_int as libc::c_long >= 5 as libc::c_int as libc::c_long
    {
        rem = quo + 1 as libc::c_int as libc::c_long;
        if rem == comp as libc::c_long {
            size += 1;
            size;
            rem = 0 as libc::c_int as off_t;
        }
    } else {
        rem = quo;
    }
    *pval = rem;
    return size;
}
unsafe extern "C" fn coolsize(mut size: off_t) -> *mut libc::c_char {
    let U: *const libc::c_char = b"BKMGTPEZY\0" as *const u8 as *const libc::c_char;
    static mut size_buf: [libc::c_char; 12] = [0; 12];
    let mut rem: off_t = 0 as libc::c_int as off_t;
    let mut ret: size_t = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    while size >= 1024 as libc::c_int as libc::c_long {
        rem = size & 0x3ff as libc::c_int as libc::c_long;
        size >>= 10 as libc::c_int;
        i += 1;
        i;
    }
    if i == 1 as libc::c_int {
        rem = rem * 1000 as libc::c_int as libc::c_long >> 10 as libc::c_int;
        rem /= 10 as libc::c_int as libc::c_long;
        size = get_size(size, &mut rem, 10 as libc::c_int);
    } else if i == 2 as libc::c_int {
        rem = rem * 1000 as libc::c_int as libc::c_long >> 10 as libc::c_int;
        size = get_size(size, &mut rem, 100 as libc::c_int);
    } else if i > 2 as libc::c_int {
        rem = rem * 10000 as libc::c_int as libc::c_long >> 10 as libc::c_int;
        size = get_size(size, &mut rem, 1000 as libc::c_int);
    }
    if i > 0 as libc::c_int && i < 6 as libc::c_int && rem != 0 {
        ret = xstrsncpy(
            size_buf.as_mut_ptr(),
            xitoa(size as uint_t),
            12 as libc::c_int as size_t,
        );
        size_buf[ret.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = '.' as i32 as libc::c_char;
        let mut frac: *mut libc::c_char = xitoa(rem as uint_t);
        let mut toprint: size_t = (if i > 3 as libc::c_int {
            3 as libc::c_int
        } else {
            i
        }) as size_t;
        let mut len: size_t = xstrlen(frac);
        if len < toprint {
            size_buf[ret.wrapping_add(2 as libc::c_int as libc::c_ulong)
                as usize] = '0' as i32 as libc::c_char;
            size_buf[ret.wrapping_add(1 as libc::c_int as libc::c_ulong)
                as usize] = size_buf[ret.wrapping_add(2 as libc::c_int as libc::c_ulong)
                as usize];
            size_buf[ret
                as usize] = size_buf[ret.wrapping_add(1 as libc::c_int as libc::c_ulong)
                as usize];
            xstrsncpy(
                size_buf
                    .as_mut_ptr()
                    .offset(ret as isize)
                    .offset(toprint.wrapping_sub(len) as isize),
                frac,
                len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        } else {
            xstrsncpy(
                size_buf.as_mut_ptr().offset(ret as isize),
                frac,
                toprint.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        }
        ret = (ret as libc::c_ulong).wrapping_add(toprint) as size_t as size_t;
    } else {
        ret = xstrsncpy(
            size_buf.as_mut_ptr(),
            if size != 0 {
                xitoa(size as uint_t) as *const libc::c_char
            } else {
                b"0\0" as *const u8 as *const libc::c_char
            },
            12 as libc::c_int as size_t,
        );
        ret = ret.wrapping_sub(1);
        ret;
    }
    size_buf[ret as usize] = *U.offset(i as isize);
    size_buf[ret.wrapping_add(1 as libc::c_int as libc::c_ulong)
        as usize] = '\0' as i32 as libc::c_char;
    return size_buf.as_mut_ptr();
}
unsafe extern "C" fn get_lsperms(mut mode: mode_t) -> *mut libc::c_char {
    static mut rwx: [*const libc::c_char; 8] = [
        b"---\0" as *const u8 as *const libc::c_char,
        b"--x\0" as *const u8 as *const libc::c_char,
        b"-w-\0" as *const u8 as *const libc::c_char,
        b"-wx\0" as *const u8 as *const libc::c_char,
        b"r--\0" as *const u8 as *const libc::c_char,
        b"r-x\0" as *const u8 as *const libc::c_char,
        b"rw-\0" as *const u8 as *const libc::c_char,
        b"rwx\0" as *const u8 as *const libc::c_char,
    ];
    static mut bits: [libc::c_char; 11] = [
        '\0' as i32 as libc::c_char,
        0,
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
    match mode & 0o170000 as libc::c_int as libc::c_uint {
        32768 => {
            bits[0 as libc::c_int as usize] = '-' as i32 as libc::c_char;
        }
        16384 => {
            bits[0 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
        }
        40960 => {
            bits[0 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
        }
        49152 => {
            bits[0 as libc::c_int as usize] = 's' as i32 as libc::c_char;
        }
        4096 => {
            bits[0 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
        }
        24576 => {
            bits[0 as libc::c_int as usize] = 'b' as i32 as libc::c_char;
        }
        8192 => {
            bits[0 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
        }
        _ => {
            bits[0 as libc::c_int as usize] = '?' as i32 as libc::c_char;
        }
    }
    xstrsncpy(
        &mut *bits.as_mut_ptr().offset(1 as libc::c_int as isize),
        rwx[(mode >> 6 as libc::c_int & 7 as libc::c_int as libc::c_uint) as usize],
        4 as libc::c_int as size_t,
    );
    xstrsncpy(
        &mut *bits.as_mut_ptr().offset(4 as libc::c_int as isize),
        rwx[(mode >> 3 as libc::c_int & 7 as libc::c_int as libc::c_uint) as usize],
        4 as libc::c_int as size_t,
    );
    xstrsncpy(
        &mut *bits.as_mut_ptr().offset(7 as libc::c_int as isize),
        rwx[(mode & 7 as libc::c_int as libc::c_uint) as usize],
        4 as libc::c_int as size_t,
    );
    if mode & 0o4000 as libc::c_int as libc::c_uint != 0 {
        bits[3 as libc::c_int
            as usize] = (if mode & 0o100 as libc::c_int as libc::c_uint != 0 {
            's' as i32
        } else {
            'S' as i32
        }) as libc::c_char;
    }
    if mode & 0o2000 as libc::c_int as libc::c_uint != 0 {
        bits[6 as libc::c_int
            as usize] = (if mode & 0o10 as libc::c_int as libc::c_uint != 0 {
            's' as i32
        } else {
            'l' as i32
        }) as libc::c_char;
    }
    if mode & 0o1000 as libc::c_int as libc::c_uint != 0 {
        bits[9 as libc::c_int
            as usize] = (if mode & 0o1 as libc::c_int as libc::c_uint != 0 {
            't' as i32
        } else {
            'T' as i32
        }) as libc::c_char;
    }
    return bits.as_mut_ptr();
}
unsafe extern "C" fn print_time(mut timep: *const time_t, flags: uchar_t) {
    let mut t: tm = tm {
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
    if flags as libc::c_int & 0x40 as libc::c_int != 0 {
        wattr_on(
            stdscr,
            (1 as libc::c_uint) << 10 as libc::c_int + 8 as libc::c_int,
            0 as *mut libc::c_void,
        );
    }
    localtime_r(timep, &mut t);
    printw(
        b"%s-%02d-%02d %02d:%02d\0" as *const u8 as *const libc::c_char,
        xitoa((t.tm_year + 1900 as libc::c_int) as uint_t),
        t.tm_mon + 1 as libc::c_int,
        t.tm_mday,
        t.tm_hour,
        t.tm_min,
    );
    if flags as libc::c_int & 0x40 as libc::c_int != 0 {
        wattr_off(
            stdscr,
            (1 as libc::c_uint) << 10 as libc::c_int + 8 as libc::c_int,
            0 as *mut libc::c_void,
        );
    }
}
unsafe extern "C" fn get_detail_ind(mode: mode_t) -> libc::c_char {
    match mode & 0o170000 as libc::c_int as libc::c_uint {
        16384 | 32768 => return ' ' as i32 as libc::c_char,
        40960 => return '@' as i32 as libc::c_char,
        49152 => return '=' as i32 as libc::c_char,
        4096 => return '|' as i32 as libc::c_char,
        24576 => return 'b' as i32 as libc::c_char,
        8192 => return 'c' as i32 as libc::c_char,
        _ => {}
    }
    return '?' as i32 as libc::c_char;
}
unsafe extern "C" fn get_color_pair_name_ind(
    mut ent: *const entry,
    mut pind: *mut libc::c_char,
    mut pattr: *mut libc::c_int,
) -> uchar_t {
    match (*ent).mode & 0o170000 as libc::c_int as libc::c_uint {
        32768 => {
            if (*ent).size == 0 {
                if (*ent).mode & 0o100 as libc::c_int as libc::c_uint != 0 {
                    *pind = '*' as i32 as libc::c_char;
                }
                return (4 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int) as uchar_t;
            }
            if ((*ent).c2rust_unnamed).flags() as libc::c_int & 0x2 as libc::c_int != 0 {
                if (*ent).mode & 0o100 as libc::c_int as libc::c_uint != 0 {
                    *pind = '*' as i32 as libc::c_char;
                }
                return (4 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int) as uchar_t;
            }
            if (*ent).mode & 0o100 as libc::c_int as libc::c_uint != 0 {
                *pind = '*' as i32 as libc::c_char;
                return (4 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int) as uchar_t;
            }
            return (4 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int) as uchar_t;
        }
        16384 => {
            *pind = '/' as i32 as libc::c_char;
            if g_state.oldcolor() != 0 {
                return (4 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int) as uchar_t;
            }
            *pattr = (*pattr as libc::c_uint
                | (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int)
                as libc::c_int;
            return (if g_state.dirctx() as libc::c_int != 0 {
                cfg.curctx() as libc::c_int + 1 as libc::c_int
            } else {
                4 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            }) as uchar_t;
        }
        40960 => {
            if ((*ent).c2rust_unnamed).flags() as libc::c_int & 0x1 as libc::c_int != 0 {
                *pind = '/' as i32 as libc::c_char;
                *pattr = (*pattr as libc::c_uint
                    | if g_state.oldcolor() as libc::c_int != 0 {
                        (1 as libc::c_uint) << 12 as libc::c_int + 8 as libc::c_int
                    } else {
                        (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int
                    }) as libc::c_int;
            } else {
                *pind = '@' as i32 as libc::c_char;
                if g_state.oldcolor() != 0 {
                    *pattr = (*pattr as libc::c_uint
                        | (1 as libc::c_uint) << 12 as libc::c_int + 8 as libc::c_int)
                        as libc::c_int;
                }
            }
            if g_state.oldcolor() == 0 || cfg.showdetail() as libc::c_int != 0 {
                return (if ((*ent).c2rust_unnamed).flags() as libc::c_int
                    & 0x4 as libc::c_int != 0
                {
                    4 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                        + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                        + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                        + 1 as libc::c_int
                } else {
                    4 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                        + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                        + 1 as libc::c_int + 1 as libc::c_int
                }) as uchar_t;
            }
            return 0 as libc::c_int as uchar_t;
        }
        49152 => {
            *pind = '=' as i32 as libc::c_char;
            return (4 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int) as uchar_t;
        }
        4096 => {
            *pind = '|' as i32 as libc::c_char;
            return (4 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int) as uchar_t;
        }
        24576 => return (4 as libc::c_int + 1 as libc::c_int) as uchar_t,
        8192 => {
            return (4 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int) as uchar_t;
        }
        _ => {}
    }
    *pind = '?' as i32 as libc::c_char;
    return (4 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
        + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
        + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
        + 1 as libc::c_int) as uchar_t;
}
unsafe extern "C" fn printent(
    mut ent: *const entry,
    mut namecols: uint_t,
    mut sel: bool,
) {
    let mut ind: libc::c_char = '\0' as i32 as libc::c_char;
    let mut attrs: libc::c_int = 0;
    if cfg.showdetail() != 0 {
        let mut type_0: libc::c_int = ((*ent).mode
            & 0o170000 as libc::c_int as libc::c_uint) as libc::c_int;
        let mut perms: [libc::c_char; 6] = [
            ' ' as i32 as libc::c_char,
            ' ' as i32 as libc::c_char,
            ('0' as i32 as libc::c_uint)
                .wrapping_add(
                    (*ent).mode >> 6 as libc::c_int & 7 as libc::c_int as libc::c_uint,
                ) as libc::c_char,
            ('0' as i32 as libc::c_uint)
                .wrapping_add(
                    (*ent).mode >> 3 as libc::c_int & 7 as libc::c_int as libc::c_uint,
                ) as libc::c_char,
            ('0' as i32 as libc::c_uint)
                .wrapping_add((*ent).mode & 7 as libc::c_int as libc::c_uint)
                as libc::c_char,
            '\0' as i32 as libc::c_char,
        ];
        waddch(stdscr, ' ' as i32 as chtype);
        attrs = (if g_state.oldcolor() as libc::c_int != 0 {
            resetdircolor(((*ent).c2rust_unnamed).flags() as libc::c_int);
            (1 as libc::c_uint) << 12 as libc::c_int + 8 as libc::c_int
        } else if fcolors[(4 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            + 1 as libc::c_int + 1 as libc::c_int) as usize] != 0
        {
            ((4 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int) as chtype)
                << 0 as libc::c_int + 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int
        } else {
            0 as libc::c_int as libc::c_uint
        }) as libc::c_int;
        if attrs != 0 {
            wattr_on(stdscr, attrs as attr_t, 0 as *mut libc::c_void);
        }
        print_time(&(*ent).sec, ((*ent).c2rust_unnamed).flags() as uchar_t);
        printw(
            b"%s%9s \0" as *const u8 as *const libc::c_char,
            perms.as_mut_ptr(),
            if type_0 == 0o100000 as libc::c_int || type_0 == 0o40000 as libc::c_int {
                coolsize(
                    if cfg.blkorder() as libc::c_int != 0 {
                        (((*ent).c2rust_unnamed).blocks() as blkcnt_t)
                            << blk_shift as libc::c_int
                    } else {
                        (*ent).size
                    },
                )
            } else {
                type_0 = get_detail_ind((*ent).mode) as uchar_t as libc::c_int;
                &mut type_0 as *mut libc::c_int as *mut libc::c_char
            },
        );
        if attrs != 0 {
            wattr_off(stdscr, attrs as attr_t, 0 as *mut libc::c_void);
        }
    }
    attrs = 0 as libc::c_int;
    let mut color_pair: uchar_t = get_color_pair_name_ind(ent, &mut ind, &mut attrs);
    waddch(
        stdscr,
        if ((*ent).c2rust_unnamed).flags() as libc::c_int & 0x10 as libc::c_int != 0 {
            '+' as i32 as libc::c_uint
                | (1 as libc::c_uint) << 10 as libc::c_int + 8 as libc::c_int
                | (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int
        } else {
            ' ' as i32 as libc::c_uint
        },
    );
    if g_state.oldcolor() != 0 {
        resetdircolor(((*ent).c2rust_unnamed).flags() as libc::c_int);
    } else {
        if ((*ent).c2rust_unnamed).flags() as libc::c_int & 0x8 as libc::c_int != 0 {
            color_pair = (4 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int) as uchar_t;
        }
        if color_pair as libc::c_int != 0 && fcolors[color_pair as usize] != 0 {
            attrs = (attrs as libc::c_uint
                | (color_pair as chtype) << 0 as libc::c_int + 8 as libc::c_int
                    & ((1 as libc::c_uint) << 8 as libc::c_int)
                        .wrapping_sub(1 as libc::c_uint)
                        << 0 as libc::c_int + 8 as libc::c_int) as libc::c_int;
        }
    }
    if sel {
        attrs = (attrs as libc::c_uint
            | (1 as libc::c_uint) << 10 as libc::c_int + 8 as libc::c_int)
            as libc::c_int;
    }
    if attrs != 0 {
        wattr_on(stdscr, attrs as attr_t, 0 as *mut libc::c_void);
    }
    if ind == 0 {
        namecols = namecols.wrapping_add(1);
        namecols;
    }
    waddnwstr(stdscr, unescape((*ent).name, namecols), -(1 as libc::c_int));
    if attrs != 0 {
        wattr_off(stdscr, attrs as attr_t, 0 as *mut libc::c_void);
    }
    if ind != 0 {
        waddch(stdscr, ind as chtype);
    }
}
unsafe extern "C" fn savecurctx(
    mut path: *mut libc::c_char,
    mut curname: *mut libc::c_char,
    mut nextctx: libc::c_int,
) {
    let mut tmpcfg: settings = cfg;
    let mut ctxr: *mut context = &mut *g_ctx.as_mut_ptr().offset(nextctx as isize)
        as *mut context;
    if !curname.is_null() {
        xstrsncpy(
            (g_ctx[tmpcfg.curctx() as usize].c_name).as_mut_ptr(),
            curname,
            (255 as libc::c_int + 1 as libc::c_int) as size_t,
        );
    } else {
        g_ctx[tmpcfg.curctx() as usize]
            .c_name[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }
    g_ctx[tmpcfg.curctx() as usize].c_cfg = tmpcfg;
    if ((*ctxr).c_cfg).ctxactive() != 0 {
        tmpcfg = (*ctxr).c_cfg;
        if !order.is_null() {
            cfgsort[4 as libc::c_int as usize] = cfgsort[nextctx as usize];
            cfgsort[nextctx as usize] = '0' as i32 as uchar_t;
        }
    } else {
        ((*ctxr).c_cfg).set_ctxactive(1 as libc::c_int as uint_t);
        xstrsncpy(((*ctxr).c_path).as_mut_ptr(), path, 4096 as libc::c_int as size_t);
        (*ctxr).c_fltr[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        (*ctxr)
            .c_fltr[0 as libc::c_int
            as usize] = (*ctxr).c_fltr[1 as libc::c_int as usize];
        (*ctxr)
            .c_name[0 as libc::c_int
            as usize] = (*ctxr).c_fltr[0 as libc::c_int as usize];
        (*ctxr)
            .c_last[0 as libc::c_int
            as usize] = (*ctxr).c_name[0 as libc::c_int as usize];
        (*ctxr).c_cfg = tmpcfg;
        if cfgsort[cfg.curctx() as usize] as libc::c_int == 'z' as i32 {
            cfgsort[nextctx as usize] = 'z' as i32 as uchar_t;
        }
    }
    tmpcfg.set_curctx(nextctx as uint_t);
    cfg = tmpcfg;
}
unsafe extern "C" fn save_session(
    mut sname: *const libc::c_char,
    mut presel: *mut libc::c_int,
) {
    let mut current_block: u64;
    let mut fd: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut header: session_header_t = {
        let mut init = session_header_t {
            ver: 0 as libc::c_int as size_t,
            pathln: [0; 4],
            lastln: [0; 4],
            nameln: [0; 4],
            fltrln: [0; 4],
        };
        init
    };
    let mut status: bool = 0 as libc::c_int != 0;
    let mut ssnpath: [libc::c_char; 4096] = [0; 4096];
    let mut spath: [libc::c_char; 4096] = [0; 4096];
    header.ver = 1 as libc::c_int as size_t;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if (g_ctx[i as usize].c_cfg).ctxactive() != 0 {
            if cfg.curctx() as libc::c_int == i && ndents != 0 {
                xstrsncpy(
                    (g_ctx[i as usize].c_name).as_mut_ptr(),
                    (*pdents.offset(cur as isize)).name,
                    (255 as libc::c_int + 1 as libc::c_int) as size_t,
                );
            }
            header
                .pathln[i
                as usize] = (strnlen(
                (g_ctx[i as usize].c_path).as_mut_ptr(),
                4096 as libc::c_int as size_t,
            ))
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            header
                .lastln[i
                as usize] = (strnlen(
                (g_ctx[i as usize].c_last).as_mut_ptr(),
                4096 as libc::c_int as size_t,
            ))
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            header
                .nameln[i
                as usize] = (strnlen(
                (g_ctx[i as usize].c_name).as_mut_ptr(),
                255 as libc::c_int as size_t,
            ))
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            header.fltrln[i as usize] = 48 as libc::c_int as size_t;
        }
        i += 1;
        i;
    }
    mkpath(cfgpath, toks[1 as libc::c_int as usize], ssnpath.as_mut_ptr());
    mkpath(ssnpath.as_mut_ptr(), sname, spath.as_mut_ptr());
    fd = open(
        spath.as_mut_ptr(),
        0o100 as libc::c_int | 0o1 as libc::c_int | 0o1000 as libc::c_int,
        0o666 as libc::c_int,
    );
    if fd == -(1 as libc::c_int) {
        printwait(messages[23 as libc::c_int as usize], presel);
        return;
    }
    if !(write(
        fd,
        &mut header as *mut session_header_t as *const libc::c_void,
        ::std::mem::size_of::<session_header_t>() as libc::c_ulong,
    ) != ::std::mem::size_of::<session_header_t>() as libc::c_ulong as ssize_t
        || write(
            fd,
            &mut cfg as *mut settings as *const libc::c_void,
            ::std::mem::size_of::<settings>() as libc::c_ulong,
        ) != ::std::mem::size_of::<settings>() as libc::c_ulong as ssize_t)
    {
        i = 0 as libc::c_int;
        loop {
            if !(i < 4 as libc::c_int) {
                current_block = 12124785117276362961;
                break;
            }
            if write(
                fd,
                &mut (*g_ctx.as_mut_ptr().offset(i as isize)).c_cfg as *mut settings
                    as *const libc::c_void,
                ::std::mem::size_of::<settings>() as libc::c_ulong,
            ) != ::std::mem::size_of::<settings>() as libc::c_ulong as ssize_t
                || write(
                    fd,
                    &mut (*g_ctx.as_mut_ptr().offset(i as isize)).color as *mut uint_t
                        as *const libc::c_void,
                    ::std::mem::size_of::<uint_t>() as libc::c_ulong,
                ) != ::std::mem::size_of::<uint_t>() as libc::c_ulong as ssize_t
                || header.nameln[i as usize] > 0 as libc::c_int as libc::c_ulong
                    && write(
                        fd,
                        (g_ctx[i as usize].c_name).as_mut_ptr() as *const libc::c_void,
                        header.nameln[i as usize],
                    ) != header.nameln[i as usize] as ssize_t
                || header.lastln[i as usize] > 0 as libc::c_int as libc::c_ulong
                    && write(
                        fd,
                        (g_ctx[i as usize].c_last).as_mut_ptr() as *const libc::c_void,
                        header.lastln[i as usize],
                    ) != header.lastln[i as usize] as ssize_t
                || header.fltrln[i as usize] > 0 as libc::c_int as libc::c_ulong
                    && write(
                        fd,
                        (g_ctx[i as usize].c_fltr).as_mut_ptr() as *const libc::c_void,
                        header.fltrln[i as usize],
                    ) != header.fltrln[i as usize] as ssize_t
                || header.pathln[i as usize] > 0 as libc::c_int as libc::c_ulong
                    && write(
                        fd,
                        (g_ctx[i as usize].c_path).as_mut_ptr() as *const libc::c_void,
                        header.pathln[i as usize],
                    ) != header.pathln[i as usize] as ssize_t
            {
                current_block = 12792864740726188671;
                break;
            }
            i += 1;
            i;
        }
        match current_block {
            12792864740726188671 => {}
            _ => {
                status = 1 as libc::c_int != 0;
            }
        }
    }
    close(fd);
    if !status {
        printwait(messages[5 as libc::c_int as usize], presel);
    }
}
unsafe extern "C" fn load_session(
    mut sname: *const libc::c_char,
    mut path: *mut *mut libc::c_char,
    mut lastdir: *mut *mut libc::c_char,
    mut lastname: *mut *mut libc::c_char,
    mut restore: bool,
) -> bool {
    let mut current_block: u64;
    let mut fd: libc::c_int = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut header: session_header_t = session_header_t {
        ver: 0,
        pathln: [0; 4],
        lastln: [0; 4],
        nameln: [0; 4],
        fltrln: [0; 4],
    };
    let mut has_loaded_dynamically: bool = !(!sname.is_null()
        || restore as libc::c_int != 0);
    let mut status: bool = !sname.is_null() && g_state.picker() as libc::c_int != 0;
    let mut ssnpath: [libc::c_char; 4096] = [0; 4096];
    let mut spath: [libc::c_char; 4096] = [0; 4096];
    mkpath(cfgpath, toks[1 as libc::c_int as usize], ssnpath.as_mut_ptr());
    if !restore {
        sname = if !sname.is_null() {
            sname
        } else {
            xreadline(0 as *const libc::c_char, messages[6 as libc::c_int as usize])
                as *const libc::c_char
        };
        if *sname.offset(0 as libc::c_int as isize) == 0 {
            return 0 as libc::c_int != 0;
        }
        mkpath(ssnpath.as_mut_ptr(), sname, spath.as_mut_ptr());
        if *sname.offset(0 as libc::c_int as isize) as libc::c_int == '@' as i32
            && *sname.offset(1 as libc::c_int as isize) == 0
        {
            has_loaded_dynamically = 0 as libc::c_int != 0;
        }
    } else {
        mkpath(
            ssnpath.as_mut_ptr(),
            b"@\0" as *const u8 as *const libc::c_char,
            spath.as_mut_ptr(),
        );
    }
    if has_loaded_dynamically {
        save_session(b"@\0" as *const u8 as *const libc::c_char, 0 as *mut libc::c_int);
    }
    fd = open(spath.as_mut_ptr(), 0 as libc::c_int, 0o666 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        if !status {
            printmsg(messages[23 as libc::c_int as usize]);
            xdelay(350000 as libc::c_int as useconds_t);
        }
        return 0 as libc::c_int != 0;
    }
    status = 0 as libc::c_int != 0;
    if !(read(
        fd,
        &mut header as *mut session_header_t as *mut libc::c_void,
        ::std::mem::size_of::<session_header_t>() as libc::c_ulong,
    ) != ::std::mem::size_of::<session_header_t>() as libc::c_ulong as ssize_t
        || header.ver != 1 as libc::c_int as libc::c_ulong
        || read(
            fd,
            &mut cfg as *mut settings as *mut libc::c_void,
            ::std::mem::size_of::<settings>() as libc::c_ulong,
        ) != ::std::mem::size_of::<settings>() as libc::c_ulong as ssize_t)
    {
        g_ctx[cfg.curctx() as usize]
            .c_fltr[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        g_ctx[cfg.curctx() as usize]
            .c_fltr[0 as libc::c_int
            as usize] = g_ctx[cfg.curctx() as usize].c_fltr[1 as libc::c_int as usize];
        g_ctx[cfg.curctx() as usize]
            .c_last[0 as libc::c_int
            as usize] = g_ctx[cfg.curctx() as usize].c_fltr[0 as libc::c_int as usize];
        g_ctx[cfg.curctx() as usize]
            .c_name[0 as libc::c_int
            as usize] = g_ctx[cfg.curctx() as usize].c_last[0 as libc::c_int as usize];
        loop {
            if !(i < 4 as libc::c_int) {
                current_block = 4495394744059808450;
                break;
            }
            if read(
                fd,
                &mut (*g_ctx.as_mut_ptr().offset(i as isize)).c_cfg as *mut settings
                    as *mut libc::c_void,
                ::std::mem::size_of::<settings>() as libc::c_ulong,
            ) != ::std::mem::size_of::<settings>() as libc::c_ulong as ssize_t
                || read(
                    fd,
                    &mut (*g_ctx.as_mut_ptr().offset(i as isize)).color as *mut uint_t
                        as *mut libc::c_void,
                    ::std::mem::size_of::<uint_t>() as libc::c_ulong,
                ) != ::std::mem::size_of::<uint_t>() as libc::c_ulong as ssize_t
                || header.nameln[i as usize] > 0 as libc::c_int as libc::c_ulong
                    && read(
                        fd,
                        (g_ctx[i as usize].c_name).as_mut_ptr() as *mut libc::c_void,
                        header.nameln[i as usize],
                    ) != header.nameln[i as usize] as ssize_t
                || header.lastln[i as usize] > 0 as libc::c_int as libc::c_ulong
                    && read(
                        fd,
                        (g_ctx[i as usize].c_last).as_mut_ptr() as *mut libc::c_void,
                        header.lastln[i as usize],
                    ) != header.lastln[i as usize] as ssize_t
                || header.fltrln[i as usize] > 0 as libc::c_int as libc::c_ulong
                    && read(
                        fd,
                        (g_ctx[i as usize].c_fltr).as_mut_ptr() as *mut libc::c_void,
                        header.fltrln[i as usize],
                    ) != header.fltrln[i as usize] as ssize_t
                || header.pathln[i as usize] > 0 as libc::c_int as libc::c_ulong
                    && read(
                        fd,
                        (g_ctx[i as usize].c_path).as_mut_ptr() as *mut libc::c_void,
                        header.pathln[i as usize],
                    ) != header.pathln[i as usize] as ssize_t
            {
                current_block = 16140734951812359075;
                break;
            }
            i += 1;
            i;
        }
        match current_block {
            16140734951812359075 => {}
            _ => {
                *path = (g_ctx[cfg.curctx() as usize].c_path).as_mut_ptr();
                *lastdir = (g_ctx[cfg.curctx() as usize].c_last).as_mut_ptr();
                *lastname = (g_ctx[cfg.curctx() as usize].c_name).as_mut_ptr();
                set_sort_flags('\0' as i32);
                status = 1 as libc::c_int != 0;
            }
        }
    }
    close(fd);
    if !status {
        printmsg(messages[5 as libc::c_int as usize]);
        xdelay(350000 as libc::c_int as useconds_t);
    } else if restore {
        unlink(spath.as_mut_ptr());
    }
    return status;
}
unsafe extern "C" fn get_free_ctx() -> uchar_t {
    let mut r: uchar_t = cfg.curctx() as uchar_t;
    loop {
        r = (r as libc::c_int + 1 as libc::c_int & !(4 as libc::c_int)) as uchar_t;
        if !((g_ctx[r as usize].c_cfg).ctxactive() as libc::c_int != 0
            && r as libc::c_int != cfg.curctx() as libc::c_int)
        {
            break;
        }
    }
    return r;
}
unsafe extern "C" fn set_smart_ctx(
    mut ctx: libc::c_int,
    mut nextpath: *mut libc::c_char,
    mut path: *mut *mut libc::c_char,
    mut file: *mut libc::c_char,
    mut lastname: *mut *mut libc::c_char,
    mut lastdir: *mut *mut libc::c_char,
) {
    if ctx == '+' as i32 {
        ctx = get_free_ctx() as libc::c_int + 1 as libc::c_int;
    }
    if ctx == 0 as libc::c_int || ctx == cfg.curctx() as libc::c_int + 1 as libc::c_int {
        xstrsncpy(*lastdir, *path, 4096 as libc::c_int as size_t);
        xstrsncpy(*path, nextpath, 4096 as libc::c_int as size_t);
    } else {
        ctx -= 1;
        ctx;
        (g_ctx[ctx as usize].c_cfg).set_ctxactive(0 as libc::c_int as uint_t);
        savecurctx(nextpath, file, ctx);
        *path = (g_ctx[ctx as usize].c_path).as_mut_ptr();
        *lastdir = (g_ctx[ctx as usize].c_last).as_mut_ptr();
        *lastname = (g_ctx[ctx as usize].c_name).as_mut_ptr();
    };
}
unsafe extern "C" fn get_output(
    mut file: *mut libc::c_char,
    mut arg1: *mut libc::c_char,
    mut arg2: *mut libc::c_char,
    mut fdout: libc::c_int,
    mut multi: bool,
    mut page: bool,
) -> bool {
    let mut pid: pid_t = 0;
    let mut pipefd: [libc::c_int; 2] = [0; 2];
    let mut index: libc::c_int = 0 as libc::c_int;
    let mut flags: libc::c_int = 0;
    let mut ret: bool = 0 as libc::c_int != 0;
    let mut tmpfile: bool = fdout == -(1 as libc::c_int) && page as libc::c_int != 0;
    let mut argv: [*mut libc::c_char; 10] = [
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
    ];
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut len: ssize_t = 0;
    if tmpfile {
        fdout = create_tmp_file();
        if fdout == -(1 as libc::c_int) {
            return 0 as libc::c_int != 0;
        }
    }
    if multi {
        cmd = parseargs(file, argv.as_mut_ptr(), &mut index);
        if cmd.is_null() {
            return 0 as libc::c_int != 0;
        }
    } else {
        let fresh25 = index;
        index = index + 1;
        argv[fresh25 as usize] = file;
    }
    argv[index as usize] = arg1;
    index += 1;
    argv[index as usize] = arg2;
    if pipe(pipefd.as_mut_ptr()) == -(1 as libc::c_int) {
        free(cmd as *mut libc::c_void);
        printerr(4395 as libc::c_int);
    }
    index = 0 as libc::c_int;
    while index < 2 as libc::c_int {
        flags = fcntl(pipefd[index as usize], 3 as libc::c_int, 0 as libc::c_int);
        flags |= 0o4000 as libc::c_int;
        fcntl(pipefd[index as usize], 4 as libc::c_int, flags);
        index += 1;
        index;
    }
    pid = fork();
    if pid == 0 as libc::c_int {
        close(pipefd[0 as libc::c_int as usize]);
        dup2(pipefd[1 as libc::c_int as usize], 1 as libc::c_int);
        dup2(pipefd[1 as libc::c_int as usize], 2 as libc::c_int);
        close(pipefd[1 as libc::c_int as usize]);
        execvp(*argv.as_mut_ptr(), argv.as_mut_ptr() as *const *mut libc::c_char);
        _exit(0 as libc::c_int);
    }
    waitpid(pid, 0 as *mut libc::c_int, 0 as libc::c_int);
    close(pipefd[1 as libc::c_int as usize]);
    free(cmd as *mut libc::c_void);
    loop {
        len = read(
            pipefd[0 as libc::c_int as usize],
            g_buf.as_mut_ptr() as *mut libc::c_void,
            (4096 as libc::c_int
                + ((255 as libc::c_int + 1 as libc::c_int) << 1 as libc::c_int)
                - 1 as libc::c_int) as size_t,
        );
        if !(len > 0 as libc::c_int as libc::c_long) {
            break;
        }
        ret = 1 as libc::c_int != 0;
        if fdout == -(1 as libc::c_int) {
            break;
        }
        if write(fdout, g_buf.as_mut_ptr() as *const libc::c_void, len as size_t) != len
        {
            break;
        }
    }
    close(pipefd[0 as libc::c_int as usize]);
    if !page {
        return ret;
    }
    if tmpfile {
        close(fdout);
        close(fd);
    }
    spawn(
        pager,
        g_tmpfpath.as_mut_ptr(),
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        (0x8 as libc::c_int | 0x1 as libc::c_int | 0x100 as libc::c_int) as ushort_t,
    );
    if tmpfile {
        unlink(g_tmpfpath.as_mut_ptr());
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn show_stats(mut fpath: *mut libc::c_char) -> bool {
    static mut cmds: [*mut libc::c_char; 3] = [
        b"file -biL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"file -b\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"stat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    let mut r: size_t = (::std::mem::size_of::<[*mut libc::c_char; 3]>()
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong);
    let mut fd: libc::c_int = create_tmp_file();
    if fd == -(1 as libc::c_int) {
        return 0 as libc::c_int != 0;
    }
    while r != 0 {
        r = r.wrapping_sub(1);
        get_output(
            cmds[r as usize],
            fpath,
            0 as *mut libc::c_char,
            fd,
            1 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
    }
    close(fd);
    spawn(
        pager,
        g_tmpfpath.as_mut_ptr(),
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        (0x8 as libc::c_int | 0x1 as libc::c_int | 0x100 as libc::c_int) as ushort_t,
    );
    unlink(g_tmpfpath.as_mut_ptr());
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn xchmod(mut fpath: *const libc::c_char, mut mode: mode_t) -> bool {
    if 0o100 as libc::c_int as libc::c_uint & mode != 0 {
        mode &= !(0o111 as libc::c_int) as libc::c_uint;
    } else {
        mode |= 0o111 as libc::c_int as libc::c_uint;
    };
    return chmod(fpath, mode) == 0 as libc::c_int;
}
unsafe extern "C" fn get_fs_info(
    mut path: *const libc::c_char,
    mut type_0: uchar_t,
) -> size_t {
    let mut svb: statvfs = statvfs {
        f_bsize: 0,
        f_frsize: 0,
        f_blocks: 0,
        f_bfree: 0,
        f_bavail: 0,
        f_files: 0,
        f_ffree: 0,
        f_favail: 0,
        f_fsid: 0,
        f_flag: 0,
        f_namemax: 0,
        __f_spare: [0; 6],
    };
    if statvfs(path, &mut svb) == -(1 as libc::c_int) {
        return 0 as libc::c_int as size_t;
    }
    if type_0 as libc::c_int == 0 as libc::c_int {
        return svb.f_bavail << ffs((svb.f_frsize >> 1 as libc::c_int) as libc::c_int);
    }
    if type_0 as libc::c_int == 1 as libc::c_int {
        return (svb.f_blocks).wrapping_sub(svb.f_bfree)
            << ffs((svb.f_frsize >> 1 as libc::c_int) as libc::c_int);
    }
    return svb.f_blocks << ffs((svb.f_frsize >> 1 as libc::c_int) as libc::c_int);
}
unsafe extern "C" fn xmktree(mut path: *mut libc::c_char, mut dir: bool) -> bool {
    let mut p: *mut libc::c_char = path;
    let mut slash: *mut libc::c_char = path;
    if p.is_null() || *p == 0 {
        return 0 as libc::c_int != 0;
    }
    p = p.offset(1);
    p;
    while *p as libc::c_int != '\0' as i32 {
        if *p as libc::c_int == '/' as i32 {
            slash = p;
            *p = '\0' as i32 as libc::c_char;
            if mkdir(path, 0o777 as libc::c_int as __mode_t) == -(1 as libc::c_int)
                && *__errno_location() != 17 as libc::c_int
            {
                *slash = '/' as i32 as libc::c_char;
                return 0 as libc::c_int != 0;
            }
            *slash = '/' as i32 as libc::c_char;
            p = p.offset(1);
            p;
        } else {
            p = p.offset(1);
            p;
        }
    }
    if dir {
        if mkdir(path, 0o777 as libc::c_int as __mode_t) == -(1 as libc::c_int)
            && *__errno_location() != 17 as libc::c_int
        {
            return 0 as libc::c_int != 0;
        }
    } else {
        let mut fd: libc::c_int = open(path, 0o100 as libc::c_int, 0o666 as libc::c_int);
        if fd == -(1 as libc::c_int) && *__errno_location() != 17 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
        close(fd);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn handle_archive(
    mut fpath: *mut libc::c_char,
    mut op: libc::c_char,
) -> bool {
    let mut arg: [libc::c_char; 5] = *::std::mem::transmute::<
        &[u8; 5],
        &mut [libc::c_char; 5],
    >(b"-tvf\0");
    let mut util: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outdir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x_to: bool = 0 as libc::c_int != 0;
    let mut is_atool: bool = g_state.usebsdtar() == 0
        && getutil(utils[1 as libc::c_int as usize]) as libc::c_int != 0;
    if op as libc::c_int == 'x' as i32 {
        outdir = xreadline(
            if is_atool as libc::c_int != 0 {
                b".\0" as *const u8 as *const libc::c_char
            } else {
                xbasename(fpath) as *const libc::c_char
            },
            messages[19 as libc::c_int as usize],
        );
        if outdir.is_null() || *outdir == 0 {
            printwait(messages[4 as libc::c_int as usize], 0 as *mut libc::c_int);
            return 0 as libc::c_int != 0;
        }
        if !(*outdir as libc::c_int == '.' as i32
            && *outdir.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32)
        {
            if !xmktree(outdir, 1 as libc::c_int != 0)
                || chdir(outdir) == -(1 as libc::c_int)
            {
                printwait(strerror(*__errno_location()), 0 as *mut libc::c_int);
                return 0 as libc::c_int != 0;
            }
            outdir = getcwd(0 as *mut libc::c_char, 0 as libc::c_int as size_t);
            x_to = 1 as libc::c_int != 0;
        }
    }
    if is_atool {
        util = utils[1 as libc::c_int as usize];
        arg[1 as libc::c_int as usize] = op;
        arg[2 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    } else if getutil(utils[2 as libc::c_int as usize]) {
        util = utils[2 as libc::c_int as usize];
        if op as libc::c_int == 'x' as i32 {
            arg[1 as libc::c_int as usize] = op;
        }
    } else if is_suffix(fpath, b".zip\0" as *const u8 as *const libc::c_char) {
        util = utils[3 as libc::c_int as usize];
        arg[1 as libc::c_int
            as usize] = (if op as libc::c_int == 'l' as i32 {
            'v' as i32
        } else {
            '\0' as i32
        }) as libc::c_char;
        arg[2 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    } else {
        util = utils[4 as libc::c_int as usize];
        if op as libc::c_int == 'x' as i32 {
            arg[1 as libc::c_int as usize] = op;
        }
    }
    if op as libc::c_int == 'x' as i32 {
        spawn(
            util,
            arg.as_mut_ptr(),
            fpath,
            0 as *mut libc::c_char,
            (0x8 as libc::c_int | 0x1 as libc::c_int) as ushort_t,
        );
    } else {
        get_output(
            util,
            arg.as_mut_ptr(),
            fpath,
            -(1 as libc::c_int),
            1 as libc::c_int != 0,
            1 as libc::c_int != 0,
        );
    }
    if x_to {
        if chdir(xdirname(fpath)) == -(1 as libc::c_int) {
            printwait(strerror(*__errno_location()), 0 as *mut libc::c_int);
            free(outdir as *mut libc::c_void);
            return 0 as libc::c_int != 0;
        }
        xstrsncpy(fpath, outdir, 4096 as libc::c_int as size_t);
        free(outdir as *mut libc::c_void);
    } else if op as libc::c_int == 'x' as i32 {
        *fpath.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn visit_parent(
    mut path: *mut libc::c_char,
    mut newpath: *mut libc::c_char,
    mut presel: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    if *path.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
        && *path.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        if cfg.filtermode() as libc::c_int != 0 && !presel.is_null() {
            *presel = '/' as i32;
        }
        return 0 as *mut libc::c_char;
    }
    if !newpath.is_null() {
        xstrsncpy(newpath, path, 4096 as libc::c_int as size_t);
    } else {
        newpath = path;
    }
    dir = xdirname(newpath);
    if chdir(dir) == -(1 as libc::c_int) {
        printwait(strerror(*__errno_location()), presel);
        return 0 as *mut libc::c_char;
    }
    return dir;
}
unsafe extern "C" fn valid_parent(
    mut path: *mut libc::c_char,
    mut lastname: *mut libc::c_char,
) {
    xstrsncpy(
        lastname,
        xbasename(path),
        (255 as libc::c_int + 1 as libc::c_int) as size_t,
    );
    while !(*path.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
        && *path.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32)
    {
        if !(visit_parent(path, 0 as *mut libc::c_char, 0 as *mut libc::c_int)).is_null()
        {
            break;
        }
    }
    printwait(strerror(*__errno_location()), 0 as *mut libc::c_int);
    xdelay(350000 as libc::c_int as useconds_t);
}
unsafe extern "C" fn archive_mount(mut newpath: *mut libc::c_char) -> bool {
    let mut str: *mut libc::c_char = b"install archivemount\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmd: *mut libc::c_char = str.offset(8 as libc::c_int as isize);
    let mut name: *mut libc::c_char = (*pdents.offset(cur as isize)).name;
    let mut len: size_t = ((*pdents.offset(cur as isize)).c2rust_unnamed).nlen()
        as size_t;
    let mut mntpath: [libc::c_char; 4096] = [0; 4096];
    if !getutil(cmd) {
        printmsg(str);
        return 0 as libc::c_int != 0;
    }
    dir = xstrdup(name);
    if dir.is_null() {
        printmsg(messages[5 as libc::c_int as usize]);
        return 0 as libc::c_int != 0;
    }
    while len > 1 as libc::c_int as libc::c_ulong {
        len = len.wrapping_sub(1);
        if !(*dir.offset(len as isize) as libc::c_int == '.' as i32) {
            continue;
        }
        *dir.offset(len as isize) = '\0' as i32 as libc::c_char;
        break;
    }
    mkpath(cfgpath, toks[2 as libc::c_int as usize], mntpath.as_mut_ptr());
    mkpath(mntpath.as_mut_ptr(), dir, newpath);
    free(dir as *mut libc::c_void);
    if !xmktree(newpath, 1 as libc::c_int != 0) {
        printwait(strerror(*__errno_location()), 0 as *mut libc::c_int);
        return 0 as libc::c_int != 0;
    }
    if spawn(cmd, name, newpath, 0 as *mut libc::c_char, 0x8 as libc::c_int as ushort_t)
        != 0
    {
        printmsg(messages[5 as libc::c_int as usize]);
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn remote_mount(mut newpath: *mut libc::c_char) -> bool {
    let mut flag: uchar_t = (0x8 as libc::c_int | 0x1 as libc::c_int) as uchar_t;
    let mut opt: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut env: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: bool = getutil(utils[10 as libc::c_int as usize]);
    let mut s: bool = getutil(utils[9 as libc::c_int as usize]);
    let mut mntpath: [libc::c_char; 4096] = [0; 4096];
    if !(r as libc::c_int != 0 || s as libc::c_int != 0) {
        printmsg(b"install sshfs/rclone\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int != 0;
    }
    if r as libc::c_int != 0 && s as libc::c_int != 0 {
        opt = get_input(messages[30 as libc::c_int as usize]);
    } else {
        opt = if !s { 'r' as i32 } else { 's' as i32 };
    }
    if opt == 's' as i32 {
        env = xgetenv(
            b"NNN_SSHFS\0" as *const u8 as *const libc::c_char,
            utils[9 as libc::c_int as usize],
        );
    } else if opt == 'r' as i32 {
        flag = (flag as libc::c_int | (0x2 as libc::c_int | 0x4 as libc::c_int))
            as uchar_t;
        env = xgetenv(
            b"NNN_RCLONE\0" as *const u8 as *const libc::c_char,
            b"rclone mount\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        printmsg(messages[40 as libc::c_int as usize]);
        return 0 as libc::c_int != 0;
    }
    tmp = xreadline(
        0 as *const libc::c_char,
        b"host[:dir] > \0" as *const u8 as *const libc::c_char,
    );
    if *tmp.offset(0 as libc::c_int as isize) == 0 {
        printmsg(messages[4 as libc::c_int as usize]);
        return 0 as libc::c_int != 0;
    }
    let mut div: *mut libc::c_char = strchr(tmp, ':' as i32);
    if !div.is_null() {
        *div = '\0' as i32 as libc::c_char;
    }
    mkpath(cfgpath, toks[2 as libc::c_int as usize], mntpath.as_mut_ptr());
    mkpath(mntpath.as_mut_ptr(), tmp, newpath);
    if !xmktree(newpath, 1 as libc::c_int != 0) {
        printwait(strerror(*__errno_location()), 0 as *mut libc::c_int);
        return 0 as libc::c_int != 0;
    }
    if div.is_null() {
        let mut len: size_t = xstrlen(tmp);
        *tmp.offset(len as isize) = ':' as i32 as libc::c_char;
        *tmp
            .offset(
                len.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
    } else {
        *div = ':' as i32 as libc::c_char;
    }
    if opt == 's' as i32 {
        if spawn(env, tmp, newpath, 0 as *mut libc::c_char, flag as ushort_t) != 0 {
            printmsg(messages[5 as libc::c_int as usize]);
            return 0 as libc::c_int != 0;
        }
    } else {
        spawn(env, tmp, newpath, 0 as *mut libc::c_char, flag as ushort_t);
        printmsg(messages[31 as libc::c_int as usize]);
        xdelay(((350000 as libc::c_int) << 2 as libc::c_int) as useconds_t);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn unmount(
    mut name: *mut libc::c_char,
    mut newpath: *mut libc::c_char,
    mut presel: *mut libc::c_int,
    mut currentpath: *mut libc::c_char,
) -> bool {
    static mut cmd: [libc::c_char; 12] = unsafe {
        *::std::mem::transmute::<&[u8; 12], &mut [libc::c_char; 12]>(b"fusermount3\0")
    };
    static mut found: bool = 0 as libc::c_int != 0;
    let mut tmp: *mut libc::c_char = name;
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
    let mut psb: stat = stat {
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
    let mut child: bool = 0 as libc::c_int != 0;
    let mut parent: bool = 0 as libc::c_int != 0;
    let mut hovered: bool = 0 as libc::c_int != 0;
    let mut mntpath: [libc::c_char; 4096] = [0; 4096];
    if !found && !getutil(cmd.as_mut_ptr()) {
        cmd[10 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        found = 1 as libc::c_int != 0;
    }
    mkpath(cfgpath, toks[2 as libc::c_int as usize], mntpath.as_mut_ptr());
    if !tmp.is_null() && strcmp(mntpath.as_mut_ptr(), currentpath) == 0 as libc::c_int {
        mkpath(mntpath.as_mut_ptr(), tmp, newpath);
        child = lstat(newpath, &mut sb) != -(1 as libc::c_int);
        parent = lstat(xdirname(newpath), &mut psb) != -(1 as libc::c_int);
        if !child && !parent {
            *presel = '$' as i32;
            return 0 as libc::c_int != 0;
        }
    }
    if tmp.is_null() || !child
        || !(sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint)
        || child as libc::c_int != 0 && parent as libc::c_int != 0
            && sb.st_dev == psb.st_dev
    {
        tmp = xreadline(0 as *const libc::c_char, messages[16 as libc::c_int as usize]);
        if *tmp.offset(0 as libc::c_int as isize) == 0 {
            return 0 as libc::c_int != 0;
        }
        if !name.is_null()
            && *tmp.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            && *tmp.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            mkpath(currentpath, name, newpath);
            hovered = 1 as libc::c_int != 0;
        }
    }
    if !hovered {
        mkpath(mntpath.as_mut_ptr(), tmp, newpath);
    }
    if !xdiraccess(newpath) {
        *presel = '$' as i32;
        return 0 as libc::c_int != 0;
    }
    if spawn(
        cmd.as_mut_ptr(),
        b"-qu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        newpath,
        0 as *mut libc::c_char,
        0x8 as libc::c_int as ushort_t,
    ) != 0
    {
        if !xconfirm(get_input(messages[37 as libc::c_int as usize])) {
            return 0 as libc::c_int != 0;
        }
        if spawn(
            cmd.as_mut_ptr(),
            b"-quz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            newpath,
            0 as *mut libc::c_char,
            0x8 as libc::c_int as ushort_t,
        ) != 0
        {
            printwait(messages[5 as libc::c_int as usize], presel);
            return 0 as libc::c_int != 0;
        }
    }
    if rmdir(newpath) == -(1 as libc::c_int) {
        printwait(strerror(*__errno_location()), presel);
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn lock_terminal() {
    spawn(
        xgetenv(
            b"NNN_LOCKER\0" as *const u8 as *const libc::c_char,
            utils[5 as libc::c_int as usize],
        ),
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        (0x8 as libc::c_int | 0x1 as libc::c_int) as ushort_t,
    );
}
unsafe extern "C" fn printkv(
    mut kvarr: *mut kv,
    mut fd: libc::c_int,
    mut max: uchar_t,
    mut id: uchar_t,
) {
    let mut val: *mut libc::c_char = if id as libc::c_int == 1 as libc::c_int {
        bmstr
    } else {
        pluginstr
    };
    let mut i: uchar_t = 0 as libc::c_int as uchar_t;
    while (i as libc::c_int) < max as libc::c_int && (*kvarr.offset(i as isize)).key != 0
    {
        dprintf(
            fd,
            b" %c: %s\n\0" as *const u8 as *const libc::c_char,
            (*kvarr.offset(i as isize)).key as libc::c_char as libc::c_int,
            val.offset((*kvarr.offset(i as isize)).off as isize),
        );
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn printkeys(
    mut kvarr: *mut kv,
    mut buf: *mut libc::c_char,
    mut max: uchar_t,
) {
    let mut i: uchar_t = 0 as libc::c_int as uchar_t;
    while (i as libc::c_int) < max as libc::c_int && (*kvarr.offset(i as isize)).key != 0
    {
        *buf
            .offset(
                ((i as libc::c_int) << 1 as libc::c_int) as isize,
            ) = ' ' as i32 as libc::c_char;
        *buf
            .offset(
                (((i as libc::c_int) << 1 as libc::c_int) + 1 as libc::c_int) as isize,
            ) = (*kvarr.offset(i as isize)).key as libc::c_char;
        i = i.wrapping_add(1);
        i;
    }
    *buf
        .offset(
            ((i as libc::c_int) << 1 as libc::c_int) as isize,
        ) = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn handle_bookmark(
    mut bmark: *const libc::c_char,
    mut newpath: *mut libc::c_char,
) -> size_t {
    let mut fd: libc::c_int = '\r' as i32;
    let mut r: size_t = 0;
    if maxbm as libc::c_int != 0 || !bmark.is_null() {
        r = xstrsncpy(
            g_buf.as_mut_ptr(),
            messages[34 as libc::c_int as usize],
            (4096 as libc::c_int
                + ((255 as libc::c_int + 1 as libc::c_int) << 1 as libc::c_int))
                as size_t,
        );
        if !bmark.is_null() {
            r = r.wrapping_sub(1);
            g_buf[r as usize] = ' ' as i32 as libc::c_char;
            r = r.wrapping_add(1);
            g_buf[r as usize] = ',' as i32 as libc::c_char;
            r = r.wrapping_add(1);
            g_buf[r as usize] = '\0' as i32 as libc::c_char;
            r = r.wrapping_add(1);
            r;
        }
        printkeys(
            bookmark,
            g_buf.as_mut_ptr().offset(r as isize).offset(-(1 as libc::c_int as isize)),
            maxbm,
        );
        printmsg(g_buf.as_mut_ptr());
        fd = get_input(0 as *const libc::c_char);
    }
    r = 0 as libc::c_int as size_t;
    if fd == ',' as i32 {
        if !bmark.is_null() {
            xstrsncpy(newpath, bmark, 4096 as libc::c_int as size_t);
        } else {
            r = 27 as libc::c_int as size_t;
        };
    } else if fd == '\r' as i32 {
        mkpath(cfgpath, toks[0 as libc::c_int as usize], newpath);
        g_state.set_selbm(1 as libc::c_int as uint_t);
    } else if (get_kv_val(bookmark, newpath, fd, maxbm, 1 as libc::c_int as uchar_t))
        .is_null()
    {
        r = 40 as libc::c_int as size_t;
    }
    if r == 0 && chdir(newpath) == -(1 as libc::c_int) {
        r = 24 as libc::c_int as size_t;
        if g_state.selbm() != 0 {
            g_state.set_selbm(0 as libc::c_int as uint_t);
        }
    }
    return r;
}
unsafe extern "C" fn add_bookmark(
    mut path: *mut libc::c_char,
    mut newpath: *mut libc::c_char,
    mut presel: *mut libc::c_int,
) {
    let mut dir: *mut libc::c_char = xbasename(path);
    dir = xreadline(
        if *dir.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            dir
        } else {
            0 as *mut libc::c_char
        },
        messages[43 as libc::c_int as usize],
    );
    if !dir.is_null() && *dir as libc::c_int != 0 {
        let mut r: size_t = mkpath(cfgpath, toks[0 as libc::c_int as usize], newpath);
        *newpath
            .offset(
                r.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '/' as i32 as libc::c_char;
        xstrsncpy(
            newpath.offset(r as isize),
            dir,
            (4096 as libc::c_int as libc::c_ulong).wrapping_sub(r),
        );
        printwait(
            if symlink(path, newpath) == -(1 as libc::c_int) {
                strerror(*__errno_location())
            } else {
                newpath
            },
            presel,
        );
    } else {
        printwait(messages[4 as libc::c_int as usize], presel);
    };
}
unsafe extern "C" fn show_help(mut path: *const libc::c_char) {
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let helpstr: [libc::c_char; 1184] = *::std::mem::transmute::<
        &[u8; 1184],
        &[libc::c_char; 1184],
    >(
        b"0\n1NAVIGATION\n9Up k  Up%-16cPgUp ^U  Page up\n9Dn j  Down%-14cPgDn ^D  Page down\n9Lt h  Parent%-12c~ ` @ -  ~, /, start, prev\n5Ret Rt l  Open%-20c'  First file/match\n9g ^A  Top%-21cJ  Jump to entry/offset\n9G ^E  End%-20c^J  Toggle auto-advance on open\n8B (,)  Book(mark)%-11cb ^/  Select bookmark\na1-4  Context%-11c(Sh)Tab  Cycle/new context\n62Esc ^Q  Quit%-20cq  Quit context\nb^G  QuitCD%-18cQ  Pick/err, quit\n0\n1FILTER & PROMPT\nc/  Filter%-17c^N  Toggle type-to-nav\naEsc  Exit prompt%-12c^L  Toggle last filter\nc.  Toggle hidden%-5cAlt+Esc  Unfilter, quit context\n0\n1FILES\n9o ^O  Open with%-15cn  Create new/link\n9f ^F  File stats%-14cd  Detail mode toggle\nb^R  Rename/dup%-14cr  Batch rename\ncz  Archive%-17ce  Edit file\nc*  Toggle exe%-14c>  Export list\n6Space +  (Un)select%-12cm-m  Select range/clear\nca  Select all%-14cA  Invert sel\n9p ^P  Copy here%-12cw ^W  Cp/mv sel as\n9v ^V  Move here%-15cE  Edit sel list\n9x ^X  Delete%-16cEsc  Send to FIFO\n0\n1MISC\n8Alt ;  Select plugin%-11c=  Launch app\n9! ^]  Shell%-19c]  Cmd prompt\ncc  Connect remote%-10cu  Unmount remote/archive\n9t ^T  Sort toggles%-12cs  Manage session\ncT  Set time type%-11c0  Lock\nb^L  Redraw%-18c?  Help, conf\n\0",
    );
    let mut fd: libc::c_int = create_tmp_file();
    if fd == -(1 as libc::c_int) {
        return;
    }
    dprintf(
        fd,
        b"  |V\\_\n  /. \\\\\n (;^; ||\n   /___3\n  (___n))\n\0" as *const u8
            as *const libc::c_char,
    );
    let mut prog: *mut libc::c_char = xgetenv(
        env_cfg[12 as libc::c_int as usize],
        0 as *mut libc::c_char,
    );
    if !prog.is_null() {
        get_output(
            prog,
            0 as *mut libc::c_char,
            0 as *mut libc::c_char,
            fd,
            1 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
    }
    end = helpstr.as_ptr();
    start = end;
    while *end != 0 {
        if *end as libc::c_int == '\n' as i32 {
            snprintf(
                g_buf.as_mut_ptr(),
                (4096 as libc::c_int
                    + ((255 as libc::c_int + 1 as libc::c_int) << 1 as libc::c_int))
                    as libc::c_ulong,
                b"%*c%.*s\0" as *const u8 as *const libc::c_char,
                xchartohex(*start as uchar_t) as libc::c_int,
                ' ' as i32,
                end.offset_from(start) as libc::c_long as libc::c_int,
                start.offset(1 as libc::c_int as isize),
            );
            dprintf(fd, g_buf.as_mut_ptr(), ' ' as i32);
            start = end.offset(1 as libc::c_int as isize);
        }
        end = end.offset(1);
        end;
    }
    dprintf(fd, b"\nLOCATIONS\n\0" as *const u8 as *const libc::c_char);
    let mut i: uchar_t = 0 as libc::c_int as uchar_t;
    while (i as libc::c_int) < 4 as libc::c_int {
        if (g_ctx[i as usize].c_cfg).ctxactive() != 0 {
            dprintf(
                fd,
                b" %u: %s\n\0" as *const u8 as *const libc::c_char,
                i as libc::c_int + 1 as libc::c_int,
                (g_ctx[i as usize].c_path).as_mut_ptr(),
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    dprintf(
        fd,
        b"\nVOLUME: avail:%s \0" as *const u8 as *const libc::c_char,
        coolsize(get_fs_info(path, 0 as libc::c_int as uchar_t) as off_t),
    );
    dprintf(
        fd,
        b"used:%s \0" as *const u8 as *const libc::c_char,
        coolsize(get_fs_info(path, 1 as libc::c_int as uchar_t) as off_t),
    );
    dprintf(
        fd,
        b"size:%s\n\n\0" as *const u8 as *const libc::c_char,
        coolsize(get_fs_info(path, 2 as libc::c_int as uchar_t) as off_t),
    );
    if !bookmark.is_null() {
        dprintf(fd, b"BOOKMARKS\n\0" as *const u8 as *const libc::c_char);
        printkv(bookmark, fd, maxbm, 1 as libc::c_int as uchar_t);
        dprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    }
    if !plug.is_null() {
        dprintf(fd, b"PLUGIN KEYS\n\0" as *const u8 as *const libc::c_char);
        printkv(plug, fd, maxplug, 2 as libc::c_int as uchar_t);
        dprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    }
    let mut i_0: uchar_t = 3 as libc::c_int as uchar_t;
    while i_0 as libc::c_int <= 13 as libc::c_int {
        start = getenv(env_cfg[i_0 as usize]);
        if !start.is_null() {
            dprintf(
                fd,
                b"%s: %s\n\0" as *const u8 as *const libc::c_char,
                env_cfg[i_0 as usize],
                start,
            );
        }
        i_0 = i_0.wrapping_add(1);
        i_0;
    }
    if !selpath.is_null() {
        dprintf(
            fd,
            b"SELECTION FILE: %s\n\0" as *const u8 as *const libc::c_char,
            selpath,
        );
    }
    dprintf(
        fd,
        b"\nv%s\n%s\n\0" as *const u8 as *const libc::c_char,
        b"4.6\0" as *const u8 as *const libc::c_char,
        b"BSD 2-Clause\nhttps://github.com/jarun/nnn\0" as *const u8
            as *const libc::c_char,
    );
    close(fd);
    spawn(
        pager,
        g_tmpfpath.as_mut_ptr(),
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        (0x8 as libc::c_int | 0x1 as libc::c_int | 0x100 as libc::c_int) as ushort_t,
    );
    unlink(g_tmpfpath.as_mut_ptr());
}
unsafe extern "C" fn setexports() {
    let mut dvar: [libc::c_char; 3] = *::std::mem::transmute::<
        &[u8; 3],
        &mut [libc::c_char; 3],
    >(b"d0\0");
    let mut fvar: [libc::c_char; 3] = *::std::mem::transmute::<
        &[u8; 3],
        &mut [libc::c_char; 3],
    >(b"f0\0");
    if ndents != 0 {
        setenv(
            envs[4 as libc::c_int as usize],
            (*pdents.offset(cur as isize)).name,
            1 as libc::c_int,
        );
        xstrsncpy(
            (g_ctx[cfg.curctx() as usize].c_name).as_mut_ptr(),
            (*pdents.offset(cur as isize)).name,
            (255 as libc::c_int + 1 as libc::c_int) as size_t,
        );
    } else if g_ctx[cfg.curctx() as usize].c_name[0 as libc::c_int as usize] != 0 {
        g_ctx[cfg.curctx() as usize]
            .c_name[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }
    let mut i: uchar_t = 0 as libc::c_int as uchar_t;
    while (i as libc::c_int) < 4 as libc::c_int {
        if (g_ctx[i as usize].c_cfg).ctxactive() != 0 {
            fvar[1 as libc::c_int
                as usize] = ('1' as i32 + i as libc::c_int) as libc::c_char;
            dvar[1 as libc::c_int as usize] = fvar[1 as libc::c_int as usize];
            setenv(
                dvar.as_mut_ptr(),
                (g_ctx[i as usize].c_path).as_mut_ptr(),
                1 as libc::c_int,
            );
            if g_ctx[i as usize].c_name[0 as libc::c_int as usize] != 0 {
                mkpath(
                    (g_ctx[i as usize].c_path).as_mut_ptr(),
                    (g_ctx[i as usize].c_name).as_mut_ptr(),
                    g_buf.as_mut_ptr(),
                );
                setenv(fvar.as_mut_ptr(), g_buf.as_mut_ptr(), 1 as libc::c_int);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    setenv(
        b"NNN_INCLUDE_HIDDEN\0" as *const u8 as *const libc::c_char,
        xitoa(cfg.showhidden()),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn run_cmd_as_plugin(
    mut file: *const libc::c_char,
    mut runfile: *mut libc::c_char,
    mut flags: uchar_t,
) {
    let mut len: size_t = 0;
    xstrsncpy(g_buf.as_mut_ptr(), file, 4096 as libc::c_int as size_t);
    len = xstrlen(g_buf.as_mut_ptr());
    if len > 1 as libc::c_int as libc::c_ulong
        && g_buf[len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
            as libc::c_int == '*' as i32
    {
        flags = (flags as libc::c_int & !(0x10 as libc::c_int)) as uchar_t;
        g_buf[len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = '\0' as i32 as libc::c_char;
        len = len.wrapping_sub(1);
        len;
    }
    if flags as libc::c_int & 0x80 as libc::c_int != 0
        || flags as libc::c_int & 0x4 as libc::c_int != 0
    {
        if is_suffix(
            g_buf.as_mut_ptr(),
            b" $nnn\0" as *const u8 as *const libc::c_char,
        ) {
            g_buf[len.wrapping_sub(5 as libc::c_int as libc::c_ulong)
                as usize] = '\0' as i32 as libc::c_char;
        } else {
            runfile = 0 as *mut libc::c_char;
        }
        if flags as libc::c_int & 0x80 as libc::c_int != 0 {
            get_output(
                g_buf.as_mut_ptr(),
                runfile,
                0 as *mut libc::c_char,
                -(1 as libc::c_int),
                1 as libc::c_int != 0,
                1 as libc::c_int != 0,
            );
        } else {
            spawn(
                g_buf.as_mut_ptr(),
                runfile,
                0 as *mut libc::c_char,
                0 as *mut libc::c_char,
                flags as ushort_t,
            );
        }
    } else {
        spawn(
            utils[7 as libc::c_int as usize],
            g_buf.as_mut_ptr(),
            0 as *mut libc::c_char,
            0 as *mut libc::c_char,
            flags as ushort_t,
        );
    };
}
unsafe extern "C" fn plctrl_init() -> bool {
    let mut len: size_t = 0;
    g_tmpfpath[(tmpfplen as libc::c_int - 1 as libc::c_int)
        as usize] = '\0' as i32 as libc::c_char;
    len = xstrsncpy(
        g_pipepath.as_mut_ptr(),
        g_tmpfpath.as_mut_ptr(),
        64 as libc::c_int as size_t,
    );
    g_pipepath[len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = '/' as i32 as libc::c_char;
    len = (xstrsncpy(
        g_pipepath.as_mut_ptr().offset(len as isize),
        b"nnn-pipe.\0" as *const u8 as *const libc::c_char,
        (64 as libc::c_int as libc::c_ulong).wrapping_sub(len),
    ))
        .wrapping_add(len);
    xstrsncpy(
        g_pipepath
            .as_mut_ptr()
            .offset(len as isize)
            .offset(-(1 as libc::c_int as isize)),
        xitoa(getpid() as uint_t),
        (64 as libc::c_int as libc::c_ulong).wrapping_sub(len),
    );
    setenv(
        env_cfg[7 as libc::c_int as usize],
        g_pipepath.as_mut_ptr(),
        1 as libc::c_int,
    );
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn rmlistpath() {
    if !listpath.is_null() {
        spawn(
            utils[20 as libc::c_int as usize],
            listpath,
            0 as *mut libc::c_char,
            0 as *mut libc::c_char,
            (0x4 as libc::c_int | 0x1 as libc::c_int) as ushort_t,
        );
        if listpath != initpath {
            free(listpath as *mut libc::c_void);
        }
        listpath = 0 as *mut libc::c_char;
    }
}
unsafe extern "C" fn read_nointr(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_void,
    mut count: size_t,
) -> ssize_t {
    let mut len: ssize_t = 0;
    loop {
        len = read(fd, buf, count);
        if !(len == -(1 as libc::c_int) as libc::c_long
            && *__errno_location() == 4 as libc::c_int)
        {
            break;
        }
    }
    return len;
}
unsafe extern "C" fn readpipe(
    mut fd: libc::c_int,
    mut ctxnum: *mut libc::c_char,
    mut path: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut ctx: libc::c_char = 0;
    let mut nextpath: *mut libc::c_char = 0 as *mut libc::c_char;
    if read_nointr(
        fd,
        g_buf.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int as size_t,
    ) != 1 as libc::c_int as libc::c_long
    {
        return 0 as *mut libc::c_char;
    }
    if g_buf[0 as libc::c_int as usize] as libc::c_int == '-' as i32 {
        clearselection();
        if read_nointr(
            fd,
            g_buf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) != 1 as libc::c_int as libc::c_long
        {
            return 0 as *mut libc::c_char;
        }
    }
    if g_buf[0 as libc::c_int as usize] as libc::c_int == '+' as i32 {
        ctx = (get_free_ctx() as libc::c_int + 1 as libc::c_int) as libc::c_char;
    } else if (g_buf[0 as libc::c_int as usize] as libc::c_int) < '0' as i32 {
        return 0 as *mut libc::c_char
    } else {
        ctx = (g_buf[0 as libc::c_int as usize] as libc::c_int - '0' as i32)
            as libc::c_char;
        if ctx as libc::c_int > 4 as libc::c_int {
            return 0 as *mut libc::c_char;
        }
    }
    if read_nointr(
        fd,
        g_buf.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int as size_t,
    ) != 1 as libc::c_int as libc::c_long
    {
        return 0 as *mut libc::c_char;
    }
    let mut op: libc::c_char = g_buf[0 as libc::c_int as usize];
    if op as libc::c_int == 'c' as i32 {
        let mut len: ssize_t = read_nointr(
            fd,
            g_buf.as_mut_ptr() as *mut libc::c_void,
            4096 as libc::c_int as size_t,
        );
        if len <= 0 as libc::c_int as libc::c_long {
            return 0 as *mut libc::c_char;
        }
        g_buf[len as usize] = '\0' as i32 as libc::c_char;
        if g_buf[0 as libc::c_int as usize] as libc::c_int == '/' as i32 {
            nextpath = g_buf.as_mut_ptr();
            len = xstrlen(g_buf.as_mut_ptr()) as ssize_t;
            loop {
                len -= 1;
                if !(len != 0 && g_buf[len as usize] as libc::c_int == '/' as i32) {
                    break;
                }
                g_buf[len as usize] = '\0' as i32 as libc::c_char;
            }
        }
    } else if op as libc::c_int == 'l' as i32 {
        rmlistpath();
        nextpath = load_input(fd, *path);
    } else if op as libc::c_int == 'p' as i32 {
        free(selpath as *mut libc::c_void);
        selpath = 0 as *mut libc::c_char;
        clearselection();
        g_state.set_picker(0 as libc::c_int as uint_t);
        g_state.set_picked(1 as libc::c_int as uint_t);
    }
    *ctxnum = ctx;
    return nextpath;
}
unsafe extern "C" fn run_plugin(
    mut path: *mut *mut libc::c_char,
    mut file: *const libc::c_char,
    mut runfile: *mut libc::c_char,
    mut lastname: *mut *mut libc::c_char,
    mut lastdir: *mut *mut libc::c_char,
) -> bool {
    let mut p: pid_t = 0;
    let mut ctx: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut flags: uchar_t = 0 as libc::c_int as uchar_t;
    let mut cmd_as_plugin: bool = 0 as libc::c_int != 0;
    let mut nextpath: *mut libc::c_char = 0 as *mut libc::c_char;
    if g_state.pluginit() == 0 {
        plctrl_init();
        g_state.set_pluginit(1 as libc::c_int as uint_t);
    }
    setexports();
    if *file as libc::c_int == '!' as i32 {
        flags = (0x1 as libc::c_int | 0x10 as libc::c_int) as uchar_t;
        file = file.offset(1);
        file;
        if *file as libc::c_int == '|' as i32 {
            flags = (flags as libc::c_int | 0x80 as libc::c_int) as uchar_t;
            file = file.offset(1);
            file;
        } else if *file as libc::c_int == '&' as i32 {
            flags = (0x4 as libc::c_int | 0x2 as libc::c_int) as uchar_t;
            file = file.offset(1);
            file;
        }
        if *file == 0 {
            return 0 as libc::c_int != 0;
        }
        if flags as libc::c_int & 0x4 as libc::c_int != 0
            || flags as libc::c_int & 0x80 as libc::c_int != 0
        {
            run_cmd_as_plugin(file, runfile, flags);
            return 1 as libc::c_int != 0;
        }
        cmd_as_plugin = 1 as libc::c_int != 0;
    }
    if mkfifo(g_pipepath.as_mut_ptr(), 0o600 as libc::c_int as __mode_t)
        != 0 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    endwin();
    p = fork();
    if p == 0 {
        let mut wfd: libc::c_int = open(
            g_pipepath.as_mut_ptr(),
            0o1 as libc::c_int | 0o2000000 as libc::c_int,
        );
        if wfd == -(1 as libc::c_int) {
            _exit(1 as libc::c_int);
        }
        if !cmd_as_plugin {
            let mut sel: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut std: [libc::c_char; 2] = *::std::mem::transmute::<
                &[u8; 2],
                &mut [libc::c_char; 2],
            >(b"-\0");
            mkpath(plgpath, file, g_buf.as_mut_ptr());
            if g_state.picker() != 0 {
                sel = if !selpath.is_null() { selpath } else { std.as_mut_ptr() };
            }
            if !runfile.is_null()
                && *runfile.offset(0 as libc::c_int as isize) as libc::c_int != 0
            {
                xstrsncpy(*lastname, runfile, 255 as libc::c_int as size_t);
                spawn(
                    g_buf.as_mut_ptr(),
                    *lastname,
                    *path,
                    sel,
                    0 as libc::c_int as ushort_t,
                );
            } else {
                spawn(
                    g_buf.as_mut_ptr(),
                    0 as *mut libc::c_char,
                    *path,
                    sel,
                    0 as libc::c_int as ushort_t,
                );
            }
        } else {
            run_cmd_as_plugin(file, runfile, flags);
        }
        close(wfd);
        _exit(0 as libc::c_int);
    }
    let mut rfd: libc::c_int = 0;
    loop {
        rfd = open(g_pipepath.as_mut_ptr(), 0 as libc::c_int);
        if !(rfd == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    nextpath = readpipe(rfd, &mut ctx, path);
    if !nextpath.is_null() {
        set_smart_ctx(ctx as libc::c_int, nextpath, path, runfile, lastname, lastdir);
    }
    close(rfd);
    waitpid(p, 0 as *mut libc::c_int, 0 as libc::c_int);
    wrefresh(stdscr);
    unlink(g_pipepath.as_mut_ptr());
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn launch_app(mut newpath: *mut libc::c_char) -> bool {
    let mut r: libc::c_int = 0x8 as libc::c_int;
    let mut tmp: *mut libc::c_char = newpath;
    mkpath(plgpath, utils[6 as libc::c_int as usize], newpath);
    if !getutil(utils[14 as libc::c_int as usize])
        || access(newpath, 1 as libc::c_int) < 0 as libc::c_int
    {
        tmp = xreadline(0 as *const libc::c_char, messages[32 as libc::c_int as usize]);
        r = 0x2 as libc::c_int | 0x4 as libc::c_int | 0x1 as libc::c_int;
    }
    if !tmp.is_null() && *tmp as libc::c_int != 0 {
        spawn(
            tmp,
            (if r == 0x8 as libc::c_int {
                b"0\0" as *const u8 as *const libc::c_char
            } else {
                0 as *const libc::c_char
            }) as *mut libc::c_char,
            0 as *mut libc::c_char,
            0 as *mut libc::c_char,
            r as ushort_t,
        );
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn prompt_run() -> bool {
    let mut ret: bool = 0 as libc::c_int != 0;
    let mut cmdline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cnt_j: libc::c_int = 0;
    let mut cnt_J: libc::c_int = 0;
    let mut cmd_ret: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut xargs_j: *const libc::c_char = b"xargs -0 -I{} %s < %s\0" as *const u8
        as *const libc::c_char;
    let mut xargs_J: *const libc::c_char = b"xargs -0 %s < %s\0" as *const u8
        as *const libc::c_char;
    let mut cmd: [libc::c_char; 4640] = [0; 4640];
    loop {
        if g_state.picker() != 0 {
            cmdline = xreadline(
                0 as *const libc::c_char,
                b">>> \0" as *const u8 as *const libc::c_char,
            );
        } else {
            cmdline = getreadline(b"\n>>> \0" as *const u8 as *const libc::c_char);
        }
        if cmdline.is_null() || *cmdline.offset(0 as libc::c_int as isize) == 0 {
            break;
        }
        free(lastcmd as *mut libc::c_void);
        lastcmd = xstrdup(cmdline);
        ret = 1 as libc::c_int != 0;
        len = xstrlen(cmdline);
        cnt_j = 0 as libc::c_int;
        next = cmdline;
        loop {
            next = strstr(next, b"%j\0" as *const u8 as *const libc::c_char);
            if next.is_null() {
                break;
            }
            cnt_j += 1;
            cnt_j;
            *next.offset(0 as libc::c_int as isize) = '{' as i32 as libc::c_char;
            *next.offset(1 as libc::c_int as isize) = '}' as i32 as libc::c_char;
            next = next.offset(1);
            next;
        }
        cnt_J = 0 as libc::c_int;
        next = cmdline;
        loop {
            next = strstr(next, b"%J\0" as *const u8 as *const libc::c_char);
            if next.is_null() {
                break;
            }
            cnt_J += 1;
            cnt_J;
            if next == cmdline.offset(len as isize).offset(-(2 as libc::c_int as isize))
            {
                *cmdline
                    .offset(
                        len.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
                    ) = '\0' as i32 as libc::c_char;
            }
            next = next.offset(1);
            next;
        }
        if cnt_j != 0 && cnt_J != 0 {
            break;
        }
        if cnt_j != 0 {
            snprintf(
                cmd.as_mut_ptr(),
                (4096 as libc::c_int
                    + ((255 as libc::c_int + 1 as libc::c_int) << 1 as libc::c_int)
                    + 32 as libc::c_int) as libc::c_ulong,
                xargs_j,
                cmdline,
                selpath,
            );
        } else if cnt_J != 0 {
            snprintf(
                cmd.as_mut_ptr(),
                (4096 as libc::c_int
                    + ((255 as libc::c_int + 1 as libc::c_int) << 1 as libc::c_int)
                    + 32 as libc::c_int) as libc::c_ulong,
                xargs_J,
                cmdline,
                selpath,
            );
        }
        cmd_ret = spawn(
            shell,
            b"-c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            if cnt_j != 0 || cnt_J != 0 { cmd.as_mut_ptr() } else { cmdline },
            0 as *mut libc::c_char,
            (0x8 as libc::c_int | 0x1 as libc::c_int | 0x10 as libc::c_int) as ushort_t,
        );
        if (cnt_j != 0 || cnt_J != 0) && cmd_ret == 0 as libc::c_int {
            clearselection();
        }
    }
    return ret;
}
unsafe extern "C" fn handle_cmd(
    mut sel: action,
    mut newpath: *mut libc::c_char,
) -> bool {
    endselection(0 as libc::c_int != 0);
    if sel as libc::c_uint == SEL_LAUNCH as libc::c_int as libc::c_uint {
        return launch_app(newpath);
    }
    setexports();
    if sel as libc::c_uint == SEL_PROMPT as libc::c_int as libc::c_uint {
        return prompt_run();
    }
    let mut tmp: *mut libc::c_char = getenv(env_cfg[6 as libc::c_int as usize]);
    let mut r: libc::c_int = if !tmp.is_null() { atoi(tmp) } else { 0 as libc::c_int };
    setenv(
        env_cfg[6 as libc::c_int as usize],
        xitoa((r + 1 as libc::c_int) as uint_t),
        1 as libc::c_int,
    );
    spawn(
        shell,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        (0x8 as libc::c_int | 0x1 as libc::c_int) as ushort_t,
    );
    setenv(env_cfg[6 as libc::c_int as usize], xitoa(r as uint_t), 1 as libc::c_int);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn dentfree() {
    free(pnamebuf as *mut libc::c_void);
    free(pdents as *mut libc::c_void);
    free(mark as *mut libc::c_void);
    free(core_blocks as *mut libc::c_void);
    free(core_data as *mut libc::c_void);
    free(core_files as *mut libc::c_void);
}
unsafe extern "C" fn du_thread(mut p_data: *mut libc::c_void) -> *mut libc::c_void {
    let mut pdata: *mut thread_data = p_data as *mut thread_data;
    let mut path: [*mut libc::c_char; 2] = [
        ((*pdata).path).as_mut_ptr(),
        0 as *mut libc::c_char,
    ];
    let mut tfiles: ullong_t = 0 as libc::c_int as ullong_t;
    let mut tblocks: blkcnt_t = 0 as libc::c_int as blkcnt_t;
    let mut sb: *mut stat = 0 as *mut stat;
    let mut tree: *mut FTS = fts_open(
        path.as_mut_ptr(),
        0x10 as libc::c_int | 0x40 as libc::c_int | 0x4 as libc::c_int,
        None,
    );
    let mut node: *mut FTSENT = 0 as *mut FTSENT;
    loop {
        node = fts_read(tree);
        if node.is_null() {
            break;
        }
        if (*node).fts_info as libc::c_int & 1 as libc::c_int != 0 {
            if g_state.interrupt() != 0 {
                break;
            }
        } else {
            sb = (*node).fts_statp;
            if cfg.apparentsz() != 0 {
                if (*sb).st_size != 0
                    && ((*node).fts_info as libc::c_int & 8 as libc::c_int != 0
                        && ((*sb).st_nlink <= 1 as libc::c_int as libc::c_ulong
                            || test_set_bit((*sb).st_ino as uint_t) as libc::c_int != 0)
                        || (*node).fts_info as libc::c_int & 6 as libc::c_int != 0)
                {
                    tblocks += (*sb).st_size;
                }
            } else if (*sb).st_blocks != 0
                && ((*node).fts_info as libc::c_int & 8 as libc::c_int != 0
                    && ((*sb).st_nlink <= 1 as libc::c_int as libc::c_ulong
                        || test_set_bit((*sb).st_ino as uint_t) as libc::c_int != 0)
                    || (*node).fts_info as libc::c_int & 6 as libc::c_int != 0)
            {
                tblocks += (*sb).st_blocks;
            }
            tfiles = tfiles.wrapping_add(1);
            tfiles;
        }
    }
    fts_close(tree);
    if (*pdata).entnum >= 0 as libc::c_int {
        let ref mut fresh26 = (*pdents.offset((*pdata).entnum as isize)).c2rust_unnamed;
        (*fresh26).set_blocks(tblocks as ullong_t);
    }
    if !(*pdata).mntpoint {
        let ref mut fresh27 = *core_blocks.offset((*pdata).core as isize);
        *fresh27 += tblocks;
        let ref mut fresh28 = *core_files.offset((*pdata).core as isize);
        *fresh28 = (*fresh28 as libc::c_ulonglong).wrapping_add(tfiles) as ullong_t
            as ullong_t;
    } else {
        let ref mut fresh29 = *core_files.offset((*pdata).core as isize);
        *fresh29 = (*fresh29 as libc::c_ulonglong)
            .wrapping_add(1 as libc::c_int as libc::c_ulonglong) as ullong_t as ullong_t;
    }
    pthread_mutex_lock(&mut running_mutex);
    threadbmp |= (1 as libc::c_int) << (*pdata).core as libc::c_int;
    ::std::ptr::write_volatile(
        &mut active_threads as *mut libc::c_int,
        ::std::ptr::read_volatile::<libc::c_int>(&active_threads as *const libc::c_int)
            - 1,
    );
    ::std::ptr::read_volatile::<libc::c_int>(&active_threads as *const libc::c_int);
    pthread_mutex_unlock(&mut running_mutex);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn dirwalk(
    mut path: *mut libc::c_char,
    mut entnum: libc::c_int,
    mut mountpoint: bool,
) {
    while active_threads == 4 as libc::c_int {}
    if g_state.interrupt() != 0 {
        return;
    }
    pthread_mutex_lock(&mut running_mutex);
    let mut core: libc::c_int = ffs(threadbmp) - 1 as libc::c_int;
    threadbmp &= !((1 as libc::c_int) << core);
    ::std::ptr::write_volatile(
        &mut active_threads as *mut libc::c_int,
        ::std::ptr::read_volatile::<libc::c_int>(&active_threads as *const libc::c_int)
            + 1,
    );
    ::std::ptr::read_volatile::<libc::c_int>(&active_threads as *const libc::c_int);
    pthread_mutex_unlock(&mut running_mutex);
    xstrsncpy(
        ((*core_data.offset(core as isize)).path).as_mut_ptr(),
        path,
        4096 as libc::c_int as size_t,
    );
    (*core_data.offset(core as isize)).entnum = entnum;
    (*core_data.offset(core as isize)).core = core as ushort_t;
    (*core_data.offset(core as isize)).mntpoint = mountpoint;
    let mut tid: pthread_t = 0 as libc::c_int as pthread_t;
    pthread_create(
        &mut tid,
        0 as *const pthread_attr_t,
        Some(du_thread as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        &mut *core_data.offset(core as isize) as *mut thread_data as *mut libc::c_void,
    );
    wmove(stdscr, xlines as libc::c_int - 1 as libc::c_int, 0 as libc::c_int);
    waddnstr(stdscr, xbasename(path), -(1 as libc::c_int));
    waddnstr(
        stdscr,
        b" [^C aborts]\n\0" as *const u8 as *const libc::c_char,
        -(1 as libc::c_int),
    );
    wrefresh(stdscr);
}
unsafe extern "C" fn prep_threads() -> bool {
    if g_state.duinit() == 0 {
        threadbmp >>= 32 as libc::c_int - 4 as libc::c_int;
        if core_blocks.is_null() {
            core_blocks = calloc(
                4 as libc::c_int as libc::c_ulong,
                ::std::mem::size_of::<blkcnt_t>() as libc::c_ulong,
            ) as *mut blkcnt_t;
        }
        if core_data.is_null() {
            core_data = calloc(
                4 as libc::c_int as libc::c_ulong,
                ::std::mem::size_of::<thread_data>() as libc::c_ulong,
            ) as *mut thread_data;
        }
        if core_files.is_null() {
            core_files = calloc(
                4 as libc::c_int as libc::c_ulong,
                ::std::mem::size_of::<ullong_t>() as libc::c_ulong,
            ) as *mut ullong_t;
        }
        if core_blocks.is_null() || core_data.is_null() || core_files.is_null() {
            printwait(strerror(*__errno_location()), 0 as *mut libc::c_int);
            return 0 as libc::c_int != 0;
        }
        max_openfds();
        g_state.set_duinit(1 as libc::c_int as uint_t);
    } else {
        memset(
            core_blocks as *mut libc::c_void,
            0 as libc::c_int,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<blkcnt_t>() as libc::c_ulong),
        );
        memset(
            core_data as *mut libc::c_void,
            0 as libc::c_int,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<thread_data>() as libc::c_ulong),
        );
        memset(
            core_files as *mut libc::c_void,
            0 as libc::c_int,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<ullong_t>() as libc::c_ulong),
        );
    }
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn selforparent(mut path: *const libc::c_char) -> bool {
    return *path.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
        && (*path.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
            || *path.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *path.offset(2 as libc::c_int as isize) as libc::c_int
                    == '\0' as i32);
}
unsafe extern "C" fn dentfill(
    mut path: *mut libc::c_char,
    mut ppdents: *mut *mut entry,
) -> libc::c_int {
    let mut current_block: u64;
    let mut entflags: uchar_t = 0 as libc::c_int as uchar_t;
    let mut flags: libc::c_int = 0 as libc::c_int;
    let mut dp: *mut dirent = 0 as *mut dirent;
    let mut namep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pnb: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dentp: *mut entry = 0 as *mut entry;
    let mut off: size_t = 0 as libc::c_int as size_t;
    let mut namebuflen: size_t = 0x800 as libc::c_int as size_t;
    let mut sb_path: stat = stat {
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
    let mut dirp: *mut DIR = opendir(path);
    ndents = 0 as libc::c_int;
    gtimesecs = time(0 as *mut time_t);
    if dirp.is_null() {
        return 0 as libc::c_int;
    }
    let mut fd: libc::c_int = dirfd(dirp);
    if cfg.blkorder() != 0 {
        num_files = 0 as libc::c_int as ullong_t;
        dir_blocks = 0 as libc::c_int as blkcnt_t;
        buf = g_buf.as_mut_ptr();
        if fstatat(fd, path, &mut sb_path, 0 as libc::c_int) == -(1 as libc::c_int) {
            current_block = 13318052949515782537;
        } else {
            if ihashbmp.is_null() {
                ihashbmp = calloc(
                    1 as libc::c_int as libc::c_ulong,
                    ((0xffffff as libc::c_int >> 6 as libc::c_int) << 3 as libc::c_int)
                        as libc::c_ulong,
                ) as *mut ullong_t;
                if ihashbmp.is_null() {
                    current_block = 13318052949515782537;
                } else {
                    current_block = 4166486009154926805;
                }
            } else {
                memset(
                    ihashbmp as *mut libc::c_void,
                    0 as libc::c_int,
                    ((0xffffff as libc::c_int >> 6 as libc::c_int) << 3 as libc::c_int)
                        as libc::c_ulong,
                );
                current_block = 4166486009154926805;
            }
            match current_block {
                13318052949515782537 => {}
                _ => {
                    if !prep_threads() {
                        current_block = 13318052949515782537;
                    } else {
                        wattr_on(
                            stdscr,
                            ((cfg.curctx() as libc::c_int + 1 as libc::c_int) as chtype)
                                << 0 as libc::c_int + 8 as libc::c_int
                                & ((1 as libc::c_uint) << 8 as libc::c_int)
                                    .wrapping_sub(1 as libc::c_uint)
                                    << 0 as libc::c_int + 8 as libc::c_int,
                            0 as *mut libc::c_void,
                        );
                        current_block = 8831408221741692167;
                    }
                }
            }
        }
    } else {
        current_block = 8831408221741692167;
    }
    match current_block {
        8831408221741692167 => {
            posix_fadvise(
                fd,
                0 as libc::c_int as off_t,
                0 as libc::c_int as off_t,
                2 as libc::c_int,
            );
            dp = readdir(dirp);
            if !dp.is_null() {
                if cfg.blkorder() as libc::c_int != 0
                    || (*dp).d_type as libc::c_int == DT_UNKNOWN as libc::c_int
                {
                    flags = 0x100 as libc::c_int;
                }
                loop {
                    namep = ((*dp).d_name).as_mut_ptr();
                    if !selforparent(namep) {
                        if cfg.showhidden() == 0
                            && *namep.offset(0 as libc::c_int as isize) as libc::c_int
                                == '.' as i32
                        {
                            if !(cfg.blkorder() == 0) {
                                if !(fstatat(fd, namep, &mut sb, 0x100 as libc::c_int)
                                    == -(1 as libc::c_int))
                                {
                                    if sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                        == 0o40000 as libc::c_int as libc::c_uint
                                    {
                                        if sb_path.st_dev == sb.st_dev {
                                            mkpath(path, namep, buf);
                                            dirwalk(buf, -(1 as libc::c_int), 0 as libc::c_int != 0);
                                            if g_state.interrupt() != 0 {
                                                break;
                                            }
                                        }
                                    } else {
                                        if sb.st_nlink <= 1 as libc::c_int as libc::c_ulong
                                            || test_set_bit(sb.st_ino as uint_t) as libc::c_int != 0
                                        {
                                            dir_blocks
                                                += if cfg.apparentsz() as libc::c_int != 0 {
                                                    sb.st_size
                                                } else {
                                                    sb.st_blocks
                                                };
                                        }
                                        num_files = num_files.wrapping_add(1);
                                        num_files;
                                    }
                                }
                            }
                        } else {
                            if fstatat(fd, namep, &mut sb, flags) == -(1 as libc::c_int)
                            {
                                if flags != 0
                                    || fstatat(fd, namep, &mut sb, 0x100 as libc::c_int)
                                        == -(1 as libc::c_int)
                                {
                                    flags == 0;
                                    entflags = 0x8 as libc::c_int as uchar_t;
                                    memset(
                                        &mut sb as *mut stat as *mut libc::c_void,
                                        0 as libc::c_int,
                                        ::std::mem::size_of::<stat>() as libc::c_ulong,
                                    );
                                } else {
                                    entflags = 0x4 as libc::c_int as uchar_t;
                                }
                            }
                            if ndents == total_dents {
                                if cfg.blkorder() != 0 {
                                    while active_threads != 0 {}
                                }
                                total_dents += 64 as libc::c_int;
                                *ppdents = xrealloc(
                                    *ppdents as *mut libc::c_void,
                                    (total_dents as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<entry>() as libc::c_ulong,
                                        ),
                                ) as *mut entry;
                                if (*ppdents).is_null() {
                                    free(pnamebuf as *mut libc::c_void);
                                    closedir(dirp);
                                    printerr(5673 as libc::c_int);
                                }
                            }
                            if namebuflen.wrapping_sub(off)
                                < (255 as libc::c_int + 1 as libc::c_int) as libc::c_ulong
                            {
                                namebuflen = (namebuflen as libc::c_ulong)
                                    .wrapping_add(0x800 as libc::c_int as libc::c_ulong)
                                    as size_t as size_t;
                                pnb = pnamebuf;
                                pnamebuf = xrealloc(
                                    pnamebuf as *mut libc::c_void,
                                    namebuflen,
                                ) as *mut libc::c_char;
                                if pnamebuf.is_null() {
                                    free(*ppdents as *mut libc::c_void);
                                    closedir(dirp);
                                    printerr(5687 as libc::c_int);
                                }
                                if pnb != pnamebuf {
                                    dentp = *ppdents;
                                    (*dentp).name = pnamebuf;
                                    let mut count: libc::c_int = 1 as libc::c_int;
                                    while count < ndents {
                                        let ref mut fresh30 = (*dentp
                                            .offset(1 as libc::c_int as isize))
                                            .name;
                                        *fresh30 = ((*dentp).name as size_t)
                                            .wrapping_add(
                                                ((*dentp).c2rust_unnamed).nlen() as libc::c_ulong,
                                            ) as *mut libc::c_char;
                                        dentp = dentp.offset(1);
                                        dentp;
                                        count += 1;
                                        count;
                                    }
                                }
                            }
                            dentp = (*ppdents).offset(ndents as isize);
                            (*dentp)
                                .name = (pnamebuf as size_t).wrapping_add(off)
                                as *mut libc::c_char;
                            ((*dentp).c2rust_unnamed)
                                .set_nlen(
                                    xstrsncpy(
                                        (*dentp).name,
                                        namep,
                                        (255 as libc::c_int + 1 as libc::c_int) as size_t,
                                    ) as ullong_t,
                                );
                            off = (off as libc::c_ulong)
                                .wrapping_add(
                                    ((*dentp).c2rust_unnamed).nlen() as libc::c_ulong,
                                ) as size_t as size_t;
                            if cfg.timetype() as libc::c_int == 2 as libc::c_int {
                                (*dentp).sec = sb.st_mtim.tv_sec;
                                (*dentp).nsec = sb.st_mtim.tv_nsec as uint_t;
                            } else if cfg.timetype() as libc::c_int == 0 as libc::c_int {
                                (*dentp).sec = sb.st_atim.tv_sec;
                                (*dentp).nsec = sb.st_atim.tv_nsec as uint_t;
                            } else {
                                (*dentp).sec = sb.st_ctim.tv_sec;
                                (*dentp).nsec = sb.st_ctim.tv_nsec as uint_t;
                            }
                            if gtimesecs - sb.st_mtim.tv_sec
                                <= 300 as libc::c_int as libc::c_long
                                || gtimesecs - sb.st_ctim.tv_sec
                                    <= 300 as libc::c_int as libc::c_long
                            {
                                entflags = (entflags as libc::c_int | 0x40 as libc::c_int)
                                    as uchar_t;
                            }
                            if flags == 0
                                && (*dp).d_type as libc::c_int == DT_LNK as libc::c_int
                            {
                                (*dentp)
                                    .mode = sb.st_mode
                                    & !(0o170000 as libc::c_int) as libc::c_uint
                                    | 0o120000 as libc::c_int as libc::c_uint;
                                (*dentp)
                                    .size = if !listpath.is_null() {
                                    sb.st_size
                                } else {
                                    0 as libc::c_int as libc::c_long
                                };
                            } else {
                                (*dentp).mode = sb.st_mode;
                                (*dentp).size = sb.st_size;
                            }
                            (*dentp).uid = sb.st_uid;
                            (*dentp).gid = sb.st_gid;
                            ((*dentp).c2rust_unnamed)
                                .set_flags(
                                    (if sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                        == 0o40000 as libc::c_int as libc::c_uint
                                    {
                                        0 as libc::c_int
                                    } else if sb.st_nlink > 1 as libc::c_int as libc::c_ulong {
                                        0x2 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    }) as ullong_t,
                                );
                            if entflags != 0 {
                                ((*dentp).c2rust_unnamed)
                                    .set_flags(
                                        ((*dentp).c2rust_unnamed).flags()
                                            | entflags as libc::c_int as ullong_t,
                                    );
                                entflags = 0 as libc::c_int as uchar_t;
                            }
                            if cfg.blkorder() != 0 {
                                if sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                    == 0o40000 as libc::c_int as libc::c_uint
                                {
                                    mkpath(path, namep, buf);
                                    dirwalk(buf, ndents, sb_path.st_dev != sb.st_dev);
                                    if g_state.interrupt() != 0 {
                                        break;
                                    }
                                } else {
                                    ((*dentp).c2rust_unnamed)
                                        .set_blocks(
                                            (if cfg.apparentsz() as libc::c_int != 0 {
                                                sb.st_size
                                            } else {
                                                sb.st_blocks
                                            }) as ullong_t,
                                        );
                                    if sb.st_nlink <= 1 as libc::c_int as libc::c_ulong
                                        || test_set_bit(sb.st_ino as uint_t) as libc::c_int != 0
                                    {
                                        dir_blocks = (dir_blocks as libc::c_ulonglong)
                                            .wrapping_add(((*dentp).c2rust_unnamed).blocks())
                                            as blkcnt_t as blkcnt_t;
                                    }
                                    num_files = num_files.wrapping_add(1);
                                    num_files;
                                }
                            }
                            if flags != 0 {
                                if sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                    == 0o120000 as libc::c_int as libc::c_uint
                                {
                                    sb.st_mode = 0 as libc::c_int as __mode_t;
                                    fstatat(fd, namep, &mut sb, 0 as libc::c_int);
                                }
                                if sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                    == 0o40000 as libc::c_int as libc::c_uint
                                {
                                    ((*dentp).c2rust_unnamed)
                                        .set_flags(
                                            ((*dentp).c2rust_unnamed).flags()
                                                | 0x1 as libc::c_int as ullong_t,
                                        );
                                }
                            } else if (*dp).d_type as libc::c_int
                                == DT_DIR as libc::c_int
                                || ((*dp).d_type as libc::c_int == DT_LNK as libc::c_int
                                    || (*dp).d_type as libc::c_int == DT_UNKNOWN as libc::c_int)
                                    && sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                        == 0o40000 as libc::c_int as libc::c_uint
                            {
                                ((*dentp).c2rust_unnamed)
                                    .set_flags(
                                        ((*dentp).c2rust_unnamed).flags()
                                            | 0x1 as libc::c_int as ullong_t,
                                    );
                            }
                            ndents += 1;
                            ndents;
                        }
                    }
                    dp = readdir(dirp);
                    if dp.is_null() {
                        break;
                    }
                }
            }
        }
        _ => {}
    }
    if g_state.duinit() as libc::c_int != 0 && cfg.blkorder() as libc::c_int != 0 {
        while active_threads != 0 {}
        wattr_off(
            stdscr,
            ((cfg.curctx() as libc::c_int + 1 as libc::c_int) as chtype)
                << 0 as libc::c_int + 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int,
            0 as *mut libc::c_void,
        );
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            num_files = (num_files as libc::c_ulonglong)
                .wrapping_add(*core_files.offset(i as isize)) as ullong_t as ullong_t;
            dir_blocks += *core_blocks.offset(i as isize);
            i += 1;
            i;
        }
    }
    if closedir(dirp) == -(1 as libc::c_int) {
        printerr(5811 as libc::c_int);
    }
    return ndents;
}
unsafe extern "C" fn populate(
    mut path: *mut libc::c_char,
    mut lastname: *mut libc::c_char,
) {
    ndents = dentfill(path, &mut pdents);
    if ndents == 0 {
        return;
    }
    qsort(
        pdents as *mut libc::c_void,
        ndents as size_t,
        ::std::mem::size_of::<entry>() as libc::c_ulong,
        entrycmpfn,
    );
    move_cursor(
        if *lastname as libc::c_int != 0 {
            dentfind(lastname, ndents)
        } else {
            0 as libc::c_int
        },
        0 as libc::c_int,
    );
    last_curscroll = -(1 as libc::c_int);
}
unsafe extern "C" fn notify_fifo(mut force: bool) {
    if fifopath.is_null() {
        return;
    }
    if fifofd == -(1 as libc::c_int) {
        fifofd = open(
            fifopath,
            0o1 as libc::c_int | 0o4000 as libc::c_int | 0o2000000 as libc::c_int,
        );
        if fifofd == -(1 as libc::c_int) {
            if *__errno_location() != 6 as libc::c_int {
                fifopath = 0 as *mut libc::c_char;
            }
            return;
        }
    }
    static mut lastentry: entry = entry {
        name: 0 as *mut libc::c_char,
        sec: 0,
        nsec: 0,
        mode: 0,
        size: 0,
        c2rust_unnamed: C2RustUnnamed_13 {
            blocks_nlen_flags: [0; 8],
        },
        uid: 0,
        gid: 0,
    };
    if !force
        && memcmp(
            &mut lastentry as *mut entry as *const libc::c_void,
            &mut *pdents.offset(cur as isize) as *mut entry as *const libc::c_void,
            ::std::mem::size_of::<entry>() as libc::c_ulong,
        ) == 0
    {
        return;
    }
    lastentry = *pdents.offset(cur as isize);
    let mut path: [libc::c_char; 4096] = [0; 4096];
    let mut len: size_t = mkpath(
        (g_ctx[cfg.curctx() as usize].c_path).as_mut_ptr(),
        if ndents != 0 {
            (*pdents.offset(cur as isize)).name as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        path.as_mut_ptr(),
    );
    path[len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = '\n' as i32 as libc::c_char;
    let mut ret: ssize_t = write(fifofd, path.as_mut_ptr() as *const libc::c_void, len);
    ret != len as ssize_t
        && !(ret == -(1 as libc::c_int) as libc::c_long
            && (*__errno_location() == 11 as libc::c_int
                || *__errno_location() == 32 as libc::c_int));
}
unsafe extern "C" fn send_to_explorer(mut presel: *mut libc::c_int) {
    if nselected != 0 {
        let mut fd: libc::c_int = open(
            fifopath,
            0o1 as libc::c_int | 0o4000 as libc::c_int | 0o2000000 as libc::c_int,
            0o600 as libc::c_int,
        );
        if fd == -(1 as libc::c_int)
            || seltofile(fd, 0 as *mut uint_t) != selbufpos as size_t
        {
            printwait(strerror(*__errno_location()), presel);
        } else {
            resetselind();
            clearselection();
        }
        if fd > 1 as libc::c_int {
            close(fd);
        }
    } else {
        notify_fifo(1 as libc::c_int != 0);
    };
}
unsafe extern "C" fn move_cursor(
    mut target: libc::c_int,
    mut ignore_scrolloff: libc::c_int,
) {
    let mut onscreen: libc::c_int = xlines as libc::c_int - 4 as libc::c_int;
    target = if 0 as libc::c_int
        > (if (ndents - 1 as libc::c_int) < target {
            ndents - 1 as libc::c_int
        } else {
            target
        })
    {
        0 as libc::c_int
    } else if (ndents - 1 as libc::c_int) < target {
        ndents - 1 as libc::c_int
    } else {
        target
    };
    last_curscroll = curscroll;
    last = cur;
    cur = target;
    if ignore_scrolloff == 0 {
        let mut delta: libc::c_int = target - last;
        let mut scrolloff: libc::c_int = if (3 as libc::c_int)
            < onscreen >> 1 as libc::c_int
        {
            3 as libc::c_int
        } else {
            onscreen >> 1 as libc::c_int
        };
        if cur < curscroll + scrolloff && delta < 0 as libc::c_int
            || cur > curscroll + onscreen - scrolloff - 1 as libc::c_int
                && delta > 0 as libc::c_int
        {
            curscroll += delta;
        }
    }
    curscroll = if curscroll
        < (if cur < ndents - onscreen { cur } else { ndents - onscreen })
    {
        curscroll
    } else if cur < ndents - onscreen {
        cur
    } else {
        ndents - onscreen
    };
    curscroll = if curscroll
        > (if cur - (onscreen - 1 as libc::c_int) > 0 as libc::c_int {
            cur - (onscreen - 1 as libc::c_int)
        } else {
            0 as libc::c_int
        })
    {
        curscroll
    } else if cur - (onscreen - 1 as libc::c_int) > 0 as libc::c_int {
        cur - (onscreen - 1 as libc::c_int)
    } else {
        0 as libc::c_int
    };
    if g_state.fifomode() == 0 {
        notify_fifo(0 as libc::c_int != 0);
    }
}
unsafe extern "C" fn handle_screen_move(mut sel: action) {
    let mut onscreen: libc::c_int = 0;
    let mut current_block_32: u64;
    match sel as libc::c_uint {
        4 => {
            if cfg.rollover() as libc::c_int != 0 || cur != ndents - 1 as libc::c_int {
                move_cursor((cur + 1 as libc::c_int) % ndents, 0 as libc::c_int);
            }
        }
        5 => {
            if cfg.rollover() as libc::c_int != 0 || cur != 0 {
                move_cursor(
                    (cur + ndents - 1 as libc::c_int) % ndents,
                    0 as libc::c_int,
                );
            }
        }
        6 => {
            onscreen = xlines as libc::c_int - 4 as libc::c_int;
            move_cursor(curscroll + (onscreen - 1 as libc::c_int), 1 as libc::c_int);
            curscroll += onscreen - 1 as libc::c_int;
        }
        8 => {
            onscreen = xlines as libc::c_int - 4 as libc::c_int;
            move_cursor(curscroll + (onscreen - 1 as libc::c_int), 1 as libc::c_int);
            curscroll += onscreen >> 1 as libc::c_int;
        }
        7 => {
            onscreen = xlines as libc::c_int - 4 as libc::c_int;
            move_cursor(curscroll, 1 as libc::c_int);
            curscroll -= onscreen - 1 as libc::c_int;
        }
        9 => {
            onscreen = xlines as libc::c_int - 4 as libc::c_int;
            move_cursor(curscroll, 1 as libc::c_int);
            curscroll -= onscreen >> 1 as libc::c_int;
        }
        13 => {
            let mut input: *mut libc::c_char = xreadline(
                0 as *const libc::c_char,
                b"jump (+n/-n/n): \0" as *const u8 as *const libc::c_char,
            );
            if !(input.is_null() || *input == 0) {
                if *input.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
                {
                    cur -= atoi(input.offset(1 as libc::c_int as isize));
                    if cur < 0 as libc::c_int {
                        cur = 0 as libc::c_int;
                    }
                    current_block_32 = 14648156034262866959;
                } else if *input.offset(0 as libc::c_int as isize) as libc::c_int
                    == '+' as i32
                {
                    cur += atoi(input.offset(1 as libc::c_int as isize));
                    if cur >= ndents {
                        cur = ndents - 1 as libc::c_int;
                    }
                    current_block_32 = 14648156034262866959;
                } else {
                    let mut index: libc::c_int = atoi(input);
                    if index < 1 as libc::c_int || index > ndents {
                        current_block_32 = 17784502470059252271;
                    } else {
                        cur = index - 1 as libc::c_int;
                        current_block_32 = 14648156034262866959;
                    }
                }
                match current_block_32 {
                    17784502470059252271 => {}
                    _ => {
                        onscreen = xlines as libc::c_int - 4 as libc::c_int;
                        move_cursor(cur, 1 as libc::c_int);
                        curscroll -= onscreen >> 1 as libc::c_int;
                    }
                }
            }
        }
        10 => {
            move_cursor(0 as libc::c_int, 1 as libc::c_int);
        }
        11 => {
            move_cursor(ndents - 1 as libc::c_int, 1 as libc::c_int);
        }
        _ => {
            let mut c: libc::c_int = get_input(messages[38 as libc::c_int as usize]);
            if !(c == 0) {
                c = if c >= 'a' as i32 && c <= 'z' as i32 {
                    c - 'a' as i32 + 'A' as i32
                } else {
                    c
                };
                let mut r: libc::c_int = if c
                    == (if *(*pdents.offset(cur as isize)).name as libc::c_int
                        >= 'a' as i32
                        && *(*pdents.offset(cur as isize)).name as libc::c_int
                            <= 'z' as i32
                    {
                        *(*pdents.offset(cur as isize)).name as libc::c_int - 'a' as i32
                            + 'A' as i32
                    } else {
                        *(*pdents.offset(cur as isize)).name as libc::c_int
                    })
                {
                    cur + 1 as libc::c_int
                } else {
                    0 as libc::c_int
                };
                while r < ndents {
                    if c == '\'' as i32
                        && ((*pdents.offset(r as isize)).c2rust_unnamed).flags()
                            as libc::c_int & 0x1 as libc::c_int == 0
                        || c
                            == (if *(*pdents.offset(r as isize)).name as libc::c_int
                                >= 'a' as i32
                                && *(*pdents.offset(r as isize)).name as libc::c_int
                                    <= 'z' as i32
                            {
                                *(*pdents.offset(r as isize)).name as libc::c_int
                                    - 'a' as i32 + 'A' as i32
                            } else {
                                *(*pdents.offset(r as isize)).name as libc::c_int
                            })
                    {
                        move_cursor(r % ndents, 0 as libc::c_int);
                        break;
                    } else {
                        r += 1;
                        r;
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn handle_openwith(
    mut path: *const libc::c_char,
    mut name: *const libc::c_char,
    mut newpath: *mut libc::c_char,
    mut tmp: *mut libc::c_char,
) {
    let mut r: libc::c_int = get_input(messages[12 as libc::c_int as usize]);
    r = if r == 'c' as i32 {
        0x8 as libc::c_int | 0x1 as libc::c_int
    } else if r == 'g' as i32 {
        0x2 as libc::c_int | 0x4 as libc::c_int | 0x1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    if r != 0 {
        mkpath(path, name, newpath);
        spawn(
            tmp,
            newpath,
            0 as *mut libc::c_char,
            0 as *mut libc::c_char,
            r as ushort_t,
        );
    }
}
unsafe extern "C" fn copynextname(mut lastname: *mut libc::c_char) {
    if cur != 0 {
        cur
            += if cur != ndents - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            };
        xstrsncpy(
            lastname,
            if ndents != 0 {
                (*pdents.offset(cur as isize)).name as *const libc::c_char
            } else {
                b"\0\0" as *const u8 as *const libc::c_char
            },
            (255 as libc::c_int + 1 as libc::c_int) as size_t,
        );
    } else {
        *lastname.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    };
}
unsafe extern "C" fn handle_context_switch(mut sel: action) -> libc::c_int {
    let mut r: libc::c_int = -(1 as libc::c_int);
    match sel as libc::c_uint {
        20 | 21 => {
            r = cfg.curctx() as libc::c_int;
            if sel as libc::c_uint == SEL_CYCLE as libc::c_int as libc::c_uint {
                loop {
                    r = r + 1 as libc::c_int & !(4 as libc::c_int);
                    if !((g_ctx[r as usize].c_cfg).ctxactive() == 0) {
                        break;
                    }
                }
            } else {
                loop {
                    r = r + 1 as libc::c_int & !(4 as libc::c_int);
                    if !((g_ctx[r as usize].c_cfg).ctxactive() as libc::c_int != 0
                        && r != cfg.curctx() as libc::c_int)
                    {
                        break;
                    }
                }
                if r == cfg.curctx() as libc::c_int {
                    loop {
                        r = r + (4 as libc::c_int - 1 as libc::c_int)
                            & 4 as libc::c_int - 1 as libc::c_int;
                        if !((g_ctx[r as usize].c_cfg).ctxactive() == 0) {
                            break;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if sel as libc::c_uint >= SEL_CTX1 as libc::c_int as libc::c_uint {
        r = (sel as libc::c_uint).wrapping_sub(SEL_CTX1 as libc::c_int as libc::c_uint)
            as libc::c_int;
    }
    if cfg.curctx() as libc::c_int == r {
        if sel as libc::c_uint == SEL_CYCLE as libc::c_int as libc::c_uint {
            if r == 4 as libc::c_int - 1 as libc::c_int {
                r = 0 as libc::c_int;
            } else {
                r += 1;
                r;
            };
        } else if sel as libc::c_uint == SEL_CYCLER as libc::c_int as libc::c_uint {
            if r == 0 as libc::c_int {
                r = 4 as libc::c_int - 1 as libc::c_int;
            } else {
                r -= 1;
                r;
            };
        } else {
            return -(1 as libc::c_int)
        }
    }
    return r;
}
unsafe extern "C" fn set_sort_flags(mut r: libc::c_int) -> libc::c_int {
    let mut session: bool = r == '\0' as i32;
    let mut reverse: bool = 0 as libc::c_int != 0;
    if r >= 'A' as i32 && r <= 'Z' as i32 && r != 'R' as i32 && r != 'C' as i32 {
        reverse = 1 as libc::c_int != 0;
        r = if r >= 'A' as i32 && r <= 'Z' as i32 {
            r - 'A' as i32 + 'a' as i32
        } else {
            r
        };
    }
    if session {
        if cfg.apparentsz() != 0 {
            cfg.set_apparentsz(0 as libc::c_int as uint_t);
            r = 'a' as i32;
        } else if cfg.blkorder() != 0 {
            cfg.set_blkorder(0 as libc::c_int as uint_t);
            r = 'd' as i32;
        }
        if cfg.version() != 0 {
            namecmpfn = Some(
                xstrverscasecmp
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> libc::c_int,
            );
        }
        if cfg.reverse() != 0 {
            entrycmpfn = Some(
                reventrycmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            );
        }
    } else if r == 'T' as i32 & 0x1f as libc::c_int {
        if cfg.timeorder() != 0 {
            r = 's' as i32;
        } else if cfg.sizeorder() != 0 {
            r = 'c' as i32;
        } else {
            r = 't' as i32;
        }
    }
    let mut current_block_86: u64;
    match r {
        97 => {
            cfg.set_apparentsz(cfg.apparentsz() ^ 1 as libc::c_int as uint_t);
            if cfg.apparentsz() != 0 {
                cfg.set_blkorder(1 as libc::c_int as uint_t);
                blk_shift = 0 as libc::c_int as uchar_t;
            } else {
                cfg.set_blkorder(0 as libc::c_int as uint_t);
            }
            current_block_86 = 7558426727818149287;
        }
        100 => {
            current_block_86 = 7558426727818149287;
        }
        99 => {
            cfg.set_timeorder(0 as libc::c_int as uint_t);
            cfg.set_sizeorder(0 as libc::c_int as uint_t);
            cfg.set_apparentsz(0 as libc::c_int as uint_t);
            cfg.set_blkorder(0 as libc::c_int as uint_t);
            cfg.set_extnorder(0 as libc::c_int as uint_t);
            cfg.set_reverse(0 as libc::c_int as uint_t);
            cfg.set_version(0 as libc::c_int as uint_t);
            entrycmpfn = Some(
                entrycmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            );
            namecmpfn = Some(
                xstricmp
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> libc::c_int,
            );
            current_block_86 = 16779030619667747692;
        }
        101 => {
            cfg.set_extnorder(cfg.extnorder() ^ 1 as libc::c_int as uint_t);
            cfg.set_sizeorder(0 as libc::c_int as uint_t);
            cfg.set_timeorder(0 as libc::c_int as uint_t);
            cfg.set_apparentsz(0 as libc::c_int as uint_t);
            cfg.set_blkorder(0 as libc::c_int as uint_t);
            cfg.set_reverse(0 as libc::c_int as uint_t);
            entrycmpfn = Some(
                entrycmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            );
            current_block_86 = 16779030619667747692;
        }
        114 => {
            cfg.set_reverse(cfg.reverse() ^ 1 as libc::c_int as uint_t);
            entrycmpfn = if cfg.reverse() as libc::c_int != 0 {
                Some(
                    reventrycmp
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                )
            } else {
                Some(
                    entrycmp
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                )
            };
            current_block_86 = 16779030619667747692;
        }
        115 => {
            cfg.set_sizeorder(cfg.sizeorder() ^ 1 as libc::c_int as uint_t);
            cfg.set_timeorder(0 as libc::c_int as uint_t);
            cfg.set_apparentsz(0 as libc::c_int as uint_t);
            cfg.set_blkorder(0 as libc::c_int as uint_t);
            cfg.set_extnorder(0 as libc::c_int as uint_t);
            cfg.set_reverse(0 as libc::c_int as uint_t);
            entrycmpfn = Some(
                entrycmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            );
            current_block_86 = 16779030619667747692;
        }
        116 => {
            cfg.set_timeorder(cfg.timeorder() ^ 1 as libc::c_int as uint_t);
            cfg.set_sizeorder(0 as libc::c_int as uint_t);
            cfg.set_apparentsz(0 as libc::c_int as uint_t);
            cfg.set_blkorder(0 as libc::c_int as uint_t);
            cfg.set_extnorder(0 as libc::c_int as uint_t);
            cfg.set_reverse(0 as libc::c_int as uint_t);
            entrycmpfn = Some(
                entrycmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            );
            current_block_86 = 16779030619667747692;
        }
        118 => {
            cfg.set_version(cfg.version() ^ 1 as libc::c_int as uint_t);
            namecmpfn = if cfg.version() as libc::c_int != 0 {
                Some(
                    xstrverscasecmp
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                        ) -> libc::c_int,
                )
            } else {
                Some(
                    xstricmp
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                        ) -> libc::c_int,
                )
            };
            cfg.set_timeorder(0 as libc::c_int as uint_t);
            cfg.set_sizeorder(0 as libc::c_int as uint_t);
            cfg.set_apparentsz(0 as libc::c_int as uint_t);
            cfg.set_blkorder(0 as libc::c_int as uint_t);
            cfg.set_extnorder(0 as libc::c_int as uint_t);
            current_block_86 = 16779030619667747692;
        }
        _ => return 0 as libc::c_int,
    }
    match current_block_86 {
        7558426727818149287 => {
            if r == 'd' as i32 {
                if cfg.apparentsz() == 0 {
                    cfg.set_blkorder(cfg.blkorder() ^ 1 as libc::c_int as uint_t);
                }
                cfg.set_apparentsz(0 as libc::c_int as uint_t);
                blk_shift = (ffs(512 as libc::c_int) - 1 as libc::c_int) as uchar_t;
            }
            if cfg.blkorder() != 0 {
                cfg.set_showdetail(1 as libc::c_int as uint_t);
            }
            cfg.set_timeorder(0 as libc::c_int as uint_t);
            cfg.set_sizeorder(0 as libc::c_int as uint_t);
            cfg.set_extnorder(0 as libc::c_int as uint_t);
            if !session {
                cfg.set_reverse(0 as libc::c_int as uint_t);
                entrycmpfn = Some(
                    entrycmp
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                );
            }
            endselection(1 as libc::c_int != 0);
        }
        _ => {}
    }
    if reverse {
        cfg.set_reverse(1 as libc::c_int as uint_t);
        entrycmpfn = Some(
            reventrycmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        );
    }
    cfgsort[cfg.curctx() as usize] = r as uchar_t;
    return r;
}
unsafe extern "C" fn set_time_type(mut presel: *mut libc::c_int) -> bool {
    let mut ret: bool = 0 as libc::c_int != 0;
    let mut buf: [libc::c_char; 32] = *::std::mem::transmute::<
        &[u8; 32],
        &mut [libc::c_char; 32],
    >(b"'a'ccess / 'c'hange / 'm'od [ ]\0");
    buf[(::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
        .wrapping_sub(3 as libc::c_int as libc::c_ulong)
        as usize] = (if cfg.timetype() as libc::c_int == 2 as libc::c_int {
        'm' as i32
    } else if cfg.timetype() as libc::c_int == 0 as libc::c_int {
        'a' as i32
    } else {
        'c' as i32
    }) as libc::c_char;
    let mut r: libc::c_int = get_input(buf.as_mut_ptr());
    if r == 'a' as i32 || r == 'c' as i32 || r == 'm' as i32 {
        r = if r == 'm' as i32 {
            2 as libc::c_int
        } else if r == 'a' as i32 {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        };
        if cfg.timetype() as libc::c_int != r {
            cfg.set_timetype(r as uint_t);
            if cfg.filtermode() as libc::c_int != 0
                || g_ctx[cfg.curctx() as usize].c_fltr[1 as libc::c_int as usize]
                    as libc::c_int != 0
            {
                *presel = '/' as i32;
            }
            ret = 1 as libc::c_int != 0;
        } else {
            r = 41 as libc::c_int;
        }
    } else {
        r = 40 as libc::c_int;
    }
    if !ret {
        printwait(messages[r as usize], presel);
    }
    return ret;
}
unsafe extern "C" fn statusbar(mut path: *mut libc::c_char) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pent: pEntry = &mut *pdents.offset(cur as isize) as *mut entry;
    if ndents == 0 {
        printmsg(b"0/0\0" as *const u8 as *const libc::c_char);
        return;
    }
    if (*pent).mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
    {
        i = ((*pent).c2rust_unnamed).nlen() as libc::c_int - 1 as libc::c_int;
        ptr = xextension((*pent).name, i as size_t);
        if !ptr.is_null() {
            len = (i as libc::c_long - ptr.offset_from((*pent).name) as libc::c_long)
                as libc::c_int;
        }
        if ptr.is_null() || len > 5 as libc::c_int || len < 2 as libc::c_int {
            ptr = b"\x08\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    } else {
        ptr = b"\x08\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    wattr_on(
        stdscr,
        ((cfg.curctx() as libc::c_int + 1 as libc::c_int) as chtype)
            << 0 as libc::c_int + 8 as libc::c_int
            & ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int,
        0 as *mut libc::c_void,
    );
    if cfg.fileinfo() as libc::c_int != 0
        && get_output(
            b"file\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"-b\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*pdents.offset(cur as isize)).name,
            -(1 as libc::c_int),
            0 as libc::c_int != 0,
            0 as libc::c_int != 0,
        ) as libc::c_int != 0
    {
        if wmove(stdscr, xlines as libc::c_int - 2 as libc::c_int, 2 as libc::c_int)
            == -(1 as libc::c_int)
        {
            -(1 as libc::c_int);
        } else {
            waddnstr(stdscr, g_buf.as_mut_ptr(), -(1 as libc::c_int));
        };
    }
    wmove(stdscr, xlines as libc::c_int - 1 as libc::c_int, 0 as libc::c_int);
    printw(
        b"%d/%s \0" as *const u8 as *const libc::c_char,
        cur + 1 as libc::c_int,
        xitoa(ndents as uint_t),
    );
    if g_state.selmode() as libc::c_int != 0 || nselected != 0 {
        wattr_on(
            stdscr,
            (1 as libc::c_uint) << 10 as libc::c_int + 8 as libc::c_int,
            0 as *mut libc::c_void,
        );
        waddch(stdscr, ' ' as i32 as chtype);
        if g_state.rangesel() != 0 {
            waddch(stdscr, '*' as i32 as chtype);
        } else if g_state.selmode() != 0 {
            waddch(stdscr, '+' as i32 as chtype);
        }
        if nselected != 0 {
            waddnstr(stdscr, xitoa(nselected as uint_t), -(1 as libc::c_int));
        }
        waddch(stdscr, ' ' as i32 as chtype);
        wattr_off(
            stdscr,
            (1 as libc::c_uint) << 10 as libc::c_int + 8 as libc::c_int,
            0 as *mut libc::c_void,
        );
        waddch(stdscr, ' ' as i32 as chtype);
    }
    if cfg.blkorder() != 0 {
        let mut buf: [libc::c_char; 24] = [0; 24];
        xstrsncpy(
            buf.as_mut_ptr(),
            coolsize(dir_blocks << blk_shift as libc::c_int),
            12 as libc::c_int as size_t,
        );
        printw(
            b"%cu:%s avail:%s files:%llu %lluB %s\n\0" as *const u8
                as *const libc::c_char,
            if cfg.apparentsz() as libc::c_int != 0 { 'a' as i32 } else { 'd' as i32 },
            buf.as_mut_ptr(),
            coolsize(get_fs_info(path, 0 as libc::c_int as uchar_t) as off_t),
            num_files,
            ((*pent).c2rust_unnamed).blocks() << blk_shift as libc::c_int,
            ptr,
        );
    } else {
        let mut sort: [libc::c_char; 6] = *::std::mem::transmute::<
            &[u8; 6],
            &mut [libc::c_char; 6],
        >(b"\0\0\0\0\0\0");
        if getorderstr(sort.as_mut_ptr()) != 0 {
            waddnstr(stdscr, sort.as_mut_ptr(), -(1 as libc::c_int));
        }
        print_time(&mut (*pent).sec, ((*pent).c2rust_unnamed).flags() as uchar_t);
        waddch(stdscr, ' ' as i32 as chtype);
        waddnstr(stdscr, get_lsperms((*pent).mode), -(1 as libc::c_int));
        waddch(stdscr, ' ' as i32 as chtype);
        if g_state.uidgid() != 0 {
            waddnstr(stdscr, getpwname((*pent).uid), -(1 as libc::c_int));
            waddch(stdscr, ':' as i32 as chtype);
            waddnstr(stdscr, getgrname((*pent).gid), -(1 as libc::c_int));
            waddch(stdscr, ' ' as i32 as chtype);
        }
        if (*pent).mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
        {
            if cfg.fileinfo() == 0 {
                i = readlink(
                    (*pent).name,
                    g_buf.as_mut_ptr(),
                    4096 as libc::c_int as size_t,
                ) as libc::c_int;
                waddnstr(
                    stdscr,
                    coolsize(
                        if i >= 0 as libc::c_int {
                            i as libc::c_long
                        } else {
                            (*pent).size
                        },
                    ),
                    -(1 as libc::c_int),
                );
                if i > 1 as libc::c_int {
                    let mut y: libc::c_int = 0;
                    waddnstr(
                        stdscr,
                        b" ->\0" as *const u8 as *const libc::c_char,
                        -(1 as libc::c_int),
                    );
                    len = (if !(stdscr as *const libc::c_void).is_null() {
                        (*stdscr)._cury as libc::c_int
                    } else {
                        -(1 as libc::c_int)
                    });
                    y = (if !(stdscr as *const libc::c_void).is_null() {
                        (*stdscr)._curx as libc::c_int
                    } else {
                        -(1 as libc::c_int)
                    });
                    i = if i < xcols as libc::c_int - y {
                        i
                    } else {
                        xcols as libc::c_int - y
                    };
                    g_buf[i as usize] = '\0' as i32 as libc::c_char;
                    waddnstr(stdscr, g_buf.as_mut_ptr(), -(1 as libc::c_int));
                }
            }
        } else {
            waddnstr(stdscr, coolsize((*pent).size), -(1 as libc::c_int));
            waddch(stdscr, ' ' as i32 as chtype);
            waddnstr(stdscr, ptr, -(1 as libc::c_int));
            if ((*pent).c2rust_unnamed).flags() as libc::c_int & 0x2 as libc::c_int != 0
            {
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
                if stat((*pent).name, &mut sb) != -(1 as libc::c_int) {
                    waddch(stdscr, ' ' as i32 as chtype);
                    waddnstr(
                        stdscr,
                        xitoa(sb.st_nlink as libc::c_int as uint_t),
                        -(1 as libc::c_int),
                    );
                    waddch(stdscr, '-' as i32 as chtype);
                    waddnstr(
                        stdscr,
                        xitoa(sb.st_ino as libc::c_int as uint_t),
                        -(1 as libc::c_int),
                    );
                }
            }
        }
        wclrtoeol(stdscr);
    }
    wattr_off(
        stdscr,
        ((cfg.curctx() as libc::c_int + 1 as libc::c_int) as chtype)
            << 0 as libc::c_int + 8 as libc::c_int
            & ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int,
        0 as *mut libc::c_void,
    );
    wmove(stdscr, cur + 2 as libc::c_int - curscroll, 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn markhovered() {
    if cfg.showdetail() as libc::c_int != 0 && ndents != 0 {
        wmove(stdscr, cur + 2 as libc::c_int - curscroll, 0 as libc::c_int);
        waddch(
            stdscr,
            '>' as i32 as libc::c_uint
                | (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int,
        );
    }
}
unsafe extern "C" fn adjust_cols(mut n: libc::c_int) -> libc::c_int {
    if cfg.showdetail() != 0 {
        if n < 36 as libc::c_int {
            cfg.set_showdetail(cfg.showdetail() ^ 1 as libc::c_int as uint_t);
        } else {
            n -= 32 as libc::c_int;
        }
    }
    return n - 2 as libc::c_int;
}
unsafe extern "C" fn draw_line(mut ncols: libc::c_int) {
    let mut dir: bool = 0 as libc::c_int != 0;
    ncols = adjust_cols(ncols);
    if g_state.oldcolor() as libc::c_int != 0
        && ((*pdents.offset(last as isize)).c2rust_unnamed).flags() as libc::c_int
            & 0x1 as libc::c_int != 0
    {
        wattr_on(
            stdscr,
            ((cfg.curctx() as libc::c_int + 1 as libc::c_int) as chtype)
                << 0 as libc::c_int + 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int
                | (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int,
            0 as *mut libc::c_void,
        );
        dir = 1 as libc::c_int != 0;
    }
    wmove(stdscr, 2 as libc::c_int + last - curscroll, 0 as libc::c_int);
    printent(&mut *pdents.offset(last as isize), ncols as uint_t, 0 as libc::c_int != 0);
    if g_state.oldcolor() as libc::c_int != 0
        && ((*pdents.offset(cur as isize)).c2rust_unnamed).flags() as libc::c_int
            & 0x1 as libc::c_int != 0
    {
        if !dir {
            wattr_on(
                stdscr,
                ((cfg.curctx() as libc::c_int + 1 as libc::c_int) as chtype)
                    << 0 as libc::c_int + 8 as libc::c_int
                    & ((1 as libc::c_uint) << 8 as libc::c_int)
                        .wrapping_sub(1 as libc::c_uint)
                        << 0 as libc::c_int + 8 as libc::c_int
                    | (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int,
                0 as *mut libc::c_void,
            );
            dir = 1 as libc::c_int != 0;
        }
    } else if dir {
        wattr_off(
            stdscr,
            ((cfg.curctx() as libc::c_int + 1 as libc::c_int) as chtype)
                << 0 as libc::c_int + 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int
                | (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int,
            0 as *mut libc::c_void,
        );
        dir = 0 as libc::c_int != 0;
    }
    wmove(stdscr, 2 as libc::c_int + cur - curscroll, 0 as libc::c_int);
    printent(&mut *pdents.offset(cur as isize), ncols as uint_t, 1 as libc::c_int != 0);
    if dir {
        wattr_off(
            stdscr,
            ((cfg.curctx() as libc::c_int + 1 as libc::c_int) as chtype)
                << 0 as libc::c_int + 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int
                | (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int,
            0 as *mut libc::c_void,
        );
    }
    markhovered();
}
unsafe extern "C" fn redraw(mut path: *mut libc::c_char) {
    xlines = (if !(stdscr as *const libc::c_void).is_null() {
        (*stdscr)._maxy as libc::c_int + 1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    }) as ushort_t;
    xcols = (if !(stdscr as *const libc::c_void).is_null() {
        (*stdscr)._maxx as libc::c_int + 1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    }) as ushort_t;
    let mut ncols: libc::c_int = if xcols as libc::c_int <= 4096 as libc::c_int {
        xcols as libc::c_int
    } else {
        4096 as libc::c_int
    };
    let mut onscreen: libc::c_int = xlines as libc::c_int - 4 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 1 as libc::c_int;
    if g_state.move_0() != 0 {
        g_state.set_move_0(0 as libc::c_int as uint_t);
        if ndents != 0 && last_curscroll == curscroll {
            return draw_line(ncols);
        }
    }
    werase(stdscr);
    move_cursor(cur, 1 as libc::c_int);
    if ncols <= 4 as libc::c_int * 2 as libc::c_int {
        printmsg(messages[29 as libc::c_int as usize]);
        return;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if (g_ctx[i as usize].c_cfg).ctxactive() == 0 {
            waddch(stdscr, (i + '1' as i32) as chtype);
        } else {
            waddch(
                stdscr,
                (i + '1' as i32) as libc::c_uint
                    | (((i + 1 as libc::c_int) as chtype)
                        << 0 as libc::c_int + 8 as libc::c_int
                        & ((1 as libc::c_uint) << 8 as libc::c_int)
                            .wrapping_sub(1 as libc::c_uint)
                            << 0 as libc::c_int + 8 as libc::c_int
                        | (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int
                        | (if cfg.curctx() as libc::c_int != i {
                            (1 as libc::c_uint) << 9 as libc::c_int + 8 as libc::c_int
                        } else {
                            (1 as libc::c_uint) << 10 as libc::c_int + 8 as libc::c_int
                        })),
            );
        }
        waddch(stdscr, ' ' as i32 as chtype);
        i += 1;
        i;
    }
    wattr_on(
        stdscr,
        (1 as libc::c_uint) << 9 as libc::c_int + 8 as libc::c_int
            | ((cfg.curctx() as libc::c_int + 1 as libc::c_int) as chtype)
                << 0 as libc::c_int + 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int,
        0 as *mut libc::c_void,
    );
    let mut in_home: bool = set_tilde_in_path(path);
    let mut ptr: *mut libc::c_char = if in_home as libc::c_int != 0 {
        &mut *path.offset((homelen as libc::c_int - 1 as libc::c_int) as isize)
            as *mut libc::c_char
    } else {
        path
    };
    i = xstrlen(ptr) as libc::c_int;
    if i + 4 as libc::c_int * 2 as libc::c_int <= ncols {
        waddnstr(stdscr, ptr, ncols - 4 as libc::c_int * 2 as libc::c_int);
    } else {
        let mut base: *mut libc::c_char = xmemrchr(
            ptr as *mut uchar_t,
            '/' as i32 as uchar_t,
            i as size_t,
        ) as *mut libc::c_char;
        if in_home {
            waddch(stdscr, *ptr as chtype);
            ptr = ptr.offset(1);
            ptr;
            i = 1 as libc::c_int;
        } else {
            i = 0 as libc::c_int;
        }
        if !ptr.is_null() && base != ptr {
            while ptr < base {
                if *ptr as libc::c_int == '/' as i32 {
                    i += 2 as libc::c_int;
                    if ncols < i + 4 as libc::c_int * 2 as libc::c_int {
                        base = 0 as *mut libc::c_char;
                        break;
                    } else {
                        waddch(stdscr, *ptr as chtype);
                        ptr = ptr.offset(1);
                        waddch(stdscr, *ptr as chtype);
                    }
                }
                ptr = ptr.offset(1);
                ptr;
            }
        }
        if !base.is_null() {
            waddnstr(stdscr, base, ncols - (4 as libc::c_int * 2 as libc::c_int + i));
        }
    }
    if in_home {
        reset_tilde_in_path(path);
    }
    wattr_off(
        stdscr,
        (1 as libc::c_uint) << 9 as libc::c_int + 8 as libc::c_int
            | ((cfg.curctx() as libc::c_int + 1 as libc::c_int) as chtype)
                << 0 as libc::c_int + 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int,
        0 as *mut libc::c_void,
    );
    if curscroll > 0 as libc::c_int {
        wmove(stdscr, 1 as libc::c_int, 0 as libc::c_int);
        waddch(stdscr, '^' as i32 as chtype);
    }
    if g_state.oldcolor() != 0 {
        wattr_on(
            stdscr,
            ((cfg.curctx() as libc::c_int + 1 as libc::c_int) as chtype)
                << 0 as libc::c_int + 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int
                | (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int,
            0 as *mut libc::c_void,
        );
        g_state.set_dircolor(1 as libc::c_int as uint_t);
    }
    onscreen = if onscreen + curscroll < ndents { onscreen + curscroll } else { ndents };
    ncols = adjust_cols(ncols);
    let mut len: libc::c_int = scanselforpath(path, 0 as libc::c_int != 0);
    i = curscroll;
    while i < onscreen {
        j += 1;
        wmove(stdscr, j, 0 as libc::c_int);
        if len != 0 {
            findmarkentry(len as size_t, &mut *pdents.offset(i as isize));
        }
        printent(&mut *pdents.offset(i as isize), ncols as uint_t, i == cur);
        i += 1;
        i;
    }
    if g_state.dircolor() != 0 {
        wattr_off(
            stdscr,
            ((cfg.curctx() as libc::c_int + 1 as libc::c_int) as chtype)
                << 0 as libc::c_int + 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int
                | (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int,
            0 as *mut libc::c_void,
        );
        g_state.set_dircolor(0 as libc::c_int as uint_t);
    }
    if onscreen < ndents {
        wmove(stdscr, xlines as libc::c_int - 2 as libc::c_int, 0 as libc::c_int);
        waddch(stdscr, 'v' as i32 as chtype);
    }
    markhovered();
}
unsafe extern "C" fn cdprep(
    mut lastdir: *mut libc::c_char,
    mut lastname: *mut libc::c_char,
    mut path: *mut libc::c_char,
    mut newpath: *mut libc::c_char,
) -> bool {
    if !lastname.is_null() {
        *lastname.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
    xstrsncpy(lastdir, path, 4096 as libc::c_int as size_t);
    xstrsncpy(path, newpath, 4096 as libc::c_int as size_t);
    clearfilter();
    return cfg.filtermode() != 0;
}
unsafe extern "C" fn browse(
    mut ipath: *mut libc::c_char,
    mut session: *const libc::c_char,
    mut pkey: libc::c_int,
) -> bool {
    let mut fd: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut current_block: u64;
    let mut newpath: [libc::c_char; 4096] = [0; 4096];
    let mut runfile: [libc::c_char; 256] = [0; 256];
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lastdir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lastname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pent: pEntry = 0 as *mut entry;
    let mut sel: action = 0 as action;
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
    let mut r: libc::c_int = -(1 as libc::c_int);
    let mut presel: libc::c_int = 0;
    let mut selstartid: libc::c_int = 0 as libc::c_int;
    let mut selendid: libc::c_int = 0 as libc::c_int;
    let opener_flags: uchar_t = (if cfg.cliopener() as libc::c_int != 0 {
        0x8 as libc::c_int | 0x1 as libc::c_int
    } else {
        0x4 as libc::c_int | 0x40 as libc::c_int | 0x2 as libc::c_int
    }) as uchar_t;
    let mut watch: bool = 0 as libc::c_int != 0;
    let mut cd: bool = 1 as libc::c_int != 0;
    let mut inode: ino_t = 0 as libc::c_int as ino_t;
    let mut event: MEVENT = {
        let mut init = MEVENT {
            id: 0 as libc::c_int as libc::c_short,
            x: 0,
            y: 0,
            z: 0,
            bstate: 0,
        };
        init
    };
    let mut mousetimings: [timespec; 2] = [
        {
            let mut init = timespec {
                tv_sec: 0 as libc::c_int as __time_t,
                tv_nsec: 0 as libc::c_int as __syscall_slong_t,
            };
            init
        },
        {
            let mut init = timespec {
                tv_sec: 0 as libc::c_int as __time_t,
                tv_nsec: 0 as libc::c_int as __syscall_slong_t,
            };
            init
        },
    ];
    let mut mousedent: [libc::c_int; 2] = [-(1 as libc::c_int), -(1 as libc::c_int)];
    let mut currentmouse: bool = 1 as libc::c_int != 0;
    let mut rightclicksel: bool = 0 as libc::c_int != 0;
    atexit(Some(dentfree as unsafe extern "C" fn() -> ()));
    xlines = (if !(stdscr as *const libc::c_void).is_null() {
        (*stdscr)._maxy as libc::c_int + 1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    }) as ushort_t;
    xcols = (if !(stdscr as *const libc::c_void).is_null() {
        (*stdscr)._maxx as libc::c_int + 1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    }) as ushort_t;
    if session.is_null()
        || !load_session(
            session,
            &mut path,
            &mut lastdir,
            &mut lastname,
            0 as libc::c_int != 0,
        )
    {
        g_ctx[0 as libc::c_int as usize]
            .c_last[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        lastdir = (g_ctx[0 as libc::c_int as usize].c_last).as_mut_ptr();
        if g_state.initfile() != 0 {
            xstrsncpy(
                (g_ctx[0 as libc::c_int as usize].c_name).as_mut_ptr(),
                xbasename(ipath),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            );
            xdirname(ipath);
        } else {
            g_ctx[0 as libc::c_int as usize]
                .c_name[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        }
        lastname = (g_ctx[0 as libc::c_int as usize].c_name).as_mut_ptr();
        xstrsncpy(
            (g_ctx[0 as libc::c_int as usize].c_path).as_mut_ptr(),
            ipath,
            4096 as libc::c_int as size_t,
        );
        if g_state.initfile() != 0 {
            free(initpath as *mut libc::c_void);
            ipath = getcwd(0 as *mut libc::c_char, 0 as libc::c_int as size_t);
            initpath = ipath;
        }
        path = (g_ctx[0 as libc::c_int as usize].c_path).as_mut_ptr();
        g_ctx[0 as libc::c_int as usize]
            .c_fltr[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        g_ctx[0 as libc::c_int as usize]
            .c_fltr[0 as libc::c_int
            as usize] = g_ctx[0 as libc::c_int as usize]
            .c_fltr[1 as libc::c_int as usize];
        g_ctx[0 as libc::c_int as usize].c_cfg = cfg;
    }
    runfile[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    newpath[0 as libc::c_int as usize] = runfile[0 as libc::c_int as usize];
    presel = if pkey != 0 {
        ';' as i32
    } else if cfg.filtermode() as libc::c_int != 0
        || !session.is_null()
            && (g_ctx[cfg.curctx() as usize].c_fltr[0 as libc::c_int as usize]
                as libc::c_int == '/' as i32
                || g_ctx[cfg.curctx() as usize].c_fltr[0 as libc::c_int as usize]
                    as libc::c_int == '\\' as i32)
            && g_ctx[cfg.curctx() as usize].c_fltr[1 as libc::c_int as usize]
                as libc::c_int != 0
    {
        '/' as i32
    } else {
        0 as libc::c_int
    };
    pdents = xrealloc(
        pdents as *mut libc::c_void,
        (total_dents as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<entry>() as libc::c_ulong),
    ) as *mut entry;
    if pdents.is_null() {
        printerr(6626 as libc::c_int);
    }
    pnamebuf = xrealloc(pnamebuf as *mut libc::c_void, 0x800 as libc::c_int as size_t)
        as *mut libc::c_char;
    if pnamebuf.is_null() {
        printerr(6631 as libc::c_int);
    }
    if presel == '/' as i32 {
        handle_key_resize();
    }
    '_begin: loop {
        if chdir(path) == -(1 as libc::c_int) {
            valid_parent(path, lastname);
            if cfg.filtermode() as libc::c_int != 0 {
                presel = '/' as i32;
            } else {
                watch = 1 as libc::c_int != 0;
            };
        }
        xterm_cfg(path);
        if (presel == '/' as i32 || watch as libc::c_int != 0)
            && inotify_wd >= 0 as libc::c_int
        {
            inotify_rm_watch(inotify_fd, inotify_wd);
            inotify_wd = -(1 as libc::c_int);
            watch = 0 as libc::c_int != 0;
        }
        if !order.is_null() && cd as libc::c_int != 0 {
            if cfgsort[cfg.curctx() as usize] as libc::c_int != '0' as i32 {
                if cfgsort[cfg.curctx() as usize] as libc::c_int == 'z' as i32 {
                    set_sort_flags('c' as i32);
                }
                if (cfgsort[cfg.curctx() as usize] == 0
                    || cfgsort[cfg.curctx() as usize] as libc::c_int == 'c' as i32)
                    && {
                        r = get_kv_key(
                            order,
                            path,
                            maxorder,
                            11 as libc::c_int as uchar_t,
                        );
                        r > 0 as libc::c_int
                    }
                {
                    set_sort_flags(r);
                    cfgsort[cfg.curctx() as usize] = 'z' as i32 as uchar_t;
                }
            } else {
                cfgsort[cfg.curctx() as usize] = cfgsort[4 as libc::c_int as usize];
            }
        }
        cd = 1 as libc::c_int != 0;
        populate(path, lastname);
        if g_state.interrupt() != 0 {
            cfg.set_blkorder(0 as libc::c_int as uint_t);
            cfg.set_apparentsz(cfg.blkorder());
            g_state.set_interrupt(cfg.apparentsz());
            blk_shift = 9 as libc::c_int as uchar_t;
            presel = 'L' as i32 & 0x1f as libc::c_int;
        }
        if presel != '/' as i32 && inotify_wd == -(1 as libc::c_int) {
            inotify_wd = inotify_add_watch(inotify_fd, path, INOTIFY_MASK);
        }
        's_254: loop {
            if presel != '/' as i32
                || g_ctx[cfg.curctx() as usize].c_fltr[1 as libc::c_int as usize] == 0
            {
                redraw(path);
                statusbar(path);
            }
            loop {
                if getppid() == 1 as libc::c_int {
                    _exit(1 as libc::c_int);
                }
                if chdir(path) == -(1 as libc::c_int) {
                    continue '_begin;
                }
                if isatty(0 as libc::c_int) == 0 && g_state.picker() == 0 {
                    return 1 as libc::c_int != 0;
                }
                sel = nextsel(presel) as action;
                if presel != 0 {
                    presel = 0 as libc::c_int;
                }
                match sel as libc::c_uint {
                    66 => {
                        if getmouse(&mut event) != 0 as libc::c_int {
                            continue;
                        }
                        if event.bstate as libc::c_long
                            == (0o2 as libc::c_long)
                                << (1 as libc::c_int - 1 as libc::c_int) * 5 as libc::c_int
                            && event.y == 0 as libc::c_int
                        {
                            r = event.x >> 1 as libc::c_int;
                            if r >= 4 as libc::c_int {
                                sel = SEL_BACK;
                            } else if r >= 0 as libc::c_int
                                && r != cfg.curctx() as libc::c_int
                            {
                                savecurctx(
                                    path,
                                    if ndents != 0 {
                                        (*pdents.offset(cur as isize)).name
                                    } else {
                                        0 as *mut libc::c_char
                                    },
                                    r,
                                );
                                path = (g_ctx[r as usize].c_path).as_mut_ptr();
                                lastdir = (g_ctx[r as usize].c_last).as_mut_ptr();
                                lastname = (g_ctx[r as usize].c_name).as_mut_ptr();
                                if cfg.filtermode() as libc::c_int != 0 {
                                    presel = '/' as i32;
                                } else {
                                    watch = 1 as libc::c_int != 0;
                                };
                                continue '_begin;
                            }
                            current_block = 8492650606187108553;
                        } else {
                            current_block = 8492650606187108553;
                        }
                    }
                    1 => {
                        current_block = 8492650606187108553;
                    }
                    3 => {
                        current_block = 15252062886580107196;
                    }
                    2 => {
                        current_block = 5766893457425082621;
                    }
                    4 => {
                        current_block = 3562629391805533650;
                        break;
                    }
                    5 => {
                        current_block = 3562629391805533650;
                        break;
                    }
                    6 => {
                        current_block = 14319209798712792045;
                        break;
                    }
                    8 => {
                        current_block = 17136875676406196640;
                        break;
                    }
                    7 => {
                        current_block = 161814443543167606;
                        break;
                    }
                    9 => {
                        current_block = 2423117211301313142;
                        break;
                    }
                    10 => {
                        current_block = 8733027823202672466;
                        break;
                    }
                    11 => {
                        current_block = 18162755968531476823;
                        break;
                    }
                    12 | 13 => {
                        current_block = 2389127549643710708;
                        break;
                    }
                    14 => {
                        current_block = 16949939609931944443;
                    }
                    15 => {
                        current_block = 16949939609931944443;
                    }
                    16 | 17 => {
                        current_block = 942779372144332707;
                    }
                    18 => {
                        current_block = 12333446989980225856;
                    }
                    19 => {
                        if sel as libc::c_uint
                            == SEL_REMOTE as libc::c_int as libc::c_uint
                            && !remote_mount(newpath.as_mut_ptr())
                        {
                            presel = '$' as i32;
                            continue;
                        } else {
                            set_smart_ctx(
                                '+' as i32,
                                newpath.as_mut_ptr(),
                                &mut path,
                                if ndents != 0 {
                                    (*pdents.offset(cur as isize)).name
                                } else {
                                    0 as *mut libc::c_char
                                },
                                &mut lastname,
                                &mut lastdir,
                            );
                            clearfilter();
                            continue '_begin;
                        }
                    }
                    20 => {
                        current_block = 12869885497274271669;
                        break;
                    }
                    21 => {
                        current_block = 12869885497274271669;
                        break;
                    }
                    22 => {
                        current_block = 13615634435221404298;
                        break;
                    }
                    23 => {
                        current_block = 16923785083244880615;
                        break;
                    }
                    24 | 25 => {
                        current_block = 9877275528163147575;
                        break;
                    }
                    26 => {
                        free(mark as *mut libc::c_void);
                        mark = xstrdup(path);
                        printwait(mark, &mut presel);
                        continue;
                    }
                    27 => {
                        add_bookmark(path, newpath.as_mut_ptr(), &mut presel);
                        continue;
                    }
                    28 => {
                        if inotify_wd >= 0 as libc::c_int {
                            inotify_rm_watch(inotify_fd, inotify_wd);
                            inotify_wd = -(1 as libc::c_int);
                        }
                        presel = filterentries(path, lastname);
                        if presel == '[' as i32 & 0x1f as libc::c_int {
                            presel = 0 as libc::c_int;
                            continue 's_254;
                        } else {
                            if !(presel == '/' as i32) {
                                continue;
                            }
                            cd = 0 as libc::c_int != 0;
                            continue '_begin;
                        }
                    }
                    29 => {
                        current_block = 4162367029103048979;
                    }
                    30 => {
                        current_block = 4162367029103048979;
                    }
                    31 | 35 => {
                        current_block = 7126770851743004489;
                    }
                    32 | 33 => {
                        if !(ndents != 0) {
                            continue 's_254;
                        }
                        tmp = if !listpath.is_null()
                            && (if *path as libc::c_int != *listpath as libc::c_int {
                                -(1 as libc::c_int)
                            } else {
                                strcmp(path, listpath)
                            }) == 0 as libc::c_int
                        {
                            listroot
                        } else {
                            path
                        };
                        mkpath(
                            tmp,
                            (*pdents.offset(cur as isize)).name,
                            newpath.as_mut_ptr(),
                        );
                        if sel as libc::c_uint
                            == SEL_STATS as libc::c_int as libc::c_uint
                            && !show_stats(newpath.as_mut_ptr())
                            || lstat(newpath.as_mut_ptr(), &mut sb)
                                == -(1 as libc::c_int)
                            || sel as libc::c_uint
                                == SEL_CHMODX as libc::c_int as libc::c_uint
                                && !xchmod(newpath.as_mut_ptr(), sb.st_mode)
                        {
                            printwait(strerror(*__errno_location()), &mut presel);
                            continue;
                        } else {
                            if sel as libc::c_uint
                                == SEL_CHMODX as libc::c_int as libc::c_uint
                            {
                                let ref mut fresh31 = (*pdents.offset(cur as isize)).mode;
                                *fresh31 ^= 0o111 as libc::c_int as libc::c_uint;
                            }
                            continue 's_254;
                        }
                    }
                    36 => {
                        current_block = 4041062724260856750;
                    }
                    49 => {
                        current_block = 4041062724260856750;
                    }
                    51 => {
                        current_block = 6070906628441949502;
                    }
                    52 => {
                        current_block = 6885328239442086976;
                    }
                    53 | 58 => {
                        current_block = 4656275727158130286;
                    }
                    37 => {
                        if ndents == 0 {
                            continue;
                        }
                        startselection();
                        if g_state.rangesel() != 0 {
                            g_state.set_rangesel(0 as libc::c_int as uint_t);
                        }
                        let ref mut fresh32 = (*pdents.offset(cur as isize))
                            .c2rust_unnamed;
                        (*fresh32)
                            .set_flags(
                                (*fresh32).flags() ^ 0x10 as libc::c_int as ullong_t,
                            );
                        if ((*pdents.offset(cur as isize)).c2rust_unnamed).flags()
                            as libc::c_int & 0x10 as libc::c_int != 0
                        {
                            nselected += 1;
                            nselected;
                            appendfpath(
                                newpath.as_mut_ptr(),
                                mkpath(
                                    path,
                                    (*pdents.offset(cur as isize)).name,
                                    newpath.as_mut_ptr(),
                                ),
                            );
                            writesel(
                                pselbuf,
                                selbufpos.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    as size_t,
                            );
                        } else {
                            nselected -= 1;
                            nselected;
                            rmfromselbuf(
                                mkpath(
                                    path,
                                    (*pdents.offset(cur as isize)).name,
                                    g_sel.as_mut_ptr(),
                                ),
                            );
                        }
                        if cfg.x11() != 0 {
                            plugscript(
                                utils[16 as libc::c_int as usize],
                                (0x2 as libc::c_int | 0x4 as libc::c_int) as uchar_t,
                            );
                        }
                        if rightclicksel {
                            rightclicksel = 0 as libc::c_int != 0;
                        } else if g_state.stayonsel() == 0
                            && cur != ndents - 1 as libc::c_int
                        {
                            move_cursor(
                                (cur + 1 as libc::c_int) % ndents,
                                0 as libc::c_int,
                            );
                        }
                        continue 's_254;
                    }
                    38 => {
                        if ndents == 0 {
                            continue;
                        }
                        startselection();
                        g_state
                            .set_rangesel(
                                g_state.rangesel() ^ 1 as libc::c_int as uint_t,
                            );
                        if stat(path, &mut sb) == -(1 as libc::c_int) {
                            printwait(strerror(*__errno_location()), &mut presel);
                            continue;
                        } else if g_state.rangesel() != 0 {
                            inode = sb.st_ino;
                            selstartid = cur;
                            continue 's_254;
                        } else if inode != sb.st_ino {
                            printwait(messages[42 as libc::c_int as usize], &mut presel);
                            continue;
                        } else {
                            if cur < selstartid {
                                selendid = selstartid;
                                selstartid = cur;
                            } else {
                                selendid = cur;
                            }
                            if selstartid == selendid {
                                resetselind();
                                clearselection();
                                continue 's_254;
                            }
                        }
                        current_block = 17169393835629258797;
                    }
                    39 | 40 => {
                        current_block = 17169393835629258797;
                    }
                    41 => {
                        r = editselection();
                        if r <= 0 as libc::c_int {
                            r = if r == 0 { 3 as libc::c_int } else { 5 as libc::c_int };
                            printwait(messages[r as usize], &mut presel);
                        } else {
                            if cfg.x11() != 0 {
                                plugscript(
                                    utils[16 as libc::c_int as usize],
                                    (0x2 as libc::c_int | 0x4 as libc::c_int) as uchar_t,
                                );
                            }
                            if cfg.filtermode() as libc::c_int != 0 {
                                presel = '/' as i32;
                            } else {
                                statusbar(path);
                            };
                        }
                        continue;
                    }
                    42 => {
                        current_block = 11213074220126547357;
                    }
                    43 => {
                        current_block = 11213074220126547357;
                    }
                    44 | 45 => {
                        current_block = 8940662058537996670;
                    }
                    34 => {
                        current_block = 2033358977384225707;
                    }
                    46 => {
                        current_block = 2033358977384225707;
                    }
                    47 | 48 => {
                        current_block = 454873545234741267;
                    }
                    54 => {
                        if !xdiraccess(plgpath) {
                            printwait(strerror(*__errno_location()), &mut presel);
                            continue;
                        } else {
                            if pkey == 0 {
                                r = xstrsncpy(
                                    g_buf.as_mut_ptr(),
                                    messages[34 as libc::c_int as usize],
                                    (4096 as libc::c_int
                                        + ((255 as libc::c_int + 1 as libc::c_int)
                                            << 1 as libc::c_int)) as size_t,
                                ) as libc::c_int;
                                printkeys(
                                    plug,
                                    g_buf
                                        .as_mut_ptr()
                                        .offset(r as isize)
                                        .offset(-(1 as libc::c_int as isize)),
                                    maxplug,
                                );
                                printmsg(g_buf.as_mut_ptr());
                                r = get_input(0 as *const libc::c_char);
                            } else {
                                r = pkey;
                                pkey = '\0' as i32;
                            }
                            if r != '\r' as i32 {
                                endselection(0 as libc::c_int != 0);
                                tmp = get_kv_val(
                                    plug,
                                    0 as *mut libc::c_char,
                                    r,
                                    maxplug,
                                    2 as libc::c_int as uchar_t,
                                );
                                if tmp.is_null() {
                                    printwait(
                                        messages[40 as libc::c_int as usize],
                                        &mut presel,
                                    );
                                    continue;
                                } else {
                                    if *tmp.offset(0 as libc::c_int as isize) as libc::c_int
                                        == '-' as i32
                                        && *tmp.offset(1 as libc::c_int as isize) as libc::c_int
                                            != 0
                                    {
                                        tmp = tmp.offset(1);
                                        tmp;
                                        r = 0 as libc::c_int;
                                    } else {
                                        r = 1 as libc::c_int;
                                    }
                                    if !run_plugin(
                                        &mut path,
                                        tmp,
                                        if ndents != 0 {
                                            (*pdents.offset(cur as isize)).name
                                        } else {
                                            0 as *mut libc::c_char
                                        },
                                        &mut lastname,
                                        &mut lastdir,
                                    ) {
                                        printwait(messages[5 as libc::c_int as usize], &mut presel);
                                        continue;
                                    } else {
                                        if g_state.picked() != 0 {
                                            return 0 as libc::c_int != 0;
                                        }
                                        xstrsncpy(
                                            lastname,
                                            if ndents != 0 {
                                                (*pdents.offset(cur as isize)).name as *const libc::c_char
                                            } else {
                                                b"\0\0" as *const u8 as *const libc::c_char
                                            },
                                            (255 as libc::c_int + 1 as libc::c_int) as size_t,
                                        );
                                        if !(r == 0) {
                                            current_block = 11850376600911223183;
                                            break 's_254;
                                        }
                                        if cfg.filtermode() as libc::c_int != 0 {
                                            presel = '/' as i32;
                                        } else {
                                            statusbar(path);
                                        };
                                        continue;
                                    }
                                }
                            } else {
                                g_state
                                    .set_runplugin(
                                        g_state.runplugin() ^ 1 as libc::c_int as uint_t,
                                    );
                                if g_state.runplugin() == 0 {
                                    current_block = 6626987748108865425;
                                    break 's_254;
                                } else {
                                    current_block = 5946490239152004759;
                                    break 's_254;
                                }
                            }
                        }
                    }
                    55 => {
                        current_block = 11899822190029240800;
                    }
                    56 | 57 => {
                        current_block = 11899822190029240800;
                    }
                    50 => {
                        presel = 0 as libc::c_int;
                        if !unmount(
                            if ndents != 0 {
                                (*pdents.offset(cur as isize)).name
                            } else {
                                0 as *mut libc::c_char
                            },
                            newpath.as_mut_ptr(),
                            &mut presel,
                            path,
                        ) {
                            if presel == 0 as libc::c_int {
                                statusbar(path);
                            }
                            continue;
                        } else {
                            copynextname(lastname);
                            cd = 0 as libc::c_int != 0;
                            continue '_begin;
                        }
                    }
                    59 => {
                        r = get_input(messages[14 as libc::c_int as usize]);
                        if r == 's' as i32 {
                            tmp = xreadline(
                                0 as *const libc::c_char,
                                messages[6 as libc::c_int as usize],
                            );
                            if !tmp.is_null() && *tmp as libc::c_int != 0 {
                                save_session(tmp, &mut presel);
                            }
                        } else if r == 'l' as i32 || r == 'r' as i32 {
                            if load_session(
                                0 as *const libc::c_char,
                                &mut path,
                                &mut lastdir,
                                &mut lastname,
                                r == 'r' as i32,
                            ) {
                                if cfg.filtermode() as libc::c_int != 0 {
                                    presel = '/' as i32;
                                } else {
                                    watch = 1 as libc::c_int != 0;
                                };
                                continue '_begin;
                            }
                        }
                        statusbar(path);
                        continue;
                    }
                    60 => {
                        export_file_list();
                        if cfg.filtermode() as libc::c_int != 0 {
                            presel = '/' as i32;
                        } else {
                            statusbar(path);
                        };
                        continue;
                    }
                    61 => {
                        if !set_time_type(&mut presel) {
                            continue;
                        }
                        cd = 0 as libc::c_int != 0;
                        continue '_begin;
                    }
                    62 => {
                        current_block = 15373361693358600705;
                        break;
                    }
                    63 | 64 | 65 => {
                        current_block = 15373361693358600705;
                        break;
                    }
                    _ => {
                        if xlines as libc::c_int != LINES || xcols as libc::c_int != COLS
                        {
                            continue 's_254;
                        }
                        if idletimeout != 0 && idle as libc::c_uint == idletimeout {
                            lock_terminal();
                            idle = 0 as libc::c_int as ushort_t;
                        }
                        xstrsncpy(
                            lastname,
                            if ndents != 0 {
                                (*pdents.offset(cur as isize)).name as *const libc::c_char
                            } else {
                                b"\0\0" as *const u8 as *const libc::c_char
                            },
                            (255 as libc::c_int + 1 as libc::c_int) as size_t,
                        );
                        continue;
                    }
                }
                match current_block {
                    17169393835629258797 => {
                        if !(sel as libc::c_uint
                            == SEL_SELALL as libc::c_int as libc::c_uint
                            || sel as libc::c_uint
                                == SEL_SELINV as libc::c_int as libc::c_uint)
                        {
                            current_block = 3635416117088024650;
                            break;
                        }
                        if ndents == 0 {
                            continue;
                        }
                        startselection();
                        if g_state.rangesel() != 0 {
                            g_state.set_rangesel(0 as libc::c_int as uint_t);
                        }
                        selstartid = 0 as libc::c_int;
                        selendid = ndents - 1 as libc::c_int;
                        current_block = 3635416117088024650;
                        break;
                    }
                    8492650606187108553 => {
                        if sel as libc::c_uint == SEL_BACK as libc::c_int as libc::c_uint
                        {
                            dir = visit_parent(path, newpath.as_mut_ptr(), &mut presel);
                            if dir.is_null() {
                                continue;
                            }
                            xstrsncpy(
                                lastname,
                                xbasename(path),
                                (255 as libc::c_int + 1 as libc::c_int) as size_t,
                            );
                            if cdprep(lastdir, 0 as *mut libc::c_char, path, dir)
                                as libc::c_int != 0
                            {
                                presel = '/' as i32;
                            } else {
                                watch = 1 as libc::c_int != 0;
                            };
                            continue '_begin;
                        } else if event.bstate as libc::c_long
                            == (0o2 as libc::c_long)
                                << (2 as libc::c_int - 1 as libc::c_int) * 5 as libc::c_int
                        {
                            presel = middle_click_key;
                            continue;
                        } else if event.bstate as libc::c_long
                            == (0o2 as libc::c_long)
                                << (4 as libc::c_int - 1 as libc::c_int) * 5 as libc::c_int
                            && ndents != 0
                            && (cfg.rollover() as libc::c_int != 0 || cur != 0)
                        {
                            move_cursor(
                                if cfg.rollover() == 0 && cur < scroll_lines {
                                    0 as libc::c_int
                                } else {
                                    (cur + ndents - scroll_lines) % ndents
                                },
                                0 as libc::c_int,
                            );
                            continue 's_254;
                        } else if event.bstate as libc::c_long
                            == (0o2 as libc::c_long)
                                << (5 as libc::c_int - 1 as libc::c_int) * 5 as libc::c_int
                            && ndents != 0
                            && (cfg.rollover() as libc::c_int != 0
                                || cur != ndents - 1 as libc::c_int)
                        {
                            move_cursor(
                                if cfg.rollover() == 0 && cur >= ndents - scroll_lines {
                                    ndents - 1 as libc::c_int
                                } else {
                                    (cur + scroll_lines) % ndents
                                },
                                0 as libc::c_int,
                            );
                            continue 's_254;
                        } else if event.y >= xlines as libc::c_int - 2 as libc::c_int
                            && event.bstate as libc::c_long
                                == (0o2 as libc::c_long)
                                    << (1 as libc::c_int - 1 as libc::c_int) * 5 as libc::c_int
                        {
                            clearfilter();
                            cfg.set_filtermode(
                                cfg.filtermode() ^ 1 as libc::c_int as uint_t,
                            );
                            if cfg.filtermode() != 0 {
                                presel = '/' as i32;
                                continue;
                            } else {
                                watch = 1 as libc::c_int != 0;
                                xstrsncpy(
                                    lastname,
                                    if ndents != 0 {
                                        (*pdents.offset(cur as isize)).name as *const libc::c_char
                                    } else {
                                        b"\0\0" as *const u8 as *const libc::c_char
                                    },
                                    (255 as libc::c_int + 1 as libc::c_int) as size_t,
                                );
                                cd = 0 as libc::c_int != 0;
                                continue '_begin;
                            }
                        } else if event.y >= 2 as libc::c_int
                            && event.y <= ndents + 1 as libc::c_int
                            && (event.bstate as libc::c_long
                                == (0o2 as libc::c_long)
                                    << (1 as libc::c_int - 1 as libc::c_int) * 5 as libc::c_int
                                || event.bstate as libc::c_long
                                    == (0o2 as libc::c_long)
                                        << (3 as libc::c_int - 1 as libc::c_int) * 5 as libc::c_int)
                        {
                            r = curscroll + (event.y - 2 as libc::c_int);
                            if r != cur {
                                move_cursor(r, 1 as libc::c_int);
                            } else if event.bstate as libc::c_long
                                == (0o2 as libc::c_long)
                                    << (1 as libc::c_int - 1 as libc::c_int) * 5 as libc::c_int
                                && g_state.fifomode() == 0
                            {
                                notify_fifo(1 as libc::c_int != 0);
                            }
                            if event.bstate as libc::c_long
                                == (0o2 as libc::c_long)
                                    << (3 as libc::c_int - 1 as libc::c_int) * 5 as libc::c_int
                            {
                                rightclicksel = 1 as libc::c_int != 0;
                                presel = ' ' as i32;
                                continue;
                            } else {
                                currentmouse = (currentmouse as libc::c_int
                                    ^ 1 as libc::c_int) != 0;
                                clock_gettime(
                                    4 as libc::c_int,
                                    &mut *mousetimings
                                        .as_mut_ptr()
                                        .offset(currentmouse as isize),
                                );
                                mousedent[currentmouse as usize] = cur;
                                if mousedent[0 as libc::c_int as usize]
                                    != mousedent[1 as libc::c_int as usize]
                                    || ((if mousetimings[0 as libc::c_int as usize].tv_sec
                                        <= mousetimings[1 as libc::c_int as usize].tv_sec
                                    {
                                        mousetimings[1 as libc::c_int as usize].tv_sec
                                            - mousetimings[0 as libc::c_int as usize].tv_sec
                                    } else {
                                        mousetimings[0 as libc::c_int as usize].tv_sec
                                            - mousetimings[1 as libc::c_int as usize].tv_sec
                                    }) << 30 as libc::c_int)
                                        + (if mousetimings[0 as libc::c_int as usize].tv_nsec
                                            <= mousetimings[1 as libc::c_int as usize].tv_nsec
                                        {
                                            mousetimings[1 as libc::c_int as usize].tv_nsec
                                                - mousetimings[0 as libc::c_int as usize].tv_nsec
                                        } else {
                                            mousetimings[0 as libc::c_int as usize].tv_nsec
                                                - mousetimings[1 as libc::c_int as usize].tv_nsec
                                        }) > 400000000 as libc::c_int as libc::c_long
                                {
                                    continue 's_254;
                                }
                                mousetimings[currentmouse as usize]
                                    .tv_sec = 0 as libc::c_int as __time_t;
                                mousedent[currentmouse as usize] = -(1 as libc::c_int);
                                sel = SEL_OPEN;
                            }
                        } else {
                            if cfg.filtermode() as libc::c_int != 0
                                || g_ctx[cfg.curctx() as usize]
                                    .c_fltr[1 as libc::c_int as usize] as libc::c_int != 0
                            {
                                presel = '/' as i32;
                            }
                            xstrsncpy(
                                lastname,
                                if ndents != 0 {
                                    (*pdents.offset(cur as isize)).name as *const libc::c_char
                                } else {
                                    b"\0\0" as *const u8 as *const libc::c_char
                                },
                                (255 as libc::c_int + 1 as libc::c_int) as size_t,
                            );
                            continue;
                        }
                        current_block = 15252062886580107196;
                    }
                    11899822190029240800 => {
                        r = handle_cmd(sel, newpath.as_mut_ptr()) as libc::c_int;
                        if cfg.filtermode() != 0 {
                            presel = '/' as i32;
                        }
                        xstrsncpy(
                            lastname,
                            if ndents != 0 {
                                (*pdents.offset(cur as isize)).name as *const libc::c_char
                            } else {
                                b"\0\0" as *const u8 as *const libc::c_char
                            },
                            (255 as libc::c_int + 1 as libc::c_int) as size_t,
                        );
                        if r == 0 {
                            continue;
                        }
                        cd = 0 as libc::c_int != 0;
                        continue '_begin;
                    }
                    16949939609931944443 => {
                        current_block = 942779372144332707;
                    }
                    4162367029103048979 => {
                        current_block = 7126770851743004489;
                    }
                    4041062724260856750 => {
                        current_block = 6070906628441949502;
                    }
                    11213074220126547357 => {
                        current_block = 8940662058537996670;
                    }
                    2033358977384225707 => {
                        current_block = 454873545234741267;
                    }
                    _ => {}
                }
                match current_block {
                    454873545234741267 => {
                        fd = 0;
                        ret = 'n' as i32;
                        if ndents == 0
                            && (sel as libc::c_uint
                                == SEL_OPENWITH as libc::c_int as libc::c_uint
                                || sel as libc::c_uint
                                    == SEL_RENAME as libc::c_int as libc::c_uint)
                        {
                            continue 's_254;
                        }
                        if sel as libc::c_uint
                            != SEL_OPENWITH as libc::c_int as libc::c_uint
                        {
                            endselection(1 as libc::c_int != 0);
                        }
                        match sel as libc::c_uint {
                            34 => {
                                r = get_cur_or_sel();
                                if r == 0 {
                                    statusbar(path);
                                    continue;
                                } else {
                                    if r == 's' as i32 {
                                        if !selsafe() {
                                            presel = '$' as i32;
                                            continue;
                                        } else {
                                            tmp = 0 as *mut libc::c_char;
                                        }
                                    } else {
                                        tmp = (*pdents.offset(cur as isize)).name;
                                    }
                                    tmp = xreadline(tmp, messages[17 as libc::c_int as usize]);
                                }
                            }
                            46 => {
                                if g_state.picker() != 0 {
                                    tmp = xreadline(
                                        0 as *const libc::c_char,
                                        messages[18 as libc::c_int as usize],
                                    );
                                } else {
                                    tmp = getreadline(messages[18 as libc::c_int as usize]);
                                }
                            }
                            47 => {
                                r = get_input(messages[11 as libc::c_int as usize]);
                                if r == 'f' as i32 || r == 'd' as i32 {
                                    tmp = xreadline(
                                        0 as *const libc::c_char,
                                        messages[19 as libc::c_int as usize],
                                    );
                                } else if r == 's' as i32 || r == 'h' as i32 {
                                    tmp = xreadline(
                                        0 as *const libc::c_char,
                                        messages[(if nselected <= 1 as libc::c_int {
                                            19 as libc::c_int
                                        } else {
                                            20 as libc::c_int
                                        }) as usize],
                                    );
                                } else {
                                    tmp = 0 as *mut libc::c_char;
                                }
                            }
                            _ => {
                                tmp = xreadline(
                                    (*pdents.offset(cur as isize)).name,
                                    b"\0" as *const u8 as *const libc::c_char,
                                );
                            }
                        }
                        if tmp.is_null() || *tmp == 0 {
                            continue 's_254;
                        }
                        match sel as libc::c_uint {
                            34 => {
                                if r == 'c' as i32
                                    && strcmp(tmp, (*pdents.offset(cur as isize)).name)
                                        == 0 as libc::c_int
                                {
                                    continue;
                                }
                                mkpath(path, tmp, newpath.as_mut_ptr());
                                if !(access(newpath.as_mut_ptr(), 0 as libc::c_int)
                                    == 0 as libc::c_int)
                                {
                                    current_block = 2727668027383962707;
                                    break;
                                }
                                if xconfirm(
                                    get_input(messages[13 as libc::c_int as usize]),
                                ) {
                                    current_block = 2727668027383962707;
                                    break;
                                }
                                statusbar(path);
                                continue;
                            }
                            46 => {
                                handle_openwith(
                                    path,
                                    (*pdents.offset(cur as isize)).name,
                                    newpath.as_mut_ptr(),
                                    tmp,
                                );
                                if cfg.filtermode() as libc::c_int != 0 {
                                    presel = '/' as i32;
                                } else {
                                    statusbar(path);
                                };
                                xstrsncpy(
                                    lastname,
                                    if ndents != 0 {
                                        (*pdents.offset(cur as isize)).name as *const libc::c_char
                                    } else {
                                        b"\0\0" as *const u8 as *const libc::c_char
                                    },
                                    (255 as libc::c_int + 1 as libc::c_int) as size_t,
                                );
                                continue;
                            }
                            48 => {
                                if strcmp(tmp, (*pdents.offset(cur as isize)).name)
                                    == 0 as libc::c_int
                                {
                                    tmp = xreadline(
                                        (*pdents.offset(cur as isize)).name,
                                        messages[21 as libc::c_int as usize],
                                    );
                                    if tmp.is_null()
                                        || *tmp.offset(0 as libc::c_int as isize) == 0
                                        || strcmp(tmp, (*pdents.offset(cur as isize)).name) == 0
                                    {
                                        if cfg.filtermode() as libc::c_int != 0 {
                                            presel = '/' as i32;
                                        } else {
                                            statusbar(path);
                                        };
                                        xstrsncpy(
                                            lastname,
                                            if ndents != 0 {
                                                (*pdents.offset(cur as isize)).name as *const libc::c_char
                                            } else {
                                                b"\0\0" as *const u8 as *const libc::c_char
                                            },
                                            (255 as libc::c_int + 1 as libc::c_int) as size_t,
                                        );
                                        continue;
                                    } else {
                                        ret = 'd' as i32;
                                    }
                                }
                            }
                            _ => {}
                        }
                        fd = open(path, 0 as libc::c_int | 0o200000 as libc::c_int);
                        if fd == -(1 as libc::c_int) {
                            printwait(strerror(*__errno_location()), &mut presel);
                            continue;
                        } else {
                            if fstatat(fd, tmp, &mut sb, 0x100 as libc::c_int)
                                == 0 as libc::c_int
                            {
                                if sel as libc::c_uint
                                    == SEL_RENAME as libc::c_int as libc::c_uint
                                {
                                    if !xconfirm(
                                        get_input(messages[13 as libc::c_int as usize]),
                                    ) {
                                        close(fd);
                                        continue 's_254;
                                    }
                                } else {
                                    close(fd);
                                    printwait(
                                        messages[28 as libc::c_int as usize],
                                        &mut presel,
                                    );
                                    continue;
                                }
                            }
                            if sel as libc::c_uint
                                == SEL_RENAME as libc::c_int as libc::c_uint
                            {
                                if ret == 'd' as i32 {
                                    spawn(
                                        b"cp -rp\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        (*pdents.offset(cur as isize)).name,
                                        tmp,
                                        0 as *mut libc::c_char,
                                        (0x8 as libc::c_int | 0x1 as libc::c_int
                                            | 0x4 as libc::c_int) as ushort_t,
                                    );
                                    current_block = 8470013831079077146;
                                    break 's_254;
                                } else {
                                    if !(renameat(
                                        fd,
                                        (*pdents.offset(cur as isize)).name,
                                        fd,
                                        tmp,
                                    ) != 0 as libc::c_int)
                                    {
                                        current_block = 8470013831079077146;
                                        break 's_254;
                                    }
                                    close(fd);
                                    printwait(strerror(*__errno_location()), &mut presel);
                                    continue;
                                }
                            } else {
                                close(fd);
                                presel = 0 as libc::c_int;
                                if r == 'f' as i32 || r == 'd' as i32 {
                                    mkpath(path, tmp, newpath.as_mut_ptr());
                                    ret = xmktree(
                                        newpath.as_mut_ptr(),
                                        if r == 'f' as i32 {
                                            0 as libc::c_int
                                        } else {
                                            1 as libc::c_int
                                        } != 0,
                                    ) as libc::c_int;
                                } else if r == 's' as i32 || r == 'h' as i32 {
                                    if nselected > 1 as libc::c_int
                                        && *tmp.offset(0 as libc::c_int as isize) as libc::c_int
                                            == '@' as i32
                                        && *tmp.offset(1 as libc::c_int as isize) as libc::c_int
                                            == '\0' as i32
                                    {
                                        *tmp
                                            .offset(
                                                0 as libc::c_int as isize,
                                            ) = '\0' as i32 as libc::c_char;
                                    }
                                    ret = xlink(
                                        tmp,
                                        path,
                                        if ndents != 0 {
                                            (*pdents.offset(cur as isize)).name
                                        } else {
                                            0 as *mut libc::c_char
                                        },
                                        newpath.as_mut_ptr(),
                                        &mut presel,
                                        r,
                                    );
                                }
                                if ret == 0 {
                                    printwait(messages[5 as libc::c_int as usize], &mut presel);
                                }
                                if ret <= 0 as libc::c_int {
                                    continue;
                                }
                                if r == 'f' as i32 || r == 'd' as i32 {
                                    xstrsncpy(
                                        lastname,
                                        tmp,
                                        (255 as libc::c_int + 1 as libc::c_int) as size_t,
                                    );
                                } else if ndents != 0 {
                                    if cfg.filtermode() != 0 {
                                        presel = '/' as i32;
                                    }
                                    xstrsncpy(
                                        lastname,
                                        if ndents != 0 {
                                            (*pdents.offset(cur as isize)).name as *const libc::c_char
                                        } else {
                                            b"\0\0" as *const u8 as *const libc::c_char
                                        },
                                        (255 as libc::c_int + 1 as libc::c_int) as size_t,
                                    );
                                }
                                clearfilter();
                                current_block = 10714207895679337196;
                                break 's_254;
                            }
                        }
                    }
                    8940662058537996670 => {
                        if sel as libc::c_uint == SEL_RM as libc::c_int as libc::c_uint {
                            r = get_cur_or_sel();
                            if r == 0 {
                                statusbar(path);
                                continue;
                            } else if r == 'c' as i32 {
                                tmp = if !listpath.is_null()
                                    && (if *path as libc::c_int != *listpath as libc::c_int {
                                        -(1 as libc::c_int)
                                    } else {
                                        strcmp(path, listpath)
                                    }) == 0 as libc::c_int
                                {
                                    listroot
                                } else {
                                    path
                                };
                                mkpath(
                                    tmp,
                                    (*pdents.offset(cur as isize)).name,
                                    newpath.as_mut_ptr(),
                                );
                                if !xrm(newpath.as_mut_ptr()) {
                                    continue 's_254;
                                }
                                xrmfromsel(tmp, newpath.as_mut_ptr());
                                copynextname(lastname);
                                if cfg.filtermode() as libc::c_int != 0
                                    || g_ctx[cfg.curctx() as usize]
                                        .c_fltr[1 as libc::c_int as usize] as libc::c_int != 0
                                {
                                    presel = '/' as i32;
                                }
                                cd = 0 as libc::c_int != 0;
                                continue '_begin;
                            }
                        }
                        if nselected == 1 as libc::c_int
                            && (sel as libc::c_uint
                                == SEL_CP as libc::c_int as libc::c_uint
                                || sel as libc::c_uint
                                    == SEL_MV as libc::c_int as libc::c_uint)
                        {
                            mkpath(path, xbasename(pselbuf), newpath.as_mut_ptr());
                        } else {
                            newpath[0 as libc::c_int
                                as usize] = '\0' as i32 as libc::c_char;
                        };
                        endselection(1 as libc::c_int != 0);
                        if !cpmvrm_selection(sel, path) {
                            presel = '$' as i32;
                            continue;
                        } else {
                            if cfg.filtermode() != 0 {
                                presel = '/' as i32;
                            }
                            clearfilter();
                            if cfg.x11() != 0 {
                                plugscript(
                                    utils[15 as libc::c_int as usize],
                                    (0x2 as libc::c_int | 0x4 as libc::c_int) as uchar_t,
                                );
                            }
                            if newpath[0 as libc::c_int as usize] as libc::c_int != 0
                                && access(newpath.as_mut_ptr(), 0 as libc::c_int) == 0
                            {
                                xstrsncpy(
                                    lastname,
                                    xbasename(newpath.as_mut_ptr()),
                                    (255 as libc::c_int + 1 as libc::c_int) as size_t,
                                );
                            } else {
                                xstrsncpy(
                                    lastname,
                                    if ndents != 0 {
                                        (*pdents.offset(cur as isize)).name as *const libc::c_char
                                    } else {
                                        b"\0\0" as *const u8 as *const libc::c_char
                                    },
                                    (255 as libc::c_int + 1 as libc::c_int) as size_t,
                                );
                            }
                            cd = 0 as libc::c_int != 0;
                            continue '_begin;
                        }
                    }
                    7126770851743004489 => {
                        match sel as libc::c_uint {
                            29 => {
                                cfg.set_filtermode(
                                    cfg.filtermode() ^ 1 as libc::c_int as uint_t,
                                );
                                if cfg.filtermode() != 0 {
                                    presel = '/' as i32;
                                    clearfilter();
                                    continue;
                                } else {
                                    watch = 1 as libc::c_int != 0;
                                    current_block = 1049211764165170956;
                                    break 's_254;
                                }
                            }
                            30 => {
                                current_block = 1049211764165170956;
                                break 's_254;
                            }
                            31 => {
                                cfg.set_showdetail(
                                    cfg.showdetail() ^ 1 as libc::c_int as uint_t,
                                );
                                cfg.set_blkorder(0 as libc::c_int as uint_t);
                                continue 's_254;
                            }
                            _ => {
                                r = set_sort_flags(
                                    get_input(messages[36 as libc::c_int as usize]),
                                );
                                if r == 0 {
                                    printwait(
                                        messages[40 as libc::c_int as usize],
                                        &mut presel,
                                    );
                                    continue;
                                } else {
                                    if cfg.filtermode() as libc::c_int != 0
                                        || g_ctx[cfg.curctx() as usize]
                                            .c_fltr[1 as libc::c_int as usize] as libc::c_int != 0
                                    {
                                        presel = '/' as i32;
                                    }
                                    if !(ndents != 0) {
                                        continue 's_254;
                                    }
                                    xstrsncpy(
                                        lastname,
                                        if ndents != 0 {
                                            (*pdents.offset(cur as isize)).name as *const libc::c_char
                                        } else {
                                            b"\0\0" as *const u8 as *const libc::c_char
                                        },
                                        (255 as libc::c_int + 1 as libc::c_int) as size_t,
                                    );
                                    if r == 'd' as i32 || r == 'a' as i32 {
                                        current_block = 14908777651318078790;
                                        break;
                                    } else {
                                        current_block = 11573789974424595537;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    942779372144332707 => {
                        dir = (if sel as libc::c_uint
                            == SEL_CDHOME as libc::c_int as libc::c_uint
                        {
                            home as *const libc::c_char
                        } else if sel as libc::c_uint
                            == SEL_CDBEGIN as libc::c_int as libc::c_uint
                        {
                            ipath as *const libc::c_char
                        } else if sel as libc::c_uint
                            == SEL_CDLAST as libc::c_int as libc::c_uint
                        {
                            lastdir as *const libc::c_char
                        } else {
                            b"/\0" as *const u8 as *const libc::c_char
                        }) as *mut libc::c_char;
                        if dir.is_null() || *dir == 0 {
                            printwait(messages[27 as libc::c_int as usize], &mut presel);
                            continue;
                        } else {
                            g_state.set_selbm(0 as libc::c_int as uint_t);
                            if strcmp(path, dir) == 0 as libc::c_int {
                                if dir == ipath {
                                    if cfg.filtermode() != 0 {
                                        presel = '/' as i32;
                                    }
                                    continue;
                                } else {
                                    dir = lastdir;
                                }
                            }
                            if chdir(dir) == -(1 as libc::c_int) {
                                presel = '$' as i32;
                                continue;
                            } else {
                                xstrsncpy(
                                    newpath.as_mut_ptr(),
                                    dir,
                                    4096 as libc::c_int as size_t,
                                );
                            }
                        }
                        current_block = 12333446989980225856;
                    }
                    15252062886580107196 => {
                        current_block = 5766893457425082621;
                    }
                    6070906628441949502 => {
                        current_block = 6885328239442086976;
                    }
                    _ => {}
                }
                match current_block {
                    12333446989980225856 => {
                        if !(sel as libc::c_uint
                            == SEL_BMOPEN as libc::c_int as libc::c_uint)
                        {
                            current_block = 4491581808830814586;
                            break 's_254;
                        }
                        r = handle_bookmark(mark, newpath.as_mut_ptr()) as libc::c_int;
                        if r != 0 {
                            printwait(messages[r as usize], &mut presel);
                            continue;
                        } else {
                            if g_state.selbm() as libc::c_int == 1 as libc::c_int {
                                presel = '/' as i32;
                            }
                            if strcmp(path, newpath.as_mut_ptr()) == 0 as libc::c_int {
                                continue 's_254;
                            } else {
                                current_block = 4491581808830814586;
                                break 's_254;
                            }
                        }
                    }
                    5766893457425082621 => {
                        if ndents == 0 {
                            cd = 0 as libc::c_int != 0;
                            g_state.set_runplugin(0 as libc::c_int as uint_t);
                            g_state.set_selbm(g_state.runplugin());
                            continue '_begin;
                        } else {
                            pent = &mut *pdents.offset(cur as isize) as *mut entry;
                            if g_state.selbm() == 0
                                || !((*pent).mode & 0o170000 as libc::c_int as libc::c_uint
                                    == 0o120000 as libc::c_int as libc::c_uint
                                    && !(realpath((*pent).name, newpath.as_mut_ptr())).is_null()
                                    && xstrsncpy(path, lastdir, 4096 as libc::c_int as size_t)
                                        != 0)
                            {
                                mkpath(path, (*pent).name, newpath.as_mut_ptr());
                            }
                            g_state.set_selbm(0 as libc::c_int as uint_t);
                            if ((*pent).c2rust_unnamed).flags() as libc::c_int
                                & 0x1 as libc::c_int != 0
                            {
                                if chdir(newpath.as_mut_ptr()) == -(1 as libc::c_int) {
                                    printwait(strerror(*__errno_location()), &mut presel);
                                    continue;
                                } else {
                                    if cdprep(lastdir, lastname, path, newpath.as_mut_ptr())
                                        as libc::c_int != 0
                                    {
                                        presel = '/' as i32;
                                    } else {
                                        watch = 1 as libc::c_int != 0;
                                    };
                                    continue '_begin;
                                }
                            } else if stat(newpath.as_mut_ptr(), &mut sb)
                                == -(1 as libc::c_int)
                            {
                                printwait(strerror(*__errno_location()), &mut presel);
                                continue;
                            } else if !(sb.st_mode
                                & 0o170000 as libc::c_int as libc::c_uint
                                == 0o100000 as libc::c_int as libc::c_uint)
                            {
                                printwait(
                                    messages[26 as libc::c_int as usize],
                                    &mut presel,
                                );
                                continue;
                            } else {
                                if g_state.runplugin() != 0 {
                                    g_state.set_runplugin(0 as libc::c_int as uint_t);
                                    if g_state.runctx() as libc::c_int
                                        == cfg.curctx() as libc::c_int && strcmp(path, plgpath) == 0
                                    {
                                        endselection(0 as libc::c_int != 0);
                                        xstrsncpy(path, lastdir, 4096 as libc::c_int as size_t);
                                        clearfilter();
                                        chdir(path) == -(1 as libc::c_int)
                                            || !run_plugin(
                                                &mut path,
                                                (*pent).name,
                                                runfile.as_mut_ptr(),
                                                &mut lastname,
                                                &mut lastdir,
                                            );
                                        if g_state.picked() != 0 {
                                            return 0 as libc::c_int != 0;
                                        }
                                        if runfile[0 as libc::c_int as usize] != 0 {
                                            xstrsncpy(
                                                lastname,
                                                runfile.as_mut_ptr(),
                                                (255 as libc::c_int + 1 as libc::c_int) as size_t,
                                            );
                                            runfile[0 as libc::c_int
                                                as usize] = '\0' as i32 as libc::c_char;
                                        }
                                        if cfg.filtermode() as libc::c_int != 0 {
                                            presel = '/' as i32;
                                        } else {
                                            watch = 1 as libc::c_int != 0;
                                        };
                                        continue '_begin;
                                    }
                                }
                                if g_state.fifomode() as libc::c_int != 0
                                    && sel as libc::c_uint
                                        == SEL_OPEN as libc::c_int as libc::c_uint
                                {
                                    send_to_explorer(&mut presel);
                                    continue 's_254;
                                } else {
                                    if g_state.picker() as libc::c_int != 0
                                        && sel as libc::c_uint
                                            == SEL_OPEN as libc::c_int as libc::c_uint
                                    {
                                        if nselected == 0 as libc::c_int {
                                            appendfpath(
                                                newpath.as_mut_ptr(),
                                                mkpath(path, (*pent).name, newpath.as_mut_ptr()),
                                            );
                                        }
                                        return 0 as libc::c_int != 0;
                                    }
                                    if sel as libc::c_uint
                                        == SEL_NAV_IN as libc::c_int as libc::c_uint
                                    {
                                        if !listpath.is_null()
                                            && (*pent).mode & 0o170000 as libc::c_int as libc::c_uint
                                                == 0o120000 as libc::c_int as libc::c_uint
                                            && is_prefix(path, listpath, xstrlen(listpath))
                                                as libc::c_int != 0
                                        {
                                            if (realpath((*pent).name, newpath.as_mut_ptr())).is_null()
                                            {
                                                printwait(strerror(*__errno_location()), &mut presel);
                                                continue;
                                            } else {
                                                xdirname(newpath.as_mut_ptr());
                                                if chdir(newpath.as_mut_ptr()) == -(1 as libc::c_int) {
                                                    printwait(strerror(*__errno_location()), &mut presel);
                                                    continue;
                                                } else {
                                                    if cdprep(
                                                        lastdir,
                                                        0 as *mut libc::c_char,
                                                        path,
                                                        newpath.as_mut_ptr(),
                                                    ) as libc::c_int != 0
                                                    {
                                                        presel = '/' as i32;
                                                    } else {
                                                        watch = 1 as libc::c_int != 0;
                                                    };
                                                    xstrsncpy(
                                                        lastname,
                                                        (*pent).name,
                                                        (255 as libc::c_int + 1 as libc::c_int) as size_t,
                                                    );
                                                    continue '_begin;
                                                }
                                            }
                                        } else if cfg.nonavopen() != 0 {
                                            continue;
                                        }
                                    }
                                    if sb.st_size == 0 {
                                        printwait(
                                            messages[25 as libc::c_int as usize],
                                            &mut presel,
                                        );
                                        continue;
                                    } else if cfg.useeditor() as libc::c_int != 0
                                        && get_output(
                                            b"file\0" as *const u8 as *const libc::c_char
                                                as *mut libc::c_char,
                                            b"-biL\0" as *const u8 as *const libc::c_char
                                                as *mut libc::c_char,
                                            newpath.as_mut_ptr(),
                                            -(1 as libc::c_int),
                                            0 as libc::c_int != 0,
                                            0 as libc::c_int != 0,
                                        ) as libc::c_int != 0
                                        && is_prefix(
                                            g_buf.as_mut_ptr(),
                                            b"text/\0" as *const u8 as *const libc::c_char,
                                            5 as libc::c_int as size_t,
                                        ) as libc::c_int != 0
                                    {
                                        spawn(
                                            editor,
                                            newpath.as_mut_ptr(),
                                            0 as *mut libc::c_char,
                                            0 as *mut libc::c_char,
                                            (0x8 as libc::c_int | 0x1 as libc::c_int) as ushort_t,
                                        );
                                        if cfg.filtermode() != 0 {
                                            presel = '/' as i32;
                                            clearfilter();
                                        }
                                        continue 's_254;
                                    } else {
                                        tmp = xextension(
                                            (*pent).name,
                                            (((*pent).c2rust_unnamed).nlen() as libc::c_int
                                                - 1 as libc::c_int) as size_t,
                                        );
                                        if !(!tmp.is_null()
                                            && regexec(
                                                &mut archive_re,
                                                tmp,
                                                0 as libc::c_int as size_t,
                                                0 as *mut regmatch_t,
                                                0 as libc::c_int,
                                            ) == 0)
                                        {
                                            current_block = 13479157322803929894;
                                            break;
                                        }
                                        r = get_input(messages[33 as libc::c_int as usize]);
                                        if r == 'l' as i32 || r == 'x' as i32 {
                                            mkpath(path, (*pent).name, newpath.as_mut_ptr());
                                            if !handle_archive(
                                                newpath.as_mut_ptr(),
                                                r as libc::c_char,
                                            ) {
                                                presel = '$' as i32;
                                                continue;
                                            } else if r == 'l' as i32 {
                                                statusbar(path);
                                                continue;
                                            }
                                        }
                                        if r == 'm' as i32 && !archive_mount(newpath.as_mut_ptr()) {
                                            presel = '$' as i32;
                                            continue;
                                        } else if r == 'x' as i32 || r == 'm' as i32 {
                                            if newpath[0 as libc::c_int as usize] != 0 {
                                                set_smart_ctx(
                                                    '+' as i32,
                                                    newpath.as_mut_ptr(),
                                                    &mut path,
                                                    if ndents != 0 {
                                                        (*pdents.offset(cur as isize)).name
                                                    } else {
                                                        0 as *mut libc::c_char
                                                    },
                                                    &mut lastname,
                                                    &mut lastdir,
                                                );
                                            } else {
                                                xstrsncpy(
                                                    lastname,
                                                    if ndents != 0 {
                                                        (*pdents.offset(cur as isize)).name as *const libc::c_char
                                                    } else {
                                                        b"\0\0" as *const u8 as *const libc::c_char
                                                    },
                                                    (255 as libc::c_int + 1 as libc::c_int) as size_t,
                                                );
                                            }
                                            clearfilter();
                                            continue '_begin;
                                        } else {
                                            if !(r != 'o' as i32) {
                                                current_block = 13479157322803929894;
                                                break;
                                            }
                                            printwait(
                                                messages[40 as libc::c_int as usize],
                                                &mut presel,
                                            );
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    6885328239442086976 => {}
                    _ => {}
                }
                let mut refresh: bool = 0 as libc::c_int != 0;
                if ndents != 0 {
                    mkpath(
                        path,
                        (*pdents.offset(cur as isize)).name,
                        newpath.as_mut_ptr(),
                    );
                } else if sel as libc::c_uint == SEL_EDIT as libc::c_int as libc::c_uint
                {
                    continue;
                }
                match sel as libc::c_uint {
                    36 => {
                        refresh = 1 as libc::c_int != 0;
                        current_block = 168643628589418436;
                    }
                    49 => {
                        endselection(1 as libc::c_int != 0);
                        setenv(
                            b"NNN_INCLUDE_HIDDEN\0" as *const u8 as *const libc::c_char,
                            xitoa(cfg.showhidden()),
                            1 as libc::c_int,
                        );
                        setenv(
                            b"NNN_LIST\0" as *const u8 as *const libc::c_char,
                            if !listpath.is_null() {
                                listroot as *const libc::c_char
                            } else {
                                b"\0" as *const u8 as *const libc::c_char
                            },
                            1 as libc::c_int,
                        );
                        if !(getutil(utils[8 as libc::c_int as usize]) as libc::c_int
                            != 0
                            && plugscript(
                                utils[17 as libc::c_int as usize],
                                (0x8 as libc::c_int | 0x1 as libc::c_int) as uchar_t,
                            ) as libc::c_int != 0) && !batch_rename()
                        {
                            printwait(messages[5 as libc::c_int as usize], &mut presel);
                            continue;
                        } else {
                            clearselection();
                            refresh = 1 as libc::c_int != 0;
                        }
                        current_block = 168643628589418436;
                    }
                    51 => {
                        show_help(path);
                        current_block = 13171485647892269704;
                    }
                    52 => {
                        current_block = 13171485647892269704;
                    }
                    53 => {
                        if !(g_state.picker() as libc::c_int != 0
                            || g_state.fifomode() as libc::c_int != 0)
                        {
                            spawn(
                                editor,
                                newpath.as_mut_ptr(),
                                0 as *mut libc::c_char,
                                0 as *mut libc::c_char,
                                (0x8 as libc::c_int | 0x1 as libc::c_int) as ushort_t,
                            );
                        }
                        continue 's_254;
                    }
                    _ => {
                        lock_terminal();
                        current_block = 168643628589418436;
                    }
                }
                match current_block {
                    13171485647892269704 => {
                        if sel as libc::c_uint
                            == SEL_AUTONEXT as libc::c_int as libc::c_uint
                        {
                            g_state
                                .set_autonext(
                                    g_state.autonext() ^ 1 as libc::c_int as uint_t,
                                );
                        }
                        if cfg.filtermode() != 0 {
                            presel = '/' as i32;
                        }
                        xstrsncpy(
                            lastname,
                            if ndents != 0 {
                                (*pdents.offset(cur as isize)).name as *const libc::c_char
                            } else {
                                b"\0\0" as *const u8 as *const libc::c_char
                            },
                            (255 as libc::c_int + 1 as libc::c_int) as size_t,
                        );
                    }
                    _ => {
                        if (cfg.filtermode() as libc::c_int != 0
                            || g_ctx[cfg.curctx() as usize]
                                .c_fltr[1 as libc::c_int as usize] as libc::c_int != 0)
                            && !refresh
                        {
                            presel = '/' as i32;
                        } else {
                            xstrsncpy(
                                lastname,
                                if ndents != 0 {
                                    (*pdents.offset(cur as isize)).name as *const libc::c_char
                                } else {
                                    b"\0\0" as *const u8 as *const libc::c_char
                                },
                                (255 as libc::c_int + 1 as libc::c_int) as size_t,
                            );
                            cd = 0 as libc::c_int != 0;
                            continue '_begin;
                        }
                    }
                }
            }
            match current_block {
                15373361693358600705 => {
                    if sel as libc::c_uint == SEL_QUITCTX as libc::c_int as libc::c_uint
                    {
                        let mut ctx: libc::c_int = cfg.curctx() as libc::c_int;
                        r = ctx - 1 as libc::c_int & 4 as libc::c_int - 1 as libc::c_int;
                        while r != ctx && (g_ctx[r as usize].c_cfg).ctxactive() == 0 {
                            r = r - 1 as libc::c_int
                                & 4 as libc::c_int - 1 as libc::c_int;
                        }
                        if !(r != ctx) {
                            break '_begin;
                        }
                        (g_ctx[ctx as usize].c_cfg)
                            .set_ctxactive(0 as libc::c_int as uint_t);
                        path = (g_ctx[r as usize].c_path).as_mut_ptr();
                        lastdir = (g_ctx[r as usize].c_last).as_mut_ptr();
                        lastname = (g_ctx[r as usize].c_name).as_mut_ptr();
                        cfg = g_ctx[r as usize].c_cfg;
                        cfg.set_curctx(r as uint_t);
                        if cfg.filtermode() as libc::c_int != 0 {
                            presel = '/' as i32;
                        } else {
                            watch = 1 as libc::c_int != 0;
                        };
                        continue '_begin;
                    } else {
                        if !(g_state.forcequit() == 0) {
                            break '_begin;
                        }
                        r = 0 as libc::c_int;
                        while r < 4 as libc::c_int {
                            if r != cfg.curctx() as libc::c_int
                                && (g_ctx[r as usize].c_cfg).ctxactive() as libc::c_int != 0
                            {
                                r = get_input(messages[15 as libc::c_int as usize]);
                                break;
                            } else {
                                r += 1;
                                r;
                            }
                        }
                        if !(r == 4 as libc::c_int || xconfirm(r) as libc::c_int != 0) {
                            continue;
                        } else {
                            break '_begin;
                        }
                    }
                }
                3635416117088024650 => {
                    if nselected > 1000 as libc::c_int
                        || nselected != 0 && ndents > 1000 as libc::c_int
                    {
                        printmsg(b"processing...\0" as *const u8 as *const libc::c_char);
                        wrefresh(stdscr);
                    }
                    r = scanselforpath(path, 1 as libc::c_int != 0);
                    if sel as libc::c_uint == SEL_SELINV as libc::c_int as libc::c_uint
                        && !findselpos.is_null()
                    {
                        invertselbuf(r);
                    } else {
                        addtoselbuf(r, selstartid, selendid);
                    };
                    if cfg.x11() != 0 {
                        plugscript(
                            utils[16 as libc::c_int as usize],
                            (0x2 as libc::c_int | 0x4 as libc::c_int) as uchar_t,
                        );
                    }
                    continue;
                }
                2727668027383962707 => {
                    get_archive_cmd(newpath.as_mut_ptr(), tmp);
                    if r == 's' as i32 {
                        archive_selection(newpath.as_mut_ptr(), tmp, path);
                    } else {
                        spawn(
                            newpath.as_mut_ptr(),
                            tmp,
                            (*pdents.offset(cur as isize)).name,
                            0 as *mut libc::c_char,
                            (0x8 as libc::c_int | 0x1 as libc::c_int
                                | 0x10 as libc::c_int) as ushort_t,
                        );
                    };
                    mkpath(path, tmp, newpath.as_mut_ptr());
                    if !(access(newpath.as_mut_ptr(), 0 as libc::c_int)
                        == 0 as libc::c_int)
                    {
                        continue;
                    }
                    xstrsncpy(
                        lastname,
                        tmp,
                        (255 as libc::c_int + 1 as libc::c_int) as size_t,
                    );
                    clearfilter();
                    clearselection();
                    cd = 0 as libc::c_int != 0;
                    continue '_begin;
                }
                11573789974424595537 => {
                    qsort(
                        pdents as *mut libc::c_void,
                        ndents as size_t,
                        ::std::mem::size_of::<entry>() as libc::c_ulong,
                        entrycmpfn,
                    );
                    move_cursor(
                        if ndents != 0 {
                            dentfind(lastname, ndents)
                        } else {
                            0 as libc::c_int
                        },
                        0 as libc::c_int,
                    );
                    continue;
                }
                13479157322803929894 => {
                    spawn(
                        opener,
                        newpath.as_mut_ptr(),
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                        opener_flags as ushort_t,
                    );
                    if g_state.autonext() as libc::c_int != 0
                        && cur != ndents - 1 as libc::c_int
                    {
                        move_cursor((cur + 1 as libc::c_int) % ndents, 0 as libc::c_int);
                    }
                    if cfg.filtermode() != 0 {
                        presel = '/' as i32;
                        clearfilter();
                    }
                    continue;
                }
                3562629391805533650 => {
                    current_block = 14319209798712792045;
                }
                14908777651318078790 => {
                    presel = 0 as libc::c_int;
                    continue '_begin;
                }
                12869885497274271669 => {
                    current_block = 13615634435221404298;
                }
                _ => {}
            }
            match current_block {
                14319209798712792045 => {
                    current_block = 17136875676406196640;
                }
                13615634435221404298 => {
                    current_block = 16923785083244880615;
                }
                _ => {}
            }
            match current_block {
                17136875676406196640 => {
                    current_block = 161814443543167606;
                }
                16923785083244880615 => {
                    current_block = 9877275528163147575;
                }
                _ => {}
            }
            match current_block {
                9877275528163147575 => {
                    r = handle_context_switch(sel);
                    if r < 0 as libc::c_int {
                        continue;
                    }
                    savecurctx(
                        path,
                        if ndents != 0 {
                            (*pdents.offset(cur as isize)).name
                        } else {
                            0 as *mut libc::c_char
                        },
                        r,
                    );
                    path = (g_ctx[r as usize].c_path).as_mut_ptr();
                    lastdir = (g_ctx[r as usize].c_last).as_mut_ptr();
                    lastname = (g_ctx[r as usize].c_name).as_mut_ptr();
                    tmp = (g_ctx[r as usize].c_fltr).as_mut_ptr();
                    if cfg.filtermode() as libc::c_int != 0
                        || (*tmp.offset(0 as libc::c_int as isize) as libc::c_int
                            == '/' as i32
                            || *tmp.offset(0 as libc::c_int as isize) as libc::c_int
                                == '\\' as i32)
                            && *tmp.offset(1 as libc::c_int as isize) as libc::c_int != 0
                    {
                        presel = '/' as i32;
                    } else {
                        watch = 1 as libc::c_int != 0;
                    }
                    continue '_begin;
                }
                161814443543167606 => {
                    current_block = 2423117211301313142;
                }
                _ => {}
            }
            match current_block {
                2423117211301313142 => {
                    current_block = 8733027823202672466;
                }
                _ => {}
            }
            match current_block {
                8733027823202672466 => {
                    current_block = 18162755968531476823;
                }
                _ => {}
            }
            match current_block {
                18162755968531476823 => {}
                _ => {}
            }
            if ndents != 0 {
                g_state.set_move_0(1 as libc::c_int as uint_t);
                handle_screen_move(sel);
            }
        }
        match current_block {
            1049211764165170956 => {
                if sel as libc::c_uint == SEL_HIDDEN as libc::c_int as libc::c_uint {
                    cfg.set_showhidden(cfg.showhidden() ^ 1 as libc::c_int as uint_t);
                    if cfg.filtermode() != 0 {
                        presel = '/' as i32;
                    }
                    clearfilter();
                }
                xstrsncpy(
                    lastname,
                    if ndents != 0 {
                        (*pdents.offset(cur as isize)).name as *const libc::c_char
                    } else {
                        b"\0\0" as *const u8 as *const libc::c_char
                    },
                    (255 as libc::c_int + 1 as libc::c_int) as size_t,
                );
                cd = 0 as libc::c_int != 0;
                continue;
            }
            6626987748108865425 => {
                if strcmp(path, plgpath) == 0 as libc::c_int {
                    xstrsncpy(path, lastdir, 4096 as libc::c_int as size_t);
                    xstrsncpy(
                        lastname,
                        runfile.as_mut_ptr(),
                        (255 as libc::c_int + 1 as libc::c_int) as size_t,
                    );
                    runfile[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                    if cfg.filtermode() as libc::c_int != 0 {
                        presel = '/' as i32;
                    } else {
                        watch = 1 as libc::c_int != 0;
                    };
                    continue;
                } else {
                    g_state.set_runplugin(1 as libc::c_int as uint_t);
                }
                current_block = 5946490239152004759;
            }
            8470013831079077146 => {
                close(fd);
                xstrsncpy(
                    lastname,
                    tmp,
                    (255 as libc::c_int + 1 as libc::c_int) as size_t,
                );
                current_block = 10714207895679337196;
            }
            4491581808830814586 => {
                if cdprep(
                    lastdir,
                    if !listpath.is_null()
                        && sel as libc::c_uint
                            == SEL_CDLAST as libc::c_int as libc::c_uint
                    {
                        0 as *mut libc::c_char
                    } else {
                        lastname
                    },
                    path,
                    newpath.as_mut_ptr(),
                ) as libc::c_int != 0
                {
                    presel = '/' as i32;
                } else {
                    watch = 1 as libc::c_int != 0;
                };
                continue;
            }
            _ => {}
        }
        match current_block {
            5946490239152004759 => {
                xstrsncpy(lastdir, path, 4096 as libc::c_int as size_t);
                xstrsncpy(path, plgpath, 4096 as libc::c_int as size_t);
                if ndents != 0 {
                    xstrsncpy(
                        runfile.as_mut_ptr(),
                        (*pdents.offset(cur as isize)).name,
                        255 as libc::c_int as size_t,
                    );
                }
                g_state.set_runctx(cfg.curctx());
                *lastname
                    .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            }
            10714207895679337196 => {
                cd = 0 as libc::c_int != 0;
                continue;
            }
            _ => {}
        }
        if cfg.filtermode() as libc::c_int != 0 {
            presel = '/' as i32;
        } else {
            watch = 1 as libc::c_int != 0;
        };
        clearfilter();
        if g_state.runplugin() as libc::c_int == 1 as libc::c_int {
            presel = '/' as i32;
        }
    }
    tmp = getenv(b"NNN_TMPFILE\0" as *const u8 as *const libc::c_char);
    if sel as libc::c_uint == SEL_QUITCD as libc::c_int as libc::c_uint || !tmp.is_null()
    {
        write_lastdir(path, tmp);
        if sel as libc::c_uint == SEL_QUITCD as libc::c_int as libc::c_uint
            && g_state.picker() as libc::c_int != 0
        {
            selbufpos = 0 as libc::c_int as uint_t;
        }
    }
    if sel as libc::c_uint != SEL_QUITERR as libc::c_int as libc::c_uint {
        return 0 as libc::c_int != 0;
    }
    if selbufpos != 0 && g_state.picker() == 0 {
        g_state.set_picker(1 as libc::c_int as uint_t);
        free(selpath as *mut libc::c_void);
        selpath = 0 as *mut libc::c_char;
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn make_tmp_tree(
    mut paths: *mut *mut libc::c_char,
    mut entries: ssize_t,
    mut prefix: *const libc::c_char,
) -> *mut libc::c_char {
    let mut err: libc::c_int = 0;
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
    let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: ssize_t = xstrlen(prefix) as ssize_t;
    let mut tmpdir: *mut libc::c_char = malloc(4096 as libc::c_int as libc::c_ulong)
        as *mut libc::c_char;
    if tmpdir.is_null() {
        return 0 as *mut libc::c_char;
    }
    tmp = tmpdir
        .offset(tmpfplen as libc::c_int as isize)
        .offset(-(1 as libc::c_int as isize));
    xstrsncpy(tmpdir, g_tmpfpath.as_mut_ptr(), tmpfplen as size_t);
    xstrsncpy(
        tmp,
        b"/nnnXXXXXX\0" as *const u8 as *const libc::c_char,
        11 as libc::c_int as size_t,
    );
    tmp = tmp.offset(10 as libc::c_int as isize);
    if *prefix.offset(1 as libc::c_int as isize) == 0
        && *prefix.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        len = 0 as libc::c_int as ssize_t;
    }
    if (mkdtemp(tmpdir)).is_null() {
        free(tmpdir as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    listpath = tmpdir;
    let mut i: ssize_t = 0 as libc::c_int as ssize_t;
    while i < entries {
        if !(*paths.offset(i as isize)).is_null() {
            err = stat(*paths.offset(i as isize), &mut sb);
            if !(err != 0 && *__errno_location() == 2 as libc::c_int) {
                xstrsncpy(
                    tmp,
                    (*paths.offset(i as isize)).offset(len as isize),
                    (xstrlen(*paths.offset(i as isize)))
                        .wrapping_sub(len as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                slash = xmemrchr(
                    tmp as *mut uchar_t,
                    '/' as i32 as uchar_t,
                    (xstrlen(*paths.offset(i as isize)))
                        .wrapping_sub(len as libc::c_ulong),
                ) as *mut libc::c_char;
                if !slash.is_null() {
                    *slash = '\0' as i32 as libc::c_char;
                }
                if access(tmpdir, 0 as libc::c_int) != 0 {
                    xmktree(tmpdir, 1 as libc::c_int != 0);
                }
                if !slash.is_null() {
                    *slash = '/' as i32 as libc::c_char;
                }
                symlink(*paths.offset(i as isize), tmpdir) != 0;
            }
        }
        i += 1;
        i;
    }
    *tmp = '\0' as i32 as libc::c_char;
    return tmpdir;
}
unsafe extern "C" fn load_input(
    mut fd: libc::c_int,
    mut path: *const libc::c_char,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut i: ssize_t = 0;
    let mut chunk_count: ssize_t = 1 as libc::c_int as ssize_t;
    let mut chunk: ssize_t = (512 as libc::c_int * 1024 as libc::c_int) as ssize_t;
    let mut entries: ssize_t = 0 as libc::c_int as ssize_t;
    let mut input: *mut libc::c_char = malloc(
        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(chunk as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut tmpdir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cwd: [libc::c_char; 4096] = [0; 4096];
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut offsets: [size_t; 16384] = [0; 16384];
    let mut paths: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut input_read: ssize_t = 0;
    let mut total_read: ssize_t = 0 as libc::c_int as ssize_t;
    let mut off: ssize_t = 0 as libc::c_int as ssize_t;
    let mut msgnum: libc::c_int = 0 as libc::c_int;
    if input.is_null() {
        return 0 as *mut libc::c_char;
    }
    if path.is_null() {
        if (getcwd(cwd.as_mut_ptr(), 4096 as libc::c_int as size_t)).is_null() {
            free(input as *mut libc::c_void);
            return 0 as *mut libc::c_char;
        }
    } else {
        xstrsncpy(cwd.as_mut_ptr(), path, 4096 as libc::c_int as size_t);
    }
    loop {
        if !(chunk_count < 512 as libc::c_int as libc::c_long && msgnum == 0) {
            current_block = 18377268871191777778;
            break;
        }
        input_read = read(
            fd,
            input.offset(total_read as isize) as *mut libc::c_void,
            chunk as size_t,
        );
        if input_read < 0 as libc::c_int as libc::c_long {
            current_block = 1520370845280664167;
            break;
        }
        if input_read == 0 as libc::c_int as libc::c_long {
            current_block = 18377268871191777778;
            break;
        }
        total_read += input_read;
        chunk_count += 1;
        chunk_count;
        while off < total_read {
            next = memchr(
                input.offset(off as isize) as *const libc::c_void,
                '\0' as i32,
                (total_read - off) as libc::c_ulong,
            ) as *mut libc::c_char;
            if next.is_null() {
                break;
            }
            next = next.offset(1);
            next;
            if next.offset_from(input) as libc::c_long
                == off + 1 as libc::c_int as libc::c_long
            {
                off = next.offset_from(input) as libc::c_long;
            } else if entries
                == ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_long
            {
                msgnum = 44 as libc::c_int;
                break;
            } else {
                let fresh33 = entries;
                entries = entries + 1;
                offsets[fresh33 as usize] = off as size_t;
                off = next.offset_from(input) as libc::c_long;
            }
        }
        if chunk_count == 512 as libc::c_int as libc::c_long {
            msgnum = 10 as libc::c_int;
            current_block = 18377268871191777778;
            break;
        } else {
            if chunk_count == (total_read - input_read) / chunk {
                continue;
            }
            chunk_count = total_read / chunk;
            if total_read % chunk != 0 {
                chunk_count += 1;
                chunk_count;
            }
            input = xrealloc(
                input as *mut libc::c_void,
                ((chunk_count + 1 as libc::c_int as libc::c_long) * chunk) as size_t,
            ) as *mut libc::c_char;
            if input.is_null() {
                return 0 as *mut libc::c_char;
            }
        }
    }
    match current_block {
        18377268871191777778 => {
            if msgnum != 0 {
                let mut buf: [libc::c_char; 512] = [0; 512];
                while read(
                    fd,
                    buf.as_mut_ptr() as *mut libc::c_void,
                    512 as libc::c_int as size_t,
                ) > 0 as libc::c_int as libc::c_long
                {}
            }
            if off != total_read {
                if entries == ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_long {
                    msgnum = 44 as libc::c_int;
                } else {
                    let fresh34 = entries;
                    entries = entries + 1;
                    offsets[fresh34 as usize] = off as size_t;
                }
            }
            if entries == 0 {
                msgnum = 1 as libc::c_int;
            } else {
                *input.offset(total_read as isize) = '\0' as i32 as libc::c_char;
                paths = malloc(
                    (entries as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut *mut libc::c_char;
                if !paths.is_null() {
                    i = 0 as libc::c_int as ssize_t;
                    while i < entries {
                        let ref mut fresh35 = *paths.offset(i as isize);
                        *fresh35 = input.offset(offsets[i as usize] as isize);
                        i += 1;
                        i;
                    }
                    listroot = malloc(
                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                            .wrapping_mul(4096 as libc::c_int as libc::c_ulong),
                    ) as *mut libc::c_char;
                    if !listroot.is_null() {
                        *listroot
                            .offset(
                                0 as libc::c_int as isize,
                            ) = '\0' as i32 as libc::c_char;
                        i = 0 as libc::c_int as ssize_t;
                        loop {
                            if !(i < entries) {
                                current_block = 3580086814630675314;
                                break;
                            }
                            if *(*paths.offset(i as isize))
                                .offset(0 as libc::c_int as isize) as libc::c_int
                                == '\n' as i32
                                || selforparent(*paths.offset(i as isize)) as libc::c_int
                                    != 0
                            {
                                let ref mut fresh36 = *paths.offset(i as isize);
                                *fresh36 = 0 as *mut libc::c_char;
                            } else {
                                let ref mut fresh37 = *paths.offset(i as isize);
                                *fresh37 = abspath(
                                    *paths.offset(i as isize),
                                    cwd.as_mut_ptr(),
                                    0 as *mut libc::c_char,
                                );
                                if (*paths.offset(i as isize)).is_null() {
                                    entries = i;
                                    current_block = 2117186239120683552;
                                    break;
                                } else {
                                    xstrsncpy(
                                        g_buf.as_mut_ptr(),
                                        *paths.offset(i as isize),
                                        4096 as libc::c_int as size_t,
                                    );
                                    if (common_prefix(xdirname(g_buf.as_mut_ptr()), listroot))
                                        .is_null()
                                    {
                                        entries = i + 1 as libc::c_int as libc::c_long;
                                        current_block = 2117186239120683552;
                                        break;
                                    }
                                }
                            }
                            i += 1;
                            i;
                        }
                        match current_block {
                            3580086814630675314 => {
                                if *listroot.offset(0 as libc::c_int as isize) != 0 {
                                    tmpdir = make_tmp_tree(paths, entries, listroot);
                                }
                            }
                            _ => {}
                        }
                        i = entries - 1 as libc::c_int as libc::c_long;
                        while i >= 0 as libc::c_int as libc::c_long {
                            free(*paths.offset(i as isize) as *mut libc::c_void);
                            i -= 1;
                            i;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if msgnum != 0 {
        if !home.is_null() {
            printmsg(messages[msgnum as usize]);
            xdelay(((350000 as libc::c_int) << 2 as libc::c_int) as useconds_t);
        } else {
            msg(messages[msgnum as usize]);
            usleep(((350000 as libc::c_int) << 2 as libc::c_int) as __useconds_t);
        }
    }
    free(input as *mut libc::c_void);
    free(paths as *mut libc::c_void);
    return tmpdir;
}
unsafe extern "C" fn check_key_collision() {
    let mut key: libc::c_int = 0;
    let mut bitmap: [bool; 511] = [
        0 as libc::c_int != 0,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    ];
    let mut i: ullong_t = 0 as libc::c_int as ullong_t;
    while i
        < (::std::mem::size_of::<[key; 85]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<key>() as libc::c_ulong)
            as libc::c_ulonglong
    {
        key = bindings[i as usize].sym as libc::c_int;
        if bitmap[key as usize] {
            dprintf(
                2 as libc::c_int,
                b"key collision! [%s]\n\0" as *const u8 as *const libc::c_char,
                keyname(key),
            );
        } else {
            bitmap[key as usize] = 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn usage() {
    dprintf(
        1 as libc::c_int,
        b"%s: nnn [OPTIONS] [PATH]\n\nThe unorthodox terminal file manager.\n\npositional args:\n  PATH   start dir/file [default: .]\n\noptional args:\n -a      auto NNN_FIFO\n -A      no dir auto-enter during filter\n -b key  open bookmark key (trumps -s/S)\n -B      use bsdtar for archives\n -c      cli-only NNN_OPENER (trumps -e)\n -C      8-color scheme\n -d      detail mode\n -D      dirs in context color\n -e      text in $VISUAL/$EDITOR/vi\n -E      internal edits in EDITOR\n -f      use readline history file\n -F val  fifo mode [0:preview 1:explore]\n -g      regex filters\n -H      show hidden files\n -i      show current file info\n -J      no auto-advance on selection\n -K      detect key collision\n -l val  set scroll lines\n -n      type-to-nav mode\n -o      open files only on Enter\n -p file selection file [-:stdout]\n -P key  run plugin key\n -Q      no quit confirmation\n -r      use advcpmv patched cp, mv\n -R      no rollover at edges\n -s name load session by name\n -S      persistent session\n -t secs timeout to lock\n -T key  sort order [a/d/e/r/s/t/v]\n -u      use selection (no prompt)\n -U      show user and group\n -V      show version\n -x      notis, selection sync, xterm title\n -h      show help\n\nv%s\n%s\n\0"
            as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"usage\0")).as_ptr(),
        b"4.6\0" as *const u8 as *const libc::c_char,
        b"BSD 2-Clause\nhttps://github.com/jarun/nnn\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn setup_config() -> bool {
    let mut r: size_t = 0;
    let mut len: size_t = 0;
    let mut xdgcfg: *mut libc::c_char = getenv(
        b"XDG_CONFIG_HOME\0" as *const u8 as *const libc::c_char,
    );
    let mut xdg: bool = 0 as libc::c_int != 0;
    if !xdgcfg.is_null() && *xdgcfg.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        if *xdgcfg.offset(0 as libc::c_int as isize) as libc::c_int == '~' as i32 {
            r = xstrsncpy(g_buf.as_mut_ptr(), home, 4096 as libc::c_int as size_t);
            xstrsncpy(
                g_buf
                    .as_mut_ptr()
                    .offset(r as isize)
                    .offset(-(1 as libc::c_int as isize)),
                xdgcfg.offset(1 as libc::c_int as isize),
                4096 as libc::c_int as size_t,
            );
            xdgcfg = g_buf.as_mut_ptr();
        }
        if !xdiraccess(xdgcfg) {
            perror(xitoa(8196 as libc::c_int as uint_t));
            return 0 as libc::c_int != 0;
        }
        len = (xstrlen(xdgcfg))
            .wrapping_add(
                xstrlen(b"/nnn/bookmarks\0" as *const u8 as *const libc::c_char),
            )
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        xdg = 1 as libc::c_int != 0;
    }
    if !xdg {
        len = (xstrlen(home))
            .wrapping_add(
                xstrlen(b"/.config/nnn/bookmarks\0" as *const u8 as *const libc::c_char),
            )
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
    }
    cfgpath = malloc(len) as *mut libc::c_char;
    plgpath = malloc(len) as *mut libc::c_char;
    if cfgpath.is_null() || plgpath.is_null() {
        perror(xitoa(8210 as libc::c_int as uint_t));
        return 0 as libc::c_int != 0;
    }
    if xdg {
        xstrsncpy(cfgpath, xdgcfg, len);
        r = len
            .wrapping_sub(
                xstrlen(b"/nnn/bookmarks\0" as *const u8 as *const libc::c_char),
            );
    } else {
        r = xstrsncpy(cfgpath, home, len);
        xstrsncpy(
            cfgpath.offset(r as isize).offset(-(1 as libc::c_int as isize)),
            b"/.config\0" as *const u8 as *const libc::c_char,
            len.wrapping_sub(r),
        );
        r = (r as libc::c_ulong).wrapping_add(8 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    xstrsncpy(
        cfgpath.offset(r as isize).offset(-(1 as libc::c_int as isize)),
        b"/nnn\0" as *const u8 as *const libc::c_char,
        len.wrapping_sub(r),
    );
    r = 0 as libc::c_int as size_t;
    while r
        < (::std::mem::size_of::<[*const libc::c_char; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        mkpath(cfgpath, toks[r as usize], plgpath);
        if access(plgpath, 0 as libc::c_int) != 0
            && !xmktree(plgpath, 1 as libc::c_int != 0)
        {
            perror(xitoa(8236 as libc::c_int as uint_t));
            return 0 as libc::c_int != 0;
        }
        r = r.wrapping_add(1);
        r;
    }
    if g_state.picker() == 0 {
        let mut env_sel: *mut libc::c_char = xgetenv(
            env_cfg[9 as libc::c_int as usize],
            0 as *mut libc::c_char,
        );
        selpath = if !env_sel.is_null() {
            xstrdup(env_sel)
        } else {
            malloc(len.wrapping_add(3 as libc::c_int as libc::c_ulong))
                as *mut libc::c_char
        };
        if selpath.is_null() {
            perror(xitoa(8249 as libc::c_int as uint_t));
            return 0 as libc::c_int != 0;
        }
        if env_sel.is_null() {
            r = xstrsncpy(
                selpath,
                cfgpath,
                len.wrapping_add(3 as libc::c_int as libc::c_ulong),
            );
            xstrsncpy(
                selpath.offset(r as isize).offset(-(1 as libc::c_int as isize)),
                b"/.selection\0" as *const u8 as *const libc::c_char,
                12 as libc::c_int as size_t,
            );
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn set_tmp_path() -> bool {
    let mut tmp: *mut libc::c_char = b"/tmp\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut path: *mut libc::c_char = if xdiraccess(tmp) as libc::c_int != 0 {
        tmp
    } else {
        getenv(b"TMPDIR\0" as *const u8 as *const libc::c_char)
    };
    if path.is_null() {
        msg(b"set TMPDIR\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int != 0;
    }
    tmpfplen = xstrsncpy(g_tmpfpath.as_mut_ptr(), path, 64 as libc::c_int as size_t)
        as uchar_t;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cleanup() {
    if cfg.x11() as libc::c_int != 0 && g_state.picker() == 0 {
        printf(b"\x1B[23;0t\0" as *const u8 as *const libc::c_char);
        fflush(stdout);
    }
    free(selpath as *mut libc::c_void);
    free(plgpath as *mut libc::c_void);
    free(cfgpath as *mut libc::c_void);
    free(initpath as *mut libc::c_void);
    free(bmstr as *mut libc::c_void);
    free(pluginstr as *mut libc::c_void);
    free(listroot as *mut libc::c_void);
    free(ihashbmp as *mut libc::c_void);
    free(bookmark as *mut libc::c_void);
    free(plug as *mut libc::c_void);
    free(lastcmd as *mut libc::c_void);
    if g_state.autofifo() != 0 {
        unlink(fifopath);
    }
    if g_state.pluginit() != 0 {
        unlink(g_pipepath.as_mut_ptr());
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut session: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fd: libc::c_int = 0;
    let mut opt: libc::c_int = 0;
    let mut sort: libc::c_int = 0 as libc::c_int;
    let mut pkey: libc::c_int = '\0' as i32;
    let mut mask: mmask_t = 0;
    let mut middle_click_env: *mut libc::c_char = xgetenv(
        env_cfg[8 as libc::c_int as usize],
        b"\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    middle_click_key = if *middle_click_env.offset(0 as libc::c_int as isize)
        as libc::c_int == '^' as i32
        && *middle_click_env.offset(1 as libc::c_int as isize) as libc::c_int != 0
    {
        *middle_click_env.offset(1 as libc::c_int as isize) as libc::c_int
            & 0x1f as libc::c_int
    } else {
        *middle_click_env.offset(0 as libc::c_int as isize) as uchar_t as libc::c_int
    };
    let env_opts: *const libc::c_char = xgetenv(
        env_cfg[0 as libc::c_int as usize],
        0 as *mut libc::c_char,
    );
    let mut env_opts_id: libc::c_int = if !env_opts.is_null() {
        xstrlen(env_opts) as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    let mut rlhist: bool = 0 as libc::c_int != 0;
    loop {
        opt = (if env_opts_id > 0 as libc::c_int {
            env_opts_id -= 1;
            *env_opts.offset(env_opts_id as isize) as libc::c_int
        } else {
            getopt(
                argc,
                argv as *const *mut libc::c_char,
                b"aAb:BcCdDeEfF:gHiJKl:nop:P:QrRs:St:T:uUVxh\0" as *const u8
                    as *const libc::c_char,
            )
        });
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        match opt {
            97 => {
                g_state.set_autofifo(1 as libc::c_int as uint_t);
            }
            65 => {
                cfg.set_autoenter(0 as libc::c_int as uint_t);
            }
            98 => {
                if env_opts_id < 0 as libc::c_int {
                    arg = optarg;
                }
            }
            66 => {
                g_state.set_usebsdtar(1 as libc::c_int as uint_t);
            }
            99 => {
                cfg.set_cliopener(1 as libc::c_int as uint_t);
            }
            67 => {
                g_state.set_oldcolor(1 as libc::c_int as uint_t);
            }
            100 => {
                cfg.set_showdetail(1 as libc::c_int as uint_t);
            }
            68 => {
                g_state.set_dirctx(1 as libc::c_int as uint_t);
            }
            101 => {
                cfg.set_useeditor(1 as libc::c_int as uint_t);
            }
            69 => {
                cfg.set_waitedit(1 as libc::c_int as uint_t);
            }
            102 => {
                rlhist = 1 as libc::c_int != 0;
            }
            70 => {
                if env_opts_id < 0 as libc::c_int {
                    fd = atoi(optarg);
                    if fd < 0 as libc::c_int || fd > 1 as libc::c_int {
                        return 1 as libc::c_int;
                    }
                    g_state.set_fifomode(fd as uint_t);
                }
            }
            103 => {
                cfg.set_regex(1 as libc::c_int as uint_t);
                filterfn = Some(
                    visible_re
                        as unsafe extern "C" fn(
                            *const fltrexp_t,
                            *const libc::c_char,
                        ) -> libc::c_int,
                );
            }
            72 => {
                cfg.set_showhidden(1 as libc::c_int as uint_t);
            }
            105 => {
                cfg.set_fileinfo(1 as libc::c_int as uint_t);
            }
            74 => {
                g_state.set_stayonsel(1 as libc::c_int as uint_t);
            }
            75 => {
                check_key_collision();
                return 0 as libc::c_int;
            }
            108 => {
                if env_opts_id < 0 as libc::c_int {
                    scroll_lines = atoi(optarg);
                }
            }
            110 => {
                cfg.set_filtermode(1 as libc::c_int as uint_t);
            }
            111 => {
                cfg.set_nonavopen(1 as libc::c_int as uint_t);
            }
            112 => {
                if !(env_opts_id >= 0 as libc::c_int) {
                    g_state.set_picker(1 as libc::c_int as uint_t);
                    if !(*optarg.offset(0 as libc::c_int as isize) as libc::c_int
                        == '-' as i32
                        && *optarg.offset(1 as libc::c_int as isize) as libc::c_int
                            == '\0' as i32)
                    {
                        fd = open(
                            optarg,
                            0o1 as libc::c_int | 0o100 as libc::c_int,
                            0o600 as libc::c_int,
                        );
                        if fd == -(1 as libc::c_int) {
                            perror(xitoa(8416 as libc::c_int as uint_t));
                            return 1 as libc::c_int;
                        }
                        close(fd);
                        selpath = abspath(
                            optarg,
                            0 as *const libc::c_char,
                            0 as *mut libc::c_char,
                        );
                        unlink(selpath);
                    }
                }
            }
            80 => {
                if env_opts_id < 0 as libc::c_int
                    && *optarg.offset(1 as libc::c_int as isize) == 0
                {
                    pkey = *optarg.offset(0 as libc::c_int as isize) as uchar_t
                        as libc::c_int;
                }
            }
            81 => {
                g_state.set_forcequit(1 as libc::c_int as uint_t);
            }
            114 => {
                mv[5 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
                mv[2 as libc::c_int as usize] = mv[5 as libc::c_int as usize];
                cp[5 as libc::c_int as usize] = mv[2 as libc::c_int as usize];
                cp[2 as libc::c_int as usize] = cp[5 as libc::c_int as usize];
                mv[4 as libc::c_int as usize] = '-' as i32 as libc::c_char;
                cp[4 as libc::c_int as usize] = mv[4 as libc::c_int as usize];
            }
            82 => {
                cfg.set_rollover(0 as libc::c_int as uint_t);
            }
            115 => {
                if env_opts_id < 0 as libc::c_int {
                    session = optarg;
                }
            }
            83 => {
                g_state.set_prstssn(1 as libc::c_int as uint_t);
                if session.is_null() {
                    session = b"@\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
            }
            116 => {
                if env_opts_id < 0 as libc::c_int {
                    idletimeout = atoi(optarg) as uint_t;
                }
            }
            84 => {
                if env_opts_id < 0 as libc::c_int {
                    sort = *optarg.offset(0 as libc::c_int as isize) as uchar_t
                        as libc::c_int;
                }
            }
            117 => {
                cfg.set_prefersel(1 as libc::c_int as uint_t);
            }
            85 => {
                g_state.set_uidgid(1 as libc::c_int as uint_t);
            }
            86 => {
                dprintf(
                    1 as libc::c_int,
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    b"4.6\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
            120 => {
                cfg.set_x11(1 as libc::c_int as uint_t);
            }
            104 => {
                usage();
                return 0 as libc::c_int;
            }
            _ => {
                usage();
                return 1 as libc::c_int;
            }
        }
        if env_opts_id == 0 as libc::c_int {
            env_opts_id = -(1 as libc::c_int);
        }
    }
    if !set_tmp_path() {
        return 1 as libc::c_int;
    }
    atexit(Some(cleanup as unsafe extern "C" fn() -> ()));
    if isatty(0 as libc::c_int) == 0 {
        initpath = load_input(0 as libc::c_int, 0 as *const libc::c_char);
        if initpath.is_null() {
            return 1 as libc::c_int;
        }
        if isatty(1 as libc::c_int) == 0 {
            fd = open(
                ctermid(0 as *mut libc::c_char),
                0 as libc::c_int,
                0o400 as libc::c_int,
            );
            dup2(fd, 0 as libc::c_int);
            close(fd);
        } else {
            dup2(1 as libc::c_int, 0 as libc::c_int);
        }
        if !session.is_null() {
            session = 0 as *mut libc::c_char;
        }
    }
    home = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
    if home.is_null() {
        msg(b"set HOME\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    homelen = xstrlen(home) as uchar_t;
    if !setup_config() {
        return 1 as libc::c_int;
    }
    opener = xgetenv(
        env_cfg[3 as libc::c_int as usize],
        utils[0 as libc::c_int as usize],
    );
    if !parsekvpair(&mut bookmark, &mut bmstr, 1 as libc::c_int as uchar_t, &mut maxbm) {
        msg(env_cfg[1 as libc::c_int as usize]);
        return 1 as libc::c_int;
    }
    if !parsekvpair(
        &mut plug,
        &mut pluginstr,
        2 as libc::c_int as uchar_t,
        &mut maxplug,
    ) {
        msg(env_cfg[2 as libc::c_int as usize]);
        return 1 as libc::c_int;
    }
    if !parsekvpair(
        &mut order,
        &mut orderstr,
        11 as libc::c_int as uchar_t,
        &mut maxorder,
    ) {
        msg(env_cfg[11 as libc::c_int as usize]);
        return 1 as libc::c_int;
    }
    if initpath.is_null() {
        if !arg.is_null() {
            if *arg.offset(1 as libc::c_int as isize) == 0 {
                initpath = get_kv_val(
                    bookmark,
                    0 as *mut libc::c_char,
                    *arg as libc::c_int,
                    maxbm,
                    1 as libc::c_int as uchar_t,
                );
            }
            if initpath.is_null() {
                msg(messages[40 as libc::c_int as usize]);
                return 1 as libc::c_int;
            }
            if !session.is_null() {
                session = 0 as *mut libc::c_char;
            }
        } else if argc == optind {
            let mut startpath: *mut libc::c_char = getenv(
                b"PWD\0" as *const u8 as *const libc::c_char,
            );
            initpath = if !startpath.is_null() {
                xstrdup(startpath)
            } else {
                getcwd(0 as *mut libc::c_char, 0 as libc::c_int as size_t)
            };
            if initpath.is_null() {
                initpath = b"/\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
        } else {
            arg = *argv.offset(optind as isize);
            if xstrlen(arg) > 7 as libc::c_int as libc::c_ulong
                && is_prefix(
                    arg,
                    b"file://\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as size_t,
                ) as libc::c_int != 0
            {
                arg = arg.offset(7 as libc::c_int as isize);
            }
            initpath = abspath(arg, 0 as *const libc::c_char, 0 as *mut libc::c_char);
            if initpath.is_null() {
                perror(xitoa(8573 as libc::c_int as uint_t));
                return 1 as libc::c_int;
            }
            if *xbasename(initpath) as libc::c_int == '.' as i32 {
                cfg.set_showhidden(1 as libc::c_int as uint_t);
            }
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
            if stat(initpath, &mut sb) == -(1 as libc::c_int) {
                perror(xitoa(8589 as libc::c_int as uint_t));
                return 1 as libc::c_int;
            }
            if !(sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint)
            {
                g_state.set_initfile(1 as libc::c_int as uint_t);
            }
            if !session.is_null() {
                session = 0 as *mut libc::c_char;
            }
        }
    }
    enveditor = getenv(env_cfg[10 as libc::c_int as usize]);
    if setfilter(
        &mut archive_re,
        if !enveditor.is_null() {
            enveditor as *const libc::c_char
        } else {
            patterns[2 as libc::c_int as usize]
        },
    ) != 0
    {
        msg(messages[35 as libc::c_int as usize]);
        return 1 as libc::c_int;
    }
    if cfg.cliopener() != 0 {
        cfg.set_useeditor(0 as libc::c_int as uint_t);
    }
    enveditor = xgetenv(
        envs[2 as libc::c_int as usize],
        utils[11 as libc::c_int as usize],
    );
    editor = xgetenv(envs[1 as libc::c_int as usize], enveditor);
    pager = xgetenv(envs[3 as libc::c_int as usize], utils[12 as libc::c_int as usize]);
    shell = xgetenv(envs[0 as libc::c_int as usize], utils[13 as libc::c_int as usize]);
    if g_state.autofifo() != 0 {
        g_tmpfpath[(tmpfplen as libc::c_int - 1 as libc::c_int)
            as usize] = '\0' as i32 as libc::c_char;
        let mut r: size_t = mkpath(
            g_tmpfpath.as_mut_ptr(),
            b"nnn-fifo.\0" as *const u8 as *const libc::c_char,
            g_buf.as_mut_ptr(),
        );
        xstrsncpy(
            g_buf.as_mut_ptr().offset(r as isize).offset(-(1 as libc::c_int as isize)),
            xitoa(getpid() as uint_t),
            (4096 as libc::c_int as libc::c_ulong).wrapping_sub(r),
        );
        setenv(
            b"NNN_FIFO\0" as *const u8 as *const libc::c_char,
            g_buf.as_mut_ptr(),
            1 as libc::c_int,
        );
    }
    fifopath = xgetenv(
        b"NNN_FIFO\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_char,
    );
    if !fifopath.is_null() {
        if mkfifo(fifopath, 0o600 as libc::c_int as __mode_t) != 0 as libc::c_int
            && !(*__errno_location() == 17 as libc::c_int
                && access(fifopath, 2 as libc::c_int) == 0 as libc::c_int)
        {
            perror(xitoa(8647 as libc::c_int as uint_t));
            return 1 as libc::c_int;
        }
        sigaction(
            13 as libc::c_int,
            &mut {
                let mut init = sigaction {
                    __sigaction_handler: C2RustUnnamed_10 {
                        sa_handler: ::std::mem::transmute::<
                            libc::intptr_t,
                            __sighandler_t,
                        >(1 as libc::c_int as libc::intptr_t),
                    },
                    sa_mask: __sigset_t { __val: [0; 16] },
                    sa_flags: 0,
                    sa_restorer: None,
                };
                init
            },
            0 as *mut sigaction,
        );
    }
    inotify_fd = inotify_init1(IN_NONBLOCK as libc::c_int | IN_CLOEXEC as libc::c_int);
    if inotify_fd < 0 as libc::c_int {
        perror(xitoa(8659 as libc::c_int as uint_t));
        return 1 as libc::c_int;
    }
    opt = xgetenv_val(env_cfg[13 as libc::c_int as usize]) as libc::c_int;
    if opt != 0 && opt <= 2 as libc::c_int {
        g_state.set_trash(opt as uint_t);
    }
    let mut act: sigaction = {
        let mut init = sigaction {
            __sigaction_handler: C2RustUnnamed_10 {
                sa_handler: Some(
                    sigint_handler as unsafe extern "C" fn(libc::c_int) -> (),
                ),
            },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        init
    };
    if sigaction(2 as libc::c_int, &mut act, 0 as *mut sigaction) < 0 as libc::c_int {
        perror(xitoa(8685 as libc::c_int as uint_t));
        return 1 as libc::c_int;
    }
    act
        .__sigaction_handler
        .sa_handler = Some(
        clean_exit_sighandler as unsafe extern "C" fn(libc::c_int) -> (),
    );
    if sigaction(15 as libc::c_int, &mut act, 0 as *mut sigaction) < 0 as libc::c_int
        || sigaction(1 as libc::c_int, &mut act, 0 as *mut sigaction) < 0 as libc::c_int
    {
        perror(xitoa(8692 as libc::c_int as uint_t));
        return 1 as libc::c_int;
    }
    act
        .__sigaction_handler
        .sa_handler = ::std::mem::transmute::<
        libc::intptr_t,
        __sighandler_t,
    >(1 as libc::c_int as libc::intptr_t);
    if sigaction(3 as libc::c_int, &mut act, 0 as *mut sigaction) < 0 as libc::c_int {
        perror(xitoa(8699 as libc::c_int as uint_t));
        return 1 as libc::c_int;
    }
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    rl_change_environment = 0 as libc::c_int;
    rl_variable_bind(
        b"completion-ignore-case\0" as *const u8 as *const libc::c_char,
        b"on\0" as *const u8 as *const libc::c_char,
    );
    rl_bind_key(
        '\t' as i32,
        Some(
            rl_menu_complete
                as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
        ),
    );
    if rlhist {
        mkpath(
            cfgpath,
            b".history\0" as *const u8 as *const libc::c_char,
            g_buf.as_mut_ptr(),
        );
        read_history(g_buf.as_mut_ptr());
    }
    if cfg.x11() as libc::c_int != 0 && g_state.picker() == 0 {
        printf(b"\x1B[22;0t\0" as *const u8 as *const libc::c_char);
        fflush(stdout);
        gethostname(
            hostname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
        hostname[(::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = '\0' as i32 as libc::c_char;
    }
    if !initcurses(&mut mask as *mut mmask_t as *mut libc::c_void) {
        return 1 as libc::c_int;
    }
    if sort != 0 {
        set_sort_flags(sort);
    }
    opt = browse(initpath, session, pkey) as libc::c_int;
    if !session.is_null() && g_state.prstssn() as libc::c_int != 0 {
        save_session(session, 0 as *mut libc::c_int);
    }
    mousemask(mask, 0 as *mut mmask_t);
    endwin();
    if rlhist {
        mkpath(
            cfgpath,
            b".history\0" as *const u8 as *const libc::c_char,
            g_buf.as_mut_ptr(),
        );
        write_history(g_buf.as_mut_ptr());
    }
    if g_state.picker() != 0 {
        if selbufpos != 0 {
            fd = if !selpath.is_null() {
                open(
                    selpath,
                    0o1 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
                    0o600 as libc::c_int,
                )
            } else {
                1 as libc::c_int
            };
            if fd == -(1 as libc::c_int)
                || seltofile(fd, 0 as *mut uint_t) != selbufpos as size_t
            {
                perror(xitoa(8773 as libc::c_int as uint_t));
            }
            if fd > 1 as libc::c_int {
                close(fd);
            }
        }
    } else if !selpath.is_null() {
        unlink(selpath);
    }
    rmlistpath();
    regfree(&mut archive_re);
    free(pselbuf as *mut libc::c_void);
    if inotify_wd >= 0 as libc::c_int {
        inotify_rm_watch(inotify_fd, inotify_wd);
    }
    close(inotify_fd);
    if g_state.fifomode() == 0 {
        notify_fifo(0 as libc::c_int != 0);
    }
    if fifofd != -(1 as libc::c_int) {
        close(fifofd);
    }
    return opt;
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
    cfg = {
        let mut init = settings {
            filtermode_timeorder_sizeorder_apparentsz_blkorder_extnorder_showhidden_reserved0_showdetail_ctxactive_reverse_version_reserved1_curctx_prefersel_fileinfo_nonavopen_autoenter_reserved2_useeditor_reserved3_regex_x11_timetype_cliopener_waitedit_rollover: [0; 4],
        };
        init.set_filtermode(0);
        init.set_timeorder(0);
        init.set_sizeorder(0);
        init.set_apparentsz(0);
        init.set_blkorder(0);
        init.set_extnorder(0);
        init.set_showhidden(0);
        init.set_reserved0(0);
        init.set_showdetail(0);
        init.set_ctxactive(1 as libc::c_int as uint_t);
        init.set_reverse(0);
        init.set_version(0);
        init.set_reserved1(0);
        init.set_curctx(0);
        init.set_prefersel(0);
        init.set_fileinfo(0);
        init.set_nonavopen(0);
        init.set_autoenter(1 as libc::c_int as uint_t);
        init.set_reserved2(0);
        init.set_useeditor(0);
        init.set_reserved3(0);
        init.set_regex(0);
        init.set_x11(0);
        init.set_timetype(2 as libc::c_int as uint_t);
        init.set_cliopener(0);
        init.set_waitedit(0);
        init.set_rollover(1 as libc::c_int as uint_t);
        init
    };
    uidcache = (2147483647 as libc::c_int as libc::c_uint)
        .wrapping_mul(2 as libc::c_uint)
        .wrapping_add(1 as libc::c_uint);
    gidcache = (2147483647 as libc::c_int as libc::c_uint)
        .wrapping_mul(2 as libc::c_uint)
        .wrapping_add(1 as libc::c_uint);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
