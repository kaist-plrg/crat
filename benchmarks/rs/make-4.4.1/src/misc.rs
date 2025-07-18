use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type variable_set_list;
    pub type commands;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    static mut stdout: *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn __errno_location() -> *mut libc::c_int;
    fn getpid() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn out_of_memory() -> !;
    fn error(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...);
    static mut stopchar_map: [libc::c_ushort; 0];
    static mut posix_pedantic: libc::c_int;
    fn os_anontmp() -> libc::c_int;
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
pub type size_t = libc::c_ulong;
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
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type mode_t = __mode_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
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
pub type va_list = __builtin_va_list;
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
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct file {
    pub name: *const libc::c_char,
    pub hname: *const libc::c_char,
    pub vpath: *const libc::c_char,
    pub deps: *mut dep,
    pub cmds: *mut commands,
    pub stem: *const libc::c_char,
    pub also_make: *mut dep,
    pub prev: *mut file,
    pub last: *mut file,
    pub renamed: *mut file,
    pub variables: *mut variable_set_list,
    pub pat_variables: *mut variable_set_list,
    pub parent: *mut file,
    pub double_colon: *mut file,
    pub last_mtime: uintmax_t,
    pub mtime_before_update: uintmax_t,
    pub considered: libc::c_uint,
    pub command_flags: libc::c_int,
    #[bitfield(name = "update_status", ty = "update_status", bits = "0..=1")]
    #[bitfield(name = "command_state", ty = "cmd_state", bits = "2..=3")]
    #[bitfield(name = "builtin", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "precious", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "loaded", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "unloaded", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "low_resolution_time", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "tried_implicit", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "updating", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "updated", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "is_target", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "cmd_target", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "phony", ty = "libc::c_uint", bits = "14..=14")]
    #[bitfield(name = "intermediate", ty = "libc::c_uint", bits = "15..=15")]
    #[bitfield(name = "is_explicit", ty = "libc::c_uint", bits = "16..=16")]
    #[bitfield(name = "secondary", ty = "libc::c_uint", bits = "17..=17")]
    #[bitfield(name = "notintermediate", ty = "libc::c_uint", bits = "18..=18")]
    #[bitfield(name = "dontcare", ty = "libc::c_uint", bits = "19..=19")]
    #[bitfield(name = "ignore_vpath", ty = "libc::c_uint", bits = "20..=20")]
    #[bitfield(name = "pat_searched", ty = "libc::c_uint", bits = "21..=21")]
    #[bitfield(name = "no_diag", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "was_shuffled", ty = "libc::c_uint", bits = "23..=23")]
    #[bitfield(name = "snapped", ty = "libc::c_uint", bits = "24..=24")]
    pub update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
pub type cmd_state = libc::c_uint;
pub const cs_finished: cmd_state = 3;
pub const cs_running: cmd_state = 2;
pub const cs_deps_running: cmd_state = 1;
pub const cs_not_started: cmd_state = 0;
pub type update_status = libc::c_uint;
pub const us_failed: update_status = 3;
pub const us_question: update_status = 2;
pub const us_none: update_status = 1;
pub const us_success: update_status = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct dep {
    pub next: *mut dep,
    pub name: *const libc::c_char,
    pub file: *mut file,
    pub shuf: *mut dep,
    pub stem: *const libc::c_char,
    #[bitfield(name = "flags", ty = "libc::c_uint", bits = "0..=7")]
    #[bitfield(name = "changed", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "ignore_mtime", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "staticpattern", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "need_2nd_expansion", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "ignore_automatic_vars", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "is_explicit", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "wait_here", ty = "libc::c_uint", bits = "14..=14")]
    pub flags_changed_ignore_mtime_staticpattern_need_2nd_expansion_ignore_automatic_vars_is_explicit_wait_here: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const libc::c_char,
    pub lineno: libc::c_ulong,
    pub offset: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const libc::c_char,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
pub unsafe extern "C" fn make_toui(
    mut str: *const libc::c_char,
    mut error_0: *mut *const libc::c_char,
) -> libc::c_uint {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: libc::c_ulong = strtoul(str, &mut end, 10 as libc::c_int);
    if !error_0.is_null() {
        if *str.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
            *error_0 = b"Missing value\0" as *const u8 as *const libc::c_char;
        } else if *end as libc::c_int != '\0' as i32 {
            *error_0 = b"Invalid value\0" as *const u8 as *const libc::c_char;
        } else {
            *error_0 = 0 as *const libc::c_char;
        }
    }
    return val as libc::c_uint;
}
pub unsafe extern "C" fn make_lltoa(
    mut val: libc::c_longlong,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    sprintf(buf, b"%lld\0" as *const u8 as *const libc::c_char, val);
    return buf;
}
pub unsafe extern "C" fn make_ulltoa(
    mut val: libc::c_ulonglong,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    sprintf(buf, b"%llu\0" as *const u8 as *const libc::c_char, val);
    return buf;
}
static mut mk_state: libc::c_uint = 0 as libc::c_int as libc::c_uint;
pub unsafe extern "C" fn make_seed(mut seed: libc::c_uint) {
    mk_state = seed;
}
pub unsafe extern "C" fn make_rand() -> libc::c_uint {
    if mk_state == 0 as libc::c_int as libc::c_uint {
        mk_state = ((time(0 as *mut time_t) ^ make_pid() as libc::c_long)
            as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint);
    }
    mk_state ^= mk_state << 13 as libc::c_int;
    mk_state ^= mk_state >> 17 as libc::c_int;
    mk_state ^= mk_state << 5 as libc::c_int;
    return mk_state;
}
pub unsafe extern "C" fn alpha_compare(
    mut v1: *const libc::c_void,
    mut v2: *const libc::c_void,
) -> libc::c_int {
    let mut s1: *const libc::c_char = *(v1 as *mut *mut libc::c_char);
    let mut s2: *const libc::c_char = *(v2 as *mut *mut libc::c_char);
    if *s1 as libc::c_int != *s2 as libc::c_int {
        return *s1 as libc::c_int - *s2 as libc::c_int;
    }
    return strcmp(s1, s2);
}
pub unsafe extern "C" fn collapse_continuations(mut line: *mut libc::c_char) {
    let mut out: *mut libc::c_char = line;
    let mut in_0: *mut libc::c_char = line;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    q = strchr(in_0, '\n' as i32);
    if q.is_null() {
        return;
    }
    loop {
        let mut p: *mut libc::c_char = q;
        let mut i: libc::c_int = 0;
        let mut out_line_length: size_t = 0;
        if q > line
            && *q.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\\' as i32
        {
            i = -(2 as libc::c_int);
            while &mut *p.offset(i as isize) as *mut libc::c_char >= line
                && *p.offset(i as isize) as libc::c_int == '\\' as i32
            {
                i -= 1;
                i;
            }
            i += 1;
            i;
        } else {
            i = 0 as libc::c_int;
        }
        out_line_length = (p.offset_from(in_0) as libc::c_long + i as libc::c_long
            - (i / 2 as libc::c_int) as libc::c_long) as size_t;
        if out != in_0 {
            memmove(
                out as *mut libc::c_void,
                in_0 as *const libc::c_void,
                out_line_length,
            );
        }
        out = out.offset(out_line_length as isize);
        in_0 = q.offset(1 as libc::c_int as isize);
        if i & 1 as libc::c_int != 0 {
            while *stopchar_map.as_mut_ptr().offset(*in_0 as libc::c_uchar as isize)
                as libc::c_int & 0x2 as libc::c_int != 0 as libc::c_int
            {
                in_0 = in_0.offset(1);
                in_0;
            }
            if posix_pedantic == 0 {
                while out > line
                    && *stopchar_map
                        .as_mut_ptr()
                        .offset(
                            *out.offset(-(1 as libc::c_int) as isize) as libc::c_uchar
                                as isize,
                        ) as libc::c_int & 0x2 as libc::c_int != 0 as libc::c_int
                {
                    out = out.offset(-1);
                    out;
                }
            }
            let fresh0 = out;
            out = out.offset(1);
            *fresh0 = ' ' as i32 as libc::c_char;
        } else {
            let fresh1 = out;
            out = out.offset(1);
            *fresh1 = '\n' as i32 as libc::c_char;
        }
        q = strchr(in_0, '\n' as i32);
        if q.is_null() {
            break;
        }
    }
    memmove(
        out as *mut libc::c_void,
        in_0 as *const libc::c_void,
        (strlen(in_0)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
}
pub unsafe extern "C" fn print_spaces(mut n: libc::c_uint) {
    loop {
        let fresh2 = n;
        n = n.wrapping_sub(1);
        if !(fresh2 > 0 as libc::c_int as libc::c_uint) {
            break;
        }
        putchar(' ' as i32);
    };
}
pub unsafe extern "C" fn concat(
    mut num: libc::c_uint,
    mut args: ...
) -> *const libc::c_char {
    static mut rlen: size_t = 0 as libc::c_int as size_t;
    static mut result: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    let mut ri: size_t = 0 as libc::c_int as size_t;
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    loop {
        let fresh3 = num;
        num = num.wrapping_sub(1);
        if !(fresh3 > 0 as libc::c_int as libc::c_uint) {
            break;
        }
        let mut s: *const libc::c_char = args_0.arg::<*const libc::c_char>();
        let mut l: size_t = if s.is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(s)
        };
        if l == 0 as libc::c_int as libc::c_ulong {
            continue;
        }
        if ri.wrapping_add(l) > rlen {
            rlen = (if rlen != 0 { rlen } else { 60 as libc::c_int as libc::c_ulong })
                .wrapping_add(l)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong);
            result = xrealloc(result as *mut libc::c_void, rlen) as *mut libc::c_char;
        }
        memcpy(
            result.offset(ri as isize) as *mut libc::c_void,
            s as *const libc::c_void,
            l,
        );
        ri = (ri as libc::c_ulong).wrapping_add(l) as size_t as size_t;
    }
    if ri == rlen {
        rlen = (if rlen != 0 { rlen } else { 60 as libc::c_int as libc::c_ulong })
            .wrapping_mul(2 as libc::c_int as libc::c_ulong);
        result = xrealloc(result as *mut libc::c_void, rlen) as *mut libc::c_char;
    }
    *result.offset(ri as isize) = '\0' as i32 as libc::c_char;
    return result;
}
pub unsafe extern "C" fn make_pid() -> pid_t {
    return getpid();
}
pub unsafe extern "C" fn xmalloc(mut size: size_t) -> *mut libc::c_void {
    let mut result: *mut libc::c_void = malloc(
        if size != 0 { size } else { 1 as libc::c_int as libc::c_ulong },
    );
    if result.is_null() {
        out_of_memory();
    }
    return result;
}
pub unsafe extern "C" fn xcalloc(mut size: size_t) -> *mut libc::c_void {
    let mut result: *mut libc::c_void = calloc(
        if size != 0 { size } else { 1 as libc::c_int as libc::c_ulong },
        1 as libc::c_int as libc::c_ulong,
    );
    if result.is_null() {
        out_of_memory();
    }
    return result;
}
pub unsafe extern "C" fn xrealloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut result: *mut libc::c_void = 0 as *mut libc::c_void;
    if size == 0 {
        size = 1 as libc::c_int as size_t;
    }
    result = if !ptr.is_null() { realloc(ptr, size) } else { malloc(size) };
    if result.is_null() {
        out_of_memory();
    }
    return result;
}
pub unsafe extern "C" fn xstrdup(mut ptr: *const libc::c_char) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    result = strdup(ptr);
    if result.is_null() {
        out_of_memory();
    }
    return result;
}
pub unsafe extern "C" fn xstrndup(
    mut str: *const libc::c_char,
    mut length: size_t,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    result = strndup(str, length);
    if result.is_null() {
        out_of_memory();
    }
    return result;
}
pub unsafe extern "C" fn lindex(
    mut s: *const libc::c_char,
    mut limit: *const libc::c_char,
    mut c: libc::c_int,
) -> *mut libc::c_char {
    while s < limit {
        let fresh4 = s;
        s = s.offset(1);
        if *fresh4 as libc::c_int == c {
            return s.offset(-(1 as libc::c_int as isize)) as *mut libc::c_char;
        }
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn end_of_token(mut s: *const libc::c_char) -> *mut libc::c_char {
    while !(*stopchar_map.as_mut_ptr().offset(*s as libc::c_uchar as isize)
        as libc::c_int & (0x2 as libc::c_int | 0x4 as libc::c_int | 0x1 as libc::c_int)
        != 0 as libc::c_int)
    {
        s = s.offset(1);
        s;
    }
    return s as *mut libc::c_char;
}
pub unsafe extern "C" fn next_token(mut s: *const libc::c_char) -> *mut libc::c_char {
    while *stopchar_map.as_mut_ptr().offset(*s as libc::c_uchar as isize) as libc::c_int
        & (0x2 as libc::c_int | 0x4 as libc::c_int) != 0 as libc::c_int
    {
        s = s.offset(1);
        s;
    }
    return s as *mut libc::c_char;
}
pub unsafe extern "C" fn find_next_token(
    mut ptr: *mut *const libc::c_char,
    mut lengthptr: *mut size_t,
) -> *mut libc::c_char {
    let mut p: *const libc::c_char = next_token(*ptr);
    if *p as libc::c_int == '\0' as i32 {
        return 0 as *mut libc::c_char;
    }
    *ptr = end_of_token(p);
    if !lengthptr.is_null() {
        *lengthptr = (*ptr).offset_from(p) as libc::c_long as size_t;
    }
    return p as *mut libc::c_char;
}
pub unsafe extern "C" fn writebuf(
    mut fd: libc::c_int,
    mut buffer: *const libc::c_void,
    mut len: size_t,
) -> ssize_t {
    let mut msg: *const libc::c_char = buffer as *const libc::c_char;
    let mut l: size_t = len;
    while l != 0 {
        let mut r: ssize_t = 0;
        loop {
            r = write(fd, msg as *const libc::c_void, l);
            if !(r == -(1 as libc::c_int) as libc::c_long
                && *__errno_location() == 4 as libc::c_int)
            {
                break;
            }
        }
        if r < 0 as libc::c_int as libc::c_long {
            return r;
        }
        l = (l as libc::c_ulong).wrapping_sub(r as libc::c_ulong) as size_t as size_t;
        msg = msg.offset(r as isize);
    }
    return len as ssize_t;
}
pub unsafe extern "C" fn readbuf(
    mut fd: libc::c_int,
    mut buffer: *mut libc::c_void,
    mut len: size_t,
) -> ssize_t {
    let mut msg: *mut libc::c_char = buffer as *mut libc::c_char;
    while len != 0 {
        let mut r: ssize_t = 0;
        loop {
            r = read(fd, msg as *mut libc::c_void, len);
            if !(r == -(1 as libc::c_int) as libc::c_long
                && *__errno_location() == 4 as libc::c_int)
            {
                break;
            }
        }
        if r < 0 as libc::c_int as libc::c_long {
            return r;
        }
        if r == 0 as libc::c_int as libc::c_long {
            break;
        }
        len = (len as libc::c_ulong).wrapping_sub(r as libc::c_ulong) as size_t
            as size_t;
        msg = msg.offset(r as isize);
    }
    return msg.offset_from(buffer as *mut libc::c_char) as libc::c_long;
}
pub unsafe extern "C" fn copy_dep_chain(mut d: *const dep) -> *mut dep {
    let mut firstnew: *mut dep = 0 as *mut dep;
    let mut lastnew: *mut dep = 0 as *mut dep;
    while !d.is_null() {
        let mut c: *mut dep = xmalloc(::std::mem::size_of::<dep>() as libc::c_ulong)
            as *mut dep;
        memcpy(
            c as *mut libc::c_void,
            d as *const libc::c_void,
            ::std::mem::size_of::<dep>() as libc::c_ulong,
        );
        if (*c).need_2nd_expansion() != 0 {
            (*c).name = xstrdup((*c).name);
        }
        (*c).next = 0 as *mut dep;
        if firstnew.is_null() {
            lastnew = c;
            firstnew = lastnew;
        } else {
            (*lastnew).next = c;
            lastnew = (*lastnew).next;
        }
        d = (*d).next;
    }
    return firstnew;
}
pub unsafe extern "C" fn free_ns_chain(mut ns: *mut nameseq) {
    while !ns.is_null() {
        let mut t: *mut nameseq = ns;
        ns = (*ns).next;
        free(t as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn get_tmpdir() -> *const libc::c_char {
    static mut tmpdir: *const libc::c_char = 0 as *const libc::c_char;
    if tmpdir.is_null() {
        let mut tlist: [*const libc::c_char; 3] = [
            b"MAKE_TMPDIR\0" as *const u8 as *const libc::c_char,
            b"TMPDIR\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
        ];
        let mut tp: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
        let mut found: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        tp = tlist.as_mut_ptr();
        while !(*tp).is_null() {
            tmpdir = getenv(*tp);
            if !tmpdir.is_null() && *tmpdir as libc::c_int != '\0' as i32 {
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
                let mut r: libc::c_int = 0;
                found = 1 as libc::c_int as libc::c_uint;
                loop {
                    r = stat(tmpdir, &mut st);
                    if !(r == -(1 as libc::c_int)
                        && *__errno_location() == 4 as libc::c_int)
                    {
                        break;
                    }
                }
                if r < 0 as libc::c_int {
                    error(
                        0 as *mut floc,
                        (strlen(*tp))
                            .wrapping_add(strlen(tmpdir))
                            .wrapping_add(strlen(strerror(*__errno_location()))),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s value %s: %s\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *tp,
                        tmpdir,
                        strerror(*__errno_location()),
                    );
                } else if !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint)
                {
                    error(
                        0 as *mut floc,
                        (strlen(*tp)).wrapping_add(strlen(tmpdir)),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s value %s: not a directory\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *tp,
                        tmpdir,
                    );
                } else {
                    return tmpdir
                }
            }
            tp = tp.offset(1);
            tp;
        }
        tmpdir = b"/tmp\0" as *const u8 as *const libc::c_char;
        if found != 0 {
            error(
                0 as *mut floc,
                strlen(tmpdir),
                dcgettext(
                    0 as *const libc::c_char,
                    b"using default temporary directory '%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                tmpdir,
            );
        }
    }
    return tmpdir;
}
unsafe extern "C" fn get_tmptemplate() -> *mut libc::c_char {
    let mut tmpdir: *const libc::c_char = get_tmpdir();
    let mut template: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    template = xmalloc(
        (strlen(tmpdir))
            .wrapping_add(
                (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    cp = stpcpy(template, tmpdir);
    if !(*stopchar_map
        .as_mut_ptr()
        .offset(*cp.offset(-(1 as libc::c_int) as isize) as libc::c_uchar as isize)
        as libc::c_int & 0x8000 as libc::c_int != 0 as libc::c_int)
    {
        let fresh5 = cp;
        cp = cp.offset(1);
        *fresh5 = '/' as i32 as libc::c_char;
    }
    strcpy(cp, b"GmXXXXXX\0" as *const u8 as *const libc::c_char);
    return template;
}
pub unsafe extern "C" fn get_tmpfd(mut name: *mut *mut libc::c_char) -> libc::c_int {
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut tmpnm: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mask: mode_t = 0;
    if !name.is_null() {
        *name = 0 as *mut libc::c_char;
    } else {
        fd = os_anontmp();
        if fd >= 0 as libc::c_int {
            return fd;
        }
    }
    mask = umask(0o77 as libc::c_int as __mode_t);
    tmpnm = get_tmptemplate();
    loop {
        fd = mkstemp(tmpnm);
        if !(fd == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    if fd < 0 as libc::c_int {
        error(
            0 as *mut floc,
            (strlen(tmpnm)).wrapping_add(strlen(strerror(*__errno_location()))),
            dcgettext(
                0 as *const libc::c_char,
                b"cannot create temporary file %s: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            tmpnm,
            strerror(*__errno_location()),
        );
        free(tmpnm as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    if !name.is_null() {
        *name = tmpnm;
    } else {
        let mut r: libc::c_int = 0;
        loop {
            r = unlink(tmpnm);
            if !(r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if r < 0 as libc::c_int {
            error(
                0 as *mut floc,
                (strlen(tmpnm)).wrapping_add(strlen(strerror(*__errno_location()))),
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot unlink temporary file %s: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                tmpnm,
                strerror(*__errno_location()),
            );
        }
        free(tmpnm as *mut libc::c_void);
    }
    umask(mask);
    return fd;
}
pub unsafe extern "C" fn get_tmpfile(mut name: *mut *mut libc::c_char) -> *mut FILE {
    let mut tmpfile_mode: *const libc::c_char = b"wb+\0" as *const u8
        as *const libc::c_char;
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut fd: libc::c_int = 0;
    fd = get_tmpfd(name);
    if fd < 0 as libc::c_int {
        return 0 as *mut FILE;
    }
    loop {
        *__errno_location() = 0 as libc::c_int;
        file = fdopen(fd, tmpfile_mode);
        if !(file.is_null() && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    if file.is_null() {
        error(
            0 as *mut floc,
            (strlen(*name)).wrapping_add(strlen(strerror(*__errno_location()))),
            dcgettext(
                0 as *const libc::c_char,
                b"fdopen: temporary file %s: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            *name,
            strerror(*__errno_location()),
        );
    }
    return file;
}
