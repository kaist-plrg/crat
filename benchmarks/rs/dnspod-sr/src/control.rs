use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn getpid() -> __pid_t;
    fn pthread_spin_lock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    fn pthread_spin_unlock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn delete_node(rbt: *mut rbtree, nd: *mut rbnode) -> *mut libc::c_void;
    fn find_node(rbt: *mut rbtree, key: *mut libc::c_void) -> *mut rbnode;
    fn get_rbt_size(rbt: *mut rbtree) -> uint;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn htable_delete(
        ht: *mut htable,
        key: *mut uchar,
        klen: libc::c_int,
        type_0: libc::c_int,
        hashd: hashval_t,
    ) -> *mut uchar;
    fn get_pre_mem_hash(
        _: *mut libc::c_void,
        klen: libc::c_int,
        hashd: *mut hashval_t,
    ) -> uint;
    fn check_dns_name(domain: *mut uchar, lowerdomain: *mut packet_type) -> libc::c_int;
    fn str_to_len_label(domain: *mut uchar, len: libc::c_int) -> *mut uchar;
    fn insert_into_ttltree(
        rbt: *mut rbtree,
        td: *mut uchar,
        len: libc::c_int,
        type_0: libc::c_int,
        ttl: uint,
        lowerdomain: *mut packet_type,
    ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type ulong = libc::c_ulong;
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
pub static mut query_type_map: [libc::c_int; 256] = [0; 256];
pub static mut global_out_info: *mut global_query_info = 0 as *const global_query_info
    as *mut global_query_info;
pub static mut global_serv: *mut server = 0 as *const server as *mut server;
pub static mut g_nameservers: [*mut libc::c_char; 2] = [0 as *const libc::c_char
    as *mut libc::c_char; 2];
pub unsafe extern "C" fn refresh_ttl_with_td(
    mut key: *mut uchar,
    mut len: libc::c_int,
    mut type_0: libc::c_int,
    mut ht: *mut htable,
    mut ttlexp: *mut rbtree,
    mut lowerdomain: *mut packet_type,
) -> libc::c_int {
    pthread_spin_lock(&mut (*ttlexp).lock);
    printf(
        b"after delete, before insert, rbt size: %d\n\0" as *const u8
            as *const libc::c_char,
        get_rbt_size(ttlexp),
    );
    insert_into_ttltree(ttlexp, key, len, type_0, 0 as libc::c_int as uint, lowerdomain);
    printf(
        b"after insert, rbt size: %d\n\0" as *const u8 as *const libc::c_char,
        get_rbt_size(ttlexp),
    );
    pthread_spin_unlock(&mut (*ttlexp).lock);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn hijack(
    mut domain: *mut uchar,
    mut type_0: uint16_t,
    mut ht: *mut htable,
    mut ttlexp: *mut rbtree,
) -> libc::c_int {
    if domain.is_null()
        || *domain.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || type_0 as libc::c_int <= 0 as libc::c_int
        || type_0 as libc::c_int > 255 as libc::c_int
    {
        kill(getpid(), 10 as libc::c_int);
    } else {
        cache_flush(domain, type_0, ht, ttlexp);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn cache_flush(
    mut domain: *mut uchar,
    mut type_0: uint16_t,
    mut ht: *mut htable,
    mut ttlexp: *mut rbtree,
) -> libc::c_int {
    printf(b"cache flush domain %s\n\0" as *const u8 as *const libc::c_char, domain);
    let mut dlen: libc::c_int = (strlen(domain as *const libc::c_char))
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut hash: hashval_t = 0 as libc::c_int as hashval_t;
    let mut lowerdomain: packet_type = packet_type {
        label_count: 0,
        domain: [0; 256],
        label: [0 as *mut uint8_t; 64],
        label_offsets: [0; 64],
        label_len: [0; 64],
        hash: [0; 64],
    };
    str_to_len_label(domain, dlen);
    check_dns_name(domain, &mut lowerdomain);
    domain = (lowerdomain.domain).as_mut_ptr();
    let mut idx: libc::c_int = get_pre_mem_hash(
        domain as *mut libc::c_void,
        dlen,
        &mut hash,
    ) as libc::c_int;
    let mut val: *mut uchar = htable_delete(
        ht.offset(idx as isize),
        domain,
        dlen,
        type_0 as libc::c_int,
        hash,
    );
    if !val.is_null() {
        let mut tmp: *mut mvalue = val as *mut mvalue;
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
        pthread_spin_lock(&mut (*ttlexp).lock);
        tn.dlen = dlen as ushort;
        tn.exp = (*tmp).ttl;
        tn.type_0 = type_0;
        tn.data = domain;
        tn.lowerdomain = 0 as *mut packet_type;
        let mut pn: *mut rbnode = find_node(
            ttlexp,
            &mut tn as *mut ttlnode as *mut libc::c_void,
        );
        if !pn.is_null() {
            tmp_tn = delete_node(ttlexp, pn) as *mut ttlnode;
            if !tmp_tn.is_null() {
                free((*tmp_tn).lowerdomain as *mut libc::c_void);
                free(tmp_tn as *mut libc::c_void);
            } else {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"control.c\0" as *const u8 as *const libc::c_char,
                    70 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 69],
                        &[libc::c_char; 69],
                    >(
                        b"int cache_flush(uchar *, uint16_t, struct htable *, struct rbtree *)\0",
                    ))
                        .as_ptr(),
                );
                'c_6913: {
                    __assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"control.c\0" as *const u8 as *const libc::c_char,
                        70 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 69],
                            &[libc::c_char; 69],
                        >(
                            b"int cache_flush(uchar *, uint16_t, struct htable *, struct rbtree *)\0",
                        ))
                            .as_ptr(),
                    );
                };
            }
        }
        pthread_spin_unlock(&mut (*ttlexp).lock);
        free(val as *mut libc::c_void);
        refresh_ttl_with_td(
            domain,
            dlen,
            type_0 as libc::c_int,
            ht,
            ttlexp,
            &mut lowerdomain,
        );
    }
    return 0 as libc::c_int;
}
