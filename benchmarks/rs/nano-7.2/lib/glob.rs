use ::libc;
extern "C" {
    pub type __dirstream;
    fn __errno_location() -> *mut libc::c_int;
    fn __fxstatat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn rpl_globfree(__pglob: *mut glob_t);
    fn getlogin_r(__name: *mut libc::c_char, __name_len: size_t) -> libc::c_int;
    fn getpwnam_r(
        __name: *const libc::c_char,
        __resultbuf: *mut passwd,
        __buffer: *mut libc::c_char,
        __buflen: size_t,
        __result: *mut *mut passwd,
    ) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn dirfd(__dirp: *mut DIR) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn rpl_free(ptr: *mut libc::c_void);
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcoll(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn gl_scratch_buffer_grow(buffer: *mut scratch_buffer) -> bool;
    fn gl_scratch_buffer_set_array_size(
        buffer: *mut scratch_buffer,
        nelem: size_t,
        size: size_t,
    ) -> bool;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: libc::c_longlong,
    pub __clang_max_align_nonce2: f128::f128,
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
pub struct glob_t {
    pub gl_pathc: size_t,
    pub gl_pathv: *mut *mut libc::c_char,
    pub gl_offs: size_t,
    pub gl_flags: libc::c_int,
    pub gl_closedir: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub gl_readdir: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut dirent>,
    pub gl_opendir: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void,
    >,
    pub gl_lstat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
    pub gl_stat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scratch_buffer {
    pub data: *mut libc::c_void,
    pub length: size_t,
    pub __space: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __align: max_align_t,
    pub __c: [libc::c_char; 1024],
}
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globnames {
    pub next: *mut globnames,
    pub count: size_t,
    pub name: [*mut libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub next: *mut globnames,
    pub count: size_t,
    pub name: [*mut libc::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct readdir_result {
    pub name: *const libc::c_char,
    pub type_0: dirent_type,
}
pub type dirent_type = uint_fast32_t;
pub type uint_fast32_t = libc::c_ulong;
pub const DT_UNKNOWN: C2RustUnnamed_2 = 0;
pub const DT_LNK: C2RustUnnamed_2 = 10;
pub const DT_DIR: C2RustUnnamed_2 = 4;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub st: stat,
    pub st64: stat,
}
pub const GLOBPAT_NONE: C2RustUnnamed_3 = 0;
pub const GLOBPAT_SPECIAL: C2RustUnnamed_3 = 1;
pub const GLOBPAT_BRACKET: C2RustUnnamed_3 = 4;
pub const GLOBPAT_BACKSLASH: C2RustUnnamed_3 = 2;
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const DT_WHT: C2RustUnnamed_2 = 14;
pub const DT_SOCK: C2RustUnnamed_2 = 12;
pub const DT_REG: C2RustUnnamed_2 = 8;
pub const DT_BLK: C2RustUnnamed_2 = 6;
pub const DT_CHR: C2RustUnnamed_2 = 2;
pub const DT_FIFO: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_3 = libc::c_uint;
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
unsafe extern "C" fn __glob_pattern_type(
    mut pattern: *const libc::c_char,
    mut quote: libc::c_int,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: libc::c_int = GLOBPAT_NONE as libc::c_int;
    p = pattern;
    while *p as libc::c_int != '\0' as i32 {
        match *p as libc::c_int {
            63 | 42 => return GLOBPAT_SPECIAL as libc::c_int,
            92 => {
                if quote != 0 {
                    if *p.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
                    {
                        p = p.offset(1);
                        p;
                    }
                    ret |= GLOBPAT_BACKSLASH as libc::c_int;
                }
            }
            91 => {
                ret |= GLOBPAT_BRACKET as libc::c_int;
            }
            93 => {
                if ret & 4 as libc::c_int != 0 {
                    return GLOBPAT_SPECIAL as libc::c_int;
                }
            }
            _ => {}
        }
        p = p.offset(1);
        p;
    }
    return ret;
}
#[inline]
unsafe extern "C" fn scratch_buffer_init(mut buffer: *mut scratch_buffer) {
    (*buffer).data = ((*buffer).__space.__c).as_mut_ptr() as *mut libc::c_void;
    (*buffer).length = ::std::mem::size_of::<C2RustUnnamed>() as libc::c_ulong;
}
#[inline]
unsafe extern "C" fn scratch_buffer_free(mut buffer: *mut scratch_buffer) {
    if (*buffer).data != ((*buffer).__space.__c).as_mut_ptr() as *mut libc::c_void {
        rpl_free((*buffer).data);
    }
}
#[inline(always)]
unsafe extern "C" fn scratch_buffer_grow(mut buffer: *mut scratch_buffer) -> bool {
    return gl_scratch_buffer_grow(buffer) as libc::c_long != 0;
}
#[inline(always)]
unsafe extern "C" fn scratch_buffer_set_array_size(
    mut buffer: *mut scratch_buffer,
    mut nelem: size_t,
    mut size: size_t,
) -> bool {
    return gl_scratch_buffer_set_array_size(buffer, nelem, size) as libc::c_long != 0;
}
unsafe extern "C" fn readdir_result_type(mut d: readdir_result) -> dirent_type {
    return d.type_0;
}
unsafe extern "C" fn convert_dirent(mut source: *const dirent) -> readdir_result {
    if source.is_null() {
        let mut result: readdir_result = {
            let mut init = readdir_result {
                name: 0 as *const libc::c_char,
                type_0: 0,
            };
            init
        };
        return result;
    }
    let mut result_0: readdir_result = {
        let mut init = readdir_result {
            name: ((*source).d_name).as_ptr(),
            type_0: (*source).d_type as dirent_type,
        };
        init
    };
    return result_0;
}
unsafe extern "C" fn glob_lstat(
    mut pglob: *mut glob_t,
    mut flags: libc::c_int,
    mut fullname: *const libc::c_char,
) -> libc::c_int {
    let mut ust: C2RustUnnamed_1 = C2RustUnnamed_1 {
        st: stat {
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
        },
    };
    return if (flags & (1 as libc::c_int) << 9 as libc::c_int) as libc::c_long != 0 {
        ((*pglob).gl_lstat).unwrap()(fullname, &mut ust.st)
    } else {
        fstatat(-(100 as libc::c_int), fullname, &mut ust.st64, 0x100 as libc::c_int)
    };
}
unsafe extern "C" fn size_add_wrapv(
    mut a: size_t,
    mut b: size_t,
    mut r: *mut size_t,
) -> bool {
    *r = a.wrapping_add(b);
    return *r < a;
}
unsafe extern "C" fn glob_use_alloca(mut alloca_used: size_t, mut len: size_t) -> bool {
    let mut size: size_t = 0;
    return !size_add_wrapv(alloca_used, len, &mut size) && 0 as libc::c_int != 0;
}
unsafe extern "C" fn is_dir(
    mut filename: *const libc::c_char,
    mut flags: libc::c_int,
    mut pglob: *const glob_t,
) -> bool {
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
    let mut st64: stat = stat {
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
    return if (flags & (1 as libc::c_int) << 9 as libc::c_int) as libc::c_long != 0 {
        (((*pglob).gl_stat).unwrap()(filename, &mut st) == 0 as libc::c_int
            && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int
    } else {
        (fstatat(-(100 as libc::c_int), filename, &mut st64, 0 as libc::c_int)
            == 0 as libc::c_int
            && st64.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int
    } != 0;
}
unsafe extern "C" fn next_brace_sub(
    mut cp: *const libc::c_char,
    mut flags: libc::c_int,
) -> *const libc::c_char {
    let mut depth: size_t = 0 as libc::c_int as size_t;
    while *cp as libc::c_int != '\0' as i32 {
        if flags & (1 as libc::c_int) << 6 as libc::c_int == 0 as libc::c_int
            && *cp as libc::c_int == '\\' as i32
        {
            cp = cp.offset(1);
            if *cp as libc::c_int == '\0' as i32 {
                break;
            }
            cp = cp.offset(1);
            cp;
        } else {
            if *cp as libc::c_int == '}' as i32
                && {
                    let fresh0 = depth;
                    depth = depth.wrapping_sub(1);
                    fresh0 == 0 as libc::c_int as libc::c_ulong
                }
                || *cp as libc::c_int == ',' as i32
                    && depth == 0 as libc::c_int as libc::c_ulong
            {
                break;
            }
            let fresh1 = cp;
            cp = cp.offset(1);
            if *fresh1 as libc::c_int == '{' as i32 {
                depth = depth.wrapping_add(1);
                depth;
            }
        }
    }
    return if *cp as libc::c_int != '\0' as i32 { cp } else { 0 as *const libc::c_char };
}
pub unsafe extern "C" fn rpl_glob(
    mut pattern: *const libc::c_char,
    mut flags: libc::c_int,
    mut errfunc: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int,
    >,
    mut pglob: *mut glob_t,
) -> libc::c_int {
    let mut i_0: size_t = 0;
    let mut current_block: u64;
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut dirname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dirlen: size_t = 0;
    let mut status: libc::c_int = 0;
    let mut oldcount: size_t = 0;
    let mut meta: libc::c_int = 0;
    let mut dirname_modified: libc::c_int = 0;
    let mut malloc_dirname: libc::c_int = 0 as libc::c_int;
    let mut dirs: glob_t = glob_t {
        gl_pathc: 0,
        gl_pathv: 0 as *mut *mut libc::c_char,
        gl_offs: 0,
        gl_flags: 0,
        gl_closedir: None,
        gl_readdir: None,
        gl_opendir: None,
        gl_lstat: None,
        gl_stat: None,
    };
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut alloca_used: size_t = 0 as libc::c_int as size_t;
    if pattern.is_null() || pglob.is_null()
        || flags
            & !((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 5 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int
                | (1 as libc::c_int) << 9 as libc::c_int
                | (1 as libc::c_int) << 10 as libc::c_int
                | (1 as libc::c_int) << 11 as libc::c_int
                | (1 as libc::c_int) << 12 as libc::c_int
                | (1 as libc::c_int) << 13 as libc::c_int
                | (1 as libc::c_int) << 14 as libc::c_int) != 0 as libc::c_int
    {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if *pattern.offset(0 as libc::c_int as isize) as libc::c_int != 0
        && *pattern
            .offset(
                (strlen(pattern)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as isize,
            ) as libc::c_int == '/' as i32
    {
        flags |= (1 as libc::c_int) << 13 as libc::c_int;
    }
    if flags & (1 as libc::c_int) << 3 as libc::c_int == 0 {
        (*pglob).gl_offs = 0 as libc::c_int as size_t;
    }
    if flags & (1 as libc::c_int) << 5 as libc::c_int == 0 {
        (*pglob).gl_pathc = 0 as libc::c_int as size_t;
        if flags & (1 as libc::c_int) << 3 as libc::c_int == 0 {
            (*pglob).gl_pathv = 0 as *mut *mut libc::c_char;
        } else {
            let mut i: size_t = 0;
            if (*pglob).gl_offs
                >= (!(0 as libc::c_int as size_t))
                    .wrapping_div(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    )
            {
                return 1 as libc::c_int;
            }
            (*pglob)
                .gl_pathv = malloc(
                ((*pglob).gl_offs)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *mut libc::c_char;
            if ((*pglob).gl_pathv).is_null() {
                return 1 as libc::c_int;
            }
            i = 0 as libc::c_int as size_t;
            while i <= (*pglob).gl_offs {
                let ref mut fresh2 = *((*pglob).gl_pathv).offset(i as isize);
                *fresh2 = 0 as *mut libc::c_char;
                i = i.wrapping_add(1);
                i;
            }
        }
    }
    if flags & (1 as libc::c_int) << 10 as libc::c_int != 0 {
        let mut begin: *const libc::c_char = 0 as *const libc::c_char;
        if flags & (1 as libc::c_int) << 6 as libc::c_int != 0 {
            begin = strchr(pattern, '{' as i32);
        } else {
            begin = pattern;
            loop {
                if *begin as libc::c_int == '\0' as i32 {
                    begin = 0 as *const libc::c_char;
                    break;
                } else {
                    if *begin as libc::c_int == '\\' as i32
                        && *begin.offset(1 as libc::c_int as isize) as libc::c_int
                            != '\0' as i32
                    {
                        begin = begin.offset(1);
                        begin;
                    } else if *begin as libc::c_int == '{' as i32 {
                        break;
                    }
                    begin = begin.offset(1);
                    begin;
                }
            }
        }
        if !begin.is_null() {
            let mut firstc: size_t = 0;
            let mut alt_start: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut p: *const libc::c_char = 0 as *const libc::c_char;
            let mut next: *const libc::c_char = 0 as *const libc::c_char;
            let mut rest: *const libc::c_char = 0 as *const libc::c_char;
            let mut rest_len: size_t = 0;
            let mut onealt: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut pattern_len: size_t = (strlen(pattern))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let mut alloca_onealt: libc::c_int = glob_use_alloca(
                alloca_used,
                pattern_len,
            ) as libc::c_int;
            if alloca_onealt != 0 {
                onealt = 0 as *mut libc::c_void as *mut libc::c_char;
            } else {
                onealt = malloc(pattern_len) as *mut libc::c_char;
                if onealt.is_null() {
                    return 1 as libc::c_int;
                }
            }
            alt_start = mempcpy(
                onealt as *mut libc::c_void,
                pattern as *const libc::c_void,
                begin.offset_from(pattern) as libc::c_long as libc::c_ulong,
            ) as *mut libc::c_char;
            next = next_brace_sub(begin.offset(1 as libc::c_int as isize), flags);
            if next.is_null() {
                current_block = 5281296928883327999;
            } else {
                rest = next;
                loop {
                    if !(*rest as libc::c_int != '}' as i32) {
                        current_block = 5891011138178424807;
                        break;
                    }
                    rest = next_brace_sub(rest.offset(1 as libc::c_int as isize), flags);
                    if rest.is_null() {
                        current_block = 5281296928883327999;
                        break;
                    }
                }
                match current_block {
                    5281296928883327999 => {}
                    _ => {
                        rest = rest.offset(1);
                        rest_len = (strlen(rest))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong);
                        firstc = (*pglob).gl_pathc;
                        p = begin.offset(1 as libc::c_int as isize);
                        loop {
                            let mut result: libc::c_int = 0;
                            mempcpy(
                                mempcpy(
                                    alt_start as *mut libc::c_void,
                                    p as *const libc::c_void,
                                    next.offset_from(p) as libc::c_long as libc::c_ulong,
                                ),
                                rest as *const libc::c_void,
                                rest_len,
                            );
                            result = rpl_glob(
                                onealt,
                                flags
                                    & !((1 as libc::c_int) << 4 as libc::c_int
                                        | (1 as libc::c_int) << 11 as libc::c_int)
                                    | (1 as libc::c_int) << 5 as libc::c_int,
                                errfunc,
                                pglob,
                            );
                            if result != 0 && result != 3 as libc::c_int {
                                if (alloca_onealt == 0) as libc::c_int as libc::c_long != 0
                                {
                                    rpl_free(onealt as *mut libc::c_void);
                                }
                                if flags & (1 as libc::c_int) << 5 as libc::c_int == 0 {
                                    rpl_globfree(pglob);
                                    (*pglob).gl_pathc = 0 as libc::c_int as size_t;
                                }
                                return result;
                            }
                            if *next as libc::c_int == '}' as i32 {
                                break;
                            }
                            p = next.offset(1 as libc::c_int as isize);
                            next = next_brace_sub(p, flags);
                        }
                        if (alloca_onealt == 0) as libc::c_int as libc::c_long != 0 {
                            rpl_free(onealt as *mut libc::c_void);
                        }
                        if (*pglob).gl_pathc != firstc {
                            return 0 as libc::c_int
                        } else if flags
                            & ((1 as libc::c_int) << 4 as libc::c_int
                                | (1 as libc::c_int) << 11 as libc::c_int) == 0
                        {
                            return 3 as libc::c_int
                        }
                        current_block = 14221408203774355243;
                    }
                }
            }
            match current_block {
                14221408203774355243 => {}
                _ => {
                    if (alloca_onealt == 0) as libc::c_int as libc::c_long != 0 {
                        rpl_free(onealt as *mut libc::c_void);
                    }
                    flags &= !((1 as libc::c_int) << 10 as libc::c_int);
                }
            }
        }
    }
    oldcount = ((*pglob).gl_pathc).wrapping_add((*pglob).gl_offs);
    filename = strrchr(pattern, '/' as i32);
    dirname_modified = 0 as libc::c_int;
    if filename.is_null() {
        if flags
            & ((1 as libc::c_int) << 12 as libc::c_int
                | (1 as libc::c_int) << 14 as libc::c_int) != 0
            && *pattern.offset(0 as libc::c_int as isize) as libc::c_int == '~' as i32
        {
            dirname = pattern as *mut libc::c_char;
            dirlen = strlen(pattern);
            filename = 0 as *const libc::c_char;
            current_block = 13201766686570145889;
        } else if (*pattern.offset(0 as libc::c_int as isize) as libc::c_int
            == '\0' as i32) as libc::c_int as libc::c_long != 0
        {
            dirs.gl_pathv = 0 as *mut *mut libc::c_char;
            current_block = 12238912827727375958;
        } else {
            filename = pattern;
            dirname = b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            dirlen = 0 as libc::c_int as size_t;
            current_block = 13201766686570145889;
        }
    } else if filename == pattern
        || filename == pattern.offset(1 as libc::c_int as isize)
            && *pattern.offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32
            && flags & (1 as libc::c_int) << 6 as libc::c_int == 0 as libc::c_int
    {
        dirname = b"/\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        dirlen = 1 as libc::c_int as size_t;
        filename = filename.offset(1);
        filename;
        current_block = 13201766686570145889;
    } else {
        let mut newp: *mut libc::c_char = 0 as *mut libc::c_char;
        dirlen = filename.offset_from(pattern) as libc::c_long as size_t;
        if glob_use_alloca(
            alloca_used,
            dirlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) {
            newp = 0 as *mut libc::c_void as *mut libc::c_char;
        } else {
            newp = malloc(dirlen.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as *mut libc::c_char;
            if newp.is_null() {
                return 1 as libc::c_int;
            }
            malloc_dirname = 1 as libc::c_int;
        }
        *(mempcpy(newp as *mut libc::c_void, pattern as *const libc::c_void, dirlen)
            as *mut libc::c_char) = '\0' as i32 as libc::c_char;
        dirname = newp;
        filename = filename.offset(1);
        filename;
        let mut drive_root: bool = 0 as libc::c_int != 0;
        if *filename.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
            && dirlen > 1 as libc::c_int as libc::c_ulong && !drive_root
        {
            let mut orig_flags: libc::c_int = flags;
            if flags & (1 as libc::c_int) << 6 as libc::c_int == 0
                && *dirname
                    .offset(
                        dirlen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int == '\\' as i32
            {
                let mut p_0: *mut libc::c_char = &mut *dirname
                    .offset(
                        dirlen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as *mut libc::c_char;
                while p_0 > dirname
                    && *p_0.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '\\' as i32
                {
                    p_0 = p_0.offset(-1);
                    p_0;
                }
                if (&mut *dirname.offset(dirlen as isize) as *mut libc::c_char)
                    .offset_from(p_0) as libc::c_long & 1 as libc::c_int as libc::c_long
                    != 0
                {
                    dirlen = dirlen.wrapping_sub(1);
                    *(&mut *dirname.offset(dirlen as isize)
                        as *mut libc::c_char) = '\0' as i32 as libc::c_char;
                    flags
                        &= !((1 as libc::c_int) << 4 as libc::c_int
                            | (1 as libc::c_int) << 11 as libc::c_int);
                }
            }
            let mut val: libc::c_int = rpl_glob(
                dirname,
                flags | (1 as libc::c_int) << 1 as libc::c_int,
                errfunc,
                pglob,
            );
            if val == 0 as libc::c_int {
                (*pglob)
                    .gl_flags = (*pglob).gl_flags
                    & !((1 as libc::c_int) << 1 as libc::c_int)
                    | flags & (1 as libc::c_int) << 1 as libc::c_int;
                current_block = 14483658890531361756;
            } else if val == 3 as libc::c_int && flags != orig_flags {
                dirs.gl_pathv = 0 as *mut *mut libc::c_char;
                flags = orig_flags;
                oldcount = ((*pglob).gl_pathc).wrapping_add((*pglob).gl_offs);
                current_block = 12238912827727375958;
            } else {
                current_block = 14483658890531361756;
            }
            match current_block {
                12238912827727375958 => {}
                _ => {
                    retval = val;
                    current_block = 4508334632585049825;
                }
            }
        } else {
            current_block = 13201766686570145889;
        }
    }
    match current_block {
        13201766686570145889 => {
            if flags
                & ((1 as libc::c_int) << 12 as libc::c_int
                    | (1 as libc::c_int) << 14 as libc::c_int) != 0
                && *dirname.offset(0 as libc::c_int as isize) as libc::c_int
                    == '~' as i32
            {
                if *dirname.offset(1 as libc::c_int as isize) as libc::c_int
                    == '\0' as i32
                    || *dirname.offset(1 as libc::c_int as isize) as libc::c_int
                        == '/' as i32
                    || flags & (1 as libc::c_int) << 6 as libc::c_int == 0
                        && *dirname.offset(1 as libc::c_int as isize) as libc::c_int
                            == '\\' as i32
                        && (*dirname.offset(2 as libc::c_int as isize) as libc::c_int
                            == '\0' as i32
                            || *dirname.offset(2 as libc::c_int as isize) as libc::c_int
                                == '/' as i32)
                {
                    let mut home_dir: *mut libc::c_char = getenv(
                        b"HOME\0" as *const u8 as *const libc::c_char,
                    );
                    let mut malloc_home_dir: libc::c_int = 0 as libc::c_int;
                    if home_dir.is_null()
                        || *home_dir.offset(0 as libc::c_int as isize) as libc::c_int
                            == '\0' as i32
                    {
                        let mut err: libc::c_int = 0;
                        let mut p_1: *mut passwd = 0 as *mut passwd;
                        let mut pwbuf: passwd = passwd {
                            pw_name: 0 as *mut libc::c_char,
                            pw_passwd: 0 as *mut libc::c_char,
                            pw_uid: 0,
                            pw_gid: 0,
                            pw_gecos: 0 as *mut libc::c_char,
                            pw_dir: 0 as *mut libc::c_char,
                            pw_shell: 0 as *mut libc::c_char,
                        };
                        let mut s: scratch_buffer = scratch_buffer {
                            data: 0 as *mut libc::c_void,
                            length: 0,
                            __space: C2RustUnnamed {
                                __align: max_align_t {
                                    __clang_max_align_nonce1: 0,
                                    __clang_max_align_nonce2: f128::f128::ZERO,
                                },
                            },
                        };
                        scratch_buffer_init(&mut s);
                        loop {
                            p_1 = 0 as *mut passwd;
                            err = getlogin_r(s.data as *mut libc::c_char, s.length);
                            if err == 0 as libc::c_int {
                                let mut ssize: size_t = (strlen(
                                    s.data as *const libc::c_char,
                                ))
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                                let mut sdata: *mut libc::c_char = s.data
                                    as *mut libc::c_char;
                                err = getpwnam_r(
                                    sdata,
                                    &mut pwbuf,
                                    sdata.offset(ssize as isize),
                                    (s.length).wrapping_sub(ssize),
                                    &mut p_1,
                                );
                            }
                            if err != 34 as libc::c_int {
                                current_block = 7728257318064351663;
                                break;
                            }
                            if scratch_buffer_grow(&mut s) {
                                continue;
                            }
                            retval = 1 as libc::c_int;
                            current_block = 4508334632585049825;
                            break;
                        }
                        match current_block {
                            4508334632585049825 => {}
                            _ => {
                                if err == 0 as libc::c_int {
                                    home_dir = strdup((*p_1).pw_dir);
                                    malloc_home_dir = 1 as libc::c_int;
                                }
                                scratch_buffer_free(&mut s);
                                if err == 0 as libc::c_int && home_dir.is_null() {
                                    retval = 1 as libc::c_int;
                                    current_block = 4508334632585049825;
                                } else {
                                    current_block = 12625034345703806147;
                                }
                            }
                        }
                    } else {
                        current_block = 12625034345703806147;
                    }
                    match current_block {
                        4508334632585049825 => {}
                        _ => {
                            if home_dir.is_null()
                                || *home_dir.offset(0 as libc::c_int as isize)
                                    as libc::c_int == '\0' as i32
                            {
                                if malloc_home_dir as libc::c_long != 0 {
                                    rpl_free(home_dir as *mut libc::c_void);
                                }
                                if flags & (1 as libc::c_int) << 14 as libc::c_int != 0 {
                                    retval = 3 as libc::c_int;
                                    current_block = 4508334632585049825;
                                } else {
                                    home_dir = b"~\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char;
                                    malloc_home_dir = 0 as libc::c_int;
                                    current_block = 5431927413890720344;
                                }
                            } else {
                                current_block = 5431927413890720344;
                            }
                            match current_block {
                                4508334632585049825 => {}
                                _ => {
                                    if *dirname.offset(1 as libc::c_int as isize) as libc::c_int
                                        == '\0' as i32
                                    {
                                        if malloc_dirname as libc::c_long != 0 {
                                            rpl_free(dirname as *mut libc::c_void);
                                        }
                                        dirname = home_dir;
                                        dirlen = strlen(dirname);
                                        malloc_dirname = malloc_home_dir;
                                        current_block = 5388205036907793036;
                                    } else {
                                        let mut newp_0: *mut libc::c_char = 0 as *mut libc::c_char;
                                        let mut home_len: size_t = strlen(home_dir);
                                        let mut use_alloca: libc::c_int = glob_use_alloca(
                                            alloca_used,
                                            home_len.wrapping_add(dirlen),
                                        ) as libc::c_int;
                                        if use_alloca != 0 {
                                            newp_0 = 0 as *mut libc::c_void as *mut libc::c_char;
                                            current_block = 5832582820025303349;
                                        } else {
                                            newp_0 = malloc(home_len.wrapping_add(dirlen))
                                                as *mut libc::c_char;
                                            if newp_0.is_null() {
                                                if malloc_home_dir as libc::c_long != 0 {
                                                    rpl_free(home_dir as *mut libc::c_void);
                                                }
                                                retval = 1 as libc::c_int;
                                                current_block = 4508334632585049825;
                                            } else {
                                                current_block = 5832582820025303349;
                                            }
                                        }
                                        match current_block {
                                            4508334632585049825 => {}
                                            _ => {
                                                mempcpy(
                                                    mempcpy(
                                                        newp_0 as *mut libc::c_void,
                                                        home_dir as *const libc::c_void,
                                                        home_len,
                                                    ),
                                                    &mut *dirname.offset(1 as libc::c_int as isize)
                                                        as *mut libc::c_char as *const libc::c_void,
                                                    dirlen,
                                                );
                                                if malloc_dirname as libc::c_long != 0 {
                                                    rpl_free(dirname as *mut libc::c_void);
                                                }
                                                dirname = newp_0;
                                                dirlen = (dirlen as libc::c_ulong)
                                                    .wrapping_add(
                                                        home_len.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                                    ) as size_t as size_t;
                                                malloc_dirname = (use_alloca == 0) as libc::c_int;
                                                if malloc_home_dir as libc::c_long != 0 {
                                                    rpl_free(home_dir as *mut libc::c_void);
                                                }
                                                current_block = 5388205036907793036;
                                            }
                                        }
                                    }
                                    match current_block {
                                        4508334632585049825 => {}
                                        _ => {
                                            dirname_modified = 1 as libc::c_int;
                                            current_block = 7188795011561844502;
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    let mut end_name: *mut libc::c_char = strchr(dirname, '/' as i32);
                    let mut user_name: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut malloc_user_name: libc::c_int = 0 as libc::c_int;
                    let mut unescape: *mut libc::c_char = 0 as *mut libc::c_char;
                    if flags & (1 as libc::c_int) << 6 as libc::c_int == 0 {
                        if end_name.is_null() {
                            unescape = strchr(dirname, '\\' as i32);
                            if !unescape.is_null() {
                                end_name = strchr(unescape, '\0' as i32);
                            }
                        } else {
                            unescape = memchr(
                                dirname as *const libc::c_void,
                                '\\' as i32,
                                end_name.offset_from(dirname) as libc::c_long
                                    as libc::c_ulong,
                            ) as *mut libc::c_char;
                        }
                    }
                    if end_name.is_null() {
                        user_name = dirname.offset(1 as libc::c_int as isize);
                        current_block = 2920409193602730479;
                    } else {
                        let mut newp_1: *mut libc::c_char = 0 as *mut libc::c_char;
                        if glob_use_alloca(
                            alloca_used,
                            end_name.offset_from(dirname) as libc::c_long as size_t,
                        ) {
                            newp_1 = 0 as *mut libc::c_void as *mut libc::c_char;
                            current_block = 2408932541243239002;
                        } else {
                            newp_1 = malloc(
                                end_name.offset_from(dirname) as libc::c_long
                                    as libc::c_ulong,
                            ) as *mut libc::c_char;
                            if newp_1.is_null() {
                                retval = 1 as libc::c_int;
                                current_block = 4508334632585049825;
                            } else {
                                malloc_user_name = 1 as libc::c_int;
                                current_block = 2408932541243239002;
                            }
                        }
                        match current_block {
                            4508334632585049825 => {}
                            _ => {
                                if !unescape.is_null() {
                                    let mut p_2: *mut libc::c_char = mempcpy(
                                        newp_1 as *mut libc::c_void,
                                        dirname.offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        (unescape.offset_from(dirname) as libc::c_long
                                            - 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                                    ) as *mut libc::c_char;
                                    let mut q: *mut libc::c_char = unescape;
                                    while q != end_name {
                                        if *q as libc::c_int == '\\' as i32 {
                                            if q.offset(1 as libc::c_int as isize) == end_name {
                                                if filename.is_null() {
                                                    let fresh3 = p_2;
                                                    p_2 = p_2.offset(1);
                                                    *fresh3 = '\\' as i32 as libc::c_char;
                                                }
                                                break;
                                            } else {
                                                q = q.offset(1);
                                                q;
                                            }
                                        }
                                        let fresh4 = q;
                                        q = q.offset(1);
                                        let fresh5 = p_2;
                                        p_2 = p_2.offset(1);
                                        *fresh5 = *fresh4;
                                    }
                                    *p_2 = '\0' as i32 as libc::c_char;
                                } else {
                                    *(mempcpy(
                                        newp_1 as *mut libc::c_void,
                                        dirname.offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        (end_name.offset_from(dirname) as libc::c_long
                                            - 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                                    ) as *mut libc::c_char) = '\0' as i32 as libc::c_char;
                                }
                                user_name = newp_1;
                                current_block = 2920409193602730479;
                            }
                        }
                    }
                    match current_block {
                        4508334632585049825 => {}
                        _ => {
                            let mut p_3: *mut passwd = 0 as *mut passwd;
                            let mut pwtmpbuf: scratch_buffer = scratch_buffer {
                                data: 0 as *mut libc::c_void,
                                length: 0,
                                __space: C2RustUnnamed {
                                    __align: max_align_t {
                                        __clang_max_align_nonce1: 0,
                                        __clang_max_align_nonce2: f128::f128::ZERO,
                                    },
                                },
                            };
                            scratch_buffer_init(&mut pwtmpbuf);
                            let mut pwbuf_0: passwd = passwd {
                                pw_name: 0 as *mut libc::c_char,
                                pw_passwd: 0 as *mut libc::c_char,
                                pw_uid: 0,
                                pw_gid: 0,
                                pw_gecos: 0 as *mut libc::c_char,
                                pw_dir: 0 as *mut libc::c_char,
                                pw_shell: 0 as *mut libc::c_char,
                            };
                            loop {
                                if !(getpwnam_r(
                                    user_name,
                                    &mut pwbuf_0,
                                    pwtmpbuf.data as *mut libc::c_char,
                                    pwtmpbuf.length,
                                    &mut p_3,
                                ) == 34 as libc::c_int)
                                {
                                    current_block = 12717620301112128284;
                                    break;
                                }
                                if scratch_buffer_grow(&mut pwtmpbuf) {
                                    continue;
                                }
                                retval = 1 as libc::c_int;
                                current_block = 4508334632585049825;
                                break;
                            }
                            match current_block {
                                4508334632585049825 => {}
                                _ => {
                                    if malloc_user_name as libc::c_long != 0 {
                                        rpl_free(user_name as *mut libc::c_void);
                                    }
                                    if !p_3.is_null() {
                                        let mut home_len_0: size_t = strlen((*p_3).pw_dir);
                                        let mut rest_len_0: size_t = if end_name.is_null() {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            strlen(end_name)
                                        };
                                        let mut prev_dirname: *mut libc::c_char = if malloc_dirname
                                            as libc::c_long != 0
                                        {
                                            dirname
                                        } else {
                                            0 as *mut libc::c_char
                                        };
                                        let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
                                        malloc_dirname = 0 as libc::c_int;
                                        if glob_use_alloca(
                                            alloca_used,
                                            home_len_0
                                                .wrapping_add(rest_len_0)
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                        ) {
                                            dirname = 0 as *mut libc::c_void as *mut libc::c_char;
                                            current_block = 16477797002856340645;
                                        } else {
                                            dirname = malloc(
                                                home_len_0
                                                    .wrapping_add(rest_len_0)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                            ) as *mut libc::c_char;
                                            if dirname.is_null() {
                                                rpl_free(prev_dirname as *mut libc::c_void);
                                                scratch_buffer_free(&mut pwtmpbuf);
                                                retval = 1 as libc::c_int;
                                                current_block = 4508334632585049825;
                                            } else {
                                                malloc_dirname = 1 as libc::c_int;
                                                current_block = 16477797002856340645;
                                            }
                                        }
                                        match current_block {
                                            4508334632585049825 => {}
                                            _ => {
                                                d = mempcpy(
                                                    dirname as *mut libc::c_void,
                                                    (*p_3).pw_dir as *const libc::c_void,
                                                    home_len_0,
                                                ) as *mut libc::c_char;
                                                if !end_name.is_null() {
                                                    d = mempcpy(
                                                        d as *mut libc::c_void,
                                                        end_name as *const libc::c_void,
                                                        rest_len_0,
                                                    ) as *mut libc::c_char;
                                                }
                                                *d = '\0' as i32 as libc::c_char;
                                                rpl_free(prev_dirname as *mut libc::c_void);
                                                dirlen = home_len_0.wrapping_add(rest_len_0);
                                                dirname_modified = 1 as libc::c_int;
                                                current_block = 11272946706888692785;
                                            }
                                        }
                                    } else if flags & (1 as libc::c_int) << 14 as libc::c_int
                                        != 0
                                    {
                                        retval = 3 as libc::c_int;
                                        current_block = 4508334632585049825;
                                    } else {
                                        current_block = 11272946706888692785;
                                    }
                                    match current_block {
                                        4508334632585049825 => {}
                                        _ => {
                                            scratch_buffer_free(&mut pwtmpbuf);
                                            current_block = 7188795011561844502;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                current_block = 7188795011561844502;
            }
            match current_block {
                4508334632585049825 => {}
                _ => {
                    if filename.is_null() {
                        let mut newcount: size_t = ((*pglob).gl_pathc)
                            .wrapping_add((*pglob).gl_offs);
                        let mut new_gl_pathv: *mut *mut libc::c_char = 0
                            as *mut *mut libc::c_char;
                        if !(newcount
                            > (18446744073709551615 as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                                )
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        {
                            new_gl_pathv = realloc(
                                (*pglob).gl_pathv as *mut libc::c_void,
                                newcount
                                    .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                                    ),
                            ) as *mut *mut libc::c_char;
                            if !new_gl_pathv.is_null() {
                                (*pglob).gl_pathv = new_gl_pathv;
                                if flags & (1 as libc::c_int) << 1 as libc::c_int != 0
                                    && is_dir(dirname, flags, pglob) as libc::c_int != 0
                                {
                                    let mut p_4: *mut libc::c_char = 0 as *mut libc::c_char;
                                    let ref mut fresh6 = *((*pglob).gl_pathv)
                                        .offset(newcount as isize);
                                    *fresh6 = malloc(
                                        dirlen.wrapping_add(2 as libc::c_int as libc::c_ulong),
                                    ) as *mut libc::c_char;
                                    if (*((*pglob).gl_pathv).offset(newcount as isize))
                                        .is_null()
                                    {
                                        current_block = 3137057002619653770;
                                    } else {
                                        p_4 = mempcpy(
                                            *((*pglob).gl_pathv).offset(newcount as isize)
                                                as *mut libc::c_void,
                                            dirname as *const libc::c_void,
                                            dirlen,
                                        ) as *mut libc::c_char;
                                        *p_4
                                            .offset(
                                                0 as libc::c_int as isize,
                                            ) = '/' as i32 as libc::c_char;
                                        *p_4
                                            .offset(
                                                1 as libc::c_int as isize,
                                            ) = '\0' as i32 as libc::c_char;
                                        if malloc_dirname as libc::c_long != 0 {
                                            rpl_free(dirname as *mut libc::c_void);
                                        }
                                        current_block = 8148047698568473920;
                                    }
                                } else if malloc_dirname as libc::c_long != 0 {
                                    let ref mut fresh7 = *((*pglob).gl_pathv)
                                        .offset(newcount as isize);
                                    *fresh7 = dirname;
                                    current_block = 8148047698568473920;
                                } else {
                                    let ref mut fresh8 = *((*pglob).gl_pathv)
                                        .offset(newcount as isize);
                                    *fresh8 = strdup(dirname);
                                    if (*((*pglob).gl_pathv).offset(newcount as isize))
                                        .is_null()
                                    {
                                        current_block = 3137057002619653770;
                                    } else {
                                        current_block = 8148047698568473920;
                                    }
                                }
                                match current_block {
                                    3137057002619653770 => {}
                                    _ => {
                                        newcount = newcount.wrapping_add(1);
                                        let ref mut fresh9 = *((*pglob).gl_pathv)
                                            .offset(newcount as isize);
                                        *fresh9 = 0 as *mut libc::c_char;
                                        (*pglob).gl_pathc = ((*pglob).gl_pathc).wrapping_add(1);
                                        (*pglob).gl_pathc;
                                        (*pglob).gl_flags = flags;
                                        return 0 as libc::c_int;
                                    }
                                }
                            }
                        }
                        rpl_free((*pglob).gl_pathv as *mut libc::c_void);
                        (*pglob).gl_pathv = 0 as *mut *mut libc::c_char;
                        (*pglob).gl_pathc = 0 as libc::c_int as size_t;
                        retval = 1 as libc::c_int;
                        current_block = 4508334632585049825;
                    } else {
                        meta = __glob_pattern_type(
                            dirname,
                            (flags & (1 as libc::c_int) << 6 as libc::c_int == 0)
                                as libc::c_int,
                        );
                        if meta
                            & (GLOBPAT_SPECIAL as libc::c_int
                                | GLOBPAT_BRACKET as libc::c_int) != 0
                        {
                            i_0 = 0;
                            if flags & (1 as libc::c_int) << 6 as libc::c_int == 0
                                && dirlen > 0 as libc::c_int as libc::c_ulong
                                && *dirname
                                    .offset(
                                        dirlen.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\\' as i32
                            {
                                let mut p_5: *mut libc::c_char = &mut *dirname
                                    .offset(
                                        dirlen.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as *mut libc::c_char;
                                while p_5 > dirname
                                    && *p_5.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                        == '\\' as i32
                                {
                                    p_5 = p_5.offset(-1);
                                    p_5;
                                }
                                if (&mut *dirname.offset(dirlen as isize)
                                    as *mut libc::c_char)
                                    .offset_from(p_5) as libc::c_long
                                    & 1 as libc::c_int as libc::c_long != 0
                                {
                                    dirlen = dirlen.wrapping_sub(1);
                                    *(&mut *dirname.offset(dirlen as isize)
                                        as *mut libc::c_char) = '\0' as i32 as libc::c_char;
                                }
                            }
                            if (flags & (1 as libc::c_int) << 9 as libc::c_int
                                != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
                            {
                                dirs.gl_opendir = (*pglob).gl_opendir;
                                dirs.gl_readdir = (*pglob).gl_readdir;
                                dirs.gl_closedir = (*pglob).gl_closedir;
                                dirs.gl_stat = (*pglob).gl_stat;
                                dirs.gl_lstat = (*pglob).gl_lstat;
                            }
                            status = rpl_glob(
                                dirname,
                                flags
                                    & ((1 as libc::c_int) << 0 as libc::c_int
                                        | (1 as libc::c_int) << 6 as libc::c_int
                                        | (1 as libc::c_int) << 9 as libc::c_int)
                                    | (1 as libc::c_int) << 2 as libc::c_int
                                    | (1 as libc::c_int) << 13 as libc::c_int,
                                errfunc,
                                &mut dirs,
                            );
                            if status != 0 as libc::c_int {
                                if flags & (1 as libc::c_int) << 4 as libc::c_int
                                    == 0 as libc::c_int || status != 3 as libc::c_int
                                {
                                    retval = status;
                                    current_block = 4508334632585049825;
                                } else {
                                    current_block = 12238912827727375958;
                                }
                            } else {
                                i_0 = 0 as libc::c_int as size_t;
                                loop {
                                    if !(i_0 < dirs.gl_pathc) {
                                        current_block = 5541483847769576901;
                                        break;
                                    }
                                    let mut old_pathc: size_t = 0;
                                    old_pathc = (*pglob).gl_pathc;
                                    status = glob_in_dir(
                                        filename,
                                        *(dirs.gl_pathv).offset(i_0 as isize),
                                        (flags | (1 as libc::c_int) << 5 as libc::c_int)
                                            & !((1 as libc::c_int) << 4 as libc::c_int
                                                | (1 as libc::c_int) << 11 as libc::c_int),
                                        errfunc,
                                        pglob,
                                        alloca_used,
                                    );
                                    if !(status == 3 as libc::c_int) {
                                        if status != 0 as libc::c_int {
                                            rpl_globfree(&mut dirs);
                                            rpl_globfree(pglob);
                                            (*pglob).gl_pathc = 0 as libc::c_int as size_t;
                                            retval = status;
                                            current_block = 4508334632585049825;
                                            break;
                                        } else if prefix_array(
                                            *(dirs.gl_pathv).offset(i_0 as isize),
                                            &mut *((*pglob).gl_pathv)
                                                .offset(old_pathc.wrapping_add((*pglob).gl_offs) as isize),
                                            ((*pglob).gl_pathc).wrapping_sub(old_pathc),
                                        ) != 0
                                        {
                                            rpl_globfree(&mut dirs);
                                            rpl_globfree(pglob);
                                            (*pglob).gl_pathc = 0 as libc::c_int as size_t;
                                            retval = 1 as libc::c_int;
                                            current_block = 4508334632585049825;
                                            break;
                                        }
                                    }
                                    i_0 = i_0.wrapping_add(1);
                                    i_0;
                                }
                                match current_block {
                                    4508334632585049825 => {}
                                    _ => {
                                        flags |= (1 as libc::c_int) << 8 as libc::c_int;
                                        if ((*pglob).gl_pathc).wrapping_add((*pglob).gl_offs)
                                            == oldcount
                                        {
                                            current_block = 12238912827727375958;
                                        } else {
                                            current_block = 11353886201549099807;
                                        }
                                    }
                                }
                            }
                        } else {
                            let mut old_pathc_0: size_t = (*pglob).gl_pathc;
                            let mut orig_flags_0: libc::c_int = flags;
                            if meta & GLOBPAT_BACKSLASH as libc::c_int != 0 {
                                let mut p_6: *mut libc::c_char = strchr(
                                    dirname,
                                    '\\' as i32,
                                );
                                let mut q_0: *mut libc::c_char = 0 as *mut libc::c_char;
                                q_0 = p_6;
                                loop {
                                    if *p_6 as libc::c_int == '\\' as i32 {
                                        p_6 = p_6.offset(1);
                                        *q_0 = *p_6;
                                        dirlen = dirlen.wrapping_sub(1);
                                        dirlen;
                                    } else {
                                        *q_0 = *p_6;
                                    }
                                    q_0 = q_0.offset(1);
                                    q_0;
                                    let fresh12 = p_6;
                                    p_6 = p_6.offset(1);
                                    if !(*fresh12 as libc::c_int != '\0' as i32) {
                                        break;
                                    }
                                }
                                dirname_modified = 1 as libc::c_int;
                            }
                            if dirname_modified != 0 {
                                flags
                                    &= !((1 as libc::c_int) << 4 as libc::c_int
                                        | (1 as libc::c_int) << 11 as libc::c_int);
                            }
                            status = glob_in_dir(
                                filename,
                                dirname,
                                flags,
                                errfunc,
                                pglob,
                                alloca_used,
                            );
                            if status != 0 as libc::c_int {
                                if status == 3 as libc::c_int && flags != orig_flags_0
                                    && ((*pglob).gl_pathc).wrapping_add((*pglob).gl_offs)
                                        == oldcount
                                {
                                    dirs.gl_pathv = 0 as *mut *mut libc::c_char;
                                    flags = orig_flags_0;
                                    current_block = 12238912827727375958;
                                } else {
                                    retval = status;
                                    current_block = 4508334632585049825;
                                }
                            } else if dirlen > 0 as libc::c_int as libc::c_ulong {
                                if prefix_array(
                                    dirname,
                                    &mut *((*pglob).gl_pathv)
                                        .offset(
                                            old_pathc_0.wrapping_add((*pglob).gl_offs) as isize,
                                        ),
                                    ((*pglob).gl_pathc).wrapping_sub(old_pathc_0),
                                ) != 0
                                {
                                    rpl_globfree(pglob);
                                    (*pglob).gl_pathc = 0 as libc::c_int as size_t;
                                    retval = 1 as libc::c_int;
                                    current_block = 4508334632585049825;
                                } else {
                                    current_block = 10421005448780431124;
                                }
                            } else {
                                current_block = 10421005448780431124;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        12238912827727375958 => {
            if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
                let mut newcount_0: size_t = ((*pglob).gl_pathc)
                    .wrapping_add((*pglob).gl_offs);
                let mut new_gl_pathv_0: *mut *mut libc::c_char = 0
                    as *mut *mut libc::c_char;
                if newcount_0
                    > (18446744073709551615 as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        )
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                {
                    current_block = 4493917863199400118;
                } else {
                    new_gl_pathv_0 = realloc(
                        (*pglob).gl_pathv as *mut libc::c_void,
                        newcount_0
                            .wrapping_add(2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                            ),
                    ) as *mut *mut libc::c_char;
                    if new_gl_pathv_0.is_null() {
                        current_block = 4493917863199400118;
                    } else {
                        (*pglob).gl_pathv = new_gl_pathv_0;
                        let ref mut fresh10 = *((*pglob).gl_pathv)
                            .offset(newcount_0 as isize);
                        *fresh10 = strdup(pattern);
                        if (*((*pglob).gl_pathv).offset(newcount_0 as isize)).is_null() {
                            rpl_globfree(&mut dirs);
                            rpl_globfree(pglob);
                            (*pglob).gl_pathc = 0 as libc::c_int as size_t;
                            retval = 1 as libc::c_int;
                            current_block = 4508334632585049825;
                        } else {
                            (*pglob).gl_pathc = ((*pglob).gl_pathc).wrapping_add(1);
                            (*pglob).gl_pathc;
                            newcount_0 = newcount_0.wrapping_add(1);
                            newcount_0;
                            let ref mut fresh11 = *((*pglob).gl_pathv)
                                .offset(newcount_0 as isize);
                            *fresh11 = 0 as *mut libc::c_char;
                            (*pglob).gl_flags = flags;
                            current_block = 11353886201549099807;
                        }
                    }
                }
                match current_block {
                    11353886201549099807 => {}
                    4508334632585049825 => {}
                    _ => {
                        rpl_globfree(&mut dirs);
                        retval = 1 as libc::c_int;
                        current_block = 4508334632585049825;
                    }
                }
            } else {
                rpl_globfree(&mut dirs);
                retval = 3 as libc::c_int;
                current_block = 4508334632585049825;
            }
        }
        _ => {}
    }
    match current_block {
        11353886201549099807 => {
            rpl_globfree(&mut dirs);
            current_block = 10421005448780431124;
        }
        _ => {}
    }
    match current_block {
        10421005448780431124 => {
            if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
                let mut i_1: size_t = 0;
                i_1 = oldcount;
                loop {
                    if !(i_1 < ((*pglob).gl_pathc).wrapping_add((*pglob).gl_offs)) {
                        current_block = 17332235711799416669;
                        break;
                    }
                    if is_dir(*((*pglob).gl_pathv).offset(i_1 as isize), flags, pglob) {
                        let mut len: size_t = (strlen(
                            *((*pglob).gl_pathv).offset(i_1 as isize),
                        ))
                            .wrapping_add(2 as libc::c_int as libc::c_ulong);
                        let mut new: *mut libc::c_char = realloc(
                            *((*pglob).gl_pathv).offset(i_1 as isize)
                                as *mut libc::c_void,
                            len,
                        ) as *mut libc::c_char;
                        if new.is_null() {
                            rpl_globfree(pglob);
                            (*pglob).gl_pathc = 0 as libc::c_int as size_t;
                            retval = 1 as libc::c_int;
                            current_block = 4508334632585049825;
                            break;
                        } else {
                            strcpy(
                                &mut *new
                                    .offset(
                                        len.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
                                    ),
                                b"/\0" as *const u8 as *const libc::c_char,
                            );
                            let ref mut fresh13 = *((*pglob).gl_pathv)
                                .offset(i_1 as isize);
                            *fresh13 = new;
                        }
                    }
                    i_1 = i_1.wrapping_add(1);
                    i_1;
                }
            } else {
                current_block = 17332235711799416669;
            }
            match current_block {
                4508334632585049825 => {}
                _ => {
                    if flags & (1 as libc::c_int) << 2 as libc::c_int == 0 {
                        qsort(
                            &mut *((*pglob).gl_pathv).offset(oldcount as isize)
                                as *mut *mut libc::c_char as *mut libc::c_void,
                            ((*pglob).gl_pathc)
                                .wrapping_add((*pglob).gl_offs)
                                .wrapping_sub(oldcount),
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                            Some(
                                collated_compare
                                    as unsafe extern "C" fn(
                                        *const libc::c_void,
                                        *const libc::c_void,
                                    ) -> libc::c_int,
                            ),
                        );
                    }
                }
            }
        }
        _ => {}
    }
    if malloc_dirname as libc::c_long != 0 {
        rpl_free(dirname as *mut libc::c_void);
    }
    return retval;
}
unsafe extern "C" fn collated_compare(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut ps1: *const *mut libc::c_char = a as *const *mut libc::c_char;
    let mut s1: *mut libc::c_char = *ps1;
    let mut ps2: *const *mut libc::c_char = b as *const *mut libc::c_char;
    let mut s2: *mut libc::c_char = *ps2;
    if s1 == s2 {
        return 0 as libc::c_int;
    }
    if s1.is_null() {
        return 1 as libc::c_int;
    }
    if s2.is_null() {
        return -(1 as libc::c_int);
    }
    return strcoll(s1, s2);
}
unsafe extern "C" fn prefix_array(
    mut dirname: *const libc::c_char,
    mut array: *mut *mut libc::c_char,
    mut n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut dirlen: size_t = strlen(dirname);
    let mut dirsep_char: libc::c_char = '/' as i32 as libc::c_char;
    if dirlen == 1 as libc::c_int as libc::c_ulong
        && *dirname.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        dirlen = 0 as libc::c_int as size_t;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut eltlen: size_t = (strlen(*array.offset(i as isize)))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        let mut new: *mut libc::c_char = malloc(
            dirlen.wrapping_add(1 as libc::c_int as libc::c_ulong).wrapping_add(eltlen),
        ) as *mut libc::c_char;
        if new.is_null() {
            while i > 0 as libc::c_int as libc::c_ulong {
                i = i.wrapping_sub(1);
                rpl_free(*array.offset(i as isize) as *mut libc::c_void);
            }
            return 1 as libc::c_int;
        }
        let mut endp: *mut libc::c_char = mempcpy(
            new as *mut libc::c_void,
            dirname as *const libc::c_void,
            dirlen,
        ) as *mut libc::c_char;
        let fresh14 = endp;
        endp = endp.offset(1);
        *fresh14 = dirsep_char;
        mempcpy(
            endp as *mut libc::c_void,
            *array.offset(i as isize) as *const libc::c_void,
            eltlen,
        );
        rpl_free(*array.offset(i as isize) as *mut libc::c_void);
        let ref mut fresh15 = *array.offset(i as isize);
        *fresh15 = new;
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn glob_in_dir(
    mut pattern: *const libc::c_char,
    mut directory: *const libc::c_char,
    mut flags: libc::c_int,
    mut errfunc: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int,
    >,
    mut pglob: *mut glob_t,
    mut alloca_used: size_t,
) -> libc::c_int {
    let mut new_gl_pathv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut current_block: u64;
    let mut dirlen: size_t = strlen(directory);
    let mut stream: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut s: scratch_buffer = scratch_buffer {
        data: 0 as *mut libc::c_void,
        length: 0,
        __space: C2RustUnnamed {
            __align: max_align_t {
                __clang_max_align_nonce1: 0,
                __clang_max_align_nonce2: f128::f128::ZERO,
            },
        },
    };
    scratch_buffer_init(&mut s);
    let mut init_names_buf: C2RustUnnamed_0 = C2RustUnnamed_0 {
        next: 0 as *mut globnames,
        count: 0,
        name: [0 as *mut libc::c_char; 64],
    };
    let mut init_names: *mut globnames = &mut init_names_buf as *mut C2RustUnnamed_0
        as *mut globnames;
    let mut names: *mut globnames = init_names;
    let mut names_alloca: *mut globnames = init_names;
    let mut nfound: size_t = 0 as libc::c_int as size_t;
    let mut cur: size_t = 0 as libc::c_int as size_t;
    let mut meta: libc::c_int = 0;
    let mut save: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    alloca_used = (alloca_used as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong)
        as size_t as size_t;
    (*init_names).next = 0 as *mut globnames;
    (*init_names)
        .count = (::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong)
        .wrapping_sub(16 as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong);
    meta = __glob_pattern_type(
        pattern,
        (flags & (1 as libc::c_int) << 6 as libc::c_int == 0) as libc::c_int,
    );
    if meta == GLOBPAT_NONE as libc::c_int
        && flags
            & ((1 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 11 as libc::c_int) != 0
    {
        flags |= (1 as libc::c_int) << 4 as libc::c_int;
        current_block = 15594603006322722090;
    } else if meta == GLOBPAT_NONE as libc::c_int {
        let mut patlen: size_t = strlen(pattern);
        let mut fullsize: size_t = 0;
        let mut alloca_fullname: bool = !size_add_wrapv(
            dirlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            patlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            &mut fullsize,
        ) && glob_use_alloca(alloca_used, fullsize) as libc::c_int != 0;
        let mut fullname: *mut libc::c_char = 0 as *mut libc::c_char;
        if alloca_fullname {
            fullname = 0 as *mut libc::c_void as *mut libc::c_char;
        } else {
            fullname = malloc(fullsize) as *mut libc::c_char;
            if fullname.is_null() {
                return 1 as libc::c_int;
            }
        }
        mempcpy(
            mempcpy(
                mempcpy(
                    fullname as *mut libc::c_void,
                    directory as *const libc::c_void,
                    dirlen,
                ),
                b"/\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
            ),
            pattern as *const libc::c_void,
            patlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        if glob_lstat(pglob, flags, fullname) == 0 as libc::c_int
            || *__errno_location() == 75 as libc::c_int
        {
            flags |= (1 as libc::c_int) << 4 as libc::c_int;
        }
        if !alloca_fullname as libc::c_int as libc::c_long != 0 {
            rpl_free(fullname as *mut libc::c_void);
        }
        current_block = 15594603006322722090;
    } else {
        stream = if (flags & (1 as libc::c_int) << 9 as libc::c_int) as libc::c_long != 0
        {
            (Some(((*pglob).gl_opendir).unwrap())).unwrap()(directory)
        } else {
            opendir(directory) as *mut libc::c_void
        };
        if stream.is_null() {
            if *__errno_location() != 20 as libc::c_int
                && (errfunc.is_some()
                    && (Some(errfunc.unwrap())).unwrap()(directory, *__errno_location())
                        != 0 || flags & (1 as libc::c_int) << 0 as libc::c_int != 0)
            {
                return 2 as libc::c_int;
            }
            current_block = 15594603006322722090;
        } else {
            let mut dirp: *mut DIR = stream as *mut DIR;
            let mut dfd: libc::c_int = dirfd(dirp);
            let mut fnm_flags: libc::c_int = (if flags
                & (1 as libc::c_int) << 7 as libc::c_int == 0
            {
                (1 as libc::c_int) << 2 as libc::c_int
            } else {
                0 as libc::c_int
            })
                | (if flags & (1 as libc::c_int) << 6 as libc::c_int != 0 {
                    (1 as libc::c_int) << 1 as libc::c_int
                } else {
                    0 as libc::c_int
                });
            flags |= (1 as libc::c_int) << 8 as libc::c_int;
            loop {
                let mut d: readdir_result = readdir_result {
                    name: 0 as *const libc::c_char,
                    type_0: 0,
                };
                if (flags & (1 as libc::c_int) << 9 as libc::c_int) as libc::c_long != 0
                {
                    d = convert_dirent(((*pglob).gl_readdir).unwrap()(stream));
                } else {
                    d = convert_dirent(readdir(stream as *mut DIR));
                }
                if (d.name).is_null() {
                    current_block = 15594603006322722090;
                    break;
                }
                if flags & (1 as libc::c_int) << 13 as libc::c_int != 0 {
                    match readdir_result_type(d) {
                        4 => {}
                        10 | 0 => {
                            if (dfd < 0 as libc::c_int
                                || flags & (1 as libc::c_int) << 9 as libc::c_int != 0)
                                as libc::c_int as libc::c_long != 0
                            {
                                let mut namelen: size_t = strlen(d.name);
                                let mut need: size_t = dirlen
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(namelen)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                                if s.length < need
                                    && !scratch_buffer_set_array_size(
                                        &mut s,
                                        need,
                                        1 as libc::c_int as size_t,
                                    )
                                {
                                    current_block = 12626817352307268239;
                                    break;
                                }
                                let mut p: *mut libc::c_char = mempcpy(
                                    s.data,
                                    directory as *const libc::c_void,
                                    dirlen,
                                ) as *mut libc::c_char;
                                *p = '/' as i32 as libc::c_char;
                                p = p
                                    .offset(
                                        (*p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                            != '/' as i32) as libc::c_int as isize,
                                    );
                                memcpy(
                                    p as *mut libc::c_void,
                                    d.name as *const libc::c_void,
                                    namelen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                );
                                if !is_dir(s.data as *const libc::c_char, flags, pglob) {
                                    continue;
                                }
                            } else {
                                let mut st64: stat = stat {
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
                                if !(fstatat(dfd, d.name, &mut st64, 0 as libc::c_int)
                                    == 0 as libc::c_int
                                    && st64.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                        == 0o40000 as libc::c_int as libc::c_uint)
                                {
                                    continue;
                                }
                            }
                        }
                        _ => {
                            continue;
                        }
                    }
                }
                if !(fnmatch(pattern, d.name, fnm_flags) == 0 as libc::c_int) {
                    continue;
                }
                if cur == (*names).count {
                    let mut newnames: *mut globnames = 0 as *mut globnames;
                    let mut count: size_t = ((*names).count)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong);
                    let mut nameoff: size_t = 16 as libc::c_ulong;
                    let mut size: size_t = (16 as libc::c_ulong)
                        .wrapping_add(
                            ::std::mem::align_of::<globnames>() as libc::c_ulong,
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            count
                                .wrapping_mul(
                                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                                ),
                        )
                        & !(::std::mem::align_of::<globnames>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    if (18446744073709551615 as libc::c_ulong)
                        .wrapping_sub(nameoff)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ) < (*names).count
                    {
                        current_block = 12626817352307268239;
                        break;
                    }
                    if glob_use_alloca(alloca_used, size) {
                        names_alloca = 0 as *mut libc::c_void as *mut globnames;
                        newnames = names_alloca;
                    } else {
                        newnames = malloc(size) as *mut globnames;
                        if newnames.is_null() {
                            current_block = 12626817352307268239;
                            break;
                        }
                    }
                    (*newnames).count = count;
                    (*newnames).next = names;
                    names = newnames;
                    cur = 0 as libc::c_int as size_t;
                }
                let ref mut fresh16 = *((*names).name).as_mut_ptr().offset(cur as isize);
                *fresh16 = strdup(d.name);
                if (*((*names).name).as_mut_ptr().offset(cur as isize)).is_null() {
                    current_block = 12626817352307268239;
                    break;
                }
                cur = cur.wrapping_add(1);
                cur;
                nfound = nfound.wrapping_add(1);
                nfound;
                if (18446744073709551615 as libc::c_ulong).wrapping_sub((*pglob).gl_offs)
                    <= nfound
                {
                    current_block = 12626817352307268239;
                    break;
                }
            }
        }
    }
    match current_block {
        15594603006322722090 => {
            if nfound == 0 as libc::c_int as libc::c_ulong
                && flags & (1 as libc::c_int) << 4 as libc::c_int != 0
            {
                let mut len: size_t = strlen(pattern);
                nfound = 1 as libc::c_int as size_t;
                let ref mut fresh17 = *((*names).name).as_mut_ptr().offset(cur as isize);
                *fresh17 = malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as *mut libc::c_char;
                if (*((*names).name).as_mut_ptr().offset(cur as isize)).is_null() {
                    current_block = 12626817352307268239;
                } else {
                    let fresh18 = cur;
                    cur = cur.wrapping_add(1);
                    *(mempcpy(
                        *((*names).name).as_mut_ptr().offset(fresh18 as isize)
                            as *mut libc::c_void,
                        pattern as *const libc::c_void,
                        len,
                    ) as *mut libc::c_char) = '\0' as i32 as libc::c_char;
                    current_block = 13660591889533726445;
                }
            } else {
                current_block = 13660591889533726445;
            }
            match current_block {
                12626817352307268239 => {}
                _ => {
                    result = 3 as libc::c_int;
                    if nfound != 0 as libc::c_int as libc::c_ulong {
                        new_gl_pathv = 0 as *mut *mut libc::c_char;
                        result = 0 as libc::c_int;
                        if (18446744073709551615 as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                            )
                            .wrapping_sub((*pglob).gl_pathc)
                            < ((*pglob).gl_offs)
                                .wrapping_add(nfound)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        {
                            current_block = 12626817352307268239;
                        } else {
                            new_gl_pathv = realloc(
                                (*pglob).gl_pathv as *mut libc::c_void,
                                ((*pglob).gl_pathc)
                                    .wrapping_add((*pglob).gl_offs)
                                    .wrapping_add(nfound)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                                    ),
                            ) as *mut *mut libc::c_char;
                            if new_gl_pathv.is_null() {
                                current_block = 12626817352307268239;
                            } else {
                                loop {
                                    let mut old_0: *mut globnames = names;
                                    let mut i_0: size_t = 0 as libc::c_int as size_t;
                                    while i_0 < cur {
                                        let fresh19 = (*pglob).gl_pathc;
                                        (*pglob).gl_pathc = ((*pglob).gl_pathc).wrapping_add(1);
                                        let ref mut fresh20 = *new_gl_pathv
                                            .offset(((*pglob).gl_offs).wrapping_add(fresh19) as isize);
                                        *fresh20 = *((*names).name)
                                            .as_mut_ptr()
                                            .offset(i_0 as isize);
                                        i_0 = i_0.wrapping_add(1);
                                        i_0;
                                    }
                                    names = (*names).next;
                                    if names.is_null() {
                                        break;
                                    }
                                    cur = (*names).count;
                                    if old_0 == names_alloca {
                                        names_alloca = names;
                                    } else {
                                        rpl_free(old_0 as *mut libc::c_void);
                                    }
                                }
                                (*pglob).gl_pathv = new_gl_pathv;
                                let ref mut fresh21 = *((*pglob).gl_pathv)
                                    .offset(
                                        ((*pglob).gl_offs).wrapping_add((*pglob).gl_pathc) as isize,
                                    );
                                *fresh21 = 0 as *mut libc::c_char;
                                (*pglob).gl_flags = flags;
                                current_block = 14865402277128115059;
                            }
                        }
                    } else {
                        current_block = 14865402277128115059;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        12626817352307268239 => {
            loop {
                let mut old: *mut globnames = names;
                let mut i: size_t = 0 as libc::c_int as size_t;
                while i < cur {
                    rpl_free(
                        *((*names).name).as_mut_ptr().offset(i as isize)
                            as *mut libc::c_void,
                    );
                    i = i.wrapping_add(1);
                    i;
                }
                names = (*names).next;
                if names.is_null() {
                    break;
                }
                cur = (*names).count;
                if old == names_alloca {
                    names_alloca = names;
                } else {
                    rpl_free(old as *mut libc::c_void);
                }
            }
            result = 1 as libc::c_int;
        }
        _ => {}
    }
    if !stream.is_null() {
        save = *__errno_location();
        if (flags & (1 as libc::c_int) << 9 as libc::c_int) as libc::c_long != 0 {
            (Some(((*pglob).gl_closedir).unwrap())).unwrap()(stream);
        } else {
            closedir(stream as *mut DIR);
        }
        *__errno_location() = save;
    }
    scratch_buffer_free(&mut s);
    return result;
}
