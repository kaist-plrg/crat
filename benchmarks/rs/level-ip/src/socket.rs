use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_rwlock_init(
        __rwlock: *mut pthread_rwlock_t,
        __attr: *const pthread_rwlockattr_t,
    ) -> libc::c_int;
    fn pthread_rwlock_rdlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
    fn pthread_rwlock_wrlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
    fn pthread_rwlock_unlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn timer_oneshot(
        expire: uint32_t,
        handler: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        arg: *mut libc::c_void,
    );
    static mut inet: net_family;
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
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __useconds_t = libc::c_uint;
pub type __socklen_t = libc::c_uint;
pub type pid_t = __pid_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: libc::c_uint,
    pub __writers: libc::c_uint,
    pub __wrphase_futex: libc::c_uint,
    pub __writers_futex: libc::c_uint,
    pub __pad3: libc::c_uint,
    pub __pad4: libc::c_uint,
    pub __cur_writer: libc::c_int,
    pub __shared: libc::c_int,
    pub __rwelision: libc::c_schar,
    pub __pad1: [libc::c_uchar; 7],
    pub __pad2: libc::c_ulong,
    pub __flags: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
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
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlockattr_t {
    pub __size: [libc::c_char; 8],
    pub __align: libc::c_long,
}
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type socklen_t = __socklen_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const PTHREAD_RWLOCK_DEFAULT_NP: C2RustUnnamed_3 = 0;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: C2RustUnnamed_3 = 2;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NP: C2RustUnnamed_3 = 1;
pub const PTHREAD_RWLOCK_PREFER_READER_NP: C2RustUnnamed_3 = 0;
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
pub struct wait_lock {
    pub ready: pthread_cond_t,
    pub lock: pthread_mutex_t,
    pub sleeping: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sk_buff_head {
    pub head: list_head,
    pub qlen: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sock {
    pub sock: *mut socket,
    pub ops: *mut net_ops,
    pub recv_wait: wait_lock,
    pub receive_queue: sk_buff_head,
    pub write_queue: sk_buff_head,
    pub protocol: libc::c_int,
    pub state: libc::c_int,
    pub err: libc::c_int,
    pub poll_events: libc::c_short,
    pub sport: uint16_t,
    pub dport: uint16_t,
    pub saddr: uint32_t,
    pub daddr: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct net_ops {
    pub alloc_sock: Option::<unsafe extern "C" fn(libc::c_int) -> *mut sock>,
    pub init: Option::<unsafe extern "C" fn(*mut sock) -> libc::c_int>,
    pub connect: Option::<
        unsafe extern "C" fn(
            *mut sock,
            *const sockaddr,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub disconnect: Option::<
        unsafe extern "C" fn(*mut sock, libc::c_int) -> libc::c_int,
    >,
    pub write: Option::<
        unsafe extern "C" fn(*mut sock, *const libc::c_void, libc::c_int) -> libc::c_int,
    >,
    pub read: Option::<
        unsafe extern "C" fn(*mut sock, *mut libc::c_void, libc::c_int) -> libc::c_int,
    >,
    pub recv_notify: Option::<unsafe extern "C" fn(*mut sock) -> libc::c_int>,
    pub close: Option::<unsafe extern "C" fn(*mut sock) -> libc::c_int>,
    pub abort: Option::<unsafe extern "C" fn(*mut sock) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct socket {
    pub list: list_head,
    pub fd: libc::c_int,
    pub pid: pid_t,
    pub refcnt: libc::c_int,
    pub state: socket_state,
    pub type_0: libc::c_short,
    pub flags: libc::c_int,
    pub sk: *mut sock,
    pub ops: *mut sock_ops,
    pub sleep: wait_lock,
    pub lock: pthread_rwlock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sock_ops {
    pub connect: Option::<
        unsafe extern "C" fn(
            *mut socket,
            *const sockaddr,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub write: Option::<
        unsafe extern "C" fn(
            *mut socket,
            *const libc::c_void,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub read: Option::<
        unsafe extern "C" fn(*mut socket, *mut libc::c_void, libc::c_int) -> libc::c_int,
    >,
    pub close: Option::<unsafe extern "C" fn(*mut socket) -> libc::c_int>,
    pub free: Option::<unsafe extern "C" fn(*mut socket) -> libc::c_int>,
    pub abort: Option::<unsafe extern "C" fn(*mut socket) -> libc::c_int>,
    pub poll: Option::<unsafe extern "C" fn(*mut socket) -> libc::c_int>,
    pub getpeername: Option::<
        unsafe extern "C" fn(*mut socket, *mut sockaddr, *mut socklen_t) -> libc::c_int,
    >,
    pub getsockname: Option::<
        unsafe extern "C" fn(*mut socket, *mut sockaddr, *mut socklen_t) -> libc::c_int,
    >,
}
pub type socket_state = libc::c_uint;
pub const SS_DISCONNECTING: socket_state = 4;
pub const SS_CONNECTED: socket_state = 3;
pub const SS_CONNECTING: socket_state = 2;
pub const SS_UNCONNECTED: socket_state = 1;
pub const SS_FREE: socket_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct net_family {
    pub create: Option::<unsafe extern "C" fn(*mut socket, libc::c_int) -> libc::c_int>,
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
#[inline]
unsafe extern "C" fn wait_free(mut w: *mut wait_lock) {
    wait_wakeup(w);
    pthread_mutex_destroy(&mut (*w).lock);
    pthread_cond_destroy(&mut (*w).ready);
}
#[inline]
unsafe extern "C" fn wait_wakeup(mut w: *mut wait_lock) -> libc::c_int {
    pthread_mutex_lock(&mut (*w).lock);
    pthread_cond_signal(&mut (*w).ready);
    (*w).sleeping = 0 as libc::c_int as uint8_t;
    pthread_mutex_unlock(&mut (*w).lock);
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn list_init(mut head: *mut list_head) {
    (*head).next = head;
    (*head).prev = (*head).next;
}
#[inline]
unsafe extern "C" fn wait_init(mut w: *mut wait_lock) -> libc::c_int {
    pthread_cond_init(&mut (*w).ready, 0 as *const pthread_condattr_t);
    pthread_mutex_init(&mut (*w).lock, 0 as *const pthread_mutexattr_t);
    (*w).sleeping = 0 as libc::c_int as uint8_t;
    return 0 as libc::c_int;
}
static mut sock_amount: libc::c_int = 0 as libc::c_int;
static mut sockets: list_head = unsafe {
    {
        let mut init = list_head {
            next: &sockets as *const list_head as *mut list_head,
            prev: &sockets as *const list_head as *mut list_head,
        };
        init
    }
};
static mut slock: pthread_rwlock_t = pthread_rwlock_t {
    __data: {
        let mut init = __pthread_rwlock_arch_t {
            __readers: 0 as libc::c_int as libc::c_uint,
            __writers: 0 as libc::c_int as libc::c_uint,
            __wrphase_futex: 0 as libc::c_int as libc::c_uint,
            __writers_futex: 0 as libc::c_int as libc::c_uint,
            __pad3: 0 as libc::c_int as libc::c_uint,
            __pad4: 0 as libc::c_int as libc::c_uint,
            __cur_writer: 0 as libc::c_int,
            __shared: 0 as libc::c_int,
            __rwelision: 0 as libc::c_int as libc::c_schar,
            __pad1: [
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
            ],
            __pad2: 0 as libc::c_int as libc::c_ulong,
            __flags: PTHREAD_RWLOCK_DEFAULT_NP as libc::c_int as libc::c_uint,
        };
        init
    },
};
static mut families: [*mut net_family; 128] = unsafe {
    [
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        &inet as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
        0 as *const net_family as *mut net_family,
    ]
};
unsafe extern "C" fn alloc_socket(mut pid: pid_t) -> *mut socket {
    static mut fd: libc::c_int = 4097 as libc::c_int;
    let mut sock: *mut socket = malloc(::std::mem::size_of::<socket>() as libc::c_ulong)
        as *mut socket;
    list_init(&mut (*sock).list);
    (*sock).pid = pid;
    (*sock).refcnt = 1 as libc::c_int;
    pthread_rwlock_wrlock(&mut slock);
    let fresh0 = fd;
    fd = fd + 1;
    (*sock).fd = fresh0;
    pthread_rwlock_unlock(&mut slock);
    (*sock).state = SS_UNCONNECTED;
    (*sock).ops = 0 as *mut sock_ops;
    (*sock).flags = 0o2 as libc::c_int;
    wait_init(&mut (*sock).sleep);
    pthread_rwlock_init(&mut (*sock).lock, 0 as *const pthread_rwlockattr_t);
    return sock;
}
pub unsafe extern "C" fn socket_rd_acquire(mut sock: *mut socket) -> libc::c_int {
    let mut rc: libc::c_int = pthread_rwlock_rdlock(&mut (*sock).lock);
    (*sock).refcnt += 1;
    (*sock).refcnt;
    return rc;
}
pub unsafe extern "C" fn socket_wr_acquire(mut sock: *mut socket) -> libc::c_int {
    let mut rc: libc::c_int = pthread_rwlock_wrlock(&mut (*sock).lock);
    (*sock).refcnt += 1;
    (*sock).refcnt;
    return rc;
}
pub unsafe extern "C" fn socket_release(mut sock: *mut socket) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    (*sock).refcnt -= 1;
    (*sock).refcnt;
    if (*sock).refcnt == 0 as libc::c_int {
        rc = pthread_rwlock_unlock(&mut (*sock).lock);
        free(sock as *mut libc::c_void);
    } else {
        rc = pthread_rwlock_unlock(&mut (*sock).lock);
    }
    return rc;
}
pub unsafe extern "C" fn socket_free(mut sock: *mut socket) -> libc::c_int {
    pthread_rwlock_wrlock(&mut slock);
    socket_wr_acquire(sock);
    list_del(&mut (*sock).list);
    sock_amount -= 1;
    sock_amount;
    pthread_rwlock_unlock(&mut slock);
    if !((*sock).ops).is_null() {
        ((*(*sock).ops).free).unwrap()(sock);
    }
    wait_free(&mut (*sock).sleep);
    socket_release(sock);
    return 0 as libc::c_int;
}
unsafe extern "C" fn socket_garbage_collect(
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut sock: *mut socket = socket_find(arg as *mut socket);
    if sock.is_null() {
        return 0 as *mut libc::c_void;
    }
    socket_free(sock);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn socket_delete(mut sock: *mut socket) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    if !((*sock).state as libc::c_uint
        == SS_DISCONNECTING as libc::c_int as libc::c_uint)
    {
        (*sock).state = SS_DISCONNECTING;
        timer_oneshot(
            10000 as libc::c_int as uint32_t,
            Some(
                socket_garbage_collect
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            sock as *mut libc::c_void,
        );
    }
    return rc;
}
pub unsafe extern "C" fn abort_sockets() {
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut tmp: *mut list_head = 0 as *mut list_head;
    let mut sock: *mut socket = 0 as *mut socket;
    item = sockets.next;
    tmp = (*item).next;
    while item != &mut sockets as *mut list_head {
        sock = (item as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut socket;
        ((*(*sock).ops).abort).unwrap()(sock);
        item = tmp;
        tmp = (*item).next;
    }
}
unsafe extern "C" fn get_socket(mut pid: pid_t, mut fd: uint32_t) -> *mut socket {
    let mut current_block: u64;
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut sock: *mut socket = 0 as *mut socket;
    pthread_rwlock_rdlock(&mut slock);
    item = sockets.next;
    loop {
        if !(item != &mut sockets as *mut list_head) {
            current_block = 15240798224410183470;
            break;
        }
        sock = (item as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut socket;
        if (*sock).pid == pid && (*sock).fd as libc::c_uint == fd {
            current_block = 1588557090142230336;
            break;
        }
        item = (*item).next;
    }
    match current_block {
        15240798224410183470 => {
            sock = 0 as *mut socket;
        }
        _ => {}
    }
    pthread_rwlock_unlock(&mut slock);
    return sock;
}
pub unsafe extern "C" fn socket_lookup(
    mut remoteport: uint16_t,
    mut localport: uint16_t,
) -> *mut socket {
    let mut current_block: u64;
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut sock: *mut socket = 0 as *mut socket;
    let mut sk: *mut sock = 0 as *mut sock;
    pthread_rwlock_rdlock(&mut slock);
    item = sockets.next;
    loop {
        if !(item != &mut sockets as *mut list_head) {
            current_block = 6937071982253665452;
            break;
        }
        sock = (item as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut socket;
        if !(sock.is_null() || ((*sock).sk).is_null()) {
            sk = (*sock).sk;
            if (*sk).sport as libc::c_int == localport as libc::c_int
                && (*sk).dport as libc::c_int == remoteport as libc::c_int
            {
                current_block = 17341552818222784024;
                break;
            }
        }
        item = (*item).next;
    }
    match current_block {
        6937071982253665452 => {
            sock = 0 as *mut socket;
        }
        _ => {}
    }
    pthread_rwlock_unlock(&mut slock);
    return sock;
}
pub unsafe extern "C" fn socket_find(mut find: *mut socket) -> *mut socket {
    let mut current_block: u64;
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut sock: *mut socket = 0 as *mut socket;
    pthread_rwlock_rdlock(&mut slock);
    item = sockets.next;
    loop {
        if !(item != &mut sockets as *mut list_head) {
            current_block = 15240798224410183470;
            break;
        }
        sock = (item as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut socket;
        if sock == find {
            current_block = 9069567062991755854;
            break;
        }
        item = (*item).next;
    }
    match current_block {
        15240798224410183470 => {
            sock = 0 as *mut socket;
        }
        _ => {}
    }
    pthread_rwlock_unlock(&mut slock);
    return sock;
}
pub unsafe extern "C" fn socket_debug() {}
pub unsafe extern "C" fn _socket(
    mut pid: pid_t,
    mut domain: libc::c_int,
    mut type_0: libc::c_int,
    mut protocol: libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut sock: *mut socket = 0 as *mut socket;
    let mut family: *mut net_family = 0 as *mut net_family;
    sock = alloc_socket(pid);
    if sock.is_null() {
        fprintf(
            stderr,
            b"Could not alloc socket\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*sock).type_0 = type_0 as libc::c_short;
    family = families[domain as usize];
    if family.is_null() {
        fprintf(
            stderr,
            b"Domain not supported: %d\n\0" as *const u8 as *const libc::c_char,
            domain,
        );
    } else if ((*family).create).unwrap()(sock, protocol) != 0 as libc::c_int {
        fprintf(
            stderr,
            b"Creating domain failed\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        pthread_rwlock_wrlock(&mut slock);
        list_add_tail(&mut (*sock).list, &mut sockets);
        sock_amount += 1;
        sock_amount;
        socket_rd_acquire(sock);
        pthread_rwlock_unlock(&mut slock);
        rc = (*sock).fd;
        socket_release(sock);
        return rc;
    }
    socket_free(sock);
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn _connect(
    mut pid: pid_t,
    mut sockfd: libc::c_int,
    mut addr: *const sockaddr,
    mut addrlen: socklen_t,
) -> libc::c_int {
    let mut sock: *mut socket = 0 as *mut socket;
    sock = get_socket(pid, sockfd as uint32_t);
    if sock.is_null() {
        fprintf(
            stderr,
            b"Connect: could not find socket (fd %u) for connection (pid %d)\n\0"
                as *const u8 as *const libc::c_char,
            sockfd,
            pid,
        );
        return -(9 as libc::c_int);
    }
    socket_wr_acquire(sock);
    let mut rc: libc::c_int = ((*(*sock).ops).connect)
        .unwrap()(sock, addr, addrlen as libc::c_int, 0 as libc::c_int);
    match rc {
        -22 | -97 | -111 | -110 => {
            socket_release(sock);
            socket_free(sock);
        }
        _ => {
            socket_release(sock);
        }
    }
    return rc;
}
pub unsafe extern "C" fn _write(
    mut pid: pid_t,
    mut sockfd: libc::c_int,
    mut buf: *const libc::c_void,
    count: libc::c_uint,
) -> libc::c_int {
    let mut sock: *mut socket = 0 as *mut socket;
    sock = get_socket(pid, sockfd as uint32_t);
    if sock.is_null() {
        fprintf(
            stderr,
            b"Write: could not find socket (fd %u) for connection (pid %d)\n\0"
                as *const u8 as *const libc::c_char,
            sockfd,
            pid,
        );
        return -(9 as libc::c_int);
    }
    socket_wr_acquire(sock);
    let mut rc: libc::c_int = ((*(*sock).ops).write)
        .unwrap()(sock, buf, count as libc::c_int);
    socket_release(sock);
    return rc;
}
pub unsafe extern "C" fn _read(
    mut pid: pid_t,
    mut sockfd: libc::c_int,
    mut buf: *mut libc::c_void,
    count: libc::c_uint,
) -> libc::c_int {
    let mut sock: *mut socket = 0 as *mut socket;
    sock = get_socket(pid, sockfd as uint32_t);
    if sock.is_null() {
        fprintf(
            stderr,
            b"Read: could not find socket (fd %u) for connection (pid %d)\n\0"
                as *const u8 as *const libc::c_char,
            sockfd,
            pid,
        );
        return -(9 as libc::c_int);
    }
    socket_wr_acquire(sock);
    let mut rc: libc::c_int = ((*(*sock).ops).read)
        .unwrap()(sock, buf, count as libc::c_int);
    socket_release(sock);
    return rc;
}
pub unsafe extern "C" fn _close(mut pid: pid_t, mut sockfd: libc::c_int) -> libc::c_int {
    let mut sock: *mut socket = 0 as *mut socket;
    sock = get_socket(pid, sockfd as uint32_t);
    if sock.is_null() {
        fprintf(
            stderr,
            b"Close: could not find socket (fd %u) for connection (pid %d)\n\0"
                as *const u8 as *const libc::c_char,
            sockfd,
            pid,
        );
        return -(9 as libc::c_int);
    }
    socket_wr_acquire(sock);
    let mut rc: libc::c_int = ((*(*sock).ops).close).unwrap()(sock);
    socket_release(sock);
    return rc;
}
pub unsafe extern "C" fn _poll(
    mut pid: pid_t,
    mut fds: *mut pollfd,
    mut nfds: nfds_t,
    mut timeout: libc::c_int,
) -> libc::c_int {
    loop {
        let mut polled: libc::c_int = 0 as libc::c_int;
        let mut i: libc::c_int = 0 as libc::c_int;
        while (i as libc::c_ulong) < nfds {
            let mut sock: *mut socket = 0 as *mut socket;
            let mut poll: *mut pollfd = &mut *fds.offset(i as isize) as *mut pollfd;
            sock = get_socket(pid, (*poll).fd as uint32_t);
            if sock.is_null() {
                fprintf(
                    stderr,
                    b"Poll: could not find socket (fd %u) for connection (pid %d)\n\0"
                        as *const u8 as *const libc::c_char,
                    (*poll).fd,
                    pid,
                );
                return -(9 as libc::c_int);
            }
            socket_rd_acquire(sock);
            (*poll)
                .revents = ((*(*sock).sk).poll_events as libc::c_int
                & ((*poll).events as libc::c_int | 0x10 as libc::c_int
                    | 0x8 as libc::c_int | 0x20 as libc::c_int)) as libc::c_short;
            if (*poll).revents as libc::c_int > 0 as libc::c_int {
                polled += 1;
                polled;
            }
            socket_release(sock);
            i += 1;
            i;
        }
        if polled > 0 as libc::c_int || timeout == 0 as libc::c_int {
            return polled
        } else {
            if timeout > 0 as libc::c_int {
                if timeout > 10 as libc::c_int {
                    timeout -= 10 as libc::c_int;
                } else {
                    timeout = 0 as libc::c_int;
                }
            }
            usleep((1000 as libc::c_int * 10 as libc::c_int) as __useconds_t);
        }
    };
}
pub unsafe extern "C" fn _fcntl(
    mut pid: pid_t,
    mut fildes: libc::c_int,
    mut cmd: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut sock: *mut socket = 0 as *mut socket;
    sock = get_socket(pid, fildes as uint32_t);
    if sock.is_null() {
        fprintf(
            stderr,
            b"Fcntl: could not find socket (fd %u) for connection (pid %d)\n\0"
                as *const u8 as *const libc::c_char,
            fildes,
            pid,
        );
        return -(9 as libc::c_int);
    }
    socket_wr_acquire(sock);
    let mut ap: ::std::ffi::VaListImpl;
    let mut rc: libc::c_int = 0 as libc::c_int;
    match cmd {
        3 => {
            rc = (*sock).flags;
        }
        4 => {
            ap = args.clone();
            (*sock).flags = ap.arg::<libc::c_int>();
            rc = 0 as libc::c_int;
        }
        _ => {
            rc = -(1 as libc::c_int);
        }
    }
    socket_release(sock);
    return rc;
}
pub unsafe extern "C" fn _getsockopt(
    mut pid: pid_t,
    mut fd: libc::c_int,
    mut level: libc::c_int,
    mut optname: libc::c_int,
    mut optval: *mut libc::c_void,
    mut optlen: *mut socklen_t,
) -> libc::c_int {
    let mut sock: *mut socket = 0 as *mut socket;
    sock = get_socket(pid, fd as uint32_t);
    if sock.is_null() {
        fprintf(
            stderr,
            b"Getsockopt: could not find socket (fd %u) for connection (pid %d)\n\0"
                as *const u8 as *const libc::c_char,
            fd,
            pid,
        );
        return -(9 as libc::c_int);
    }
    let mut rc: libc::c_int = 0 as libc::c_int;
    socket_rd_acquire(sock);
    match level {
        1 => {
            match optname {
                4 => {
                    *optlen = 4 as libc::c_int as socklen_t;
                    *(optval as *mut libc::c_int) = (*(*sock).sk).err;
                    rc = 0 as libc::c_int;
                }
                _ => {
                    fprintf(
                        stderr,
                        b"Getsockopt unsupported optname %d\n\0" as *const u8
                            as *const libc::c_char,
                        optname,
                    );
                    rc = -(92 as libc::c_int);
                }
            }
        }
        _ => {
            fprintf(
                stderr,
                b"Getsockopt: Unsupported level %d\n\0" as *const u8
                    as *const libc::c_char,
                level,
            );
            rc = -(22 as libc::c_int);
        }
    }
    socket_release(sock);
    return rc;
}
pub unsafe extern "C" fn _getpeername(
    mut pid: pid_t,
    mut socket: libc::c_int,
    mut address: *mut sockaddr,
    mut address_len: *mut socklen_t,
) -> libc::c_int {
    let mut sock: *mut socket = 0 as *mut socket;
    sock = get_socket(pid, socket as uint32_t);
    if sock.is_null() {
        fprintf(
            stderr,
            b"Getpeername: could not find socket (fd %u) for connection (pid %d)\n\0"
                as *const u8 as *const libc::c_char,
            socket,
            pid,
        );
        return -(9 as libc::c_int);
    }
    socket_rd_acquire(sock);
    let mut rc: libc::c_int = ((*(*sock).ops).getpeername)
        .unwrap()(sock, address, address_len);
    socket_release(sock);
    return rc;
}
pub unsafe extern "C" fn _getsockname(
    mut pid: pid_t,
    mut socket: libc::c_int,
    mut address: *mut sockaddr,
    mut address_len: *mut socklen_t,
) -> libc::c_int {
    let mut sock: *mut socket = 0 as *mut socket;
    sock = get_socket(pid, socket as uint32_t);
    if sock.is_null() {
        fprintf(
            stderr,
            b"Getsockname: could not find socket (fd %u) for connection (pid %d)\n\0"
                as *const u8 as *const libc::c_char,
            socket,
            pid,
        );
        return -(9 as libc::c_int);
    }
    socket_rd_acquire(sock);
    let mut rc: libc::c_int = ((*(*sock).ops).getsockname)
        .unwrap()(sock, address, address_len);
    socket_release(sock);
    return rc;
}
