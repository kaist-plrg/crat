use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn die(format: *mut libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audio_output {
    pub help: Option::<unsafe extern "C" fn() -> ()>,
    pub name: *mut libc::c_char,
    pub init: Option::<
        unsafe extern "C" fn(libc::c_int, *mut *mut libc::c_char) -> libc::c_int,
    >,
    pub deinit: Option::<unsafe extern "C" fn() -> ()>,
    pub start: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub play: Option::<unsafe extern "C" fn(*mut libc::c_short, libc::c_int) -> ()>,
    pub stop: Option::<unsafe extern "C" fn() -> ()>,
    pub volume: Option::<unsafe extern "C" fn(libc::c_double) -> ()>,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
static mut fd: libc::c_int = -(1 as libc::c_int);
static mut pipename: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut Fs: libc::c_int = 0;
static mut starttime: libc::c_longlong = 0;
static mut samples_played: libc::c_longlong = 0;
unsafe extern "C" fn stop() {
    close(fd);
    fd = -(1 as libc::c_int);
}
unsafe extern "C" fn start(mut sample_rate: libc::c_int) {
    if fd >= 0 as libc::c_int {
        stop();
    }
    fd = open(pipename, 0o1 as libc::c_int | 0o4000 as libc::c_int);
    if fd < 0 as libc::c_int && *__errno_location() != 6 as libc::c_int {
        perror(b"open\0" as *const u8 as *const libc::c_char);
        die(
            b"could not open specified pipe for writing\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    if fd >= 0 as libc::c_int {
        close(fd);
        fd = open(pipename, 0o1 as libc::c_int);
    }
    Fs = sample_rate;
    starttime = 0 as libc::c_int as libc::c_longlong;
    samples_played = 0 as libc::c_int as libc::c_longlong;
}
unsafe extern "C" fn wait_samples(mut samples: libc::c_int) {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    let mut nowtime: libc::c_longlong = (tv.tv_usec as libc::c_double
        + 1e6f64 * tv.tv_sec as libc::c_double) as libc::c_longlong;
    if starttime == 0 {
        starttime = nowtime;
    }
    samples_played += samples as libc::c_longlong;
    let mut finishtime: libc::c_longlong = (starttime as libc::c_double
        + samples_played as libc::c_double * 1e6f64 / Fs as libc::c_double)
        as libc::c_longlong;
    usleep((finishtime - nowtime) as __useconds_t);
}
unsafe extern "C" fn play(mut buf: *mut libc::c_short, mut samples: libc::c_int) {
    if fd < 0 as libc::c_int {
        wait_samples(samples);
        if samples_played > (5 as libc::c_int * Fs) as libc::c_longlong {
            start(Fs);
        }
        return;
    }
    if write(fd, buf as *const libc::c_void, (samples * 4 as libc::c_int) as size_t)
        < 0 as libc::c_int as libc::c_long
    {
        stop();
    }
}
unsafe extern "C" fn init(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
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
    if argc != 1 as libc::c_int {
        die(
            b"bad argument(s) to pipe\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    pipename = strdup(*argv.offset(0 as libc::c_int as isize));
    if stat(pipename, &mut sb) < 0 as libc::c_int {
        die(
            b"could not stat() pipe\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if !(sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o10000 as libc::c_int as libc::c_uint)
    {
        die(b"not a pipe\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn deinit() {
    if fd >= 0 as libc::c_int {
        stop();
    }
    if !pipename.is_null() {
        free(pipename as *mut libc::c_void);
    }
}
unsafe extern "C" fn help() {
    printf(
        b"    pipe takes 1 argument: the name of the FIFO to write to.\n\0" as *const u8
            as *const libc::c_char,
    );
}
pub static mut audio_pipe: audio_output = {
    let mut init = audio_output {
        help: Some(help as unsafe extern "C" fn() -> ()),
        name: b"pipe\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        init: Some(
            init
                as unsafe extern "C" fn(
                    libc::c_int,
                    *mut *mut libc::c_char,
                ) -> libc::c_int,
        ),
        deinit: Some(deinit as unsafe extern "C" fn() -> ()),
        start: Some(start as unsafe extern "C" fn(libc::c_int) -> ()),
        play: Some(play as unsafe extern "C" fn(*mut libc::c_short, libc::c_int) -> ()),
        stop: Some(stop as unsafe extern "C" fn() -> ()),
        volume: None,
    };
    init
};
