use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type strm_queue;
    pub type node_error;
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
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strm_nil_value() -> strm_value;
    fn strm_ptr_tag_p(_: strm_value, _: strm_ptr_type) -> libc::c_int;
    fn strm_value_ptr(_: strm_value, _: strm_ptr_type) -> *mut libc::c_void;
    fn strm_ptr_value(_: *mut libc::c_void) -> strm_value;
    fn strm_foreign_value(_: *mut libc::c_void) -> strm_value;
    fn strm_value_foreign(_: strm_value) -> *mut libc::c_void;
    fn strm_str_new(_: *const libc::c_char, _: strm_int) -> strm_string;
    fn strm_strp_ptr(_: *mut strm_string) -> *const libc::c_char;
    fn strm_str_len(_: strm_string) -> strm_int;
    fn strm_to_str(v: strm_value) -> strm_string;
    fn strm_stream_new(
        mode: strm_stream_mode,
        start: strm_callback,
        close_0: strm_callback,
        data: *mut libc::c_void,
    ) -> *mut strm_stream;
    fn strm_emit(strm: *mut strm_stream, data: strm_value, cb: strm_callback);
    fn strm_task_new(func: strm_callback, data: strm_value) -> *mut strm_task;
    fn strm_stream_close(strm: *mut strm_stream);
    fn strm_task_push(strm: *mut strm_stream, func: strm_callback, data: strm_value);
    fn strm_task_add(strm: *mut strm_stream, _: *mut strm_task);
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn epoll_create(__size: libc::c_int) -> libc::c_int;
    fn epoll_ctl(
        __epfd: libc::c_int,
        __op: libc::c_int,
        __fd: libc::c_int,
        __event: *mut epoll_event,
    ) -> libc::c_int;
    fn epoll_wait(
        __epfd: libc::c_int,
        __events: *mut epoll_event,
        __maxevents: libc::c_int,
        __timeout: libc::c_int,
    ) -> libc::c_int;
    fn writev(__fd: libc::c_int, __iovec: *const iovec, __count: libc::c_int) -> ssize_t;
    fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
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
pub type ssize_t = __ssize_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type strm_value = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_stream {
    pub type_0: strm_ptr_type,
    pub flags: libc::c_uint,
    pub mode: strm_stream_mode,
    pub start_func: strm_callback,
    pub close_func: strm_callback,
    pub data: *mut libc::c_void,
    pub dst: *mut strm_stream,
    pub rest: *mut *mut strm_stream,
    pub rsize: size_t,
    pub rcapa: size_t,
    pub exc: *mut node_error,
    pub refcnt: strm_int,
    pub queue: *mut strm_queue,
    pub excl: strm_int,
}
pub type strm_int = int32_t;
pub type strm_callback = Option::<
    unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
