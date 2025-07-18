use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strlcat(_: *mut libc::c_char, _: *const libc::c_char, _: size_t) -> size_t;
    fn strlcpy(_: *mut libc::c_char, _: *const libc::c_char, _: size_t) -> size_t;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn tcp_connect(
        tcp: *mut tcp_t,
        hostname: *mut libc::c_char,
        port: libc::c_int,
        secure: libc::c_int,
        local_if: *mut libc::c_char,
        io_timeout: libc::c_uint,
    ) -> libc::c_int;
    fn tcp_close(tcp: *mut tcp_t);
    fn tcp_read(
        tcp: *mut tcp_t,
        buffer: *mut libc::c_void,
        size: libc::c_int,
    ) -> ssize_t;
    fn tcp_write(
        tcp: *mut tcp_t,
        buffer: *mut libc::c_void,
        size: libc::c_int,
    ) -> ssize_t;
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
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
pub type ssize_t = __ssize_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tcp_t {
    pub fd: libc::c_int,
    pub ai_family: sa_family_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ftp_t {
    pub cwd: [libc::c_char; 1024],
    pub message: *mut libc::c_char,
    pub status: libc::c_int,
    pub tcp: tcp_t,
    pub data_tcp: tcp_t,
    pub proto: libc::c_int,
    pub ftp_mode: libc::c_int,
    pub local_if: *mut libc::c_char,
}
pub unsafe extern "C" fn ftp_connect(
    mut conn: *mut ftp_t,
    mut proto: libc::c_int,
    mut host: *mut libc::c_char,
    mut port: libc::c_int,
    mut user: *mut libc::c_char,
    mut pass: *mut libc::c_char,
    mut io_timeout: libc::c_uint,
) -> libc::c_int {
    (*conn).data_tcp.fd = -(1 as libc::c_int);
    (*conn).message = malloc(1024 as libc::c_int as size_t) as *mut libc::c_char;
    if ((*conn).message).is_null() {
        return 0 as libc::c_int;
    }
    (*conn).proto = proto;
    if tcp_connect(
        &mut (*conn).tcp,
        host,
        port,
        ((*conn).proto & (1 as libc::c_int) << 0 as libc::c_int
            == (1 as libc::c_int) << 0 as libc::c_int) as libc::c_int,
        (*conn).local_if,
        io_timeout,
    ) == -(1 as libc::c_int)
    {
        return 0 as libc::c_int;
    }
    if ftp_wait(conn) / 100 as libc::c_int != 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    ftp_command(conn, b"USER %s\0" as *const u8 as *const libc::c_char, user);
    if ftp_wait(conn) / 100 as libc::c_int != 2 as libc::c_int {
        if (*conn).status / 100 as libc::c_int == 3 as libc::c_int {
            ftp_command(conn, b"PASS %s\0" as *const u8 as *const libc::c_char, pass);
            if ftp_wait(conn) / 100 as libc::c_int != 2 as libc::c_int {
                return 0 as libc::c_int;
            }
        } else {
            return 0 as libc::c_int
        }
    }
    ftp_command(conn, b"TYPE I\0" as *const u8 as *const libc::c_char);
    if ftp_wait(conn) / 100 as libc::c_int != 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn ftp_disconnect(mut conn: *mut ftp_t) {
    tcp_close(&mut (*conn).tcp);
    tcp_close(&mut (*conn).data_tcp);
    if !((*conn).message).is_null() {
        free((*conn).message as *mut libc::c_void);
        (*conn).message = 0 as *mut libc::c_char;
    }
    *((*conn).cwd).as_mut_ptr() = 0 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn ftp_cwd(
    mut conn: *mut ftp_t,
    mut cwd: *mut libc::c_char,
) -> libc::c_int {
    if strncmp(((*conn).cwd).as_mut_ptr(), cwd, 1024 as libc::c_int as size_t)
        == 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    ftp_command(conn, b"CWD %s\0" as *const u8 as *const libc::c_char, cwd);
    if ftp_wait(conn) / 100 as libc::c_int != 2 as libc::c_int {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Can't change directory to %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            cwd,
        );
        return 0 as libc::c_int;
    }
    strlcpy(
        ((*conn).cwd).as_mut_ptr(),
        cwd,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn ftp_size(
    mut conn: *mut ftp_t,
    mut file: *mut libc::c_char,
    mut maxredir: libc::c_int,
    mut io_timeout: libc::c_uint,
) -> off_t {
    let mut i: off_t = 0;
    let mut j: off_t = 0;
    let mut size: off_t = 1024 as libc::c_int as size_t as off_t;
    let mut reply: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fn_0: [libc::c_char; 1024] = [0; 1024];
    if (strchr(file, '*' as i32)).is_null() && (strchr(file, '?' as i32)).is_null() {
        ftp_command(conn, b"SIZE %s\0" as *const u8 as *const libc::c_char, file);
        if ftp_wait(conn) / 100 as libc::c_int == 2 as libc::c_int {
            sscanf(
                (*conn).message,
                b"%*i %jd\0" as *const u8 as *const libc::c_char,
                &mut i as *mut off_t,
            );
            return i;
        } else if (*conn).status / 10 as libc::c_int != 50 as libc::c_int {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"File not found.\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return -(1 as libc::c_int) as off_t;
        }
    }
    if maxredir == 0 as libc::c_int {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Too many redirects.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return -(1 as libc::c_int) as off_t;
    }
    if ftp_data(conn, io_timeout) == 0 {
        return -(1 as libc::c_int) as off_t;
    }
    ftp_command(conn, b"LIST %s\0" as *const u8 as *const libc::c_char, file);
    if ftp_wait(conn) / 100 as libc::c_int != 1 as libc::c_int {
        return -(1 as libc::c_int) as off_t;
    }
    reply = calloc(1 as libc::c_int as libc::c_ulong, size as libc::c_ulong)
        as *mut libc::c_char;
    if reply.is_null() {
        return -(1 as libc::c_int) as off_t;
    }
    *reply = '\n' as i32 as libc::c_char;
    i = 1 as libc::c_int as off_t;
    loop {
        j = tcp_read(
            &mut (*conn).data_tcp,
            reply.offset(i as isize) as *mut libc::c_void,
            (size - i - 3 as libc::c_int as libc::c_long) as libc::c_int,
        );
        if !(j > 0 as libc::c_int as libc::c_long) {
            break;
        }
        i += j;
        *reply.offset(i as isize) = 0 as libc::c_int as libc::c_char;
        if size - i <= 10 as libc::c_int as libc::c_long {
            size *= 2 as libc::c_int as libc::c_long;
            let mut tmp: *mut libc::c_char = realloc(
                reply as *mut libc::c_void,
                size as libc::c_ulong,
            ) as *mut libc::c_char;
            if tmp.is_null() {
                free(reply as *mut libc::c_void);
                return -(1 as libc::c_int) as off_t;
            }
            reply = tmp;
            memset(
                reply.offset((size / 2 as libc::c_int as libc::c_long) as isize)
                    as *mut libc::c_void,
                0 as libc::c_int,
                (size / 2 as libc::c_int as libc::c_long) as libc::c_ulong,
            );
        }
    }
    tcp_close(&mut (*conn).data_tcp);
    if ftp_wait(conn) / 100 as libc::c_int != 2 as libc::c_int {
        free(reply as *mut libc::c_void);
        return -(1 as libc::c_int) as off_t;
    }
    j = 0 as libc::c_int as off_t;
    i = 1 as libc::c_int as off_t;
    while *reply.offset(i as isize) as libc::c_int != 0
        && *reply.offset((i + 1 as libc::c_int as libc::c_long) as isize) as libc::c_int
            != 0
    {
        if *reply.offset(i as isize) as libc::c_int == '-' as i32
            || *reply.offset(i as isize) as libc::c_int == 'l' as i32
        {
            j += 1;
            j;
        } else {
            while *reply.offset(i as isize) as libc::c_int != '\n' as i32
                && *reply.offset(i as isize) as libc::c_int != 0
            {
                i += 1;
                i;
            }
        }
        i += 1;
        i;
    }
    if j != 1 as libc::c_int as libc::c_long {
        if j == 0 as libc::c_int as libc::c_long {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"File not found.\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Multiple matches for this URL.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        free(reply as *mut libc::c_void);
        return -(1 as libc::c_int) as off_t;
    }
    s = strstr(reply, b"\nl\0" as *const u8 as *const libc::c_char);
    if !s.is_null() {
        sscanf(
            s,
            b"%*s %*i %*s %*s %*i %*s %*i %*s %100s\0" as *const u8
                as *const libc::c_char,
            fn_0.as_mut_ptr(),
        );
        strcpy(file, fn_0.as_mut_ptr());
        strlcpy(
            fn_0.as_mut_ptr(),
            (strstr(s, b"->\0" as *const u8 as *const libc::c_char))
                .offset(3 as libc::c_int as isize),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        fn_0[(::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = '\0' as i32 as libc::c_char;
        free(reply as *mut libc::c_void);
        reply = strchr(fn_0.as_mut_ptr(), '\r' as i32);
        if !reply.is_null() {
            *reply = 0 as libc::c_int as libc::c_char;
        }
        reply = strchr(fn_0.as_mut_ptr(), '\n' as i32);
        if !reply.is_null() {
            *reply = 0 as libc::c_int as libc::c_char;
        }
        return ftp_size(
            conn,
            fn_0.as_mut_ptr(),
            maxredir - 1 as libc::c_int,
            io_timeout,
        );
    } else {
        s = strstr(reply, b"\n-\0" as *const u8 as *const libc::c_char);
        i = sscanf(
            s,
            b"%*s %*i %*s %*s %jd %*s %*i %*s %100s\0" as *const u8
                as *const libc::c_char,
            &mut size as *mut off_t,
            fn_0.as_mut_ptr(),
        ) as off_t;
        if i < 2 as libc::c_int as libc::c_long {
            i = sscanf(
                s,
                b"%*s %*i %jd %*i %*s %*i %*i %100s\0" as *const u8
                    as *const libc::c_char,
                &mut size as *mut off_t,
                fn_0.as_mut_ptr(),
            ) as off_t;
            if i < 2 as libc::c_int as libc::c_long {
                return -(2 as libc::c_int) as off_t;
            }
        }
        strcpy(file, fn_0.as_mut_ptr());
        free(reply as *mut libc::c_void);
        return size;
    };
}
pub unsafe extern "C" fn ftp_data(
    mut conn: *mut ftp_t,
    mut io_timeout: libc::c_uint,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut info: [libc::c_int; 6] = [0; 6];
    let mut host: [libc::c_char; 1024] = [0; 1024];
    if (*conn).data_tcp.fd > 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    ftp_command(conn, b"PASV\0" as *const u8 as *const libc::c_char);
    if ftp_wait(conn) / 100 as libc::c_int != 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    *host.as_mut_ptr() = 0 as libc::c_int as libc::c_char;
    i = 0 as libc::c_int;
    while *((*conn).message).offset(i as isize) != 0 {
        if sscanf(
            &mut *((*conn).message).offset(i as isize) as *mut libc::c_char,
            b"%i,%i,%i,%i,%i,%i\0" as *const u8 as *const libc::c_char,
            &mut *info.as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *info.as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *info.as_mut_ptr().offset(2 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *info.as_mut_ptr().offset(3 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *info.as_mut_ptr().offset(4 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *info.as_mut_ptr().offset(5 as libc::c_int as isize) as *mut libc::c_int,
        ) == 6 as libc::c_int
        {
            snprintf(
                host.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                b"%i.%i.%i.%i\0" as *const u8 as *const libc::c_char,
                info[0 as libc::c_int as usize],
                info[1 as libc::c_int as usize],
                info[2 as libc::c_int as usize],
                info[3 as libc::c_int as usize],
            );
            break;
        } else {
            i += 1;
            i;
        }
    }
    if *host.as_mut_ptr() == 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Error opening passive data connection.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int;
    }
    if tcp_connect(
        &mut (*conn).data_tcp,
        host.as_mut_ptr(),
        info[4 as libc::c_int as usize] * 256 as libc::c_int
            + info[5 as libc::c_int as usize],
        ((*conn).proto & (1 as libc::c_int) << 0 as libc::c_int
            == (1 as libc::c_int) << 0 as libc::c_int) as libc::c_int,
        (*conn).local_if,
        io_timeout,
    ) == -(1 as libc::c_int)
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn ftp_command(
    mut conn: *mut ftp_t,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut params: ::std::ffi::VaListImpl;
    let mut cmd: [libc::c_char; 1024] = [0; 1024];
    params = args.clone();
    vsnprintf(
        cmd.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(3 as libc::c_int as libc::c_ulong),
        format,
        params.as_va_list(),
    );
    strlcat(
        cmd.as_mut_ptr(),
        b"\r\n\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    if tcp_write(
        &mut (*conn).tcp,
        cmd.as_mut_ptr() as *mut libc::c_void,
        strlen(cmd.as_mut_ptr()) as libc::c_int,
    ) != strlen(cmd.as_mut_ptr()) as ssize_t
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Error writing command %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            cmd.as_mut_ptr(),
        );
        return 0 as libc::c_int;
    } else {
        return 1 as libc::c_int
    };
}
pub unsafe extern "C" fn ftp_wait(mut conn: *mut ftp_t) -> libc::c_int {
    let mut size: libc::c_int = 1024 as libc::c_int as size_t as libc::c_int;
    let mut r: libc::c_int = 0 as libc::c_int;
    let mut complete: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut new_msg: *mut libc::c_void = realloc(
        (*conn).message as *mut libc::c_void,
        size as libc::c_ulong,
    );
    if new_msg.is_null() {
        return -(1 as libc::c_int);
    }
    (*conn).message = new_msg as *mut libc::c_char;
    loop {
        loop {
            i = tcp_read(
                &mut (*conn).tcp,
                ((*conn).message).offset(r as isize) as *mut libc::c_void,
                1 as libc::c_int,
            ) as libc::c_int;
            r += i;
            if i <= 0 as libc::c_int {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Connection gone.\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return -(1 as libc::c_int);
            }
            if r + 10 as libc::c_int >= size {
                size = (size as libc::c_ulong)
                    .wrapping_add(1024 as libc::c_int as size_t) as libc::c_int
                    as libc::c_int;
                let mut new_msg_0: *mut libc::c_void = realloc(
                    (*conn).message as *mut libc::c_void,
                    size as libc::c_ulong,
                );
                if new_msg_0.is_null() {
                    return -(1 as libc::c_int);
                }
                (*conn).message = new_msg_0 as *mut libc::c_char;
            }
            if !(*((*conn).message).offset((r - 1 as libc::c_int) as isize)
                as libc::c_int != '\n' as i32)
            {
                break;
            }
        }
        *((*conn).message).offset(r as isize) = 0 as libc::c_int as libc::c_char;
        sscanf(
            (*conn).message,
            b"%i\0" as *const u8 as *const libc::c_char,
            &mut (*conn).status as *mut libc::c_int,
        );
        if *((*conn).message).offset(3 as libc::c_int as isize) as libc::c_int
            == ' ' as i32
        {
            complete = 1 as libc::c_int;
        } else {
            complete = 0 as libc::c_int;
        }
        i = 0 as libc::c_int;
        while *((*conn).message).offset(i as isize) != 0 {
            if *((*conn).message).offset(i as isize) as libc::c_int == '\n' as i32 {
                if complete == 1 as libc::c_int {
                    complete = 2 as libc::c_int;
                    break;
                } else if *((*conn).message).offset((i + 4 as libc::c_int) as isize)
                    as libc::c_int == ' ' as i32
                {
                    j = -(1 as libc::c_int);
                    sscanf(
                        &mut *((*conn).message).offset((i + 1 as libc::c_int) as isize)
                            as *mut libc::c_char,
                        b"%3i\0" as *const u8 as *const libc::c_char,
                        &mut j as *mut libc::c_int,
                    );
                    if j == (*conn).status {
                        complete = 1 as libc::c_int;
                    }
                }
            }
            i += 1;
            i;
        }
        if !(complete != 2 as libc::c_int) {
            break;
        }
    }
    let mut k: libc::c_int = strcspn(
        (*conn).message,
        b"\r\n\0" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    *((*conn).message).offset(k as isize) = 0 as libc::c_int as libc::c_char;
    (*conn)
        .message = realloc(
        (*conn).message as *mut libc::c_void,
        (k + 1 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_char;
    return (*conn).status;
}
