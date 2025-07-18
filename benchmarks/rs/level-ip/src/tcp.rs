use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn rand() -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn abs(_: libc::c_int) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_rwlock_wrlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
    fn pthread_rwlock_unlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn parse_ipv4_string(addr: *mut libc::c_char) -> uint32_t;
    fn free_skb(skb: *mut sk_buff);
    fn checksum(
        addr: *mut libc::c_void,
        count: libc::c_int,
        start_sum: libc::c_int,
    ) -> uint16_t;
    fn socket_wr_acquire(sock: *mut socket) -> libc::c_int;
    fn socket_delete(sock: *mut socket) -> libc::c_int;
    fn socket_release(sock: *mut socket) -> libc::c_int;
    fn inet_lookup(skb: *mut sk_buff, sport: uint16_t, dport: uint16_t) -> *mut sock;
    fn timer_add(
        expire: uint32_t,
        handler: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        arg: *mut libc::c_void,
    ) -> *mut timer;
    fn timer_release(t: *mut timer);
    fn timer_cancel(t: *mut timer);
    fn timer_get_tick() -> libc::c_int;
    fn tcp_input_state(sk: *mut sock, th: *mut tcphdr, skb: *mut sk_buff) -> libc::c_int;
    fn tcp_connect(sk: *mut sock) -> libc::c_int;
    fn tcp_queue_fin(sk: *mut sock) -> libc::c_int;
    fn tcp_receive(
        tsk: *mut tcp_sock,
        buf: *mut libc::c_void,
        len: libc::c_int,
    ) -> libc::c_int;
    fn tcp_send(
        tsk: *mut tcp_sock,
        buf: *const libc::c_void,
        len: libc::c_int,
    ) -> libc::c_int;
    fn tcp_send_reset(tsk: *mut tcp_sock) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
