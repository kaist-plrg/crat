use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn perror(__s: *const libc::c_char);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn writebuf(_: libc::c_int, _: *const libc::c_void, _: size_t) -> ssize_t;
    static mut starting_directory: *mut libc::c_char;
    static mut makelevel: libc::c_uint;
    static mut program: *const libc::c_char;
    static mut print_data_base_flag: libc::c_int;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn should_print_dir() -> libc::c_int;
    static mut output_sync: libc::c_int;
    fn get_tmpfd(_: *mut *mut libc::c_char) -> libc::c_int;
    fn die(_: libc::c_int) -> !;
    fn check_io_state() -> libc::c_uint;
    fn fd_noinherit(_: libc::c_int);
    fn fd_set_append(_: libc::c_int);
    fn osync_clear();
    fn osync_acquire() -> libc::c_uint;
    fn osync_release();
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const libc::c_char,
    pub lineno: libc::c_ulong,
    pub offset: libc::c_ulong,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct output {
    pub out: libc::c_int,
    pub err: libc::c_int,
    #[bitfield(name = "syncout", ty = "libc::c_uint", bits = "0..=0")]
    pub syncout: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fmtstring {
    pub buffer: *mut libc::c_char,
    pub size: size_t,
}
pub static mut output_context: *mut output = 0 as *const output as *mut output;
pub static mut stdio_traced: libc::c_uint = 0 as libc::c_int as libc::c_uint;
unsafe extern "C" fn _outputs(
    mut out: *mut output,
    mut is_err: libc::c_int,
    mut msg: *const libc::c_char,
) {
    let mut f: *mut FILE = 0 as *mut FILE;
    if !out.is_null() && (*out).syncout() as libc::c_int != 0 {
        let mut fd: libc::c_int = if is_err != 0 { (*out).err } else { (*out).out };
        if fd != -(1 as libc::c_int) {
            let mut len: size_t = strlen(msg);
            let mut r: libc::c_int = 0;
            loop {
                r = lseek(fd, 0 as libc::c_int as __off_t, 2 as libc::c_int)
                    as libc::c_int;
                if !(r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
            }
            writebuf(fd, msg as *const libc::c_void, len);
            return;
        }
    }
    f = if is_err != 0 { stderr } else { stdout };
    fputs(msg, f);
    fflush(f);
}
unsafe extern "C" fn log_working_directory(mut entering: libc::c_int) -> libc::c_int {
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut len: size_t = 0 as libc::c_int as size_t;
    let mut need: size_t = 0;
    let mut fmt: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    need = (strlen(program))
        .wrapping_add(
            (53 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                .wrapping_div(22 as libc::c_int as libc::c_ulong)
                .wrapping_add(3 as libc::c_int as libc::c_ulong),
        )
        .wrapping_add(2 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    if !starting_directory.is_null() {
        need = (need as libc::c_ulong).wrapping_add(strlen(starting_directory)) as size_t
            as size_t;
    }
    if makelevel == 0 as libc::c_int as libc::c_uint {
        if starting_directory.is_null() {
            if entering != 0 {
                fmt = dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Entering an unknown directory\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                );
            } else {
                fmt = dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Leaving an unknown directory\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                );
            }
        } else if entering != 0 {
            fmt = dcgettext(
                0 as *const libc::c_char,
                b"%s: Entering directory '%s'\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        } else {
            fmt = dcgettext(
                0 as *const libc::c_char,
                b"%s: Leaving directory '%s'\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
    } else if starting_directory.is_null() {
        if entering != 0 {
            fmt = dcgettext(
                0 as *const libc::c_char,
                b"%s[%u]: Entering an unknown directory\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            );
        } else {
            fmt = dcgettext(
                0 as *const libc::c_char,
                b"%s[%u]: Leaving an unknown directory\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            );
        }
    } else if entering != 0 {
        fmt = dcgettext(
            0 as *const libc::c_char,
            b"%s[%u]: Entering directory '%s'\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    } else {
        fmt = dcgettext(
            0 as *const libc::c_char,
            b"%s[%u]: Leaving directory '%s'\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    }
    need = (need as libc::c_ulong).wrapping_add(strlen(fmt)) as size_t as size_t;
    if need > len {
        buf = xrealloc(buf as *mut libc::c_void, need) as *mut libc::c_char;
        len = need;
    }
    p = buf;
    if print_data_base_flag != 0 {
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = '#' as i32 as libc::c_char;
        let fresh1 = p;
        p = p.offset(1);
        *fresh1 = ' ' as i32 as libc::c_char;
    }
    if makelevel == 0 as libc::c_int as libc::c_uint {
        if starting_directory.is_null() {
            sprintf(p, fmt, program);
        } else {
            sprintf(p, fmt, program, starting_directory);
        }
    } else if starting_directory.is_null() {
        sprintf(p, fmt, program, makelevel);
    } else {
        sprintf(p, fmt, program, makelevel, starting_directory);
    }
    _outputs(0 as *mut output, 0 as libc::c_int, buf);
    return 1 as libc::c_int;
}
unsafe extern "C" fn pump_from_tmp(mut from: libc::c_int, mut to: *mut FILE) {
    static mut buffer: [libc::c_char; 8192] = [0; 8192];
    if lseek(from, 0 as libc::c_int as __off_t, 0 as libc::c_int)
        == -(1 as libc::c_int) as libc::c_long
    {
        perror(b"lseek()\0" as *const u8 as *const libc::c_char);
    }
    loop {
        let mut len: libc::c_int = 0;
        loop {
            len = read(
                from,
                buffer.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            ) as libc::c_int;
            if !(len == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if len < 0 as libc::c_int {
            perror(b"read()\0" as *const u8 as *const libc::c_char);
        }
        if len <= 0 as libc::c_int {
            break;
        }
        if fwrite(
            buffer.as_mut_ptr() as *const libc::c_void,
            len as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            to,
        ) < 1 as libc::c_int as libc::c_ulong
        {
            perror(b"fwrite()\0" as *const u8 as *const libc::c_char);
            break;
        } else {
            fflush(to);
        }
    };
}
pub unsafe extern "C" fn output_tmpfd() -> libc::c_int {
    let mut fd: libc::c_int = get_tmpfd(0 as *mut *mut libc::c_char);
    fd_set_append(fd);
    return fd;
}
unsafe extern "C" fn setup_tmpfile(mut out: *mut output) {
    let mut current_block: u64;
    static mut in_setup: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut io_state: libc::c_uint = 0;
    if in_setup != 0 {
        return;
    }
    in_setup = 1 as libc::c_int as libc::c_uint;
    io_state = check_io_state();
    if !(io_state & (0x8 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
        != 0 as libc::c_int as libc::c_uint)
    {
        perror_with_name(
            b"output-sync suppressed: \0" as *const u8 as *const libc::c_char,
            b"stderr\0" as *const u8 as *const libc::c_char,
        );
    } else {
        if io_state & 0x8 as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            let mut fd: libc::c_int = output_tmpfd();
            if fd < 0 as libc::c_int {
                current_block = 10266274846898779876;
            } else {
                fd_noinherit(fd);
                (*out).out = fd;
                current_block = 3276175668257526147;
            }
        } else {
            current_block = 3276175668257526147;
        }
        match current_block {
            10266274846898779876 => {}
            _ => {
                if io_state & 0x10 as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint
                {
                    if (*out).out != -(1 as libc::c_int)
                        && io_state & 0x2 as libc::c_int as libc::c_uint
                            != 0 as libc::c_int as libc::c_uint
                    {
                        (*out).err = (*out).out;
                        current_block = 9606288038608642794;
                    } else {
                        let mut fd_0: libc::c_int = output_tmpfd();
                        if fd_0 < 0 as libc::c_int {
                            current_block = 10266274846898779876;
                        } else {
                            fd_noinherit(fd_0);
                            (*out).err = fd_0;
                            current_block = 9606288038608642794;
                        }
                    }
                } else {
                    current_block = 9606288038608642794;
                }
                match current_block {
                    10266274846898779876 => {}
                    _ => {
                        in_setup = 0 as libc::c_int as libc::c_uint;
                        return;
                    }
                }
            }
        }
    }
    error(
        0 as *mut floc,
        0 as libc::c_int as size_t,
        dcgettext(
            0 as *const libc::c_char,
            b"cannot open output-sync lock file, suppressing output-sync.\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    output_close(out);
    output_sync = 0 as libc::c_int;
    osync_clear();
    in_setup = 0 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn output_dump(mut out: *mut output) {
    let mut outfd_not_empty: libc::c_int = ((*out).out != -(1 as libc::c_int)
        && lseek((*out).out, 0 as libc::c_int as __off_t, 2 as libc::c_int)
            > 0 as libc::c_int as libc::c_long) as libc::c_int;
    let mut errfd_not_empty: libc::c_int = ((*out).err != -(1 as libc::c_int)
        && lseek((*out).err, 0 as libc::c_int as __off_t, 2 as libc::c_int)
            > 0 as libc::c_int as libc::c_long) as libc::c_int;
    if outfd_not_empty != 0 || errfd_not_empty != 0 {
        let mut traced: libc::c_int = 0 as libc::c_int;
        if osync_acquire() == 0 {
            error(
                0 as *mut floc,
                0 as libc::c_int as size_t,
                dcgettext(
                    0 as *const libc::c_char,
                    b"warning: Cannot acquire output lock, disabling output sync.\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            osync_clear();
        }
        if output_sync != 3 as libc::c_int && should_print_dir() != 0 {
            traced = log_working_directory(1 as libc::c_int);
        }
        if outfd_not_empty != 0 {
            pump_from_tmp((*out).out, stdout);
        }
        if errfd_not_empty != 0 && (*out).err != (*out).out {
            pump_from_tmp((*out).err, stderr);
        }
        if traced != 0 {
            log_working_directory(0 as libc::c_int);
        }
        osync_release();
        if (*out).out != -(1 as libc::c_int) {
            let mut e: libc::c_int = 0;
            lseek((*out).out, 0 as libc::c_int as __off_t, 0 as libc::c_int);
            loop {
                e = ftruncate((*out).out, 0 as libc::c_int as __off_t);
                if !(e == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
            }
        }
        if (*out).err != -(1 as libc::c_int) && (*out).err != (*out).out {
            let mut e_0: libc::c_int = 0;
            lseek((*out).err, 0 as libc::c_int as __off_t, 0 as libc::c_int);
            loop {
                e_0 = ftruncate((*out).err, 0 as libc::c_int as __off_t);
                if !(e_0 == -(1 as libc::c_int)
                    && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
            }
        }
    }
}
pub unsafe extern "C" fn output_init(mut out: *mut output) {
    if !out.is_null() {
        (*out).err = -(1 as libc::c_int);
        (*out).out = (*out).err;
        (*out).set_syncout((output_sync != 0) as libc::c_int as libc::c_uint);
        return;
    }
    fd_set_append(fileno(stdout));
    fd_set_append(fileno(stderr));
}
pub unsafe extern "C" fn output_close(mut out: *mut output) {
    if out.is_null() {
        if stdio_traced != 0 {
            log_working_directory(0 as libc::c_int);
        }
        return;
    }
    output_dump(out);
    if (*out).out >= 0 as libc::c_int {
        close((*out).out);
    }
    if (*out).err >= 0 as libc::c_int && (*out).err != (*out).out {
        close((*out).err);
    }
    output_init(out);
}
pub unsafe extern "C" fn output_start() {
    if !output_context.is_null() && (*output_context).syncout() as libc::c_int != 0 {
        if !((*output_context).out >= 0 as libc::c_int
            || (*output_context).err >= 0 as libc::c_int)
        {
            setup_tmpfile(output_context);
        }
    }
    if output_sync == 0 as libc::c_int || output_sync == 3 as libc::c_int {
        if stdio_traced == 0 && should_print_dir() != 0 {
            stdio_traced = log_working_directory(1 as libc::c_int) as libc::c_uint;
        }
    }
}
pub unsafe extern "C" fn outputs(mut is_err: libc::c_int, mut msg: *const libc::c_char) {
    if msg.is_null() || *msg as libc::c_int == '\0' as i32 {
        return;
    }
    output_start();
    _outputs(output_context, is_err, msg);
}
static mut fmtbuf: fmtstring = {
    let mut init = fmtstring {
        buffer: 0 as *const libc::c_char as *mut libc::c_char,
        size: 0 as libc::c_int as size_t,
    };
    init
};
unsafe extern "C" fn get_buffer(mut need: size_t) -> *mut libc::c_char {
    if need > fmtbuf.size {
        fmtbuf
            .size = (fmtbuf.size as libc::c_ulong)
            .wrapping_add(need.wrapping_mul(2 as libc::c_int as libc::c_ulong)) as size_t
            as size_t;
        fmtbuf
            .buffer = xrealloc(fmtbuf.buffer as *mut libc::c_void, fmtbuf.size)
            as *mut libc::c_char;
    }
    *(fmtbuf.buffer)
        .offset(
            need.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = '\0' as i32 as libc::c_char;
    return fmtbuf.buffer;
}
pub unsafe extern "C" fn message(
    mut prefix: libc::c_int,
    mut len: size_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    len = (len as libc::c_ulong)
        .wrapping_add(
            (strlen(fmt))
                .wrapping_add(strlen(program))
                .wrapping_add(
                    (53 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<uintmax_t>() as libc::c_ulong,
                        )
                        .wrapping_div(22 as libc::c_int as libc::c_ulong)
                        .wrapping_add(3 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as size_t as size_t;
    p = get_buffer(len);
    start = p;
    if prefix != 0 {
        if makelevel == 0 as libc::c_int as libc::c_uint {
            sprintf(p, b"%s: \0" as *const u8 as *const libc::c_char, program);
        } else {
            sprintf(
                p,
                b"%s[%u]: \0" as *const u8 as *const libc::c_char,
                program,
                makelevel,
            );
        }
        p = p.offset(strlen(p) as isize);
    }
    args_0 = args.clone();
    vsprintf(p, fmt, args_0.as_va_list());
    strcat(p, b"\n\0" as *const u8 as *const libc::c_char);
    outputs(0 as libc::c_int, start);
}
pub unsafe extern "C" fn error(
    mut flocp: *const floc,
    mut len: size_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    len = (len as libc::c_ulong)
        .wrapping_add(
            (strlen(fmt))
                .wrapping_add(strlen(program))
                .wrapping_add(
                    (if !flocp.is_null() && !((*flocp).filenm).is_null() {
                        strlen((*flocp).filenm)
                    } else {
                        0 as libc::c_int as libc::c_ulong
                    }),
                )
                .wrapping_add(
                    (53 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<uintmax_t>() as libc::c_ulong,
                        )
                        .wrapping_div(22 as libc::c_int as libc::c_ulong)
                        .wrapping_add(3 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as size_t as size_t;
    p = get_buffer(len);
    start = p;
    if !flocp.is_null() && !((*flocp).filenm).is_null() {
        sprintf(
            p,
            b"%s:%lu: \0" as *const u8 as *const libc::c_char,
            (*flocp).filenm,
            ((*flocp).lineno).wrapping_add((*flocp).offset),
        );
    } else if makelevel == 0 as libc::c_int as libc::c_uint {
        sprintf(p, b"%s: \0" as *const u8 as *const libc::c_char, program);
    } else {
        sprintf(
            p,
            b"%s[%u]: \0" as *const u8 as *const libc::c_char,
            program,
            makelevel,
        );
    }
    p = p.offset(strlen(p) as isize);
    args_0 = args.clone();
    vsprintf(p, fmt, args_0.as_va_list());
    strcat(p, b"\n\0" as *const u8 as *const libc::c_char);
    outputs(1 as libc::c_int, start);
}
pub unsafe extern "C" fn fatal(
    mut flocp: *const floc,
    mut len: size_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> ! {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut stop: *const libc::c_char = dcgettext(
        0 as *const libc::c_char,
        b".  Stop.\n\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    len = (len as libc::c_ulong)
        .wrapping_add(
            (strlen(fmt))
                .wrapping_add(strlen(program))
                .wrapping_add(
                    (if !flocp.is_null() && !((*flocp).filenm).is_null() {
                        strlen((*flocp).filenm)
                    } else {
                        0 as libc::c_int as libc::c_ulong
                    }),
                )
                .wrapping_add(
                    (53 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<uintmax_t>() as libc::c_ulong,
                        )
                        .wrapping_div(22 as libc::c_int as libc::c_ulong)
                        .wrapping_add(3 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(8 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen(stop))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as size_t as size_t;
    p = get_buffer(len);
    start = p;
    if !flocp.is_null() && !((*flocp).filenm).is_null() {
        sprintf(
            p,
            b"%s:%lu: *** \0" as *const u8 as *const libc::c_char,
            (*flocp).filenm,
            ((*flocp).lineno).wrapping_add((*flocp).offset),
        );
    } else if makelevel == 0 as libc::c_int as libc::c_uint {
        sprintf(p, b"%s: *** \0" as *const u8 as *const libc::c_char, program);
    } else {
        sprintf(
            p,
            b"%s[%u]: *** \0" as *const u8 as *const libc::c_char,
            program,
            makelevel,
        );
    }
    p = p.offset(strlen(p) as isize);
    args_0 = args.clone();
    vsprintf(p, fmt, args_0.as_va_list());
    strcat(p, stop);
    outputs(1 as libc::c_int, start);
    die(2 as libc::c_int);
}
pub unsafe extern "C" fn perror_with_name(
    mut str: *const libc::c_char,
    mut name: *const libc::c_char,
) {
    let mut err: *const libc::c_char = strerror(*__errno_location());
    error(
        0 as *mut floc,
        (strlen(str)).wrapping_add(strlen(name)).wrapping_add(strlen(err)),
        dcgettext(
            0 as *const libc::c_char,
            b"%s%s: %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        str,
        name,
        err,
    );
}
pub unsafe extern "C" fn pfatal_with_name(mut name: *const libc::c_char) -> ! {
    let mut err: *const libc::c_char = strerror(*__errno_location());
    fatal(
        0 as *mut floc,
        (strlen(name)).wrapping_add(strlen(err)),
        dcgettext(
            0 as *const libc::c_char,
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        name,
        err,
    );
}
pub unsafe extern "C" fn out_of_memory() -> ! {
    writebuf(fileno(stdout), program as *const libc::c_void, strlen(program));
    writebuf(
        fileno(stdout),
        b": *** virtual memory exhausted\n\0" as *const u8 as *const libc::c_char
            as *const libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    exit(2 as libc::c_int);
}
