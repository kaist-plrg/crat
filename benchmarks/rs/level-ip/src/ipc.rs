use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strnlen(__string: *const libc::c_char, __maxlen: size_t) -> size_t;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn free(__ptr: *mut libc::c_void);
    fn _socket(
        pid: pid_t,
        domain: libc::c_int,
        type_0: libc::c_int,
        protocol: libc::c_int,
    ) -> libc::c_int;
    fn _connect(
        pid: pid_t,
        sockfd: libc::c_int,
        addr: *const sockaddr,
        addrlen: socklen_t,
    ) -> libc::c_int;
    fn _write(
        pid: pid_t,
        sockfd: libc::c_int,
        buf: *const libc::c_void,
        count: libc::c_uint,
    ) -> libc::c_int;
    fn _read(
        pid: pid_t,
        sockfd: libc::c_int,
        buf: *mut libc::c_void,
        count: libc::c_uint,
    ) -> libc::c_int;
    fn _close(pid: pid_t, sockfd: libc::c_int) -> libc::c_int;
    fn _poll(
        pid: pid_t,
        fds: *mut pollfd,
        nfds: nfds_t,
        timeout: libc::c_int,
    ) -> libc::c_int;
    fn _fcntl(pid: pid_t, fildes: libc::c_int, cmd: libc::c_int, _: ...) -> libc::c_int;
    fn _getsockopt(
        pid: pid_t,
        fd: libc::c_int,
        level: libc::c_int,
        optname: libc::c_int,
        optval: *mut libc::c_void,
        optlen: *mut socklen_t,
    ) -> libc::c_int;
    fn _getpeername(
        pid: pid_t,
        socket_0: libc::c_int,
        address: *mut sockaddr,
        address_len: *mut socklen_t,
    ) -> libc::c_int;
    fn _getsockname(
        pid: pid_t,
        socket_0: libc::c_int,
        address: *mut sockaddr,
        address_len: *mut socklen_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type C2RustUnnamed = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed = 67108864;
pub const MSG_BATCH: C2RustUnnamed = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed = 65536;
pub const MSG_MORE: C2RustUnnamed = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed = 8192;
pub const MSG_RST: C2RustUnnamed = 4096;
pub const MSG_CONFIRM: C2RustUnnamed = 2048;
pub const MSG_SYN: C2RustUnnamed = 1024;
pub const MSG_FIN: C2RustUnnamed = 512;
pub const MSG_WAITALL: C2RustUnnamed = 256;
pub const MSG_EOR: C2RustUnnamed = 128;
pub const MSG_DONTWAIT: C2RustUnnamed = 64;
pub const MSG_TRUNC: C2RustUnnamed = 32;
pub const MSG_PROXY: C2RustUnnamed = 16;
pub const MSG_CTRUNC: C2RustUnnamed = 8;
pub const MSG_DONTROUTE: C2RustUnnamed = 4;
pub const MSG_PEEK: C2RustUnnamed = 2;
pub const MSG_OOB: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_0 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_0 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_0 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_thread {
    pub list: list_head,
    pub sock: libc::c_int,
    pub id: pthread_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_msg {
    pub type_0: uint16_t,
    pub pid: pid_t,
    pub data: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_sockname {
    pub socket: libc::c_int,
    pub address_len: socklen_t,
    pub sa_data: [uint8_t; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_err {
    pub rc: libc::c_int,
    pub err: libc::c_int,
    pub data: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_sockopt {
    pub fd: libc::c_int,
    pub level: libc::c_int,
    pub optname: libc::c_int,
    pub optlen: socklen_t,
    pub optval: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_fcntl {
    pub sockfd: libc::c_int,
    pub cmd: libc::c_int,
    pub data: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_poll {
    pub nfds: nfds_t,
    pub timeout: libc::c_int,
    pub fds: [ipc_pollfd; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_close {
    pub sockfd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_read {
    pub sockfd: libc::c_int,
    pub len: size_t,
    pub buf: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_write {
    pub sockfd: libc::c_int,
    pub len: size_t,
    pub buf: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_connect {
    pub sockfd: libc::c_int,
    pub addr: sockaddr,
    pub addrlen: socklen_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_socket {
    pub domain: libc::c_int,
    pub type_0: libc::c_int,
    pub protocol: libc::c_int,
}
#[inline]
unsafe extern "C" fn list_init(mut head: *mut list_head) {
    (*head).next = head;
    (*head).prev = (*head).next;
}
#[inline]
unsafe extern "C" fn list_add_tail(mut new: *mut list_head, mut head: *mut list_head) {
    (*(*head).prev).next = new;
    (*new).prev = (*head).prev;
    (*new).next = head;
    (*head).prev = new;
}
#[inline]
unsafe extern "C" fn list_del(mut elem: *mut list_head) {
    let mut prev: *mut list_head = (*elem).prev;
    let mut next: *mut list_head = (*elem).next;
    (*prev).next = next;
    (*next).prev = prev;
}
static mut sockets: list_head = unsafe {
    {
        let mut init = list_head {
            next: &sockets as *const list_head as *mut list_head,
            prev: &sockets as *const list_head as *mut list_head,
        };
        init
    }
};
static mut lock: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut socket_count: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn ipc_alloc_thread(mut sock: libc::c_int) -> *mut ipc_thread {
    let mut th: *mut ipc_thread = calloc(
        ::std::mem::size_of::<ipc_thread>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as *mut ipc_thread;
    list_init(&mut (*th).list);
    (*th).sock = sock;
    pthread_mutex_lock(&mut lock);
    list_add_tail(&mut (*th).list, &mut sockets);
    socket_count += 1;
    socket_count;
    pthread_mutex_unlock(&mut lock);
    return th;
}
unsafe extern "C" fn ipc_free_thread(mut sock: libc::c_int) {
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut tmp: *mut list_head = 0 as *mut list_head;
    let mut th: *mut ipc_thread = 0 as *mut ipc_thread;
    pthread_mutex_lock(&mut lock);
    item = sockets.next;
    tmp = (*item).next;
    while item != &mut sockets as *mut list_head {
        th = (item as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut ipc_thread;
        if (*th).sock == sock {
            list_del(&mut (*th).list);
            close((*th).sock);
            free(th as *mut libc::c_void);
            socket_count -= 1;
            socket_count;
            break;
        } else {
            item = tmp;
            tmp = (*item).next;
        }
    }
    pthread_mutex_unlock(&mut lock);
}
unsafe extern "C" fn ipc_try_send(
    mut sockfd: libc::c_int,
    mut buf: *const libc::c_void,
    mut len: size_t,
) -> libc::c_int {
    return send(sockfd, buf, len, MSG_NOSIGNAL as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn ipc_write_rc(
    mut sockfd: libc::c_int,
    mut pid: pid_t,
    mut type_0: uint16_t,
    mut rc: libc::c_int,
) -> libc::c_int {
    let mut resplen: libc::c_int = (::std::mem::size_of::<ipc_msg>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ipc_err>() as libc::c_ulong) as libc::c_int;
    let mut fresh0 = ::std::vec::from_elem(0, resplen as libc::c_ulong as usize);
    let mut response: *mut ipc_msg = fresh0.leak().as_mut_ptr() as *mut ipc_msg;
    if response.is_null() {
        fprintf(
            stderr,
            b"Could not allocate memory for IPC write response\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*response).type_0 = type_0;
    (*response).pid = pid;
    let mut err: ipc_err = ipc_err { rc: 0, err: 0, data: [] };
    if rc < 0 as libc::c_int {
        err.err = -rc;
        err.rc = -(1 as libc::c_int);
    } else {
        err.err = 0 as libc::c_int;
        err.rc = rc;
    }
    memcpy(
        ((*response).data).as_mut_ptr() as *mut libc::c_void,
        &mut err as *mut ipc_err as *const libc::c_void,
        ::std::mem::size_of::<ipc_err>() as libc::c_ulong,
    );
    if ipc_try_send(
        sockfd,
        response as *mut libc::c_char as *const libc::c_void,
        resplen as size_t,
    ) == -(1 as libc::c_int)
    {
        perror(
            b"Error on writing IPC write response\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ipc_read(
    mut sockfd: libc::c_int,
    mut msg: *mut ipc_msg,
) -> libc::c_int {
    let mut requested: *mut ipc_read = ((*msg).data).as_mut_ptr() as *mut ipc_read;
    let mut pid: pid_t = (*msg).pid;
    let mut rlen: libc::c_int = -(1 as libc::c_int);
    let vla = (*requested).len as usize;
    let mut rbuf: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla);
    memset(rbuf.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int, (*requested).len);
    rlen = _read(
        pid,
        (*requested).sockfd,
        rbuf.as_mut_ptr() as *mut libc::c_void,
        (*requested).len as libc::c_uint,
    );
    let mut resplen: libc::c_int = (::std::mem::size_of::<ipc_msg>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ipc_err>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ipc_read>() as libc::c_ulong)
        .wrapping_add(
            (if rlen > 0 as libc::c_int { rlen } else { 0 as libc::c_int })
                as libc::c_ulong,
        ) as libc::c_int;
    let mut fresh1 = ::std::vec::from_elem(0, resplen as libc::c_ulong as usize);
    let mut response: *mut ipc_msg = fresh1.leak().as_mut_ptr() as *mut ipc_msg;
    let mut error: *mut ipc_err = ((*response).data).as_mut_ptr() as *mut ipc_err;
    let mut actual: *mut ipc_read = ((*error).data).as_mut_ptr() as *mut ipc_read;
    if response.is_null() {
        fprintf(
            stderr,
            b"Could not allocate memory for IPC read response\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*response).type_0 = 0x4 as libc::c_int as uint16_t;
    (*response).pid = pid;
    (*error).rc = if rlen < 0 as libc::c_int { -(1 as libc::c_int) } else { rlen };
    (*error).err = if rlen < 0 as libc::c_int { -rlen } else { 0 as libc::c_int };
    (*actual).sockfd = (*requested).sockfd;
    (*actual).len = rlen as size_t;
    memcpy(
        ((*actual).buf).as_mut_ptr() as *mut libc::c_void,
        rbuf.as_mut_ptr() as *const libc::c_void,
        (if rlen > 0 as libc::c_int { rlen } else { 0 as libc::c_int }) as libc::c_ulong,
    );
    if ipc_try_send(
        sockfd,
        response as *mut libc::c_char as *const libc::c_void,
        resplen as size_t,
    ) == -(1 as libc::c_int)
    {
        perror(
            b"Error on writing IPC read response\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ipc_write(
    mut sockfd: libc::c_int,
    mut msg: *mut ipc_msg,
) -> libc::c_int {
    let mut payload: *mut ipc_write = ((*msg).data).as_mut_ptr() as *mut ipc_write;
    let mut pid: pid_t = (*msg).pid;
    let mut rc: libc::c_int = -(1 as libc::c_int);
    let mut head: libc::c_int = (8192 as libc::c_int as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<ipc_write>() as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<ipc_msg>() as libc::c_ulong) as libc::c_int;
    let vla = (*payload).len as usize;
    let mut buf: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla);
    memset(buf.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int, (*payload).len);
    memcpy(
        buf.as_mut_ptr() as *mut libc::c_void,
        ((*payload).buf).as_mut_ptr() as *const libc::c_void,
        if (*payload).len > head as libc::c_ulong {
            head as libc::c_ulong
        } else {
            (*payload).len
        },
    );
    if (*payload).len > head as libc::c_ulong {
        let mut tail: libc::c_int = ((*payload).len).wrapping_sub(head as libc::c_ulong)
            as libc::c_int;
        let mut res: libc::c_int = read(
            sockfd,
            &mut *buf.as_mut_ptr().offset(head as isize) as *mut libc::c_char
                as *mut libc::c_void,
            tail as size_t,
        ) as libc::c_int;
        if res == -(1 as libc::c_int) {
            perror(b"Read on IPC payload guard\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        } else if res != tail {
            fprintf(
                stderr,
                b"Hmm, we did not read exact payload amount in IPC write\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
    rc = _write(
        pid,
        (*payload).sockfd,
        buf.as_mut_ptr() as *const libc::c_void,
        (*payload).len as libc::c_uint,
    );
    return ipc_write_rc(sockfd, pid, 0x3 as libc::c_int as uint16_t, rc);
}
unsafe extern "C" fn ipc_connect(
    mut sockfd: libc::c_int,
    mut msg: *mut ipc_msg,
) -> libc::c_int {
    let mut payload: *mut ipc_connect = ((*msg).data).as_mut_ptr() as *mut ipc_connect;
    let mut pid: pid_t = (*msg).pid;
    let mut rc: libc::c_int = -(1 as libc::c_int);
    rc = _connect(pid, (*payload).sockfd, &mut (*payload).addr, (*payload).addrlen);
    return ipc_write_rc(sockfd, pid, 0x2 as libc::c_int as uint16_t, rc);
}
unsafe extern "C" fn ipc_socket(
    mut sockfd: libc::c_int,
    mut msg: *mut ipc_msg,
) -> libc::c_int {
    let mut sock: *mut ipc_socket = ((*msg).data).as_mut_ptr() as *mut ipc_socket;
    let mut pid: pid_t = (*msg).pid;
    let mut rc: libc::c_int = -(1 as libc::c_int);
    rc = _socket(pid, (*sock).domain, (*sock).type_0, (*sock).protocol);
    return ipc_write_rc(sockfd, pid, 0x1 as libc::c_int as uint16_t, rc);
}
unsafe extern "C" fn ipc_close(
    mut sockfd: libc::c_int,
    mut msg: *mut ipc_msg,
) -> libc::c_int {
    let mut payload: *mut ipc_close = ((*msg).data).as_mut_ptr() as *mut ipc_close;
    let mut pid: pid_t = (*msg).pid;
    let mut rc: libc::c_int = -(1 as libc::c_int);
    rc = _close(pid, (*payload).sockfd);
    rc = ipc_write_rc(sockfd, pid, 0x5 as libc::c_int as uint16_t, rc);
    return rc;
}
unsafe extern "C" fn ipc_poll(
    mut sockfd: libc::c_int,
    mut msg: *mut ipc_msg,
) -> libc::c_int {
    let mut data: *mut ipc_poll = ((*msg).data).as_mut_ptr() as *mut ipc_poll;
    let mut pid: pid_t = (*msg).pid;
    let mut rc: libc::c_int = -(1 as libc::c_int);
    let vla = (*data).nfds as usize;
    let mut fds: Vec::<pollfd> = ::std::vec::from_elem(
        pollfd {
            fd: 0,
            events: 0,
            revents: 0,
        },
        vla,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_ulong) < (*data).nfds {
        (*fds.as_mut_ptr().offset(i as isize))
            .fd = (*((*data).fds).as_mut_ptr().offset(i as isize)).fd;
        (*fds.as_mut_ptr().offset(i as isize))
            .events = (*((*data).fds).as_mut_ptr().offset(i as isize)).events;
        (*fds.as_mut_ptr().offset(i as isize))
            .revents = (*((*data).fds).as_mut_ptr().offset(i as isize)).revents;
        i += 1;
        i;
    }
    rc = _poll(pid, fds.as_mut_ptr(), (*data).nfds, (*data).timeout);
    let mut resplen: libc::c_int = (::std::mem::size_of::<ipc_msg>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ipc_err>() as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<ipc_pollfd>() as libc::c_ulong)
                .wrapping_mul((*data).nfds),
        ) as libc::c_int;
    let mut fresh2 = ::std::vec::from_elem(0, resplen as libc::c_ulong as usize);
    let mut response: *mut ipc_msg = fresh2.leak().as_mut_ptr() as *mut ipc_msg;
    if response.is_null() {
        fprintf(
            stderr,
            b"Could not allocate memory for IPC write response\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*response).type_0 = 0x6 as libc::c_int as uint16_t;
    (*response).pid = pid;
    let mut err: ipc_err = ipc_err { rc: 0, err: 0, data: [] };
    if rc < 0 as libc::c_int {
        err.err = -rc;
        err.rc = -(1 as libc::c_int);
    } else {
        err.err = 0 as libc::c_int;
        err.rc = rc;
    }
    memcpy(
        ((*response).data).as_mut_ptr() as *mut libc::c_void,
        &mut err as *mut ipc_err as *const libc::c_void,
        ::std::mem::size_of::<ipc_err>() as libc::c_ulong,
    );
    let mut polled: *mut ipc_pollfd = ((*(((*response).data).as_mut_ptr()
        as *mut ipc_err))
        .data)
        .as_mut_ptr() as *mut ipc_pollfd;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while (i_0 as libc::c_ulong) < (*data).nfds {
        (*polled.offset(i_0 as isize)).fd = (*fds.as_mut_ptr().offset(i_0 as isize)).fd;
        (*polled.offset(i_0 as isize))
            .events = (*fds.as_mut_ptr().offset(i_0 as isize)).events;
        (*polled.offset(i_0 as isize))
            .revents = (*fds.as_mut_ptr().offset(i_0 as isize)).revents;
        i_0 += 1;
        i_0;
    }
    if ipc_try_send(
        sockfd,
        response as *mut libc::c_char as *const libc::c_void,
        resplen as size_t,
    ) == -(1 as libc::c_int)
    {
        perror(
            b"Error on writing IPC poll response\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ipc_fcntl(
    mut sockfd: libc::c_int,
    mut msg: *mut ipc_msg,
) -> libc::c_int {
    let mut fc: *mut ipc_fcntl = ((*msg).data).as_mut_ptr() as *mut ipc_fcntl;
    let mut pid: pid_t = (*msg).pid;
    let mut rc: libc::c_int = -(1 as libc::c_int);
    match (*fc).cmd {
        3 => {
            rc = _fcntl(pid, (*fc).sockfd, (*fc).cmd);
        }
        4 => {
            rc = _fcntl(
                pid,
                (*fc).sockfd,
                (*fc).cmd,
                *(((*fc).data).as_mut_ptr() as *mut libc::c_int),
            );
        }
        _ => {
            fprintf(
                stderr,
                b"IPC Fcntl cmd not supported %d\n\0" as *const u8
                    as *const libc::c_char,
                (*fc).cmd,
            );
            rc = -(22 as libc::c_int);
        }
    }
    return ipc_write_rc(sockfd, pid, 0x7 as libc::c_int as uint16_t, rc);
}
unsafe extern "C" fn ipc_getsockopt(
    mut sockfd: libc::c_int,
    mut msg: *mut ipc_msg,
) -> libc::c_int {
    let mut opts: *mut ipc_sockopt = ((*msg).data).as_mut_ptr() as *mut ipc_sockopt;
    let mut pid: pid_t = (*msg).pid;
    let mut rc: libc::c_int = -(1 as libc::c_int);
    rc = _getsockopt(
        pid,
        (*opts).fd,
        (*opts).level,
        (*opts).optname,
        ((*opts).optval).as_mut_ptr() as *mut libc::c_void,
        &mut (*opts).optlen,
    );
    let mut resplen: libc::c_int = (::std::mem::size_of::<ipc_msg>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ipc_err>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ipc_sockopt>() as libc::c_ulong)
        .wrapping_add((*opts).optlen as libc::c_ulong) as libc::c_int;
    let mut fresh3 = ::std::vec::from_elem(0, resplen as libc::c_ulong as usize);
    let mut response: *mut ipc_msg = fresh3.leak().as_mut_ptr() as *mut ipc_msg;
    if response.is_null() {
        fprintf(
            stderr,
            b"Could not allocate memory for IPC getsockopt response\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*response).type_0 = 0x8 as libc::c_int as uint16_t;
    (*response).pid = pid;
    let mut err: ipc_err = ipc_err { rc: 0, err: 0, data: [] };
    if rc < 0 as libc::c_int {
        err.err = -rc;
        err.rc = -(1 as libc::c_int);
    } else {
        err.err = 0 as libc::c_int;
        err.rc = rc;
    }
    memcpy(
        ((*response).data).as_mut_ptr() as *mut libc::c_void,
        &mut err as *mut ipc_err as *const libc::c_void,
        ::std::mem::size_of::<ipc_err>() as libc::c_ulong,
    );
    let mut optres: *mut ipc_sockopt = ((*(((*response).data).as_mut_ptr()
        as *mut ipc_err))
        .data)
        .as_mut_ptr() as *mut ipc_sockopt;
    (*optres).fd = (*opts).fd;
    (*optres).level = (*opts).level;
    (*optres).optname = (*opts).optname;
    (*optres).optlen = (*opts).optlen;
    memcpy(
        &mut (*optres).optval as *mut [uint8_t; 0] as *mut libc::c_void,
        ((*opts).optval).as_mut_ptr() as *const libc::c_void,
        (*opts).optlen as libc::c_ulong,
    );
    if ipc_try_send(
        sockfd,
        response as *mut libc::c_char as *const libc::c_void,
        resplen as size_t,
    ) == -(1 as libc::c_int)
    {
        perror(
            b"Error on writing IPC getsockopt response\0" as *const u8
                as *const libc::c_char,
        );
    }
    return rc;
}
unsafe extern "C" fn ipc_getpeername(
    mut sockfd: libc::c_int,
    mut msg: *mut ipc_msg,
) -> libc::c_int {
    let mut name: *mut ipc_sockname = ((*msg).data).as_mut_ptr() as *mut ipc_sockname;
    let mut pid: pid_t = (*msg).pid;
    let mut rc: libc::c_int = -(1 as libc::c_int);
    let mut resplen: libc::c_int = (::std::mem::size_of::<ipc_msg>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ipc_err>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ipc_sockname>() as libc::c_ulong)
        as libc::c_int;
    let mut fresh4 = ::std::vec::from_elem(0, resplen as libc::c_ulong as usize);
    let mut response: *mut ipc_msg = fresh4.leak().as_mut_ptr() as *mut ipc_msg;
    if response.is_null() {
        fprintf(
            stderr,
            b"Could not allocate memory for IPC getpeername response\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*response).type_0 = 0xa as libc::c_int as uint16_t;
    (*response).pid = pid;
    let mut nameres: *mut ipc_sockname = ((*(((*response).data).as_mut_ptr()
        as *mut ipc_err))
        .data)
        .as_mut_ptr() as *mut ipc_sockname;
    rc = _getpeername(
        pid,
        (*name).socket,
        ((*nameres).sa_data).as_mut_ptr() as *mut sockaddr,
        &mut (*nameres).address_len,
    );
    let mut err: ipc_err = ipc_err { rc: 0, err: 0, data: [] };
    if rc < 0 as libc::c_int {
        err.err = -rc;
        err.rc = -(1 as libc::c_int);
    } else {
        err.err = 0 as libc::c_int;
        err.rc = rc;
    }
    memcpy(
        ((*response).data).as_mut_ptr() as *mut libc::c_void,
        &mut err as *mut ipc_err as *const libc::c_void,
        ::std::mem::size_of::<ipc_err>() as libc::c_ulong,
    );
    (*nameres).socket = (*name).socket;
    if ipc_try_send(
        sockfd,
        response as *mut libc::c_char as *const libc::c_void,
        resplen as size_t,
    ) == -(1 as libc::c_int)
    {
        perror(
            b"Error on writing IPC getpeername response\0" as *const u8
                as *const libc::c_char,
        );
    }
    return rc;
}
unsafe extern "C" fn ipc_getsockname(
    mut sockfd: libc::c_int,
    mut msg: *mut ipc_msg,
) -> libc::c_int {
    let mut name: *mut ipc_sockname = ((*msg).data).as_mut_ptr() as *mut ipc_sockname;
    let mut pid: pid_t = (*msg).pid;
    let mut rc: libc::c_int = -(1 as libc::c_int);
    let mut resplen: libc::c_int = (::std::mem::size_of::<ipc_msg>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ipc_err>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ipc_sockname>() as libc::c_ulong)
        as libc::c_int;
    let mut fresh5 = ::std::vec::from_elem(0, resplen as libc::c_ulong as usize);
    let mut response: *mut ipc_msg = fresh5.leak().as_mut_ptr() as *mut ipc_msg;
    if response.is_null() {
        fprintf(
            stderr,
            b"Could not allocate memory for IPC getsockname response\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*response).type_0 = 0xb as libc::c_int as uint16_t;
    (*response).pid = pid;
    let mut nameres: *mut ipc_sockname = ((*(((*response).data).as_mut_ptr()
        as *mut ipc_err))
        .data)
        .as_mut_ptr() as *mut ipc_sockname;
    rc = _getsockname(
        pid,
        (*name).socket,
        ((*nameres).sa_data).as_mut_ptr() as *mut sockaddr,
        &mut (*nameres).address_len,
    );
    let mut err: ipc_err = ipc_err { rc: 0, err: 0, data: [] };
    if rc < 0 as libc::c_int {
        err.err = -rc;
        err.rc = -(1 as libc::c_int);
    } else {
        err.err = 0 as libc::c_int;
        err.rc = rc;
    }
    memcpy(
        ((*response).data).as_mut_ptr() as *mut libc::c_void,
        &mut err as *mut ipc_err as *const libc::c_void,
        ::std::mem::size_of::<ipc_err>() as libc::c_ulong,
    );
    (*nameres).socket = (*name).socket;
    if ipc_try_send(
        sockfd,
        response as *mut libc::c_char as *const libc::c_void,
        resplen as size_t,
    ) == -(1 as libc::c_int)
    {
        perror(
            b"Error on writing IPC getsockname response\0" as *const u8
                as *const libc::c_char,
        );
    }
    return rc;
}
unsafe extern "C" fn demux_ipc_socket_call(
    mut sockfd: libc::c_int,
    mut cmdbuf: *mut libc::c_char,
    mut blen: libc::c_int,
) -> libc::c_int {
    let mut msg: *mut ipc_msg = cmdbuf as *mut ipc_msg;
    match (*msg).type_0 as libc::c_int {
        1 => return ipc_socket(sockfd, msg),
        2 => return ipc_connect(sockfd, msg),
        3 => return ipc_write(sockfd, msg),
        4 => return ipc_read(sockfd, msg),
        5 => return ipc_close(sockfd, msg),
        6 => return ipc_poll(sockfd, msg),
        7 => return ipc_fcntl(sockfd, msg),
        8 => return ipc_getsockopt(sockfd, msg),
        10 => return ipc_getpeername(sockfd, msg),
        11 => return ipc_getsockname(sockfd, msg),
        _ => {
            fprintf(
                stderr,
                b"No such IPC type %d\n\0" as *const u8 as *const libc::c_char,
                (*msg).type_0 as libc::c_int,
            );
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn socket_ipc_open(
    mut args: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut blen: libc::c_int = 8192 as libc::c_int;
    let vla = blen as usize;
    let mut buf: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla);
    let mut sockfd: libc::c_int = *(args as *mut libc::c_int);
    let mut rc: libc::c_int = -(1 as libc::c_int);
    loop {
        rc = read(sockfd, buf.as_mut_ptr() as *mut libc::c_void, blen as size_t)
            as libc::c_int;
        if !(rc > 0 as libc::c_int) {
            break;
        }
        rc = demux_ipc_socket_call(sockfd, buf.as_mut_ptr(), blen);
        if rc == -(1 as libc::c_int) {
            fprintf(
                stderr,
                b"Error on demuxing IPC socket call\n\0" as *const u8
                    as *const libc::c_char,
            );
            close(sockfd);
            return 0 as *mut libc::c_void;
        }
    }
    ipc_free_thread(sockfd);
    if rc == -(1 as libc::c_int) {
        perror(b"socket ipc read\0" as *const u8 as *const libc::c_char);
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn start_ipc_listener() -> *mut libc::c_void {
    let mut fd: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut datasock: libc::c_int = 0;
    let mut un: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut sockname: *mut libc::c_char = b"/tmp/lvlip.socket\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    unlink(sockname);
    if strnlen(sockname, ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong)
        == ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong
    {
        fprintf(
            stderr,
            b"Path for UNIX socket is too long\n\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    fd = socket(1 as libc::c_int, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        perror(b"IPC listener UNIX socket\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    memset(
        &mut un as *mut sockaddr_un as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong,
    );
    un.sun_family = 1 as libc::c_int as sa_family_t;
    strncpy(
        (un.sun_path).as_mut_ptr(),
        sockname,
        (::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    rc = bind(
        fd,
        &mut un as *mut sockaddr_un as *const sockaddr,
        ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t,
    );
    if rc == -(1 as libc::c_int) {
        perror(b"IPC bind\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    rc = listen(fd, 20 as libc::c_int);
    if rc == -(1 as libc::c_int) {
        perror(b"IPC listen\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if chmod(
        sockname,
        (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int
            | 0o100 as libc::c_int >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as __mode_t,
    ) == -(1 as libc::c_int)
    {
        perror(
            b"Chmod on lvl-ip IPC UNIX socket failed\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    loop {
        datasock = accept(fd, 0 as *mut sockaddr, 0 as *mut socklen_t);
        if datasock == -(1 as libc::c_int) {
            perror(b"IPC accept\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        let mut th: *mut ipc_thread = ipc_alloc_thread(datasock);
        if pthread_create(
            &mut (*th).id,
            0 as *const pthread_attr_t,
            Some(
                socket_ipc_open
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            &mut (*th).sock as *mut libc::c_int as *mut libc::c_void,
        ) != 0 as libc::c_int
        {
            fprintf(
                stderr,
                b"Error on socket thread creation\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    };
}
