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
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn SaveStr(_: *const libc::c_char) -> *mut libc::c_char;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct logfile {
    pub next: *mut logfile,
    pub fp: *mut FILE,
    pub name: *mut libc::c_char,
    pub opencount: libc::c_int,
    pub writecount: libc::c_int,
    pub flushcount: libc::c_int,
    pub st: *mut stat,
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
static mut logroot: *mut logfile = 0 as *const logfile as *mut logfile;
unsafe extern "C" fn changed_logfile(mut l: *mut logfile) {
    let mut o: stat = stat {
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
    let mut s: *mut stat = (*l).st;
    if fstat(fileno((*l).fp), &mut o) < 0 as libc::c_int {
        return;
    }
    if o.st_size > (*s).st_size {
        (*s).st_size = o.st_size;
        (*s).st_mtim.tv_sec = o.st_mtim.tv_sec;
    }
}
pub unsafe extern "C" fn lf_move_fd(
    mut fd: libc::c_int,
    mut need_fd: libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = -(1 as libc::c_int);
    if fd == need_fd {
        return fd;
    }
    if fd >= 0 as libc::c_int && fd < need_fd {
        r = lf_move_fd(dup(fd), need_fd);
    }
    close(fd);
    return r;
}
unsafe extern "C" fn logfile_reopen(
    mut name: *mut libc::c_char,
    mut wantfd: libc::c_int,
    mut l: *mut logfile,
) -> libc::c_int {
    let mut got_fd: libc::c_int = 0;
    close(wantfd);
    got_fd = open(
        name,
        0o1 as libc::c_int | 0o100 as libc::c_int | 0o2000 as libc::c_int,
        0o666 as libc::c_int,
    );
    if got_fd < 0 as libc::c_int || lf_move_fd(got_fd, wantfd) < 0 as libc::c_int {
        logfclose(l);
        return -(1 as libc::c_int);
    }
    changed_logfile(l);
    return 0 as libc::c_int;
}
static mut lf_reopen_fn: Option::<unsafe extern "C" fn() -> libc::c_int> = unsafe {
    ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut libc::c_char,
                libc::c_int,
                *mut logfile,
            ) -> libc::c_int,
        >,
        Option::<unsafe extern "C" fn() -> libc::c_int>,
    >(
        Some(
            logfile_reopen
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    libc::c_int,
                    *mut logfile,
                ) -> libc::c_int,
        ),
    )
};
pub unsafe extern "C" fn logreopen_register(
    mut fn_0: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_int, *mut logfile) -> libc::c_int,
    >,
) {
    lf_reopen_fn = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut libc::c_char,
                libc::c_int,
                *mut logfile,
            ) -> libc::c_int,
        >,
        Option::<unsafe extern "C" fn() -> libc::c_int>,
    >(
        if fn_0.is_some() {
            fn_0
        } else {
            Some(
                logfile_reopen
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                        *mut logfile,
                    ) -> libc::c_int,
            )
        },
    );
}
unsafe extern "C" fn stolen_logfile(mut l: *mut logfile) -> libc::c_int {
    let mut o: stat = stat {
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
    let mut s: *mut stat = (*l).st;
    o = *s;
    if fstat(fileno((*l).fp), s) < 0 as libc::c_int {
        (*s).st_dev = 0 as libc::c_int as __dev_t;
        (*s).st_ino = (*s).st_dev;
    }
    if o.st_dev == 0 && o.st_ino == 0 {
        return 0 as libc::c_int;
    }
    if (*s).st_dev == 0 && (*s).st_ino == 0 || (*s).st_nlink == 0
        || (*s).st_size < o.st_size || (*s).st_mtim.tv_sec != o.st_mtim.tv_sec
        || (*s).st_ctim.tv_sec != o.st_ctim.tv_sec
            && !((*s).st_mtim.tv_sec == (*s).st_ctim.tv_sec
                && o.st_ctim.tv_sec < (*s).st_ctim.tv_sec)
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn lookup_logfile(mut name: *mut libc::c_char) -> *mut logfile {
    let mut l: *mut logfile = 0 as *mut logfile;
    l = logroot;
    while !l.is_null() {
        if strcmp(name, (*l).name) == 0 {
            return l;
        }
        l = (*l).next;
    }
    return 0 as *mut logfile;
}
pub unsafe extern "C" fn logfopen(
    mut name: *mut libc::c_char,
    mut fp: *mut FILE,
) -> *mut logfile {
    let mut l: *mut logfile = 0 as *mut logfile;
    if fp.is_null() {
        l = lookup_logfile(name);
        if l.is_null() {
            return 0 as *mut logfile;
        }
        (*l).opencount += 1;
        (*l).opencount;
        return l;
    }
    l = malloc(::std::mem::size_of::<logfile>() as libc::c_ulong) as *mut logfile;
    if l.is_null() {
        return 0 as *mut logfile;
    }
    (*l).st = malloc(::std::mem::size_of::<stat>() as libc::c_ulong) as *mut stat;
    if ((*l).st).is_null() {
        free(l as *mut libc::c_char as *mut libc::c_void);
        return 0 as *mut logfile;
    }
    (*l).name = SaveStr(name);
    if ((*l).name).is_null() {
        free((*l).st as *mut libc::c_char as *mut libc::c_void);
        free(l as *mut libc::c_char as *mut libc::c_void);
        return 0 as *mut logfile;
    }
    (*l).fp = fp;
    (*l).opencount = 1 as libc::c_int;
    (*l).writecount = 0 as libc::c_int;
    (*l).flushcount = 0 as libc::c_int;
    changed_logfile(l);
    (*l).next = logroot;
    logroot = l;
    return l;
}
pub unsafe extern "C" fn islogfile(mut name: *mut libc::c_char) -> libc::c_int {
    if name.is_null() {
        return if !logroot.is_null() { 1 as libc::c_int } else { 0 as libc::c_int };
    }
    return if !(lookup_logfile(name)).is_null() {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
pub unsafe extern "C" fn logfclose(mut l: *mut logfile) -> libc::c_int {
    let mut lp: *mut *mut logfile = 0 as *mut *mut logfile;
    lp = &mut logroot;
    while !(*lp).is_null() {
        if *lp == l {
            break;
        }
        lp = &mut (**lp).next;
    }
    if (*lp).is_null() {
        return -(1 as libc::c_int);
    }
    (*l).opencount -= 1;
    if (*l).opencount > 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*l).opencount < 0 as libc::c_int {
        abort();
    }
    *lp = (*l).next;
    fclose((*l).fp);
    free((*l).name as *mut libc::c_void);
    free(l as *mut libc::c_char as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn logfwrite(
    mut l: *mut logfile,
    mut buf: *mut libc::c_char,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    if stolen_logfile(l) != 0
        && ::std::mem::transmute::<
            _,
            fn(_, _, _) -> libc::c_int,
        >(lf_reopen_fn.unwrap())((*l).name, fileno((*l).fp), l) != 0
    {
        return -(1 as libc::c_int);
    }
    r = fwrite(
        buf as *const libc::c_void,
        n as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*l).fp,
    ) as libc::c_int;
    (*l).writecount += (*l).flushcount + 1 as libc::c_int;
    (*l).flushcount = 0 as libc::c_int;
    changed_logfile(l);
    return r;
}
pub unsafe extern "C" fn logfflush(mut l: *mut logfile) -> libc::c_int {
    let mut r: libc::c_int = 0 as libc::c_int;
    if l.is_null() {
        l = logroot;
        while !l.is_null() {
            if stolen_logfile(l) != 0
                && ::std::mem::transmute::<
                    _,
                    fn(_, _, _) -> libc::c_int,
                >(lf_reopen_fn.unwrap())((*l).name, fileno((*l).fp), l) != 0
            {
                return -(1 as libc::c_int);
            }
            r |= fflush((*l).fp);
            (*l).flushcount += 1;
            (*l).flushcount;
            changed_logfile(l);
            l = (*l).next;
        }
    } else {
        if stolen_logfile(l) != 0
            && ::std::mem::transmute::<
                _,
                fn(_, _, _) -> libc::c_int,
            >(lf_reopen_fn.unwrap())((*l).name, fileno((*l).fp), l) != 0
        {
            return -(1 as libc::c_int);
        }
        r = fflush((*l).fp);
        (*l).flushcount += 1;
        (*l).flushcount;
        changed_logfile(l);
    }
    return r;
}
