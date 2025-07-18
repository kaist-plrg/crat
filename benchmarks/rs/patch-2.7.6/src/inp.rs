use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut buf: *mut libc::c_char;
    static mut using_plan_a: bool;
    static mut inerrno: libc::c_int;
    static mut invc: libc::c_int;
    static mut instat: stat;
    static mut dry_run: bool;
    static mut TMPINNAME: *const libc::c_char;
    static mut TMPINNAME_needs_removal: bool;
    static mut debug: libc::c_int;
    static mut force: bool;
    static mut batch: bool;
    static mut verbosity: C2RustUnnamed_0;
    static mut patch_get: libc::c_int;
    static mut revision: *mut libc::c_char;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn geteuid() -> __uid_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn quotearg(arg: *const libc::c_char) -> *mut libc::c_char;
    fn ask(_: *const libc::c_char, _: ...);
    fn say(_: *const libc::c_char, _: ...);
    fn pfatal(_: *const libc::c_char, _: ...) -> !;
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
    fn systemic(_: *const libc::c_char) -> libc::c_int;
    fn Fseek(_: *mut FILE, _: file_offset, _: libc::c_int);
    fn read_fatal() -> !;
    fn write_fatal() -> !;
    fn stat_file(_: *const libc::c_char, _: *mut stat) -> libc::c_int;
    fn make_tempfile(
        _: *mut *const libc::c_char,
        _: libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: mode_t,
    ) -> libc::c_int;
    fn create_file(
        _: *const libc::c_char,
        _: libc::c_int,
        _: mode_t,
        _: bool,
    ) -> libc::c_int;
    fn fatal(_: *const libc::c_char, _: ...) -> !;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn safe_open(
        pathname: *const libc::c_char,
        flags: libc::c_int,
        mode: mode_t,
    ) -> libc::c_int;
    fn safe_readlink(
        pathname: *const libc::c_char,
        buf_0: *mut libc::c_char,
        bufsiz: size_t,
    ) -> ssize_t;
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
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
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
pub type file_offset = libc::c_long;
pub static mut input_lines: lin = 0;
static mut i_buffer: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut i_ptr: *mut *const libc::c_char = 0 as *const *const libc::c_char
    as *mut *const libc::c_char;
static mut tibufsize: size_t = 0;
static mut tifd: libc::c_int = -(1 as libc::c_int);
static mut tibuf: [*mut libc::c_char; 2] = [0 as *const libc::c_char
    as *mut libc::c_char; 2];
