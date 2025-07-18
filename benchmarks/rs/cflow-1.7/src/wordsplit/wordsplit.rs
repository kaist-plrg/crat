use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type stat;
    pub type dirent;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn getuid() -> __uid_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn glob(
        __pattern: *const libc::c_char,
        __flags: libc::c_int,
        __errfunc: Option::<
            unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int,
        >,
        __pglob: *mut glob_t,
    ) -> libc::c_int;
    fn globfree(__pglob: *mut glob_t);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
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
pub type __int32_t = libc::c_int;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub type __size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut libc::c_char,
    pub gl_offs: __size_t,
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
pub struct wordsplit {
    pub ws_wordc: size_t,
    pub ws_wordv: *mut *mut libc::c_char,
    pub ws_offs: size_t,
    pub ws_wordn: size_t,
    pub ws_flags: libc::c_int,
    pub ws_options: libc::c_int,
    pub ws_maxwords: size_t,
    pub ws_wordi: size_t,
    pub ws_delim: *const libc::c_char,
    pub ws_comment: *const libc::c_char,
    pub ws_escape: [*const libc::c_char; 2],
    pub ws_namechar: *const libc::c_char,
    pub ws_alloc_die: Option::<unsafe extern "C" fn(*mut wordsplit_t) -> ()>,
    pub ws_error: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>,
    pub ws_debug: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>,
    pub ws_env: *mut *const libc::c_char,
    pub ws_envbuf: *mut *mut libc::c_char,
    pub ws_envidx: size_t,
    pub ws_envsiz: size_t,
    pub ws_paramv: *mut *const libc::c_char,
    pub ws_paramc: size_t,
    pub ws_parambuf: *mut *mut libc::c_char,
    pub ws_paramidx: size_t,
    pub ws_paramsiz: size_t,
    pub ws_getvar: Option::<
        unsafe extern "C" fn(
            *mut *mut libc::c_char,
            *const libc::c_char,
            size_t,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub ws_closure: *mut libc::c_void,
    pub ws_command: Option::<
        unsafe extern "C" fn(
            *mut *mut libc::c_char,
            *const libc::c_char,
            size_t,
            *mut *mut libc::c_char,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub ws_input: *const libc::c_char,
    pub ws_len: size_t,
    pub ws_endp: size_t,
    pub ws_errno: libc::c_int,
    pub ws_usererr: *mut libc::c_char,
    pub ws_errctx: *mut libc::c_char,
    pub ws_head: *mut wordsplit_node,
    pub ws_tail: *mut wordsplit_node,
    pub ws_sep: [libc::c_char; 2],
    pub ws_lvl: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wordsplit_node {
    pub prev: *mut wordsplit_node,
    pub next: *mut wordsplit_node,
    pub flags: libc::c_int,
    pub v: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub segm: C2RustUnnamed_0,
    pub word: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub beg: size_t,
    pub end: size_t,
}
pub type wordsplit_t = wordsplit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exptab {
    pub descr: *const libc::c_char,
    pub flag: libc::c_int,
    pub opt: libc::c_int,
    pub expansion: Option::<unsafe extern "C" fn(*mut wordsplit) -> libc::c_int>,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const st_dquote: C2RustUnnamed_1 = 2;
pub const st_squote: C2RustUnnamed_1 = 1;
pub const st_init: C2RustUnnamed_1 = 0;
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn is_name_char(
    mut wsp: *mut wordsplit,
    mut c: libc::c_int,
) -> libc::c_int {
    return ('A' as i32 as libc::c_uint <= c as libc::c_uint
        && c as libc::c_uint <= 'Z' as i32 as libc::c_uint
        || 'a' as i32 as libc::c_uint <= c as libc::c_uint
            && c as libc::c_uint <= 'z' as i32 as libc::c_uint
        || '0' as i32 as libc::c_uint <= c as libc::c_uint
            && c as libc::c_uint <= '9' as i32 as libc::c_uint || c == '_' as i32
        || (*wsp).ws_options & 0x10000 as libc::c_int != 0
            && !(strchr((*wsp).ws_namechar, c)).is_null()) as libc::c_int;
}
unsafe extern "C" fn _wsplt_alloc_die(mut wsp: *mut wordsplit) {
    ((*wsp).ws_error)
        .unwrap()(
        b"%s\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"memory exhausted\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    abort();
}
unsafe extern "C" fn _wsplt_error(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    vfprintf(stderr, fmt, ap.as_va_list());
    fputc('\n' as i32, stderr);
}
unsafe extern "C" fn _wsplt_seterr(
    mut wsp: *mut wordsplit,
    mut ec: libc::c_int,
) -> libc::c_int {
    (*wsp).ws_errno = ec;
    if (*wsp).ws_flags & 0x10 as libc::c_int != 0 {
        wordsplit_perror(wsp);
    }
    if ec == 3 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
    }
    return ec;
}
unsafe extern "C" fn _wsplt_nomem(mut wsp: *mut wordsplit) -> libc::c_int {
    *__errno_location() = 12 as libc::c_int;
    (*wsp).ws_errno = 2 as libc::c_int;
    if (*wsp).ws_flags & 0x80 as libc::c_int != 0 {
        ((*wsp).ws_alloc_die).unwrap()(wsp);
    }
    if (*wsp).ws_flags & 0x10 as libc::c_int != 0 {
        wordsplit_perror(wsp);
    }
    if (*wsp).ws_flags & 0x8 as libc::c_int == 0 {
        wordsplit_free(wsp);
    }
    wordsplit_free_nodes(wsp);
    return (*wsp).ws_errno;
}
unsafe extern "C" fn _wsplt_store_errctx(
    mut wsp: *mut wordsplit,
    mut str: *const libc::c_char,
    mut len: size_t,
) {
    free((*wsp).ws_errctx as *mut libc::c_void);
    (*wsp)
        .ws_errctx = malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    if ((*wsp).ws_errctx).is_null() {
        ((*wsp).ws_error)
            .unwrap()(
            b"%s\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"memory exhausted while trying to store error context\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        memcpy((*wsp).ws_errctx as *mut libc::c_void, str as *const libc::c_void, len);
        *((*wsp).ws_errctx).offset(len as isize) = 0 as libc::c_int as libc::c_char;
    };
}
#[inline]
unsafe extern "C" fn _wsplt_setctxerr(
    mut wsp: *mut wordsplit,
    mut ec: libc::c_int,
    mut str: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    _wsplt_store_errctx(wsp, str, len);
    return _wsplt_seterr(wsp, ec);
}
unsafe extern "C" fn _wsplt_subsplit(
    mut wsp: *mut wordsplit,
    mut wss: *mut wordsplit,
    mut str: *const libc::c_char,
    mut len: libc::c_int,
    mut flags: libc::c_int,
    mut finalize: libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    (*wss).ws_delim = (*wsp).ws_delim;
    (*wss).ws_debug = (*wsp).ws_debug;
    (*wss).ws_error = (*wsp).ws_error;
    (*wss).ws_alloc_die = (*wsp).ws_alloc_die;
    if flags & 0x40 as libc::c_int == 0 {
        (*wss).ws_env = (*wsp).ws_env;
        (*wss).ws_getvar = (*wsp).ws_getvar;
        flags
            |= (*wsp).ws_flags
                & (0x80000 as libc::c_int | 0x8000000 as libc::c_int
                    | 0x100000 as libc::c_int);
    }
    if flags & 0x4 as libc::c_int == 0 {
        (*wss).ws_command = (*wsp).ws_command;
    }
    if flags & (0x40 as libc::c_int | 0x4 as libc::c_int)
        != 0x40 as libc::c_int | 0x4 as libc::c_int
    {
        (*wss).ws_closure = (*wsp).ws_closure;
        flags |= (*wsp).ws_flags & 0x4000000 as libc::c_int;
    }
    (*wss).ws_options = (*wsp).ws_options & !(0x80 as libc::c_int);
    (*wss).ws_namechar = (*wsp).ws_namechar;
    flags = (flags as libc::c_uint
        | ((0x4000 as libc::c_int | 0x10000 as libc::c_int | 0x20000 as libc::c_int
            | 0x40000 as libc::c_int) as libc::c_uint
            | (*wsp).ws_flags as libc::c_uint
                & ((0x200000 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
                    | 0x80000000 as libc::c_uint))) as libc::c_int;
    rc = wordsplit_init(wss, str, len as size_t, flags);
    if rc != 0 {
        return rc;
    }
    (*wss).ws_lvl = (*wsp).ws_lvl + 1 as libc::c_int;
    rc = wordsplit_process_list(wss, 0 as libc::c_int as size_t);
    if rc != 0 {
        wordsplit_free_nodes(wss);
        return rc;
    }
    if finalize != 0 {
        rc = wordsplit_finish(wss);
        wordsplit_free_nodes(wss);
    }
    return rc;
}
unsafe extern "C" fn _wsplt_seterr_sub(
    mut wsp: *mut wordsplit,
    mut wss: *mut wordsplit,
) {
    if (*wsp).ws_errno == 9 as libc::c_int {
        free((*wsp).ws_usererr as *mut libc::c_void);
    }
    (*wsp).ws_errno = (*wss).ws_errno;
    if (*wss).ws_errno == 9 as libc::c_int {
        (*wsp).ws_usererr = (*wss).ws_usererr;
        (*wss).ws_errno = 0 as libc::c_int;
        (*wss).ws_usererr = 0 as *mut libc::c_char;
    }
    free((*wsp).ws_errctx as *mut libc::c_void);
    (*wsp).ws_errctx = (*wss).ws_errctx;
    (*wss).ws_errctx = 0 as *mut libc::c_char;
}
unsafe extern "C" fn wordsplit_init0(mut wsp: *mut wordsplit) {
    if (*wsp).ws_flags & 0x8 as libc::c_int != 0 {
        if (*wsp).ws_flags & 0x1 as libc::c_int == 0 {
            wordsplit_free_words(wsp);
        }
        wordsplit_clearerr(wsp);
    } else {
        (*wsp).ws_wordv = 0 as *mut *mut libc::c_char;
        (*wsp).ws_wordc = 0 as libc::c_int as size_t;
        (*wsp).ws_wordn = 0 as libc::c_int as size_t;
    }
    (*wsp).ws_errno = 0 as libc::c_int;
}
pub static mut wordsplit_c_escape_tab: [libc::c_char; 19] = unsafe {
    *::std::mem::transmute::<
        &[u8; 19],
        &mut [libc::c_char; 19],
    >(b"\\\\\"\"a\x07b\x08f\x0Cn\nr\rt\tv\x0B\0")
};
unsafe extern "C" fn wordsplit_init(
    mut wsp: *mut wordsplit,
    mut input: *const libc::c_char,
    mut len: size_t,
    mut flags: libc::c_int,
) -> libc::c_int {
    (*wsp).ws_flags = flags;
    if (*wsp).ws_flags & 0x10000 as libc::c_int == 0 {
        (*wsp)
            .ws_alloc_die = Some(
            _wsplt_alloc_die as unsafe extern "C" fn(*mut wordsplit) -> (),
        );
    }
    if (*wsp).ws_flags & 0x20000 as libc::c_int == 0 {
        (*wsp)
            .ws_error = Some(
            _wsplt_error as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
        );
    }
    if (*wsp).ws_flags & 0x40 as libc::c_int == 0 {
        (*wsp).ws_envsiz = 0 as libc::c_int as size_t;
        (*wsp).ws_envidx = (*wsp).ws_envsiz;
        (*wsp).ws_envbuf = 0 as *mut *mut libc::c_char;
    }
    if (*wsp).ws_flags & 0x4 as libc::c_int == 0 {
        if ((*wsp).ws_command).is_none() {
            return _wsplt_seterr(wsp, 3 as libc::c_int);
        }
    }
    if (*wsp).ws_flags & 0x200000 as libc::c_int != 0 {
        if (*wsp).ws_flags & 0x40000 as libc::c_int == 0 {
            if (*wsp).ws_flags & 0x20000 as libc::c_int != 0 {
                (*wsp).ws_debug = (*wsp).ws_error;
            } else if (*wsp).ws_flags & 0x10 as libc::c_int != 0 {
                (*wsp)
                    .ws_debug = Some(
                    _wsplt_error as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                );
            } else {
                (*wsp).ws_flags &= !(0x200000 as libc::c_int);
            }
        }
    }
    (*wsp).ws_input = input;
    (*wsp).ws_len = len;
    if (*wsp).ws_flags & 0x2 as libc::c_int == 0 {
        (*wsp).ws_offs = 0 as libc::c_int as size_t;
    }
    if (*wsp).ws_flags & 0x4000 as libc::c_int == 0 {
        (*wsp).ws_delim = b" \t\n\0" as *const u8 as *const libc::c_char;
    }
    (*wsp)
        .ws_sep[0 as libc::c_int
        as usize] = *((*wsp).ws_delim).offset(0 as libc::c_int as isize);
    (*wsp).ws_sep[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if (*wsp).ws_flags & 0x8000 as libc::c_int == 0 {
        (*wsp).ws_comment = 0 as *const libc::c_char;
    }
    if (*wsp).ws_flags & 0x4000000 as libc::c_int == 0 {
        (*wsp).ws_closure = 0 as *mut libc::c_void;
    }
    if (*wsp).ws_flags as libc::c_uint & 0x80000000 as libc::c_uint == 0 {
        (*wsp).ws_options = 0 as libc::c_int;
    }
    if (*wsp).ws_flags & 0x10000000 as libc::c_int != 0 {
        if ((*wsp).ws_escape[0 as libc::c_int as usize]).is_null() {
            (*wsp)
                .ws_escape[0 as libc::c_int
                as usize] = b"\0" as *const u8 as *const libc::c_char;
        }
        if ((*wsp).ws_escape[1 as libc::c_int as usize]).is_null() {
            (*wsp)
                .ws_escape[1 as libc::c_int
                as usize] = b"\0" as *const u8 as *const libc::c_char;
        }
    } else if (*wsp).ws_flags & 0x2000000 as libc::c_int != 0 {
        (*wsp)
            .ws_escape[0 as libc::c_int as usize] = wordsplit_c_escape_tab.as_mut_ptr();
        (*wsp)
            .ws_escape[1 as libc::c_int as usize] = wordsplit_c_escape_tab.as_mut_ptr();
        (*wsp).ws_options
            |= 0x200 as libc::c_int | 0x20 as libc::c_int | 0x400 as libc::c_int
                | 0x40 as libc::c_int;
    } else {
        (*wsp)
            .ws_escape[0 as libc::c_int
            as usize] = b"\0" as *const u8 as *const libc::c_char;
        (*wsp)
            .ws_escape[1 as libc::c_int
            as usize] = b"\\\\\"\"\0" as *const u8 as *const libc::c_char;
        (*wsp).ws_options |= 0x100 as libc::c_int;
    }
    if (*wsp).ws_options & 0x4000 as libc::c_int == 0 {
        (*wsp).ws_paramv = 0 as *mut *const libc::c_char;
        (*wsp).ws_paramc = 0 as libc::c_int as size_t;
    }
    (*wsp).ws_paramsiz = 0 as libc::c_int as size_t;
    (*wsp).ws_paramidx = (*wsp).ws_paramsiz;
    (*wsp).ws_parambuf = 0 as *mut *mut libc::c_char;
    if (*wsp).ws_options & 0x10000 as libc::c_int != 0 {
        if *((*wsp).ws_namechar)
            .offset(
                strcspn(
                    (*wsp).ws_namechar,
                    b"${}*@-+?=\0" as *const u8 as *const libc::c_char,
                ) as isize,
            ) != 0
        {
            return _wsplt_seterr(wsp, 3 as libc::c_int);
        }
    } else {
        (*wsp).ws_namechar = 0 as *const libc::c_char;
    }
    (*wsp).ws_endp = 0 as libc::c_int as size_t;
    (*wsp).ws_wordi = 0 as libc::c_int as size_t;
    if (*wsp).ws_flags & 0x8 as libc::c_int != 0 {
        wordsplit_free_nodes(wsp);
    }
    (*wsp).ws_tail = 0 as *mut wordsplit_node;
    (*wsp).ws_head = (*wsp).ws_tail;
    (*wsp).ws_errctx = 0 as *mut libc::c_char;
    wordsplit_init0(wsp);
    return 0 as libc::c_int;
}
unsafe extern "C" fn alloc_space(
    mut wsp: *mut wordsplit,
    mut count: size_t,
) -> libc::c_int {
    let mut offs: size_t = if (*wsp).ws_flags & 0x2 as libc::c_int != 0 {
        (*wsp).ws_offs
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut ptr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut newalloc: size_t = 0;
    if ((*wsp).ws_wordv).is_null() {
        newalloc = if offs.wrapping_add(count) > 128 as libc::c_int as libc::c_ulong {
            count
        } else {
            128 as libc::c_int as libc::c_ulong
        };
        ptr = calloc(
            newalloc,
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        ) as *mut *mut libc::c_char;
    } else if (*wsp).ws_wordn < offs.wrapping_add((*wsp).ws_wordc).wrapping_add(count) {
        newalloc = offs
            .wrapping_add((*wsp).ws_wordc)
            .wrapping_add(
                (if count > 128 as libc::c_int as libc::c_ulong {
                    count
                } else {
                    128 as libc::c_int as libc::c_ulong
                }),
            );
        ptr = realloc(
            (*wsp).ws_wordv as *mut libc::c_void,
            newalloc
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
    } else {
        return 0 as libc::c_int
    }
    if !ptr.is_null() {
        (*wsp).ws_wordn = newalloc;
        (*wsp).ws_wordv = ptr;
    } else {
        return _wsplt_nomem(wsp)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn wsnode_flagstr(mut flags: libc::c_int) -> *const libc::c_char {
    static mut retbuf: [libc::c_char; 7] = [0; 7];
    let mut p: *mut libc::c_char = retbuf.as_mut_ptr();
    if flags & 0x2 as libc::c_int != 0 {
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = 'w' as i32 as libc::c_char;
    } else if flags & 0x1 as libc::c_int != 0 {
        let fresh1 = p;
        p = p.offset(1);
        *fresh1 = 'n' as i32 as libc::c_char;
    } else {
        let fresh2 = p;
        p = p.offset(1);
        *fresh2 = '-' as i32 as libc::c_char;
    }
    if flags & 0x4 as libc::c_int != 0 {
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = 'q' as i32 as libc::c_char;
    } else {
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = '-' as i32 as libc::c_char;
    }
    if flags & 0x8 as libc::c_int != 0 {
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = 'E' as i32 as libc::c_char;
    } else {
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = '-' as i32 as libc::c_char;
    }
    if flags & 0x10 as libc::c_int != 0 {
        let fresh7 = p;
        p = p.offset(1);
        *fresh7 = 'j' as i32 as libc::c_char;
    } else {
        let fresh8 = p;
        p = p.offset(1);
        *fresh8 = '-' as i32 as libc::c_char;
    }
    if flags & 0x20 as libc::c_int != 0 {
        let fresh9 = p;
        p = p.offset(1);
        *fresh9 = 's' as i32 as libc::c_char;
    } else {
        let fresh10 = p;
        p = p.offset(1);
        *fresh10 = '-' as i32 as libc::c_char;
    }
    if flags & 0x40 as libc::c_int != 0 {
        let fresh11 = p;
        p = p.offset(1);
        *fresh11 = 'd' as i32 as libc::c_char;
    } else {
        let fresh12 = p;
        p = p.offset(1);
        *fresh12 = '-' as i32 as libc::c_char;
    }
    *p = 0 as libc::c_int as libc::c_char;
    return retbuf.as_mut_ptr();
}
unsafe extern "C" fn wsnode_ptr(
    mut wsp: *mut wordsplit,
    mut p: *mut wordsplit_node,
) -> *const libc::c_char {
    if (*p).flags & 0x1 as libc::c_int != 0 {
        return b"\0" as *const u8 as *const libc::c_char
    } else if (*p).flags & 0x2 as libc::c_int != 0 {
        return (*p).v.word
    } else {
        return ((*wsp).ws_input).offset((*p).v.segm.beg as isize)
    };
}
unsafe extern "C" fn wsnode_len(mut p: *mut wordsplit_node) -> size_t {
    if (*p).flags & 0x1 as libc::c_int != 0 {
        return 0 as libc::c_int as size_t
    } else if (*p).flags & 0x2 as libc::c_int != 0 {
        return strlen((*p).v.word)
    } else {
        return ((*p).v.segm.end).wrapping_sub((*p).v.segm.beg)
    };
}
unsafe extern "C" fn wsnode_new(
    mut wsp: *mut wordsplit,
    mut pnode: *mut *mut wordsplit_node,
) -> libc::c_int {
    let mut node: *mut wordsplit_node = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<wordsplit_node>() as libc::c_ulong,
    ) as *mut wordsplit_node;
    if node.is_null() {
        return _wsplt_nomem(wsp);
    }
    *pnode = node;
    return 0 as libc::c_int;
}
unsafe extern "C" fn wsnode_free(mut p: *mut wordsplit_node) {
    if (*p).flags & (0x2 as libc::c_int | 0x80 as libc::c_int) == 0x2 as libc::c_int {
        free((*p).v.word as *mut libc::c_void);
    }
    free(p as *mut libc::c_void);
}
unsafe extern "C" fn wsnode_append(
    mut wsp: *mut wordsplit,
    mut node: *mut wordsplit_node,
) {
    (*node).next = 0 as *mut wordsplit_node;
    (*node).prev = (*wsp).ws_tail;
    if !((*wsp).ws_tail).is_null() {
        (*(*wsp).ws_tail).next = node;
    } else {
        (*wsp).ws_head = node;
    }
    (*wsp).ws_tail = node;
}
unsafe extern "C" fn wsnode_remove(
    mut wsp: *mut wordsplit,
    mut node: *mut wordsplit_node,
) {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    p = (*node).prev;
    if !p.is_null() {
        (*p).next = (*node).next;
        if ((*node).next).is_null() {
            (*p).flags &= !(0x10 as libc::c_int);
        }
    } else {
        (*wsp).ws_head = (*node).next;
    }
    p = (*node).next;
    if !p.is_null() {
        (*p).prev = (*node).prev;
    } else {
        (*wsp).ws_tail = (*node).prev;
    }
    wsnode_free(node);
}
unsafe extern "C" fn wsnode_tail(mut p: *mut wordsplit_node) -> *mut wordsplit_node {
    while !p.is_null() && !((*p).next).is_null() {
        p = (*p).next;
    }
    return p;
}
unsafe extern "C" fn wsnode_insert(
    mut wsp: *mut wordsplit,
    mut node: *mut wordsplit_node,
    mut anchor: *mut wordsplit_node,
    mut before: libc::c_int,
) {
    if ((*wsp).ws_head).is_null() {
        (*node).prev = 0 as *mut wordsplit_node;
        (*node).next = (*node).prev;
        (*wsp).ws_tail = node;
        (*wsp).ws_head = (*wsp).ws_tail;
    } else if before != 0 {
        if !((*anchor).prev).is_null() {
            wsnode_insert(wsp, node, (*anchor).prev, 0 as libc::c_int);
        } else {
            let mut tail: *mut wordsplit_node = wsnode_tail(node);
            (*node).prev = 0 as *mut wordsplit_node;
            (*tail).next = anchor;
            (*anchor).prev = tail;
            (*wsp).ws_head = node;
        }
    } else {
        let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
        let mut tail_0: *mut wordsplit_node = wsnode_tail(node);
        p = (*anchor).next;
        if !p.is_null() {
            (*p).prev = tail_0;
        } else {
            (*wsp).ws_tail = tail_0;
        }
        (*tail_0).next = p;
        (*node).prev = anchor;
        (*anchor).next = node;
    };
}
unsafe extern "C" fn wordsplit_add_segm(
    mut wsp: *mut wordsplit,
    mut beg: size_t,
    mut end: size_t,
    mut flg: libc::c_int,
) -> libc::c_int {
    let mut node: *mut wordsplit_node = 0 as *mut wordsplit_node;
    let mut rc: libc::c_int = 0;
    if end == beg && flg & 0x100 as libc::c_int == 0 {
        return 0 as libc::c_int;
    }
    rc = wsnode_new(wsp, &mut node);
    if rc != 0 {
        return rc;
    }
    (*node).flags = flg & !(0x2 as libc::c_int | 0x100 as libc::c_int);
    (*node).v.segm.beg = beg;
    (*node).v.segm.end = end;
    wsnode_append(wsp, node);
    return 0 as libc::c_int;
}
unsafe extern "C" fn wordsplit_free_nodes(mut wsp: *mut wordsplit) {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    p = (*wsp).ws_head;
    while !p.is_null() {
        let mut next: *mut wordsplit_node = (*p).next;
        wsnode_free(p);
        p = next;
    }
    (*wsp).ws_tail = 0 as *mut wordsplit_node;
    (*wsp).ws_head = (*wsp).ws_tail;
}
unsafe extern "C" fn wordsplit_dump_nodes(mut wsp: *mut wordsplit) {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    let mut n: libc::c_int = 0 as libc::c_int;
    p = (*wsp).ws_head;
    n = 0 as libc::c_int;
    while !p.is_null() {
        if (*p).flags & 0x2 as libc::c_int != 0 {
            ((*wsp).ws_debug)
                .unwrap()(
                b"(%02d) %4d: %p: %#04x (%s):%s;\0" as *const u8 as *const libc::c_char,
                (*wsp).ws_lvl,
                n,
                p,
                (*p).flags,
                wsnode_flagstr((*p).flags),
                (*p).v.word,
            );
        } else {
            ((*wsp).ws_debug)
                .unwrap()(
                b"(%02d) %4d: %p: %#04x (%s):%.*s;\0" as *const u8
                    as *const libc::c_char,
                (*wsp).ws_lvl,
                n,
                p,
                (*p).flags,
                wsnode_flagstr((*p).flags),
                ((*p).v.segm.end).wrapping_sub((*p).v.segm.beg) as libc::c_int,
                ((*wsp).ws_input).offset((*p).v.segm.beg as isize),
            );
        }
        p = (*p).next;
        n += 1;
        n;
    }
}
unsafe extern "C" fn coalesce_segment(
    mut wsp: *mut wordsplit,
    mut node: *mut wordsplit_node,
) -> libc::c_int {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    let mut end: *mut wordsplit_node = 0 as *mut wordsplit_node;
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cur: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stop: libc::c_int = 0;
    if (*node).flags & 0x10 as libc::c_int == 0 {
        return 0 as libc::c_int;
    }
    p = node;
    while !p.is_null() && (*p).flags & 0x10 as libc::c_int != 0 {
        len = (len as libc::c_ulong).wrapping_add(wsnode_len(p)) as size_t as size_t;
        p = (*p).next;
    }
    if !p.is_null() {
        len = (len as libc::c_ulong).wrapping_add(wsnode_len(p)) as size_t as size_t;
    }
    end = p;
    buf = malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    if buf.is_null() {
        return _wsplt_nomem(wsp);
    }
    cur = buf;
    p = node;
    stop = 0 as libc::c_int;
    while stop == 0 {
        let mut next: *mut wordsplit_node = (*p).next;
        let mut str: *const libc::c_char = wsnode_ptr(wsp, p);
        let mut slen: size_t = wsnode_len(p);
        memcpy(cur as *mut libc::c_void, str as *const libc::c_void, slen);
        cur = cur.offset(slen as isize);
        if p != node {
            (*node).flags |= (*p).flags & 0x4 as libc::c_int;
            stop = (p == end) as libc::c_int;
            wsnode_remove(wsp, p);
        }
        p = next;
    }
    *cur = 0 as libc::c_int as libc::c_char;
    (*node).flags &= !(0x10 as libc::c_int);
    if (*node).flags & 0x2 as libc::c_int != 0 {
        free((*node).v.word as *mut libc::c_void);
    } else {
        (*node).flags |= 0x2 as libc::c_int;
    }
    (*node).v.word = buf;
    return 0 as libc::c_int;
}
unsafe extern "C" fn wsnode_quoteremoval(mut wsp: *mut wordsplit) -> libc::c_int {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    p = (*wsp).ws_head;
    while !p.is_null() {
        let mut str: *const libc::c_char = wsnode_ptr(wsp, p);
        let mut slen: size_t = wsnode_len(p);
        let mut unquote: libc::c_int = 0;
        if (*wsp).ws_flags & (0x200 as libc::c_int | 0x400 as libc::c_int) != 0 {
            unquote = ((*p).flags & 0x8 as libc::c_int == 0) as libc::c_int;
        } else {
            unquote = 0 as libc::c_int;
        }
        if unquote != 0 {
            if (*p).flags & 0x2 as libc::c_int == 0 {
                let mut newstr: *mut libc::c_char = malloc(
                    slen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                if newstr.is_null() {
                    return _wsplt_nomem(wsp);
                }
                memcpy(newstr as *mut libc::c_void, str as *const libc::c_void, slen);
                *newstr.offset(slen as isize) = 0 as libc::c_int as libc::c_char;
                (*p).v.word = newstr;
                (*p).flags |= 0x2 as libc::c_int;
            }
            wordsplit_string_unquote_copy(
                wsp,
                (*p).flags & 0x4 as libc::c_int,
                (*p).v.word,
                str,
                slen,
            );
        }
        p = (*p).next;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn wsnode_coalesce(mut wsp: *mut wordsplit) -> libc::c_int {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    p = (*wsp).ws_head;
    while !p.is_null() {
        if (*p).flags & 0x10 as libc::c_int != 0 {
            if coalesce_segment(wsp, p) != 0 {
                return 1 as libc::c_int;
            }
        }
        p = (*p).next;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn wsnode_tail_coalesce(
    mut wsp: *mut wordsplit,
    mut p: *mut wordsplit_node,
) -> libc::c_int {
    if !((*p).next).is_null() {
        let mut np: *mut wordsplit_node = p;
        while !np.is_null() && !((*np).next).is_null() {
            (*np).flags |= 0x10 as libc::c_int;
            np = (*np).next;
        }
        if coalesce_segment(wsp, p) != 0 {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn wordsplit_finish(mut wsp: *mut wordsplit) -> libc::c_int {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    let mut n: size_t = 0;
    let mut delim: libc::c_int = 0;
    loop {
        delim = 0 as libc::c_int;
        n = 0 as libc::c_int as size_t;
        p = (*wsp).ws_head;
        while !p.is_null() {
            let mut next: *mut wordsplit_node = (*p).next;
            if (*p).flags & 0x40 as libc::c_int != 0 {
                if (*wsp).ws_flags & 0x1000 as libc::c_int != 0 {
                    if (*wsp).ws_flags & 0x800 as libc::c_int != 0 {
                        let mut s: *const libc::c_char = wsnode_ptr(wsp, p);
                        if delim != 0 {
                            if delim == *s as libc::c_int {
                                wsnode_remove(wsp, p);
                                p = next;
                                continue;
                            } else {
                                delim = 0 as libc::c_int;
                                n = n.wrapping_add(1);
                                n;
                            }
                        } else {
                            delim = *s as libc::c_int;
                            p = next;
                            continue;
                        }
                    }
                } else if (*wsp).ws_options & 0x80 as libc::c_int != 0 {
                    wsnode_remove(wsp, p);
                    p = next;
                    continue;
                }
            } else {
                if delim != 0 {
                    n = n.wrapping_add(1);
                    n;
                    delim = 0 as libc::c_int;
                }
                if (*wsp).ws_options & 0x80 as libc::c_int != 0 {
                    if ((*wsp).ws_wordi)
                        .wrapping_add(n)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        == (*wsp).ws_maxwords
                    {
                        break;
                    }
                }
            }
            n = n.wrapping_add(1);
            n;
            if (*wsp).ws_flags & 0x20000000 as libc::c_int != 0 {
                p = 0 as *mut wordsplit_node;
            } else {
                p = next;
            }
        }
        if !p.is_null() {
            if wsnode_tail_coalesce(wsp, p) != 0 {
                return (*wsp).ws_errno;
            }
            n = n.wrapping_add(1);
            n;
        }
        if !(n == 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        if (*wsp).ws_flags & 0x20000000 as libc::c_int != 0 {
            if (*wsp).ws_endp < (*wsp).ws_len {
                let mut rc: libc::c_int = 0;
                if (*wsp).ws_flags & 0x200000 as libc::c_int != 0 {
                    ((*wsp).ws_debug)
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Restarting\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                rc = wordsplit_process_list(wsp, skip_delim(wsp));
                if rc != 0 {
                    return rc;
                }
            } else {
                (*wsp).ws_errno = 0 as libc::c_int;
                return 0 as libc::c_int;
            }
        } else {
            if (*wsp).ws_flags & 0x400000 as libc::c_int != 0 {
                if wordsplit_add_segm(
                    wsp,
                    0 as libc::c_int as size_t,
                    0 as libc::c_int as size_t,
                    0x100 as libc::c_int,
                ) != 0
                {
                    return (*wsp).ws_errno;
                }
                n = 1 as libc::c_int as size_t;
            }
            break;
        }
    }
    if alloc_space(wsp, n.wrapping_add(1 as libc::c_int as libc::c_ulong)) != 0 {
        return (*wsp).ws_errno;
    }
    while !((*wsp).ws_head).is_null() {
        let mut str: *const libc::c_char = wsnode_ptr(wsp, (*wsp).ws_head);
        let mut slen: size_t = wsnode_len((*wsp).ws_head);
        let mut newstr: *mut libc::c_char = malloc(
            slen.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        let ref mut fresh13 = *((*wsp).ws_wordv)
            .offset(((*wsp).ws_offs).wrapping_add((*wsp).ws_wordc) as isize);
        *fresh13 = newstr;
        if newstr.is_null() {
            return _wsplt_nomem(wsp);
        }
        memcpy(newstr as *mut libc::c_void, str as *const libc::c_void, slen);
        *newstr.offset(slen as isize) = 0 as libc::c_int as libc::c_char;
        wsnode_remove(wsp, (*wsp).ws_head);
        (*wsp).ws_wordc = ((*wsp).ws_wordc).wrapping_add(1);
        (*wsp).ws_wordc;
        (*wsp).ws_wordi = ((*wsp).ws_wordi).wrapping_add(1);
        (*wsp).ws_wordi;
        if (*wsp).ws_flags & 0x20000000 as libc::c_int != 0 {
            break;
        }
    }
    let ref mut fresh14 = *((*wsp).ws_wordv)
        .offset(((*wsp).ws_offs).wrapping_add((*wsp).ws_wordc) as isize);
    *fresh14 = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn wordsplit_append(
    mut wsp: *mut wordsplit_t,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut i: size_t = 0;
    rc = alloc_space(
        wsp,
        ((*wsp).ws_wordc)
            .wrapping_add(argc as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    if rc != 0 {
        return rc;
    }
    i = 0 as libc::c_int as size_t;
    while i < argc as libc::c_ulong {
        let mut newstr: *mut libc::c_char = strdup(*argv.offset(i as isize));
        if newstr.is_null() {
            while i > 0 as libc::c_int as libc::c_ulong {
                free(
                    *((*wsp).ws_wordv)
                        .offset(
                            ((*wsp).ws_offs)
                                .wrapping_add((*wsp).ws_wordc)
                                .wrapping_add(i)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as *mut libc::c_void,
                );
                let ref mut fresh15 = *((*wsp).ws_wordv)
                    .offset(
                        ((*wsp).ws_offs)
                            .wrapping_add((*wsp).ws_wordc)
                            .wrapping_add(i)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                *fresh15 = 0 as *mut libc::c_char;
                i = i.wrapping_sub(1);
                i;
            }
            return _wsplt_nomem(wsp);
        }
        let ref mut fresh16 = *((*wsp).ws_wordv)
            .offset(
                ((*wsp).ws_offs).wrapping_add((*wsp).ws_wordc).wrapping_add(i) as isize,
            );
        *fresh16 = newstr;
        i = i.wrapping_add(1);
        i;
    }
    (*wsp)
        .ws_wordc = ((*wsp).ws_wordc as libc::c_ulong).wrapping_add(i) as size_t
        as size_t;
    let ref mut fresh17 = *((*wsp).ws_wordv)
        .offset(((*wsp).ws_offs).wrapping_add((*wsp).ws_wordc) as isize);
    *fresh17 = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn node_split_prefix(
    mut wsp: *mut wordsplit,
    mut ptail: *mut *mut wordsplit_node,
    mut node: *mut wordsplit_node,
    mut beg: size_t,
    mut len: size_t,
    mut flg: libc::c_int,
) -> libc::c_int {
    let mut newnode: *mut wordsplit_node = 0 as *mut wordsplit_node;
    if len == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if wsnode_new(wsp, &mut newnode) != 0 {
        return 1 as libc::c_int;
    }
    wsnode_insert(wsp, newnode, *ptail, 0 as libc::c_int);
    if (*node).flags & 0x2 as libc::c_int != 0 {
        let mut str: *const libc::c_char = wsnode_ptr(wsp, node);
        let mut newstr: *mut libc::c_char = malloc(
            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        if newstr.is_null() {
            return _wsplt_nomem(wsp);
        }
        memcpy(
            newstr as *mut libc::c_void,
            str.offset(beg as isize) as *const libc::c_void,
            len,
        );
        *newstr.offset(len as isize) = 0 as libc::c_int as libc::c_char;
        (*newnode).flags = 0x2 as libc::c_int;
        (*newnode).v.word = newstr;
    } else {
        (*newnode).v.segm.beg = ((*node).v.segm.beg).wrapping_add(beg);
        (*newnode).v.segm.end = ((*newnode).v.segm.beg).wrapping_add(len);
    }
    (*newnode).flags |= flg;
    *ptail = newnode;
    return 0 as libc::c_int;
}
unsafe extern "C" fn find_closing_paren(
    mut str: *const libc::c_char,
    mut i: size_t,
    mut len: size_t,
    mut poff: *mut size_t,
    mut paren: *const libc::c_char,
) -> libc::c_int {
    let mut state: C2RustUnnamed_1 = st_init;
    let mut level: size_t = 1 as libc::c_int as size_t;
    while i < len {
        match state as libc::c_uint {
            0 => {
                match *str.offset(i as isize) as libc::c_int {
                    34 => {
                        state = st_dquote;
                    }
                    39 => {
                        state = st_squote;
                    }
                    _ => {
                        if *str.offset(i as isize) as libc::c_int
                            == *paren.offset(0 as libc::c_int as isize) as libc::c_int
                        {
                            level = level.wrapping_add(1);
                            level;
                        } else if *str.offset(i as isize) as libc::c_int
                            == *paren.offset(1 as libc::c_int as isize) as libc::c_int
                        {
                            level = level.wrapping_sub(1);
                            if level == 0 as libc::c_int as libc::c_ulong {
                                *poff = i;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                }
            }
            1 => {
                if *str.offset(i as isize) as libc::c_int == '\'' as i32 {
                    state = st_init;
                }
            }
            2 => {
                if *str.offset(i as isize) as libc::c_int == '\\' as i32 {
                    i = i.wrapping_add(1);
                    i;
                } else if *str.offset(i as isize) as libc::c_int == '"' as i32 {
                    state = st_init;
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn wsplt_env_find(
    mut wsp: *mut wordsplit,
    mut name: *const libc::c_char,
    mut len: size_t,
) -> *const libc::c_char {
    let mut i: size_t = 0;
    if ((*wsp).ws_env).is_null() {
        return 0 as *const libc::c_char;
    }
    if (*wsp).ws_flags & 0x8000000 as libc::c_int != 0 {
        i = 0 as libc::c_int as size_t;
        while !(*((*wsp).ws_env).offset(i as isize)).is_null() {
            let mut elen: size_t = strlen(*((*wsp).ws_env).offset(i as isize));
            if elen == len
                && memcmp(
                    *((*wsp).ws_env).offset(i as isize) as *const libc::c_void,
                    name as *const libc::c_void,
                    elen,
                ) == 0 as libc::c_int
            {
                return *((*wsp).ws_env)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            }
            i = i.wrapping_add(1);
            i;
            if (*((*wsp).ws_env).offset(i as isize)).is_null() {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else {
        i = 0 as libc::c_int as size_t;
        while !(*((*wsp).ws_env).offset(i as isize)).is_null() {
            let mut j: size_t = 0;
            let mut var: *const libc::c_char = *((*wsp).ws_env).offset(i as isize);
            j = 0 as libc::c_int as size_t;
            while j < len {
                if *name.offset(j as isize) as libc::c_int
                    != *var.offset(j as isize) as libc::c_int
                {
                    break;
                }
                j = j.wrapping_add(1);
                j;
            }
            if j == len && *var.offset(j as isize) as libc::c_int == '=' as i32 {
                return var.offset(j as isize).offset(1 as libc::c_int as isize);
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn wsplt_env_lookup(
    mut wsp: *mut wordsplit,
    mut name: *const libc::c_char,
    mut len: size_t,
    mut ret: *mut *mut libc::c_char,
) -> libc::c_int {
    if (*wsp).ws_flags & 0x80000 as libc::c_int != 0 {
        let mut val: *const libc::c_char = wsplt_env_find(wsp, name, len);
        if !val.is_null() {
            let mut retval: *mut libc::c_char = strdup(val);
            if retval.is_null() {
                return 2 as libc::c_int;
            }
            *ret = retval;
            return 0 as libc::c_int;
        }
    }
    return 5 as libc::c_int;
}
unsafe extern "C" fn wsplt_env_getvar(
    mut wsp: *mut wordsplit,
    mut name: *const libc::c_char,
    mut len: size_t,
    mut ret: *mut *mut libc::c_char,
) -> libc::c_int {
    return ((*wsp).ws_getvar).unwrap()(ret, name, len, (*wsp).ws_closure);
}
unsafe extern "C" fn wsplt_assign_var(
    mut wsp: *mut wordsplit,
    mut name: *const libc::c_char,
    mut namelen: size_t,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut n: libc::c_int = if (*wsp).ws_flags & 0x8000000 as libc::c_int != 0 {
        2 as libc::c_int
    } else {
        1 as libc::c_int
    };
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    if ((*wsp).ws_envidx).wrapping_add(n as libc::c_ulong) >= (*wsp).ws_envsiz {
        let mut sz: size_t = 0;
        let mut newenv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        if ((*wsp).ws_envbuf).is_null() {
            if (*wsp).ws_flags & 0x80000 as libc::c_int != 0 {
                let mut i: size_t = 0 as libc::c_int as size_t;
                let mut j: size_t = 0;
                if !((*wsp).ws_env).is_null() {
                    while !(*((*wsp).ws_env).offset(i as isize)).is_null() {
                        i = i.wrapping_add(1);
                        i;
                    }
                }
                sz = i
                    .wrapping_add(n as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                newenv = calloc(
                    sz,
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ) as *mut *mut libc::c_char;
                if newenv.is_null() {
                    return _wsplt_nomem(wsp);
                }
                j = 0 as libc::c_int as size_t;
                while j < i {
                    let ref mut fresh18 = *newenv.offset(j as isize);
                    *fresh18 = strdup(*((*wsp).ws_env).offset(j as isize));
                    if (*newenv.offset(j as isize)).is_null() {
                        while j > 1 as libc::c_int as libc::c_ulong {
                            free(
                                *newenv
                                    .offset(
                                        j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                    ) as *mut libc::c_void,
                            );
                            j = j.wrapping_sub(1);
                            j;
                        }
                        free(newenv as *mut libc::c_void);
                        return _wsplt_nomem(wsp);
                    }
                    j = j.wrapping_add(1);
                    j;
                }
                let ref mut fresh19 = *newenv.offset(j as isize);
                *fresh19 = 0 as *mut libc::c_char;
                (*wsp).ws_envbuf = newenv;
                (*wsp).ws_envidx = i;
                (*wsp).ws_envsiz = sz;
                (*wsp).ws_env = (*wsp).ws_envbuf as *mut *const libc::c_char;
            } else {
                newenv = calloc(
                    16 as libc::c_int as libc::c_ulong,
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ) as *mut *mut libc::c_char;
                if newenv.is_null() {
                    return _wsplt_nomem(wsp);
                }
                (*wsp).ws_envbuf = newenv;
                (*wsp).ws_envidx = 0 as libc::c_int as size_t;
                (*wsp).ws_envsiz = 16 as libc::c_int as size_t;
                (*wsp).ws_env = (*wsp).ws_envbuf as *mut *const libc::c_char;
                (*wsp).ws_flags |= 0x80000 as libc::c_int;
            }
        } else {
            let mut n_0: size_t = (*wsp).ws_envsiz;
            if (-(1 as libc::c_int) as size_t)
                .wrapping_div(3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ) <= n_0
            {
                return _wsplt_nomem(wsp);
            }
            n_0 = (n_0 as libc::c_ulong)
                .wrapping_add(
                    n_0
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong),
                ) as size_t as size_t;
            newenv = realloc(
                (*wsp).ws_envbuf as *mut libc::c_void,
                n_0
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *mut libc::c_char;
            if newenv.is_null() {
                return _wsplt_nomem(wsp);
            }
            (*wsp).ws_envbuf = newenv;
            (*wsp).ws_envsiz = n_0;
            (*wsp).ws_env = (*wsp).ws_envbuf as *mut *const libc::c_char;
        }
    }
    if (*wsp).ws_flags & 0x8000000 as libc::c_int != 0 {
        let mut p: *mut libc::c_char = malloc(
            namelen.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        if p.is_null() {
            return _wsplt_nomem(wsp);
        }
        memcpy(p as *mut libc::c_void, name as *const libc::c_void, namelen);
        *p.offset(namelen as isize) = 0 as libc::c_int as libc::c_char;
        v = strdup(value);
        if v.is_null() {
            free(p as *mut libc::c_void);
            return _wsplt_nomem(wsp);
        }
        let fresh20 = (*wsp).ws_envidx;
        (*wsp).ws_envidx = ((*wsp).ws_envidx).wrapping_add(1);
        let ref mut fresh21 = *((*wsp).ws_env).offset(fresh20 as isize);
        *fresh21 = p;
        let fresh22 = (*wsp).ws_envidx;
        (*wsp).ws_envidx = ((*wsp).ws_envidx).wrapping_add(1);
        let ref mut fresh23 = *((*wsp).ws_env).offset(fresh22 as isize);
        *fresh23 = v;
    } else {
        v = malloc(
            namelen
                .wrapping_add(strlen(value))
                .wrapping_add(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        if v.is_null() {
            return _wsplt_nomem(wsp);
        }
        memcpy(v as *mut libc::c_void, name as *const libc::c_void, namelen);
        let fresh24 = namelen;
        namelen = namelen.wrapping_add(1);
        *v.offset(fresh24 as isize) = '=' as i32 as libc::c_char;
        strcpy(v.offset(namelen as isize), value);
        let fresh25 = (*wsp).ws_envidx;
        (*wsp).ws_envidx = ((*wsp).ws_envidx).wrapping_add(1);
        let ref mut fresh26 = *((*wsp).ws_env).offset(fresh25 as isize);
        *fresh26 = v;
    }
    let ref mut fresh27 = *((*wsp).ws_env).offset((*wsp).ws_envidx as isize);
    *fresh27 = 0 as *const libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn wsplt_assign_param(
    mut wsp: *mut wordsplit,
    mut param_idx: libc::c_int,
    mut value: *mut libc::c_char,
) -> libc::c_int {
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    if param_idx < 0 as libc::c_int {
        return _wsplt_seterr(wsp, 10 as libc::c_int);
    }
    if param_idx as libc::c_ulong == (*wsp).ws_paramc {
        let mut parambuf: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        if ((*wsp).ws_parambuf).is_null() {
            let mut i: size_t = 0;
            parambuf = calloc(
                (param_idx as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong),
                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            ) as *mut *mut libc::c_char;
            if parambuf.is_null() {
                return _wsplt_nomem(wsp);
            }
            i = 0 as libc::c_int as size_t;
            while i < (*wsp).ws_paramc {
                let ref mut fresh28 = *parambuf.offset(i as isize);
                *fresh28 = strdup(*((*wsp).ws_paramv).offset(i as isize));
                if (*parambuf.offset(i as isize)).is_null() {
                    while i > 1 as libc::c_int as libc::c_ulong {
                        free(
                            *parambuf
                                .offset(
                                    i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                ) as *mut libc::c_void,
                        );
                        i = i.wrapping_sub(1);
                        i;
                    }
                    free(parambuf as *mut libc::c_void);
                    return _wsplt_nomem(wsp);
                }
                i = i.wrapping_add(1);
                i;
            }
            (*wsp).ws_parambuf = parambuf;
            (*wsp).ws_paramidx = param_idx as size_t;
            (*wsp).ws_paramsiz = (param_idx + 1 as libc::c_int) as size_t;
        } else {
            let mut n: size_t = (*wsp).ws_paramsiz;
            if (-(1 as libc::c_int) as size_t)
                .wrapping_div(3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ) <= n
            {
                return _wsplt_nomem(wsp);
            }
            n = (n as libc::c_ulong)
                .wrapping_add(
                    n
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong),
                ) as size_t as size_t;
            parambuf = realloc(
                (*wsp).ws_parambuf as *mut libc::c_void,
                n
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *mut libc::c_char;
            if parambuf.is_null() {
                return _wsplt_nomem(wsp);
            }
            (*wsp).ws_parambuf = parambuf;
            (*wsp).ws_paramsiz = n;
            let ref mut fresh29 = *((*wsp).ws_parambuf).offset(param_idx as isize);
            *fresh29 = 0 as *mut libc::c_char;
        }
        (*wsp).ws_paramv = (*wsp).ws_parambuf as *mut *const libc::c_char;
        (*wsp).ws_paramc = (param_idx + 1 as libc::c_int) as size_t;
    } else if param_idx as libc::c_ulong > (*wsp).ws_paramc {
        return _wsplt_seterr(wsp, 10 as libc::c_int)
    }
    v = strdup(value);
    if v.is_null() {
        return _wsplt_nomem(wsp);
    }
    free(*((*wsp).ws_parambuf).offset(param_idx as isize) as *mut libc::c_void);
    let ref mut fresh30 = *((*wsp).ws_parambuf).offset(param_idx as isize);
    *fresh30 = v;
    return 0 as libc::c_int;
}
unsafe extern "C" fn expvar_recover(
    mut wsp: *mut wordsplit,
    mut str: *const libc::c_char,
    mut ptail: *mut *mut wordsplit_node,
    mut pend: *mut *const libc::c_char,
    mut flg: libc::c_int,
) -> libc::c_int {
    let mut newnode: *mut wordsplit_node = 0 as *mut wordsplit_node;
    if wsnode_new(wsp, &mut newnode) != 0 {
        return 1 as libc::c_int;
    }
    wsnode_insert(wsp, newnode, *ptail, 0 as libc::c_int);
    *ptail = newnode;
    (*newnode).flags = 0x2 as libc::c_int | flg;
    (*newnode).v.word = malloc(3 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    if ((*newnode).v.word).is_null() {
        return _wsplt_nomem(wsp);
    }
    *((*newnode).v.word).offset(0 as libc::c_int as isize) = '$' as i32 as libc::c_char;
    *((*newnode).v.word)
        .offset(1 as libc::c_int as isize) = *str.offset(0 as libc::c_int as isize);
    *((*newnode).v.word)
        .offset(2 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    *pend = str;
    return 0 as libc::c_int;
}
unsafe extern "C" fn expand_paramv(
    mut wsp: *mut wordsplit,
    mut ptail: *mut *mut wordsplit_node,
    mut flg: libc::c_int,
    mut q: libc::c_int,
) -> libc::c_int {
    let mut ws: wordsplit = wordsplit {
        ws_wordc: 0,
        ws_wordv: 0 as *mut *mut libc::c_char,
        ws_offs: 0,
        ws_wordn: 0,
        ws_flags: 0,
        ws_options: 0,
        ws_maxwords: 0,
        ws_wordi: 0,
        ws_delim: 0 as *const libc::c_char,
        ws_comment: 0 as *const libc::c_char,
        ws_escape: [0 as *const libc::c_char; 2],
        ws_namechar: 0 as *const libc::c_char,
        ws_alloc_die: None,
        ws_error: None,
        ws_debug: None,
        ws_env: 0 as *mut *const libc::c_char,
        ws_envbuf: 0 as *mut *mut libc::c_char,
        ws_envidx: 0,
        ws_envsiz: 0,
        ws_paramv: 0 as *mut *const libc::c_char,
        ws_paramc: 0,
        ws_parambuf: 0 as *mut *mut libc::c_char,
        ws_paramidx: 0,
        ws_paramsiz: 0,
        ws_getvar: None,
        ws_closure: 0 as *mut libc::c_void,
        ws_command: None,
        ws_input: 0 as *const libc::c_char,
        ws_len: 0,
        ws_endp: 0,
        ws_errno: 0,
        ws_usererr: 0 as *mut libc::c_char,
        ws_errctx: 0 as *mut libc::c_char,
        ws_head: 0 as *mut wordsplit_node,
        ws_tail: 0 as *mut wordsplit_node,
        ws_sep: [0; 2],
        ws_lvl: 0,
    };
    let mut wsflags: libc::c_int = 0x40 as libc::c_int | 0x4 as libc::c_int
        | (0x200 as libc::c_int | 0x400 as libc::c_int)
        | (if (*wsp).ws_flags & 0x1000 as libc::c_int != 0
            || (*wsp).ws_options & 0x80 as libc::c_int != 0
        {
            0x1000 as libc::c_int
        } else {
            0 as libc::c_int
        }) | (if q != 0 { 0x400000 as libc::c_int } else { 0 as libc::c_int });
    let mut i: size_t = 0;
    let mut tail: *mut wordsplit_node = *ptail;
    i = 0 as libc::c_int as size_t;
    while i < (*wsp).ws_paramc {
        let mut np: *mut wordsplit_node = 0 as *mut wordsplit_node;
        let mut rc: libc::c_int = _wsplt_subsplit(
            wsp,
            &mut ws,
            *((*wsp).ws_paramv).offset(i as isize),
            strlen(*((*wsp).ws_paramv).offset(i as isize)) as libc::c_int,
            wsflags,
            q,
        );
        if rc != 0 {
            _wsplt_seterr_sub(wsp, &mut ws);
            wordsplit_free(&mut ws);
            return 1 as libc::c_int;
        }
        if q != 0 {
            if wsnode_new(wsp, &mut np) != 0 {
                return 1 as libc::c_int;
            }
            wsnode_insert(wsp, np, *ptail, 0 as libc::c_int);
            *ptail = np;
            (*np).flags = 0x2 as libc::c_int | 0x8 as libc::c_int | flg;
            (*np).v.word = *(ws.ws_wordv).offset(0 as libc::c_int as isize);
            let ref mut fresh31 = *(ws.ws_wordv).offset(0 as libc::c_int as isize);
            *fresh31 = 0 as *mut libc::c_char;
        } else {
            np = ws.ws_head;
            while !np.is_null() {
                (*np).flags = 0x2 as libc::c_int | 0x8 as libc::c_int | flg;
                np = (*np).next;
            }
            wsnode_insert(wsp, ws.ws_head, *ptail, 0 as libc::c_int);
            *ptail = ws.ws_tail;
            ws.ws_tail = 0 as *mut wordsplit_node;
            ws.ws_head = ws.ws_tail;
        }
        wsflags |= 0x8 as libc::c_int;
        i = i.wrapping_add(1);
        i;
    }
    if wsflags & 0x8 as libc::c_int != 0 {
        wordsplit_free(&mut ws);
    }
    if flg & 0x4 as libc::c_int != 0 {
        tail = (*tail).next;
        while tail != *ptail {
            let mut next: *mut wordsplit_node = (*tail).next;
            let mut newnode: *mut wordsplit_node = 0 as *mut wordsplit_node;
            (*tail).flags |= 0x10 as libc::c_int;
            if wsnode_new(wsp, &mut newnode) != 0 {
                return 1 as libc::c_int;
            }
            (*newnode)
                .flags = 0x2 as libc::c_int | 0x80 as libc::c_int | 0x8 as libc::c_int
                | 0x10 as libc::c_int;
            (*newnode).v.word = ((*wsp).ws_sep).as_mut_ptr();
            wsnode_insert(wsp, newnode, tail, 0 as libc::c_int);
            tail = next;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn expvar(
    mut wsp: *mut wordsplit,
    mut str: *const libc::c_char,
    mut len: size_t,
    mut ptail: *mut *mut wordsplit_node,
    mut pend: *mut *const libc::c_char,
    mut flg: libc::c_int,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut defstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newnode: *mut wordsplit_node = 0 as *mut wordsplit_node;
    let mut start: *const libc::c_char = str.offset(-(1 as libc::c_int as isize));
    let mut rc: libc::c_int = 0;
    let mut ws: wordsplit = wordsplit {
        ws_wordc: 0,
        ws_wordv: 0 as *mut *mut libc::c_char,
        ws_offs: 0,
        ws_wordn: 0,
        ws_flags: 0,
        ws_options: 0,
        ws_maxwords: 0,
        ws_wordi: 0,
        ws_delim: 0 as *const libc::c_char,
        ws_comment: 0 as *const libc::c_char,
        ws_escape: [0 as *const libc::c_char; 2],
        ws_namechar: 0 as *const libc::c_char,
        ws_alloc_die: None,
        ws_error: None,
        ws_debug: None,
        ws_env: 0 as *mut *const libc::c_char,
        ws_envbuf: 0 as *mut *mut libc::c_char,
        ws_envidx: 0,
        ws_envsiz: 0,
        ws_paramv: 0 as *mut *const libc::c_char,
        ws_paramc: 0,
        ws_parambuf: 0 as *mut *mut libc::c_char,
        ws_paramidx: 0,
        ws_paramsiz: 0,
        ws_getvar: None,
        ws_closure: 0 as *mut libc::c_void,
        ws_command: None,
        ws_input: 0 as *const libc::c_char,
        ws_len: 0,
        ws_endp: 0,
        ws_errno: 0,
        ws_usererr: 0 as *mut libc::c_char,
        ws_errctx: 0 as *mut libc::c_char,
        ws_head: 0 as *mut wordsplit_node,
        ws_tail: 0 as *mut wordsplit_node,
        ws_sep: [0; 2],
        ws_lvl: 0,
    };
    let mut is_param: libc::c_int = 0 as libc::c_int;
    let mut param_idx: libc::c_long = 0 as libc::c_int as libc::c_long;
    if 'A' as i32 as libc::c_uint
        <= *str.offset(0 as libc::c_int as isize) as libc::c_uint
        && *str.offset(0 as libc::c_int as isize) as libc::c_uint
            <= 'Z' as i32 as libc::c_uint
        || 'a' as i32 as libc::c_uint
            <= *str.offset(0 as libc::c_int as isize) as libc::c_uint
            && *str.offset(0 as libc::c_int as isize) as libc::c_uint
                <= 'z' as i32 as libc::c_uint
        || *str.offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32
    {
        i = 1 as libc::c_int as size_t;
        while i < len {
            if is_name_char(wsp, *str.offset(i as isize) as libc::c_int) == 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
        *pend = str.offset(i as isize).offset(-(1 as libc::c_int as isize));
    } else if (*wsp).ws_options & 0x4000 as libc::c_int != 0
        && ('0' as i32 as libc::c_uint
            <= *str.offset(0 as libc::c_int as isize) as libc::c_uint
            && *str.offset(0 as libc::c_int as isize) as libc::c_uint
                <= '9' as i32 as libc::c_uint)
    {
        i = 1 as libc::c_int as size_t;
        *pend = str;
        is_param = 1 as libc::c_int;
        param_idx = (if '0' as i32 as libc::c_uint
            <= *str.offset(0 as libc::c_int as isize) as libc::c_uint
            && *str.offset(0 as libc::c_int as isize) as libc::c_uint
                <= '9' as i32 as libc::c_uint
        {
            *str.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32
        } else if !(strchr(
            b"abcdefABCDEF\0" as *const u8 as *const libc::c_char,
            *str.offset(0 as libc::c_int as isize) as libc::c_int,
        ))
            .is_null()
        {
            ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *str.offset(0 as libc::c_int as isize)
                            as libc::c_int;
                        __res = (if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = toupper(
                            *str.offset(0 as libc::c_int as isize) as libc::c_int,
                        );
                    }
                } else {
                    __res = *(*__ctype_toupper_loc())
                        .offset(
                            *str.offset(0 as libc::c_int as isize) as libc::c_int
                                as isize,
                        );
                }
                __res
            }) - 'A' as i32 + 10 as libc::c_int
        } else {
            255 as libc::c_int
        }) as libc::c_long;
    } else if (*wsp).ws_options & 0x4000 as libc::c_int != 0
        && *str.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
    {
        let mut b: [libc::c_char; 16] = [0; 16];
        snprintf(
            b.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
            b"%d\0" as *const u8 as *const libc::c_char,
            (*wsp).ws_paramc as libc::c_int,
        );
        value = strdup(b.as_mut_ptr());
        if value.is_null() {
            return _wsplt_nomem(wsp);
        }
        if wsnode_new(wsp, &mut newnode) != 0 {
            return 1 as libc::c_int;
        }
        wsnode_insert(wsp, newnode, *ptail, 0 as libc::c_int);
        *ptail = newnode;
        (*newnode).flags = 0x2 as libc::c_int | 0x8 as libc::c_int | flg;
        (*newnode).v.word = value;
        return 0 as libc::c_int;
    } else if (*wsp).ws_options & 0x4000 as libc::c_int != 0
        && *str.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32
    {
        return expand_paramv(wsp, ptail, flg, 0 as libc::c_int)
    } else if (*wsp).ws_options & 0x4000 as libc::c_int != 0
        && *str.offset(0 as libc::c_int as isize) as libc::c_int == '@' as i32
    {
        return expand_paramv(wsp, ptail, flg, 1 as libc::c_int)
    } else if *str.offset(0 as libc::c_int as isize) as libc::c_int == '{' as i32
        && ('A' as i32 as libc::c_uint
            <= *str.offset(1 as libc::c_int as isize) as libc::c_uint
            && *str.offset(1 as libc::c_int as isize) as libc::c_uint
                <= 'Z' as i32 as libc::c_uint
            || 'a' as i32 as libc::c_uint
                <= *str.offset(1 as libc::c_int as isize) as libc::c_uint
                && *str.offset(1 as libc::c_int as isize) as libc::c_uint
                    <= 'z' as i32 as libc::c_uint
            || *str.offset(1 as libc::c_int as isize) as libc::c_int == '_' as i32
            || {
                is_param = ((*wsp).ws_options & 0x4000 as libc::c_int != 0
                    && ('0' as i32 as libc::c_uint
                        <= *str.offset(1 as libc::c_int as isize) as libc::c_uint
                        && *str.offset(1 as libc::c_int as isize) as libc::c_uint
                            <= '9' as i32 as libc::c_uint)
                    || (*wsp).ws_options & 0x8000 as libc::c_int != 0
                        && (*str.offset(1 as libc::c_int as isize) as libc::c_int
                            == '-' as i32
                            && ('0' as i32 as libc::c_uint
                                <= *str.offset(2 as libc::c_int as isize) as libc::c_uint
                                && *str.offset(2 as libc::c_int as isize) as libc::c_uint
                                    <= '9' as i32 as libc::c_uint))) as libc::c_int;
                is_param != 0 as libc::c_int
            })
    {
        let mut i0: libc::c_int = if *str.offset(0 as libc::c_int as isize)
            as libc::c_int == '-' as i32
        {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        str = str.offset(1);
        str;
        len = len.wrapping_sub(1);
        len;
        i = i0 as size_t;
        while i < len {
            if *str.offset(i as isize) as libc::c_int == '}' as i32 {
                defstr = 0 as *const libc::c_char;
                *pend = str.offset(i as isize);
                break;
            } else if !(strchr(
                b"-+?=\0" as *const u8 as *const libc::c_char,
                *str.offset(i as isize) as libc::c_int,
            ))
                .is_null()
            {
                let mut j: size_t = 0;
                defstr = str.offset(i as isize);
                if find_closing_paren(
                    str,
                    i,
                    len,
                    &mut j,
                    b"{}\0" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    return _wsplt_seterr(wsp, 4 as libc::c_int);
                }
                if i > (i0 + 1 as libc::c_int) as libc::c_ulong
                    && *str
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int == ':' as i32
                {
                    i = i.wrapping_sub(1);
                    i;
                }
                *pend = str.offset(j as isize);
                break;
            } else {
                if is_param != 0 {
                    if '0' as i32 as libc::c_uint
                        <= *str.offset(i as isize) as libc::c_uint
                        && *str.offset(i as isize) as libc::c_uint
                            <= '9' as i32 as libc::c_uint
                    {
                        param_idx = param_idx * 10 as libc::c_int as libc::c_long
                            + (if '0' as i32 as libc::c_uint
                                <= *str.offset(i as isize) as libc::c_uint
                                && *str.offset(i as isize) as libc::c_uint
                                    <= '9' as i32 as libc::c_uint
                            {
                                *str.offset(i as isize) as libc::c_int - '0' as i32
                            } else {
                                (if !(strchr(
                                    b"abcdefABCDEF\0" as *const u8 as *const libc::c_char,
                                    *str.offset(i as isize) as libc::c_int,
                                ))
                                    .is_null()
                                {
                                    ({
                                        let mut __res: libc::c_int = 0;
                                        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                            > 1 as libc::c_int as libc::c_ulong
                                        {
                                            if 0 != 0 {
                                                let mut __c: libc::c_int = *str.offset(i as isize)
                                                    as libc::c_int;
                                                __res = (if __c < -(128 as libc::c_int)
                                                    || __c > 255 as libc::c_int
                                                {
                                                    __c
                                                } else {
                                                    *(*__ctype_toupper_loc()).offset(__c as isize)
                                                });
                                            } else {
                                                __res = toupper(*str.offset(i as isize) as libc::c_int);
                                            }
                                        } else {
                                            __res = *(*__ctype_toupper_loc())
                                                .offset(*str.offset(i as isize) as libc::c_int as isize);
                                        }
                                        __res
                                    }) - 'A' as i32 + 10 as libc::c_int
                                } else {
                                    255 as libc::c_int
                                })
                            }) as libc::c_long;
                        if *str.offset(0 as libc::c_int as isize) as libc::c_int
                            == '-' as i32
                            && -param_idx
                                < (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long
                            || param_idx > 2147483647 as libc::c_int as libc::c_long
                        {
                            return expvar_recover(
                                wsp,
                                str.offset(-(1 as libc::c_int as isize)),
                                ptail,
                                pend,
                                flg,
                            );
                        }
                    } else {
                        return expvar_recover(
                            wsp,
                            str.offset(-(1 as libc::c_int as isize)),
                            ptail,
                            pend,
                            flg,
                        )
                    }
                } else if is_name_char(wsp, *str.offset(i as isize) as libc::c_int) == 0
                {
                    if !(*str.offset(i as isize) as libc::c_int == ':' as i32
                        && i.wrapping_add(1 as libc::c_int as libc::c_ulong) < len
                        && !(strchr(
                            b"-+?=\0" as *const u8 as *const libc::c_char,
                            *str
                                .offset(
                                    i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                ) as libc::c_int,
                        ))
                            .is_null())
                    {
                        return expvar_recover(
                            wsp,
                            str.offset(-(1 as libc::c_int as isize)),
                            ptail,
                            pend,
                            flg,
                        );
                    }
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        if is_param != 0
            && *str.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
        {
            param_idx = ((*wsp).ws_paramc).wrapping_sub(param_idx as libc::c_ulong)
                as libc::c_long;
        }
        if i == len {
            return _wsplt_seterr(wsp, 4 as libc::c_int);
        }
    } else {
        return expvar_recover(wsp, str, ptail, pend, flg)
    }
    if is_param != 0 {
        if param_idx >= 0 as libc::c_int as libc::c_long
            && (param_idx as libc::c_ulong) < (*wsp).ws_paramc
        {
            value = strdup(*((*wsp).ws_paramv).offset(param_idx as isize));
            if value.is_null() {
                rc = 2 as libc::c_int;
            } else {
                rc = 0 as libc::c_int;
            }
        } else {
            rc = 5 as libc::c_int;
        }
    } else if (*wsp).ws_flags & 0x100000 as libc::c_int != 0 {
        if (*wsp).ws_options & 0x8 as libc::c_int != 0 {
            rc = wsplt_env_getvar(wsp, str, i, &mut value);
            if rc == 5 as libc::c_int {
                rc = wsplt_env_lookup(wsp, str, i, &mut value);
            }
        } else {
            rc = wsplt_env_lookup(wsp, str, i, &mut value);
            if rc == 5 as libc::c_int {
                rc = wsplt_env_getvar(wsp, str, i, &mut value);
            }
        }
    } else {
        rc = wsplt_env_lookup(wsp, str, i, &mut value);
    }
    if rc == 0 as libc::c_int
        && (value.is_null()
            || *value.offset(0 as libc::c_int as isize) as libc::c_int
                == 0 as libc::c_int) && !defstr.is_null()
        && *defstr.offset(-(1 as libc::c_int) as isize) as libc::c_int == ':' as i32
    {
        free(value as *mut libc::c_void);
        rc = 5 as libc::c_int;
    }
    let mut current_block_145: u64;
    match rc {
        0 => {
            if !defstr.is_null() && *defstr as libc::c_int == '+' as i32 {
                defstr = defstr.offset(1);
                let mut size: size_t = (*pend).offset_from(defstr) as libc::c_long
                    as size_t;
                rc = _wsplt_subsplit(
                    wsp,
                    &mut ws,
                    defstr,
                    size as libc::c_int,
                    0x400000 as libc::c_int | 0x100 as libc::c_int
                        | (0x200 as libc::c_int | 0x400 as libc::c_int)
                        | (*wsp).ws_flags & (0x40 as libc::c_int | 0x4 as libc::c_int),
                    1 as libc::c_int,
                );
                if rc != 0 {
                    return rc;
                }
                free(value as *mut libc::c_void);
                value = *(ws.ws_wordv).offset(0 as libc::c_int as isize);
                let ref mut fresh32 = *(ws.ws_wordv).offset(0 as libc::c_int as isize);
                *fresh32 = 0 as *mut libc::c_char;
                wordsplit_free(&mut ws);
            }
            current_block_145 = 17736998403848444560;
        }
        5 => {
            if !defstr.is_null() {
                let mut size_0: size_t = 0;
                if *defstr as libc::c_int == '-' as i32
                    || *defstr as libc::c_int == '=' as i32
                {
                    defstr = defstr.offset(1);
                    size_0 = (*pend).offset_from(defstr) as libc::c_long as size_t;
                    rc = _wsplt_subsplit(
                        wsp,
                        &mut ws,
                        defstr,
                        size_0 as libc::c_int,
                        0x400000 as libc::c_int | 0x100 as libc::c_int
                            | (0x200 as libc::c_int | 0x400 as libc::c_int)
                            | (*wsp).ws_flags
                                & (0x40 as libc::c_int | 0x4 as libc::c_int),
                        1 as libc::c_int,
                    );
                    if rc != 0 {
                        return rc;
                    }
                    value = *(ws.ws_wordv).offset(0 as libc::c_int as isize);
                    let ref mut fresh33 = *(ws.ws_wordv)
                        .offset(0 as libc::c_int as isize);
                    *fresh33 = 0 as *mut libc::c_char;
                    wordsplit_free(&mut ws);
                    if *defstr.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '=' as i32
                    {
                        if is_param != 0 {
                            rc = wsplt_assign_param(
                                wsp,
                                param_idx as libc::c_int,
                                value,
                            );
                        } else {
                            rc = wsplt_assign_var(wsp, str, i, value);
                        }
                    }
                    if rc != 0 {
                        free(value as *mut libc::c_void);
                        return rc;
                    }
                } else {
                    if *defstr as libc::c_int == '?' as i32 {
                        defstr = defstr.offset(1);
                        size_0 = (*pend).offset_from(defstr) as libc::c_long as size_t;
                        if size_0 == 0 as libc::c_int as libc::c_ulong {
                            ((*wsp).ws_error)
                                .unwrap()(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"%.*s: variable null or not set\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                i as libc::c_int,
                                str,
                            );
                        } else {
                            rc = _wsplt_subsplit(
                                wsp,
                                &mut ws,
                                defstr,
                                size_0 as libc::c_int,
                                0x400000 as libc::c_int | 0x100 as libc::c_int
                                    | (0x200 as libc::c_int | 0x400 as libc::c_int)
                                    | (*wsp).ws_flags
                                        & (0x40 as libc::c_int | 0x4 as libc::c_int),
                                1 as libc::c_int,
                            );
                            if rc == 0 as libc::c_int {
                                ((*wsp).ws_error)
                                    .unwrap()(
                                    b"%.*s: %s\0" as *const u8 as *const libc::c_char,
                                    i as libc::c_int,
                                    str,
                                    *(ws.ws_wordv).offset(0 as libc::c_int as isize),
                                );
                            } else {
                                ((*wsp).ws_error)
                                    .unwrap()(
                                    b"%.*s: %.*s\0" as *const u8 as *const libc::c_char,
                                    i as libc::c_int,
                                    str,
                                    size_0 as libc::c_int,
                                    defstr,
                                );
                            }
                            wordsplit_free(&mut ws);
                        }
                    }
                    value = 0 as *mut libc::c_char;
                }
            } else if (*wsp).ws_flags & 0x20 as libc::c_int != 0 {
                _wsplt_setctxerr(wsp, 5 as libc::c_int, str, i);
                return 1 as libc::c_int;
            } else {
                if (*wsp).ws_flags & 0x1000000 as libc::c_int != 0 {
                    ((*wsp).ws_error)
                        .unwrap()(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"warning: undefined variable `%.*s'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        i as libc::c_int,
                        str,
                    );
                }
                if (*wsp).ws_flags & 0x800000 as libc::c_int != 0 {
                    value = 0 as *mut libc::c_char;
                } else {
                    value = strdup(b"\0" as *const u8 as *const libc::c_char);
                    if value.is_null() {
                        return _wsplt_nomem(wsp);
                    }
                }
            }
            current_block_145 = 17736998403848444560;
        }
        2 => return _wsplt_nomem(wsp),
        9 => {
            if (*wsp).ws_errno == 9 as libc::c_int {
                free((*wsp).ws_usererr as *mut libc::c_void);
            }
            (*wsp).ws_usererr = value;
            current_block_145 = 13250156143392460182;
        }
        _ => {
            current_block_145 = 13250156143392460182;
        }
    }
    match current_block_145 {
        17736998403848444560 => {}
        _ => {
            _wsplt_seterr(wsp, rc);
            return 1 as libc::c_int;
        }
    }
    if !value.is_null() {
        if flg & 0x4 as libc::c_int != 0 {
            if wsnode_new(wsp, &mut newnode) != 0 {
                return 1 as libc::c_int;
            }
            wsnode_insert(wsp, newnode, *ptail, 0 as libc::c_int);
            *ptail = newnode;
            (*newnode).flags = 0x2 as libc::c_int | 0x8 as libc::c_int | flg;
            (*newnode).v.word = value;
        } else if *value as libc::c_int == 0 as libc::c_int {
            free(value as *mut libc::c_void);
            if wsnode_new(wsp, &mut newnode) != 0 {
                return 1 as libc::c_int;
            }
            wsnode_insert(wsp, newnode, *ptail, 0 as libc::c_int);
            *ptail = newnode;
            (*newnode).flags = 0x1 as libc::c_int;
        } else {
            let mut ws_0: wordsplit = wordsplit {
                ws_wordc: 0,
                ws_wordv: 0 as *mut *mut libc::c_char,
                ws_offs: 0,
                ws_wordn: 0,
                ws_flags: 0,
                ws_options: 0,
                ws_maxwords: 0,
                ws_wordi: 0,
                ws_delim: 0 as *const libc::c_char,
                ws_comment: 0 as *const libc::c_char,
                ws_escape: [0 as *const libc::c_char; 2],
                ws_namechar: 0 as *const libc::c_char,
                ws_alloc_die: None,
                ws_error: None,
                ws_debug: None,
                ws_env: 0 as *mut *const libc::c_char,
                ws_envbuf: 0 as *mut *mut libc::c_char,
                ws_envidx: 0,
                ws_envsiz: 0,
                ws_paramv: 0 as *mut *const libc::c_char,
                ws_paramc: 0,
                ws_parambuf: 0 as *mut *mut libc::c_char,
                ws_paramidx: 0,
                ws_paramsiz: 0,
                ws_getvar: None,
                ws_closure: 0 as *mut libc::c_void,
                ws_command: None,
                ws_input: 0 as *const libc::c_char,
                ws_len: 0,
                ws_endp: 0,
                ws_errno: 0,
                ws_usererr: 0 as *mut libc::c_char,
                ws_errctx: 0 as *mut libc::c_char,
                ws_head: 0 as *mut wordsplit_node,
                ws_tail: 0 as *mut wordsplit_node,
                ws_sep: [0; 2],
                ws_lvl: 0,
            };
            let mut rc_0: libc::c_int = 0;
            rc_0 = _wsplt_subsplit(
                wsp,
                &mut ws_0,
                value,
                strlen(value) as libc::c_int,
                0x40 as libc::c_int | 0x4 as libc::c_int
                    | (0x200 as libc::c_int | 0x400 as libc::c_int)
                    | (if (*wsp).ws_flags & 0x1000 as libc::c_int != 0
                        || (*wsp).ws_options & 0x80 as libc::c_int != 0
                    {
                        0x1000 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }),
                0 as libc::c_int,
            );
            free(value as *mut libc::c_void);
            if rc_0 != 0 {
                _wsplt_seterr_sub(wsp, &mut ws_0);
                wordsplit_free(&mut ws_0);
                return 1 as libc::c_int;
            }
            wsnode_insert(wsp, ws_0.ws_head, *ptail, 0 as libc::c_int);
            *ptail = ws_0.ws_tail;
            ws_0.ws_tail = 0 as *mut wordsplit_node;
            ws_0.ws_head = ws_0.ws_tail;
            wordsplit_free(&mut ws_0);
        }
    } else if (*wsp).ws_flags & 0x800000 as libc::c_int != 0 {
        let mut size_1: size_t = ((*pend).offset_from(start) as libc::c_long
            + 1 as libc::c_int as libc::c_long) as size_t;
        if wsnode_new(wsp, &mut newnode) != 0 {
            return 1 as libc::c_int;
        }
        wsnode_insert(wsp, newnode, *ptail, 0 as libc::c_int);
        *ptail = newnode;
        (*newnode).flags = 0x2 as libc::c_int | 0x8 as libc::c_int | flg;
        (*newnode)
            .v
            .word = malloc(size_1.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        if ((*newnode).v.word).is_null() {
            return _wsplt_nomem(wsp);
        }
        memcpy(
            (*newnode).v.word as *mut libc::c_void,
            start as *const libc::c_void,
            size_1,
        );
        *((*newnode).v.word).offset(size_1 as isize) = 0 as libc::c_int as libc::c_char;
    } else {
        if wsnode_new(wsp, &mut newnode) != 0 {
            return 1 as libc::c_int;
        }
        wsnode_insert(wsp, newnode, *ptail, 0 as libc::c_int);
        *ptail = newnode;
        (*newnode).flags = 0x1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn begin_var_p(mut c: libc::c_int) -> libc::c_int {
    return (!(memchr(
        b"{#@*\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        c,
        4 as libc::c_int as libc::c_ulong,
    ))
        .is_null()
        || ('A' as i32 as libc::c_uint <= c as libc::c_uint
            && c as libc::c_uint <= 'Z' as i32 as libc::c_uint
            || 'a' as i32 as libc::c_uint <= c as libc::c_uint
                && c as libc::c_uint <= 'z' as i32 as libc::c_uint || c == '_' as i32)
        || '0' as i32 as libc::c_uint <= c as libc::c_uint
            && c as libc::c_uint <= '9' as i32 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn node_expand(
    mut wsp: *mut wordsplit,
    mut node: *mut wordsplit_node,
    mut beg_p: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    mut ws_exp_fn: Option::<
        unsafe extern "C" fn(
            *mut wordsplit,
            *const libc::c_char,
            size_t,
            *mut *mut wordsplit_node,
            *mut *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
) -> libc::c_int {
    let mut str: *const libc::c_char = wsnode_ptr(wsp, node);
    let mut slen: size_t = wsnode_len(node);
    let mut end: *const libc::c_char = str.offset(slen as isize);
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut off: size_t = 0 as libc::c_int as size_t;
    let mut tail: *mut wordsplit_node = node;
    p = str;
    while p < end {
        if *p as libc::c_int == '\\' as i32 {
            p = p.offset(1);
            p;
        } else if *p as libc::c_int == '$' as i32
            && beg_p.unwrap()(*p.offset(1 as libc::c_int as isize) as libc::c_int) != 0
        {
            let mut n: size_t = p.offset_from(str) as libc::c_long as size_t;
            if tail != node {
                (*tail).flags |= 0x10 as libc::c_int;
            }
            if node_split_prefix(wsp, &mut tail, node, off, n, 0x10 as libc::c_int) != 0
            {
                return 1 as libc::c_int;
            }
            p = p.offset(1);
            p;
            if ws_exp_fn
                .unwrap()(
                wsp,
                p,
                slen.wrapping_sub(n),
                &mut tail,
                &mut p,
                (*node).flags & (0x10 as libc::c_int | 0x4 as libc::c_int),
            ) != 0
            {
                return 1 as libc::c_int;
            }
            off = (off as libc::c_ulong)
                .wrapping_add(
                    (p.offset_from(str) as libc::c_long
                        + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                ) as size_t as size_t;
            str = p.offset(1 as libc::c_int as isize);
        }
        p = p.offset(1);
        p;
    }
    if p > str {
        if tail != node {
            (*tail).flags |= 0x10 as libc::c_int;
        }
        if node_split_prefix(
            wsp,
            &mut tail,
            node,
            off,
            p.offset_from(str) as libc::c_long as size_t,
            (*node).flags & (0x10 as libc::c_int | 0x4 as libc::c_int),
        ) != 0
        {
            return 1 as libc::c_int;
        }
    }
    if tail != node {
        wsnode_remove(wsp, node);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn wsnode_nullelim(mut wsp: *mut wordsplit) {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    p = (*wsp).ws_head;
    while !p.is_null() {
        let mut next: *mut wordsplit_node = (*p).next;
        if (*p).flags & 0x40 as libc::c_int != 0 && !((*p).prev).is_null() {
            (*(*p).prev).flags &= !(0x10 as libc::c_int);
        }
        if (*p).flags & 0x1 as libc::c_int != 0 {
            wsnode_remove(wsp, p);
        }
        p = next;
    }
}
unsafe extern "C" fn wordsplit_varexp(mut wsp: *mut wordsplit) -> libc::c_int {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    p = (*wsp).ws_head;
    while !p.is_null() {
        let mut next: *mut wordsplit_node = (*p).next;
        if (*p).flags & (0x8 as libc::c_int | 0x40 as libc::c_int) == 0 {
            if node_expand(
                wsp,
                p,
                Some(begin_var_p as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
                Some(
                    expvar
                        as unsafe extern "C" fn(
                            *mut wordsplit,
                            *const libc::c_char,
                            size_t,
                            *mut *mut wordsplit_node,
                            *mut *const libc::c_char,
                            libc::c_int,
                        ) -> libc::c_int,
                ),
            ) != 0
            {
                return 1 as libc::c_int;
            }
        }
        p = next;
    }
    wsnode_nullelim(wsp);
    return 0 as libc::c_int;
}
unsafe extern "C" fn begin_cmd_p(mut c: libc::c_int) -> libc::c_int {
    return (c == '(' as i32) as libc::c_int;
}
unsafe extern "C" fn expcmd(
    mut wsp: *mut wordsplit,
    mut str: *const libc::c_char,
    mut len: size_t,
    mut ptail: *mut *mut wordsplit_node,
    mut pend: *mut *const libc::c_char,
    mut flg: libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut j: size_t = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newnode: *mut wordsplit_node = 0 as *mut wordsplit_node;
    let mut ws: wordsplit = wordsplit {
        ws_wordc: 0,
        ws_wordv: 0 as *mut *mut libc::c_char,
        ws_offs: 0,
        ws_wordn: 0,
        ws_flags: 0,
        ws_options: 0,
        ws_maxwords: 0,
        ws_wordi: 0,
        ws_delim: 0 as *const libc::c_char,
        ws_comment: 0 as *const libc::c_char,
        ws_escape: [0 as *const libc::c_char; 2],
        ws_namechar: 0 as *const libc::c_char,
        ws_alloc_die: None,
        ws_error: None,
        ws_debug: None,
        ws_env: 0 as *mut *const libc::c_char,
        ws_envbuf: 0 as *mut *mut libc::c_char,
        ws_envidx: 0,
        ws_envsiz: 0,
        ws_paramv: 0 as *mut *const libc::c_char,
        ws_paramc: 0,
        ws_parambuf: 0 as *mut *mut libc::c_char,
        ws_paramidx: 0,
        ws_paramsiz: 0,
        ws_getvar: None,
        ws_closure: 0 as *mut libc::c_void,
        ws_command: None,
        ws_input: 0 as *const libc::c_char,
        ws_len: 0,
        ws_endp: 0,
        ws_errno: 0,
        ws_usererr: 0 as *mut libc::c_char,
        ws_errctx: 0 as *mut libc::c_char,
        ws_head: 0 as *mut wordsplit_node,
        ws_tail: 0 as *mut wordsplit_node,
        ws_sep: [0; 2],
        ws_lvl: 0,
    };
    str = str.offset(1);
    str;
    len = len.wrapping_sub(1);
    len;
    if find_closing_paren(
        str,
        0 as libc::c_int as size_t,
        len,
        &mut j,
        b"()\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        _wsplt_seterr(wsp, 7 as libc::c_int);
        return 1 as libc::c_int;
    }
    *pend = str.offset(j as isize);
    rc = _wsplt_subsplit(
        wsp,
        &mut ws,
        str,
        j as libc::c_int,
        0x100 as libc::c_int | (0x200 as libc::c_int | 0x400 as libc::c_int),
        1 as libc::c_int,
    );
    if rc != 0 {
        _wsplt_seterr_sub(wsp, &mut ws);
        wordsplit_free(&mut ws);
        return 1 as libc::c_int;
    }
    rc = ((*wsp).ws_command)
        .unwrap()(&mut value, str, j, ws.ws_wordv, (*wsp).ws_closure);
    wordsplit_free(&mut ws);
    if rc == 2 as libc::c_int {
        return _wsplt_nomem(wsp)
    } else if rc != 0 {
        if rc == 9 as libc::c_int {
            if (*wsp).ws_errno == 9 as libc::c_int {
                free((*wsp).ws_usererr as *mut libc::c_void);
            }
            (*wsp).ws_usererr = value;
        }
        _wsplt_seterr(wsp, rc);
        return 1 as libc::c_int;
    }
    if !value.is_null() {
        if flg & 0x4 as libc::c_int != 0 {
            if wsnode_new(wsp, &mut newnode) != 0 {
                return 1 as libc::c_int;
            }
            wsnode_insert(wsp, newnode, *ptail, 0 as libc::c_int);
            *ptail = newnode;
            (*newnode).flags = 0x2 as libc::c_int | 0x8 as libc::c_int | flg;
            (*newnode).v.word = value;
        } else if *value as libc::c_int == 0 as libc::c_int {
            free(value as *mut libc::c_void);
            if wsnode_new(wsp, &mut newnode) != 0 {
                return 1 as libc::c_int;
            }
            wsnode_insert(wsp, newnode, *ptail, 0 as libc::c_int);
            *ptail = newnode;
            (*newnode).flags = 0x1 as libc::c_int;
        } else {
            let mut ws_0: wordsplit = wordsplit {
                ws_wordc: 0,
                ws_wordv: 0 as *mut *mut libc::c_char,
                ws_offs: 0,
                ws_wordn: 0,
                ws_flags: 0,
                ws_options: 0,
                ws_maxwords: 0,
                ws_wordi: 0,
                ws_delim: 0 as *const libc::c_char,
                ws_comment: 0 as *const libc::c_char,
                ws_escape: [0 as *const libc::c_char; 2],
                ws_namechar: 0 as *const libc::c_char,
                ws_alloc_die: None,
                ws_error: None,
                ws_debug: None,
                ws_env: 0 as *mut *const libc::c_char,
                ws_envbuf: 0 as *mut *mut libc::c_char,
                ws_envidx: 0,
                ws_envsiz: 0,
                ws_paramv: 0 as *mut *const libc::c_char,
                ws_paramc: 0,
                ws_parambuf: 0 as *mut *mut libc::c_char,
                ws_paramidx: 0,
                ws_paramsiz: 0,
                ws_getvar: None,
                ws_closure: 0 as *mut libc::c_void,
                ws_command: None,
                ws_input: 0 as *const libc::c_char,
                ws_len: 0,
                ws_endp: 0,
                ws_errno: 0,
                ws_usererr: 0 as *mut libc::c_char,
                ws_errctx: 0 as *mut libc::c_char,
                ws_head: 0 as *mut wordsplit_node,
                ws_tail: 0 as *mut wordsplit_node,
                ws_sep: [0; 2],
                ws_lvl: 0,
            };
            let mut rc_0: libc::c_int = 0;
            rc_0 = _wsplt_subsplit(
                wsp,
                &mut ws_0,
                value,
                strlen(value) as libc::c_int,
                0x40 as libc::c_int | 0x4 as libc::c_int | 0x100 as libc::c_int
                    | (0x200 as libc::c_int | 0x400 as libc::c_int)
                    | (if (*wsp).ws_flags & 0x1000 as libc::c_int != 0
                        || (*wsp).ws_options & 0x80 as libc::c_int != 0
                    {
                        0x1000 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }),
                0 as libc::c_int,
            );
            free(value as *mut libc::c_void);
            if rc_0 != 0 {
                _wsplt_seterr_sub(wsp, &mut ws_0);
                wordsplit_free(&mut ws_0);
                return 1 as libc::c_int;
            }
            wsnode_insert(wsp, ws_0.ws_head, *ptail, 0 as libc::c_int);
            *ptail = ws_0.ws_tail;
            ws_0.ws_tail = 0 as *mut wordsplit_node;
            ws_0.ws_head = ws_0.ws_tail;
            wordsplit_free(&mut ws_0);
        }
    } else {
        if wsnode_new(wsp, &mut newnode) != 0 {
            return 1 as libc::c_int;
        }
        wsnode_insert(wsp, newnode, *ptail, 0 as libc::c_int);
        *ptail = newnode;
        (*newnode).flags = 0x1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn wordsplit_cmdexp(mut wsp: *mut wordsplit) -> libc::c_int {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    p = (*wsp).ws_head;
    while !p.is_null() {
        let mut next: *mut wordsplit_node = (*p).next;
        if (*p).flags & 0x8 as libc::c_int == 0 {
            if node_expand(
                wsp,
                p,
                Some(begin_cmd_p as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
                Some(
                    expcmd
                        as unsafe extern "C" fn(
                            *mut wordsplit,
                            *const libc::c_char,
                            size_t,
                            *mut *mut wordsplit_node,
                            *mut *const libc::c_char,
                            libc::c_int,
                        ) -> libc::c_int,
                ),
            ) != 0
            {
                return 1 as libc::c_int;
            }
        }
        p = next;
    }
    wsnode_nullelim(wsp);
    return 0 as libc::c_int;
}
unsafe extern "C" fn wordsplit_trimws(mut wsp: *mut wordsplit) -> libc::c_int {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    p = (*wsp).ws_head;
    while !p.is_null() {
        let mut n: size_t = 0;
        if (*p).flags & 0x4 as libc::c_int == 0 {
            n = (*p).v.segm.beg;
            while n < (*p).v.segm.end
                && (*((*wsp).ws_input).offset(n as isize) as libc::c_int == ' ' as i32
                    || *((*wsp).ws_input).offset(n as isize) as libc::c_int
                        == '\t' as i32
                    || *((*wsp).ws_input).offset(n as isize) as libc::c_int
                        == '\n' as i32)
            {
                n = n.wrapping_add(1);
                n;
            }
            (*p).v.segm.beg = n;
        }
        while !((*p).next).is_null() && (*p).flags & 0x10 as libc::c_int != 0 {
            p = (*p).next;
        }
        if !((*p).flags & 0x4 as libc::c_int != 0) {
            n = (*p).v.segm.end;
            while n > (*p).v.segm.beg
                && (*((*wsp).ws_input)
                    .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int == ' ' as i32
                    || *((*wsp).ws_input)
                        .offset(
                            n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int == '\t' as i32
                    || *((*wsp).ws_input)
                        .offset(
                            n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int == '\n' as i32)
            {
                n = n.wrapping_sub(1);
                n;
            }
            (*p).v.segm.end = n;
            if (*p).v.segm.beg == (*p).v.segm.end {
                (*p).flags |= 0x1 as libc::c_int;
            }
        }
        p = (*p).next;
    }
    wsnode_nullelim(wsp);
    return 0 as libc::c_int;
}
unsafe extern "C" fn wordsplit_tildexpand(mut wsp: *mut wordsplit) -> libc::c_int {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    let mut uname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut usize: size_t = 0 as libc::c_int as size_t;
    p = (*wsp).ws_head;
    while !p.is_null() {
        let mut str: *const libc::c_char = 0 as *const libc::c_char;
        if !((*p).flags & 0x4 as libc::c_int != 0) {
            str = wsnode_ptr(wsp, p);
            if *str.offset(0 as libc::c_int as isize) as libc::c_int == '~' as i32 {
                let mut i: size_t = 0;
                let mut size: size_t = 0;
                let mut dlen: size_t = 0;
                let mut slen: size_t = wsnode_len(p);
                let mut pw: *mut passwd = 0 as *mut passwd;
                let mut newstr: *mut libc::c_char = 0 as *mut libc::c_char;
                i = 1 as libc::c_int as size_t;
                while i < slen && *str.offset(i as isize) as libc::c_int != '/' as i32 {
                    i = i.wrapping_add(1);
                    i;
                }
                if !(i == slen) {
                    if i > 1 as libc::c_int as libc::c_ulong {
                        if i > usize {
                            let mut p_0: *mut libc::c_char = realloc(
                                uname as *mut libc::c_void,
                                i,
                            ) as *mut libc::c_char;
                            if p_0.is_null() {
                                free(uname as *mut libc::c_void);
                                return _wsplt_nomem(wsp);
                            }
                            uname = p_0;
                            usize = i;
                        }
                        i = i.wrapping_sub(1);
                        i;
                        memcpy(
                            uname as *mut libc::c_void,
                            str.offset(1 as libc::c_int as isize) as *const libc::c_void,
                            i,
                        );
                        *uname.offset(i as isize) = 0 as libc::c_int as libc::c_char;
                        pw = getpwnam(uname);
                    } else {
                        pw = getpwuid(getuid());
                    }
                    if !pw.is_null() {
                        dlen = strlen((*pw).pw_dir);
                        size = slen.wrapping_sub(i).wrapping_add(dlen);
                        newstr = malloc(size) as *mut libc::c_char;
                        if newstr.is_null() {
                            free(uname as *mut libc::c_void);
                            return _wsplt_nomem(wsp);
                        }
                        size = size.wrapping_sub(1);
                        size;
                        memcpy(
                            newstr as *mut libc::c_void,
                            (*pw).pw_dir as *const libc::c_void,
                            dlen,
                        );
                        memcpy(
                            newstr.offset(dlen as isize) as *mut libc::c_void,
                            str.offset(i as isize).offset(1 as libc::c_int as isize)
                                as *const libc::c_void,
                            slen
                                .wrapping_sub(i)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        );
                        *newstr.offset(size as isize) = 0 as libc::c_int as libc::c_char;
                        if (*p).flags & 0x2 as libc::c_int != 0 {
                            free((*p).v.word as *mut libc::c_void);
                        }
                        (*p).v.word = newstr;
                        (*p).flags |= 0x2 as libc::c_int;
                    }
                }
            }
        }
        p = (*p).next;
    }
    free(uname as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn isglob(
    mut s: *const libc::c_char,
    mut l: libc::c_int,
) -> libc::c_int {
    loop {
        let fresh34 = l;
        l = l - 1;
        if !(fresh34 != 0) {
            break;
        }
        let fresh35 = s;
        s = s.offset(1);
        if !(strchr(
            b"*?[\0" as *const u8 as *const libc::c_char,
            *fresh35 as libc::c_int,
        ))
            .is_null()
        {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn wordsplit_pathexpand(mut wsp: *mut wordsplit) -> libc::c_int {
    let mut p: *mut wordsplit_node = 0 as *mut wordsplit_node;
    let mut next: *mut wordsplit_node = 0 as *mut wordsplit_node;
    let mut pattern: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut patsize: size_t = 0 as libc::c_int as size_t;
    let mut slen: size_t = 0;
    let mut flags: libc::c_int = 0 as libc::c_int;
    if (*wsp).ws_options & 0x4 as libc::c_int != 0 {
        flags = (1 as libc::c_int) << 7 as libc::c_int;
    }
    p = (*wsp).ws_head;
    while !p.is_null() {
        let mut str: *const libc::c_char = 0 as *const libc::c_char;
        next = (*p).next;
        if !((*p).flags & 0x4 as libc::c_int != 0) {
            str = wsnode_ptr(wsp, p);
            slen = wsnode_len(p);
            if isglob(str, slen as libc::c_int) != 0 {
                let mut i: libc::c_int = 0;
                let mut g: glob_t = glob_t {
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
                let mut prev: *mut wordsplit_node = 0 as *mut wordsplit_node;
                if slen.wrapping_add(1 as libc::c_int as libc::c_ulong) > patsize {
                    let mut p_0: *mut libc::c_char = realloc(
                        pattern as *mut libc::c_void,
                        slen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as *mut libc::c_char;
                    if p_0.is_null() {
                        return _wsplt_nomem(wsp);
                    }
                    pattern = p_0;
                    patsize = slen.wrapping_add(1 as libc::c_int as libc::c_ulong);
                }
                memcpy(pattern as *mut libc::c_void, str as *const libc::c_void, slen);
                *pattern.offset(slen as isize) = 0 as libc::c_int as libc::c_char;
                match glob(pattern, flags, None, &mut g) {
                    0 => {
                        prev = p;
                        i = 0 as libc::c_int;
                        while (i as libc::c_ulong) < g.gl_pathc {
                            let mut newnode: *mut wordsplit_node = 0
                                as *mut wordsplit_node;
                            let mut newstr: *mut libc::c_char = 0 as *mut libc::c_char;
                            if wsnode_new(wsp, &mut newnode) != 0 {
                                return 1 as libc::c_int;
                            }
                            newstr = strdup(*(g.gl_pathv).offset(i as isize));
                            if newstr.is_null() {
                                return _wsplt_nomem(wsp);
                            }
                            (*newnode).v.word = newstr;
                            (*newnode).flags |= 0x2 as libc::c_int | 0x4 as libc::c_int;
                            wsnode_insert(wsp, newnode, prev, 0 as libc::c_int);
                            prev = newnode;
                            i += 1;
                            i;
                        }
                        globfree(&mut g);
                        wsnode_remove(wsp, p);
                    }
                    1 => {
                        free(pattern as *mut libc::c_void);
                        return _wsplt_nomem(wsp);
                    }
                    3 => {
                        if (*wsp).ws_options & 0x1 as libc::c_int != 0 {
                            wsnode_remove(wsp, p);
                        } else if (*wsp).ws_options & 0x2 as libc::c_int != 0 {
                            let mut buf: [libc::c_char; 128] = [0; 128];
                            if (*wsp).ws_errno == 9 as libc::c_int {
                                free((*wsp).ws_usererr as *mut libc::c_void);
                            }
                            snprintf(
                                buf.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 128]>()
                                    as libc::c_ulong,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"no files match pattern %s\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                pattern,
                            );
                            free(pattern as *mut libc::c_void);
                            (*wsp).ws_usererr = strdup(buf.as_mut_ptr());
                            if ((*wsp).ws_usererr).is_null() {
                                return _wsplt_nomem(wsp)
                            } else {
                                return _wsplt_seterr(wsp, 9 as libc::c_int)
                            }
                        }
                    }
                    _ => {
                        free(pattern as *mut libc::c_void);
                        return _wsplt_setctxerr(wsp, 8 as libc::c_int, pattern, slen);
                    }
                }
            }
        }
        p = next;
    }
    free(pattern as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn skip_sed_expr(
    mut command: *const libc::c_char,
    mut i: size_t,
    mut len: size_t,
) -> libc::c_int {
    let mut state: libc::c_int = 0;
    loop {
        let mut delim: libc::c_int = 0;
        if *command.offset(i as isize) as libc::c_int == ';' as i32 {
            i = i.wrapping_add(1);
            i;
        }
        if !(*command.offset(i as isize) as libc::c_int == 's' as i32
            && i.wrapping_add(3 as libc::c_int as libc::c_ulong) < len
            && !(strchr(
                b"!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~\0" as *const u8
                    as *const libc::c_char,
                *command
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int,
            ))
                .is_null())
        {
            break;
        }
        i = i.wrapping_add(1);
        delim = *command.offset(i as isize) as libc::c_int;
        state = 1 as libc::c_int;
        i = i.wrapping_add(1);
        i;
        while i < len {
            if state == 3 as libc::c_int {
                if *command.offset(i as isize) as libc::c_int == delim
                    || !('A' as i32 as libc::c_uint
                        <= *command.offset(i as isize) as libc::c_uint
                        && *command.offset(i as isize) as libc::c_uint
                            <= 'Z' as i32 as libc::c_uint
                        || 'a' as i32 as libc::c_uint
                            <= *command.offset(i as isize) as libc::c_uint
                            && *command.offset(i as isize) as libc::c_uint
                                <= 'z' as i32 as libc::c_uint
                        || '0' as i32 as libc::c_uint
                            <= *command.offset(i as isize) as libc::c_uint
                            && *command.offset(i as isize) as libc::c_uint
                                <= '9' as i32 as libc::c_uint)
                {
                    break;
                }
            } else if *command.offset(i as isize) as libc::c_int == '\\' as i32 {
                i = i.wrapping_add(1);
                i;
            } else if *command.offset(i as isize) as libc::c_int == delim {
                state += 1;
                state;
            }
            i = i.wrapping_add(1);
            i;
        }
        if !(state == 3 as libc::c_int && i < len
            && *command.offset(i as isize) as libc::c_int == ';' as i32)
        {
            break;
        }
    }
    return i as libc::c_int;
}
#[inline]
unsafe extern "C" fn skip_delim_internal(
    mut wsp: *mut wordsplit,
    mut return_delims: libc::c_int,
) -> size_t {
    return if return_delims != 0 {
        (*wsp).ws_endp
    } else {
        ((*wsp).ws_endp).wrapping_add(1 as libc::c_int as libc::c_ulong)
    };
}
#[inline]
unsafe extern "C" fn skip_delim(mut wsp: *mut wordsplit) -> size_t {
    return skip_delim_internal(
        wsp,
        ((*wsp).ws_flags & 0x1000 as libc::c_int != 0
            || (*wsp).ws_options & 0x80 as libc::c_int != 0) as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn skip_delim_real(mut wsp: *mut wordsplit) -> size_t {
    return skip_delim_internal(wsp, (*wsp).ws_flags & 0x1000 as libc::c_int);
}
unsafe extern "C" fn scan_qstring(
    mut wsp: *mut wordsplit,
    mut start: size_t,
    mut end: *mut size_t,
) -> libc::c_int {
    let mut j: size_t = 0;
    let mut command: *const libc::c_char = (*wsp).ws_input;
    let mut len: size_t = (*wsp).ws_len;
    let mut q: libc::c_char = *command.offset(start as isize);
    j = start.wrapping_add(1 as libc::c_int as libc::c_ulong);
    while j < len && *command.offset(j as isize) as libc::c_int != q as libc::c_int {
        if q as libc::c_int == '"' as i32
            && *command.offset(j as isize) as libc::c_int == '\\' as i32
        {
            j = j.wrapping_add(1);
            j;
        }
        j = j.wrapping_add(1);
        j;
    }
    if j < len && *command.offset(j as isize) as libc::c_int == q as libc::c_int {
        let mut flags: libc::c_int = 0x4 as libc::c_int | 0x100 as libc::c_int;
        if q as libc::c_int == '\'' as i32 {
            flags |= 0x8 as libc::c_int;
        }
        if wordsplit_add_segm(
            wsp,
            start.wrapping_add(1 as libc::c_int as libc::c_ulong),
            j,
            flags,
        ) != 0
        {
            return 2 as libc::c_int;
        }
        *end = j;
    } else {
        (*wsp).ws_endp = start;
        _wsplt_seterr(wsp, 1 as libc::c_int);
        return 2 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn scan_word(
    mut wsp: *mut wordsplit,
    mut start: size_t,
    mut consume_all: libc::c_int,
) -> libc::c_int {
    let mut len: size_t = (*wsp).ws_len;
    let mut command: *const libc::c_char = (*wsp).ws_input;
    let mut comment: *const libc::c_char = (*wsp).ws_comment;
    let mut join: libc::c_int = 0 as libc::c_int;
    let mut flags: libc::c_int = 0 as libc::c_int;
    let mut np: *mut wordsplit_node = (*wsp).ws_tail;
    let mut i: size_t = start;
    if i >= len {
        (*wsp).ws_errno = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    start = i;
    if (*wsp).ws_flags & 0x2000 as libc::c_int != 0
        && *command.offset(i as isize) as libc::c_int == 's' as i32
        && i.wrapping_add(3 as libc::c_int as libc::c_ulong) < len
        && !(strchr(
            b"!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~\0" as *const u8 as *const libc::c_char,
            *command.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int,
        ))
            .is_null()
    {
        flags = 0x20 as libc::c_int;
        i = skip_sed_expr(command, i, len) as size_t;
    } else if consume_all != 0
        || (strchr((*wsp).ws_delim, *command.offset(i as isize) as libc::c_int))
            .is_null()
    {
        while i < len {
            if !comment.is_null()
                && !(strchr(comment, *command.offset(i as isize) as libc::c_int))
                    .is_null()
            {
                let mut j: size_t = 0;
                j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
                while j < len
                    && *command.offset(j as isize) as libc::c_int != '\n' as i32
                {
                    j = j.wrapping_add(1);
                    j;
                }
                if wordsplit_add_segm(wsp, start, i, 0 as libc::c_int) != 0 {
                    return 2 as libc::c_int;
                }
                (*wsp).ws_endp = j;
                return 1 as libc::c_int;
            }
            if (*wsp).ws_flags & (0x200 as libc::c_int | 0x400 as libc::c_int) != 0 {
                if *command.offset(i as isize) as libc::c_int == '\\' as i32 {
                    i = i.wrapping_add(1);
                    if i == len {
                        break;
                    }
                    i = i.wrapping_add(1);
                    i;
                    continue;
                } else if (*wsp).ws_flags & 0x200 as libc::c_int != 0
                    && *command.offset(i as isize) as libc::c_int == '\'' as i32
                    || (*wsp).ws_flags & 0x400 as libc::c_int != 0
                        && *command.offset(i as isize) as libc::c_int == '"' as i32
                {
                    if join != 0 && !((*wsp).ws_tail).is_null() {
                        (*(*wsp).ws_tail).flags |= 0x10 as libc::c_int;
                    }
                    if wordsplit_add_segm(wsp, start, i, 0x10 as libc::c_int) != 0 {
                        return 2 as libc::c_int;
                    }
                    if scan_qstring(wsp, i, &mut i) != 0 {
                        return 2 as libc::c_int;
                    }
                    start = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
                    join = 1 as libc::c_int;
                }
            }
            if *command.offset(i as isize) as libc::c_int == '$' as i32 {
                if ((*wsp).ws_flags & 0x40 as libc::c_int == 0
                    || (*wsp).ws_options & 0x1000 as libc::c_int != 0)
                    && *command
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int == '{' as i32
                    && find_closing_paren(
                        command,
                        i.wrapping_add(2 as libc::c_int as libc::c_ulong),
                        len,
                        &mut i,
                        b"{}\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                {
                    continue;
                }
                if ((*wsp).ws_flags & 0x4 as libc::c_int == 0
                    || (*wsp).ws_options & 0x2000 as libc::c_int != 0)
                    && *command
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int == '(' as i32
                    && find_closing_paren(
                        command,
                        i.wrapping_add(2 as libc::c_int as libc::c_ulong),
                        len,
                        &mut i,
                        b"()\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                {
                    continue;
                }
            }
            if consume_all == 0
                && !(strchr((*wsp).ws_delim, *command.offset(i as isize) as libc::c_int))
                    .is_null()
            {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else if (*wsp).ws_flags & 0x1000 as libc::c_int != 0
        || (*wsp).ws_options & 0x80 as libc::c_int != 0
    {
        i = i.wrapping_add(1);
        i;
        flags |= 0x40 as libc::c_int;
    } else if (*wsp).ws_flags & 0x800 as libc::c_int == 0 {
        flags |= 0x100 as libc::c_int;
    }
    if join != 0 && i > start && !((*wsp).ws_tail).is_null() {
        (*(*wsp).ws_tail).flags |= 0x10 as libc::c_int;
    }
    if wordsplit_add_segm(wsp, start, i, flags) != 0 {
        return 2 as libc::c_int;
    }
    (*wsp).ws_endp = i;
    if (*wsp).ws_flags & 0x20000000 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    if consume_all != 0 {
        if np.is_null() {
            np = (*wsp).ws_head;
        }
        while !np.is_null() {
            (*np).flags |= 0x4 as libc::c_int;
            np = (*np).next;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn xtonum(
    mut pval: *mut libc::c_int,
    mut src: *const libc::c_char,
    mut base: libc::c_int,
    mut cnt: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    i = 0 as libc::c_int;
    val = 0 as libc::c_int;
    while i < cnt {
        let mut n: libc::c_int = *(src as *mut libc::c_uchar) as libc::c_int;
        if n > 127 as libc::c_int
            || {
                n = (if '0' as i32 as libc::c_uint <= n as libc::c_uint
                    && n as libc::c_uint <= '9' as i32 as libc::c_uint
                {
                    n - '0' as i32
                } else {
                    (if !(strchr(
                        b"abcdefABCDEF\0" as *const u8 as *const libc::c_char,
                        n,
                    ))
                        .is_null()
                    {
                        ({
                            let mut __res: libc::c_int = 0;
                            if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                > 1 as libc::c_int as libc::c_ulong
                            {
                                if 0 != 0 {
                                    let mut __c: libc::c_int = n;
                                    __res = (if __c < -(128 as libc::c_int)
                                        || __c > 255 as libc::c_int
                                    {
                                        __c
                                    } else {
                                        *(*__ctype_toupper_loc()).offset(__c as isize)
                                    });
                                } else {
                                    __res = toupper(n);
                                }
                            } else {
                                __res = *(*__ctype_toupper_loc()).offset(n as isize);
                            }
                            __res
                        }) - 'A' as i32 + 10 as libc::c_int
                    } else {
                        255 as libc::c_int
                    })
                });
                n >= base
            }
        {
            break;
        }
        val = val * base + n;
        i += 1;
        i;
        src = src.offset(1);
        src;
    }
    *pval = val;
    return i;
}
pub unsafe extern "C" fn wordsplit_c_quoted_length(
    mut str: *const libc::c_char,
    mut quote_hex: libc::c_int,
    mut quote: *mut libc::c_int,
) -> size_t {
    let mut len: size_t = 0 as libc::c_int as size_t;
    *quote = 0 as libc::c_int;
    while *str != 0 {
        if !(strchr(b" \"\0" as *const u8 as *const libc::c_char, *str as libc::c_int))
            .is_null()
        {
            *quote = 1 as libc::c_int;
        }
        if *str as libc::c_int == ' ' as i32 {
            len = len.wrapping_add(1);
            len;
        } else if *str as libc::c_int == '"' as i32 {
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        } else if *str as libc::c_int != '\t' as i32
            && *str as libc::c_int != '\\' as i32
            && (' ' as i32 as libc::c_uint <= *str as libc::c_uint
                && *str as libc::c_uint <= 127 as libc::c_int as libc::c_uint)
        {
            len = len.wrapping_add(1);
            len;
        } else if quote_hex != 0 {
            len = (len as libc::c_ulong).wrapping_add(3 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        } else if wordsplit_c_quote_char(*str as libc::c_int) != 0 {
            len = (len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        } else {
            len = (len as libc::c_ulong).wrapping_add(4 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        }
        str = str.offset(1);
        str;
    }
    return len;
}
unsafe extern "C" fn wsplt_unquote_char(
    mut transtab: *const libc::c_char,
    mut c: libc::c_int,
) -> libc::c_int {
    while *transtab as libc::c_int != 0
        && *transtab.offset(1 as libc::c_int as isize) as libc::c_int != 0
    {
        let fresh36 = transtab;
        transtab = transtab.offset(1);
        if *fresh36 as libc::c_int == c {
            return *transtab as libc::c_int;
        }
        transtab = transtab.offset(1);
        transtab;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn wsplt_quote_char(
    mut transtab: *const libc::c_char,
    mut c: libc::c_int,
) -> libc::c_int {
    while *transtab as libc::c_int != 0
        && *transtab.offset(1 as libc::c_int as isize) as libc::c_int != 0
    {
        if *transtab.offset(1 as libc::c_int as isize) as libc::c_int == c {
            return *transtab as libc::c_int;
        }
        transtab = transtab.offset(2 as libc::c_int as isize);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn wordsplit_c_unquote_char(mut c: libc::c_int) -> libc::c_int {
    return wsplt_unquote_char(wordsplit_c_escape_tab.as_mut_ptr(), c);
}
pub unsafe extern "C" fn wordsplit_c_quote_char(mut c: libc::c_int) -> libc::c_int {
    return wsplt_quote_char(wordsplit_c_escape_tab.as_mut_ptr(), c);
}
unsafe extern "C" fn wordsplit_string_unquote_copy(
    mut ws: *mut wordsplit,
    mut inquote: libc::c_int,
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut n: size_t,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    inquote = (inquote != 0) as libc::c_int;
    while (i as libc::c_ulong) < n {
        if *src.offset(i as isize) as libc::c_int == '\\' as i32 {
            i += 1;
            i;
            if (*ws).ws_options & (0x40 as libc::c_int) << 4 as libc::c_int * inquote
                != 0
                && (*src.offset(i as isize) as libc::c_int == 'x' as i32
                    || *src.offset(i as isize) as libc::c_int == 'X' as i32)
            {
                if n.wrapping_sub(i as libc::c_ulong) < 2 as libc::c_int as libc::c_ulong
                {
                    let fresh37 = dst;
                    dst = dst.offset(1);
                    *fresh37 = '\\' as i32 as libc::c_char;
                    let fresh38 = i;
                    i = i + 1;
                    let fresh39 = dst;
                    dst = dst.offset(1);
                    *fresh39 = *src.offset(fresh38 as isize);
                } else {
                    let mut off: libc::c_int = xtonum(
                        &mut c,
                        src.offset(i as isize).offset(1 as libc::c_int as isize),
                        16 as libc::c_int,
                        2 as libc::c_int,
                    );
                    if off == 0 as libc::c_int {
                        let fresh40 = dst;
                        dst = dst.offset(1);
                        *fresh40 = '\\' as i32 as libc::c_char;
                        let fresh41 = i;
                        i = i + 1;
                        let fresh42 = dst;
                        dst = dst.offset(1);
                        *fresh42 = *src.offset(fresh41 as isize);
                    } else {
                        let fresh43 = dst;
                        dst = dst.offset(1);
                        *fresh43 = c as libc::c_char;
                        i += off + 1 as libc::c_int;
                    }
                }
            } else if (*ws).ws_options
                & (0x20 as libc::c_int) << 4 as libc::c_int * inquote != 0
                && (*src.offset(i as isize) as libc::c_uchar as libc::c_int)
                    < 128 as libc::c_int
                && ('0' as i32 as libc::c_uint <= *src.offset(i as isize) as libc::c_uint
                    && *src.offset(i as isize) as libc::c_uint
                        <= '9' as i32 as libc::c_uint)
            {
                if n.wrapping_sub(i as libc::c_ulong) < 1 as libc::c_int as libc::c_ulong
                {
                    let fresh44 = dst;
                    dst = dst.offset(1);
                    *fresh44 = '\\' as i32 as libc::c_char;
                    let fresh45 = i;
                    i = i + 1;
                    let fresh46 = dst;
                    dst = dst.offset(1);
                    *fresh46 = *src.offset(fresh45 as isize);
                } else {
                    let mut off_0: libc::c_int = xtonum(
                        &mut c,
                        src.offset(i as isize),
                        8 as libc::c_int,
                        3 as libc::c_int,
                    );
                    if off_0 == 0 as libc::c_int {
                        let fresh47 = dst;
                        dst = dst.offset(1);
                        *fresh47 = '\\' as i32 as libc::c_char;
                        let fresh48 = i;
                        i = i + 1;
                        let fresh49 = dst;
                        dst = dst.offset(1);
                        *fresh49 = *src.offset(fresh48 as isize);
                    } else {
                        let fresh50 = dst;
                        dst = dst.offset(1);
                        *fresh50 = c as libc::c_char;
                        i += off_0;
                    }
                }
            } else {
                c = wsplt_unquote_char(
                    (*ws).ws_escape[inquote as usize],
                    *src.offset(i as isize) as libc::c_int,
                );
                if c != 0 {
                    let fresh51 = dst;
                    dst = dst.offset(1);
                    *fresh51 = c as libc::c_char;
                    i += 1;
                    i;
                } else {
                    if (*ws).ws_options
                        & (0x10 as libc::c_int) << 4 as libc::c_int * inquote != 0
                    {
                        let fresh52 = dst;
                        dst = dst.offset(1);
                        *fresh52 = '\\' as i32 as libc::c_char;
                    }
                    let fresh53 = i;
                    i = i + 1;
                    let fresh54 = dst;
                    dst = dst.offset(1);
                    *fresh54 = *src.offset(fresh53 as isize);
                }
            }
        } else {
            let fresh55 = i;
            i = i + 1;
            let fresh56 = dst;
            dst = dst.offset(1);
            *fresh56 = *src.offset(fresh55 as isize);
        }
    }
    *dst = 0 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn wordsplit_c_quote_copy(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut quote_hex: libc::c_int,
) {
    while *src != 0 {
        if *src as libc::c_int == '"' as i32 {
            let fresh57 = dst;
            dst = dst.offset(1);
            *fresh57 = '\\' as i32 as libc::c_char;
            let fresh58 = dst;
            dst = dst.offset(1);
            *fresh58 = *src;
        } else if *src as libc::c_int != '\t' as i32
            && *src as libc::c_int != '\\' as i32
            && (' ' as i32 as libc::c_uint <= *src as libc::c_uint
                && *src as libc::c_uint <= 127 as libc::c_int as libc::c_uint)
        {
            let fresh59 = dst;
            dst = dst.offset(1);
            *fresh59 = *src;
        } else {
            let mut tmp: [libc::c_char; 4] = [0; 4];
            if quote_hex != 0 {
                snprintf(
                    tmp.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong,
                    b"%%%02X\0" as *const u8 as *const libc::c_char,
                    *(src as *mut libc::c_uchar) as libc::c_int,
                );
                memcpy(
                    dst as *mut libc::c_void,
                    tmp.as_mut_ptr() as *const libc::c_void,
                    3 as libc::c_int as libc::c_ulong,
                );
                dst = dst.offset(3 as libc::c_int as isize);
            } else {
                let mut c: libc::c_int = wordsplit_c_quote_char(*src as libc::c_int);
                let fresh60 = dst;
                dst = dst.offset(1);
                *fresh60 = '\\' as i32 as libc::c_char;
                if c != 0 {
                    let fresh61 = dst;
                    dst = dst.offset(1);
                    *fresh61 = c as libc::c_char;
                } else {
                    snprintf(
                        tmp.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong,
                        b"%03o\0" as *const u8 as *const libc::c_char,
                        *(src as *mut libc::c_uchar) as libc::c_int,
                    );
                    memcpy(
                        dst as *mut libc::c_void,
                        tmp.as_mut_ptr() as *const libc::c_void,
                        3 as libc::c_int as libc::c_ulong,
                    );
                    dst = dst.offset(3 as libc::c_int as isize);
                }
            }
        }
        src = src.offset(1);
        src;
    }
}
static mut exptab: [exptab; 9] = unsafe {
    [
        {
            let mut init = exptab {
                descr: b"WS trimming\0" as *const u8 as *const libc::c_char,
                flag: 0x100 as libc::c_int,
                opt: 0 as libc::c_int,
                expansion: Some(
                    wordsplit_trimws
                        as unsafe extern "C" fn(*mut wordsplit) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = exptab {
                descr: b"command substitution\0" as *const u8 as *const libc::c_char,
                flag: 0x4 as libc::c_int,
                opt: 0x1 as libc::c_int | 0x4 as libc::c_int,
                expansion: Some(
                    wordsplit_cmdexp
                        as unsafe extern "C" fn(*mut wordsplit) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = exptab {
                descr: b"coalesce list\0" as *const u8 as *const libc::c_char,
                flag: 0 as libc::c_int,
                opt: 0x1 as libc::c_int | 0x4 as libc::c_int,
                expansion: None,
            };
            init
        },
        {
            let mut init = exptab {
                descr: b"tilde expansion\0" as *const u8 as *const libc::c_char,
                flag: 0x40000000 as libc::c_int,
                opt: 0 as libc::c_int,
                expansion: Some(
                    wordsplit_tildexpand
                        as unsafe extern "C" fn(*mut wordsplit) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = exptab {
                descr: b"variable expansion\0" as *const u8 as *const libc::c_char,
                flag: 0x40 as libc::c_int,
                opt: 0x1 as libc::c_int,
                expansion: Some(
                    wordsplit_varexp
                        as unsafe extern "C" fn(*mut wordsplit) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = exptab {
                descr: b"quote removal\0" as *const u8 as *const libc::c_char,
                flag: 0 as libc::c_int,
                opt: 0x1 as libc::c_int,
                expansion: Some(
                    wsnode_quoteremoval
                        as unsafe extern "C" fn(*mut wordsplit) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = exptab {
                descr: b"coalesce list\0" as *const u8 as *const libc::c_char,
                flag: 0 as libc::c_int,
                opt: 0x1 as libc::c_int | 0x4 as libc::c_int,
                expansion: None,
            };
            init
        },
        {
            let mut init = exptab {
                descr: b"path expansion\0" as *const u8 as *const libc::c_char,
                flag: 0x40000000 as libc::c_int,
                opt: 0 as libc::c_int,
                expansion: Some(
                    wordsplit_pathexpand
                        as unsafe extern "C" fn(*mut wordsplit) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = exptab {
                descr: 0 as *const libc::c_char,
                flag: 0,
                opt: 0,
                expansion: None,
            };
            init
        },
    ]
};
#[inline]
unsafe extern "C" fn exptab_matches(
    mut p: *mut exptab,
    mut wsp: *mut wordsplit,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    result = (*wsp).ws_flags & (*p).flag;
    if (*p).opt & 0x2 as libc::c_int != 0 {
        result = (result == (*p).flag) as libc::c_int;
    }
    if (*p).opt & 0x1 as libc::c_int != 0 {
        result = (result == 0) as libc::c_int;
    }
    return result;
}
unsafe extern "C" fn wordsplit_process_list(
    mut wsp: *mut wordsplit,
    mut start: size_t,
) -> libc::c_int {
    let mut p: *mut exptab = 0 as *mut exptab;
    if (*wsp).ws_flags & 0x200000 as libc::c_int != 0 {
        ((*wsp).ws_debug)
            .unwrap()(
            dcgettext(
                0 as *const libc::c_char,
                b"(%02d) Input:%.*s;\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*wsp).ws_lvl,
            (*wsp).ws_len as libc::c_int,
            (*wsp).ws_input,
        );
    }
    if (*wsp).ws_flags & 0x400000 as libc::c_int != 0
        || (*wsp).ws_options & 0x80 as libc::c_int != 0
            && ((*wsp).ws_wordi).wrapping_add(1 as libc::c_int as libc::c_ulong)
                == (*wsp).ws_maxwords
    {
        if scan_word(wsp, start, 1 as libc::c_int) == 2 as libc::c_int {
            return (*wsp).ws_errno;
        }
    } else {
        let mut rc: libc::c_int = 0;
        loop {
            rc = scan_word(wsp, start, 0 as libc::c_int);
            if !(rc == 1 as libc::c_int) {
                break;
            }
            start = skip_delim(wsp);
        }
        if !((*wsp).ws_tail).is_null() {
            (*(*wsp).ws_tail).flags &= !(0x10 as libc::c_int);
        }
        if rc == 2 as libc::c_int {
            return (*wsp).ws_errno;
        }
    }
    if (*wsp).ws_flags & 0x200000 as libc::c_int != 0 {
        ((*wsp).ws_debug)
            .unwrap()(
            b"(%02d) %s\0" as *const u8 as *const libc::c_char,
            (*wsp).ws_lvl,
            dcgettext(
                0 as *const libc::c_char,
                b"Initial list:\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        wordsplit_dump_nodes(wsp);
    }
    p = exptab.as_mut_ptr();
    while !((*p).descr).is_null() {
        if exptab_matches(p, wsp) != 0 {
            if (*p).opt & 0x4 as libc::c_int != 0 {
                if wsnode_coalesce(wsp) != 0 {
                    break;
                }
                if (*wsp).ws_flags & 0x200000 as libc::c_int != 0 {
                    ((*wsp).ws_debug)
                        .unwrap()(
                        b"(%02d) %s\0" as *const u8 as *const libc::c_char,
                        (*wsp).ws_lvl,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Coalesced list:\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    wordsplit_dump_nodes(wsp);
                }
            }
            if ((*p).expansion).is_some() {
                if ((*p).expansion).unwrap()(wsp) != 0 {
                    break;
                }
                if (*wsp).ws_flags & 0x200000 as libc::c_int != 0 {
                    ((*wsp).ws_debug)
                        .unwrap()(
                        b"(%02d) %s\0" as *const u8 as *const libc::c_char,
                        (*wsp).ws_lvl,
                        dcgettext(0 as *const libc::c_char, (*p).descr, 5 as libc::c_int),
                    );
                    wordsplit_dump_nodes(wsp);
                }
            }
        }
        p = p.offset(1);
        p;
    }
    return (*wsp).ws_errno;
}
unsafe extern "C" fn wordsplit_run(
    mut command: *const libc::c_char,
    mut length: size_t,
    mut wsp: *mut wordsplit,
    mut flags: libc::c_int,
    mut lvl: libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut start: size_t = 0;
    (*wsp).ws_errctx = 0 as *mut libc::c_char;
    if command.is_null() {
        if flags & 0x20000000 as libc::c_int == 0 {
            return _wsplt_seterr(wsp, 3 as libc::c_int);
        }
        if !((*wsp).ws_head).is_null() {
            return wordsplit_finish(wsp);
        }
        start = skip_delim_real(wsp);
        if (*wsp).ws_endp == (*wsp).ws_len {
            return _wsplt_seterr(wsp, 6 as libc::c_int);
        }
        (*wsp).ws_flags |= 0x8 as libc::c_int;
        wordsplit_init0(wsp);
    } else {
        start = 0 as libc::c_int as size_t;
        rc = wordsplit_init(wsp, command, length, flags);
        if rc != 0 {
            return rc;
        }
        (*wsp).ws_lvl = lvl;
    }
    rc = wordsplit_process_list(wsp, start);
    if rc != 0 {
        return rc;
    }
    return wordsplit_finish(wsp);
}
pub unsafe extern "C" fn wordsplit_len(
    mut command: *const libc::c_char,
    mut length: size_t,
    mut wsp: *mut wordsplit,
    mut flags: libc::c_int,
) -> libc::c_int {
    return wordsplit_run(command, length, wsp, flags, 0 as libc::c_int);
}
pub unsafe extern "C" fn wordsplit(
    mut command: *const libc::c_char,
    mut ws: *mut wordsplit,
    mut flags: libc::c_int,
) -> libc::c_int {
    return wordsplit_len(
        command,
        if !command.is_null() {
            strlen(command)
        } else {
            0 as libc::c_int as libc::c_ulong
        },
        ws,
        flags,
    );
}
pub unsafe extern "C" fn wordsplit_free_words(mut ws: *mut wordsplit) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*ws).ws_wordc {
        let mut p: *mut libc::c_char = *((*ws).ws_wordv)
            .offset(((*ws).ws_offs).wrapping_add(i) as isize);
        if !p.is_null() {
            free(p as *mut libc::c_void);
            let ref mut fresh62 = *((*ws).ws_wordv)
                .offset(((*ws).ws_offs).wrapping_add(i) as isize);
            *fresh62 = 0 as *mut libc::c_char;
        }
        i = i.wrapping_add(1);
        i;
    }
    (*ws).ws_wordc = 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn wordsplit_free_envbuf(mut ws: *mut wordsplit) {
    if (*ws).ws_flags & 0x80000 as libc::c_int == 0 {
        return;
    }
    if !((*ws).ws_envbuf).is_null() {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while !(*((*ws).ws_envbuf).offset(i as isize)).is_null() {
            free(*((*ws).ws_envbuf).offset(i as isize) as *mut libc::c_void);
            i = i.wrapping_add(1);
            i;
        }
        free((*ws).ws_envbuf as *mut libc::c_void);
        (*ws).ws_envsiz = 0 as libc::c_int as size_t;
        (*ws).ws_envidx = (*ws).ws_envsiz;
        (*ws).ws_envbuf = 0 as *mut *mut libc::c_char;
    }
}
pub unsafe extern "C" fn wordsplit_free_parambuf(mut ws: *mut wordsplit) {
    if (*ws).ws_options & 0x4000 as libc::c_int == 0 {
        return;
    }
    if !((*ws).ws_parambuf).is_null() {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while !(*((*ws).ws_parambuf).offset(i as isize)).is_null() {
            free(*((*ws).ws_parambuf).offset(i as isize) as *mut libc::c_void);
            i = i.wrapping_add(1);
            i;
        }
        free((*ws).ws_parambuf as *mut libc::c_void);
        (*ws).ws_paramsiz = 0 as libc::c_int as size_t;
        (*ws).ws_paramidx = (*ws).ws_paramsiz;
        (*ws).ws_parambuf = 0 as *mut *mut libc::c_char;
    }
}
pub unsafe extern "C" fn wordsplit_clearerr(mut ws: *mut wordsplit) {
    if (*ws).ws_errno == 9 as libc::c_int {
        free((*ws).ws_usererr as *mut libc::c_void);
    }
    (*ws).ws_usererr = 0 as *mut libc::c_char;
    free((*ws).ws_errctx as *mut libc::c_void);
    (*ws).ws_errctx = 0 as *mut libc::c_char;
    (*ws).ws_errno = 0 as libc::c_int;
}
pub unsafe extern "C" fn wordsplit_free(mut ws: *mut wordsplit) {
    if (*ws).ws_errno == 3 as libc::c_int {
        return;
    }
    wordsplit_clearerr(ws);
    wordsplit_free_nodes(ws);
    wordsplit_free_words(ws);
    free((*ws).ws_wordv as *mut libc::c_void);
    (*ws).ws_wordv = 0 as *mut *mut libc::c_char;
    wordsplit_free_envbuf(ws);
    wordsplit_free_parambuf(ws);
}
pub unsafe extern "C" fn wordsplit_get_words(
    mut ws: *mut wordsplit,
    mut wordc: *mut size_t,
    mut wordv: *mut *mut *mut libc::c_char,
) -> libc::c_int {
    let mut p: *mut *mut libc::c_char = realloc(
        (*ws).ws_wordv as *mut libc::c_void,
        ((*ws).ws_wordc)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    *wordv = p;
    *wordc = (*ws).ws_wordc;
    (*ws).ws_wordv = 0 as *mut *mut libc::c_char;
    (*ws).ws_wordc = 0 as libc::c_int as size_t;
    (*ws).ws_wordn = 0 as libc::c_int as size_t;
    return 0 as libc::c_int;
}
pub static mut _wordsplit_errstr: [*const libc::c_char; 11] = [
    b"no error\0" as *const u8 as *const libc::c_char,
    b"missing closing quote\0" as *const u8 as *const libc::c_char,
    b"memory exhausted\0" as *const u8 as *const libc::c_char,
    b"invalid wordsplit usage\0" as *const u8 as *const libc::c_char,
    b"unbalanced curly brace\0" as *const u8 as *const libc::c_char,
    b"undefined variable\0" as *const u8 as *const libc::c_char,
    b"input exhausted\0" as *const u8 as *const libc::c_char,
    b"unbalanced parenthesis\0" as *const u8 as *const libc::c_char,
    b"globbing error\0" as *const u8 as *const libc::c_char,
    b"user-defined error\0" as *const u8 as *const libc::c_char,
    b"invalid parameter number in assignment\0" as *const u8 as *const libc::c_char,
];
pub static mut _wordsplit_nerrs: libc::c_int = 0;
pub unsafe extern "C" fn wordsplit_strerror(
    mut ws: *mut wordsplit,
) -> *const libc::c_char {
    if (*ws).ws_errno == 9 as libc::c_int {
        return (*ws).ws_usererr;
    }
    if (*ws).ws_errno < _wordsplit_nerrs {
        return _wordsplit_errstr[(*ws).ws_errno as usize];
    }
    return b"unknown error\0" as *const u8 as *const libc::c_char;
}
pub unsafe extern "C" fn wordsplit_perror(mut wsp: *mut wordsplit) {
    match (*wsp).ws_errno {
        1 => {
            ((*wsp).ws_error)
                .unwrap()(
                dcgettext(
                    0 as *const libc::c_char,
                    b"missing closing %c (start near #%lu)\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                *((*wsp).ws_input).offset((*wsp).ws_endp as isize) as libc::c_int,
                (*wsp).ws_endp,
            );
        }
        _ => {
            if !((*wsp).ws_errctx).is_null() {
                ((*wsp).ws_error)
                    .unwrap()(
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    wordsplit_strerror(wsp),
                    (*wsp).ws_errctx,
                );
            } else {
                ((*wsp).ws_error)
                    .unwrap()(
                    b"%s\0" as *const u8 as *const libc::c_char,
                    wordsplit_strerror(wsp),
                );
            }
        }
    };
}
unsafe extern "C" fn run_static_initializers() {
    _wordsplit_nerrs = (::std::mem::size_of::<[*const libc::c_char; 11]>()
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
        as libc::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
