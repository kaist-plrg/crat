use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn execl(
        __path: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn fork() -> __pid_t;
    fn ixsysdep_time(pimicros: *mut libc::c_long) -> libc::c_long;
    static mut gnu_optarg: *mut libc::c_char;
    fn gnu_getopt(
        argc: libc::c_int,
        argv: *const *mut libc::c_char,
        shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn times(__buffer: *mut tms) -> clock_t;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
}
pub type size_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type pointer = *mut libc::c_void;
pub type clock_t = __clock_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
pub type boolean = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sbuf {
    pub qnext: *mut sbuf,
    pub cstart: libc::c_int,
    pub cend: libc::c_int,
    pub ab: [libc::c_char; 512],
}
pub const _ISprint: C2RustUnnamed = 16384;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tms {
    pub tms_utime: clock_t,
    pub tms_stime: clock_t,
    pub tms_cutime: clock_t,
    pub tms_cstime: clock_t,
}
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub static mut tstuu_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: tstuu.c,v 1.89 2002/03/05 19:10:41 ian Rel $\0")
};
static mut zDebug: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut iTest: libc::c_int = 0;
static mut fCall_uucico: boolean = 0;
static mut iPercent: libc::c_int = 0;
static mut iPid1: pid_t = 0;
static mut iPid2: pid_t = 0;
static mut cFrom1: libc::c_int = 0;
static mut cFrom2: libc::c_int = 0;
static mut abLogout1: [libc::c_char; 18] = [0; 18];
static mut abLogout2: [libc::c_char; 18] = [0; 18];
static mut zProtocols: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut iopt: libc::c_int = 0;
    let mut zcmd1: *const libc::c_char = 0 as *const libc::c_char;
    let mut zcmd2: *const libc::c_char = 0 as *const libc::c_char;
    let mut zsys: *const libc::c_char = 0 as *const libc::c_char;
    let mut fmake: boolean = 1 as libc::c_int;
    let mut omaster1: libc::c_int = 0;
    let mut oslave1: libc::c_int = 0;
    let mut omaster2: libc::c_int = 0;
    let mut oslave2: libc::c_int = 0;
    let mut abpty1: [libc::c_char; 11] = [0; 11];
    let mut abpty2: [libc::c_char; 11] = [0; 11];
    let mut qbuf1: *mut sbuf = 0 as *mut sbuf;
    let mut qbuf2: *mut sbuf = 0 as *mut sbuf;
    zcmd1 = 0 as *const libc::c_char;
    zcmd2 = 0 as *const libc::c_char;
    zsys = b"test2\0" as *const u8 as *const libc::c_char;
    loop {
        iopt = gnu_getopt(
            argc,
            argv,
            b"c:np:s:t:ux:1:2:\0" as *const u8 as *const libc::c_char,
        );
        if !(iopt != -(1 as libc::c_int)) {
            break;
        }
        match iopt {
            99 => {
                zProtocols = gnu_optarg;
            }
            110 => {
                fmake = 0 as libc::c_int;
            }
            112 => {
                iPercent = strtol(
                    gnu_optarg,
                    0 as *mut libc::c_void as *mut *mut libc::c_char,
                    10 as libc::c_int,
                ) as libc::c_int;
                srand(
                    ixsysdep_time(0 as *mut libc::c_void as *mut libc::c_long)
                        as libc::c_uint,
                );
            }
            115 => {
                zsys = gnu_optarg;
            }
            116 => {
                iTest = strtol(
                    gnu_optarg,
                    0 as *mut libc::c_void as *mut *mut libc::c_char,
                    10 as libc::c_int,
                ) as libc::c_int;
            }
            117 => {
                fCall_uucico = 1 as libc::c_int;
            }
            120 => {
                zDebug = gnu_optarg;
            }
            49 => {
                zcmd1 = gnu_optarg;
            }
            50 => {
                zcmd2 = gnu_optarg;
            }
            _ => {
                fprintf(
                    stderr,
                    b"Taylor UUCP %s, copyright (C) 1991, 92, 93, 94, 1995 Ian Lance Taylor\n\0"
                        as *const u8 as *const libc::c_char,
                    b"1.07\0" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    stderr,
                    b"Usage: tstuu [-xn] [-t #] [-u] [-1 cmd] [-2 cmd]\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        }
    }
    if fCall_uucico != 0 && zcmd2.is_null() {
        zcmd2 = b"login uucp\0" as *const u8 as *const libc::c_char;
    }
    uprepare_test(fmake, iTest, fCall_uucico, zsys);
    remove(b"/usr/tmp/tstuu/spool1/core\0" as *const u8 as *const libc::c_char);
    remove(b"/usr/tmp/tstuu/spool2/core\0" as *const u8 as *const libc::c_char);
    omaster1 = -(1 as libc::c_int);
    oslave1 = -(1 as libc::c_int);
    omaster2 = -(1 as libc::c_int);
    oslave2 = -(1 as libc::c_int);
    let mut zptyname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zpty: *const libc::c_char = 0 as *const libc::c_char;
    zptyname = abpty1.as_mut_ptr();
    zpty = b"pqrs\0" as *const u8 as *const libc::c_char;
    while *zpty as libc::c_int != '\0' as i32 {
        let mut ipty: libc::c_int = 0;
        ipty = 0 as libc::c_int;
        while ipty < 16 as libc::c_int {
            let mut om: libc::c_int = 0;
            let mut os: libc::c_int = 0;
            let mut e: *mut FILE = 0 as *mut FILE;
            sprintf(
                zptyname,
                b"/dev/pty%c%c\0" as *const u8 as *const libc::c_char,
                *zpty as libc::c_int,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"0123456789abcdef\0"))[ipty as usize] as libc::c_int,
            );
            om = open(zptyname, 0o2 as libc::c_int);
            if !(om < 0 as libc::c_int) {
                *zptyname.offset(5 as libc::c_int as isize) = 't' as i32 as libc::c_char;
                os = open(zptyname, 0o2 as libc::c_int);
                if os < 0 as libc::c_int {
                    close(om);
                } else if omaster1 == -(1 as libc::c_int) {
                    omaster1 = om;
                    oslave1 = os;
                    e = fopen(
                        b"/usr/tmp/tstuu/pty1\0" as *const u8 as *const libc::c_char,
                        b"w\0" as *const u8 as *const libc::c_char,
                    );
                    if e.is_null() {
                        perror(b"fopen\0" as *const u8 as *const libc::c_char);
                        exit(1 as libc::c_int);
                    }
                    fprintf(
                        e,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        zptyname.offset(5 as libc::c_int as isize),
                    );
                    if fclose(e) != 0 as libc::c_int {
                        perror(b"fclose\0" as *const u8 as *const libc::c_char);
                        exit(1 as libc::c_int);
                    }
                    zptyname = abpty2.as_mut_ptr();
                } else {
                    omaster2 = om;
                    oslave2 = os;
                    e = fopen(
                        b"/usr/tmp/tstuu/pty2\0" as *const u8 as *const libc::c_char,
                        b"w\0" as *const u8 as *const libc::c_char,
                    );
                    if e.is_null() {
                        perror(b"fopen\0" as *const u8 as *const libc::c_char);
                        exit(1 as libc::c_int);
                    }
                    fprintf(
                        e,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        zptyname.offset(5 as libc::c_int as isize),
                    );
                    if fclose(e) != 0 as libc::c_int {
                        perror(b"fclose\0" as *const u8 as *const libc::c_char);
                        exit(1 as libc::c_int);
                    }
                    break;
                }
            }
            ipty += 1;
            ipty;
        }
        if omaster1 != -(1 as libc::c_int) && omaster2 != -(1 as libc::c_int) {
            break;
        }
        zpty = zpty.offset(1);
        zpty;
    }
    if omaster2 == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"No pseudo-terminals available\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if omaster1 > 15 as libc::c_int || omaster2 > 15 as libc::c_int {
        fprintf(
            stderr,
            b"File descriptors are too large\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if zcmd1.is_null()
        || strncmp(
            zcmd1,
            b"login\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) != 0 as libc::c_int
    {
        abLogout1[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    } else {
        sprintf(
            abLogout1.as_mut_ptr(),
            b"tstout %s\0" as *const u8 as *const libc::c_char,
            abpty1.as_mut_ptr(),
        );
    }
    if zcmd2.is_null()
        || strncmp(
            zcmd2,
            b"login\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) != 0 as libc::c_int
    {
        abLogout2[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    } else {
        sprintf(
            abLogout2.as_mut_ptr(),
            b"tstout %s\0" as *const u8 as *const libc::c_char,
            abpty2.as_mut_ptr(),
        );
    }
    iPid1 = fork();
    if iPid1 < 0 as libc::c_int {
        perror(b"fork\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    } else if iPid1 == 0 as libc::c_int {
        if close(0 as libc::c_int) < 0 as libc::c_int
            || close(1 as libc::c_int) < 0 as libc::c_int
            || close(omaster1) < 0 as libc::c_int || close(omaster2) < 0 as libc::c_int
            || close(oslave2) < 0 as libc::c_int
        {
            perror(b"close\0" as *const u8 as *const libc::c_char);
        }
        if dup2(oslave1, 0 as libc::c_int) < 0 as libc::c_int
            || dup2(oslave1, 1 as libc::c_int) < 0 as libc::c_int
        {
            perror(b"dup2\0" as *const u8 as *const libc::c_char);
        }
        if close(oslave1) < 0 as libc::c_int {
            perror(b"close\0" as *const u8 as *const libc::c_char);
        }
        sleep(3 as libc::c_int as libc::c_uint);
        if !zDebug.is_null() {
            fprintf(
                stderr,
                b"About to exec first process\n\0" as *const u8 as *const libc::c_char,
            );
        }
        if !zcmd1.is_null() {
            exit(system(zcmd1 as *mut libc::c_char));
        } else {
            execl(
                b"uucico\0" as *const u8 as *const libc::c_char,
                b"uucico\0" as *const u8 as *const libc::c_char,
                b"-I\0" as *const u8 as *const libc::c_char,
                b"/usr/tmp/tstuu/Config1\0" as *const u8 as *const libc::c_char,
                b"-q\0" as *const u8 as *const libc::c_char,
                b"-S\0" as *const u8 as *const libc::c_char,
                zsys,
                b"-pstdin\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_void as *const libc::c_char,
            );
            perror(b"execl failed\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
    }
    iPid2 = fork();
    if iPid2 < 0 as libc::c_int {
        perror(b"fork\0" as *const u8 as *const libc::c_char);
        kill(iPid1, 15 as libc::c_int);
        exit(1 as libc::c_int);
    } else if iPid2 == 0 as libc::c_int {
        if close(0 as libc::c_int) < 0 as libc::c_int
            || close(1 as libc::c_int) < 0 as libc::c_int
            || close(omaster1) < 0 as libc::c_int || close(oslave1) < 0 as libc::c_int
            || close(omaster2) < 0 as libc::c_int
        {
            perror(b"close\0" as *const u8 as *const libc::c_char);
        }
        if dup2(oslave2, 0 as libc::c_int) < 0 as libc::c_int
            || dup2(oslave2, 1 as libc::c_int) < 0 as libc::c_int
        {
            perror(b"dup2\0" as *const u8 as *const libc::c_char);
        }
        if close(oslave2) < 0 as libc::c_int {
            perror(b"close\0" as *const u8 as *const libc::c_char);
        }
        sleep(5 as libc::c_int as libc::c_uint);
        if !zDebug.is_null() {
            fprintf(
                stderr,
                b"About to exec second process\n\0" as *const u8 as *const libc::c_char,
            );
        }
        if fCall_uucico != 0 {
            execl(
                b"/bin/login\0" as *const u8 as *const libc::c_char,
                b"login\0" as *const u8 as *const libc::c_char,
                b"uucp\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_void as *const libc::c_char,
            );
            perror(b"execl failed\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        } else if !zcmd2.is_null() {
            exit(system(zcmd2 as *mut libc::c_char));
        } else {
            execl(
                b"uucico\0" as *const u8 as *const libc::c_char,
                b"uucico\0" as *const u8 as *const libc::c_char,
                b"-I\0" as *const u8 as *const libc::c_char,
                b"/usr/tmp/tstuu/Config2\0" as *const u8 as *const libc::c_char,
                b"-lq\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_void as *const libc::c_char,
            );
            perror(b"execl failed\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
    }
    signal(17 as libc::c_int, Some(uchild as unsafe extern "C" fn(libc::c_int) -> ()));
    if fcntl(omaster1, 4 as libc::c_int, 0o4000 as libc::c_int | 0o4000 as libc::c_int)
        < 0 as libc::c_int && *__errno_location() == 22 as libc::c_int
    {
        fcntl(omaster1, 4 as libc::c_int, 0o4000 as libc::c_int);
    }
    if fcntl(omaster2, 4 as libc::c_int, 0o4000 as libc::c_int | 0o4000 as libc::c_int)
        < 0 as libc::c_int && *__errno_location() == 22 as libc::c_int
    {
        fcntl(omaster2, 4 as libc::c_int, 0o4000 as libc::c_int);
    }
    qbuf1 = 0 as *mut sbuf;
    qbuf2 = 0 as *mut sbuf;
    loop {
        let mut o1: libc::c_int = 0;
        let mut o2: libc::c_int = 0;
        let mut fcont: boolean = 0;
        o1 = omaster1;
        o2 = omaster2;
        uchoose(&mut o1, &mut o2);
        if o1 == -(1 as libc::c_int) && o2 == -(1 as libc::c_int) {
            if !zDebug.is_null() {
                fprintf(
                    stderr,
                    b"Five second pause\n\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            if o1 != -(1 as libc::c_int) {
                cFrom1 = (cFrom1 as libc::c_long + cread(omaster1, &mut qbuf1))
                    as libc::c_int;
            }
            if o2 != -(1 as libc::c_int) {
                cFrom2 = (cFrom2 as libc::c_long + cread(omaster2, &mut qbuf2))
                    as libc::c_int;
            }
            loop {
                fcont = 0 as libc::c_int;
                if !qbuf1.is_null() && fwritable(omaster2) != 0
                    && fsend(omaster2, oslave2, &mut qbuf1) != 0
                {
                    fcont = 1 as libc::c_int;
                }
                if !qbuf2.is_null() && fwritable(omaster1) != 0
                    && fsend(omaster1, oslave1, &mut qbuf2) != 0
                {
                    fcont = 1 as libc::c_int;
                }
                if fcont == 0 && (!qbuf1.is_null() || !qbuf2.is_null()) {
                    let mut cgot1: libc::c_long = 0;
                    let mut cgot2: libc::c_long = 0;
                    cgot1 = cread(omaster1, &mut qbuf1);
                    cFrom1 = (cFrom1 as libc::c_long + cgot1) as libc::c_int;
                    cgot2 = cread(omaster2, &mut qbuf2);
                    cFrom2 = (cFrom2 as libc::c_long + cgot2) as libc::c_int;
                    fcont = 1 as libc::c_int;
                }
                if !(fcont != 0) {
                    break;
                }
            }
        }
    };
}
unsafe extern "C" fn uchild(mut isig: libc::c_int) {
    let mut sbase: tms = tms {
        tms_utime: 0,
        tms_stime: 0,
        tms_cutime: 0,
        tms_cstime: 0,
    };
    let mut s1: tms = tms {
        tms_utime: 0,
        tms_stime: 0,
        tms_cutime: 0,
        tms_cstime: 0,
    };
    let mut s2: tms = tms {
        tms_utime: 0,
        tms_stime: 0,
        tms_cutime: 0,
        tms_cstime: 0,
    };
    signal(17 as libc::c_int, None);
    sleep(2 as libc::c_int as libc::c_uint);
    kill(iPid1, 15 as libc::c_int);
    kill(iPid2, 15 as libc::c_int);
    times(&mut sbase);
    waitpid(iPid1, 0 as *mut libc::c_void as *mut libc::c_int, 0 as libc::c_int);
    times(&mut s1);
    waitpid(iPid2, 0 as *mut libc::c_void as *mut libc::c_int, 0 as libc::c_int);
    times(&mut s2);
    fprintf(
        stderr,
        b" First child: user: %g; system: %g\n\0" as *const u8 as *const libc::c_char,
        (s1.tms_cutime - sbase.tms_cutime) as libc::c_double
            / 60 as libc::c_int as libc::c_double,
        (s1.tms_cstime - sbase.tms_cstime) as libc::c_double
            / 60 as libc::c_int as libc::c_double,
    );
    fprintf(
        stderr,
        b"Second child: user: %g; system: %g\n\0" as *const u8 as *const libc::c_char,
        (s2.tms_cutime - s1.tms_cutime) as libc::c_double
            / 60 as libc::c_int as libc::c_double,
        (s2.tms_cstime - s1.tms_cstime) as libc::c_double
            / 60 as libc::c_int as libc::c_double,
    );
    ucheck_test(iTest, fCall_uucico);
    if abLogout1[0 as libc::c_int as usize] as libc::c_int != '\0' as i32 {
        if !zDebug.is_null() {
            fprintf(
                stderr,
                b"Executing %s\n\0" as *const u8 as *const libc::c_char,
                abLogout1.as_mut_ptr(),
            );
        }
        system(abLogout1.as_mut_ptr());
    }
    if abLogout2[0 as libc::c_int as usize] as libc::c_int != '\0' as i32 {
        if !zDebug.is_null() {
            fprintf(
                stderr,
                b"Executing %s\n\0" as *const u8 as *const libc::c_char,
                abLogout2.as_mut_ptr(),
            );
        }
        system(abLogout2.as_mut_ptr());
    }
    fprintf(
        stderr,
        b"Wrote %d bytes from 1 to 2\n\0" as *const u8 as *const libc::c_char,
        cFrom1,
    );
    fprintf(
        stderr,
        b"Wrote %d bytes from 2 to 1\n\0" as *const u8 as *const libc::c_char,
        cFrom2,
    );
    if access(
        b"/usr/tmp/tstuu/spool1/core\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int,
    ) == 0 as libc::c_int
    {
        fprintf(stderr, b"core file 1 exists\n\0" as *const u8 as *const libc::c_char);
    }
    if access(
        b"/usr/tmp/tstuu/spool2/core\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int,
    ) == 0 as libc::c_int
    {
        fprintf(stderr, b"core file 2 exists\n\0" as *const u8 as *const libc::c_char);
    }
    exit(0 as libc::c_int);
}
unsafe extern "C" fn xfopen(
    mut zname: *const libc::c_char,
    mut zmode: *const libc::c_char,
) -> *mut FILE {
    let mut eret: *mut FILE = 0 as *mut FILE;
    eret = fopen(zname, zmode);
    if eret.is_null() {
        perror(zname);
        exit(1 as libc::c_int);
    }
    return eret;
}
unsafe extern "C" fn xfclose(mut e: *mut FILE) {
    if fclose(e) != 0 as libc::c_int {
        perror(b"fclose\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn umake_file(mut z: *const libc::c_char, mut c: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut e: *mut FILE = 0 as *mut FILE;
    e = xfopen(z, b"w\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        let mut i2: libc::c_int = 0;
        i2 = 0 as libc::c_int;
        while i2 < 256 as libc::c_int {
            putc(i, e);
            i2 += 1;
            i2;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < c {
        putc(i, e);
        i += 1;
        i;
    }
    xfclose(e);
}
unsafe extern "C" fn ucheck_file(
    mut z: *const libc::c_char,
    mut zerr: *const libc::c_char,
    mut c: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut e: *mut FILE = 0 as *mut FILE;
    e = xfopen(z, b"r\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        let mut i2: libc::c_int = 0;
        i2 = 0 as libc::c_int;
        while i2 < 256 as libc::c_int {
            let mut bread: libc::c_int = 0;
            bread = getc(e);
            if bread == -(1 as libc::c_int) {
                fprintf(
                    stderr,
                    b"%s: Unexpected EOF at position %d,%d\n\0" as *const u8
                        as *const libc::c_char,
                    zerr,
                    i,
                    i2,
                );
                xfclose(e);
                return;
            }
            if bread != i {
                fprintf(
                    stderr,
                    b"%s: At position %d,%d got %d expected %d\n\0" as *const u8
                        as *const libc::c_char,
                    zerr,
                    i,
                    i2,
                    bread,
                    i,
                );
            }
            i2 += 1;
            i2;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < c {
        let mut bread_0: libc::c_int = 0;
        bread_0 = getc(e);
        if bread_0 == -(1 as libc::c_int) {
            fprintf(
                stderr,
                b"%s: Unexpected EOF at extra %d\n\0" as *const u8
                    as *const libc::c_char,
                zerr,
                i,
            );
            xfclose(e);
            return;
        }
        if bread_0 != i {
            fprintf(
                stderr,
                b"%s: At extra %d got %d expected %d\n\0" as *const u8
                    as *const libc::c_char,
                zerr,
                i,
                bread_0,
                i,
            );
        }
        i += 1;
        i;
    }
    if getc(e) != -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"%s: File is too long\0" as *const u8 as *const libc::c_char,
            zerr,
        );
    }
    xfclose(e);
}
unsafe extern "C" fn uprepare_test(
    mut fmake: boolean,
    mut itest: libc::c_int,
    mut fcall_uucico: boolean,
    mut zsys: *const libc::c_char,
) {
    let mut e: *mut FILE = 0 as *mut FILE;
    let mut zuucp1: *const libc::c_char = 0 as *const libc::c_char;
    let mut zuucp2: *const libc::c_char = 0 as *const libc::c_char;
    let mut zuux1: *const libc::c_char = 0 as *const libc::c_char;
    let mut zuux2: *const libc::c_char = 0 as *const libc::c_char;
    let mut ab: [libc::c_char; 1000] = [0; 1000];
    let mut zfrom: *const libc::c_char = 0 as *const libc::c_char;
    let mut zto: *const libc::c_char = 0 as *const libc::c_char;
    umask(0 as libc::c_int as __mode_t);
    if mkdir(
        b"/usr/tmp/tstuu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as __mode_t,
    ) != 0 as libc::c_int && *__errno_location() != 17 as libc::c_int
    {
        perror(b"mkdir\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if mkdir(
        b"/usr/tmp/tstuu/spool1\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int >> 3 as libc::c_int) as __mode_t,
    ) != 0 as libc::c_int && *__errno_location() != 17 as libc::c_int
    {
        perror(b"mkdir\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if mkdir(
        b"/usr/tmp/tstuu/spool2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int >> 3 as libc::c_int) as __mode_t,
    ) != 0 as libc::c_int && *__errno_location() != 17 as libc::c_int
    {
        perror(b"mkdir\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if fmake != 0 {
        e = xfopen(
            b"/usr/tmp/tstuu/Config1\0" as *const u8 as *const libc::c_char,
            b"w\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            e,
            b"# First test configuration file\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(e, b"nodename test1\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            e,
            b"spool /usr/tmp/tstuu/spool1\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            e,
            b"lockdir /usr/tmp/tstuu/spool1\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            e,
            b"sysfile /usr/tmp/tstuu/System1\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            e,
            b"sysfile /usr/tmp/tstuu/System1.2\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            e,
            b"portfile /usr/tmp/tstuu/Port1\n\0" as *const u8 as *const libc::c_char,
        );
        remove(b"/usr/tmp/tstuu/Log1\0" as *const u8 as *const libc::c_char);
        fprintf(
            e,
            b"logfile /usr/tmp/tstuu/Log1\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            e,
            b"statfile /usr/tmp/tstuu/Stats1\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            e,
            b"debugfile /usr/tmp/tstuu/Debug1\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            e,
            b"callfile /usr/tmp/tstuu/Call1\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(e, b"pubdir /usr/tmp/tstuu\n\0" as *const u8 as *const libc::c_char);
        if !zDebug.is_null() {
            fprintf(e, b"debug %s\n\0" as *const u8 as *const libc::c_char, zDebug);
        }
        xfclose(e);
        e = xfopen(
            b"/usr/tmp/tstuu/System1\0" as *const u8 as *const libc::c_char,
            b"w\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            e,
            b"# This file is ignored, to test multiple system files\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(e, b"time never\n\0" as *const u8 as *const libc::c_char);
        xfclose(e);
        e = xfopen(
            b"/usr/tmp/tstuu/System1.2\0" as *const u8 as *const libc::c_char,
            b"w\0" as *const u8 as *const libc::c_char,
        );
        fprintf(e, b"# First test system file\n\0" as *const u8 as *const libc::c_char);
        fprintf(e, b"time any\n\0" as *const u8 as *const libc::c_char);
        fprintf(e, b"port stdin\n\0" as *const u8 as *const libc::c_char);
        fprintf(e, b"# That was the defaults\n\0" as *const u8 as *const libc::c_char);
        fprintf(e, b"system %s\n\0" as *const u8 as *const libc::c_char, zsys);
        if fcall_uucico == 0 {
            let mut eprog: *mut FILE = 0 as *mut FILE;
            eprog = xfopen(
                b"/usr/tmp/tstuu/Chat1\0" as *const u8 as *const libc::c_char,
                b"w\0" as *const u8 as *const libc::c_char,
            );
            fprintf(eprog, b"sleep 2\n\0" as *const u8 as *const libc::c_char);
            fprintf(
                eprog,
                b"echo password $1 speed $2 1>&2\n\0" as *const u8 as *const libc::c_char,
            );
            fprintf(eprog, b"echo test1\n\0" as *const u8 as *const libc::c_char);
            fprintf(eprog, b"exit 0\n\0" as *const u8 as *const libc::c_char);
            xfclose(eprog);
            if chmod(
                b"/usr/tmp/tstuu/Chat1\0" as *const u8 as *const libc::c_char,
                (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int
                    | 0o100 as libc::c_int >> 3 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                    as __mode_t,
            ) < 0 as libc::c_int
            {
                perror(
                    b"chmod (/usr/tmp/tstuu/Chat1)\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            fprintf(
                e,
                b"chat-program /usr/tmp/tstuu/Chat1 \\P \\S\n\0" as *const u8
                    as *const libc::c_char,
            );
            fprintf(e, b"chat word: \\P\n\0" as *const u8 as *const libc::c_char);
            fprintf(e, b"chat-fail login;\n\0" as *const u8 as *const libc::c_char);
            fprintf(e, b"call-login *\n\0" as *const u8 as *const libc::c_char);
            fprintf(e, b"call-password *\n\0" as *const u8 as *const libc::c_char);
        } else {
            fprintf(e, b"chat \"\"\n\0" as *const u8 as *const libc::c_char);
        }
        fprintf(e, b"call-transfer yes\n\0" as *const u8 as *const libc::c_char);
        fprintf(e, b"commands cat\n\0" as *const u8 as *const libc::c_char);
        if fcall_uucico == 0 && iPercent == 0 as libc::c_int {
            fprintf(
                e,
                b"protocol-parameter g window 7\n\0" as *const u8 as *const libc::c_char,
            );
            fprintf(
                e,
                b"protocol-parameter g packet-size 4096\n\0" as *const u8
                    as *const libc::c_char,
            );
            fprintf(
                e,
                b"protocol-parameter j avoid \\377\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if !zProtocols.is_null() {
            fprintf(
                e,
                b"protocol %s\n\0" as *const u8 as *const libc::c_char,
                zProtocols,
            );
        }
        xfclose(e);
        e = xfopen(
            b"/usr/tmp/tstuu/Port1\0" as *const u8 as *const libc::c_char,
            b"w\0" as *const u8 as *const libc::c_char,
        );
        fprintf(e, b"port stdin\n\0" as *const u8 as *const libc::c_char);
        fprintf(e, b"type stdin\n\0" as *const u8 as *const libc::c_char);
        xfclose(e);
        e = xfopen(
            b"/usr/tmp/tstuu/Call1\0" as *const u8 as *const libc::c_char,
            b"w\0" as *const u8 as *const libc::c_char,
        );
        fprintf(e, b"Call out password file\n\0" as *const u8 as *const libc::c_char);
        fprintf(e, b"%s test1 pass\\s1\n\0" as *const u8 as *const libc::c_char, zsys);
        xfclose(e);
        if fcall_uucico == 0 {
            let mut eprog_0: *mut FILE = 0 as *mut FILE;
            e = xfopen(
                b"/usr/tmp/tstuu/Config2\0" as *const u8 as *const libc::c_char,
                b"w\0" as *const u8 as *const libc::c_char,
            );
            fprintf(
                e,
                b"# Second test configuration file\n\0" as *const u8
                    as *const libc::c_char,
            );
            fprintf(e, b"nodename test2\n\0" as *const u8 as *const libc::c_char);
            fprintf(
                e,
                b"spool /usr/tmp/tstuu/spool2\n\0" as *const u8 as *const libc::c_char,
            );
            fprintf(
                e,
                b"lockdir /usr/tmp/tstuu/spool2\n\0" as *const u8 as *const libc::c_char,
            );
            fprintf(
                e,
                b"sysfile /usr/tmp/tstuu/System2\n\0" as *const u8 as *const libc::c_char,
            );
            remove(b"/usr/tmp/tstuu/Log2\0" as *const u8 as *const libc::c_char);
            fprintf(
                e,
                b"logfile /usr/tmp/tstuu/Log2\n\0" as *const u8 as *const libc::c_char,
            );
            fprintf(
                e,
                b"statfile /usr/tmp/tstuu/Stats2\n\0" as *const u8 as *const libc::c_char,
            );
            fprintf(
                e,
                b"debugfile /usr/tmp/tstuu/Debug2\n\0" as *const u8
                    as *const libc::c_char,
            );
            fprintf(
                e,
                b"passwdfile /usr/tmp/tstuu/Pass2\n\0" as *const u8
                    as *const libc::c_char,
            );
            fprintf(e, b"pubdir /usr/tmp/tstuu\n\0" as *const u8 as *const libc::c_char);
            if !zDebug.is_null() {
                fprintf(e, b"debug %s\n\0" as *const u8 as *const libc::c_char, zDebug);
            }
            xfclose(e);
            e = xfopen(
                b"/usr/tmp/tstuu/System2\0" as *const u8 as *const libc::c_char,
                b"w\0" as *const u8 as *const libc::c_char,
            );
            fprintf(
                e,
                b"# Second test system file\n\0" as *const u8 as *const libc::c_char,
            );
            fprintf(e, b"system test1\n\0" as *const u8 as *const libc::c_char);
            fprintf(e, b"called-login test1\n\0" as *const u8 as *const libc::c_char);
            fprintf(e, b"request true\n\0" as *const u8 as *const libc::c_char);
            fprintf(e, b"commands cat\n\0" as *const u8 as *const libc::c_char);
            if !zProtocols.is_null() {
                fprintf(
                    e,
                    b"protocol %s\n\0" as *const u8 as *const libc::c_char,
                    zProtocols,
                );
            }
            eprog_0 = xfopen(
                b"/usr/tmp/tstuu/Chat2\0" as *const u8 as *const libc::c_char,
                b"w\0" as *const u8 as *const libc::c_char,
            );
            fprintf(
                eprog_0,
                b"echo port $1 1>&2\n\0" as *const u8 as *const libc::c_char,
            );
            fprintf(eprog_0, b"exit 0\n\0" as *const u8 as *const libc::c_char);
            xfclose(eprog_0);
            if chmod(
                b"/usr/tmp/tstuu/Chat2\0" as *const u8 as *const libc::c_char,
                (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int
                    | 0o100 as libc::c_int >> 3 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                    as __mode_t,
            ) < 0 as libc::c_int
            {
                perror(
                    b"chmod (/usr/tmp/tstuu/Chat2\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            fprintf(
                e,
                b"called-chat-program /bin/sh /usr/tmp/tstuu/Chat2 \\Y\n\0" as *const u8
                    as *const libc::c_char,
            );
            fprintf(e, b"time any\n\0" as *const u8 as *const libc::c_char);
            xfclose(e);
            e = xfopen(
                b"/usr/tmp/tstuu/Pass2\0" as *const u8 as *const libc::c_char,
                b"w\0" as *const u8 as *const libc::c_char,
            );
            fprintf(
                e,
                b"# Call in password file\n\0" as *const u8 as *const libc::c_char,
            );
            fprintf(e, b"test1 pass\\s1\n\0" as *const u8 as *const libc::c_char);
            xfclose(e);
        }
    }
    zuucp1 = b"./uucp -I /usr/tmp/tstuu/Config1 -r\0" as *const u8
        as *const libc::c_char;
    zuux1 = b"./uux -I /usr/tmp/tstuu/Config1 -r\0" as *const u8 as *const libc::c_char;
    if fcall_uucico != 0 {
        zuucp2 = b"/usr/bin/uucp -r\0" as *const u8 as *const libc::c_char;
        zuux2 = b"/usr/bin/uux -r\0" as *const u8 as *const libc::c_char;
    } else {
        zuucp2 = b"./uucp -I /usr/tmp/tstuu/Config2 -r\0" as *const u8
            as *const libc::c_char;
        zuux2 = b"./uux -I /usr/tmp/tstuu/Config2 -r\0" as *const u8
            as *const libc::c_char;
    }
    if itest == 0 as libc::c_int || itest == 1 as libc::c_int {
        zfrom = b"/usr/tmp/tstuu/from1\0" as *const u8 as *const libc::c_char;
        if fcall_uucico != 0 {
            zto = b"/usr/spool/uucppublic/to1\0" as *const u8 as *const libc::c_char;
        } else {
            zto = b"/usr/tmp/tstuu/to1\0" as *const u8 as *const libc::c_char;
        }
        remove(zto);
        umake_file(zfrom, 0 as libc::c_int);
        sprintf(
            ab.as_mut_ptr(),
            b"%s %s %s!%s\0" as *const u8 as *const libc::c_char,
            zuucp1,
            zfrom,
            zsys,
            zto,
        );
        xsystem(ab.as_mut_ptr());
    }
    if itest == 0 as libc::c_int || itest == 2 as libc::c_int {
        if fcall_uucico != 0 {
            zfrom = b"/usr/spool/uucppublic/from2\0" as *const u8 as *const libc::c_char;
        } else {
            zfrom = b"/usr/tmp/tstuu/from2\0" as *const u8 as *const libc::c_char;
        }
        zto = b"/usr/tmp/tstuu/to2\0" as *const u8 as *const libc::c_char;
        remove(zto);
        umake_file(zfrom, 3 as libc::c_int);
        sprintf(
            ab.as_mut_ptr(),
            b"%s %s!%s %s\0" as *const u8 as *const libc::c_char,
            zuucp1,
            zsys,
            zfrom,
            zto,
        );
        xsystem(ab.as_mut_ptr());
    }
    if itest == 0 as libc::c_int || itest == 3 as libc::c_int {
        if fcall_uucico != 0 {
            zfrom = b"/usr/spool/uucppublic/from3\0" as *const u8 as *const libc::c_char;
        } else {
            zfrom = b"/usr/tmp/tstuu/from3\0" as *const u8 as *const libc::c_char;
        }
        zto = b"/usr/tmp/tstuu/to3\0" as *const u8 as *const libc::c_char;
        remove(zto);
        umake_file(zfrom, 5 as libc::c_int);
        sprintf(
            ab.as_mut_ptr(),
            b"%s -c \\~/from3 test1!~/to3\0" as *const u8 as *const libc::c_char,
            zuucp2,
        );
        xsystem(ab.as_mut_ptr());
    }
    if itest == 0 as libc::c_int || itest == 4 as libc::c_int {
        zfrom = b"/usr/tmp/tstuu/from4\0" as *const u8 as *const libc::c_char;
        if fcall_uucico != 0 {
            zto = b"/usr/spool/uucppublic/to4\0" as *const u8 as *const libc::c_char;
        } else {
            zto = b"/usr/tmp/tstuu/to4\0" as *const u8 as *const libc::c_char;
        }
        remove(zto);
        umake_file(zfrom, 7 as libc::c_int);
        sprintf(
            ab.as_mut_ptr(),
            b"%s test1!%s %s\0" as *const u8 as *const libc::c_char,
            zuucp2,
            zfrom,
            zto,
        );
        xsystem(ab.as_mut_ptr());
    }
    if itest == 0 as libc::c_int || itest == 5 as libc::c_int {
        zfrom = b"/usr/tmp/tstuu/from5\0" as *const u8 as *const libc::c_char;
        if fcall_uucico != 0 {
            zto = b"/usr/spool/uucppublic/to5\0" as *const u8 as *const libc::c_char;
        } else {
            zto = b"/usr/tmp/tstuu/to5\0" as *const u8 as *const libc::c_char;
        }
        remove(zto);
        umake_file(zfrom, 11 as libc::c_int);
        sprintf(
            ab.as_mut_ptr(),
            b"%s test1!cat '<%s' '>%s'\0" as *const u8 as *const libc::c_char,
            zuux2,
            zfrom,
            zto,
        );
        xsystem(ab.as_mut_ptr());
    }
    if itest == 0 as libc::c_int || itest == 6 as libc::c_int {
        let mut zfrom1: *const libc::c_char = 0 as *const libc::c_char;
        let mut zfrom2: *const libc::c_char = 0 as *const libc::c_char;
        if fcall_uucico != 0 {
            zfrom = b"/usr/spool/uucppublic/to6\\*\0" as *const u8
                as *const libc::c_char;
            zfrom1 = b"/usr/spool/uucppublic/to6.1\0" as *const u8
                as *const libc::c_char;
            zfrom2 = b"/usr/spool/uucppublic/to6.2\0" as *const u8
                as *const libc::c_char;
        } else {
            zfrom = b"/usr/tmp/tstuu/spool2/to6\\*\0" as *const u8
                as *const libc::c_char;
            zfrom1 = b"/usr/tmp/tstuu/spool2/to6.1\0" as *const u8
                as *const libc::c_char;
            zfrom2 = b"/usr/tmp/tstuu/spool2/to6.2\0" as *const u8
                as *const libc::c_char;
        }
        umake_file(zfrom1, 100 as libc::c_int);
        umake_file(zfrom2, 101 as libc::c_int);
        remove(b"/usr/tmp/tstuu/to6.1\0" as *const u8 as *const libc::c_char);
        remove(b"/usr/tmp/tstuu/to6.2\0" as *const u8 as *const libc::c_char);
        sprintf(
            ab.as_mut_ptr(),
            b"%s %s!%s /usr/tmp/tstuu\0" as *const u8 as *const libc::c_char,
            zuucp1,
            zsys,
            zfrom,
        );
        xsystem(ab.as_mut_ptr());
    }
    if itest == 0 as libc::c_int || itest == 7 as libc::c_int {
        let mut zto1: *const libc::c_char = 0 as *const libc::c_char;
        let mut zto2: *const libc::c_char = 0 as *const libc::c_char;
        if fcall_uucico != 0 {
            zto = b"/usr/spool/uucppublic\0" as *const u8 as *const libc::c_char;
            zto1 = b"/usr/spool/uucppublic/to7.1\0" as *const u8 as *const libc::c_char;
            zto2 = b"/usr/spool/uucppublic/to7.2\0" as *const u8 as *const libc::c_char;
        } else {
            zto = b"/usr/tmp/tstuu\0" as *const u8 as *const libc::c_char;
            zto1 = b"/usr/tmp/tstuu/to7.1\0" as *const u8 as *const libc::c_char;
            zto2 = b"/usr/tmp/tstuu/to7.2\0" as *const u8 as *const libc::c_char;
        }
        umake_file(
            b"/usr/tmp/tstuu/spool1/to7.1\0" as *const u8 as *const libc::c_char,
            150 as libc::c_int,
        );
        umake_file(
            b"/usr/tmp/tstuu/spool1/to7.2\0" as *const u8 as *const libc::c_char,
            155 as libc::c_int,
        );
        remove(zto1);
        remove(zto2);
        sprintf(
            ab.as_mut_ptr(),
            b"%s test1!/usr/tmp/tstuu/spool1/to7.\\* %s\0" as *const u8
                as *const libc::c_char,
            zuucp2,
            zto,
        );
        xsystem(ab.as_mut_ptr());
    }
    if (itest == 0 as libc::c_int || itest == 8 as libc::c_int) && fcall_uucico == 0 {
        umake_file(
            b"/usr/tmp/tstuu/from8\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
        );
        sprintf(
            ab.as_mut_ptr(),
            b"%s - test2!cat < /usr/tmp/tstuu/from8\0" as *const u8
                as *const libc::c_char,
            zuux1,
        );
        xsystem(ab.as_mut_ptr());
    }
}
unsafe extern "C" fn ucheck_test(mut itest: libc::c_int, mut fcall_uucico: boolean) {
    if itest == 0 as libc::c_int || itest == 1 as libc::c_int {
        if fcall_uucico != 0 {
            ucheck_file(
                b"/usr/spool/uucppublic/to1\0" as *const u8 as *const libc::c_char,
                b"test 1\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        } else {
            ucheck_file(
                b"/usr/tmp/tstuu/to1\0" as *const u8 as *const libc::c_char,
                b"test 1\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
    }
    if itest == 0 as libc::c_int || itest == 2 as libc::c_int {
        ucheck_file(
            b"/usr/tmp/tstuu/to2\0" as *const u8 as *const libc::c_char,
            b"test 2\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int,
        );
    }
    if itest == 0 as libc::c_int || itest == 3 as libc::c_int {
        ucheck_file(
            b"/usr/tmp/tstuu/to3\0" as *const u8 as *const libc::c_char,
            b"test 3\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    }
    if itest == 0 as libc::c_int || itest == 4 as libc::c_int {
        if fcall_uucico != 0 {
            ucheck_file(
                b"/usr/spool/uucppublic/to4\0" as *const u8 as *const libc::c_char,
                b"test 4\0" as *const u8 as *const libc::c_char,
                7 as libc::c_int,
            );
        } else {
            ucheck_file(
                b"/usr/tmp/tstuu/to4\0" as *const u8 as *const libc::c_char,
                b"test 4\0" as *const u8 as *const libc::c_char,
                7 as libc::c_int,
            );
        }
    }
    if itest == 0 as libc::c_int || itest == 6 as libc::c_int {
        ucheck_file(
            b"/usr/tmp/tstuu/to6.1\0" as *const u8 as *const libc::c_char,
            b"test 6.1\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
        );
        ucheck_file(
            b"/usr/tmp/tstuu/to6.2\0" as *const u8 as *const libc::c_char,
            b"test 6.2\0" as *const u8 as *const libc::c_char,
            101 as libc::c_int,
        );
    }
    if itest == 0 as libc::c_int || itest == 7 as libc::c_int {
        let mut zto1: *const libc::c_char = 0 as *const libc::c_char;
        let mut zto2: *const libc::c_char = 0 as *const libc::c_char;
        if fcall_uucico != 0 {
            zto1 = b"/usr/spool/uucppublic/to7.1\0" as *const u8 as *const libc::c_char;
            zto2 = b"/usr/spool/uucppublic/to7.2\0" as *const u8 as *const libc::c_char;
        } else {
            zto1 = b"/usr/tmp/tstuu/to7.1\0" as *const u8 as *const libc::c_char;
            zto2 = b"/usr/tmp/tstuu/to7.2\0" as *const u8 as *const libc::c_char;
        }
        ucheck_file(
            zto1,
            b"test 7.1\0" as *const u8 as *const libc::c_char,
            150 as libc::c_int,
        );
        ucheck_file(
            zto2,
            b"test 7.2\0" as *const u8 as *const libc::c_char,
            155 as libc::c_int,
        );
    }
}
unsafe extern "C" fn cpshow(
    mut z: *mut libc::c_char,
    mut ichar: libc::c_int,
) -> libc::c_int {
    if *(*__ctype_b_loc()).offset(ichar as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
        && ichar != '"' as i32
    {
        *z = ichar as libc::c_char;
        return 1 as libc::c_int;
    }
    let fresh0 = z;
    z = z.offset(1);
    *fresh0 = '\\' as i32 as libc::c_char;
    match ichar {
        10 => {
            *z = 'n' as i32 as libc::c_char;
            return 2 as libc::c_int;
        }
        13 => {
            *z = 'r' as i32 as libc::c_char;
            return 2 as libc::c_int;
        }
        34 => {
            *z = '"' as i32 as libc::c_char;
            return 2 as libc::c_int;
        }
        _ => {
            sprintf(
                z,
                b"%03o\0" as *const u8 as *const libc::c_char,
                (ichar & 0xff as libc::c_int) as libc::c_uint,
            );
            return (strlen(z)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_int;
        }
    };
}
unsafe extern "C" fn uchoose(mut po1: *mut libc::c_int, mut po2: *mut libc::c_int) {
    let mut iread: libc::c_int = 0;
    let mut stime: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    iread = (1 as libc::c_int) << *po1 | (1 as libc::c_int) << *po2;
    stime.tv_sec = 5 as libc::c_int as __time_t;
    stime.tv_usec = 0 as libc::c_int as __suseconds_t;
    if select(
        (if *po1 > *po2 { *po1 } else { *po2 }) + 1 as libc::c_int,
        &mut iread as *mut libc::c_int as pointer as *mut fd_set,
        0 as *mut libc::c_void as *mut fd_set,
        0 as *mut libc::c_void as *mut fd_set,
        &mut stime,
    ) < 0 as libc::c_int
    {
        perror(b"select\0" as *const u8 as *const libc::c_char);
        uchild(17 as libc::c_int);
    }
    if iread & (1 as libc::c_int) << *po1 == 0 as libc::c_int {
        *po1 = -(1 as libc::c_int);
    }
    if iread & (1 as libc::c_int) << *po2 == 0 as libc::c_int {
        *po2 = -(1 as libc::c_int);
    }
}
unsafe extern "C" fn cread(
    mut o: libc::c_int,
    mut pqbuf: *mut *mut sbuf,
) -> libc::c_long {
    let mut ctotal: libc::c_long = 0;
    while !(*pqbuf).is_null() && !((**pqbuf).qnext).is_null() {
        pqbuf = &mut (**pqbuf).qnext;
    }
    ctotal = 0 as libc::c_int as libc::c_long;
    loop {
        let mut cgot: libc::c_int = 0;
        if !(*pqbuf).is_null()
            && (**pqbuf).cend as size_t
                >= ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong
        {
            pqbuf = &mut (**pqbuf).qnext;
        }
        if (*pqbuf).is_null() {
            *pqbuf = malloc(::std::mem::size_of::<sbuf>() as libc::c_ulong) as *mut sbuf;
            if (*pqbuf).is_null() {
                fprintf(
                    stderr,
                    b"Out of memory\n\0" as *const u8 as *const libc::c_char,
                );
                uchild(17 as libc::c_int);
            }
            (**pqbuf).qnext = 0 as *mut sbuf;
            (**pqbuf).cstart = 0 as libc::c_int;
            (**pqbuf).cend = 0 as libc::c_int;
        }
        cgot = read(
            o,
            ((**pqbuf).ab).as_mut_ptr().offset((**pqbuf).cend as isize)
                as *mut libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
                .wrapping_sub((**pqbuf).cend as libc::c_ulong),
        ) as libc::c_int;
        if cgot < 0 as libc::c_int {
            if *__errno_location() == 11 as libc::c_int
                || *__errno_location() == 11 as libc::c_int
                || *__errno_location() == 61 as libc::c_int
            {
                cgot = 0 as libc::c_int;
            } else {
                perror(b"read\0" as *const u8 as *const libc::c_char);
                uchild(17 as libc::c_int);
            }
        }
        if cgot == 0 as libc::c_int {
            return ctotal;
        }
        ctotal += cgot as libc::c_long;
        if !zDebug.is_null() {
            let mut abshow: [libc::c_char; 325] = [0; 325];
            let mut zfrom: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut zshow: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut i: libc::c_int = 0;
            zfrom = ((**pqbuf).ab).as_mut_ptr().offset((**pqbuf).cend as isize);
            zshow = abshow.as_mut_ptr();
            i = 0 as libc::c_int;
            while i < cgot && i < 80 as libc::c_int {
                zshow = zshow.offset(cpshow(zshow, *zfrom as libc::c_int) as isize);
                i += 1;
                i;
                zfrom = zfrom.offset(1);
                zfrom;
            }
            if i < cgot {
                let fresh1 = zshow;
                zshow = zshow.offset(1);
                *fresh1 = '.' as i32 as libc::c_char;
                let fresh2 = zshow;
                zshow = zshow.offset(1);
                *fresh2 = '.' as i32 as libc::c_char;
                let fresh3 = zshow;
                zshow = zshow.offset(1);
                *fresh3 = '.' as i32 as libc::c_char;
            }
            *zshow = '\0' as i32 as libc::c_char;
            fprintf(
                stderr,
                b"Read from %d: %d \"%s\"\n\0" as *const u8 as *const libc::c_char,
                o,
                cgot,
                abshow.as_mut_ptr(),
            );
            fflush(stderr);
        }
        if iPercent > 0 as libc::c_int {
            let mut i_0: libc::c_int = 0;
            let mut c: libc::c_int = 0;
            c = 0 as libc::c_int;
            i_0 = 0 as libc::c_int;
            while i_0 < cgot {
                if (rand() % 1000 as libc::c_int) < iPercent {
                    (**pqbuf).ab[((**pqbuf).cend + i_0) as usize] += 1;
                    (**pqbuf).ab[((**pqbuf).cend + i_0) as usize];
                    c += 1;
                    c;
                }
                i_0 += 1;
                i_0;
            }
            if !zDebug.is_null() && c > 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"Clobbered %d bytes\n\0" as *const u8 as *const libc::c_char,
                    c,
                );
            }
        }
        (**pqbuf).cend += cgot;
        if ctotal > 256 as libc::c_int as libc::c_long {
            return ctotal;
        }
    };
}
unsafe extern "C" fn fsend(
    mut o: libc::c_int,
    mut oslave: libc::c_int,
    mut pqbuf: *mut *mut sbuf,
) -> boolean {
    let mut ctotal: libc::c_long = 0;
    ctotal = 0 as libc::c_int as libc::c_long;
    while !(*pqbuf).is_null() {
        let mut cwrite: libc::c_int = 0;
        let mut cwrote: libc::c_int = 0;
        if (**pqbuf).cstart >= (**pqbuf).cend {
            let mut qfree: *mut sbuf = 0 as *mut sbuf;
            qfree = *pqbuf;
            *pqbuf = (**pqbuf).qnext;
            free(qfree as pointer);
        } else {
            let mut cunread: libc::c_long = 0;
            if ioctl(
                oslave,
                0x541b as libc::c_int as libc::c_ulong,
                &mut cunread as *mut libc::c_long,
            ) < 0 as libc::c_int
            {
                perror(b"FIONREAD\0" as *const u8 as *const libc::c_char);
                uchild(17 as libc::c_int);
            }
            if !zDebug.is_null() {
                fprintf(
                    stderr,
                    b"%ld unread\n\0" as *const u8 as *const libc::c_char,
                    cunread,
                );
            }
            cwrite = (256 as libc::c_int as libc::c_long - cunread) as libc::c_int;
            if cwrite <= 0 as libc::c_int {
                break;
            }
            if cwrite > (**pqbuf).cend - (**pqbuf).cstart {
                cwrite = (**pqbuf).cend - (**pqbuf).cstart;
            }
            cwrote = write(
                o,
                ((**pqbuf).ab).as_mut_ptr().offset((**pqbuf).cstart as isize)
                    as *const libc::c_void,
                cwrite as size_t,
            ) as libc::c_int;
            if cwrote < 0 as libc::c_int {
                if *__errno_location() == 11 as libc::c_int
                    || *__errno_location() == 11 as libc::c_int
                    || *__errno_location() == 61 as libc::c_int
                {
                    cwrote = 0 as libc::c_int;
                } else {
                    perror(b"write\0" as *const u8 as *const libc::c_char);
                    uchild(17 as libc::c_int);
                }
            }
            if cwrote == 0 as libc::c_int {
                break;
            }
            ctotal += cwrote as libc::c_long;
            (**pqbuf).cstart += cwrote;
        }
    }
    if !zDebug.is_null() && ctotal > 0 as libc::c_int as libc::c_long {
        fprintf(
            stderr,
            b"Wrote %ld to %d\n\0" as *const u8 as *const libc::c_char,
            ctotal,
            o,
        );
    }
    return (ctotal > 0 as libc::c_int as libc::c_long) as libc::c_int;
}
unsafe extern "C" fn fwritable(mut o: libc::c_int) -> boolean {
    let mut iwrite: libc::c_int = 0;
    let mut stime: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut cfds: libc::c_int = 0;
    iwrite = (1 as libc::c_int) << o;
    stime.tv_sec = 0 as libc::c_int as __time_t;
    stime.tv_usec = 0 as libc::c_int as __suseconds_t;
    cfds = select(
        o + 1 as libc::c_int,
        0 as *mut libc::c_void as *mut fd_set,
        &mut iwrite as *mut libc::c_int as pointer as *mut fd_set,
        0 as *mut libc::c_void as *mut fd_set,
        &mut stime,
    );
    if cfds < 0 as libc::c_int {
        perror(b"select\0" as *const u8 as *const libc::c_char);
        uchild(17 as libc::c_int);
    }
    return (cfds > 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn xsystem(mut zcmd: *const libc::c_char) {
    let mut istat: libc::c_int = 0;
    istat = system(zcmd as *mut libc::c_char);
    if istat != 0 as libc::c_int {
        fprintf(
            stderr,
            b"Command failed with status %d\n\0" as *const u8 as *const libc::c_char,
            istat,
        );
        fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, zcmd);
        exit(1 as libc::c_int);
    }
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
