use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn alloc_skb(size: libc::c_uint) -> *mut sk_buff;
    fn free_skb(skb: *mut sk_buff);
    fn skb_push(skb: *mut sk_buff, len: libc::c_uint) -> *mut uint8_t;
    fn skb_reserve(skb: *mut sk_buff, len: libc::c_uint) -> *mut libc::c_void;
    fn skb_reset_header(skb: *mut sk_buff);
    fn socket_wr_acquire(sock: *mut socket) -> libc::c_int;
    fn socket_release(sock: *mut socket) -> libc::c_int;
    fn ip_output(sk: *mut sock, skb: *mut sk_buff) -> libc::c_int;
    fn timer_add(
        expire: uint32_t,
        handler: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        arg: *mut libc::c_void,
    ) -> *mut timer;
    fn generate_iss() -> libc::c_int;
    fn tcp_v4_checksum(
        skb: *mut sk_buff,
        saddr: uint32_t,
        daddr: uint32_t,
    ) -> libc::c_int;
    fn tcp_handle_fin_state(sk: *mut sock);
    fn tcp_done(sk: *mut sock) -> libc::c_int;
    fn tcp_release_rto_timer(tsk: *mut tcp_sock);
    fn tcp_rearm_user_timeout(sk: *mut sock);
    fn tcp_release_delack_timer(tsk: *mut tcp_sock);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __socklen_t = libc::c_uint;
