use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
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
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn _glp_xstrerr(_: libc::c_int) -> *mut libc::c_char;
    fn _glp_get_env_ptr() -> *mut ENV;
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn _glp_put_err_msg(msg: *const libc::c_char);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn _glp_zlib_gzread(file: gzFile, buf: voidp, len: libc::c_uint) -> libc::c_int;
    fn _glp_zlib_gzwrite(file: gzFile, buf: voidpc, len: libc::c_uint) -> libc::c_int;
    fn _glp_zlib_gzclose(file: gzFile) -> libc::c_int;
    fn _glp_zlib_gzerror(file: gzFile, errnum: *mut libc::c_int) -> *const libc::c_char;
    fn _glp_zlib_gzopen(_: *const libc::c_char, _: *const libc::c_char) -> gzFile;
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENV {
    pub self_0: *mut ENV,
    pub term_buf: *mut libc::c_char,
    pub term_out: libc::c_int,
    pub term_hook: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int,
    >,
    pub term_info: *mut libc::c_void,
    pub tee_file: *mut FILE,
    pub err_st: libc::c_int,
    pub err_file: *const libc::c_char,
    pub err_line: libc::c_int,
    pub err_hook: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub err_info: *mut libc::c_void,
    pub err_buf: *mut libc::c_char,
    pub mem_limit: size_t,
    pub mem_ptr: *mut MBD,
    pub mem_count: libc::c_int,
    pub mem_cpeak: libc::c_int,
    pub mem_total: size_t,
    pub mem_tpeak: size_t,
    pub gmp_pool: *mut libc::c_void,
    pub gmp_size: libc::c_int,
    pub gmp_work: *mut libc::c_ushort,
    pub h_odbc: *mut libc::c_void,
    pub h_mysql: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MBD {
    pub size: size_t,
    pub self_0: *mut MBD,
    pub prev: *mut MBD,
    pub next: *mut MBD,
}
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_file {
    pub base: *mut libc::c_char,
    pub size: libc::c_int,
    pub ptr: *mut libc::c_char,
    pub cnt: libc::c_int,
    pub flag: libc::c_int,
    pub file: *mut libc::c_void,
}
pub type gzFile = voidp;
pub type voidp = *mut libc::c_void;
pub type voidpc = *const libc::c_void;
pub unsafe extern "C" fn _glp_open(
    mut name: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> *mut glp_file {
    let mut f: *mut glp_file = 0 as *mut glp_file;
    let mut flag: libc::c_int = 0;
    let mut file: *mut libc::c_void = 0 as *mut libc::c_void;
    if strcmp(mode, b"r\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(mode, b"rb\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        flag = 0 as libc::c_int;
    } else if strcmp(mode, b"w\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp(mode, b"wb\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        flag = 0x8 as libc::c_int;
    } else if strcmp(mode, b"a\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp(mode, b"ab\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        flag = 0x8 as libc::c_int;
    } else {
        (glp_error_(
            b"env/stream.c\0" as *const u8 as *const libc::c_char,
            101 as libc::c_int,
        ))
            .unwrap()(
            b"glp_open: invalid mode string\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if strcmp(name, b"/dev/null\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        flag |= 0x1 as libc::c_int;
        file = 0 as *mut libc::c_void;
    } else if strcmp(name, b"/dev/stdin\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        flag |= 0x2 as libc::c_int;
        file = stdin as *mut libc::c_void;
    } else if strcmp(name, b"/dev/stdout\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        flag |= 0x2 as libc::c_int;
        file = stdout as *mut libc::c_void;
    } else if strcmp(name, b"/dev/stderr\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        flag |= 0x2 as libc::c_int;
        file = stderr as *mut libc::c_void;
    } else {
        let mut ext: *mut libc::c_char = strrchr(name, '.' as i32);
        if ext.is_null()
            || strcmp(ext, b".gz\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int
        {
            file = fopen(name, mode) as *mut libc::c_void;
            if file.is_null() {
                _glp_put_err_msg(_glp_xstrerr(*__errno_location()));
                return 0 as *mut glp_file;
            }
        } else {
            flag |= 0x4 as libc::c_int;
            if strcmp(mode, b"r\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                mode = b"rb\0" as *const u8 as *const libc::c_char;
            } else if strcmp(mode, b"w\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                mode = b"wb\0" as *const u8 as *const libc::c_char;
            } else if strcmp(mode, b"a\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                mode = b"ab\0" as *const u8 as *const libc::c_char;
            }
            file = _glp_zlib_gzopen(name, mode);
            if file.is_null() {
                _glp_put_err_msg(_glp_xstrerr(*__errno_location()));
                return 0 as *mut glp_file;
            }
        }
    }
    f = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<glp_file>() as libc::c_ulong as libc::c_int,
    ) as *mut glp_file;
    (*f)
        .base = glp_alloc(
        8192 as libc::c_int,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    (*f).size = 8192 as libc::c_int;
    (*f).ptr = (*f).base;
    (*f).cnt = 0 as libc::c_int;
    (*f).flag = flag;
    (*f).file = file;
    return f;
}
pub unsafe extern "C" fn _glp_eof(mut f: *mut glp_file) -> libc::c_int {
    return (*f).flag & 0x10 as libc::c_int;
}
pub unsafe extern "C" fn _glp_ioerr(mut f: *mut glp_file) -> libc::c_int {
    return (*f).flag & 0x20 as libc::c_int;
}
pub unsafe extern "C" fn _glp_read(
    mut f: *mut glp_file,
    mut buf: *mut libc::c_void,
    mut nnn: libc::c_int,
) -> libc::c_int {
    let mut nrd: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    if (*f).flag & 0x8 as libc::c_int != 0 {
        (glp_error_(
            b"env/stream.c\0" as *const u8 as *const libc::c_char,
            235 as libc::c_int,
        ))
            .unwrap()(
            b"glp_read: attempt to read from output stream\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if nnn < 1 as libc::c_int {
        (glp_error_(
            b"env/stream.c\0" as *const u8 as *const libc::c_char,
            237 as libc::c_int,
        ))
            .unwrap()(
            b"glp_read: nnn = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            nnn,
        );
    }
    nrd = 0 as libc::c_int;
    while nrd < nnn {
        if (*f).cnt == 0 as libc::c_int {
            if (*f).flag & 0x1 as libc::c_int != 0 {
                cnt = 0 as libc::c_int;
            } else if (*f).flag & 0x4 as libc::c_int == 0 {
                cnt = fread(
                    (*f).base as *mut libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    (*f).size as libc::c_ulong,
                    (*f).file as *mut FILE,
                ) as libc::c_int;
                if ferror((*f).file as *mut FILE) != 0 {
                    (*f).flag |= 0x20 as libc::c_int;
                    _glp_put_err_msg(_glp_xstrerr(*__errno_location()));
                    return -(1 as libc::c_int);
                }
            } else {
                let mut errnum: libc::c_int = 0;
                let mut msg: *const libc::c_char = 0 as *const libc::c_char;
                cnt = _glp_zlib_gzread(
                    (*f).file,
                    (*f).base as voidp,
                    (*f).size as libc::c_uint,
                );
                if cnt < 0 as libc::c_int {
                    (*f).flag |= 0x20 as libc::c_int;
                    msg = _glp_zlib_gzerror((*f).file, &mut errnum);
                    if errnum == -(1 as libc::c_int) {
                        _glp_put_err_msg(_glp_xstrerr(*__errno_location()));
                    } else {
                        _glp_put_err_msg(msg);
                    }
                    return -(1 as libc::c_int);
                }
            }
            if cnt == 0 as libc::c_int {
                if nrd == 0 as libc::c_int {
                    (*f).flag |= 0x10 as libc::c_int;
                }
                break;
            } else {
                (*f).ptr = (*f).base;
                (*f).cnt = cnt;
            }
        }
        cnt = nnn - nrd;
        if cnt > (*f).cnt {
            cnt = (*f).cnt;
        }
        memcpy(
            (buf as *mut libc::c_char).offset(nrd as isize) as *mut libc::c_void,
            (*f).ptr as *const libc::c_void,
            cnt as libc::c_ulong,
        );
        (*f).ptr = ((*f).ptr).offset(cnt as isize);
        (*f).cnt -= cnt;
        nrd += cnt;
    }
    return nrd;
}
pub unsafe extern "C" fn _glp_getc(mut f: *mut glp_file) -> libc::c_int {
    let mut buf: [libc::c_uchar; 1] = [0; 1];
    if (*f).flag & 0x8 as libc::c_int != 0 {
        (glp_error_(
            b"env/stream.c\0" as *const u8 as *const libc::c_char,
            315 as libc::c_int,
        ))
            .unwrap()(
            b"glp_getc: attempt to read from output stream\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if _glp_read(f, buf.as_mut_ptr() as *mut libc::c_void, 1 as libc::c_int)
        != 1 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    return buf[0 as libc::c_int as usize] as libc::c_int;
}
unsafe extern "C" fn do_flush(mut f: *mut glp_file) -> libc::c_int {
    ((*f).flag & 0x8 as libc::c_int != 0
        || {
            glp_assert_(
                b"f->flag & IOWRT\0" as *const u8 as *const libc::c_char,
                b"env/stream.c\0" as *const u8 as *const libc::c_char,
                331 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*f).cnt > 0 as libc::c_int {
        if !((*f).flag & 0x1 as libc::c_int != 0) {
            if (*f).flag & 0x4 as libc::c_int == 0 {
                if fwrite(
                    (*f).base as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    (*f).cnt as libc::c_ulong,
                    (*f).file as *mut FILE,
                ) as libc::c_int != (*f).cnt
                {
                    (*f).flag |= 0x20 as libc::c_int;
                    _glp_put_err_msg(_glp_xstrerr(*__errno_location()));
                    return -(1 as libc::c_int);
                }
            } else {
                let mut errnum: libc::c_int = 0;
                let mut msg: *const libc::c_char = 0 as *const libc::c_char;
                if _glp_zlib_gzwrite(
                    (*f).file,
                    (*f).base as voidpc,
                    (*f).cnt as libc::c_uint,
                ) != (*f).cnt
                {
                    (*f).flag |= 0x20 as libc::c_int;
                    msg = _glp_zlib_gzerror((*f).file, &mut errnum);
                    if errnum == -(1 as libc::c_int) {
                        _glp_put_err_msg(_glp_xstrerr(*__errno_location()));
                    } else {
                        _glp_put_err_msg(msg);
                    }
                    return -(1 as libc::c_int);
                }
            }
        }
    }
    (*f).ptr = (*f).base;
    (*f).cnt = 0 as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_write(
    mut f: *mut glp_file,
    mut buf: *const libc::c_void,
    mut nnn: libc::c_int,
) -> libc::c_int {
    let mut nwr: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    if (*f).flag & 0x8 as libc::c_int == 0 {
        (glp_error_(
            b"env/stream.c\0" as *const u8 as *const libc::c_char,
            394 as libc::c_int,
        ))
            .unwrap()(
            b"glp_write: attempt to write to input stream\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if nnn < 1 as libc::c_int {
        (glp_error_(
            b"env/stream.c\0" as *const u8 as *const libc::c_char,
            396 as libc::c_int,
        ))
            .unwrap()(
            b"glp_write: nnn = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            nnn,
        );
    }
    nwr = 0 as libc::c_int;
    while nwr < nnn {
        cnt = nnn - nwr;
        if cnt > (*f).size - (*f).cnt {
            cnt = (*f).size - (*f).cnt;
        }
        memcpy(
            (*f).ptr as *mut libc::c_void,
            (buf as *const libc::c_char).offset(nwr as isize) as *const libc::c_void,
            cnt as libc::c_ulong,
        );
        (*f).ptr = ((*f).ptr).offset(cnt as isize);
        (*f).cnt += cnt;
        if (*f).cnt == (*f).size {
            if do_flush(f) != 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
        nwr += cnt;
    }
    return nwr;
}
pub unsafe extern "C" fn _glp_format(
    mut f: *mut glp_file,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut env: *mut ENV = _glp_get_env_ptr();
    let mut arg: ::std::ffi::VaListImpl;
    let mut nnn: libc::c_int = 0;
    if (*f).flag & 0x8 as libc::c_int == 0 {
        (glp_error_(
            b"env/stream.c\0" as *const u8 as *const libc::c_char,
            438 as libc::c_int,
        ))
            .unwrap()(
            b"glp_format: attempt to write to input stream\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    arg = args.clone();
    nnn = vsprintf((*env).term_buf, fmt, arg.as_va_list());
    (0 as libc::c_int <= nnn && nnn < 4096 as libc::c_int
        || {
            glp_assert_(
                b"0 <= nnn && nnn < TBUF_SIZE\0" as *const u8 as *const libc::c_char,
                b"env/stream.c\0" as *const u8 as *const libc::c_char,
                441 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return if nnn == 0 as libc::c_int {
        0 as libc::c_int
    } else {
        _glp_write(f, (*env).term_buf as *const libc::c_void, nnn)
    };
}
pub unsafe extern "C" fn _glp_close(mut f: *mut glp_file) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if (*f).flag & 0x8 as libc::c_int != 0 {
        if do_flush(f) != 0 as libc::c_int {
            ret = -(1 as libc::c_int);
        }
    }
    if !((*f).flag & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0) {
        if (*f).flag & 0x4 as libc::c_int == 0 {
            if fclose((*f).file as *mut FILE) != 0 as libc::c_int {
                if ret == 0 as libc::c_int {
                    _glp_put_err_msg(_glp_xstrerr(*__errno_location()));
                    ret = -(1 as libc::c_int);
                }
            }
        } else {
            let mut errnum: libc::c_int = 0;
            errnum = _glp_zlib_gzclose((*f).file);
            if !(errnum == 0 as libc::c_int) {
                if errnum == -(1 as libc::c_int) {
                    if ret == 0 as libc::c_int {
                        _glp_put_err_msg(_glp_xstrerr(*__errno_location()));
                        ret = -(1 as libc::c_int);
                    }
                } else if ret == 0 as libc::c_int {
                    let mut env: *mut ENV = _glp_get_env_ptr();
                    sprintf(
                        (*env).term_buf,
                        b"gzclose returned %d\0" as *const u8 as *const libc::c_char,
                        errnum,
                    );
                    _glp_put_err_msg((*env).term_buf);
                    ret = -(1 as libc::c_int);
                }
            }
        }
    }
    glp_free((*f).base as *mut libc::c_void);
    glp_free(f as *mut libc::c_void);
    return ret;
}
