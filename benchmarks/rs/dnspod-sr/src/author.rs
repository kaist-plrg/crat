use ::libc;
extern "C" {
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn random() -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn pthread_detach(__th: pthread_t) -> libc::c_int;
    fn pthread_self() -> pthread_t;
    static mut global_now: time_t;
    fn get_random_data(_: *mut uchar, _: libc::c_int) -> libc::c_int;
    fn delete_node(rbt: *mut rbtree, nd: *mut rbnode) -> *mut libc::c_void;
    fn min_node(rbt: *mut rbtree) -> *mut rbnode;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn mbuf_alloc() -> *mut mbuf_type;
    fn mbuf_free(mbuf: *mut mbuf_type) -> libc::c_int;
    static MAX_ELE_NUM: uint;
    fn htable_find_io(
        ht: *mut htable,
        idx: libc::c_int,
        limit: uint32_t,
        rbt: *mut rbtree,
        ttl_update: libc::c_int,
    ) -> libc::c_int;
    fn find_record_with_ttl(
        _: *mut htable,
        _: *mut uchar,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut uchar,
        _: libc::c_int,
        metadata: *mut mvalue,
        hash: *mut hashval_t,
    ) -> libc::c_int;
    fn htable_find_list_io(
        ht: *mut htable,
        idx: libc::c_int,
        off: libc::c_int,
        typeoff: *mut libc::c_int,
        buffer: *mut *mut uchar,
    ) -> libc::c_int;
    fn htable_find_list(
        ht: *mut htable,
        key: *mut uchar,
        typeoff: libc::c_int,
        idx: libc::c_int,
        buffer: *mut *mut uchar,
    ) -> libc::c_int;
    fn htable_delete_list(
        ht: *mut htable,
        key: *mut uchar,
        typeoff: libc::c_int,
        idx: libc::c_int,
    ) -> *mut uchar;
    fn htable_insert_list(
        _: *mut htable,
        _: *mut uchar,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut uchar,
        _: libc::c_int,
        _: *mut mvalue,
        hashd: *mut hashval_t,
    ) -> libc::c_int;
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
    fn add_backdoor(fd: libc::c_int) -> libc::c_int;
    fn udp_write_info(mbuf: *mut mbuf_type, _: libc::c_int) -> libc::c_int;
    fn udp_read_msg(mbuf: *mut mbuf_type, _: libc::c_int) -> libc::c_int;
    fn tcp_write_info(mbuf: *mut mbuf_type, _: libc::c_int) -> libc::c_int;
    fn tcp_read_dns_msg(mbuf: *mut mbuf_type, _: uint, _: libc::c_int) -> libc::c_int;
    fn connect_to(_: *mut sockinfo) -> libc::c_int;
    fn set_recv_timeout(
        fd: libc::c_int,
        sec: libc::c_int,
        usec: libc::c_int,
    ) -> libc::c_int;
    fn set_non_block(fd: libc::c_int) -> libc::c_int;
    fn make_addr_from_bin(addr: *mut sockaddr_in, data: *mut uchar) -> libc::c_int;
    fn find_addr(
        fwd: *mut htable,
        _: *mut htable,
        mbuf: *mut mbuf_type,
        _: *mut uchar,
        _: libc::c_int,
    ) -> libc::c_int;
    fn fill_header_in_msg(_: *mut setheader) -> *mut uchar;
    fn fill_rrset_in_msg(
        _: *mut hlpc,
        _: *mut uchar,
        _: *mut uchar,
        _: *mut libc::c_int,
        _: *mut uchar,
    ) -> *mut uchar;
    fn check_an_msg(_: ushort, _: *mut uchar, _: *mut libc::c_int) -> libc::c_int;
    fn check_dns_name(domain: *mut uchar, lowerdomain: *mut packet_type) -> libc::c_int;
    fn make_dns_msg_for_new(
        _: *mut uchar,
        _: ushort,
        _: *mut uchar,
        _: libc::c_int,
        _: ushort,
    ) -> libc::c_int;
    fn send_tc_to_client(mbuf: *mut mbuf_type) -> libc::c_int;
    fn passer_dns_data(mbuf: *mut mbuf_type);
    fn process_rdata(_: *mut hlpp, _: *mut uchar, _: libc::c_int) -> *mut uchar;
    fn write_log(
        _: *mut log_info,
        _: libc::c_int,
        _: *const uchar,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut sockaddr_in,
    ) -> libc::c_int;
    fn refresh_records(_: *mut htable, _: *mut rbtree) -> libc::c_int;
    fn pthread_spin_unlock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    fn pthread_spin_lock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type time_t = __time_t;
pub type ulong = libc::c_ulong;
pub type ushort = libc::c_ushort;
pub type uint = libc::c_uint;
pub type int32_t = __int32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ttlnode {
    pub exp: uint,
    pub dlen: ushort,
    pub type_0: ushort,
    pub hash: *mut hashval_t,
    pub lowerdomain: *mut packet_type,
    pub data: *mut uchar,
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
pub type htable_insert_ret = libc::c_int;
pub const HTABLE_INSERT_RET_NO_REPLACE: htable_insert_ret = 3;
pub const HTABLE_INSERT_RET_NEVER_EXPIRE: htable_insert_ret = 2;
pub const HTABLE_INSERT_RET_REPLACE: htable_insert_ret = 1;
pub const HTABLE_INSERT_RET_NORMAL: htable_insert_ret = 0;
pub const HTABLE_INSERT_RET_INVALID_TYPE: htable_insert_ret = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mvalue {
    pub len: uint16_t,
    pub num: uint16_t,
    pub ttl: uint32_t,
    pub hits: uint32_t,
    pub seg: uint16_t,
}
pub type hashfunc = unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> hashval_t;
pub type comparefunc = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hentry {
    pub c2rust_unnamed: C2RustUnnamed,
    pub next: *mut hentry,
    pub count: libc::c_int,
    pub key: [uchar; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IP_DATA_LEN: C2RustUnnamed_0 = 2000;
pub const MAX_TRY_TIMES: C2RustUnnamed_0 = 15;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct setheader {
    pub an: ushort,
    pub ns: ushort,
    pub id: ushort,
    pub dlen: ushort,
    pub od: *mut uchar,
    pub itor: *mut uchar,
    pub type_0: ushort,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const DMS_SIZE: C2RustUnnamed_1 = 256;
pub const AR_SECTION: C2RustUnnamed_1 = 7;
pub const NS_SECTION: C2RustUnnamed_1 = 5;
pub const AN_SECTION: C2RustUnnamed_1 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hlpp {
    pub stype: *mut libc::c_int,
    pub ds: *mut htable,
    pub rbt: *mut rbtree,
    pub buf: *mut uchar,
    pub datalen: libc::c_int,
    pub dms: *mut uchar,
    pub dmsidx: libc::c_int,
    pub section: libc::c_int,
    pub tmpbuf: *mut uchar,
    pub domainbuf: *mut uchar,
    pub dmbuf: *mut uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hlpc {
    pub name: *mut uchar,
    pub off: libc::c_short,
    pub level: libc::c_short,
    pub ref_0: libc::c_short,
    pub mt: libc::c_short,
    pub len: libc::c_short,
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
#[repr(C, packed)]
pub struct tag_dq {
    pub type_0: uint16_t,
    pub dclass: uint16_t,
}
pub type qdns = tag_dq;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const MOST_TRY_PER_QUERY: C2RustUnnamed_2 = 3;
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const Q_NS: C2RustUnnamed_3 = 6;
pub const Q_DOMAIN: C2RustUnnamed_3 = 4;
pub const Q_CNAME: C2RustUnnamed_3 = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct log_info {
    pub logfd: libc::c_int,
    pub lastlog: time_t,
    pub log_type: libc::c_int,
    pub log_cache: [uchar; 1048576],
    pub log_cache_cursor: libc::c_int,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const SERVER_PORT: C2RustUnnamed_4 = 53;
pub const FETCHER_NUM: C2RustUnnamed_4 = 2;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const QUIZZER_NUM: C2RustUnnamed_5 = 2;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const SHM_KEY: C2RustUnnamed_6 = 38899;
pub const TTL_UPDATE: C2RustUnnamed_6 = 3;
pub const PROCESS_QUERY: C2RustUnnamed_6 = 1;
pub const NEW_QUERY: C2RustUnnamed_6 = 0;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const EP_TCP_FDS: C2RustUnnamed_7 = 65530;
pub const AUTH_DATA_LEN: C2RustUnnamed_7 = 65528;
pub const ID_SPACE: C2RustUnnamed_7 = 60000;
pub const RANDOM_SIZE: C2RustUnnamed_7 = 3000;
pub const BIG_MEM_STEP: C2RustUnnamed_7 = 2000;
pub const AUTH_DB_NUM: C2RustUnnamed_7 = 101;
pub const REFRESH_INTERVAL: C2RustUnnamed_7 = 10;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const LIST_SPACE: C2RustUnnamed_8 = 10000;
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
pub union grifa {
    pub val: libc::c_int,
    pub randombuffer: [uchar; 4],
}
pub static mut query_type_map: [libc::c_int; 256] = [0; 256];
pub static mut global_out_info: *mut global_query_info = 0 as *const global_query_info
    as *mut global_query_info;
pub static mut g_nameservers: [*mut libc::c_char; 2] = [0 as *const libc::c_char
    as *mut libc::c_char; 2];
pub static mut global_serv: *mut server = 0 as *const server as *mut server;
pub static mut qlist_val: [uchar; 10] = unsafe {
    *::std::mem::transmute::<&[u8; 10], &mut [uchar; 10]>(b"qlist val\0")
};
pub unsafe extern "C" fn add_query_info(
    mut log_type: libc::c_int,
    mut idx: libc::c_int,
    mut type_0: uint16_t,
) -> libc::c_int {
    let mut thread_num: libc::c_int = 0 as libc::c_int;
    if log_type == 112 as libc::c_int {
        thread_num = idx;
    } else if log_type == 233 as libc::c_int {
        thread_num = idx + FETCHER_NUM as libc::c_int;
    } else {
        return -(1 as libc::c_int)
    }
    let mut query_type_num: libc::c_int = query_type_map[type_0 as usize];
    if query_type_num < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*global_out_info)
        .query_info[thread_num as usize]
        .query_num[query_type_num
        as usize] = ((*global_out_info)
        .query_info[thread_num as usize]
        .query_num[query_type_num as usize])
        .wrapping_add(1);
    (*global_out_info)
        .query_info[thread_num as usize]
        .query_num[query_type_num as usize];
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn get_random_int_from_author(
    mut author: *mut author,
) -> libc::c_int {
    let mut val: libc::c_int = 0 as libc::c_int;
    let mut tmp: grifa = grifa { val: 0 };
    if ((*author).rndidx as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        >= RANDOM_SIZE as libc::c_int as libc::c_ulong
    {
        get_random_data(
            ((*author).randombuffer).as_mut_ptr(),
            RANDOM_SIZE as libc::c_int,
        );
        (*author).rndidx = 0 as libc::c_int;
    }
    memcpy(
        (tmp.randombuffer).as_mut_ptr() as *mut libc::c_void,
        ((*author).randombuffer).as_mut_ptr().offset((*author).rndidx as isize)
            as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
    val = tmp.val;
    (*author)
        .rndidx = ((*author).rndidx as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int as libc::c_int;
    return val;
}
pub unsafe extern "C" fn delete_close_event(
    mut fd: libc::c_int,
    mut f: *mut fetcher,
) -> libc::c_int {
    let mut el: *mut list = 0 as *mut list;
    let mut nd: *mut list_node = 0 as *mut list_node;
    el = (*f).el;
    if el.is_null() {
        return -(1 as libc::c_int);
    }
    nd = malloc(::std::mem::size_of::<list_node>() as libc::c_ulong) as *mut list_node;
    if nd.is_null() {
        return -(1 as libc::c_int);
    }
    (*nd).data = malloc(::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if ((*nd).data).is_null() {
        free(nd as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    memcpy(
        (*nd).data,
        &mut fd as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
    pthread_spin_lock(&mut (*el).lock);
    (*nd).next = (*el).head;
    (*el).head = nd;
    pthread_spin_unlock(&mut (*el).lock);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn write_back_to_client(
    mut mbuf: *mut mbuf_type,
    mut fr: *mut uchar,
    mut vlen: libc::c_int,
) -> libc::c_int {
    let mut sh: setheader = {
        let mut init = setheader {
            an: 0 as libc::c_int as ushort,
            ns: 0,
            id: 0,
            dlen: 0,
            od: 0 as *mut uchar,
            itor: 0 as *mut uchar,
            type_0: 0,
        };
        init
    };
    let mut main_val: libc::c_int = 0 as libc::c_int;
    let mut dnslen: libc::c_int = 0 as libc::c_int;
    let mut msg: *mut uchar = (*mbuf).buf;
    let mut type_0: uchar = 0;
    let mut from: *mut uchar = fr;
    let mut to: *mut uchar = msg;
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    let mut jump: libc::c_int = 0 as libc::c_int;
    let mut temp: uint16_t = 0 as libc::c_int as uint16_t;
    let mut hlp: [hlpc; 100] = [hlpc {
        name: 0 as *mut uchar,
        off: 0,
        level: 0,
        ref_0: 0,
        mt: 0,
        len: 0,
    }; 100];
    hlp[0 as libc::c_int as usize].name = (*mbuf).td;
    hlp[0 as libc::c_int as usize]
        .off = ::std::mem::size_of::<dnsheader>() as libc::c_ulong as libc::c_short;
    hlp[0 as libc::c_int as usize]
        .level = (*mbuf).lowerdomain.label_count as libc::c_short;
    hlp[0 as libc::c_int as usize].ref_0 = -(1 as libc::c_int) as libc::c_short;
    hlp[0 as libc::c_int as usize].mt = 0 as libc::c_int as libc::c_short;
    hlp[0 as libc::c_int as usize].len = (*mbuf).dlen as libc::c_short;
    if (*mbuf).dlen == 2 as libc::c_int {
        jump = (::std::mem::size_of::<dnsheader>() as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<qdns>() as libc::c_ulong) as libc::c_int;
    } else {
        jump = (::std::mem::size_of::<dnsheader>() as libc::c_ulong)
            .wrapping_add((*mbuf).dlen as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<qdns>() as libc::c_ulong) as libc::c_int;
    }
    to = to.offset(jump as isize);
    while vlen > 1 as libc::c_int {
        type_0 = *from.offset(0 as libc::c_int as isize);
        mv = from.offset(1 as libc::c_int as isize) as *mut mvalue;
        to = fill_rrset_in_msg(hlp.as_mut_ptr(), from, to, &mut main_val, msg);
        if to.is_null() {
            return -(1 as libc::c_int);
        }
        vlen = ((vlen - 1 as libc::c_int - (*mv).len as libc::c_int) as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<mvalue>() as libc::c_ulong)
            as libc::c_int;
        sh.an = (sh.an as libc::c_int + (*mv).num as libc::c_int) as ushort;
        from = from
            .offset((*mv).len as libc::c_int as isize)
            .offset(1 as libc::c_int as isize)
            .offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
        if type_0 as libc::c_int == CNAME as libc::c_int && vlen > 1 as libc::c_int
            && (*from.offset(0 as libc::c_int as isize) as libc::c_int
                == A as libc::c_int
                || *from.offset(0 as libc::c_int as isize) as libc::c_int
                    == AAAA as libc::c_int
                || *from.offset(0 as libc::c_int as isize) as libc::c_int
                    == CNAME as libc::c_int)
        {
            main_val += 1;
            main_val;
            hlp[main_val as usize]
                .name = hlp[(main_val - 1 as libc::c_int) as usize].name;
            hlp[main_val as usize].off = hlp[(main_val - 1 as libc::c_int) as usize].off;
            hlp[main_val as usize]
                .level = hlp[(main_val - 1 as libc::c_int) as usize].level;
            hlp[main_val as usize].len = hlp[(main_val - 1 as libc::c_int) as usize].len;
            hlp[main_val as usize].ref_0 = -(1 as libc::c_int) as libc::c_short;
            hlp[main_val as usize].mt = 0 as libc::c_int as libc::c_short;
        }
    }
    sh.itor = msg;
    sh
        .dlen = (if (*mbuf).dlen == 2 as libc::c_int {
        1 as libc::c_int
    } else {
        (*mbuf).dlen
    }) as ushort;
    sh.od = (*mbuf).td;
    sh.id = (*mbuf).id;
    sh.type_0 = (*mbuf).qtype as ushort;
    fill_header_in_msg(&mut sh);
    dnslen = to.offset_from(msg) as libc::c_long as libc::c_int;
    (*mbuf).buflen = dnslen;
    (*mbuf).addr = &mut (*mbuf).caddr;
    if (*mbuf).socktype == SOCK_DGRAM as libc::c_int as libc::c_uint {
        if dnslen > 512 as libc::c_int {
            send_tc_to_client(mbuf);
        } else {
            udp_write_info(mbuf, 0 as libc::c_int);
        }
    } else {
        temp = (dnslen as uint16_t as libc::c_int >> 8 as libc::c_int
            | (dnslen << 8 as libc::c_int) as uint16_t as libc::c_int) as uint16_t;
        memcpy(
            msg.offset(-(2 as libc::c_int as isize)) as *mut libc::c_void,
            &mut temp as *mut uint16_t as *const libc::c_void,
            ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
        );
        (*mbuf).buflen = dnslen + 2 as libc::c_int;
        (*mbuf).buf = msg.offset(-(2 as libc::c_int as isize));
        tcp_write_info(mbuf, 0 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn passer_related_data(
    mut si: *mut sockinfo,
    mut mbuf: *mut mbuf_type,
    mut author: *mut author,
) -> libc::c_int {
    let mut buf: *mut uchar = (*si).buf;
    let mut tail: *mut uchar = 0 as *mut uchar;
    let mut stype: libc::c_int = 0 as libc::c_int;
    let mut rbt: *mut rbtree = 0 as *mut rbtree;
    let mut datalen: libc::c_int = 0 as libc::c_int;
    let mut n: ushort = 0;
    let mut hlp: hlpp = hlpp {
        stype: 0 as *mut libc::c_int,
        ds: 0 as *mut htable,
        rbt: 0 as *mut rbtree,
        buf: 0 as *mut uchar,
        datalen: 0,
        dms: 0 as *mut uchar,
        dmsidx: 0,
        section: 0,
        tmpbuf: 0 as *mut uchar,
        domainbuf: 0 as *mut uchar,
        dmbuf: 0 as *mut uchar,
    };
    let mut hdr: *mut dnsheader = buf as *mut dnsheader;
    tail = buf
        .offset(::std::mem::size_of::<dnsheader>() as libc::c_ulong as isize)
        .offset(
            (*(*si).lowerdomain).label_len[0 as libc::c_int as usize] as libc::c_int
                as isize,
        );
    tail = tail.offset(4 as libc::c_int as isize);
    datalen = (*si).buflen;
    rbt = (*(*author).s).ttlexp;
    n = ntohs((*hdr).ancount);
    hlp.stype = &mut stype;
    hlp.ds = (*(*author).s).datasets;
    hlp.rbt = rbt;
    hlp.buf = buf;
    hlp.datalen = datalen;
    hlp.tmpbuf = (*mbuf).tempbuffer;
    hlp.domainbuf = (*mbuf).tdbuffer;
    hlp.dmbuf = (*mbuf).dmbuffer;
    if n as libc::c_int > 0 as libc::c_int {
        hlp.section = AN_SECTION as libc::c_int;
        tail = process_rdata(&mut hlp, tail, n as libc::c_int);
        if tail.is_null() {
            return -(1 as libc::c_int);
        }
    }
    n = ntohs((*hdr).nscount);
    if n as libc::c_int > 0 as libc::c_int {
        hlp.section = NS_SECTION as libc::c_int;
        tail = process_rdata(&mut hlp, tail, n as libc::c_int);
        if tail.is_null() {
            return -(1 as libc::c_int);
        }
    }
    n = ntohs((*hdr).arcount);
    if n as libc::c_int > 0 as libc::c_int {
        if tail.offset(9 as libc::c_uint as isize).offset(2 as libc::c_int as isize)
            <= buf.offset(datalen as isize)
        {
            let mut opt_owner: uint8_t = *(tail as *mut uint8_t);
            let mut opt_type: uint16_t = *(tail.offset(1 as libc::c_int as isize)
                as *mut uint16_t);
            if opt_owner as libc::c_int == 0 as libc::c_int
                && opt_type as libc::c_int >> 8 as libc::c_int
                    | ((opt_type as libc::c_int) << 8 as libc::c_int) as uint16_t
                        as libc::c_int == OPT as libc::c_int
            {
                return stype;
            }
        }
        hlp.section = AR_SECTION as libc::c_int;
        tail = process_rdata(&mut hlp, tail, n as libc::c_int);
        if tail.is_null() {
            return -(1 as libc::c_int);
        }
    }
    return stype;
}
pub unsafe extern "C" fn send_msg_tcp(
    mut author: *mut author,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut id: ushort = 0;
    let mut typeoff: ushort = 0;
    let mut temp: ushort = 0;
    let mut type_0: ushort = 0;
    let mut buffer: *mut uchar = ((*author).tmpbuffer).as_mut_ptr();
    let mut len: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut mbuf: *mut mbuf_type = 0 as *mut mbuf_type;
    let mut domain: *mut uchar = 0 as *mut uchar;
    ret = (*author).eptcpfds[fd as usize].ret;
    if ret <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    domain = ((*author).eptcpfds[fd as usize].domain).as_mut_ptr();
    id = (ret & 0xfff as libc::c_int) as ushort;
    typeoff = (ret >> 12 as libc::c_int) as ushort;
    ret = htable_find_list(
        (*(*author).s).qlist,
        domain,
        typeoff as libc::c_int,
        id as libc::c_int,
        &mut mbuf as *mut *mut mbuf_type as *mut *mut uchar,
    );
    if ret < 0 as libc::c_int {
        return ret;
    }
    type_0 = (*mbuf).qtype as ushort;
    if (*mbuf).qname as libc::c_int == Q_NS as libc::c_int {
        type_0 = A as libc::c_int as ushort;
    }
    len = make_dns_msg_for_new(
        buffer.offset(2 as libc::c_int as isize),
        (*mbuf).aid,
        (*mbuf).qing,
        (*mbuf).qlen as libc::c_int,
        type_0,
    );
    temp = htons(len as uint16_t);
    memcpy(
        buffer as *mut libc::c_void,
        &mut temp as *mut ushort as *const libc::c_void,
        ::std::mem::size_of::<ushort>() as libc::c_ulong,
    );
    (*mbuf).fd = fd;
    (*mbuf).buf = buffer;
    (*mbuf).buflen = len + 2 as libc::c_int;
    tcp_write_info(mbuf, 0 as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn query_from_auth_tcp(
    mut author: *mut author,
    mut mbuf: *mut mbuf_type,
) -> libc::c_int {
    let mut si: sockinfo = sockinfo {
        addr: sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        },
        fd: 0,
        buflen: 0,
        socktype: 0,
        buf: 0 as *mut uchar,
        lowerdomain: 0 as *mut packet_type,
        mbuf: 0 as *mut mbuf_type,
    };
    let mut i: libc::c_int = 0;
    let mut st: libc::c_int = 0 as libc::c_int;
    let mut ip: *mut uchar = ((*author).ip).as_mut_ptr();
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    mv = ip as *mut mvalue;
    while (*mv).num as libc::c_int > 0 as libc::c_int {
        ip = ip.offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
        i = 0 as libc::c_int;
        while i < (*mv).num as libc::c_int {
            if st == (*mbuf).tcpnums - 1 as libc::c_int {
                si.fd = (*mbuf).tcpfd;
                make_addr_from_bin(&mut si.addr, ip);
                si.addr.sin_port = htons(53 as libc::c_int as uint16_t);
                si.addr.sin_family = 2 as libc::c_int as sa_family_t;
                connect_to(&mut si);
                st = MOST_TRY_PER_QUERY as libc::c_int + 1 as libc::c_int;
            }
            st += 1;
            st;
            i += 1;
            i;
        }
        ip = ip.offset((*mv).len as libc::c_int as isize);
        mv = ip as *mut mvalue;
        if st > MOST_TRY_PER_QUERY as libc::c_int {
            break;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn query_from_auth_server(
    mut mbuf: *mut mbuf_type,
    mut author: *mut author,
) -> libc::c_int {
    let mut id: ushort = (*mbuf).aid;
    let mut type_0: ushort = 0;
    let mut buffer: *mut uchar = (*mbuf).tempbuffer;
    let mut ip: *mut uchar = ((*author).ip).as_mut_ptr();
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut st: libc::c_int = 1 as libc::c_int;
    let mut ret: libc::c_int = 0;
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    if (*mbuf).qname as libc::c_int == Q_NS as libc::c_int {
        type_0 = A as libc::c_int as ushort;
    } else {
        type_0 = (*mbuf).qtype as ushort;
    }
    (*mbuf).mxtry += 1;
    (*mbuf).mxtry;
    if (*mbuf).socktype == SOCK_DGRAM as libc::c_int as libc::c_uint {
        len = make_dns_msg_for_new(
            buffer,
            id,
            (*mbuf).qing,
            (*mbuf).qlen as libc::c_int,
            type_0,
        );
        (*mbuf).buf = buffer;
        (*mbuf).buflen = len;
        (*mbuf).fd = (*author).audp;
        mv = ip as *mut mvalue;
        while (*mv).num as libc::c_int > 0 as libc::c_int {
            ip = ip.offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
            i = 0 as libc::c_int;
            while i < (*mv).num as libc::c_int {
                make_addr_from_bin(
                    &mut (*mbuf).aaddr,
                    ip.offset((i * 4 as libc::c_int) as isize),
                );
                (*mbuf).aaddr.sin_port = htons(53 as libc::c_int as uint16_t);
                (*mbuf).addr = &mut (*mbuf).aaddr;
                ret = udp_write_info(mbuf, 0 as libc::c_int);
                if ret > 0 as libc::c_int {
                    st += 1;
                    st;
                }
                if st > (*mbuf).mxtry {
                    return 0 as libc::c_int;
                }
                i += 1;
                i;
            }
            ip = ip.offset((*mv).len as libc::c_int as isize);
            mv = ip as *mut mvalue;
            if st > MOST_TRY_PER_QUERY as libc::c_int {
                break;
            }
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn release_qoutinfo(
    mut author: *mut author,
    mut mbuf: *mut mbuf_type,
    mut idx: uint32_t,
) -> libc::c_int {
    let mut fd: libc::c_int = (*mbuf).tcpfd;
    let mut epfd: libc::c_int = 0;
    let mut id: libc::c_int = 0;
    let mut typeoff: libc::c_int = 0;
    let mut val: *mut uchar = 0 as *mut uchar;
    if fd > 0 as libc::c_int {
        let mut ev: epoll_event = {
            let mut init = epoll_event {
                events: 0 as libc::c_int as uint32_t,
                data: epoll_data {
                    ptr: 0 as *mut libc::c_void,
                },
            };
            init
        };
        epfd = (*author).bdepfd;
        (*author).tcpinuse -= 1;
        (*author).tcpinuse;
        epoll_ctl(epfd, 2 as libc::c_int, fd, &mut ev);
        (*author).eptcpfds[fd as usize].ret = 0 as libc::c_int;
        close(fd);
    }
    id = (idx & 0xfff as libc::c_int as libc::c_uint) as libc::c_int;
    typeoff = (idx >> 12 as libc::c_int) as libc::c_int;
    val = htable_delete_list(
        (*(*author).s).qlist,
        ((*mbuf).lowerdomain.domain).as_mut_ptr(),
        typeoff,
        id,
    );
    if val.is_null() {
        printf(
            b"del list val =0, mbuf:0x%0x\n\0" as *const u8 as *const libc::c_char,
            mbuf,
        );
        return 0 as libc::c_int;
    }
    if val == mbuf as *mut libc::c_void as *mut uchar {} else {
        __assert_fail(
            b"val == (void *)mbuf\0" as *const u8 as *const libc::c_char,
            b"author.c\0" as *const u8 as *const libc::c_char,
            446 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"int release_qoutinfo(struct author *, mbuf_type *, uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_6639: {
        if val == mbuf as *mut libc::c_void as *mut uchar {} else {
            __assert_fail(
                b"val == (void *)mbuf\0" as *const u8 as *const libc::c_char,
                b"author.c\0" as *const u8 as *const libc::c_char,
                446 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"int release_qoutinfo(struct author *, mbuf_type *, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
    mbuf_free(mbuf);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn init_qoutinfo(mut mbuf: *mut mbuf_type) -> libc::c_int {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    (*mbuf).socktype = SOCK_DGRAM as libc::c_int as uint;
    (*mbuf).mxtry = 0 as libc::c_int;
    (*mbuf).qns = 1 as libc::c_int;
    (*mbuf).sq = 1 as libc::c_int as ushort;
    (*mbuf)
        .stime = (tv.tv_sec * 1000 as libc::c_int as libc::c_long
        + tv.tv_usec / 1000 as libc::c_int as libc::c_long) as uint64_t;
    (*mbuf).tcpfd = 0 as libc::c_int;
    (*mbuf).qtimes = 0 as libc::c_int as ushort;
    (*mbuf).tdbuffer = 0 as *mut uchar;
    (*mbuf).tempbuffer = 0 as *mut uchar;
    (*mbuf).dmbuffer = 0 as *mut uchar;
    (*mbuf).ipbuffer = 0 as *mut uchar;
    (*mbuf).hascname = 0 as libc::c_int as ushort;
    (*mbuf).tcpnums = 0 as libc::c_int;
    (*mbuf).stat = NEW_QUERY as libc::c_int as ushort;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn check_enter(
    mut author: *mut author,
    mut buf: *mut uchar,
    mut idx: *mut libc::c_int,
    mut mbuf: *mut *mut mbuf_type,
    mut lowerdomain: *mut packet_type,
) -> libc::c_int {
    let mut id: int32_t = 0;
    let mut typeoff: int32_t = 0;
    let mut ret: libc::c_int = 0;
    let mut tx: libc::c_int = 0 as libc::c_int;
    let mut hdr: *mut dnsheader = buf as *mut dnsheader;
    *idx = (*hdr).id as libc::c_int;
    id = (*hdr).id as libc::c_int & 0xfff as libc::c_int;
    typeoff = (*hdr).id as libc::c_int >> 12 as libc::c_int;
    if id >= 4095 as libc::c_int || typeoff >= 9 as libc::c_int {
        return -(1 as libc::c_int);
    }
    ret = check_dns_name(
        buf.offset(::std::mem::size_of::<dnsheader>() as libc::c_ulong as isize),
        lowerdomain,
    );
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    ret = htable_find_list(
        (*(*author).s).qlist,
        ((*lowerdomain).domain).as_mut_ptr(),
        typeoff,
        id,
        mbuf as *mut *mut uchar,
    );
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (**mbuf).stat as libc::c_int == NEW_QUERY as libc::c_int {
        *mbuf = 0 as *mut mbuf_type;
        return -(1 as libc::c_int);
    }
    ret = check_an_msg((*hdr).flags, 0 as *mut uchar, &mut tx);
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if ret == 1 as libc::c_int {
        return -(2 as libc::c_int);
    }
    if ret == 2 as libc::c_int && tx == 1 as libc::c_int {
        return -(3 as libc::c_int);
    }
    (**mbuf).socktype = SOCK_DGRAM as libc::c_int as uint;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn passer_auth_data(
    mut author: *mut author,
    mut buf: *mut uchar,
    mut si: *mut sockinfo,
) -> libc::c_int {
    let mut idx: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut pret: libc::c_int = 0;
    let mut mbuf: *mut mbuf_type = 0 as *mut mbuf_type;
    let mut xtype: ushort = 0 as libc::c_int as ushort;
    let mut hdr: *mut dnsheader = buf as *mut dnsheader;
    let mut lowerdomain: packet_type = packet_type {
        label_count: 0,
        domain: [0; 256],
        label: [0 as *mut uint8_t; 64],
        label_offsets: [0; 64],
        label_len: [0; 64],
        hash: [0; 64],
    };
    ret = check_enter(author, buf, &mut idx, &mut mbuf, &mut lowerdomain);
    mbuf_free((*si).mbuf);
    (*si).mbuf = mbuf;
    if ret == -(2 as libc::c_int) {
        return -idx - 1 as libc::c_int;
    }
    if ret == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if ret == -(1 as libc::c_int) {
        return idx + 1 as libc::c_int;
    }
    (*mbuf).mxtry -= 1;
    (*mbuf).mxtry;
    if ret == -(3 as libc::c_int) {
        (*mbuf).qtimes = ((*mbuf).qtimes).wrapping_add(1);
        (*mbuf).qtimes;
        return 0 as libc::c_int;
    }
    (*si).lowerdomain = &mut lowerdomain;
    pret = passer_related_data(si, mbuf, author);
    if pret < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    (*mbuf).fd = (*(*author).s).ludp;
    (*mbuf).addr = &mut (*mbuf).caddr;
    if pret == CNAME as libc::c_int
        && (*mbuf).qtype as libc::c_uint == CNAME as libc::c_int as libc::c_uint
    {
        if (*mbuf).fd != -(1 as libc::c_int) {
            *(buf as *mut ushort) = (*mbuf).cid;
            (*mbuf).buf = buf;
            (*mbuf).buflen = (*si).buflen;
            if (*si).buflen > 512 as libc::c_int {
                send_tc_to_client(mbuf);
            } else {
                udp_write_info(mbuf, 0 as libc::c_int);
                write_log(
                    (*author).loginfo,
                    (*author).idx,
                    (*mbuf).td,
                    (*mbuf).dlen,
                    (*mbuf).qtype as libc::c_int,
                    (*mbuf).addr,
                );
            }
        }
        return idx + 1 as libc::c_int;
    }
    if pret == CNAME as libc::c_int
        || (*mbuf).qname as libc::c_int != Q_DOMAIN as libc::c_int
    {
        (*mbuf).stat = PROCESS_QUERY as libc::c_int as ushort;
        (*mbuf).socktype = SOCK_DGRAM as libc::c_int as uint;
        return 0 as libc::c_int;
    }
    if pret == SOA as libc::c_int
        || ntohs((*hdr).ancount) as libc::c_int > 0 as libc::c_int
    {
        if (*mbuf).fd != -(1 as libc::c_int) {
            if (*mbuf).hascname as libc::c_int == 0 as libc::c_int {
                *(buf as *mut ushort) = (*mbuf).cid;
                (*mbuf).buf = buf;
                (*mbuf).buflen = (*si).buflen;
                if (*si).buflen > 512 as libc::c_int {
                    send_tc_to_client(mbuf);
                } else {
                    udp_write_info(mbuf, 0 as libc::c_int);
                    write_log(
                        (*author).loginfo,
                        (*author).idx,
                        (*mbuf).td,
                        (*mbuf).dlen,
                        (*mbuf).qtype as libc::c_int,
                        (*mbuf).addr,
                    );
                }
            } else {
                if pret == SOA as libc::c_int {
                    xtype = CNAME as libc::c_int as ushort;
                } else {
                    xtype = (*mbuf).qtype as ushort;
                }
                ret = find_record_from_mem(
                    (*mbuf).td,
                    (*mbuf).dlen,
                    xtype as libc::c_int,
                    (*(*author).s).datasets,
                    ((*author).tmpbuffer).as_mut_ptr(),
                    ((*author).databuffer).as_mut_ptr(),
                    &mut *((*mbuf).lowerdomain.hash)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize),
                );
                if ret > 0 as libc::c_int {
                    (*author).response += 1;
                    (*author).response;
                    if (*mbuf).fd != -(1 as libc::c_int) {
                        (*mbuf)
                            .buf = ((*mbuf).data)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as isize);
                        write_back_to_client(
                            mbuf,
                            ((*author).databuffer).as_mut_ptr(),
                            ret,
                        );
                    }
                    write_log(
                        (*author).loginfo,
                        (*author).idx,
                        (*mbuf).td,
                        (*mbuf).dlen,
                        (*mbuf).qtype as libc::c_int,
                        (*mbuf).addr,
                    );
                }
            }
        }
        return idx + 1 as libc::c_int;
    }
    (*mbuf).stat = PROCESS_QUERY as libc::c_int as ushort;
    (*mbuf).socktype = SOCK_DGRAM as libc::c_int as uint;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn cb_read_auth(
    mut ev: *mut epoll_event,
    mut si: *mut sockinfo,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut szhdr: libc::c_int = ::std::mem::size_of::<dnsheader>() as libc::c_ulong
        as libc::c_int;
    let mut mbuf: *mut mbuf_type = mbuf_alloc();
    if mbuf.is_null() {
        return -(1 as libc::c_int);
    }
    memset(
        mbuf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mbuf_type>() as libc::c_ulong,
    );
    (*mbuf).fd = (*ev).data.fd;
    (*mbuf).buf = (*si).buf;
    (*mbuf).buflen = BIG_MEM_STEP as libc::c_int;
    (*mbuf).addr = &mut (*mbuf).aaddr;
    if (*si).socktype == SOCK_STREAM as libc::c_int {
        ret = tcp_read_dns_msg(
            mbuf,
            (4096 as libc::c_int - 2 as libc::c_int) as uint,
            0 as libc::c_int,
        );
    } else {
        ret = udp_read_msg(mbuf, 0 as libc::c_int);
    }
    if ret < szhdr {
        mbuf_free(mbuf);
        return -(1 as libc::c_int);
    }
    (*mbuf).buflen = ret;
    (*si).buflen = (*mbuf).buflen;
    (*si).mbuf = mbuf;
    return ret;
}
pub unsafe extern "C" fn launch_new_query(mut author: *mut author) -> libc::c_int {
    let mut new_query: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut mbuf: *mut mbuf_type = 0 as *mut mbuf_type;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut msnow: uint64_t = 0 as libc::c_int as uint64_t;
    let mut slotoff: libc::c_int = 0;
    let mut typeoff: libc::c_int = 0;
    start = (*author).start;
    end = (*author).end;
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    msnow = (tv.tv_sec * 1000 as libc::c_int as libc::c_long
        + tv.tv_usec / 1000 as libc::c_int as libc::c_long) as uint64_t;
    i = start;
    while i < end {
        slotoff = 0 as libc::c_int;
        typeoff = 0 as libc::c_int;
        mbuf = 0 as *mut mbuf_type;
        ret = htable_find_list_io(
            (*(*author).s).qlist,
            i,
            slotoff,
            &mut typeoff,
            &mut mbuf as *mut *mut mbuf_type as *mut *mut uchar,
        );
        while ret >= 0 as libc::c_int {
            if ret > 0 as libc::c_int {
                if (*mbuf).qtimes as libc::c_int > MAX_TRY_TIMES as libc::c_int
                    || msnow.wrapping_sub((*mbuf).stime)
                        > 5000 as libc::c_int as libc::c_ulong
                {
                    release_qoutinfo(
                        author,
                        mbuf,
                        (i | typeoff << 12 as libc::c_int) as uint32_t,
                    );
                } else {
                    if (*mbuf).stat as libc::c_int == NEW_QUERY as libc::c_int {
                        if i < 4095 as libc::c_int && typeoff < 9 as libc::c_int
                        {} else {
                            __assert_fail(
                                b"i < QLIST_TABLE_SIZE && typeoff < SUPPORT_TYPE_NUM\0"
                                    as *const u8 as *const libc::c_char,
                                b"author.c\0" as *const u8 as *const libc::c_char,
                                684 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 38],
                                    &[libc::c_char; 38],
                                >(b"int launch_new_query(struct author *)\0"))
                                    .as_ptr(),
                            );
                        }
                        'c_10299: {
                            if i < 4095 as libc::c_int && typeoff < 9 as libc::c_int
                            {} else {
                                __assert_fail(
                                    b"i < QLIST_TABLE_SIZE && typeoff < SUPPORT_TYPE_NUM\0"
                                        as *const u8 as *const libc::c_char,
                                    b"author.c\0" as *const u8 as *const libc::c_char,
                                    684 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 38],
                                        &[libc::c_char; 38],
                                    >(b"int launch_new_query(struct author *)\0"))
                                        .as_ptr(),
                                );
                            }
                        };
                        (*mbuf).aid = (i | typeoff << 12 as libc::c_int) as ushort;
                        (*mbuf).backid = (*mbuf).aid;
                        (*mbuf).mxtry = 0 as libc::c_int;
                        if (*mbuf).fd != -(1 as libc::c_int) {
                            (*mbuf).fd = (*author).cudp;
                        }
                        (*mbuf).tdbuffer = ((*author).tdbuffer).as_mut_ptr();
                        (*mbuf).tempbuffer = ((*author).tempbuffer).as_mut_ptr();
                        (*mbuf).dmbuffer = ((*author).dmbuffer).as_mut_ptr();
                        (*mbuf).ipbuffer = ((*author).ipbuffer).as_mut_ptr();
                        new_query += 1;
                        new_query;
                        (*mbuf).stat = PROCESS_QUERY as libc::c_int as ushort;
                    }
                    if msnow.wrapping_sub((*mbuf).stime)
                        > 1000 as libc::c_int as libc::c_ulong
                        && (*mbuf).sq as libc::c_int == 0 as libc::c_int
                    {
                        (*mbuf).sq = 1 as libc::c_int as ushort;
                    }
                    if (*mbuf).socktype == SOCK_DGRAM as libc::c_int as libc::c_uint
                        && (*mbuf).sq as libc::c_int == 1 as libc::c_int
                    {
                        ret = find_addr(
                            (*(*author).s).forward,
                            (*(*author).s).datasets,
                            mbuf,
                            ((*author).ip).as_mut_ptr(),
                            (*(*author).s).is_forward,
                        );
                        if (*mbuf).stat as libc::c_int == PROCESS_QUERY as libc::c_int
                            && ret == 0 as libc::c_int
                        {
                            query_from_auth_server(mbuf, author);
                        }
                        (*mbuf).qtimes = ((*mbuf).qtimes).wrapping_add(1);
                        (*mbuf).qtimes;
                    }
                }
            }
            if ret == 0 as libc::c_int || typeoff == 9 as libc::c_int - 1 as libc::c_int
            {
                slotoff += 1;
                slotoff;
                typeoff = 0 as libc::c_int;
            } else {
                typeoff += 1;
                typeoff;
            }
            mbuf = 0 as *mut mbuf_type;
            ret = htable_find_list_io(
                (*(*author).s).qlist,
                i,
                slotoff,
                &mut typeoff,
                &mut mbuf as *mut *mut mbuf_type as *mut *mut uchar,
            );
        }
        i += 1;
        i;
    }
    return new_query;
}
pub unsafe extern "C" fn after_pass_data(
    mut ret: libc::c_int,
    mut author: *mut author,
    mut mbuf: *mut mbuf_type,
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
    let mut fd: libc::c_int = 0;
    if ret == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if mbuf.is_null() {
        return -(1 as libc::c_int);
    }
    if ret < 0 as libc::c_int {
        ret = ret + 1 as libc::c_int;
        ret = -ret;
        if (*mbuf).tcpfd > 0 as libc::c_int
            && (*mbuf).qtimes as libc::c_int
                % (MAX_TRY_TIMES as libc::c_int / 3 as libc::c_int) == 0 as libc::c_int
        {
            ev.data.fd = (*mbuf).tcpfd;
            (*mbuf).tcpfd = 0 as libc::c_int;
            (*author).tcpinuse -= 1;
            (*author).tcpinuse;
            epoll_ctl((*author).bdepfd, 2 as libc::c_int, ev.data.fd, &mut ev);
            close(ev.data.fd);
        }
        if (*mbuf).tcpfd > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        if (*author).tcpinuse > LIST_SPACE as libc::c_int / 10 as libc::c_int {
            fd = -(1 as libc::c_int);
        } else {
            (*mbuf).tcpnums += 1;
            (*mbuf).tcpnums;
            fd = socket(2 as libc::c_int, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
        }
        if fd > 0 as libc::c_int {
            (*author).tcpinuse += 1;
            (*author).tcpinuse;
            (*mbuf).tcpfd = fd;
            (*mbuf).socktype = SOCK_STREAM as libc::c_int as uint;
            ev.data.fd = fd;
            ev.events = EPOLLOUT as libc::c_int as uint32_t;
            (*author).eptcpfds[fd as usize].ret = ret;
            memcpy(
                ((*author).eptcpfds[fd as usize].domain).as_mut_ptr()
                    as *mut libc::c_void,
                (*mbuf).td as *const libc::c_void,
                (*mbuf).dlen as libc::c_ulong,
            );
            set_non_block(fd);
            set_recv_timeout(fd, 0 as libc::c_int, 500 as libc::c_int);
            epoll_ctl((*author).bdepfd, 1 as libc::c_int, fd, &mut ev);
            query_from_auth_tcp(author, mbuf);
            return 0 as libc::c_int;
        } else {
            ret += 1;
            ret;
        }
    }
    if ret > 0 as libc::c_int {
        ret = ret - 1 as libc::c_int;
        release_qoutinfo(author, mbuf, ret as uint32_t);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn handle_back_event(mut author: *mut author) -> libc::c_int {
    let mut infinite: libc::c_int = 1 as libc::c_int;
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut epfd: libc::c_int = (*author).bdepfd;
    let mut si: sockinfo = {
        let mut init = sockinfo {
            addr: {
                let mut init = sockaddr_in {
                    sin_family: 0 as libc::c_int as sa_family_t,
                    sin_port: 0,
                    sin_addr: in_addr { s_addr: 0 },
                    sin_zero: [0; 8],
                };
                init
            },
            fd: 0,
            buflen: 0,
            socktype: 0,
            buf: 0 as *mut uchar,
            lowerdomain: 0 as *mut packet_type,
            mbuf: 0 as *mut mbuf_type,
        };
        init
    };
    let mut bf: libc::c_int = 0 as libc::c_int;
    let mut rx: libc::c_int = 0;
    let mut ev: epoll_event = {
        let mut init = epoll_event {
            events: 0 as libc::c_int as uint32_t,
            data: epoll_data {
                ptr: 0 as *mut libc::c_void,
            },
        };
        init
    };
    let mut e: *mut epoll_event = ((*author).e).as_mut_ptr();
    let mut buf: *mut uchar = ((*author).tmpbuffer).as_mut_ptr();
    while 1 as libc::c_int != 0 && infinite != 0 {
        bf = (*author).audp;
        ret = epoll_wait(epfd, e, 1000 as libc::c_int, 500 as libc::c_int);
        if ret <= 0 as libc::c_int {
            break;
        }
        i = 0 as libc::c_int;
        while i < ret {
            memset(
                &mut si as *mut sockinfo as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<sockinfo>() as libc::c_ulong,
            );
            si.buf = buf;
            if (*e.offset(i as isize)).data.fd == bf {
                si.socktype = SOCK_DGRAM as libc::c_int;
                while cb_read_auth(e.offset(i as isize), &mut si) > 0 as libc::c_int {
                    rx = passer_auth_data(author, buf, &mut si);
                    after_pass_data(rx, author, si.mbuf);
                }
            } else if (*e.offset(i as isize)).data.fd > 0 as libc::c_int {
                if (*e.offset(i as isize)).events
                    == EPOLLOUT as libc::c_int as libc::c_uint
                {
                    rx = send_msg_tcp(author, (*e.offset(i as isize)).data.fd);
                    if rx < 0 as libc::c_int {
                        printf(
                            b"send msg tcp error\n\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    ev.data.fd = (*e.offset(i as isize)).data.fd;
                    ev.events = EPOLLIN as libc::c_int as uint32_t;
                    epoll_ctl(
                        epfd,
                        3 as libc::c_int,
                        (*e.offset(i as isize)).data.fd,
                        &mut ev,
                    );
                } else if (*e.offset(i as isize)).events
                    == EPOLLIN as libc::c_int as libc::c_uint
                {
                    si.socktype = SOCK_STREAM as libc::c_int;
                    rx = cb_read_auth(e.offset(i as isize), &mut si);
                    if rx < 0 as libc::c_int {
                        (*author)
                            .eptcpfds[(*e.offset(i as isize)).data.fd as usize]
                            .ret = 0 as libc::c_int;
                        close((*e.offset(i as isize)).data.fd);
                        ev.data.fd = (*e.offset(i as isize)).data.fd;
                        epoll_ctl(epfd, 2 as libc::c_int, ev.data.fd, &mut ev);
                    } else {
                        rx = passer_auth_data(author, buf, &mut si);
                        after_pass_data(rx, author, si.mbuf);
                    }
                } else {
                    ev.data.fd = (*e.offset(i as isize)).data.fd;
                    rx = epoll_ctl(
                        epfd,
                        2 as libc::c_int,
                        (*e.offset(i as isize)).data.fd,
                        &mut ev,
                    );
                    (*author)
                        .eptcpfds[(*e.offset(i as isize)).data.fd as usize]
                        .ret = 0 as libc::c_int;
                    close((*e.offset(i as isize)).data.fd);
                }
            }
            i += 1;
            i;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dup_data_into_db(mut a: *mut author) -> libc::c_int {
    let mut i: uint = 0;
    let mut limit: uint = 0;
    let mut rbt: *mut rbtree = (*(*a).s).ttlexp;
    let mut dboff: uint = 0;
    let mut dbidx: uint = 0;
    if (*a).dupbefore == 1 as libc::c_int {
        (*a).limits += 5 as libc::c_int;
        if (*a).limits > 1000 as libc::c_int {
            (*a).limits = 1000 as libc::c_int;
        }
    }
    limit = (*a).limits as uint;
    (*a).hsidx += 1;
    (*a).hsidx;
    if (*a).hsidx == 10 as libc::c_int {
        (*a).hsidx = 0 as libc::c_int;
    }
    i = 0 as libc::c_int as uint;
    while i < 65536 as libc::c_int as libc::c_uint {
        dbidx = i;
        dboff = (*a).hsidx as uint;
        htable_find_io(
            ((*(*a).s).datasets).offset(dboff as isize),
            dbidx as libc::c_int,
            limit,
            rbt,
            TTL_UPDATE as libc::c_int,
        );
        i = i.wrapping_add(1);
        i;
    }
    (*a).dupbefore = 1 as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn check_mm_cache(mut author: *mut author) -> libc::c_int {
    let mut total: uint = 0 as libc::c_int as uint;
    let mut i: libc::c_int = 0;
    static mut tmx: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        pthread_spin_lock(&mut (*((*(*author).s).datasets).offset(i as isize)).lock);
        total = (total as libc::c_uint)
            .wrapping_add((*((*(*author).s).datasets).offset(i as isize)).now) as uint
            as uint;
        pthread_spin_unlock(&mut (*((*(*author).s).datasets).offset(i as isize)).lock);
        i += 1;
        i;
    }
    tmx += 1;
    tmx;
    if total > MAX_ELE_NUM {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn check_ttl_expire(mut author: *mut author) -> libc::c_int {
    let mut now: time_t = 0;
    let mut tn: *mut ttlnode = 0 as *mut ttlnode;
    let mut pn: *mut rbnode = 0 as *mut rbnode;
    let mut mbuf: *mut mbuf_type = 0 as *mut mbuf_type;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut rbt: *mut rbtree = 0 as *mut rbtree;
    mbuf = mbuf_alloc();
    if mbuf.is_null() {
        return -(1 as libc::c_int);
    }
    now = global_now;
    rbt = (*(*author).s).ttlexp;
    pthread_spin_lock(&mut (*rbt).lock);
    pn = min_node(rbt);
    while !pn.is_null() {
        tn = (*pn).key as *mut ttlnode;
        if (*tn).exp as libc::c_long > now + TTL_UPDATE as libc::c_int as libc::c_long {
            break;
        }
        tn = delete_node(rbt, pn) as *mut ttlnode;
        pthread_spin_unlock(&mut (*rbt).lock);
        if !tn.is_null() {
            memset(
                mbuf as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<mbuf_type>() as libc::c_ulong,
            );
            (*mbuf).qname = (*tn).type_0;
            (*mbuf).qtype = (*tn).type_0 as rrtype;
            (*mbuf).dlen = (*tn).dlen as libc::c_int;
            memcpy(
                &mut (*mbuf).lowerdomain as *mut packet_type as *mut libc::c_void,
                (*tn).lowerdomain as *const libc::c_void,
                ::std::mem::size_of::<packet_type>() as libc::c_ulong,
            );
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < (*(*tn).lowerdomain).label_count as libc::c_int {
                (*mbuf)
                    .lowerdomain
                    .label[i
                    as usize] = ((*mbuf).lowerdomain.domain)
                    .as_mut_ptr()
                    .offset(
                        (*mbuf).lowerdomain.label_offsets[i as usize] as libc::c_int
                            as isize,
                    );
                i += 1;
                i;
            }
            (*mbuf)
                .qhash = &mut *((*mbuf).lowerdomain.hash)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut hashval_t;
            (*mbuf).td = ((*mbuf).lowerdomain.domain).as_mut_ptr();
            (*mbuf).qing = (*mbuf).td;
            (*mbuf).qlen = (*mbuf).dlen as ushort;
            (*mbuf).cid = 0 as libc::c_int as ushort;
            (*mbuf).fd = -(1 as libc::c_int);
            init_qoutinfo(mbuf);
            ret = htable_insert_list(
                (*(*author).s).qlist,
                (*tn).data,
                (*tn).dlen as libc::c_int,
                (*tn).type_0 as libc::c_int,
                mbuf as *mut uchar,
                0 as libc::c_int,
                0 as *mut mvalue,
                (*tn).hash,
            );
            if HTABLE_INSERT_RET_NORMAL as libc::c_int == ret {
                mbuf = mbuf_alloc();
                if mbuf.is_null() {
                    free((*tn).lowerdomain as *mut libc::c_void);
                    free(tn as *mut libc::c_void);
                    return -(1 as libc::c_int);
                }
            }
            free((*tn).lowerdomain as *mut libc::c_void);
            free(tn as *mut libc::c_void);
        }
        pthread_spin_lock(&mut (*rbt).lock);
        pn = min_node(rbt);
    }
    mbuf_free(mbuf);
    pthread_spin_unlock(&mut (*rbt).lock);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn check_refresh_flag(mut author: *mut author) -> libc::c_int {
    let mut s: *mut server = (*author).s;
    if (*s).lastrefresh + REFRESH_INTERVAL as libc::c_int as libc::c_long > global_now {
        return 0 as libc::c_int;
    }
    if (*s).refreshflag as libc::c_int == 1 as libc::c_int {
        (*s).refreshflag = 0 as libc::c_int as uint16_t;
        (*s).lastrefresh = global_now;
        refresh_records((*s).datasets, (*s).ttlexp);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn run_quizzer(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut author: *mut author = arg as *mut author;
    let mut epfd: libc::c_int = 0;
    pthread_detach(pthread_self());
    epfd = add_backdoor((*author).audp);
    (*author).bdepfd = epfd;
    loop {
        launch_new_query(author);
        handle_back_event(author);
        if (*author).idx == 0 as libc::c_int {
            check_ttl_expire(author);
            if check_mm_cache(author) == 1 as libc::c_int {
                dup_data_into_db(author);
            } else {
                (*author).dupbefore = 0 as libc::c_int;
            }
            check_refresh_flag(author);
        }
    };
}
pub unsafe extern "C" fn add_to_quizzer(
    mut qo: *mut qoutinfo,
    mut s: *mut server,
    mut qidx: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut randomoff: libc::c_int = 0 as libc::c_int;
    let mut qi: *mut qoutinfo = qo;
    (*qi).stat = NEW_QUERY as libc::c_int as ushort;
    randomoff = (random() % LIST_SPACE as libc::c_int as libc::c_long) as libc::c_int;
    j = qidx;
    while j < QUIZZER_NUM as libc::c_int {
        i = randomoff;
        while i < LIST_SPACE as libc::c_int {
            if ((*((*s).authors).offset(j as isize)).list[i as usize]).is_null() {
                pthread_spin_lock(&mut (*((*s).authors).offset(j as isize)).lock);
                if !((*((*s).authors).offset(j as isize)).list[i as usize]).is_null() {
                    pthread_spin_unlock(&mut (*((*s).authors).offset(j as isize)).lock);
                } else {
                    let ref mut fresh0 = (*((*s).authors).offset(j as isize))
                        .list[i as usize];
                    *fresh0 = qi;
                    let ref mut fresh1 = (*((*s).authors).offset(j as isize)).qnum;
                    *fresh1 += 1;
                    *fresh1;
                    pthread_spin_unlock(&mut (*((*s).authors).offset(j as isize)).lock);
                    return 0 as libc::c_int;
                }
            }
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < randomoff {
            if ((*((*s).authors).offset(j as isize)).list[i as usize]).is_null() {
                pthread_spin_lock(&mut (*((*s).authors).offset(j as isize)).lock);
                if !((*((*s).authors).offset(j as isize)).list[i as usize]).is_null() {
                    pthread_spin_unlock(&mut (*((*s).authors).offset(j as isize)).lock);
                } else {
                    let ref mut fresh2 = (*((*s).authors).offset(j as isize))
                        .list[i as usize];
                    *fresh2 = qi;
                    let ref mut fresh3 = (*((*s).authors).offset(j as isize)).qnum;
                    *fresh3 += 1;
                    *fresh3;
                    pthread_spin_unlock(&mut (*((*s).authors).offset(j as isize)).lock);
                    return 0 as libc::c_int;
                }
            }
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    j = 0 as libc::c_int;
    while j < qidx {
        i = randomoff;
        while i < LIST_SPACE as libc::c_int {
            if ((*((*s).authors).offset(j as isize)).list[i as usize]).is_null() {
                pthread_spin_lock(&mut (*((*s).authors).offset(j as isize)).lock);
                if !((*((*s).authors).offset(j as isize)).list[i as usize]).is_null() {
                    pthread_spin_unlock(&mut (*((*s).authors).offset(j as isize)).lock);
                } else {
                    let ref mut fresh4 = (*((*s).authors).offset(j as isize))
                        .list[i as usize];
                    *fresh4 = qi;
                    let ref mut fresh5 = (*((*s).authors).offset(j as isize)).qnum;
                    *fresh5 += 1;
                    *fresh5;
                    pthread_spin_unlock(&mut (*((*s).authors).offset(j as isize)).lock);
                    return 0 as libc::c_int;
                }
            }
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < randomoff {
            if ((*((*s).authors).offset(j as isize)).list[i as usize]).is_null() {
                pthread_spin_lock(&mut (*((*s).authors).offset(j as isize)).lock);
                if !((*((*s).authors).offset(j as isize)).list[i as usize]).is_null() {
                    pthread_spin_unlock(&mut (*((*s).authors).offset(j as isize)).lock);
                } else {
                    let ref mut fresh6 = (*((*s).authors).offset(j as isize))
                        .list[i as usize];
                    *fresh6 = qi;
                    let ref mut fresh7 = (*((*s).authors).offset(j as isize)).qnum;
                    *fresh7 += 1;
                    *fresh7;
                    pthread_spin_unlock(&mut (*((*s).authors).offset(j as isize)).lock);
                    return 0 as libc::c_int;
                }
            }
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn lock_and_add_to_quizz(
    mut mbuf: *mut mbuf_type,
    mut f: *mut fetcher,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if (*mbuf).dlen < 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*mbuf).qname = (*mbuf).qtype as ushort;
    (*mbuf).td = ((*mbuf).lowerdomain.domain).as_mut_ptr();
    (*mbuf).qing = (*mbuf).td;
    (*mbuf)
        .qhash = &mut *((*mbuf).lowerdomain.hash)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut hashval_t;
    (*mbuf).qlen = (*mbuf).dlen as ushort;
    (*mbuf).cid = (*mbuf).id;
    init_qoutinfo(mbuf);
    ret = htable_insert_list(
        (*(*f).s).qlist,
        ((*mbuf).lowerdomain.domain).as_mut_ptr(),
        (*mbuf).dlen,
        (*mbuf).qtype as libc::c_int,
        mbuf as *mut uchar,
        0 as libc::c_int,
        0 as *mut mvalue,
        &mut *((*mbuf).lowerdomain.hash).as_mut_ptr().offset(0 as libc::c_int as isize),
    );
    if ret != HTABLE_INSERT_RET_NORMAL as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn find_record_from_mem(
    mut otd: *mut uchar,
    mut dlen: libc::c_int,
    mut type_0: libc::c_int,
    mut datasets: *mut htable,
    mut tdbuffer: *mut uchar,
    mut databuffer: *mut uchar,
    mut hash: *mut hashval_t,
) -> libc::c_int {
    let mut td: *mut uchar = otd;
    let mut ret: libc::c_int = 0;
    let mut dataidx: libc::c_int = 0 as libc::c_int;
    let mut clen: libc::c_int = 0;
    let mut debug: libc::c_int = 100 as libc::c_int;
    let mut thash: hashval_t = 0;
    let mut h: *mut hashval_t = hash;
    dataidx += 1;
    dataidx;
    if type_0 != CNAME as libc::c_int {
        loop {
            ret = find_record_with_ttl(
                datasets,
                td,
                dlen,
                CNAME as libc::c_int,
                databuffer.offset(dataidx as isize),
                AUTH_DATA_LEN as libc::c_int - dataidx,
                0 as *mut mvalue,
                h,
            );
            if !(ret > 0 as libc::c_int) {
                break;
            }
            *databuffer
                .offset(
                    (dataidx - 1 as libc::c_int) as isize,
                ) = CNAME as libc::c_int as uchar;
            clen = (ret as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<mvalue>() as libc::c_ulong)
                as libc::c_int;
            td = tdbuffer;
            memcpy(
                td as *mut libc::c_void,
                databuffer
                    .offset(dataidx as isize)
                    .offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize)
                    as *const libc::c_void,
                clen as libc::c_ulong,
            );
            dataidx += ret;
            dataidx += 1;
            dataidx;
            let fresh8 = debug;
            debug = debug - 1;
            if fresh8 == 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            thash = 0 as libc::c_int as hashval_t;
            h = &mut thash;
            dlen = clen;
        }
        thash = 0 as libc::c_int as hashval_t;
    }
    ret = find_record_with_ttl(
        datasets,
        td,
        dlen,
        type_0,
        databuffer.offset(dataidx as isize),
        AUTH_DATA_LEN as libc::c_int - dataidx,
        0 as *mut mvalue,
        h,
    );
    if ret > 0 as libc::c_int {
        *databuffer.offset((dataidx - 1 as libc::c_int) as isize) = type_0 as uchar;
        dataidx += ret;
        return dataidx;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn global_cron(mut s: *mut server) -> libc::c_int {
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut nds: *mut list_node = 0 as *mut list_node;
    let mut tmp: *mut list_node = 0 as *mut list_node;
    let mut el: *mut list = &mut (*s).eventlist;
    pthread_spin_lock(&mut (*el).lock);
    nds = (*el).head;
    (*el).head = 0 as *mut list_node;
    pthread_spin_unlock(&mut (*el).lock);
    while !nds.is_null() {
        fd = *((*nds).data as *mut libc::c_int);
        if fd > 0 as libc::c_int {
            close(fd);
        }
        tmp = (*nds).next;
        free((*nds).data);
        free(nds as *mut libc::c_void);
        nds = tmp;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn run_fetcher(mut f: *mut fetcher) -> libc::c_int {
    let mut mc: *mut msgcache = (*f).mc;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut mbuf: *mut mbuf_type = 0 as *mut mbuf_type;
    let mut fd: libc::c_int = 0;
    loop {
        fd = -(1 as libc::c_int);
        pthread_spin_lock(&mut (*mc).lock);
        if (*mc).pkt == 0 as libc::c_int as libc::c_uint {
            pthread_spin_unlock(&mut (*mc).lock);
            usleep(1000 as libc::c_int as __useconds_t);
        } else {
            memcpy(
                &mut mbuf as *mut *mut mbuf_type as *mut libc::c_void,
                ((*mc).data).as_mut_ptr().offset((*mc).head as isize)
                    as *const libc::c_void,
                ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
            );
            (*mc)
                .head = ((*mc).head)
                .wrapping_add(
                    ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                );
            if ((*mc).head).wrapping_add(8 as libc::c_int as libc::c_ulong)
                > (*mc).size as libc::c_ulong
            {
                (*mc).head = 0 as libc::c_int as uint64_t;
            }
            (*mc).pkt = ((*mc).pkt).wrapping_sub(1);
            (*mc).pkt;
            pthread_spin_unlock(&mut (*mc).lock);
            if (*mbuf).socktype == SOCK_DGRAM as libc::c_int as libc::c_uint {
                (*mbuf).fd = (*(*f).s).ludp;
            }
            passer_dns_data(mbuf);
            if (*mbuf).err == 1 as libc::c_int {
                mbuf_free(mbuf);
            } else {
                (*f).dataidx = 0 as libc::c_int;
                (*mbuf).td = ((*mbuf).lowerdomain.domain).as_mut_ptr();
                ret = find_record_from_mem(
                    (*mbuf).td,
                    (*mbuf).dlen,
                    (*mbuf).qtype as libc::c_int,
                    (*(*f).s).datasets,
                    ((*f).tdbuffer).as_mut_ptr(),
                    ((*f).databuffer).as_mut_ptr(),
                    &mut *((*mbuf).lowerdomain.hash)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize),
                );
                if ret > 0 as libc::c_int {
                    write_back_to_client(mbuf, ((*f).databuffer).as_mut_ptr(), ret);
                    write_log(
                        (*f).loginfo,
                        (*f).idx,
                        (*mbuf).td,
                        (*mbuf).dlen - 1 as libc::c_int,
                        (*mbuf).qtype as libc::c_int,
                        (*mbuf).addr,
                    );
                    mbuf_free(mbuf);
                } else {
                    if (*mbuf).socktype == SOCK_STREAM as libc::c_int as libc::c_uint {
                        fd = (*mbuf).fd;
                        (*mbuf).fd = -(1 as libc::c_int);
                    }
                    if lock_and_add_to_quizz(mbuf, f) < 0 as libc::c_int {
                        (*f).miss = ((*f).miss).wrapping_add(1);
                        (*f).miss;
                        mbuf_free(mbuf);
                    }
                }
                if fd != -(1 as libc::c_int) {
                    delete_close_event(fd, f);
                }
            }
        }
    };
}
