use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type htab;
    pub type upstream;
    pub type buffer_s;
    pub type http_message_s;
    fn __errno_location() -> *mut libc::c_int;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn http_message_create(
        response_code: libc::c_int,
        response_string: *const libc::c_char,
    ) -> http_message_t;
    fn http_message_destroy(msg: http_message_t) -> libc::c_int;
    fn http_message_send(msg: http_message_t, fd: libc::c_int) -> libc::c_int;
    fn http_message_set_body(
        msg: http_message_t,
        body: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
    fn http_message_add_headers(
        msg: http_message_t,
        headers: *mut *const libc::c_char,
        num_headers: libc::c_uint,
    ) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conn_s {
    pub client_fd: libc::c_int,
    pub server_fd: libc::c_int,
    pub cbuffer: *mut buffer_s,
    pub sbuffer: *mut buffer_s,
    pub request_line: *mut libc::c_char,
    pub connect_method: libc::c_uint,
    pub show_stats: libc::c_uint,
    pub error_variables: *mut htab,
    pub error_number: libc::c_int,
    pub error_string: *mut libc::c_char,
    pub content_length: C2RustUnnamed_0,
    pub server_ip_addr: *mut libc::c_char,
    pub client_ip_addr: *mut libc::c_char,
    pub protocol: C2RustUnnamed,
    pub reversepath: *mut libc::c_char,
    pub upstream_proxy: *mut upstream,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub major: libc::c_uint,
    pub minor: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub server: libc::c_long,
    pub client: libc::c_long,
}
pub type http_message_t = *mut http_message_s;
pub unsafe extern "C" fn send_http_message(
    mut connptr: *mut conn_s,
    mut http_code: libc::c_int,
    mut error_title: *const libc::c_char,
    mut message: *const libc::c_char,
) -> libc::c_int {
    static mut headers: [*const libc::c_char; 3] = [
        b"Server: tinyproxy/1.11.2\0" as *const u8 as *const libc::c_char,
        b"Content-type: text/html\0" as *const u8 as *const libc::c_char,
        b"Connection: close\0" as *const u8 as *const libc::c_char,
    ];
    let mut msg: http_message_t = 0 as *mut http_message_s;
    msg = http_message_create(http_code, error_title);
    if msg.is_null() {
        return -(1 as libc::c_int);
    }
    http_message_add_headers(
        msg,
        headers.as_mut_ptr(),
        3 as libc::c_int as libc::c_uint,
    );
    http_message_set_body(msg, message, strlen(message));
    http_message_send(msg, (*connptr).client_fd);
    http_message_destroy(msg);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn create_file_safely(
    mut filename: *const libc::c_char,
    mut truncate_file: libc::c_uint,
) -> libc::c_int {
    let mut lstatinfo: stat = stat {
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
    let mut fildes: libc::c_int = 0;
    if lstat(filename, &mut lstatinfo) < 0 as libc::c_int {
        if *__errno_location() != 2 as libc::c_int {
            fprintf(
                stderr,
                b"%s: Error checking file %s: %s\n\0" as *const u8
                    as *const libc::c_char,
                b"tinyproxy\0" as *const u8 as *const libc::c_char,
                filename,
                strerror(*__errno_location()),
            );
            return -(13 as libc::c_int);
        }
        fildes = open(
            filename,
            0o2 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int,
            0o600 as libc::c_int,
        );
        if fildes < 0 as libc::c_int {
            fprintf(
                stderr,
                b"%s: Could not create file %s: %s\n\0" as *const u8
                    as *const libc::c_char,
                b"tinyproxy\0" as *const u8 as *const libc::c_char,
                filename,
                strerror(*__errno_location()),
            );
            return fildes;
        }
    } else {
        let mut fstatinfo: stat = stat {
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
        let mut flags: libc::c_int = 0;
        flags = 0o2 as libc::c_int;
        if truncate_file == 0 {
            flags |= 0o2000 as libc::c_int;
        }
        fildes = open(filename, flags);
        if fildes < 0 as libc::c_int {
            fprintf(
                stderr,
                b"%s: Could not open file %s: %s\n\0" as *const u8
                    as *const libc::c_char,
                b"tinyproxy\0" as *const u8 as *const libc::c_char,
                filename,
                strerror(*__errno_location()),
            );
            return fildes;
        }
        if fstat(fildes, &mut fstatinfo) < 0 as libc::c_int
            || lstatinfo.st_mode != fstatinfo.st_mode
            || lstatinfo.st_ino != fstatinfo.st_ino
            || lstatinfo.st_dev != fstatinfo.st_dev
        {
            fprintf(
                stderr,
                b"%s: The file %s has been changed before it could be opened\n\0"
                    as *const u8 as *const libc::c_char,
                b"tinyproxy\0" as *const u8 as *const libc::c_char,
                filename,
            );
            close(fildes);
            return -(5 as libc::c_int);
        }
        if fstatinfo.st_nlink > 1 as libc::c_int as libc::c_ulong
            || !(lstatinfo.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint)
        {
            fprintf(
                stderr,
                b"%s: The file %s has too many links, or is not a regular file: %s\n\0"
                    as *const u8 as *const libc::c_char,
                b"tinyproxy\0" as *const u8 as *const libc::c_char,
                filename,
                strerror(*__errno_location()),
            );
            close(fildes);
            return -(31 as libc::c_int);
        }
        if truncate_file == 0 {
            return fildes;
        }
        close(fildes);
        fildes = open(
            filename,
            0o2 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
            0o600 as libc::c_int,
        );
        if fildes < 0 as libc::c_int {
            fprintf(
                stderr,
                b"%s: Could not open file %s: %s.\0" as *const u8 as *const libc::c_char,
                b"tinyproxy\0" as *const u8 as *const libc::c_char,
                filename,
                strerror(*__errno_location()),
            );
            return fildes;
        }
    }
    return fildes;
}
pub unsafe extern "C" fn pidfile_create(
    mut filename: *const libc::c_char,
) -> libc::c_int {
    let mut fildes: libc::c_int = 0;
    let mut fd: *mut FILE = 0 as *mut FILE;
    fildes = create_file_safely(
        filename,
        (0 as libc::c_int == 0) as libc::c_int as libc::c_uint,
    );
    if fildes < 0 as libc::c_int {
        return fildes;
    }
    fd = fdopen(fildes, b"w\0" as *const u8 as *const libc::c_char);
    if fd.is_null() {
        fprintf(
            stderr,
            b"%s: Could not write PID file %s: %s.\0" as *const u8
                as *const libc::c_char,
            b"tinyproxy\0" as *const u8 as *const libc::c_char,
            filename,
            strerror(*__errno_location()),
        );
        close(fildes);
        unlink(filename);
        return -(5 as libc::c_int);
    }
    fprintf(fd, b"%d\n\0" as *const u8 as *const libc::c_char, getpid());
    fclose(fd);
    return 0 as libc::c_int;
}
