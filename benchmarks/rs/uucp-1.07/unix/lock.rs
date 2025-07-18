use ::libc;
extern "C" {
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn getpid() -> __pid_t;
    fn link(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    static mut zSlockdir: *const libc::c_char;
    fn fsysdep_make_dirs(zfile: *const libc::c_char, fpublic: boolean) -> boolean;
    fn zsysdep_in_dir(
        zdir: *const libc::c_char,
        zfile: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn creat(__file: *const libc::c_char, __mode: mode_t) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn localtime(__timer: *const time_t) -> *mut tm;
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
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type mode_t = __mode_t;
pub type time_t = __time_t;
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
pub static mut lock_rcsid: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"$Id: lock.c,v 1.23 2002/03/05 19:10:42 ian Rel $\0")
};
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
pub unsafe extern "C" fn fsdo_lock(
    mut zlock: *const libc::c_char,
    mut fspooldir: boolean,
    mut pferr: *mut boolean,
) -> boolean {
    let mut zfree: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zpath: *const libc::c_char = 0 as *const libc::c_char;
    let mut zslash: *const libc::c_char = 0 as *const libc::c_char;
    let mut cslash: size_t = 0;
    let mut ime: pid_t = 0;
    let mut ztempfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut abtempfile: [libc::c_char; 24] = [0; 24];
    let mut o: libc::c_int = 0;
    let mut ab: [libc::c_char; 12] = [0; 12];
    let mut cwrote: libc::c_int = 0;
    let mut zerr: *const libc::c_char = 0 as *const libc::c_char;
    let mut fret: boolean = 0;
    if !pferr.is_null() {
        *pferr = 1 as libc::c_int;
    }
    if fspooldir != 0 {
        zfree = 0 as *mut libc::c_char;
        zpath = zlock;
    } else {
        zfree = zsysdep_in_dir(zSlockdir, zlock);
        zpath = zfree;
    }
    ime = getpid();
    zslash = strrchr(zpath, '/' as i32);
    if zslash.is_null() {
        cslash = 0 as libc::c_int as size_t;
    } else {
        cslash = (zslash.offset_from(zpath) as libc::c_long
            + 1 as libc::c_int as libc::c_long) as size_t;
    }
    sprintf(
        abtempfile.as_mut_ptr(),
        b"TMP%010lx\0" as *const u8 as *const libc::c_char,
        ime as libc::c_ulong,
    );
    ztempfile = zbufalc(
        cslash.wrapping_add(::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong),
    );
    memcpy(ztempfile as *mut libc::c_void, zpath as *const libc::c_void, cslash);
    memcpy(
        ztempfile.offset(cslash as isize) as *mut libc::c_void,
        abtempfile.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong,
    );
    o = creat(
        ztempfile,
        (0o400 as libc::c_int | 0o200 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as mode_t,
    );
    if o < 0 as libc::c_int {
        if *__errno_location() == 2 as libc::c_int {
            if fsysdep_make_dirs(ztempfile, 0 as libc::c_int) == 0 {
                ubuffree(zfree);
                ubuffree(ztempfile);
                return 0 as libc::c_int;
            }
            o = creat(
                ztempfile,
                (0o400 as libc::c_int | 0o200 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                    as mode_t,
            );
        }
        if o < 0 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"creat (%s): %s\0" as *const u8 as *const libc::c_char,
                ztempfile,
                strerror(*__errno_location()),
            );
            ubuffree(zfree);
            ubuffree(ztempfile);
            return 0 as libc::c_int;
        }
    }
    sprintf(
        ab.as_mut_ptr(),
        b"%10ld\n\0" as *const u8 as *const libc::c_char,
        ime as libc::c_long,
    );
    cwrote = write(o, ab.as_mut_ptr() as *const libc::c_void, strlen(ab.as_mut_ptr()))
        as libc::c_int;
    zerr = 0 as *const libc::c_char;
    if cwrote < 0 as libc::c_int {
        zerr = b"write\0" as *const u8 as *const libc::c_char;
    }
    if close(o) < 0 as libc::c_int {
        zerr = b"close\0" as *const u8 as *const libc::c_char;
    }
    if !zerr.is_null() {
        ulog(
            LOG_ERROR,
            b"%s (%s): %s\0" as *const u8 as *const libc::c_char,
            zerr,
            ztempfile,
            strerror(*__errno_location()),
        );
        remove(ztempfile);
        ubuffree(zfree);
        ubuffree(ztempfile);
        return 0 as libc::c_int;
    }
    fret = 1 as libc::c_int;
    if !pferr.is_null() {
        *pferr = 0 as libc::c_int;
    }
    o = -(1 as libc::c_int);
    zerr = 0 as *const libc::c_char;
    while link(ztempfile, zpath) != 0 as libc::c_int {
        let mut cgot: libc::c_int = 0;
        let mut ipid: pid_t = 0;
        let mut freadonly: boolean = 0;
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
        let mut abtime: [libc::c_char; 20] = [0; 20];
        fret = 0 as libc::c_int;
        if *__errno_location() != 17 as libc::c_int {
            ulog(
                LOG_ERROR,
                b"link (%s, %s): %s\0" as *const u8 as *const libc::c_char,
                ztempfile,
                zpath,
                strerror(*__errno_location()),
            );
            if !pferr.is_null() {
                *pferr = 1 as libc::c_int;
            }
            break;
        } else {
            freadonly = 0 as libc::c_int;
            o = open(
                zpath as *mut libc::c_char,
                0o2 as libc::c_int | 0o400 as libc::c_int,
                0 as libc::c_int,
            );
            if o < 0 as libc::c_int {
                if *__errno_location() == 13 as libc::c_int {
                    freadonly = 1 as libc::c_int;
                    o = open(
                        zpath as *mut libc::c_char,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    );
                }
                if o < 0 as libc::c_int {
                    if *__errno_location() == 2 as libc::c_int {
                        fret = 1 as libc::c_int;
                        continue;
                    } else {
                        zerr = b"open\0" as *const u8 as *const libc::c_char;
                        break;
                    }
                }
            }
            cgot = read(
                o,
                ab.as_mut_ptr() as *mut libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_int;
            if cgot < 0 as libc::c_int {
                zerr = b"read\0" as *const u8 as *const libc::c_char;
                break;
            } else {
                if cgot == 4 as libc::c_int {
                    ulog(
                        LOG_ERROR,
                        b"Lock file %s may be V2 format; check LOCKFILES in policy.h\0"
                            as *const u8 as *const libc::c_char,
                        zpath,
                    );
                }
                ab[cgot as usize] = '\0' as i32 as libc::c_char;
                ipid = strtol(
                    ab.as_mut_ptr(),
                    0 as *mut libc::c_void as *mut *mut libc::c_char,
                    10 as libc::c_int,
                ) as pid_t;
                if ipid == ime {
                    fret = 1 as libc::c_int;
                    break;
                } else {
                    if cgot > 0 as libc::c_int {
                        if kill(ipid, 0 as libc::c_int) == 0 as libc::c_int
                            || *__errno_location() == 1 as libc::c_int
                        {
                            break;
                        }
                    }
                    if fstat(o, &mut st) < 0 as libc::c_int {
                        strcpy(
                            abtime.as_mut_ptr(),
                            b"unknown\0" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        let mut itm: time_t = 0;
                        let mut q: *mut tm = 0 as *mut tm;
                        itm = st.st_mtim.tv_sec;
                        q = localtime(&mut itm);
                        sprintf(
                            abtime.as_mut_ptr(),
                            b"%04d-%02d-%02d %02d:%02d:%02d\0" as *const u8
                                as *const libc::c_char,
                            (*q).tm_year + 1900 as libc::c_int,
                            (*q).tm_mon + 1 as libc::c_int,
                            (*q).tm_mday,
                            (*q).tm_hour,
                            (*q).tm_min,
                            (*q).tm_sec,
                        );
                    }
                    ulog(
                        LOG_ERROR,
                        b"Stale lock %s held by process %ld created %s\0" as *const u8
                            as *const libc::c_char,
                        zpath,
                        ipid as libc::c_long,
                        abtime.as_mut_ptr(),
                    );
                    if freadonly != 0 {
                        close(o);
                        o = -(1 as libc::c_int);
                        if remove(zpath) != 0 as libc::c_int {
                            zerr = b"remove\0" as *const u8 as *const libc::c_char;
                            break;
                        } else {
                            fret = 1 as libc::c_int;
                        }
                    } else if lseek(o, 0 as libc::c_int as off_t, 0 as libc::c_int)
                        != 0 as libc::c_int as libc::c_long
                    {
                        zerr = b"lseek\0" as *const u8 as *const libc::c_char;
                        break;
                    } else {
                        sprintf(
                            ab.as_mut_ptr(),
                            b"%10ld\n\0" as *const u8 as *const libc::c_char,
                            ime as libc::c_long,
                        );
                        cwrote = write(
                            o,
                            ab.as_mut_ptr() as *const libc::c_void,
                            strlen(ab.as_mut_ptr()),
                        ) as libc::c_int;
                        if cwrote < 0 as libc::c_int {
                            zerr = b"write\0" as *const u8 as *const libc::c_char;
                            break;
                        } else {
                            sleep(5 as libc::c_int as libc::c_uint);
                            if lseek(o, 0 as libc::c_int as off_t, 0 as libc::c_int)
                                != 0 as libc::c_int as libc::c_long
                            {
                                zerr = b"lseek\0" as *const u8 as *const libc::c_char;
                                break;
                            } else {
                                cgot = read(
                                    o,
                                    ab.as_mut_ptr() as *mut libc::c_void,
                                    (::std::mem::size_of::<[libc::c_char; 12]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                ) as libc::c_int;
                                if cgot < 0 as libc::c_int {
                                    zerr = b"read\0" as *const u8 as *const libc::c_char;
                                    break;
                                } else {
                                    ab[cgot as usize] = '\0' as i32 as libc::c_char;
                                    ipid = strtol(
                                        ab.as_mut_ptr(),
                                        0 as *mut libc::c_void as *mut *mut libc::c_char,
                                        10 as libc::c_int,
                                    ) as pid_t;
                                    if ipid == ime {
                                        let mut sfile: stat = stat {
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
                                        let mut sdescriptor: stat = stat {
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
                                        if stat(zpath as *mut libc::c_char, &mut sfile)
                                            < 0 as libc::c_int
                                        {
                                            if *__errno_location() != 2 as libc::c_int {
                                                zerr = b"stat\0" as *const u8 as *const libc::c_char;
                                                break;
                                            }
                                        } else if fstat(o, &mut sdescriptor) < 0 as libc::c_int {
                                            zerr = b"fstat\0" as *const u8 as *const libc::c_char;
                                            break;
                                        } else if sfile.st_ino == sdescriptor.st_ino
                                            && sfile.st_dev == sdescriptor.st_dev
                                        {
                                            if close(o) < 0 as libc::c_int {
                                                zerr = b"close\0" as *const u8 as *const libc::c_char;
                                                break;
                                            } else {
                                                o = -(1 as libc::c_int);
                                                fret = 1 as libc::c_int;
                                                break;
                                            }
                                        }
                                    }
                                    close(o);
                                    o = -(1 as libc::c_int);
                                    fret = 1 as libc::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !zerr.is_null() {
        ulog(
            LOG_ERROR,
            b"%s (%s): %s\0" as *const u8 as *const libc::c_char,
            zerr,
            zpath,
            strerror(*__errno_location()),
        );
        if !pferr.is_null() {
            *pferr = 1 as libc::c_int;
        }
    }
    if o >= 0 as libc::c_int {
        close(o);
    }
    ubuffree(zfree);
    if remove(ztempfile) != 0 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"remove (%s): %s\0" as *const u8 as *const libc::c_char,
            ztempfile,
            strerror(*__errno_location()),
        );
    }
    ubuffree(ztempfile);
    return fret;
}
pub unsafe extern "C" fn fsdo_unlock(
    mut zlock: *const libc::c_char,
    mut fspooldir: boolean,
) -> boolean {
    let mut zfree: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zpath: *const libc::c_char = 0 as *const libc::c_char;
    if fspooldir != 0 {
        zfree = 0 as *mut libc::c_char;
        zpath = zlock;
    } else {
        zfree = zsysdep_in_dir(zSlockdir, zlock);
        zpath = zfree;
    }
    if remove(zpath) == 0 as libc::c_int || *__errno_location() == 2 as libc::c_int {
        ubuffree(zfree);
        return 1 as libc::c_int;
    } else {
        ulog(
            LOG_ERROR,
            b"remove (%s): %s\0" as *const u8 as *const libc::c_char,
            zpath,
            strerror(*__errno_location()),
        );
        ubuffree(zfree);
        return 0 as libc::c_int;
    };
}
