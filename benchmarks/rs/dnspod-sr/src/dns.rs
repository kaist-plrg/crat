use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn make_bin_from_str(bin: *mut uchar, str: *const libc::c_char) -> libc::c_int;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn udp_write_info(mbuf: *mut mbuf_type, _: libc::c_int) -> libc::c_int;
    fn pthread_spin_lock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    fn pthread_spin_unlock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    static mut global_now: time_t;
    fn dict_comp_str_equ(a: *mut libc::c_void, b: *mut libc::c_void) -> libc::c_int;
    fn to_lowercase(buf: *mut uchar, n: libc::c_int) -> libc::c_int;
    fn delete_node(rbt: *mut rbtree, nd: *mut rbnode) -> *mut libc::c_void;
    fn insert_node(rbt: *mut rbtree, nd: *mut rbnode) -> libc::c_int;
    fn find_node(rbt: *mut rbtree, key: *mut libc::c_void) -> *mut rbnode;
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn htable_insert(
        _: *mut htable,
        _: *mut uchar,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut uchar,
        _: libc::c_int,
        _: *mut mvalue,
        hashd: *mut hashval_t,
    ) -> libc::c_int;
    fn htable_find(
        ht: *mut htable,
        key: *mut uchar,
        klen: libc::c_int,
        type_0: libc::c_int,
        buffer: *mut uchar,
        vlen: libc::c_int,
        metadata: *mut mvalue,
        hashd: *mut hashval_t,
    ) -> libc::c_int;
    fn get_pre_mem_hash(
        _: *mut libc::c_void,
        klen: libc::c_int,
        hashd: *mut hashval_t,
    ) -> uint;
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
    static mut g_nameservers: [*mut libc::c_char; 0];
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type ushort = libc::c_ushort;
pub type uint = libc::c_uint;
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
pub type C2RustUnnamed = libc::c_uint;
pub const MAX_MSG_SIZE: C2RustUnnamed = 2048;
pub const MAX_MSG_SEG: C2RustUnnamed = 32;
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const REFUSED: C2RustUnnamed_1 = 5;
pub const NOT_IMPL: C2RustUnnamed_1 = 4;
pub const NAME_ERROR: C2RustUnnamed_1 = 3;
pub const SERVER_FAIL: C2RustUnnamed_1 = 2;
pub const FORMAT_ERROR: C2RustUnnamed_1 = 1;
pub const NO_ERROR: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const IP_DATA_LEN: C2RustUnnamed_2 = 2000;
pub const MAX_TRY_TIMES: C2RustUnnamed_2 = 15;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const CLASS_IN: C2RustUnnamed_3 = 1;
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
#[repr(C)]
pub struct hlpf {
    pub type_0: ushort,
    pub len: ushort,
    pub ttl: uint,
    pub hdr: *mut uchar,
    pub from: *mut uchar,
    pub to: *mut uchar,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct fillmsg {
    pub type_0: uint16_t,
    pub dclass: uint16_t,
    pub ttl: uint32_t,
    pub len: uint16_t,
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct srv {
    pub pri: uint16_t,
    pub wei: uint16_t,
    pub port: uint16_t,
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
pub type C2RustUnnamed_4 = libc::c_uint;
pub const Q_NS: C2RustUnnamed_4 = 6;
pub const Q_DOMAIN: C2RustUnnamed_4 = 4;
pub const Q_CNAME: C2RustUnnamed_4 = 3;
pub static mut support_type: [rrtype; 9] = [A, NS, CNAME, SOA, MX, TXT, AAAA, SRV, PTR];
pub unsafe extern "C" fn str_to_len_label(
    mut domain: *mut uchar,
    mut len: libc::c_int,
) -> *mut uchar {
    let mut l: uchar = 0 as libc::c_int as uchar;
    let mut i: libc::c_int = 0;
    if *domain.offset((len - 1 as libc::c_int) as isize) as libc::c_int
        != 0 as libc::c_int
        || *domain.offset((len - 2 as libc::c_int) as isize) as libc::c_int != '.' as i32
    {
        return 0 as *mut uchar;
    }
    i = len - 2 as libc::c_int;
    while i > 0 as libc::c_int {
        *domain.offset(i as isize) = *domain.offset((i - 1 as libc::c_int) as isize);
        l = l.wrapping_add(1);
        l;
        if *domain.offset(i as isize) as libc::c_int == '.' as i32 {
            *domain.offset(i as isize) = (l as libc::c_int - 1 as libc::c_int) as uchar;
            l = 0 as libc::c_int as uchar;
        }
        i -= 1;
        i;
    }
    *domain.offset(0 as libc::c_int as isize) = l;
    return domain;
}
pub static mut SupportTypeTable: [libc::c_uchar; 256] = [
    0 as libc::c_int as libc::c_uchar,
    A as libc::c_int as libc::c_uchar,
    NS as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    CNAME as libc::c_int as libc::c_uchar,
    SOA as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    PTR as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    MX as libc::c_int as libc::c_uchar,
    TXT as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    AAAA as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    SRV as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
pub unsafe extern "C" fn check_support_type(mut type_0: ushort) -> libc::c_int {
    if type_0 as libc::c_int > 0xff as libc::c_int {
        return -(1 as libc::c_int);
    }
    if SupportTypeTable[type_0 as libc::c_uchar as usize] as libc::c_int
        != 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn passer_dns_data(mut mbuf: *mut mbuf_type) {
    let mut buf: *mut uchar = (*mbuf).buf;
    let mut num: libc::c_int = 0;
    let mut dlen: libc::c_int = 0 as libc::c_int;
    let mut tail: *mut uchar = 0 as *mut uchar;
    let mut hdr: *mut dnsheader = buf as *mut dnsheader;
    (*mbuf).err = 1 as libc::c_int;
    num = ntohs((*hdr).qdcount) as libc::c_int;
    if num != 1 as libc::c_int {
        return;
    }
    num = ntohs((*hdr).ancount) as libc::c_int;
    if num != 0 as libc::c_int {
        return;
    }
    num = ntohs((*hdr).nscount) as libc::c_int;
    if num != 0 as libc::c_int {
        return;
    }
    num = ntohs((*hdr).arcount) as libc::c_int;
    if num > 1 as libc::c_int {
        return;
    }
    (*mbuf).id = (*hdr).id;
    dlen = check_dns_name(
        buf.offset(::std::mem::size_of::<dnsheader>() as libc::c_ulong as isize),
        &mut (*mbuf).lowerdomain,
    );
    if dlen < 0 as libc::c_int {
        return;
    }
    (*mbuf).dlen = dlen;
    (*mbuf)
        .origindomain = buf
        .offset(::std::mem::size_of::<dnsheader>() as libc::c_ulong as isize);
    tail = (*mbuf).origindomain;
    if dlen == 2 as libc::c_int {
        tail = tail.offset(1 as libc::c_int as isize);
    } else {
        tail = tail.offset(dlen as isize);
    }
    (*mbuf).qtype = ntohs(*(tail as *mut ushort)) as rrtype;
    if check_support_type((*mbuf).qtype as ushort) == 0 as libc::c_int {
        (*mbuf).err = 0 as libc::c_int;
    }
}
pub unsafe extern "C" fn send_tc_to_client(mut mbuf: *mut mbuf_type) -> libc::c_int {
    let mut itor: *mut uchar = (*mbuf).buf;
    let mut hdr: *mut dnsheader = itor as *mut dnsheader;
    let mut qd: *mut qdns = 0 as *mut qdns;
    if ((*mbuf).td).is_null() {
        return -(1 as libc::c_int);
    }
    (*hdr).id = (*mbuf).id;
    (*hdr).flags = 0 as libc::c_int as uint16_t;
    (*hdr).flags = ((*hdr).flags as libc::c_int | 0x8000 as libc::c_int) as uint16_t;
    (*hdr).flags = ((*hdr).flags as libc::c_int | 0x80 as libc::c_int) as uint16_t;
    (*hdr).flags = ((*hdr).flags as libc::c_int | 0x200 as libc::c_int) as uint16_t;
    (*hdr).flags = htons((*hdr).flags);
    (*hdr).qdcount = htons(1 as libc::c_int as uint16_t);
    (*hdr).arcount = htons(0 as libc::c_int as uint16_t);
    (*hdr).nscount = (*hdr).arcount;
    (*hdr).ancount = (*hdr).nscount;
    itor = itor.offset(::std::mem::size_of::<dnsheader>() as libc::c_ulong as isize);
    memcpy(
        itor as *mut libc::c_void,
        (*mbuf).td as *const libc::c_void,
        (*mbuf).dlen as libc::c_ulong,
    );
    itor = itor.offset((*mbuf).dlen as isize);
    qd = itor as *mut qdns;
    (*qd).type_0 = htons((*mbuf).qtype as uint16_t);
    (*qd).dclass = htons(CLASS_IN as libc::c_int as uint16_t);
    itor = itor.offset(::std::mem::size_of::<qdns>() as libc::c_ulong as isize);
    (*mbuf).buflen = itor.offset_from((*mbuf).buf) as libc::c_long as libc::c_int;
    udp_write_info(mbuf, 0 as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn get_domain_from_msg(
    mut itor: *mut uchar,
    mut hdr: *mut uchar,
    mut to: *mut uchar,
    mut tmplen: *mut libc::c_int,
) -> libc::c_int {
    let mut len: uchar = 0;
    let mut offset: ushort = 0 as libc::c_int as ushort;
    len = *itor.offset(0 as libc::c_int as isize);
    let mut dlen: libc::c_int = 0 as libc::c_int;
    let mut hasptr: libc::c_int = 0 as libc::c_int;
    let mut infinite: libc::c_int = 20 as libc::c_int;
    offset = ntohs(*(itor as *mut ushort));
    *tmplen = 0 as libc::c_int;
    while len as libc::c_int != 0 as libc::c_int
        && {
            let fresh0 = infinite;
            infinite = infinite - 1;
            fresh0 != 0
        }
    {
        if offset as libc::c_int >= 0xc000 as libc::c_int
            && offset as libc::c_int <= 0xcfff as libc::c_int
        {
            itor = hdr.offset((offset as libc::c_int & 0x3fff as libc::c_int) as isize);
            if hasptr == 0 as libc::c_int {
                dlen = 2 as libc::c_int;
                if *tmplen != 0 as libc::c_int {
                    dlen += *tmplen;
                }
            }
            hasptr = 1 as libc::c_int;
            offset = ntohs(*(itor as *mut ushort));
        } else {
            *to
                .offset(
                    0 as libc::c_int as isize,
                ) = *itor.offset(0 as libc::c_int as isize);
            *tmplen += 1 as libc::c_int;
            *tmplen += *to.offset(0 as libc::c_int as isize) as libc::c_int;
            if *to.offset(0 as libc::c_int as isize) as libc::c_int > 64 as libc::c_int {
                return -(1 as libc::c_int);
            }
            to = to.offset(1);
            to;
            memcpy(
                to as *mut libc::c_void,
                itor.offset(1 as libc::c_int as isize) as *const libc::c_void,
                *itor.offset(0 as libc::c_int as isize) as libc::c_ulong,
            );
            to = to
                .offset(*itor.offset(0 as libc::c_int as isize) as libc::c_int as isize);
            itor = itor
                .offset(*itor.offset(0 as libc::c_int as isize) as libc::c_int as isize)
                .offset(1 as libc::c_int as isize);
            len = *itor.offset(0 as libc::c_int as isize);
            offset = ntohs(*(itor as *mut ushort));
        }
    }
    if infinite <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    *to.offset(0 as libc::c_int as isize) = 0 as libc::c_int as uchar;
    to = to.offset(1);
    to;
    *tmplen += 1;
    *tmplen;
    if dlen == 0 as libc::c_int {
        dlen = *tmplen;
    }
    if dlen > 255 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return dlen;
}
pub unsafe extern "C" fn insert_into_ttltree(
    mut rbt: *mut rbtree,
    mut td: *mut uchar,
    mut len: libc::c_int,
    mut type_0: libc::c_int,
    mut ttl: uint,
    mut lowerdomain: *mut packet_type,
) -> libc::c_int {
    let mut node: rbnode = {
        let mut init = rbnode {
            parent: 0 as *mut rbnode,
            left: 0 as *mut rbnode,
            right: 0 as *mut rbnode,
            color: 0,
            key: 0 as *mut libc::c_void,
        };
        init
    };
    let mut tn: *mut ttlnode = 0 as *mut ttlnode;
    tn = malloc(::std::mem::size_of::<ttlnode>() as libc::c_ulong) as *mut ttlnode;
    if tn.is_null() {
        return -(1 as libc::c_int);
    }
    (*tn)
        .lowerdomain = malloc(::std::mem::size_of::<packet_type>() as libc::c_ulong)
        as *mut packet_type;
    if ((*tn).lowerdomain).is_null() {
        free(tn as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    (*tn).dlen = len as ushort;
    (*tn).exp = ttl;
    (*tn).type_0 = type_0 as ushort;
    (*tn)
        .hash = &mut *((*(*tn).lowerdomain).hash)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut hashval_t;
    memcpy(
        (*tn).lowerdomain as *mut libc::c_void,
        lowerdomain as *const libc::c_void,
        ::std::mem::size_of::<packet_type>() as libc::c_ulong,
    );
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*(*tn).lowerdomain).label_count as libc::c_int {
        (*(*tn).lowerdomain)
            .label[i
            as usize] = ((*(*tn).lowerdomain).domain)
            .as_mut_ptr()
            .offset(
                (*(*tn).lowerdomain).label_offsets[i as usize] as libc::c_int as isize,
            );
        i += 1;
        i;
    }
    (*tn).data = ((*(*tn).lowerdomain).domain).as_mut_ptr();
    node.key = tn as *mut libc::c_void;
    insert_node(rbt, &mut node);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn random_ttl(mut ttl: uint) -> uint {
    let mut ret: uint = ttl.wrapping_rem(7 as libc::c_int as libc::c_uint);
    ttl = ttl.wrapping_add(ret.wrapping_mul(3 as libc::c_int as libc::c_uint));
    if ttl > (7 as libc::c_int * 86400 as libc::c_int) as libc::c_uint {
        ttl = ((7 as libc::c_int * 86400 as libc::c_int) as libc::c_uint)
            .wrapping_sub(
                ttl
                    .wrapping_rem(
                        (7 as libc::c_int * 86400 as libc::c_int) as libc::c_uint,
                    ),
            );
    }
    return ttl;
}
pub unsafe extern "C" fn is_parent(
    mut parent: *mut uchar,
    mut son: *mut uchar,
) -> libc::c_int {
    let mut sp: libc::c_int = 0;
    let mut ss: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    sp = strlen(parent as *const libc::c_char) as libc::c_int;
    ss = strlen(son as *const libc::c_char) as libc::c_int;
    if ss < sp {
        return -(1 as libc::c_int);
    }
    x = ss - sp;
    son = son.offset(x as isize);
    if strcmp(parent as *const libc::c_char, son as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn check_dms(
    mut ck: *mut uchar,
    mut dms: *mut uchar,
    mut num: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn process_rdata(
    mut hlp: *mut hlpp,
    mut label: *mut uchar,
    mut n: libc::c_int,
) -> *mut uchar {
    let mut buffer: *mut uchar = (*hlp).tmpbuf;
    let mut type_0: ushort = 0 as libc::c_int as ushort;
    let mut classin: ushort = 0;
    let mut lth: ushort = 0;
    let mut tmptype: ushort = 0 as libc::c_int as ushort;
    let mut ttl: uint = 0 as libc::c_int as uint;
    let mut tmpttl: uint = 0 as libc::c_int as uint;
    let mut tx: uint = 0;
    let mut i: libc::c_int = 0;
    let mut dlen: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut tmplen: libc::c_int = 0 as libc::c_int;
    let mut stype: *mut libc::c_int = (*hlp).stype;
    let mut ds: *mut htable = (*hlp).ds;
    let mut rbt: *mut rbtree = (*hlp).rbt;
    let mut hdr: *mut uchar = (*hlp).buf;
    let mut mlen: libc::c_int = (*hlp).datalen;
    let mut mv: *mut mvalue = buffer as *mut mvalue;
    let mut tmpdomain: *mut uchar = (*hlp).domainbuf;
    let mut dm: *mut uchar = 0 as *mut uchar;
    let mut itor: *mut uchar = 0 as *mut uchar;
    let mut lowerdomain: packet_type = packet_type {
        label_count: 0,
        domain: [0; 256],
        label: [0 as *mut uint8_t; 64],
        label_offsets: [0; 64],
        label_len: [0; 64],
        hash: [0; 64],
    };
    dm = (lowerdomain.domain).as_mut_ptr();
    memset(
        mv as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mvalue>() as libc::c_ulong,
    );
    itor = buffer.offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
    tx = global_now as uint;
    let ref mut fresh1 = *dm.offset(1 as libc::c_int as isize);
    *fresh1 = 0 as libc::c_int as uchar;
    *dm.offset(0 as libc::c_int as isize) = *fresh1;
    i = 0 as libc::c_int;
    while i < n {
        dlen = get_domain_from_msg(label, hdr, tmpdomain, &mut tmplen);
        if *dm.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            && *dm.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        {
            check_dns_name(tmpdomain, &mut lowerdomain);
        }
        if dlen < 0 as libc::c_int {
            return 0 as *mut uchar;
        }
        label = label.offset(dlen as isize);
        if get_dns_info(label, &mut tmptype, &mut classin, &mut ttl, &mut lth)
            < 0 as libc::c_int
        {
            return 0 as *mut uchar;
        }
        if ttl < 10 as libc::c_int as libc::c_uint {
            ttl = 10 as libc::c_int as uint;
        }
        ttl = random_ttl(ttl.wrapping_add(n as libc::c_uint));
        label = label.offset(10 as libc::c_int as isize);
        if (tmptype as libc::c_int == SOA as libc::c_int
            || tmptype as libc::c_int == CNAME as libc::c_int)
            && i == n - 1 as libc::c_int
        {
            *stype = tmptype as libc::c_int;
        }
        if type_0 as libc::c_int == 0 as libc::c_int {
            type_0 = tmptype;
        }
        if ttl > (7 as libc::c_int * 86400 as libc::c_int) as libc::c_uint {
            ttl = (7 as libc::c_int * 86400 as libc::c_int) as uint;
        }
        if tmpttl == 0 as libc::c_int as libc::c_uint {
            tmpttl = ttl;
        }
        if dict_comp_str_equ(tmpdomain as *mut libc::c_void, dm as *mut libc::c_void)
            != 0 as libc::c_int || type_0 as libc::c_int != tmptype as libc::c_int
        {
            (*mv)
                .ttl = (random_ttl(
                tmpttl
                    .wrapping_add(i as libc::c_uint)
                    .wrapping_add(tx.wrapping_rem(5 as libc::c_int as libc::c_uint)),
            ))
                .wrapping_add(tx);
            if *dm
                .offset(
                    (*dm.offset(0 as libc::c_int as isize) as libc::c_int
                        + 2 as libc::c_int) as isize,
                ) as libc::c_int != 0 as libc::c_int
            {
                insert_kv_mem(
                    rbt,
                    ds,
                    dm,
                    lowerdomain.label_len[0 as libc::c_int as usize] as libc::c_int,
                    type_0 as libc::c_int,
                    buffer,
                    ((*mv).len as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<mvalue>() as libc::c_ulong)
                        as libc::c_int,
                    0 as libc::c_int,
                    &mut lowerdomain,
                );
            }
            type_0 = tmptype;
            check_dns_name(tmpdomain, &mut lowerdomain);
            memset(
                mv as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<mvalue>() as libc::c_ulong,
            );
            itor = buffer
                .offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
        }
        ret = fill_rrset_in_buffer(
            itor,
            label,
            hdr,
            lth as libc::c_int,
            type_0 as libc::c_int,
            hlp,
        );
        if ret > 0 as libc::c_int {
            itor = itor.offset(ret as isize);
            (*mv).len = ((*mv).len as libc::c_int + ret) as uint16_t;
            (*mv).num = ((*mv).num).wrapping_add(1);
            (*mv).num;
        }
        tmpttl = ttl;
        label = label.offset(lth as libc::c_int as isize);
        if label < hdr || label > hdr.offset(mlen as isize) {
            return 0 as *mut uchar;
        }
        i += 1;
        i;
    }
    if (*mv).num as libc::c_int > 0 as libc::c_int {
        (*mv)
            .ttl = (random_ttl(
            tmpttl
                .wrapping_add(i as libc::c_uint)
                .wrapping_add(tx.wrapping_rem(5 as libc::c_int as libc::c_uint)),
        ))
            .wrapping_add(tx);
        (*mv).hits = 0 as libc::c_int as uint32_t;
        (*mv).seg = 0 as libc::c_int as uint16_t;
        if *dm
            .offset(
                (*dm.offset(0 as libc::c_int as isize) as libc::c_int + 2 as libc::c_int)
                    as isize,
            ) as libc::c_int != 0 as libc::c_int
        {
            insert_kv_mem(
                rbt,
                ds,
                dm,
                lowerdomain.label_len[0 as libc::c_int as usize] as libc::c_int,
                type_0 as libc::c_int,
                buffer,
                ((*mv).len as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<mvalue>() as libc::c_ulong)
                    as libc::c_int,
                0 as libc::c_int,
                &mut lowerdomain,
            );
        }
    }
    return label;
}
pub unsafe extern "C" fn check_domain_mask(
    mut domain: *mut uchar,
    mut origin: *mut uchar,
    mut len: libc::c_int,
) -> libc::c_int {
    return strncmp(
        origin as *const libc::c_char,
        domain as *const libc::c_char,
        len as libc::c_ulong,
    );
}
pub unsafe extern "C" fn get_dns_info(
    mut label: *mut uchar,
    mut tp: *mut ushort,
    mut cls: *mut ushort,
    mut ttl: *mut uint,
    mut lth: *mut ushort,
) -> libc::c_int {
    let mut us: *mut ushort = 0 as *mut ushort;
    let mut ui: *mut uint = 0 as *mut uint;
    us = label as *mut ushort;
    *tp = ntohs(*us);
    if *tp as libc::c_int > 254 as libc::c_int {
        printf(
            b"type is %u\n\0" as *const u8 as *const libc::c_char,
            *tp as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    label = label.offset(::std::mem::size_of::<ushort>() as libc::c_ulong as isize);
    us = label as *mut ushort;
    *cls = ntohs(*us);
    if *cls as libc::c_int != CLASS_IN as libc::c_int {
        return -(1 as libc::c_int);
    }
    label = label.offset(::std::mem::size_of::<ushort>() as libc::c_ulong as isize);
    ui = label as *mut uint;
    *ttl = ntohl(*ui);
    label = label.offset(::std::mem::size_of::<uint>() as libc::c_ulong as isize);
    us = label as *mut ushort;
    *lth = ntohs(*us);
    return 0 as libc::c_int;
}
pub static mut DnsNameTable: [libc::c_uchar; 256] = [
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
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
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
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
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
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
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
pub static mut InvalidDnsNameTable: [libc::c_uchar; 256] = [
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
];
pub unsafe extern "C" fn check_dns_name(
    mut domain: *mut uchar,
    mut lowerdomain: *mut packet_type,
) -> libc::c_int {
    let mut len: uchar = *domain.offset(0 as libc::c_int as isize);
    let mut i: uchar = 0;
    let mut tlen: libc::c_int = 0 as libc::c_int;
    let mut dst: *mut uchar = ((*lowerdomain).domain).as_mut_ptr();
    let mut hash: *mut hashval_t = &mut *((*lowerdomain).hash)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut hashval_t;
    (*lowerdomain).label_count = 0 as libc::c_int as uint8_t;
    (*lowerdomain).label[(*lowerdomain).label_count as usize] = dst;
    (*lowerdomain)
        .label_offsets[(*lowerdomain).label_count
        as usize] = 0 as libc::c_int as uint8_t;
    (*lowerdomain).hash[0 as libc::c_int as usize] = 5381 as libc::c_int as hashval_t;
    *dst = len;
    let fresh2 = dst;
    dst = dst.offset(1);
    *hash = (*hash << 5 as libc::c_int)
        .wrapping_add(*hash)
        .wrapping_add(*fresh2 as libc::c_uint);
    domain = domain.offset(1);
    domain;
    if len as libc::c_int == 0 as libc::c_int {
        *dst = '\0' as i32 as uchar;
        tlen = 2 as libc::c_int;
        return tlen;
    }
    while len as libc::c_int != 0 as libc::c_int {
        if len as libc::c_int > 63 as libc::c_int {
            return -(1 as libc::c_int);
        }
        i = 0 as libc::c_int as uchar;
        while (i as libc::c_int) < len as libc::c_int {
            *dst = DnsNameTable[*domain.offset(i as isize) as usize];
            if *dst == 0 {
                return -(1 as libc::c_int);
            }
            *hash = (*hash << 5 as libc::c_int)
                .wrapping_add(*hash)
                .wrapping_add(*dst as libc::c_uint);
            dst = dst.offset(1);
            dst;
            i = i.wrapping_add(1);
            i;
        }
        domain = domain.offset(len as libc::c_int as isize);
        len = *domain.offset(0 as libc::c_int as isize);
        (*lowerdomain).label_count = ((*lowerdomain).label_count).wrapping_add(1);
        (*lowerdomain).label_count;
        (*lowerdomain).label[(*lowerdomain).label_count as usize] = dst;
        (*lowerdomain)
            .label_offsets[(*lowerdomain).label_count
            as usize] = dst.offset_from(((*lowerdomain).domain).as_mut_ptr())
            as libc::c_long as uint8_t;
        (*lowerdomain)
            .hash[(*lowerdomain).label_count as usize] = 0 as libc::c_int as hashval_t;
        *dst = len;
        let fresh3 = dst;
        dst = dst.offset(1);
        *hash = (*hash << 5 as libc::c_int)
            .wrapping_add(*hash)
            .wrapping_add(*fresh3 as libc::c_uint);
        domain = domain.offset(1);
        domain;
    }
    i = 0 as libc::c_int as uchar;
    while (i as libc::c_int) < (*lowerdomain).label_count as libc::c_int {
        (*lowerdomain)
            .label_len[i
            as usize] = dst.offset_from((*lowerdomain).label[i as usize]) as libc::c_long
            as uint8_t;
        i = i.wrapping_add(1);
        i;
    }
    tlen = (*lowerdomain).label_len[0 as libc::c_int as usize] as libc::c_int;
    if tlen > 255 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return tlen;
}
pub unsafe extern "C" fn make_type_domain(
    mut domain: *mut uchar,
    mut dlen: libc::c_int,
    mut type_0: libc::c_int,
    mut buffer: *mut uchar,
) -> libc::c_int {
    if buffer.is_null() || domain.is_null() {
        return -(1 as libc::c_int);
    }
    *buffer.offset(0 as libc::c_int as isize) = type_0 as uchar;
    memcpy(
        buffer.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        domain as *const libc::c_void,
        dlen as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn check_memcpy(
    mut to: *mut uchar,
    mut from: *mut uchar,
    mut vlen: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < vlen {
        if *to.offset(i as isize) as libc::c_int
            != *from.offset(i as isize) as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn insert_kv_mem(
    mut rbt: *mut rbtree,
    mut ds: *mut htable,
    mut k: *mut uchar,
    mut klen: libc::c_int,
    mut type_0: libc::c_int,
    mut v: *mut uchar,
    mut vlen: libc::c_int,
    mut hijack: libc::c_int,
    mut lowerdomain: *mut packet_type,
) -> libc::c_int {
    let mut val: *mut uchar = 0 as *mut uchar;
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    let mut tmp: mvalue = {
        let mut init = mvalue {
            len: 0 as libc::c_int as uint16_t,
            num: 0,
            ttl: 0,
            hits: 0,
            seg: 0,
        };
        init
    };
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut pn: *mut rbnode = 0 as *mut rbnode;
    let mut tn: ttlnode = {
        let mut init = ttlnode {
            exp: 0 as libc::c_int as uint,
            dlen: 0,
            type_0: 0,
            hash: 0 as *mut hashval_t,
            lowerdomain: 0 as *mut packet_type,
            data: 0 as *mut uchar,
        };
        init
    };
    let mut tmp_tn: *mut ttlnode = 0 as *mut ttlnode;
    let mut idx: libc::c_int = 0;
    if vlen < 0 as libc::c_int || vlen > 4096 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let mut hash: *mut hashval_t = &mut *((*lowerdomain).hash)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut hashval_t;
    idx = get_pre_mem_hash(k as *mut libc::c_void, klen, hash) as libc::c_int;
    val = malloc(vlen as libc::c_ulong) as *mut uchar;
    if val.is_null() {
        return -(1 as libc::c_int);
    }
    memcpy(val as *mut libc::c_void, v as *const libc::c_void, vlen as libc::c_ulong);
    mv = v as *mut mvalue;
    ret = htable_insert(
        ds.offset(idx as isize),
        k,
        klen,
        type_0,
        val,
        1 as libc::c_int,
        &mut tmp,
        hash,
    );
    if ret >= HTABLE_INSERT_RET_NEVER_EXPIRE as libc::c_int {
        free(val as *mut libc::c_void);
    }
    if !rbt.is_null() {
        if ret == HTABLE_INSERT_RET_REPLACE as libc::c_int {
            pthread_spin_lock(&mut (*rbt).lock);
            tn.dlen = klen as ushort;
            tn.exp = tmp.ttl;
            tn.type_0 = type_0 as ushort;
            tn.lowerdomain = 0 as *mut packet_type;
            tn.data = k;
            pn = find_node(rbt, &mut tn as *mut ttlnode as *mut libc::c_void);
            if !pn.is_null() {
                tmp_tn = delete_node(rbt, pn) as *mut ttlnode;
                if !tmp_tn.is_null() {
                    free((*tmp_tn).lowerdomain as *mut libc::c_void);
                    free(tmp_tn as *mut libc::c_void);
                }
            }
            pthread_spin_unlock(&mut (*rbt).lock);
        }
    }
    if (*mv).ttl
        == (7 as libc::c_int * 86400 as libc::c_int + 1 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if rbt.is_null() {
        return 0 as libc::c_int;
    }
    pthread_spin_lock(&mut (*rbt).lock);
    if type_0 != NS as libc::c_int && ret == HTABLE_INSERT_RET_REPLACE as libc::c_int {
        ret = insert_into_ttltree(rbt, k, klen, type_0, (*mv).ttl, lowerdomain);
    } else {
        ret = insert_into_ttltree(rbt, k, klen, type_0, tmp.ttl, lowerdomain);
    }
    pthread_spin_unlock(&mut (*rbt).lock);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn get_level(mut itor: *mut uchar) -> libc::c_int {
    let mut lvl: libc::c_int = 0 as libc::c_int;
    let mut len: uchar = *itor.offset(0 as libc::c_int as isize);
    while len as libc::c_int != 0 as libc::c_int {
        lvl += 1;
        lvl;
        itor = itor
            .offset(
                (*itor.offset(0 as libc::c_int as isize) as libc::c_int
                    + 1 as libc::c_int) as isize,
            );
        len = *itor.offset(0 as libc::c_int as isize);
        if len as libc::c_int > 63 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return lvl;
}
pub unsafe extern "C" fn fill_all_records_in_msg(
    mut h: *mut hlpc,
    mut hf: *mut hlpf,
    mut pidx: *mut libc::c_int,
) -> *mut uchar {
    let mut step: libc::c_int = 0 as libc::c_int;
    let mut txtlen: uint16_t = 0;
    let mut tmp: *mut uchar = 0 as *mut uchar;
    let mut to: *mut uchar = (*hf).to;
    let mut from: *mut uchar = (*hf).from;
    let mut fm: *mut fillmsg = (*hf).to as *mut fillmsg;
    let mut idx: libc::c_int = *pidx;
    (*fm).type_0 = htons((*hf).type_0);
    (*fm).dclass = htons(CLASS_IN as libc::c_int as uint16_t);
    (*fm).ttl = htonl(((*hf).ttl as libc::c_long - global_now) as uint32_t);
    if (*hf).ttl
        == (7 as libc::c_int * 86400 as libc::c_int + 1 as libc::c_int) as libc::c_uint
    {
        (*fm).ttl = htonl(((*hf).ttl).wrapping_sub(1 as libc::c_int as libc::c_uint));
    }
    to = to.offset(::std::mem::size_of::<fillmsg>() as libc::c_ulong as isize);
    if (*hf).type_0 as libc::c_int == A as libc::c_int {
        step = 4 as libc::c_int;
    }
    if (*hf).type_0 as libc::c_int == AAAA as libc::c_int {
        step = 16 as libc::c_int;
    }
    let mut current_block_56: u64;
    match (*hf).type_0 as libc::c_int {
        1 => {
            current_block_56 = 11459758814641233183;
        }
        28 => {
            current_block_56 = 11459758814641233183;
        }
        5 | 2 => {
            idx += 1;
            idx;
            *pidx = idx;
            let ref mut fresh4 = (*h.offset(idx as isize)).name;
            *fresh4 = from;
            (*h.offset(idx as isize))
                .off = to.offset_from((*hf).hdr) as libc::c_long as libc::c_short;
            (*h.offset(idx as isize)).ref_0 = -(1 as libc::c_int) as libc::c_short;
            (*h.offset(idx as isize))
                .level = get_level((*h.offset(idx as isize)).name) as libc::c_short;
            (*h.offset(idx as isize)).mt = 0 as libc::c_int as libc::c_short;
            (*h.offset(idx as isize)).len = (*hf).len as libc::c_short;
            tmp = fill_name_in_msg(h, to, idx);
            (*fm).len = htons(tmp.offset_from(to) as libc::c_long as uint16_t);
            to = tmp;
            current_block_56 = 12997042908615822766;
        }
        15 => {
            memcpy(
                to as *mut libc::c_void,
                from as *const libc::c_void,
                ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
            );
            from = from
                .offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
            to = to.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
            idx += 1;
            idx;
            *pidx = idx;
            let ref mut fresh5 = (*h.offset(idx as isize)).name;
            *fresh5 = from;
            (*h.offset(idx as isize))
                .off = to.offset_from((*hf).hdr) as libc::c_long as libc::c_short;
            (*h.offset(idx as isize)).ref_0 = -(1 as libc::c_int) as libc::c_short;
            (*h.offset(idx as isize))
                .level = get_level((*h.offset(idx as isize)).name) as libc::c_short;
            (*h.offset(idx as isize)).mt = 0 as libc::c_int as libc::c_short;
            (*h.offset(idx as isize)).len = (*hf).len as libc::c_short;
            tmp = fill_name_in_msg(h, to, idx);
            (*fm)
                .len = htons(
                (tmp.offset_from(to) as libc::c_long as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                    as uint16_t,
            );
            to = tmp;
            current_block_56 = 12997042908615822766;
        }
        16 => {
            txtlen = *(from as *mut uint16_t);
            from = from
                .offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
            memcpy(
                to as *mut libc::c_void,
                from as *const libc::c_void,
                txtlen as libc::c_ulong,
            );
            (*fm).len = htons(txtlen);
            to = to.offset(txtlen as libc::c_int as isize);
            current_block_56 = 12997042908615822766;
        }
        33 => {
            memcpy(
                to as *mut libc::c_void,
                from as *const libc::c_void,
                (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
            );
            from = from
                .offset(
                    (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                        .wrapping_mul(3 as libc::c_int as libc::c_ulong) as isize,
                );
            to = to
                .offset(
                    (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                        .wrapping_mul(3 as libc::c_int as libc::c_ulong) as isize,
                );
            idx += 1;
            idx;
            *pidx = idx;
            let ref mut fresh6 = (*h.offset(idx as isize)).name;
            *fresh6 = from;
            (*h.offset(idx as isize))
                .off = to.offset_from((*hf).hdr) as libc::c_long as libc::c_short;
            (*h.offset(idx as isize)).ref_0 = -(1 as libc::c_int) as libc::c_short;
            (*h.offset(idx as isize))
                .level = get_level((*h.offset(idx as isize)).name) as libc::c_short;
            (*h.offset(idx as isize)).mt = 0 as libc::c_int as libc::c_short;
            (*h.offset(idx as isize)).len = (*hf).len as libc::c_short;
            tmp = fill_name_in_msg(h, to, idx);
            (*fm)
                .len = htons(
                (tmp.offset_from(to) as libc::c_long as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                            .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                    ) as uint16_t,
            );
            to = tmp;
            current_block_56 = 12997042908615822766;
        }
        _ => {
            current_block_56 = 12997042908615822766;
        }
    }
    match current_block_56 {
        11459758814641233183 => {
            (*fm).len = htons(step as uint16_t);
            memcpy(
                to as *mut libc::c_void,
                from as *const libc::c_void,
                step as libc::c_ulong,
            );
            to = to.offset(step as isize);
        }
        _ => {}
    }
    return to;
}
pub unsafe extern "C" fn reverse_compare(
    mut from: *mut uchar,
    mut flen: libc::c_int,
    mut to: *mut uchar,
    mut tolen: libc::c_int,
) -> libc::c_int {
    let mut fi: uchar = 0;
    let mut ti: uchar = 0;
    let mut rec: uchar = 0 as libc::c_int as uchar;
    let mut match_0: libc::c_int = 0 as libc::c_int;
    flen -= 1 as libc::c_int;
    tolen -= 1 as libc::c_int;
    loop {
        flen -= 1;
        fi = *from.offset(flen as isize);
        tolen -= 1;
        ti = *to.offset(tolen as isize);
        if fi as libc::c_int != ti as libc::c_int {
            break;
        }
        rec = rec.wrapping_add(1);
        rec;
        if fi as libc::c_int == rec as libc::c_int - 1 as libc::c_int {
            match_0 += 1;
            match_0;
            rec = 0 as libc::c_int as uchar;
        }
        if !(flen != 0 && tolen != 0) {
            break;
        }
    }
    return match_0;
}
pub unsafe extern "C" fn fill_name_in_msg(
    mut h: *mut hlpc,
    mut to: *mut uchar,
    mut idx: libc::c_int,
) -> *mut uchar {
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0;
    let mut fill: libc::c_int = 0 as libc::c_int;
    let mut jump: libc::c_int = 0 as libc::c_int;
    let mut off: libc::c_int = 0 as libc::c_int;
    let base: ushort = 0xc000 as libc::c_int as ushort;
    let mut itor: *mut uchar = (*h.offset(idx as isize)).name;
    let mut dn: *mut uchar = 0 as *mut uchar;
    if idx == 0 as libc::c_int {
        *(to
            as *mut ushort) = htons(
            ((*h.offset(0 as libc::c_int as isize)).off as libc::c_int
                + base as libc::c_int) as uint16_t,
        );
        to = to.offset(::std::mem::size_of::<ushort>() as libc::c_ulong as isize);
        return to;
    }
    len = (*h.offset(idx as isize)).len as libc::c_int;
    i = idx - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        m = reverse_compare(
            (*h.offset(i as isize)).name,
            (*h.offset(i as isize)).len as libc::c_int,
            (*h.offset(idx as isize)).name,
            len,
        );
        if m > (*h.offset(idx as isize)).mt as libc::c_int {
            (*h.offset(idx as isize)).mt = m as libc::c_short;
            (*h.offset(idx as isize)).ref_0 = i as libc::c_short;
        }
        i -= 1;
        i;
    }
    if (*h.offset(idx as isize)).mt as libc::c_int >= 0 as libc::c_int {
        fill = (*h.offset(idx as isize)).level as libc::c_int
            - (*h.offset(idx as isize)).mt as libc::c_int;
    } else {
        fill = (*h.offset(idx as isize)).level as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < fill {
        memcpy(
            to as *mut libc::c_void,
            itor as *const libc::c_void,
            (*itor.offset(0 as libc::c_int as isize) as libc::c_int + 1 as libc::c_int)
                as libc::c_ulong,
        );
        to = to
            .offset(*itor.offset(0 as libc::c_int as isize) as libc::c_int as isize)
            .offset(1 as libc::c_int as isize);
        itor = itor
            .offset(*itor.offset(0 as libc::c_int as isize) as libc::c_int as isize)
            .offset(1 as libc::c_int as isize);
        i += 1;
        i;
    }
    len = 0 as libc::c_int;
    if (*h.offset(idx as isize)).ref_0 as libc::c_int >= 0 as libc::c_int {
        dn = (*h.offset((*h.offset(idx as isize)).ref_0 as isize)).name;
        jump = (*h.offset((*h.offset(idx as isize)).ref_0 as isize)).level as libc::c_int
            - (*h.offset(idx as isize)).mt as libc::c_int;
        i = 0 as libc::c_int;
        while i < jump {
            len
                += *dn.offset(0 as libc::c_int as isize) as libc::c_int
                    + 1 as libc::c_int;
            dn = dn
                .offset(
                    (*dn.offset(0 as libc::c_int as isize) as libc::c_int
                        + 1 as libc::c_int) as isize,
                );
            i += 1;
            i;
        }
        off = (*h.offset((*h.offset(idx as isize)).ref_0 as isize)).off as libc::c_int
            + len;
        *(to as *mut ushort) = htons((off + base as libc::c_int) as uint16_t);
        to = to.offset(2 as libc::c_int as isize);
    } else {
        *to.offset(0 as libc::c_int as isize) = 0 as libc::c_int as uchar;
        to = to.offset(1);
        to;
    }
    return to;
}
pub unsafe extern "C" fn fill_rrset_in_msg(
    mut h: *mut hlpc,
    mut from: *mut uchar,
    mut to: *mut uchar,
    mut pn: *mut libc::c_int,
    mut hdr: *mut uchar,
) -> *mut uchar {
    let mut type_0: uchar = 0;
    let mut i: libc::c_int = 0;
    let mut step: libc::c_int = 0 as libc::c_int;
    let mut txtlen: uint16_t = 0 as libc::c_int as uint16_t;
    let mut hf: hlpf = hlpf {
        type_0: 0,
        len: 0,
        ttl: 0,
        hdr: 0 as *mut uchar,
        from: 0 as *mut uchar,
        to: 0 as *mut uchar,
    };
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    let mut n: libc::c_int = *pn;
    type_0 = *from.offset(0 as libc::c_int as isize);
    from = from.offset(1);
    from;
    mv = from as *mut mvalue;
    from = from.offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
    num = (*mv).num as libc::c_int;
    if num > MAX_MSG_SEG as libc::c_int {
        num = MAX_MSG_SEG as libc::c_int;
    }
    hf.hdr = hdr;
    hf.ttl = (*mv).ttl;
    hf.type_0 = type_0 as ushort;
    if type_0 as libc::c_int == A as libc::c_int {
        step = 4 as libc::c_int;
    }
    if type_0 as libc::c_int == AAAA as libc::c_int {
        step = 16 as libc::c_int;
    }
    match type_0 as libc::c_int {
        1 | 28 => {
            i = 0 as libc::c_int;
            while i < num {
                to = fill_name_in_msg(h, to, n);
                hf.from = from;
                hf.to = to;
                to = fill_all_records_in_msg(h, &mut hf, pn);
                from = from.offset(step as isize);
                i += 1;
                i;
            }
            return to;
        }
        5 => {
            to = fill_name_in_msg(h, to, n);
            hf.from = from;
            hf.to = to;
            hf
                .len = (strlen(from as *const libc::c_char))
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as ushort;
            to = fill_all_records_in_msg(h, &mut hf, pn);
            return to;
        }
        2 => {
            i = 0 as libc::c_int;
            while i < num {
                to = fill_name_in_msg(h, to, n);
                hf.from = from;
                hf.to = to;
                hf
                    .len = (strlen(from as *const libc::c_char))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as ushort;
                to = fill_all_records_in_msg(h, &mut hf, pn);
                from = from.offset(hf.len as libc::c_int as isize);
                i += 1;
                i;
            }
            return to;
        }
        15 => {
            i = 0 as libc::c_int;
            while i < num {
                to = fill_name_in_msg(h, to, n);
                hf.from = from;
                hf.to = to;
                hf
                    .len = (strlen(
                    from
                        .offset(
                            ::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize,
                        ) as *const libc::c_char,
                ))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as ushort;
                to = fill_all_records_in_msg(h, &mut hf, pn);
                from = from
                    .offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
                from = from.offset(hf.len as libc::c_int as isize);
                i += 1;
                i;
            }
            return to;
        }
        16 => {
            i = 0 as libc::c_int;
            while i < num {
                txtlen = *(from as *mut uint16_t);
                (*h.offset(n as isize)).len = txtlen as libc::c_short;
                to = fill_name_in_msg(h, to, n);
                hf.from = from;
                hf.to = to;
                to = fill_all_records_in_msg(h, &mut hf, pn);
                from = from
                    .offset(txtlen as libc::c_int as isize)
                    .offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
                i += 1;
                i;
            }
            return to;
        }
        33 => {
            i = 0 as libc::c_int;
            while i < num {
                to = fill_name_in_msg(h, to, n);
                hf.from = from;
                hf.to = to;
                hf
                    .len = (strlen(
                    from
                        .offset(
                            (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                                .wrapping_mul(3 as libc::c_int as libc::c_ulong) as isize,
                        ) as *const libc::c_char,
                ))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as ushort;
                to = fill_all_records_in_msg(h, &mut hf, pn);
                from = from
                    .offset(
                        (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                            .wrapping_mul(3 as libc::c_int as libc::c_ulong) as isize,
                    );
                from = from.offset(hf.len as libc::c_int as isize);
                i += 1;
                i;
            }
            return to;
        }
        _ => {
            printf(
                b"not support or error in fill msg\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    return 0 as *mut uchar;
}
pub unsafe extern "C" fn fill_header_in_msg(mut sh: *mut setheader) -> *mut uchar {
    let mut itor: *mut uchar = (*sh).itor;
    let mut hdr: *mut dnsheader = (*sh).itor as *mut dnsheader;
    let mut qd: *mut qdns = 0 as *mut qdns;
    (*hdr).flags = 0 as libc::c_int as uint16_t;
    (*hdr).flags = ((*hdr).flags as libc::c_int | 0x8000 as libc::c_int) as uint16_t;
    (*hdr).flags = ((*hdr).flags as libc::c_int | 0x80 as libc::c_int) as uint16_t;
    (*hdr)
        .flags = ((*hdr).flags as libc::c_int >> 8 as libc::c_int
        | (((*hdr).flags as libc::c_int) << 8 as libc::c_int) as uint16_t as libc::c_int)
        as uint16_t;
    (*hdr)
        .ancount = ((*sh).an as libc::c_int >> 8 as libc::c_int
        | (((*sh).an as libc::c_int) << 8 as libc::c_int) as uint16_t as libc::c_int)
        as uint16_t;
    (*hdr)
        .nscount = ((*sh).ns as libc::c_int >> 8 as libc::c_int
        | (((*sh).ns as libc::c_int) << 8 as libc::c_int) as uint16_t as libc::c_int)
        as uint16_t;
    (*hdr).arcount = 0 as libc::c_int as uint16_t;
    itor = itor.offset(::std::mem::size_of::<dnsheader>() as libc::c_ulong as isize);
    itor = itor.offset((*sh).dlen as libc::c_int as isize);
    qd = itor as *mut qdns;
    (*qd)
        .type_0 = ((*sh).type_0 as libc::c_int >> 8 as libc::c_int
        | (((*sh).type_0 as libc::c_int) << 8 as libc::c_int) as uint16_t as libc::c_int)
        as uint16_t;
    (*qd)
        .dclass = (CLASS_IN as libc::c_int as uint16_t as libc::c_int >> 8 as libc::c_int
        | ((CLASS_IN as libc::c_int) << 8 as libc::c_int) as uint16_t as libc::c_int)
        as uint16_t;
    itor = itor.offset(::std::mem::size_of::<qdns>() as libc::c_ulong as isize);
    return itor;
}
pub unsafe extern "C" fn make_dns_msg_for_new(
    mut itor: *mut uchar,
    mut msgid: ushort,
    mut d: *mut uchar,
    mut len: libc::c_int,
    mut type_0: ushort,
) -> libc::c_int {
    let mut buf: *mut uchar = itor;
    let mut hdr: *mut dnsheader = 0 as *mut dnsheader;
    let mut qd: *mut qdns = 0 as *mut qdns;
    hdr = buf as *mut dnsheader;
    (*hdr).id = msgid;
    (*hdr).flags = htons(0x100 as libc::c_int as uint16_t);
    (*hdr).qdcount = htons(1 as libc::c_int as uint16_t);
    (*hdr).arcount = htons(0 as libc::c_int as uint16_t);
    (*hdr).nscount = (*hdr).arcount;
    (*hdr).ancount = (*hdr).nscount;
    buf = buf.offset(::std::mem::size_of::<dnsheader>() as libc::c_ulong as isize);
    memcpy(buf as *mut libc::c_void, d as *const libc::c_void, len as libc::c_ulong);
    *buf.offset((len - 1 as libc::c_int) as isize) = 0 as libc::c_int as uchar;
    buf = buf.offset(len as isize);
    qd = buf as *mut qdns;
    (*qd).type_0 = htons(type_0);
    (*qd).dclass = htons(CLASS_IN as libc::c_int as uint16_t);
    buf = buf.offset(4 as libc::c_int as isize);
    return buf.offset_from(itor) as libc::c_long as libc::c_int;
}
pub unsafe extern "C" fn fill_rrset_in_buffer(
    mut buffer: *mut uchar,
    mut label: *mut uchar,
    mut hdr: *mut uchar,
    mut lth: libc::c_int,
    mut type_0: libc::c_int,
    mut hlp: *mut hlpp,
) -> libc::c_int {
    let mut mlen: libc::c_int = 0 as libc::c_int;
    let mut len: uint16_t = lth as uint16_t;
    let mut from: *mut srv = 0 as *mut srv;
    let mut to: *mut srv = 0 as *mut srv;
    match type_0 {
        1 => {
            mlen = 4 as libc::c_int;
            memcpy(
                buffer as *mut libc::c_void,
                label as *const libc::c_void,
                4 as libc::c_int as libc::c_ulong,
            );
        }
        2 | 5 => {
            get_domain_from_msg(label, hdr, buffer, &mut mlen);
            to_lowercase(buffer, mlen);
        }
        6 => {
            mlen = 0 as libc::c_int;
        }
        28 => {
            mlen = 16 as libc::c_int;
            memcpy(
                buffer as *mut libc::c_void,
                label as *const libc::c_void,
                16 as libc::c_int as libc::c_ulong,
            );
        }
        15 => {
            memcpy(
                buffer as *mut libc::c_void,
                label as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            );
            label = label.offset(2 as libc::c_int as isize);
            buffer = buffer.offset(2 as libc::c_int as isize);
            get_domain_from_msg(label, hdr, buffer, &mut mlen);
            mlen += 2 as libc::c_int;
        }
        33 => {
            from = label as *mut srv;
            to = buffer as *mut srv;
            (*to).pri = (*from).pri;
            (*to).wei = (*from).wei;
            (*to).port = (*from).port;
            buffer = buffer
                .offset(
                    (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                        .wrapping_mul(3 as libc::c_int as libc::c_ulong) as isize,
                );
            label = label
                .offset(
                    (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                        .wrapping_mul(3 as libc::c_int as libc::c_ulong) as isize,
                );
            get_domain_from_msg(label, hdr, buffer, &mut mlen);
            mlen = (mlen as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                        .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                ) as libc::c_int as libc::c_int;
        }
        16 => {
            memcpy(
                buffer as *mut libc::c_void,
                &mut len as *mut uint16_t as *const libc::c_void,
                ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
            );
            buffer = buffer
                .offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
            memcpy(
                buffer as *mut libc::c_void,
                label as *const libc::c_void,
                lth as libc::c_ulong,
            );
            mlen = (lth as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                as libc::c_int;
        }
        _ => return -(1 as libc::c_int),
    }
    return mlen;
}
pub unsafe extern "C" fn check_an_msg(
    mut flag: ushort,
    mut domain: *mut uchar,
    mut bk: *mut libc::c_int,
) -> libc::c_int {
    let mut get: uint = 0 as libc::c_int as uint;
    flag = ntohs(flag);
    get = ((flag as libc::c_int & 0x8000 as libc::c_int) / 0x8000 as libc::c_int)
        as uint;
    if get == 0 as libc::c_int as libc::c_uint {
        printf(b"answer set Q sign\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    get = ((flag as libc::c_int & 0x7800 as libc::c_int) >> 11 as libc::c_int) as uint;
    get = ((flag as libc::c_int & 0x400 as libc::c_int) / 0x400 as libc::c_int) as uint;
    get = ((flag as libc::c_int & 0x200 as libc::c_int) / 0x200 as libc::c_int) as uint;
    if get == 1 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int;
    }
    get = ((flag as libc::c_int & 0x100 as libc::c_int) / 0x100 as libc::c_int) as uint;
    get = (flag as libc::c_int & 0x7 as libc::c_int) as uint;
    if get != 0 as libc::c_int as libc::c_uint
        && get != NAME_ERROR as libc::c_int as libc::c_uint
    {
        match get {
            1 => {}
            4 => {}
            5 => {}
            2 | _ => {}
        }
        return 2 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn check_out_msg(
    mut cid: ushort,
    mut buf: *mut uchar,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut hdr: *mut dnsheader = buf as *mut dnsheader;
    (*hdr).id = cid;
    (*hdr).flags = 0 as libc::c_int as uint16_t;
    (*hdr)
        .flags = htons(
        ((*hdr).flags as libc::c_int | 0x8000 as libc::c_int) as uint16_t,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn check_td(mut td: *mut uchar) -> libc::c_int {
    let mut type_0: uchar = *td.offset(0 as libc::c_int as isize);
    let mut itor: *mut uchar = td.offset(1 as libc::c_int as isize);
    let mut len: uchar = *itor.offset(0 as libc::c_int as isize);
    if type_0 as libc::c_int != A as libc::c_int
        && type_0 as libc::c_int != NS as libc::c_int
        && type_0 as libc::c_int != CNAME as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    while len as libc::c_int != 0 as libc::c_int {
        if len as libc::c_int > 50 as libc::c_int {
            return -(1 as libc::c_int);
        }
        itor = itor
            .offset(len as libc::c_int as isize)
            .offset(1 as libc::c_int as isize);
        len = *itor.offset(0 as libc::c_int as isize);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn is_glue(
    mut domain: *mut uchar,
    mut ns: *mut uchar,
) -> libc::c_int {
    let mut d: uchar = 0;
    let mut n: uchar = 0;
    let mut dlen: libc::c_int = 0;
    let mut nlen: libc::c_int = 0;
    dlen = strlen(domain as *const libc::c_char) as libc::c_int;
    nlen = strlen(ns as *const libc::c_char) as libc::c_int;
    dlen -= 1;
    dlen;
    nlen -= 1;
    nlen;
    if dlen >= nlen {
        return 0 as libc::c_int;
    }
    d = *domain.offset(dlen as isize);
    n = *ns.offset(nlen as isize);
    while d as libc::c_int == n as libc::c_int {
        dlen -= 1;
        dlen;
        nlen -= 1;
        nlen;
        if dlen == 0 as libc::c_int {
            return 1 as libc::c_int;
        }
        d = *domain.offset(dlen as isize);
        n = *ns.offset(nlen as isize);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn pre_find(
    mut mbuf: *mut mbuf_type,
    mut fwd: *mut htable,
    mut ht: *mut htable,
    mut ip: *mut uchar,
) -> libc::c_int {
    let mut td: *mut uchar = 0 as *mut uchar;
    let mut itor: *mut uchar = 0 as *mut uchar;
    let mut xlen: libc::c_int = 0 as libc::c_int;
    let mut dbg: libc::c_int = 100 as libc::c_int;
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    let mut td_len: libc::c_int = 0;
    let mut new_td_len: libc::c_int = 0;
    let mut hash: *mut hashval_t = 0 as *mut hashval_t;
    let mut thash: hashval_t = 0 as libc::c_int as hashval_t;
    (*mbuf).qname = Q_DOMAIN as libc::c_int as ushort;
    if (*mbuf).hascname as libc::c_int == 1 as libc::c_int {
        (*mbuf).qing = ((*mbuf).qbuffer).as_mut_ptr();
        td_len = (*mbuf).qlen as libc::c_int;
        td = ((*mbuf).qbuffer).as_mut_ptr();
        (*mbuf).qhash = &mut (*mbuf).qbuffer_hash;
    } else {
        td_len = (*mbuf).dlen;
        (*mbuf).qing = (*mbuf).td;
        (*mbuf).qlen = (*mbuf).dlen as ushort;
        td = (*mbuf).td;
        (*mbuf)
            .qhash = &mut *((*mbuf).lowerdomain.hash)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut hashval_t;
    }
    hash = (*mbuf).qhash;
    xlen = htable_find(
        fwd,
        td,
        td_len,
        A as libc::c_int,
        ip,
        1900 as libc::c_int,
        0 as *mut mvalue,
        hash,
    );
    if xlen > 0 as libc::c_int {
        ip = ip.offset(xlen as isize);
        mv = ip as *mut mvalue;
        (*mv).num = 0 as libc::c_int as uint16_t;
        (*mv).ttl = 0 as libc::c_int as uint32_t;
        (*mv).hits = 0 as libc::c_int as uint32_t;
        (*mv).len = 0 as libc::c_int as uint16_t;
        return xlen;
    } else {
        let mut new_td: *mut uchar = (*mbuf).tdbuffer;
        if (*mbuf).lowerdomain.label_count as libc::c_int > 1 as libc::c_int {
            *new_td.offset(0 as libc::c_int as isize) = 1 as libc::c_int as uchar;
            *new_td.offset(1 as libc::c_int as isize) = '*' as i32 as uchar;
            new_td_len = (*mbuf)
                .lowerdomain
                .label_len[((*mbuf).lowerdomain.label_count as libc::c_int
                - 2 as libc::c_int) as usize] as libc::c_int;
            memcpy(
                new_td.offset(2 as libc::c_int as isize) as *mut libc::c_void,
                (*mbuf)
                    .lowerdomain
                    .label[((*mbuf).lowerdomain.label_count as libc::c_int
                    - 2 as libc::c_int) as usize] as *const libc::c_void,
                new_td_len as libc::c_ulong,
            );
            thash = 0 as libc::c_int as hashval_t;
            let mut rlen: libc::c_int = htable_find(
                fwd,
                new_td,
                new_td_len + 2 as libc::c_int,
                A as libc::c_int,
                ip,
                1900 as libc::c_int,
                0 as *mut mvalue,
                &mut thash,
            );
            if rlen > 0 as libc::c_int {
                ip = ip.offset(rlen as isize);
                mv = ip as *mut mvalue;
                (*mv).num = 0 as libc::c_int as uint16_t;
                (*mv).ttl = 0 as libc::c_int as uint32_t;
                (*mv).hits = 0 as libc::c_int as uint32_t;
                (*mv).len = 0 as libc::c_int as uint16_t;
                return rlen;
            }
        }
    }
    if (*mbuf).qtype as libc::c_uint == CNAME as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    itor = (*mbuf).tempbuffer;
    loop {
        xlen = find_record_with_ttl(
            ht,
            td,
            td_len,
            CNAME as libc::c_int,
            itor,
            2000 as libc::c_int,
            0 as *mut mvalue,
            hash,
        );
        if !(xlen > 0 as libc::c_int) {
            break;
        }
        (*mbuf).qname = Q_CNAME as libc::c_int as ushort;
        (*mbuf).hascname = 1 as libc::c_int as ushort;
        mv = itor as *mut mvalue;
        itor = itor.offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
        if (*mv).len as libc::c_int > 256 as libc::c_int - 1 as libc::c_int {
            return -(1 as libc::c_int);
        }
        memcpy(
            ((*mbuf).qbuffer).as_mut_ptr() as *mut libc::c_void,
            itor as *const libc::c_void,
            (*mv).len as libc::c_ulong,
        );
        (*mbuf).qing = ((*mbuf).qbuffer).as_mut_ptr();
        td_len = (*mv).len as libc::c_int;
        (*mbuf).qlen = td_len as ushort;
        (*mbuf).qbuffer_hash = 0 as libc::c_int as hashval_t;
        hash = &mut (*mbuf).qbuffer_hash;
        td = ((*mbuf).qbuffer).as_mut_ptr();
        let fresh7 = dbg;
        dbg = dbg - 1;
        if fresh7 == 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn transfer_record_to_msg(
    mut buff: *mut uchar,
    mut key: *mut uchar,
    mut msg: *mut uchar,
    mut msglen: libc::c_int,
    mut ttloff: *mut uint16_t,
) -> libc::c_int {
    let mut segs: uint16_t = *ttloff.offset(0 as libc::c_int as isize);
    let mut totallen: uint16_t = 0 as libc::c_int as uint16_t;
    let mut itor: *mut uchar = 0 as *mut uchar;
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    if segs as libc::c_int == 0 as libc::c_int
        || segs as libc::c_int > 100 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    totallen = msglen as uint16_t;
    totallen = (totallen as libc::c_ulong)
        .wrapping_add(
            (segs as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
        )
        .wrapping_add(::std::mem::size_of::<mvalue>() as libc::c_ulong) as uint16_t;
    if totallen as libc::c_int > MAX_MSG_SIZE as libc::c_int {
        return -(1 as libc::c_int);
    }
    itor = buff;
    mv = itor as *mut mvalue;
    (*mv).seg = segs;
    (*mv).len = msglen as uint16_t;
    itor = itor.offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
    memcpy(
        itor as *mut libc::c_void,
        ttloff.offset(1 as libc::c_int as isize) as *const libc::c_void,
        (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
            .wrapping_mul(segs as libc::c_ulong),
    );
    itor = itor
        .offset(
            (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                .wrapping_mul(segs as libc::c_ulong) as isize,
        );
    memcpy(
        itor as *mut libc::c_void,
        msg as *const libc::c_void,
        msglen as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn make_A_record_from_segment(
    mut ipmsg: *mut uchar,
    mut iitor: *mut uchar,
) -> libc::c_int {
    let mut reallen: libc::c_int = 0 as libc::c_int;
    let mut ipto: *mut uchar = 0 as *mut uchar;
    let mut ipfrom: *mut uchar = 0 as *mut uchar;
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    let mut off: uint16_t = 0;
    let mut segs: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    mv = ipmsg as *mut mvalue;
    segs = (*mv).seg as libc::c_int;
    ipto = iitor.offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
    i = 0 as libc::c_int;
    while i < segs {
        off = *(ipmsg
            .offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize)
            .offset(
                (i as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                    as isize,
            ) as *mut uint16_t);
        ipfrom = ipmsg.offset(off as libc::c_int as isize);
        memcpy(
            ipto as *mut libc::c_void,
            ipfrom as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        reallen += 4 as libc::c_int;
        ipto = ipto.offset(4 as libc::c_int as isize);
        i += 1;
        i;
    }
    (*mv).len = reallen as uint16_t;
    memcpy(
        iitor as *mut libc::c_void,
        ipmsg as *const libc::c_void,
        ::std::mem::size_of::<mvalue>() as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn retrive_ip(
    mut mbuf: *mut mbuf_type,
    mut itor: *mut uchar,
    mut num: libc::c_int,
    mut ip: *mut uchar,
    mut ht: *mut htable,
    mut fq: *mut libc::c_int,
) -> libc::c_int {
    let mut mi: *mut mvalue = 0 as *mut mvalue;
    let mut i: libc::c_int = 0;
    let mut xlen: libc::c_int = 0;
    let mut iplen: libc::c_int = IP_DATA_LEN as libc::c_int;
    let mut got: libc::c_int = 0 as libc::c_int;
    let mut ipbuffer: *mut uchar = (*mbuf).ipbuffer;
    *fq = 0 as libc::c_int;
    let mut nstd: *mut uchar = 0 as *mut uchar;
    let mut iitor: *mut uchar = ip;
    let mut hash: hashval_t = 0;
    i = 0 as libc::c_int;
    while i < num {
        xlen = (strlen(itor as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        nstd = itor;
        itor = itor.offset(xlen as isize);
        hash = 0 as libc::c_int as hashval_t;
        xlen = find_record_with_ttl(
            ht,
            nstd,
            xlen,
            A as libc::c_int,
            ipbuffer,
            (iplen as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<mvalue>() as libc::c_ulong)
                as libc::c_int,
            0 as *mut mvalue,
            &mut hash,
        );
        if xlen > 0 as libc::c_int {
            mi = ipbuffer as *mut mvalue;
            if (*mi).seg as libc::c_int > 0 as libc::c_int {
                make_A_record_from_segment(ipbuffer, iitor);
            } else {
                memcpy(
                    iitor as *mut libc::c_void,
                    ipbuffer as *const libc::c_void,
                    ((*mi).len as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<mvalue>() as libc::c_ulong),
                );
            }
            iitor = iitor
                .offset((*mi).len as libc::c_int as isize)
                .offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
            iplen = ((iplen - (*mi).len as libc::c_int) as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<mvalue>() as libc::c_ulong)
                as libc::c_int;
            got += 1;
            got;
        }
        if xlen < 0 as libc::c_int {
            *fq = i;
            break;
        } else {
            i += 1;
            i;
        }
    }
    if iitor != ip {
        mi = iitor as *mut mvalue;
        (*mi).num = 0 as libc::c_int as uint16_t;
        (*mi).ttl = 0 as libc::c_int as uint32_t;
        (*mi).hits = 0 as libc::c_int as uint32_t;
        (*mi).len = 0 as libc::c_int as uint16_t;
        return got;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn fill_extra_addr(mut ip: *mut uchar) -> libc::c_int {
    let mut extra: [*const libc::c_char; 2] = [
        *g_nameservers.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *const libc::c_char,
        *g_nameservers.as_mut_ptr().offset(1 as libc::c_int as isize)
            as *const libc::c_char,
    ];
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    n = (::std::mem::size_of::<[*const libc::c_char; 2]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
        as libc::c_int;
    mv = ip as *mut mvalue;
    ip = ip.offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
    (*mv).num = 0 as libc::c_int as uint16_t;
    (*mv).ttl = 0 as libc::c_int as uint32_t;
    (*mv).hits = 0 as libc::c_int as uint32_t;
    (*mv).len = 0 as libc::c_int as uint16_t;
    (*mv).seg = 0 as libc::c_int as uint16_t;
    i = 0 as libc::c_int;
    while i < n {
        if make_bin_from_str(ip, extra[i as usize]) == 0 as libc::c_int {
            (*mv).num = ((*mv).num).wrapping_add(1);
            (*mv).num;
            (*mv).len = ((*mv).len as libc::c_int + 4 as libc::c_int) as uint16_t;
            ip = ip.offset(4 as libc::c_int as isize);
        }
        i += 1;
        i;
    }
    mv = ip as *mut mvalue;
    (*mv).num = 0 as libc::c_int as uint16_t;
    (*mv).ttl = 0 as libc::c_int as uint32_t;
    (*mv).hits = 0 as libc::c_int as uint32_t;
    (*mv).len = 0 as libc::c_int as uint16_t;
    (*mv).seg = 0 as libc::c_int as uint16_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn find_addr(
    mut fwd: *mut htable,
    mut ht: *mut htable,
    mut mbuf: *mut mbuf_type,
    mut ip: *mut uchar,
    mut forward: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut xlen: libc::c_int = 0 as libc::c_int;
    let mut dbg: libc::c_int = 100 as libc::c_int;
    let mut first_query: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    let mut td: *mut uchar = 0 as *mut uchar;
    let mut buffer: *mut uchar = (*mbuf).tempbuffer;
    let mut itor: *mut uchar = 0 as *mut uchar;
    let mut glue: *mut uchar = 0 as *mut uchar;
    let mut td_len: libc::c_int = 0;
    let mut diff_len: libc::c_int = 0;
    let mut ori_flag: libc::c_int = 0 as libc::c_int;
    let mut root_flag: libc::c_int = 0 as libc::c_int;
    let mut thash: hashval_t = 0;
    let mut hash: *mut hashval_t = 0 as *mut hashval_t;
    let mut label_count: libc::c_int = 0 as libc::c_int;
    if (*mbuf).qtimes as libc::c_int > MAX_TRY_TIMES as libc::c_int - 3 as libc::c_int {
        fill_extra_addr(ip);
        return 0 as libc::c_int;
    }
    ret = pre_find(mbuf, fwd, ht, ip);
    if ret > 0 as libc::c_int {
        return 0 as libc::c_int
    } else if ret < 0 as libc::c_int {
        return ret
    } else if forward != 0 {
        fill_extra_addr(ip);
        return 0 as libc::c_int;
    }
    td = (*mbuf).qing;
    itor = td;
    hash = (*mbuf).qhash;
    td_len = (*mbuf).qlen as libc::c_int;
    if (*mbuf).hascname != 0 {
        ori_flag = 1 as libc::c_int;
    }
    loop {
        loop {
            ret = find_record_with_ttl(
                ht,
                itor,
                td_len,
                NS as libc::c_int,
                buffer,
                IP_DATA_LEN as libc::c_int,
                0 as *mut mvalue,
                hash,
            );
            if ret > 0 as libc::c_int {
                break;
            }
            let fresh8 = dbg;
            dbg = dbg - 1;
            if fresh8 == 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            if ori_flag != 0 {
                diff_len = *itor.offset(0 as libc::c_int as isize) as libc::c_int
                    + 1 as libc::c_int;
                itor = itor.offset(diff_len as isize);
                if *itor.offset(0 as libc::c_int as isize) as libc::c_int
                    == 0 as libc::c_int
                {
                    if root_flag == 0 as libc::c_int {
                        *itor.offset(1 as libc::c_int as isize) = '\0' as i32 as uchar;
                        root_flag = 1 as libc::c_int;
                        td_len = 2 as libc::c_int;
                    } else {
                        return -(1 as libc::c_int)
                    }
                } else {
                    td_len -= diff_len;
                }
                thash = 0 as libc::c_int as hashval_t;
                hash = &mut thash;
            } else {
                label_count += 1;
                label_count;
                if label_count > (*mbuf).lowerdomain.label_count as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if label_count == (*mbuf).lowerdomain.label_count as libc::c_int {
                    itor = ((*mbuf)
                        .lowerdomain
                        .label[(label_count - 1 as libc::c_int) as usize])
                        .offset(
                            (*mbuf)
                                .lowerdomain
                                .label_len[(label_count - 1 as libc::c_int) as usize]
                                as libc::c_int as isize,
                        );
                    *itor.offset(1 as libc::c_int as isize) = '\0' as i32 as uchar;
                    td_len = 2 as libc::c_int;
                    thash = 0 as libc::c_int as hashval_t;
                    hash = &mut thash;
                } else {
                    itor = (*mbuf).lowerdomain.label[label_count as usize];
                    td_len = (*mbuf).lowerdomain.label_len[label_count as usize]
                        as libc::c_int;
                    hash = &mut *((*mbuf).lowerdomain.hash)
                        .as_mut_ptr()
                        .offset(label_count as isize) as *mut hashval_t;
                }
            }
        }
        mv = buffer as *mut mvalue;
        glue = itor;
        itor = buffer.offset(::std::mem::size_of::<mvalue>() as libc::c_ulong as isize);
        ret = retrive_ip(mbuf, itor, (*mv).num as libc::c_int, ip, ht, &mut first_query);
        if ret > 0 as libc::c_int {
            if ret < (*mv).num as libc::c_int && (*mbuf).qns == 1 as libc::c_int {
                (*mbuf).qns = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < first_query {
                    xlen = (strlen(itor as *const libc::c_char))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
                    itor = itor.offset(xlen as isize);
                    i += 1;
                    i;
                }
            } else {
                return 0 as libc::c_int
            }
        }
        if is_glue(glue, itor) != 1 as libc::c_int {
            if ori_flag == 0 {
                ori_flag = 1 as libc::c_int;
            }
            xlen = (strlen(itor as *const libc::c_char))
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
            if xlen > 256 as libc::c_int - 1 as libc::c_int
                || *itor.offset(0 as libc::c_int as isize) as libc::c_int
                    == 0 as libc::c_int
                || *itor.offset(0 as libc::c_int as isize) as libc::c_int > xlen
            {
                return -(1 as libc::c_int);
            }
            memcpy(
                ((*mbuf).qbuffer).as_mut_ptr() as *mut libc::c_void,
                itor as *const libc::c_void,
                xlen as libc::c_ulong,
            );
            (*mbuf).qbuffer_hash = 0 as libc::c_int as hashval_t;
            (*mbuf).qing = ((*mbuf).qbuffer).as_mut_ptr();
            (*mbuf).qhash = &mut (*mbuf).qbuffer_hash;
            (*mbuf).qlen = xlen as ushort;
            hash = (*mbuf).qhash;
            td_len = (*mbuf).qlen as libc::c_int;
            td = (*mbuf).qing;
            itor = td;
        } else if ori_flag != 0 {
            diff_len = *glue.offset(0 as libc::c_int as isize) as libc::c_int
                + 1 as libc::c_int;
            itor = glue.offset(diff_len as isize);
            if *itor.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            td_len -= diff_len;
            thash = 0 as libc::c_int as hashval_t;
            hash = &mut thash;
        } else {
            label_count += 1;
            label_count;
            if label_count >= (*mbuf).lowerdomain.label_count as libc::c_int {
                return -(1 as libc::c_int);
            }
            itor = (*mbuf).lowerdomain.label[label_count as usize];
            td_len = (*mbuf).lowerdomain.label_len[label_count as usize] as libc::c_int;
            hash = &mut *((*mbuf).lowerdomain.hash)
                .as_mut_ptr()
                .offset(label_count as isize) as *mut hashval_t;
        }
        (*mbuf).qname = Q_NS as libc::c_int as ushort;
        let fresh9 = dbg;
        dbg = dbg - 1;
        if fresh9 == 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    };
}
pub unsafe extern "C" fn check_qo(mut qo: *mut qoutinfo) -> libc::c_int {
    if qo.is_null() {
        return 0 as libc::c_int;
    }
    if (*qo).hascname as libc::c_int > 1 as libc::c_int {
        printf(b"qo error\n\0" as *const u8 as *const libc::c_char);
    }
    if ((*qo).td).is_null() {
        printf(b"qo error2\n\0" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dbg_print_label(
    mut label: *mut uchar,
    mut visible: libc::c_int,
) -> *mut uchar {
    let mut i: uchar = 0;
    let mut len: uchar = *label;
    if visible == 1 as libc::c_int {
        i = 1 as libc::c_int as uchar;
        while (i as libc::c_int) < len as libc::c_int + 1 as libc::c_int {
            printf(
                b"%c\0" as *const u8 as *const libc::c_char,
                *label.offset(i as isize) as libc::c_int,
            );
            i = i.wrapping_add(1);
            i;
        }
    }
    return label
        .offset(*label.offset(0 as libc::c_int as isize) as libc::c_int as isize)
        .offset(1 as libc::c_int as isize);
}
pub unsafe extern "C" fn dbg_print_domain(
    mut hdr: *mut uchar,
    mut itor: *mut uchar,
) -> *mut uchar {
    let mut len: uchar = 0;
    let mut tmp: *mut uchar = 0 as *mut uchar;
    let mut offset: ushort = 0;
    let mut debug: libc::c_int = 100 as libc::c_int;
    len = *itor.offset(0 as libc::c_int as isize);
    if len as libc::c_int == 0 as libc::c_int {
        printf(b"root\n\0" as *const u8 as *const libc::c_char);
        return 0 as *mut uchar;
    }
    offset = ntohs(*(itor as *mut ushort));
    if offset as libc::c_int >= 0xc000 as libc::c_int
        && offset as libc::c_int <= 0xcfff as libc::c_int
    {
        itor = hdr.offset((offset as libc::c_int & 0x3fff as libc::c_int) as isize);
    }
    while len as libc::c_int != 0 as libc::c_int
        && {
            let fresh10 = debug;
            debug = debug - 1;
            fresh10 != 0
        }
    {
        if offset as libc::c_int >= 0xc000 as libc::c_int
            && offset as libc::c_int <= 0xcfff as libc::c_int
        {
            tmp = itor.offset(2 as libc::c_int as isize);
            itor = dbg_print_label(
                hdr.offset((offset as libc::c_int & 0x3fff as libc::c_int) as isize),
                1 as libc::c_int,
            );
        } else {
            itor = dbg_print_label(itor, 1 as libc::c_int);
        }
        printf(b".\0" as *const u8 as *const libc::c_char);
        len = *itor.offset(0 as libc::c_int as isize);
        offset = ntohs(*(itor as *mut ushort));
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if tmp.is_null() {
        tmp = itor.offset(1 as libc::c_int as isize);
    }
    return tmp;
}
pub unsafe extern "C" fn dbg_print_ip(mut ip: *mut uchar, mut type_0: rrtype) {
    let mut i: libc::c_int = 0;
    let mut ipv4: [uint; 4] = [0 as libc::c_int as uint, 0, 0, 0];
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        ipv4[i as usize] = *ip.offset(i as isize) as uint;
        i += 1;
        i;
    }
    if type_0 as libc::c_uint == A as libc::c_int as libc::c_uint {
        printf(
            b"%u.%u.%u.%u\n\0" as *const u8 as *const libc::c_char,
            ipv4[0 as libc::c_int as usize] as libc::c_ushort as libc::c_int,
            ipv4[1 as libc::c_int as usize],
            ipv4[2 as libc::c_int as usize],
            ipv4[3 as libc::c_int as usize],
        );
    } else if type_0 as libc::c_uint == AAAA as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            if *ip.offset((i * 2 as libc::c_int) as isize) as libc::c_int
                != 0 as libc::c_int
            {
                if (*ip.offset((i * 2 as libc::c_int) as isize) as libc::c_int)
                    < 0x10 as libc::c_int
                {
                    printf(b"0\0" as *const u8 as *const libc::c_char);
                }
                printf(
                    b"%x\0" as *const u8 as *const libc::c_char,
                    *ip.offset((i * 2 as libc::c_int) as isize) as uint,
                );
            }
            if (*ip.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize)
                as libc::c_int) < 0x10 as libc::c_int
            {
                printf(b"0\0" as *const u8 as *const libc::c_char);
            }
            printf(
                b"%x\0" as *const u8 as *const libc::c_char,
                *ip.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize) as uint,
            );
            if i != 7 as libc::c_int {
                printf(b":\0" as *const u8 as *const libc::c_char);
            }
            i += 1;
            i;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(
            b"unknow type %d\n\0" as *const u8 as *const libc::c_char,
            type_0 as libc::c_uint,
        );
    };
}
pub unsafe extern "C" fn dbg_print_td(mut td: *mut uchar) -> libc::c_int {
    let mut c: uchar = *td.offset(0 as libc::c_int as isize);
    printf(b"%d,\0" as *const u8 as *const libc::c_char, c as libc::c_int);
    dbg_print_domain(0 as *mut uchar, td.offset(1 as libc::c_int as isize));
    return 0 as libc::c_int;
}
