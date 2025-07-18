use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ansi_state;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn realpath(
        __name: *const libc::c_char,
        __resolved: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn save(s: *const libc::c_char) -> *mut libc::c_char;
    fn ecalloc(count: libc::c_int, size: libc::c_uint) -> *mut libc::c_void;
    fn ch_ungetchar(c: libc::c_int);
    fn ch_tell() -> POSITION;
    fn seekable(f: libc::c_int) -> libc::c_int;
    fn binary_char(c: LWCHAR) -> libc::c_int;
    fn is_utf8_well_formed(ss: *mut libc::c_char, slen: libc::c_int) -> libc::c_int;
    fn utf_skip_to_lead(pp: *mut *mut libc::c_char, limit: *mut libc::c_char);
    fn step_char(
        pp: *mut *mut libc::c_char,
        dir: libc::c_int,
        limit: *const libc::c_char,
    ) -> LWCHAR;
    fn lgetenv(var: *mut libc::c_char) -> *mut libc::c_char;
    fn isnullenv(s: *mut libc::c_char) -> libc::c_int;
    fn get_filename(ifile: *mut libc::c_void) -> *mut libc::c_char;
    fn ansi_done(pansi: *mut ansi_state);
    fn skip_ansi(
        pansi: *mut ansi_state,
        pp: *mut *mut libc::c_char,
        limit: *const libc::c_char,
    );
    fn ansi_start(ch: LWCHAR) -> *mut ansi_state;
    fn error(fmt: *mut libc::c_char, parg: *mut PARG);
    fn errno_message(filename: *mut libc::c_char) -> *mut libc::c_char;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
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
    static mut force_open: libc::c_int;
    static mut secure: libc::c_int;
    static mut use_lessopen: libc::c_int;
    static mut ctldisp: libc::c_int;
    static mut utf_mode: libc::c_int;
    static mut curr_ifile: *mut libc::c_void;
    static mut old_ifile: *mut libc::c_void;
    static mut openquote: libc::c_char;
    static mut closequote: libc::c_char;
    static mut curr_ino: ino_t;
    static mut curr_dev: dev_t;
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
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub type LWCHAR = libc::c_ulong;
pub type POSITION = off_t;
pub type LINENUM = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union parg {
    pub p_string: *mut libc::c_char,
    pub p_int: libc::c_int,
    pub p_linenum: LINENUM,
    pub p_char: libc::c_char,
}
pub type PARG = parg;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
pub unsafe extern "C" fn shell_unquote(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = ecalloc(
        (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
    ) as *mut libc::c_char;
    name = p;
    if *str as libc::c_int == openquote as libc::c_int {
        str = str.offset(1);
        str;
        while *str as libc::c_int != '\0' as i32 {
            if *str as libc::c_int == closequote as libc::c_int {
                if *str.offset(1 as libc::c_int as isize) as libc::c_int
                    != closequote as libc::c_int
                {
                    break;
                }
                str = str.offset(1);
                str;
            }
            let fresh0 = str;
            str = str.offset(1);
            let fresh1 = p;
            p = p.offset(1);
            *fresh1 = *fresh0;
        }
    } else {
        let mut esc: *mut libc::c_char = get_meta_escape();
        let mut esclen: libc::c_int = strlen(esc) as libc::c_int;
        while *str as libc::c_int != '\0' as i32 {
            if esclen > 0 as libc::c_int
                && strncmp(str, esc, esclen as libc::c_ulong) == 0 as libc::c_int
            {
                str = str.offset(esclen as isize);
            }
            let fresh2 = str;
            str = str.offset(1);
            let fresh3 = p;
            p = p.offset(1);
            *fresh3 = *fresh2;
        }
    }
    *p = '\0' as i32 as libc::c_char;
    return name;
}
pub unsafe extern "C" fn get_meta_escape() -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = lgetenv(
        b"LESSMETAESCAPE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if s.is_null() {
        s = b"\\\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    return s;
}
unsafe extern "C" fn metachars() -> *mut libc::c_char {
    static mut mchars: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    if mchars.is_null() {
        mchars = lgetenv(
            b"LESSMETACHARS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if mchars.is_null() {
            mchars = b"; *?\t\n'\"()<>[]|&^`#\\$%=~{},\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
        }
    }
    return mchars;
}
unsafe extern "C" fn metachar(mut c: libc::c_char) -> libc::c_int {
    return (strchr(metachars(), c as libc::c_int)
        != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
}
pub unsafe extern "C" fn shell_quote(mut s: *mut libc::c_char) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut esc: *mut libc::c_char = get_meta_escape();
    let mut esclen: libc::c_int = strlen(esc) as libc::c_int;
    let mut use_quotes: libc::c_int = 0 as libc::c_int;
    let mut have_quotes: libc::c_int = 0 as libc::c_int;
    len = 1 as libc::c_int;
    p = s;
    while *p as libc::c_int != '\0' as i32 {
        len += 1;
        len;
        if *p as libc::c_int == openquote as libc::c_int
            || *p as libc::c_int == closequote as libc::c_int
        {
            have_quotes = 1 as libc::c_int;
        }
        if metachar(*p) != 0 {
            if esclen == 0 as libc::c_int {
                use_quotes = 1 as libc::c_int;
            } else {
                len += esclen;
            }
        }
        p = p.offset(1);
        p;
    }
    if use_quotes != 0 {
        if have_quotes != 0 {
            return 0 as *mut libc::c_char;
        }
        len = strlen(s) as libc::c_int + 3 as libc::c_int;
    }
    p = ecalloc(
        len,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
    ) as *mut libc::c_char;
    newstr = p;
    if use_quotes != 0 {
        snprintf(
            newstr,
            len as libc::c_ulong,
            b"%c%s%c\0" as *const u8 as *const libc::c_char,
            openquote as libc::c_int,
            s,
            closequote as libc::c_int,
        );
    } else {
        while *s as libc::c_int != '\0' as i32 {
            if metachar(*s) != 0 {
                strcpy(p, esc);
                p = p.offset(esclen as isize);
            }
            let fresh4 = s;
            s = s.offset(1);
            let fresh5 = p;
            p = p.offset(1);
            *fresh5 = *fresh4;
        }
        *p = '\0' as i32 as libc::c_char;
    }
    return newstr;
}
pub unsafe extern "C" fn dirfile(
    mut dirname: *mut libc::c_char,
    mut filename: *mut libc::c_char,
    mut must_exist: libc::c_int,
) -> *mut libc::c_char {
    let mut pathname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    if dirname.is_null() || *dirname as libc::c_int == '\0' as i32 {
        return 0 as *mut libc::c_char;
    }
    len = (strlen(dirname))
        .wrapping_add(strlen(filename))
        .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    pathname = calloc(
        len as libc::c_ulong,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
    ) as *mut libc::c_char;
    if pathname.is_null() {
        return 0 as *mut libc::c_char;
    }
    snprintf(
        pathname,
        len as libc::c_ulong,
        b"%s%s%s\0" as *const u8 as *const libc::c_char,
        dirname,
        b"/\0" as *const u8 as *const libc::c_char,
        filename,
    );
    if must_exist != 0 {
        f = open(pathname, 0 as libc::c_int);
        if f < 0 as libc::c_int {
            free(pathname as *mut libc::c_void);
            pathname = 0 as *mut libc::c_char;
        } else {
            close(f);
        }
    }
    return pathname;
}
pub unsafe extern "C" fn homefile(mut filename: *mut libc::c_char) -> *mut libc::c_char {
    let mut pathname: *mut libc::c_char = 0 as *mut libc::c_char;
    pathname = dirfile(
        lgetenv(b"HOME\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        filename,
        1 as libc::c_int,
    );
    if !pathname.is_null() {
        return pathname;
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn fexpand(mut s: *mut libc::c_char) -> *mut libc::c_char {
    let mut fr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut to: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ifile: *mut libc::c_void = 0 as *mut libc::c_void;
    n = 0 as libc::c_int;
    fr = s;
    while *fr as libc::c_int != '\0' as i32 {
        match *fr as libc::c_int {
            37 | 35 => {
                if fr > s
                    && *fr.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == *fr as libc::c_int
                {
                    n += 1;
                    n;
                } else if *fr.offset(1 as libc::c_int as isize) as libc::c_int
                    != *fr as libc::c_int
                {
                    ifile = if *fr as libc::c_int == '%' as i32 {
                        curr_ifile
                    } else if *fr as libc::c_int == '#' as i32 {
                        old_ifile
                    } else {
                        0 as *mut libc::c_void
                    };
                    if ifile == 0 as *mut libc::c_void {
                        n += 1;
                        n;
                    } else {
                        n += strlen(get_filename(ifile)) as libc::c_int;
                    }
                }
            }
            _ => {
                n += 1;
                n;
            }
        }
        fr = fr.offset(1);
        fr;
    }
    e = ecalloc(
        n + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
    ) as *mut libc::c_char;
    to = e;
    fr = s;
    while *fr as libc::c_int != '\0' as i32 {
        match *fr as libc::c_int {
            37 | 35 => {
                if fr > s
                    && *fr.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == *fr as libc::c_int
                {
                    let fresh6 = to;
                    to = to.offset(1);
                    *fresh6 = *fr;
                } else if *fr.offset(1 as libc::c_int as isize) as libc::c_int
                    != *fr as libc::c_int
                {
                    ifile = if *fr as libc::c_int == '%' as i32 {
                        curr_ifile
                    } else if *fr as libc::c_int == '#' as i32 {
                        old_ifile
                    } else {
                        0 as *mut libc::c_void
                    };
                    if ifile == 0 as *mut libc::c_void {
                        let fresh7 = to;
                        to = to.offset(1);
                        *fresh7 = *fr;
                    } else {
                        strcpy(to, get_filename(ifile));
                        to = to.offset(strlen(to) as isize);
                    }
                }
            }
            _ => {
                let fresh8 = to;
                to = to.offset(1);
                *fresh8 = *fr;
            }
        }
        fr = fr.offset(1);
        fr;
    }
    *to = '\0' as i32 as libc::c_char;
    return e;
}
pub unsafe extern "C" fn fcomplete(mut s: *mut libc::c_char) -> *mut libc::c_char {
    let mut fpat: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut qs: *mut libc::c_char = 0 as *mut libc::c_char;
    if secure != 0 {
        return 0 as *mut libc::c_char;
    }
    let mut len: libc::c_int = strlen(s) as libc::c_int + 2 as libc::c_int;
    fpat = ecalloc(
        len,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
    ) as *mut libc::c_char;
    snprintf(
        fpat,
        len as libc::c_ulong,
        b"%s*\0" as *const u8 as *const libc::c_char,
        s,
    );
    qs = lglob(fpat);
    s = shell_unquote(qs);
    if strcmp(s, fpat) == 0 as libc::c_int {
        free(qs as *mut libc::c_void);
        qs = 0 as *mut libc::c_char;
    }
    free(s as *mut libc::c_void);
    free(fpat as *mut libc::c_void);
    return qs;
}
pub unsafe extern "C" fn bin_file(mut f: libc::c_int) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut bin_count: libc::c_int = 0 as libc::c_int;
    let mut data: [libc::c_char; 256] = [0; 256];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut edata: *mut libc::c_char = 0 as *mut libc::c_char;
    if seekable(f) == 0 {
        return 0 as libc::c_int;
    }
    if lseek(f, 0 as libc::c_int as off_t, 0 as libc::c_int)
        == -(1 as libc::c_int) as off_t
    {
        return 0 as libc::c_int;
    }
    n = read(
        f,
        data.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    ) as libc::c_int;
    if n <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    edata = &mut *data.as_mut_ptr().offset(n as isize) as *mut libc::c_char;
    p = data.as_mut_ptr();
    while p < edata {
        if utf_mode != 0
            && is_utf8_well_formed(
                p,
                edata.offset_from(p) as libc::c_long as libc::c_int,
            ) == 0
        {
            bin_count += 1;
            bin_count;
            utf_skip_to_lead(&mut p, edata);
        } else {
            let mut c: LWCHAR = step_char(&mut p, 1 as libc::c_int, edata);
            let mut pansi: *mut ansi_state = 0 as *mut ansi_state;
            if ctldisp == 2 as libc::c_int
                && {
                    pansi = ansi_start(c);
                    !pansi.is_null()
                }
            {
                skip_ansi(pansi, &mut p, edata);
                ansi_done(pansi);
            } else if binary_char(c) != 0 {
                bin_count += 1;
                bin_count;
            }
        }
    }
    return (bin_count > 5 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn seek_filesize(mut f: libc::c_int) -> POSITION {
    let mut spos: off_t = 0;
    spos = lseek(f, 0 as libc::c_int as off_t, 2 as libc::c_int);
    if spos == -(1 as libc::c_int) as off_t {
        return -(1 as libc::c_int) as POSITION;
    }
    return spos;
}
unsafe extern "C" fn readfd(mut fd: *mut FILE) -> *mut libc::c_char {
    let mut len: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    len = 100 as libc::c_int;
    buf = ecalloc(
        len,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
    ) as *mut libc::c_char;
    p = buf;
    loop {
        ch = getc(fd);
        if ch == '\n' as i32 || ch == -(1 as libc::c_int) {
            break;
        }
        if p.offset_from(buf) as libc::c_long >= (len - 1 as libc::c_int) as libc::c_long
        {
            len *= 2 as libc::c_int;
            *p = '\0' as i32 as libc::c_char;
            p = ecalloc(
                len,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
            ) as *mut libc::c_char;
            strcpy(p, buf);
            free(buf as *mut libc::c_void);
            buf = p;
            p = buf.offset(strlen(buf) as isize);
        }
        *p = ch as libc::c_char;
        p = p.offset(1);
        p;
    }
    *p = '\0' as i32 as libc::c_char;
    return buf;
}
unsafe extern "C" fn shellcmd(mut cmd: *mut libc::c_char) -> *mut FILE {
    let mut fd: *mut FILE = 0 as *mut FILE;
    let mut shell: *mut libc::c_char = 0 as *mut libc::c_char;
    shell = lgetenv(b"SHELL\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if isnullenv(shell) == 0 {
        let mut scmd: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut esccmd: *mut libc::c_char = 0 as *mut libc::c_char;
        esccmd = shell_quote(cmd);
        if esccmd.is_null() {
            fd = popen(cmd, b"r\0" as *const u8 as *const libc::c_char);
        } else {
            let mut len: libc::c_int = (strlen(shell))
                .wrapping_add(strlen(esccmd))
                .wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int;
            scmd = ecalloc(
                len,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
            ) as *mut libc::c_char;
            snprintf(
                scmd,
                len as libc::c_ulong,
                b"%s %s %s\0" as *const u8 as *const libc::c_char,
                shell,
                shell_coption(),
                esccmd,
            );
            free(esccmd as *mut libc::c_void);
            fd = popen(scmd, b"r\0" as *const u8 as *const libc::c_char);
            free(scmd as *mut libc::c_void);
        }
    } else {
        fd = popen(cmd, b"r\0" as *const u8 as *const libc::c_char);
    }
    return fd;
}
pub unsafe extern "C" fn lglob(mut filename: *mut libc::c_char) -> *mut libc::c_char {
    let mut gfilename: *mut libc::c_char = 0 as *mut libc::c_char;
    filename = fexpand(filename);
    if secure != 0 {
        return filename;
    }
    let mut fd: *mut FILE = 0 as *mut FILE;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lessecho: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut esc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    esc = get_meta_escape();
    if strlen(esc) == 0 as libc::c_int as libc::c_ulong {
        esc = b"-\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    esc = shell_quote(esc);
    if esc.is_null() {
        return filename;
    }
    lessecho = lgetenv(
        b"LESSECHO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if isnullenv(lessecho) != 0 {
        lessecho = b"lessecho\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    len = (strlen(lessecho))
        .wrapping_add(strlen(filename))
        .wrapping_add(
            (7 as libc::c_int as libc::c_ulong).wrapping_mul(strlen(metachars())),
        )
        .wrapping_add(24 as libc::c_int as libc::c_ulong) as libc::c_int;
    cmd = ecalloc(
        len,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
    ) as *mut libc::c_char;
    snprintf(
        cmd,
        len as libc::c_ulong,
        b"%s -p0x%x -d0x%x -e%s \0" as *const u8 as *const libc::c_char,
        lessecho,
        openquote as libc::c_uchar as libc::c_int,
        closequote as libc::c_uchar as libc::c_int,
        esc,
    );
    free(esc as *mut libc::c_void);
    s = metachars();
    while *s as libc::c_int != '\0' as i32 {
        sprintf(
            cmd.offset(strlen(cmd) as isize),
            b"-n0x%x \0" as *const u8 as *const libc::c_char,
            *s as libc::c_uchar as libc::c_int,
        );
        s = s.offset(1);
        s;
    }
    sprintf(
        cmd.offset(strlen(cmd) as isize),
        b"-- %s\0" as *const u8 as *const libc::c_char,
        filename,
    );
    fd = shellcmd(cmd);
    free(cmd as *mut libc::c_void);
    if fd.is_null() {
        return filename;
    }
    gfilename = readfd(fd);
    pclose(fd);
    if *gfilename as libc::c_int == '\0' as i32 {
        free(gfilename as *mut libc::c_void);
        return filename;
    }
    free(filename as *mut libc::c_void);
    return gfilename;
}
pub unsafe extern "C" fn is_fake_pathname(mut path: *mut libc::c_char) -> libc::c_int {
    return (strcmp(path, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(
            path,
            b"@/\\less/\\help/\\file/\\@\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        || strcmp(
            path,
            b"@/\\less/\\empty/\\file/\\@\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn lrealpath(mut path: *mut libc::c_char) -> *mut libc::c_char {
    if is_fake_pathname(path) == 0 {
        let mut rpath: [libc::c_char; 4096] = [0; 4096];
        if !(realpath(path, rpath.as_mut_ptr())).is_null() {
            return save(rpath.as_mut_ptr());
        }
    }
    return save(path);
}
unsafe extern "C" fn num_pct_s(mut lessopen: *mut libc::c_char) -> libc::c_int {
    let mut num: libc::c_int = 0 as libc::c_int;
    while *lessopen as libc::c_int != '\0' as i32 {
        if *lessopen as libc::c_int == '%' as i32 {
            if *lessopen.offset(1 as libc::c_int as isize) as libc::c_int == '%' as i32 {
                lessopen = lessopen.offset(1);
                lessopen;
            } else if *lessopen.offset(1 as libc::c_int as isize) as libc::c_int
                == 's' as i32
            {
                num += 1;
                num;
            } else {
                return 999 as libc::c_int
            }
        }
        lessopen = lessopen.offset(1);
        lessopen;
    }
    return num;
}
pub unsafe extern "C" fn open_altfile(
    mut filename: *mut libc::c_char,
    mut pf: *mut libc::c_int,
    mut pfd: *mut *mut libc::c_void,
) -> *mut libc::c_char {
    let mut lessopen: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut qfilename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut fd: *mut FILE = 0 as *mut FILE;
    let mut returnfd: libc::c_int = 0 as libc::c_int;
    if use_lessopen == 0 || secure != 0 {
        return 0 as *mut libc::c_char;
    }
    ch_ungetchar(-(1 as libc::c_int));
    lessopen = lgetenv(
        b"LESSOPEN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if lessopen.is_null() {
        return 0 as *mut libc::c_char;
    }
    while *lessopen as libc::c_int == '|' as i32 {
        lessopen = lessopen.offset(1);
        lessopen;
        returnfd += 1;
        returnfd;
    }
    if *lessopen as libc::c_int == '-' as i32 {
        lessopen = lessopen.offset(1);
        lessopen;
    } else if strcmp(filename, b"-\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 0 as *mut libc::c_char
    }
    if num_pct_s(lessopen) != 1 as libc::c_int {
        error(
            b"LESSOPEN ignored: must contain exactly one %%s\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
        return 0 as *mut libc::c_char;
    }
    qfilename = shell_quote(filename);
    len = (strlen(lessopen))
        .wrapping_add(strlen(qfilename))
        .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    cmd = ecalloc(
        len,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
    ) as *mut libc::c_char;
    snprintf(cmd, len as libc::c_ulong, lessopen, qfilename);
    free(qfilename as *mut libc::c_void);
    fd = shellcmd(cmd);
    free(cmd as *mut libc::c_void);
    if fd.is_null() {
        return 0 as *mut libc::c_char;
    }
    if returnfd != 0 {
        let mut c: libc::c_char = 0;
        let mut f: libc::c_int = 0;
        f = fileno(fd);
        if read(
            f,
            &mut c as *mut libc::c_char as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) != 1 as libc::c_int as libc::c_long
        {
            let mut status: libc::c_int = pclose(fd);
            if returnfd > 1 as libc::c_int && status == 0 as libc::c_int {
                *pfd = 0 as *mut libc::c_void;
                *pf = -(1 as libc::c_int);
                return save(
                    b"@/\\less/\\empty/\\file/\\@\0" as *const u8 as *const libc::c_char,
                );
            }
            return 0 as *mut libc::c_char;
        }
        ch_ungetchar(c as libc::c_int);
        *pfd = fd as *mut libc::c_void;
        *pf = f;
        return save(b"-\0" as *const u8 as *const libc::c_char);
    }
    cmd = readfd(fd);
    pclose(fd);
    if *cmd as libc::c_int == '\0' as i32 {
        free(cmd as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    return cmd;
}
pub unsafe extern "C" fn close_altfile(
    mut altfilename: *mut libc::c_char,
    mut filename: *mut libc::c_char,
) {
    let mut lessclose: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut qfilename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut qaltfilename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fd: *mut FILE = 0 as *mut FILE;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    if secure != 0 {
        return;
    }
    ch_ungetchar(-(1 as libc::c_int));
    lessclose = lgetenv(
        b"LESSCLOSE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if lessclose.is_null() {
        return;
    }
    if num_pct_s(lessclose) > 2 as libc::c_int {
        error(
            b"LESSCLOSE ignored; must contain no more than 2 %%s\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
        return;
    }
    qfilename = shell_quote(filename);
    qaltfilename = shell_quote(altfilename);
    len = (strlen(lessclose))
        .wrapping_add(strlen(qfilename))
        .wrapping_add(strlen(qaltfilename))
        .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    cmd = ecalloc(
        len,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
    ) as *mut libc::c_char;
    snprintf(cmd, len as libc::c_ulong, lessclose, qfilename, qaltfilename);
    free(qaltfilename as *mut libc::c_void);
    free(qfilename as *mut libc::c_void);
    fd = shellcmd(cmd);
    free(cmd as *mut libc::c_void);
    if !fd.is_null() {
        pclose(fd);
    }
}
pub unsafe extern "C" fn is_dir(mut filename: *mut libc::c_char) -> libc::c_int {
    let mut isdir: libc::c_int = 0 as libc::c_int;
    let mut r: libc::c_int = 0;
    let mut statbuf: stat = stat {
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
    r = stat(filename, &mut statbuf);
    isdir = (r >= 0 as libc::c_int
        && statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int;
    return isdir;
}
pub unsafe extern "C" fn bad_file(mut filename: *mut libc::c_char) -> *mut libc::c_char {
    let mut m: *mut libc::c_char = 0 as *mut libc::c_char;
    if force_open == 0 && is_dir(filename) != 0 {
        static mut is_a_dir: [libc::c_char; 16] = unsafe {
            *::std::mem::transmute::<
                &[u8; 16],
                &mut [libc::c_char; 16],
            >(b" is a directory\0")
        };
        m = ecalloc(
            (strlen(filename))
                .wrapping_add(
                    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                ) as libc::c_int,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
        ) as *mut libc::c_char;
        strcpy(m, filename);
        strcat(m, is_a_dir.as_mut_ptr());
    } else {
        let mut r: libc::c_int = 0;
        let mut statbuf: stat = stat {
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
        r = stat(filename, &mut statbuf);
        if r < 0 as libc::c_int {
            m = errno_message(filename);
        } else if force_open != 0 {
            m = 0 as *mut libc::c_char;
        } else if !(statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint)
        {
            static mut not_reg: [libc::c_char; 42] = unsafe {
                *::std::mem::transmute::<
                    &[u8; 42],
                    &mut [libc::c_char; 42],
                >(b" is not a regular file (use -f to see it)\0")
            };
            m = ecalloc(
                (strlen(filename))
                    .wrapping_add(
                        ::std::mem::size_of::<[libc::c_char; 42]>() as libc::c_ulong,
                    ) as libc::c_int,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
            ) as *mut libc::c_char;
            strcpy(m, filename);
            strcat(m, not_reg.as_mut_ptr());
        }
    }
    return m;
}
pub unsafe extern "C" fn filesize(mut f: libc::c_int) -> POSITION {
    let mut statbuf: stat = stat {
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
    if fstat(f, &mut statbuf) >= 0 as libc::c_int {
        return statbuf.st_size;
    }
    return seek_filesize(f);
}
pub unsafe extern "C" fn curr_ifile_changed() -> libc::c_int {
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
    let mut curr_pos: POSITION = ch_tell();
    let mut r: libc::c_int = stat(get_filename(curr_ifile), &mut st);
    if r == 0 as libc::c_int
        && (st.st_ino != curr_ino || st.st_dev != curr_dev
            || curr_pos != -(1 as libc::c_int) as POSITION && st.st_size < curr_pos)
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn shell_coption() -> *mut libc::c_char {
    return b"-c\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
pub unsafe extern "C" fn last_component(
    mut name: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
    slash = name.offset(strlen(name) as isize);
    while slash > name {
        slash = slash.offset(-1);
        slash;
        if *slash as libc::c_int
            == *(b"/\0" as *const u8 as *const libc::c_char) as libc::c_int
            || *slash as libc::c_int == '/' as i32
        {
            return slash.offset(1 as libc::c_int as isize);
        }
    }
    return name;
}
