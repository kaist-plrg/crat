use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type real_pcre;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn vasprintf(
        __ptr: *mut *mut libc::c_char,
        __f: *const libc::c_char,
        __arg: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn __uflow(_: *mut FILE) -> libc::c_int;
    fn flockfile(__stream: *mut FILE);
    fn funlockfile(__stream: *mut FILE);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn pcre_compile(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut *const libc::c_char,
        _: *mut libc::c_int,
        _: *const libc::c_uchar,
    ) -> *mut pcre;
    fn pcre_study(
        _: *const pcre,
        _: libc::c_int,
        _: *mut *const libc::c_char,
    ) -> *mut pcre_extra;
    fn vplog(level: libc::c_uint, fmt: *const libc::c_char, args: ::std::ffi::VaList);
    fn log_debug(fmt: *const libc::c_char, _: ...);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const DT_WHT: C2RustUnnamed_0 = 14;
pub const DT_SOCK: C2RustUnnamed_0 = 12;
pub const DT_LNK: C2RustUnnamed_0 = 10;
pub const DT_REG: C2RustUnnamed_0 = 8;
pub const DT_BLK: C2RustUnnamed_0 = 6;
pub const DT_DIR: C2RustUnnamed_0 = 4;
pub const DT_CHR: C2RustUnnamed_0 = 2;
pub const DT_FIFO: C2RustUnnamed_0 = 1;
pub const DT_UNKNOWN: C2RustUnnamed_0 = 0;
pub type pcre = real_pcre;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcre_extra {
    pub flags: libc::c_ulong,
    pub study_data: *mut libc::c_void,
    pub match_limit: libc::c_ulong,
    pub callout_data: *mut libc::c_void,
    pub tables: *const libc::c_uchar,
    pub match_limit_recursion: libc::c_ulong,
    pub mark: *mut *mut libc::c_uchar,
    pub executable_jit: *mut libc::c_void,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type log_level = libc::c_uint;
pub const LOG_LEVEL_NONE: log_level = 100;
pub const LOG_LEVEL_ERR: log_level = 40;
pub const LOG_LEVEL_WARN: log_level = 30;
pub const LOG_LEVEL_MSG: log_level = 20;
pub const LOG_LEVEL_DEBUG: log_level = 10;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct match_t {
    pub start: size_t,
    pub end: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ag_stats {
    pub total_bytes: size_t,
    pub total_files: size_t,
    pub total_matches: size_t,
    pub total_file_matches: size_t,
    pub time_start: timeval,
    pub time_end: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union word_t {
    pub as_chars: [libc::c_char; 2],
    pub as_word: uint16_t,
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int
        as libc::c_long != 0
    {
        __uflow(__fp)
    } else {
        let fresh0 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = ((*__fp)._IO_read_ptr).offset(1);
        *(fresh0 as *mut libc::c_uchar) as libc::c_int
    };
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
pub static mut out_fd: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut stats: ag_stats = ag_stats {
    total_bytes: 0,
    total_files: 0,
    total_matches: 0,
    total_file_matches: 0,
    time_start: timeval { tv_sec: 0, tv_usec: 0 },
    time_end: timeval { tv_sec: 0, tv_usec: 0 },
};
pub unsafe extern "C" fn ag_malloc(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = malloc(size);
    if ptr.is_null() {
        die(b"Memory allocation failed.\0" as *const u8 as *const libc::c_char);
    }
    return ptr;
}
pub unsafe extern "C" fn ag_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut new_ptr: *mut libc::c_void = realloc(ptr, size);
    if new_ptr.is_null() {
        die(b"Memory allocation failed.\0" as *const u8 as *const libc::c_char);
    }
    return new_ptr;
}
pub unsafe extern "C" fn ag_calloc(
    mut count: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = calloc(count, size);
    if ptr.is_null() {
        die(b"Memory allocation failed.\0" as *const u8 as *const libc::c_char);
    }
    return ptr;
}
pub unsafe extern "C" fn ag_strdup(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut str: *mut libc::c_char = strdup(s);
    if str.is_null() {
        die(b"Memory allocation failed.\0" as *const u8 as *const libc::c_char);
    }
    return str;
}
pub unsafe extern "C" fn ag_strndup(
    mut s: *const libc::c_char,
    mut size: size_t,
) -> *mut libc::c_char {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    str = strndup(s, size);
    if str.is_null() {
        die(b"Memory allocation failed.\0" as *const u8 as *const libc::c_char);
    }
    return str;
}
pub unsafe extern "C" fn free_strings(
    mut strs: *mut *mut libc::c_char,
    strs_len: size_t,
) {
    if strs.is_null() {
        return;
    }
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < strs_len {
        free(*strs.offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1);
        i;
    }
    free(strs as *mut libc::c_void);
}
pub unsafe extern "C" fn generate_alpha_skip(
    mut find: *const libc::c_char,
    mut f_len: size_t,
    mut skip_lookup: *mut size_t,
    case_sensitive: libc::c_int,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < 256 as libc::c_int as libc::c_ulong {
        *skip_lookup.offset(i as isize) = f_len;
        i = i.wrapping_add(1);
        i;
    }
    f_len = f_len.wrapping_sub(1);
    f_len;
    i = 0 as libc::c_int as size_t;
    while i < f_len {
        if case_sensitive != 0 {
            *skip_lookup
                .offset(
                    *find.offset(i as isize) as libc::c_uchar as isize,
                ) = f_len.wrapping_sub(i);
        } else {
            *skip_lookup
                .offset(
                    ({
                        let mut __res: libc::c_int = 0;
                        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int = *find.offset(i as isize)
                                    as libc::c_int;
                                __res = if __c < -(128 as libc::c_int)
                                    || __c > 255 as libc::c_int
                                {
                                    __c
                                } else {
                                    *(*__ctype_tolower_loc()).offset(__c as isize)
                                };
                            } else {
                                __res = tolower(*find.offset(i as isize) as libc::c_int);
                            }
                        } else {
                            __res = *(*__ctype_tolower_loc())
                                .offset(*find.offset(i as isize) as libc::c_int as isize);
                        }
                        __res
                    }) as libc::c_uchar as isize,
                ) = f_len.wrapping_sub(i);
            *skip_lookup
                .offset(
                    ({
                        let mut __res: libc::c_int = 0;
                        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int = *find.offset(i as isize)
                                    as libc::c_int;
                                __res = if __c < -(128 as libc::c_int)
                                    || __c > 255 as libc::c_int
                                {
                                    __c
                                } else {
                                    *(*__ctype_toupper_loc()).offset(__c as isize)
                                };
                            } else {
                                __res = toupper(*find.offset(i as isize) as libc::c_int);
                            }
                        } else {
                            __res = *(*__ctype_toupper_loc())
                                .offset(*find.offset(i as isize) as libc::c_int as isize);
                        }
                        __res
                    }) as libc::c_uchar as isize,
                ) = f_len.wrapping_sub(i);
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn is_prefix(
    mut s: *const libc::c_char,
    s_len: size_t,
    pos: size_t,
    case_sensitive: libc::c_int,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while pos.wrapping_add(i) < s_len {
        if case_sensitive != 0 {
            if *s.offset(i as isize) as libc::c_int
                != *s.offset(i.wrapping_add(pos) as isize) as libc::c_int
            {
                return 0 as libc::c_int;
            }
        } else if ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *s.offset(i as isize) as libc::c_int;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    });
                } else {
                    __res = tolower(*s.offset(i as isize) as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(*s.offset(i as isize) as libc::c_int as isize);
            }
            __res
        })
            != ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *s
                            .offset(i.wrapping_add(pos) as isize) as libc::c_int;
                        __res = (if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = tolower(
                            *s.offset(i.wrapping_add(pos) as isize) as libc::c_int,
                        );
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(
                            *s.offset(i.wrapping_add(pos) as isize) as libc::c_int
                                as isize,
                        );
                }
                __res
            })
        {
            return 0 as libc::c_int
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn suffix_len(
    mut s: *const libc::c_char,
    s_len: size_t,
    pos: size_t,
    case_sensitive: libc::c_int,
) -> size_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < pos {
        if case_sensitive != 0 {
            if *s.offset(pos.wrapping_sub(i) as isize) as libc::c_int
                != *s
                    .offset(
                        s_len
                            .wrapping_sub(i)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int
            {
                break;
            }
        } else if ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *s.offset(pos.wrapping_sub(i) as isize)
                        as libc::c_int;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    });
                } else {
                    __res = tolower(
                        *s.offset(pos.wrapping_sub(i) as isize) as libc::c_int,
                    );
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(
                        *s.offset(pos.wrapping_sub(i) as isize) as libc::c_int as isize,
                    );
            }
            __res
        })
            != ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *s
                            .offset(
                                s_len
                                    .wrapping_sub(i)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int;
                        __res = (if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = tolower(
                            *s
                                .offset(
                                    s_len
                                        .wrapping_sub(i)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                ) as libc::c_int,
                        );
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(
                            *s
                                .offset(
                                    s_len
                                        .wrapping_sub(i)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                ) as libc::c_int as isize,
                        );
                }
                __res
            })
        {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    return i;
}
pub unsafe extern "C" fn generate_find_skip(
    mut find: *const libc::c_char,
    f_len: size_t,
    mut skip_lookup: *mut *mut size_t,
    case_sensitive: libc::c_int,
) {
    let mut i: size_t = 0;
    let mut s_len: size_t = 0;
    let mut sl: *mut size_t = ag_malloc(
        f_len.wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
    ) as *mut size_t;
    *skip_lookup = sl;
    let mut last_prefix: size_t = f_len;
    i = last_prefix;
    while i > 0 as libc::c_int as libc::c_ulong {
        if is_prefix(find, f_len, i, case_sensitive) != 0 {
            last_prefix = i;
        }
        *sl
            .offset(
                i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = last_prefix.wrapping_add(f_len.wrapping_sub(i));
        i = i.wrapping_sub(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < f_len {
        s_len = suffix_len(find, f_len, i, case_sensitive);
        if *find.offset(i.wrapping_sub(s_len) as isize) as libc::c_int
            != *find
                .offset(
                    f_len
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(s_len) as isize,
                ) as libc::c_int
        {
            *sl
                .offset(
                    f_len
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(s_len) as isize,
                ) = f_len
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(i)
                .wrapping_add(s_len);
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn ag_max(mut a: size_t, mut b: size_t) -> size_t {
    if b > a {
        return b;
    }
    return a;
}
pub unsafe extern "C" fn ag_min(mut a: size_t, mut b: size_t) -> size_t {
    if b < a {
        return b;
    }
    return a;
}
pub unsafe extern "C" fn generate_hash(
    mut find: *const libc::c_char,
    f_len: size_t,
    mut h_table: *mut uint8_t,
    case_sensitive: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = f_len.wrapping_sub(::std::mem::size_of::<uint16_t>() as libc::c_ulong)
        as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut caps_set: libc::c_int = 0;
        caps_set = 0 as libc::c_int;
        while caps_set
            < (1 as libc::c_int) << ::std::mem::size_of::<uint16_t>() as libc::c_ulong
        {
            let mut word: word_t = word_t { as_chars: [0; 2] };
            memcpy(
                &mut word.as_chars as *mut [libc::c_char; 2] as *mut libc::c_void,
                find.offset(i as isize) as *const libc::c_void,
                ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
            );
            let mut cap_index: libc::c_int = 0;
            cap_index = 0 as libc::c_int;
            while caps_set >> cap_index != 0 {
                if caps_set >> cap_index & 1 as libc::c_int != 0 {
                    word
                        .as_chars[cap_index
                        as usize] = (word.as_chars[cap_index as usize] as libc::c_int
                        - ('a' as i32 - 'A' as i32)) as libc::c_char;
                }
                cap_index += 1;
                cap_index;
            }
            let mut h: size_t = 0;
            h = (word.as_word as libc::c_int % (64 as libc::c_int * 1024 as libc::c_int))
                as size_t;
            while *h_table.offset(h as isize) != 0 {
                h = h
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_rem(
                        (64 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong,
                    );
            }
            *h_table.offset(h as isize) = (i + 1 as libc::c_int) as uint8_t;
            if case_sensitive != 0 {
                break;
            }
            caps_set += 1;
            caps_set;
        }
        i -= 1;
        i;
    }
}
pub unsafe extern "C" fn boyer_moore_strnstr(
    mut s: *const libc::c_char,
    mut find: *const libc::c_char,
    s_len: size_t,
    f_len: size_t,
    mut alpha_skip_lookup: *const size_t,
    mut find_skip_lookup: *const size_t,
    case_insensitive: libc::c_int,
) -> *const libc::c_char {
    let mut i: ssize_t = 0;
    let mut pos: size_t = f_len.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while pos < s_len {
        i = f_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as ssize_t;
        while i >= 0 as libc::c_int as libc::c_long
            && (if case_insensitive != 0 {
                ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *s.offset(pos as isize)
                                as libc::c_int;
                            __res = (if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            });
                        } else {
                            __res = tolower(*s.offset(pos as isize) as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(*s.offset(pos as isize) as libc::c_int as isize);
                    }
                    __res
                })
            } else {
                *s.offset(pos as isize) as libc::c_int
            }) == *find.offset(i as isize) as libc::c_int
        {
            pos = pos.wrapping_sub(1);
            pos;
            i -= 1;
            i;
        }
        if i < 0 as libc::c_int as libc::c_long {
            return s.offset(pos as isize).offset(1 as libc::c_int as isize);
        }
        pos = (pos as libc::c_ulong)
            .wrapping_add(
                ag_max(
                    *alpha_skip_lookup
                        .offset(*s.offset(pos as isize) as libc::c_uchar as isize),
                    *find_skip_lookup.offset(i as isize),
                ),
            ) as size_t as size_t;
    }
    return 0 as *const libc::c_char;
}
pub unsafe extern "C" fn hash_strnstr(
    mut s: *const libc::c_char,
    mut find: *const libc::c_char,
    s_len: size_t,
    f_len: size_t,
    mut h_table: *mut uint8_t,
    case_sensitive: libc::c_int,
) -> *const libc::c_char {
    if s_len < f_len {
        return 0 as *const libc::c_char;
    }
    let step: size_t = f_len
        .wrapping_sub(::std::mem::size_of::<uint16_t>() as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut s_i: size_t = f_len
        .wrapping_sub(::std::mem::size_of::<uint16_t>() as libc::c_ulong);
    while s_i <= s_len.wrapping_sub(f_len) {
        let mut h: size_t = 0;
        h = (*(s.offset(s_i as isize) as *const uint16_t) as libc::c_int
            % (64 as libc::c_int * 1024 as libc::c_int)) as size_t;
        while *h_table.offset(h as isize) != 0 {
            let mut current_block_5: u64;
            let mut R: *const libc::c_char = s
                .offset(s_i as isize)
                .offset(
                    -((*h_table.offset(h as isize) as libc::c_int - 1 as libc::c_int)
                        as isize),
                );
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            loop {
                if !(i < f_len) {
                    current_block_5 = 3640593987805443782;
                    break;
                }
                if (if case_sensitive != 0 {
                    *R.offset(i as isize) as libc::c_int
                } else {
                    ({
                        let mut __res: libc::c_int = 0;
                        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int = *R.offset(i as isize)
                                    as libc::c_int;
                                __res = (if __c < -(128 as libc::c_int)
                                    || __c > 255 as libc::c_int
                                {
                                    __c
                                } else {
                                    *(*__ctype_tolower_loc()).offset(__c as isize)
                                });
                            } else {
                                __res = tolower(*R.offset(i as isize) as libc::c_int);
                            }
                        } else {
                            __res = *(*__ctype_tolower_loc())
                                .offset(*R.offset(i as isize) as libc::c_int as isize);
                        }
                        __res
                    })
                }) != *find.offset(i as isize) as libc::c_int
                {
                    current_block_5 = 2968425633554183086;
                    break;
                }
                i = i.wrapping_add(1);
                i;
            }
            match current_block_5 {
                2968425633554183086 => {}
                _ => return R,
            }
            h = h
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_rem(
                    (64 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong,
                );
        }
        s_i = (s_i as libc::c_ulong).wrapping_add(step) as size_t as size_t;
    }
    s_i = s_i.wrapping_sub(step).wrapping_add(1 as libc::c_int as libc::c_ulong);
    while s_i <= s_len.wrapping_sub(f_len) {
        let mut current_block_13: u64;
        let mut i_0: size_t = 0;
        let mut R_0: *const libc::c_char = s.offset(s_i as isize);
        i_0 = 0 as libc::c_int as size_t;
        loop {
            if !(i_0 < f_len) {
                current_block_13 = 26972500619410423;
                break;
            }
            let mut s_c: libc::c_char = (if case_sensitive != 0 {
                *R_0.offset(i_0 as isize) as libc::c_int
            } else {
                ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *R_0.offset(i_0 as isize)
                                as libc::c_int;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = tolower(*R_0.offset(i_0 as isize) as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(*R_0.offset(i_0 as isize) as libc::c_int as isize);
                    }
                    __res
                })
            }) as libc::c_char;
            if s_c as libc::c_int != *find.offset(i_0 as isize) as libc::c_int {
                current_block_13 = 11042950489265723346;
                break;
            }
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
        match current_block_13 {
            11042950489265723346 => {}
            _ => return R_0,
        }
        s_i = s_i.wrapping_add(1);
        s_i;
    }
    return 0 as *const libc::c_char;
}
pub unsafe extern "C" fn invert_matches(
    mut buf: *const libc::c_char,
    buf_len: size_t,
    mut matches: *mut match_t,
    mut matches_len: size_t,
) -> size_t {
    let mut i: size_t = 0;
    let mut match_read_index: size_t = 0 as libc::c_int as size_t;
    let mut inverted_match_count: size_t = 0 as libc::c_int as size_t;
    let mut inverted_match_start: size_t = 0 as libc::c_int as size_t;
    let mut last_line_end: size_t = 0 as libc::c_int as size_t;
    let mut in_inverted_match: libc::c_int = 1 as libc::c_int;
    let mut next_match: match_t = match_t { start: 0, end: 0 };
    log_debug(
        b"Inverting %u matches.\0" as *const u8 as *const libc::c_char,
        matches_len,
    );
    if matches_len > 0 as libc::c_int as libc::c_ulong {
        next_match = *matches.offset(0 as libc::c_int as isize);
    } else {
        next_match.start = buf_len.wrapping_add(1 as libc::c_int as libc::c_ulong);
    }
    if matches_len == 0 as libc::c_int as libc::c_ulong {
        (*matches.offset(0 as libc::c_int as isize)).start = 0 as libc::c_int as size_t;
        (*matches.offset(0 as libc::c_int as isize))
            .end = buf_len.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        return 1 as libc::c_int as size_t;
    }
    i = 0 as libc::c_int as size_t;
    while i < buf_len {
        if i == next_match.start {
            i = (next_match.end).wrapping_sub(1 as libc::c_int as libc::c_ulong);
            match_read_index = match_read_index.wrapping_add(1);
            match_read_index;
            if match_read_index < matches_len {
                next_match = *matches.offset(match_read_index as isize);
            }
            if in_inverted_match != 0 && last_line_end > inverted_match_start {
                (*matches.offset(inverted_match_count as isize))
                    .start = inverted_match_start;
                (*matches.offset(inverted_match_count as isize))
                    .end = last_line_end.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                inverted_match_count = inverted_match_count.wrapping_add(1);
                inverted_match_count;
            }
            in_inverted_match = 0 as libc::c_int;
        } else if i == buf_len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && in_inverted_match != 0
        {
            (*matches.offset(inverted_match_count as isize))
                .start = inverted_match_start;
            (*matches.offset(inverted_match_count as isize)).end = i;
            inverted_match_count = inverted_match_count.wrapping_add(1);
            inverted_match_count;
        } else if *buf.offset(i as isize) as libc::c_int == '\n' as i32 {
            last_line_end = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            if in_inverted_match == 0 {
                inverted_match_start = last_line_end;
            }
            in_inverted_match = 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < matches_len {
        log_debug(
            b"Inverted match %i start %i end %i.\0" as *const u8 as *const libc::c_char,
            i,
            (*matches.offset(i as isize)).start,
            (*matches.offset(i as isize)).end,
        );
        i = i.wrapping_add(1);
        i;
    }
    return inverted_match_count;
}
pub unsafe extern "C" fn realloc_matches(
    mut matches: *mut *mut match_t,
    mut matches_size: *mut size_t,
    mut matches_len: size_t,
) {
    if matches_len < *matches_size {
        return;
    }
    *matches_size = if !(*matches).is_null() {
        (*matches_size).wrapping_mul(2 as libc::c_int as libc::c_ulong)
    } else {
        100 as libc::c_int as libc::c_ulong
    };
    *matches = ag_realloc(
        *matches as *mut libc::c_void,
        (*matches_size).wrapping_mul(::std::mem::size_of::<match_t>() as libc::c_ulong),
    ) as *mut match_t;
}
pub unsafe extern "C" fn compile_study(
    mut re: *mut *mut pcre,
    mut re_extra: *mut *mut pcre_extra,
    mut q: *mut libc::c_char,
    pcre_opts: libc::c_int,
    study_opts: libc::c_int,
) {
    let mut pcre_err: *const libc::c_char = 0 as *const libc::c_char;
    let mut pcre_err_offset: libc::c_int = 0 as libc::c_int;
    *re = pcre_compile(
        q,
        pcre_opts,
        &mut pcre_err,
        &mut pcre_err_offset,
        0 as *const libc::c_uchar,
    );
    if (*re).is_null() {
        die(
            b"Bad regex! pcre_compile() failed at position %i: %s\nIf you meant to search for a literal string, run ag with -Q\0"
                as *const u8 as *const libc::c_char,
            pcre_err_offset,
            pcre_err,
        );
    }
    *re_extra = pcre_study(*re, study_opts, &mut pcre_err);
    if (*re_extra).is_null() {
        log_debug(
            b"pcre_study returned nothing useful. Error: %s\0" as *const u8
                as *const libc::c_char,
            pcre_err,
        );
    }
}
pub unsafe extern "C" fn is_binary(
    mut buf: *const libc::c_void,
    buf_len: size_t,
) -> libc::c_int {
    let mut suspicious_bytes: size_t = 0 as libc::c_int as size_t;
    let mut total_bytes: size_t = if buf_len > 512 as libc::c_int as libc::c_ulong {
        512 as libc::c_int as libc::c_ulong
    } else {
        buf_len
    };
    let mut buf_c: *const libc::c_uchar = buf as *const libc::c_uchar;
    let mut i: size_t = 0;
    if buf_len == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if buf_len >= 3 as libc::c_int as libc::c_ulong
        && *buf_c.offset(0 as libc::c_int as isize) as libc::c_int == 0xef as libc::c_int
        && *buf_c.offset(1 as libc::c_int as isize) as libc::c_int == 0xbb as libc::c_int
        && *buf_c.offset(2 as libc::c_int as isize) as libc::c_int == 0xbf as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if buf_len >= 5 as libc::c_int as libc::c_ulong
        && strncmp(
            buf as *const libc::c_char,
            b"%PDF-\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    let mut current_block_18: u64;
    i = 0 as libc::c_int as size_t;
    while i < total_bytes {
        if *buf_c.offset(i as isize) as libc::c_int == '\0' as i32 {
            return 1 as libc::c_int
        } else {
            if ((*buf_c.offset(i as isize) as libc::c_int) < 7 as libc::c_int
                || *buf_c.offset(i as isize) as libc::c_int > 14 as libc::c_int)
                && ((*buf_c.offset(i as isize) as libc::c_int) < 32 as libc::c_int
                    || *buf_c.offset(i as isize) as libc::c_int > 127 as libc::c_int)
            {
                if *buf_c.offset(i as isize) as libc::c_int > 193 as libc::c_int
                    && (*buf_c.offset(i as isize) as libc::c_int) < 224 as libc::c_int
                    && i.wrapping_add(1 as libc::c_int as libc::c_ulong) < total_bytes
                {
                    i = i.wrapping_add(1);
                    i;
                    if *buf_c.offset(i as isize) as libc::c_int > 127 as libc::c_int
                        && (*buf_c.offset(i as isize) as libc::c_int)
                            < 192 as libc::c_int
                    {
                        current_block_18 = 14523784380283086299;
                    } else {
                        current_block_18 = 2370887241019905314;
                    }
                } else if *buf_c.offset(i as isize) as libc::c_int > 223 as libc::c_int
                    && (*buf_c.offset(i as isize) as libc::c_int) < 240 as libc::c_int
                    && i.wrapping_add(2 as libc::c_int as libc::c_ulong) < total_bytes
                {
                    i = i.wrapping_add(1);
                    i;
                    if *buf_c.offset(i as isize) as libc::c_int > 127 as libc::c_int
                        && (*buf_c.offset(i as isize) as libc::c_int)
                            < 192 as libc::c_int
                        && *buf_c
                            .offset(
                                i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int > 127 as libc::c_int
                        && (*buf_c
                            .offset(
                                i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int) < 192 as libc::c_int
                    {
                        i = i.wrapping_add(1);
                        i;
                        current_block_18 = 14523784380283086299;
                    } else {
                        current_block_18 = 2370887241019905314;
                    }
                } else {
                    current_block_18 = 2370887241019905314;
                }
                match current_block_18 {
                    14523784380283086299 => {}
                    _ => {
                        suspicious_bytes = suspicious_bytes.wrapping_add(1);
                        suspicious_bytes;
                        if i >= 32 as libc::c_int as libc::c_ulong
                            && suspicious_bytes
                                .wrapping_mul(100 as libc::c_int as libc::c_ulong)
                                .wrapping_div(total_bytes)
                                > 10 as libc::c_int as libc::c_ulong
                        {
                            return 1 as libc::c_int;
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    if suspicious_bytes
        .wrapping_mul(100 as libc::c_int as libc::c_ulong)
        .wrapping_div(total_bytes) > 10 as libc::c_int as libc::c_ulong
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn is_regex(mut query: *const libc::c_char) -> libc::c_int {
    let mut regex_chars: [libc::c_char; 13] = [
        '$' as i32 as libc::c_char,
        '(' as i32 as libc::c_char,
        ')' as i32 as libc::c_char,
        '*' as i32 as libc::c_char,
        '+' as i32 as libc::c_char,
        '.' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '[' as i32 as libc::c_char,
        '\\' as i32 as libc::c_char,
        '^' as i32 as libc::c_char,
        '{' as i32 as libc::c_char,
        '|' as i32 as libc::c_char,
        '\0' as i32 as libc::c_char,
    ];
    return (strpbrk(query, regex_chars.as_mut_ptr())
        != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
}
pub unsafe extern "C" fn is_fnmatch(mut filename: *const libc::c_char) -> libc::c_int {
    let mut fnmatch_chars: [libc::c_char; 6] = [
        '!' as i32 as libc::c_char,
        '*' as i32 as libc::c_char,
        '?' as i32 as libc::c_char,
        '[' as i32 as libc::c_char,
        ']' as i32 as libc::c_char,
        '\0' as i32 as libc::c_char,
    ];
    return (strpbrk(filename, fnmatch_chars.as_mut_ptr())
        != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
}
pub unsafe extern "C" fn binary_search(
    mut needle: *const libc::c_char,
    mut haystack: *mut *mut libc::c_char,
    mut start: libc::c_int,
    mut end: libc::c_int,
) -> libc::c_int {
    let mut mid: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    if start == end {
        return -(1 as libc::c_int);
    }
    mid = start + (end - start) / 2 as libc::c_int;
    rc = strcmp(needle, *haystack.offset(mid as isize));
    if rc < 0 as libc::c_int {
        return binary_search(needle, haystack, start, mid)
    } else if rc > 0 as libc::c_int {
        return binary_search(needle, haystack, mid + 1 as libc::c_int, end)
    }
    return mid;
}
static mut wordchar_table: [libc::c_int; 256] = [0; 256];
pub unsafe extern "C" fn init_wordchar_table() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        let mut ch: libc::c_char = i as libc::c_char;
        wordchar_table[i
            as usize] = ('a' as i32 <= ch as libc::c_int
            && ch as libc::c_int <= 'z' as i32
            || 'A' as i32 <= ch as libc::c_int && ch as libc::c_int <= 'Z' as i32
            || '0' as i32 <= ch as libc::c_int && ch as libc::c_int <= '9' as i32
            || ch as libc::c_int == '_' as i32) as libc::c_int;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn is_wordchar(mut ch: libc::c_char) -> libc::c_int {
    return wordchar_table[ch as libc::c_uchar as usize];
}
pub unsafe extern "C" fn is_lowercase(mut s: *const libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while *s.offset(i as isize) as libc::c_int != '\0' as i32 {
        if !(*s.offset(i as isize) as libc::c_int & !(0x7f as libc::c_int)
            == 0 as libc::c_int)
            || *(*__ctype_b_loc()).offset(*s.offset(i as isize) as libc::c_int as isize)
                as libc::c_int & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn is_directory(
    mut path: *const libc::c_char,
    mut d: *const dirent,
) -> libc::c_int {
    if (*d).d_type as libc::c_int != DT_UNKNOWN as libc::c_int
        && (*d).d_type as libc::c_int != DT_LNK as libc::c_int
    {
        return ((*d).d_type as libc::c_int == DT_DIR as libc::c_int) as libc::c_int;
    }
    let mut full_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: stat = stat {
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
    ag_asprintf(
        &mut full_path as *mut *mut libc::c_char,
        b"%s/%s\0" as *const u8 as *const libc::c_char,
        path,
        ((*d).d_name).as_ptr(),
    );
    if stat(full_path, &mut s) != 0 as libc::c_int {
        free(full_path as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    let mut is_dir: libc::c_int = (s.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int;
    free(full_path as *mut libc::c_void);
    return is_dir;
}
pub unsafe extern "C" fn is_symlink(
    mut path: *const libc::c_char,
    mut d: *const dirent,
) -> libc::c_int {
    if (*d).d_type as libc::c_int != DT_UNKNOWN as libc::c_int {
        return ((*d).d_type as libc::c_int == DT_LNK as libc::c_int) as libc::c_int;
    }
    let mut full_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: stat = stat {
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
    ag_asprintf(
        &mut full_path as *mut *mut libc::c_char,
        b"%s/%s\0" as *const u8 as *const libc::c_char,
        path,
        ((*d).d_name).as_ptr(),
    );
    if lstat(full_path, &mut s) != 0 as libc::c_int {
        free(full_path as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    free(full_path as *mut libc::c_void);
    return (s.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o120000 as libc::c_int as libc::c_uint) as libc::c_int;
}
pub unsafe extern "C" fn is_named_pipe(
    mut path: *const libc::c_char,
    mut d: *const dirent,
) -> libc::c_int {
    if (*d).d_type as libc::c_int != DT_UNKNOWN as libc::c_int
        && (*d).d_type as libc::c_int != DT_LNK as libc::c_int
    {
        return ((*d).d_type as libc::c_int == DT_FIFO as libc::c_int
            || (*d).d_type as libc::c_int == DT_SOCK as libc::c_int) as libc::c_int;
    }
    let mut full_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: stat = stat {
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
    ag_asprintf(
        &mut full_path as *mut *mut libc::c_char,
        b"%s/%s\0" as *const u8 as *const libc::c_char,
        path,
        ((*d).d_name).as_ptr(),
    );
    if stat(full_path, &mut s) != 0 as libc::c_int {
        free(full_path as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    free(full_path as *mut libc::c_void);
    return (s.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o10000 as libc::c_int as libc::c_uint
        || s.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o140000 as libc::c_int as libc::c_uint) as libc::c_int;
}
pub unsafe extern "C" fn ag_asprintf(
    mut ret: *mut *mut libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    if vasprintf(ret, fmt, args_0.as_va_list()) == -(1 as libc::c_int) {
        die(b"vasprintf returned -1\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn die(mut fmt: *const libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vplog(LOG_LEVEL_ERR as libc::c_int as libc::c_uint, fmt, args_0.as_va_list());
    exit(2 as libc::c_int);
}
pub unsafe extern "C" fn fgetln(
    mut fp: *mut FILE,
    mut lenp: *mut size_t,
) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut used: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0 as libc::c_int;
    flockfile(fp);
    loop {
        c = getc_unlocked(fp);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        if buf.is_null() || len >= used {
            let mut nsize: size_t = 0;
            let mut newbuf: *mut libc::c_char = 0 as *mut libc::c_char;
            nsize = (used + 8192 as libc::c_int) as size_t;
            newbuf = realloc(buf as *mut libc::c_void, nsize) as *mut libc::c_char;
            if newbuf.is_null() {
                funlockfile(fp);
                if !buf.is_null() {
                    free(buf as *mut libc::c_void);
                }
                return 0 as *mut libc::c_char;
            }
            buf = newbuf;
            used = nsize as libc::c_int;
        }
        let fresh1 = len;
        len = len + 1;
        *buf.offset(fresh1 as isize) = c as libc::c_char;
        if c == '\n' as i32 {
            break;
        }
    }
    funlockfile(fp);
    *lenp = len as size_t;
    return buf;
}
pub unsafe extern "C" fn buf_getline(
    mut line: *mut *const libc::c_char,
    mut buf: *const libc::c_char,
    buf_len: size_t,
    buf_offset: size_t,
) -> ssize_t {
    let mut cur: *const libc::c_char = buf.offset(buf_offset as isize);
    let mut i: ssize_t = 0;
    i = 0 as libc::c_int as ssize_t;
    while buf_offset.wrapping_add(i as libc::c_ulong) < buf_len
        && *cur.offset(i as isize) as libc::c_int != '\n' as i32
    {
        i += 1;
        i;
    }
    *line = cur;
    return i;
}
pub unsafe extern "C" fn strlcpy(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut size: size_t,
) -> size_t {
    let mut d: *mut libc::c_char = dst;
    let mut s: *const libc::c_char = src;
    let mut n: size_t = size;
    if n != 0 as libc::c_int as libc::c_ulong {
        loop {
            n = n.wrapping_sub(1);
            if !(n != 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            let fresh2 = s;
            s = s.offset(1);
            let fresh3 = d;
            d = d.offset(1);
            *fresh3 = *fresh2;
            if *fresh3 as libc::c_int == '\0' as i32 {
                break;
            }
        }
    }
    if n == 0 as libc::c_int as libc::c_ulong {
        if size != 0 as libc::c_int as libc::c_ulong {
            *d = '\0' as i32 as libc::c_char;
        }
        loop {
            let fresh4 = s;
            s = s.offset(1);
            if !(*fresh4 != 0) {
                break;
            }
        }
    }
    return (s.offset_from(src) as libc::c_long - 1 as libc::c_int as libc::c_long)
        as size_t;
}
