use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut buf: *mut libc::c_char;
    static mut bufsize: size_t;
    static mut using_plan_a: bool;
    static mut inname: *mut libc::c_char;
    static mut outfile: *mut libc::c_char;
    static mut inerrno: libc::c_int;
    static mut invc: libc::c_int;
    static mut instat: stat;
    static mut dry_run: bool;
    static mut posixly_correct: bool;
    static mut TMPPATNAME: *const libc::c_char;
    static mut TMPPATNAME_needs_removal: bool;
    static mut debug: libc::c_int;
    static mut force: bool;
    static mut batch: bool;
    static mut reverse: bool;
    static mut verbosity: C2RustUnnamed_0;
    static mut skip_rest_of_patch: bool;
    static mut strippath: libc::c_int;
    static mut canonicalize_ws: bool;
    static mut patch_get: libc::c_int;
    static mut diff_type: diff;
    static mut revision: *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut no_strip_trailing_cr: bool;
    fn base_len(file: *const libc::c_char) -> size_t;
    fn quotearg(arg: *const libc::c_char) -> *mut libc::c_char;
    fn ok_to_reverse(_: *const libc::c_char, _: ...) -> bool;
    fn ask(_: *const libc::c_char, _: ...);
    fn say(_: *const libc::c_char, _: ...);
    fn fatal(_: *const libc::c_char, _: ...) -> !;
    fn pfatal(_: *const libc::c_char, _: ...) -> !;
    fn fetchname(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        _: *mut *mut libc::c_char,
        _: *mut timespec,
    );
    fn parse_name(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut *const libc::c_char,
    ) -> *mut libc::c_char;
    fn savebuf(_: *const libc::c_char, _: size_t) -> *mut libc::c_char;
    fn savestr(_: *const libc::c_char) -> *mut libc::c_char;
    fn version_controller(
        _: *const libc::c_char,
        _: bool,
        _: *const stat,
        _: *mut *mut libc::c_char,
        _: *mut *mut libc::c_char,
    ) -> *const libc::c_char;
    fn version_get(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: bool,
        _: bool,
        _: *const libc::c_char,
        _: *mut stat,
    ) -> bool;
    fn format_linenum(_: *mut libc::c_char, _: lin) -> *mut libc::c_char;
    fn Fseek(_: *mut FILE, _: file_offset, _: libc::c_int);
    fn copy_file(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut stat,
        _: libc::c_int,
        _: mode_t,
        _: bool,
    );
    fn xalloc_die() -> !;
    fn read_fatal() -> !;
    fn remove_prefix(_: *mut libc::c_char, _: size_t);
    fn write_fatal() -> !;
    fn lookup_file_id(_: *const stat) -> file_id_type;
    fn make_tempfile(
        _: *mut *const libc::c_char,
        _: libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: mode_t,
    ) -> libc::c_int;
    fn stat_file(_: *const libc::c_char, _: *mut stat) -> libc::c_int;
    fn filename_is_safe(_: *const libc::c_char) -> bool;
    fn cwd_is_root(_: *const libc::c_char) -> bool;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xmemdup0(p: *const libc::c_void, s: size_t) -> *mut libc::c_char;
    fn safe_stat(pathname: *const libc::c_char, buf_0: *mut stat) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub type off_t = __off_t;
pub type mode_t = __mode_t;
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
pub type lin = off_t;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const VERBOSE: C2RustUnnamed_0 = 2;
pub const SILENT: C2RustUnnamed_0 = 1;
pub const DEFAULT_VERBOSITY: C2RustUnnamed_0 = 0;
pub type diff = libc::c_uint;
pub const GIT_BINARY_DIFF: diff = 6;
pub const UNI_DIFF: diff = 5;
pub const NEW_CONTEXT_DIFF: diff = 4;
pub const ED_DIFF: diff = 3;
pub const NORMAL_DIFF: diff = 2;
pub const CONTEXT_DIFF: diff = 1;
pub const NO_DIFF: diff = 0;
pub type file_offset = libc::c_long;
pub type file_id_type = libc::c_uint;
pub const OVERWRITTEN: file_id_type = 3;
pub const DELETE_LATER: file_id_type = 2;
pub const CREATED: file_id_type = 1;
pub const UNKNOWN: file_id_type = 0;
pub type nametype = libc::c_uint;
pub const NONE: nametype = 3;
pub const INDEX: nametype = 2;
pub const NEW: nametype = 1;
pub const OLD: nametype = 0;
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn skip_spaces(mut str: *const libc::c_char) -> *const libc::c_char {
    while 1 as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*str as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        str = str.offset(1);
        str;
    }
    return str;
}
pub static mut p_timestamp: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
static mut pfp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut p_says_nonexistent: [libc::c_int; 2] = [0; 2];
static mut p_rfc934_nesting: libc::c_int = 0;
static mut p_name: [*mut libc::c_char; 3] = [0 as *const libc::c_char
    as *mut libc::c_char; 3];
static mut invalid_names: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
pub static mut p_copy: [bool; 2] = [false; 2];
pub static mut p_rename: [bool; 2] = [false; 2];
static mut p_timestr: [*mut libc::c_char; 2] = [0 as *const libc::c_char
    as *mut libc::c_char; 2];
static mut p_sha1: [*mut libc::c_char; 2] = [0 as *const libc::c_char
    as *mut libc::c_char; 2];
