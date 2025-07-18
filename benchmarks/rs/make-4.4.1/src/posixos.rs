use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn pselect(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn mkfifo(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn tmpfile() -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    static mut handling_fatal_signal: sig_atomic_t;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn error(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...) -> !;
    fn pfatal_with_name(_: *const libc::c_char) -> !;
    fn perror_with_name(_: *const libc::c_char, _: *const libc::c_char);
    fn make_pid() -> pid_t;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn get_tmpdir() -> *const libc::c_char;
    fn get_tmpfd(_: *mut *mut libc::c_char) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    static mut db_level: libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type mode_t = __mode_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
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
    pub fds_bits: [__fd_mask; 16],
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
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const libc::c_char,
    pub lineno: libc::c_ulong,
    pub offset: libc::c_ulong,
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
pub const js_none: js_type = 0;
pub type js_type = libc::c_uint;
pub const js_fifo: js_type = 2;
pub const js_pipe: js_type = 1;
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
pub unsafe extern "C" fn check_io_state() -> libc::c_uint {
    static mut state: libc::c_uint = 0x1 as libc::c_int as libc::c_uint;
    if state != 0x1 as libc::c_int as libc::c_uint {
        return state;
    }
    if fcntl(fileno(stdin), 1 as libc::c_int) != -(1 as libc::c_int)
        || *__errno_location() != 9 as libc::c_int
    {
        state |= 0x4 as libc::c_int as libc::c_uint;
    }
    if fcntl(fileno(stdout), 1 as libc::c_int) != -(1 as libc::c_int)
        || *__errno_location() != 9 as libc::c_int
    {
        state |= 0x8 as libc::c_int as libc::c_uint;
    }
    if fcntl(fileno(stderr), 1 as libc::c_int) != -(1 as libc::c_int)
        || *__errno_location() != 9 as libc::c_int
    {
        state |= 0x10 as libc::c_int as libc::c_uint;
    }
    if state & (0x8 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
        == (0x8 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
    {
        let mut stbuf_o: stat = stat {
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
        let mut stbuf_e: stat = stat {
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
        if fstat(fileno(stdout), &mut stbuf_o) == 0 as libc::c_int
            && fstat(fileno(stderr), &mut stbuf_e) == 0 as libc::c_int
            && stbuf_o.st_dev == stbuf_e.st_dev && stbuf_o.st_ino == stbuf_e.st_ino
        {
            state |= 0x2 as libc::c_int as libc::c_uint;
        }
    }
    return state;
}
static mut job_root: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
static mut job_fds: [libc::c_int; 2] = [-(1 as libc::c_int), -(1 as libc::c_int)];
static mut job_rfd: libc::c_int = -(1 as libc::c_int);
static mut token: libc::c_char = '+' as i32 as libc::c_char;
static mut js_type: js_type = js_none;
static mut fifo_name: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn make_job_rfd() -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn set_blocking(mut fd: libc::c_int, mut blocking: libc::c_int) {
    let mut flags: libc::c_int = 0;
    loop {
        flags = fcntl(fd, 3 as libc::c_int);
        if !(flags == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    if flags >= 0 as libc::c_int {
        let mut r: libc::c_int = 0;
        flags = if blocking != 0 {
            flags & !(0o4000 as libc::c_int)
        } else {
            flags | 0o4000 as libc::c_int
        };
        loop {
            r = fcntl(fd, 4 as libc::c_int, flags);
            if !(r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if r < 0 as libc::c_int {
            pfatal_with_name(b"fcntl(O_NONBLOCK)\0" as *const u8 as *const libc::c_char);
        }
    }
}
pub unsafe extern "C" fn jobserver_setup(
    mut slots: libc::c_int,
    mut style: *const libc::c_char,
) -> libc::c_uint {
    let mut r: libc::c_int = 0;
    if style.is_null()
        || strcmp(style, b"fifo\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        let mut tmpdir: *const libc::c_char = get_tmpdir();
        fifo_name = xmalloc(
            (strlen(tmpdir))
                .wrapping_add(
                    (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(
                    (53 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<uintmax_t>() as libc::c_ulong,
                        )
                        .wrapping_div(22 as libc::c_int as libc::c_ulong)
                        .wrapping_add(3 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        sprintf(
            fifo_name,
            b"%s/GMfifo%lld\0" as *const u8 as *const libc::c_char,
            tmpdir,
            make_pid() as libc::c_longlong,
        );
        loop {
            r = mkfifo(fifo_name, 0o600 as libc::c_int as __mode_t);
            if !(r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if r < 0 as libc::c_int {
            perror_with_name(
                b"jobserver mkfifo: \0" as *const u8 as *const libc::c_char,
                fifo_name,
            );
            free(fifo_name as *mut libc::c_void);
            fifo_name = 0 as *mut libc::c_char;
        } else {
            loop {
                job_fds[0 as libc::c_int
                    as usize] = open(
                    fifo_name,
                    0 as libc::c_int | 0o4000 as libc::c_int,
                );
                if !(job_fds[0 as libc::c_int as usize] == -(1 as libc::c_int)
                    && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
            }
            if job_fds[0 as libc::c_int as usize] < 0 as libc::c_int {
                fatal(
                    0 as *mut floc,
                    (strlen(fifo_name))
                        .wrapping_add(strlen(strerror(*__errno_location()))),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot open jobserver %s: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    fifo_name,
                    strerror(*__errno_location()),
                );
            }
            loop {
                job_fds[1 as libc::c_int as usize] = open(fifo_name, 0o1 as libc::c_int);
                if !(job_fds[1 as libc::c_int as usize] == -(1 as libc::c_int)
                    && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
            }
            if job_fds[0 as libc::c_int as usize] < 0 as libc::c_int {
                fatal(
                    0 as *mut floc,
                    (strlen(fifo_name))
                        .wrapping_add(strlen(strerror(*__errno_location()))),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot open jobserver %s: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    fifo_name,
                    strerror(*__errno_location()),
                );
            }
            js_type = js_fifo;
        }
    }
    if js_type as libc::c_uint == js_none as libc::c_int as libc::c_uint {
        if !style.is_null()
            && strcmp(style, b"pipe\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int
        {
            fatal(
                0 as *mut floc,
                strlen(style),
                dcgettext(
                    0 as *const libc::c_char,
                    b"unknown jobserver auth style '%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                style,
            );
        }
        loop {
            r = pipe(job_fds.as_mut_ptr());
            if !(r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if r < 0 as libc::c_int {
            pfatal_with_name(
                dcgettext(
                    0 as *const libc::c_char,
                    b"creating jobs pipe\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        js_type = js_pipe;
    }
    fd_noinherit(job_fds[0 as libc::c_int as usize]);
    fd_noinherit(job_fds[1 as libc::c_int as usize]);
    if make_job_rfd() < 0 as libc::c_int {
        pfatal_with_name(
            dcgettext(
                0 as *const libc::c_char,
                b"duping jobs pipe\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    loop {
        let fresh0 = slots;
        slots = slots - 1;
        if !(fresh0 != 0) {
            break;
        }
        loop {
            r = write(
                job_fds[1 as libc::c_int as usize],
                &mut token as *mut libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            ) as libc::c_int;
            if !(r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if r != 1 as libc::c_int {
            pfatal_with_name(
                dcgettext(
                    0 as *const libc::c_char,
                    b"init jobserver pipe\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    set_blocking(job_fds[0 as libc::c_int as usize], 0 as libc::c_int);
    job_root = 1 as libc::c_int as libc::c_uchar;
    return 1 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn jobserver_parse_auth(
    mut auth: *const libc::c_char,
) -> libc::c_uint {
    let mut rfd: libc::c_int = 0;
    let mut wfd: libc::c_int = 0;
    if strncmp(
        auth,
        b"fifo:\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) == 0 as libc::c_int
    {
        fifo_name = xstrdup(
            auth
                .offset(
                    (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ),
        );
        loop {
            job_fds[0 as libc::c_int as usize] = open(fifo_name, 0 as libc::c_int);
            if !(job_fds[0 as libc::c_int as usize] == -(1 as libc::c_int)
                && *__errno_location() == 4 as libc::c_int)
            {
                break;
            }
        }
        if job_fds[0 as libc::c_int as usize] < 0 as libc::c_int {
            error(
                0 as *mut floc,
                (strlen(fifo_name)).wrapping_add(strlen(strerror(*__errno_location()))),
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot open jobserver %s: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fifo_name,
                strerror(*__errno_location()),
            );
            return 0 as libc::c_int as libc::c_uint;
        }
        loop {
            job_fds[1 as libc::c_int as usize] = open(fifo_name, 0o1 as libc::c_int);
            if !(job_fds[1 as libc::c_int as usize] == -(1 as libc::c_int)
                && *__errno_location() == 4 as libc::c_int)
            {
                break;
            }
        }
        if job_fds[1 as libc::c_int as usize] < 0 as libc::c_int {
            error(
                0 as *mut floc,
                (strlen(fifo_name)).wrapping_add(strlen(strerror(*__errno_location()))),
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot open jobserver %s: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fifo_name,
                strerror(*__errno_location()),
            );
            return 0 as libc::c_int as libc::c_uint;
        }
        js_type = js_fifo;
    } else if sscanf(
        auth,
        b"%d,%d\0" as *const u8 as *const libc::c_char,
        &mut rfd as *mut libc::c_int,
        &mut wfd as *mut libc::c_int,
    ) == 2 as libc::c_int
    {
        if rfd == -(2 as libc::c_int) || wfd == -(2 as libc::c_int) {
            return 0 as libc::c_int as libc::c_uint;
        }
        if !(fcntl(rfd, 1 as libc::c_int) != -(1 as libc::c_int))
            || !(fcntl(wfd, 1 as libc::c_int) != -(1 as libc::c_int))
        {
            return 0 as libc::c_int as libc::c_uint;
        }
        job_fds[0 as libc::c_int as usize] = rfd;
        job_fds[1 as libc::c_int as usize] = wfd;
        js_type = js_pipe;
    } else {
        error(
            0 as *mut floc,
            strlen(auth),
            dcgettext(
                0 as *const libc::c_char,
                b"invalid --jobserver-auth string '%s'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            auth,
        );
        return 0 as libc::c_int as libc::c_uint;
    }
    if make_job_rfd() < 0 as libc::c_int {
        if *__errno_location() != 9 as libc::c_int {
            pfatal_with_name(b"jobserver readfd\0" as *const u8 as *const libc::c_char);
        }
        jobserver_clear();
        return 0 as libc::c_int as libc::c_uint;
    }
    set_blocking(job_fds[0 as libc::c_int as usize], 0 as libc::c_int);
    fd_noinherit(job_fds[0 as libc::c_int as usize]);
    fd_noinherit(job_fds[1 as libc::c_int as usize]);
    return 1 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn jobserver_get_auth() -> *mut libc::c_char {
    let mut auth: *mut libc::c_char = 0 as *mut libc::c_char;
    if js_type as libc::c_uint == js_fifo as libc::c_int as libc::c_uint {
        auth = xmalloc(
            (strlen(fifo_name))
                .wrapping_add(
                    (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        sprintf(auth, b"fifo:%s\0" as *const u8 as *const libc::c_char, fifo_name);
    } else {
        auth = xmalloc(
            (53 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                .wrapping_div(22 as libc::c_int as libc::c_ulong)
                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        sprintf(
            auth,
            b"%d,%d\0" as *const u8 as *const libc::c_char,
            job_fds[0 as libc::c_int as usize],
            job_fds[1 as libc::c_int as usize],
        );
    }
    return auth;
}
pub unsafe extern "C" fn jobserver_get_invalid_auth() -> *const libc::c_char {
    if js_type as libc::c_uint == js_fifo as libc::c_int as libc::c_uint {
        return 0 as *const libc::c_char;
    }
    return b" --jobserver-auth=-2,-2\0" as *const u8 as *const libc::c_char;
}
pub unsafe extern "C" fn jobserver_enabled() -> libc::c_uint {
    return (js_type as libc::c_uint != js_none as libc::c_int as libc::c_uint)
        as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn jobserver_clear() {
    if job_fds[0 as libc::c_int as usize] >= 0 as libc::c_int {
        close(job_fds[0 as libc::c_int as usize]);
    }
    if job_fds[1 as libc::c_int as usize] >= 0 as libc::c_int {
        close(job_fds[1 as libc::c_int as usize]);
    }
    if job_rfd >= 0 as libc::c_int {
        close(job_rfd);
    }
    job_rfd = -(1 as libc::c_int);
    job_fds[1 as libc::c_int as usize] = job_rfd;
    job_fds[0 as libc::c_int as usize] = job_fds[1 as libc::c_int as usize];
    if !fifo_name.is_null() {
        if job_root != 0 {
            let mut r: libc::c_int = 0;
            loop {
                r = unlink(fifo_name);
                if !(r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
            }
        }
        if handling_fatal_signal == 0 {
            free(fifo_name as *mut libc::c_void);
            fifo_name = 0 as *mut libc::c_char;
        }
    }
    js_type = js_none;
}
pub unsafe extern "C" fn jobserver_release(mut is_fatal: libc::c_int) {
    let mut r: libc::c_int = 0;
    loop {
        r = write(
            job_fds[1 as libc::c_int as usize],
            &mut token as *mut libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        ) as libc::c_int;
        if !(r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    if r != 1 as libc::c_int {
        if is_fatal != 0 {
            pfatal_with_name(
                dcgettext(
                    0 as *const libc::c_char,
                    b"write jobserver\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        perror_with_name(
            b"write\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
}
pub unsafe extern "C" fn jobserver_acquire_all() -> libc::c_uint {
    let mut r: libc::c_int = 0;
    let mut tokens: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    set_blocking(job_fds[0 as libc::c_int as usize], 1 as libc::c_int);
    close(job_fds[1 as libc::c_int as usize]);
    job_fds[1 as libc::c_int as usize] = -(1 as libc::c_int);
    loop {
        let mut intake: libc::c_char = 0;
        loop {
            r = read(
                job_fds[0 as libc::c_int as usize],
                &mut intake as *mut libc::c_char as *mut libc::c_void,
                1 as libc::c_int as size_t,
            ) as libc::c_int;
            if !(r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if r != 1 as libc::c_int {
            break;
        }
        tokens = tokens.wrapping_add(1);
        tokens;
    }
    if 0x4 as libc::c_int & db_level != 0 {
        printf(
            b"Acquired all %u jobserver tokens.\n\0" as *const u8 as *const libc::c_char,
            tokens,
        );
        fflush(stdout);
    }
    jobserver_clear();
    return tokens;
}
pub unsafe extern "C" fn jobserver_pre_child(mut recursive: libc::c_int) {
    if recursive != 0
        && js_type as libc::c_uint == js_pipe as libc::c_int as libc::c_uint
    {
        fd_inherit(job_fds[0 as libc::c_int as usize]);
        fd_inherit(job_fds[1 as libc::c_int as usize]);
    }
}
pub unsafe extern "C" fn jobserver_post_child(mut recursive: libc::c_int) {
    if recursive != 0
        && js_type as libc::c_uint == js_pipe as libc::c_int as libc::c_uint
    {
        fd_noinherit(job_fds[0 as libc::c_int as usize]);
        fd_noinherit(job_fds[1 as libc::c_int as usize]);
    }
}
pub unsafe extern "C" fn jobserver_signal() {
    if job_rfd >= 0 as libc::c_int {
        close(job_rfd);
        job_rfd = -(1 as libc::c_int);
    }
}
pub unsafe extern "C" fn jobserver_pre_acquire() {
    if job_rfd < 0 as libc::c_int
        && job_fds[0 as libc::c_int as usize] >= 0 as libc::c_int
        && make_job_rfd() < 0 as libc::c_int
    {
        pfatal_with_name(
            dcgettext(
                0 as *const libc::c_char,
                b"duping jobs pipe\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
pub unsafe extern "C" fn jobserver_acquire(mut timeout: libc::c_int) -> libc::c_uint {
    let mut spec: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut specp: *mut timespec = 0 as *mut timespec;
    let mut empty: sigset_t = sigset_t { __val: [0; 16] };
    sigemptyset(&mut empty);
    if timeout != 0 {
        spec.tv_sec = 1 as libc::c_int as __time_t;
        spec.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
        specp = &mut spec;
    }
    loop {
        let mut readfds: fd_set = fd_set { fds_bits: [0; 16] };
        let mut r: libc::c_int = 0;
        let mut intake: libc::c_char = 0;
        let mut __d0: libc::c_int = 0;
        let mut __d1: libc::c_int = 0;
        let fresh1 = &mut __d0;
        let fresh2;
        let fresh3 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh4 = &mut __d1;
        let fresh5;
        let fresh6 = &mut *(readfds.fds_bits)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh1,
            fresh3) => fresh2, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh4,
            fresh6) => fresh5, inlateout("ax") 0 as libc::c_int => _,
            options(preserves_flags, att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh1, fresh3, fresh2);
        c2rust_asm_casts::AsmCast::cast_out(fresh4, fresh6, fresh5);
        readfds
            .fds_bits[(job_fds[0 as libc::c_int as usize]
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << job_fds[0 as libc::c_int as usize]
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        r = pselect(
            job_fds[0 as libc::c_int as usize] + 1 as libc::c_int,
            &mut readfds,
            0 as *mut fd_set,
            0 as *mut fd_set,
            specp,
            &mut empty,
        );
        if r < 0 as libc::c_int {
            match *__errno_location() {
                4 => return 0 as libc::c_int as libc::c_uint,
                9 => {
                    fatal(
                        0 as *mut floc,
                        0 as libc::c_int as size_t,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"job server shut down\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                _ => {
                    pfatal_with_name(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"pselect jobs pipe\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            }
        }
        if r == 0 as libc::c_int {
            return 0 as libc::c_int as libc::c_uint;
        }
        loop {
            r = read(
                job_fds[0 as libc::c_int as usize],
                &mut intake as *mut libc::c_char as *mut libc::c_void,
                1 as libc::c_int as size_t,
            ) as libc::c_int;
            if !(r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if r < 0 as libc::c_int {
            if *__errno_location() == 11 as libc::c_int {
                continue;
            }
            pfatal_with_name(
                dcgettext(
                    0 as *const libc::c_char,
                    b"read jobs pipe\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            return (r > 0 as libc::c_int) as libc::c_int as libc::c_uint
        }
    };
}
static mut osync_handle: libc::c_int = -(1 as libc::c_int);
static mut osync_tmpfile: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut sync_root: libc::c_uint = 0 as libc::c_int as libc::c_uint;
pub unsafe extern "C" fn osync_enabled() -> libc::c_uint {
    return (osync_handle >= 0 as libc::c_int) as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn osync_setup() {
    osync_handle = get_tmpfd(&mut osync_tmpfile);
    fd_noinherit(osync_handle);
    sync_root = 1 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn osync_get_mutex() -> *mut libc::c_char {
    let mut mutex: *mut libc::c_char = 0 as *mut libc::c_char;
    if osync_enabled() != 0 {
        mutex = xmalloc(
            (strlen(osync_tmpfile))
                .wrapping_add(
                    (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        sprintf(mutex, b"fnm:%s\0" as *const u8 as *const libc::c_char, osync_tmpfile);
    }
    return mutex;
}
pub unsafe extern "C" fn osync_parse_mutex(
    mut mutex: *const libc::c_char,
) -> libc::c_uint {
    if strncmp(
        mutex,
        b"fnm:\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) != 0 as libc::c_int
    {
        error(
            0 as *mut floc,
            strlen(mutex),
            dcgettext(
                0 as *const libc::c_char,
                b"invalid --sync-mutex string '%s'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            mutex,
        );
        return 0 as libc::c_int as libc::c_uint;
    }
    free(osync_tmpfile as *mut libc::c_void);
    osync_tmpfile = xstrdup(
        mutex
            .offset(
                (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ),
    );
    loop {
        osync_handle = open(osync_tmpfile, 0o1 as libc::c_int);
        if !(osync_handle == -(1 as libc::c_int)
            && *__errno_location() == 4 as libc::c_int)
        {
            break;
        }
    }
    if osync_handle < 0 as libc::c_int {
        fatal(
            0 as *mut floc,
            (strlen(osync_tmpfile)).wrapping_add(strlen(strerror(*__errno_location()))),
            dcgettext(
                0 as *const libc::c_char,
                b"cannot open output sync mutex %s: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            osync_tmpfile,
            strerror(*__errno_location()),
        );
    }
    fd_noinherit(osync_handle);
    return 1 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn osync_clear() {
    if osync_handle >= 0 as libc::c_int {
        close(osync_handle);
        osync_handle = -(1 as libc::c_int);
    }
    if sync_root != 0 && !osync_tmpfile.is_null() {
        let mut r: libc::c_int = 0;
        loop {
            r = unlink(osync_tmpfile);
            if !(r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        free(osync_tmpfile as *mut libc::c_void);
        osync_tmpfile = 0 as *mut libc::c_char;
    }
}
pub unsafe extern "C" fn osync_acquire() -> libc::c_uint {
    if osync_enabled() != 0 {
        let mut fl: flock = flock {
            l_type: 0,
            l_whence: 0,
            l_start: 0,
            l_len: 0,
            l_pid: 0,
        };
        fl.l_type = 1 as libc::c_int as libc::c_short;
        fl.l_whence = 0 as libc::c_int as libc::c_short;
        fl.l_start = 0 as libc::c_int as __off_t;
        fl.l_len = 1 as libc::c_int as __off_t;
        if fcntl(osync_handle, 7 as libc::c_int, &mut fl as *mut flock)
            == -(1 as libc::c_int)
        {
            perror(b"fcntl()\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int as libc::c_uint;
        }
    }
    return 1 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn osync_release() {
    if osync_enabled() != 0 {
        let mut fl: flock = flock {
            l_type: 0,
            l_whence: 0,
            l_start: 0,
            l_len: 0,
            l_pid: 0,
        };
        fl.l_type = 2 as libc::c_int as libc::c_short;
        fl.l_whence = 0 as libc::c_int as libc::c_short;
        fl.l_start = 0 as libc::c_int as __off_t;
        fl.l_len = 1 as libc::c_int as __off_t;
        if fcntl(osync_handle, 7 as libc::c_int, &mut fl as *mut flock)
            == -(1 as libc::c_int)
        {
            perror(b"fcntl()\0" as *const u8 as *const libc::c_char);
        }
    }
}
pub unsafe extern "C" fn get_bad_stdin() -> libc::c_int {
    static mut bad_stdin: libc::c_int = -(1 as libc::c_int);
    if bad_stdin == -(1 as libc::c_int) {
        let mut pd: [libc::c_int; 2] = [0; 2];
        if pipe(pd.as_mut_ptr()) == 0 as libc::c_int {
            close(pd[1 as libc::c_int as usize]);
            bad_stdin = pd[0 as libc::c_int as usize];
            fd_noinherit(bad_stdin);
        }
    }
    return bad_stdin;
}
pub unsafe extern "C" fn fd_inherit(mut fd: libc::c_int) {
    let mut flags: libc::c_int = 0;
    loop {
        flags = fcntl(fd, 1 as libc::c_int);
        if !(flags == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    if flags >= 0 as libc::c_int {
        let mut r: libc::c_int = 0;
        flags &= !(1 as libc::c_int);
        loop {
            r = fcntl(fd, 2 as libc::c_int, flags);
            if !(r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
    }
}
pub unsafe extern "C" fn fd_noinherit(mut fd: libc::c_int) {
    let mut flags: libc::c_int = 0;
    loop {
        flags = fcntl(fd, 1 as libc::c_int);
        if !(flags == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    if flags >= 0 as libc::c_int {
        let mut r: libc::c_int = 0;
        flags |= 1 as libc::c_int;
        loop {
            r = fcntl(fd, 2 as libc::c_int, flags);
            if !(r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
    }
}
pub unsafe extern "C" fn fd_set_append(mut fd: libc::c_int) {
    let mut stbuf: stat = stat {
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
    let mut flags: libc::c_int = 0;
    if fstat(fd, &mut stbuf) == 0 as libc::c_int
        && stbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint
    {
        flags = fcntl(fd, 3 as libc::c_int, 0 as libc::c_int);
        if flags >= 0 as libc::c_int {
            let mut r: libc::c_int = 0;
            loop {
                r = fcntl(fd, 4 as libc::c_int, flags | 0o2000 as libc::c_int);
                if !(r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
            }
        }
    }
}
pub unsafe extern "C" fn os_anontmp() -> libc::c_int {
    let mut tdir: *const libc::c_char = get_tmpdir();
    let mut fd: libc::c_int = -(1 as libc::c_int);
    static mut tmpfile_works: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    if tmpfile_works != 0 {
        loop {
            fd = open(
                tdir,
                0o2 as libc::c_int
                    | (0o20000000 as libc::c_int | 0o200000 as libc::c_int)
                    | 0o200 as libc::c_int,
                0o600 as libc::c_int,
            );
            if !(fd == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if fd >= 0 as libc::c_int {
            return fd;
        }
        if 0x1 as libc::c_int & db_level != 0 {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cannot open '%s' with O_TMPFILE: %s.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                tdir,
                strerror(*__errno_location()),
            );
            fflush(stdout);
        }
        tmpfile_works = 0 as libc::c_int as libc::c_uint;
    }
    if tdir == b"/tmp\0" as *const u8 as *const libc::c_char
        || *tdir as libc::c_int
            == *(b"/tmp\0" as *const u8 as *const libc::c_char) as libc::c_int
            && (*tdir as libc::c_int == '\0' as i32
                || strcmp(
                    tdir.offset(1 as libc::c_int as isize),
                    (b"/tmp\0" as *const u8 as *const libc::c_char)
                        .offset(1 as libc::c_int as isize),
                ) == 0)
    {
        let mut mask: mode_t = umask(0o77 as libc::c_int as __mode_t);
        let mut tfile: *mut FILE = 0 as *mut FILE;
        loop {
            *__errno_location() = 0 as libc::c_int;
            tfile = tmpfile();
            if !(tfile.is_null() && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if tfile.is_null() {
            error(
                0 as *mut floc,
                strlen(strerror(*__errno_location())),
                b"tmpfile: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return -(1 as libc::c_int);
        }
        umask(mask);
        loop {
            fd = dup(fileno(tfile));
            if !(fd == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if fd < 0 as libc::c_int {
            error(
                0 as *mut floc,
                strlen(strerror(*__errno_location())),
                b"dup: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        fclose(tfile);
    }
    return fd;
}