pub type pid_t = __pid_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
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
pub struct netdev {
    pub addr: uint32_t,
    pub addr_len: uint8_t,
    pub hwaddr: [uint8_t; 6],
    pub mtu: uint32_t,
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
pub struct sk_buff_head {
    pub head: list_head,
    pub qlen: uint32_t,
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
pub struct wait_lock {
    pub ready: pthread_cond_t,
    pub lock: pthread_mutex_t,
    pub sleeping: uint8_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tcp_options {
    pub options: uint16_t,
    pub mss: uint16_t,
    pub sack: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tcp_opt_mss {
    pub kind: uint8_t,
    pub len: uint8_t,
    pub mss: uint16_t,
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
unsafe extern "C" fn skb_queue_len(mut list: *const sk_buff_head) -> uint32_t {
    return (*list).qlen;
}
#[inline]
unsafe extern "C" fn skb_queue_tail(mut list: *mut sk_buff_head, mut new: *mut sk_buff) {
    list_add_tail(&mut (*new).list, &mut (*list).head);
    (*list)
        .qlen = ((*list).qlen as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
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
unsafe extern "C" fn wait_wakeup(mut w: *mut wait_lock) -> libc::c_int {
    pthread_mutex_lock(&mut (*w).lock);
    pthread_cond_signal(&mut (*w).ready);
    (*w).sleeping = 0 as libc::c_int as uint8_t;
    pthread_mutex_unlock(&mut (*w).lock);
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn write_queue_head(mut sk: *mut sock) -> *mut sk_buff {
    return skb_peek(&mut (*sk).write_queue);
}
#[inline]
unsafe extern "C" fn tcp_hdr(mut skb: *const sk_buff) -> *mut tcphdr {
    return ((*skb).head)
        .offset(::std::mem::size_of::<eth_hdr>() as libc::c_ulong as isize)
        .offset(::std::mem::size_of::<iphdr>() as libc::c_ulong as isize) as *mut tcphdr;
}
#[inline]
unsafe extern "C" fn list_add_tail(mut new: *mut list_head, mut head: *mut list_head) {
    (*(*head).prev).next = new;
    (*new).prev = (*head).prev;
    (*new).next = head;
    (*head).prev = new;
}
unsafe extern "C" fn tcp_alloc_skb(
    mut optlen: libc::c_int,
    mut size: libc::c_int,
) -> *mut sk_buff {
    let mut reserved: libc::c_int = (::std::mem::size_of::<eth_hdr>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<iphdr>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<tcphdr>() as libc::c_ulong)
        .wrapping_add(optlen as libc::c_ulong)
        .wrapping_add(size as libc::c_ulong) as libc::c_int;
    let mut skb: *mut sk_buff = alloc_skb(reserved as libc::c_uint);
    skb_reserve(skb, reserved as libc::c_uint);
    (*skb).protocol = 0x6 as libc::c_int as uint16_t;
    (*skb).dlen = size as uint32_t;
    (*skb).seq = 0 as libc::c_int as uint32_t;
    return skb;
}
unsafe extern "C" fn tcp_write_syn_options(
    mut th: *mut tcphdr,
    mut opts: *mut tcp_options,
    mut optlen: libc::c_int,
) -> libc::c_int {
    let mut opt_mss: *mut tcp_opt_mss = ((*th).data).as_mut_ptr() as *mut tcp_opt_mss;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    (*opt_mss).kind = 2 as libc::c_int as uint8_t;
    (*opt_mss).len = 4 as libc::c_int as uint8_t;
    (*opt_mss).mss = htons((*opts).mss);
    i = (i as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<tcp_opt_mss>() as libc::c_ulong) as uint32_t
        as uint32_t;
    if (*opts).sack != 0 {
        let fresh0 = i;
        i = i.wrapping_add(1);
        *((*th).data).as_mut_ptr().offset(fresh0 as isize) = 1 as libc::c_int as uint8_t;
        let fresh1 = i;
        i = i.wrapping_add(1);
        *((*th).data).as_mut_ptr().offset(fresh1 as isize) = 1 as libc::c_int as uint8_t;
        let fresh2 = i;
        i = i.wrapping_add(1);
        *((*th).data).as_mut_ptr().offset(fresh2 as isize) = 4 as libc::c_int as uint8_t;
        let fresh3 = i;
        i = i.wrapping_add(1);
        *((*th).data).as_mut_ptr().offset(fresh3 as isize) = 2 as libc::c_int as uint8_t;
    }
    (*th)
        .set_hl(
            (::std::mem::size_of::<tcphdr>() as libc::c_ulong)
                .wrapping_div(4 as libc::c_int as libc::c_ulong)
                .wrapping_add((optlen / 4 as libc::c_int) as libc::c_ulong) as uint8_t,
        );
    return 0 as libc::c_int;
}
unsafe extern "C" fn tcp_syn_options(
    mut sk: *mut sock,
    mut opts: *mut tcp_options,
) -> libc::c_int {
    let mut tsk: *mut tcp_sock = sk as *mut tcp_sock;
    let mut optlen: libc::c_int = 0 as libc::c_int;
    (*opts).mss = (*tsk).rmss;
    optlen += 4 as libc::c_int;
    if (*tsk).sackok != 0 {
        (*opts).sack = 1 as libc::c_int as uint8_t;
        optlen += 1 as libc::c_int * 2 as libc::c_int;
        optlen += 2 as libc::c_int;
    } else {
        (*opts).sack = 0 as libc::c_int as uint8_t;
    }
    return optlen;
}
unsafe extern "C" fn tcp_write_options(
    mut tsk: *mut tcp_sock,
    mut th: *mut tcphdr,
) -> libc::c_int {
    let mut ptr: *mut uint8_t = ((*th).data).as_mut_ptr();
    if (*tsk).sackok == 0
        || (*tsk).sacks[0 as libc::c_int as usize].left
            == 0 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    let fresh4 = ptr;
    ptr = ptr.offset(1);
    *fresh4 = 1 as libc::c_int as uint8_t;
    let fresh5 = ptr;
    ptr = ptr.offset(1);
    *fresh5 = 1 as libc::c_int as uint8_t;
    let fresh6 = ptr;
    ptr = ptr.offset(1);
    *fresh6 = 5 as libc::c_int as uint8_t;
    let fresh7 = ptr;
    ptr = ptr.offset(1);
    *fresh7 = (2 as libc::c_int + (*tsk).sacklen as libc::c_int * 8 as libc::c_int)
        as uint8_t;
    let mut sb: *mut tcp_sack_block = ptr as *mut tcp_sack_block;
    let mut i: libc::c_int = (*tsk).sacklen as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        (*sb).left = htonl((*tsk).sacks[i as usize].left);
        (*sb).right = htonl((*tsk).sacks[i as usize].right);
        (*tsk).sacks[i as usize].left = 0 as libc::c_int as uint32_t;
        (*tsk).sacks[i as usize].right = 0 as libc::c_int as uint32_t;
        sb = sb.offset(1 as libc::c_int as isize);
        ptr = ptr
            .offset(::std::mem::size_of::<tcp_sack_block>() as libc::c_ulong as isize);
        i -= 1;
        i;
    }
    (*tsk).sacklen = 0 as libc::c_int as uint8_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn tcp_transmit_skb(
    mut sk: *mut sock,
    mut skb: *mut sk_buff,
    mut seq: uint32_t,
) -> libc::c_int {
    let mut tsk: *mut tcp_sock = sk as *mut tcp_sock;
    let mut tcb: *mut tcb = &mut (*tsk).tcb;
    let mut thdr: *mut tcphdr = tcp_hdr(skb);
    if (*thdr).hl() as libc::c_int == 0 as libc::c_int {
        (*thdr)
            .set_hl(
                (::std::mem::size_of::<tcphdr>() as libc::c_ulong)
                    .wrapping_div(4 as libc::c_int as libc::c_ulong) as uint8_t,
            );
    }
    skb_push(skb, ((*thdr).hl() as libc::c_int * 4 as libc::c_int) as libc::c_uint);
    (*thdr).sport = (*sk).sport;
    (*thdr).dport = (*sk).dport;
    (*thdr).seq = seq;
    (*thdr).ack_seq = (*tcb).rcv_nxt;
    (*thdr).set_rsvd(0 as libc::c_int as uint8_t);
    (*thdr).win = (*tcb).rcv_wnd as uint16_t;
    (*thdr).csum = 0 as libc::c_int as uint16_t;
    (*thdr).urp = 0 as libc::c_int as uint16_t;
    if (*thdr).hl() as libc::c_int > 5 as libc::c_int {
        tcp_write_options(tsk, thdr);
    }
    (*thdr).sport = htons((*thdr).sport);
    (*thdr).dport = htons((*thdr).dport);
    (*thdr).seq = htonl((*thdr).seq);
    (*thdr).ack_seq = htonl((*thdr).ack_seq);
    (*thdr).win = htons((*thdr).win);
    (*thdr).csum = htons((*thdr).csum);
    (*thdr).urp = htons((*thdr).urp);
    (*thdr)
        .csum = tcp_v4_checksum(skb, htonl((*sk).saddr), htonl((*sk).daddr)) as uint16_t;
    return ip_output(sk, skb);
}
unsafe extern "C" fn tcp_queue_transmit_skb(
    mut sk: *mut sock,
    mut skb: *mut sk_buff,
) -> libc::c_int {
    let mut tsk: *mut tcp_sock = sk as *mut tcp_sock;
    let mut tcb: *mut tcb = &mut (*tsk).tcb;
    let mut th: *mut tcphdr = tcp_hdr(skb);
    let mut rc: libc::c_int = 0 as libc::c_int;
    if skb_queue_empty(&mut (*sk).write_queue) != 0 {
        tcp_rearm_rto_timer(tsk);
    }
    if (*tsk).inflight == 0 as libc::c_int as libc::c_uint {
        rc = tcp_transmit_skb(sk, skb, (*tcb).snd_nxt);
        (*tsk).inflight = ((*tsk).inflight).wrapping_add(1);
        (*tsk).inflight;
        (*skb).seq = (*tcb).snd_nxt;
        (*tcb)
            .snd_nxt = ((*tcb).snd_nxt as libc::c_uint).wrapping_add((*skb).dlen)
            as uint32_t as uint32_t;
        (*skb).end_seq = (*tcb).snd_nxt;
        if (*th).fin() != 0 {
            (*tcb).snd_nxt = ((*tcb).snd_nxt).wrapping_add(1);
            (*tcb).snd_nxt;
        }
    }
    skb_queue_tail(&mut (*sk).write_queue, skb);
    return rc;
}
pub unsafe extern "C" fn tcp_send_synack(mut sk: *mut sock) -> libc::c_int {
    if (*sk).state != TCP_SYN_SENT as libc::c_int {
        fprintf(
            stderr,
            b"TCP synack: Socket was not in correct state (SYN_SENT)\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    let mut tcb: *mut tcb = &mut (*(sk as *mut tcp_sock)).tcb;
    let mut rc: libc::c_int = 0 as libc::c_int;
    skb = tcp_alloc_skb(0 as libc::c_int, 0 as libc::c_int);
    th = tcp_hdr(skb);
    (*th).set_syn(1 as libc::c_int as uint8_t);
    (*th).set_ack(1 as libc::c_int as uint8_t);
    rc = tcp_transmit_skb(sk, skb, (*tcb).snd_nxt);
    free_skb(skb);
    return rc;
}
pub unsafe extern "C" fn tcp_send_delack(
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut sk: *mut sock = arg as *mut sock;
    socket_wr_acquire((*sk).sock);
    let mut tsk: *mut tcp_sock = sk as *mut tcp_sock;
    (*tsk).delacks = 0 as libc::c_int as uint8_t;
    tcp_release_delack_timer(tsk);
    tcp_send_ack(sk);
    socket_release((*sk).sock);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn tcp_send_next(
    mut sk: *mut sock,
    mut amount: libc::c_int,
) -> libc::c_int {
    let mut tsk: *mut tcp_sock = sk as *mut tcp_sock;
    let mut tcb: *mut tcb = &mut (*tsk).tcb;
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    let mut next: *mut sk_buff = 0 as *mut sk_buff;
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut tmp: *mut list_head = 0 as *mut list_head;
    let mut i: libc::c_int = 0 as libc::c_int;
    item = (*sk).write_queue.head.next;
    tmp = (*item).next;
    while item != &mut (*sk).write_queue.head as *mut list_head {
        i += 1;
        if i > amount {
            break;
        }
        next = (item as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut sk_buff;
        if next.is_null() {
            return -(1 as libc::c_int);
        }
        skb_reset_header(next);
        tcp_transmit_skb(sk, next, (*tcb).snd_nxt);
        (*next).seq = (*tcb).snd_nxt;
        (*tcb)
            .snd_nxt = ((*tcb).snd_nxt as libc::c_uint).wrapping_add((*next).dlen)
            as uint32_t as uint32_t;
        (*next).end_seq = (*tcb).snd_nxt;
        th = tcp_hdr(next);
        if (*th).fin() != 0 {
            (*tcb).snd_nxt = ((*tcb).snd_nxt).wrapping_add(1);
            (*tcb).snd_nxt;
        }
        item = tmp;
        tmp = (*item).next;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn tcp_options_len(mut sk: *mut sock) -> libc::c_int {
    let mut tsk: *mut tcp_sock = sk as *mut tcp_sock;
    let mut optlen: uint8_t = 0 as libc::c_int as uint8_t;
    if (*tsk).sackok as libc::c_int != 0
        && (*tsk).sacklen as libc::c_int > 0 as libc::c_int
    {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*tsk).sacklen as libc::c_int {
            if (*tsk).sacks[i as usize].left != 0 as libc::c_int as libc::c_uint {
                optlen = (optlen as libc::c_int + 8 as libc::c_int) as uint8_t;
            }
            i += 1;
            i;
        }
        optlen = (optlen as libc::c_int + 2 as libc::c_int) as uint8_t;
    }
    while optlen as libc::c_int % 4 as libc::c_int > 0 as libc::c_int {
        optlen = optlen.wrapping_add(1);
        optlen;
    }
    return optlen as libc::c_int;
}
pub unsafe extern "C" fn tcp_send_ack(mut sk: *mut sock) -> libc::c_int {
    if (*sk).state == TCP_CLOSE as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    let mut tcb: *mut tcb = &mut (*(sk as *mut tcp_sock)).tcb;
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut optlen: libc::c_int = tcp_options_len(sk);
    skb = tcp_alloc_skb(optlen, 0 as libc::c_int);
    th = tcp_hdr(skb);
    (*th).set_ack(1 as libc::c_int as uint8_t);
    (*th)
        .set_hl(
            (::std::mem::size_of::<tcphdr>() as libc::c_ulong)
                .wrapping_div(4 as libc::c_int as libc::c_ulong)
                .wrapping_add((optlen / 4 as libc::c_int) as libc::c_ulong) as uint8_t,
        );
    rc = tcp_transmit_skb(sk, skb, (*tcb).snd_nxt);
    free_skb(skb);
    return rc;
}
unsafe extern "C" fn tcp_send_syn(mut sk: *mut sock) -> libc::c_int {
    if (*sk).state != TCP_SYN_SENT as libc::c_int
        && (*sk).state != TCP_CLOSE as libc::c_int
        && (*sk).state != TCP_LISTEN as libc::c_int
    {
        fprintf(
            stderr,
            b"Socket was not in correct state (closed or listen)\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    let mut opts: tcp_options = {
        let mut init = tcp_options {
            options: 0 as libc::c_int as uint16_t,
            mss: 0,
            sack: 0,
        };
        init
    };
    let mut tcp_options_len_0: libc::c_int = 0 as libc::c_int;
    tcp_options_len_0 = tcp_syn_options(sk, &mut opts);
    skb = tcp_alloc_skb(tcp_options_len_0, 0 as libc::c_int);
    th = tcp_hdr(skb);
    tcp_write_syn_options(th, &mut opts, tcp_options_len_0);
    (*sk).state = TCP_SYN_SENT as libc::c_int;
    (*th).set_syn(1 as libc::c_int as uint8_t);
    return tcp_queue_transmit_skb(sk, skb);
}
pub unsafe extern "C" fn tcp_send_fin(mut sk: *mut sock) -> libc::c_int {
    if (*sk).state == TCP_CLOSE as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    let mut rc: libc::c_int = 0 as libc::c_int;
    skb = tcp_alloc_skb(0 as libc::c_int, 0 as libc::c_int);
    th = tcp_hdr(skb);
    (*th).set_fin(1 as libc::c_int as uint8_t);
    (*th).set_ack(1 as libc::c_int as uint8_t);
    rc = tcp_queue_transmit_skb(sk, skb);
    return rc;
}
pub unsafe extern "C" fn tcp_select_initial_window(mut rcv_wnd: *mut uint32_t) {
    *rcv_wnd = 44477 as libc::c_int as uint32_t;
}
unsafe extern "C" fn tcp_notify_user(mut sk: *mut sock) {
    match (*sk).state {
        7 => {
            wait_wakeup(&mut (*(*sk).sock).sleep);
        }
        _ => {}
    };
}
unsafe extern "C" fn tcp_connect_rto(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut tsk: *mut tcp_sock = arg as *mut tcp_sock;
    let mut tcb: *mut tcb = &mut (*tsk).tcb;
    let mut sk: *mut sock = &mut (*tsk).sk;
    socket_wr_acquire((*sk).sock);
    tcp_release_rto_timer(tsk);
    if (*sk).state == TCP_SYN_SENT as libc::c_int {
        if (*tsk).backoff as libc::c_int > 3 as libc::c_int {
            (*tsk).sk.err = -(110 as libc::c_int);
            (*sk)
                .poll_events = ((*sk).poll_events as libc::c_int
                | (0x4 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int))
                as libc::c_short;
            tcp_done(sk);
        } else {
            let mut skb: *mut sk_buff = write_queue_head(sk);
            if !skb.is_null() {
                skb_reset_header(skb);
                tcp_transmit_skb(sk, skb, (*tcb).snd_una);
                (*tsk).backoff = ((*tsk).backoff).wrapping_add(1);
                (*tsk).backoff;
                tcp_rearm_rto_timer(tsk);
            }
        }
    } else {
        fprintf(
            stderr,
            b"TCP connect RTO triggered even when not in SYNSENT\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    socket_release((*sk).sock);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn tcp_retransmission_timeout(
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    let mut tsk: *mut tcp_sock = arg as *mut tcp_sock;
    let mut tcb: *mut tcb = &mut (*tsk).tcb;
    let mut sk: *mut sock = &mut (*tsk).sk;
    socket_wr_acquire((*sk).sock);
    tcp_release_rto_timer(tsk);
    let mut skb: *mut sk_buff = write_queue_head(sk);
    if skb.is_null() {
        (*tsk).backoff = 0 as libc::c_int as uint8_t;
        tcp_notify_user(sk);
    } else {
        th = tcp_hdr(skb);
        skb_reset_header(skb);
        tcp_transmit_skb(sk, skb, (*tcb).snd_una);
        if (*tsk).rto > 60000 as libc::c_int as libc::c_uint {
            tcp_done(sk);
            (*tsk).sk.err = -(110 as libc::c_int);
            (*sk)
                .poll_events = ((*sk).poll_events as libc::c_int
                | (0x4 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int))
                as libc::c_short;
            socket_release((*sk).sock);
            return 0 as *mut libc::c_void;
        } else {
            (*tsk)
                .rto = ((*tsk).rto as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
            (*tsk).backoff = ((*tsk).backoff).wrapping_add(1);
            (*tsk).backoff;
            (*tsk)
                .retransmit = timer_add(
                (*tsk).rto,
                Some(
                    tcp_retransmission_timeout
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
                tsk as *mut libc::c_void,
            );
            if (*th).fin() != 0 {
                tcp_handle_fin_state(sk);
            }
        }
    }
    socket_release((*sk).sock);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn tcp_rearm_rto_timer(mut tsk: *mut tcp_sock) {
    let mut sk: *mut sock = &mut (*tsk).sk;
    tcp_release_rto_timer(tsk);
    if (*sk).state == TCP_SYN_SENT as libc::c_int {
        (*tsk)
            .retransmit = timer_add(
            ((500 as libc::c_int) << (*tsk).backoff as libc::c_int) as uint32_t,
            Some(
                tcp_connect_rto
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            tsk as *mut libc::c_void,
        );
    } else {
        (*tsk)
            .retransmit = timer_add(
            (*tsk).rto,
            Some(
                tcp_retransmission_timeout
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            tsk as *mut libc::c_void,
        );
    };
}
pub unsafe extern "C" fn tcp_connect(mut sk: *mut sock) -> libc::c_int {
    let mut tsk: *mut tcp_sock = sk as *mut tcp_sock;
    let mut tcb: *mut tcb = &mut (*tsk).tcb;
    let mut rc: libc::c_int = 0 as libc::c_int;
    (*tsk).tcp_header_len = ::std::mem::size_of::<tcphdr>() as libc::c_ulong as uint16_t;
    (*tcb).iss = generate_iss() as uint32_t;
    (*tcb).snd_wnd = 0 as libc::c_int as uint32_t;
    (*tcb).snd_wl1 = 0 as libc::c_int as uint32_t;
    (*tcb).snd_una = (*tcb).iss;
    (*tcb).snd_up = (*tcb).iss;
    (*tcb).snd_nxt = (*tcb).iss;
    (*tcb).rcv_nxt = 0 as libc::c_int as uint32_t;
    tcp_select_initial_window(&mut (*tsk).tcb.rcv_wnd);
    rc = tcp_send_syn(sk);
    (*tcb).snd_nxt = ((*tcb).snd_nxt).wrapping_add(1);
    (*tcb).snd_nxt;
    return rc;
}
pub unsafe extern "C" fn tcp_send(
    mut tsk: *mut tcp_sock,
    mut buf: *const libc::c_void,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    let mut slen: libc::c_int = len;
    let mut mss: libc::c_int = (*tsk).smss as libc::c_int;
    let mut dlen: libc::c_int = 0 as libc::c_int;
    while slen > 0 as libc::c_int {
        dlen = if slen > mss { mss } else { slen };
        slen -= dlen;
        skb = tcp_alloc_skb(0 as libc::c_int, dlen);
        skb_push(skb, dlen as libc::c_uint);
        memcpy((*skb).data as *mut libc::c_void, buf, dlen as libc::c_ulong);
        buf = buf.offset(dlen as isize);
        th = tcp_hdr(skb);
        (*th).set_ack(1 as libc::c_int as uint8_t);
        if slen == 0 as libc::c_int {
            (*th).set_psh(1 as libc::c_int as uint8_t);
        }
        if tcp_queue_transmit_skb(&mut (*tsk).sk, skb) == -(1 as libc::c_int) {
            perror(b"Error on TCP skb queueing\0" as *const u8 as *const libc::c_char);
        }
    }
    tcp_rearm_user_timeout(&mut (*tsk).sk);
    return len;
}
pub unsafe extern "C" fn tcp_send_reset(mut tsk: *mut tcp_sock) -> libc::c_int {
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    let mut tcb: *mut tcb = 0 as *mut tcb;
    let mut rc: libc::c_int = 0 as libc::c_int;
    skb = tcp_alloc_skb(0 as libc::c_int, 0 as libc::c_int);
    th = tcp_hdr(skb);
    tcb = &mut (*tsk).tcb;
    (*th).set_rst(1 as libc::c_int as uint8_t);
    (*tcb).snd_una = (*tcb).snd_nxt;
    rc = tcp_transmit_skb(&mut (*tsk).sk, skb, (*tcb).snd_nxt);
    free_skb(skb);
    return rc;
}
pub unsafe extern "C" fn tcp_send_challenge_ack(
    mut sk: *mut sock,
    mut skb: *mut sk_buff,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tcp_queue_fin(mut sk: *mut sock) -> libc::c_int {
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    let mut rc: libc::c_int = 0 as libc::c_int;
    skb = tcp_alloc_skb(0 as libc::c_int, 0 as libc::c_int);
    th = tcp_hdr(skb);
    (*th).set_fin(1 as libc::c_int as uint8_t);
    (*th).set_ack(1 as libc::c_int as uint8_t);
    rc = tcp_queue_transmit_skb(sk, skb);
    return rc;
}
