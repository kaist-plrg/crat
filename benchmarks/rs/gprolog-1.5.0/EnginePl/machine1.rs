use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn fork() -> __pid_t;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn uname(__name: *mut utsname) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
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
pub type __pid_t = libc::c_int;
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uintptr_t = libc::c_ulong;
pub type PlULong = uintptr_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub __domainname: [libc::c_char; 65],
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
pub static mut pl_m_os_type: libc::c_int = 0;
pub static mut pl_m_architecture: [libc::c_char; 32] = [0; 32];
pub static mut pl_m_os_version: [libc::c_char; 256] = [0; 256];
pub unsafe extern "C" fn Pl_Init_Machine1() {
    let mut uname_info: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        __domainname: [0; 65],
    };
    pl_m_os_type = 0 as libc::c_int;
    if uname(&mut uname_info) < 0 as libc::c_int {
        strcpy(
            pl_m_architecture.as_mut_ptr(),
            b"unknown architecture\0" as *const u8 as *const libc::c_char,
        );
        strcpy(
            pl_m_os_version.as_mut_ptr(),
            b"unknown OS version\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    strcpy(pl_m_architecture.as_mut_ptr(), (uname_info.machine).as_mut_ptr());
    sprintf(
        pl_m_os_version.as_mut_ptr(),
        b"%s %s\0" as *const u8 as *const libc::c_char,
        (uname_info.sysname).as_mut_ptr(),
        (uname_info.release).as_mut_ptr(),
    );
}
pub unsafe extern "C" fn Pl_M_Create_Shell_Command(
    mut cmd: *mut libc::c_char,
) -> *mut *mut libc::c_char {
    static mut arg: [*mut libc::c_char; 4] = [0 as *const libc::c_char
        as *mut libc::c_char; 4];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = getenv(b"SHELL\0" as *const u8 as *const libc::c_char);
    arg[0 as libc::c_int
        as usize] = (if !p.is_null() {
        p as *const libc::c_char
    } else {
        b"/bin/sh\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    arg[1 as libc::c_int
        as usize] = b"-c\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if !cmd.is_null() {
        arg[2 as libc::c_int as usize] = cmd;
        arg[3 as libc::c_int as usize] = 0 as *mut libc::c_char;
    } else {
        arg[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
    }
    return arg.as_mut_ptr();
}
pub unsafe extern "C" fn Pl_M_Cmd_Line_To_Argv(
    mut cmd: *mut libc::c_char,
    mut argc: *mut libc::c_int,
) -> *mut *mut libc::c_char {
    static mut arg: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
        as *mut *mut libc::c_char;
    static mut nb_arg: libc::c_int = 0 as libc::c_int;
    let mut p: *mut libc::c_char = cmd;
    let mut i: libc::c_int = 0 as libc::c_int;
    loop {
        while *p as libc::c_int == ' ' as i32 || *p as libc::c_int == '\t' as i32 {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int == '\0' as i32 {
            break;
        }
        if i >= nb_arg {
            nb_arg += 64 as libc::c_int;
            arg = (if arg.is_null() {
                malloc(
                    (nb_arg as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                )
            } else {
                realloc(
                    arg as *mut libc::c_void,
                    (nb_arg as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                )
            }) as *mut *mut libc::c_char;
        }
        let fresh0 = i;
        i = i + 1;
        let ref mut fresh1 = *arg.offset(fresh0 as isize);
        *fresh1 = p;
        while *p as libc::c_int != ' ' as i32 && *p as libc::c_int != '\t' as i32
            && *p as libc::c_int != '\0' as i32
        {
            if *p as libc::c_int == '"' as i32 {
                loop {
                    p = p.offset(1);
                    p;
                    if !(*p as libc::c_int != '"' as i32
                        && *p as libc::c_int != '\0' as i32)
                    {
                        break;
                    }
                }
                if *p as libc::c_int == '"' as i32 {
                    p = p.offset(1);
                    p;
                }
            } else {
                p = p.offset(1);
                p;
            }
        }
        if *p as libc::c_int == '\0' as i32 {
            break;
        }
        let fresh2 = p;
        p = p.offset(1);
        *fresh2 = '\0' as i32 as libc::c_char;
    }
    let ref mut fresh3 = *arg.offset(i as isize);
    *fresh3 = 0 as *mut libc::c_char;
    if !argc.is_null() {
        *argc = i;
    }
    return arg;
}
pub unsafe extern "C" fn Pl_M_Shell(mut cmd: *mut libc::c_char) -> libc::c_int {
    return Pl_M_Spawn(Pl_M_Create_Shell_Command(cmd));
}
pub unsafe extern "C" fn Pl_M_Spawn(mut arg: *mut *mut libc::c_char) -> libc::c_int {
    let mut pid: libc::c_int = 0;
    fflush(stdout);
    fflush(stderr);
    if *arg.offset(1 as libc::c_int as isize) == 1 as libc::c_int as *mut libc::c_char {
        arg = Pl_M_Cmd_Line_To_Argv(
            *arg.offset(0 as libc::c_int as isize),
            0 as *mut libc::c_int,
        );
    }
    pid = fork();
    if pid == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if pid == 0 as libc::c_int {
        execvp(*arg.offset(0 as libc::c_int as isize), arg as *const *mut libc::c_char);
        exit(
            if *__errno_location() == 2 as libc::c_int
                || *__errno_location() == 20 as libc::c_int
            {
                126 as libc::c_int
            } else {
                127 as libc::c_int
            },
        );
    }
    return Pl_M_Get_Status(pid);
}
pub unsafe extern "C" fn Pl_M_Spawn_Redirect(
    mut arg: *mut *mut libc::c_char,
    mut detach: libc::c_int,
    mut f_in: *mut *mut FILE,
    mut f_out: *mut *mut FILE,
    mut f_err: *mut *mut FILE,
) -> libc::c_int {
    let mut current_block: u64;
    let mut pipe_in: [libc::c_int; 2] = [0; 2];
    let mut pipe_out: [libc::c_int; 2] = [0; 2];
    let mut pipe_err: [libc::c_int; 2] = [0; 2];
    let mut pid: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    fflush(stdout);
    fflush(stderr);
    if *arg.offset(1 as libc::c_int as isize) == 1 as libc::c_int as *mut libc::c_char {
        arg = Pl_M_Cmd_Line_To_Argv(
            *arg.offset(0 as libc::c_int as isize),
            0 as *mut libc::c_int,
        );
    }
    if !(!f_in.is_null() && pipe(pipe_in.as_mut_ptr()) != 0
        || !f_out.is_null() && pipe(pipe_out.as_mut_ptr()) != 0
        || !f_err.is_null() && f_err != f_out && pipe(pipe_err.as_mut_ptr()) != 0)
    {
        pid = fork();
        if !(pid == -(1 as libc::c_int)) {
            if pid == 0 as libc::c_int {
                if detach == 0 || fork() == 0 as libc::c_int {
                    if !(!f_in.is_null()
                        && (close(pipe_in[1 as libc::c_int as usize]) != 0
                            || pipe_in[0 as libc::c_int as usize] != 0 as libc::c_int
                                && (dup2(
                                    pipe_in[0 as libc::c_int as usize],
                                    0 as libc::c_int,
                                ) == -(1 as libc::c_int)
                                    || close(pipe_in[0 as libc::c_int as usize]) != 0)))
                    {
                        if !(!f_out.is_null()
                            && (close(pipe_out[0 as libc::c_int as usize]) != 0
                                || pipe_out[1 as libc::c_int as usize] != 1 as libc::c_int
                                    && (dup2(
                                        pipe_out[1 as libc::c_int as usize],
                                        1 as libc::c_int,
                                    ) == -(1 as libc::c_int)
                                        || close(pipe_out[1 as libc::c_int as usize]) != 0)))
                        {
                            if !f_err.is_null() {
                                if f_err != f_out {
                                    if close(pipe_err[0 as libc::c_int as usize]) != 0
                                        || pipe_err[1 as libc::c_int as usize] != 2 as libc::c_int
                                            && (dup2(
                                                pipe_err[1 as libc::c_int as usize],
                                                2 as libc::c_int,
                                            ) == -(1 as libc::c_int)
                                                || close(pipe_err[1 as libc::c_int as usize]) != 0)
                                    {
                                        current_block = 16676627605025757181;
                                    } else {
                                        current_block = 12349973810996921269;
                                    }
                                } else if dup2(1 as libc::c_int, 2 as libc::c_int)
                                    == -(1 as libc::c_int)
                                {
                                    current_block = 16676627605025757181;
                                } else {
                                    current_block = 12349973810996921269;
                                }
                            } else {
                                current_block = 12349973810996921269;
                            }
                            match current_block {
                                16676627605025757181 => {}
                                _ => {
                                    execvp(
                                        *arg.offset(0 as libc::c_int as isize),
                                        arg as *const *mut libc::c_char,
                                    );
                                    exit(
                                        if *__errno_location() == 2 as libc::c_int
                                            || *__errno_location() == 20 as libc::c_int
                                        {
                                            126 as libc::c_int
                                        } else {
                                            127 as libc::c_int
                                        },
                                    );
                                }
                            }
                        }
                    }
                } else {
                    exit(0 as libc::c_int);
                }
            } else {
                if detach != 0 {
                    if waitpid(pid, &mut status, 0 as libc::c_int) < 0 as libc::c_int {
                        current_block = 16676627605025757181;
                    } else {
                        pid = 0 as libc::c_int;
                        current_block = 4808432441040389987;
                    }
                } else {
                    current_block = 4808432441040389987;
                }
                match current_block {
                    16676627605025757181 => {}
                    _ => {
                        if !(!f_in.is_null()
                            && (close(pipe_in[0 as libc::c_int as usize]) != 0
                                || {
                                    *f_in = fdopen(
                                        pipe_in[1 as libc::c_int as usize],
                                        b"wt\0" as *const u8 as *const libc::c_char,
                                    );
                                    (*f_in).is_null()
                                }))
                        {
                            if !(!f_out.is_null()
                                && (close(pipe_out[1 as libc::c_int as usize]) != 0
                                    || {
                                        *f_out = fdopen(
                                            pipe_out[0 as libc::c_int as usize],
                                            b"rt\0" as *const u8 as *const libc::c_char,
                                        );
                                        (*f_out).is_null()
                                    }))
                            {
                                if !(!f_err.is_null() && f_err != f_out
                                    && (close(pipe_err[1 as libc::c_int as usize]) != 0
                                        || {
                                            *f_err = fdopen(
                                                pipe_err[0 as libc::c_int as usize],
                                                b"rt\0" as *const u8 as *const libc::c_char,
                                            );
                                            (*f_err).is_null()
                                        }))
                                {
                                    return pid;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn Pl_M_Get_Status(mut pid: libc::c_int) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    if waitpid(pid, &mut status, 0 as libc::c_int) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if status & 0x7f as libc::c_int == 0 as libc::c_int {
        status = (status & 0xff00 as libc::c_int) >> 8 as libc::c_int;
        if status == 127 as libc::c_int {
            status = -(2 as libc::c_int);
        } else if status == 126 as libc::c_int {
            status = -(1 as libc::c_int);
            *__errno_location() = 2 as libc::c_int;
        }
    }
    return status;
}
pub unsafe extern "C" fn Pl_M_Mktemp(mut tmpl: *mut libc::c_char) -> *mut libc::c_char {
    let mut len: libc::c_int = 0;
    let mut XXXXXX: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut value: PlULong = 0;
    let mut count: libc::c_int = 0;
    let mut buf: stat = stat {
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
    static mut letters: [libc::c_char; 63] = unsafe {
        *::std::mem::transmute::<
            &[u8; 63],
            &[libc::c_char; 63],
        >(b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789\0")
    };
    len = strlen(tmpl) as libc::c_int;
    if len < 6 as libc::c_int
        || strcmp(
            &mut *tmpl.offset((len - 6 as libc::c_int) as isize),
            b"XXXXXX\0" as *const u8 as *const libc::c_char,
        ) != 0
    {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut libc::c_char;
    }
    XXXXXX = &mut *tmpl.offset((len - 6 as libc::c_int) as isize) as *mut libc::c_char;
    value = (value as libc::c_ulong)
        .wrapping_add(time(0 as *mut time_t) as PlULong ^ getpid() as libc::c_ulong)
        as PlULong as PlULong;
    count = 0 as libc::c_int;
    while count < 238328 as libc::c_int {
        let mut v: PlULong = value;
        *XXXXXX
            .offset(
                0 as libc::c_int as isize,
            ) = letters[v.wrapping_rem(62 as libc::c_int as libc::c_ulong) as usize];
        v = (v as libc::c_ulong).wrapping_div(62 as libc::c_int as libc::c_ulong)
            as PlULong as PlULong;
        *XXXXXX
            .offset(
                1 as libc::c_int as isize,
            ) = letters[v.wrapping_rem(62 as libc::c_int as libc::c_ulong) as usize];
        v = (v as libc::c_ulong).wrapping_div(62 as libc::c_int as libc::c_ulong)
            as PlULong as PlULong;
        *XXXXXX
            .offset(
                2 as libc::c_int as isize,
            ) = letters[v.wrapping_rem(62 as libc::c_int as libc::c_ulong) as usize];
        v = (v as libc::c_ulong).wrapping_div(62 as libc::c_int as libc::c_ulong)
            as PlULong as PlULong;
        *XXXXXX
            .offset(
                3 as libc::c_int as isize,
            ) = letters[v.wrapping_rem(62 as libc::c_int as libc::c_ulong) as usize];
        v = (v as libc::c_ulong).wrapping_div(62 as libc::c_int as libc::c_ulong)
            as PlULong as PlULong;
        *XXXXXX
            .offset(
                4 as libc::c_int as isize,
            ) = letters[v.wrapping_rem(62 as libc::c_int as libc::c_ulong) as usize];
        v = (v as libc::c_ulong).wrapping_div(62 as libc::c_int as libc::c_ulong)
            as PlULong as PlULong;
        *XXXXXX
            .offset(
                5 as libc::c_int as isize,
            ) = letters[v.wrapping_rem(62 as libc::c_int as libc::c_ulong) as usize];
        if lstat(tmpl, &mut buf) < 0 as libc::c_int {
            if *__errno_location() == 2 as libc::c_int {
                *__errno_location() = 0 as libc::c_int;
                return tmpl;
            } else {
                return 0 as *mut libc::c_char
            }
        }
        value = (value as libc::c_ulong)
            .wrapping_add(7777 as libc::c_int as libc::c_ulong) as PlULong as PlULong;
        count += 1;
        count;
    }
    *__errno_location() = 17 as libc::c_int;
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn Pl_M_Tempnam(
    mut dir: *mut libc::c_char,
    mut pfx: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut tmpl: [libc::c_char; 4096] = [0; 4096];
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dlen: libc::c_int = 0;
    let mut plen: libc::c_int = 0;
    let mut buf: stat = stat {
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
    if pfx.is_null() || *pfx.offset(0 as libc::c_int as isize) == 0 {
        pfx = b"file\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        plen = 4 as libc::c_int;
    } else {
        plen = strlen(pfx) as libc::c_int;
        if plen > 5 as libc::c_int {
            plen = 5 as libc::c_int;
        }
    }
    d = getenv(b"TMPDIR\0" as *const u8 as *const libc::c_char);
    if !d.is_null()
        && (stat(d, &mut buf) == 0 as libc::c_int
            && buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint)
    {
        dir = d;
    } else if !(!dir.is_null()
        && (stat(dir, &mut buf) == 0 as libc::c_int
            && buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint))
    {
        dir = 0 as *mut libc::c_char;
    }
    if dir.is_null() {
        if stat(b"/tmp\0" as *const u8 as *const libc::c_char, &mut buf)
            == 0 as libc::c_int
            && buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint
        {
            dir = b"/tmp\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if strcmp(
            b"/tmp\0" as *const u8 as *const libc::c_char,
            b"/tmp\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
            && (stat(b"/tmp\0" as *const u8 as *const libc::c_char, &mut buf)
                == 0 as libc::c_int
                && buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint)
        {
            dir = b"/tmp\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else {
            *__errno_location() = 2 as libc::c_int;
            return 0 as *mut libc::c_char;
        }
    }
    dlen = strlen(dir) as libc::c_int;
    while dlen > 1 as libc::c_int
        && *dir.offset((dlen - 1 as libc::c_int) as isize) as libc::c_int == '/' as i32
    {
        dlen -= 1;
        dlen;
    }
    if (4096 as libc::c_int)
        < dlen + 1 as libc::c_int + plen + 6 as libc::c_int + 1 as libc::c_int
    {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut libc::c_char;
    }
    sprintf(
        tmpl.as_mut_ptr(),
        b"%.*s/%.*sXXXXXX\0" as *const u8 as *const libc::c_char,
        dlen,
        dir,
        plen,
        pfx,
    );
    d = Pl_M_Mktemp(tmpl.as_mut_ptr());
    if !d.is_null() {
        d = strdup(d);
    }
    return d;
}