>;
pub type strm_stream_mode = libc::c_uint;
pub const strm_killed: strm_stream_mode = 4;
pub const strm_dying: strm_stream_mode = 3;
pub const strm_consumer: strm_stream_mode = 2;
pub const strm_filter: strm_stream_mode = 1;
pub const strm_producer: strm_stream_mode = 0;
pub type strm_ptr_type = libc::c_uint;
pub const STRM_PTR_AUX: strm_ptr_type = 4;
pub const STRM_PTR_IO: strm_ptr_type = 3;
pub const STRM_PTR_GENFUNC: strm_ptr_type = 2;
pub const STRM_PTR_LAMBDA: strm_ptr_type = 1;
pub const STRM_PTR_STREAM: strm_ptr_type = 0;
pub type strm_string = uint64_t;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct epoll_event {
    pub events: uint32_t,
    pub data: epoll_data_t,
}
pub type epoll_data_t = epoll_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub union epoll_data {
    pub ptr: *mut libc::c_void,
    pub fd: libc::c_int,
    pub u32_0: uint32_t,
    pub u64_0: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_task {
    pub func: strm_callback,
    pub data: strm_value,
}
pub const EPOLLONESHOT: EPOLL_EVENTS = 1073741824;
pub const EPOLLIN: EPOLL_EVENTS = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_io {
    pub type_0: strm_ptr_type,
    pub fd: libc::c_int,
    pub mode: libc::c_int,
    pub read_stream: *mut strm_stream,
    pub write_stream: *mut strm_stream,
}
pub type strm_io_0 = *mut strm_io;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct write_data {
    pub f: *mut FILE,
    pub io: strm_io_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_read_buffer {
    pub fd: libc::c_int,
    pub beg: *mut libc::c_char,
    pub end: *mut libc::c_char,
    pub io: strm_io_0,
    pub buf: *mut libc::c_char,
    pub fixed: [libc::c_char; 8192],
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
pub type EPOLL_EVENTS = libc::c_uint;
pub const EPOLLET: EPOLL_EVENTS = 2147483648;
pub const EPOLLWAKEUP: EPOLL_EVENTS = 536870912;
pub const EPOLLEXCLUSIVE: EPOLL_EVENTS = 268435456;
pub const EPOLLRDHUP: EPOLL_EVENTS = 8192;
pub const EPOLLHUP: EPOLL_EVENTS = 16;
pub const EPOLLERR: EPOLL_EVENTS = 8;
pub const EPOLLMSG: EPOLL_EVENTS = 1024;
pub const EPOLLWRBAND: EPOLL_EVENTS = 512;
pub const EPOLLWRNORM: EPOLL_EVENTS = 256;
pub const EPOLLRDBAND: EPOLL_EVENTS = 128;
pub const EPOLLRDNORM: EPOLL_EVENTS = 64;
pub const EPOLLOUT: EPOLL_EVENTS = 4;
pub const EPOLLPRI: EPOLL_EVENTS = 2;
static mut io_worker: pthread_t = 0;
static mut io_wait_num: libc::c_int = 0 as libc::c_int;
static mut epoll_fd: libc::c_int = 0;
unsafe extern "C" fn io_task(
    mut strm: *mut strm_stream,
    mut func: strm_callback,
) -> *mut strm_task {
    return strm_task_new(func, strm_foreign_value(strm as *mut libc::c_void));
}
unsafe extern "C" fn io_task_add(mut task: *mut strm_task) {
    let mut strm: *mut strm_stream = strm_value_foreign((*task).data)
        as *mut strm_stream;
    (*task).data = strm_nil_value();
    strm_task_add(strm, task);
}
unsafe extern "C" fn io_push(
    mut fd: libc::c_int,
    mut strm: *mut strm_stream,
    mut cb: strm_callback,
) -> libc::c_int {
    let mut ev: epoll_event = {
        let mut init = epoll_event {
            events: 0 as libc::c_int as uint32_t,
            data: epoll_data {
                ptr: 0 as *mut libc::c_void,
            },
        };
        init
    };
    ev.events = (EPOLLIN as libc::c_int | EPOLLONESHOT as libc::c_int) as uint32_t;
    ev.data.ptr = io_task(strm, cb) as *mut libc::c_void;
    return epoll_ctl(epoll_fd, 1 as libc::c_int, fd, &mut ev);
}
unsafe extern "C" fn io_kick(
    mut fd: libc::c_int,
    mut strm: *mut strm_stream,
    mut cb: strm_callback,
) -> libc::c_int {
    let mut ev: epoll_event = epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    };
    ev.events = (EPOLLIN as libc::c_int | EPOLLONESHOT as libc::c_int) as uint32_t;
    ev.data.ptr = io_task(strm, cb) as *mut libc::c_void;
    return epoll_ctl(epoll_fd, 3 as libc::c_int, fd, &mut ev);
}
unsafe extern "C" fn io_pop(mut fd: libc::c_int) -> libc::c_int {
    return epoll_ctl(epoll_fd, 2 as libc::c_int, fd, 0 as *mut epoll_event);
}
unsafe extern "C" fn io_loop(mut d: *mut libc::c_void) -> *mut libc::c_void {
    let mut events: [epoll_event; 10] = [epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    }; 10];
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    loop {
        n = epoll_wait(
            epoll_fd,
            events.as_mut_ptr(),
            10 as libc::c_int,
            -(1 as libc::c_int),
        );
        if n < 0 as libc::c_int {
            return 0 as *mut libc::c_void;
        }
        i = 0 as libc::c_int;
        while i < n {
            io_task_add(events[i as usize].data.ptr as *mut strm_task);
            i += 1;
            i;
        }
    };
}
pub unsafe extern "C" fn strm_init_io_loop() {
    epoll_fd = epoll_create(10 as libc::c_int);
    if epoll_fd >= 0 as libc::c_int {} else {
        __assert_fail(
            b"epoll_fd >= 0\0" as *const u8 as *const libc::c_char,
            b"io.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"void strm_init_io_loop()\0"))
                .as_ptr(),
        );
    }
    'c_5367: {
        if epoll_fd >= 0 as libc::c_int {} else {
            __assert_fail(
                b"epoll_fd >= 0\0" as *const u8 as *const libc::c_char,
                b"io.c\0" as *const u8 as *const libc::c_char,
                106 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"void strm_init_io_loop()\0"))
                    .as_ptr(),
            );
        }
    };
    pthread_create(
        &mut io_worker,
        0 as *const pthread_attr_t,
        Some(io_loop as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn strm_io_start_read(
    mut strm: *mut strm_stream,
    mut fd: libc::c_int,
    mut cb: strm_callback,
) {
    if io_push(fd, strm, cb) == 0 as libc::c_int {
        io_wait_num += 1;
        io_wait_num;
    }
}
unsafe extern "C" fn strm_io_stop(mut strm: *mut strm_stream, mut fd: libc::c_int) {
    if (*strm).flags & 1 as libc::c_int as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        io_wait_num -= 1;
        io_wait_num;
        io_pop(fd);
    }
    strm_stream_close(strm);
}
pub unsafe extern "C" fn strm_io_emit(
    mut strm: *mut strm_stream,
    mut data: strm_value,
    mut fd: libc::c_int,
    mut cb: strm_callback,
) {
    strm_emit(strm, data, None);
    io_kick(fd, strm, cb);
}
unsafe extern "C" fn read_str(
    mut beg: *const libc::c_char,
    mut len: strm_int,
) -> strm_value {
    let mut p: *mut libc::c_char = malloc(len as libc::c_ulong) as *mut libc::c_char;
    memcpy(p as *mut libc::c_void, beg as *const libc::c_void, len as libc::c_ulong);
    return strm_str_new(p, len);
}
unsafe extern "C" fn read_cb(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut b: *mut fd_read_buffer = (*strm).data as *mut fd_read_buffer;
    let mut count: strm_int = 0;
    let mut n: strm_int = 0;
    count = (8192 as libc::c_int as libc::c_long
        - ((*b).end).offset_from((*b).buf) as libc::c_long) as strm_int;
    n = read((*b).fd, (*b).end as *mut libc::c_void, count as size_t) as strm_int;
    if n <= 0 as libc::c_int {
        if (*b).buf < (*b).end {
            let mut s: strm_value = read_str(
                (*b).beg,
                ((*b).end).offset_from((*b).beg) as libc::c_long as strm_int,
            );
            (*b).end = (*b).buf;
            (*b).beg = (*b).end;
            strm_io_emit(
                strm,
                s,
                (*b).fd,
                Some(
                    read_cb
                        as unsafe extern "C" fn(
                            *mut strm_stream,
                            strm_value,
                        ) -> libc::c_int,
                ),
            );
        } else {
            strm_io_stop(strm, (*b).fd);
        }
        return 0 as libc::c_int;
    }
    (*b).end = ((*b).end).offset(n as isize);
    (Some(
        (Some(
            readline_cb
                as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
        ))
            .unwrap(),
    ))
        .unwrap()(strm, strm_nil_value());
    return 0 as libc::c_int;
}
unsafe extern "C" fn readline_cb(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut b: *mut fd_read_buffer = (*strm).data as *mut fd_read_buffer;
    let mut s: strm_value = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: strm_int = ((*b).end).offset_from((*b).beg) as libc::c_long as strm_int;
    p = memchr((*b).beg as *const libc::c_void, '\n' as i32, len as libc::c_ulong)
        as *mut libc::c_char;
    if !p.is_null() {
        len = p.offset_from((*b).beg) as libc::c_long as strm_int;
    } else if (*strm).flags & 2 as libc::c_int as libc::c_uint != 0 {
        if len <= 0 as libc::c_int {
            if (*strm).flags & 4 as libc::c_int as libc::c_uint != 0 {
                munmap(
                    (*b).buf as *mut libc::c_void,
                    ((*b).end).offset_from((*b).beg) as libc::c_long as size_t,
                );
            }
            strm_io_stop(strm, (*b).fd);
            return 0 as libc::c_int;
        }
    } else {
        if (len as libc::c_ulong)
            < ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong
        {
            memmove(
                (*b).buf as *mut libc::c_void,
                (*b).beg as *const libc::c_void,
                len as libc::c_ulong,
            );
            (*b).beg = (*b).buf;
            (*b).end = ((*b).beg).offset(len as isize);
        }
        if (*strm).flags & 1 as libc::c_int as libc::c_uint != 0 {
            strm_task_push(
                strm,
                Some(
                    read_cb
                        as unsafe extern "C" fn(
                            *mut strm_stream,
                            strm_value,
                        ) -> libc::c_int,
                ),
                strm_nil_value(),
            );
        } else {
            io_kick(
                (*b).fd,
                strm,
                Some(
                    read_cb
                        as unsafe extern "C" fn(
                            *mut strm_stream,
                            strm_value,
                        ) -> libc::c_int,
                ),
            );
        }
        return 0 as libc::c_int;
    }
    s = read_str((*b).beg, len);
    (*b).beg = ((*b).beg).offset((len + 1 as libc::c_int) as isize);
    strm_emit(
        strm,
        s,
        Some(
            readline_cb
                as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
        ),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn stdio_read(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut b: *mut fd_read_buffer = (*strm).data as *mut fd_read_buffer;
    strm_io_start_read(
        strm,
        (*b).fd,
        Some(
            read_cb as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
        ),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn read_close(
    mut strm: *mut strm_stream,
    mut d: strm_value,
) -> libc::c_int {
    let mut b: *mut fd_read_buffer = (*strm).data as *mut fd_read_buffer;
    close((*b).fd);
    free(b as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn strm_readio(mut io: strm_io_0) -> *mut strm_stream {
    let mut cb: strm_callback = Some(
        stdio_read as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
    );
    let mut flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if ((*io).read_stream).is_null() {
        let mut buf: *mut fd_read_buffer = malloc(
            ::std::mem::size_of::<fd_read_buffer>() as libc::c_ulong,
        ) as *mut fd_read_buffer;
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
        (*io).mode |= 8 as libc::c_int;
        (*buf).fd = (*io).fd;
        (*buf).io = io;
        (*buf).buf = ((*buf).fixed).as_mut_ptr();
        if fstat((*io).fd, &mut st) == 0 as libc::c_int
            && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint
        {
            flags |= 1 as libc::c_int as libc::c_uint;
            let mut map: *mut libc::c_void = mmap(
                0 as *mut libc::c_void,
                st.st_size as size_t,
                0x1 as libc::c_int,
                0x2 as libc::c_int,
                (*buf).fd,
                0 as libc::c_int as __off_t,
            );
            if map == -(1 as libc::c_int) as *mut libc::c_void {
                (*buf).end = (*buf).buf;
                (*buf).beg = (*buf).end;
            } else {
                (*buf).beg = map as *mut libc::c_char;
                (*buf).buf = (*buf).beg;
                (*buf).end = map.offset(st.st_size as isize) as *mut libc::c_char;
                flags |= 2 as libc::c_int as libc::c_uint;
                cb = Some(
                    readline_cb
                        as unsafe extern "C" fn(
                            *mut strm_stream,
                            strm_value,
                        ) -> libc::c_int,
                );
            }
        } else {
            (*buf).end = (*buf).buf;
            (*buf).beg = (*buf).end;
        }
        (*io)
            .read_stream = strm_stream_new(
            strm_producer,
            cb,
            Some(
                read_close
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            buf as *mut libc::c_void,
        );
        (*(*io).read_stream).flags |= flags;
    }
    return (*io).read_stream;
}
unsafe extern "C" fn write_cb(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut write_data = (*strm).data as *mut write_data;
    let mut p: strm_string = strm_to_str(data);
    let mut buf: [iovec; 2] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 2];
    buf[0 as libc::c_int as usize].iov_base = strm_strp_ptr(&mut p) as *mut libc::c_void;
    buf[0 as libc::c_int as usize].iov_len = strm_str_len(p) as size_t;
    buf[1 as libc::c_int as usize]
        .iov_base = b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_void;
    buf[1 as libc::c_int as usize].iov_len = 1 as libc::c_int as size_t;
    if writev(fileno((*d).f), buf.as_mut_ptr(), 2 as libc::c_int)
        < 0 as libc::c_int as libc::c_long
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn write_close(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut write_data = (*strm).data as *mut write_data;
    shutdown(fileno((*d).f), 1 as libc::c_int);
    if (*(*d).io).mode & 8 as libc::c_int == 0 as libc::c_int {
        fclose((*d).f);
    }
    free(d as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn strm_writeio(mut io: strm_io_0) -> *mut strm_stream {
    let mut d: *mut write_data = 0 as *mut write_data;
    if ((*io).write_stream).is_null() {
        d = malloc(::std::mem::size_of::<write_data>() as libc::c_ulong)
            as *mut write_data;
        (*d).f = fdopen((*io).fd, b"w\0" as *const u8 as *const libc::c_char);
        (*d).io = io;
        (*io)
            .write_stream = strm_stream_new(
            strm_consumer,
            Some(
                write_cb
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            Some(
                write_close
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            d as *mut libc::c_void,
        );
    }
    return (*io).write_stream;
}
pub unsafe extern "C" fn strm_io_new(
    mut fd: libc::c_int,
    mut mode: libc::c_int,
) -> strm_value {
    let mut io: strm_io_0 = malloc(::std::mem::size_of::<strm_io>() as libc::c_ulong)
        as strm_io_0;
    (*io).fd = fd;
    (*io).mode = mode;
    (*io).type_0 = STRM_PTR_IO;
    (*io).write_stream = 0 as *mut strm_stream;
    (*io).read_stream = (*io).write_stream;
    return strm_ptr_value(io as *mut libc::c_void);
}
pub unsafe extern "C" fn strm_io_stream(
    mut iov: strm_value,
    mut mode: libc::c_int,
) -> *mut strm_stream {
    let mut io: strm_io_0 = 0 as *mut strm_io;
    if strm_ptr_tag_p(iov, STRM_PTR_IO) != 0 {} else {
        __assert_fail(
            b"strm_io_p(iov)\0" as *const u8 as *const libc::c_char,
            b"io.c\0" as *const u8 as *const libc::c_char,
            380 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"strm_stream *strm_io_stream(strm_value, int)\0"))
                .as_ptr(),
        );
    }
    'c_4081: {
        if strm_ptr_tag_p(iov, STRM_PTR_IO) != 0 {} else {
            __assert_fail(
                b"strm_io_p(iov)\0" as *const u8 as *const libc::c_char,
                b"io.c\0" as *const u8 as *const libc::c_char,
                380 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"strm_stream *strm_io_stream(strm_value, int)\0"))
                    .as_ptr(),
            );
        }
    };
    io = strm_value_ptr(iov, STRM_PTR_IO) as strm_io_0;
    match mode {
        1 => return strm_readio(io),
        2 => return strm_writeio(io),
        _ => return 0 as *mut strm_stream,
    };
}
