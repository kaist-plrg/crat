use ::libc;
extern "C" {
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn rpl_free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execl(
        __path: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getegid() -> __gid_t;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn setgid(__gid: __gid_t) -> libc::c_int;
    fn fork() -> __pid_t;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn abort() -> !;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn initgroups(__user: *const libc::c_char, __group: __gid_t) -> libc::c_int;
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> size_t;
    fn gethostbyname(__name: *const libc::c_char) -> *mut hostent;
}
pub type __intmax_t = libc::c_long;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __daddr_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type intmax_t = __intmax_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtop {
    pub mt_op: libc::c_short,
    pub mt_count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtget {
    pub mt_type: libc::c_long,
    pub mt_resid: libc::c_long,
    pub mt_dsreg: libc::c_long,
    pub mt_gstat: libc::c_long,
    pub mt_erreg: libc::c_long,
    pub mt_fileno: __daddr_t,
    pub mt_blkno: __daddr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
    pub h_name: *mut libc::c_char,
    pub h_aliases: *mut *mut libc::c_char,
    pub h_addrtype: libc::c_int,
    pub h_length: libc::c_int,
    pub h_addr_list: *mut *mut libc::c_char,
}
pub type C2RustUnnamed = libc::c_uint;
pub const countlen: C2RustUnnamed = 11;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const oplen: C2RustUnnamed_0 = 6;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
static mut from_remote: [[libc::c_int; 2]; 4] = [
    [-(1 as libc::c_int), -(1 as libc::c_int)],
    [-(1 as libc::c_int), -(1 as libc::c_int)],
    [-(1 as libc::c_int), -(1 as libc::c_int)],
    [-(1 as libc::c_int), -(1 as libc::c_int)],
];
static mut to_remote: [[libc::c_int; 2]; 4] = [
    [-(1 as libc::c_int), -(1 as libc::c_int)],
    [-(1 as libc::c_int), -(1 as libc::c_int)],
    [-(1 as libc::c_int), -(1 as libc::c_int)],
    [-(1 as libc::c_int), -(1 as libc::c_int)],
];
pub static mut rmt_command: *const libc::c_char = b"/usr/local/libexec/rmt\0"
    as *const u8 as *const libc::c_char;
pub static mut rmt_dev_name__: *const libc::c_char = 0 as *const libc::c_char;
pub static mut force_local_option: bool = false;
unsafe extern "C" fn _rmt_shutdown(
    mut handle: libc::c_int,
    mut errno_value: libc::c_int,
) {
    close(from_remote[handle as usize][0 as libc::c_int as usize]);
    close(to_remote[handle as usize][1 as libc::c_int as usize]);
    from_remote[handle as usize][0 as libc::c_int as usize] = -(1 as libc::c_int);
    to_remote[handle as usize][1 as libc::c_int as usize] = -(1 as libc::c_int);
    *__errno_location() = errno_value;
}
unsafe extern "C" fn do_command(
    mut handle: libc::c_int,
    mut buffer: *const libc::c_char,
) -> libc::c_int {
    let mut length: size_t = strlen(buffer);
    let mut pipe_handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    let mut written: ssize_t = full_write(
        to_remote[handle as usize][1 as libc::c_int as usize],
        buffer as *const libc::c_void,
        length,
    ) as ssize_t;
    signal(13 as libc::c_int, pipe_handler);
    if written as libc::c_ulong == length {
        return 0 as libc::c_int;
    }
    _rmt_shutdown(handle, 5 as libc::c_int);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn get_status_string(
    mut handle: libc::c_int,
    mut command_buffer: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut cursor: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut counter: libc::c_int = 0;
    counter = 0 as libc::c_int;
    cursor = command_buffer;
    while counter < 64 as libc::c_int {
        if safe_read(
            from_remote[handle as usize][0 as libc::c_int as usize],
            cursor as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) != 1 as libc::c_int as libc::c_ulong
        {
            _rmt_shutdown(handle, 5 as libc::c_int);
            return 0 as *mut libc::c_char;
        }
        if *cursor as libc::c_int == '\n' as i32 {
            *cursor = '\0' as i32 as libc::c_char;
            break;
        } else {
            counter += 1;
            counter;
            cursor = cursor.offset(1);
            cursor;
        }
    }
    if counter == 64 as libc::c_int {
        _rmt_shutdown(handle, 5 as libc::c_int);
        return 0 as *mut libc::c_char;
    }
    cursor = command_buffer;
    while *cursor != 0 {
        if *cursor as libc::c_int != ' ' as i32 {
            break;
        }
        cursor = cursor.offset(1);
        cursor;
    }
    if *cursor as libc::c_int == 'E' as i32 || *cursor as libc::c_int == 'F' as i32 {
        let mut character: libc::c_char = 0;
        while safe_read(
            from_remote[handle as usize][0 as libc::c_int as usize],
            &mut character as *mut libc::c_char as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) == 1 as libc::c_int as libc::c_ulong
        {
            if character as libc::c_int == '\n' as i32 {
                break;
            }
        }
        *__errno_location() = atoi(cursor.offset(1 as libc::c_int as isize));
        if *cursor as libc::c_int == 'F' as i32 {
            _rmt_shutdown(handle, *__errno_location());
        }
        return 0 as *mut libc::c_char;
    }
    if *cursor as libc::c_int != 'A' as i32 {
        _rmt_shutdown(handle, 5 as libc::c_int);
        return 0 as *mut libc::c_char;
    }
    return cursor.offset(1 as libc::c_int as isize);
}
unsafe extern "C" fn get_status(mut handle: libc::c_int) -> libc::c_long {
    let mut command_buffer: [libc::c_char; 64] = [0; 64];
    let mut status: *const libc::c_char = get_status_string(
        handle,
        command_buffer.as_mut_ptr(),
    );
    if !status.is_null() {
        let mut result: libc::c_long = atol(status);
        if 0 as libc::c_int as libc::c_long <= result {
            return result;
        }
        *__errno_location() = 5 as libc::c_int;
    }
    return -(1 as libc::c_int) as libc::c_long;
}
unsafe extern "C" fn get_status_off(mut handle: libc::c_int) -> off_t {
    let mut command_buffer: [libc::c_char; 64] = [0; 64];
    let mut status: *const libc::c_char = get_status_string(
        handle,
        command_buffer.as_mut_ptr(),
    );
    if status.is_null() {
        return -(1 as libc::c_int) as off_t
    } else {
        let mut count: off_t = 0 as libc::c_int as off_t;
        let mut negative: libc::c_int = 0;
        while *status as libc::c_int == ' ' as i32
            || *status as libc::c_int == '\t' as i32
        {
            status = status.offset(1);
            status;
        }
        negative = (*status as libc::c_int == '-' as i32) as libc::c_int;
        status = status
            .offset(
                (negative != 0 || *status as libc::c_int == '+' as i32) as libc::c_int
                    as isize,
            );
        loop {
            let fresh0 = status;
            status = status.offset(1);
            let mut digit: libc::c_int = *fresh0 as libc::c_int - '0' as i32;
            if (9 as libc::c_int as libc::c_uint) < digit as libc::c_uint {
                break;
            }
            let mut c10: off_t = 10 as libc::c_int as libc::c_long * count;
            let mut nc: off_t = if negative != 0 {
                c10 - digit as libc::c_long
            } else {
                c10 + digit as libc::c_long
            };
            if c10 / 10 as libc::c_int as libc::c_long != count
                || (if negative != 0 {
                    (c10 < nc) as libc::c_int
                } else {
                    (nc < c10) as libc::c_int
                }) != 0
            {
                return -(1 as libc::c_int) as off_t;
            }
            count = nc;
        }
        return count;
    };
}
unsafe extern "C" fn encode_oflag(mut buf: *mut libc::c_char, mut oflag: libc::c_int) {
    sprintf(buf, b"%d \0" as *const u8 as *const libc::c_char, oflag);
    match oflag & 0o3 as libc::c_int {
        0 => {
            strcat(buf, b"O_RDONLY\0" as *const u8 as *const libc::c_char);
        }
        2 => {
            strcat(buf, b"O_RDWR\0" as *const u8 as *const libc::c_char);
        }
        1 => {
            strcat(buf, b"O_WRONLY\0" as *const u8 as *const libc::c_char);
        }
        _ => {
            abort();
        }
    }
    if oflag & 0o2000 as libc::c_int != 0 {
        strcat(buf, b"|O_APPEND\0" as *const u8 as *const libc::c_char);
    }
    if oflag & 0o100 as libc::c_int != 0 {
        strcat(buf, b"|O_CREAT\0" as *const u8 as *const libc::c_char);
    }
    if oflag & 0o10000 as libc::c_int != 0 {
        strcat(buf, b"|O_DSYNC\0" as *const u8 as *const libc::c_char);
    }
    if oflag & 0o200 as libc::c_int != 0 {
        strcat(buf, b"|O_EXCL\0" as *const u8 as *const libc::c_char);
    }
    if oflag & 0 as libc::c_int != 0 {
        strcat(buf, b"|O_LARGEFILE\0" as *const u8 as *const libc::c_char);
    }
    if oflag & 0o400 as libc::c_int != 0 {
        strcat(buf, b"|O_NOCTTY\0" as *const u8 as *const libc::c_char);
    }
    if oflag & 0o4000 as libc::c_int != 0 {
        strcat(buf, b"|O_NONBLOCK\0" as *const u8 as *const libc::c_char);
    }
    if oflag & 0o4010000 as libc::c_int != 0 {
        strcat(buf, b"|O_RSYNC\0" as *const u8 as *const libc::c_char);
    }
    if oflag & 0o4010000 as libc::c_int != 0 {
        strcat(buf, b"|O_SYNC\0" as *const u8 as *const libc::c_char);
    }
    if oflag & 0o1000 as libc::c_int != 0 {
        strcat(buf, b"|O_TRUNC\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn sys_reset_uid_gid() -> *const libc::c_char {
    let mut uid: uid_t = getuid();
    let mut gid: gid_t = getgid();
    let mut pw: *mut passwd = getpwuid(uid);
    if pw.is_null() {
        return b"getpwuid\0" as *const u8 as *const libc::c_char;
    }
    if initgroups((*pw).pw_name, gid) != 0 as libc::c_int {
        return b"initgroups\0" as *const u8 as *const libc::c_char;
    }
    if gid != getegid() && setgid(gid) != 0 as libc::c_int
        && *__errno_location() != 1 as libc::c_int
    {
        return b"setgid\0" as *const u8 as *const libc::c_char;
    }
    if uid != geteuid() && setuid(uid) != 0 as libc::c_int
        && *__errno_location() != 1 as libc::c_int
    {
        return b"setuid\0" as *const u8 as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
pub unsafe extern "C" fn rmt_open__(
    mut file_name: *const libc::c_char,
    mut open_mode: libc::c_int,
    mut bias: libc::c_int,
    mut remote_shell: *const libc::c_char,
) -> libc::c_int {
    let mut remote_pipe_number: libc::c_int = 0;
    let mut file_name_copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut remote_host: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut remote_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut remote_user: *mut libc::c_char = 0 as *mut libc::c_char;
    remote_pipe_number = 0 as libc::c_int;
    while remote_pipe_number < 4 as libc::c_int {
        if from_remote[remote_pipe_number as usize][0 as libc::c_int as usize]
            == -(1 as libc::c_int)
            && to_remote[remote_pipe_number as usize][1 as libc::c_int as usize]
                == -(1 as libc::c_int)
        {
            break;
        }
        remote_pipe_number += 1;
        remote_pipe_number;
    }
    if remote_pipe_number == 4 as libc::c_int {
        *__errno_location() = 24 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut cursor: *mut libc::c_char = 0 as *mut libc::c_char;
    file_name_copy = xstrdup(file_name);
    remote_host = file_name_copy;
    remote_user = 0 as *mut libc::c_char;
    remote_file = 0 as *mut libc::c_char;
    cursor = file_name_copy;
    while *cursor != 0 {
        match *cursor as libc::c_int {
            10 => {
                rpl_free(file_name_copy as *mut libc::c_void);
                *__errno_location() = 2 as libc::c_int;
                return -(1 as libc::c_int);
            }
            64 => {
                if remote_user.is_null() {
                    remote_user = remote_host;
                    *cursor = '\0' as i32 as libc::c_char;
                    remote_host = cursor.offset(1 as libc::c_int as isize);
                }
            }
            58 => {
                if remote_file.is_null() {
                    *cursor = '\0' as i32 as libc::c_char;
                    remote_file = cursor.offset(1 as libc::c_int as isize);
                }
            }
            _ => {}
        }
        cursor = cursor.offset(1);
        cursor;
    }
    if !remote_file.is_null() {} else {
        unreachable!();
    };
    if (gethostbyname(remote_host)).is_null() {
        error(
            128 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Cannot connect to %s: resolve failed\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            remote_host,
        );
    }
    if !remote_user.is_null() && *remote_user as libc::c_int == '\0' as i32 {
        remote_user = 0 as *mut libc::c_char;
    }
    let mut remote_shell_basename: *const libc::c_char = 0 as *const libc::c_char;
    let mut status: pid_t = 0;
    if remote_shell.is_null() {
        rpl_free(file_name_copy as *mut libc::c_void);
        *__errno_location() = 5 as libc::c_int;
        return -(1 as libc::c_int);
    }
    remote_shell_basename = last_component(remote_shell);
    if pipe((to_remote[remote_pipe_number as usize]).as_mut_ptr()) == -(1 as libc::c_int)
        || pipe((from_remote[remote_pipe_number as usize]).as_mut_ptr())
            == -(1 as libc::c_int)
    {
        let mut e: libc::c_int = *__errno_location();
        rpl_free(file_name_copy as *mut libc::c_void);
        *__errno_location() = e;
        return -(1 as libc::c_int);
    }
    status = fork();
    if status == -(1 as libc::c_int) {
        let mut e_0: libc::c_int = *__errno_location();
        rpl_free(file_name_copy as *mut libc::c_void);
        *__errno_location() = e_0;
        return -(1 as libc::c_int);
    }
    if status == 0 as libc::c_int {
        if dup2(
            to_remote[remote_pipe_number as usize][0 as libc::c_int as usize],
            0 as libc::c_int,
        ) < 0 as libc::c_int
            || to_remote[remote_pipe_number as usize][0 as libc::c_int as usize]
                != 0 as libc::c_int
                && close(
                    to_remote[remote_pipe_number as usize][0 as libc::c_int as usize],
                ) != 0 as libc::c_int
            || to_remote[remote_pipe_number as usize][1 as libc::c_int as usize]
                != 0 as libc::c_int
                && close(
                    to_remote[remote_pipe_number as usize][1 as libc::c_int as usize],
                ) != 0 as libc::c_int
            || dup2(
                from_remote[remote_pipe_number as usize][1 as libc::c_int as usize],
                1 as libc::c_int,
            ) < 0 as libc::c_int
            || close(from_remote[remote_pipe_number as usize][0 as libc::c_int as usize])
                != 0 as libc::c_int
            || close(from_remote[remote_pipe_number as usize][1 as libc::c_int as usize])
                != 0 as libc::c_int
        {
            error(
                128 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cannot redirect files for remote shell\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        let mut reseterr: *const libc::c_char = sys_reset_uid_gid();
        if !reseterr.is_null() {
            error(
                128 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cannot reset uid and gid: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                reseterr,
            );
        }
        if !remote_user.is_null() {
            execl(
                remote_shell,
                remote_shell_basename,
                remote_host,
                b"-l\0" as *const u8 as *const libc::c_char,
                remote_user,
                rmt_command,
                0 as *mut libc::c_char,
            );
        } else {
            execl(
                remote_shell,
                remote_shell_basename,
                remote_host,
                rmt_command,
                0 as *mut libc::c_char,
            );
        }
        error(
            128 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"Cannot execute remote shell\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    close(from_remote[remote_pipe_number as usize][1 as libc::c_int as usize]);
    close(to_remote[remote_pipe_number as usize][0 as libc::c_int as usize]);
    let mut remote_file_len: size_t = strlen(remote_file);
    let mut command_buffer: *mut libc::c_char = xmalloc(
        remote_file_len.wrapping_add(1000 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    sprintf(command_buffer, b"O%s\n\0" as *const u8 as *const libc::c_char, remote_file);
    encode_oflag(
        command_buffer
            .offset(remote_file_len as isize)
            .offset(2 as libc::c_int as isize),
        open_mode,
    );
    strcat(command_buffer, b"\n\0" as *const u8 as *const libc::c_char);
    if do_command(remote_pipe_number, command_buffer) == -(1 as libc::c_int)
        || get_status(remote_pipe_number) == -(1 as libc::c_int) as libc::c_long
    {
        let mut e_1: libc::c_int = *__errno_location();
        rpl_free(command_buffer as *mut libc::c_void);
        rpl_free(file_name_copy as *mut libc::c_void);
        _rmt_shutdown(remote_pipe_number, e_1);
        return -(1 as libc::c_int);
    }
    rpl_free(command_buffer as *mut libc::c_void);
    rpl_free(file_name_copy as *mut libc::c_void);
    return remote_pipe_number + bias;
}
pub unsafe extern "C" fn rmt_close__(mut handle: libc::c_int) -> libc::c_int {
    let mut status: libc::c_long = 0;
    if do_command(handle, b"C\n\0" as *const u8 as *const libc::c_char)
        == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int);
    }
    status = get_status(handle);
    _rmt_shutdown(handle, *__errno_location());
    return status as libc::c_int;
}
pub unsafe extern "C" fn rmt_read__(
    mut handle: libc::c_int,
    mut buffer: *mut libc::c_char,
    mut length: size_t,
) -> size_t {
    let mut command_buffer: [libc::c_char; 23] = [0; 23];
    let mut status: size_t = 0;
    let mut rlen: size_t = 0;
    let mut counter: size_t = 0;
    sprintf(
        command_buffer.as_mut_ptr(),
        b"R%zu\n\0" as *const u8 as *const libc::c_char,
        length,
    );
    if do_command(handle, command_buffer.as_mut_ptr()) == -(1 as libc::c_int)
        || {
            status = get_status(handle) as size_t;
            status == -(1 as libc::c_int) as size_t
        } || status > length
    {
        return -(1 as libc::c_int) as size_t;
    }
    counter = 0 as libc::c_int as size_t;
    while counter < status {
        rlen = safe_read(
            from_remote[handle as usize][0 as libc::c_int as usize],
            buffer as *mut libc::c_void,
            status.wrapping_sub(counter),
        );
        if rlen == -(1 as libc::c_int) as size_t
            || rlen == 0 as libc::c_int as libc::c_ulong
        {
            _rmt_shutdown(handle, 5 as libc::c_int);
            return -(1 as libc::c_int) as size_t;
        }
        counter = (counter as libc::c_ulong).wrapping_add(rlen) as size_t as size_t;
        buffer = buffer.offset(rlen as isize);
    }
    return status;
}
pub unsafe extern "C" fn rmt_write__(
    mut handle: libc::c_int,
    mut buffer: *mut libc::c_char,
    mut length: size_t,
) -> size_t {
    let mut command_buffer: [libc::c_char; 23] = [0; 23];
    let mut pipe_handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
    let mut written: size_t = 0;
    sprintf(
        command_buffer.as_mut_ptr(),
        b"W%zu\n\0" as *const u8 as *const libc::c_char,
        length,
    );
    if do_command(handle, command_buffer.as_mut_ptr()) == -(1 as libc::c_int) {
        return 0 as libc::c_int as size_t;
    }
    pipe_handler = signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    written = full_write(
        to_remote[handle as usize][1 as libc::c_int as usize],
        buffer as *const libc::c_void,
        length,
    );
    signal(13 as libc::c_int, pipe_handler);
    if written == length {
        let mut r: libc::c_long = get_status(handle);
        if r < 0 as libc::c_int as libc::c_long {
            return 0 as libc::c_int as size_t;
        }
        if r as libc::c_ulong == length {
            return length;
        }
        written = r as size_t;
    }
    _rmt_shutdown(handle, 5 as libc::c_int);
    return written;
}
pub unsafe extern "C" fn rmt_lseek__(
    mut handle: libc::c_int,
    mut offset: off_t,
    mut whence: libc::c_int,
) -> off_t {
    let mut command_buffer: [libc::c_char; 25] = [0; 25];
    match whence {
        0 => {
            whence = 0 as libc::c_int;
        }
        1 => {
            whence = 1 as libc::c_int;
        }
        2 => {
            whence = 2 as libc::c_int;
        }
        _ => {
            abort();
        }
    }
    let mut off: intmax_t = offset;
    sprintf(
        command_buffer.as_mut_ptr(),
        b"L%jd\n%d\n\0" as *const u8 as *const libc::c_char,
        off,
        whence,
    );
    if do_command(handle, command_buffer.as_mut_ptr()) == -(1 as libc::c_int) {
        return -(1 as libc::c_int) as off_t;
    }
    return get_status_off(handle);
}
pub unsafe extern "C" fn rmt_ioctl__(
    mut handle: libc::c_int,
    mut operation: libc::c_ulong,
    mut argument: *mut libc::c_void,
) -> libc::c_int {
    match operation {
        1074294017 => {
            let mut mtop: *mut mtop = argument as *mut mtop;
            let mut command_buffer: [libc::c_char; 21] = [0; 21];
            let mut count: intmax_t = (*mtop).mt_count as intmax_t;
            sprintf(
                command_buffer.as_mut_ptr(),
                b"I%d\n%jd\n\0" as *const u8 as *const libc::c_char,
                (*mtop).mt_op as libc::c_int,
                count,
            );
            if do_command(handle, command_buffer.as_mut_ptr()) == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            return get_status(handle) as libc::c_int;
        }
        2150657282 => {
            let mut status: ssize_t = 0;
            let mut counter: size_t = 0;
            if do_command(handle, b"S\0" as *const u8 as *const libc::c_char)
                == -(1 as libc::c_int)
                || {
                    status = get_status(handle);
                    status == -(1 as libc::c_int) as libc::c_long
                }
            {
                return -(1 as libc::c_int);
            }
            if status as libc::c_ulong > ::std::mem::size_of::<mtop>() as libc::c_ulong {
                *__errno_location() = 75 as libc::c_int;
                return -(1 as libc::c_int);
            }
            let mut p: *mut libc::c_char = argument as *mut libc::c_char;
            while status > 0 as libc::c_int as libc::c_long {
                counter = safe_read(
                    from_remote[handle as usize][0 as libc::c_int as usize],
                    p as *mut libc::c_void,
                    status as size_t,
                );
                if counter == -(1 as libc::c_int) as size_t
                    || counter == 0 as libc::c_int as libc::c_ulong
                {
                    _rmt_shutdown(handle, 5 as libc::c_int);
                    return -(1 as libc::c_int);
                }
                status = (status as libc::c_ulong).wrapping_sub(counter) as ssize_t
                    as ssize_t;
                p = p.offset(counter as isize);
            }
            let mut mtget: *mut mtget = argument as *mut mtget;
            if (*mtget).mt_type < 256 as libc::c_int as libc::c_long {
                return 0 as libc::c_int;
            }
            let mut buf: *mut libc::c_char = argument as *mut libc::c_char;
            counter = 0 as libc::c_int as size_t;
            while counter < status as libc::c_ulong {
                let mut copy: libc::c_char = *buf.offset(counter as isize);
                *buf
                    .offset(
                        counter as isize,
                    ) = *buf
                    .offset(
                        counter.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                *buf
                    .offset(
                        counter.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = copy;
                counter = (counter as libc::c_ulong)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            }
            return 0 as libc::c_int;
        }
        _ => {
            *__errno_location() = 95 as libc::c_int;
            return -(1 as libc::c_int);
        }
    };
}