static mut p_mode: [mode_t; 2] = [0; 2];
static mut p_filesize: off_t = 0;
static mut p_first: lin = 0;
static mut p_newfirst: lin = 0;
static mut p_ptrn_lines: lin = 0;
static mut p_repl_lines: lin = 0;
static mut p_end: lin = -(1 as libc::c_int) as lin;
static mut p_max: lin = 0;
static mut p_prefix_context: lin = 0;
static mut p_suffix_context: lin = 0;
static mut p_input_line: lin = 0;
static mut p_line: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut p_len: *mut size_t = 0 as *const size_t as *mut size_t;
static mut p_Char: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut hunkmax: lin = 125 as libc::c_int as lin;
static mut p_indent: size_t = 0;
static mut p_strip_trailing_cr: bool = false;
static mut p_pass_comments_through: bool = false;
static mut p_base: file_offset = 0;
static mut p_bline: lin = 0;
static mut p_start: file_offset = 0;
static mut p_sline: lin = 0;
static mut p_hunk_beg: lin = 0;
static mut p_efake: lin = -(1 as libc::c_int) as lin;
static mut p_bfake: lin = -(1 as libc::c_int) as lin;
static mut p_c_function: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut p_git_diff: bool = false;
pub unsafe extern "C" fn re_patch() {
    p_first = 0 as libc::c_int as lin;
    p_newfirst = 0 as libc::c_int as lin;
    p_ptrn_lines = 0 as libc::c_int as lin;
    p_repl_lines = 0 as libc::c_int as lin;
    p_end = -(1 as libc::c_int) as lin;
    p_max = 0 as libc::c_int as lin;
    p_indent = 0 as libc::c_int as size_t;
    p_strip_trailing_cr = 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn open_patch_file(mut filename: *const libc::c_char) {
    let mut file_pos: file_offset = 0 as libc::c_int as file_offset;
    let mut pos: file_offset = 0;
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
    if filename.is_null() || *filename == 0
        || strcmp(filename, b"-\0" as *const u8 as *const libc::c_char) == 0
    {
        pfp = stdin;
    } else {
        pfp = fopen(
            filename,
            if 0 as libc::c_int != 0 {
                b"rb\0" as *const u8 as *const libc::c_char
            } else {
                b"r\0" as *const u8 as *const libc::c_char
            },
        );
        if pfp.is_null() {
            pfatal(
                b"Can't open patch file %s\0" as *const u8 as *const libc::c_char,
                quotearg(filename),
            );
        }
    }
    if fstat(fileno(pfp), &mut st) != 0 as libc::c_int {
        pfatal(b"fstat\0" as *const u8 as *const libc::c_char);
    }
    if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
        && {
            pos = ftell(pfp);
            pos != -(1 as libc::c_int) as libc::c_long
        }
    {
        file_pos = pos;
    } else {
        let mut charsread: size_t = 0;
        let mut fd: libc::c_int = 0;
        let mut read_pfp: *mut FILE = pfp;
        fd = make_tempfile(
            &mut TMPPATNAME,
            'p' as i32 as libc::c_char,
            0 as *const libc::c_char,
            0o2 as libc::c_int | 0 as libc::c_int,
            0 as libc::c_int as mode_t,
        );
        if fd == -(1 as libc::c_int) {
            pfatal(
                b"Can't create temporary file %s\0" as *const u8 as *const libc::c_char,
                TMPPATNAME,
            );
        }
        TMPPATNAME_needs_removal = 1 as libc::c_int != 0;
        pfp = fdopen(fd, b"w+b\0" as *const u8 as *const libc::c_char);
        if pfp.is_null() {
            pfatal(
                b"Can't open stream for file %s\0" as *const u8 as *const libc::c_char,
                quotearg(TMPPATNAME),
            );
        }
        st.st_size = 0 as libc::c_int as __off_t;
        loop {
            charsread = fread(
                buf as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                bufsize,
                read_pfp,
            );
            if !(charsread != 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            if fwrite(
                buf as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                charsread,
                pfp,
            ) != charsread
            {
                write_fatal();
            }
            st
                .st_size = (st.st_size as libc::c_ulong).wrapping_add(charsread)
                as __off_t as __off_t;
        }
        if ferror(read_pfp) != 0 || fclose(read_pfp) != 0 as libc::c_int {
            read_fatal();
        }
        if fflush(pfp) != 0 as libc::c_int
            || fseek(pfp, 0 as libc::c_int as file_offset, 0 as libc::c_int)
                != 0 as libc::c_int
        {
            write_fatal();
        }
    }
    p_filesize = st.st_size;
    if p_filesize != p_filesize {
        fatal(b"patch file is too long\0" as *const u8 as *const libc::c_char);
    }
    next_intuit_at(file_pos, 1 as libc::c_int as lin);
}
unsafe extern "C" fn set_hunkmax() {
    if p_line.is_null() {
        p_line = xmalloc(
            (hunkmax as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
    }
    if p_len.is_null() {
        p_len = xmalloc(
            (hunkmax as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
        ) as *mut size_t;
    }
    if p_Char.is_null() {
        p_Char = xmalloc(
            (hunkmax as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
    }
}
unsafe extern "C" fn grow_hunkmax() -> bool {
    hunkmax *= 2 as libc::c_int as libc::c_long;
    if !p_line.is_null() && !p_len.is_null() && !p_Char.is_null() {} else {
        __assert_fail(
            b"p_line && p_len && p_Char\0" as *const u8 as *const libc::c_char,
            b"pch.c\0" as *const u8 as *const libc::c_char,
            186 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"_Bool grow_hunkmax(void)\0"))
                .as_ptr(),
        );
    }
    'c_10299: {
        if !p_line.is_null() && !p_len.is_null() && !p_Char.is_null() {} else {
            __assert_fail(
                b"p_line && p_len && p_Char\0" as *const u8 as *const libc::c_char,
                b"pch.c\0" as *const u8 as *const libc::c_char,
                186 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"_Bool grow_hunkmax(void)\0"))
                    .as_ptr(),
            );
        }
    };
    p_line = realloc(
        p_line as *mut libc::c_void,
        (hunkmax as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if !p_line.is_null()
        && {
            p_len = realloc(
                p_len as *mut libc::c_void,
                (hunkmax as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
            ) as *mut size_t;
            !p_len.is_null()
        }
        && {
            p_Char = realloc(
                p_Char as *mut libc::c_void,
                (hunkmax as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            ) as *mut libc::c_char;
            !p_Char.is_null()
        }
    {
        return 1 as libc::c_int != 0;
    }
    if !using_plan_a {
        xalloc_die();
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn maybe_reverse(
    mut name: *const libc::c_char,
    mut nonexistent: bool,
    mut is_empty: bool,
) -> bool {
    let mut looks_reversed: bool = (!is_empty as libc::c_int)
        < p_says_nonexistent[(reverse as libc::c_int ^ is_empty as libc::c_int)
            as usize];
    if is_empty as libc::c_int != 0
        && p_says_nonexistent[(reverse as libc::c_int ^ nonexistent as libc::c_int)
            as usize] == 1 as libc::c_int
        && p_says_nonexistent[(!reverse as libc::c_int ^ nonexistent as libc::c_int)
            as usize] == 2 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    if looks_reversed {
        reverse = (reverse as libc::c_int
            ^ ok_to_reverse(
                b"The next patch%s would %s the file %s,\nwhich %s!\0" as *const u8
                    as *const libc::c_char,
                if reverse as libc::c_int != 0 {
                    b", when reversed,\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                if nonexistent as libc::c_int != 0 {
                    b"delete\0" as *const u8 as *const libc::c_char
                } else if is_empty as libc::c_int != 0 {
                    b"empty out\0" as *const u8 as *const libc::c_char
                } else {
                    b"create\0" as *const u8 as *const libc::c_char
                },
                quotearg(name),
                if nonexistent as libc::c_int != 0 {
                    b"does not exist\0" as *const u8 as *const libc::c_char
                } else if is_empty as libc::c_int != 0 {
                    b"is already empty\0" as *const u8 as *const libc::c_char
                } else {
                    b"already exists\0" as *const u8 as *const libc::c_char
                },
            ) as libc::c_int) != 0;
    }
    return looks_reversed;
}
pub unsafe extern "C" fn there_is_another_patch(
    mut need_header: bool,
    mut file_type: *mut mode_t,
) -> bool {
    if p_base != 0 as libc::c_int as libc::c_long && p_base >= p_filesize {
        if verbosity as libc::c_uint == VERBOSE as libc::c_int as libc::c_uint {
            say(b"done\n\0" as *const u8 as *const libc::c_char);
        }
        return 0 as libc::c_int != 0;
    }
    if verbosity as libc::c_uint == VERBOSE as libc::c_int as libc::c_uint {
        say(b"Hmm...\0" as *const u8 as *const libc::c_char);
    }
    diff_type = intuit_diff_type(need_header, file_type);
    if diff_type as libc::c_uint == NO_DIFF as libc::c_int as libc::c_uint {
        if verbosity as libc::c_uint == VERBOSE as libc::c_int as libc::c_uint {
            say(
                if p_base != 0 {
                    b"  Ignoring the trailing garbage.\ndone\n\0" as *const u8
                        as *const libc::c_char
                } else {
                    b"  I can't seem to find a patch in there anywhere.\n\0" as *const u8
                        as *const libc::c_char
                },
            );
        }
        if p_base == 0 && p_filesize != 0 {
            fatal(
                b"Only garbage was found in the patch input.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0 as libc::c_int != 0;
    }
    if skip_rest_of_patch {
        Fseek(pfp, p_start, 0 as libc::c_int);
        p_input_line = p_sline - 1 as libc::c_int as libc::c_long;
        return 1 as libc::c_int != 0;
    }
    if verbosity as libc::c_uint == VERBOSE as libc::c_int as libc::c_uint {
        say(
            b"  %sooks like %s to me...\n\0" as *const u8 as *const libc::c_char,
            if p_base == 0 as libc::c_int as libc::c_long {
                b"L\0" as *const u8 as *const libc::c_char
            } else {
                b"The next patch l\0" as *const u8 as *const libc::c_char
            },
            if diff_type as libc::c_uint == UNI_DIFF as libc::c_int as libc::c_uint {
                b"a unified diff\0" as *const u8 as *const libc::c_char
            } else if diff_type as libc::c_uint
                == CONTEXT_DIFF as libc::c_int as libc::c_uint
            {
                b"a context diff\0" as *const u8 as *const libc::c_char
            } else if diff_type as libc::c_uint
                == NEW_CONTEXT_DIFF as libc::c_int as libc::c_uint
            {
                b"a new-style context diff\0" as *const u8 as *const libc::c_char
            } else if diff_type as libc::c_uint
                == NORMAL_DIFF as libc::c_int as libc::c_uint
            {
                b"a normal diff\0" as *const u8 as *const libc::c_char
            } else if diff_type as libc::c_uint
                == GIT_BINARY_DIFF as libc::c_int as libc::c_uint
            {
                b"a git binary diff\0" as *const u8 as *const libc::c_char
            } else {
                b"an ed script\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if no_strip_trailing_cr {
        p_strip_trailing_cr = 0 as libc::c_int != 0;
    }
    if verbosity as libc::c_uint != SILENT as libc::c_int as libc::c_uint {
        if p_indent != 0 {
            say(
                b"(Patch is indented %lu space%s.)\n\0" as *const u8
                    as *const libc::c_char,
                p_indent,
                if p_indent == 1 as libc::c_int as libc::c_ulong {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    b"s\0" as *const u8 as *const libc::c_char
                },
            );
        }
        if p_strip_trailing_cr {
            say(
                b"(Stripping trailing CRs from patch; use --binary to disable.)\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if inname.is_null() {
            let mut numbuf: [libc::c_char; 23] = [0; 23];
            say(
                b"can't find file to patch at input line %s\n\0" as *const u8
                    as *const libc::c_char,
                format_linenum(numbuf.as_mut_ptr(), p_sline),
            );
            if diff_type as libc::c_uint != ED_DIFF as libc::c_int as libc::c_uint
                && diff_type as libc::c_uint
                    != NORMAL_DIFF as libc::c_int as libc::c_uint
            {
                say(
                    if strippath == -(1 as libc::c_int) {
                        b"Perhaps you should have used the -p or --strip option?\n\0"
                            as *const u8 as *const libc::c_char
                    } else {
                        b"Perhaps you used the wrong -p or --strip option?\n\0"
                            as *const u8 as *const libc::c_char
                    },
                );
            }
        }
    }
    skip_to(p_start, p_sline);
    while inname.is_null() {
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        if force as libc::c_int | batch as libc::c_int != 0 {
            say(
                b"No file to patch.  Skipping patch.\n\0" as *const u8
                    as *const libc::c_char,
            );
            skip_rest_of_patch = 1 as libc::c_int != 0;
            return 1 as libc::c_int != 0;
        }
        ask(b"File to patch: \0" as *const u8 as *const libc::c_char);
        t = buf.offset(strlen(buf) as isize);
        if t > buf.offset(1 as libc::c_int as isize)
            && *t.offset(-(1 as libc::c_int as isize)) as libc::c_int == '\n' as i32
        {
            inname = xmemdup0(
                buf as *const libc::c_void,
                (t.offset_from(buf) as libc::c_long - 1 as libc::c_int as libc::c_long)
                    as size_t,
            );
            inerrno = stat_file(inname, &mut instat);
            if inerrno != 0 {
                perror(inname);
                fflush(stderr);
                free(inname as *mut libc::c_void);
                inname = 0 as *mut libc::c_char;
            } else {
                invc = -(1 as libc::c_int);
            }
        }
        if inname.is_null() {
            ask(b"Skip this patch? [y] \0" as *const u8 as *const libc::c_char);
            if *buf as libc::c_int != 'n' as i32 {
                if verbosity as libc::c_uint != SILENT as libc::c_int as libc::c_uint {
                    say(b"Skipping patch.\n\0" as *const u8 as *const libc::c_char);
                }
                skip_rest_of_patch = 1 as libc::c_int != 0;
                return 1 as libc::c_int != 0;
            }
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn fetchmode(mut str: *const libc::c_char) -> mode_t {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut mode: mode_t = 0;
    while 1 as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*str as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        str = str.offset(1);
        str;
    }
    s = str;
    mode = 0 as libc::c_int as mode_t;
    while s < str.offset(6 as libc::c_int as isize) {
        if *s as libc::c_int >= '0' as i32 && *s as libc::c_int <= '7' as i32 {
            mode = (mode << 3 as libc::c_int)
                .wrapping_add((*s as libc::c_int - '0' as i32) as libc::c_uint);
            s = s.offset(1);
            s;
        } else {
            mode = 0 as libc::c_int as mode_t;
            break;
        }
    }
    if *s as libc::c_int == '\r' as i32 {
        s = s.offset(1);
        s;
    }
    if *s as libc::c_int != '\n' as i32 {
        mode = 0 as libc::c_int as mode_t;
    }
    return mode;
}
unsafe extern "C" fn get_sha1(
    mut sha1: *mut *mut libc::c_char,
    mut start: *const libc::c_char,
    mut end: *const libc::c_char,
) {
    let mut len: libc::c_uint = end.offset_from(start) as libc::c_long as libc::c_uint;
    *sha1 = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_uint) as size_t)
        as *mut libc::c_char;
    memcpy(
        *sha1 as *mut libc::c_void,
        start as *const libc::c_void,
        len as libc::c_ulong,
    );
    *(*sha1).offset(len as isize) = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn sha1_says_nonexistent(
    mut sha1: *const libc::c_char,
) -> libc::c_int {
    let mut empty_sha1: *const libc::c_char = b"e69de29bb2d1d6434b8b29ae775ad8c2e48c5391\0"
        as *const u8 as *const libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    s = sha1;
    while *s != 0 {
        if *s as libc::c_int != '0' as i32 {
            break;
        }
        s = s.offset(1);
        s;
    }
    if *s == 0 {
        return 2 as libc::c_int;
    }
    s = sha1;
    while *s != 0 {
        if *s as libc::c_int != *empty_sha1 as libc::c_int {
            break;
        }
        s = s.offset(1);
        s;
        empty_sha1 = empty_sha1.offset(1);
        empty_sha1;
    }
    return (*s == 0) as libc::c_int;
}
unsafe extern "C" fn skip_hex_digits(
    mut str: *const libc::c_char,
) -> *const libc::c_char {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    s = str;
    while *s as libc::c_int >= '0' as i32 && *s as libc::c_int <= '9' as i32
        || *s as libc::c_int >= 'a' as i32 && *s as libc::c_int <= 'f' as i32
    {
        s = s.offset(1);
        s;
    }
    return if s == str { 0 as *const libc::c_char } else { s };
}
unsafe extern "C" fn name_is_valid(mut name: *const libc::c_char) -> bool {
    let mut i: libc::c_int = 0;
    let mut is_valid: bool = 1 as libc::c_int != 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[*const libc::c_char; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        if (invalid_names[i as usize]).is_null() {
            break;
        }
        if strcmp(invalid_names[i as usize], name) == 0 {
            return 0 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    is_valid = filename_is_safe(name);
    if !is_valid && cwd_is_root(name) as libc::c_int != 0 {
        is_valid = 1 as libc::c_int != 0;
    }
    if !is_valid {
        say(
            b"Ignoring potentially dangerous file name %s\n\0" as *const u8
                as *const libc::c_char,
            quotearg(name),
        );
        if (i as libc::c_ulong)
            < (::std::mem::size_of::<[*const libc::c_char; 2]>() as libc::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                )
        {
            invalid_names[i as usize] = name;
        }
    }
    return is_valid;
}
unsafe extern "C" fn intuit_diff_type(
    mut need_header: bool,
    mut p_file_type: *mut mode_t,
) -> diff {
    let mut this_line: file_offset = 0 as libc::c_int as file_offset;
    let mut first_command_line: file_offset = -(1 as libc::c_int) as file_offset;
    let mut first_ed_command_letter: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut fcl_line: lin = 0 as libc::c_int as lin;
    let mut this_is_a_command: bool = 0 as libc::c_int != 0;
    let mut stars_this_line: bool = 0 as libc::c_int != 0;
    let mut extended_headers: bool = 0 as libc::c_int != 0;
    let mut i: nametype = OLD;
    let mut st: [stat; 3] = [stat {
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
    }; 3];
    let mut stat_errno: [libc::c_int; 3] = [0; 3];
    let mut version_controlled: [libc::c_int; 3] = [0; 3];
    let mut retval: diff = NO_DIFF;
    let mut file_type: mode_t = 0;
    let mut indent: size_t = 0 as libc::c_int as size_t;
    i = OLD;
    while i as libc::c_uint <= INDEX as libc::c_int as libc::c_uint {
        if !(p_name[i as usize]).is_null() {
            free(p_name[i as usize] as *mut libc::c_void);
            p_name[i as usize] = 0 as *mut libc::c_char;
        }
        i += 1;
        i;
    }
    i = OLD;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[*const libc::c_char; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        invalid_names[i as usize] = 0 as *const libc::c_char;
        i += 1;
        i;
    }
    i = OLD;
    while i as libc::c_uint <= NEW as libc::c_int as libc::c_uint {
        if !(p_timestr[i as usize]).is_null() {
            free(p_timestr[i as usize] as *mut libc::c_void);
            p_timestr[i as usize] = 0 as *mut libc::c_char;
        }
        i += 1;
        i;
    }
    i = OLD;
    while i as libc::c_uint <= NEW as libc::c_int as libc::c_uint {
        if !(p_sha1[i as usize]).is_null() {
            free(p_sha1[i as usize] as *mut libc::c_void);
            p_sha1[i as usize] = 0 as *mut libc::c_char;
        }
        i += 1;
        i;
    }
    p_git_diff = 0 as libc::c_int != 0;
    i = OLD;
    while i as libc::c_uint <= NEW as libc::c_int as libc::c_uint {
        p_mode[i as usize] = 0 as libc::c_int as mode_t;
        p_copy[i as usize] = 0 as libc::c_int != 0;
        p_rename[i as usize] = 0 as libc::c_int != 0;
        i += 1;
        i;
    }
    if diff_type as libc::c_uint == ED_DIFF as libc::c_int as libc::c_uint
        || diff_type as libc::c_uint == NORMAL_DIFF as libc::c_int as libc::c_uint
    {
        need_header = 0 as libc::c_int != 0;
    }
    version_controlled[OLD as libc::c_int as usize] = -(1 as libc::c_int);
    version_controlled[NEW as libc::c_int as usize] = -(1 as libc::c_int);
    version_controlled[INDEX as libc::c_int as usize] = -(1 as libc::c_int);
    p_rfc934_nesting = 0 as libc::c_int;
    p_timestamp[NEW as libc::c_int as usize].tv_sec = -(1 as libc::c_int) as __time_t;
    p_timestamp[OLD as libc::c_int as usize]
        .tv_sec = p_timestamp[NEW as libc::c_int as usize].tv_sec;
    p_says_nonexistent[NEW as libc::c_int as usize] = 0 as libc::c_int;
    p_says_nonexistent[OLD as libc::c_int
        as usize] = p_says_nonexistent[NEW as libc::c_int as usize];
    Fseek(pfp, p_base, 0 as libc::c_int);
    p_input_line = p_bline - 1 as libc::c_int as libc::c_long;
    loop {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut previous_line: file_offset = this_line;
        let mut last_line_was_command: bool = this_is_a_command;
        let mut stars_last_line: bool = stars_this_line;
        let mut indent_last_line: size_t = indent;
        let mut ed_command_letter: libc::c_char = 0;
        let mut strip_trailing_cr: bool = false;
        let mut chars_read: size_t = 0;
        indent = 0 as libc::c_int as size_t;
        this_line = ftell(pfp);
        chars_read = pget_line(
            0 as libc::c_int as size_t,
            0 as libc::c_int,
            0 as libc::c_int != 0,
            0 as libc::c_int != 0,
        );
        if chars_read == -(1 as libc::c_int) as size_t {
            xalloc_die();
        }
        if chars_read == 0 {
            if first_ed_command_letter != 0 {
                p_start = first_command_line;
                p_sline = fcl_line;
                retval = ED_DIFF;
                break;
            } else {
                p_start = this_line;
                p_sline = p_input_line;
                if extended_headers {
                    retval = UNI_DIFF;
                    break;
                } else {
                    return NO_DIFF
                }
            }
        } else {
            strip_trailing_cr = 2 as libc::c_int as libc::c_ulong <= chars_read
                && *buf
                    .offset(
                        chars_read.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as libc::c_int == '\r' as i32;
            s = buf;
            while *s as libc::c_int == ' ' as i32 || *s as libc::c_int == '\t' as i32
                || *s as libc::c_int == 'X' as i32
            {
                if *s as libc::c_int == '\t' as i32 {
                    indent = indent.wrapping_add(8 as libc::c_int as libc::c_ulong)
                        & !(7 as libc::c_int) as libc::c_ulong;
                } else {
                    indent = indent.wrapping_add(1);
                    indent;
                }
                s = s.offset(1);
                s;
            }
            if (*s as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint
            {
                t = s.offset(1 as libc::c_int as isize);
                while (*t as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                    <= 9 as libc::c_int as libc::c_uint
                    || *t as libc::c_int == ',' as i32
                {
                    t = t.offset(1);
                    t;
                }
                if *t as libc::c_int == 'd' as i32 || *t as libc::c_int == 'c' as i32
                    || *t as libc::c_int == 'a' as i32
                {
                    t = t.offset(1);
                    t;
                    while (*t as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                        <= 9 as libc::c_int as libc::c_uint
                        || *t as libc::c_int == ',' as i32
                    {
                        t = t.offset(1);
                        t;
                    }
                    while *t as libc::c_int == ' ' as i32
                        || *t as libc::c_int == '\t' as i32
                    {
                        t = t.offset(1);
                        t;
                    }
                    if *t as libc::c_int == '\r' as i32 {
                        t = t.offset(1);
                        t;
                    }
                    this_is_a_command = *t as libc::c_int == '\n' as i32;
                }
            }
            if !need_header && first_command_line < 0 as libc::c_int as libc::c_long
                && {
                    ed_command_letter = get_ed_command_letter(s);
                    ed_command_letter as libc::c_int != 0
                        || this_is_a_command as libc::c_int != 0
                }
            {
                first_command_line = this_line;
                first_ed_command_letter = ed_command_letter;
                fcl_line = p_input_line;
                p_indent = indent;
                p_strip_trailing_cr = strip_trailing_cr;
            }
            if !stars_last_line
                && strncmp(
                    s,
                    b"*** \0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
            {
                fetchname(
                    s.offset(4 as libc::c_int as isize),
                    strippath,
                    &mut *p_name.as_mut_ptr().offset(OLD as libc::c_int as isize),
                    &mut *p_timestr.as_mut_ptr().offset(OLD as libc::c_int as isize),
                    &mut *p_timestamp.as_mut_ptr().offset(OLD as libc::c_int as isize),
                );
                need_header = 0 as libc::c_int != 0;
            } else if strncmp(
                s,
                b"+++ \0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                fetchname(
                    s.offset(4 as libc::c_int as isize),
                    strippath,
                    &mut *p_name.as_mut_ptr().offset(OLD as libc::c_int as isize),
                    &mut *p_timestr.as_mut_ptr().offset(OLD as libc::c_int as isize),
                    &mut *p_timestamp.as_mut_ptr().offset(OLD as libc::c_int as isize),
                );
                need_header = 0 as libc::c_int != 0;
                p_strip_trailing_cr = strip_trailing_cr;
            } else if strncmp(
                s,
                b"Index:\0" as *const u8 as *const libc::c_char,
                6 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                fetchname(
                    s.offset(6 as libc::c_int as isize),
                    strippath,
                    &mut *p_name.as_mut_ptr().offset(INDEX as libc::c_int as isize),
                    0 as *mut *mut libc::c_char,
                    0 as *mut timespec,
                );
                need_header = 0 as libc::c_int != 0;
                p_strip_trailing_cr = strip_trailing_cr;
            } else if strncmp(
                s,
                b"Prereq:\0" as *const u8 as *const libc::c_char,
                7 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                t = s.offset(7 as libc::c_int as isize);
                while 1 as libc::c_int != 0
                    && *(*__ctype_b_loc())
                        .offset(*t as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    t = t.offset(1);
                    t;
                }
                revision = t;
                t = revision;
                while *t != 0 {
                    if 1 as libc::c_int != 0
                        && *(*__ctype_b_loc())
                            .offset(*t as libc::c_uchar as libc::c_int as isize)
                            as libc::c_int
                            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                    {
                        let mut u: *const libc::c_char = 0 as *const libc::c_char;
                        u = t.offset(1 as libc::c_int as isize);
                        while 1 as libc::c_int != 0
                            && *(*__ctype_b_loc())
                                .offset(*u as libc::c_uchar as libc::c_int as isize)
                                as libc::c_int
                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                        {
                            u = u.offset(1);
                            u;
                        }
                        if *u != 0 {
                            let mut numbuf: [libc::c_char; 23] = [0; 23];
                            say(
                                b"Prereq: with multiple words at line %s of patch\n\0"
                                    as *const u8 as *const libc::c_char,
                                format_linenum(numbuf.as_mut_ptr(), this_line),
                            );
                        }
                        break;
                    } else {
                        t = t.offset(1);
                        t;
                    }
                }
                if t == revision {
                    revision = 0 as *mut libc::c_char;
                } else {
                    let mut oldc: libc::c_char = *t;
                    *t = '\0' as i32 as libc::c_char;
                    revision = xstrdup(revision);
                    *t = oldc;
                }
            } else if strncmp(
                s,
                b"diff --git \0" as *const u8 as *const libc::c_char,
                11 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                let mut u_0: *const libc::c_char = 0 as *const libc::c_char;
                if extended_headers {
                    p_start = this_line;
                    p_sline = p_input_line;
                    retval = UNI_DIFF;
                    break;
                } else {
                    i = OLD;
                    while i as libc::c_uint <= NEW as libc::c_int as libc::c_uint {
                        free(p_name[i as usize] as *mut libc::c_void);
                        p_name[i as usize] = 0 as *mut libc::c_char;
                        i += 1;
                        i;
                    }
                    p_name[OLD as libc::c_int
                        as usize] = parse_name(
                        s.offset(11 as libc::c_int as isize),
                        strippath,
                        &mut u_0,
                    );
                    if !(!(p_name[OLD as libc::c_int as usize]).is_null()
                        && (1 as libc::c_int != 0
                            && *(*__ctype_b_loc())
                                .offset(*u_0 as libc::c_uchar as libc::c_int as isize)
                                as libc::c_int
                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                != 0)
                        && {
                            p_name[NEW as libc::c_int
                                as usize] = parse_name(u_0, strippath, &mut u_0);
                            !(p_name[NEW as libc::c_int as usize]).is_null()
                        }
                        && {
                            u_0 = skip_spaces(u_0);
                            *u_0 == 0
                        })
                    {
                        i = OLD;
                        while i as libc::c_uint <= NEW as libc::c_int as libc::c_uint {
                            free(p_name[i as usize] as *mut libc::c_void);
                            p_name[i as usize] = 0 as *mut libc::c_char;
                            i += 1;
                            i;
                        }
                    }
                    p_git_diff = 1 as libc::c_int != 0;
                    need_header = 0 as libc::c_int != 0;
                }
            } else if p_git_diff as libc::c_int != 0
                && strncmp(
                    s,
                    b"index \0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0
            {
                let mut u_1: *const libc::c_char = 0 as *const libc::c_char;
                let mut v: *const libc::c_char = 0 as *const libc::c_char;
                u_1 = skip_hex_digits(s.offset(6 as libc::c_int as isize));
                if !u_1.is_null()
                    && *u_1.offset(0 as libc::c_int as isize) as libc::c_int
                        == '.' as i32
                    && *u_1.offset(1 as libc::c_int as isize) as libc::c_int
                        == '.' as i32
                    && {
                        v = skip_hex_digits(u_1.offset(2 as libc::c_int as isize));
                        !v.is_null()
                    }
                    && (*v == 0
                        || 1 as libc::c_int != 0
                            && *(*__ctype_b_loc())
                                .offset(*v as libc::c_uchar as libc::c_int as isize)
                                as libc::c_int
                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                != 0)
                {
                    get_sha1(
                        &mut *p_sha1.as_mut_ptr().offset(OLD as libc::c_int as isize),
                        s.offset(6 as libc::c_int as isize),
                        u_1,
                    );
                    get_sha1(
                        &mut *p_sha1.as_mut_ptr().offset(NEW as libc::c_int as isize),
                        u_1.offset(2 as libc::c_int as isize),
                        v,
                    );
                    p_says_nonexistent[OLD as libc::c_int
                        as usize] = sha1_says_nonexistent(
                        p_sha1[OLD as libc::c_int as usize],
                    );
                    p_says_nonexistent[NEW as libc::c_int
                        as usize] = sha1_says_nonexistent(
                        p_sha1[NEW as libc::c_int as usize],
                    );
                    v = skip_spaces(v);
                    if *v != 0 {
                        p_mode[NEW as libc::c_int as usize] = fetchmode(v);
                        p_mode[OLD as libc::c_int
                            as usize] = p_mode[NEW as libc::c_int as usize];
                    }
                    extended_headers = 1 as libc::c_int != 0;
                }
            } else if p_git_diff as libc::c_int != 0
                && strncmp(
                    s,
                    b"old mode \0" as *const u8 as *const libc::c_char,
                    9 as libc::c_int as libc::c_ulong,
                ) == 0
            {
                p_mode[OLD as libc::c_int
                    as usize] = fetchmode(s.offset(9 as libc::c_int as isize));
                extended_headers = 1 as libc::c_int != 0;
            } else if p_git_diff as libc::c_int != 0
                && strncmp(
                    s,
                    b"new mode \0" as *const u8 as *const libc::c_char,
                    9 as libc::c_int as libc::c_ulong,
                ) == 0
            {
                p_mode[NEW as libc::c_int
                    as usize] = fetchmode(s.offset(9 as libc::c_int as isize));
                extended_headers = 1 as libc::c_int != 0;
            } else if p_git_diff as libc::c_int != 0
                && strncmp(
                    s,
                    b"deleted file mode \0" as *const u8 as *const libc::c_char,
                    18 as libc::c_int as libc::c_ulong,
                ) == 0
            {
                p_mode[OLD as libc::c_int
                    as usize] = fetchmode(s.offset(18 as libc::c_int as isize));
                p_says_nonexistent[NEW as libc::c_int as usize] = 2 as libc::c_int;
                extended_headers = 1 as libc::c_int != 0;
            } else if p_git_diff as libc::c_int != 0
                && strncmp(
                    s,
                    b"new file mode \0" as *const u8 as *const libc::c_char,
                    14 as libc::c_int as libc::c_ulong,
                ) == 0
            {
                p_mode[NEW as libc::c_int
                    as usize] = fetchmode(s.offset(14 as libc::c_int as isize));
                p_says_nonexistent[OLD as libc::c_int as usize] = 2 as libc::c_int;
                extended_headers = 1 as libc::c_int != 0;
            } else if p_git_diff as libc::c_int != 0
                && strncmp(
                    s,
                    b"rename from \0" as *const u8 as *const libc::c_char,
                    12 as libc::c_int as libc::c_ulong,
                ) == 0
            {
                p_rename[OLD as libc::c_int as usize] = 1 as libc::c_int != 0;
                extended_headers = 1 as libc::c_int != 0;
            } else if p_git_diff as libc::c_int != 0
                && strncmp(
                    s,
                    b"rename to \0" as *const u8 as *const libc::c_char,
                    10 as libc::c_int as libc::c_ulong,
                ) == 0
            {
                p_rename[NEW as libc::c_int as usize] = 1 as libc::c_int != 0;
                extended_headers = 1 as libc::c_int != 0;
            } else if p_git_diff as libc::c_int != 0
                && strncmp(
                    s,
                    b"copy from \0" as *const u8 as *const libc::c_char,
                    10 as libc::c_int as libc::c_ulong,
                ) == 0
            {
                p_copy[OLD as libc::c_int as usize] = 1 as libc::c_int != 0;
                extended_headers = 1 as libc::c_int != 0;
            } else if p_git_diff as libc::c_int != 0
                && strncmp(
                    s,
                    b"copy to \0" as *const u8 as *const libc::c_char,
                    8 as libc::c_int as libc::c_ulong,
                ) == 0
            {
                p_copy[NEW as libc::c_int as usize] = 1 as libc::c_int != 0;
                extended_headers = 1 as libc::c_int != 0;
            } else if p_git_diff as libc::c_int != 0
                && strncmp(
                    s,
                    b"GIT binary patch\0" as *const u8 as *const libc::c_char,
                    16 as libc::c_int as libc::c_ulong,
                ) == 0
            {
                p_start = this_line;
                p_sline = p_input_line;
                retval = GIT_BINARY_DIFF;
                break;
            } else {
                t = s;
                while *t.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
                    && *t.offset(1 as libc::c_int as isize) as libc::c_int == ' ' as i32
                {
                    t = t.offset(2 as libc::c_int as isize);
                }
                if strncmp(
                    t,
                    b"--- \0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    let mut timestamp: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
                    timestamp.tv_sec = -(1 as libc::c_int) as __time_t;
                    fetchname(
                        t.offset(4 as libc::c_int as isize),
                        strippath,
                        &mut *p_name.as_mut_ptr().offset(NEW as libc::c_int as isize),
                        &mut *p_timestr.as_mut_ptr().offset(NEW as libc::c_int as isize),
                        &mut timestamp,
                    );
                    need_header = 0 as libc::c_int != 0;
                    if timestamp.tv_sec != -(1 as libc::c_int) as libc::c_long {
                        p_timestamp[NEW as libc::c_int as usize] = timestamp;
                        p_rfc934_nesting = (t.offset_from(s) as libc::c_long
                            >> 1 as libc::c_int) as libc::c_int;
                    }
                    p_strip_trailing_cr = strip_trailing_cr;
                }
            }
            if need_header {
                continue;
            }
            if (diff_type as libc::c_uint == NO_DIFF as libc::c_int as libc::c_uint
                || diff_type as libc::c_uint == ED_DIFF as libc::c_int as libc::c_uint)
                && first_command_line >= 0 as libc::c_int as libc::c_long
                && strcmp(s, b".\n\0" as *const u8 as *const libc::c_char) == 0
            {
                p_start = first_command_line;
                p_sline = fcl_line;
                retval = ED_DIFF;
                break;
            } else if (diff_type as libc::c_uint
                == NO_DIFF as libc::c_int as libc::c_uint
                || diff_type as libc::c_uint == UNI_DIFF as libc::c_int as libc::c_uint)
                && strncmp(
                    s,
                    b"@@ -\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
            {
                let mut ti: timespec = p_timestamp[OLD as libc::c_int as usize];
                p_timestamp[OLD as libc::c_int
                    as usize] = p_timestamp[NEW as libc::c_int as usize];
                p_timestamp[NEW as libc::c_int as usize] = ti;
                t = p_name[OLD as libc::c_int as usize];
                p_name[OLD as libc::c_int
                    as usize] = p_name[NEW as libc::c_int as usize];
                p_name[NEW as libc::c_int as usize] = t;
                t = p_timestr[OLD as libc::c_int as usize];
                p_timestr[OLD as libc::c_int
                    as usize] = p_timestr[NEW as libc::c_int as usize];
                p_timestr[NEW as libc::c_int as usize] = t;
                s = s.offset(4 as libc::c_int as isize);
                if *s.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
                    && !((*s.offset(1 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_sub('0' as i32 as libc::c_uint)
                        <= 9 as libc::c_int as libc::c_uint)
                {
                    p_says_nonexistent[OLD as libc::c_int
                        as usize] = 1 as libc::c_int
                        + (p_timestamp[OLD as libc::c_int as usize].tv_sec == 0)
                            as libc::c_int;
                }
                while *s as libc::c_int != ' ' as i32 && *s as libc::c_int != '\n' as i32
                {
                    s = s.offset(1);
                    s;
                }
                while *s as libc::c_int == ' ' as i32 {
                    s = s.offset(1);
                    s;
                }
                if *s.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
                    && *s.offset(1 as libc::c_int as isize) as libc::c_int == '0' as i32
                    && !((*s.offset(2 as libc::c_int as isize) as libc::c_uint)
                        .wrapping_sub('0' as i32 as libc::c_uint)
                        <= 9 as libc::c_int as libc::c_uint)
                {
                    p_says_nonexistent[NEW as libc::c_int
                        as usize] = 1 as libc::c_int
                        + (p_timestamp[NEW as libc::c_int as usize].tv_sec == 0)
                            as libc::c_int;
                }
                p_indent = indent;
                p_start = this_line;
                p_sline = p_input_line;
                retval = UNI_DIFF;
                if !((!(p_name[OLD as libc::c_int as usize]).is_null()
                    || p_timestamp[OLD as libc::c_int as usize].tv_sec == 0)
                    && (!(p_name[NEW as libc::c_int as usize]).is_null()
                        || p_timestamp[NEW as libc::c_int as usize].tv_sec == 0))
                    && (p_name[INDEX as libc::c_int as usize]).is_null()
                    && need_header as libc::c_int != 0
                {
                    let mut numbuf_0: [libc::c_char; 23] = [0; 23];
                    say(
                        b"missing header for unified diff at line %s of patch\n\0"
                            as *const u8 as *const libc::c_char,
                        format_linenum(numbuf_0.as_mut_ptr(), p_sline),
                    );
                }
                break;
            } else {
                stars_this_line = strncmp(
                    s,
                    b"********\0" as *const u8 as *const libc::c_char,
                    8 as libc::c_int as libc::c_ulong,
                ) == 0;
                if (diff_type as libc::c_uint == NO_DIFF as libc::c_int as libc::c_uint
                    || diff_type as libc::c_uint
                        == CONTEXT_DIFF as libc::c_int as libc::c_uint
                    || diff_type as libc::c_uint
                        == NEW_CONTEXT_DIFF as libc::c_int as libc::c_uint)
                    && stars_last_line as libc::c_int != 0 && indent_last_line == indent
                    && strncmp(
                        s,
                        b"*** \0" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as libc::c_ulong,
                    ) == 0
                {
                    s = s.offset(4 as libc::c_int as isize);
                    if *s.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
                        && !((*s.offset(1 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_sub('0' as i32 as libc::c_uint)
                            <= 9 as libc::c_int as libc::c_uint)
                    {
                        p_says_nonexistent[OLD as libc::c_int
                            as usize] = 1 as libc::c_int
                            + (p_timestamp[OLD as libc::c_int as usize].tv_sec == 0)
                                as libc::c_int;
                    }
                    while *s as libc::c_int != '\n' as i32 {
                        s = s.offset(1);
                        s;
                    }
                    p_indent = indent;
                    p_strip_trailing_cr = strip_trailing_cr;
                    p_start = previous_line;
                    p_sline = p_input_line - 1 as libc::c_int as libc::c_long;
                    retval = (if *s.offset(-(1 as libc::c_int as isize)) as libc::c_int
                        == '*' as i32
                    {
                        NEW_CONTEXT_DIFF as libc::c_int
                    } else {
                        CONTEXT_DIFF as libc::c_int
                    }) as diff;
                    let mut saved_p_base: file_offset = p_base;
                    let mut saved_p_bline: lin = p_bline;
                    Fseek(pfp, previous_line, 0 as libc::c_int);
                    p_input_line -= 2 as libc::c_int as libc::c_long;
                    if another_hunk(retval, 0 as libc::c_int != 0) != 0
                        && p_repl_lines == 0
                        && p_newfirst == 1 as libc::c_int as libc::c_long
                    {
                        p_says_nonexistent[NEW as libc::c_int
                            as usize] = 1 as libc::c_int
                            + (p_timestamp[NEW as libc::c_int as usize].tv_sec == 0)
                                as libc::c_int;
                    }
                    next_intuit_at(saved_p_base, saved_p_bline);
                    if !((!(p_name[OLD as libc::c_int as usize]).is_null()
                        || p_timestamp[OLD as libc::c_int as usize].tv_sec == 0)
                        && (!(p_name[NEW as libc::c_int as usize]).is_null()
                            || p_timestamp[NEW as libc::c_int as usize].tv_sec == 0))
                        && (p_name[INDEX as libc::c_int as usize]).is_null()
                        && need_header as libc::c_int != 0
                    {
                        let mut numbuf_1: [libc::c_char; 23] = [0; 23];
                        say(
                            b"missing header for context diff at line %s of patch\n\0"
                                as *const u8 as *const libc::c_char,
                            format_linenum(numbuf_1.as_mut_ptr(), p_sline),
                        );
                    }
                    break;
                } else {
                    if !((diff_type as libc::c_uint
                        == NO_DIFF as libc::c_int as libc::c_uint
                        || diff_type as libc::c_uint
                            == NORMAL_DIFF as libc::c_int as libc::c_uint)
                        && last_line_was_command as libc::c_int != 0
                        && (strncmp(
                            s,
                            b"< \0" as *const u8 as *const libc::c_char,
                            2 as libc::c_int as libc::c_ulong,
                        ) == 0
                            || strncmp(
                                s,
                                b"> \0" as *const u8 as *const libc::c_char,
                                2 as libc::c_int as libc::c_ulong,
                            ) == 0))
                    {
                        continue;
                    }
                    p_start = previous_line;
                    p_sline = p_input_line - 1 as libc::c_int as libc::c_long;
                    p_indent = indent;
                    retval = NORMAL_DIFF;
                    break;
                }
            }
        }
    }
    file_type = p_mode[OLD as libc::c_int as usize]
        & 0o170000 as libc::c_int as libc::c_uint;
    if file_type != 0 {
        let mut new_file_type: mode_t = p_mode[NEW as libc::c_int as usize]
            & 0o170000 as libc::c_int as libc::c_uint;
        if new_file_type != 0 && file_type != new_file_type {
            file_type = 0 as libc::c_int as mode_t;
        }
    } else {
        file_type = p_mode[NEW as libc::c_int as usize]
            & 0o170000 as libc::c_int as libc::c_uint;
        if file_type == 0 {
            file_type = 0o100000 as libc::c_int as mode_t;
        }
    }
    *p_file_type = file_type;
    i = NONE;
    if inname.is_null() {
        let mut i0: nametype = NONE;
        if !posixly_correct
            && (!(p_name[OLD as libc::c_int as usize]).is_null()
                || !(p_name[NEW as libc::c_int as usize]).is_null())
            && !(p_name[INDEX as libc::c_int as usize]).is_null()
        {
            free(p_name[INDEX as libc::c_int as usize] as *mut libc::c_void);
            p_name[INDEX as libc::c_int as usize] = 0 as *mut libc::c_char;
        }
        i = OLD;
        while i as libc::c_uint <= INDEX as libc::c_int as libc::c_uint {
            if !(p_name[i as usize]).is_null() {
                if i0 as libc::c_uint != NONE as libc::c_int as libc::c_uint
                    && strcmp(p_name[i0 as usize], p_name[i as usize])
                        == 0 as libc::c_int
                {
                    stat_errno[i as usize] = stat_errno[i0 as usize];
                    if stat_errno[i as usize] == 0 {
                        st[i as usize] = st[i0 as usize];
                    }
                } else {
                    stat_errno[i
                        as usize] = stat_file(
                        p_name[i as usize],
                        &mut *st.as_mut_ptr().offset(i as isize),
                    );
                    if stat_errno[i as usize] == 0 {
                        if lookup_file_id(&mut *st.as_mut_ptr().offset(i as isize))
                            as libc::c_uint
                            == DELETE_LATER as libc::c_int as libc::c_uint
                        {
                            stat_errno[i as usize] = 2 as libc::c_int;
                        } else if posixly_correct as libc::c_int != 0
                            && name_is_valid(p_name[i as usize]) as libc::c_int != 0
                        {
                            break;
                        }
                    }
                }
                i0 = i;
            }
            i += 1;
            i;
        }
        if !posixly_correct {
            i = best_name(p_name.as_mut_ptr(), stat_errno.as_mut_ptr());
            if i as libc::c_uint == NONE as libc::c_int as libc::c_uint && patch_get != 0
            {
                let mut nope: nametype = NONE;
                i = OLD;
                while i as libc::c_uint <= INDEX as libc::c_int as libc::c_uint {
                    if !(p_name[i as usize]).is_null() {
                        let mut cs: *const libc::c_char = 0 as *const libc::c_char;
                        let mut getbuf: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut diffbuf: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut readonly: bool = !outfile.is_null()
                            && strcmp(outfile, p_name[i as usize]) != 0 as libc::c_int;
                        if nope as libc::c_uint == NONE as libc::c_int as libc::c_uint
                            || strcmp(p_name[nope as usize], p_name[i as usize])
                                != 0 as libc::c_int
                        {
                            cs = version_controller(
                                p_name[i as usize],
                                readonly,
                                0 as *mut stat,
                                &mut getbuf,
                                &mut diffbuf,
                            );
                            version_controlled[i
                                as usize] = !cs.is_null() as libc::c_int;
                            if !cs.is_null() {
                                if version_get(
                                    p_name[i as usize],
                                    cs,
                                    0 as libc::c_int != 0,
                                    readonly,
                                    getbuf,
                                    &mut *st.as_mut_ptr().offset(i as isize),
                                ) {
                                    stat_errno[i as usize] = 0 as libc::c_int;
                                } else {
                                    version_controlled[i as usize] = 0 as libc::c_int;
                                }
                                free(getbuf as *mut libc::c_void);
                                free(diffbuf as *mut libc::c_void);
                                if stat_errno[i as usize] == 0 {
                                    break;
                                }
                            }
                        }
                        nope = i;
                    }
                    i += 1;
                    i;
                }
            }
            if i0 as libc::c_uint != NONE as libc::c_int as libc::c_uint
                && (i as libc::c_uint == NONE as libc::c_int as libc::c_uint
                    || st[i as usize].st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == file_type)
                && maybe_reverse(
                    p_name[(if i as libc::c_uint == NONE as libc::c_int as libc::c_uint {
                        i0 as libc::c_uint
                    } else {
                        i as libc::c_uint
                    }) as usize],
                    i as libc::c_uint == NONE as libc::c_int as libc::c_uint,
                    i as libc::c_uint == NONE as libc::c_int as libc::c_uint
                        || st[i as usize].st_size == 0 as libc::c_int as libc::c_long,
                ) as libc::c_int != 0
                && i as libc::c_uint == NONE as libc::c_int as libc::c_uint
            {
                i = i0;
            }
            if i as libc::c_uint == NONE as libc::c_int as libc::c_uint
                && p_says_nonexistent[reverse as usize] != 0
            {
                let mut newdirs: [libc::c_int; 3] = [0; 3];
                let mut newdirs_min: libc::c_int = 2147483647 as libc::c_int;
                let mut distance_from_minimum: [libc::c_int; 3] = [0; 3];
                i = OLD;
                while i as libc::c_uint <= INDEX as libc::c_int as libc::c_uint {
                    if !(p_name[i as usize]).is_null() {
                        newdirs[i
                            as usize] = prefix_components(
                            p_name[i as usize],
                            0 as libc::c_int != 0,
                        ) - prefix_components(p_name[i as usize], 1 as libc::c_int != 0);
                        if newdirs[i as usize] < newdirs_min {
                            newdirs_min = newdirs[i as usize];
                        }
                    }
                    i += 1;
                    i;
                }
                i = OLD;
                while i as libc::c_uint <= INDEX as libc::c_int as libc::c_uint {
                    if !(p_name[i as usize]).is_null() {
                        distance_from_minimum[i
                            as usize] = newdirs[i as usize] - newdirs_min;
                    }
                    i += 1;
                    i;
                }
                i = best_name(p_name.as_mut_ptr(), distance_from_minimum.as_mut_ptr());
            }
        }
    }
    if (pch_rename() as libc::c_int != 0 || pch_copy() as libc::c_int != 0)
        && inname.is_null()
        && !((i as libc::c_uint == OLD as libc::c_int as libc::c_uint
            || i as libc::c_uint == NEW as libc::c_int as libc::c_uint)
            && !(p_name[!reverse as libc::c_int as usize]).is_null()
            && name_is_valid(p_name[!reverse as libc::c_int as usize]) as libc::c_int
                != 0)
    {
        say(
            b"Cannot %s file without two valid file names\n\0" as *const u8
                as *const libc::c_char,
            if pch_rename() as libc::c_int != 0 {
                b"rename\0" as *const u8 as *const libc::c_char
            } else {
                b"copy\0" as *const u8 as *const libc::c_char
            },
        );
        skip_rest_of_patch = 1 as libc::c_int != 0;
    }
    if i as libc::c_uint == NONE as libc::c_int as libc::c_uint {
        if !inname.is_null() {
            inerrno = stat_file(inname, &mut instat);
            if inerrno != 0
                || instat.st_mode & 0o170000 as libc::c_int as libc::c_uint == file_type
            {
                maybe_reverse(
                    inname,
                    inerrno != 0,
                    inerrno != 0 || instat.st_size == 0 as libc::c_int as libc::c_long,
                );
            }
        } else {
            inerrno = -(1 as libc::c_int);
        }
    } else {
        inname = xstrdup(p_name[i as usize]);
        inerrno = stat_errno[i as usize];
        invc = version_controlled[i as usize];
        instat = st[i as usize];
    }
    return retval;
}
unsafe extern "C" fn prefix_components(
    mut filename: *mut libc::c_char,
    mut checkdirs: bool,
) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut stat_buf: stat = stat {
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
    let mut stat_result: libc::c_int = 0;
    let mut f: *mut libc::c_char = filename.offset(0 as libc::c_int as isize);
    if *f != 0 {
        loop {
            f = f.offset(1);
            if !(*f != 0) {
                break;
            }
            if !(*f.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
                && !(*f.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == '/' as i32))
            {
                continue;
            }
            if checkdirs {
                *f = '\0' as i32 as libc::c_char;
                stat_result = safe_stat(filename, &mut stat_buf);
                *f = '/' as i32 as libc::c_char;
                if !(stat_result == 0 as libc::c_int
                    && stat_buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint)
                {
                    break;
                }
            }
            count += 1;
            count;
        }
    }
    return count;
}
unsafe extern "C" fn best_name(
    mut name: *const *mut libc::c_char,
    mut ignore: *const libc::c_int,
) -> nametype {
    let mut i: nametype = OLD;
    let mut components: [libc::c_int; 3] = [0; 3];
    let mut components_min: libc::c_int = 2147483647 as libc::c_int;
    let mut basename_len: [size_t; 3] = [0; 3];
    let mut basename_len_min: size_t = 18446744073709551615 as libc::c_ulong;
    let mut len: [size_t; 3] = [0; 3];
    let mut len_min: size_t = 18446744073709551615 as libc::c_ulong;
    i = OLD;
    while i as libc::c_uint <= INDEX as libc::c_int as libc::c_uint {
        if !(*name.offset(i as isize)).is_null() && *ignore.offset(i as isize) == 0 {
            components[i
                as usize] = prefix_components(
                *name.offset(i as isize),
                0 as libc::c_int != 0,
            );
            if !(components_min < components[i as usize]) {
                components_min = components[i as usize];
                basename_len[i as usize] = base_len(*name.offset(i as isize));
                if !(basename_len_min < basename_len[i as usize]) {
                    basename_len_min = basename_len[i as usize];
                    len[i as usize] = strlen(*name.offset(i as isize));
                    if !(len_min < len[i as usize]) {
                        len_min = len[i as usize];
                    }
                }
            }
        }
        i += 1;
        i;
    }
    i = OLD;
    while i as libc::c_uint <= INDEX as libc::c_int as libc::c_uint {
        if !(*name.offset(i as isize)).is_null() && *ignore.offset(i as isize) == 0
            && name_is_valid(*name.offset(i as isize)) as libc::c_int != 0
            && components[i as usize] == components_min
            && basename_len[i as usize] == basename_len_min && len[i as usize] == len_min
        {
            break;
        }
        i += 1;
        i;
    }
    return i;
}
unsafe extern "C" fn next_intuit_at(mut file_pos: file_offset, mut file_line: lin) {
    p_base = file_pos;
    p_bline = file_line;
}
unsafe extern "C" fn skip_to(mut file_pos: file_offset, mut file_line: lin) {
    let mut i: *mut FILE = pfp;
    let mut o: *mut FILE = stdout;
    let mut c: libc::c_int = 0;
    if p_base <= file_pos {} else {
        __assert_fail(
            b"p_base <= file_pos\0" as *const u8 as *const libc::c_char,
            b"pch.c\0" as *const u8 as *const libc::c_char,
            1101 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void skip_to(file_offset, lin)\0"))
                .as_ptr(),
        );
    }
    'c_6750: {
        if p_base <= file_pos {} else {
            __assert_fail(
                b"p_base <= file_pos\0" as *const u8 as *const libc::c_char,
                b"pch.c\0" as *const u8 as *const libc::c_char,
                1101 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void skip_to(file_offset, lin)\0"))
                    .as_ptr(),
            );
        }
    };
    if (verbosity as libc::c_uint == VERBOSE as libc::c_int as libc::c_uint
        || inname.is_null()) && p_base < file_pos
    {
        Fseek(i, p_base, 0 as libc::c_int);
        say(
            b"The text leading up to this was:\n--------------------------\n\0"
                as *const u8 as *const libc::c_char,
        );
        while ftell(i) < file_pos {
            putc('|' as i32, o);
            loop {
                c = getc(i);
                if c == -(1 as libc::c_int) {
                    read_fatal();
                }
                putc(c, o);
                if !(c != '\n' as i32) {
                    break;
                }
            }
        }
        say(b"--------------------------\n\0" as *const u8 as *const libc::c_char);
    } else {
        Fseek(i, file_pos, 0 as libc::c_int);
    }
    p_input_line = file_line - 1 as libc::c_int as libc::c_long;
}
unsafe extern "C" fn malformed() -> ! {
    let mut numbuf: [libc::c_char; 23] = [0; 23];
    fatal(
        b"malformed patch at line %s: %s\0" as *const u8 as *const libc::c_char,
        format_linenum(numbuf.as_mut_ptr(), p_input_line),
        buf,
    );
}
unsafe extern "C" fn scan_linenum(
    mut s0: *mut libc::c_char,
    mut linenum: *mut lin,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: lin = 0 as libc::c_int as lin;
    let mut overflow: bool = 0 as libc::c_int != 0;
    let mut numbuf: [libc::c_char; 23] = [0; 23];
    s = s0;
    while (*s as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
        <= 9 as libc::c_int as libc::c_uint
    {
        let mut new_n: lin = 10 as libc::c_int as libc::c_long * n
            + (*s as libc::c_int - '0' as i32) as libc::c_long;
        overflow = (overflow as libc::c_int
            | (new_n / 10 as libc::c_int as libc::c_long != n) as libc::c_int) != 0;
        n = new_n;
        s = s.offset(1);
        s;
    }
    if s == s0 {
        fatal(
            b"missing line number at line %s: %s\0" as *const u8 as *const libc::c_char,
            format_linenum(numbuf.as_mut_ptr(), p_input_line),
            buf,
        );
    }
    if overflow {
        fatal(
            b"line number %.*s is too large at line %s: %s\0" as *const u8
                as *const libc::c_char,
            s.offset_from(s0) as libc::c_long as libc::c_int,
            s0,
            format_linenum(numbuf.as_mut_ptr(), p_input_line),
            buf,
        );
    }
    *linenum = n;
    return s;
}
pub unsafe extern "C" fn another_hunk(mut difftype: diff, mut rev: bool) -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut context: lin = 0 as libc::c_int as lin;
    let mut chars_read: size_t = 0;
    let mut numbuf0: [libc::c_char; 23] = [0; 23];
    let mut numbuf1: [libc::c_char; 23] = [0; 23];
    let mut numbuf2: [libc::c_char; 23] = [0; 23];
    let mut numbuf3: [libc::c_char; 23] = [0; 23];
    set_hunkmax();
    while p_end >= 0 as libc::c_int as libc::c_long {
        if p_end == p_efake {
            p_end = p_bfake;
        } else {
            free(*p_line.offset(p_end as isize) as *mut libc::c_void);
        }
        p_end -= 1;
        p_end;
    }
    if p_end == -(1 as libc::c_int) as libc::c_long {} else {
        __assert_fail(
            b"p_end == -1\0" as *const u8 as *const libc::c_char,
            b"pch.c\0" as *const u8 as *const libc::c_char,
            1187 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"int another_hunk(enum diff, _Bool)\0"))
                .as_ptr(),
        );
    }
    'c_14606: {
        if p_end == -(1 as libc::c_int) as libc::c_long {} else {
            __assert_fail(
                b"p_end == -1\0" as *const u8 as *const libc::c_char,
                b"pch.c\0" as *const u8 as *const libc::c_char,
                1187 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"int another_hunk(enum diff, _Bool)\0"))
                    .as_ptr(),
            );
        }
    };
    p_efake = -(1 as libc::c_int) as lin;
    if !p_c_function.is_null() {
        free(p_c_function as *mut libc::c_void);
        p_c_function = 0 as *mut libc::c_char;
    }
    p_max = hunkmax;
    if difftype as libc::c_uint == CONTEXT_DIFF as libc::c_int as libc::c_uint
        || difftype as libc::c_uint == NEW_CONTEXT_DIFF as libc::c_int as libc::c_uint
    {
        let mut line_beginning: file_offset = ftell(pfp);
        let mut repl_beginning: lin = 0 as libc::c_int as lin;
        let mut fillcnt: lin = 0 as libc::c_int as lin;
        let mut fillsrc: lin = 0;
        let mut filldst: lin = 0;
        let mut ptrn_spaces_eaten: bool = 0 as libc::c_int != 0;
        let mut some_context: bool = 0 as libc::c_int != 0;
        let mut repl_could_be_missing: bool = 1 as libc::c_int != 0;
        let mut ptrn_missing: bool = 0 as libc::c_int != 0;
        let mut repl_missing: bool = 0 as libc::c_int != 0;
        let mut repl_backtrack_position: file_offset = 0 as libc::c_int as file_offset;
        let mut repl_patch_line: lin = 0;
        let mut repl_context: lin = 0;
        let mut ptrn_prefix_context: lin = -(1 as libc::c_int) as lin;
        let mut ptrn_suffix_context: lin = -(1 as libc::c_int) as lin;
        let mut repl_prefix_context: lin = -(1 as libc::c_int) as lin;
        let mut ptrn_copiable: lin = 0 as libc::c_int as lin;
        let mut repl_copiable: lin = 0 as libc::c_int as lin;
        repl_context = 0 as libc::c_int as lin;
        repl_patch_line = repl_context;
        filldst = repl_patch_line;
        fillsrc = filldst;
        chars_read = get_line();
        if chars_read == -(1 as libc::c_int) as size_t
            || chars_read <= 8 as libc::c_int as libc::c_ulong
            || strncmp(
                buf,
                b"********\0" as *const u8 as *const libc::c_char,
                8 as libc::c_int as libc::c_ulong,
            ) != 0 as libc::c_int
        {
            next_intuit_at(line_beginning, p_input_line);
            return if chars_read == -(1 as libc::c_int) as size_t {
                -(1 as libc::c_int)
            } else {
                0 as libc::c_int
            };
        }
        s = buf;
        while *s as libc::c_int == '*' as i32 {
            s = s.offset(1);
            s;
        }
        if *s as libc::c_int == ' ' as i32 {
            p_c_function = s;
            while *s as libc::c_int != '\n' as i32 {
                s = s.offset(1);
                s;
            }
            *s = '\0' as i32 as libc::c_char;
            p_c_function = savestr(p_c_function);
            if p_c_function.is_null() {
                return -(1 as libc::c_int);
            }
        }
        p_hunk_beg = p_input_line + 1 as libc::c_int as libc::c_long;
        while p_end < p_max {
            chars_read = get_line();
            if chars_read == -(1 as libc::c_int) as size_t {
                return -(1 as libc::c_int);
            }
            if chars_read == 0 {
                if repl_beginning != 0 && repl_could_be_missing as libc::c_int != 0 {
                    repl_missing = 1 as libc::c_int != 0;
                    break;
                } else if p_max - p_end < 4 as libc::c_int as libc::c_long {
                    strcpy(buf, b"  \n\0" as *const u8 as *const libc::c_char);
                    chars_read = 3 as libc::c_int as size_t;
                } else {
                    fatal(
                        b"unexpected end of file in patch\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            p_end += 1;
            p_end;
            if p_end == hunkmax {
                fatal(
                    b"unterminated hunk starting at line %s; giving up at line %s: %s\0"
                        as *const u8 as *const libc::c_char,
                    format_linenum(numbuf0.as_mut_ptr(), pch_hunk_beg()),
                    format_linenum(numbuf1.as_mut_ptr(), p_input_line),
                    buf,
                );
            }
            if p_end < hunkmax {} else {
                __assert_fail(
                    b"p_end < hunkmax\0" as *const u8 as *const libc::c_char,
                    b"pch.c\0" as *const u8 as *const libc::c_char,
                    1264 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 35],
                        &[libc::c_char; 35],
                    >(b"int another_hunk(enum diff, _Bool)\0"))
                        .as_ptr(),
                );
            }
            'c_14298: {
                if p_end < hunkmax {} else {
                    __assert_fail(
                        b"p_end < hunkmax\0" as *const u8 as *const libc::c_char,
                        b"pch.c\0" as *const u8 as *const libc::c_char,
                        1264 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 35],
                            &[libc::c_char; 35],
                        >(b"int another_hunk(enum diff, _Bool)\0"))
                            .as_ptr(),
                    );
                }
            };
            *p_Char.offset(p_end as isize) = *buf;
            *p_len.offset(p_end as isize) = 0 as libc::c_int as size_t;
            let ref mut fresh0 = *p_line.offset(p_end as isize);
            *fresh0 = 0 as *mut libc::c_char;
            match *buf as libc::c_int {
                42 => {
                    if strncmp(
                        buf,
                        b"********\0" as *const u8 as *const libc::c_char,
                        8 as libc::c_int as libc::c_ulong,
                    ) == 0
                    {
                        if repl_beginning != 0
                            && repl_could_be_missing as libc::c_int != 0
                        {
                            repl_missing = 1 as libc::c_int != 0;
                            break;
                        } else {
                            fatal(
                                b"unexpected end of hunk at line %s\0" as *const u8
                                    as *const libc::c_char,
                                format_linenum(numbuf0.as_mut_ptr(), p_input_line),
                            );
                        }
                    } else if p_end != 0 as libc::c_int as libc::c_long {
                        if repl_beginning != 0
                            && repl_could_be_missing as libc::c_int != 0
                        {
                            repl_missing = 1 as libc::c_int != 0;
                            break;
                        } else {
                            fatal(
                                b"unexpected '***' at line %s: %s\0" as *const u8
                                    as *const libc::c_char,
                                format_linenum(numbuf0.as_mut_ptr(), p_input_line),
                                buf,
                            );
                        }
                    } else {
                        context = 0 as libc::c_int as lin;
                        *p_len.offset(p_end as isize) = strlen(buf);
                        let ref mut fresh1 = *p_line.offset(p_end as isize);
                        *fresh1 = savestr(buf);
                        if (*fresh1).is_null() {
                            p_end -= 1;
                            p_end;
                            return -(1 as libc::c_int);
                        }
                        s = buf;
                        while *s as libc::c_int != 0
                            && !((*s as libc::c_uint)
                                .wrapping_sub('0' as i32 as libc::c_uint)
                                <= 9 as libc::c_int as libc::c_uint)
                        {
                            s = s.offset(1);
                            s;
                        }
                        if strncmp(
                            s,
                            b"0,0\0" as *const u8 as *const libc::c_char,
                            3 as libc::c_int as libc::c_ulong,
                        ) == 0
                        {
                            remove_prefix(s, 2 as libc::c_int as size_t);
                        }
                        s = scan_linenum(s, &mut p_first);
                        if *s as libc::c_int == ',' as i32 {
                            while *s as libc::c_int != 0
                                && !((*s as libc::c_uint)
                                    .wrapping_sub('0' as i32 as libc::c_uint)
                                    <= 9 as libc::c_int as libc::c_uint)
                            {
                                s = s.offset(1);
                                s;
                            }
                            scan_linenum(s, &mut p_ptrn_lines);
                            p_ptrn_lines += 1 as libc::c_int as libc::c_long - p_first;
                            if p_ptrn_lines < 0 as libc::c_int as libc::c_long {
                                malformed();
                            }
                        } else if p_first != 0 {
                            p_ptrn_lines = 1 as libc::c_int as lin;
                        } else {
                            p_ptrn_lines = 0 as libc::c_int as lin;
                            p_first = 1 as libc::c_int as lin;
                        }
                        if p_first
                            >= (if (0 as libc::c_int as lin) < -(1 as libc::c_int) as lin
                            {
                                -(1 as libc::c_int) as lin
                            } else {
                                (((1 as libc::c_int as lin)
                                    << (::std::mem::size_of::<lin>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            }) - p_ptrn_lines
                            || p_ptrn_lines
                                >= (if (0 as libc::c_int as lin)
                                    < -(1 as libc::c_int) as lin
                                {
                                    -(1 as libc::c_int) as lin
                                } else {
                                    (((1 as libc::c_int as lin)
                                        << (::std::mem::size_of::<lin>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                }) - 6 as libc::c_int as libc::c_long
                        {
                            malformed();
                        }
                        p_max = p_ptrn_lines + 6 as libc::c_int as libc::c_long;
                        while p_max + 1 as libc::c_int as libc::c_long >= hunkmax {
                            if !grow_hunkmax() {
                                return -(1 as libc::c_int);
                            }
                        }
                        p_max = hunkmax;
                        continue;
                    }
                }
                45 => {
                    if !(*buf.offset(1 as libc::c_int as isize) as libc::c_int
                        != '-' as i32)
                    {
                        if ptrn_prefix_context == -(1 as libc::c_int) as libc::c_long {
                            ptrn_prefix_context = context;
                        }
                        ptrn_suffix_context = context;
                        if repl_beginning != 0
                            || p_end
                                != p_ptrn_lines + 1 as libc::c_int as libc::c_long
                                    + (*p_Char
                                        .offset((p_end - 1 as libc::c_int as libc::c_long) as isize)
                                        as libc::c_int == '\n' as i32) as libc::c_int
                                        as libc::c_long
                        {
                            if p_end == 1 as libc::c_int as libc::c_long {
                                ptrn_missing = 1 as libc::c_int != 0;
                                p_end = p_ptrn_lines + 1 as libc::c_int as libc::c_long;
                                ptrn_suffix_context = -(1 as libc::c_int) as lin;
                                ptrn_prefix_context = ptrn_suffix_context;
                                fillsrc = p_end + 1 as libc::c_int as libc::c_long;
                                filldst = 1 as libc::c_int as lin;
                                fillcnt = p_ptrn_lines;
                            } else if repl_beginning == 0 {
                                fatal(
                                    b"%s '---' at line %s; check line numbers at line %s\0"
                                        as *const u8 as *const libc::c_char,
                                    if p_end <= p_ptrn_lines {
                                        b"Premature\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b"Overdue\0" as *const u8 as *const libc::c_char
                                    },
                                    format_linenum(numbuf0.as_mut_ptr(), p_input_line),
                                    format_linenum(numbuf1.as_mut_ptr(), p_hunk_beg),
                                );
                            } else if !repl_could_be_missing {
                                fatal(
                                    b"duplicate '---' at line %s; check line numbers at line %s\0"
                                        as *const u8 as *const libc::c_char,
                                    format_linenum(numbuf0.as_mut_ptr(), p_input_line),
                                    format_linenum(
                                        numbuf1.as_mut_ptr(),
                                        p_hunk_beg + repl_beginning,
                                    ),
                                );
                            } else {
                                repl_missing = 1 as libc::c_int != 0;
                                break;
                            }
                        }
                        repl_beginning = p_end;
                        repl_backtrack_position = ftell(pfp);
                        repl_patch_line = p_input_line;
                        repl_context = context;
                        *p_len.offset(p_end as isize) = strlen(buf);
                        let ref mut fresh2 = *p_line.offset(p_end as isize);
                        *fresh2 = savestr(buf);
                        if (*fresh2).is_null() {
                            p_end -= 1;
                            p_end;
                            return -(1 as libc::c_int);
                        }
                        *p_Char.offset(p_end as isize) = '=' as i32 as libc::c_char;
                        s = buf;
                        while *s as libc::c_int != 0
                            && !((*s as libc::c_uint)
                                .wrapping_sub('0' as i32 as libc::c_uint)
                                <= 9 as libc::c_int as libc::c_uint)
                        {
                            s = s.offset(1);
                            s;
                        }
                        s = scan_linenum(s, &mut p_newfirst);
                        if *s as libc::c_int == ',' as i32 {
                            loop {
                                s = s.offset(1);
                                if *s == 0 {
                                    malformed();
                                }
                                if (*s as libc::c_uint)
                                    .wrapping_sub('0' as i32 as libc::c_uint)
                                    <= 9 as libc::c_int as libc::c_uint
                                {
                                    break;
                                }
                            }
                            scan_linenum(s, &mut p_repl_lines);
                            p_repl_lines
                                += 1 as libc::c_int as libc::c_long - p_newfirst;
                            if p_repl_lines < 0 as libc::c_int as libc::c_long {
                                malformed();
                            }
                        } else if p_newfirst != 0 {
                            p_repl_lines = 1 as libc::c_int as lin;
                        } else {
                            p_repl_lines = 0 as libc::c_int as lin;
                            p_newfirst = 1 as libc::c_int as lin;
                        }
                        if p_newfirst
                            >= (if (0 as libc::c_int as lin) < -(1 as libc::c_int) as lin
                            {
                                -(1 as libc::c_int) as lin
                            } else {
                                (((1 as libc::c_int as lin)
                                    << (::std::mem::size_of::<lin>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            }) - p_repl_lines
                            || p_repl_lines
                                >= (if (0 as libc::c_int as lin)
                                    < -(1 as libc::c_int) as lin
                                {
                                    -(1 as libc::c_int) as lin
                                } else {
                                    (((1 as libc::c_int as lin)
                                        << (::std::mem::size_of::<lin>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                }) - p_end
                        {
                            malformed();
                        }
                        p_max = p_repl_lines + p_end;
                        while p_max + 1 as libc::c_int as libc::c_long >= hunkmax {
                            if !grow_hunkmax() {
                                return -(1 as libc::c_int);
                            }
                        }
                        if p_repl_lines != ptrn_copiable
                            && (p_prefix_context != 0 as libc::c_int as libc::c_long
                                || context != 0 as libc::c_int as libc::c_long
                                || p_repl_lines != 1 as libc::c_int as libc::c_long)
                        {
                            repl_could_be_missing = 0 as libc::c_int != 0;
                        }
                        context = 0 as libc::c_int as lin;
                        continue;
                    }
                }
                43 | 33 => {
                    repl_could_be_missing = 0 as libc::c_int != 0;
                }
                9 | 10 => {
                    s = buf;
                    if *buf as libc::c_int == '\t' as i32 {
                        s = s.offset(1);
                        s;
                        chars_read = chars_read.wrapping_sub(1);
                        chars_read;
                    }
                    if repl_beginning != 0 && repl_could_be_missing as libc::c_int != 0
                        && (!ptrn_spaces_eaten
                            || difftype as libc::c_uint
                                == NEW_CONTEXT_DIFF as libc::c_int as libc::c_uint)
                    {
                        repl_missing = 1 as libc::c_int != 0;
                        break;
                    } else {
                        chars_read = (chars_read as libc::c_ulong)
                            .wrapping_sub(
                                ((1 as libc::c_int as libc::c_ulong) < chars_read
                                    && p_end
                                        == (if repl_beginning != 0 { p_max } else { p_ptrn_lines })
                                    && incomplete_line() as libc::c_int != 0) as libc::c_int
                                    as libc::c_ulong,
                            ) as size_t as size_t;
                        *p_len.offset(p_end as isize) = chars_read;
                        let ref mut fresh4 = *p_line.offset(p_end as isize);
                        *fresh4 = savebuf(buf, chars_read);
                        if chars_read != 0 && (*p_line.offset(p_end as isize)).is_null()
                        {
                            p_end -= 1;
                            p_end;
                            return -(1 as libc::c_int);
                        }
                        if p_end != p_ptrn_lines + 1 as libc::c_int as libc::c_long {
                            ptrn_spaces_eaten = (ptrn_spaces_eaten as libc::c_int
                                | (repl_beginning != 0 as libc::c_int as libc::c_long)
                                    as libc::c_int) != 0;
                            some_context = 1 as libc::c_int != 0;
                            context += 1;
                            context;
                            if repl_beginning != 0 {
                                repl_copiable += 1;
                                repl_copiable;
                            } else {
                                ptrn_copiable += 1;
                                ptrn_copiable;
                            }
                            *p_Char.offset(p_end as isize) = ' ' as i32 as libc::c_char;
                        }
                        continue;
                    }
                }
                32 => {
                    s = buf.offset(1 as libc::c_int as isize);
                    chars_read = chars_read.wrapping_sub(1);
                    chars_read;
                    if *s as libc::c_int == '\n' as i32
                        && canonicalize_ws as libc::c_int != 0
                    {
                        strcpy(s, b"\n\0" as *const u8 as *const libc::c_char);
                        chars_read = 2 as libc::c_int as size_t;
                    }
                    if *s as libc::c_int == ' ' as i32
                        || *s as libc::c_int == '\t' as i32
                    {
                        s = s.offset(1);
                        s;
                        chars_read = chars_read.wrapping_sub(1);
                        chars_read;
                    } else if repl_beginning != 0
                        && repl_could_be_missing as libc::c_int != 0
                    {
                        repl_missing = 1 as libc::c_int != 0;
                        break;
                    }
                    some_context = 1 as libc::c_int != 0;
                    context += 1;
                    context;
                    if repl_beginning != 0 {
                        repl_copiable += 1;
                        repl_copiable;
                    } else {
                        ptrn_copiable += 1;
                        ptrn_copiable;
                    }
                    chars_read = (chars_read as libc::c_ulong)
                        .wrapping_sub(
                            ((1 as libc::c_int as libc::c_ulong) < chars_read
                                && p_end
                                    == (if repl_beginning != 0 { p_max } else { p_ptrn_lines })
                                && incomplete_line() as libc::c_int != 0) as libc::c_int
                                as libc::c_ulong,
                        ) as size_t as size_t;
                    *p_len.offset(p_end as isize) = chars_read;
                    let ref mut fresh5 = *p_line.offset(p_end as isize);
                    *fresh5 = savebuf(s, chars_read);
                    if chars_read != 0 && (*p_line.offset(p_end as isize)).is_null() {
                        p_end -= 1;
                        p_end;
                        return -(1 as libc::c_int);
                    }
                    continue;
                }
                _ => {
                    if repl_beginning != 0 && repl_could_be_missing as libc::c_int != 0 {
                        repl_missing = 1 as libc::c_int != 0;
                        break;
                    } else {
                        malformed();
                    }
                }
            }
            s = buf.offset(1 as libc::c_int as isize);
            chars_read = chars_read.wrapping_sub(1);
            chars_read;
            if *s as libc::c_int == '\n' as i32 && canonicalize_ws as libc::c_int != 0 {
                strcpy(s, b" \n\0" as *const u8 as *const libc::c_char);
                chars_read = 2 as libc::c_int as size_t;
            }
            if *s as libc::c_int == ' ' as i32 || *s as libc::c_int == '\t' as i32 {
                s = s.offset(1);
                s;
                chars_read = chars_read.wrapping_sub(1);
                chars_read;
            } else if repl_beginning != 0 && repl_could_be_missing as libc::c_int != 0 {
                repl_missing = 1 as libc::c_int != 0;
                break;
            }
            if repl_beginning == 0 {
                if ptrn_prefix_context == -(1 as libc::c_int) as libc::c_long {
                    ptrn_prefix_context = context;
                }
            } else if repl_prefix_context == -(1 as libc::c_int) as libc::c_long {
                repl_prefix_context = context;
            }
            chars_read = (chars_read as libc::c_ulong)
                .wrapping_sub(
                    ((1 as libc::c_int as libc::c_ulong) < chars_read
                        && p_end
                            == (if repl_beginning != 0 { p_max } else { p_ptrn_lines })
                        && incomplete_line() as libc::c_int != 0) as libc::c_int
                        as libc::c_ulong,
                ) as size_t as size_t;
            *p_len.offset(p_end as isize) = chars_read;
            let ref mut fresh3 = *p_line.offset(p_end as isize);
            *fresh3 = savebuf(s, chars_read);
            if chars_read != 0 && (*p_line.offset(p_end as isize)).is_null() {
                p_end -= 1;
                p_end;
                return -(1 as libc::c_int);
            }
            context = 0 as libc::c_int as lin;
        }
        if p_end >= 0 as libc::c_int as libc::c_long && repl_beginning == 0 {
            fatal(
                b"no '---' found in patch at line %s\0" as *const u8
                    as *const libc::c_char,
                format_linenum(numbuf0.as_mut_ptr(), pch_hunk_beg()),
            );
        }
        if repl_missing {
            p_input_line = repl_patch_line;
            context = repl_context;
            p_end -= 1;
            p_end;
            while p_end > repl_beginning {
                free(*p_line.offset(p_end as isize) as *mut libc::c_void);
                p_end -= 1;
                p_end;
            }
            Fseek(pfp, repl_backtrack_position, 0 as libc::c_int);
            fillsrc = 1 as libc::c_int as lin;
            filldst = repl_beginning + 1 as libc::c_int as libc::c_long;
            fillcnt = p_repl_lines;
            p_end = p_max;
        } else if !ptrn_missing && ptrn_copiable != repl_copiable {
            fatal(
                b"context mangled in hunk at line %s\0" as *const u8
                    as *const libc::c_char,
                format_linenum(numbuf0.as_mut_ptr(), p_hunk_beg),
            );
        } else if !some_context && fillcnt == 1 as libc::c_int as libc::c_long {
            while filldst < p_end {
                let ref mut fresh6 = *p_line.offset(filldst as isize);
                *fresh6 = *p_line
                    .offset((filldst + 1 as libc::c_int as libc::c_long) as isize);
                *p_Char
                    .offset(
                        filldst as isize,
                    ) = *p_Char
                    .offset((filldst + 1 as libc::c_int as libc::c_long) as isize);
                *p_len
                    .offset(
                        filldst as isize,
                    ) = *p_len
                    .offset((filldst + 1 as libc::c_int as libc::c_long) as isize);
                filldst += 1;
                filldst;
            }
            p_end -= 1;
            p_end;
            p_first += 1;
            p_first;
            fillcnt = 0 as libc::c_int as lin;
            p_ptrn_lines = 0 as libc::c_int as lin;
        }
        p_prefix_context = if repl_prefix_context == -(1 as libc::c_int) as libc::c_long
            || ptrn_prefix_context != -(1 as libc::c_int) as libc::c_long
                && ptrn_prefix_context < repl_prefix_context
        {
            ptrn_prefix_context
        } else {
            repl_prefix_context
        };
        p_suffix_context = if ptrn_suffix_context != -(1 as libc::c_int) as libc::c_long
            && ptrn_suffix_context < context
        {
            ptrn_suffix_context
        } else {
            context
        };
        if p_prefix_context == -(1 as libc::c_int) as libc::c_long
            || p_suffix_context == -(1 as libc::c_int) as libc::c_long
        {
            fatal(
                b"replacement text or line numbers mangled in hunk at line %s\0"
                    as *const u8 as *const libc::c_char,
                format_linenum(numbuf0.as_mut_ptr(), p_hunk_beg),
            );
        }
        if difftype as libc::c_uint == CONTEXT_DIFF as libc::c_int as libc::c_uint
            && (fillcnt != 0
                || p_first > 1 as libc::c_int as libc::c_long
                    && p_prefix_context + p_suffix_context < ptrn_copiable)
        {
            if verbosity as libc::c_uint == VERBOSE as libc::c_int as libc::c_uint {
                say(
                    b"%s\n%s\n%s\n\0" as *const u8 as *const libc::c_char,
                    b"(Fascinating -- this is really a new-style context diff but without\0"
                        as *const u8 as *const libc::c_char,
                    b"the telltale extra asterisks on the *** line that usually indicate\0"
                        as *const u8 as *const libc::c_char,
                    b"the new style...)\0" as *const u8 as *const libc::c_char,
                );
            }
            difftype = NEW_CONTEXT_DIFF;
            diff_type = difftype;
        }
        if fillcnt != 0 {
            p_bfake = filldst;
            p_efake = filldst + fillcnt - 1 as libc::c_int as libc::c_long;
            loop {
                let fresh7 = fillcnt;
                fillcnt = fillcnt - 1;
                if !(fresh7 > 0 as libc::c_int as libc::c_long) {
                    break;
                }
                while fillsrc <= p_end && fillsrc != repl_beginning
                    && *p_Char.offset(fillsrc as isize) as libc::c_int != ' ' as i32
                {
                    fillsrc += 1;
                    fillsrc;
                }
                if p_end < fillsrc || fillsrc == repl_beginning {
                    fatal(
                        b"replacement text or line numbers mangled in hunk at line %s\0"
                            as *const u8 as *const libc::c_char,
                        format_linenum(numbuf0.as_mut_ptr(), p_hunk_beg),
                    );
                }
                let ref mut fresh8 = *p_line.offset(filldst as isize);
                *fresh8 = *p_line.offset(fillsrc as isize);
                *p_Char.offset(filldst as isize) = *p_Char.offset(fillsrc as isize);
                *p_len.offset(filldst as isize) = *p_len.offset(fillsrc as isize);
                fillsrc += 1;
                fillsrc;
                filldst += 1;
                filldst;
            }
            while fillsrc <= p_end && fillsrc != repl_beginning {
                if *p_Char.offset(fillsrc as isize) as libc::c_int == ' ' as i32 {
                    fatal(
                        b"replacement text or line numbers mangled in hunk at line %s\0"
                            as *const u8 as *const libc::c_char,
                        format_linenum(numbuf0.as_mut_ptr(), p_hunk_beg),
                    );
                }
                fillsrc += 1;
                fillsrc;
            }
            if debug & 64 as libc::c_int != 0 {
                printf(
                    b"fillsrc %s, filldst %s, rb %s, e+1 %s\n\0" as *const u8
                        as *const libc::c_char,
                    format_linenum(numbuf0.as_mut_ptr(), fillsrc),
                    format_linenum(numbuf1.as_mut_ptr(), filldst),
                    format_linenum(numbuf2.as_mut_ptr(), repl_beginning),
                    format_linenum(
                        numbuf3.as_mut_ptr(),
                        p_end + 1 as libc::c_int as libc::c_long,
                    ),
                );
            }
            if fillsrc == p_end + 1 as libc::c_int as libc::c_long
                || fillsrc == repl_beginning
            {} else {
                __assert_fail(
                    b"fillsrc==p_end+1 || fillsrc==repl_beginning\0" as *const u8
                        as *const libc::c_char,
                    b"pch.c\0" as *const u8 as *const libc::c_char,
                    1614 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 35],
                        &[libc::c_char; 35],
                    >(b"int another_hunk(enum diff, _Bool)\0"))
                        .as_ptr(),
                );
            }
            'c_12173: {
                if fillsrc == p_end + 1 as libc::c_int as libc::c_long
                    || fillsrc == repl_beginning
                {} else {
                    __assert_fail(
                        b"fillsrc==p_end+1 || fillsrc==repl_beginning\0" as *const u8
                            as *const libc::c_char,
                        b"pch.c\0" as *const u8 as *const libc::c_char,
                        1614 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 35],
                            &[libc::c_char; 35],
                        >(b"int another_hunk(enum diff, _Bool)\0"))
                            .as_ptr(),
                    );
                }
            };
            if filldst == p_end + 1 as libc::c_int as libc::c_long
                || filldst == repl_beginning
            {} else {
                __assert_fail(
                    b"filldst==p_end+1 || filldst==repl_beginning\0" as *const u8
                        as *const libc::c_char,
                    b"pch.c\0" as *const u8 as *const libc::c_char,
                    1615 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 35],
                        &[libc::c_char; 35],
                    >(b"int another_hunk(enum diff, _Bool)\0"))
                        .as_ptr(),
                );
            }
            'c_12112: {
                if filldst == p_end + 1 as libc::c_int as libc::c_long
                    || filldst == repl_beginning
                {} else {
                    __assert_fail(
                        b"filldst==p_end+1 || filldst==repl_beginning\0" as *const u8
                            as *const libc::c_char,
                        b"pch.c\0" as *const u8 as *const libc::c_char,
                        1615 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 35],
                            &[libc::c_char; 35],
                        >(b"int another_hunk(enum diff, _Bool)\0"))
                            .as_ptr(),
                    );
                }
            };
        }
    } else if difftype as libc::c_uint == UNI_DIFF as libc::c_int as libc::c_uint {
        let mut line_beginning_0: file_offset = ftell(pfp);
        let mut fillsrc_0: lin = 0;
        let mut filldst_0: lin = 0;
        let mut ch: libc::c_char = '\0' as i32 as libc::c_char;
        chars_read = get_line();
        if chars_read == -(1 as libc::c_int) as size_t
            || chars_read <= 4 as libc::c_int as libc::c_ulong
            || strncmp(
                buf,
                b"@@ -\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) != 0 as libc::c_int
        {
            next_intuit_at(line_beginning_0, p_input_line);
            return if chars_read == -(1 as libc::c_int) as size_t {
                -(1 as libc::c_int)
            } else {
                0 as libc::c_int
            };
        }
        s = scan_linenum(buf.offset(4 as libc::c_int as isize), &mut p_first);
        if *s as libc::c_int == ',' as i32 {
            s = scan_linenum(s.offset(1 as libc::c_int as isize), &mut p_ptrn_lines);
        } else {
            p_ptrn_lines = 1 as libc::c_int as lin;
        }
        if p_first
            >= (if (0 as libc::c_int as lin) < -(1 as libc::c_int) as lin {
                -(1 as libc::c_int) as lin
            } else {
                (((1 as libc::c_int as lin)
                    << (::std::mem::size_of::<lin>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            }) - p_ptrn_lines
        {
            malformed();
        }
        if *s as libc::c_int == ' ' as i32 {
            s = s.offset(1);
            s;
        }
        if *s as libc::c_int != '+' as i32 {
            malformed();
        }
        s = scan_linenum(s.offset(1 as libc::c_int as isize), &mut p_newfirst);
        if *s as libc::c_int == ',' as i32 {
            s = scan_linenum(s.offset(1 as libc::c_int as isize), &mut p_repl_lines);
        } else {
            p_repl_lines = 1 as libc::c_int as lin;
        }
        if p_newfirst
            >= (if (0 as libc::c_int as lin) < -(1 as libc::c_int) as lin {
                -(1 as libc::c_int) as lin
            } else {
                (((1 as libc::c_int as lin)
                    << (::std::mem::size_of::<lin>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            }) - p_repl_lines
        {
            malformed();
        }
        if *s as libc::c_int == ' ' as i32 {
            s = s.offset(1);
            s;
        }
        let fresh9 = s;
        s = s.offset(1);
        if *fresh9 as libc::c_int != '@' as i32 {
            malformed();
        }
        let fresh10 = s;
        s = s.offset(1);
        if *fresh10 as libc::c_int == '@' as i32 && *s as libc::c_int == ' ' as i32 {
            p_c_function = s;
            while *s as libc::c_int != '\n' as i32 {
                s = s.offset(1);
                s;
            }
            *s = '\0' as i32 as libc::c_char;
            p_c_function = savestr(p_c_function);
            if p_c_function.is_null() {
                return -(1 as libc::c_int);
            }
        }
        if p_ptrn_lines == 0 {
            p_first += 1;
            p_first;
        }
        if p_repl_lines == 0 {
            p_newfirst += 1;
            p_newfirst;
        }
        if p_ptrn_lines
            >= (if (0 as libc::c_int as lin) < -(1 as libc::c_int) as lin {
                -(1 as libc::c_int) as lin
            } else {
                (((1 as libc::c_int as lin)
                    << (::std::mem::size_of::<lin>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            }) - (p_repl_lines + 1 as libc::c_int as libc::c_long)
        {
            malformed();
        }
        p_max = p_ptrn_lines + p_repl_lines + 1 as libc::c_int as libc::c_long;
        while p_max + 1 as libc::c_int as libc::c_long >= hunkmax {
            if !grow_hunkmax() {
                return -(1 as libc::c_int);
            }
        }
        fillsrc_0 = 1 as libc::c_int as lin;
        filldst_0 = fillsrc_0 + p_ptrn_lines;
        p_end = filldst_0 + p_repl_lines;
        sprintf(
            buf,
            b"*** %s,%s ****\n\0" as *const u8 as *const libc::c_char,
            format_linenum(numbuf0.as_mut_ptr(), p_first),
            format_linenum(
                numbuf1.as_mut_ptr(),
                p_first + p_ptrn_lines - 1 as libc::c_int as libc::c_long,
            ),
        );
        *p_len.offset(0 as libc::c_int as isize) = strlen(buf);
        let ref mut fresh11 = *p_line.offset(0 as libc::c_int as isize);
        *fresh11 = savestr(buf);
        if (*fresh11).is_null() {
            p_end = -(1 as libc::c_int) as lin;
            return -(1 as libc::c_int);
        }
        *p_Char.offset(0 as libc::c_int as isize) = '*' as i32 as libc::c_char;
        sprintf(
            buf,
            b"--- %s,%s ----\n\0" as *const u8 as *const libc::c_char,
            format_linenum(numbuf0.as_mut_ptr(), p_newfirst),
            format_linenum(
                numbuf1.as_mut_ptr(),
                p_newfirst + p_repl_lines - 1 as libc::c_int as libc::c_long,
            ),
        );
        *p_len.offset(filldst_0 as isize) = strlen(buf);
        let ref mut fresh12 = *p_line.offset(filldst_0 as isize);
        *fresh12 = savestr(buf);
        if (*fresh12).is_null() {
            p_end = 0 as libc::c_int as lin;
            return -(1 as libc::c_int);
        }
        let fresh13 = filldst_0;
        filldst_0 = filldst_0 + 1;
        *p_Char.offset(fresh13 as isize) = '=' as i32 as libc::c_char;
        p_prefix_context = -(1 as libc::c_int) as lin;
        p_hunk_beg = p_input_line + 1 as libc::c_int as libc::c_long;
        while fillsrc_0 <= p_ptrn_lines || filldst_0 <= p_end {
            chars_read = get_line();
            if chars_read == 0 {
                if p_max - filldst_0 < 3 as libc::c_int as libc::c_long {
                    strcpy(buf, b" \n\0" as *const u8 as *const libc::c_char);
                    chars_read = 2 as libc::c_int as size_t;
                } else {
                    fatal(
                        b"unexpected end of file in patch\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            if chars_read == -(1 as libc::c_int) as size_t {
                s = 0 as *mut libc::c_char;
            } else if *buf as libc::c_int == '\t' as i32
                || *buf as libc::c_int == '\n' as i32
            {
                ch = ' ' as i32 as libc::c_char;
                s = savebuf(buf, chars_read);
            } else {
                ch = *buf;
                chars_read = chars_read.wrapping_sub(1);
                s = savebuf(buf.offset(1 as libc::c_int as isize), chars_read);
            }
            if chars_read != 0 && s.is_null() {
                loop {
                    filldst_0 -= 1;
                    if !(filldst_0 > p_ptrn_lines) {
                        break;
                    }
                    free(*p_line.offset(filldst_0 as isize) as *mut libc::c_void);
                }
                p_end = fillsrc_0 - 1 as libc::c_int as libc::c_long;
                return -(1 as libc::c_int);
            }
            let mut current_block_399: u64;
            match ch as libc::c_int {
                45 => {
                    if fillsrc_0 > p_ptrn_lines {
                        free(s as *mut libc::c_void);
                        p_end = filldst_0 - 1 as libc::c_int as libc::c_long;
                        malformed();
                    }
                    chars_read = (chars_read as libc::c_ulong)
                        .wrapping_sub(
                            (fillsrc_0 == p_ptrn_lines
                                && incomplete_line() as libc::c_int != 0) as libc::c_int
                                as libc::c_ulong,
                        ) as size_t as size_t;
                    *p_Char.offset(fillsrc_0 as isize) = ch;
                    let ref mut fresh14 = *p_line.offset(fillsrc_0 as isize);
                    *fresh14 = s;
                    let fresh15 = fillsrc_0;
                    fillsrc_0 = fillsrc_0 + 1;
                    *p_len.offset(fresh15 as isize) = chars_read;
                    current_block_399 = 4606643245351973654;
                }
                61 => {
                    ch = ' ' as i32 as libc::c_char;
                    current_block_399 = 111708979099322087;
                }
                32 => {
                    current_block_399 = 111708979099322087;
                }
                43 => {
                    current_block_399 = 17777263985481006496;
                }
                _ => {
                    p_end = filldst_0;
                    malformed();
                }
            }
            match current_block_399 {
                111708979099322087 => {
                    if fillsrc_0 > p_ptrn_lines {
                        free(s as *mut libc::c_void);
                        loop {
                            filldst_0 -= 1;
                            if !(filldst_0 > p_ptrn_lines) {
                                break;
                            }
                            free(
                                *p_line.offset(filldst_0 as isize) as *mut libc::c_void,
                            );
                        }
                        p_end = fillsrc_0 - 1 as libc::c_int as libc::c_long;
                        malformed();
                    }
                    context += 1;
                    context;
                    chars_read = (chars_read as libc::c_ulong)
                        .wrapping_sub(
                            (fillsrc_0 == p_ptrn_lines
                                && incomplete_line() as libc::c_int != 0) as libc::c_int
                                as libc::c_ulong,
                        ) as size_t as size_t;
                    *p_Char.offset(fillsrc_0 as isize) = ch;
                    let ref mut fresh16 = *p_line.offset(fillsrc_0 as isize);
                    *fresh16 = s;
                    let fresh17 = fillsrc_0;
                    fillsrc_0 = fillsrc_0 + 1;
                    *p_len.offset(fresh17 as isize) = chars_read;
                    s = savebuf(s, chars_read);
                    if chars_read != 0 && s.is_null() {
                        loop {
                            filldst_0 -= 1;
                            if !(filldst_0 > p_ptrn_lines) {
                                break;
                            }
                            free(
                                *p_line.offset(filldst_0 as isize) as *mut libc::c_void,
                            );
                        }
                        p_end = fillsrc_0 - 1 as libc::c_int as libc::c_long;
                        return -(1 as libc::c_int);
                    }
                    current_block_399 = 17777263985481006496;
                }
                _ => {}
            }
            match current_block_399 {
                17777263985481006496 => {
                    if filldst_0 > p_end {
                        free(s as *mut libc::c_void);
                        loop {
                            filldst_0 -= 1;
                            if !(filldst_0 > p_ptrn_lines) {
                                break;
                            }
                            free(
                                *p_line.offset(filldst_0 as isize) as *mut libc::c_void,
                            );
                        }
                        p_end = fillsrc_0 - 1 as libc::c_int as libc::c_long;
                        malformed();
                    }
                    chars_read = (chars_read as libc::c_ulong)
                        .wrapping_sub(
                            (filldst_0 == p_end && incomplete_line() as libc::c_int != 0)
                                as libc::c_int as libc::c_ulong,
                        ) as size_t as size_t;
                    *p_Char.offset(filldst_0 as isize) = ch;
                    let ref mut fresh18 = *p_line.offset(filldst_0 as isize);
                    *fresh18 = s;
                    let fresh19 = filldst_0;
                    filldst_0 = filldst_0 + 1;
                    *p_len.offset(fresh19 as isize) = chars_read;
                }
                _ => {}
            }
            if ch as libc::c_int != ' ' as i32 {
                if p_prefix_context == -(1 as libc::c_int) as libc::c_long {
                    p_prefix_context = context;
                }
                context = 0 as libc::c_int as lin;
            }
        }
        if p_prefix_context == -(1 as libc::c_int) as libc::c_long {
            malformed();
        }
        p_suffix_context = context;
    } else {
        let mut hunk_type: libc::c_char = 0;
        let mut i: libc::c_int = 0;
        let mut min: lin = 0;
        let mut max: lin = 0;
        let mut line_beginning_1: file_offset = ftell(pfp);
        p_suffix_context = 0 as libc::c_int as lin;
        p_prefix_context = p_suffix_context;
        chars_read = get_line();
        if chars_read == -(1 as libc::c_int) as size_t || chars_read == 0
            || !((*buf as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint)
        {
            next_intuit_at(line_beginning_1, p_input_line);
            return if chars_read == -(1 as libc::c_int) as size_t {
                -(1 as libc::c_int)
            } else {
                0 as libc::c_int
            };
        }
        s = scan_linenum(buf, &mut p_first);
        if *s as libc::c_int == ',' as i32 {
            s = scan_linenum(s.offset(1 as libc::c_int as isize), &mut p_ptrn_lines);
            p_ptrn_lines += 1 as libc::c_int as libc::c_long - p_first;
        } else {
            p_ptrn_lines = (*s as libc::c_int != 'a' as i32) as libc::c_int as lin;
        }
        if p_first
            >= (if (0 as libc::c_int as lin) < -(1 as libc::c_int) as lin {
                -(1 as libc::c_int) as lin
            } else {
                (((1 as libc::c_int as lin)
                    << (::std::mem::size_of::<lin>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            }) - p_ptrn_lines
        {
            malformed();
        }
        hunk_type = *s;
        if hunk_type as libc::c_int == 'a' as i32 {
            p_first += 1;
            p_first;
        }
        s = scan_linenum(s.offset(1 as libc::c_int as isize), &mut min);
        if *s as libc::c_int == ',' as i32 {
            scan_linenum(s.offset(1 as libc::c_int as isize), &mut max);
        } else {
            max = min;
        }
        if min > max
            || max - min
                == (if (0 as libc::c_int as lin) < -(1 as libc::c_int) as lin {
                    -(1 as libc::c_int) as lin
                } else {
                    (((1 as libc::c_int as lin)
                        << (::std::mem::size_of::<lin>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                })
        {
            malformed();
        }
        if hunk_type as libc::c_int == 'd' as i32 {
            min += 1;
            min;
        }
        p_newfirst = min;
        p_repl_lines = max - min + 1 as libc::c_int as libc::c_long;
        if p_newfirst
            >= (if (0 as libc::c_int as lin) < -(1 as libc::c_int) as lin {
                -(1 as libc::c_int) as lin
            } else {
                (((1 as libc::c_int as lin)
                    << (::std::mem::size_of::<lin>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            }) - p_repl_lines
        {
            malformed();
        }
        if p_ptrn_lines
            >= (if (0 as libc::c_int as lin) < -(1 as libc::c_int) as lin {
                -(1 as libc::c_int) as lin
            } else {
                (((1 as libc::c_int as lin)
                    << (::std::mem::size_of::<lin>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            }) - (p_repl_lines + 1 as libc::c_int as libc::c_long)
        {
            malformed();
        }
        p_end = p_ptrn_lines + p_repl_lines + 1 as libc::c_int as libc::c_long;
        while p_end + 1 as libc::c_int as libc::c_long >= hunkmax {
            if !grow_hunkmax() {
                p_end = -(1 as libc::c_int) as lin;
                return -(1 as libc::c_int);
            }
        }
        sprintf(
            buf,
            b"*** %s,%s\n\0" as *const u8 as *const libc::c_char,
            format_linenum(numbuf0.as_mut_ptr(), p_first),
            format_linenum(
                numbuf1.as_mut_ptr(),
                p_first + p_ptrn_lines - 1 as libc::c_int as libc::c_long,
            ),
        );
        *p_len.offset(0 as libc::c_int as isize) = strlen(buf);
        let ref mut fresh20 = *p_line.offset(0 as libc::c_int as isize);
        *fresh20 = savestr(buf);
        if (*fresh20).is_null() {
            p_end = -(1 as libc::c_int) as lin;
            return -(1 as libc::c_int);
        }
        *p_Char.offset(0 as libc::c_int as isize) = '*' as i32 as libc::c_char;
        i = 1 as libc::c_int;
        while i as libc::c_long <= p_ptrn_lines {
            chars_read = get_line();
            if chars_read == -(1 as libc::c_int) as size_t {
                p_end = (i - 1 as libc::c_int) as lin;
                return -(1 as libc::c_int);
            }
            if chars_read == 0 {
                fatal(
                    b"unexpected end of file in patch at line %s\0" as *const u8
                        as *const libc::c_char,
                    format_linenum(numbuf0.as_mut_ptr(), p_input_line),
                );
            }
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int != '<' as i32
                || *buf.offset(1 as libc::c_int as isize) as libc::c_int != ' ' as i32
                    && *buf.offset(1 as libc::c_int as isize) as libc::c_int
                        != '\t' as i32
            {
                fatal(
                    b"'<' followed by space or tab expected at line %s of patch\0"
                        as *const u8 as *const libc::c_char,
                    format_linenum(numbuf0.as_mut_ptr(), p_input_line),
                );
            }
            chars_read = (chars_read as libc::c_ulong)
                .wrapping_sub(
                    (2 as libc::c_int
                        + (i as libc::c_long == p_ptrn_lines
                            && incomplete_line() as libc::c_int != 0) as libc::c_int)
                        as libc::c_ulong,
                ) as size_t as size_t;
            *p_len.offset(i as isize) = chars_read;
            let ref mut fresh21 = *p_line.offset(i as isize);
            *fresh21 = savebuf(buf.offset(2 as libc::c_int as isize), chars_read);
            if chars_read != 0 && (*p_line.offset(i as isize)).is_null() {
                p_end = (i - 1 as libc::c_int) as lin;
                return -(1 as libc::c_int);
            }
            *p_Char.offset(i as isize) = '-' as i32 as libc::c_char;
            i += 1;
            i;
        }
        if hunk_type as libc::c_int == 'c' as i32 {
            chars_read = get_line();
            if chars_read == -(1 as libc::c_int) as size_t {
                p_end = (i - 1 as libc::c_int) as lin;
                return -(1 as libc::c_int);
            }
            if chars_read == 0 {
                fatal(
                    b"unexpected end of file in patch at line %s\0" as *const u8
                        as *const libc::c_char,
                    format_linenum(numbuf0.as_mut_ptr(), p_input_line),
                );
            }
            if *buf as libc::c_int != '-' as i32 {
                fatal(
                    b"'---' expected at line %s of patch\0" as *const u8
                        as *const libc::c_char,
                    format_linenum(numbuf0.as_mut_ptr(), p_input_line),
                );
            }
        }
        sprintf(
            buf,
            b"--- %s,%s\n\0" as *const u8 as *const libc::c_char,
            format_linenum(numbuf0.as_mut_ptr(), min),
            format_linenum(numbuf1.as_mut_ptr(), max),
        );
        *p_len.offset(i as isize) = strlen(buf);
        let ref mut fresh22 = *p_line.offset(i as isize);
        *fresh22 = savestr(buf);
        if (*fresh22).is_null() {
            p_end = (i - 1 as libc::c_int) as lin;
            return -(1 as libc::c_int);
        }
        *p_Char.offset(i as isize) = '=' as i32 as libc::c_char;
        i += 1;
        i;
        while i as libc::c_long <= p_end {
            chars_read = get_line();
            if chars_read == -(1 as libc::c_int) as size_t {
                p_end = (i - 1 as libc::c_int) as lin;
                return -(1 as libc::c_int);
            }
            if chars_read == 0 {
                fatal(
                    b"unexpected end of file in patch at line %s\0" as *const u8
                        as *const libc::c_char,
                    format_linenum(numbuf0.as_mut_ptr(), p_input_line),
                );
            }
            if *buf.offset(0 as libc::c_int as isize) as libc::c_int != '>' as i32
                || *buf.offset(1 as libc::c_int as isize) as libc::c_int != ' ' as i32
                    && *buf.offset(1 as libc::c_int as isize) as libc::c_int
                        != '\t' as i32
            {
                fatal(
                    b"'>' followed by space or tab expected at line %s of patch\0"
                        as *const u8 as *const libc::c_char,
                    format_linenum(numbuf0.as_mut_ptr(), p_input_line),
                );
            }
            chars_read = (chars_read as libc::c_ulong)
                .wrapping_sub(
                    (2 as libc::c_int
                        + (i as libc::c_long == p_end
                            && incomplete_line() as libc::c_int != 0) as libc::c_int)
                        as libc::c_ulong,
                ) as size_t as size_t;
            *p_len.offset(i as isize) = chars_read;
            let ref mut fresh23 = *p_line.offset(i as isize);
            *fresh23 = savebuf(buf.offset(2 as libc::c_int as isize), chars_read);
            if chars_read != 0 && (*p_line.offset(i as isize)).is_null() {
                p_end = (i - 1 as libc::c_int) as lin;
                return -(1 as libc::c_int);
            }
            *p_Char.offset(i as isize) = '+' as i32 as libc::c_char;
            i += 1;
            i;
        }
    }
    if rev {
        if !pch_swap() {
            say(
                b"Not enough memory to swap next hunk!\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if (p_end + 1 as libc::c_int as libc::c_long) < hunkmax {} else {
        __assert_fail(
            b"p_end + 1 < hunkmax\0" as *const u8 as *const libc::c_char,
            b"pch.c\0" as *const u8 as *const libc::c_char,
            1910 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"int another_hunk(enum diff, _Bool)\0"))
                .as_ptr(),
        );
    }
    'c_9129: {
        if (p_end + 1 as libc::c_int as libc::c_long) < hunkmax {} else {
            __assert_fail(
                b"p_end + 1 < hunkmax\0" as *const u8 as *const libc::c_char,
                b"pch.c\0" as *const u8 as *const libc::c_char,
                1910 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"int another_hunk(enum diff, _Bool)\0"))
                    .as_ptr(),
            );
        }
    };
    *p_Char
        .offset(
            (p_end + 1 as libc::c_int as libc::c_long) as isize,
        ) = '^' as i32 as libc::c_char;
    if debug & 2 as libc::c_int != 0 {
        let mut i_0: lin = 0;
        i_0 = 0 as libc::c_int as lin;
        while i_0 <= p_end + 1 as libc::c_int as libc::c_long {
            fprintf(
                stderr,
                b"%s %c\0" as *const u8 as *const libc::c_char,
                format_linenum(numbuf0.as_mut_ptr(), i_0),
                *p_Char.offset(i_0 as isize) as libc::c_int,
            );
            if *p_Char.offset(i_0 as isize) as libc::c_int == '*' as i32 {
                fprintf(
                    stderr,
                    b" %s,%s\n\0" as *const u8 as *const libc::c_char,
                    format_linenum(numbuf0.as_mut_ptr(), p_first),
                    format_linenum(numbuf1.as_mut_ptr(), p_ptrn_lines),
                );
            } else if *p_Char.offset(i_0 as isize) as libc::c_int == '=' as i32 {
                fprintf(
                    stderr,
                    b" %s,%s\n\0" as *const u8 as *const libc::c_char,
                    format_linenum(numbuf0.as_mut_ptr(), p_newfirst),
                    format_linenum(numbuf1.as_mut_ptr(), p_repl_lines),
                );
            } else if *p_Char.offset(i_0 as isize) as libc::c_int != '^' as i32 {
                fputs(b" |\0" as *const u8 as *const libc::c_char, stderr);
                pch_write_line(i_0, stderr);
            } else {
                fputc('\n' as i32, stderr);
            }
            i_0 += 1;
            i_0;
        }
        fflush(stderr);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn get_line() -> size_t {
    return pget_line(
        p_indent,
        p_rfc934_nesting,
        p_strip_trailing_cr,
        p_pass_comments_through,
    );
}
unsafe extern "C" fn pget_line(
    mut indent: size_t,
    mut rfc934_nesting: libc::c_int,
    mut strip_trailing_cr: bool,
    mut pass_comments_through: bool,
) -> size_t {
    let mut current_block: u64;
    let mut fp: *mut FILE = pfp;
    let mut c: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: size_t = 0;
    's_14: loop {
        i = 0 as libc::c_int as size_t;
        loop {
            c = getc(fp);
            if c == -(1 as libc::c_int) {
                if ferror(fp) != 0 {
                    read_fatal();
                }
                return 0 as libc::c_int as size_t;
            }
            if indent <= i {
                break;
            }
            if c == ' ' as i32 || c == 'X' as i32 {
                i = i.wrapping_add(1);
                i;
            } else {
                if !(c == '\t' as i32) {
                    break;
                }
                i = i.wrapping_add(8 as libc::c_int as libc::c_ulong)
                    & !(7 as libc::c_int) as libc::c_ulong;
            }
        }
        i = 0 as libc::c_int as size_t;
        b = buf;
        while c == '-' as i32
            && {
                rfc934_nesting -= 1;
                0 as libc::c_int <= rfc934_nesting
            }
        {
            c = getc(fp);
            if c == -(1 as libc::c_int) {
                current_block = 3169862890712579680;
                break 's_14;
            }
            if c != ' ' as i32 {
                i = 1 as libc::c_int as size_t;
                *b.offset(0 as libc::c_int as isize) = '-' as i32 as libc::c_char;
                break;
            } else {
                c = getc(fp);
                if c == -(1 as libc::c_int) {
                    current_block = 3169862890712579680;
                    break 's_14;
                }
            }
        }
        s = bufsize;
        loop {
            if i == s.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                s = (s as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
                b = realloc(b as *mut libc::c_void, s) as *mut libc::c_char;
                if b.is_null() {
                    if !using_plan_a {
                        xalloc_die();
                    }
                    return -(1 as libc::c_int) as size_t;
                }
                buf = b;
                bufsize = s;
            }
            let fresh24 = i;
            i = i.wrapping_add(1);
            *b.offset(fresh24 as isize) = c as libc::c_char;
            if c == '\n' as i32 {
                break;
            }
            c = getc(fp);
            if c == -(1 as libc::c_int) {
                current_block = 3169862890712579680;
                break 's_14;
            }
        }
        p_input_line += 1;
        p_input_line;
        if !(*b as libc::c_int == '#' as i32 && !pass_comments_through) {
            current_block = 6417057564578538666;
            break;
        }
    }
    match current_block {
        3169862890712579680 => {
            if ferror(fp) != 0 {
                read_fatal();
            }
            say(
                b"patch unexpectedly ends in middle of line\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int as size_t;
        }
        _ => {
            if strip_trailing_cr as libc::c_int != 0
                && 2 as libc::c_int as libc::c_ulong <= i
                && *b.offset(i.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int == '\r' as i32
            {
                let fresh25 = i;
                i = i.wrapping_sub(1);
                *b
                    .offset(
                        fresh25.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
                    ) = '\n' as i32 as libc::c_char;
            }
            *b.offset(i as isize) = '\0' as i32 as libc::c_char;
            return i;
        }
    };
}
unsafe extern "C" fn incomplete_line() -> bool {
    let mut fp: *mut FILE = pfp;
    let mut c: libc::c_int = 0;
    let mut line_beginning: file_offset = ftell(fp);
    if getc(fp) == '\\' as i32 {
        loop {
            c = getc(fp);
            if !(c != '\n' as i32 && c != -(1 as libc::c_int)) {
                break;
            }
        }
        return 1 as libc::c_int != 0;
    } else {
        Fseek(pfp, line_beginning, 0 as libc::c_int);
        return 0 as libc::c_int != 0;
    };
}
pub unsafe extern "C" fn pch_swap() -> bool {
    let mut tp_line: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tp_len: *mut size_t = 0 as *mut size_t;
    let mut tp_char: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: lin = 0;
    let mut n: lin = 0;
    let mut blankline: bool = 0 as libc::c_int != 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    i = p_first;
    p_first = p_newfirst;
    p_newfirst = i;
    tp_line = p_line;
    tp_len = p_len;
    tp_char = p_Char;
    p_line = 0 as *mut *mut libc::c_char;
    p_len = 0 as *mut size_t;
    p_Char = 0 as *mut libc::c_char;
    set_hunkmax();
    if p_line.is_null() || p_len.is_null() || p_Char.is_null() {
        free(p_line as *mut libc::c_void);
        p_line = tp_line;
        free(p_len as *mut libc::c_void);
        p_len = tp_len;
        free(p_Char as *mut libc::c_void);
        p_Char = tp_char;
        return 0 as libc::c_int != 0;
    }
    i = p_ptrn_lines + 1 as libc::c_int as libc::c_long;
    if *tp_char.offset(i as isize) as libc::c_int == '\n' as i32 {
        blankline = 1 as libc::c_int != 0;
        i += 1;
        i;
    }
    if p_efake >= 0 as libc::c_int as libc::c_long {
        if p_efake <= i {
            n = p_end - i + 1 as libc::c_int as libc::c_long;
        } else {
            n = -i;
        }
        p_efake += n;
        p_bfake += n;
    }
    n = 0 as libc::c_int as lin;
    while i <= p_end {
        let ref mut fresh26 = *p_line.offset(n as isize);
        *fresh26 = *tp_line.offset(i as isize);
        *p_Char.offset(n as isize) = *tp_char.offset(i as isize);
        if *p_Char.offset(n as isize) as libc::c_int == '+' as i32 {
            *p_Char.offset(n as isize) = '-' as i32 as libc::c_char;
        }
        *p_len.offset(n as isize) = *tp_len.offset(i as isize);
        i += 1;
        i;
        n += 1;
        n;
    }
    if blankline {
        i = p_ptrn_lines + 1 as libc::c_int as libc::c_long;
        let ref mut fresh27 = *p_line.offset(n as isize);
        *fresh27 = *tp_line.offset(i as isize);
        *p_Char.offset(n as isize) = *tp_char.offset(i as isize);
        *p_len.offset(n as isize) = *tp_len.offset(i as isize);
        n += 1;
        n;
    }
    if *p_Char.offset(0 as libc::c_int as isize) as libc::c_int == '=' as i32 {} else {
        __assert_fail(
            b"p_Char[0] == '='\0" as *const u8 as *const libc::c_char,
            b"pch.c\0" as *const u8 as *const libc::c_char,
            2136 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"_Bool pch_swap(void)\0"))
                .as_ptr(),
        );
    }
    'c_5933: {
        if *p_Char.offset(0 as libc::c_int as isize) as libc::c_int == '=' as i32
        {} else {
            __assert_fail(
                b"p_Char[0] == '='\0" as *const u8 as *const libc::c_char,
                b"pch.c\0" as *const u8 as *const libc::c_char,
                2136 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"_Bool pch_swap(void)\0"))
                    .as_ptr(),
            );
        }
    };
    *p_Char.offset(0 as libc::c_int as isize) = '*' as i32 as libc::c_char;
    s = *p_line.offset(0 as libc::c_int as isize);
    while *s != 0 {
        if *s as libc::c_int == '-' as i32 {
            *s = '*' as i32 as libc::c_char;
        }
        s = s.offset(1);
        s;
    }
    if *tp_char.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32 {} else {
        __assert_fail(
            b"tp_char[0] == '*'\0" as *const u8 as *const libc::c_char,
            b"pch.c\0" as *const u8 as *const libc::c_char,
            2144 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"_Bool pch_swap(void)\0"))
                .as_ptr(),
        );
    }
    'c_5852: {
        if *tp_char.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32
        {} else {
            __assert_fail(
                b"tp_char[0] == '*'\0" as *const u8 as *const libc::c_char,
                b"pch.c\0" as *const u8 as *const libc::c_char,
                2144 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"_Bool pch_swap(void)\0"))
                    .as_ptr(),
            );
        }
    };
    *tp_char.offset(0 as libc::c_int as isize) = '=' as i32 as libc::c_char;
    s = *tp_line.offset(0 as libc::c_int as isize);
    while *s != 0 {
        if *s as libc::c_int == '*' as i32 {
            *s = '-' as i32 as libc::c_char;
        }
        s = s.offset(1);
        s;
    }
    i = 0 as libc::c_int as lin;
    while n <= p_end {
        let ref mut fresh28 = *p_line.offset(n as isize);
        *fresh28 = *tp_line.offset(i as isize);
        *p_Char.offset(n as isize) = *tp_char.offset(i as isize);
        if *p_Char.offset(n as isize) as libc::c_int == '-' as i32 {
            *p_Char.offset(n as isize) = '+' as i32 as libc::c_char;
        }
        *p_len.offset(n as isize) = *tp_len.offset(i as isize);
        i += 1;
        i;
        n += 1;
        n;
    }
    if i == p_ptrn_lines + 1 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"i == p_ptrn_lines + 1\0" as *const u8 as *const libc::c_char,
            b"pch.c\0" as *const u8 as *const libc::c_char,
            2156 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"_Bool pch_swap(void)\0"))
                .as_ptr(),
        );
    }
    'c_5689: {
        if i == p_ptrn_lines + 1 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"i == p_ptrn_lines + 1\0" as *const u8 as *const libc::c_char,
                b"pch.c\0" as *const u8 as *const libc::c_char,
                2156 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"_Bool pch_swap(void)\0"))
                    .as_ptr(),
            );
        }
    };
    i = p_ptrn_lines;
    p_ptrn_lines = p_repl_lines;
    p_repl_lines = i;
    *p_Char
        .offset(
            (p_end + 1 as libc::c_int as libc::c_long) as isize,
        ) = '^' as i32 as libc::c_char;
    free(tp_line as *mut libc::c_void);
    free(tp_len as *mut libc::c_void);
    free(tp_char as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pch_says_nonexistent(mut which: bool) -> libc::c_int {
    return p_says_nonexistent[which as usize];
}
pub unsafe extern "C" fn pch_name(mut type_0: nametype) -> *const libc::c_char {
    return if type_0 as libc::c_uint == NONE as libc::c_int as libc::c_uint {
        0 as *mut libc::c_char
    } else {
        p_name[type_0 as usize]
    };
}
pub unsafe extern "C" fn pch_copy() -> bool {
    return p_copy[OLD as libc::c_int as usize] as libc::c_int != 0
        && p_copy[NEW as libc::c_int as usize] as libc::c_int != 0;
}
pub unsafe extern "C" fn pch_rename() -> bool {
    return p_rename[OLD as libc::c_int as usize] as libc::c_int != 0
        && p_rename[NEW as libc::c_int as usize] as libc::c_int != 0;
}
pub unsafe extern "C" fn pch_first() -> lin {
    return p_first;
}
pub unsafe extern "C" fn pch_ptrn_lines() -> lin {
    return p_ptrn_lines;
}
pub unsafe extern "C" fn pch_newfirst() -> lin {
    return p_newfirst;
}
pub unsafe extern "C" fn pch_repl_lines() -> lin {
    return p_repl_lines;
}
pub unsafe extern "C" fn pch_end() -> lin {
    return p_end;
}
pub unsafe extern "C" fn pch_prefix_context() -> lin {
    return p_prefix_context;
}
pub unsafe extern "C" fn pch_suffix_context() -> lin {
    return p_suffix_context;
}
pub unsafe extern "C" fn pch_line_len(mut line: lin) -> size_t {
    return *p_len.offset(line as isize);
}
pub unsafe extern "C" fn pch_char(mut line: lin) -> libc::c_char {
    return *p_Char.offset(line as isize);
}
pub unsafe extern "C" fn pfetch(mut line: lin) -> *mut libc::c_char {
    return *p_line.offset(line as isize);
}
pub unsafe extern "C" fn pch_write_line(mut line: lin, mut file: *mut FILE) -> bool {
    let mut after_newline: bool = *p_len.offset(line as isize)
        > 0 as libc::c_int as libc::c_ulong
        && *(*p_line.offset(line as isize))
            .offset(
                (*p_len.offset(line as isize))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) as libc::c_int == '\n' as i32;
    if fwrite(
        *p_line.offset(line as isize) as *const libc::c_void,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        *p_len.offset(line as isize),
        file,
    ) == 0
    {
        write_fatal();
    }
    return after_newline;
}
pub unsafe extern "C" fn pch_hunk_beg() -> lin {
    return p_hunk_beg;
}
pub unsafe extern "C" fn pch_c_function() -> *const libc::c_char {
    return p_c_function;
}
pub unsafe extern "C" fn pch_git_diff() -> bool {
    return p_git_diff;
}
pub unsafe extern "C" fn pch_timestr(mut which: bool) -> *const libc::c_char {
    return p_timestr[which as usize];
}
pub unsafe extern "C" fn pch_sha1(mut which: bool) -> *const libc::c_char {
    return p_sha1[which as usize];
}
pub unsafe extern "C" fn pch_mode(mut which: bool) -> mode_t {
    return p_mode[which as usize];
}
unsafe extern "C" fn get_ed_command_letter(
    mut line: *const libc::c_char,
) -> libc::c_char {
    let mut p: *const libc::c_char = line;
    let mut letter: libc::c_char = 0;
    let mut pair: bool = 0 as libc::c_int != 0;
    if (*p as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
        <= 9 as libc::c_int as libc::c_uint
    {
        loop {
            p = p.offset(1);
            if !((*p as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint)
            {
                break;
            }
        }
        if *p as libc::c_int == ',' as i32 {
            p = p.offset(1);
            if !((*p as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint)
            {
                return 0 as libc::c_int as libc::c_char;
            }
            loop {
                p = p.offset(1);
                if !((*p as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                    <= 9 as libc::c_int as libc::c_uint)
                {
                    break;
                }
            }
            pair = 1 as libc::c_int != 0;
        }
    }
    let fresh29 = p;
    p = p.offset(1);
    letter = *fresh29;
    match letter as libc::c_int {
        97 | 105 => {
            if pair {
                return 0 as libc::c_int as libc::c_char;
            }
        }
        99 | 100 => {}
        115 => {
            if strncmp(
                p,
                b"/.//\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) != 0 as libc::c_int
            {
                return 0 as libc::c_int as libc::c_char;
            }
            p = p.offset(4 as libc::c_int as isize);
        }
        _ => return 0 as libc::c_int as libc::c_char,
    }
    while *p as libc::c_int == ' ' as i32 || *p as libc::c_int == '\t' as i32 {
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int == '\n' as i32 {
        return letter;
    }
    return 0 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn do_ed_script(
    mut inname_0: *const libc::c_char,
    mut outname: *const libc::c_char,
    mut outname_needs_removal: *mut bool,
    mut ofp: *mut FILE,
) {
    static mut editor_program: [libc::c_char; 12] = unsafe {
        *::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"/usr/bin/ed\0")
    };
    let mut beginning_of_this_line: file_offset = 0;
    let mut pipefp: *mut FILE = 0 as *mut FILE;
    let mut chars_read: size_t = 0;
    if !dry_run && !skip_rest_of_patch {
        let mut exclusive: libc::c_int = if *outname_needs_removal as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            0o200 as libc::c_int
        };
        if inerrno == 0 {} else {
            __assert_fail(
                b"! inerrno\0" as *const u8 as *const libc::c_char,
                b"pch.c\0" as *const u8 as *const libc::c_char,
                2396 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 63],
                    &[libc::c_char; 63],
                >(b"void do_ed_script(const char *, const char *, _Bool *, FILE *)\0"))
                    .as_ptr(),
            );
        }
        'c_18031: {
            if inerrno == 0 {} else {
                __assert_fail(
                    b"! inerrno\0" as *const u8 as *const libc::c_char,
                    b"pch.c\0" as *const u8 as *const libc::c_char,
                    2396 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 63],
                        &[libc::c_char; 63],
                    >(
                        b"void do_ed_script(const char *, const char *, _Bool *, FILE *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        *outname_needs_removal = 1 as libc::c_int != 0;
        copy_file(
            inname_0,
            outname,
            0 as *mut stat,
            exclusive,
            instat.st_mode,
            1 as libc::c_int != 0,
        );
        sprintf(
            buf,
            b"%s %s%s\0" as *const u8 as *const libc::c_char,
            editor_program.as_ptr(),
            if verbosity as libc::c_uint == VERBOSE as libc::c_int as libc::c_uint {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"- \0" as *const u8 as *const libc::c_char
            },
            outname,
        );
        fflush(stdout);
        pipefp = popen(
            buf,
            if 0 as libc::c_int != 0 {
                b"wb\0" as *const u8 as *const libc::c_char
            } else {
                b"w\0" as *const u8 as *const libc::c_char
            },
        );
        if pipefp.is_null() {
            pfatal(
                b"Can't open pipe to %s\0" as *const u8 as *const libc::c_char,
                quotearg(buf),
            );
        }
    }
    loop {
        let mut ed_command_letter: libc::c_char = 0;
        beginning_of_this_line = ftell(pfp);
        chars_read = get_line();
        if chars_read == 0 {
            next_intuit_at(beginning_of_this_line, p_input_line);
            break;
        } else {
            ed_command_letter = get_ed_command_letter(buf);
            if ed_command_letter != 0 {
                if !pipefp.is_null() {
                    if fwrite(
                        buf as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        chars_read,
                        pipefp,
                    ) == 0
                    {
                        write_fatal();
                    }
                }
                if ed_command_letter as libc::c_int != 'd' as i32
                    && ed_command_letter as libc::c_int != 's' as i32
                {
                    p_pass_comments_through = 1 as libc::c_int != 0;
                    loop {
                        chars_read = get_line();
                        if !(chars_read != 0 as libc::c_int as libc::c_ulong) {
                            break;
                        }
                        if !pipefp.is_null() {
                            if fwrite(
                                buf as *const libc::c_void,
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                chars_read,
                                pipefp,
                            ) == 0
                            {
                                write_fatal();
                            }
                        }
                        if chars_read == 2 as libc::c_int as libc::c_ulong
                            && strcmp(buf, b".\n\0" as *const u8 as *const libc::c_char)
                                == 0
                        {
                            break;
                        }
                    }
                    p_pass_comments_through = 0 as libc::c_int != 0;
                }
            } else {
                next_intuit_at(beginning_of_this_line, p_input_line);
                break;
            }
        }
    }
    if pipefp.is_null() {
        return;
    }
    if fwrite(
        b"w\nq\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        4 as libc::c_int as size_t,
        pipefp,
    ) == 0 as libc::c_int as libc::c_ulong || fflush(pipefp) != 0 as libc::c_int
    {
        write_fatal();
    }
    if pclose(pipefp) != 0 as libc::c_int {
        fatal(
            b"%s FAILED\0" as *const u8 as *const libc::c_char,
            editor_program.as_ptr(),
        );
    }
    if !ofp.is_null() {
        let mut ifp: *mut FILE = fopen(
            outname,
            if 0 as libc::c_int != 0 {
                b"rb\0" as *const u8 as *const libc::c_char
            } else {
                b"r\0" as *const u8 as *const libc::c_char
            },
        );
        let mut c: libc::c_int = 0;
        if ifp.is_null() {
            pfatal(b"can't open '%s'\0" as *const u8 as *const libc::c_char, outname);
        }
        loop {
            c = getc(ifp);
            if !(c != -(1 as libc::c_int)) {
                break;
            }
            if putc(c, ofp) == -(1 as libc::c_int) {
                write_fatal();
            }
        }
        if ferror(ifp) != 0 || fclose(ifp) != 0 as libc::c_int {
            read_fatal();
        }
    }
}
pub unsafe extern "C" fn pch_normalize(mut format: diff) {
    let mut old: lin = 1 as libc::c_int as lin;
    let mut new: lin = p_ptrn_lines + 1 as libc::c_int as libc::c_long;
    while *p_Char.offset(new as isize) as libc::c_int == '=' as i32
        || *p_Char.offset(new as isize) as libc::c_int == '\n' as i32
    {
        new += 1;
        new;
    }
    if format as libc::c_uint == UNI_DIFF as libc::c_int as libc::c_uint {
        while old <= p_ptrn_lines {
            if *p_Char.offset(old as isize) as libc::c_int == '!' as i32 {
                *p_Char.offset(old as isize) = '-' as i32 as libc::c_char;
            }
            old += 1;
            old;
        }
        while new <= p_end {
            if *p_Char.offset(new as isize) as libc::c_int == '!' as i32 {
                *p_Char.offset(new as isize) = '+' as i32 as libc::c_char;
            }
            new += 1;
            new;
        }
    } else {
        while old <= p_ptrn_lines {
            if *p_Char.offset(old as isize) as libc::c_int == '-' as i32 {
                if new <= p_end
                    && *p_Char.offset(new as isize) as libc::c_int == '+' as i32
                {
                    loop {
                        *p_Char.offset(old as isize) = '!' as i32 as libc::c_char;
                        old += 1;
                        old;
                        if !(old <= p_ptrn_lines
                            && *p_Char.offset(old as isize) as libc::c_int == '-' as i32)
                        {
                            break;
                        }
                    }
                    loop {
                        *p_Char.offset(new as isize) = '!' as i32 as libc::c_char;
                        new += 1;
                        new;
                        if !(new <= p_end
                            && *p_Char.offset(new as isize) as libc::c_int == '+' as i32)
                        {
                            break;
                        }
                    }
                } else {
                    loop {
                        old += 1;
                        old;
                        if !(old <= p_ptrn_lines
                            && *p_Char.offset(old as isize) as libc::c_int == '-' as i32)
                        {
                            break;
                        }
                    }
                }
            } else if new <= p_end
                && *p_Char.offset(new as isize) as libc::c_int == '+' as i32
            {
                loop {
                    new += 1;
                    new;
                    if !(new <= p_end
                        && *p_Char.offset(new as isize) as libc::c_int == '+' as i32)
                    {
                        break;
                    }
                }
            } else {
                old += 1;
                old;
                new += 1;
                new;
            }
        }
    };
}
