use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn __errno_location() -> *mut libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
    pub fds_bits: [__fd_mask; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linebuffer {
    pub buffer: *mut libc::c_char,
    pub length: size_t,
}
pub unsafe extern "C" fn linebuffer_reset(mut lb: *mut linebuffer) -> libc::c_int {
    (*lb).buffer = 0 as *mut libc::c_char;
    (*lb).length = 0 as libc::c_int as size_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn linebuffer_free(mut lb: *mut linebuffer) -> libc::c_int {
    if !((*lb).buffer).is_null() {
        free((*lb).buffer as *mut libc::c_void);
    }
    linebuffer_reset(lb);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn linebuffer_concatenate(
    mut lb: *mut linebuffer,
    mut buff: *mut libc::c_char,
    mut size: size_t,
) -> libc::c_int {
    if buff.is_null() || size <= 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    (*lb)
        .buffer = realloc(
        (*lb).buffer as *mut libc::c_void,
        ((*lb).length).wrapping_add(size).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if ((*lb).buffer).is_null() && 1 as libc::c_int > 0 as libc::c_int {
        fprintf(
            stderr,
            b"linebuffer.c: %s.\n\0" as *const u8 as *const libc::c_char,
            dcgettext(
                0 as *const libc::c_char,
                b"memory exhausted\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        abort();
    }
    memcpy(
        ((*lb).buffer).offset((*lb).length as isize) as *mut libc::c_void,
        buff as *const libc::c_void,
        size,
    );
    (*lb)
        .length = ((*lb).length as libc::c_ulong).wrapping_add(size) as size_t as size_t;
    *((*lb).buffer).offset((*lb).length as isize) = 0 as libc::c_int as libc::c_char;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn linebuffer_read_line(
    mut fd: libc::c_int,
    mut lb: *mut linebuffer,
    mut timeout: libc::c_int,
) -> *mut libc::c_char {
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut eoc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buff: [libc::c_char; 256] = [0; 256];
    let mut length: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut ptv: *mut timeval = 0 as *mut timeval;
    let mut tn: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut set: fd_set = fd_set { fds_bits: [0; 16] };
    if timeout > 0 as libc::c_int {
        tv.tv_sec = timeout as __time_t;
        tv.tv_usec = 0 as libc::c_int as __suseconds_t;
        ptv = &mut tv;
    } else {
        ptv = 0 as *mut timeval;
    }
    loop {
        if !((*lb).buffer).is_null() {
            eoc = memchr(
                (*lb).buffer as *const libc::c_void,
                10 as libc::c_int,
                (*lb).length,
            ) as *mut libc::c_char;
        } else {
            eoc = 0 as *mut libc::c_char;
        }
        if !eoc.is_null() {
            length = eoc.offset_from((*lb).buffer) as libc::c_long as libc::c_int;
            line = malloc((1 as libc::c_int + length) as libc::c_ulong)
                as *mut libc::c_char;
            if line.is_null() && 1 as libc::c_int > 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"linebuffer.c: %s.\n\0" as *const u8 as *const libc::c_char,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"memory exhausted\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                abort();
            }
            memcpy(
                line as *mut libc::c_void,
                (*lb).buffer as *const libc::c_void,
                length as libc::c_ulong,
            );
            *line.offset(length as isize) = 0 as libc::c_int as libc::c_char;
            length += 1;
            length;
            if length as libc::c_ulong >= (*lb).length {
                free((*lb).buffer as *mut libc::c_void);
                (*lb).buffer = 0 as *mut libc::c_char;
                (*lb).length = 0 as libc::c_int as size_t;
            } else {
                memmove(
                    (*lb).buffer as *mut libc::c_void,
                    ((*lb).buffer).offset(length as isize) as *const libc::c_void,
                    ((*lb).length).wrapping_sub(length as libc::c_ulong),
                );
                (*lb)
                    .length = ((*lb).length as libc::c_ulong)
                    .wrapping_sub(length as libc::c_ulong) as size_t as size_t;
            }
            return line;
        }
        let mut __d0: libc::c_int = 0;
        let mut __d1: libc::c_int = 0;
        let fresh0 = &mut __d0;
        let fresh1;
        let fresh2 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh3 = &mut __d1;
        let fresh4;
        let fresh5 = &mut *(set.fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh0,
            fresh2) => fresh1, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh3,
            fresh5) => fresh4, inlateout("ax") 0 as libc::c_int => _,
            options(preserves_flags, att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
        c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
        set
            .fds_bits[(fd
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << fd
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        tn.tv_sec = 0 as libc::c_int as __time_t;
        tn.tv_usec = 0 as libc::c_int as __suseconds_t;
        select(
            fd + 1 as libc::c_int,
            &mut set,
            0 as *mut fd_set,
            0 as *mut fd_set,
            &mut tn,
        );
        if set
            .fds_bits[(fd
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            & ((1 as libc::c_ulong)
                << fd
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask
            != 0 as libc::c_int as libc::c_long
        {
            n = read(
                fd,
                buff.as_mut_ptr() as *mut libc::c_void,
                256 as libc::c_int as size_t,
            ) as libc::c_int;
            if n > 0 as libc::c_int {
                linebuffer_concatenate(lb, buff.as_mut_ptr(), n as size_t);
            } else if !(n < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int)
            {
                return 0 as *mut libc::c_char
            }
        } else {
            let mut __d0_0: libc::c_int = 0;
            let mut __d1_0: libc::c_int = 0;
            let fresh6 = &mut __d0_0;
            let fresh7;
            let fresh8 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
            let fresh9 = &mut __d1_0;
            let fresh10;
            let fresh11 = &mut *(set.fds_bits)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut __fd_mask;
            asm!(
                "cld; rep; stosq", inlateout("cx")
                c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8) => fresh7,
                inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh9, fresh11) =>
                fresh10, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
                att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
            c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
            set
                .fds_bits[(fd
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                |= ((1 as libc::c_ulong)
                    << fd
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask;
            select(
                fd + 1 as libc::c_int,
                &mut set,
                0 as *mut fd_set,
                0 as *mut fd_set,
                ptv,
            );
            if set
                .fds_bits[(fd
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << fd
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
            {
                n = read(
                    fd,
                    buff.as_mut_ptr() as *mut libc::c_void,
                    256 as libc::c_int as size_t,
                ) as libc::c_int;
                if n > 0 as libc::c_int {
                    linebuffer_concatenate(lb, buff.as_mut_ptr(), n as size_t);
                } else if !(n < 0 as libc::c_int
                    && *__errno_location() == 4 as libc::c_int)
                {
                    return 0 as *mut libc::c_char
                }
            } else {
                return 0 as *mut libc::c_char
            }
        }
    };
}
pub unsafe extern "C" fn linebuffer_fetch(mut lb: *mut linebuffer) -> *mut libc::c_char {
    let mut eoc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut length: size_t = 0;
    if !((*lb).buffer).is_null() && (*lb).length > 0 as libc::c_int as libc::c_ulong {
        eoc = memchr(
            (*lb).buffer as *const libc::c_void,
            10 as libc::c_int,
            (*lb).length,
        ) as *mut libc::c_char;
    } else {
        eoc = 0 as *mut libc::c_char;
    }
    if !eoc.is_null() {
        length = eoc.offset_from((*lb).buffer) as libc::c_long as size_t;
        line = malloc((1 as libc::c_int as libc::c_ulong).wrapping_add(length))
            as *mut libc::c_char;
        if line.is_null() && 1 as libc::c_int > 0 as libc::c_int {
            fprintf(
                stderr,
                b"linebuffer.c: %s.\n\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"memory exhausted\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            abort();
        }
        memcpy(line as *mut libc::c_void, (*lb).buffer as *const libc::c_void, length);
        *line.offset(length as isize) = 0 as libc::c_int as libc::c_char;
        length = length.wrapping_add(1);
        length;
        if length >= (*lb).length {
            free((*lb).buffer as *mut libc::c_void);
            (*lb).buffer = 0 as *mut libc::c_char;
            (*lb).length = 0 as libc::c_int as size_t;
        } else {
            memmove(
                (*lb).buffer as *mut libc::c_void,
                ((*lb).buffer).offset(length as isize) as *const libc::c_void,
                ((*lb).length).wrapping_sub(length),
            );
            (*lb)
                .length = ((*lb).length as libc::c_ulong).wrapping_sub(length) as size_t
                as size_t;
        }
        return line;
    } else {
        return 0 as *mut libc::c_char
    };
}
pub unsafe extern "C" fn linebuffer_flush(mut lb: *mut linebuffer) -> *mut libc::c_char {
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut length: size_t = 0;
    if !((*lb).buffer).is_null() && (*lb).length > 0 as libc::c_int as libc::c_ulong {
        line = malloc(((*lb).length).wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        if line.is_null() && 1 as libc::c_int > 0 as libc::c_int {
            fprintf(
                stderr,
                b"linebuffer.c: %s.\n\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"memory exhausted\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            abort();
        }
        length = (*lb).length;
        memcpy(line as *mut libc::c_void, (*lb).buffer as *const libc::c_void, length);
        *line.offset(length as isize) = 0 as libc::c_int as libc::c_char;
        free((*lb).buffer as *mut libc::c_void);
        linebuffer_reset(lb);
        return line;
    } else {
        if !((*lb).buffer).is_null() {
            free((*lb).buffer as *mut libc::c_void);
        }
        linebuffer_reset(lb);
        return 0 as *mut libc::c_char;
    };
}
