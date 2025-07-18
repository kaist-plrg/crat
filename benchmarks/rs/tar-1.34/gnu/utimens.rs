use ::libc;
extern "C" {
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
    fn futimens(__fd: libc::c_int, __times: *const timespec) -> libc::c_int;
    fn utimensat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __times: *const timespec,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn utimes(__file: *const libc::c_char, __tvp: *const timeval) -> libc::c_int;
    fn futimesat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __tvp: *const timeval,
    ) -> libc::c_int;
    fn gettime(_: *mut timespec);
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type time_t = __time_t;
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
pub const TIMESPEC_HZ: C2RustUnnamed = 1000000000;
pub type C2RustUnnamed = libc::c_uint;
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn lutimensat(
    mut dir: libc::c_int,
    mut file: *const libc::c_char,
    mut times: *const timespec,
) -> libc::c_int {
    return utimensat(dir, file, times, 0x100 as libc::c_int);
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn get_stat_atime_ns(mut st: *const stat) -> libc::c_long {
    return (*st).st_atim.tv_nsec;
}
#[inline]
unsafe extern "C" fn get_stat_mtime_ns(mut st: *const stat) -> libc::c_long {
    return (*st).st_mtim.tv_nsec;
}
#[inline]
unsafe extern "C" fn get_stat_atime(mut st: *const stat) -> timespec {
    return (*st).st_atim;
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
static mut utimensat_works_really: libc::c_int = 0;
static mut lutimensat_works_really: libc::c_int = 0;
unsafe extern "C" fn validate_timespec(mut timespec: *mut timespec) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut utime_omit_count: libc::c_int = 0 as libc::c_int;
    if (*timespec.offset(0 as libc::c_int as isize)).tv_nsec
        != ((1 as libc::c_long) << 30 as libc::c_int) - 1 as libc::c_long
        && (*timespec.offset(0 as libc::c_int as isize)).tv_nsec
            != ((1 as libc::c_long) << 30 as libc::c_int) - 2 as libc::c_long
        && !(0 as libc::c_int as libc::c_long
            <= (*timespec.offset(0 as libc::c_int as isize)).tv_nsec
            && (*timespec.offset(0 as libc::c_int as isize)).tv_nsec
                < TIMESPEC_HZ as libc::c_int as libc::c_long)
        || (*timespec.offset(1 as libc::c_int as isize)).tv_nsec
            != ((1 as libc::c_long) << 30 as libc::c_int) - 1 as libc::c_long
            && (*timespec.offset(1 as libc::c_int as isize)).tv_nsec
                != ((1 as libc::c_long) << 30 as libc::c_int) - 2 as libc::c_long
            && !(0 as libc::c_int as libc::c_long
                <= (*timespec.offset(1 as libc::c_int as isize)).tv_nsec
                && (*timespec.offset(1 as libc::c_int as isize)).tv_nsec
                    < TIMESPEC_HZ as libc::c_int as libc::c_long)
    {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if (*timespec.offset(0 as libc::c_int as isize)).tv_nsec
        == ((1 as libc::c_long) << 30 as libc::c_int) - 1 as libc::c_long
        || (*timespec.offset(0 as libc::c_int as isize)).tv_nsec
            == ((1 as libc::c_long) << 30 as libc::c_int) - 2 as libc::c_long
    {
        (*timespec.offset(0 as libc::c_int as isize))
            .tv_sec = 0 as libc::c_int as __time_t;
        result = 1 as libc::c_int;
        if (*timespec.offset(0 as libc::c_int as isize)).tv_nsec
            == ((1 as libc::c_long) << 30 as libc::c_int) - 2 as libc::c_long
        {
            utime_omit_count += 1;
            utime_omit_count;
        }
    }
    if (*timespec.offset(1 as libc::c_int as isize)).tv_nsec
        == ((1 as libc::c_long) << 30 as libc::c_int) - 1 as libc::c_long
        || (*timespec.offset(1 as libc::c_int as isize)).tv_nsec
            == ((1 as libc::c_long) << 30 as libc::c_int) - 2 as libc::c_long
    {
        (*timespec.offset(1 as libc::c_int as isize))
            .tv_sec = 0 as libc::c_int as __time_t;
        result = 1 as libc::c_int;
        if (*timespec.offset(1 as libc::c_int as isize)).tv_nsec
            == ((1 as libc::c_long) << 30 as libc::c_int) - 2 as libc::c_long
        {
            utime_omit_count += 1;
            utime_omit_count;
        }
    }
    return result + (utime_omit_count == 1 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn update_timespec(
    mut statbuf: *const stat,
    mut ts: *mut *mut timespec,
) -> bool {
    let mut timespec: *mut timespec = *ts;
    if (*timespec.offset(0 as libc::c_int as isize)).tv_nsec
        == ((1 as libc::c_long) << 30 as libc::c_int) - 2 as libc::c_long
        && (*timespec.offset(1 as libc::c_int as isize)).tv_nsec
            == ((1 as libc::c_long) << 30 as libc::c_int) - 2 as libc::c_long
    {
        return 1 as libc::c_int != 0;
    }
    if (*timespec.offset(0 as libc::c_int as isize)).tv_nsec
        == ((1 as libc::c_long) << 30 as libc::c_int) - 1 as libc::c_long
        && (*timespec.offset(1 as libc::c_int as isize)).tv_nsec
            == ((1 as libc::c_long) << 30 as libc::c_int) - 1 as libc::c_long
    {
        *ts = 0 as *mut timespec;
        return 0 as libc::c_int != 0;
    }
    if (*timespec.offset(0 as libc::c_int as isize)).tv_nsec
        == ((1 as libc::c_long) << 30 as libc::c_int) - 2 as libc::c_long
    {
        *timespec.offset(0 as libc::c_int as isize) = get_stat_atime(statbuf);
    } else if (*timespec.offset(0 as libc::c_int as isize)).tv_nsec
        == ((1 as libc::c_long) << 30 as libc::c_int) - 1 as libc::c_long
    {
        gettime(&mut *timespec.offset(0 as libc::c_int as isize));
    }
    if (*timespec.offset(1 as libc::c_int as isize)).tv_nsec
        == ((1 as libc::c_long) << 30 as libc::c_int) - 2 as libc::c_long
    {
        *timespec.offset(1 as libc::c_int as isize) = get_stat_mtime(statbuf);
    } else if (*timespec.offset(1 as libc::c_int as isize)).tv_nsec
        == ((1 as libc::c_long) << 30 as libc::c_int) - 1 as libc::c_long
    {
        gettime(&mut *timespec.offset(1 as libc::c_int as isize));
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn fdutimens(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut timespec: *const timespec,
) -> libc::c_int {
    let mut adjusted_timespec: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    let mut ts: *mut timespec = if !timespec.is_null() {
        adjusted_timespec.as_mut_ptr()
    } else {
        0 as *mut timespec
    };
    let mut adjustment_needed: libc::c_int = 0 as libc::c_int;
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
    if !ts.is_null() {
        adjusted_timespec[0 as libc::c_int
            as usize] = *timespec.offset(0 as libc::c_int as isize);
        adjusted_timespec[1 as libc::c_int
            as usize] = *timespec.offset(1 as libc::c_int as isize);
        adjustment_needed = validate_timespec(ts);
    }
    if adjustment_needed < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if fd < 0 as libc::c_int && file.is_null() {
        *__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if 0 as libc::c_int <= utimensat_works_really {
        let mut result: libc::c_int = 0;
        if adjustment_needed == 2 as libc::c_int {
            if if fd < 0 as libc::c_int {
                stat(file, &mut st)
            } else {
                fstat(fd, &mut st)
            } != 0
            {
                return -(1 as libc::c_int);
            }
            if (*ts.offset(0 as libc::c_int as isize)).tv_nsec
                == ((1 as libc::c_long) << 30 as libc::c_int) - 2 as libc::c_long
            {
                *ts.offset(0 as libc::c_int as isize) = get_stat_atime(&mut st);
            } else if (*ts.offset(1 as libc::c_int as isize)).tv_nsec
                == ((1 as libc::c_long) << 30 as libc::c_int) - 2 as libc::c_long
            {
                *ts.offset(1 as libc::c_int as isize) = get_stat_mtime(&mut st);
            }
            adjustment_needed += 1;
            adjustment_needed;
        }
        if fd < 0 as libc::c_int {
            result = utimensat(
                -(100 as libc::c_int),
                file,
                ts as *const timespec,
                0 as libc::c_int,
            );
            if (0 as libc::c_int) < result {
                *__errno_location() = 38 as libc::c_int;
            }
            if result == 0 as libc::c_int || *__errno_location() != 38 as libc::c_int {
                utimensat_works_really = 1 as libc::c_int;
                return result;
            }
        }
        if 0 as libc::c_int <= fd {
            result = futimens(fd, ts as *const timespec);
            if (0 as libc::c_int) < result {
                *__errno_location() = 38 as libc::c_int;
            }
            if result == 0 as libc::c_int || *__errno_location() != 38 as libc::c_int {
                utimensat_works_really = 1 as libc::c_int;
                return result;
            }
        }
    }
    utimensat_works_really = -(1 as libc::c_int);
    lutimensat_works_really = -(1 as libc::c_int);
    if adjustment_needed != 0 || 0 as libc::c_int != 0 && fd < 0 as libc::c_int {
        if adjustment_needed != 3 as libc::c_int
            && (if fd < 0 as libc::c_int {
                stat(file, &mut st)
            } else {
                fstat(fd, &mut st)
            }) != 0
        {
            return -(1 as libc::c_int);
        }
        if !ts.is_null() && update_timespec(&mut st, &mut ts) as libc::c_int != 0 {
            return 0 as libc::c_int;
        }
    }
    let mut timeval: [timeval; 2] = [timeval { tv_sec: 0, tv_usec: 0 }; 2];
    let mut t: *mut timeval = 0 as *mut timeval;
    if !ts.is_null() {
        timeval[0 as libc::c_int as usize]
            .tv_sec = (*ts.offset(0 as libc::c_int as isize)).tv_sec;
        timeval[0 as libc::c_int as usize]
            .tv_usec = (*ts.offset(0 as libc::c_int as isize)).tv_nsec
            / 1000 as libc::c_int as libc::c_long;
        timeval[1 as libc::c_int as usize]
            .tv_sec = (*ts.offset(1 as libc::c_int as isize)).tv_sec;
        timeval[1 as libc::c_int as usize]
            .tv_usec = (*ts.offset(1 as libc::c_int as isize)).tv_nsec
            / 1000 as libc::c_int as libc::c_long;
        t = timeval.as_mut_ptr();
    } else {
        t = 0 as *mut timeval;
    }
    if fd < 0 as libc::c_int {
        return futimesat(-(100 as libc::c_int), file, t as *const timeval)
    } else if futimesat(fd, 0 as *const libc::c_char, t as *const timeval)
        == 0 as libc::c_int
    {
        if !t.is_null() {
            let mut abig: bool = 500000 as libc::c_int as libc::c_long
                <= (*t.offset(0 as libc::c_int as isize)).tv_usec;
            let mut mbig: bool = 500000 as libc::c_int as libc::c_long
                <= (*t.offset(1 as libc::c_int as isize)).tv_usec;
            if abig as libc::c_int | mbig as libc::c_int != 0
                && fstat(fd, &mut st) == 0 as libc::c_int
            {
                let mut adiff: time_t = st.st_atim.tv_sec
                    - (*t.offset(0 as libc::c_int as isize)).tv_sec;
                let mut mdiff: time_t = st.st_mtim.tv_sec
                    - (*t.offset(1 as libc::c_int as isize)).tv_sec;
                let mut tt: *mut timeval = 0 as *mut timeval;
                let mut truncated_timeval: [timeval; 2] = [timeval {
                    tv_sec: 0,
                    tv_usec: 0,
                }; 2];
                truncated_timeval[0 as libc::c_int
                    as usize] = *t.offset(0 as libc::c_int as isize);
                truncated_timeval[1 as libc::c_int
                    as usize] = *t.offset(1 as libc::c_int as isize);
                if abig as libc::c_int != 0 && adiff == 1 as libc::c_int as libc::c_long
                    && get_stat_atime_ns(&mut st) == 0 as libc::c_int as libc::c_long
                {
                    tt = truncated_timeval.as_mut_ptr();
                    (*tt.offset(0 as libc::c_int as isize))
                        .tv_usec = 0 as libc::c_int as __suseconds_t;
                }
                if mbig as libc::c_int != 0 && mdiff == 1 as libc::c_int as libc::c_long
                    && get_stat_mtime_ns(&mut st) == 0 as libc::c_int as libc::c_long
                {
                    tt = truncated_timeval.as_mut_ptr();
                    (*tt.offset(1 as libc::c_int as isize))
                        .tv_usec = 0 as libc::c_int as __suseconds_t;
                }
                if !tt.is_null() {
                    futimesat(fd, 0 as *const libc::c_char, tt as *const timeval);
                }
            }
        }
        return 0 as libc::c_int;
    }
    if file.is_null() {
        return -(1 as libc::c_int);
    }
    return utimes(file, t as *const timeval);
}
pub unsafe extern "C" fn utimens(
    mut file: *const libc::c_char,
    mut timespec: *const timespec,
) -> libc::c_int {
    return fdutimens(-(1 as libc::c_int), file, timespec);
}
pub unsafe extern "C" fn lutimens(
    mut file: *const libc::c_char,
    mut timespec: *const timespec,
) -> libc::c_int {
    let mut adjusted_timespec: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    let mut ts: *mut timespec = if !timespec.is_null() {
        adjusted_timespec.as_mut_ptr()
    } else {
        0 as *mut timespec
    };
    let mut adjustment_needed: libc::c_int = 0 as libc::c_int;
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
    if !ts.is_null() {
        adjusted_timespec[0 as libc::c_int
            as usize] = *timespec.offset(0 as libc::c_int as isize);
        adjusted_timespec[1 as libc::c_int
            as usize] = *timespec.offset(1 as libc::c_int as isize);
        adjustment_needed = validate_timespec(ts);
    }
    if adjustment_needed < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if 0 as libc::c_int <= lutimensat_works_really {
        let mut result: libc::c_int = 0;
        if adjustment_needed == 2 as libc::c_int {
            if lstat(file, &mut st) != 0 {
                return -(1 as libc::c_int);
            }
            if (*ts.offset(0 as libc::c_int as isize)).tv_nsec
                == ((1 as libc::c_long) << 30 as libc::c_int) - 2 as libc::c_long
            {
                *ts.offset(0 as libc::c_int as isize) = get_stat_atime(&mut st);
            } else if (*ts.offset(1 as libc::c_int as isize)).tv_nsec
                == ((1 as libc::c_long) << 30 as libc::c_int) - 2 as libc::c_long
            {
                *ts.offset(1 as libc::c_int as isize) = get_stat_mtime(&mut st);
            }
            adjustment_needed += 1;
            adjustment_needed;
        }
        result = utimensat(
            -(100 as libc::c_int),
            file,
            ts as *const timespec,
            0x100 as libc::c_int,
        );
        if (0 as libc::c_int) < result {
            *__errno_location() = 38 as libc::c_int;
        }
        if result == 0 as libc::c_int || *__errno_location() != 38 as libc::c_int {
            utimensat_works_really = 1 as libc::c_int;
            lutimensat_works_really = 1 as libc::c_int;
            return result;
        }
    }
    lutimensat_works_really = -(1 as libc::c_int);
    if adjustment_needed != 0 || 0 as libc::c_int != 0 {
        if adjustment_needed != 3 as libc::c_int && lstat(file, &mut st) != 0 {
            return -(1 as libc::c_int);
        }
        if !ts.is_null() && update_timespec(&mut st, &mut ts) as libc::c_int != 0 {
            return 0 as libc::c_int;
        }
    }
    if !(adjustment_needed != 0 || 0 as libc::c_int != 0) && lstat(file, &mut st) != 0 {
        return -(1 as libc::c_int);
    }
    if !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o120000 as libc::c_int as libc::c_uint)
    {
        return fdutimens(-(1 as libc::c_int), file, ts as *const timespec);
    }
    *__errno_location() = 38 as libc::c_int;
    return -(1 as libc::c_int);
}