static mut tiline: [lin; 2] = [-(1 as libc::c_int) as lin, -(1 as libc::c_int) as lin];
static mut lines_per_buf: lin = 0;
static mut tireclen: size_t = 0;
static mut last_line_size: size_t = 0;
pub unsafe extern "C" fn re_input() {
    if using_plan_a {
        if !i_buffer.is_null() {
            free(i_buffer as *mut libc::c_void);
            i_buffer = 0 as *mut libc::c_char;
            free(i_ptr as *mut libc::c_void);
        }
    } else {
        if tifd >= 0 as libc::c_int {
            close(tifd);
        }
        tifd = -(1 as libc::c_int);
        if !(tibuf[0 as libc::c_int as usize]).is_null() {
            free(tibuf[0 as libc::c_int as usize] as *mut libc::c_void);
            tibuf[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
        }
        tiline[1 as libc::c_int as usize] = -(1 as libc::c_int) as lin;
        tiline[0 as libc::c_int as usize] = tiline[1 as libc::c_int as usize];
        tireclen = 0 as libc::c_int as size_t;
    };
}
pub unsafe extern "C" fn scan_input(
    mut filename: *mut libc::c_char,
    mut file_type: mode_t,
) {
    using_plan_a = debug & 16 as libc::c_int == 0
        && plan_a(filename) as libc::c_int != 0;
    if !using_plan_a {
        if !(file_type & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint)
        {
            if file_type & 0o170000 as libc::c_int as libc::c_uint
                == 0o120000 as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"S_ISLNK (file_type)\0" as *const u8 as *const libc::c_char,
                    b"inp.c\0" as *const u8 as *const libc::c_char,
                    89 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 32],
                        &[libc::c_char; 32],
                    >(b"void scan_input(char *, mode_t)\0"))
                        .as_ptr(),
                );
            }
            'c_7058: {
                if file_type & 0o170000 as libc::c_int as libc::c_uint
                    == 0o120000 as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"S_ISLNK (file_type)\0" as *const u8 as *const libc::c_char,
                        b"inp.c\0" as *const u8 as *const libc::c_char,
                        89 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 32],
                            &[libc::c_char; 32],
                        >(b"void scan_input(char *, mode_t)\0"))
                            .as_ptr(),
                    );
                }
            };
            fatal(
                b"Can't handle %s %s\0" as *const u8 as *const libc::c_char,
                b"symbolic link\0" as *const u8 as *const libc::c_char,
                quotearg(filename),
            );
        }
        plan_b(filename);
    }
}
unsafe extern "C" fn report_revision(mut found_revision: bool) {
    let mut rev: *const libc::c_char = quotearg(revision);
    if found_revision {
        if verbosity as libc::c_uint == VERBOSE as libc::c_int as libc::c_uint {
            say(
                b"Good.  This file appears to be the %s version.\n\0" as *const u8
                    as *const libc::c_char,
                rev,
            );
        }
    } else if force {
        if verbosity as libc::c_uint != SILENT as libc::c_int as libc::c_uint {
            say(
                b"Warning: this file doesn't appear to be the %s version -- patching anyway.\n\0"
                    as *const u8 as *const libc::c_char,
                rev,
            );
        }
    } else if batch {
        fatal(
            b"This file doesn't appear to be the %s version -- aborting.\0" as *const u8
                as *const libc::c_char,
            rev,
        );
    } else {
        ask(
            b"This file doesn't appear to be the %s version -- patch anyway? [n] \0"
                as *const u8 as *const libc::c_char,
            rev,
        );
        if *buf as libc::c_int != 'y' as i32 {
            fatal(b"aborted\0" as *const u8 as *const libc::c_char);
        }
    };
}
unsafe extern "C" fn too_many_lines(mut filename: *const libc::c_char) -> ! {
    fatal(
        b"File %s has too many lines\0" as *const u8 as *const libc::c_char,
        quotearg(filename),
    );
}
unsafe extern "C" fn lines_too_long(mut filename: *const libc::c_char) -> ! {
    fatal(
        b"Lines in file %s are too long\0" as *const u8 as *const libc::c_char,
        quotearg(filename),
    );
}
pub unsafe extern "C" fn get_input_file(
    mut filename: *const libc::c_char,
    mut outname: *const libc::c_char,
    mut file_type: mode_t,
) -> bool {
    let mut elsewhere: bool = strcmp(filename, outname) != 0 as libc::c_int;
    let mut cs: *const libc::c_char = 0 as *const libc::c_char;
    let mut diffbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut getbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    if inerrno == -(1 as libc::c_int) {
        inerrno = stat_file(filename, &mut instat);
    }
    if file_type & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint && patch_get != 0
        && invc != 0 as libc::c_int
        && (inerrno != 0
            || !elsewhere
                && (instat.st_mode
                    & (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                        | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                        as libc::c_uint == 0 as libc::c_int as libc::c_uint
                    || instat.st_mode
                        & (0o200 as libc::c_int >> 3 as libc::c_int
                            | 0o200 as libc::c_int >> 3 as libc::c_int
                                >> 3 as libc::c_int) as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                        && instat.st_uid != geteuid()))
        && {
            cs = version_controller(
                filename,
                elsewhere,
                if inerrno != 0 { 0 as *mut stat } else { &mut instat },
                &mut getbuf,
                &mut diffbuf,
            );
            invc = !cs.is_null() as libc::c_int;
            invc != 0
        }
    {
        if inerrno == 0 {
            if !elsewhere
                && instat.st_mode
                    & (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                        | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                        as libc::c_uint != 0 as libc::c_int as libc::c_uint
            {
                fatal(
                    b"File %s seems to be locked by somebody else under %s\0"
                        as *const u8 as *const libc::c_char,
                    quotearg(filename),
                    cs,
                );
            }
            if !diffbuf.is_null() {
                if verbosity as libc::c_uint == VERBOSE as libc::c_int as libc::c_uint {
                    say(
                        b"Comparing file %s to default %s version...\n\0" as *const u8
                            as *const libc::c_char,
                        quotearg(filename),
                        cs,
                    );
                }
                if systemic(diffbuf) != 0 as libc::c_int {
                    say(
                        b"warning: Patching file %s, which does not match default %s version\n\0"
                            as *const u8 as *const libc::c_char,
                        quotearg(filename),
                        cs,
                    );
                    cs = 0 as *const libc::c_char;
                }
            }
            if dry_run {
                cs = 0 as *const libc::c_char;
            }
        }
        if !cs.is_null()
            && version_get(filename, cs, inerrno == 0, elsewhere, getbuf, &mut instat)
                as libc::c_int != 0
        {
            inerrno = 0 as libc::c_int;
        }
        free(getbuf as *mut libc::c_void);
        free(diffbuf as *mut libc::c_void);
    }
    if inerrno != 0 {
        instat
            .st_mode = (0o400 as libc::c_int | 0o200 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as __mode_t;
        instat.st_size = 0 as libc::c_int as __off_t;
    } else if !((file_type & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
        || file_type & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint)
        && file_type & 0o170000 as libc::c_int as libc::c_uint
            == instat.st_mode & 0o170000 as libc::c_int as libc::c_uint)
    {
        say(
            b"File %s is not a %s -- refusing to patch\n\0" as *const u8
                as *const libc::c_char,
            quotearg(filename),
            if file_type & 0o170000 as libc::c_int as libc::c_uint
                == 0o120000 as libc::c_int as libc::c_uint
            {
                b"symbolic link\0" as *const u8 as *const libc::c_char
            } else {
                b"regular file\0" as *const u8 as *const libc::c_char
            },
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn plan_a(mut filename: *const libc::c_char) -> bool {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut lim: *const libc::c_char = 0 as *const libc::c_char;
    let mut ptr: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut iline: lin = 0;
    let mut size: size_t = instat.st_size as size_t;
    if !(size == instat.st_size as libc::c_ulong
        && {
            buffer = malloc(if size != 0 { size } else { 1 as libc::c_int as size_t })
                as *mut libc::c_char;
            !buffer.is_null()
        })
    {
        return 0 as libc::c_int != 0;
    }
    if size != 0 {
        if instat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint
        {
            let mut ifd: libc::c_int = safe_open(
                filename,
                0 as libc::c_int | 0 as libc::c_int,
                0 as libc::c_int as mode_t,
            );
            let mut buffered: size_t = 0 as libc::c_int as size_t;
            let mut n: size_t = 0;
            if ifd < 0 as libc::c_int {
                pfatal(
                    b"can't open file %s\0" as *const u8 as *const libc::c_char,
                    quotearg(filename),
                );
            }
            while size.wrapping_sub(buffered) != 0 as libc::c_int as libc::c_ulong {
                n = read(
                    ifd,
                    buffer.offset(buffered as isize) as *mut libc::c_void,
                    size.wrapping_sub(buffered),
                ) as size_t;
                if n == 0 as libc::c_int as libc::c_ulong {
                    size = buffered;
                    break;
                } else {
                    if n == -(1 as libc::c_int) as size_t {
                        close(ifd);
                        free(buffer as *mut libc::c_void);
                        return 0 as libc::c_int != 0;
                    }
                    buffered = (buffered as libc::c_ulong).wrapping_add(n) as size_t
                        as size_t;
                }
            }
            if close(ifd) != 0 as libc::c_int {
                read_fatal();
            }
        } else if instat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
        {
            let mut n_0: ssize_t = 0;
            n_0 = safe_readlink(filename, buffer, size);
            if n_0 < 0 as libc::c_int as libc::c_long {
                pfatal(
                    b"can't read %s %s\0" as *const u8 as *const libc::c_char,
                    b"symbolic link\0" as *const u8 as *const libc::c_char,
                    quotearg(filename),
                );
            }
            size = n_0 as size_t;
        } else {
            free(buffer as *mut libc::c_void);
            return 0 as libc::c_int != 0;
        }
    }
    lim = buffer.offset(size as isize);
    iline = 3 as libc::c_int as lin;
    s = buffer;
    loop {
        s = memchr(
            s as *const libc::c_void,
            '\n' as i32,
            lim.offset_from(s) as libc::c_long as libc::c_ulong,
        ) as *mut libc::c_char;
        if s.is_null() {
            break;
        }
        iline += 1;
        if iline < 0 as libc::c_int as libc::c_long {
            too_many_lines(filename);
        }
        s = s.offset(1);
        s;
    }
    if !(iline as libc::c_ulong == iline as size_t
        && (iline as size_t)
            .wrapping_mul(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            == iline as size_t
        && {
            ptr = malloc(
                (iline as size_t)
                    .wrapping_mul(
                        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *const libc::c_char;
            !ptr.is_null()
        })
    {
        free(buffer as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    iline = 0 as libc::c_int as lin;
    s = buffer;
    loop {
        iline += 1;
        let ref mut fresh0 = *ptr.offset(iline as isize);
        *fresh0 = s;
        s = memchr(
            s as *const libc::c_void,
            '\n' as i32,
            lim.offset_from(s) as libc::c_long as libc::c_ulong,
        ) as *mut libc::c_char;
        if s.is_null() {
            break;
        }
        s = s.offset(1);
        s;
    }
    if size != 0
        && *lim.offset(-(1 as libc::c_int) as isize) as libc::c_int != '\n' as i32
    {
        iline += 1;
        let ref mut fresh1 = *ptr.offset(iline as isize);
        *fresh1 = lim;
    }
    input_lines = iline - 1 as libc::c_int as libc::c_long;
    if !revision.is_null() {
        let mut rev: *const libc::c_char = revision;
        let mut rev0: libc::c_int = *rev.offset(0 as libc::c_int as isize)
            as libc::c_int;
        let mut found_revision: bool = 0 as libc::c_int != 0;
        let mut revlen: size_t = strlen(rev);
        if revlen <= size {
            let mut limrev: *const libc::c_char = lim.offset(-(revlen as isize));
            s = buffer;
            loop {
                s = memchr(
                    s as *const libc::c_void,
                    rev0,
                    limrev.offset_from(s) as libc::c_long as libc::c_ulong,
                ) as *mut libc::c_char;
                if s.is_null() {
                    break;
                }
                if memcmp(s as *const libc::c_void, rev as *const libc::c_void, revlen)
                    == 0 as libc::c_int
                    && (s == buffer as *const libc::c_char
                        || 1 as libc::c_int != 0
                            && *(*__ctype_b_loc())
                                .offset(
                                    *s.offset(-(1 as libc::c_int) as isize) as libc::c_uchar
                                        as libc::c_int as isize,
                                ) as libc::c_int
                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                != 0)
                    && (s.offset(1 as libc::c_int as isize) == limrev
                        || 1 as libc::c_int != 0
                            && *(*__ctype_b_loc())
                                .offset(
                                    *s.offset(revlen as isize) as libc::c_uchar as libc::c_int
                                        as isize,
                                ) as libc::c_int
                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                != 0)
                {
                    found_revision = 1 as libc::c_int != 0;
                    break;
                } else {
                    s = s.offset(1);
                    s;
                }
            }
        }
        report_revision(found_revision);
    }
    i_buffer = buffer;
    i_ptr = ptr;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn plan_b(mut filename: *const libc::c_char) {
    let mut ifd: libc::c_int = 0;
    let mut ifp: *mut FILE = 0 as *mut FILE;
    let mut c: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut maxlen: size_t = 0;
    let mut found_revision: bool = false;
    let mut i: size_t = 0;
    let mut rev: *const libc::c_char = 0 as *const libc::c_char;
    let mut revlen: size_t = 0;
    let mut line: lin = 1 as libc::c_int as lin;
    if instat.st_size == 0 as libc::c_int as libc::c_long {
        filename = b"/dev/null\0" as *const u8 as *const libc::c_char;
    }
    ifd = safe_open(
        filename,
        0 as libc::c_int | 0 as libc::c_int,
        0 as libc::c_int as mode_t,
    );
    if ifd < 0 as libc::c_int
        || {
            ifp = fdopen(
                ifd,
                if 0 as libc::c_int != 0 {
                    b"rb\0" as *const u8 as *const libc::c_char
                } else {
                    b"r\0" as *const u8 as *const libc::c_char
                },
            );
            ifp.is_null()
        }
    {
        pfatal(
            b"Can't open file %s\0" as *const u8 as *const libc::c_char,
            quotearg(filename),
        );
    }
    if TMPINNAME_needs_removal {
        tifd = create_file(
            TMPINNAME,
            0o2 as libc::c_int | 0 as libc::c_int,
            0 as libc::c_int as mode_t,
            1 as libc::c_int != 0,
        );
    } else {
        tifd = make_tempfile(
            &mut TMPINNAME,
            'i' as i32 as libc::c_char,
            0 as *const libc::c_char,
            0o2 as libc::c_int | 0 as libc::c_int,
            (0o400 as libc::c_int | 0o200 as libc::c_int) as mode_t,
        );
        if tifd == -(1 as libc::c_int) {
            pfatal(
                b"Can't create temporary file %s\0" as *const u8 as *const libc::c_char,
                TMPINNAME,
            );
        }
        TMPINNAME_needs_removal = 1 as libc::c_int != 0;
    }
    i = 0 as libc::c_int as size_t;
    len = 0 as libc::c_int as size_t;
    maxlen = 1 as libc::c_int as size_t;
    rev = revision;
    found_revision = rev.is_null();
    revlen = if !rev.is_null() {
        strlen(rev)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    loop {
        c = getc(ifp);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        len = len.wrapping_add(1);
        if len
            > (-(1 as libc::c_int) as size_t)
                .wrapping_div(2 as libc::c_int as libc::c_ulong)
        {
            lines_too_long(filename);
        }
        if c == '\n' as i32 {
            line += 1;
            if line < 0 as libc::c_int as libc::c_long {
                too_many_lines(filename);
            }
            if maxlen < len {
                maxlen = len;
            }
            len = 0 as libc::c_int as size_t;
        }
        if !found_revision {
            if i == revlen {
                found_revision = 1 as libc::c_int != 0
                    && *(*__ctype_b_loc())
                        .offset(c as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0;
                i = -(1 as libc::c_int) as size_t;
            } else if i != -(1 as libc::c_int) as size_t {
                i = if *rev.offset(i as isize) as libc::c_int == c {
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                } else {
                    -(1 as libc::c_int) as size_t
                };
            }
            if i == -(1 as libc::c_int) as size_t
                && (1 as libc::c_int != 0
                    && *(*__ctype_b_loc())
                        .offset(c as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0)
            {
                i = 0 as libc::c_int as size_t;
            }
        }
    }
    if !revision.is_null() {
        report_revision(found_revision);
    }
    Fseek(ifp, 0 as libc::c_int as file_offset, 0 as libc::c_int);
    tibufsize = (8 as libc::c_int * 1024 as libc::c_int) as size_t;
    while tibufsize < maxlen {
        tibufsize <<= 1 as libc::c_int;
    }
    lines_per_buf = tibufsize.wrapping_div(maxlen) as lin;
    tireclen = maxlen;
    tibuf[0 as libc::c_int
        as usize] = xmalloc((2 as libc::c_int as libc::c_ulong).wrapping_mul(tibufsize))
        as *mut libc::c_char;
    tibuf[1 as libc::c_int
        as usize] = (tibuf[0 as libc::c_int as usize]).offset(tibufsize as isize);
    line = 1 as libc::c_int as lin;
    's_214: loop {
        let mut p: *mut libc::c_char = (tibuf[0 as libc::c_int as usize])
            .offset(
                maxlen.wrapping_mul((line % lines_per_buf) as libc::c_ulong) as isize,
            );
        let mut p0: *const libc::c_char = p;
        if line % lines_per_buf == 0 {
            if write(
                tifd,
                tibuf[0 as libc::c_int as usize] as *const libc::c_void,
                tibufsize,
            ) as libc::c_ulong != tibufsize
            {
                write_fatal();
            }
        }
        c = getc(ifp);
        if c == -(1 as libc::c_int) {
            break;
        }
        loop {
            let fresh2 = p;
            p = p.offset(1);
            *fresh2 = c as libc::c_char;
            if c == '\n' as i32 {
                last_line_size = p.offset_from(p0) as libc::c_long as size_t;
                break;
            } else {
                c = getc(ifp);
                if !(c == -(1 as libc::c_int)) {
                    continue;
                }
                last_line_size = p.offset_from(p0) as libc::c_long as size_t;
                line += 1;
                line;
                break 's_214;
            }
        }
        line += 1;
        line;
    }
    if ferror(ifp) != 0 || fclose(ifp) != 0 as libc::c_int {
        read_fatal();
    }
    if line % lines_per_buf != 0 as libc::c_int as libc::c_long {
        if write(
            tifd,
            tibuf[0 as libc::c_int as usize] as *const libc::c_void,
            tibufsize,
        ) as libc::c_ulong != tibufsize
        {
            write_fatal();
        }
    }
    input_lines = line - 1 as libc::c_int as libc::c_long;
}
pub unsafe extern "C" fn ifetch(
    mut line: lin,
    mut whichbuf: bool,
    mut psize: *mut size_t,
) -> *const libc::c_char {
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if line < 1 as libc::c_int as libc::c_long || line > input_lines {
        *psize = 0 as libc::c_int as size_t;
        return b"\0" as *const u8 as *const libc::c_char;
    }
    if using_plan_a {
        p = *i_ptr.offset(line as isize);
        *psize = (*i_ptr.offset((line + 1 as libc::c_int as libc::c_long) as isize))
            .offset_from(p) as libc::c_long as size_t;
        return p;
    } else {
        let mut offline: lin = line % lines_per_buf;
        let mut baseline: lin = line - offline;
        if tiline[0 as libc::c_int as usize] == baseline {
            whichbuf = 0 as libc::c_int != 0;
        } else if tiline[1 as libc::c_int as usize] == baseline {
            whichbuf = 1 as libc::c_int != 0;
        } else {
            tiline[whichbuf as usize] = baseline;
            if lseek(
                tifd,
                ((baseline / lines_per_buf) as libc::c_ulong).wrapping_mul(tibufsize)
                    as __off_t,
                0 as libc::c_int,
            ) == -(1 as libc::c_int) as libc::c_long
                || read(tifd, tibuf[whichbuf as usize] as *mut libc::c_void, tibufsize)
                    < 0 as libc::c_int as libc::c_long
            {
                read_fatal();
            }
        }
        p = (tibuf[whichbuf as usize])
            .offset(tireclen.wrapping_mul(offline as libc::c_ulong) as isize);
        if line == input_lines {
            *psize = last_line_size;
        } else {
            q = p;
            loop {
                let fresh3 = q;
                q = q.offset(1);
                if !(*fresh3 as libc::c_int != '\n' as i32) {
                    break;
                }
            }
            *psize = q.offset_from(p) as libc::c_long as size_t;
        }
        return p;
    };
}
