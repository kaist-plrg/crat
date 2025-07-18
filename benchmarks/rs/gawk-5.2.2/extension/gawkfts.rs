use ::libc;
extern "C" {
    pub type __dirstream;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
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
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn dirfd(__dirp: *mut DIR) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fchdir(__fd: libc::c_int) -> libc::c_int;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
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
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type C2RustUnnamed = libc::c_uint;
pub const DT_WHT: C2RustUnnamed = 14;
pub const DT_SOCK: C2RustUnnamed = 12;
pub const DT_LNK: C2RustUnnamed = 10;
pub const DT_REG: C2RustUnnamed = 8;
pub const DT_BLK: C2RustUnnamed = 6;
pub const DT_DIR: C2RustUnnamed = 4;
pub const DT_CHR: C2RustUnnamed = 2;
pub const DT_FIFO: C2RustUnnamed = 1;
pub const DT_UNKNOWN: C2RustUnnamed = 0;
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FTS {
    pub fts_cur: *mut _ftsent,
    pub fts_child: *mut _ftsent,
    pub fts_array: *mut *mut _ftsent,
    pub fts_dev: dev_t,
    pub fts_path: *mut libc::c_char,
    pub fts_rfd: libc::c_int,
    pub fts_pathlen: libc::c_uint,
    pub fts_nitems: libc::c_uint,
    pub fts_compar: Option::<
        unsafe extern "C" fn(*mut *const _ftsent, *mut *const _ftsent) -> libc::c_int,
    >,
    pub fts_options: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ftsent {
    pub fts_cycle: *mut _ftsent,
    pub fts_parent: *mut _ftsent,
    pub fts_link: *mut _ftsent,
    pub fts_number: libc::c_longlong,
    pub fts_pointer: *mut libc::c_void,
    pub fts_accpath: *mut libc::c_char,
    pub fts_path: *mut libc::c_char,
    pub fts_errno: libc::c_int,
    pub fts_symfd: libc::c_int,
    pub fts_pathlen: libc::c_uint,
    pub fts_namelen: libc::c_uint,
    pub fts_ino: ino_t,
    pub fts_dev: dev_t,
    pub fts_nlink: libc::c_uint,
    pub fts_level: libc::c_int,
    pub fts_info: libc::c_ushort,
    pub fts_flags: libc::c_ushort,
    pub fts_instr: libc::c_ushort,
    pub fts_statp: *mut stat,
    pub fts_name: [libc::c_char; 1],
}
pub type FTSENT = _ftsent;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[inline]
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
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
pub unsafe extern "C" fn gawk_fts_open(
    mut argv: *const *mut libc::c_char,
    mut options: libc::c_int,
    mut compar: Option::<
        unsafe extern "C" fn(*mut *const FTSENT, *mut *const FTSENT) -> libc::c_int,
    >,
) -> *mut FTS {
    let mut current_block: u64;
    let mut sp: *mut FTS = 0 as *mut FTS;
    let mut p: *mut FTSENT = 0 as *mut FTSENT;
    let mut root: *mut FTSENT = 0 as *mut FTSENT;
    let mut nitems: size_t = 0;
    let mut parent: *mut FTSENT = 0 as *mut FTSENT;
    let mut tmp: *mut FTSENT = 0 as *mut FTSENT;
    let mut len: size_t = 0;
    if options & !(0xff as libc::c_int) != 0 {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut FTS;
    }
    sp = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<FTS>() as libc::c_ulong as libc::c_uint as libc::c_ulong,
    ) as *mut FTS;
    if sp.is_null() {
        return 0 as *mut FTS;
    }
    (*sp).fts_compar = compar;
    (*sp).fts_options = options;
    if (*sp).fts_options & 0x2 as libc::c_int != 0 {
        (*sp).fts_options |= 0x4 as libc::c_int;
    }
    if !(fts_palloc(
        sp,
        if fts_maxarglen(argv) > 4096 as libc::c_int as libc::c_ulong {
            fts_maxarglen(argv)
        } else {
            4096 as libc::c_int as libc::c_ulong
        },
    ) != 0)
    {
        parent = fts_alloc(
            sp,
            b"\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        if !parent.is_null() {
            (*parent).fts_level = -(1 as libc::c_int);
            root = 0 as *mut FTSENT;
            nitems = 0 as libc::c_int as size_t;
            loop {
                if (*argv).is_null() {
                    current_block = 5689316957504528238;
                    break;
                }
                len = strlen(*argv);
                if len == 0 as libc::c_int as libc::c_ulong {
                    *__errno_location() = 2 as libc::c_int;
                    current_block = 529128228588314491;
                    break;
                } else {
                    p = fts_alloc(sp, *argv, len);
                    if p.is_null() {
                        current_block = 529128228588314491;
                        break;
                    }
                    (*p).fts_level = 0 as libc::c_int;
                    (*p).fts_parent = parent;
                    (*p).fts_accpath = ((*p).fts_name).as_mut_ptr();
                    (*p)
                        .fts_info = fts_stat(
                        sp,
                        p,
                        (*sp).fts_options & 0x1 as libc::c_int,
                    );
                    if (*p).fts_info as libc::c_int == 5 as libc::c_int {
                        (*p).fts_info = 1 as libc::c_int as libc::c_ushort;
                    }
                    if compar.is_some() {
                        (*p).fts_link = root;
                        root = p;
                    } else {
                        (*p).fts_link = 0 as *mut _ftsent;
                        if root.is_null() {
                            root = p;
                            tmp = root;
                        } else {
                            (*tmp).fts_link = p;
                            tmp = p;
                        }
                    }
                    argv = argv.offset(1);
                    argv;
                    nitems = nitems.wrapping_add(1);
                    nitems;
                }
            }
            match current_block {
                5689316957504528238 => {
                    if compar.is_some() && nitems > 1 as libc::c_int as libc::c_ulong {
                        root = fts_sort(sp, root, nitems);
                    }
                    (*sp)
                        .fts_cur = fts_alloc(
                        sp,
                        b"\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int as size_t,
                    );
                    if ((*sp).fts_cur).is_null() {
                        if !((*sp).fts_array).is_null() {
                            free((*sp).fts_array as *mut libc::c_void);
                        }
                    } else {
                        (*(*sp).fts_cur).fts_link = root;
                        (*(*sp).fts_cur).fts_info = 9 as libc::c_int as libc::c_ushort;
                        if (*sp).fts_options & 0x4 as libc::c_int == 0 {
                            (*sp)
                                .fts_rfd = open(
                                b".\0" as *const u8 as *const libc::c_char,
                                0 as libc::c_int | 0o2000000 as libc::c_int,
                                0 as libc::c_int,
                            );
                            if (*sp).fts_rfd == -(1 as libc::c_int) {
                                (*sp).fts_options |= 0x4 as libc::c_int;
                            }
                        }
                        if nitems == 0 as libc::c_int as libc::c_ulong {
                            fts_free(parent);
                        }
                        return sp;
                    }
                }
                _ => {}
            }
            fts_lfree(root);
            fts_free(parent);
        }
        free((*sp).fts_path as *mut libc::c_void);
    }
    free(sp as *mut libc::c_void);
    return 0 as *mut FTS;
}
unsafe extern "C" fn fts_load(mut sp: *mut FTS, mut p: *mut FTSENT) {
    let mut len: size_t = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    (*p).fts_pathlen = (*p).fts_namelen;
    len = (*p).fts_pathlen as size_t;
    memmove(
        (*sp).fts_path as *mut libc::c_void,
        ((*p).fts_name).as_mut_ptr() as *const libc::c_void,
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    cp = strrchr(((*p).fts_name).as_mut_ptr(), '/' as i32);
    if !cp.is_null()
        && (cp != ((*p).fts_name).as_mut_ptr()
            || *cp.offset(1 as libc::c_int as isize) as libc::c_int != 0)
    {
        cp = cp.offset(1);
        len = strlen(cp);
        memmove(
            ((*p).fts_name).as_mut_ptr() as *mut libc::c_void,
            cp as *const libc::c_void,
            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        (*p)
            .fts_namelen = if len
            > (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint) as libc::c_ulong
        {
            (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
        } else {
            len as libc::c_uint
        };
    }
    (*p).fts_path = (*sp).fts_path;
    (*p).fts_accpath = (*p).fts_path;
    (*sp).fts_dev = (*p).fts_dev;
}
pub unsafe extern "C" fn gawk_fts_close(mut sp: *mut FTS) -> libc::c_int {
    let mut freep: *mut FTSENT = 0 as *mut FTSENT;
    let mut p: *mut FTSENT = 0 as *mut FTSENT;
    let mut saved_errno: libc::c_int = 0 as libc::c_int;
    if !((*sp).fts_cur).is_null() {
        if (*(*sp).fts_cur).fts_flags as libc::c_int & 0x2 as libc::c_int != 0 {
            close((*(*sp).fts_cur).fts_symfd);
        }
        p = (*sp).fts_cur;
        while (*p).fts_level >= 0 as libc::c_int {
            freep = p;
            p = if !((*p).fts_link).is_null() { (*p).fts_link } else { (*p).fts_parent };
            fts_free(freep);
        }
        fts_free(p);
    }
    if !((*sp).fts_child).is_null() {
        fts_lfree((*sp).fts_child);
    }
    if !((*sp).fts_array).is_null() {
        free((*sp).fts_array as *mut libc::c_void);
    }
    free((*sp).fts_path as *mut libc::c_void);
    if (*sp).fts_options & 0x4 as libc::c_int == 0 {
        if fchdir((*sp).fts_rfd) == -(1 as libc::c_int) {
            saved_errno = *__errno_location();
        }
        close((*sp).fts_rfd);
    }
    free(sp as *mut libc::c_void);
    if saved_errno != 0 {
        *__errno_location() = saved_errno;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gawk_fts_read(mut sp: *mut FTS) -> *mut FTSENT {
    let mut p: *mut FTSENT = 0 as *mut FTSENT;
    let mut tmp: *mut FTSENT = 0 as *mut FTSENT;
    let mut instr: libc::c_int = 0;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut saved_errno: libc::c_int = 0;
    if ((*sp).fts_cur).is_null() || (*sp).fts_options & 0x200 as libc::c_int != 0 {
        return 0 as *mut FTSENT;
    }
    p = (*sp).fts_cur;
    instr = (*p).fts_instr as libc::c_int;
    (*p).fts_instr = 3 as libc::c_int as libc::c_ushort;
    if instr == 1 as libc::c_int {
        (*p).fts_info = fts_stat(sp, p, 0 as libc::c_int);
        return p;
    }
    if instr == 2 as libc::c_int
        && ((*p).fts_info as libc::c_int == 12 as libc::c_int
            || (*p).fts_info as libc::c_int == 13 as libc::c_int)
    {
        (*p).fts_info = fts_stat(sp, p, 1 as libc::c_int);
        if (*p).fts_info as libc::c_int == 1 as libc::c_int
            && (*sp).fts_options & 0x4 as libc::c_int == 0
        {
            (*p)
                .fts_symfd = open(
                b".\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int | 0o2000000 as libc::c_int,
                0 as libc::c_int,
            );
            if (*p).fts_symfd == -(1 as libc::c_int) {
                (*p).fts_errno = *__errno_location();
                (*p).fts_info = 7 as libc::c_int as libc::c_ushort;
            } else {
                (*p)
                    .fts_flags = ((*p).fts_flags as libc::c_int | 0x2 as libc::c_int)
                    as libc::c_ushort;
            }
        }
        return p;
    }
    if (*p).fts_info as libc::c_int == 1 as libc::c_int {
        if instr == 4 as libc::c_int
            || (*sp).fts_options & 0x40 as libc::c_int != 0
                && (*p).fts_dev != (*sp).fts_dev
        {
            if (*p).fts_flags as libc::c_int & 0x2 as libc::c_int != 0 {
                close((*p).fts_symfd);
            }
            if !((*sp).fts_child).is_null() {
                fts_lfree((*sp).fts_child);
                (*sp).fts_child = 0 as *mut _ftsent;
            }
            (*p).fts_info = 6 as libc::c_int as libc::c_ushort;
            return p;
        }
        if !((*sp).fts_child).is_null() && (*sp).fts_options & 0x100 as libc::c_int != 0
        {
            (*sp).fts_options &= !(0x100 as libc::c_int);
            fts_lfree((*sp).fts_child);
            (*sp).fts_child = 0 as *mut _ftsent;
        }
        if !((*sp).fts_child).is_null() {
            if fts_safe_changedir(sp, p, -(1 as libc::c_int), (*p).fts_accpath) != 0 {
                (*p).fts_errno = *__errno_location();
                (*p)
                    .fts_flags = ((*p).fts_flags as libc::c_int | 0x1 as libc::c_int)
                    as libc::c_ushort;
                p = (*sp).fts_child;
                while !p.is_null() {
                    (*p).fts_accpath = (*(*p).fts_parent).fts_accpath;
                    p = (*p).fts_link;
                }
            }
        } else {
            (*sp).fts_child = fts_build(sp, 3 as libc::c_int);
            if ((*sp).fts_child).is_null() {
                if (*sp).fts_options & 0x200 as libc::c_int != 0 {
                    return 0 as *mut FTSENT;
                }
                return p;
            }
        }
        p = (*sp).fts_child;
        (*sp).fts_child = 0 as *mut _ftsent;
    } else {
        loop {
            tmp = p;
            p = (*p).fts_link;
            if !p.is_null() {
                fts_free(tmp);
                if (*p).fts_level == 0 as libc::c_int {
                    if (*sp).fts_options & 0x4 as libc::c_int == 0
                        && fchdir((*sp).fts_rfd) != 0
                    {
                        (*sp).fts_options |= 0x200 as libc::c_int;
                        return 0 as *mut FTSENT;
                    }
                    fts_load(sp, p);
                    (*sp).fts_cur = p;
                    return (*sp).fts_cur;
                }
                if (*p).fts_instr as libc::c_int == 4 as libc::c_int {
                    continue;
                }
                if (*p).fts_instr as libc::c_int == 2 as libc::c_int {
                    (*p).fts_info = fts_stat(sp, p, 1 as libc::c_int);
                    if (*p).fts_info as libc::c_int == 1 as libc::c_int
                        && (*sp).fts_options & 0x4 as libc::c_int == 0
                    {
                        (*p)
                            .fts_symfd = open(
                            b".\0" as *const u8 as *const libc::c_char,
                            0 as libc::c_int | 0o2000000 as libc::c_int,
                            0 as libc::c_int,
                        );
                        if (*p).fts_symfd == -(1 as libc::c_int) {
                            (*p).fts_errno = *__errno_location();
                            (*p).fts_info = 7 as libc::c_int as libc::c_ushort;
                        } else {
                            (*p)
                                .fts_flags = ((*p).fts_flags as libc::c_int
                                | 0x2 as libc::c_int) as libc::c_ushort;
                        }
                    }
                    (*p).fts_instr = 3 as libc::c_int as libc::c_ushort;
                }
                break;
            } else {
                p = (*tmp).fts_parent;
                fts_free(tmp);
                if (*p).fts_level == -(1 as libc::c_int) {
                    fts_free(p);
                    *__errno_location() = 0 as libc::c_int;
                    (*sp).fts_cur = 0 as *mut _ftsent;
                    return (*sp).fts_cur;
                }
                *((*sp).fts_path)
                    .offset((*p).fts_pathlen as isize) = '\0' as i32 as libc::c_char;
                if (*p).fts_level == 0 as libc::c_int {
                    if (*sp).fts_options & 0x4 as libc::c_int == 0
                        && fchdir((*sp).fts_rfd) != 0
                    {
                        (*sp).fts_options |= 0x200 as libc::c_int;
                        return 0 as *mut FTSENT;
                    }
                } else if (*p).fts_flags as libc::c_int & 0x2 as libc::c_int != 0 {
                    if (*sp).fts_options & 0x4 as libc::c_int == 0
                        && fchdir((*p).fts_symfd) != 0
                    {
                        saved_errno = *__errno_location();
                        close((*p).fts_symfd);
                        *__errno_location() = saved_errno;
                        (*sp).fts_options |= 0x200 as libc::c_int;
                        return 0 as *mut FTSENT;
                    }
                    close((*p).fts_symfd);
                } else if (*p).fts_flags as libc::c_int & 0x1 as libc::c_int == 0
                    && fts_safe_changedir(
                        sp,
                        (*p).fts_parent,
                        -(1 as libc::c_int),
                        b"..\0" as *const u8 as *const libc::c_char,
                    ) != 0
                {
                    (*sp).fts_options |= 0x200 as libc::c_int;
                    return 0 as *mut FTSENT;
                }
                (*p)
                    .fts_info = (if (*p).fts_errno != 0 {
                    7 as libc::c_int
                } else {
                    6 as libc::c_int
                }) as libc::c_ushort;
                (*sp).fts_cur = p;
                return (*sp).fts_cur;
            }
        }
    }
    t = ((*sp).fts_path)
        .offset(
            (if *((*(*p).fts_parent).fts_path)
                .offset(
                    ((*(*p).fts_parent).fts_pathlen)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                ) as libc::c_int == '/' as i32
            {
                ((*(*p).fts_parent).fts_pathlen)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
            } else {
                (*(*p).fts_parent).fts_pathlen
            }) as isize,
        );
    let fresh0 = t;
    t = t.offset(1);
    *fresh0 = '/' as i32 as libc::c_char;
    memmove(
        t as *mut libc::c_void,
        ((*p).fts_name).as_mut_ptr() as *const libc::c_void,
        ((*p).fts_namelen).wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
    );
    (*sp).fts_cur = p;
    return (*sp).fts_cur;
}
pub unsafe extern "C" fn gawk_fts_set(
    mut sp: *mut FTS,
    mut p: *mut FTSENT,
    mut instr: libc::c_int,
) -> libc::c_int {
    if instr != 0 && instr != 1 as libc::c_int && instr != 2 as libc::c_int
        && instr != 3 as libc::c_int && instr != 4 as libc::c_int
    {
        *__errno_location() = 22 as libc::c_int;
        return 1 as libc::c_int;
    }
    (*p).fts_instr = instr as libc::c_ushort;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gawk_fts_children(
    mut sp: *mut FTS,
    mut instr: libc::c_int,
) -> *mut FTSENT {
    let mut p: *mut FTSENT = 0 as *mut FTSENT;
    let mut fd: libc::c_int = 0;
    if instr != 0 && instr != 0x100 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut FTSENT;
    }
    p = (*sp).fts_cur;
    *__errno_location() = 0 as libc::c_int;
    if (*sp).fts_options & 0x200 as libc::c_int != 0 {
        return 0 as *mut FTSENT;
    }
    if (*p).fts_info as libc::c_int == 9 as libc::c_int {
        return (*p).fts_link;
    }
    if (*p).fts_info as libc::c_int != 1 as libc::c_int {
        return 0 as *mut FTSENT;
    }
    if !((*sp).fts_child).is_null() {
        fts_lfree((*sp).fts_child);
    }
    if instr == 0x100 as libc::c_int {
        (*sp).fts_options |= 0x100 as libc::c_int;
        instr = 2 as libc::c_int;
    } else {
        instr = 1 as libc::c_int;
    }
    if (*p).fts_level != 0 as libc::c_int
        || *((*p).fts_accpath).offset(0 as libc::c_int as isize) as libc::c_int
            == '/' as i32 || (*sp).fts_options & 0x4 as libc::c_int != 0
    {
        (*sp).fts_child = fts_build(sp, instr);
        return (*sp).fts_child;
    }
    fd = open(
        b".\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    if fd == -(1 as libc::c_int) {
        (*sp).fts_child = 0 as *mut _ftsent;
        return (*sp).fts_child;
    }
    (*sp).fts_child = fts_build(sp, instr);
    if fchdir(fd) != 0 {
        close(fd);
        return 0 as *mut FTSENT;
    }
    close(fd);
    return (*sp).fts_child;
}
unsafe extern "C" fn fts_build(
    mut sp: *mut FTS,
    mut type_0: libc::c_int,
) -> *mut FTSENT {
    let mut dp: *mut dirent = 0 as *mut dirent;
    let mut p: *mut FTSENT = 0 as *mut FTSENT;
    let mut head: *mut FTSENT = 0 as *mut FTSENT;
    let mut nitems: size_t = 0;
    let mut cur: *mut FTSENT = 0 as *mut FTSENT;
    let mut tail: *mut FTSENT = 0 as *mut FTSENT;
    let mut dirp: *mut DIR = 0 as *mut DIR;
    let mut oldaddr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut dnamlen: size_t = 0;
    let mut cderrno: libc::c_int = 0;
    let mut descend: libc::c_int = 0;
    let mut level: libc::c_int = 0;
    let mut nlinks: libc::c_int = 0;
    let mut saved_errno: libc::c_int = 0;
    let mut nostat: libc::c_int = 0;
    let mut doadjust: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut maxlen: size_t = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    cur = (*sp).fts_cur;
    dirp = opendir((*cur).fts_accpath);
    if dirp.is_null() {
        if type_0 == 3 as libc::c_int {
            (*cur).fts_info = 4 as libc::c_int as libc::c_ushort;
            (*cur).fts_errno = *__errno_location();
        }
        return 0 as *mut FTSENT;
    }
    if type_0 == 2 as libc::c_int {
        nlinks = 0 as libc::c_int;
        nostat = 1 as libc::c_int;
    } else if (*sp).fts_options & 0x8 as libc::c_int != 0
        && (*sp).fts_options & 0x10 as libc::c_int != 0
    {
        nlinks = ((*cur).fts_nlink)
            .wrapping_sub(
                (if (*sp).fts_options & 0x20 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    2 as libc::c_int
                }) as libc::c_uint,
            ) as libc::c_int;
        nostat = 1 as libc::c_int;
    } else {
        nlinks = -(1 as libc::c_int);
        nostat = 0 as libc::c_int;
    }
    cderrno = 0 as libc::c_int;
    if nlinks != 0 || type_0 == 3 as libc::c_int {
        if fts_safe_changedir(sp, cur, dirfd(dirp), 0 as *const libc::c_char) != 0 {
            if nlinks != 0 && type_0 == 3 as libc::c_int {
                (*cur).fts_errno = *__errno_location();
            }
            (*cur)
                .fts_flags = ((*cur).fts_flags as libc::c_int | 0x1 as libc::c_int)
                as libc::c_ushort;
            descend = 0 as libc::c_int;
            cderrno = *__errno_location();
        } else {
            descend = 1 as libc::c_int;
        }
    } else {
        descend = 0 as libc::c_int;
    }
    len = (if *((*cur).fts_path)
        .offset(
            ((*cur).fts_pathlen).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
        ) as libc::c_int == '/' as i32
    {
        ((*cur).fts_pathlen).wrapping_sub(1 as libc::c_int as libc::c_uint)
    } else {
        (*cur).fts_pathlen
    }) as size_t;
    if (*sp).fts_options & 0x4 as libc::c_int != 0 {
        cp = ((*sp).fts_path).offset(len as isize);
        let fresh1 = cp;
        cp = cp.offset(1);
        *fresh1 = '/' as i32 as libc::c_char;
    }
    len = len.wrapping_add(1);
    len;
    maxlen = ((*sp).fts_pathlen as libc::c_ulong).wrapping_sub(len);
    level = (*cur).fts_level + 1 as libc::c_int;
    doadjust = 0 as libc::c_int;
    let mut current_block_89: u64;
    tail = 0 as *mut FTSENT;
    head = tail;
    nitems = 0 as libc::c_int as size_t;
    loop {
        dp = readdir(dirp);
        if dp.is_null() {
            break;
        }
        if (*sp).fts_options & 0x20 as libc::c_int == 0
            && ((*dp).d_name[0 as libc::c_int as usize] as libc::c_int == '.' as i32
                && ((*dp).d_name[1 as libc::c_int as usize] == 0
                    || (*dp).d_name[1 as libc::c_int as usize] as libc::c_int
                        == '.' as i32 && (*dp).d_name[2 as libc::c_int as usize] == 0))
        {
            continue;
        }
        dnamlen = strlen(((*dp).d_name).as_mut_ptr());
        p = fts_alloc(sp, ((*dp).d_name).as_mut_ptr(), dnamlen);
        if !p.is_null() {
            if dnamlen >= maxlen {
                oldaddr = (*sp).fts_path as *mut libc::c_void;
                if fts_palloc(
                    sp,
                    dnamlen
                        .wrapping_add(len)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) != 0
                {
                    current_block_89 = 104389497517293876;
                } else {
                    if oldaddr != (*sp).fts_path as *mut libc::c_void {
                        doadjust = 1 as libc::c_int;
                        if (*sp).fts_options & 0x4 as libc::c_int != 0 {
                            cp = ((*sp).fts_path).offset(len as isize);
                        }
                    }
                    maxlen = ((*sp).fts_pathlen as libc::c_ulong).wrapping_sub(len);
                    current_block_89 = 11777552016271000781;
                }
            } else {
                current_block_89 = 11777552016271000781;
            }
            match current_block_89 {
                104389497517293876 => {}
                _ => {
                    (*p).fts_level = level;
                    (*p)
                        .fts_pathlen = if len.wrapping_add(dnamlen)
                        > (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint) as libc::c_ulong
                    {
                        (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint)
                    } else {
                        len.wrapping_add(dnamlen) as libc::c_uint
                    };
                    (*p).fts_parent = (*sp).fts_cur;
                    if cderrno != 0 {
                        if nlinks != 0 {
                            (*p).fts_info = 10 as libc::c_int as libc::c_ushort;
                            (*p).fts_errno = cderrno;
                        } else {
                            (*p).fts_info = 11 as libc::c_int as libc::c_ushort;
                        }
                        (*p).fts_accpath = (*cur).fts_accpath;
                    } else if nlinks == 0 as libc::c_int
                        || nostat != 0
                            && (*dp).d_type as libc::c_int != DT_DIR as libc::c_int
                            && (*dp).d_type as libc::c_int != DT_UNKNOWN as libc::c_int
                    {
                        (*p)
                            .fts_accpath = if (*sp).fts_options & 0x4 as libc::c_int != 0
                        {
                            (*p).fts_path
                        } else {
                            ((*p).fts_name).as_mut_ptr()
                        };
                        (*p).fts_info = 11 as libc::c_int as libc::c_ushort;
                    } else {
                        if (*sp).fts_options & 0x4 as libc::c_int != 0 {
                            (*p).fts_accpath = (*p).fts_path;
                            memmove(
                                cp as *mut libc::c_void,
                                ((*p).fts_name).as_mut_ptr() as *const libc::c_void,
                                ((*p).fts_namelen)
                                    .wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
                            );
                        } else {
                            (*p).fts_accpath = ((*p).fts_name).as_mut_ptr();
                        }
                        (*p).fts_info = fts_stat(sp, p, 0 as libc::c_int);
                        if nlinks > 0 as libc::c_int
                            && ((*p).fts_info as libc::c_int == 1 as libc::c_int
                                || (*p).fts_info as libc::c_int == 2 as libc::c_int
                                || (*p).fts_info as libc::c_int == 5 as libc::c_int)
                        {
                            nlinks -= 1;
                            nlinks;
                        }
                    }
                    (*p).fts_link = 0 as *mut _ftsent;
                    if head.is_null() {
                        tail = p;
                        head = tail;
                    } else {
                        (*tail).fts_link = p;
                        tail = p;
                    }
                    nitems = nitems.wrapping_add(1);
                    nitems;
                    continue;
                }
            }
        }
        saved_errno = *__errno_location();
        if !p.is_null() {
            fts_free(p);
        }
        fts_lfree(head);
        closedir(dirp);
        *__errno_location() = saved_errno;
        (*cur).fts_info = 7 as libc::c_int as libc::c_ushort;
        (*sp).fts_options |= 0x200 as libc::c_int;
        return 0 as *mut FTSENT;
    }
    closedir(dirp);
    if doadjust != 0 {
        fts_padjust(sp, head);
    }
    if (*sp).fts_options & 0x4 as libc::c_int != 0 {
        if len == (*sp).fts_pathlen as libc::c_ulong
            || nitems == 0 as libc::c_int as libc::c_ulong
        {
            cp = cp.offset(-1);
            cp;
        }
        *cp = '\0' as i32 as libc::c_char;
    }
    if descend != 0 && (type_0 == 1 as libc::c_int || nitems == 0)
        && (if (*cur).fts_level == 0 as libc::c_int {
            ((*sp).fts_options & 0x4 as libc::c_int == 0 && fchdir((*sp).fts_rfd) != 0)
                as libc::c_int
        } else {
            fts_safe_changedir(
                sp,
                (*cur).fts_parent,
                -(1 as libc::c_int),
                b"..\0" as *const u8 as *const libc::c_char,
            )
        }) != 0
    {
        fts_lfree(head);
        (*cur).fts_info = 7 as libc::c_int as libc::c_ushort;
        (*sp).fts_options |= 0x200 as libc::c_int;
        return 0 as *mut FTSENT;
    }
    if nitems == 0 {
        if type_0 == 3 as libc::c_int {
            (*cur).fts_info = 6 as libc::c_int as libc::c_ushort;
        }
        return 0 as *mut FTSENT;
    }
    if ((*sp).fts_compar).is_some() && nitems > 1 as libc::c_int as libc::c_ulong {
        head = fts_sort(sp, head, nitems);
    }
    return head;
}
unsafe extern "C" fn fts_stat(
    mut sp: *mut FTS,
    mut p: *mut FTSENT,
    mut follow: libc::c_int,
) -> libc::c_ushort {
    let mut t: *mut FTSENT = 0 as *mut FTSENT;
    let mut dev: dev_t = 0;
    let mut ino: ino_t = 0;
    let mut sbp: *mut stat = 0 as *mut stat;
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
    let mut saved_errno: libc::c_int = 0;
    sbp = if (*sp).fts_options & 0x8 as libc::c_int != 0 {
        &mut sb
    } else {
        (*p).fts_statp
    };
    let mut current_block_11: u64;
    if (*sp).fts_options & 0x2 as libc::c_int != 0 || follow != 0 {
        if stat((*p).fts_accpath, sbp) != 0 {
            saved_errno = *__errno_location();
            if lstat((*p).fts_accpath, sbp) == 0 {
                *__errno_location() = 0 as libc::c_int;
                return 13 as libc::c_int as libc::c_ushort;
            }
            (*p).fts_errno = saved_errno;
            current_block_11 = 8208621168702972200;
        } else {
            current_block_11 = 12800627514080957624;
        }
    } else if lstat((*p).fts_accpath, sbp) != 0 {
        (*p).fts_errno = *__errno_location();
        current_block_11 = 8208621168702972200;
    } else {
        current_block_11 = 12800627514080957624;
    }
    match current_block_11 {
        12800627514080957624 => {}
        _ => {
            memset(
                sbp as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<stat>() as libc::c_ulong,
            );
            return 10 as libc::c_int as libc::c_ushort;
        }
    }
    if (*sbp).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint
    {
        (*p).fts_dev = (*sbp).st_dev;
        dev = (*p).fts_dev;
        (*p).fts_ino = (*sbp).st_ino;
        ino = (*p).fts_ino;
        (*p).fts_nlink = (*sbp).st_nlink as libc::c_uint;
        if *((*p).fts_name).as_mut_ptr().offset(0 as libc::c_int as isize) as libc::c_int
            == '.' as i32
            && (*((*p).fts_name).as_mut_ptr().offset(1 as libc::c_int as isize) == 0
                || *((*p).fts_name).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as libc::c_int == '.' as i32
                    && *((*p).fts_name).as_mut_ptr().offset(2 as libc::c_int as isize)
                        == 0)
        {
            return 5 as libc::c_int as libc::c_ushort;
        }
        t = (*p).fts_parent;
        while (*t).fts_level >= 0 as libc::c_int {
            if ino == (*t).fts_ino && dev == (*t).fts_dev {
                (*p).fts_cycle = t;
                return 2 as libc::c_int as libc::c_ushort;
            }
            t = (*t).fts_parent;
        }
        return 1 as libc::c_int as libc::c_ushort;
    }
    if (*sbp).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o120000 as libc::c_int as libc::c_uint
    {
        return 12 as libc::c_int as libc::c_ushort;
    }
    if (*sbp).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
    {
        return 8 as libc::c_int as libc::c_ushort;
    }
    return 3 as libc::c_int as libc::c_ushort;
}
unsafe extern "C" fn fts_sort(
    mut sp: *mut FTS,
    mut head: *mut FTSENT,
    mut nitems: size_t,
) -> *mut FTSENT {
    let mut ap: *mut *mut FTSENT = 0 as *mut *mut FTSENT;
    let mut p: *mut FTSENT = 0 as *mut FTSENT;
    if nitems > (*sp).fts_nitems as libc::c_ulong {
        let mut new: *mut *mut FTSENT = 0 as *mut *mut FTSENT;
        new = realloc(
            (*sp).fts_array as *mut libc::c_void,
            (::std::mem::size_of::<*mut FTSENT>() as libc::c_ulong)
                .wrapping_mul(nitems.wrapping_add(40 as libc::c_int as libc::c_ulong)),
        ) as *mut *mut FTSENT;
        if new.is_null() {
            return head;
        }
        (*sp).fts_array = new;
        (*sp)
            .fts_nitems = if nitems.wrapping_add(40 as libc::c_int as libc::c_ulong)
            > (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint) as libc::c_ulong
        {
            (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
        } else {
            nitems.wrapping_add(40 as libc::c_int as libc::c_ulong) as libc::c_uint
        };
    }
    ap = (*sp).fts_array;
    p = head;
    while !p.is_null() {
        let fresh2 = ap;
        ap = ap.offset(1);
        *fresh2 = p;
        p = (*p).fts_link;
    }
    qsort(
        (*sp).fts_array as *mut libc::c_void,
        nitems,
        ::std::mem::size_of::<*mut FTSENT>() as libc::c_ulong,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut *const _ftsent,
                    *mut *const _ftsent,
                ) -> libc::c_int,
            >,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
            >,
        >((*sp).fts_compar),
    );
    ap = (*sp).fts_array;
    head = *ap;
    loop {
        nitems = nitems.wrapping_sub(1);
        if !(nitems != 0) {
            break;
        }
        let ref mut fresh3 = (**ap.offset(0 as libc::c_int as isize)).fts_link;
        *fresh3 = *ap.offset(1 as libc::c_int as isize);
        ap = ap.offset(1);
        ap;
    }
    let ref mut fresh4 = (**ap.offset(0 as libc::c_int as isize)).fts_link;
    *fresh4 = 0 as *mut _ftsent;
    return head;
}
unsafe extern "C" fn fts_alloc(
    mut sp: *mut FTS,
    mut name: *const libc::c_char,
    mut namelen: size_t,
) -> *mut FTSENT {
    let mut p: *mut FTSENT = 0 as *mut FTSENT;
    p = malloc((::std::mem::size_of::<FTSENT>() as libc::c_ulong).wrapping_add(namelen))
        as *mut FTSENT;
    if p.is_null() {
        return 0 as *mut FTSENT;
    }
    if (*sp).fts_options & 0x8 as libc::c_int == 0 {
        (*p)
            .fts_statp = malloc(::std::mem::size_of::<stat>() as libc::c_ulong)
            as *mut stat;
        if ((*p).fts_statp).is_null() {
            free(p as *mut libc::c_void);
            return 0 as *mut FTSENT;
        }
    }
    if (*sp).fts_options & 0x8 as libc::c_int != 0 {
        (*p).fts_statp = 0 as *mut stat;
    }
    memmove(
        ((*p).fts_name).as_mut_ptr() as *mut libc::c_void,
        name as *const libc::c_void,
        namelen.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    (*p)
        .fts_namelen = if namelen
        > (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_ulong
    {
        (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint)
    } else {
        namelen as libc::c_uint
    };
    (*p).fts_path = (*sp).fts_path;
    (*p).fts_errno = 0 as libc::c_int;
    (*p).fts_flags = 0 as libc::c_int as libc::c_ushort;
    (*p).fts_instr = 3 as libc::c_int as libc::c_ushort;
    (*p).fts_number = 0 as libc::c_int as libc::c_longlong;
    (*p).fts_pointer = 0 as *mut libc::c_void;
    return p;
}
unsafe extern "C" fn fts_free(mut p: *mut FTSENT) {
    if !((*p).fts_statp).is_null() {
        free((*p).fts_statp as *mut libc::c_void);
    }
    free(p as *mut libc::c_void);
}
unsafe extern "C" fn fts_lfree(mut head: *mut FTSENT) {
    let mut p: *mut FTSENT = 0 as *mut FTSENT;
    loop {
        p = head;
        if p.is_null() {
            break;
        }
        head = (*head).fts_link;
        fts_free(p);
    };
}
unsafe extern "C" fn fts_pow2(mut x: size_t) -> size_t {
    x = x.wrapping_sub(1);
    x;
    x |= x >> 1 as libc::c_int;
    x |= x >> 2 as libc::c_int;
    x |= x >> 4 as libc::c_int;
    x |= x >> 8 as libc::c_int;
    x |= x >> 16 as libc::c_int;
    x |= x >> 32 as libc::c_int;
    x = x.wrapping_add(1);
    x;
    return x;
}
unsafe extern "C" fn fts_palloc(mut sp: *mut FTS, mut size: size_t) -> libc::c_int {
    let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
    size = fts_pow2(size);
    new = realloc((*sp).fts_path as *mut libc::c_void, size) as *mut libc::c_char;
    if new.is_null() {
        return 1 as libc::c_int;
    }
    (*sp).fts_path = new;
    (*sp)
        .fts_pathlen = if size
        > (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_ulong
    {
        (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint)
    } else {
        size as libc::c_uint
    };
    return 0 as libc::c_int;
}
unsafe extern "C" fn fts_padjust(mut sp: *mut FTS, mut head: *mut FTSENT) {
    let mut p: *mut FTSENT = 0 as *mut FTSENT;
    let mut addr: *mut libc::c_char = 0 as *mut libc::c_char;
    addr = (*sp).fts_path;
    p = (*sp).fts_child;
    while !p.is_null() {
        if (*p).fts_accpath != ((*p).fts_name).as_mut_ptr() {
            (*p)
                .fts_accpath = addr
                .offset(
                    ((*p).fts_accpath).offset_from((*p).fts_path) as libc::c_long
                        as isize,
                );
        }
        (*p).fts_path = addr;
        p = (*p).fts_link;
    }
    p = head;
    while (*p).fts_level >= 0 as libc::c_int {
        if (*p).fts_accpath != ((*p).fts_name).as_mut_ptr() {
            (*p)
                .fts_accpath = addr
                .offset(
                    ((*p).fts_accpath).offset_from((*p).fts_path) as libc::c_long
                        as isize,
                );
        }
        (*p).fts_path = addr;
        p = if !((*p).fts_link).is_null() { (*p).fts_link } else { (*p).fts_parent };
    }
}
unsafe extern "C" fn fts_maxarglen(mut argv: *const *mut libc::c_char) -> size_t {
    let mut len: size_t = 0;
    let mut max: size_t = 0;
    max = 0 as libc::c_int as size_t;
    while !(*argv).is_null() {
        len = strlen(*argv);
        if len > max {
            max = len;
        }
        argv = argv.offset(1);
        argv;
    }
    return max.wrapping_add(1 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn fts_safe_changedir(
    mut sp: *const FTS,
    mut p: *const FTSENT,
    mut fd: libc::c_int,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut oldfd: libc::c_int = fd;
    let mut ret: libc::c_int = -(1 as libc::c_int);
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
    if (*sp).fts_options & 0x4 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    if oldfd < 0 as libc::c_int
        && (path.is_null()
            || {
                fd = open(path, 0 as libc::c_int);
                fd == -(1 as libc::c_int)
            })
    {
        return -(1 as libc::c_int);
    }
    if !(fstat(fd, &mut sb) == -(1 as libc::c_int)) {
        if sb.st_ino != (*p).fts_ino || sb.st_dev != (*p).fts_dev {
            *__errno_location() = 2 as libc::c_int;
        } else {
            ret = fchdir(fd);
        }
    }
    if oldfd < 0 as libc::c_int {
        let mut save_errno: libc::c_int = *__errno_location();
        close(fd);
        *__errno_location() = save_errno;
    }
    return ret;
}
