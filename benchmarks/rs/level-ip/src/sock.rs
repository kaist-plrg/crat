use ::libc;
extern "C" {
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn free_skb(skb: *mut sk_buff);
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __pid_t = libc::c_int;
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
pub struct sk_buff_head {
    pub head: list_head,
    pub qlen: uint32_t,
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
pub type socket_state = libc::c_uint;
pub const SS_DISCONNECTING: socket_state = 4;
pub const SS_CONNECTED: socket_state = 3;
pub const SS_CONNECTING: socket_state = 2;
pub const SS_UNCONNECTED: socket_state = 1;
pub const SS_FREE: socket_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sk_buff {
    pub list: list_head,
    pub rt: *mut rtentry,
    pub dev: *mut netdev,
    pub refcnt: libc::c_int,
    pub protocol: uint16_t,
    pub len: uint32_t,
    pub dlen: uint32_t,
    pub seq: uint32_t,
    pub end_seq: uint32_t,
    pub end: *mut uint8_t,
    pub head: *mut uint8_t,
    pub data: *mut uint8_t,
    pub payload: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct netdev {
    pub addr: uint32_t,
    pub addr_len: uint8_t,
    pub hwaddr: [uint8_t; 6],
    pub mtu: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtentry {
    pub list: list_head,
    pub dst: uint32_t,
    pub gateway: uint32_t,
    pub netmask: uint32_t,
    pub flags: uint8_t,
    pub metric: uint32_t,
    pub dev: *mut netdev,
}
#[inline]
unsafe extern "C" fn list_init(mut head: *mut list_head) {
    (*head).next = head;
    (*head).prev = (*head).next;
}
#[inline]
unsafe extern "C" fn list_del(mut elem: *mut list_head) {
    let mut prev: *mut list_head = (*elem).prev;
    let mut next: *mut list_head = (*elem).next;
    (*prev).next = next;
    (*next).prev = prev;
}
#[inline]
unsafe extern "C" fn skb_queue_len(mut list: *const sk_buff_head) -> uint32_t {
    return (*list).qlen;
}
#[inline]
unsafe extern "C" fn skb_queue_init(mut list: *mut sk_buff_head) {
    list_init(&mut (*list).head);
    (*list).qlen = 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn skb_dequeue(mut list: *mut sk_buff_head) -> *mut sk_buff {
    let mut skb: *mut sk_buff = ((*list).head.next as *mut libc::c_char)
        .offset(-(0 as libc::c_ulong as isize)) as *mut sk_buff;
    list_del(&mut (*skb).list);
    (*list)
        .qlen = ((*list).qlen as libc::c_uint)
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
    return skb;
}
#[inline]
unsafe extern "C" fn skb_queue_empty(mut list: *const sk_buff_head) -> libc::c_int {
    return (skb_queue_len(list) < 1 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn skb_peek(mut list: *mut sk_buff_head) -> *mut sk_buff {
    if skb_queue_empty(list) != 0 {
        return 0 as *mut sk_buff;
    }
    return ((*list).head.next as *mut libc::c_char)
        .offset(-(0 as libc::c_ulong as isize)) as *mut sk_buff;
}
#[inline]
unsafe extern "C" fn skb_queue_free(mut list: *mut sk_buff_head) {
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    loop {
        skb = skb_peek(list);
        if skb.is_null() {
            break;
        }
        skb_dequeue(list);
        (*skb).refcnt -= 1;
        (*skb).refcnt;
        free_skb(skb);
    };
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
unsafe extern "C" fn wait_init(mut w: *mut wait_lock) -> libc::c_int {
    pthread_cond_init(&mut (*w).ready, 0 as *const pthread_condattr_t);
    pthread_mutex_init(&mut (*w).lock, 0 as *const pthread_mutexattr_t);
    (*w).sleeping = 0 as libc::c_int as uint8_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sk_alloc(
    mut ops: *mut net_ops,
    mut protocol: libc::c_int,
) -> *mut sock {
    let mut sk: *mut sock = 0 as *mut sock;
    sk = ((*ops).alloc_sock).unwrap()(protocol);
    (*sk).ops = ops;
    return sk;
}
pub unsafe extern "C" fn sock_init_data(mut sock: *mut socket, mut sk: *mut sock) {
    (*sock).sk = sk;
    (*sk).sock = sock;
    wait_init(&mut (*sk).recv_wait);
    skb_queue_init(&mut (*sk).receive_queue);
    skb_queue_init(&mut (*sk).write_queue);
    (*sk).poll_events = 0 as libc::c_int as libc::c_short;
    ((*(*sk).ops).init).unwrap()(sk);
}
pub unsafe extern "C" fn sock_free(mut sk: *mut sock) {
    skb_queue_free(&mut (*sk).receive_queue);
    skb_queue_free(&mut (*sk).write_queue);
}
pub unsafe extern "C" fn sock_connected(mut sk: *mut sock) {
    let mut sock: *mut socket = (*sk).sock;
    (*sock).state = SS_CONNECTED;
    (*sk).err = 0 as libc::c_int;
    (*sk)
        .poll_events = (0x4 as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int)
        as libc::c_short;
    wait_wakeup(&mut (*sock).sleep);
}
