use ::libc;
extern "C" {
    pub type sockaddr_x25;
    pub type sockaddr_un;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    fn global_cron(_: *mut server) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
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
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_setaffinity_np(
        __th: pthread_t,
        __cpusetsize: size_t,
        __cpuset: *const cpu_set_t,
    ) -> libc::c_int;
    fn pthread_spin_lock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    fn pthread_spin_unlock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn dns_error(_: libc::c_int, _: *mut libc::c_char);
    fn recvfrom(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    fn accept(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn mbuf_alloc() -> *mut mbuf_type;
    fn mbuf_free(mbuf: *mut mbuf_type) -> libc::c_int;
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
    fn udp_read_msg(mbuf: *mut mbuf_type, _: libc::c_int) -> libc::c_int;
    fn tcp_read_dns_msg(mbuf: *mut mbuf_type, _: uint, _: libc::c_int) -> libc::c_int;
    fn set_non_block(fd: libc::c_int) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type ulong = libc::c_ulong;
pub type ushort = libc::c_ushort;
pub type uint = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type pthread_t = libc::c_ulong;
pub type pthread_spinlock_t = libc::c_int;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type socklen_t = __socklen_t;
pub type __cpu_mask = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16],
}
pub type uchar = libc::c_uchar;
pub type hashval_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _packet_type {
    pub label_count: uint8_t,
    pub domain: [uchar; 256],
    pub label: [*mut uint8_t; 64],
    pub label_offsets: [uint8_t; 64],
    pub label_len: [uint8_t; 64],
    pub hash: [hashval_t; 64],
}
pub type packet_type = _packet_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_node {
    pub data: *mut libc::c_void,
    pub next: *mut list_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list {
    pub lock: pthread_spinlock_t,
    pub head: *mut list_node,
}
pub type rrtype = libc::c_uint;
pub const ANY: rrtype = 255;
pub const MAILA: rrtype = 254;
pub const MAILB: rrtype = 253;
pub const AXFR: rrtype = 252;
pub const TKEY: rrtype = 249;
pub const DHCID: rrtype = 49;
pub const DNSKEY: rrtype = 48;
pub const NSEC: rrtype = 47;
pub const RRSIG: rrtype = 46;
pub const DS: rrtype = 43;
pub const APL: rrtype = 42;
pub const OPT: rrtype = 41;
pub const DNAME: rrtype = 39;
pub const A6: rrtype = 38;
pub const CERT: rrtype = 37;
pub const SRV: rrtype = 33;
pub const NXT: rrtype = 30;
pub const AAAA: rrtype = 28;
pub const KEY: rrtype = 25;
pub const SIG: rrtype = 24;
pub const AFSDB: rrtype = 18;
pub const RP: rrtype = 17;
pub const TXT: rrtype = 16;
pub const MX: rrtype = 15;
pub const MINFO: rrtype = 14;
pub const HINFO: rrtype = 13;
pub const PTR: rrtype = 12;
pub const WKS: rrtype = 11;
pub const NUL: rrtype = 10;
pub const MR: rrtype = 9;
pub const MG: rrtype = 8;
pub const MB: rrtype = 7;
pub const SOA: rrtype = 6;
pub const CNAME: rrtype = 5;
pub const MF: rrtype = 4;
pub const MD: rrtype = 3;
pub const NS: rrtype = 2;
pub const A: rrtype = 1;
pub const BEGIN_TYPE: rrtype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _type_value {
    pub A: *mut uchar,
    pub NS: *mut uchar,
    pub CNAME: *mut uchar,
    pub SOA: *mut uchar,
    pub MX: *mut uchar,
    pub TXT: *mut uchar,
    pub AAAA: *mut uchar,
    pub SRV: *mut uchar,
    pub PTR: *mut uchar,
}
pub type type_value = _type_value;
pub type comprbt = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rbnode {
    pub parent: *mut rbnode,
    pub left: *mut rbnode,
    pub right: *mut rbnode,
    pub color: libc::c_int,
    pub key: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rbtree {
    pub root: *mut rbnode,
    pub nil: rbnode,
    pub lock: pthread_spinlock_t,
    pub size: uint,
    pub c: Option::<comprbt>,
    pub argv: *mut libc::c_void,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbuf_ring {
    pub prod: prod,
    pub cons: cons,
    pub ring: [*mut libc::c_void; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cons {
    pub sc_dequeue: uint32_t,
    pub size: uint32_t,
    pub mask: uint32_t,
    pub head: uint32_t,
    pub tail: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prod {
    pub watermark: uint32_t,
    pub sp_enqueue: uint32_t,
    pub size: uint32_t,
    pub mask: uint32_t,
    pub head: uint32_t,
    pub tail: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mem_buf {
    pub mbuf: *mut mbuf_ring,
    pub fetch_len: uint,
    pub socktype: uint,
    pub fd: libc::c_int,
    pub addr: *mut sockaddr_in,
    pub caddr: sockaddr_in,
    pub aaddr: sockaddr_in,
    pub data: [uchar; 4096],
    pub qtype: rrtype,
    pub err: libc::c_int,
    pub dlen: libc::c_int,
    pub id: ushort,
    pub lowerdomain: packet_type,
    pub origindomain: *mut uchar,
    pub buflen: libc::c_int,
    pub buf: *mut uchar,
    pub td: *mut uchar,
    pub cid: ushort,
    pub qlen: ushort,
    pub lables: ushort,
    pub qing: *mut uchar,
    pub qhash: *mut hashval_t,
    pub backid: ushort,
    pub aid: ushort,
    pub mask: ushort,
    pub qname: ushort,
    pub sq: ushort,
    pub qtimes: ushort,
    pub auth_socktype: ushort,
    pub stat: ushort,
    pub qbuffer: [uchar; 256],
    pub qbuffer_hash: hashval_t,
    pub tdbuffer: *mut uchar,
    pub tempbuffer: *mut uchar,
    pub dmbuffer: *mut uchar,
    pub ipbuffer: *mut uchar,
    pub hascname: ushort,
    pub tcpfd: libc::c_int,
    pub tcpnums: libc::c_int,
    pub mxtry: libc::c_int,
    pub qns: libc::c_int,
    pub stime: uint64_t,
}
pub type mbuf_type = _mem_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msgcache {
    pub head: uint64_t,
    pub tail: uint64_t,
    pub size: uint32_t,
    pub pkt: uint32_t,
    pub lock: pthread_spinlock_t,
    pub data: [uchar; 0],
}
pub type hashfunc = unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> hashval_t;
pub type comparefunc = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hentry {
    pub c2rust_unnamed: C2RustUnnamed_0,
    pub next: *mut hentry,
    pub count: libc::c_int,
    pub key: [uchar; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub vals: [*mut uchar; 9],
    pub val: type_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hdata {
    pub list: *mut hentry,
    pub now: uint64_t,
    pub lock: pthread_spinlock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htable {
    pub lock: pthread_spinlock_t,
    pub table: *mut hdata,
    pub edata: *mut uchar,
    pub h: Option::<hashfunc>,
    pub size: uint,
    pub mask: uint,
    pub now: uint,
    pub c: Option::<comparefunc>,
}
pub type EPOLL_EVENTS = libc::c_uint;
pub const EPOLLET: EPOLL_EVENTS = 2147483648;
pub const EPOLLONESHOT: EPOLL_EVENTS = 1073741824;
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
pub const EPOLLIN: EPOLL_EVENTS = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union epoll_data {
    pub ptr: *mut libc::c_void,
    pub fd: libc::c_int,
    pub u32_0: uint32_t,
    pub u64_0: uint64_t,
}
pub type epoll_data_t = epoll_data;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct epoll_event {
    pub events: uint32_t,
    pub data: epoll_data_t,
}
pub type SA = sockaddr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockinfo {
    pub addr: sockaddr_in,
    pub fd: libc::c_int,
    pub buflen: libc::c_int,
    pub socktype: libc::c_int,
    pub buf: *mut uchar,
    pub lowerdomain: *mut packet_type,
    pub mbuf: *mut mbuf_type,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct tag_dnsheader {
    pub id: uint16_t,
    pub flags: uint16_t,
    pub qdcount: uint16_t,
    pub ancount: uint16_t,
    pub nscount: uint16_t,
    pub arcount: uint16_t,
}
pub type dnsheader = tag_dnsheader;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qoutinfo {
    pub td: *mut uchar,
    pub type_0: uchar,
    pub lowerdomain: *mut packet_type,
    pub cli: *mut sockinfo,
    pub cid: ushort,
    pub dlen: ushort,
    pub qlen: ushort,
    pub lables: ushort,
    pub qing: *mut uchar,
    pub qhash: *mut hashval_t,
    pub backid: ushort,
    pub aid: ushort,
    pub mask: ushort,
    pub qname: ushort,
    pub sq: ushort,
    pub qtimes: ushort,
    pub socktype: ushort,
    pub stat: ushort,
    pub qbuffer: [uchar; 256],
    pub qbuffer_hash: hashval_t,
    pub tdbuffer: *mut uchar,
    pub tempbuffer: *mut uchar,
    pub dmbuffer: *mut uchar,
    pub ipbuffer: *mut uchar,
    pub hascname: ushort,
    pub tcpfd: libc::c_int,
    pub tcpnums: libc::c_int,
    pub mxtry: libc::c_int,
    pub qns: libc::c_int,
    pub stime: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct log_info {
    pub logfd: libc::c_int,
    pub lastlog: time_t,
    pub log_type: libc::c_int,
    pub log_cache: [uchar; 1048576],
    pub log_cache_cursor: libc::c_int,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const SERVER_PORT: C2RustUnnamed_1 = 53;
pub const FETCHER_NUM: C2RustUnnamed_1 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eptcpfds {
    pub ret: libc::c_int,
    pub domain: [uchar; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct author {
    pub audp: libc::c_int,
    pub cudp: libc::c_int,
    pub idx: libc::c_int,
    pub s: *mut server,
    pub lock: pthread_spinlock_t,
    pub list: [*mut qoutinfo; 10000],
    pub qnum: libc::c_int,
    pub response: libc::c_int,
    pub qidx: libc::c_int,
    pub timex: libc::c_int,
    pub el: *mut list,
    pub bdepfd: libc::c_int,
    pub loginfo: *mut log_info,
    pub dblock: [pthread_spinlock_t; 101],
    pub databuffer: [uchar; 65528],
    pub randombuffer: [uchar; 3000],
    pub tmpbuffer: [uchar; 2000],
    pub tdbuffer: [uchar; 256],
    pub tempbuffer: [uchar; 2000],
    pub dmbuffer: [uchar; 512],
    pub ipbuffer: [uchar; 512],
    pub e: [epoll_event; 1000],
    pub rndidx: libc::c_int,
    pub dataidx: libc::c_int,
    pub ip: [uchar; 2000],
    pub eptcpfds: [eptcpfds; 65530],
    pub rdb: uint,
    pub ddbefore: libc::c_int,
    pub underattack: libc::c_int,
    pub tcpinuse: libc::c_int,
    pub fwd: *mut htable,
    pub ds: *mut htable,
    pub dupbefore: libc::c_int,
    pub limits: libc::c_int,
    pub hsidx: libc::c_int,
    pub quizz: uint,
    pub drop: uint,
    pub timeout: uint,
    pub start: libc::c_int,
    pub end: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server {
    pub nquizzer: ushort,
    pub nfetcher: ushort,
    pub ludp: libc::c_int,
    pub ltcp: libc::c_int,
    pub fetchers: *mut fetcher,
    pub authors: *mut author,
    pub eventlist: list,
    pub datasets: *mut htable,
    pub forward: *mut htable,
    pub qlist: *mut htable,
    pub pkg: ulong,
    pub logpath: [uchar; 255],
    pub recordsindb: ulong,
    pub ttlexp: *mut rbtree,
    pub refreshflag: uint16_t,
    pub lastrefresh: time_t,
    pub is_forward: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fetcher {
    pub idx: libc::c_int,
    pub s: *mut server,
    pub mc: *mut msgcache,
    pub el: *mut list,
    pub loginfo: *mut log_info,
    pub originbuffer: [uchar; 65528],
    pub tdbuffer: [uchar; 256],
    pub databuffer: [uchar; 65528],
    pub cbbuffer: [uchar; 512],
    pub dataidx: libc::c_int,
    pub qidx: libc::c_int,
    pub pkg: uint64_t,
    pub send: uint64_t,
    pub miss: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread_query_info {
    pub query_num: [libc::c_ulong; 9],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct global_query_info {
    pub thread_num: libc::c_int,
    pub log_flag: libc::c_int,
    pub query_info: [thread_query_info; 65],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_data {
    pub fd: libc::c_int,
    pub cb: noti_chain_callback,
    pub ext: *mut libc::c_void,
}
pub type noti_chain_callback = Option::<
    unsafe extern "C" fn(*mut event_data, *mut libc::c_void, libc::c_int) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iner_event {
    pub epfd: libc::c_int,
    pub buf: *mut libc::c_char,
    pub e: [epoll_event; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub size: libc::c_int,
    pub onexit: libc::c_int,
    pub ie: *mut iner_event,
    pub data: [event_data; 0],
}
pub type event_type = libc::c_uint;
pub const ET_ALL: event_type = 3;
pub const ET_WRITE: event_type = 2;
pub const ET_READ: event_type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_help {
    pub fd: libc::c_int,
    pub spfd: libc::c_int,
    pub num: libc::c_int,
    pub type_0: event_type,
    pub to: *mut timeval,
    pub cb: noti_chain_callback,
    pub ext: *mut libc::c_void,
}
pub static mut query_type_map: [libc::c_int; 256] = [0; 256];
pub static mut global_out_info: *mut global_query_info = 0 as *const global_query_info
    as *mut global_query_info;
pub static mut g_nameservers: [*mut libc::c_char; 2] = [0 as *const libc::c_char
    as *mut libc::c_char; 2];
pub static mut global_serv: *mut server = 0 as *const server as *mut server;
pub unsafe extern "C" fn create_event(mut size: libc::c_int) -> *mut event {
    let mut ev: *mut event = malloc(
        (::std::mem::size_of::<event>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<event_data>() as libc::c_ulong)
                    .wrapping_mul(size as libc::c_ulong),
            ),
    ) as *mut event;
    let mut epfd: libc::c_int = epoll_create(size);
    if epfd == 0 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"epoll create\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    (*ev).size = size;
    (*ev)
        .ie = malloc(
        (::std::mem::size_of::<iner_event>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<epoll_event>() as libc::c_ulong)
                    .wrapping_mul((*ev).size as libc::c_ulong),
            ),
    ) as *mut iner_event;
    if ((*ev).ie).is_null() {
        dns_error(
            0 as libc::c_int,
            b"alloc iner event\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    memset(
        (*ev).ie as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<iner_event>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<epoll_event>() as libc::c_ulong)
                    .wrapping_mul((*ev).size as libc::c_ulong),
            ),
    );
    (*(*ev).ie).epfd = epfd;
    return ev;
}
pub unsafe extern "C" fn add_event(
    mut ev: *mut event,
    mut help: *mut event_help,
) -> libc::c_int {
    let mut e: epoll_event = {
        let mut init = epoll_event {
            events: 0 as libc::c_int as uint32_t,
            data: epoll_data {
                ptr: 0 as *mut libc::c_void,
            },
        };
        init
    };
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut epfd: libc::c_int = (*(*ev).ie).epfd;
    e.data.fd = (*help).fd;
    if e.data.fd < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*help).type_0 as libc::c_uint == ET_READ as libc::c_int as libc::c_uint {
        e.events = EPOLLIN as libc::c_int as uint32_t;
    }
    if (*help).type_0 as libc::c_uint == ET_WRITE as libc::c_int as libc::c_uint {
        e.events = EPOLLOUT as libc::c_int as uint32_t;
    }
    let ref mut fresh0 = (*((*ev).data).as_mut_ptr().offset((*help).fd as isize)).cb;
    *fresh0 = (*help).cb;
    if !((*help).ext).is_null() {
        let ref mut fresh1 = (*((*ev).data).as_mut_ptr().offset((*help).fd as isize))
            .ext;
        *fresh1 = (*help).ext;
    }
    ret = epoll_ctl(epfd, 1 as libc::c_int, (*help).fd, &mut e);
    if ret < 0 as libc::c_int {
        printf(b"fd is %d\n\0" as *const u8 as *const libc::c_char, (*help).fd);
        perror(b"epoll_ctl\0" as *const u8 as *const libc::c_char);
    }
    return ret;
}
pub unsafe extern "C" fn del_event(
    mut ev: *mut event,
    mut help: *mut event_help,
) -> libc::c_int {
    let mut e: epoll_event = epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    };
    let mut ie: *mut iner_event = (*ev).ie;
    let mut ret: libc::c_int = 0 as libc::c_int;
    e.data.fd = (*help).fd;
    ret = epoll_ctl((*ie).epfd, 2 as libc::c_int, (*help).fd, &mut e);
    return ret;
}
pub unsafe extern "C" fn handle_event(
    mut ev: *mut event,
    mut to: libc::c_int,
) -> libc::c_int {
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut ie: *mut iner_event = (*ev).ie;
    if to == 0 as libc::c_int {
        to = -(1 as libc::c_int);
    } else {
        to = to * 100 as libc::c_int;
    }
    (*ev).size = 100 as libc::c_int;
    loop {
        num = epoll_wait((*ie).epfd, ((*ie).e).as_mut_ptr(), (*ev).size, to);
        if num >= 0 as libc::c_int {
            break;
        }
        num < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int;
    }
    return num;
}
pub unsafe extern "C" fn cb_get_tcp_msg(
    mut data: *mut event_data,
    mut v: *mut libc::c_void,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut szhdr: libc::c_int = ::std::mem::size_of::<dnsheader>() as libc::c_ulong
        as libc::c_int;
    let mut mc: *mut msgcache = 0 as *mut msgcache;
    let mut f: *mut fetcher = v as *mut fetcher;
    let mut mbuf: *mut mbuf_type = 0 as *mut mbuf_type;
    mbuf = mbuf_alloc();
    if mbuf.is_null() {
        return 0 as libc::c_int;
    }
    memset(
        mbuf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mbuf_type>() as libc::c_ulong,
    );
    mc = (*f.offset(idx as isize)).mc;
    pthread_spin_lock(&mut (*mc).lock);
    if ((*mc).tail).wrapping_add(8 as libc::c_int as libc::c_ulong)
        > (*mc).size as libc::c_ulong
    {
        (*mc).tail = 0 as libc::c_int as uint64_t;
    }
    if ((*mc).tail).wrapping_add(8 as libc::c_int as libc::c_ulong) > (*mc).head
        && (*mc).tail < (*mc).head
        || (*mc).tail == (*mc).head && (*mc).pkt != 0 as libc::c_int as libc::c_uint
    {
        close((*data).fd);
        let ref mut fresh2 = (*f.offset(idx as isize)).miss;
        *fresh2 = (*fresh2).wrapping_add(1);
        *fresh2;
        pthread_spin_unlock(&mut (*mc).lock);
        mbuf_free(mbuf);
        return 0 as libc::c_int;
    }
    let ref mut fresh3 = (*f.offset(idx as isize)).pkg;
    *fresh3 = (*fresh3).wrapping_add(1);
    *fresh3;
    (*mbuf).socktype = SOCK_STREAM as libc::c_int as uint;
    (*mbuf).fd = (*data).fd;
    (*mbuf).buf = ((*mbuf).data).as_mut_ptr();
    (*mbuf).buflen = 4096 as libc::c_int;
    ret = tcp_read_dns_msg(mbuf, 4096 as libc::c_int as uint, 0 as libc::c_int);
    if ret < szhdr {
        pthread_spin_unlock(&mut (*mc).lock);
        mbuf_free(mbuf);
        return -(1 as libc::c_int);
    }
    (*mbuf).fetch_len = ret as uint;
    memcpy(
        ((*mc).data).as_mut_ptr().offset((*mc).tail as isize) as *mut libc::c_void,
        &mut mbuf as *mut *mut mbuf_type as *const libc::c_void,
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
    );
    (*mc)
        .tail = ((*mc).tail)
        .wrapping_add(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong);
    if ((*mc).tail).wrapping_add(8 as libc::c_int as libc::c_ulong)
        > (*mc).size as libc::c_ulong
    {
        (*mc).tail = 0 as libc::c_int as uint64_t;
    }
    (*mc).pkt = ((*mc).pkt).wrapping_add(1);
    (*mc).pkt;
    pthread_spin_unlock(&mut (*mc).lock);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fake_recv(
    mut data: *mut event_data,
    mut v: *mut libc::c_void,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut f: *mut fetcher = v as *mut fetcher;
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut buffer: [uchar; 512] = [
        0 as libc::c_int as uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut ret: libc::c_int = 0;
    idx = 0 as libc::c_int;
    let mut len: socklen_t = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
        as socklen_t;
    loop {
        ret = recvfrom(
            (*data).fd,
            buffer.as_mut_ptr() as *mut libc::c_void,
            512 as libc::c_int as size_t,
            0 as libc::c_int,
            __SOCKADDR_ARG {
                __sockaddr__: &mut addr as *mut sockaddr_in as *mut SA as *mut sockaddr,
            },
            &mut len,
        ) as libc::c_int;
        if ret > 0 as libc::c_int {
            let ref mut fresh4 = (*f.offset(idx as isize)).pkg;
            *fresh4 = (*fresh4).wrapping_add(1);
            *fresh4;
        } else {
            return 0 as libc::c_int
        }
    };
}
pub unsafe extern "C" fn cb_get_udp_msg(
    mut data: *mut event_data,
    mut v: *mut libc::c_void,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut szhdr: libc::c_int = ::std::mem::size_of::<dnsheader>() as libc::c_ulong
        as libc::c_int;
    let mut mc: *mut msgcache = 0 as *mut msgcache;
    let mut f: *mut fetcher = v as *mut fetcher;
    let mut mbuf: *mut mbuf_type = 0 as *mut mbuf_type;
    loop {
        mbuf = mbuf_alloc();
        if mbuf.is_null() {
            return 0 as libc::c_int;
        }
        memset(
            mbuf as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<mbuf_type>() as libc::c_ulong,
        );
        mc = (*f.offset(idx as isize)).mc;
        pthread_spin_lock(&mut (*mc).lock);
        if ((*mc).tail).wrapping_add(8 as libc::c_int as libc::c_ulong) > (*mc).head
            && (*mc).tail < (*mc).head
            || (*mc).tail == (*mc).head && (*mc).pkt != 0 as libc::c_int as libc::c_uint
        {
            let ref mut fresh5 = (*f.offset(idx as isize)).miss;
            *fresh5 = (*fresh5).wrapping_add(1);
            *fresh5;
            pthread_spin_unlock(&mut (*mc).lock);
            mbuf_free(mbuf);
            return 0 as libc::c_int;
        }
        let ref mut fresh6 = (*f.offset(idx as isize)).pkg;
        *fresh6 = (*fresh6).wrapping_add(1);
        *fresh6;
        (*mbuf).socktype = SOCK_DGRAM as libc::c_int as uint;
        (*mbuf).fd = (*data).fd;
        (*mbuf).buf = ((*mbuf).data).as_mut_ptr().offset(2 as libc::c_int as isize);
        (*mbuf).buflen = 4096 as libc::c_int - 2 as libc::c_int;
        (*mbuf).addr = &mut (*mbuf).caddr;
        ret = udp_read_msg(mbuf, 0 as libc::c_int);
        if ret < szhdr {
            pthread_spin_unlock(&mut (*mc).lock);
            mbuf_free(mbuf);
            return -(1 as libc::c_int);
        }
        (*mbuf).fetch_len = ret as uint;
        memcpy(
            ((*mc).data).as_mut_ptr().offset((*mc).tail as isize) as *mut libc::c_void,
            &mut mbuf as *mut *mut mbuf_type as *const libc::c_void,
            ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
        );
        (*mc)
            .tail = ((*mc).tail)
            .wrapping_add(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong);
        if ((*mc).tail).wrapping_add(8 as libc::c_int as libc::c_ulong)
            > (*mc).size as libc::c_ulong
        {
            (*mc).tail = 0 as libc::c_int as uint64_t;
        }
        (*mc).pkt = ((*mc).pkt).wrapping_add(1);
        (*mc).pkt;
        pthread_spin_unlock(&mut (*mc).lock);
    };
}
pub unsafe extern "C" fn insert_events(
    mut ev: *mut event,
    mut fd: libc::c_int,
    mut type_0: libc::c_int,
) -> libc::c_int {
    let mut h: event_help = event_help {
        fd: 0,
        spfd: 0,
        num: 0,
        type_0: 0 as event_type,
        to: 0 as *mut timeval,
        cb: None,
        ext: 0 as *mut libc::c_void,
    };
    if fd > 0 as libc::c_int {
        memset(
            &mut h as *mut event_help as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<event_help>() as libc::c_ulong,
        );
        h.type_0 = ET_READ;
        h.fd = fd;
        if type_0 == SOCK_DGRAM as libc::c_int {
            h
                .cb = Some(
                cb_get_udp_msg
                    as unsafe extern "C" fn(
                        *mut event_data,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> libc::c_int,
            );
        } else {
            h
                .cb = Some(
                cb_get_tcp_msg
                    as unsafe extern "C" fn(
                        *mut event_data,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> libc::c_int,
            );
        }
        if add_event(ev, &mut h) < 0 as libc::c_int {
            dns_error(
                1 as libc::c_int,
                b"add event notify\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn run_sentinel(mut s: *mut server) -> libc::c_int {
    let mut num: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ls: libc::c_int = 0;
    let mut connfd: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut fidx: libc::c_int = 0 as libc::c_int;
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut h: event_help = event_help {
        fd: 0,
        spfd: 0,
        num: 0,
        type_0: 0 as event_type,
        to: 0 as *mut timeval,
        cb: None,
        ext: 0 as *mut libc::c_void,
    };
    let mut len: socklen_t = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
        as socklen_t;
    let mut f: *mut fetcher = (*s).fetchers;
    let mut ev: *mut event = create_event(1000 as libc::c_int);
    let mut cpuinfo: cpu_set_t = cpu_set_t { __bits: [0; 16] };
    let mut pt: pthread_t = pthread_self();
    libc::memset(
        &mut cpuinfo as *mut cpu_set_t as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong as libc::size_t,
    );
    let mut __cpu: size_t = 0 as libc::c_int as size_t;
    if __cpu.wrapping_div(8 as libc::c_int as libc::c_ulong)
        < ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong
    {
        let ref mut fresh7 = *(cpuinfo.__bits)
            .as_mut_ptr()
            .offset(
                __cpu
                    .wrapping_div(
                        (8 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                            ),
                    ) as isize,
            );
        *fresh7
            |= (1 as libc::c_int as __cpu_mask)
                << __cpu
                    .wrapping_rem(
                        (8 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                            ),
                    );
    } else {};
    if 0 as libc::c_int
        != pthread_setaffinity_np(
            pt,
            ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong,
            &mut cpuinfo,
        )
    {
        printf(b"set affinity fetcher\n\0" as *const u8 as *const libc::c_char);
        exit(0 as libc::c_int);
    }
    if ev.is_null() {
        dns_error(
            0 as libc::c_int,
            b"create event st\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    insert_events(ev, (*s).ludp, SOCK_DGRAM as libc::c_int);
    insert_events(ev, (*s).ltcp, SOCK_STREAM as libc::c_int);
    ls = (*s).ltcp;
    loop {
        num = handle_event(ev, 1 as libc::c_int);
        global_cron(s);
        i = 0 as libc::c_int;
        while i < num {
            let mut fd: libc::c_int = (*((*(*ev).ie).e).as_mut_ptr().offset(i as isize))
                .data
                .fd;
            let mut cb: noti_chain_callback = (*((*ev).data)
                .as_mut_ptr()
                .offset(fd as isize))
                .cb;
            (*((*ev).data).as_mut_ptr().offset(fd as isize)).fd = fd;
            if fd == ls {
                connfd = accept(
                    fd,
                    __SOCKADDR_ARG {
                        __sockaddr__: &mut addr as *mut sockaddr_in as *mut SA
                            as *mut sockaddr,
                    },
                    &mut len,
                );
                set_non_block(connfd);
                insert_events(ev, connfd, SOCK_STREAM as libc::c_int);
            } else if cb.is_some() {
                fidx += 1;
                fidx;
                fidx = fidx % FETCHER_NUM as libc::c_int;
                if fidx >= FETCHER_NUM as libc::c_int {
                    fidx = FETCHER_NUM as libc::c_int - 1 as libc::c_int;
                }
                ret = (Some(cb.unwrap()))
                    .unwrap()(
                    ((*ev).data).as_mut_ptr().offset(fd as isize),
                    f as *mut libc::c_void,
                    fidx,
                );
                if cb
                    == Some(
                        cb_get_tcp_msg
                            as unsafe extern "C" fn(
                                *mut event_data,
                                *mut libc::c_void,
                                libc::c_int,
                            ) -> libc::c_int,
                    )
                {
                    if ret == -(1 as libc::c_int) {
                        close(fd);
                    }
                    h.fd = fd;
                    del_event(ev, &mut h);
                }
            } else {
                dns_error(
                    1 as libc::c_int,
                    b"call back func is null\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            i += 1;
            i;
        }
    };
}
