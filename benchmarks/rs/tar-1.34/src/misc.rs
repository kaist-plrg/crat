use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type quoting_options;
    fn renameat(
        __oldfd: libc::c_int,
        __old: *const libc::c_char,
        __newfd: libc::c_int,
        __new: *const libc::c_char,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strnlen(__string: *const libc::c_char, __maxlen: size_t) -> size_t;
    fn __errno_location() -> *mut libc::c_int;
    fn openat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __oflag: libc::c_int,
        _: ...
    ) -> libc::c_int;
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
    fn __fxstatat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn fork() -> __pid_t;
    fn unlinkat(
        __fd: libc::c_int,
        __name: *const libc::c_char,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn getpagesize() -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __strtol_internal(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
        __group: libc::c_int,
    ) -> libc::c_long;
    fn __strtoul_internal(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
        __group: libc::c_int,
    ) -> libc::c_ulong;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn fdopendir(__fd: libc::c_int) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn streamsavedir(_: *mut DIR, _: savedir_option) -> *mut libc::c_char;
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn x2realloc(p: *mut libc::c_void, pn: *mut size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn imaxtostr(_: intmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    static mut error_hook: Option::<unsafe extern "C" fn() -> ()>;
    static mut exit_status: libc::c_int;
    fn call_arg_fatal(call: *const libc::c_char, name: *const libc::c_char) -> !;
    fn close_error(_: *const libc::c_char);
    fn close_warn(_: *const libc::c_char);
    fn open_error(_: *const libc::c_char);
    fn open_fatal(_: *const libc::c_char) -> !;
    fn open_warn(_: *const libc::c_char);
    fn read_error_details(_: *const libc::c_char, _: off_t, _: size_t);
    fn read_warn_details(_: *const libc::c_char, _: off_t, _: size_t);
    fn readlink_error(_: *const libc::c_char);
    fn readlink_warn(_: *const libc::c_char);
    fn savedir_error(_: *const libc::c_char);
    fn savedir_warn(_: *const libc::c_char);
    fn seek_error_details(_: *const libc::c_char, _: off_t);
    fn seek_warn_details(_: *const libc::c_char, _: off_t);
    fn stat_error(_: *const libc::c_char);
    fn stat_warn(_: *const libc::c_char);
    static mut rmt_dev_name__: *const libc::c_char;
    static mut force_local_option: bool;
    static mut backup_type: backup_type;
    static mut ignore_failed_read_option: bool;
    static mut verbose_option: libc::c_int;
    static mut open_read_flags: libc::c_int;
    static mut open_searchdir_flags: libc::c_int;
    static mut fstatat_flags: libc::c_int;
    static mut savedir_sort_order: libc::c_int;
    static mut stdlis: *mut FILE;
    fn remove_delayed_set_stat(fname: *const libc::c_char);
    fn make_file_name(
        dir_name: *const libc::c_char,
        name: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn set_exit_status(val: libc::c_int);
    static mut warning_option: libc::c_int;
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> size_t;
    fn find_backup_file_name(
        _: libc::c_int,
        _: *const libc::c_char,
        _: backup_type,
    ) -> *mut libc::c_char;
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn get_quoting_style(o: *const quoting_options) -> quoting_style;
    fn quotearg_colon(arg: *const libc::c_char) -> *mut libc::c_char;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn xgetcwd() -> *mut libc::c_char;
    fn fdutimensat(
        fd: libc::c_int,
        dir: libc::c_int,
        name: *const libc::c_char,
        _: *const timespec,
        atflag: libc::c_int,
    ) -> libc::c_int;
}
pub type __intmax_t = libc::c_long;
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
pub type __syscall_slong_t = libc::c_long;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
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
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type DIR = __dirstream;
pub type savedir_option = libc::c_uint;
pub const SAVEDIR_SORT_FASTREAD: savedir_option = 2;
pub const SAVEDIR_SORT_INODE: savedir_option = 2;
pub const SAVEDIR_SORT_NAME: savedir_option = 1;
pub const SAVEDIR_SORT_NONE: savedir_option = 0;
pub const DEFAULT_MXFAST: C2RustUnnamed = 128;
pub type C2RustUnnamed = libc::c_uint;
pub type backup_type = libc::c_uint;
pub const numbered_backups: backup_type = 3;
pub const numbered_existing_backups: backup_type = 2;
pub const simple_backups: backup_type = 1;
pub const no_backups: backup_type = 0;
pub type quoting_style = libc::c_uint;
pub const custom_quoting_style: quoting_style = 10;
pub const clocale_quoting_style: quoting_style = 9;
pub const locale_quoting_style: quoting_style = 8;
pub const escape_quoting_style: quoting_style = 7;
pub const c_maybe_quoting_style: quoting_style = 6;
pub const c_quoting_style: quoting_style = 5;
pub const shell_escape_always_quoting_style: quoting_style = 4;
pub const shell_escape_quoting_style: quoting_style = 3;
pub const shell_always_quoting_style: quoting_style = 2;
pub const shell_quoting_style: quoting_style = 1;
pub const literal_quoting_style: quoting_style = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wd {
    pub name: *const libc::c_char,
    pub abspath: *mut libc::c_char,
    pub fd: libc::c_int,
}
pub const CHDIR_CACHE_SIZE: C2RustUnnamed_1 = 16;
pub type namebuf_t = *mut namebuf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct namebuf {
    pub buffer: *mut libc::c_char,
    pub buffer_size: size_t,
    pub dir_length: size_t,
}
pub const BILLION: C2RustUnnamed_0 = 1000000000;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const LOG10_BILLION: C2RustUnnamed_0 = 9;
pub type remove_option = libc::c_uint;
pub const WANT_DIRECTORY_REMOVE_OPTION: remove_option = 2;
pub const RECURSIVE_REMOVE_OPTION: remove_option = 1;
pub const ORDINARY_REMOVE_OPTION: remove_option = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
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
unsafe extern "C" fn strtoimax(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut base: libc::c_int,
) -> intmax_t {
    return __strtol_internal(nptr, endptr, base, 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn strtoumax(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut base: libc::c_int,
) -> uintmax_t {
    return __strtoul_internal(nptr, endptr, base, 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn x2nrealloc(
    mut p: *mut libc::c_void,
    mut pn: *mut size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut n: size_t = *pn;
    if p.is_null() {
        if n == 0 {
            n = (DEFAULT_MXFAST as libc::c_int as libc::c_ulong).wrapping_div(s);
            n = (n as libc::c_ulong)
                .wrapping_add((n == 0) as libc::c_int as libc::c_ulong) as size_t
                as size_t;
        }
        if (if (9223372036854775807 as libc::c_long as libc::c_ulong)
            < 18446744073709551615 as libc::c_ulong
        {
            9223372036854775807 as libc::c_long as libc::c_ulong
        } else {
            (18446744073709551615 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        })
            .wrapping_div(s) < n
        {
            xalloc_die();
        }
    } else {
        if (if (9223372036854775807 as libc::c_long as libc::c_ulong)
            < 18446744073709551615 as libc::c_ulong
        {
            9223372036854775807 as libc::c_long as libc::c_ulong
        } else {
            18446744073709551615 as libc::c_ulong
        })
            .wrapping_div(3 as libc::c_int as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_div(s) <= n
        {
            xalloc_die();
        }
        n = (n as libc::c_ulong)
            .wrapping_add(
                n
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
    }
    *pn = n;
    return xrealloc(p, n.wrapping_mul(s));
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn represent_uintmax(mut n: uintmax_t) -> intmax_t {
    if n <= 9223372036854775807 as libc::c_long as libc::c_ulong {
        return n as intmax_t
    } else {
        let mut nd: intmax_t = n
            .wrapping_sub(
                (-(9223372036854775807 as libc::c_long)
                    - 1 as libc::c_int as libc::c_long) as libc::c_ulong,
            ) as intmax_t;
        return nd
            + (-(9223372036854775807 as libc::c_long)
                - 1 as libc::c_int as libc::c_long);
    };
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn valid_timespec(mut t: timespec) -> bool {
    return 0 as libc::c_int as libc::c_long <= t.tv_nsec;
}
pub unsafe extern "C" fn quote_n_colon(
    mut n: libc::c_int,
    mut arg: *const libc::c_char,
) -> *const libc::c_char {
    return quotearg_n_style_colon(
        n,
        get_quoting_style(0 as *const quoting_options),
        arg,
    );
}
pub unsafe extern "C" fn assign_string(
    mut string: *mut *mut libc::c_char,
    mut value: *const libc::c_char,
) {
    rpl_free(*string as *mut libc::c_void);
    *string = if !value.is_null() { xstrdup(value) } else { 0 as *mut libc::c_char };
}
pub unsafe extern "C" fn assign_string_n(
    mut string: *mut *mut libc::c_char,
    mut value: *const libc::c_char,
    mut n: size_t,
) {
    rpl_free(*string as *mut libc::c_void);
    if !value.is_null() {
        let mut l: size_t = strnlen(value, n);
        let mut p: *mut libc::c_char = xmalloc(
            l.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        memcpy(p as *mut libc::c_void, value as *const libc::c_void, l);
        *p.offset(l as isize) = 0 as libc::c_int as libc::c_char;
        *string = p;
    }
}
pub unsafe extern "C" fn unquote_string(mut string: *mut libc::c_char) -> libc::c_int {
    let mut result: libc::c_int = 1 as libc::c_int;
    let mut source: *mut libc::c_char = string;
    let mut destination: *mut libc::c_char = string;
    while *source != 0 {
        if *source as libc::c_int == '\\' as i32 {
            source = source.offset(1);
            match *source as libc::c_int {
                92 => {
                    let fresh0 = destination;
                    destination = destination.offset(1);
                    *fresh0 = '\\' as i32 as libc::c_char;
                    source = source.offset(1);
                    source;
                }
                97 => {
                    let fresh1 = destination;
                    destination = destination.offset(1);
                    *fresh1 = '\u{7}' as i32 as libc::c_char;
                    source = source.offset(1);
                    source;
                }
                98 => {
                    let fresh2 = destination;
                    destination = destination.offset(1);
                    *fresh2 = '\u{8}' as i32 as libc::c_char;
                    source = source.offset(1);
                    source;
                }
                102 => {
                    let fresh3 = destination;
                    destination = destination.offset(1);
                    *fresh3 = '\u{c}' as i32 as libc::c_char;
                    source = source.offset(1);
                    source;
                }
                110 => {
                    let fresh4 = destination;
                    destination = destination.offset(1);
                    *fresh4 = '\n' as i32 as libc::c_char;
                    source = source.offset(1);
                    source;
                }
                114 => {
                    let fresh5 = destination;
                    destination = destination.offset(1);
                    *fresh5 = '\r' as i32 as libc::c_char;
                    source = source.offset(1);
                    source;
                }
                116 => {
                    let fresh6 = destination;
                    destination = destination.offset(1);
                    *fresh6 = '\t' as i32 as libc::c_char;
                    source = source.offset(1);
                    source;
                }
                118 => {
                    let fresh7 = destination;
                    destination = destination.offset(1);
                    *fresh7 = '\u{b}' as i32 as libc::c_char;
                    source = source.offset(1);
                    source;
                }
                63 => {
                    let fresh8 = destination;
                    destination = destination.offset(1);
                    *fresh8 = 0o177 as libc::c_int as libc::c_char;
                    source = source.offset(1);
                    source;
                }
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                    let fresh9 = source;
                    source = source.offset(1);
                    let mut value: libc::c_int = *fresh9 as libc::c_int - '0' as i32;
                    if (*source as libc::c_int) < '0' as i32
                        || *source as libc::c_int > '7' as i32
                    {
                        let fresh10 = destination;
                        destination = destination.offset(1);
                        *fresh10 = value as libc::c_char;
                    } else {
                        let fresh11 = source;
                        source = source.offset(1);
                        value = value * 8 as libc::c_int + *fresh11 as libc::c_int
                            - '0' as i32;
                        if (*source as libc::c_int) < '0' as i32
                            || *source as libc::c_int > '7' as i32
                        {
                            let fresh12 = destination;
                            destination = destination.offset(1);
                            *fresh12 = value as libc::c_char;
                        } else {
                            let fresh13 = source;
                            source = source.offset(1);
                            value = value * 8 as libc::c_int + *fresh13 as libc::c_int
                                - '0' as i32;
                            let fresh14 = destination;
                            destination = destination.offset(1);
                            *fresh14 = value as libc::c_char;
                        }
                    }
                }
                _ => {
                    result = 0 as libc::c_int;
                    let fresh15 = destination;
                    destination = destination.offset(1);
                    *fresh15 = '\\' as i32 as libc::c_char;
                    if *source != 0 {
                        let fresh16 = source;
                        source = source.offset(1);
                        let fresh17 = destination;
                        destination = destination.offset(1);
                        *fresh17 = *fresh16;
                    }
                }
            }
        } else if source != destination {
            let fresh18 = source;
            source = source.offset(1);
            let fresh19 = destination;
            destination = destination.offset(1);
            *fresh19 = *fresh18;
        } else {
            source = source.offset(1);
            source;
            destination = destination.offset(1);
            destination;
        }
    }
    if source != destination {
        *destination = '\0' as i32 as libc::c_char;
    }
    return result;
}
pub unsafe extern "C" fn zap_slashes(mut name: *mut libc::c_char) -> *mut libc::c_char {
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    if name.is_null() || *name as libc::c_int == 0 as libc::c_int {
        return name;
    }
    q = name.offset(strlen(name) as isize).offset(-(1 as libc::c_int as isize));
    while q > name && *q as libc::c_int == '/' as i32 {
        let fresh20 = q;
        q = q.offset(-1);
        *fresh20 = '\0' as i32 as libc::c_char;
    }
    return name;
}
pub unsafe extern "C" fn normalize_filename_x(mut file_name: *mut libc::c_char) {
    let mut name: *mut libc::c_char = file_name.offset(0 as libc::c_int as isize);
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_char = 0;
    name = name
        .offset(
            (0 as libc::c_int != 0 && *name as libc::c_int == '/' as i32
                && *name.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
                && !(*name.offset(2 as libc::c_int as isize) as libc::c_int
                    == '/' as i32)) as libc::c_int as isize,
        );
    p = name;
    q = p;
    loop {
        *p = *q;
        if !(*p as libc::c_int == '.' as i32
            && *q.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32)
        {
            break;
        }
        q = q.offset(2 as libc::c_int as isize);
        while *q as libc::c_int == '/' as i32 {
            q = q.offset(1);
            q;
        }
        p = p.offset((*q == 0) as libc::c_int as isize);
    }
    loop {
        let fresh21 = q;
        q = q.offset(1);
        c = *fresh21;
        let fresh22 = p;
        p = p.offset(1);
        *fresh22 = c;
        if !(*fresh22 as libc::c_int != '\0' as i32) {
            break;
        }
        if c as libc::c_int == '/' as i32 {
            while *q.offset((*q as libc::c_int == '.' as i32) as libc::c_int as isize)
                as libc::c_int == '/' as i32
            {
                q = q
                    .offset(
                        ((*q as libc::c_int == '.' as i32) as libc::c_int
                            + 1 as libc::c_int) as isize,
                    );
            }
        }
    }
    if (2 as libc::c_int as libc::c_long) < p.offset_from(name) as libc::c_long {
        p = p
            .offset(
                -((*p.offset(-(2 as libc::c_int) as isize) as libc::c_int == '.' as i32
                    && *p.offset(-(3 as libc::c_int) as isize) as libc::c_int
                        == '/' as i32) as libc::c_int as isize),
            );
        p = p
            .offset(
                -(((2 as libc::c_int as libc::c_long)
                    < p.offset_from(name) as libc::c_long
                    && *p.offset(-(2 as libc::c_int) as isize) as libc::c_int
                        == '/' as i32) as libc::c_int as isize),
            );
        *p.offset(-(1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
    }
}
pub unsafe extern "C" fn normalize_filename(
    mut cdidx: libc::c_int,
    mut name: *const libc::c_char,
) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(*name.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32) {
        let mut cdpath: *const libc::c_char = tar_getcdpath(cdidx);
        let mut copylen: size_t = 0;
        let mut need_separator: bool = false;
        copylen = strlen(cdpath);
        need_separator = !(0 as libc::c_int != 0
            && copylen == 2 as libc::c_int as libc::c_ulong
            && *cdpath.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32);
        copy = xmalloc(
            copylen
                .wrapping_add(need_separator as libc::c_ulong)
                .wrapping_add(strlen(name))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy(copy, cdpath);
        *copy.offset(copylen as isize) = '/' as i32 as libc::c_char;
        strcpy(
            copy.offset(copylen as isize).offset(need_separator as libc::c_int as isize),
            name,
        );
    }
    if copy.is_null() {
        copy = xstrdup(name);
    }
    normalize_filename_x(copy);
    return copy;
}
pub unsafe extern "C" fn replace_prefix(
    mut pname: *mut *mut libc::c_char,
    mut samp: *const libc::c_char,
    mut slen: size_t,
    mut repl: *const libc::c_char,
    mut rlen: size_t,
) {
    let mut name: *mut libc::c_char = *pname;
    let mut nlen: size_t = strlen(name);
    if nlen > slen
        && memcmp(name as *const libc::c_void, samp as *const libc::c_void, slen)
            == 0 as libc::c_int
        && *name.offset(slen as isize) as libc::c_int == '/' as i32
    {
        if rlen > slen {
            name = xrealloc(
                name as *mut libc::c_void,
                nlen
                    .wrapping_sub(slen)
                    .wrapping_add(rlen)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            *pname = name;
        }
        memmove(
            name.offset(rlen as isize) as *mut libc::c_void,
            name.offset(slen as isize) as *const libc::c_void,
            nlen.wrapping_sub(slen).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        memcpy(name as *mut libc::c_void, repl as *const libc::c_void, rlen);
    }
}
pub unsafe extern "C" fn sysinttostr(
    mut value: uintmax_t,
    mut minval: intmax_t,
    mut maxval: uintmax_t,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    if value <= maxval {
        return umaxtostr(value, buf)
    } else {
        let mut i: intmax_t = value.wrapping_sub(minval as libc::c_ulong) as intmax_t;
        return imaxtostr(i + minval, buf);
    };
}
pub unsafe extern "C" fn strtosysint(
    mut arg: *const libc::c_char,
    mut arglim: *mut *mut libc::c_char,
    mut minval: intmax_t,
    mut maxval: uintmax_t,
) -> intmax_t {
    *__errno_location() = 0 as libc::c_int;
    if maxval <= 9223372036854775807 as libc::c_long as libc::c_ulong {
        if (*arg.offset((*arg as libc::c_int == '-' as i32) as libc::c_int as isize)
            as libc::c_uint)
            .wrapping_sub('0' as i32 as libc::c_uint) <= 9 as libc::c_int as libc::c_uint
        {
            let mut i: intmax_t = strtoimax(arg, arglim, 10 as libc::c_int);
            let mut imaxval: intmax_t = maxval as intmax_t;
            if minval <= i && i <= imaxval {
                return i;
            }
            *__errno_location() = 34 as libc::c_int;
            return (if i < minval { minval as libc::c_ulong } else { maxval })
                as intmax_t;
        }
    } else if (*arg as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
        <= 9 as libc::c_int as libc::c_uint
    {
        let mut i_0: uintmax_t = strtoumax(arg, arglim, 10 as libc::c_int);
        if i_0 <= maxval {
            return represent_uintmax(i_0);
        }
        *__errno_location() = 34 as libc::c_int;
        return maxval as intmax_t;
    }
    *__errno_location() = 22 as libc::c_int;
    return 0 as libc::c_int as intmax_t;
}
pub unsafe extern "C" fn code_ns_fraction(
    mut ns: libc::c_int,
    mut p: *mut libc::c_char,
) {
    if ns == 0 as libc::c_int {
        *p = '\0' as i32 as libc::c_char;
    } else {
        let mut i: libc::c_int = 9 as libc::c_int;
        let fresh23 = p;
        p = p.offset(1);
        *fresh23 = '.' as i32 as libc::c_char;
        while ns % 10 as libc::c_int == 0 as libc::c_int {
            ns /= 10 as libc::c_int;
            i -= 1;
            i;
        }
        *p.offset(i as isize) = '\0' as i32 as libc::c_char;
        loop {
            i -= 1;
            *p
                .offset(
                    i as isize,
                ) = ('0' as i32 + ns % 10 as libc::c_int) as libc::c_char;
            if i == 0 as libc::c_int {
                break;
            }
            ns /= 10 as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn code_timespec(
    mut t: timespec,
    mut sbuf: *mut libc::c_char,
) -> *const libc::c_char {
    let mut s: time_t = t.tv_sec;
    let mut ns: libc::c_int = t.tv_nsec as libc::c_int;
    let mut np: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut negative: bool = s < 0 as libc::c_int as libc::c_long;
    if BILLION as libc::c_int <= ns || ns < 0 as libc::c_int {
        ns = 0 as libc::c_int;
    }
    if negative as libc::c_int != 0 && ns != 0 as libc::c_int {
        s += 1;
        s;
        ns = BILLION as libc::c_int - ns;
    }
    np = umaxtostr(
        if negative as libc::c_int != 0 {
            (s as uintmax_t).wrapping_neg()
        } else {
            s as uintmax_t
        },
        sbuf.offset(1 as libc::c_int as isize),
    );
    if negative {
        np = np.offset(-1);
        *np = '-' as i32 as libc::c_char;
    }
    code_ns_fraction(
        ns,
        sbuf
            .offset(
                (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(
                        !((0 as libc::c_int as uintmax_t)
                            < -(1 as libc::c_int) as uintmax_t) as libc::c_int
                            as libc::c_ulong,
                    )
                    .wrapping_mul(146 as libc::c_int as libc::c_ulong)
                    .wrapping_add(484 as libc::c_int as libc::c_ulong)
                    .wrapping_div(485 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        !((0 as libc::c_int as uintmax_t)
                            < -(1 as libc::c_int) as uintmax_t) as libc::c_int
                            as libc::c_ulong,
                    )
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ),
    );
    return np;
}
pub unsafe extern "C" fn decode_timespec(
    mut arg: *const libc::c_char,
    mut arg_lim: *mut *mut libc::c_char,
    mut parse_fraction: bool,
) -> timespec {
    let mut s: time_t = !if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t
    {
        -(1 as libc::c_int) as time_t
    } else {
        (((1 as libc::c_int as time_t)
            << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
            - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long
    };
    let mut ns: libc::c_int = -(1 as libc::c_int);
    let mut p: *const libc::c_char = arg;
    let mut negative: bool = *arg as libc::c_int == '-' as i32;
    let mut r: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    if !((*arg.offset(negative as isize) as libc::c_uint)
        .wrapping_sub('0' as i32 as libc::c_uint) <= 9 as libc::c_int as libc::c_uint)
    {
        *__errno_location() = 22 as libc::c_int;
    } else {
        *__errno_location() = 0 as libc::c_int;
        if negative {
            let mut i: intmax_t = strtoimax(arg, arg_lim, 10 as libc::c_int);
            if if !((0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t) {
                (!(if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
                    -(1 as libc::c_int) as time_t
                } else {
                    (((1 as libc::c_int as time_t)
                        << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                }) <= i) as libc::c_int
            } else {
                (0 as libc::c_int as libc::c_long <= i) as libc::c_int
            } != 0
            {
                s = i;
            } else {
                *__errno_location() = 34 as libc::c_int;
            }
        } else {
            let mut i_0: uintmax_t = strtoumax(arg, arg_lim, 10 as libc::c_int);
            if i_0
                <= (if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
                    -(1 as libc::c_int) as time_t
                } else {
                    (((1 as libc::c_int as time_t)
                        << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                }) as libc::c_ulong
            {
                s = i_0 as time_t;
            } else {
                *__errno_location() = 34 as libc::c_int;
            }
        }
        p = *arg_lim;
        ns = 0 as libc::c_int;
        if parse_fraction as libc::c_int != 0 && *p as libc::c_int == '.' as i32 {
            let mut digits: libc::c_int = 0 as libc::c_int;
            let mut trailing_nonzero: bool = 0 as libc::c_int != 0;
            loop {
                p = p.offset(1);
                if !((*p as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                    <= 9 as libc::c_int as libc::c_uint)
                {
                    break;
                }
                if digits < LOG10_BILLION as libc::c_int {
                    digits += 1;
                    digits;
                    ns = 10 as libc::c_int * ns + (*p as libc::c_int - '0' as i32);
                } else {
                    trailing_nonzero = (trailing_nonzero as libc::c_int
                        | (*p as libc::c_int != '0' as i32) as libc::c_int) != 0;
                }
            }
            while digits < LOG10_BILLION as libc::c_int {
                digits += 1;
                digits;
                ns *= 10 as libc::c_int;
            }
            if negative {
                ns += trailing_nonzero as libc::c_int;
                if ns != 0 as libc::c_int {
                    if s
                        == !(if (0 as libc::c_int as time_t)
                            < -(1 as libc::c_int) as time_t
                        {
                            -(1 as libc::c_int) as time_t
                        } else {
                            (((1 as libc::c_int as time_t)
                                << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        })
                    {
                        ns = -(1 as libc::c_int);
                    } else {
                        s -= 1;
                        s;
                        ns = BILLION as libc::c_int - ns;
                    }
                }
            }
        }
        if *__errno_location() == 34 as libc::c_int {
            ns = -(1 as libc::c_int);
        }
    }
    *arg_lim = p as *mut libc::c_char;
    r.tv_sec = s;
    r.tv_nsec = ns as __syscall_slong_t;
    return r;
}
static mut before_backup_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut after_backup_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub unsafe extern "C" fn must_be_dot_or_slash(
    mut file_name: *const libc::c_char,
) -> bool {
    file_name = file_name.offset(0 as libc::c_int as isize);
    if *file_name.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
        loop {
            if *file_name.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
            {
                file_name = file_name.offset(1);
                file_name;
            } else if *file_name.offset(1 as libc::c_int as isize) as libc::c_int
                == '.' as i32
                && *file_name
                    .offset(
                        (2 as libc::c_int
                            + (*file_name.offset(2 as libc::c_int as isize)
                                as libc::c_int == '.' as i32) as libc::c_int) as isize,
                    ) as libc::c_int == '/' as i32
            {
                file_name = file_name
                    .offset(
                        (2 as libc::c_int
                            + (*file_name.offset(2 as libc::c_int as isize)
                                as libc::c_int == '.' as i32) as libc::c_int) as isize,
                    );
            } else {
                return *file_name.offset(1 as libc::c_int as isize) == 0
            }
        }
    } else {
        while *file_name.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && *file_name.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            file_name = file_name.offset(2 as libc::c_int as isize);
            while *file_name as libc::c_int == '/' as i32 {
                file_name = file_name.offset(1);
                file_name;
            }
        }
        return *file_name.offset(0 as libc::c_int as isize) == 0
            || *file_name.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *file_name.offset(1 as libc::c_int as isize) == 0;
    };
}
unsafe extern "C" fn safer_rmdir(mut file_name: *const libc::c_char) -> libc::c_int {
    if must_be_dot_or_slash(file_name) {
        *__errno_location() = 0 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if unlinkat(chdir_fd, file_name, 0x200 as libc::c_int) == 0 as libc::c_int {
        remove_delayed_set_stat(file_name);
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn remove_any_file(
    mut file_name: *const libc::c_char,
    mut option: remove_option,
) -> libc::c_int {
    let mut try_unlink_first: bool = 1 as libc::c_int != 0;
    if try_unlink_first {
        if unlinkat(chdir_fd, file_name, 0 as libc::c_int) == 0 as libc::c_int {
            return 1 as libc::c_int;
        }
        if *__errno_location() != 1 as libc::c_int
            && *__errno_location() != 21 as libc::c_int
        {
            return 0 as libc::c_int;
        }
    }
    if safer_rmdir(file_name) == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    match *__errno_location() {
        20 => {
            return (!try_unlink_first
                && unlinkat(chdir_fd, file_name, 0 as libc::c_int) == 0 as libc::c_int)
                as libc::c_int;
        }
        0 | 17 | 39 => {
            match option as libc::c_uint {
                2 => return -(1 as libc::c_int),
                1 => {
                    let mut directory: *mut libc::c_char = tar_savedir(
                        file_name,
                        0 as libc::c_int,
                    );
                    let mut entry: *const libc::c_char = 0 as *const libc::c_char;
                    let mut entrylen: size_t = 0;
                    if directory.is_null() {
                        return 0 as libc::c_int;
                    }
                    entry = directory;
                    loop {
                        entrylen = strlen(entry);
                        if !(entrylen != 0 as libc::c_int as libc::c_ulong) {
                            break;
                        }
                        let mut file_name_buffer: *mut libc::c_char = make_file_name(
                            file_name,
                            entry,
                        );
                        let mut r: libc::c_int = remove_any_file(
                            file_name_buffer,
                            RECURSIVE_REMOVE_OPTION,
                        );
                        let mut e: libc::c_int = *__errno_location();
                        rpl_free(file_name_buffer as *mut libc::c_void);
                        if r == 0 {
                            rpl_free(directory as *mut libc::c_void);
                            *__errno_location() = e;
                            return 0 as libc::c_int;
                        }
                        entry = entry
                            .offset(
                                entrylen.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            );
                    }
                    rpl_free(directory as *mut libc::c_void);
                    return (safer_rmdir(file_name) == 0 as libc::c_int) as libc::c_int;
                }
                0 | _ => {}
            }
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn maybe_backup_file(
    mut file_name: *const libc::c_char,
    mut this_is_the_archive: bool,
) -> bool {
    let mut file_stat: stat = stat {
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
    assign_string(&mut before_backup_name, file_name);
    assign_string(&mut after_backup_name, 0 as *const libc::c_char);
    if this_is_the_archive as libc::c_int != 0
        && (!force_local_option
            && {
                rmt_dev_name__ = strchr(file_name, ':' as i32);
                !rmt_dev_name__.is_null()
            } && rmt_dev_name__ > file_name
            && (memchr(
                file_name as *const libc::c_void,
                '/' as i32,
                rmt_dev_name__.offset_from(file_name) as libc::c_long as libc::c_ulong,
            ))
                .is_null())
    {
        return 1 as libc::c_int != 0;
    }
    if deref_stat(file_name, &mut file_stat) != 0 as libc::c_int {
        if *__errno_location() == 2 as libc::c_int {
            return 1 as libc::c_int != 0;
        }
        stat_error(file_name);
        return 0 as libc::c_int != 0;
    }
    if file_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int != 0;
    }
    if this_is_the_archive as libc::c_int != 0
        && (file_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o60000 as libc::c_int as libc::c_uint
            || file_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o20000 as libc::c_int as libc::c_uint)
    {
        return 1 as libc::c_int != 0;
    }
    after_backup_name = find_backup_file_name(chdir_fd, file_name, backup_type);
    if after_backup_name.is_null() {
        xalloc_die();
    }
    if renameat(chdir_fd, before_backup_name, chdir_fd, after_backup_name)
        == 0 as libc::c_int
    {
        if verbose_option != 0 {
            fprintf(
                stdlis,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Renaming %s to %s\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote_n(0 as libc::c_int, before_backup_name),
                quote_n(1 as libc::c_int, after_backup_name),
            );
        }
        return 1 as libc::c_int != 0;
    } else {
        let mut e: libc::c_int = *__errno_location();
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            e,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Cannot rename to %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_colon(before_backup_name),
            quote_n(1 as libc::c_int, after_backup_name),
        );
        exit_status = 2 as libc::c_int;
        assign_string(&mut after_backup_name, 0 as *const libc::c_char);
        return 0 as libc::c_int != 0;
    };
}
pub unsafe extern "C" fn undo_last_backup() {
    if !after_backup_name.is_null() {
        if renameat(chdir_fd, after_backup_name, chdir_fd, before_backup_name)
            != 0 as libc::c_int
        {
            let mut e: libc::c_int = *__errno_location();
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                e,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Cannot rename to %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_colon(after_backup_name),
                quote_n(1 as libc::c_int, before_backup_name),
            );
            exit_status = 2 as libc::c_int;
        }
        if verbose_option != 0 {
            fprintf(
                stdlis,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Renaming %s back to %s\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote_n(0 as libc::c_int, after_backup_name),
                quote_n(1 as libc::c_int, before_backup_name),
            );
        }
        assign_string(&mut after_backup_name, 0 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn deref_stat(
    mut name: *const libc::c_char,
    mut buf: *mut stat,
) -> libc::c_int {
    return fstatat(chdir_fd, name, buf, fstatat_flags);
}
pub unsafe extern "C" fn blocking_read(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_void,
    mut count: size_t,
) -> size_t {
    let mut bytes: size_t = safe_read(fd, buf, count);
    if bytes == -(1 as libc::c_int) as size_t && *__errno_location() == 11 as libc::c_int
    {
        let mut flags: libc::c_int = rpl_fcntl(fd, 3 as libc::c_int);
        if 0 as libc::c_int <= flags && flags & 0o4000 as libc::c_int != 0
            && rpl_fcntl(fd, 4 as libc::c_int, flags & !(0o4000 as libc::c_int))
                != -(1 as libc::c_int)
        {
            bytes = safe_read(fd, buf, count);
        }
    }
    return bytes;
}
pub unsafe extern "C" fn blocking_write(
    mut fd: libc::c_int,
    mut buf: *const libc::c_void,
    mut count: size_t,
) -> size_t {
    let mut bytes: size_t = full_write(fd, buf, count);
    if bytes < count && *__errno_location() == 11 as libc::c_int {
        let mut flags: libc::c_int = rpl_fcntl(fd, 3 as libc::c_int);
        if 0 as libc::c_int <= flags && flags & 0o4000 as libc::c_int != 0
            && rpl_fcntl(fd, 4 as libc::c_int, flags & !(0o4000 as libc::c_int))
                != -(1 as libc::c_int)
        {
            let mut buffer: *const libc::c_char = buf as *const libc::c_char;
            bytes = (bytes as libc::c_ulong)
                .wrapping_add(
                    full_write(
                        fd,
                        buffer.offset(bytes as isize) as *const libc::c_void,
                        count.wrapping_sub(bytes),
                    ),
                ) as size_t as size_t;
        }
    }
    return bytes;
}
pub unsafe extern "C" fn set_file_atime(
    mut fd: libc::c_int,
    mut parentfd: libc::c_int,
    mut file: *const libc::c_char,
    mut atime: timespec,
) -> libc::c_int {
    let mut ts: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    ts[0 as libc::c_int as usize] = atime;
    ts[1 as libc::c_int as usize]
        .tv_nsec = ((1 as libc::c_long) << 30 as libc::c_int) - 2 as libc::c_long;
    return fdutimensat(
        fd,
        parentfd,
        file,
        ts.as_mut_ptr() as *const timespec,
        fstatat_flags,
    );
}
static mut wd: *mut wd = 0 as *const wd as *mut wd;
static mut wd_count: size_t = 0;
static mut wd_alloc: size_t = 0;
static mut wdcache: [libc::c_int; 16] = [0; 16];
static mut wdcache_count: size_t = 0;
pub unsafe extern "C" fn chdir_count() -> libc::c_int {
    if wd_count == 0 as libc::c_int as libc::c_ulong {
        return wd_count as libc::c_int;
    }
    return wd_count.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
}
pub unsafe extern "C" fn chdir_arg(mut dir: *const libc::c_char) -> libc::c_int {
    if wd_count == wd_alloc {
        if wd_alloc == 0 as libc::c_int as libc::c_ulong {
            wd_alloc = 2 as libc::c_int as size_t;
        }
        wd = x2nrealloc(
            wd as *mut libc::c_void,
            &mut wd_alloc,
            ::std::mem::size_of::<wd>() as libc::c_ulong,
        ) as *mut wd;
        if wd_count == 0 {
            let ref mut fresh24 = (*wd.offset(wd_count as isize)).name;
            *fresh24 = b".\0" as *const u8 as *const libc::c_char;
            let ref mut fresh25 = (*wd.offset(wd_count as isize)).abspath;
            *fresh25 = 0 as *mut libc::c_char;
            (*wd.offset(wd_count as isize)).fd = -(100 as libc::c_int);
            wd_count = wd_count.wrapping_add(1);
            wd_count;
        }
    }
    if *dir.offset(0 as libc::c_int as isize) != 0 {
        while *dir.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && *dir.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            dir = dir.offset(2 as libc::c_int as isize);
            while *dir as libc::c_int == '/' as i32 {
                dir = dir.offset(1);
                dir;
            }
        }
        if *dir
            .offset(
                (*dir.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32)
                    as libc::c_int as isize,
            ) == 0
        {
            return wd_count.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                as libc::c_int;
        }
    }
    let ref mut fresh26 = (*wd.offset(wd_count as isize)).name;
    *fresh26 = dir;
    let ref mut fresh27 = (*wd.offset(wd_count as isize)).abspath;
    *fresh27 = 0 as *mut libc::c_char;
    (*wd.offset(wd_count as isize)).fd = 0 as libc::c_int;
    let fresh28 = wd_count;
    wd_count = wd_count.wrapping_add(1);
    return fresh28 as libc::c_int;
}
pub static mut chdir_current: libc::c_int = 0;
pub static mut chdir_fd: libc::c_int = -(100 as libc::c_int);
pub unsafe extern "C" fn chdir_do(mut i: libc::c_int) {
    if chdir_current != i {
        let mut curr: *mut wd = &mut *wd.offset(i as isize) as *mut wd;
        let mut fd: libc::c_int = (*curr).fd;
        if fd == 0 {
            if !(*((*curr).name).offset(0 as libc::c_int as isize) as libc::c_int
                == '/' as i32)
            {
                chdir_do(i - 1 as libc::c_int);
            }
            fd = openat(
                chdir_fd,
                (*curr).name,
                open_searchdir_flags & !(0o400000 as libc::c_int),
            );
            if fd < 0 as libc::c_int {
                open_fatal((*curr).name);
            }
            (*curr).fd = fd;
            if wdcache_count < CHDIR_CACHE_SIZE as libc::c_int as libc::c_ulong {
                let fresh29 = wdcache_count;
                wdcache_count = wdcache_count.wrapping_add(1);
                wdcache[fresh29 as usize] = i;
            } else {
                let mut stale: *mut wd = &mut *wd
                    .offset(
                        *wdcache
                            .as_mut_ptr()
                            .offset(
                                (CHDIR_CACHE_SIZE as libc::c_int - 1 as libc::c_int)
                                    as isize,
                            ) as isize,
                    ) as *mut wd;
                if close((*stale).fd) != 0 as libc::c_int {
                    close_diag((*stale).name);
                }
                (*stale).fd = 0 as libc::c_int;
                wdcache[(CHDIR_CACHE_SIZE as libc::c_int - 1 as libc::c_int)
                    as usize] = i;
            }
        }
        if (0 as libc::c_int) < fd {
            let mut ci: size_t = 0;
            let mut prev: libc::c_int = wdcache[0 as libc::c_int as usize];
            ci = 1 as libc::c_int as size_t;
            while prev != i {
                let mut cur: libc::c_int = wdcache[ci as usize];
                wdcache[ci as usize] = prev;
                if cur == i {
                    break;
                }
                prev = cur;
                ci = ci.wrapping_add(1);
                ci;
            }
            wdcache[0 as libc::c_int as usize] = i;
        }
        chdir_current = i;
        chdir_fd = fd;
    }
}
pub unsafe extern "C" fn tar_dirname() -> *const libc::c_char {
    return (*wd.offset(chdir_current as isize)).name;
}
unsafe extern "C" fn tar_getcdpath(mut idx: libc::c_int) -> *const libc::c_char {
    if wd.is_null() {
        static mut cwd: *mut libc::c_char = 0 as *const libc::c_char
            as *mut libc::c_char;
        if cwd.is_null() {
            cwd = xgetcwd();
            if cwd.is_null() {
                call_arg_fatal(
                    b"getcwd\0" as *const u8 as *const libc::c_char,
                    b".\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        return cwd;
    }
    if ((*wd.offset(idx as isize)).abspath).is_null() {
        let mut i: libc::c_int = 0;
        let mut save_cwdi: libc::c_int = chdir_current;
        i = idx;
        while i >= 0 as libc::c_int {
            if !((*wd.offset(i as isize)).abspath).is_null() {
                break;
            }
            i -= 1;
            i;
        }
        loop {
            i += 1;
            if !(i <= idx) {
                break;
            }
            chdir_do(i);
            if i == 0 as libc::c_int {
                let ref mut fresh30 = (*wd.offset(i as isize)).abspath;
                *fresh30 = xgetcwd();
                if (*fresh30).is_null() {
                    call_arg_fatal(
                        b"getcwd\0" as *const u8 as *const libc::c_char,
                        b".\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else if *((*wd.offset(i as isize)).name).offset(0 as libc::c_int as isize)
                as libc::c_int == '/' as i32
            {
                let ref mut fresh31 = (*wd.offset(i as isize)).abspath;
                *fresh31 = xstrdup((*wd.offset(i as isize)).name);
            } else {
                let mut nbuf: namebuf_t = namebuf_create(
                    (*wd.offset((i - 1 as libc::c_int) as isize)).abspath,
                );
                namebuf_add_dir(nbuf, (*wd.offset(i as isize)).name);
                let ref mut fresh32 = (*wd.offset(i as isize)).abspath;
                *fresh32 = namebuf_finish(nbuf);
            }
        }
        chdir_do(save_cwdi);
    }
    return (*wd.offset(idx as isize)).abspath;
}
pub unsafe extern "C" fn close_diag(mut name: *const libc::c_char) {
    if ignore_failed_read_option {
        if warning_option & 0x800000 as libc::c_int != 0 {
            close_warn(name);
        }
    } else {
        close_error(name);
    };
}
pub unsafe extern "C" fn open_diag(mut name: *const libc::c_char) {
    if ignore_failed_read_option {
        if warning_option & 0x800000 as libc::c_int != 0 {
            open_warn(name);
        }
    } else {
        open_error(name);
    };
}
pub unsafe extern "C" fn read_diag_details(
    mut name: *const libc::c_char,
    mut offset: off_t,
    mut size: size_t,
) {
    if ignore_failed_read_option {
        if warning_option & 0x800000 as libc::c_int != 0 {
            read_warn_details(name, offset, size);
        }
    } else {
        read_error_details(name, offset, size);
    };
}
pub unsafe extern "C" fn readlink_diag(mut name: *const libc::c_char) {
    if ignore_failed_read_option {
        if warning_option & 0x800000 as libc::c_int != 0 {
            readlink_warn(name);
        }
    } else {
        readlink_error(name);
    };
}
pub unsafe extern "C" fn savedir_diag(mut name: *const libc::c_char) {
    if ignore_failed_read_option {
        if warning_option & 0x800000 as libc::c_int != 0 {
            savedir_warn(name);
        }
    } else {
        savedir_error(name);
    };
}
pub unsafe extern "C" fn seek_diag_details(
    mut name: *const libc::c_char,
    mut offset: off_t,
) {
    if ignore_failed_read_option {
        if warning_option & 0x800000 as libc::c_int != 0 {
            seek_warn_details(name, offset);
        }
    } else {
        seek_error_details(name, offset);
    };
}
pub unsafe extern "C" fn stat_diag(mut name: *const libc::c_char) {
    if ignore_failed_read_option {
        if warning_option & 0x800000 as libc::c_int != 0 {
            stat_warn(name);
        }
    } else {
        stat_error(name);
    };
}
pub unsafe extern "C" fn file_removed_diag(
    mut name: *const libc::c_char,
    mut top_level: bool,
    mut diagfn: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
) {
    if !top_level && *__errno_location() == 2 as libc::c_int {
        if warning_option & 0x40 as libc::c_int != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: File removed before we read it\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_colon(name),
            );
        }
        set_exit_status(1 as libc::c_int);
    } else {
        diagfn.unwrap()(name);
    };
}
pub unsafe extern "C" fn xfork() -> pid_t {
    let mut p: pid_t = fork();
    if p == -(1 as libc::c_int) {
        call_arg_fatal(
            b"fork\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"child process\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    return p;
}
pub unsafe extern "C" fn xpipe(mut fd: *mut libc::c_int) {
    if pipe(fd) < 0 as libc::c_int {
        call_arg_fatal(
            b"pipe\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"interprocess channel\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
#[inline]
unsafe extern "C" fn ptr_align(
    mut ptr: *mut libc::c_void,
    mut alignment: size_t,
) -> *mut libc::c_void {
    let mut p0: *mut libc::c_char = ptr as *mut libc::c_char;
    let mut p1: *mut libc::c_char = p0
        .offset(alignment as isize)
        .offset(-(1 as libc::c_int as isize));
    return p1.offset(-((p1 as size_t).wrapping_rem(alignment) as isize))
        as *mut libc::c_void;
}
pub unsafe extern "C" fn page_aligned_alloc(
    mut ptr: *mut *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut alignment: size_t = getpagesize() as size_t;
    let mut size1: size_t = size.wrapping_add(alignment);
    if size1 < size {
        xalloc_die();
    }
    *ptr = xmalloc(size1);
    return ptr_align(*ptr, alignment);
}
pub unsafe extern "C" fn namebuf_create(mut dir: *const libc::c_char) -> namebuf_t {
    let mut buf: namebuf_t = xmalloc(::std::mem::size_of::<namebuf>() as libc::c_ulong)
        as namebuf_t;
    (*buf).buffer_size = (strlen(dir)).wrapping_add(2 as libc::c_int as libc::c_ulong);
    (*buf).buffer = xmalloc((*buf).buffer_size) as *mut libc::c_char;
    strcpy((*buf).buffer, dir);
    (*buf).dir_length = strlen((*buf).buffer);
    if !(*((*buf).buffer)
        .offset(
            ((*buf).dir_length).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) as libc::c_int == '/' as i32)
    {
        let fresh33 = (*buf).dir_length;
        (*buf).dir_length = ((*buf).dir_length).wrapping_add(1);
        *((*buf).buffer).offset(fresh33 as isize) = '/' as i32 as libc::c_char;
    }
    return buf;
}
pub unsafe extern "C" fn namebuf_free(mut buf: namebuf_t) {
    rpl_free((*buf).buffer as *mut libc::c_void);
    rpl_free(buf as *mut libc::c_void);
}
pub unsafe extern "C" fn namebuf_name(
    mut buf: namebuf_t,
    mut name: *const libc::c_char,
) -> *mut libc::c_char {
    let mut len: size_t = strlen(name);
    while ((*buf).dir_length)
        .wrapping_add(len)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) >= (*buf).buffer_size
    {
        (*buf)
            .buffer = x2realloc(
            (*buf).buffer as *mut libc::c_void,
            &mut (*buf).buffer_size,
        ) as *mut libc::c_char;
    }
    strcpy(((*buf).buffer).offset((*buf).dir_length as isize), name);
    return (*buf).buffer;
}
unsafe extern "C" fn namebuf_add_dir(mut buf: namebuf_t, mut name: *const libc::c_char) {
    static mut dirsep: [libc::c_char; 2] = [
        '/' as i32 as libc::c_char,
        0 as libc::c_int as libc::c_char,
    ];
    if !(*((*buf).buffer)
        .offset(
            ((*buf).dir_length).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) as libc::c_int == '/' as i32)
    {
        namebuf_name(buf, dirsep.as_mut_ptr());
        (*buf).dir_length = ((*buf).dir_length).wrapping_add(1);
        (*buf).dir_length;
    }
    namebuf_name(buf, name);
    (*buf)
        .dir_length = ((*buf).dir_length as libc::c_ulong).wrapping_add(strlen(name))
        as size_t as size_t;
}
unsafe extern "C" fn namebuf_finish(mut buf: namebuf_t) -> *mut libc::c_char {
    let mut res: *mut libc::c_char = (*buf).buffer;
    if *((*buf).buffer)
        .offset(
            ((*buf).dir_length).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) as libc::c_int == '/' as i32
    {
        *((*buf).buffer)
            .offset((*buf).dir_length as isize) = 0 as libc::c_int as libc::c_char;
    }
    rpl_free(buf as *mut libc::c_void);
    return res;
}
pub unsafe extern "C" fn tar_savedir(
    mut name: *const libc::c_char,
    mut must_exist: libc::c_int,
) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut fd: libc::c_int = openat(
        chdir_fd,
        name,
        open_read_flags | 0o200000 as libc::c_int,
    );
    if fd < 0 as libc::c_int {
        if must_exist == 0 && *__errno_location() == 2 as libc::c_int {
            return 0 as *mut libc::c_char;
        }
        open_error(name);
    } else {
        dir = fdopendir(fd);
        if !(!dir.is_null()
            && {
                ret = streamsavedir(dir, savedir_sort_order as savedir_option);
                !ret.is_null()
            })
        {
            savedir_error(name);
        }
    }
    if if !dir.is_null() {
        (closedir(dir) != 0 as libc::c_int) as libc::c_int
    } else {
        (0 as libc::c_int <= fd && close(fd) != 0 as libc::c_int) as libc::c_int
    } != 0
    {
        savedir_error(name);
    }
    return ret;
}
