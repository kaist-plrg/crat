use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn free(__ptr: *mut libc::c_void);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn socket_lookup(sport: uint16_t, dport: uint16_t) -> *mut socket;
    fn socket_wr_acquire(sock: *mut socket) -> libc::c_int;
    fn socket_release(sock: *mut socket) -> libc::c_int;
    fn sk_alloc(ops: *mut net_ops, protocol: libc::c_int) -> *mut sock;
    fn sock_free(sk: *mut sock);
    fn sock_init_data(sock: *mut socket, sk: *mut sock);
    static mut tcp_ops: net_ops;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_3 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_3 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_3 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_3 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_3 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_3 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_3 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_3 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_3 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_3 = 92;
pub const IPPROTO_AH: C2RustUnnamed_3 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_3 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_3 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_3 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_3 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_3 = 33;
pub const IPPROTO_TP: C2RustUnnamed_3 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_3 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_3 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_3 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_3 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_3 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_3 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_3 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_3 = 1;
pub const IPPROTO_IP: C2RustUnnamed_3 = 0;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
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
pub struct sock_type {
    pub sock_ops: *mut sock_ops,
    pub net_ops: *mut net_ops,
    pub type_0: libc::c_int,
    pub protocol: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct net_family {
    pub create: Option::<unsafe extern "C" fn(*mut socket, libc::c_int) -> libc::c_int>,
}
pub const TCP_CLOSE: tcp_states = 6;
pub type tcp_states = libc::c_uint;
pub const TCP_TIME_WAIT: tcp_states = 10;
pub const TCP_LAST_ACK: tcp_states = 9;
pub const TCP_CLOSING: tcp_states = 8;
pub const TCP_CLOSE_WAIT: tcp_states = 7;
pub const TCP_FIN_WAIT_2: tcp_states = 5;
pub const TCP_FIN_WAIT_1: tcp_states = 4;
pub const TCP_ESTABLISHED: tcp_states = 3;
pub const TCP_SYN_RECEIVED: tcp_states = 2;
pub const TCP_SYN_SENT: tcp_states = 1;
pub const TCP_LISTEN: tcp_states = 0;
#[inline]
unsafe extern "C" fn wait_sleep(mut w: *mut wait_lock) -> libc::c_int {
    (*w).sleeping = 1 as libc::c_int as uint8_t;
    pthread_cond_wait(&mut (*w).ready, &mut (*w).lock);
    return 0 as libc::c_int;
}
static mut INET_OPS: libc::c_int = 1 as libc::c_int;
pub static mut inet: net_family = unsafe {
    {
        let mut init = net_family {
            create: Some(
                inet_create
                    as unsafe extern "C" fn(*mut socket, libc::c_int) -> libc::c_int,
            ),
        };
        init
    }
};
static mut inet_stream_ops: sock_ops = {
    let mut init = sock_ops {
        connect: Some(
            inet_stream_connect
                as unsafe extern "C" fn(
                    *mut socket,
                    *const sockaddr,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        write: Some(
            inet_write
                as unsafe extern "C" fn(
                    *mut socket,
                    *const libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        read: Some(
            inet_read
                as unsafe extern "C" fn(
                    *mut socket,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        close: Some(inet_close as unsafe extern "C" fn(*mut socket) -> libc::c_int),
        free: Some(inet_free as unsafe extern "C" fn(*mut socket) -> libc::c_int),
        abort: Some(inet_abort as unsafe extern "C" fn(*mut socket) -> libc::c_int),
        poll: None,
        getpeername: Some(
            inet_getpeername
                as unsafe extern "C" fn(
                    *mut socket,
                    *mut sockaddr,
                    *mut socklen_t,
                ) -> libc::c_int,
        ),
        getsockname: Some(
            inet_getsockname
                as unsafe extern "C" fn(
                    *mut socket,
                    *mut sockaddr,
                    *mut socklen_t,
                ) -> libc::c_int,
        ),
    };
    init
};
static mut inet_ops: [sock_type; 1] = unsafe {
    [
        {
            let mut init = sock_type {
                sock_ops: &inet_stream_ops as *const sock_ops as *mut sock_ops,
                net_ops: &tcp_ops as *const net_ops as *mut net_ops,
                type_0: SOCK_STREAM as libc::c_int,
                protocol: IPPROTO_TCP as libc::c_int,
            };
            init
        },
    ]
};
pub unsafe extern "C" fn inet_create(
    mut sock: *mut socket,
    mut protocol: libc::c_int,
) -> libc::c_int {
    let mut sk: *mut sock = 0 as *mut sock;
    let mut skt: *mut sock_type = 0 as *mut sock_type;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < INET_OPS {
        if inet_ops[i as usize].type_0 & (*sock).type_0 as libc::c_int != 0 {
            skt = &mut *inet_ops.as_mut_ptr().offset(i as isize) as *mut sock_type;
            break;
        } else {
            i += 1;
            i;
        }
    }
    if skt.is_null() {
        fprintf(
            stderr,
            b"Could not find socktype for socket\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    (*sock).ops = (*skt).sock_ops;
    sk = sk_alloc((*skt).net_ops, protocol);
    (*sk).protocol = protocol;
    sock_init_data(sock, sk);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn inet_socket(
    mut sock: *mut socket,
    mut protocol: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn inet_connect(
    mut sock: *mut socket,
    mut addr: *mut sockaddr,
    mut addr_len: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn inet_stream_connect(
    mut sock: *mut socket,
    mut addr: *const sockaddr,
    mut addr_len: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut sk: *mut sock = (*sock).sk;
    let mut rc: libc::c_int = 0 as libc::c_int;
    if (addr_len as libc::c_ulong)
        < ::std::mem::size_of::<sa_family_t>() as libc::c_ulong
    {
        return -(22 as libc::c_int);
    }
    if (*addr).sa_family as libc::c_int == 0 as libc::c_int {
        ((*(*sk).ops).disconnect).unwrap()(sk, flags);
        return -(97 as libc::c_int);
    }
    match (*sock).state as libc::c_uint {
        3 => {
            (*sk).err = -(106 as libc::c_int);
        }
        2 => {
            (*sk).err = -(114 as libc::c_int);
        }
        1 => {
            (*sk).err = -(106 as libc::c_int);
            if !((*sk).state != TCP_CLOSE as libc::c_int) {
                ((*(*sk).ops).connect).unwrap()(sk, addr, addr_len, flags);
                (*sock).state = SS_CONNECTING;
                (*sk).err = -(115 as libc::c_int);
                if !((*sock).flags & 0o4000 as libc::c_int != 0) {
                    pthread_mutex_lock(&mut (*sock).sleep.lock);
                    while (*sock).state as libc::c_uint
                        == SS_CONNECTING as libc::c_int as libc::c_uint
                        && (*sk).err == -(115 as libc::c_int)
                    {
                        socket_release(sock);
                        wait_sleep(&mut (*sock).sleep);
                        socket_wr_acquire(sock);
                    }
                    pthread_mutex_unlock(&mut (*sock).sleep.lock);
                    socket_wr_acquire(sock);
                    match (*sk).err {
                        -110 | -111 => {
                            rc = (*sk).err;
                            return rc;
                        }
                        _ => {
                            if !((*sk).err != 0 as libc::c_int) {
                                (*sock).state = SS_CONNECTED;
                            }
                        }
                    }
                }
            }
        }
        _ => {
            (*sk).err = -(22 as libc::c_int);
        }
    }
    return (*sk).err;
}
pub unsafe extern "C" fn inet_write(
    mut sock: *mut socket,
    mut buf: *const libc::c_void,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut sk: *mut sock = (*sock).sk;
    return ((*(*sk).ops).write).unwrap()(sk, buf, len);
}
pub unsafe extern "C" fn inet_read(
    mut sock: *mut socket,
    mut buf: *mut libc::c_void,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut sk: *mut sock = (*sock).sk;
    return ((*(*sk).ops).read).unwrap()(sk, buf, len);
}
pub unsafe extern "C" fn inet_lookup(
    mut skb: *mut sk_buff,
    mut sport: uint16_t,
    mut dport: uint16_t,
) -> *mut sock {
    let mut sock: *mut socket = socket_lookup(sport, dport);
    if sock.is_null() {
        return 0 as *mut sock;
    }
    return (*sock).sk;
}
pub unsafe extern "C" fn inet_close(mut sock: *mut socket) -> libc::c_int {
    if sock.is_null() {
        return 0 as libc::c_int;
    }
    let mut sk: *mut sock = (*sock).sk;
    return ((*(*(*sock).sk).ops).close).unwrap()(sk);
}
pub unsafe extern "C" fn inet_free(mut sock: *mut socket) -> libc::c_int {
    let mut sk: *mut sock = (*sock).sk;
    sock_free(sk);
    free((*sock).sk as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn inet_abort(mut sock: *mut socket) -> libc::c_int {
    let mut sk: *mut sock = (*sock).sk;
    if !sk.is_null() {
        ((*(*sk).ops).abort).unwrap()(sk);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn inet_getpeername(
    mut sock: *mut socket,
    mut address: *mut sockaddr,
    mut address_len: *mut socklen_t,
) -> libc::c_int {
    let mut sk: *mut sock = (*sock).sk;
    if sk.is_null() {
        return -(1 as libc::c_int);
    }
    let mut res: *mut sockaddr_in = address as *mut sockaddr_in;
    (*res).sin_family = 2 as libc::c_int as sa_family_t;
    (*res).sin_port = htons((*sk).dport);
    (*res).sin_addr.s_addr = htonl((*sk).daddr);
    *address_len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn inet_getsockname(
    mut sock: *mut socket,
    mut address: *mut sockaddr,
    mut address_len: *mut socklen_t,
) -> libc::c_int {
    let mut sk: *mut sock = (*sock).sk;
    if sk.is_null() {
        return -(1 as libc::c_int);
    }
    let mut res: *mut sockaddr_in = address as *mut sockaddr_in;
    (*res).sin_family = 2 as libc::c_int as sa_family_t;
    (*res).sin_port = htons((*sk).sport);
    (*res).sin_addr.s_addr = htonl((*sk).saddr);
    *address_len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    return 0 as libc::c_int;
}
