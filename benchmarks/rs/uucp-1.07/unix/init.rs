use ::libc;
extern "C" {
    fn exit(_: libc::c_int) -> !;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn ulog_uuconf(ttype: tlog, puuconf: pointer, iuuconf: libc::c_int);
    fn ulog_id(iid: libc::c_int);
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> pointer;
    fn xrealloc(_: pointer, _: size_t) -> pointer;
    fn xfree(_: pointer);
    fn uuconf_localname(
        uuconf_pglobal: *mut libc::c_void,
        pzname: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_spooldir(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pzspool: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn uuconf_lockdir(
        uuconf_pglobal: *mut libc::c_void,
        uuconf_pzlock: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn getdtablesize() -> libc::c_int;
    fn getpid() -> __pid_t;
    fn getgid() -> __gid_t;
    fn setgid(__gid: __gid_t) -> libc::c_int;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    fn getlogin() -> *mut libc::c_char;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pointer = *mut libc::c_void;
pub type boolean = libc::c_int;
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
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
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
static mut zSlogin: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut zSspooldir: *const libc::c_char = 0 as *const libc::c_char;
pub static mut zSlockdir: *const libc::c_char = 0 as *const libc::c_char;
pub static mut zSlocalname: *const libc::c_char = 0 as *const libc::c_char;
pub static mut zScwd: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut cSysdep_max_name_len: size_t = 255 as libc::c_int as size_t;
pub unsafe extern "C" fn usysdep_initialize(
    mut puuconf: pointer,
    mut iflags: libc::c_int,
) {
    let mut iuuconf: libc::c_int = 0;
    let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut passwd = 0 as *mut passwd;
    ulog_id(getpid());
    if iflags & 0o10 as libc::c_int == 0 as libc::c_int {
        let mut cdescs: libc::c_int = 0;
        let mut o: libc::c_int = 0;
        cdescs = getdtablesize();
        o = 3 as libc::c_int;
        while o < cdescs {
            close(o);
            o += 1;
            o;
        }
    }
    if fcntl(0 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int) < 0 as libc::c_int
        && open(
            b"/dev/null\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
        ) != 0 as libc::c_int
    {
        exit(1 as libc::c_int);
    }
    if fcntl(1 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int) < 0 as libc::c_int
        && open(
            b"/dev/null\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0o1 as libc::c_int,
            0 as libc::c_int,
        ) != 1 as libc::c_int
    {
        exit(1 as libc::c_int);
    }
    if fcntl(2 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int) < 0 as libc::c_int
        && open(
            b"/dev/null\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0o1 as libc::c_int,
            0 as libc::c_int,
        ) != 2 as libc::c_int
    {
        exit(1 as libc::c_int);
    }
    iuuconf = uuconf_spooldir(puuconf, &mut zSspooldir);
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    iuuconf = uuconf_lockdir(puuconf, &mut zSlockdir);
    if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    iuuconf = uuconf_localname(puuconf, &mut zSlocalname);
    if iuuconf == 1 as libc::c_int {
        let mut ab: [libc::c_char; 256] = [0; 256];
        if gethostname(
            ab.as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) < 0 as libc::c_int
        {
            ulog(
                LOG_FATAL,
                b"gethostname: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        ab[(::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = '\0' as i32 as libc::c_char;
        ab[strcspn(ab.as_mut_ptr(), b".\0" as *const u8 as *const libc::c_char)
            as usize] = '\0' as i32 as libc::c_char;
        zSlocalname = zbufcpy(ab.as_mut_ptr());
    } else if iuuconf != 0 as libc::c_int {
        ulog_uuconf(LOG_FATAL, puuconf, iuuconf);
    }
    umask(0 as libc::c_int as __mode_t);
    z = getenv(b"LOGNAME\0" as *const u8 as *const libc::c_char);
    if z.is_null() {
        z = getenv(b"USER\0" as *const u8 as *const libc::c_char);
    }
    if z.is_null() {
        z = getlogin();
    }
    if z.is_null() {
        q = 0 as *mut passwd;
    } else {
        q = getpwnam(z);
        if !q.is_null() {
            z = (*q).pw_name;
        }
    }
    if q.is_null() || (*q).pw_uid != getuid() {
        q = getpwuid(getuid());
        if q.is_null() {
            z = 0 as *mut libc::c_char;
        } else {
            z = (*q).pw_name;
        }
    }
    if !z.is_null() {
        zSlogin = zbufcpy(z);
    }
    if iflags & 0o4 as libc::c_int != 0 as libc::c_int
        && geteuid() == 0 as libc::c_int as libc::c_uint
    {
        q = getpwnam(b"uucp\0" as *const u8 as *const libc::c_char);
        if !q.is_null() {
            setuid((*q).pw_uid);
        }
    }
    if iflags & 0o1 as libc::c_int != 0 as libc::c_int {
        let mut zenv: *const libc::c_char = 0 as *const libc::c_char;
        let mut senv: stat = stat {
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
        let mut sdot: stat = stat {
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
        zenv = getenv(b"PWD\0" as *const u8 as *const libc::c_char);
        if !zenv.is_null()
            && stat(zenv as *mut libc::c_char, &mut senv) == 0 as libc::c_int
            && stat(
                b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                &mut sdot,
            ) == 0 as libc::c_int && senv.st_ino == sdot.st_ino
            && senv.st_dev == sdot.st_dev
        {
            zScwd = zbufcpy(zenv);
        } else {
            let mut c: size_t = 0;
            c = 128 as libc::c_int as size_t;
            loop {
                zScwd = xmalloc(c) as *mut libc::c_char;
                if !(getcwd(zScwd, c)).is_null() {
                    break;
                }
                xfree(zScwd as pointer);
                zScwd = 0 as *mut libc::c_char;
                if *__errno_location() != 34 as libc::c_int {
                    break;
                }
                c <<= 1 as libc::c_int;
            }
            if !zScwd.is_null() {
                zScwd = xrealloc(
                    zScwd as pointer,
                    (strlen(zScwd)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
            }
        }
    }
    if iflags & 0o2 as libc::c_int == 0 as libc::c_int {
        if chdir(zSspooldir) < 0 as libc::c_int {
            if *__errno_location() == 2 as libc::c_int
                && mkdir(
                    zSspooldir as *mut libc::c_char,
                    (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
                        | 0o400 as libc::c_int >> 3 as libc::c_int
                        | 0o100 as libc::c_int >> 3 as libc::c_int
                        | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                        | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                        as __mode_t,
                ) < 0 as libc::c_int
            {
                ulog(
                    LOG_FATAL,
                    b"mkdir (%s): %s\0" as *const u8 as *const libc::c_char,
                    zSspooldir,
                    strerror(*__errno_location()),
                );
            }
            if chdir(zSspooldir) < 0 as libc::c_int {
                ulog(
                    LOG_FATAL,
                    b"chdir (%s): %s\0" as *const u8 as *const libc::c_char,
                    zSspooldir,
                    strerror(*__errno_location()),
                );
            }
        }
    }
}
pub unsafe extern "C" fn usysdep_exit(mut fsuccess: boolean) {
    exit(if fsuccess != 0 { 0 as libc::c_int } else { 1 as libc::c_int });
}
pub unsafe extern "C" fn fsysdep_other_config(mut z: *const libc::c_char) -> boolean {
    setuid(getuid());
    setgid(getgid());
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn zsysdep_localname() -> *const libc::c_char {
    return zSlocalname;
}
pub unsafe extern "C" fn zsysdep_login_name() -> *const libc::c_char {
    if zSlogin.is_null() {
        ulog(LOG_FATAL, b"Can't get login name\0" as *const u8 as *const libc::c_char);
    }
    return zSlogin;
}
