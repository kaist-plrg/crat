use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn min(x: uint32_t, y: uint32_t) -> uint32_t;
    fn socket_wr_acquire(sock: *mut socket) -> libc::c_int;
    fn socket_release(sock: *mut socket) -> libc::c_int;
    fn sock_connected(sk: *mut sock);
    fn timer_add(
        expire: uint32_t,
        handler: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        arg: *mut libc::c_void,
    ) -> *mut timer;
    fn __tcp_set_state(sk: *mut sock, state: uint32_t);
    fn tcp_rearm_user_timeout(sk: *mut sock);
    fn tcp_send_delack(arg: *mut libc::c_void) -> *mut libc::c_void;
    fn tcp_send_ack(sk: *mut sock) -> libc::c_int;
    fn tcp_rearm_rto_timer(tsk: *mut tcp_sock);
    fn tcp_send_next(sk: *mut sock, amount: libc::c_int) -> libc::c_int;
    fn tcp_stop_delack_timer(tsk: *mut tcp_sock);
    fn tcp_enter_time_wait(sk: *mut sock);
    fn tcp_send_reset(tsk: *mut tcp_sock) -> libc::c_int;
    fn tcp_done(sk: *mut sock) -> libc::c_int;
    fn tcp_stop_rto_timer(tsk: *mut tcp_sock);
    fn tcp_rtt(tsk: *mut tcp_sock);
    fn tcp_send_challenge_ack(sk: *mut sock, skb: *mut sk_buff) -> libc::c_int;
    fn tcp_send_synack(sk: *mut sock) -> libc::c_int;
    fn free_skb(skb: *mut sk_buff);
    fn tcp_data_dequeue(
        tsk: *mut tcp_sock,
        user_buf: *mut libc::c_void,
        len: libc::c_int,
    ) -> libc::c_int;
    fn tcp_data_queue(
        tsk: *mut tcp_sock,
        th: *mut tcphdr,
        skb: *mut sk_buff,
    ) -> libc::c_int;
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
unsafe extern "C" fn wait_sleep(mut w: *mut wait_lock) -> libc::c_int {
    (*w).sleeping = 1 as libc::c_int as uint8_t;
    pthread_cond_wait(&mut (*w).ready, &mut (*w).lock);
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn list_del(mut elem: *mut list_head) {
    let mut prev: *mut list_head = (*elem).prev;
    let mut next: *mut list_head = (*elem).next;
    (*prev).next = next;
    (*next).prev = prev;
}
#[inline]
unsafe extern "C" fn skb_queue_empty(mut list: *const sk_buff_head) -> libc::c_int {
    return (skb_queue_len(list) < 1 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn skb_queue_len(mut list: *const sk_buff_head) -> uint32_t {
    return (*list).qlen;
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
unsafe extern "C" fn skb_peek(mut list: *mut sk_buff_head) -> *mut sk_buff {
    if skb_queue_empty(list) != 0 {
        return 0 as *mut sk_buff;
    }
    return ((*list).head.next as *mut libc::c_char)
        .offset(-(0 as libc::c_ulong as isize)) as *mut sk_buff;
}
unsafe extern "C" fn tcp_parse_opts(
    mut tsk: *mut tcp_sock,
    mut th: *mut tcphdr,
) -> libc::c_int {
    let mut ptr: *mut uint8_t = ((*th).data).as_mut_ptr();
    let mut optlen: uint8_t = ((((*th).hl() as libc::c_int) << 2 as libc::c_int)
        - 20 as libc::c_int) as uint8_t;
    let mut opt_mss: *mut tcp_opt_mss = 0 as *mut tcp_opt_mss;
    let mut sack_seen: uint8_t = 0 as libc::c_int as uint8_t;
    let mut tsopt_seen: uint8_t = 0 as libc::c_int as uint8_t;
    while optlen as libc::c_int > 0 as libc::c_int
        && (optlen as libc::c_int) < 20 as libc::c_int
    {
        let mut mss: uint16_t = 0;
        match *ptr as libc::c_int {
            2 => {
                opt_mss = ptr as *mut tcp_opt_mss;
                mss = ntohs((*opt_mss).mss);
                if mss as libc::c_int > 536 as libc::c_int
                    && mss as libc::c_int <= 1460 as libc::c_int
                {
                    (*tsk).smss = mss;
                }
                ptr = ptr
                    .offset(
                        ::std::mem::size_of::<tcp_opt_mss>() as libc::c_ulong as isize,
                    );
                optlen = (optlen as libc::c_int - 4 as libc::c_int) as uint8_t;
            }
            1 => {
                ptr = ptr.offset(1 as libc::c_int as isize);
                optlen = optlen.wrapping_sub(1);
                optlen;
            }
            4 => {
                sack_seen = 1 as libc::c_int as uint8_t;
                optlen = optlen.wrapping_sub(1);
                optlen;
            }
            8 => {
                tsopt_seen = 1 as libc::c_int as uint8_t;
                optlen = optlen.wrapping_sub(1);
                optlen;
            }
            _ => {
                fprintf(
                    stderr,
                    b"Unrecognized TCPOPT\n\0" as *const u8 as *const libc::c_char,
                );
                optlen = optlen.wrapping_sub(1);
                optlen;
            }
        }
    }
    if tsopt_seen == 0 {
        (*tsk).tsopt = 0 as libc::c_int as uint8_t;
    }
    if sack_seen as libc::c_int != 0 && (*tsk).sackok as libc::c_int != 0 {
        if (*tsk).tsopt != 0 {
            (*tsk).sacks_allowed = 3 as libc::c_int as uint8_t;
        } else {
            (*tsk).sacks_allowed = 4 as libc::c_int as uint8_t;
        }
    } else {
        (*tsk).sackok = 0 as libc::c_int as uint8_t;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn tcp_clean_rto_queue(
    mut sk: *mut sock,
    mut una: uint32_t,
) -> libc::c_int {
    let mut tsk: *mut tcp_sock = sk as *mut tcp_sock;
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut rc: libc::c_int = 0 as libc::c_int;
    loop {
        skb = skb_peek(&mut (*sk).write_queue);
        if skb.is_null() {
            break;
        }
        if !((*skb).seq > 0 as libc::c_int as libc::c_uint && (*skb).end_seq <= una) {
            break;
        }
        skb_dequeue(&mut (*sk).write_queue);
        (*skb).refcnt -= 1;
        (*skb).refcnt;
        free_skb(skb);
        if (*tsk).inflight > 0 as libc::c_int as libc::c_uint {
            (*tsk).inflight = ((*tsk).inflight).wrapping_sub(1);
            (*tsk).inflight;
        }
    }
    if skb.is_null() || (*tsk).inflight == 0 as libc::c_int as libc::c_uint {
        tcp_stop_rto_timer(tsk);
    }
    return rc;
}
#[inline]
unsafe extern "C" fn __tcp_drop(
    mut sk: *mut sock,
    mut skb: *mut sk_buff,
) -> libc::c_int {
    free_skb(skb);
    return 0 as libc::c_int;
}
unsafe extern "C" fn tcp_verify_segment(
    mut tsk: *mut tcp_sock,
    mut th: *mut tcphdr,
    mut skb: *mut sk_buff,
) -> libc::c_int {
    let mut tcb: *mut tcb = &mut (*tsk).tcb;
    if (*skb).dlen > 0 as libc::c_int as libc::c_uint
        && (*tcb).rcv_wnd == 0 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if (*th).seq < (*tcb).rcv_nxt
        || (*th).seq > ((*tcb).rcv_nxt).wrapping_add((*tcb).rcv_wnd)
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn tcp_reset(mut sk: *mut sock) {
    (*sk)
        .poll_events = (0x4 as libc::c_int | 0x100 as libc::c_int | 0x8 as libc::c_int
        | 0x10 as libc::c_int) as libc::c_short;
    match (*sk).state {
        1 => {
            (*sk).err = -(111 as libc::c_int);
        }
        7 => {
            (*sk).err = -(32 as libc::c_int);
        }
        6 => return,
        _ => {
            (*sk).err = -(104 as libc::c_int);
        }
    }
    tcp_done(sk);
}
#[inline]
unsafe extern "C" fn tcp_discard(
    mut tsk: *mut tcp_sock,
    mut skb: *mut sk_buff,
    mut th: *mut tcphdr,
) -> libc::c_int {
    free_skb(skb);
    return 0 as libc::c_int;
}
unsafe extern "C" fn tcp_listen(
    mut tsk: *mut tcp_sock,
    mut skb: *mut sk_buff,
    mut th: *mut tcphdr,
) -> libc::c_int {
    free_skb(skb);
    return 0 as libc::c_int;
}
unsafe extern "C" fn tcp_synsent(
    mut tsk: *mut tcp_sock,
    mut skb: *mut sk_buff,
    mut th: *mut tcphdr,
) -> libc::c_int {
    let mut current_block: u64;
    let mut tcb: *mut tcb = &mut (*tsk).tcb;
    let mut sk: *mut sock = &mut (*tsk).sk;
    if (*th).ack() != 0 {
        if (*th).ack_seq <= (*tcb).iss || (*th).ack_seq > (*tcb).snd_nxt {
            if (*th).rst() != 0 {
                current_block = 5688227503701976846;
            } else {
                current_block = 11514472009261332083;
            }
        } else if (*th).ack_seq < (*tcb).snd_una || (*th).ack_seq > (*tcb).snd_nxt {
            current_block = 11514472009261332083;
        } else {
            current_block = 7815301370352969686;
        }
        match current_block {
            5688227503701976846 => {}
            7815301370352969686 => {}
            _ => {
                __tcp_drop(sk, skb);
                return 0 as libc::c_int;
            }
        }
    } else {
        current_block = 7815301370352969686;
    }
    match current_block {
        7815301370352969686 => {
            if (*th).rst() != 0 {
                tcp_reset(&mut (*tsk).sk);
            } else if !((*th).syn() == 0) {
                (*tcb)
                    .rcv_nxt = ((*th).seq)
                    .wrapping_add(1 as libc::c_int as libc::c_uint);
                (*tcb).irs = (*th).seq;
                if (*th).ack() != 0 {
                    (*tcb).snd_una = (*th).ack_seq;
                    tcp_clean_rto_queue(sk, (*tcb).snd_una);
                }
                if (*tcb).snd_una > (*tcb).iss {
                    __tcp_set_state(sk, TCP_ESTABLISHED as libc::c_int as uint32_t);
                    (*tcb).snd_una = (*tcb).snd_nxt;
                    (*tsk).backoff = 0 as libc::c_int as uint8_t;
                    (*tsk).rto = 1000 as libc::c_int as uint32_t;
                    tcp_send_ack(&mut (*tsk).sk);
                    tcp_rearm_user_timeout(&mut (*tsk).sk);
                    tcp_parse_opts(tsk, th);
                    sock_connected(sk);
                } else {
                    __tcp_set_state(sk, TCP_SYN_RECEIVED as libc::c_int as uint32_t);
                    (*tcb).snd_una = (*tcb).iss;
                    tcp_send_synack(&mut (*tsk).sk);
                }
            }
        }
        _ => {}
    }
    __tcp_drop(sk, skb);
    return 0 as libc::c_int;
}
unsafe extern "C" fn tcp_closed(
    mut tsk: *mut tcp_sock,
    mut skb: *mut sk_buff,
    mut th: *mut tcphdr,
) -> libc::c_int {
    let mut rc: libc::c_int = -(1 as libc::c_int);
    if (*th).rst() != 0 {
        tcp_discard(tsk, skb, th);
        rc = 0 as libc::c_int;
    } else {
        (*th).ack() != 0;
        rc = tcp_send_reset(tsk);
        free_skb(skb);
    }
    return rc;
}
pub unsafe extern "C" fn tcp_input_state(
    mut sk: *mut sock,
    mut th: *mut tcphdr,
    mut skb: *mut sk_buff,
) -> libc::c_int {
    let mut current_block: u64;
    let mut tsk: *mut tcp_sock = sk as *mut tcp_sock;
    let mut tcb: *mut tcb = &mut (*tsk).tcb;
    match (*sk).state {
        6 => return tcp_closed(tsk, skb, th),
        0 => return tcp_listen(tsk, skb, th),
        1 => return tcp_synsent(tsk, skb, th),
        _ => {}
    }
    if tcp_verify_segment(tsk, th, skb) == 0 {
        if (*th).rst() == 0 {
            tcp_send_ack(sk);
        }
        return __tcp_drop(sk, skb);
    }
    if (*th).rst() != 0 {
        free_skb(skb);
        tcp_enter_time_wait(sk);
        ((*(*tsk).sk.ops).recv_notify).unwrap()(&mut (*tsk).sk);
        return 0 as libc::c_int;
    }
    if (*th).syn() != 0 {
        tcp_send_challenge_ack(sk, skb);
        return __tcp_drop(sk, skb);
    }
    if (*th).ack() == 0 {
        return __tcp_drop(sk, skb);
    }
    let mut current_block_41: u64;
    match (*sk).state {
        2 => {
            if (*tcb).snd_una <= (*th).ack_seq && (*th).ack_seq < (*tcb).snd_nxt {
                __tcp_set_state(sk, TCP_ESTABLISHED as libc::c_int as uint32_t);
            } else {
                return __tcp_drop(sk, skb)
            }
            current_block_41 = 8584116283024297870;
        }
        3 | 4 | 5 | 7 | 8 | 9 => {
            current_block_41 = 8584116283024297870;
        }
        _ => {
            current_block_41 = 11385396242402735691;
        }
    }
    match current_block_41 {
        8584116283024297870 => {
            if (*tcb).snd_una < (*th).ack_seq && (*th).ack_seq <= (*tcb).snd_nxt {
                (*tcb).snd_una = (*th).ack_seq;
                tcp_rtt(tsk);
                tcp_clean_rto_queue(sk, (*tcb).snd_una);
            }
            if (*th).ack_seq < (*tcb).snd_una {
                return __tcp_drop(sk, skb);
            }
            if (*th).ack_seq > (*tcb).snd_nxt {
                return __tcp_drop(sk, skb);
            }
            (*tcb).snd_una < (*th).ack_seq && (*th).ack_seq <= (*tcb).snd_nxt;
        }
        _ => {}
    }
    if skb_queue_empty(&mut (*sk).write_queue) != 0 {
        match (*sk).state {
            4 => {
                __tcp_set_state(sk, TCP_FIN_WAIT_2 as libc::c_int as uint32_t);
            }
            8 => {
                __tcp_set_state(sk, TCP_TIME_WAIT as libc::c_int as uint32_t);
            }
            9 => {
                free_skb(skb);
                return tcp_done(sk);
            }
            10 => {
                if (*tcb).rcv_nxt == (*th).seq {
                    (*tsk)
                        .flags = ((*tsk).flags as libc::c_int | 0x1 as libc::c_int)
                        as uint8_t;
                    tcp_send_ack(sk);
                }
            }
            5 | _ => {}
        }
    }
    (*th).urg() != 0;
    let mut expected: libc::c_int = ((*skb).seq == (*tcb).rcv_nxt) as libc::c_int;
    match (*sk).state {
        3 | 4 | 5 => {
            if (*th).psh() as libc::c_int != 0
                || (*skb).dlen > 0 as libc::c_int as libc::c_uint
            {
                tcp_data_queue(tsk, th, skb);
            }
        }
        7 | 8 | 9 | 10 | _ => {}
    }
    if (*th).fin() as libc::c_int != 0 && expected != 0 {
        match (*sk).state {
            6 | 0 | 1 => {
                __tcp_drop(sk, skb);
                current_block = 5899868364378153265;
            }
            _ => {
                (*tcb)
                    .rcv_nxt = ((*tcb).rcv_nxt as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as uint32_t
                    as uint32_t;
                (*tsk)
                    .flags = ((*tsk).flags as libc::c_int | 0x1 as libc::c_int)
                    as uint8_t;
                (*sk)
                    .poll_events = ((*sk).poll_events as libc::c_int
                    | (0x1 as libc::c_int | 0x2 as libc::c_int | 0x40 as libc::c_int
                        | 0x80 as libc::c_int)) as libc::c_short;
                tcp_send_ack(sk);
                ((*(*tsk).sk.ops).recv_notify).unwrap()(&mut (*tsk).sk);
                match (*sk).state {
                    2 | 3 => {
                        __tcp_set_state(sk, TCP_CLOSE_WAIT as libc::c_int as uint32_t);
                    }
                    4 => {
                        if skb_queue_empty(&mut (*sk).write_queue) != 0 {
                            tcp_enter_time_wait(sk);
                        } else {
                            __tcp_set_state(sk, TCP_CLOSING as libc::c_int as uint32_t);
                        }
                    }
                    5 => {
                        tcp_enter_time_wait(sk);
                    }
                    10 => {}
                    7 | 8 | 9 | _ => {}
                }
                current_block = 2723324002591448311;
            }
        }
    } else {
        current_block = 2723324002591448311;
    }
    match current_block {
        2723324002591448311 => {
            match (*sk).state {
                3 | 4 | 5 => {
                    if expected != 0 {
                        tcp_stop_delack_timer(tsk);
                        let mut pending: libc::c_int = min(
                            skb_queue_len(&mut (*sk).write_queue),
                            3 as libc::c_int as uint32_t,
                        ) as libc::c_int;
                        if (*tsk).inflight == 0 as libc::c_int as libc::c_uint
                            && pending > 0 as libc::c_int
                        {
                            tcp_send_next(sk, pending);
                            (*tsk)
                                .inflight = ((*tsk).inflight as libc::c_uint)
                                .wrapping_add(pending as libc::c_uint) as uint32_t
                                as uint32_t;
                            tcp_rearm_rto_timer(tsk);
                        } else if (*th).psh() as libc::c_int != 0
                            || (*skb).dlen > 1000 as libc::c_int as libc::c_uint
                                && {
                                    (*tsk).delacks = ((*tsk).delacks).wrapping_add(1);
                                    (*tsk).delacks as libc::c_int > 1 as libc::c_int
                                }
                        {
                            (*tsk).delacks = 0 as libc::c_int as uint8_t;
                            tcp_send_ack(sk);
                        } else if (*skb).dlen > 0 as libc::c_int as libc::c_uint {
                            (*tsk)
                                .delack = timer_add(
                                200 as libc::c_int as uint32_t,
                                Some(
                                    tcp_send_delack
                                        as unsafe extern "C" fn(
                                            *mut libc::c_void,
                                        ) -> *mut libc::c_void,
                                ),
                                &mut (*tsk).sk as *mut sock as *mut libc::c_void,
                            );
                        }
                    }
                }
                _ => {}
            }
            free_skb(skb);
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tcp_receive(
    mut tsk: *mut tcp_sock,
    mut buf: *mut libc::c_void,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut rlen: libc::c_int = 0 as libc::c_int;
    let mut curlen: libc::c_int = 0 as libc::c_int;
    let mut sk: *mut sock = &mut (*tsk).sk;
    let mut sock: *mut socket = (*sk).sock;
    memset(buf, 0 as libc::c_int, len as libc::c_ulong);
    while rlen < len {
        curlen = tcp_data_dequeue(tsk, buf.offset(rlen as isize), len - rlen);
        rlen += curlen;
        if (*tsk).flags as libc::c_int & 0x8 as libc::c_int != 0 {
            (*tsk)
                .flags = ((*tsk).flags as libc::c_int & !(0x8 as libc::c_int))
                as uint8_t;
            break;
        } else {
            if (*tsk).flags as libc::c_int & 0x1 as libc::c_int != 0 || rlen == len {
                break;
            }
            if (*sock).flags & 0o4000 as libc::c_int != 0 {
                if rlen == 0 as libc::c_int {
                    rlen = -(11 as libc::c_int);
                }
                break;
            } else {
                pthread_mutex_lock(&mut (*tsk).sk.recv_wait.lock);
                socket_release(sock);
                wait_sleep(&mut (*tsk).sk.recv_wait);
                pthread_mutex_unlock(&mut (*tsk).sk.recv_wait.lock);
                socket_wr_acquire(sock);
            }
        }
    }
    if rlen >= 0 as libc::c_int {
        tcp_rearm_user_timeout(sk);
    }
    return rlen;
}
