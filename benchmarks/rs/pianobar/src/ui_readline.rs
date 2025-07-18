use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
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
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    static mut interrupted: *mut sig_atomic_t;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
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
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
pub type BarReadlineFlags_t = libc::c_uint;
pub const BAR_RL_NOINT: BarReadlineFlags_t = 4;
pub const BAR_RL_NOECHO: BarReadlineFlags_t = 2;
pub const BAR_RL_FULLRETURN: BarReadlineFlags_t = 1;
pub const BAR_RL_DEFAULT: BarReadlineFlags_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BarReadlineFds_t {
    pub set: fd_set,
    pub maxfd: libc::c_int,
    pub fds: [libc::c_int; 2],
}
pub type sig_atomic_t = __sig_atomic_t;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
unsafe extern "C" fn BarReadlinePrevUtf8(mut ptr: *mut libc::c_char) -> size_t {
    let mut i: size_t = 0 as libc::c_int as size_t;
    loop {
        i = i.wrapping_add(1);
        i;
        ptr = ptr.offset(-1);
        ptr;
        if !(*ptr as libc::c_int & (1 as libc::c_int) << 7 as libc::c_int != 0
            && *ptr as libc::c_int & (1 as libc::c_int) << 6 as libc::c_int == 0)
        {
            break;
        }
    }
    return i;
}
pub unsafe extern "C" fn BarReadline(
    mut buf: *mut libc::c_char,
    bufSize: size_t,
    mut mask: *const libc::c_char,
    mut input: *mut BarReadlineFds_t,
    flags: BarReadlineFlags_t,
    mut timeout: libc::c_int,
) -> size_t {
    let mut bufLen: size_t = 0 as libc::c_int as size_t;
    let mut escapeState: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut set: fd_set = fd_set { __fds_bits: [0; 16] };
    let echo: bool = flags as libc::c_uint & BAR_RL_NOECHO as libc::c_int as libc::c_uint
        == 0;
    let mut done: bool = 0 as libc::c_int != 0;
    let mut prevInt: *mut sig_atomic_t = interrupted;
    let mut localInt: sig_atomic_t = 0 as libc::c_int;
    if flags as libc::c_uint & BAR_RL_NOINT as libc::c_int as libc::c_uint == 0 {
        interrupted = &mut localInt;
    }
    memset(buf as *mut libc::c_void, 0 as libc::c_int, bufSize);
    while !done {
        let mut curFd: libc::c_int = -(1 as libc::c_int);
        let mut chr: libc::c_uchar = 0;
        let mut timeoutstruct: timeval = timeval { tv_sec: 0, tv_usec: 0 };
        memcpy(
            &mut set as *mut fd_set as *mut libc::c_void,
            &mut (*input).set as *mut fd_set as *const libc::c_void,
            ::std::mem::size_of::<fd_set>() as libc::c_ulong,
        );
        timeoutstruct.tv_sec = timeout as __time_t;
        timeoutstruct.tv_usec = 0 as libc::c_int as __suseconds_t;
        if select(
            (*input).maxfd,
            &mut set,
            0 as *mut fd_set,
            0 as *mut fd_set,
            (if timeout == -(1 as libc::c_int) {
                0 as *mut timeval
            } else {
                &mut timeoutstruct
            }),
        ) <= 0 as libc::c_int
        {
            bufLen = 0 as libc::c_int as size_t;
            break;
        } else {
            if set
                .__fds_bits[((*input).fds[0 as libc::c_int as usize]
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << (*input).fds[0 as libc::c_int as usize]
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
            {
                curFd = (*input).fds[0 as libc::c_int as usize];
            } else if (*input).fds[1 as libc::c_int as usize] != -(1 as libc::c_int)
                && set
                    .__fds_bits[((*input).fds[1 as libc::c_int as usize]
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    & ((1 as libc::c_ulong)
                        << (*input).fds[1 as libc::c_int as usize]
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask
                    != 0 as libc::c_int as libc::c_long
            {
                curFd = (*input).fds[1 as libc::c_int as usize];
            }
            if read(
                curFd,
                &mut chr as *mut libc::c_uchar as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
            ) <= 0 as libc::c_int as libc::c_long
            {
                if curFd == 0 as libc::c_int {
                    (*input)
                        .set
                        .__fds_bits[(curFd
                        / (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as usize]
                        &= !(((1 as libc::c_ulong)
                            << curFd
                                % (8 as libc::c_int
                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                        as libc::c_int)) as __fd_mask);
                }
            } else {
                let mut current_block_53: u64;
                match chr as libc::c_int {
                    4 => {
                        current_block_53 = 2057008230656152631;
                    }
                    10 => {
                        current_block_53 = 2057008230656152631;
                    }
                    21 => {
                        if echo {
                            while bufLen > 0 as libc::c_int as libc::c_ulong {
                                let moveSize: size_t = BarReadlinePrevUtf8(
                                    &mut *buf.offset(bufLen as isize),
                                );
                                fputs(
                                    b"\x1B[D\x1B[K\0" as *const u8 as *const libc::c_char,
                                    stdout,
                                );
                                bufLen = (bufLen as libc::c_ulong).wrapping_sub(moveSize)
                                    as size_t as size_t;
                            }
                            fflush(stdout);
                        }
                        bufLen = 0 as libc::c_int as size_t;
                        current_block_53 = 168769493162332264;
                    }
                    27 => {
                        escapeState = 1 as libc::c_int as libc::c_uchar;
                        current_block_53 = 168769493162332264;
                    }
                    126 => {
                        current_block_53 = 168769493162332264;
                    }
                    8 | 127 => {
                        if bufLen > 0 as libc::c_int as libc::c_ulong {
                            let mut moveSize_0: size_t = BarReadlinePrevUtf8(
                                &mut *buf.offset(bufLen as isize),
                            );
                            memmove(
                                &mut *buf.offset(bufLen.wrapping_sub(moveSize_0) as isize)
                                    as *mut libc::c_char as *mut libc::c_void,
                                &mut *buf.offset(bufLen as isize) as *mut libc::c_char
                                    as *const libc::c_void,
                                moveSize_0,
                            );
                            bufLen = (bufLen as libc::c_ulong).wrapping_sub(moveSize_0)
                                as size_t as size_t;
                            if echo {
                                fputs(
                                    b"\x1B[D\x1B[K\0" as *const u8 as *const libc::c_char,
                                    stdout,
                                );
                                fflush(stdout);
                            }
                        }
                        current_block_53 = 168769493162332264;
                    }
                    _ => {
                        if chr as libc::c_int <= 0x1f as libc::c_int {
                            current_block_53 = 168769493162332264;
                        } else {
                            if escapeState as libc::c_int == 2 as libc::c_int {
                                escapeState = 0 as libc::c_int as libc::c_uchar;
                            } else if escapeState as libc::c_int == 1 as libc::c_int
                                && chr as libc::c_int == '[' as i32
                            {
                                escapeState = 2 as libc::c_int as libc::c_uchar;
                            } else if !(!mask.is_null()
                                && (strchr(mask, chr as libc::c_int)).is_null())
                            {
                                if bufLen
                                    < bufSize.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                {
                                    *buf.offset(bufLen as isize) = chr as libc::c_char;
                                    bufLen = bufLen.wrapping_add(1);
                                    bufLen;
                                    if echo {
                                        putchar(chr as libc::c_int);
                                        fflush(stdout);
                                    }
                                    if bufLen
                                        >= bufSize.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        && flags as libc::c_uint
                                            & BAR_RL_FULLRETURN as libc::c_int as libc::c_uint != 0
                                    {
                                        done = 1 as libc::c_int != 0;
                                    }
                                }
                            }
                            current_block_53 = 168769493162332264;
                        }
                    }
                }
                match current_block_53 {
                    2057008230656152631 => {
                        done = 1 as libc::c_int != 0;
                    }
                    _ => {}
                }
            }
        }
    }
    if echo {
        fputs(b"\n\0" as *const u8 as *const libc::c_char, stdout);
    }
    interrupted = prevInt;
    *buf.offset(bufLen as isize) = '\0' as i32 as libc::c_char;
    return bufLen;
}
pub unsafe extern "C" fn BarReadlineStr(
    mut buf: *mut libc::c_char,
    bufSize: size_t,
    mut input: *mut BarReadlineFds_t,
    flags: BarReadlineFlags_t,
) -> size_t {
    return BarReadline(
        buf,
        bufSize,
        0 as *const libc::c_char,
        input,
        flags,
        -(1 as libc::c_int),
    );
}
pub unsafe extern "C" fn BarReadlineInt(
    mut ret: *mut libc::c_int,
    mut input: *mut BarReadlineFds_t,
) -> size_t {
    let mut rlRet: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 16] = [0; 16];
    rlRet = BarReadline(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        b"0123456789\0" as *const u8 as *const libc::c_char,
        input,
        BAR_RL_DEFAULT,
        -(1 as libc::c_int),
    ) as libc::c_int;
    *ret = atoi(buf.as_mut_ptr());
    return rlRet as size_t;
}
pub unsafe extern "C" fn BarReadlineYesNo(
    mut def: bool,
    mut input: *mut BarReadlineFds_t,
) -> bool {
    let mut buf: [libc::c_char; 2] = [0; 2];
    BarReadline(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
        b"yYnN\0" as *const u8 as *const libc::c_char,
        input,
        BAR_RL_FULLRETURN,
        -(1 as libc::c_int),
    );
    if *buf.as_mut_ptr() as libc::c_int == 'y' as i32
        || *buf.as_mut_ptr() as libc::c_int == 'Y' as i32
        || def as libc::c_int == 1 as libc::c_int
            && *buf.as_mut_ptr() as libc::c_int == '\0' as i32
    {
        return 1 as libc::c_int != 0
    } else {
        return 0 as libc::c_int != 0
    };
}
