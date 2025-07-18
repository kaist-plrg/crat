use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    static mut optarg: *mut libc::c_char;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn daemon(__nochdir: libc::c_int, __noclose: libc::c_int) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_setaffinity_np(
        __th: pthread_t,
        __cpusetsize: size_t,
        __cpuset: *const cpu_set_t,
    ) -> libc::c_int;
    fn pthread_spin_init(
        __lock: *mut pthread_spinlock_t,
        __pshared: libc::c_int,
    ) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigtimedwait(
        __set: *const sigset_t,
        __info: *mut siginfo_t,
        __timeout: *const timespec,
    ) -> libc::c_int;
    fn trig_signals(_: libc::c_int) -> libc::c_int;
    fn drop_privilege(_: *mut libc::c_char);
    fn dict_comp_str_equ(a: *mut libc::c_void, b: *mut libc::c_void) -> libc::c_int;
    fn rbt_comp_ttl_gt(
        v1: *mut libc::c_void,
        v2: *mut libc::c_void,
        argv: *mut libc::c_void,
    ) -> libc::c_int;
    fn dns_error(_: libc::c_int, _: *mut libc::c_char);
    fn get_random_data(_: *mut uchar, _: libc::c_int) -> libc::c_int;
    fn create_rbtree(c: Option::<comprbt>, argv: *mut libc::c_void) -> *mut rbtree;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn mempool_create(num: uint32_t) -> libc::c_int;
    fn init_msgcache(n: libc::c_int) -> *mut msgcache;
    static MAX_ELE_NUM: uint;
    fn htable_create(
        h: Option::<hashfunc>,
        c: Option::<comparefunc>,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut htable;
    fn create_socket(_: libc::c_int, _: libc::c_int, _: *mut uchar) -> libc::c_int;
    fn set_non_block(fd: libc::c_int) -> libc::c_int;
    fn set_sock_buff(fd: libc::c_int, m: libc::c_int) -> libc::c_int;
    fn read_config(
        _: *const libc::c_char,
        _: *mut libc::c_char,
        _: *mut htable,
        _: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn create_new_log(
        prefix: *mut uchar,
        idx: libc::c_int,
        type_0: libc::c_int,
    ) -> libc::c_int;
    fn read_root(_: *mut htable, _: *mut rbtree) -> libc::c_int;
    fn shmget(__key: key_t, __size: size_t, __shmflg: libc::c_int) -> libc::c_int;
    fn shmat(
        __shmid: libc::c_int,
        __shmaddr: *const libc::c_void,
        __shmflg: libc::c_int,
    ) -> *mut libc::c_void;
    fn run_quizzer(_: *mut libc::c_void) -> *mut libc::c_void;
    fn run_fetcher(f: *mut fetcher) -> libc::c_int;
    fn run_sentinel(s: *mut server) -> libc::c_int;
    fn start_local_server(s: *mut server) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __key_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type key_t = __key_t;
pub type time_t = __time_t;
pub type ulong = libc::c_ulong;
pub type ushort = libc::c_ushort;
pub type uint = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type pthread_spinlock_t = libc::c_int;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type __cpu_mask = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16],
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
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
    pub c2rust_unnamed: C2RustUnnamed_9,
    pub next: *mut hentry,
    pub count: libc::c_int,
    pub key: [uchar; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
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
pub type C2RustUnnamed_10 = libc::c_uint;
pub const IP_DATA_LEN: C2RustUnnamed_10 = 2000;
pub const MAX_TRY_TIMES: C2RustUnnamed_10 = 15;
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
pub type C2RustUnnamed_11 = libc::c_uint;
pub const SERVER_PORT: C2RustUnnamed_11 = 53;
pub const FETCHER_NUM: C2RustUnnamed_11 = 2;
pub type C2RustUnnamed_12 = libc::c_uint;
pub const QUIZZER_NUM: C2RustUnnamed_12 = 2;
pub type C2RustUnnamed_13 = libc::c_uint;
pub const SHM_KEY: C2RustUnnamed_13 = 38899;
pub const TTL_UPDATE: C2RustUnnamed_13 = 3;
pub const PROCESS_QUERY: C2RustUnnamed_13 = 1;
pub const NEW_QUERY: C2RustUnnamed_13 = 0;
pub type C2RustUnnamed_14 = libc::c_uint;
pub const EP_TCP_FDS: C2RustUnnamed_14 = 65530;
pub const AUTH_DATA_LEN: C2RustUnnamed_14 = 65528;
pub const ID_SPACE: C2RustUnnamed_14 = 60000;
pub const RANDOM_SIZE: C2RustUnnamed_14 = 3000;
pub const BIG_MEM_STEP: C2RustUnnamed_14 = 2000;
pub const AUTH_DB_NUM: C2RustUnnamed_14 = 101;
pub const REFRESH_INTERVAL: C2RustUnnamed_14 = 10;
pub type C2RustUnnamed_15 = libc::c_uint;
pub const LIST_SPACE: C2RustUnnamed_15 = 10000;
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
pub static mut global_serv: *mut server = 0 as *const server as *mut server;
pub static mut g_nameservers: [*mut libc::c_char; 2] = [0 as *const libc::c_char
    as *mut libc::c_char; 2];
pub static mut global_out_info: *mut global_query_info = 0 as *const global_query_info
    as *mut global_query_info;
pub static mut query_type_map: [libc::c_int; 256] = [0; 256];
pub static mut global_now: time_t = 0 as libc::c_int as time_t;
pub static mut gnlock: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0,
        __count: 0,
        __owner: 0,
        __nusers: 0,
        __kind: 0,
        __spins: 0,
        __elision: 0,
        __list: __pthread_list_t {
            __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
            __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
        },
    },
};
pub static mut refresh_record: sig_atomic_t = 0 as libc::c_int;
unsafe extern "C" fn daemonrize(mut dm: libc::c_int) -> libc::c_int {
    if dm == 1 as libc::c_int {
        if daemon(1 as libc::c_int, 0 as libc::c_int) == -(1 as libc::c_int) {
            dns_error(
                0 as libc::c_int,
                b"daemonrize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else {
            printf(b"daemon!!!\n\0" as *const u8 as *const libc::c_char);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn create_listen_ports(
    mut port: libc::c_int,
    mut proto: libc::c_int,
    mut addr: *mut uchar,
) -> libc::c_int {
    let mut fd: libc::c_int = -(1 as libc::c_int);
    fd = create_socket(port, proto, addr);
    if fd < 0 as libc::c_int || set_non_block(fd) < 0 as libc::c_int {
        printf(b"port:%d,proto:%d\n\0" as *const u8 as *const libc::c_char, port, proto);
        dns_error(
            0 as libc::c_int,
            b"fd < 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return fd;
}
pub unsafe extern "C" fn create_author(
    mut s: *mut server,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut authors: *mut author = 0 as *mut author;
    let mut cpuinfo: cpu_set_t = cpu_set_t { __bits: [0; 16] };
    let mut apt: [pthread_t; 2] = [0; 2];
    if n < 1 as libc::c_int || n > 50 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"quizzer bad range\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    authors = malloc(
        (::std::mem::size_of::<author>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut author;
    if authors.is_null() {
        dns_error(
            0 as libc::c_int,
            b"out of memory in quizzer\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    memset(
        authors as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<author>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    );
    (*s).authors = authors;
    i = 0 as libc::c_int;
    while i < n {
        (*authors.offset(i as isize)).idx = i;
        (*authors.offset(i as isize)).cudp = (*s).ludp;
        (*authors.offset(i as isize))
            .audp = create_listen_ports(
            i * 1000 as libc::c_int + 998 as libc::c_int,
            SOCK_DGRAM as libc::c_int,
            0 as *mut uchar,
        );
        if (*authors.offset(i as isize)).audp < 0 as libc::c_int {
            dns_error(
                0 as libc::c_int,
                b"auth fd error\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        set_sock_buff((*authors.offset(i as isize)).audp, 1 as libc::c_int);
        let ref mut fresh0 = (*authors.offset(i as isize)).el;
        *fresh0 = &mut (*s).eventlist;
        let ref mut fresh1 = (*authors.offset(i as isize)).s;
        *fresh1 = s;
        get_random_data(
            ((*authors.offset(i as isize)).randombuffer).as_mut_ptr(),
            RANDOM_SIZE as libc::c_int,
        );
        (*authors.offset(i as isize)).rndidx = 0 as libc::c_int;
        (*authors.offset(i as isize)).dupbefore = 0 as libc::c_int;
        (*authors.offset(i as isize)).limits = 10 as libc::c_int;
        (*authors.offset(i as isize)).bdepfd = 0 as libc::c_int;
        let ref mut fresh2 = (*authors.offset(i as isize)).fwd;
        *fresh2 = (*s).forward;
        let ref mut fresh3 = (*authors.offset(i as isize)).ds;
        *fresh3 = (*s).datasets;
        (*authors.offset(i as isize)).qnum = 0 as libc::c_int;
        (*authors.offset(i as isize)).underattack = 0 as libc::c_int;
        (*authors.offset(i as isize)).timex = 0 as libc::c_int;
        (*authors.offset(i as isize)).response = 0 as libc::c_int;
        (*authors.offset(i as isize)).tcpinuse = 0 as libc::c_int;
        (*authors.offset(i as isize)).rdb = 0 as libc::c_int as uint;
        (*authors.offset(i as isize)).quizz = 0 as libc::c_int as uint;
        (*authors.offset(i as isize)).drop = 0 as libc::c_int as uint;
        (*authors.offset(i as isize)).timeout = 0 as libc::c_int as uint;
        (*authors.offset(i as isize)).qidx = 0 as libc::c_int;
        (*authors.offset(i as isize))
            .start = 4095 as libc::c_int / QUIZZER_NUM as libc::c_int * i;
        if i == QUIZZER_NUM as libc::c_int - 1 as libc::c_int {
            (*authors.offset(i as isize)).end = 4095 as libc::c_int;
        } else {
            (*authors.offset(i as isize))
                .end = 4095 as libc::c_int / QUIZZER_NUM as libc::c_int
                * (i + 1 as libc::c_int);
        }
        memset(
            ((*authors.offset(i as isize)).ip).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            IP_DATA_LEN as libc::c_int as libc::c_ulong,
        );
        let ref mut fresh4 = (*authors.offset(i as isize)).loginfo;
        *fresh4 = malloc(::std::mem::size_of::<log_info>() as libc::c_ulong)
            as *mut log_info;
        memset(
            (*authors.offset(i as isize)).loginfo as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<log_info>() as libc::c_ulong,
        );
        (*(*authors.offset(i as isize)).loginfo).log_type = 233 as libc::c_int;
        (*(*authors.offset(i as isize)).loginfo)
            .logfd = create_new_log(((*s).logpath).as_mut_ptr(), i, 233 as libc::c_int);
        j = 0 as libc::c_int;
        while j < AUTH_DB_NUM as libc::c_int {
            pthread_spin_init(
                &mut *((*authors.offset(i as isize)).dblock)
                    .as_mut_ptr()
                    .offset(j as isize),
                0 as libc::c_int,
            );
            j += 1;
            j;
        }
        j = 0 as libc::c_int;
        while j < LIST_SPACE as libc::c_int {
            let ref mut fresh5 = (*authors.offset(i as isize)).list[j as usize];
            *fresh5 = 0 as *mut qoutinfo;
            j += 1;
            j;
        }
        j = 0 as libc::c_int;
        while j < EP_TCP_FDS as libc::c_int {
            (*authors.offset(i as isize)).eptcpfds[j as usize].ret = -(1 as libc::c_int);
            j += 1;
            j;
        }
        pthread_spin_init(&mut (*authors.offset(i as isize)).lock, 0 as libc::c_int);
        (*(*authors.offset(i as isize)).loginfo).lastlog = global_now;
        if (*authors.offset(i as isize)).cudp < 0 as libc::c_int
            || (*authors.offset(i as isize)).audp < 0 as libc::c_int
        {
            dns_error(
                0 as libc::c_int,
                b"create quizzer2\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        if pthread_create(
            apt.as_mut_ptr().offset(i as isize),
            0 as *const pthread_attr_t,
            Some(
                run_quizzer
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            &mut *authors.offset(i as isize) as *mut author as *mut libc::c_void,
        ) != 0 as libc::c_int
        {
            dns_error(
                0 as libc::c_int,
                b"create quizzer\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        i += 1;
        i;
    }
    (*global_out_info).thread_num += i;
    i = 0 as libc::c_int;
    while i < QUIZZER_NUM as libc::c_int {
        libc::memset(
            &mut cpuinfo as *mut cpu_set_t as *mut libc::c_void,
            '\0' as i32,
            ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong as libc::size_t,
        );
        let mut __cpu: size_t = (i + FETCHER_NUM as libc::c_int + 1 as libc::c_int)
            as size_t;
        if __cpu.wrapping_div(8 as libc::c_int as libc::c_ulong)
            < ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong
        {
            let ref mut fresh6 = *(cpuinfo.__bits)
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
            *fresh6
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
                apt[i as usize],
                ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong,
                &mut cpuinfo,
            )
        {
            printf(
                b"set affinity quizzer failed, may be the cpu cores num less than (FETCHER_NUM + QUIZZER_NUM + 1)\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn create_fetcher(
    mut s: *mut server,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ws: *mut fetcher = 0 as *mut fetcher;
    let mut tmp: *mut fetcher = 0 as *mut fetcher;
    let mut cpuinfo: cpu_set_t = cpu_set_t { __bits: [0; 16] };
    let mut fpt: [pthread_t; 2] = [0; 2];
    if n < 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    ws = malloc(
        (::std::mem::size_of::<fetcher>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut fetcher;
    if ws.is_null() {
        return -(1 as libc::c_int);
    }
    memset(
        ws as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<fetcher>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    );
    (*s).fetchers = ws;
    i = 0 as libc::c_int;
    while i < n {
        tmp = ws.offset(i as isize);
        (*tmp).s = s;
        (*tmp).idx = i;
        (*tmp).pkg = 0 as libc::c_int as uint64_t;
        (*tmp).send = 0 as libc::c_int as uint64_t;
        (*tmp).miss = 0 as libc::c_int as uint64_t;
        (*tmp).el = &mut (*s).eventlist;
        (*tmp).qidx = i % QUIZZER_NUM as libc::c_int;
        (*tmp).mc = init_msgcache(100 as libc::c_int);
        if ((*tmp).mc).is_null() {
            dns_error(
                0 as libc::c_int,
                b"get msgcache\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        (*tmp)
            .loginfo = malloc(::std::mem::size_of::<log_info>() as libc::c_ulong)
            as *mut log_info;
        memset(
            (*tmp).loginfo as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<log_info>() as libc::c_ulong,
        );
        (*(*tmp).loginfo).lastlog = global_now;
        (*(*tmp).loginfo).log_type = 112 as libc::c_int;
        (*(*tmp).loginfo)
            .logfd = create_new_log(((*s).logpath).as_mut_ptr(), i, 112 as libc::c_int);
        if (*(*tmp).loginfo).logfd < 0 as libc::c_int {
            dns_error(
                0 as libc::c_int,
                b"log file error\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        if pthread_create(
            fpt.as_mut_ptr().offset(i as isize),
            0 as *const pthread_attr_t,
            ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
            >(
                ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut fetcher) -> libc::c_int>,
                    *mut libc::c_void,
                >(Some(run_fetcher as unsafe extern "C" fn(*mut fetcher) -> libc::c_int)),
            ),
            tmp as *mut libc::c_void,
        ) != 0 as libc::c_int
        {
            dns_error(
                0 as libc::c_int,
                b"init worker\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        i += 1;
        i;
    }
    (*global_out_info).thread_num += i;
    i = 0 as libc::c_int;
    while i < FETCHER_NUM as libc::c_int {
        libc::memset(
            &mut cpuinfo as *mut cpu_set_t as *mut libc::c_void,
            '\0' as i32,
            ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong as libc::size_t,
        );
        let mut __cpu: size_t = (i + 1 as libc::c_int) as size_t;
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
                fpt[i as usize],
                ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong,
                &mut cpuinfo,
            )
        {
            printf(
                b"set affinity fetcher failed,  may be the cpu cores num less than (FETCHER_NUM + QUIZZER_NUM + 1)\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn server_init() -> *mut server {
    let mut s: *mut server = malloc(::std::mem::size_of::<server>() as libc::c_ulong)
        as *mut server;
    if s.is_null() {
        dns_error(
            0 as libc::c_int,
            b"out of memory in server_init\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    (*s).nfetcher = FETCHER_NUM as libc::c_int as ushort;
    (*s).nquizzer = QUIZZER_NUM as libc::c_int as ushort;
    (*s).authors = 0 as *mut author;
    (*s).fetchers = 0 as *mut fetcher;
    (*s).pkg = 0 as libc::c_int as ulong;
    pthread_spin_init(&mut (*s).eventlist.lock, 0 as libc::c_int);
    (*s).eventlist.head = 0 as *mut list_node;
    (*s)
        .ludp = create_listen_ports(
        SERVER_PORT as libc::c_int,
        SOCK_DGRAM as libc::c_int,
        b"0.0.0.0\0" as *const u8 as *const libc::c_char as *mut uchar,
    );
    if (*s).ludp < 0 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"can not open udp\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    set_sock_buff((*s).ludp, 10 as libc::c_int);
    (*s)
        .ltcp = create_listen_ports(
        SERVER_PORT as libc::c_int,
        SOCK_STREAM as libc::c_int,
        b"0.0.0.0\0" as *const u8 as *const libc::c_char as *mut uchar,
    );
    if (*s).ltcp < 0 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"can not open tcp\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    (*s)
        .datasets = htable_create(
        None,
        Some(
            dict_comp_str_equ
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        65536 as libc::c_int,
        10 as libc::c_int,
    );
    if ((*s).datasets).is_null() {
        dns_error(
            0 as libc::c_int,
            b"htable create\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    (*s)
        .forward = htable_create(
        None,
        Some(
            dict_comp_str_equ
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        1024 as libc::c_int,
        1 as libc::c_int,
    );
    if ((*s).forward).is_null() {
        dns_error(
            0 as libc::c_int,
            b"create forward\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    (*s)
        .qlist = htable_create(
        None,
        Some(
            dict_comp_str_equ
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        4095 as libc::c_int,
        1 as libc::c_int,
    );
    if ((*s).qlist).is_null() {
        dns_error(
            0 as libc::c_int,
            b"create qlist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    (*s)
        .ttlexp = create_rbtree(
        Some(
            rbt_comp_ttl_gt
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        0 as *mut libc::c_void,
    );
    if ((*s).ttlexp).is_null() {
        dns_error(
            0 as libc::c_int,
            b"create ttl tree\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    (*s).recordsindb = 0 as libc::c_int as ulong;
    (*s).refreshflag = 0 as libc::c_int as uint16_t;
    (*s).lastrefresh = global_now;
    (*s).is_forward = 0 as libc::c_int;
    return s;
}
pub unsafe extern "C" fn time_cron(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut s: *mut server = arg as *mut server;
    let mut tv: timespec = {
        let mut init = timespec {
            tv_sec: 0 as libc::c_int as __time_t,
            tv_nsec: 0,
        };
        init
    };
    let mut waitset: sigset_t = sigset_t { __val: [0; 16] };
    let mut info: siginfo_t = siginfo_t {
        si_signo: 0,
        si_errno: 0,
        si_code: 0,
        __pad0: 0,
        _sifields: C2RustUnnamed { _pad: [0; 28] },
    };
    let mut ret: libc::c_int = 0;
    sigemptyset(&mut waitset);
    sigaddset(&mut waitset, 10 as libc::c_int);
    global_now = time(0 as *mut time_t);
    loop {
        tv.tv_sec = 1 as libc::c_int as __time_t;
        tv.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
        ret = sigtimedwait(&mut waitset, &mut info, &mut tv);
        if ret > 0 as libc::c_int {
            (*s).refreshflag = 1 as libc::c_int as uint16_t;
        }
        global_now = time(0 as *mut time_t);
    };
}
pub unsafe extern "C" fn recv_update(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut s: *mut server = arg as *mut server;
    start_local_server(s);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn sanity_test(mut exi: libc::c_int) -> libc::c_int {
    if exi != 0 {
        exit(0 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn print_basic_debug() -> libc::c_int {
    printf(
        b"[DBG:] dnspod-sr is successfully running now!!\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"[DBG:] max_ele_size is %u - 1808\n\0" as *const u8 as *const libc::c_char,
        MAX_ELE_NUM,
    );
    printf(
        b"[DBG:] server may contain %u useful records\n\0" as *const u8
            as *const libc::c_char,
        MAX_ELE_NUM
            .wrapping_sub(1808 as libc::c_int as libc::c_uint)
            .wrapping_div(3 as libc::c_int as libc::c_uint),
    );
    printf(
        b"[DBG:] hash_table_size is %u\n\0" as *const u8 as *const libc::c_char,
        65536 as libc::c_int,
    );
    printf(
        b"[DBG:] we have %u hash tables\n\0" as *const u8 as *const libc::c_char,
        10 as libc::c_int,
    );
    printf(
        b"[DBG:] we have %u fetchers,%u quizzers\n\0" as *const u8
            as *const libc::c_char,
        FETCHER_NUM as libc::c_int,
        QUIZZER_NUM as libc::c_int,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn help(mut progname: *const libc::c_char) {
    printf(b"DNSPod recursive dns server\n\0" as *const u8 as *const libc::c_char);
    printf(b"version 0.01\n\0" as *const u8 as *const libc::c_char);
    printf(b"Usage: %s [-c config]\n\0" as *const u8 as *const libc::c_char, progname);
}
pub unsafe extern "C" fn init_globe() -> libc::c_int {
    let mut shmid: libc::c_int = 0;
    shmid = shmget(
        SHM_KEY as libc::c_int,
        ::std::mem::size_of::<global_query_info>() as libc::c_ulong,
        0o1000 as libc::c_int | 0o600 as libc::c_int | 0 as libc::c_int,
    );
    if shmid < 0 as libc::c_int {
        printf(
            b"%lu\n\0" as *const u8 as *const libc::c_char,
            (SHM_KEY as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    ::std::mem::size_of::<global_query_info>() as libc::c_ulong,
                ),
        );
        perror(b"shmget\0" as *const u8 as *const libc::c_char);
        dns_error(
            0 as libc::c_int,
            b"shmget error\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    global_out_info = shmat(shmid, 0 as *const libc::c_void, 0 as libc::c_int)
        as *mut global_query_info;
    memset(
        global_out_info as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<global_query_info>() as libc::c_ulong,
    );
    (*global_out_info).thread_num = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[libc::c_int; 256]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        query_type_map[i as usize] = -(1 as libc::c_int);
        i += 1;
        i;
    }
    query_type_map[A as libc::c_int as usize] = 0 as libc::c_int;
    query_type_map[NS as libc::c_int as usize] = 1 as libc::c_int;
    query_type_map[CNAME as libc::c_int as usize] = 2 as libc::c_int;
    query_type_map[SOA as libc::c_int as usize] = 3 as libc::c_int;
    query_type_map[MX as libc::c_int as usize] = 4 as libc::c_int;
    query_type_map[TXT as libc::c_int as usize] = 5 as libc::c_int;
    query_type_map[AAAA as libc::c_int as usize] = 6 as libc::c_int;
    query_type_map[SRV as libc::c_int as usize] = 7 as libc::c_int;
    query_type_map[ANY as libc::c_int as usize] = 8 as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn init_mempool() {
    let mut ret: libc::c_int = 0;
    ret = mempool_create(65536 as libc::c_int as uint32_t);
    if ret < 0 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"create mempool failed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut s: *mut server = 0 as *mut server;
    let mut pt: pthread_t = 0;
    let mut ctl: pthread_t = 0;
    let mut c: libc::c_int = 0;
    let mut is_forward: libc::c_int = 0 as libc::c_int;
    let mut config: *const libc::c_char = b"../sr.conf\0" as *const u8
        as *const libc::c_char;
    let mut daemon_0: libc::c_int = 0 as libc::c_int;
    loop {
        c = getopt(argc, argv, b"c:vhfd\0" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            99 => {
                config = optarg;
            }
            104 => {
                help(*argv.offset(0 as libc::c_int as isize));
                exit(0 as libc::c_int);
            }
            102 => {
                is_forward = 1 as libc::c_int;
            }
            100 => {
                daemon_0 = 1 as libc::c_int;
            }
            63 => {
                printf(b"Try -h please\n\0" as *const u8 as *const libc::c_char);
                exit(0 as libc::c_int);
            }
            118 => {
                printf(b"dnspod-sr 0.01\n\0" as *const u8 as *const libc::c_char);
                exit(0 as libc::c_int);
            }
            _ => {
                exit(0 as libc::c_int);
            }
        }
    }
    sanity_test(0 as libc::c_int);
    drop_privilege(b"./\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    daemonrize(daemon_0);
    trig_signals(1 as libc::c_int);
    global_now = time(0 as *mut time_t);
    g_nameservers[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
    g_nameservers[0 as libc::c_int as usize] = g_nameservers[1 as libc::c_int as usize];
    init_globe();
    init_mempool();
    s = server_init();
    (*s).is_forward = is_forward;
    read_config(
        config,
        ((*s).logpath).as_mut_ptr() as *mut libc::c_char,
        (*s).forward,
        g_nameservers.as_mut_ptr(),
    );
    if (g_nameservers[0 as libc::c_int as usize]).is_null() {
        if (g_nameservers[1 as libc::c_int as usize]).is_null() {} else {
            __assert_fail(
                b"g_nameservers[1] == NULL\0" as *const u8 as *const libc::c_char,
                b"init.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
        'c_9089: {
            if (g_nameservers[1 as libc::c_int as usize]).is_null() {} else {
                __assert_fail(
                    b"g_nameservers[1] == NULL\0" as *const u8 as *const libc::c_char,
                    b"init.c\0" as *const u8 as *const libc::c_char,
                    412 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 23],
                        &[libc::c_char; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            }
        };
        g_nameservers[0 as libc::c_int
            as usize] = strdup(b"119.29.29.29\0" as *const u8 as *const libc::c_char);
        g_nameservers[1 as libc::c_int
            as usize] = strdup(b"8.8.4.4\0" as *const u8 as *const libc::c_char);
    }
    if (g_nameservers[1 as libc::c_int as usize]).is_null() {
        if strcmp(
            g_nameservers[0 as libc::c_int as usize],
            b"119.29.29.29\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            g_nameservers[1 as libc::c_int
                as usize] = strdup(b"8.8.4.4\0" as *const u8 as *const libc::c_char);
        } else {
            g_nameservers[1 as libc::c_int
                as usize] = strdup(
                b"119.29.29.29\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if create_fetcher(s, (*s).nfetcher as libc::c_int) < 0 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"create worker\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if create_author(s, (*s).nquizzer as libc::c_int) < 0 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"create author\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if pthread_create(
        &mut pt,
        0 as *const pthread_attr_t,
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        >(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
                *mut libc::c_void,
            >(
                Some(
                    time_cron
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
            ),
        ),
        s as *mut libc::c_void,
    ) != 0 as libc::c_int
    {
        dns_error(
            0 as libc::c_int,
            b"time cron error\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if pthread_create(
        &mut ctl,
        0 as *const pthread_attr_t,
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        >(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
                *mut libc::c_void,
            >(
                Some(
                    recv_update
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
            ),
        ),
        s as *mut libc::c_void,
    ) != 0 as libc::c_int
    {
        dns_error(
            0 as libc::c_int,
            b"recv update thread error\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    read_root((*s).datasets, (*s).ttlexp);
    print_basic_debug();
    global_serv = s;
    run_sentinel(s);
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
