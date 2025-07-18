use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigismember(__set: *const sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn __errno_location() -> *mut libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn rpl_fflush(gl_stream: *mut FILE) -> libc::c_int;
    fn rpl_fprintf(fp: *mut FILE, format: *const libc::c_char, _: ...) -> libc::c_int;
    fn __printf__(format: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn rpl_free(_: *mut libc::c_void);
    static mut stderr: *mut FILE;
    static mut header_bytes: off_t;
    fn zip(in_0: libc::c_int, out: libc::c_int) -> libc::c_int;
    static mut unzip_crc: ulg;
    fn unzip(in_0: libc::c_int, out: libc::c_int) -> libc::c_int;
    fn check_zipfile(in_0: libc::c_int) -> libc::c_int;
    fn unpack(in_0: libc::c_int, out: libc::c_int) -> libc::c_int;
    fn unlzh(in_0: libc::c_int, out: libc::c_int) -> libc::c_int;
    fn xunlink(fname: *mut libc::c_char) -> libc::c_int;
    fn copy(in_0: libc::c_int, out: libc::c_int) -> libc::c_int;
    fn updcrc(s: *const uch, n: libc::c_uint) -> ulg;
    fn clear_bufs();
    fn fill_inbuf(eof_ok: libc::c_int) -> libc::c_int;
    fn write_buf(fd: libc::c_int, buf: voidp, cnt: libc::c_uint);
    fn strlwr(s: *mut libc::c_char) -> *mut libc::c_char;
    fn gzip_base_name(fname: *mut libc::c_char) -> *mut libc::c_char;
    fn add_envopt(
        argcp: *mut libc::c_int,
        argvp: *mut *mut *mut libc::c_char,
        env_0: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn gzip_error(m: *const libc::c_char);
    fn read_error();
    fn write_error();
    fn display_ratio(num: off_t, den: off_t, file: *mut FILE);
    fn fprint_off(_: *mut FILE, _: off_t, _: libc::c_int);
    static mut stdout: *mut FILE;
    fn unlzw(in_0: libc::c_int, out: libc::c_int) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn openat_safer(
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: ...
    ) -> libc::c_int;
    fn open_safer(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut Version: *const libc::c_char;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn yesno() -> bool;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn unlinkat(
        __fd: libc::c_int,
        __name: *const libc::c_char,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn fdatasync(__fildes: libc::c_int) -> libc::c_int;
    fn fdopendir(__fd: libc::c_int) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn streamsavedir(_: *mut DIR, _: savedir_option) -> *mut libc::c_char;
    fn fdutimens(
        _: libc::c_int,
        _: *const libc::c_char,
        _: *const timespec,
    ) -> libc::c_int;
}
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
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
pub type ptrdiff_t = libc::c_long;
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
pub type voidp = *mut libc::c_void;
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
pub type uch = libc::c_uchar;
pub type ush = libc::c_ushort;
pub type ulg = libc::c_ulong;
pub type C2RustUnnamed_10 = libc::c_uint;
pub const TIMESPEC_RESOLUTION: C2RustUnnamed_10 = 1000000000;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type DIR = __dirstream;
pub type savedir_option = libc::c_uint;
pub const SAVEDIR_SORT_FASTREAD: savedir_option = 0;
pub const SAVEDIR_SORT_NAME: savedir_option = 1;
pub const SAVEDIR_SORT_NONE: savedir_option = 0;
pub type C2RustUnnamed_11 = libc::c_uint;
pub const ENV_OPTION: C2RustUnnamed_11 = 131;
pub const SYNCHRONOUS_OPTION: C2RustUnnamed_11 = 130;
pub const RSYNCABLE_OPTION: C2RustUnnamed_11 = 129;
pub const PRESUME_INPUT_TTY_OPTION: C2RustUnnamed_11 = 128;
pub const try_opening_directories: C2RustUnnamed_12 = 1;
pub type C2RustUnnamed_12 = libc::c_uint;
static mut license_msg: [*const libc::c_char; 6] = [
    b"Copyright (C) 2018 Free Software Foundation, Inc.\0" as *const u8
        as *const libc::c_char,
    b"Copyright (C) 1993 Jean-loup Gailly.\0" as *const u8 as *const libc::c_char,
    b"This is free software.  You may redistribute copies of it under the terms of\0"
        as *const u8 as *const libc::c_char,
    b"the GNU General Public License <https://www.gnu.org/licenses/gpl.html>.\0"
        as *const u8 as *const libc::c_char,
    b"There is NO WARRANTY, to the extent permitted by law.\0" as *const u8
        as *const libc::c_char,
    0 as *const libc::c_char,
];
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
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
#[inline]
unsafe extern "C" fn get_stat_atime(mut st: *const stat) -> timespec {
    return (*st).st_atim;
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
pub static mut inbuf: [uch; 262208] = [0; 262208];
pub static mut outbuf: [uch; 264192] = [0; 264192];
pub static mut d_buf: [ush; 32768] = [0; 32768];
pub static mut window: [uch; 65536] = [0; 65536];
pub static mut prev: [ush; 65536] = [0; 65536];
static mut presume_input_tty: bool = false;
static mut synchronous: bool = false;
static mut ascii: libc::c_int = 0 as libc::c_int;
pub static mut to_stdout: libc::c_int = 0 as libc::c_int;
static mut decompress: libc::c_int = 0 as libc::c_int;
static mut force: libc::c_int = 0 as libc::c_int;
static mut keep: libc::c_int = 0 as libc::c_int;
static mut no_name: libc::c_int = -(1 as libc::c_int);
static mut no_time: libc::c_int = -(1 as libc::c_int);
static mut recursive: libc::c_int = 0 as libc::c_int;
static mut list: libc::c_int = 0 as libc::c_int;
static mut verbose: libc::c_int = 0 as libc::c_int;
pub static mut quiet: libc::c_int = 0 as libc::c_int;
pub static mut test: libc::c_int = 0 as libc::c_int;
static mut foreground: libc::c_int = 0 as libc::c_int;
pub static mut program_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut maxbits: libc::c_int = 16 as libc::c_int;
pub static mut method: libc::c_int = 8 as libc::c_int;
pub static mut level: libc::c_int = 6 as libc::c_int;
pub static mut exit_code: libc::c_int = 0 as libc::c_int;
pub static mut save_orig_name: libc::c_int = 0;
static mut last_member: libc::c_int = 0;
static mut part_nb: libc::c_int = 0;
pub static mut ifile_size: off_t = 0;
static mut env: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut z_suffix: *const libc::c_char = 0 as *const libc::c_char;
static mut z_len: size_t = 0;
pub static mut time_stamp: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
static mut caught_signals: sigset_t = sigset_t { __val: [0; 16] };
static mut exiting_signal: libc::c_int = 0;
static mut remove_ofname_fd: libc::c_int = -(1 as libc::c_int);
static mut remove_ofname: [libc::c_char; 1024] = [0; 1024];
static mut stdin_was_read: bool = false;
pub static mut bytes_in: off_t = 0;
pub static mut bytes_out: off_t = 0;
static mut total_in: off_t = 0;
static mut total_out: off_t = 0;
pub static mut ifname: [libc::c_char; 1024] = [0; 1024];
pub static mut ofname: [libc::c_char; 1024] = [0; 1024];
static mut dfname: [libc::c_char; 1024] = [0; 1024];
static mut istat: stat = stat {
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
pub static mut ifd: libc::c_int = 0;
pub static mut ofd: libc::c_int = 0;
static mut dfd: libc::c_int = -(1 as libc::c_int);
pub static mut insize: libc::c_uint = 0;
pub static mut inptr: libc::c_uint = 0;
pub static mut outcnt: libc::c_uint = 0;
pub static mut rsync: libc::c_int = 0 as libc::c_int;
static mut handled_sig: [libc::c_int; 6] = [
    2 as libc::c_int,
    1 as libc::c_int,
    13 as libc::c_int,
    15 as libc::c_int,
    24 as libc::c_int,
    25 as libc::c_int,
];
static mut shortopts: [libc::c_char; 34] = unsafe {
    *::std::mem::transmute::<
        &[u8; 34],
        &[libc::c_char; 34],
    >(b"ab:cdfhH?klLmMnNqrS:tvVZ123456789\0")
};
static mut longopts: [option; 27] = [
    {
        let mut init = option {
            name: b"ascii\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"to-stdout\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"stdout\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"decompress\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"uncompress\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"force\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"keep\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'k' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"list\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"license\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'L' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-name\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"name\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'N' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"-presume-input-tty\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: PRESUME_INPUT_TTY_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"quiet\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"silent\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"synchronous\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: SYNCHRONOUS_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"recursive\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"suffix\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"test\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'V' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"fast\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: '1' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"best\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: '9' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"lzw\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'Z' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"bits\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"rsyncable\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: RSYNCABLE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
static mut work: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
> = unsafe {
    Some(zip as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int)
};
unsafe extern "C" fn try_help() {
    rpl_fprintf(
        stderr,
        b"Try `%s --help' for more information.\n\0" as *const u8 as *const libc::c_char,
        program_name,
    );
    do_exit(1 as libc::c_int);
}
unsafe extern "C" fn help() {
    static mut help_msg: [*const libc::c_char; 28] = [
        b"Compress or uncompress FILEs (by default, compress FILES in-place).\0"
            as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b"Mandatory arguments to long options are mandatory for short options too.\0"
            as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b"  -c, --stdout      write on standard output, keep original files unchanged\0"
            as *const u8 as *const libc::c_char,
        b"  -d, --decompress  decompress\0" as *const u8 as *const libc::c_char,
        b"  -f, --force       force overwrite of output file and compress links\0"
            as *const u8 as *const libc::c_char,
        b"  -h, --help        give this help\0" as *const u8 as *const libc::c_char,
        b"  -k, --keep        keep (don't delete) input files\0" as *const u8
            as *const libc::c_char,
        b"  -l, --list        list compressed file contents\0" as *const u8
            as *const libc::c_char,
        b"  -L, --license     display software license\0" as *const u8
            as *const libc::c_char,
        b"  -n, --no-name     do not save or restore the original name and timestamp\0"
            as *const u8 as *const libc::c_char,
        b"  -N, --name        save or restore the original name and timestamp\0"
            as *const u8 as *const libc::c_char,
        b"  -q, --quiet       suppress all warnings\0" as *const u8
            as *const libc::c_char,
        b"  -r, --recursive   operate recursively on directories\0" as *const u8
            as *const libc::c_char,
        b"      --rsyncable   make rsync-friendly archive\0" as *const u8
            as *const libc::c_char,
        b"  -S, --suffix=SUF  use suffix SUF on compressed files\0" as *const u8
            as *const libc::c_char,
        b"      --synchronous synchronous output (safer if system crashes, but slower)\0"
            as *const u8 as *const libc::c_char,
        b"  -t, --test        test compressed file integrity\0" as *const u8
            as *const libc::c_char,
        b"  -v, --verbose     verbose mode\0" as *const u8 as *const libc::c_char,
        b"  -V, --version     display version number\0" as *const u8
            as *const libc::c_char,
        b"  -1, --fast        compress faster\0" as *const u8 as *const libc::c_char,
        b"  -9, --best        compress better\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b"With no FILE, or when FILE is -, read standard input.\0" as *const u8
            as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b"Report bugs to <bug-gzip@gnu.org>.\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut p: *const *const libc::c_char = help_msg.as_ptr();
    __printf__(
        b"Usage: %s [OPTION]... [FILE]...\n\0" as *const u8 as *const libc::c_char,
        program_name,
    );
    while !(*p).is_null() {
        let fresh0 = p;
        p = p.offset(1);
        __printf__(b"%s\n\0" as *const u8 as *const libc::c_char, *fresh0);
    }
}
unsafe extern "C" fn license() {
    let mut p: *const *const libc::c_char = license_msg.as_ptr();
    __printf__(b"%s %s\n\0" as *const u8 as *const libc::c_char, program_name, Version);
    while !(*p).is_null() {
        let fresh1 = p;
        p = p.offset(1);
        __printf__(b"%s\n\0" as *const u8 as *const libc::c_char, *fresh1);
    }
}
unsafe extern "C" fn version() {
    license();
    __printf__(b"\n\0" as *const u8 as *const libc::c_char);
    __printf__(b"Written by Jean-loup Gailly.\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn progerror(mut string: *const libc::c_char) {
    let mut e: libc::c_int = *__errno_location();
    rpl_fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program_name);
    *__errno_location() = e;
    perror(string);
    exit_code = 1 as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut file_count: libc::c_int = 0;
    let mut proglen: size_t = 0;
    let mut argv_copy: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut env_argc: libc::c_int = 0;
    let mut env_argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    program_name = gzip_base_name(*argv.offset(0 as libc::c_int as isize));
    proglen = strlen(program_name);
    if (4 as libc::c_int as libc::c_ulong) < proglen
        && strcmp(
            program_name.offset(proglen as isize).offset(-(4 as libc::c_int as isize)),
            b".exe\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        *program_name
            .offset(
                proglen.wrapping_sub(4 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
    }
    argv_copy = argv;
    env = add_envopt(
        &mut env_argc,
        &mut argv_copy,
        b"GZIP\0" as *const u8 as *const libc::c_char,
    );
    env_argv = if !env.is_null() { argv_copy } else { 0 as *mut *mut libc::c_char };
    z_suffix = b".gz\0" as *const u8 as *const libc::c_char;
    z_len = strlen(z_suffix);
    loop {
        let mut optc: libc::c_int = 0;
        let mut longind: libc::c_int = -(1 as libc::c_int);
        if !env_argv.is_null() {
            if !(*env_argv.offset(optind as isize)).is_null()
                && strcmp(
                    *env_argv.offset(optind as isize),
                    b"--\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                optc = ENV_OPTION as libc::c_int + '-' as i32;
            } else {
                optc = getopt_long(
                    env_argc,
                    env_argv,
                    shortopts.as_ptr(),
                    longopts.as_ptr(),
                    &mut longind,
                );
                if 0 as libc::c_int <= optc {
                    optc += ENV_OPTION as libc::c_int;
                } else {
                    if optind != env_argc {
                        rpl_fprintf(
                            stderr,
                            b"%s: %s: non-option in GZIP environment variable\n\0"
                                as *const u8 as *const libc::c_char,
                            program_name,
                            *env_argv.offset(optind as isize),
                        );
                        try_help();
                    }
                    if env_argc != 1 as libc::c_int && quiet == 0 {
                        rpl_fprintf(
                            stderr,
                            b"%s: warning: GZIP environment variable is deprecated; use an alias or script\n\0"
                                as *const u8 as *const libc::c_char,
                            program_name,
                        );
                    }
                    rpl_free(env_argv as *mut libc::c_void);
                    env_argv = 0 as *mut *mut libc::c_char;
                    optind = 1 as libc::c_int;
                    longind = -(1 as libc::c_int);
                }
            }
        }
        if env_argv.is_null() {
            optc = getopt_long(
                argc,
                argv,
                shortopts.as_ptr(),
                longopts.as_ptr(),
                &mut longind,
            );
        }
        if optc < 0 as libc::c_int {
            break;
        }
        let mut current_block_76: u64;
        match optc {
            97 => {
                ascii = 1 as libc::c_int;
                current_block_76 = 15864857819291709765;
            }
            98 => {
                maxbits = atoi(optarg);
                while *optarg != 0 {
                    if !('0' as i32 <= *optarg as libc::c_int
                        && *optarg as libc::c_int <= '9' as i32)
                    {
                        rpl_fprintf(
                            stderr,
                            b"%s: -b operand is not an integer\n\0" as *const u8
                                as *const libc::c_char,
                            program_name,
                        );
                        try_help();
                    }
                    optarg = optarg.offset(1);
                    optarg;
                }
                current_block_76 = 15864857819291709765;
            }
            99 => {
                to_stdout = 1 as libc::c_int;
                current_block_76 = 15864857819291709765;
            }
            100 => {
                decompress = 1 as libc::c_int;
                current_block_76 = 15864857819291709765;
            }
            102 => {
                force += 1;
                force;
                current_block_76 = 15864857819291709765;
            }
            104 | 72 => {
                help();
                finish_out();
                current_block_76 = 15864857819291709765;
            }
            107 => {
                keep = 1 as libc::c_int;
                current_block_76 = 15864857819291709765;
            }
            108 => {
                to_stdout = 1 as libc::c_int;
                test = to_stdout;
                decompress = test;
                list = decompress;
                current_block_76 = 15864857819291709765;
            }
            76 => {
                license();
                finish_out();
                current_block_76 = 15864857819291709765;
            }
            109 => {
                no_time = 1 as libc::c_int;
                current_block_76 = 15864857819291709765;
            }
            77 => {
                no_time = 0 as libc::c_int;
                current_block_76 = 15864857819291709765;
            }
            110 | 241 => {
                no_time = 1 as libc::c_int;
                no_name = no_time;
                current_block_76 = 15864857819291709765;
            }
            78 | 209 => {
                no_time = 0 as libc::c_int;
                no_name = no_time;
                current_block_76 = 15864857819291709765;
            }
            128 => {
                presume_input_tty = 1 as libc::c_int != 0;
                current_block_76 = 15864857819291709765;
            }
            113 | 244 => {
                quiet = 1 as libc::c_int;
                verbose = 0 as libc::c_int;
                current_block_76 = 15864857819291709765;
            }
            114 => {
                recursive = 1 as libc::c_int;
                current_block_76 = 15864857819291709765;
            }
            129 | 260 => {
                rsync = 1 as libc::c_int;
                current_block_76 = 15864857819291709765;
            }
            83 => {
                z_len = strlen(optarg);
                z_suffix = optarg;
                current_block_76 = 15864857819291709765;
            }
            130 => {
                synchronous = 1 as libc::c_int != 0;
                current_block_76 = 15864857819291709765;
            }
            116 => {
                to_stdout = 1 as libc::c_int;
                decompress = to_stdout;
                test = decompress;
                current_block_76 = 15864857819291709765;
            }
            118 | 249 => {
                verbose += 1;
                verbose;
                quiet = 0 as libc::c_int;
                current_block_76 = 15864857819291709765;
            }
            86 => {
                version();
                finish_out();
                current_block_76 = 15864857819291709765;
            }
            90 => {
                rpl_fprintf(
                    stderr,
                    b"%s: -Z not supported in this version\n\0" as *const u8
                        as *const libc::c_char,
                    program_name,
                );
                try_help();
                current_block_76 = 15864857819291709765;
            }
            180 | 181 | 182 | 183 | 184 | 185 | 186 | 187 | 188 => {
                optc -= ENV_OPTION as libc::c_int;
                current_block_76 = 9262209197394929236;
            }
            49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                current_block_76 = 9262209197394929236;
            }
            _ => {
                if ENV_OPTION as libc::c_int <= optc
                    && optc != ENV_OPTION as libc::c_int + '?' as i32
                {
                    rpl_fprintf(
                        stderr,
                        b"%s: \0" as *const u8 as *const libc::c_char,
                        program_name,
                    );
                    if longind < 0 as libc::c_int {
                        rpl_fprintf(
                            stderr,
                            b"-%c: \0" as *const u8 as *const libc::c_char,
                            optc - ENV_OPTION as libc::c_int,
                        );
                    } else {
                        rpl_fprintf(
                            stderr,
                            b"--%s: \0" as *const u8 as *const libc::c_char,
                            longopts[longind as usize].name,
                        );
                    }
                    rpl_fprintf(
                        stderr,
                        b"option not valid in GZIP environment variable\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                try_help();
                current_block_76 = 15864857819291709765;
            }
        }
        match current_block_76 {
            9262209197394929236 => {
                level = optc - '0' as i32;
            }
            _ => {}
        }
    }
    if no_time < 0 as libc::c_int {
        no_time = decompress;
    }
    if no_name < 0 as libc::c_int {
        no_name = decompress;
    }
    file_count = argc - optind;
    if ascii != 0 && quiet == 0 {
        rpl_fprintf(
            stderr,
            b"%s: option --ascii ignored on this system\n\0" as *const u8
                as *const libc::c_char,
            program_name,
        );
    }
    if z_len == 0 as libc::c_int as libc::c_ulong
        || z_len > 30 as libc::c_int as libc::c_ulong
    {
        rpl_fprintf(
            stderr,
            b"%s: invalid suffix '%s'\n\0" as *const u8 as *const libc::c_char,
            program_name,
            z_suffix,
        );
        do_exit(1 as libc::c_int);
    }
    ::std::ptr::write_volatile(
        &mut exiting_signal as *mut libc::c_int,
        if quiet != 0 { 13 as libc::c_int } else { 0 as libc::c_int },
    );
    install_signal_handlers();
    if file_count != 0 as libc::c_int {
        to_stdout != 0 && test == 0 && (decompress == 0 || ascii == 0);
        while optind < argc {
            let fresh2 = optind;
            optind = optind + 1;
            treat_file(*argv.offset(fresh2 as isize));
        }
    } else {
        treat_stdin();
    }
    if stdin_was_read as libc::c_int != 0 && close(0 as libc::c_int) != 0 as libc::c_int
    {
        strcpy(ifname.as_mut_ptr(), b"stdin\0" as *const u8 as *const libc::c_char);
        read_error();
    }
    if list != 0 {
        if quiet == 0 && (1 as libc::c_int) < file_count {
            do_list(-(1 as libc::c_int));
        }
        if rpl_fflush(stdout) != 0 as libc::c_int {
            write_error();
        }
    }
    if to_stdout != 0
        && (synchronous as libc::c_int != 0
            && fdatasync(1 as libc::c_int) != 0 as libc::c_int
            && *__errno_location() != 22 as libc::c_int
            || close(1 as libc::c_int) != 0 as libc::c_int)
        && *__errno_location() != 9 as libc::c_int
    {
        write_error();
    }
    do_exit(exit_code);
    return 0;
}
unsafe extern "C" fn input_eof() -> libc::c_int {
    if decompress == 0 || last_member != 0 {
        return 1 as libc::c_int;
    }
    if inptr == insize {
        if insize != 0x40000 as libc::c_int as libc::c_uint
            || fill_inbuf(1 as libc::c_int) == -(1 as libc::c_int)
        {
            return 1 as libc::c_int;
        }
        inptr = 0 as libc::c_int as libc::c_uint;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_input_size_and_time() {
    ifile_size = -(1 as libc::c_int) as off_t;
    time_stamp.tv_nsec = -(1 as libc::c_int) as __syscall_slong_t;
    if istat.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
    {
        ifile_size = istat.st_size;
        if no_time == 0 || list != 0 {
            time_stamp = get_stat_mtime(&mut istat);
        }
    }
}
unsafe extern "C" fn treat_stdin() {
    if force == 0 && list == 0
        && (presume_input_tty as libc::c_int != 0
            || isatty(
                (if decompress != 0 { 0 as libc::c_int } else { 1 as libc::c_int }),
            ) != 0)
    {
        if quiet == 0 {
            rpl_fprintf(
                stderr,
                b"%s: compressed data not %s a terminal. Use -f to force %scompression.\nFor help, type: %s -h\n\0"
                    as *const u8 as *const libc::c_char,
                program_name,
                if decompress != 0 {
                    b"read from\0" as *const u8 as *const libc::c_char
                } else {
                    b"written to\0" as *const u8 as *const libc::c_char
                },
                if decompress != 0 {
                    b"de\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                program_name,
            );
        }
        do_exit(1 as libc::c_int);
    }
    decompress != 0 || ascii == 0;
    test == 0 && (decompress == 0 || ascii == 0);
    strcpy(ifname.as_mut_ptr(), b"stdin\0" as *const u8 as *const libc::c_char);
    strcpy(ofname.as_mut_ptr(), b"stdout\0" as *const u8 as *const libc::c_char);
    if fstat(0 as libc::c_int, &mut istat) != 0 as libc::c_int {
        progerror(b"standard input\0" as *const u8 as *const libc::c_char);
        do_exit(1 as libc::c_int);
    }
    get_input_size_and_time();
    clear_bufs();
    to_stdout = 1 as libc::c_int;
    part_nb = 0 as libc::c_int;
    ifd = 0 as libc::c_int;
    stdin_was_read = 1 as libc::c_int != 0;
    if decompress != 0 {
        method = get_method(ifd);
        if method < 0 as libc::c_int {
            do_exit(exit_code);
        }
    }
    loop {
        if work.unwrap()(0 as libc::c_int, 1 as libc::c_int) != 0 as libc::c_int {
            return;
        }
        if input_eof() != 0 {
            break;
        }
        method = get_method(ifd);
        if method < 0 as libc::c_int {
            return;
        }
        bytes_out = 0 as libc::c_int as off_t;
    }
    if list != 0 {
        do_list(method);
        return;
    }
    if verbose != 0 {
        if test != 0 {
            rpl_fprintf(stderr, b" OK\n\0" as *const u8 as *const libc::c_char);
        } else if decompress == 0 {
            display_ratio(bytes_in - (bytes_out - header_bytes), bytes_in, stderr);
            rpl_fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        }
    }
}
static mut dot: libc::c_char = '.' as i32 as libc::c_char;
unsafe extern "C" fn atdir_eq(
    mut dir: *const libc::c_char,
    mut dirlen: ptrdiff_t,
) -> bool {
    if dirlen == 0 as libc::c_int as libc::c_long {
        dir = &dot;
        dirlen = 1 as libc::c_int as ptrdiff_t;
    }
    return memcmp(
        dfname.as_mut_ptr() as *const libc::c_void,
        dir as *const libc::c_void,
        dirlen as libc::c_ulong,
    ) == 0 as libc::c_int && dfname[dirlen as usize] == 0;
}
unsafe extern "C" fn atdir_set(
    mut dir: *const libc::c_char,
    mut dirlen: ptrdiff_t,
) -> libc::c_int {
    if try_opening_directories as libc::c_int != 0 && !atdir_eq(dir, dirlen) {
        if 0 as libc::c_int <= dfd {
            close(dfd);
        }
        if dirlen == 0 as libc::c_int as libc::c_long {
            dir = &dot;
            dirlen = 1 as libc::c_int as ptrdiff_t;
        }
        memcpy(
            dfname.as_mut_ptr() as *mut libc::c_void,
            dir as *const libc::c_void,
            dirlen as libc::c_ulong,
        );
        dfname[dirlen as usize] = '\0' as i32 as libc::c_char;
        dfd = open_safer(
            dfname.as_mut_ptr(),
            0 as libc::c_int | 0o200000 as libc::c_int,
        );
    }
    return dfd;
}
unsafe extern "C" fn treat_file(mut iname: *mut libc::c_char) {
    if strcmp(iname, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        let mut cflag: libc::c_int = to_stdout;
        treat_stdin();
        to_stdout = cflag;
        return;
    }
    ifd = open_input_file(iname, &mut istat);
    if ifd < 0 as libc::c_int {
        return;
    }
    if istat.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint
    {
        if recursive != 0 {
            treat_dir(ifd, iname);
            return;
        }
        close(ifd);
        if quiet == 0 {
            rpl_fprintf(
                stderr,
                b"%s: %s is a directory -- ignored\n\0" as *const u8
                    as *const libc::c_char,
                program_name,
                ifname.as_mut_ptr(),
            );
        }
        if exit_code == 0 as libc::c_int {
            exit_code = 2 as libc::c_int;
        }
        return;
    }
    if to_stdout == 0 {
        if !(istat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint)
        {
            if quiet == 0 {
                rpl_fprintf(
                    stderr,
                    b"%s: %s is not a directory or a regular file - ignored\n\0"
                        as *const u8 as *const libc::c_char,
                    program_name,
                    ifname.as_mut_ptr(),
                );
            }
            if exit_code == 0 as libc::c_int {
                exit_code = 2 as libc::c_int;
            }
            close(ifd);
            return;
        }
        if istat.st_mode & 0o4000 as libc::c_int as libc::c_uint != 0 {
            if quiet == 0 {
                rpl_fprintf(
                    stderr,
                    b"%s: %s is set-user-ID on execution - ignored\n\0" as *const u8
                        as *const libc::c_char,
                    program_name,
                    ifname.as_mut_ptr(),
                );
            }
            if exit_code == 0 as libc::c_int {
                exit_code = 2 as libc::c_int;
            }
            close(ifd);
            return;
        }
        if istat.st_mode & 0o2000 as libc::c_int as libc::c_uint != 0 {
            if quiet == 0 {
                rpl_fprintf(
                    stderr,
                    b"%s: %s is set-group-ID on execution - ignored\n\0" as *const u8
                        as *const libc::c_char,
                    program_name,
                    ifname.as_mut_ptr(),
                );
            }
            if exit_code == 0 as libc::c_int {
                exit_code = 2 as libc::c_int;
            }
            close(ifd);
            return;
        }
        if force == 0 {
            if istat.st_mode & 0o1000 as libc::c_int as libc::c_uint != 0 {
                if quiet == 0 {
                    rpl_fprintf(
                        stderr,
                        b"%s: %s has the sticky bit set - file ignored\n\0" as *const u8
                            as *const libc::c_char,
                        program_name,
                        ifname.as_mut_ptr(),
                    );
                }
                if exit_code == 0 as libc::c_int {
                    exit_code = 2 as libc::c_int;
                }
                close(ifd);
                return;
            }
            if 2 as libc::c_int as libc::c_ulong <= istat.st_nlink {
                if quiet == 0 {
                    rpl_fprintf(
                        stderr,
                        b"%s: %s has %lu other link%s -- file ignored\n\0" as *const u8
                            as *const libc::c_char,
                        program_name,
                        ifname.as_mut_ptr(),
                        (istat.st_nlink).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        if istat.st_nlink == 2 as libc::c_int as libc::c_ulong {
                            b"\0" as *const u8 as *const libc::c_char
                        } else {
                            b"s\0" as *const u8 as *const libc::c_char
                        },
                    );
                }
                if exit_code == 0 as libc::c_int {
                    exit_code = 2 as libc::c_int;
                }
                close(ifd);
                return;
            }
        }
    }
    get_input_size_and_time();
    if to_stdout != 0 && test == 0 {
        strcpy(ofname.as_mut_ptr(), b"stdout\0" as *const u8 as *const libc::c_char);
    } else if make_ofname() != 0 as libc::c_int {
        close(ifd);
        return;
    }
    clear_bufs();
    part_nb = 0 as libc::c_int;
    if decompress != 0 {
        method = get_method(ifd);
        if method < 0 as libc::c_int {
            close(ifd);
            return;
        }
    }
    if to_stdout != 0 {
        ofd = 1 as libc::c_int;
    } else {
        if create_outfile() != 0 as libc::c_int {
            return;
        }
        if decompress == 0 && save_orig_name != 0 && verbose == 0 && quiet == 0 {
            rpl_fprintf(
                stderr,
                b"%s: %s compressed to %s\n\0" as *const u8 as *const libc::c_char,
                program_name,
                ifname.as_mut_ptr(),
                ofname.as_mut_ptr(),
            );
        }
    }
    if save_orig_name == 0 {
        save_orig_name = (no_name == 0) as libc::c_int;
    }
    if verbose != 0 && list == 0 {
        rpl_fprintf(
            stderr,
            b"%s:\t\0" as *const u8 as *const libc::c_char,
            ifname.as_mut_ptr(),
        );
    }
    loop {
        if (Some(work.unwrap())).unwrap()(ifd, ofd) != 0 as libc::c_int {
            method = -(1 as libc::c_int);
            break;
        } else {
            if input_eof() != 0 {
                break;
            }
            method = get_method(ifd);
            if method < 0 as libc::c_int {
                break;
            }
            bytes_out = 0 as libc::c_int as off_t;
        }
    }
    if close(ifd) != 0 as libc::c_int {
        read_error();
    }
    if list != 0 {
        do_list(method);
        return;
    }
    if to_stdout == 0 {
        copy_stat(&mut istat);
        if synchronous as libc::c_int != 0
            && (0 as libc::c_int <= dfd && fdatasync(dfd) != 0 as libc::c_int
                && *__errno_location() != 22 as libc::c_int
                || fsync(ofd) != 0 as libc::c_int
                    && *__errno_location() != 22 as libc::c_int)
            || close(ofd) != 0 as libc::c_int
        {
            write_error();
        }
        if keep == 0 {
            let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
            let mut unlink_errno: libc::c_int = 0;
            let mut ifbase: *mut libc::c_char = last_component(ifname.as_mut_ptr());
            let mut ufd: libc::c_int = if atdir_eq(
                ifname.as_mut_ptr(),
                ifbase.offset_from(ifname.as_mut_ptr()) as libc::c_long,
            ) as libc::c_int != 0
            {
                dfd
            } else {
                -(1 as libc::c_int)
            };
            let mut res: libc::c_int = 0;
            sigprocmask(0 as libc::c_int, &mut caught_signals, &mut oldset);
            ::std::ptr::write_volatile(
                &mut remove_ofname_fd as *mut libc::c_int,
                -(1 as libc::c_int),
            );
            res = if ufd < 0 as libc::c_int {
                xunlink(ifname.as_mut_ptr())
            } else {
                unlinkat(ufd, ifbase, 0 as libc::c_int)
            };
            unlink_errno = if res == 0 as libc::c_int {
                0 as libc::c_int
            } else {
                *__errno_location()
            };
            sigprocmask(2 as libc::c_int, &mut oldset, 0 as *mut sigset_t);
            if unlink_errno != 0 {
                if quiet == 0 {
                    rpl_fprintf(
                        stderr,
                        b"%s: \0" as *const u8 as *const libc::c_char,
                        program_name,
                    );
                }
                if exit_code == 0 as libc::c_int {
                    exit_code = 2 as libc::c_int;
                }
                if quiet == 0 {
                    *__errno_location() = unlink_errno;
                    perror(ifname.as_mut_ptr());
                }
            }
        }
    }
    if method == -(1 as libc::c_int) {
        if to_stdout == 0 {
            remove_output_file(0 as libc::c_int != 0);
        }
        return;
    }
    if verbose != 0 {
        if test != 0 {
            rpl_fprintf(stderr, b" OK\0" as *const u8 as *const libc::c_char);
        } else if decompress != 0 {
            display_ratio(bytes_out - (bytes_in - header_bytes), bytes_out, stderr);
        } else {
            display_ratio(bytes_in - (bytes_out - header_bytes), bytes_in, stderr);
        }
        if test == 0 {
            rpl_fprintf(
                stderr,
                b" -- %s %s\0" as *const u8 as *const libc::c_char,
                if keep != 0 {
                    b"created\0" as *const u8 as *const libc::c_char
                } else {
                    b"replaced with\0" as *const u8 as *const libc::c_char
                },
                ofname.as_mut_ptr(),
            );
        }
        rpl_fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn volatile_strcpy(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
) {
    loop {
        let fresh3 = src;
        src = src.offset(1);
        let fresh4 = dst;
        dst = dst.offset(1);
        ::std::ptr::write_volatile(fresh4, *fresh3);
        if !(::std::ptr::read_volatile::<libc::c_char>(fresh4 as *const libc::c_char)
            != 0)
        {
            break;
        }
    };
}
unsafe extern "C" fn create_outfile() -> libc::c_int {
    let mut name_shortened: libc::c_int = 0 as libc::c_int;
    let mut flags: libc::c_int = 0o1 as libc::c_int | 0o100 as libc::c_int
        | 0o200 as libc::c_int
        | (if ascii != 0 && decompress != 0 {
            0 as libc::c_int
        } else {
            0 as libc::c_int
        });
    let mut base: *const libc::c_char = ofname.as_mut_ptr();
    let mut atfd: libc::c_int = -(100 as libc::c_int);
    if keep == 0 {
        let mut b: *const libc::c_char = last_component(ofname.as_mut_ptr());
        let mut f: libc::c_int = atdir_set(
            ofname.as_mut_ptr(),
            b.offset_from(ofname.as_mut_ptr()) as libc::c_long,
        );
        if 0 as libc::c_int <= f {
            base = b;
            atfd = f;
        }
    }
    loop {
        let mut open_errno: libc::c_int = 0;
        let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
        volatile_strcpy(remove_ofname.as_mut_ptr(), ofname.as_mut_ptr());
        sigprocmask(0 as libc::c_int, &mut caught_signals, &mut oldset);
        ofd = openat_safer(
            atfd,
            base,
            flags,
            0o400 as libc::c_int | 0o200 as libc::c_int,
        );
        ::std::ptr::write_volatile(&mut remove_ofname_fd as *mut libc::c_int, ofd);
        open_errno = *__errno_location();
        sigprocmask(2 as libc::c_int, &mut oldset, 0 as *mut sigset_t);
        if 0 as libc::c_int <= ofd {
            break;
        }
        match open_errno {
            36 => {
                shorten_name(ofname.as_mut_ptr());
                name_shortened = 1 as libc::c_int;
            }
            17 => {
                if check_ofname() != 0 as libc::c_int {
                    close(ifd);
                    return 1 as libc::c_int;
                }
            }
            _ => {
                progerror(ofname.as_mut_ptr());
                close(ifd);
                return 1 as libc::c_int;
            }
        }
    }
    if name_shortened != 0 && decompress != 0 {
        if quiet == 0 {
            rpl_fprintf(
                stderr,
                b"%s: %s: warning, name truncated\n\0" as *const u8
                    as *const libc::c_char,
                program_name,
                ofname.as_mut_ptr(),
            );
        }
        if exit_code == 0 as libc::c_int {
            exit_code = 2 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_suffix(mut name: *mut libc::c_char) -> *mut libc::c_char {
    let mut nlen: libc::c_int = 0;
    let mut slen: libc::c_int = 0;
    let mut suffix: [libc::c_char; 33] = [0; 33];
    static mut known_suffixes: [*const libc::c_char; 10] = [
        0 as *const libc::c_char,
        b".gz\0" as *const u8 as *const libc::c_char,
        b".z\0" as *const u8 as *const libc::c_char,
        b".taz\0" as *const u8 as *const libc::c_char,
        b".tgz\0" as *const u8 as *const libc::c_char,
        b"-gz\0" as *const u8 as *const libc::c_char,
        b"-z\0" as *const u8 as *const libc::c_char,
        b"_z\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut suf: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut suffix_of_builtin: bool = 0 as libc::c_int != 0;
    suf = known_suffixes.as_mut_ptr().offset(1 as libc::c_int as isize);
    while !(*suf).is_null() {
        let mut suflen: size_t = strlen(*suf);
        if z_len < suflen
            && strcmp(z_suffix, (*suf).offset(suflen as isize).offset(-(z_len as isize)))
                == 0 as libc::c_int
        {
            suffix_of_builtin = 1 as libc::c_int != 0;
            break;
        } else {
            suf = suf.offset(1);
            suf;
        }
    }
    let mut z_lower: *mut libc::c_char = xstrdup(z_suffix);
    strlwr(z_lower);
    known_suffixes[(if suffix_of_builtin as libc::c_int != 0 {
        (::std::mem::size_of::<[*const libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
    } else {
        0 as libc::c_int as libc::c_ulong
    }) as usize] = z_lower;
    suf = known_suffixes.as_mut_ptr().offset(suffix_of_builtin as libc::c_int as isize);
    nlen = strlen(name) as libc::c_int;
    if nlen <= 30 as libc::c_int + 2 as libc::c_int {
        strcpy(suffix.as_mut_ptr(), name);
    } else {
        strcpy(
            suffix.as_mut_ptr(),
            name
                .offset(nlen as isize)
                .offset(-(30 as libc::c_int as isize))
                .offset(-(2 as libc::c_int as isize)),
        );
    }
    strlwr(suffix.as_mut_ptr());
    slen = strlen(suffix.as_mut_ptr()) as libc::c_int;
    let mut match_0: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        let mut s: libc::c_int = strlen(*suf) as libc::c_int;
        if slen > s
            && !(suffix[(slen - s - 1 as libc::c_int) as usize] as libc::c_int
                == '/' as i32)
            && strcmp(
                suffix.as_mut_ptr().offset(slen as isize).offset(-(s as isize)),
                *suf,
            ) == 0 as libc::c_int
        {
            match_0 = name.offset(nlen as isize).offset(-(s as isize));
            break;
        } else {
            suf = suf.offset(1);
            if (*suf).is_null() {
                break;
            }
        }
    }
    rpl_free(z_lower as *mut libc::c_void);
    return match_0;
}
unsafe extern "C" fn open_and_stat(
    mut name: *mut libc::c_char,
    mut flags: libc::c_int,
    mut st: *mut stat,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut atfd: libc::c_int = -(100 as libc::c_int);
    let mut base: *const libc::c_char = name;
    if to_stdout == 0 && force == 0 {
        flags |= 0o400000 as libc::c_int;
    }
    if keep == 0 {
        let mut b: *const libc::c_char = last_component(name);
        let mut f: libc::c_int = atdir_set(name, b.offset_from(name) as libc::c_long);
        if 0 as libc::c_int <= f {
            base = b;
            atfd = f;
        }
    }
    fd = openat_safer(atfd, base, flags);
    if 0 as libc::c_int <= fd && fstat(fd, st) != 0 as libc::c_int {
        let mut e: libc::c_int = *__errno_location();
        close(fd);
        *__errno_location() = e;
        return -(1 as libc::c_int);
    }
    return fd;
}
unsafe extern "C" fn open_input_file(
    mut iname: *mut libc::c_char,
    mut sbuf: *mut stat,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ilen: libc::c_int = 0;
    let mut z_suffix_errno: libc::c_int = 0 as libc::c_int;
    static mut suffixes: [*const libc::c_char; 6] = [
        0 as *const libc::c_char,
        b".gz\0" as *const u8 as *const libc::c_char,
        b".z\0" as *const u8 as *const libc::c_char,
        b"-z\0" as *const u8 as *const libc::c_char,
        b".Z\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut suf: *mut *const libc::c_char = suffixes.as_mut_ptr();
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut fd: libc::c_int = 0;
    let mut open_flags: libc::c_int = 0 as libc::c_int | 0o4000 as libc::c_int
        | 0o400 as libc::c_int
        | (if ascii != 0 && decompress == 0 {
            0 as libc::c_int
        } else {
            0 as libc::c_int
        });
    *suf = z_suffix;
    if !((::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) <= strlen(iname))
    {
        strcpy(ifname.as_mut_ptr(), iname);
        fd = open_and_stat(ifname.as_mut_ptr(), open_flags, sbuf);
        if 0 as libc::c_int <= fd {
            return fd;
        }
        if decompress == 0 || *__errno_location() != 2 as libc::c_int {
            progerror(ifname.as_mut_ptr());
            return -(1 as libc::c_int);
        }
        s = get_suffix(ifname.as_mut_ptr());
        if !s.is_null() {
            progerror(ifname.as_mut_ptr());
            return -(1 as libc::c_int);
        }
        ilen = strlen(ifname.as_mut_ptr()) as libc::c_int;
        if strcmp(z_suffix, b".gz\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            suf = suf.offset(1);
            suf;
        }
        loop {
            s = *suf;
            let mut s0: *const libc::c_char = s;
            strcpy(ifname.as_mut_ptr(), iname);
            if ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                <= (ilen as libc::c_ulong).wrapping_add(strlen(s))
            {
                current_block = 5834018566588557122;
                break;
            }
            strcat(ifname.as_mut_ptr(), s);
            fd = open_and_stat(ifname.as_mut_ptr(), open_flags, sbuf);
            if 0 as libc::c_int <= fd {
                return fd;
            }
            if *__errno_location() != 2 as libc::c_int {
                progerror(ifname.as_mut_ptr());
                return -(1 as libc::c_int);
            }
            if strcmp(s0, z_suffix) == 0 as libc::c_int {
                z_suffix_errno = *__errno_location();
            }
            suf = suf.offset(1);
            if (*suf).is_null() {
                current_block = 9828876828309294594;
                break;
            }
        }
        match current_block {
            5834018566588557122 => {}
            _ => {
                strcpy(ifname.as_mut_ptr(), iname);
                strcat(ifname.as_mut_ptr(), z_suffix);
                *__errno_location() = z_suffix_errno;
                progerror(ifname.as_mut_ptr());
                return -(1 as libc::c_int);
            }
        }
    }
    rpl_fprintf(
        stderr,
        b"%s: %s: file name too long\n\0" as *const u8 as *const libc::c_char,
        program_name,
        iname,
    );
    exit_code = 1 as libc::c_int;
    return -(1 as libc::c_int);
}
unsafe extern "C" fn make_ofname() -> libc::c_int {
    let mut suff: *mut libc::c_char = 0 as *mut libc::c_char;
    strcpy(ofname.as_mut_ptr(), ifname.as_mut_ptr());
    suff = get_suffix(ofname.as_mut_ptr());
    if decompress != 0 {
        if suff.is_null() {
            if recursive == 0 && test != 0 {
                return 0 as libc::c_int;
            }
            if verbose != 0 || recursive == 0 && quiet == 0 {
                if quiet == 0 {
                    rpl_fprintf(
                        stderr,
                        b"%s: %s: unknown suffix -- ignored\n\0" as *const u8
                            as *const libc::c_char,
                        program_name,
                        ifname.as_mut_ptr(),
                    );
                }
                if exit_code == 0 as libc::c_int {
                    exit_code = 2 as libc::c_int;
                }
            }
            return 2 as libc::c_int;
        }
        strlwr(suff);
        if strcmp(suff, b".tgz\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(suff, b".taz\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            strcpy(suff, b".tar\0" as *const u8 as *const libc::c_char);
        } else {
            *suff = '\0' as i32 as libc::c_char;
        }
    } else if !suff.is_null() && force == 0 {
        if verbose != 0 || recursive == 0 && quiet == 0 {
            rpl_fprintf(
                stderr,
                b"%s: %s already has %s suffix -- unchanged\n\0" as *const u8
                    as *const libc::c_char,
                program_name,
                ifname.as_mut_ptr(),
                suff,
            );
        }
        return 2 as libc::c_int;
    } else {
        save_orig_name = 0 as libc::c_int;
        if ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
            <= (strlen(ofname.as_mut_ptr())).wrapping_add(z_len)
        {
            if quiet == 0 {
                rpl_fprintf(
                    stderr,
                    b"%s: %s: file name too long\n\0" as *const u8
                        as *const libc::c_char,
                    program_name,
                    ifname.as_mut_ptr(),
                );
            }
            if exit_code == 0 as libc::c_int {
                exit_code = 2 as libc::c_int;
            }
            return 2 as libc::c_int;
        } else {
            strcat(ofname.as_mut_ptr(), z_suffix);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn discard_input_bytes(mut nbytes: size_t, mut flags: libc::c_uint) {
    while nbytes != 0 as libc::c_int as libc::c_ulong {
        let mut c: uch = (if inptr < insize {
            let fresh5 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf[fresh5 as usize] as libc::c_int
        } else {
            fill_inbuf(0 as libc::c_int)
        }) as uch;
        if flags & 0x2 as libc::c_int as libc::c_uint != 0 {
            updcrc(&mut c, 1 as libc::c_int as libc::c_uint);
        }
        if nbytes != -(1 as libc::c_int) as size_t {
            nbytes = nbytes.wrapping_sub(1);
            nbytes;
        } else if c == 0 {
            break;
        }
    }
}
unsafe extern "C" fn get_method(mut in_0: libc::c_int) -> libc::c_int {
    let mut flags: uch = 0;
    let mut magic: [uch; 10] = [0; 10];
    let mut imagic0: libc::c_int = 0;
    let mut imagic1: libc::c_int = 0;
    let mut stamp: ulg = 0;
    if force != 0 && to_stdout != 0 {
        imagic0 = if inptr < insize {
            let fresh6 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf[fresh6 as usize] as libc::c_int
        } else {
            fill_inbuf(1 as libc::c_int)
        };
        magic[0 as libc::c_int as usize] = imagic0 as uch;
        imagic1 = if inptr < insize {
            let fresh7 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf[fresh7 as usize] as libc::c_int
        } else {
            fill_inbuf(1 as libc::c_int)
        };
        magic[1 as libc::c_int as usize] = imagic1 as uch;
    } else {
        magic[0 as libc::c_int
            as usize] = (if inptr < insize {
            let fresh8 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf[fresh8 as usize] as libc::c_int
        } else {
            fill_inbuf(0 as libc::c_int)
        }) as uch;
        imagic0 = 0 as libc::c_int;
        if magic[0 as libc::c_int as usize] != 0 {
            magic[1 as libc::c_int
                as usize] = (if inptr < insize {
                let fresh9 = inptr;
                inptr = inptr.wrapping_add(1);
                inbuf[fresh9 as usize] as libc::c_int
            } else {
                fill_inbuf(0 as libc::c_int)
            }) as uch;
            imagic1 = 0 as libc::c_int;
        } else {
            imagic1 = if inptr < insize {
                let fresh10 = inptr;
                inptr = inptr.wrapping_add(1);
                inbuf[fresh10 as usize] as libc::c_int
            } else {
                fill_inbuf(1 as libc::c_int)
            };
            magic[1 as libc::c_int as usize] = imagic1 as uch;
        }
    }
    method = -(1 as libc::c_int);
    part_nb += 1;
    part_nb;
    header_bytes = 0 as libc::c_int as off_t;
    last_member = 0 as libc::c_int;
    if memcmp(
        magic.as_mut_ptr() as *const libc::c_void,
        b"\x1F\x8B\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        || memcmp(
            magic.as_mut_ptr() as *const libc::c_void,
            b"\x1F\x9E\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        method = if inptr < insize {
            let fresh11 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf[fresh11 as usize] as libc::c_int
        } else {
            fill_inbuf(0 as libc::c_int)
        };
        if method != 8 as libc::c_int {
            rpl_fprintf(
                stderr,
                b"%s: %s: unknown method %d -- not supported\n\0" as *const u8
                    as *const libc::c_char,
                program_name,
                ifname.as_mut_ptr(),
                method,
            );
            exit_code = 1 as libc::c_int;
            return -(1 as libc::c_int);
        }
        work = Some(
            unzip as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
        );
        flags = (if inptr < insize {
            let fresh12 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf[fresh12 as usize] as libc::c_int
        } else {
            fill_inbuf(0 as libc::c_int)
        }) as uch;
        if flags as libc::c_int & 0x20 as libc::c_int != 0 as libc::c_int {
            rpl_fprintf(
                stderr,
                b"%s: %s is encrypted -- not supported\n\0" as *const u8
                    as *const libc::c_char,
                program_name,
                ifname.as_mut_ptr(),
            );
            exit_code = 1 as libc::c_int;
            return -(1 as libc::c_int);
        }
        if flags as libc::c_int & 0xc0 as libc::c_int != 0 as libc::c_int {
            rpl_fprintf(
                stderr,
                b"%s: %s has flags 0x%x -- not supported\n\0" as *const u8
                    as *const libc::c_char,
                program_name,
                ifname.as_mut_ptr(),
                flags as libc::c_int,
            );
            exit_code = 1 as libc::c_int;
            if force <= 1 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
        stamp = (if inptr < insize {
            let fresh13 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf[fresh13 as usize] as libc::c_int
        } else {
            fill_inbuf(0 as libc::c_int)
        }) as ulg;
        stamp
            |= ((if inptr < insize {
                let fresh14 = inptr;
                inptr = inptr.wrapping_add(1);
                inbuf[fresh14 as usize] as libc::c_int
            } else {
                fill_inbuf(0 as libc::c_int)
            }) as ulg) << 8 as libc::c_int;
        stamp
            |= ((if inptr < insize {
                let fresh15 = inptr;
                inptr = inptr.wrapping_add(1);
                inbuf[fresh15 as usize] as libc::c_int
            } else {
                fill_inbuf(0 as libc::c_int)
            }) as ulg) << 16 as libc::c_int;
        stamp
            |= ((if inptr < insize {
                let fresh16 = inptr;
                inptr = inptr.wrapping_add(1);
                inbuf[fresh16 as usize] as libc::c_int
            } else {
                fill_inbuf(0 as libc::c_int)
            }) as ulg) << 24 as libc::c_int;
        if stamp != 0 as libc::c_int as libc::c_ulong && no_time == 0 {
            if stamp
                <= (if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
                    -(1 as libc::c_int) as time_t
                } else {
                    (((1 as libc::c_int as time_t)
                        << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                }) as libc::c_ulong
            {
                time_stamp.tv_sec = stamp as __time_t;
                time_stamp.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
            } else {
                if quiet == 0 {
                    rpl_fprintf(
                        stderr,
                        b"%s: %s: MTIME %lu out of range for this platform\n\0"
                            as *const u8 as *const libc::c_char,
                        program_name,
                        ifname.as_mut_ptr(),
                        stamp,
                    );
                }
                if exit_code == 0 as libc::c_int {
                    exit_code = 2 as libc::c_int;
                }
                time_stamp
                    .tv_sec = if (0 as libc::c_int as time_t)
                    < -(1 as libc::c_int) as time_t
                {
                    -(1 as libc::c_int) as time_t
                } else {
                    (((1 as libc::c_int as time_t)
                        << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                };
                time_stamp
                    .tv_nsec = (TIMESPEC_RESOLUTION as libc::c_int - 1 as libc::c_int)
                    as __syscall_slong_t;
            }
        }
        magic[8 as libc::c_int
            as usize] = (if inptr < insize {
            let fresh17 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf[fresh17 as usize] as libc::c_int
        } else {
            fill_inbuf(0 as libc::c_int)
        }) as uch;
        magic[9 as libc::c_int
            as usize] = (if inptr < insize {
            let fresh18 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf[fresh18 as usize] as libc::c_int
        } else {
            fill_inbuf(0 as libc::c_int)
        }) as uch;
        if flags as libc::c_int & 0x2 as libc::c_int != 0 {
            magic[2 as libc::c_int as usize] = 8 as libc::c_int as uch;
            magic[3 as libc::c_int as usize] = flags;
            magic[4 as libc::c_int
                as usize] = (stamp & 0xff as libc::c_int as libc::c_ulong) as uch;
            magic[5 as libc::c_int
                as usize] = (stamp >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_ulong) as uch;
            magic[6 as libc::c_int
                as usize] = (stamp >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_ulong) as uch;
            magic[7 as libc::c_int as usize] = (stamp >> 24 as libc::c_int) as uch;
            updcrc(0 as *const uch, 0 as libc::c_int as libc::c_uint);
            updcrc(magic.as_mut_ptr(), 10 as libc::c_int as libc::c_uint);
        }
        if flags as libc::c_int & 0x4 as libc::c_int != 0 as libc::c_int {
            let mut lenbuf: [uch; 2] = [0; 2];
            lenbuf[0 as libc::c_int
                as usize] = (if inptr < insize {
                let fresh19 = inptr;
                inptr = inptr.wrapping_add(1);
                inbuf[fresh19 as usize] as libc::c_int
            } else {
                fill_inbuf(0 as libc::c_int)
            }) as uch;
            let mut len: libc::c_uint = lenbuf[0 as libc::c_int as usize]
                as libc::c_uint;
            lenbuf[1 as libc::c_int
                as usize] = (if inptr < insize {
                let fresh20 = inptr;
                inptr = inptr.wrapping_add(1);
                inbuf[fresh20 as usize] as libc::c_int
            } else {
                fill_inbuf(0 as libc::c_int)
            }) as uch;
            len
                |= ((lenbuf[1 as libc::c_int as usize] as libc::c_int)
                    << 8 as libc::c_int) as libc::c_uint;
            if verbose != 0 {
                rpl_fprintf(
                    stderr,
                    b"%s: %s: extra field of %u bytes ignored\n\0" as *const u8
                        as *const libc::c_char,
                    program_name,
                    ifname.as_mut_ptr(),
                    len,
                );
            }
            if flags as libc::c_int & 0x2 as libc::c_int != 0 {
                updcrc(lenbuf.as_mut_ptr(), 2 as libc::c_int as libc::c_uint);
            }
            discard_input_bytes(len as size_t, flags as libc::c_uint);
        }
        if flags as libc::c_int & 0x8 as libc::c_int != 0 as libc::c_int {
            if no_name != 0 || to_stdout != 0 && list == 0 || part_nb > 1 as libc::c_int
            {
                discard_input_bytes(
                    -(1 as libc::c_int) as size_t,
                    flags as libc::c_uint,
                );
            } else {
                let mut p: *mut libc::c_char = gzip_base_name(ofname.as_mut_ptr());
                let mut base: *mut libc::c_char = p;
                loop {
                    *p = (if inptr < insize {
                        let fresh21 = inptr;
                        inptr = inptr.wrapping_add(1);
                        inbuf[fresh21 as usize] as libc::c_int
                    } else {
                        fill_inbuf(0 as libc::c_int)
                    }) as libc::c_char;
                    let fresh22 = p;
                    p = p.offset(1);
                    if *fresh22 as libc::c_int == '\0' as i32 {
                        break;
                    }
                    if p
                        >= ofname
                            .as_mut_ptr()
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong as isize,
                            )
                    {
                        gzip_error(
                            b"corrupted input -- file name too large\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
                if flags as libc::c_int & 0x2 as libc::c_int != 0 {
                    updcrc(
                        base as *mut uch,
                        p.offset_from(base) as libc::c_long as libc::c_uint,
                    );
                }
                p = gzip_base_name(base);
                memmove(
                    base as *mut libc::c_void,
                    p as *const libc::c_void,
                    (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                if list == 0 {
                    if !base.is_null() {
                        list = 0 as libc::c_int;
                    }
                }
            }
        }
        if flags as libc::c_int & 0x10 as libc::c_int != 0 as libc::c_int {
            discard_input_bytes(-(1 as libc::c_int) as size_t, flags as libc::c_uint);
        }
        if flags as libc::c_int & 0x2 as libc::c_int != 0 {
            let mut crc16: libc::c_uint = (updcrc(
                magic.as_mut_ptr(),
                0 as libc::c_int as libc::c_uint,
            ) & 0xffff as libc::c_int as libc::c_ulong) as libc::c_uint;
            let mut header16: libc::c_uint = (if inptr < insize {
                let fresh23 = inptr;
                inptr = inptr.wrapping_add(1);
                inbuf[fresh23 as usize] as libc::c_int
            } else {
                fill_inbuf(0 as libc::c_int)
            }) as libc::c_uint;
            header16
                |= ((if inptr < insize {
                    let fresh24 = inptr;
                    inptr = inptr.wrapping_add(1);
                    inbuf[fresh24 as usize] as libc::c_int
                } else {
                    fill_inbuf(0 as libc::c_int)
                }) as libc::c_uint) << 8 as libc::c_int;
            if header16 != crc16 {
                rpl_fprintf(
                    stderr,
                    b"%s: %s: header checksum 0x%04x != computed checksum 0x%04x\n\0"
                        as *const u8 as *const libc::c_char,
                    program_name,
                    ifname.as_mut_ptr(),
                    header16,
                    crc16,
                );
                exit_code = 1 as libc::c_int;
                if force <= 1 as libc::c_int {
                    return -(1 as libc::c_int);
                }
            }
        }
        if part_nb == 1 as libc::c_int {
            header_bytes = inptr
                .wrapping_add((2 as libc::c_int * 4 as libc::c_int) as libc::c_uint)
                as off_t;
        }
    } else if memcmp(
        magic.as_mut_ptr() as *const libc::c_void,
        b"PK\x03\x04\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int && inptr == 2 as libc::c_int as libc::c_uint
        && memcmp(
            inbuf.as_mut_ptr() as *mut libc::c_char as *const libc::c_void,
            b"PK\x03\x04\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        inptr = 0 as libc::c_int as libc::c_uint;
        work = Some(
            unzip as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
        );
        if check_zipfile(in_0) != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        last_member = 1 as libc::c_int;
    } else if memcmp(
        magic.as_mut_ptr() as *const libc::c_void,
        b"\x1F\x1E\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        work = Some(
            unpack as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
        );
        method = 2 as libc::c_int;
    } else if memcmp(
        magic.as_mut_ptr() as *const libc::c_void,
        b"\x1F\x9D\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        work = Some(
            unlzw as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
        );
        method = 1 as libc::c_int;
        last_member = 1 as libc::c_int;
    } else if memcmp(
        magic.as_mut_ptr() as *const libc::c_void,
        b"\x1F\xA0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        work = Some(
            unlzh as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
        );
        method = 3 as libc::c_int;
        last_member = 1 as libc::c_int;
    } else if force != 0 && to_stdout != 0 && list == 0 {
        method = 0 as libc::c_int;
        work = Some(
            copy as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
        );
        if imagic1 != -(1 as libc::c_int) {
            inptr = inptr.wrapping_sub(1);
            inptr;
        }
        last_member = 1 as libc::c_int;
        if imagic0 != -(1 as libc::c_int) {
            write_buf(
                1 as libc::c_int,
                magic.as_mut_ptr() as voidp,
                1 as libc::c_int as libc::c_uint,
            );
        }
    }
    if method >= 0 as libc::c_int {
        return method;
    }
    if part_nb == 1 as libc::c_int {
        rpl_fprintf(
            stderr,
            b"\n%s: %s: not in gzip format\n\0" as *const u8 as *const libc::c_char,
            program_name,
            ifname.as_mut_ptr(),
        );
        exit_code = 1 as libc::c_int;
        return -(1 as libc::c_int);
    } else {
        if magic[0 as libc::c_int as usize] as libc::c_int == 0 as libc::c_int {
            let mut inbyte: libc::c_int = 0;
            inbyte = imagic1;
            while inbyte == 0 as libc::c_int {
                inbyte = if inptr < insize {
                    let fresh25 = inptr;
                    inptr = inptr.wrapping_add(1);
                    inbuf[fresh25 as usize] as libc::c_int
                } else {
                    fill_inbuf(1 as libc::c_int)
                };
            }
            if inbyte == -(1 as libc::c_int) {
                if verbose != 0 {
                    if quiet == 0 {
                        rpl_fprintf(
                            stderr,
                            b"\n%s: %s: decompression OK, trailing zero bytes ignored\n\0"
                                as *const u8 as *const libc::c_char,
                            program_name,
                            ifname.as_mut_ptr(),
                        );
                    }
                    if exit_code == 0 as libc::c_int {
                        exit_code = 2 as libc::c_int;
                    }
                }
                return -(3 as libc::c_int);
            }
        }
        if quiet == 0 {
            rpl_fprintf(
                stderr,
                b"\n%s: %s: decompression OK, trailing garbage ignored\n\0" as *const u8
                    as *const libc::c_char,
                program_name,
                ifname.as_mut_ptr(),
            );
        }
        if exit_code == 0 as libc::c_int {
            exit_code = 2 as libc::c_int;
        }
        return -(2 as libc::c_int);
    };
}
unsafe extern "C" fn do_list(mut method_0: libc::c_int) {
    let mut crc: ulg = 0;
    static mut first_time: libc::c_int = 1 as libc::c_int;
    static mut methods: [*const libc::c_char; 9] = [
        b"store\0" as *const u8 as *const libc::c_char,
        b"compr\0" as *const u8 as *const libc::c_char,
        b"pack \0" as *const u8 as *const libc::c_char,
        b"lzh  \0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b"defla\0" as *const u8 as *const libc::c_char,
    ];
    let mut positive_off_t_width: libc::c_int = (::std::mem::size_of::<off_t>()
        as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_sub(
            !((0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t) as libc::c_int
                as libc::c_ulong,
        )
        .wrapping_mul(146 as libc::c_int as libc::c_ulong)
        .wrapping_add(484 as libc::c_int as libc::c_ulong)
        .wrapping_div(485 as libc::c_int as libc::c_ulong)
        .wrapping_add(
            !((0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t) as libc::c_int
                as libc::c_ulong,
        )
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    if first_time != 0 && method_0 >= 0 as libc::c_int {
        first_time = 0 as libc::c_int;
        if verbose != 0 {
            __printf__(
                b"method  crc     date  time  \0" as *const u8 as *const libc::c_char,
            );
        }
        if quiet == 0 {
            __printf__(
                b"%*.*s %*.*s  ratio uncompressed_name\n\0" as *const u8
                    as *const libc::c_char,
                positive_off_t_width,
                positive_off_t_width,
                b"compressed\0" as *const u8 as *const libc::c_char,
                positive_off_t_width,
                positive_off_t_width,
                b"uncompressed\0" as *const u8 as *const libc::c_char,
            );
        }
    } else if method_0 < 0 as libc::c_int {
        if total_in <= 0 as libc::c_int as libc::c_long
            || total_out <= 0 as libc::c_int as libc::c_long
        {
            return;
        }
        if verbose != 0 {
            __printf__(
                b"                            \0" as *const u8 as *const libc::c_char,
            );
        }
        if verbose != 0 || quiet == 0 {
            fprint_off(stdout, total_in, positive_off_t_width);
            __printf__(b" \0" as *const u8 as *const libc::c_char);
            fprint_off(stdout, total_out, positive_off_t_width);
            __printf__(b" \0" as *const u8 as *const libc::c_char);
        }
        display_ratio(total_out - (total_in - header_bytes), total_out, stdout);
        __printf__(b" (totals)\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    crc = !(0 as libc::c_int) as ulg;
    if method_0 == 8 as libc::c_int && last_member == 0 {
        crc = unzip_crc;
    }
    if verbose != 0 {
        static mut month_abbr: [[libc::c_char; 4]; 12] = unsafe {
            [
                *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"Jan\0"),
                *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"Feb\0"),
                *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"Mar\0"),
                *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"Apr\0"),
                *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"May\0"),
                *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"Jun\0"),
                *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"Jul\0"),
                *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"Aug\0"),
                *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"Sep\0"),
                *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"Oct\0"),
                *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"Nov\0"),
                *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"Dec\0"),
            ]
        };
        let mut tm: *mut tm = localtime(&mut time_stamp.tv_sec);
        __printf__(
            b"%5s %08lx \0" as *const u8 as *const libc::c_char,
            methods[method_0 as usize],
            crc,
        );
        if !tm.is_null() {
            __printf__(
                b"%s%3d %02d:%02d \0" as *const u8 as *const libc::c_char,
                (month_abbr[(*tm).tm_mon as usize]).as_ptr(),
                (*tm).tm_mday,
                (*tm).tm_hour,
                (*tm).tm_min,
            );
        } else {
            __printf__(b"??? ?? ??:?? \0" as *const u8 as *const libc::c_char);
        }
    }
    fprint_off(stdout, bytes_in, positive_off_t_width);
    __printf__(b" \0" as *const u8 as *const libc::c_char);
    fprint_off(stdout, bytes_out, positive_off_t_width);
    __printf__(b" \0" as *const u8 as *const libc::c_char);
    if bytes_in == -(1 as libc::c_long) {
        total_in = -(1 as libc::c_long);
        header_bytes = 0 as libc::c_int as off_t;
        bytes_out = header_bytes;
        bytes_in = bytes_out;
    } else if total_in >= 0 as libc::c_int as libc::c_long {
        total_in += bytes_in;
    }
    if bytes_out == -(1 as libc::c_long) {
        total_out = -(1 as libc::c_long);
        header_bytes = 0 as libc::c_int as off_t;
        bytes_out = header_bytes;
        bytes_in = bytes_out;
    } else if total_out >= 0 as libc::c_int as libc::c_long {
        total_out += bytes_out;
    }
    display_ratio(bytes_out - (bytes_in - header_bytes), bytes_out, stdout);
    __printf__(b" %s\n\0" as *const u8 as *const libc::c_char, ofname.as_mut_ptr());
}
unsafe extern "C" fn shorten_name(mut name: *mut libc::c_char) {
    let mut len: libc::c_int = 0;
    let mut trunc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut plen: libc::c_int = 0;
    let mut min_part: libc::c_int = 3 as libc::c_int;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    len = strlen(name) as libc::c_int;
    if decompress != 0 {
        if len <= 1 as libc::c_int {
            gzip_error(b"name too short\0" as *const u8 as *const libc::c_char);
        }
        *name.offset((len - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        return;
    }
    p = get_suffix(name);
    if p.is_null() {
        gzip_error(b"can't recover suffix\n\0" as *const u8 as *const libc::c_char);
    }
    *p = '\0' as i32 as libc::c_char;
    save_orig_name = 1 as libc::c_int;
    if len > 4 as libc::c_int
        && strcmp(
            p.offset(-(4 as libc::c_int as isize)),
            b".tar\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        strcpy(
            p.offset(-(4 as libc::c_int as isize)),
            b".tgz\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    loop {
        p = last_component(name);
        while *p != 0 {
            plen = strcspn(p, b".\0" as *const u8 as *const libc::c_char) as libc::c_int;
            p = p.offset(plen as isize);
            if plen > min_part {
                trunc = p.offset(-(1 as libc::c_int as isize));
            }
            if *p != 0 {
                p = p.offset(1);
                p;
            }
        }
        if !(trunc.is_null()
            && {
                min_part -= 1;
                min_part != 0 as libc::c_int
            })
        {
            break;
        }
    }
    if !trunc.is_null() {
        loop {
            *trunc
                .offset(
                    0 as libc::c_int as isize,
                ) = *trunc.offset(1 as libc::c_int as isize);
            let fresh26 = trunc;
            trunc = trunc.offset(1);
            if !(*fresh26 != 0) {
                break;
            }
        }
        trunc = trunc.offset(-1);
        trunc;
    } else {
        trunc = strrchr(
            name,
            (*::std::mem::transmute::<
                &[u8; 2],
                &[libc::c_char; 2],
            >(b".\0"))[0 as libc::c_int as usize] as libc::c_int,
        );
        if trunc.is_null() {
            gzip_error(
                b"internal error in shorten_name\0" as *const u8 as *const libc::c_char,
            );
        }
        if *trunc.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
            trunc = trunc.offset(-1);
            trunc;
        }
    }
    strcpy(trunc, z_suffix);
}
unsafe extern "C" fn check_ofname() -> libc::c_int {
    if force == 0 {
        let mut ok: libc::c_int = 0 as libc::c_int;
        rpl_fprintf(
            stderr,
            b"%s: %s already exists;\0" as *const u8 as *const libc::c_char,
            program_name,
            ofname.as_mut_ptr(),
        );
        if foreground != 0
            && (presume_input_tty as libc::c_int != 0 || isatty(0 as libc::c_int) != 0)
        {
            rpl_fprintf(
                stderr,
                b" do you wish to overwrite (y or n)? \0" as *const u8
                    as *const libc::c_char,
            );
            rpl_fflush(stderr);
            ok = yesno() as libc::c_int;
        }
        if ok == 0 {
            rpl_fprintf(
                stderr,
                b"\tnot overwritten\n\0" as *const u8 as *const libc::c_char,
            );
            if exit_code == 0 as libc::c_int {
                exit_code = 2 as libc::c_int;
            }
            return 1 as libc::c_int;
        }
    }
    if xunlink(ofname.as_mut_ptr()) != 0 {
        progerror(ofname.as_mut_ptr());
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn do_chown(
    mut fd: libc::c_int,
    mut name: *const libc::c_char,
    mut uid: uid_t,
    mut gid: gid_t,
) {
    fchown(fd, uid, gid);
}
unsafe extern "C" fn copy_stat(mut ifstat: *mut stat) {
    let mut mode: mode_t = (*ifstat).st_mode
        & (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint;
    let mut r: libc::c_int = 0;
    let mut restoring: bool = false;
    let mut timespec: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    timespec[0 as libc::c_int as usize] = get_stat_atime(ifstat);
    timespec[1 as libc::c_int as usize] = get_stat_mtime(ifstat);
    restoring = decompress != 0 && 0 as libc::c_int as libc::c_long <= time_stamp.tv_nsec
        && !(timespec[1 as libc::c_int as usize].tv_sec == time_stamp.tv_sec
            && timespec[1 as libc::c_int as usize].tv_nsec == time_stamp.tv_nsec);
    if restoring {
        timespec[1 as libc::c_int as usize] = time_stamp;
    }
    if fdutimens(ofd, ofname.as_mut_ptr(), timespec.as_mut_ptr() as *const timespec)
        == 0 as libc::c_int
    {
        if restoring as libc::c_int != 0 && (1 as libc::c_int) < verbose {
            rpl_fprintf(
                stderr,
                b"%s: timestamp restored\n\0" as *const u8 as *const libc::c_char,
                ofname.as_mut_ptr(),
            );
        }
    } else {
        let mut e: libc::c_int = *__errno_location();
        if quiet == 0 {
            rpl_fprintf(
                stderr,
                b"%s: \0" as *const u8 as *const libc::c_char,
                program_name,
            );
        }
        if exit_code == 0 as libc::c_int {
            exit_code = 2 as libc::c_int;
        }
        if quiet == 0 {
            *__errno_location() = e;
            perror(ofname.as_mut_ptr());
        }
    }
    do_chown(ofd, ofname.as_mut_ptr(), -(1 as libc::c_int) as uid_t, (*ifstat).st_gid);
    r = fchmod(ofd, mode);
    if r != 0 as libc::c_int {
        let mut e_0: libc::c_int = *__errno_location();
        if quiet == 0 {
            rpl_fprintf(
                stderr,
                b"%s: \0" as *const u8 as *const libc::c_char,
                program_name,
            );
        }
        if exit_code == 0 as libc::c_int {
            exit_code = 2 as libc::c_int;
        }
        if quiet == 0 {
            *__errno_location() = e_0;
            perror(ofname.as_mut_ptr());
        }
    }
    do_chown(ofd, ofname.as_mut_ptr(), (*ifstat).st_uid, -(1 as libc::c_int) as gid_t);
}
unsafe extern "C" fn treat_dir(mut fd: libc::c_int, mut dir: *mut libc::c_char) {
    let mut dirp: *mut DIR = 0 as *mut DIR;
    let mut nbuf: [libc::c_char; 1024] = [0; 1024];
    let mut entries: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut entry: *const libc::c_char = 0 as *const libc::c_char;
    let mut entrylen: size_t = 0;
    dirp = fdopendir(fd);
    if dirp.is_null() {
        progerror(dir);
        close(fd);
        return;
    }
    entries = streamsavedir(dirp, SAVEDIR_SORT_NONE);
    if entries.is_null() {
        progerror(dir);
    }
    if closedir(dirp) != 0 as libc::c_int {
        progerror(dir);
    }
    if entries.is_null() {
        return;
    }
    entry = entries;
    while *entry != 0 {
        let mut len: size_t = strlen(dir);
        entrylen = strlen(entry);
        if !(strcmp(entry, b".\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(entry, b"..\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int)
        {
            if len.wrapping_add(entrylen)
                < (1024 as libc::c_int - 2 as libc::c_int) as libc::c_ulong
            {
                strcpy(nbuf.as_mut_ptr(), dir);
                if *last_component(nbuf.as_mut_ptr()) as libc::c_int != 0
                    && !(nbuf[len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as usize] as libc::c_int == '/' as i32)
                {
                    let fresh27 = len;
                    len = len.wrapping_add(1);
                    nbuf[fresh27 as usize] = '/' as i32 as libc::c_char;
                }
                strcpy(nbuf.as_mut_ptr().offset(len as isize), entry);
                treat_file(nbuf.as_mut_ptr());
            } else {
                rpl_fprintf(
                    stderr,
                    b"%s: %s/%s: pathname too long\n\0" as *const u8
                        as *const libc::c_char,
                    program_name,
                    dir,
                    entry,
                );
                exit_code = 1 as libc::c_int;
            }
        }
        entry = entry
            .offset(entrylen.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    }
    rpl_free(entries as *mut libc::c_void);
}
unsafe extern "C" fn install_signal_handlers() {
    let mut nsigs: libc::c_int = (::std::mem::size_of::<[libc::c_int; 6]>()
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut act: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sigemptyset(&mut caught_signals);
    i = 0 as libc::c_int;
    while i < nsigs {
        sigaction(handled_sig[i as usize], 0 as *const sigaction, &mut act);
        if act.__sigaction_handler.sa_handler
            != ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t)
        {
            sigaddset(&mut caught_signals, handled_sig[i as usize]);
        }
        i += 1;
        i;
    }
    act
        .__sigaction_handler
        .sa_handler = Some(abort_gzip_signal as unsafe extern "C" fn(libc::c_int) -> ());
    act.sa_mask = caught_signals;
    act.sa_flags = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nsigs {
        if sigismember(&mut caught_signals, handled_sig[i as usize]) != 0 {
            if i == 0 as libc::c_int {
                foreground = 1 as libc::c_int;
            }
            sigaction(handled_sig[i as usize], &mut act, 0 as *mut sigaction);
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn do_exit(mut exitcode: libc::c_int) {
    static mut in_exit: libc::c_int = 0 as libc::c_int;
    if in_exit != 0 {
        exit(exitcode);
    }
    in_exit = 1 as libc::c_int;
    rpl_free(env as *mut libc::c_void);
    env = 0 as *mut libc::c_char;
    exit(exitcode);
}
unsafe extern "C" fn finish_out() {
    if rpl_fclose(stdout) != 0 as libc::c_int {
        write_error();
    }
    do_exit(0 as libc::c_int);
}
unsafe extern "C" fn remove_output_file(mut signals_already_blocked: bool) {
    let mut fd: libc::c_int = 0;
    let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
    if !signals_already_blocked {
        sigprocmask(0 as libc::c_int, &mut caught_signals, &mut oldset);
    }
    fd = remove_ofname_fd;
    if 0 as libc::c_int <= fd {
        let mut fname: [libc::c_char; 1024] = [0; 1024];
        ::std::ptr::write_volatile(
            &mut remove_ofname_fd as *mut libc::c_int,
            -(1 as libc::c_int),
        );
        close(fd);
        volatile_strcpy(
            fname.as_mut_ptr() as *mut libc::c_char,
            remove_ofname.as_mut_ptr(),
        );
        xunlink(fname.as_mut_ptr());
    }
    if !signals_already_blocked {
        sigprocmask(2 as libc::c_int, &mut oldset, 0 as *mut sigset_t);
    }
}
pub unsafe extern "C" fn abort_gzip() {
    remove_output_file(0 as libc::c_int != 0);
    do_exit(1 as libc::c_int);
}
unsafe extern "C" fn abort_gzip_signal(mut sig: libc::c_int) {
    remove_output_file(1 as libc::c_int != 0);
    if sig == exiting_signal {
        _exit(2 as libc::c_int);
    }
    signal(sig, None);
    raise(sig);
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