pub type int32_t = __int32_t;
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
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const PTHREAD_RWLOCK_DEFAULT_NP: C2RustUnnamed_3 = 0;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: C2RustUnnamed_3 = 2;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NP: C2RustUnnamed_3 = 1;
pub const PTHREAD_RWLOCK_PREFER_READER_NP: C2RustUnnamed_3 = 0;
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
pub struct eth_hdr {
    pub dmac: [uint8_t; 6],
    pub smac: [uint8_t; 6],
    pub ethertype: uint16_t,
    pub payload: [uint8_t; 0],
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct iphdr {
    #[bitfield(name = "ihl", ty = "uint8_t", bits = "0..=3")]
    #[bitfield(name = "version", ty = "uint8_t", bits = "4..=7")]
    pub ihl_version: [u8; 1],
    pub tos: uint8_t,
    pub len: uint16_t,
    pub id: uint16_t,
    pub frag_offset: uint16_t,
    pub ttl: uint8_t,
    pub proto: uint8_t,
    pub csum: uint16_t,
    pub saddr: uint32_t,
    pub daddr: uint32_t,
    pub data: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timer {
    pub list: list_head,
    pub refcnt: libc::c_int,
    pub expires: uint32_t,
    pub cancelled: libc::c_int,
    pub handler: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    pub arg: *mut libc::c_void,
    pub lock: pthread_mutex_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct tcphdr {
    pub sport: uint16_t,
    pub dport: uint16_t,
    pub seq: uint32_t,
    pub ack_seq: uint32_t,
    #[bitfield(name = "rsvd", ty = "uint8_t", bits = "0..=3")]
    #[bitfield(name = "hl", ty = "uint8_t", bits = "4..=7")]
    #[bitfield(name = "fin", ty = "uint8_t", bits = "8..=8")]
    #[bitfield(name = "syn", ty = "uint8_t", bits = "9..=9")]
    #[bitfield(name = "rst", ty = "uint8_t", bits = "10..=10")]
    #[bitfield(name = "psh", ty = "uint8_t", bits = "11..=11")]
    #[bitfield(name = "ack", ty = "uint8_t", bits = "12..=12")]
    #[bitfield(name = "urg", ty = "uint8_t", bits = "13..=13")]
    #[bitfield(name = "ece", ty = "uint8_t", bits = "14..=14")]
    #[bitfield(name = "cwr", ty = "uint8_t", bits = "15..=15")]
    pub rsvd_hl_fin_syn_rst_psh_ack_urg_ece_cwr: [u8; 2],
    pub win: uint16_t,
    pub csum: uint16_t,
    pub urp: uint16_t,
    pub data: [uint8_t; 0],
}
pub type tcp_states = libc::c_uint;
pub const TCP_TIME_WAIT: tcp_states = 10;
pub const TCP_LAST_ACK: tcp_states = 9;
pub const TCP_CLOSING: tcp_states = 8;
pub const TCP_CLOSE_WAIT: tcp_states = 7;
pub const TCP_CLOSE: tcp_states = 6;
pub const TCP_FIN_WAIT_2: tcp_states = 5;
pub const TCP_FIN_WAIT_1: tcp_states = 4;
pub const TCP_ESTABLISHED: tcp_states = 3;
pub const TCP_SYN_RECEIVED: tcp_states = 2;
pub const TCP_SYN_SENT: tcp_states = 1;
pub const TCP_LISTEN: tcp_states = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tcb {
    pub snd_una: uint32_t,
    pub snd_nxt: uint32_t,
    pub snd_wnd: uint32_t,
    pub snd_up: uint32_t,
    pub snd_wl1: uint32_t,
    pub snd_wl2: uint32_t,
    pub iss: uint32_t,
    pub rcv_nxt: uint32_t,
    pub rcv_wnd: uint32_t,
    pub rcv_up: uint32_t,
    pub irs: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tcp_sack_block {
    pub left: uint32_t,
    pub right: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tcp_sock {
    pub sk: sock,
    pub fd: libc::c_int,
    pub tcp_header_len: uint16_t,
    pub tcb: tcb,
    pub flags: uint8_t,
    pub backoff: uint8_t,
    pub srtt: int32_t,
    pub rttvar: int32_t,
    pub rto: uint32_t,
    pub retransmit: *mut timer,
    pub delack: *mut timer,
    pub keepalive: *mut timer,
    pub linger: *mut timer,
    pub delacks: uint8_t,
    pub rmss: uint16_t,
    pub smss: uint16_t,
    pub cwnd: uint16_t,
    pub inflight: uint32_t,
    pub sackok: uint8_t,
    pub sacks_allowed: uint8_t,
    pub sacklen: uint8_t,
    pub sacks: [tcp_sack_block; 4],
    pub tsopt: uint8_t,
    pub ofo_queue: sk_buff_head,
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
unsafe extern "C" fn ip_hdr(mut skb: *const sk_buff) -> *mut iphdr {
    return ((*skb).head)
        .offset(::std::mem::size_of::<eth_hdr>() as libc::c_ulong as isize)
        as *mut iphdr;
}
static mut tcplock: pthread_rwlock_t = pthread_rwlock_t {
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
pub static mut tcp_ops: net_ops = unsafe {
    {
        let mut init = net_ops {
            alloc_sock: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> *mut sock>,
                Option::<unsafe extern "C" fn(libc::c_int) -> *mut sock>,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> *mut sock,
                        unsafe extern "C" fn() -> *mut sock,
                    >(tcp_alloc_sock),
                ),
            ),
            init: Some(
                tcp_v4_init_sock as unsafe extern "C" fn(*mut sock) -> libc::c_int,
            ),
            connect: Some(
                tcp_v4_connect
                    as unsafe extern "C" fn(
                        *mut sock,
                        *const sockaddr,
                        libc::c_int,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            disconnect: Some(
                tcp_disconnect
                    as unsafe extern "C" fn(*mut sock, libc::c_int) -> libc::c_int,
            ),
            write: Some(
                tcp_write
                    as unsafe extern "C" fn(
                        *mut sock,
                        *const libc::c_void,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            read: Some(
                tcp_read
                    as unsafe extern "C" fn(
                        *mut sock,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            recv_notify: Some(
                tcp_recv_notify as unsafe extern "C" fn(*mut sock) -> libc::c_int,
            ),
            close: Some(tcp_close as unsafe extern "C" fn(*mut sock) -> libc::c_int),
            abort: Some(tcp_abort as unsafe extern "C" fn(*mut sock) -> libc::c_int),
        };
        init
    }
};
pub unsafe extern "C" fn tcp_init() {}
unsafe extern "C" fn tcp_init_segment(
    mut th: *mut tcphdr,
    mut ih: *mut iphdr,
    mut skb: *mut sk_buff,
) {
    (*th).sport = ntohs((*th).sport);
    (*th).dport = ntohs((*th).dport);
    (*th).seq = ntohl((*th).seq);
    (*th).ack_seq = ntohl((*th).ack_seq);
    (*th).win = ntohs((*th).win);
    (*th).csum = ntohs((*th).csum);
    (*th).urp = ntohs((*th).urp);
    (*skb).seq = (*th).seq;
    (*skb)
        .dlen = ((*ih).len as libc::c_int - (*ih).ihl() as libc::c_int * 4 as libc::c_int
        - (((*th).hl() as libc::c_int) << 2 as libc::c_int)) as uint32_t;
    (*skb)
        .len = ((*skb).dlen)
        .wrapping_add((*th).syn() as libc::c_uint)
        .wrapping_add((*th).fin() as libc::c_uint);
    (*skb).end_seq = ((*skb).seq).wrapping_add((*skb).dlen);
    (*skb).payload = ((*th).data).as_mut_ptr();
}
unsafe extern "C" fn tcp_clear_queues(mut tsk: *mut tcp_sock) {
    skb_queue_free(&mut (*tsk).ofo_queue);
}
pub unsafe extern "C" fn tcp_in(mut skb: *mut sk_buff) {
    let mut sk: *mut sock = 0 as *mut sock;
    let mut iph: *mut iphdr = 0 as *mut iphdr;
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    iph = ip_hdr(skb);
    th = ((*iph).data).as_mut_ptr() as *mut tcphdr;
    tcp_init_segment(th, iph, skb);
    sk = inet_lookup(skb, (*th).sport, (*th).dport);
    if sk.is_null() {
        fprintf(
            stderr,
            b"No TCP socket for sport %d dport %d\n\0" as *const u8
                as *const libc::c_char,
            (*th).sport as libc::c_int,
            (*th).dport as libc::c_int,
        );
        free_skb(skb);
        return;
    }
    socket_wr_acquire((*sk).sock);
    tcp_input_state(sk, th, skb);
    socket_release((*sk).sock);
}
pub unsafe extern "C" fn tcp_udp_checksum(
    mut saddr: uint32_t,
    mut daddr: uint32_t,
    mut proto: uint8_t,
    mut data: *mut uint8_t,
    mut len: uint16_t,
) -> libc::c_int {
    let mut sum: uint32_t = 0 as libc::c_int as uint32_t;
    sum = (sum as libc::c_uint).wrapping_add(saddr) as uint32_t as uint32_t;
    sum = (sum as libc::c_uint).wrapping_add(daddr) as uint32_t as uint32_t;
    sum = (sum as libc::c_uint).wrapping_add(htons(proto as uint16_t) as libc::c_uint)
        as uint32_t as uint32_t;
    sum = (sum as libc::c_uint).wrapping_add(htons(len) as libc::c_uint) as uint32_t
        as uint32_t;
    return checksum(data as *mut libc::c_void, len as libc::c_int, sum as libc::c_int)
        as libc::c_int;
}
pub unsafe extern "C" fn tcp_v4_checksum(
    mut skb: *mut sk_buff,
    mut saddr: uint32_t,
    mut daddr: uint32_t,
) -> libc::c_int {
    return tcp_udp_checksum(
        saddr,
        daddr,
        0x6 as libc::c_int as uint8_t,
        (*skb).data,
        (*skb).len as uint16_t,
    );
}
pub unsafe extern "C" fn tcp_alloc_sock() -> *mut sock {
    let mut tsk: *mut tcp_sock = malloc(
        ::std::mem::size_of::<tcp_sock>() as libc::c_ulong,
    ) as *mut tcp_sock;
    memset(
        tsk as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<tcp_sock>() as libc::c_ulong,
    );
    (*tsk).sk.state = TCP_CLOSE as libc::c_int;
    (*tsk).sackok = 1 as libc::c_int as uint8_t;
    (*tsk).rmss = 1460 as libc::c_int as uint16_t;
    (*tsk).smss = 536 as libc::c_int as uint16_t;
    skb_queue_init(&mut (*tsk).ofo_queue);
    return tsk as *mut sock;
}
pub unsafe extern "C" fn tcp_v4_init_sock(mut sk: *mut sock) -> libc::c_int {
    tcp_init_sock(sk);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tcp_init_sock(mut sk: *mut sock) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn __tcp_set_state(mut sk: *mut sock, mut state: uint32_t) {
    (*sk).state = state as libc::c_int;
}
unsafe extern "C" fn generate_port() -> uint16_t {
    static mut port: libc::c_int = 40000 as libc::c_int;
    pthread_rwlock_wrlock(&mut tcplock);
    port += 1;
    let mut copy: libc::c_int = port + timer_get_tick() % 10000 as libc::c_int;
    pthread_rwlock_unlock(&mut tcplock);
    return copy as uint16_t;
}
pub unsafe extern "C" fn generate_iss() -> libc::c_int {
    return time(0 as *mut time_t) as libc::c_int * rand();
}
pub unsafe extern "C" fn tcp_v4_connect(
    mut sk: *mut sock,
    mut addr: *const sockaddr,
    mut addrlen: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut dport: uint16_t = (*(addr as *mut sockaddr_in)).sin_port;
    let mut daddr: uint32_t = (*(addr as *mut sockaddr_in)).sin_addr.s_addr;
    (*sk).dport = ntohs(dport);
    (*sk).sport = generate_port();
    (*sk).daddr = ntohl(daddr);
    (*sk)
        .saddr = parse_ipv4_string(
        b"10.0.0.4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return tcp_connect(sk);
}
pub unsafe extern "C" fn tcp_disconnect(
    mut sk: *mut sock,
    mut flags: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tcp_write(
    mut sk: *mut sock,
    mut buf: *const libc::c_void,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut tsk: *mut tcp_sock = sk as *mut tcp_sock;
    let mut ret: libc::c_int = (*sk).err;
    if !(ret != 0 as libc::c_int) {
        match (*sk).state {
            3 | 7 => return tcp_send(tsk, buf, len),
            _ => {
                ret = -(9 as libc::c_int);
            }
        }
    }
    return ret;
}
pub unsafe extern "C" fn tcp_read(
    mut sk: *mut sock,
    mut buf: *mut libc::c_void,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut tsk: *mut tcp_sock = sk as *mut tcp_sock;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    match (*sk).state {
        6 => {
            ret = -(9 as libc::c_int);
            current_block = 3400886120766329092;
        }
        0 | 1 | 2 => {
            current_block = 12209867499936983673;
        }
        3 | 4 | 5 => {
            current_block = 12209867499936983673;
        }
        7 => {
            if skb_queue_empty(&mut (*tsk).sk.receive_queue) == 0 {
                current_block = 12209867499936983673;
            } else {
                if (*tsk).flags as libc::c_int & 0x1 as libc::c_int != 0 {
                    (*tsk)
                        .flags = ((*tsk).flags as libc::c_int & !(0x1 as libc::c_int))
                        as uint8_t;
                    return 0 as libc::c_int;
                }
                current_block = 12209867499936983673;
            }
        }
        8 | 9 | 10 => {
            ret = (*sk).err;
            current_block = 3400886120766329092;
        }
        _ => {
            current_block = 3400886120766329092;
        }
    }
    match current_block {
        3400886120766329092 => return ret,
        _ => return tcp_receive(tsk, buf, len),
    };
}
pub unsafe extern "C" fn tcp_recv_notify(mut sk: *mut sock) -> libc::c_int {
    if !(&mut (*sk).recv_wait as *mut wait_lock).is_null() {
        return wait_wakeup(&mut (*sk).recv_wait);
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn tcp_close(mut sk: *mut sock) -> libc::c_int {
    match (*sk).state {
        6 | 8 | 9 | 10 | 4 | 5 => {
            (*sk).err = -(9 as libc::c_int);
            return -(1 as libc::c_int);
        }
        0 | 1 | 2 => return tcp_done(sk),
        3 => {
            __tcp_set_state(sk, TCP_FIN_WAIT_1 as libc::c_int as uint32_t);
            tcp_queue_fin(sk);
        }
        7 => {
            tcp_queue_fin(sk);
        }
        _ => {
            fprintf(
                stderr,
                b"Unknown TCP state for close\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tcp_abort(mut sk: *mut sock) -> libc::c_int {
    let mut tsk: *mut tcp_sock = sk as *mut tcp_sock;
    tcp_send_reset(tsk);
    return tcp_done(sk);
}
unsafe extern "C" fn tcp_free(mut sk: *mut sock) -> libc::c_int {
    let mut tsk: *mut tcp_sock = sk as *mut tcp_sock;
    tcp_clear_timers(sk);
    tcp_clear_queues(tsk);
    wait_wakeup(&mut (*(*sk).sock).sleep);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tcp_done(mut sk: *mut sock) -> libc::c_int {
    __tcp_set_state(sk, TCP_CLOSING as libc::c_int as uint32_t);
    tcp_free(sk);
    return socket_delete((*sk).sock);
}
pub unsafe extern "C" fn tcp_clear_timers(mut sk: *mut sock) {
    let mut tsk: *mut tcp_sock = sk as *mut tcp_sock;
    tcp_stop_rto_timer(tsk);
    tcp_stop_delack_timer(tsk);
    timer_cancel((*tsk).keepalive);
    (*tsk).keepalive = 0 as *mut timer;
    timer_cancel((*tsk).linger);
    (*tsk).linger = 0 as *mut timer;
}
pub unsafe extern "C" fn tcp_stop_rto_timer(mut tsk: *mut tcp_sock) {
    if !tsk.is_null() {
        timer_cancel((*tsk).retransmit);
        (*tsk).retransmit = 0 as *mut timer;
        (*tsk).backoff = 0 as libc::c_int as uint8_t;
    }
}
pub unsafe extern "C" fn tcp_release_rto_timer(mut tsk: *mut tcp_sock) {
    if !tsk.is_null() {
        timer_release((*tsk).retransmit);
        (*tsk).retransmit = 0 as *mut timer;
    }
}
pub unsafe extern "C" fn tcp_stop_delack_timer(mut tsk: *mut tcp_sock) {
    timer_cancel((*tsk).delack);
    (*tsk).delack = 0 as *mut timer;
}
pub unsafe extern "C" fn tcp_release_delack_timer(mut tsk: *mut tcp_sock) {
    timer_release((*tsk).delack);
    (*tsk).delack = 0 as *mut timer;
}
pub unsafe extern "C" fn tcp_handle_fin_state(mut sk: *mut sock) {
    match (*sk).state {
        7 => {
            __tcp_set_state(sk, TCP_LAST_ACK as libc::c_int as uint32_t);
        }
        3 => {
            __tcp_set_state(sk, TCP_FIN_WAIT_1 as libc::c_int as uint32_t);
        }
        _ => {}
    };
}
unsafe extern "C" fn tcp_linger(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut sk: *mut sock = arg as *mut sock;
    socket_wr_acquire((*sk).sock);
    let mut tsk: *mut tcp_sock = sk as *mut tcp_sock;
    timer_cancel((*tsk).linger);
    (*tsk).linger = 0 as *mut timer;
    tcp_done(sk);
    socket_release((*sk).sock);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn tcp_user_timeout(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut sk: *mut sock = arg as *mut sock;
    socket_wr_acquire((*sk).sock);
    let mut tsk: *mut tcp_sock = sk as *mut tcp_sock;
    timer_cancel((*tsk).linger);
    (*tsk).linger = 0 as *mut timer;
    tcp_abort(sk);
    socket_release((*sk).sock);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn tcp_enter_time_wait(mut sk: *mut sock) {
    let mut tsk: *mut tcp_sock = sk as *mut tcp_sock;
    __tcp_set_state(sk, TCP_TIME_WAIT as libc::c_int as uint32_t);
    tcp_clear_timers(sk);
    (*tsk)
        .linger = timer_add(
        60000 as libc::c_int as uint32_t,
        Some(tcp_linger as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        sk as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn tcp_rearm_user_timeout(mut sk: *mut sock) {
    let mut tsk: *mut tcp_sock = sk as *mut tcp_sock;
    if (*sk).state == TCP_TIME_WAIT as libc::c_int {
        return;
    }
    timer_cancel((*tsk).linger);
    (*tsk)
        .linger = timer_add(
        180000 as libc::c_int as uint32_t,
        Some(
            tcp_user_timeout
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        sk as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn tcp_rtt(mut tsk: *mut tcp_sock) {
    if (*tsk).backoff as libc::c_int > 0 as libc::c_int || ((*tsk).retransmit).is_null()
    {
        return;
    }
    let mut r: libc::c_int = (timer_get_tick() as libc::c_uint)
        .wrapping_sub(((*(*tsk).retransmit).expires).wrapping_sub((*tsk).rto))
        as libc::c_int;
    if r < 0 as libc::c_int {
        return;
    }
    if (*tsk).srtt == 0 {
        (*tsk).srtt = r;
        (*tsk).rttvar = r / 2 as libc::c_int;
    } else {
        let mut beta: libc::c_double = 0.25f64;
        let mut alpha: libc::c_double = 0.125f64;
        (*tsk)
            .rttvar = ((1 as libc::c_int as libc::c_double - beta)
            * (*tsk).rttvar as libc::c_double
            + beta * abs((*tsk).srtt - r) as libc::c_double) as int32_t;
        (*tsk)
            .srtt = ((1 as libc::c_int as libc::c_double - alpha)
            * (*tsk).srtt as libc::c_double + alpha * r as libc::c_double) as int32_t;
    }
    let mut k: libc::c_int = 4 as libc::c_int * (*tsk).rttvar;
    if k < 200 as libc::c_int {
        k = 200 as libc::c_int;
    }
    (*tsk).rto = ((*tsk).srtt + k) as uint32_t;
}
pub unsafe extern "C" fn tcp_calculate_sacks(mut tsk: *mut tcp_sock) -> libc::c_int {
    let mut sb: *mut tcp_sack_block = &mut *((*tsk).sacks)
        .as_mut_ptr()
        .offset((*tsk).sacklen as isize) as *mut tcp_sack_block;
    (*sb).left = 0 as libc::c_int as uint32_t;
    (*sb).right = 0 as libc::c_int as uint32_t;
    let mut next: *mut sk_buff = 0 as *mut sk_buff;
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut tmp: *mut list_head = 0 as *mut list_head;
    item = (*tsk).ofo_queue.head.next;
    tmp = (*item).next;
    while item != &mut (*tsk).ofo_queue.head as *mut list_head {
        next = (item as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut sk_buff;
        if (*sb).left == 0 as libc::c_int as libc::c_uint {
            (*sb).left = (*next).seq;
            (*tsk).sacklen = ((*tsk).sacklen).wrapping_add(1);
            (*tsk).sacklen;
        }
        if (*sb).right == 0 as libc::c_int as libc::c_uint {
            (*sb).right = (*next).end_seq;
        } else if (*sb).right == (*next).seq {
            (*sb).right = (*next).end_seq;
        } else {
            if (*tsk).sacklen as libc::c_int >= (*tsk).sacks_allowed as libc::c_int {
                break;
            }
            sb = &mut *((*tsk).sacks).as_mut_ptr().offset((*tsk).sacklen as isize)
                as *mut tcp_sack_block;
            (*sb).left = (*next).seq;
            (*sb).right = (*next).end_seq;
            (*tsk).sacklen = ((*tsk).sacklen).wrapping_add(1);
            (*tsk).sacklen;
        }
        item = tmp;
        tmp = (*item).next;
    }
    return 0 as libc::c_int;
}
