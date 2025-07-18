use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn random() -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vprintf(_: *const libc::c_char, _: ::std::ffi::VaList) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn pthread_spin_trylock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    fn pthread_spin_unlock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn pthread_sigmask(
        __how: libc::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> libc::c_int;
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
pub type __uint64_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
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
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type pthread_spinlock_t = libc::c_int;
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
pub type uint64_t = __uint64_t;
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
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
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
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
pub type comparefunc = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
pub type hashfunc = unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> hashval_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hdata {
    pub list: *mut hentry,
    pub now: uint64_t,
    pub lock: pthread_spinlock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hentry {
    pub c2rust_unnamed: C2RustUnnamed_10,
    pub next: *mut hentry,
    pub count: libc::c_int,
    pub key: [uchar; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub vals: [*mut uchar; 9],
    pub val: type_value,
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
pub struct eptcpfds {
    pub ret: libc::c_int,
    pub domain: [uchar; 256],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct epoll_event {
    pub events: uint32_t,
    pub data: epoll_data_t,
}
pub type epoll_data_t = epoll_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub union epoll_data {
    pub ptr: *mut libc::c_void,
    pub fd: libc::c_int,
    pub u32_0: uint32_t,
    pub u64_0: uint64_t,
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
pub struct sockinfo {
    pub addr: sockaddr_in,
    pub fd: libc::c_int,
    pub buflen: libc::c_int,
    pub socktype: libc::c_int,
    pub buf: *mut uchar,
    pub lowerdomain: *mut packet_type,
    pub mbuf: *mut mbuf_type,
}
pub type mbuf_type = _mem_buf;
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
pub type in_port_t = uint16_t;
pub type sa_family_t = libc::c_ushort;
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
pub struct msgcache {
    pub head: uint64_t,
    pub tail: uint64_t,
    pub size: uint32_t,
    pub pkt: uint32_t,
    pub lock: pthread_spinlock_t,
    pub data: [uchar; 0],
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
pub static mut query_type_map: [libc::c_int; 256] = [0; 256];
pub static mut global_out_info: *mut global_query_info = 0 as *const global_query_info
    as *mut global_query_info;
pub static mut g_nameservers: [*mut libc::c_char; 2] = [0 as *const libc::c_char
    as *mut libc::c_char; 2];
pub unsafe extern "C" fn slog(
    mut msg: *mut uchar,
    mut fd: libc::c_int,
    mut lock: *mut pthread_spinlock_t,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn get_random_data(
    mut buffer: *mut uchar,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut ret: libc::c_int = 0 as libc::c_int;
    if buffer.is_null() || len < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    fd = open(b"/dev/urandom\0" as *const u8 as *const libc::c_char, 0 as libc::c_int);
    if fd <= 0 as libc::c_int {
        return fd;
    }
    ret = read(fd, buffer as *mut libc::c_void, len as size_t) as libc::c_int;
    if ret == -(1 as libc::c_int) {
        perror(b"read\0" as *const u8 as *const libc::c_char);
    }
    close(fd);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn get_str(
    mut str: *mut uchar,
    mut len: libc::c_int,
) -> *mut uchar {
    let mut ret: *mut uchar = malloc((len + 1 as libc::c_int) as libc::c_ulong)
        as *mut uchar;
    strncpy(
        ret as *mut libc::c_char,
        str as *mut libc::c_char,
        (len + 1 as libc::c_int) as libc::c_ulong,
    );
    *ret.offset(len as isize) = 0 as libc::c_int as uchar;
    return ret;
}
pub unsafe extern "C" fn put_str(mut str: *mut uchar) {
    free(str as *mut libc::c_void);
}
pub unsafe extern "C" fn flush_all_to_disk(mut s: *mut server) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut log: *mut log_info = 0 as *mut log_info;
    i = 0 as libc::c_int;
    while i < (*s).nfetcher as libc::c_int {
        log = (*((*s).fetchers).offset(i as isize)).loginfo;
        ret = write(
            (*log).logfd,
            ((*log).log_cache).as_mut_ptr() as *const libc::c_void,
            (*log).log_cache_cursor as size_t,
        ) as libc::c_int;
        if ret == -(1 as libc::c_int) {
            perror(b"write\0" as *const u8 as *const libc::c_char);
        }
        (*log).log_cache_cursor = 0 as libc::c_int;
        close((*log).logfd);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*s).nquizzer as libc::c_int {
        log = (*((*s).authors).offset(i as isize)).loginfo;
        ret = write(
            (*log).logfd,
            ((*log).log_cache).as_mut_ptr() as *const libc::c_void,
            (*log).log_cache_cursor as size_t,
        ) as libc::c_int;
        if ret == -(1 as libc::c_int) {
            perror(b"write\0" as *const u8 as *const libc::c_char);
        }
        (*log).log_cache_cursor = 0 as libc::c_int;
        close((*log).logfd);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn sig_segment_fault(mut signo: libc::c_int) {
    printf(b"sig number is %d\n\0" as *const u8 as *const libc::c_char, signo);
    flush_all_to_disk(global_serv);
    exit(0 as libc::c_int);
}
pub unsafe extern "C" fn trig_signals(mut sig: libc::c_int) -> libc::c_int {
    let mut bset: sigset_t = sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = sigset_t { __val: [0; 16] };
    let mut sigs: [libc::c_int; 4] = [
        2 as libc::c_int,
        7 as libc::c_int,
        11 as libc::c_int,
        13 as libc::c_int,
    ];
    let mut i: libc::c_int = 0;
    let mut sig_num: libc::c_int = 0;
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut oa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    memset(
        &mut sa as *mut sigaction as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sigaction>() as libc::c_ulong,
    );
    sa
        .__sigaction_handler
        .sa_handler = Some(sig_segment_fault as unsafe extern "C" fn(libc::c_int) -> ());
    sa.sa_flags = 0x10000000 as libc::c_int;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[libc::c_int; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        sig_num = sigs[i as usize];
        sigaction(sig_num, &mut sa, &mut oa);
        i += 1;
        i;
    }
    sigemptyset(&mut bset);
    sigaddset(&mut bset, 10 as libc::c_int);
    if pthread_sigmask(0 as libc::c_int, &mut bset, &mut oset) != 0 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"sig error\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn drop_privilege(mut root: *mut libc::c_char) {
    if root.is_null() {
        return;
    }
}
pub unsafe extern "C" fn dict_comp_uint_equ(
    mut a: *mut libc::c_void,
    mut b: *mut libc::c_void,
) -> libc::c_int {
    let mut u1: *mut uint = a as *mut uint;
    let mut u2: *mut uint = b as *mut uint;
    if u1.is_null() {
        return -(1 as libc::c_int);
    }
    if u2.is_null() {
        return 1 as libc::c_int;
    }
    if *u1 == *u2 {
        return 0 as libc::c_int;
    }
    return if u1 > u2 { 1 as libc::c_int } else { -(1 as libc::c_int) };
}
pub unsafe extern "C" fn dict_comp_str_equ(
    mut a: *mut libc::c_void,
    mut b: *mut libc::c_void,
) -> libc::c_int {
    let mut d1: *mut uchar = a as *mut uchar;
    let mut d2: *mut uchar = b as *mut uchar;
    let mut to: libc::c_int = 256 as libc::c_int;
    if d1.is_null() {
        return -(1 as libc::c_int);
    }
    if d2.is_null() {
        return 1 as libc::c_int;
    }
    while *d1 as libc::c_int != 0 as libc::c_int
        && *d2 as libc::c_int != 0 as libc::c_int
    {
        if *d1 as libc::c_int > *d2 as libc::c_int {
            return 1 as libc::c_int;
        }
        if (*d1 as libc::c_int) < *d2 as libc::c_int {
            return -(1 as libc::c_int);
        }
        d1 = d1.offset(1);
        d1;
        d2 = d2.offset(1);
        d2;
        let fresh0 = to;
        to = to - 1;
        if fresh0 == 0 as libc::c_int {
            printf(b"str compare error\n\0" as *const u8 as *const libc::c_char);
            exit(0 as libc::c_int);
        }
    }
    if *d1 as libc::c_int == 0 as libc::c_int && *d2 as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if *d1 as libc::c_int == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn rbt_comp_ttl_gt(
    mut v1: *mut libc::c_void,
    mut v2: *mut libc::c_void,
    mut argv: *mut libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut n1: *mut ttlnode = 0 as *mut ttlnode;
    let mut n2: *mut ttlnode = 0 as *mut ttlnode;
    if v2.is_null() {
        return 1 as libc::c_int;
    }
    if v1.is_null() {
        return -(1 as libc::c_int);
    }
    n1 = v1 as *mut ttlnode;
    n2 = v2 as *mut ttlnode;
    if (*n1).exp > (*n2).exp {
        return 1 as libc::c_int;
    }
    if (*n1).exp < (*n2).exp {
        return -(1 as libc::c_int);
    }
    if (*n1).type_0 as libc::c_int > (*n2).type_0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if ((*n1).type_0 as libc::c_int) < (*n2).type_0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    ret = dict_comp_str_equ(
        (*n1).data as *mut libc::c_void,
        (*n2).data as *mut libc::c_void,
    );
    if ret > 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn rbt_comp_uint_gt(
    mut v1: *mut libc::c_void,
    mut v2: *mut libc::c_void,
    mut argv: *mut libc::c_void,
) -> libc::c_int {
    let mut n1: uint = 0;
    let mut n2: uint = 0;
    if v2.is_null() {
        return 1 as libc::c_int;
    }
    if v1.is_null() {
        return -(1 as libc::c_int);
    }
    n1 = *(v1 as *mut uint);
    n2 = *(v2 as *mut uint);
    if n1 == n2 {
        return 0 as libc::c_int;
    }
    return if n1 > n2 { 1 as libc::c_int } else { -(1 as libc::c_int) };
}
pub static mut LowerTable: [libc::c_uchar; 256] = [
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0x6 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0xd as libc::c_int as libc::c_uchar,
    0xe as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0x11 as libc::c_int as libc::c_uchar,
    0x12 as libc::c_int as libc::c_uchar,
    0x13 as libc::c_int as libc::c_uchar,
    0x14 as libc::c_int as libc::c_uchar,
    0x15 as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0x17 as libc::c_int as libc::c_uchar,
    0x18 as libc::c_int as libc::c_uchar,
    0x19 as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0x1c as libc::c_int as libc::c_uchar,
    0x1d as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x23 as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x26 as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0x29 as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0x33 as libc::c_int as libc::c_uchar,
    0x34 as libc::c_int as libc::c_uchar,
    0x35 as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0x37 as libc::c_int as libc::c_uchar,
    0x38 as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0x40 as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0x62 as libc::c_int as libc::c_uchar,
    0x63 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0x6d as libc::c_int as libc::c_uchar,
    0x6e as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0x71 as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
    0x73 as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x78 as libc::c_int as libc::c_uchar,
    0x79 as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0x5b as libc::c_int as libc::c_uchar,
    0x5c as libc::c_int as libc::c_uchar,
    0x5d as libc::c_int as libc::c_uchar,
    0x5e as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0x62 as libc::c_int as libc::c_uchar,
    0x63 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0x6d as libc::c_int as libc::c_uchar,
    0x6e as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0x71 as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
    0x73 as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x78 as libc::c_int as libc::c_uchar,
    0x79 as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0x7d as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0x7f as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0x81 as libc::c_int as libc::c_uchar,
    0x82 as libc::c_int as libc::c_uchar,
    0x83 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0x85 as libc::c_int as libc::c_uchar,
    0x86 as libc::c_int as libc::c_uchar,
    0x87 as libc::c_int as libc::c_uchar,
    0x88 as libc::c_int as libc::c_uchar,
    0x89 as libc::c_int as libc::c_uchar,
    0x8a as libc::c_int as libc::c_uchar,
    0x8b as libc::c_int as libc::c_uchar,
    0x8c as libc::c_int as libc::c_uchar,
    0x8d as libc::c_int as libc::c_uchar,
    0x8e as libc::c_int as libc::c_uchar,
    0x8f as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0x91 as libc::c_int as libc::c_uchar,
    0x92 as libc::c_int as libc::c_uchar,
    0x93 as libc::c_int as libc::c_uchar,
    0x94 as libc::c_int as libc::c_uchar,
    0x95 as libc::c_int as libc::c_uchar,
    0x96 as libc::c_int as libc::c_uchar,
    0x97 as libc::c_int as libc::c_uchar,
    0x98 as libc::c_int as libc::c_uchar,
    0x99 as libc::c_int as libc::c_uchar,
    0x9a as libc::c_int as libc::c_uchar,
    0x9b as libc::c_int as libc::c_uchar,
    0x9c as libc::c_int as libc::c_uchar,
    0x9d as libc::c_int as libc::c_uchar,
    0x9e as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0xa0 as libc::c_int as libc::c_uchar,
    0xa1 as libc::c_int as libc::c_uchar,
    0xa2 as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0xa4 as libc::c_int as libc::c_uchar,
    0xa5 as libc::c_int as libc::c_uchar,
    0xa6 as libc::c_int as libc::c_uchar,
    0xa7 as libc::c_int as libc::c_uchar,
    0xa8 as libc::c_int as libc::c_uchar,
    0xa9 as libc::c_int as libc::c_uchar,
    0xaa as libc::c_int as libc::c_uchar,
    0xab as libc::c_int as libc::c_uchar,
    0xac as libc::c_int as libc::c_uchar,
    0xad as libc::c_int as libc::c_uchar,
    0xae as libc::c_int as libc::c_uchar,
    0xaf as libc::c_int as libc::c_uchar,
    0xb0 as libc::c_int as libc::c_uchar,
    0xb1 as libc::c_int as libc::c_uchar,
    0xb2 as libc::c_int as libc::c_uchar,
    0xb3 as libc::c_int as libc::c_uchar,
    0xb4 as libc::c_int as libc::c_uchar,
    0xb5 as libc::c_int as libc::c_uchar,
    0xb6 as libc::c_int as libc::c_uchar,
    0xb7 as libc::c_int as libc::c_uchar,
    0xb8 as libc::c_int as libc::c_uchar,
    0xb9 as libc::c_int as libc::c_uchar,
    0xba as libc::c_int as libc::c_uchar,
    0xbb as libc::c_int as libc::c_uchar,
    0xbc as libc::c_int as libc::c_uchar,
    0xbd as libc::c_int as libc::c_uchar,
    0xbe as libc::c_int as libc::c_uchar,
    0xbf as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0xc1 as libc::c_int as libc::c_uchar,
    0xc2 as libc::c_int as libc::c_uchar,
    0xc3 as libc::c_int as libc::c_uchar,
    0xc4 as libc::c_int as libc::c_uchar,
    0xc5 as libc::c_int as libc::c_uchar,
    0xc6 as libc::c_int as libc::c_uchar,
    0xc7 as libc::c_int as libc::c_uchar,
    0xc8 as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0xca as libc::c_int as libc::c_uchar,
    0xcb as libc::c_int as libc::c_uchar,
    0xcc as libc::c_int as libc::c_uchar,
    0xcd as libc::c_int as libc::c_uchar,
    0xce as libc::c_int as libc::c_uchar,
    0xcf as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0xd2 as libc::c_int as libc::c_uchar,
    0xd3 as libc::c_int as libc::c_uchar,
    0xd4 as libc::c_int as libc::c_uchar,
    0xd5 as libc::c_int as libc::c_uchar,
    0xd6 as libc::c_int as libc::c_uchar,
    0xd7 as libc::c_int as libc::c_uchar,
    0xd8 as libc::c_int as libc::c_uchar,
    0xd9 as libc::c_int as libc::c_uchar,
    0xda as libc::c_int as libc::c_uchar,
    0xdb as libc::c_int as libc::c_uchar,
    0xdc as libc::c_int as libc::c_uchar,
    0xdd as libc::c_int as libc::c_uchar,
    0xde as libc::c_int as libc::c_uchar,
    0xdf as libc::c_int as libc::c_uchar,
    0xe0 as libc::c_int as libc::c_uchar,
    0xe1 as libc::c_int as libc::c_uchar,
    0xe2 as libc::c_int as libc::c_uchar,
    0xe3 as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0xe5 as libc::c_int as libc::c_uchar,
    0xe6 as libc::c_int as libc::c_uchar,
    0xe7 as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0xe9 as libc::c_int as libc::c_uchar,
    0xea as libc::c_int as libc::c_uchar,
    0xeb as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0xed as libc::c_int as libc::c_uchar,
    0xee as libc::c_int as libc::c_uchar,
    0xef as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
    0xf1 as libc::c_int as libc::c_uchar,
    0xf2 as libc::c_int as libc::c_uchar,
    0xf3 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0xf5 as libc::c_int as libc::c_uchar,
    0xf6 as libc::c_int as libc::c_uchar,
    0xf7 as libc::c_int as libc::c_uchar,
    0xf8 as libc::c_int as libc::c_uchar,
    0xf9 as libc::c_int as libc::c_uchar,
    0xfa as libc::c_int as libc::c_uchar,
    0xfb as libc::c_int as libc::c_uchar,
    0xfc as libc::c_int as libc::c_uchar,
    0xfd as libc::c_int as libc::c_uchar,
    0xfe as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
];
pub static mut UpperTable: [libc::c_uchar; 256] = [
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0x6 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0xd as libc::c_int as libc::c_uchar,
    0xe as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0x11 as libc::c_int as libc::c_uchar,
    0x12 as libc::c_int as libc::c_uchar,
    0x13 as libc::c_int as libc::c_uchar,
    0x14 as libc::c_int as libc::c_uchar,
    0x15 as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0x17 as libc::c_int as libc::c_uchar,
    0x18 as libc::c_int as libc::c_uchar,
    0x19 as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0x1c as libc::c_int as libc::c_uchar,
    0x1d as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x23 as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x26 as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0x29 as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0x33 as libc::c_int as libc::c_uchar,
    0x34 as libc::c_int as libc::c_uchar,
    0x35 as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0x37 as libc::c_int as libc::c_uchar,
    0x38 as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0x40 as libc::c_int as libc::c_uchar,
    0x41 as libc::c_int as libc::c_uchar,
    0x42 as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x46 as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x4a as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0x4c as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0x4e as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0x50 as libc::c_int as libc::c_uchar,
    0x51 as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0x59 as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0x5b as libc::c_int as libc::c_uchar,
    0x5c as libc::c_int as libc::c_uchar,
    0x5d as libc::c_int as libc::c_uchar,
    0x5e as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0x41 as libc::c_int as libc::c_uchar,
    0x42 as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x46 as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x4a as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0x4c as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0x4e as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0x50 as libc::c_int as libc::c_uchar,
    0x51 as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0x59 as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0x7d as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0x7f as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0x81 as libc::c_int as libc::c_uchar,
    0x82 as libc::c_int as libc::c_uchar,
    0x83 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0x85 as libc::c_int as libc::c_uchar,
    0x86 as libc::c_int as libc::c_uchar,
    0x87 as libc::c_int as libc::c_uchar,
    0x88 as libc::c_int as libc::c_uchar,
    0x89 as libc::c_int as libc::c_uchar,
    0x8a as libc::c_int as libc::c_uchar,
    0x8b as libc::c_int as libc::c_uchar,
    0x8c as libc::c_int as libc::c_uchar,
    0x8d as libc::c_int as libc::c_uchar,
    0x8e as libc::c_int as libc::c_uchar,
    0x8f as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0x91 as libc::c_int as libc::c_uchar,
    0x92 as libc::c_int as libc::c_uchar,
    0x93 as libc::c_int as libc::c_uchar,
    0x94 as libc::c_int as libc::c_uchar,
    0x95 as libc::c_int as libc::c_uchar,
    0x96 as libc::c_int as libc::c_uchar,
    0x97 as libc::c_int as libc::c_uchar,
    0x98 as libc::c_int as libc::c_uchar,
    0x99 as libc::c_int as libc::c_uchar,
    0x9a as libc::c_int as libc::c_uchar,
    0x9b as libc::c_int as libc::c_uchar,
    0x9c as libc::c_int as libc::c_uchar,
    0x9d as libc::c_int as libc::c_uchar,
    0x9e as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0xa0 as libc::c_int as libc::c_uchar,
    0xa1 as libc::c_int as libc::c_uchar,
    0xa2 as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0xa4 as libc::c_int as libc::c_uchar,
    0xa5 as libc::c_int as libc::c_uchar,
    0xa6 as libc::c_int as libc::c_uchar,
    0xa7 as libc::c_int as libc::c_uchar,
    0xa8 as libc::c_int as libc::c_uchar,
    0xa9 as libc::c_int as libc::c_uchar,
    0xaa as libc::c_int as libc::c_uchar,
    0xab as libc::c_int as libc::c_uchar,
    0xac as libc::c_int as libc::c_uchar,
    0xad as libc::c_int as libc::c_uchar,
    0xae as libc::c_int as libc::c_uchar,
    0xaf as libc::c_int as libc::c_uchar,
    0xb0 as libc::c_int as libc::c_uchar,
    0xb1 as libc::c_int as libc::c_uchar,
    0xb2 as libc::c_int as libc::c_uchar,
    0xb3 as libc::c_int as libc::c_uchar,
    0xb4 as libc::c_int as libc::c_uchar,
    0xb5 as libc::c_int as libc::c_uchar,
    0xb6 as libc::c_int as libc::c_uchar,
    0xb7 as libc::c_int as libc::c_uchar,
    0xb8 as libc::c_int as libc::c_uchar,
    0xb9 as libc::c_int as libc::c_uchar,
    0xba as libc::c_int as libc::c_uchar,
    0xbb as libc::c_int as libc::c_uchar,
    0xbc as libc::c_int as libc::c_uchar,
    0xbd as libc::c_int as libc::c_uchar,
    0xbe as libc::c_int as libc::c_uchar,
    0xbf as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0xc1 as libc::c_int as libc::c_uchar,
    0xc2 as libc::c_int as libc::c_uchar,
    0xc3 as libc::c_int as libc::c_uchar,
    0xc4 as libc::c_int as libc::c_uchar,
    0xc5 as libc::c_int as libc::c_uchar,
    0xc6 as libc::c_int as libc::c_uchar,
    0xc7 as libc::c_int as libc::c_uchar,
    0xc8 as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0xca as libc::c_int as libc::c_uchar,
    0xcb as libc::c_int as libc::c_uchar,
    0xcc as libc::c_int as libc::c_uchar,
    0xcd as libc::c_int as libc::c_uchar,
    0xce as libc::c_int as libc::c_uchar,
    0xcf as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0xd2 as libc::c_int as libc::c_uchar,
    0xd3 as libc::c_int as libc::c_uchar,
    0xd4 as libc::c_int as libc::c_uchar,
    0xd5 as libc::c_int as libc::c_uchar,
    0xd6 as libc::c_int as libc::c_uchar,
    0xd7 as libc::c_int as libc::c_uchar,
    0xd8 as libc::c_int as libc::c_uchar,
    0xd9 as libc::c_int as libc::c_uchar,
    0xda as libc::c_int as libc::c_uchar,
    0xdb as libc::c_int as libc::c_uchar,
    0xdc as libc::c_int as libc::c_uchar,
    0xdd as libc::c_int as libc::c_uchar,
    0xde as libc::c_int as libc::c_uchar,
    0xdf as libc::c_int as libc::c_uchar,
    0xe0 as libc::c_int as libc::c_uchar,
    0xe1 as libc::c_int as libc::c_uchar,
    0xe2 as libc::c_int as libc::c_uchar,
    0xe3 as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0xe5 as libc::c_int as libc::c_uchar,
    0xe6 as libc::c_int as libc::c_uchar,
    0xe7 as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0xe9 as libc::c_int as libc::c_uchar,
    0xea as libc::c_int as libc::c_uchar,
    0xeb as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0xed as libc::c_int as libc::c_uchar,
    0xee as libc::c_int as libc::c_uchar,
    0xef as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
    0xf1 as libc::c_int as libc::c_uchar,
    0xf2 as libc::c_int as libc::c_uchar,
    0xf3 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0xf5 as libc::c_int as libc::c_uchar,
    0xf6 as libc::c_int as libc::c_uchar,
    0xf7 as libc::c_int as libc::c_uchar,
    0xf8 as libc::c_int as libc::c_uchar,
    0xf9 as libc::c_int as libc::c_uchar,
    0xfa as libc::c_int as libc::c_uchar,
    0xfb as libc::c_int as libc::c_uchar,
    0xfc as libc::c_int as libc::c_uchar,
    0xfd as libc::c_int as libc::c_uchar,
    0xfe as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
];
pub unsafe extern "C" fn to_lowercase(
    mut msg: *mut uchar,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        *msg.offset(i as isize) = LowerTable[*msg.offset(i as isize) as usize];
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn to_uppercase(
    mut buf: *mut uchar,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *buf.offset(i as isize) = UpperTable[*buf.offset(i as isize) as usize];
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn str_to_uchar4(
    mut addr: *const libc::c_char,
    mut val: *mut uchar,
) -> libc::c_int {
    let mut tv: [uint; 4] = [0 as libc::c_int as uint, 0, 0, 0];
    let mut idx: uint = 0 as libc::c_int as uint;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = strlen(addr) as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        if *addr.offset(i as isize) as libc::c_int >= '0' as i32
            && *addr.offset(i as isize) as libc::c_int <= '9' as i32
        {
            tv[idx
                as usize] = (tv[idx as usize])
                .wrapping_mul(10 as libc::c_int as libc::c_uint)
                .wrapping_add(*addr.offset(i as isize) as libc::c_uint)
                .wrapping_sub('0' as i32 as libc::c_uint);
        } else {
            idx = idx.wrapping_add(1);
            idx;
            if *addr.offset(i as isize) as libc::c_int != '.' as i32
                || idx == 4 as libc::c_int as libc::c_uint
            {
                *val = 0 as libc::c_int as uchar;
                return -(1 as libc::c_int);
            }
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        *val.offset(i as isize) = tv[i as usize] as uchar;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn str_to_uchar6(
    mut addr: *mut uchar,
    mut val: *mut uchar,
) -> libc::c_int {
    let mut tv: [ushort; 8] = [0 as libc::c_int as ushort, 0, 0, 0, 0, 0, 0, 0];
    let mut idx: libc::c_int = 0 as libc::c_int;
    let mut gap: libc::c_int = 0 as libc::c_int;
    let mut gapidx: libc::c_int = -(1 as libc::c_int);
    let mut hasgap: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = strlen(addr as *const libc::c_char) as libc::c_int;
    let mut tmp: libc::c_char = 0;
    to_lowercase(addr, n);
    memset(
        val as *mut libc::c_void,
        0 as libc::c_int,
        16 as libc::c_int as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < n {
        tmp = *addr.offset(i as isize) as libc::c_char;
        if tmp as libc::c_int >= '0' as i32 && tmp as libc::c_int <= '9' as i32 {
            gap = 0 as libc::c_int;
            tv[idx
                as usize] = (tv[idx as usize] as libc::c_int * 16 as libc::c_int
                + tmp as libc::c_int - '0' as i32) as ushort;
        } else if tmp as libc::c_int >= 'a' as i32 && tmp as libc::c_int <= 'z' as i32 {
            gap = 0 as libc::c_int;
            tv[idx
                as usize] = (tv[idx as usize] as libc::c_int * 16 as libc::c_int
                + tmp as libc::c_int - 'a' as i32 + 10 as libc::c_int) as ushort;
        } else {
            idx += 1;
            idx;
            if gap == 1 as libc::c_int {
                if hasgap == 1 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                hasgap = 1 as libc::c_int;
                gapidx = idx - 1 as libc::c_int;
            }
            if gap == 0 as libc::c_int {
                gap = 1 as libc::c_int;
            }
            if tmp as libc::c_int != ':' as i32 || idx == 8 as libc::c_int {
                let ref mut fresh1 = *val.offset(3 as libc::c_int as isize);
                *fresh1 = 0 as libc::c_int as uchar;
                let ref mut fresh2 = *val.offset(2 as libc::c_int as isize);
                *fresh2 = *fresh1;
                let ref mut fresh3 = *val.offset(1 as libc::c_int as isize);
                *fresh3 = *fresh2;
                *val.offset(0 as libc::c_int as isize) = *fresh3;
                return 0 as libc::c_int;
            }
        }
        i += 1;
        i;
    }
    if hasgap == 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < gapidx {
            *val
                .offset(
                    (i * 2 as libc::c_int) as isize,
                ) = (tv[i as usize] as libc::c_int / 256 as libc::c_int) as uchar;
            *val
                .offset(
                    (i * 2 as libc::c_int + 1 as libc::c_int) as isize,
                ) = (tv[i as usize] as libc::c_int % 256 as libc::c_int) as uchar;
            i += 1;
            i;
        }
        i = idx - 1 as libc::c_int;
        while i >= gapidx {
            *val
                .offset(
                    ((i + 7 as libc::c_int - idx + 1 as libc::c_int) * 2 as libc::c_int)
                        as isize,
                ) = (tv[(i + 1 as libc::c_int) as usize] as libc::c_int
                / 256 as libc::c_int) as uchar;
            *val
                .offset(
                    ((i + 7 as libc::c_int - idx + 1 as libc::c_int) * 2 as libc::c_int
                        + 1 as libc::c_int) as isize,
                ) = (tv[(i + 1 as libc::c_int) as usize] as libc::c_int
                % 256 as libc::c_int) as uchar;
            i -= 1;
            i;
        }
    } else {
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            *val
                .offset(
                    (i * 2 as libc::c_int) as isize,
                ) = (tv[i as usize] as libc::c_int / 256 as libc::c_int) as uchar;
            *val
                .offset(
                    (i * 2 as libc::c_int + 1 as libc::c_int) as isize,
                ) = (tv[i as usize] as libc::c_int % 256 as libc::c_int) as uchar;
            i += 1;
            i;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fix_tail(mut domain: *mut libc::c_char) -> libc::c_int {
    let mut len: libc::c_int = strlen(domain) as libc::c_int;
    let mut c: uchar = 0;
    len -= 1;
    len;
    c = *domain.offset(len as isize) as uchar;
    if c as libc::c_int == '\r' as i32 || c as libc::c_int == '\n' as i32 {
        *domain.offset(len as isize) = 0 as libc::c_int as libc::c_char;
        len -= 1;
        len;
    }
    c = *domain.offset(len as isize) as uchar;
    if c as libc::c_int == '\r' as i32 || c as libc::c_int == '\n' as i32 {
        *domain.offset(len as isize) = 0 as libc::c_int as libc::c_char;
        len -= 1;
        len;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn opr_bit(
    mut bit: *mut libc::c_ushort,
    mut off: libc::c_int,
    mut set: libc::c_int,
) -> libc::c_int {
    let mut mask: libc::c_ushort = 1 as libc::c_int as libc::c_ushort;
    if off > 15 as libc::c_int || off < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    mask = ((mask as libc::c_int) << off) as libc::c_ushort;
    if set == 0 as libc::c_int {
        mask = !(mask as libc::c_int) as libc::c_ushort;
    }
    if set == 0 as libc::c_int {
        *bit = (*bit as libc::c_int & mask as libc::c_int) as libc::c_ushort;
    } else {
        *bit = (*bit as libc::c_int | mask as libc::c_int) as libc::c_ushort;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn set_bit(
    mut bit: *mut libc::c_ushort,
    mut off: libc::c_int,
) -> libc::c_int {
    opr_bit(bit, off, 1 as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn clr_bit(
    mut bit: *mut libc::c_ushort,
    mut off: libc::c_int,
) -> libc::c_int {
    opr_bit(bit, off, 0 as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tst_bit(
    bit: libc::c_ushort,
    mut off: libc::c_int,
) -> libc::c_int {
    let mut mask: libc::c_ushort = 1 as libc::c_int as libc::c_ushort;
    if off > 15 as libc::c_int || off < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    mask = ((mask as libc::c_int) << off) as libc::c_ushort;
    if bit as libc::c_int & mask as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn get_time_usage(
    mut tv: *mut timeval,
    mut start: libc::c_int,
) -> libc::c_int {
    let mut msec: ulong = 0 as libc::c_int as ulong;
    let mut tmp: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    if tv.is_null() {
        return -(1 as libc::c_int);
    }
    if start == 1 as libc::c_int {
        gettimeofday(tv, 0 as *mut libc::c_void);
    } else {
        tmp = *tv;
        gettimeofday(tv, 0 as *mut libc::c_void);
        if (*tv).tv_usec < tmp.tv_usec {
            msec = (((*tv).tv_usec - tmp.tv_usec
                + 1000000 as libc::c_int as libc::c_long)
                / 1000 as libc::c_int as libc::c_long) as ulong;
            (*tv).tv_sec -= 1;
            (*tv).tv_sec;
        }
        printf(
            b"%lu s,%lu ms\n\0" as *const u8 as *const libc::c_char,
            (*tv).tv_sec - tmp.tv_sec,
            msec,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn is_uppercase(mut c: libc::c_int) -> libc::c_int {
    if c >= 'A' as i32 && c <= 'Z' as i32 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn is_lowercase(mut c: libc::c_int) -> libc::c_int {
    if c >= 'a' as i32 && c <= 'z' as i32 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn empty_function(mut i: libc::c_int) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn insert_mem_bar() {
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = (random() % 99 as libc::c_int as libc::c_long
        + 10000 as libc::c_int as libc::c_long) as libc::c_int;
    let mut ptr: *mut uchar = malloc(size as libc::c_ulong) as *mut uchar;
    if ptr.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < size {
        *ptr.offset(i as isize) = i as uchar;
        i += 1;
        i;
    }
    free(ptr as *mut libc::c_void);
}
pub unsafe extern "C" fn test_lock(mut l: *mut pthread_spinlock_t) -> libc::c_int {
    if pthread_spin_trylock(l) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    pthread_spin_unlock(l);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dbg_print_bit(mut bit: libc::c_ushort) {
    let mut i: libc::c_int = 0;
    let mut val: libc::c_ushort = ((1 as libc::c_int) << 15 as libc::c_int)
        as libc::c_ushort;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if bit as libc::c_int & val as libc::c_int == 0 as libc::c_int {
            printf(b"0\0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"1\0" as *const u8 as *const libc::c_char);
        }
        if (i + 1 as libc::c_int) % 4 as libc::c_int == 0 as libc::c_int
            && i != 15 as libc::c_int
        {
            printf(b",\0" as *const u8 as *const libc::c_char);
        }
        val = (val as libc::c_int >> 1 as libc::c_int) as libc::c_ushort;
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn print_hex(mut val: *mut uchar, mut n: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        printf(
            b"%x,\0" as *const u8 as *const libc::c_char,
            *val.offset(i as isize) as libc::c_int,
        );
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn dns_error(mut level: libc::c_int, mut msg: *mut libc::c_char) {
    dbg(b"Error:%s\n\0" as *const u8 as *const libc::c_char, msg);
    fflush(stdout);
    if level == 0 as libc::c_int {
        exit(-(1 as libc::c_int));
    }
}
pub unsafe extern "C" fn dbg(
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    printf(b"dbg:\0" as *const u8 as *const libc::c_char);
    ret = vprintf(format, ap.as_va_list());
    return ret;
}
pub unsafe extern "C" fn uint_hash_function(mut ptr: *mut libc::c_void) -> hashval_t {
    let mut key: uint = *(ptr as *mut uint);
    key = (key as libc::c_uint).wrapping_add(!(key << 15 as libc::c_int)) as uint
        as uint;
    key ^= key >> 10 as libc::c_int;
    key = (key as libc::c_uint).wrapping_add(key << 3 as libc::c_int) as uint as uint;
    key ^= key >> 6 as libc::c_int;
    key = (key as libc::c_uint).wrapping_add(!(key << 11 as libc::c_int)) as uint
        as uint;
    key ^= key >> 16 as libc::c_int;
    return key;
}
pub unsafe extern "C" fn nocase_char_hash_function(
    mut argv: *mut libc::c_void,
    mut klen: libc::c_int,
) -> hashval_t {
    let mut len: libc::c_int = if klen == 2 as libc::c_int {
        1 as libc::c_int
    } else {
        klen
    };
    let mut buf: *mut uchar = argv as *mut uchar;
    let mut hash: hashval_t = 5381 as libc::c_int as hashval_t;
    loop {
        let fresh4 = len;
        len = len - 1;
        if !(fresh4 != 0) {
            break;
        }
        let fresh5 = buf;
        buf = buf.offset(1);
        hash = (hash << 5 as libc::c_int)
            .wrapping_add(hash)
            .wrapping_add(*fresh5 as libc::c_uint);
    }
    return hash;
}
