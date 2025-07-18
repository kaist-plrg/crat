use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn free_skb(skb: *mut sk_buff);
    fn tcp_send_ack(sk: *mut sock) -> libc::c_int;
    fn tcp_calculate_sacks(tsk: *mut tcp_sock) -> libc::c_int;
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
unsafe extern "C" fn skb_queue_len(mut list: *const sk_buff_head) -> uint32_t {
    return (*list).qlen;
}
#[inline]
unsafe extern "C" fn skb_queue_add(
    mut list: *mut sk_buff_head,
    mut new: *mut sk_buff,
    mut next: *mut sk_buff,
) {
    list_add_tail(&mut (*new).list, &mut (*next).list);
    (*list)
        .qlen = ((*list).qlen as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
}
#[inline]
unsafe extern "C" fn skb_queue_tail(mut list: *mut sk_buff_head, mut new: *mut sk_buff) {
    list_add_tail(&mut (*new).list, &mut (*list).head);
    (*list)
        .qlen = ((*list).qlen as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
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
unsafe extern "C" fn tcp_hdr(mut skb: *const sk_buff) -> *mut tcphdr {
    return ((*skb).head)
        .offset(::std::mem::size_of::<eth_hdr>() as libc::c_ulong as isize)
        .offset(::std::mem::size_of::<iphdr>() as libc::c_ulong as isize) as *mut tcphdr;
}
unsafe extern "C" fn tcp_data_insert_ordered(
    mut queue: *mut sk_buff_head,
    mut skb: *mut sk_buff,
) {
    let mut next: *mut sk_buff = 0 as *mut sk_buff;
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut tmp: *mut list_head = 0 as *mut list_head;
    item = (*queue).head.next;
    tmp = (*item).next;
    while item != &mut (*queue).head as *mut list_head {
        next = (item as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut sk_buff;
        if (*skb).seq < (*next).seq {
            if (*skb).end_seq > (*next).seq {
                fprintf(
                    stderr,
                    b"Could not join skbs\n\0" as *const u8 as *const libc::c_char,
                );
            } else {
                (*skb).refcnt += 1;
                (*skb).refcnt;
                skb_queue_add(queue, skb, next);
                return;
            }
        } else if (*skb).seq == (*next).seq {
            return
        }
        item = tmp;
        tmp = (*item).next;
    }
    (*skb).refcnt += 1;
    (*skb).refcnt;
    skb_queue_tail(queue, skb);
}
unsafe extern "C" fn tcp_consume_ofo_queue(mut tsk: *mut tcp_sock) {
    let mut sk: *mut sock = &mut (*tsk).sk;
    let mut tcb: *mut tcb = &mut (*tsk).tcb;
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    loop {
        skb = skb_peek(&mut (*tsk).ofo_queue);
        if !(!skb.is_null() && (*tcb).rcv_nxt == (*skb).seq) {
            break;
        }
        (*tcb)
            .rcv_nxt = ((*tcb).rcv_nxt as libc::c_uint).wrapping_add((*skb).dlen)
            as uint32_t as uint32_t;
        skb_dequeue(&mut (*tsk).ofo_queue);
        skb_queue_tail(&mut (*sk).receive_queue, skb);
    };
}
pub unsafe extern "C" fn tcp_data_dequeue(
    mut tsk: *mut tcp_sock,
    mut user_buf: *mut libc::c_void,
    mut userlen: libc::c_int,
) -> libc::c_int {
    let mut sk: *mut sock = &mut (*tsk).sk;
    let mut th: *mut tcphdr = 0 as *mut tcphdr;
    let mut rlen: libc::c_int = 0 as libc::c_int;
    while skb_queue_empty(&mut (*sk).receive_queue) == 0 && rlen < userlen {
        let mut skb: *mut sk_buff = skb_peek(&mut (*sk).receive_queue);
        if skb.is_null() {
            break;
        }
        th = tcp_hdr(skb);
        let mut dlen: libc::c_int = (if (rlen as libc::c_uint).wrapping_add((*skb).dlen)
            > userlen as libc::c_uint
        {
            (userlen - rlen) as libc::c_uint
        } else {
            (*skb).dlen
        }) as libc::c_int;
        memcpy(user_buf, (*skb).payload as *const libc::c_void, dlen as libc::c_ulong);
        (*skb)
            .dlen = ((*skb).dlen as libc::c_uint).wrapping_sub(dlen as libc::c_uint)
            as uint32_t as uint32_t;
        (*skb).payload = ((*skb).payload).offset(dlen as isize);
        rlen += dlen;
        user_buf = user_buf.offset(dlen as isize);
        if (*skb).dlen == 0 as libc::c_int as libc::c_uint {
            if (*th).psh() != 0 {
                (*tsk)
                    .flags = ((*tsk).flags as libc::c_int | 0x8 as libc::c_int)
                    as uint8_t;
            }
            skb_dequeue(&mut (*sk).receive_queue);
            (*skb).refcnt -= 1;
            (*skb).refcnt;
            free_skb(skb);
        }
    }
    if skb_queue_empty(&mut (*sk).receive_queue) != 0
        && (*tsk).flags as libc::c_int & 0x1 as libc::c_int == 0
    {
        (*sk)
            .poll_events = ((*sk).poll_events as libc::c_int & !(0x1 as libc::c_int))
            as libc::c_short;
    }
    return rlen;
}
pub unsafe extern "C" fn tcp_data_queue(
    mut tsk: *mut tcp_sock,
    mut th: *mut tcphdr,
    mut skb: *mut sk_buff,
) -> libc::c_int {
    let mut sk: *mut sock = &mut (*tsk).sk;
    let mut tcb: *mut tcb = &mut (*tsk).tcb;
    let mut rc: libc::c_int = 0 as libc::c_int;
    if (*tcb).rcv_wnd == 0 {
        free_skb(skb);
        return -(1 as libc::c_int);
    }
    let mut expected: libc::c_int = ((*skb).seq == (*tcb).rcv_nxt) as libc::c_int;
    if expected != 0 {
        (*tcb)
            .rcv_nxt = ((*tcb).rcv_nxt as libc::c_uint).wrapping_add((*skb).dlen)
            as uint32_t as uint32_t;
        (*skb).refcnt += 1;
        (*skb).refcnt;
        skb_queue_tail(&mut (*sk).receive_queue, skb);
        tcp_consume_ofo_queue(tsk);
        (*sk)
            .poll_events = ((*sk).poll_events as libc::c_int
            | (0x1 as libc::c_int | 0x2 as libc::c_int | 0x40 as libc::c_int
                | 0x80 as libc::c_int)) as libc::c_short;
        ((*(*tsk).sk.ops).recv_notify).unwrap()(&mut (*tsk).sk);
    } else {
        tcp_data_insert_ordered(&mut (*tsk).ofo_queue, skb);
        if (*tsk).sackok != 0 {
            tcp_calculate_sacks(tsk);
        }
        tcp_send_ack(sk);
    }
    return rc;
}
