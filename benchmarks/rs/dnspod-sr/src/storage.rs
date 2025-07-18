use ::libc;
extern "C" {
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn getpagesize() -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_spin_init(
        __lock: *mut pthread_spinlock_t,
        __pshared: libc::c_int,
    ) -> libc::c_int;
    fn pthread_spin_lock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    fn pthread_spin_unlock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    static mut global_now: time_t;
    fn dns_error(_: libc::c_int, _: *mut libc::c_char);
    fn nocase_char_hash_function(
        argv: *mut libc::c_void,
        klen: libc::c_int,
    ) -> hashval_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn delete_node(rbt: *mut rbtree, nd: *mut rbnode) -> *mut libc::c_void;
    fn find_node(rbt: *mut rbtree, key: *mut libc::c_void) -> *mut rbnode;
    fn check_support_type(type_0: ushort) -> libc::c_int;
    static support_type: [rrtype; 0];
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type ushort = libc::c_ushort;
pub type uint = libc::c_uint;
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type pthread_spinlock_t = libc::c_int;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uchar = libc::c_uchar;
pub type hashval_t = uint32_t;
pub type utils_numberic = libc::c_uint;
pub const DEBUG_TIMES: utils_numberic = 500;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_hlp {
    pub ht: *mut htable,
    pub idx: libc::c_int,
}
pub static mut MAX_ELE_NUM: uint = 1000000 as libc::c_int as uint;
pub unsafe extern "C" fn init_msgcache(mut n: libc::c_int) -> *mut msgcache {
    let mut mc: *mut msgcache = 0 as *mut msgcache;
    let mut pgsz: libc::c_int = 0;
    if n < 1 as libc::c_int || n > 5000 as libc::c_int {
        return 0 as *mut msgcache;
    }
    pgsz = getpagesize();
    mc = malloc(
        (::std::mem::size_of::<msgcache>() as libc::c_ulong)
            .wrapping_add((pgsz * n) as libc::c_ulong),
    ) as *mut msgcache;
    if mc.is_null() {
        return 0 as *mut msgcache;
    }
    (*mc).size = (pgsz * n) as uint32_t;
    pthread_spin_init(&mut (*mc).lock, 0 as libc::c_int);
    (*mc).tail = 0 as libc::c_int as uint64_t;
    (*mc).head = (*mc).tail;
    (*mc).pkt = 0 as libc::c_int as uint32_t;
    return mc;
}
pub unsafe extern "C" fn free_msgcache(mut mc: *mut msgcache) {
    if !mc.is_null() {
        free(mc as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn get_mvalue_len(mut val: *mut uchar) -> libc::c_int {
    let mut mv: *mut mvalue = val as *mut mvalue;
    return (*mv).len as libc::c_int;
}
pub unsafe extern "C" fn ttl_expired(mut val: *mut uchar) -> libc::c_int {
    let mut mv: *mut mvalue = val as *mut mvalue;
    let mut tx: uint = global_now as uint;
    if (*mv).ttl
        == (7 as libc::c_int * 86400 as libc::c_int + 1 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if (*mv).ttl < tx {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn deep_copy(
    mut from: *mut uchar,
    mut to: *mut uchar,
    mut tlen: libc::c_int,
) -> libc::c_int {
    let mut mv: *mut mvalue = from as *mut mvalue;
    let mut sz: libc::c_int = ((*mv).len as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<mvalue>() as libc::c_ulong)
        .wrapping_add(
            ((*mv).seg as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
        ) as libc::c_int;
    if sz >= tlen {
        return -(1 as libc::c_int);
    }
    (*mv).hits = ((*mv).hits).wrapping_add(1);
    (*mv).hits;
    memcpy(to as *mut libc::c_void, from as *const libc::c_void, sz as libc::c_ulong);
    return sz;
}
pub unsafe extern "C" fn get_pre_mem_hash(
    mut argv: *mut libc::c_void,
    mut klen: libc::c_int,
    mut hash: *mut hashval_t,
) -> uint {
    let mut ret: uint = 0 as libc::c_int as uint;
    if *hash == 0 as libc::c_int as libc::c_uint {
        *hash = nocase_char_hash_function(argv, klen);
    }
    ret = (*hash)
        .wrapping_div(10 as libc::c_int as libc::c_uint)
        .wrapping_rem(10 as libc::c_int as libc::c_uint);
    return ret;
}
pub unsafe extern "C" fn htable_create(
    mut h: Option::<hashfunc>,
    mut c: Option::<comparefunc>,
    mut size: libc::c_int,
    mut num: libc::c_int,
) -> *mut htable {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ht: *mut htable = 0 as *mut htable;
    if c.is_none() {
        return 0 as *mut htable;
    }
    ht = malloc(
        (::std::mem::size_of::<htable>() as libc::c_ulong)
            .wrapping_mul(num as libc::c_ulong),
    ) as *mut htable;
    if ht.is_null() {
        return 0 as *mut htable;
    }
    i = 0 as libc::c_int;
    while i < num {
        let ref mut fresh0 = (*ht.offset(i as isize)).h;
        *fresh0 = h;
        if h.is_none() {
            let ref mut fresh1 = (*ht.offset(i as isize)).h;
            *fresh1 = Some(
                nocase_char_hash_function
                    as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> hashval_t,
            );
        }
        let ref mut fresh2 = (*ht.offset(i as isize)).c;
        *fresh2 = c;
        (*ht.offset(i as isize)).size = size as uint;
        let ref mut fresh3 = (*ht.offset(i as isize)).edata;
        *fresh3 = 0 as *mut uchar;
        (*ht.offset(i as isize)).now = 0 as libc::c_int as uint;
        (*ht.offset(i as isize)).mask = (size - 1 as libc::c_int) as uint;
        pthread_spin_init(&mut (*ht.offset(i as isize)).lock, 0 as libc::c_int);
        let ref mut fresh4 = (*ht.offset(i as isize)).table;
        *fresh4 = malloc(
            (::std::mem::size_of::<hdata>() as libc::c_ulong)
                .wrapping_mul((*ht.offset(i as isize)).size as libc::c_ulong),
        ) as *mut hdata;
        if (*fresh4).is_null() {
            j = 0 as libc::c_int;
            while j < i {
                free((*ht.offset(j as isize)).table as *mut libc::c_void);
                j += 1;
                j;
            }
            free(ht as *mut libc::c_void);
            return 0 as *mut htable;
        }
        j = 0 as libc::c_int;
        while j < size {
            let ref mut fresh5 = (*((*ht.offset(i as isize)).table).offset(j as isize))
                .list;
            *fresh5 = 0 as *mut hentry;
            pthread_spin_init(
                &mut (*((*ht.offset(i as isize)).table).offset(j as isize)).lock,
                0 as libc::c_int,
            );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return ht;
}
pub unsafe extern "C" fn find_io_from_he(
    mut he: *mut hentry,
    mut limit: uint32_t,
    mut rbt: *mut rbtree,
    mut ttl_update: libc::c_int,
) {
    let mut mv: *mut mvalue = 0 as *mut mvalue;
    let mut i: libc::c_int = 0;
    let mut val_num: libc::c_int = 9 as libc::c_int;
    let mut val: *mut uchar = 0 as *mut uchar;
    let mut now: time_t = global_now;
    let mut tn: ttlnode = ttlnode {
        exp: 0,
        dlen: 0,
        type_0: 0,
        hash: 0 as *mut hashval_t,
        lowerdomain: 0 as *mut packet_type,
        data: 0 as *mut uchar,
    };
    let mut ptn: *mut ttlnode = 0 as *mut ttlnode;
    let mut pn: *mut rbnode = 0 as *mut rbnode;
    if (*he).count > 0 as libc::c_int {} else {
        __assert_fail(
            b"he->count > 0\0" as *const u8 as *const libc::c_char,
            b"storage.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 70],
                &[libc::c_char; 70],
            >(
                b"void find_io_from_he(struct hentry *, uint32_t, struct rbtree *, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_7564: {
        if (*he).count > 0 as libc::c_int {} else {
            __assert_fail(
                b"he->count > 0\0" as *const u8 as *const libc::c_char,
                b"storage.c\0" as *const u8 as *const libc::c_char,
                162 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 70],
                    &[libc::c_char; 70],
                >(
                    b"void find_io_from_he(struct hentry *, uint32_t, struct rbtree *, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int;
    while i < val_num {
        val = (*he).c2rust_unnamed.vals[i as usize];
        if !val.is_null() {
            mv = val as *mut mvalue;
            if (*mv).ttl as libc::c_long
                > now + ttl_update as libc::c_long + 1 as libc::c_int as libc::c_long
                && (*mv).hits < limit
            {
                tn.data = ((*he).key).as_mut_ptr();
                tn.type_0 = *support_type.as_ptr().offset(i as isize) as ushort;
                tn.exp = (*mv).ttl;
                tn
                    .dlen = (strlen(((*he).key).as_mut_ptr() as *const libc::c_char))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as ushort;
                tn.lowerdomain = 0 as *mut packet_type;
                pthread_spin_lock(&mut (*rbt).lock);
                pn = find_node(rbt, &mut tn as *mut ttlnode as *mut libc::c_void);
                if !pn.is_null() {
                    ptn = delete_node(rbt, pn) as *mut ttlnode;
                    if !ptn.is_null() {
                        free((*ptn).lowerdomain as *mut libc::c_void);
                        free(ptn as *mut libc::c_void);
                    } else {
                        printf(b"delete error\n\0" as *const u8 as *const libc::c_char);
                    }
                }
                pthread_spin_unlock(&mut (*rbt).lock);
                free(val as *mut libc::c_void);
                (*he).c2rust_unnamed.vals[i as usize] = 0 as *mut uchar;
                (*he).count -= 1;
                (*he).count;
            }
            if 0 as libc::c_int == (*he).count {
                break;
            }
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn htable_find_io(
    mut ht: *mut htable,
    mut idx: libc::c_int,
    mut limit: uint32_t,
    mut rbt: *mut rbtree,
    mut ttl_update: libc::c_int,
) -> libc::c_int {
    let mut debug: libc::c_int = DEBUG_TIMES as libc::c_int;
    let mut hd: *mut hdata = 0 as *mut hdata;
    let mut he: *mut hentry = 0 as *mut hentry;
    let mut prev: *mut hentry = 0 as *mut hentry;
    let mut tmp: *mut hentry = 0 as *mut hentry;
    if idx > 65536 as libc::c_int {
        return -(1 as libc::c_int);
    }
    hd = ((*ht).table).offset(idx as isize);
    pthread_spin_lock(&mut (*hd).lock);
    if ((*hd).list).is_null() {
        pthread_spin_unlock(&mut (*hd).lock);
        return -(1 as libc::c_int);
    }
    he = (*hd).list;
    while !he.is_null() {
        find_io_from_he(he, limit, rbt, ttl_update);
        if 0 as libc::c_int == (*he).count {
            tmp = he;
            if prev.is_null() {
                (*hd).list = (*he).next;
            } else {
                (*prev).next = (*he).next;
            }
            he = (*he).next;
            free(tmp as *mut libc::c_void);
            (*hd).now = ((*hd).now).wrapping_sub(1);
            (*hd).now;
            pthread_spin_lock(&mut (*ht).lock);
            (*ht).now = ((*ht).now).wrapping_sub(1);
            (*ht).now;
            pthread_spin_unlock(&mut (*ht).lock);
        } else {
            prev = he;
            he = (*he).next;
        }
        debug -= 1;
        debug;
        if debug == 0 as libc::c_int {
            printf(b"error in storage...\n\0" as *const u8 as *const libc::c_char);
            exit(0 as libc::c_int);
        }
    }
    pthread_spin_unlock(&mut (*hd).lock);
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn get_val_from_he(
    mut he: *mut hentry,
    mut type_0: libc::c_int,
) -> *mut uchar {
    let mut val: *mut uchar = 0 as *mut uchar;
    if (*he).count > 0 as libc::c_int {} else {
        __assert_fail(
            b"he->count > 0\0" as *const u8 as *const libc::c_char,
            b"storage.c\0" as *const u8 as *const libc::c_char,
            263 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"uchar *get_val_from_he(struct hentry *, int)\0"))
                .as_ptr(),
        );
    }
    'c_6990: {
        if (*he).count > 0 as libc::c_int {} else {
            __assert_fail(
                b"he->count > 0\0" as *const u8 as *const libc::c_char,
                b"storage.c\0" as *const u8 as *const libc::c_char,
                263 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"uchar *get_val_from_he(struct hentry *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    match type_0 {
        1 => {
            val = (*he).c2rust_unnamed.val.A;
        }
        2 => {
            val = (*he).c2rust_unnamed.val.NS;
        }
        5 => {
            val = (*he).c2rust_unnamed.val.CNAME;
        }
        6 => {
            val = (*he).c2rust_unnamed.val.SOA;
        }
        15 => {
            val = (*he).c2rust_unnamed.val.MX;
        }
        16 => {
            val = (*he).c2rust_unnamed.val.TXT;
        }
        28 => {
            val = (*he).c2rust_unnamed.val.AAAA;
        }
        33 => {
            val = (*he).c2rust_unnamed.val.SRV;
        }
        12 => {
            val = (*he).c2rust_unnamed.val.PTR;
        }
        _ => {
            val = 0 as *mut uchar;
        }
    }
    return val;
}
pub unsafe extern "C" fn htable_find(
    mut ht: *mut htable,
    mut key: *mut uchar,
    mut klen: libc::c_int,
    mut type_0: libc::c_int,
    mut buffer: *mut uchar,
    mut vlen: libc::c_int,
    mut md: *mut mvalue,
    mut hashd: *mut hashval_t,
) -> libc::c_int {
    let mut idx: libc::c_int = 0;
    let mut debug: libc::c_int = DEBUG_TIMES as libc::c_int;
    let mut ret: libc::c_int = 0;
    let mut hd: *mut hdata = 0 as *mut hdata;
    let mut he: *mut hentry = 0 as *mut hentry;
    let mut mx: *mut mvalue = 0 as *mut mvalue;
    let mut val: *mut uchar = 0 as *mut uchar;
    if *hashd == 0 as libc::c_int as libc::c_uint {
        *hashd = ((*ht).h).unwrap()(key as *mut libc::c_void, klen);
    }
    idx = (*hashd & (*ht).mask) as libc::c_int;
    hd = ((*ht).table).offset(idx as isize);
    pthread_spin_lock(&mut (*hd).lock);
    if ((*hd).list).is_null() {
        pthread_spin_unlock(&mut (*hd).lock);
        return -(1 as libc::c_int);
    }
    he = (*hd).list;
    while !he.is_null() {
        if ((*ht).c)
            .unwrap()(
            key as *mut libc::c_void,
            ((*he).key).as_mut_ptr() as *mut libc::c_void,
        ) == 0 as libc::c_int
        {
            val = get_val_from_he(he, type_0);
            if val.is_null() {
                ret = -(1 as libc::c_int);
            } else if !buffer.is_null() {
                ret = deep_copy(val, buffer, vlen);
            } else {
                if !md.is_null() {
                    mx = val as *mut mvalue;
                    *md = *mx;
                }
                ret = 1 as libc::c_int;
            }
            pthread_spin_unlock(&mut (*hd).lock);
            return ret;
        }
        he = (*he).next;
        let fresh6 = debug;
        debug = debug - 1;
        if fresh6 == 0 as libc::c_int {
            printf(b"error in htable find\n\0" as *const u8 as *const libc::c_char);
            exit(0 as libc::c_int);
        }
    }
    pthread_spin_unlock(&mut (*hd).lock);
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn find_list_io_from_he(
    mut he: *mut hentry,
    mut typeoff: *mut libc::c_int,
    mut buffer: *mut *mut uchar,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut val_num: libc::c_int = 9 as libc::c_int;
    let mut val: *mut uchar = 0 as *mut uchar;
    if (*he).count > 0 as libc::c_int {} else {
        __assert_fail(
            b"he->count > 0\0" as *const u8 as *const libc::c_char,
            b"storage.c\0" as *const u8 as *const libc::c_char,
            356 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"int find_list_io_from_he(struct hentry *, int *, uchar **)\0"))
                .as_ptr(),
        );
    }
    'c_8036: {
        if (*he).count > 0 as libc::c_int {} else {
            __assert_fail(
                b"he->count > 0\0" as *const u8 as *const libc::c_char,
                b"storage.c\0" as *const u8 as *const libc::c_char,
                356 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"int find_list_io_from_he(struct hentry *, int *, uchar **)\0"))
                    .as_ptr(),
            );
        }
    };
    i = *typeoff;
    while i < val_num {
        val = (*he).c2rust_unnamed.vals[i as usize];
        if val.is_null() {
            i += 1;
            i;
        } else {
            *buffer = val;
            *typeoff = i;
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn htable_find_list_io(
    mut ht: *mut htable,
    mut idx: libc::c_int,
    mut off: libc::c_int,
    mut typeoff: *mut libc::c_int,
    mut buffer: *mut *mut uchar,
) -> libc::c_int {
    let mut debug: libc::c_int = DEBUG_TIMES as libc::c_int;
    let mut ret: libc::c_int = 0;
    let mut hd: *mut hdata = 0 as *mut hdata;
    let mut he: *mut hentry = 0 as *mut hentry;
    hd = ((*ht).table).offset(idx as isize);
    pthread_spin_lock(&mut (*hd).lock);
    if ((*hd).list).is_null() {
        pthread_spin_unlock(&mut (*hd).lock);
        return -(1 as libc::c_int);
    }
    he = (*hd).list;
    while !he.is_null() {
        if off == 0 as libc::c_int {
            ret = find_list_io_from_he(he, typeoff, buffer);
            pthread_spin_unlock(&mut (*hd).lock);
            return ret;
        }
        off -= 1;
        off;
        he = (*he).next;
        let fresh7 = debug;
        debug = debug - 1;
        if fresh7 == 0 as libc::c_int {
            printf(
                b"error in htable find list io\n\0" as *const u8 as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        }
    }
    pthread_spin_unlock(&mut (*hd).lock);
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn get_list_val_from_he(
    mut he: *mut hentry,
    mut typeoff: libc::c_int,
    mut buffer: *mut *mut uchar,
) -> libc::c_int {
    let mut val: *mut uchar = 0 as *mut uchar;
    if (*he).count > 0 as libc::c_int {} else {
        __assert_fail(
            b"he->count > 0\0" as *const u8 as *const libc::c_char,
            b"storage.c\0" as *const u8 as *const libc::c_char,
            408 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"int get_list_val_from_he(struct hentry *, int, uchar **)\0"))
                .as_ptr(),
        );
    }
    'c_9311: {
        if (*he).count > 0 as libc::c_int {} else {
            __assert_fail(
                b"he->count > 0\0" as *const u8 as *const libc::c_char,
                b"storage.c\0" as *const u8 as *const libc::c_char,
                408 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"int get_list_val_from_he(struct hentry *, int, uchar **)\0"))
                    .as_ptr(),
            );
        }
    };
    val = (*he).c2rust_unnamed.vals[typeoff as usize];
    *buffer = val;
    if val.is_null() {
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn htable_find_list(
    mut ht: *mut htable,
    mut key: *mut uchar,
    mut typeoff: libc::c_int,
    mut idx: libc::c_int,
    mut buffer: *mut *mut uchar,
) -> libc::c_int {
    let mut debug: libc::c_int = DEBUG_TIMES as libc::c_int;
    let mut hd: *mut hdata = 0 as *mut hdata;
    let mut he: *mut hentry = 0 as *mut hentry;
    let mut mbuf: *mut mbuf_type = 0 as *mut mbuf_type;
    hd = ((*ht).table).offset(idx as isize);
    pthread_spin_lock(&mut (*hd).lock);
    if ((*hd).list).is_null() {
        pthread_spin_unlock(&mut (*hd).lock);
        return -(1 as libc::c_int);
    }
    he = (*hd).list;
    while !he.is_null() {
        mbuf = (*he).c2rust_unnamed.vals[typeoff as usize] as *mut mbuf_type;
        if !mbuf.is_null()
            && ((*ht).c)
                .unwrap()(key as *mut libc::c_void, (*mbuf).qing as *mut libc::c_void)
                == 0 as libc::c_int
        {
            *buffer = mbuf as *mut uchar;
            pthread_spin_unlock(&mut (*hd).lock);
            return 1 as libc::c_int;
        }
        he = (*he).next;
        let fresh8 = debug;
        debug = debug - 1;
        if fresh8 == 0 as libc::c_int {
            printf(b"error in htable find\n\0" as *const u8 as *const libc::c_char);
            exit(0 as libc::c_int);
        }
    }
    pthread_spin_unlock(&mut (*hd).lock);
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn delete_val_from_he(
    mut he: *mut hentry,
    mut type_0: libc::c_int,
) -> *mut uchar {
    let mut oval: *mut *mut uchar = 0 as *mut *mut uchar;
    let mut val: *mut uchar = 0 as *mut uchar;
    if (*he).count > 0 as libc::c_int {} else {
        __assert_fail(
            b"he->count > 0\0" as *const u8 as *const libc::c_char,
            b"storage.c\0" as *const u8 as *const libc::c_char,
            453 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"uchar *delete_val_from_he(struct hentry *, int)\0"))
                .as_ptr(),
        );
    }
    'c_6486: {
        if (*he).count > 0 as libc::c_int {} else {
            __assert_fail(
                b"he->count > 0\0" as *const u8 as *const libc::c_char,
                b"storage.c\0" as *const u8 as *const libc::c_char,
                453 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"uchar *delete_val_from_he(struct hentry *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    match type_0 {
        1 => {
            oval = &mut (*he).c2rust_unnamed.val.A;
        }
        2 => {
            oval = &mut (*he).c2rust_unnamed.val.NS;
        }
        5 => {
            oval = &mut (*he).c2rust_unnamed.val.CNAME;
        }
        6 => {
            oval = &mut (*he).c2rust_unnamed.val.SOA;
        }
        15 => {
            oval = &mut (*he).c2rust_unnamed.val.MX;
        }
        16 => {
            oval = &mut (*he).c2rust_unnamed.val.TXT;
        }
        28 => {
            oval = &mut (*he).c2rust_unnamed.val.AAAA;
        }
        33 => {
            oval = &mut (*he).c2rust_unnamed.val.SRV;
        }
        12 => {
            oval = &mut (*he).c2rust_unnamed.val.PTR;
        }
        _ => return 0 as *mut uchar,
    }
    if !(*oval).is_null() {
        val = *oval;
        *oval = 0 as *mut uchar;
        (*he).count -= 1;
        (*he).count;
    }
    return val;
}
pub unsafe extern "C" fn htable_delete(
    mut ht: *mut htable,
    mut key: *mut uchar,
    mut klen: libc::c_int,
    mut type_0: libc::c_int,
    mut hashd: hashval_t,
) -> *mut uchar {
    let mut h: hashval_t = if hashd != 0 {
        hashd
    } else {
        ((*ht).h).unwrap()(key as *mut libc::c_void, klen)
    };
    let mut idx: libc::c_int = (h & (*ht).mask) as libc::c_int;
    let mut debug: libc::c_int = DEBUG_TIMES as libc::c_int;
    let mut hd: *mut hdata = 0 as *mut hdata;
    let mut he: *mut hentry = 0 as *mut hentry;
    let mut prev: *mut hentry = 0 as *mut hentry;
    hd = ((*ht).table).offset(idx as isize);
    let mut val: *mut uchar = 0 as *mut uchar;
    pthread_spin_lock(&mut (*hd).lock);
    if ((*hd).list).is_null() {
        pthread_spin_unlock(&mut (*hd).lock);
        return 0 as *mut uchar;
    }
    he = (*hd).list;
    while !he.is_null() {
        if ((*ht).c)
            .unwrap()(
            key as *mut libc::c_void,
            ((*he).key).as_mut_ptr() as *mut libc::c_void,
        ) == 0 as libc::c_int
        {
            val = delete_val_from_he(he, type_0);
            if 0 as libc::c_int == (*he).count {
                if prev.is_null() {
                    (*hd).list = (*he).next;
                } else {
                    (*prev).next = (*he).next;
                }
                free(he as *mut libc::c_void);
                (*hd).now = ((*hd).now).wrapping_sub(1);
                (*hd).now;
                pthread_spin_lock(&mut (*ht).lock);
                (*ht).now = ((*ht).now).wrapping_sub(1);
                (*ht).now;
                pthread_spin_unlock(&mut (*ht).lock);
            }
            pthread_spin_unlock(&mut (*hd).lock);
            return val;
        }
        prev = he;
        he = (*he).next;
        debug -= 1;
        debug;
        if debug == 0 as libc::c_int {
            printf(b"error in storage\n\0" as *const u8 as *const libc::c_char);
            exit(0 as libc::c_int);
        }
    }
    pthread_spin_unlock(&mut (*hd).lock);
    return 0 as *mut uchar;
}
pub unsafe extern "C" fn delete_list_val_from_he(
    mut he: *mut hentry,
    mut typeoff: libc::c_int,
) -> *mut uchar {
    let mut oval: *mut *mut uchar = 0 as *mut *mut uchar;
    let mut val: *mut uchar = 0 as *mut uchar;
    if (*he).count > 0 as libc::c_int {} else {
        __assert_fail(
            b"he->count > 0\0" as *const u8 as *const libc::c_char,
            b"storage.c\0" as *const u8 as *const libc::c_char,
            549 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"uchar *delete_list_val_from_he(struct hentry *, int)\0"))
                .as_ptr(),
        );
    }
    'c_8537: {
        if (*he).count > 0 as libc::c_int {} else {
            __assert_fail(
                b"he->count > 0\0" as *const u8 as *const libc::c_char,
                b"storage.c\0" as *const u8 as *const libc::c_char,
                549 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"uchar *delete_list_val_from_he(struct hentry *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    oval = &mut *((*he).c2rust_unnamed.vals).as_mut_ptr().offset(typeoff as isize)
        as *mut *mut uchar;
    if !(*oval).is_null() {
        val = *oval;
        *oval = 0 as *mut uchar;
        (*he).count -= 1;
        (*he).count;
    }
    return val;
}
pub unsafe extern "C" fn htable_delete_list_io(
    mut ht: *mut htable,
    mut typeoff: libc::c_int,
    mut idx: libc::c_int,
    mut off: libc::c_int,
) -> *mut uchar {
    let mut debug: libc::c_int = DEBUG_TIMES as libc::c_int;
    let mut hd: *mut hdata = 0 as *mut hdata;
    let mut he: *mut hentry = 0 as *mut hentry;
    let mut prev: *mut hentry = 0 as *mut hentry;
    let mut val: *mut uchar = 0 as *mut uchar;
    hd = ((*ht).table).offset(idx as isize);
    pthread_spin_lock(&mut (*hd).lock);
    if ((*hd).list).is_null() {
        pthread_spin_unlock(&mut (*hd).lock);
        return 0 as *mut uchar;
    }
    he = (*hd).list;
    while !he.is_null() {
        if off == 0 as libc::c_int {
            val = delete_list_val_from_he(he, typeoff);
            if 0 as libc::c_int == (*he).count {
                if prev.is_null() {
                    (*hd).list = (*he).next;
                } else {
                    (*prev).next = (*he).next;
                }
                free(he as *mut libc::c_void);
                (*hd).now = ((*hd).now).wrapping_sub(1);
                (*hd).now;
                pthread_spin_lock(&mut (*ht).lock);
                (*ht).now = ((*ht).now).wrapping_sub(1);
                (*ht).now;
                pthread_spin_unlock(&mut (*ht).lock);
            }
            pthread_spin_unlock(&mut (*hd).lock);
            return val;
        }
        off -= 1;
        off;
        prev = he;
        he = (*he).next;
        let fresh9 = debug;
        debug = debug - 1;
        if fresh9 == 0 as libc::c_int {
            printf(
                b"error in htable find list io\n\0" as *const u8 as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        }
    }
    pthread_spin_unlock(&mut (*hd).lock);
    return 0 as *mut uchar;
}
pub unsafe extern "C" fn htable_delete_list(
    mut ht: *mut htable,
    mut key: *mut uchar,
    mut typeoff: libc::c_int,
    mut idx: libc::c_int,
) -> *mut uchar {
    let mut debug: libc::c_int = DEBUG_TIMES as libc::c_int;
    let mut hd: *mut hdata = 0 as *mut hdata;
    let mut he: *mut hentry = 0 as *mut hentry;
    let mut prev: *mut hentry = 0 as *mut hentry;
    let mut val: *mut uchar = 0 as *mut uchar;
    hd = ((*ht).table).offset(idx as isize);
    pthread_spin_lock(&mut (*hd).lock);
    if ((*hd).list).is_null() {
        pthread_spin_unlock(&mut (*hd).lock);
        return 0 as *mut uchar;
    }
    he = (*hd).list;
    while !he.is_null() {
        if ((*ht).c)
            .unwrap()(
            key as *mut libc::c_void,
            ((*he).key).as_mut_ptr() as *mut libc::c_void,
        ) == 0 as libc::c_int
        {
            val = delete_list_val_from_he(he, typeoff);
            if 0 as libc::c_int == (*he).count {
                if prev.is_null() {
                    (*hd).list = (*he).next;
                } else {
                    (*prev).next = (*he).next;
                }
                free(he as *mut libc::c_void);
                (*hd).now = ((*hd).now).wrapping_sub(1);
                (*hd).now;
                pthread_spin_lock(&mut (*ht).lock);
                (*ht).now = ((*ht).now).wrapping_sub(1);
                (*ht).now;
                pthread_spin_unlock(&mut (*ht).lock);
            }
            pthread_spin_unlock(&mut (*hd).lock);
            return val;
        }
        prev = he;
        he = (*he).next;
        let fresh10 = debug;
        debug = debug - 1;
        if fresh10 == 0 as libc::c_int {
            printf(
                b"error in htable find list io\n\0" as *const u8 as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        }
    }
    pthread_spin_unlock(&mut (*hd).lock);
    return 0 as *mut uchar;
}
pub unsafe extern "C" fn append_value_to_he(
    mut he: *mut hentry,
    mut val: *mut uchar,
    mut type_0: libc::c_int,
    mut replace: libc::c_int,
    mut mv: *mut mvalue,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut oval: *mut *mut uchar = 0 as *mut *mut uchar;
    match type_0 {
        1 => {
            oval = &mut (*he).c2rust_unnamed.val.A;
        }
        2 => {
            oval = &mut (*he).c2rust_unnamed.val.NS;
        }
        5 => {
            oval = &mut (*he).c2rust_unnamed.val.CNAME;
        }
        6 => {
            oval = &mut (*he).c2rust_unnamed.val.SOA;
        }
        15 => {
            oval = &mut (*he).c2rust_unnamed.val.MX;
        }
        16 => {
            oval = &mut (*he).c2rust_unnamed.val.TXT;
        }
        28 => {
            oval = &mut (*he).c2rust_unnamed.val.AAAA;
        }
        33 => {
            oval = &mut (*he).c2rust_unnamed.val.SRV;
        }
        12 => {
            oval = &mut (*he).c2rust_unnamed.val.PTR;
        }
        _ => return HTABLE_INSERT_RET_INVALID_TYPE as libc::c_int,
    }
    if !(*oval).is_null() {
        if replace != 0 {
            if !mv.is_null() {
                *mv = *(*oval as *mut mvalue);
            }
            if !mv.is_null()
                && (*mv).ttl
                    != (7 as libc::c_int * 86400 as libc::c_int + 1 as libc::c_int)
                        as libc::c_uint
            {
                if NS as libc::c_int == type_0 {
                    (*(val as *mut mvalue)).ttl = (*mv).ttl;
                }
                free(*oval as *mut libc::c_void);
                *oval = val;
                ret = HTABLE_INSERT_RET_REPLACE as libc::c_int;
            } else {
                ret = HTABLE_INSERT_RET_NEVER_EXPIRE as libc::c_int;
            }
        } else {
            ret = HTABLE_INSERT_RET_NO_REPLACE as libc::c_int;
        }
    } else {
        (*he).count += 1;
        (*he).count;
        *oval = val;
        ret = HTABLE_INSERT_RET_NORMAL as libc::c_int;
    }
    return ret;
}
pub unsafe extern "C" fn htable_insert(
    mut ht: *mut htable,
    mut key: *mut uchar,
    mut klen: libc::c_int,
    mut type_0: libc::c_int,
    mut val: *mut uchar,
    mut replace: libc::c_int,
    mut mv: *mut mvalue,
    mut hashd: *mut hashval_t,
) -> libc::c_int {
    let mut idx: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut debug: libc::c_int = DEBUG_TIMES as libc::c_int;
    let mut he: *mut hentry = 0 as *mut hentry;
    let mut cl: *mut hentry = 0 as *mut hentry;
    let mut hd: *mut hdata = 0 as *mut hdata;
    let mut dlen: uchar = klen as uchar;
    if check_support_type(type_0 as ushort) == -(1 as libc::c_int) {
        printf(
            b"invalud type:%d, key:[%s]\n\0" as *const u8 as *const libc::c_char,
            type_0,
            key,
        );
        return -(1 as libc::c_int);
    }
    he = malloc(
        (::std::mem::size_of::<hentry>() as libc::c_ulong)
            .wrapping_add(dlen as libc::c_ulong),
    ) as *mut hentry;
    if he.is_null() {
        printf(b"oom\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    memset(
        he as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<hentry>() as libc::c_ulong,
    );
    memcpy(
        ((*he).key).as_mut_ptr() as *mut libc::c_void,
        key as *const libc::c_void,
        dlen as libc::c_ulong,
    );
    if *hashd == 0 as libc::c_int as libc::c_uint {
        *hashd = ((*ht).h).unwrap()(key as *mut libc::c_void, klen);
    }
    idx = (*hashd & (*ht).mask) as libc::c_int;
    hd = ((*ht).table).offset(idx as isize);
    pthread_spin_lock(&mut (*hd).lock);
    if ((*hd).list).is_null() {
        ret = append_value_to_he(he, val, type_0, replace, 0 as *mut mvalue);
        (*hd).now = 1 as libc::c_int as uint64_t;
        (*hd).list = he;
    } else {
        cl = (*hd).list;
        while !cl.is_null() {
            if ((*ht).c)
                .unwrap()(
                ((*cl).key).as_mut_ptr() as *mut libc::c_void,
                ((*he).key).as_mut_ptr() as *mut libc::c_void,
            ) == 0 as libc::c_int
            {
                ret = append_value_to_he(cl, val, type_0, replace, mv);
                pthread_spin_unlock(&mut (*hd).lock);
                free(he as *mut libc::c_void);
                return ret;
            }
            cl = (*cl).next;
            debug -= 1;
            debug;
            if debug == 0 as libc::c_int {
                printf(b"error in storage2\n\0" as *const u8 as *const libc::c_char);
                exit(0 as libc::c_int);
            }
        }
        ret = append_value_to_he(he, val, type_0, replace, 0 as *mut mvalue);
        (*he).next = (*hd).list;
        (*hd).list = he;
        (*hd).now = ((*hd).now).wrapping_add(1);
        (*hd).now;
    }
    pthread_spin_unlock(&mut (*hd).lock);
    pthread_spin_lock(&mut (*ht).lock);
    (*ht).now = ((*ht).now).wrapping_add(1);
    (*ht).now;
    pthread_spin_unlock(&mut (*ht).lock);
    return HTABLE_INSERT_RET_NORMAL as libc::c_int;
}
pub unsafe extern "C" fn htable_insert_list(
    mut ht: *mut htable,
    mut key: *mut uchar,
    mut klen: libc::c_int,
    mut type_0: libc::c_int,
    mut val: *mut uchar,
    mut replace: libc::c_int,
    mut mv: *mut mvalue,
    mut hashd: *mut hashval_t,
) -> libc::c_int {
    let mut idx: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut debug: libc::c_int = DEBUG_TIMES as libc::c_int;
    let mut he: *mut hentry = 0 as *mut hentry;
    let mut cl: *mut hentry = 0 as *mut hentry;
    let mut prev: *mut hentry = 0 as *mut hentry;
    let mut hd: *mut hdata = 0 as *mut hdata;
    let mut dlen: uchar = klen as uchar;
    if check_support_type(type_0 as ushort) == -(1 as libc::c_int) {
        printf(
            b"invalud type:%d, key:[%s]\n\0" as *const u8 as *const libc::c_char,
            type_0,
            key,
        );
        return -(1 as libc::c_int);
    }
    he = malloc(
        (::std::mem::size_of::<hentry>() as libc::c_ulong)
            .wrapping_add(dlen as libc::c_ulong),
    ) as *mut hentry;
    if he.is_null() {
        printf(b"oom\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    memset(
        he as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<hentry>() as libc::c_ulong,
    );
    memcpy(
        ((*he).key).as_mut_ptr() as *mut libc::c_void,
        key as *const libc::c_void,
        dlen as libc::c_ulong,
    );
    if *hashd == 0 as libc::c_int as libc::c_uint {
        *hashd = ((*ht).h).unwrap()(key as *mut libc::c_void, klen);
    }
    idx = (*hashd & (*ht).mask) as libc::c_int;
    hd = ((*ht).table).offset(idx as isize);
    pthread_spin_lock(&mut (*hd).lock);
    if ((*hd).list).is_null() {
        ret = append_value_to_he(he, val, type_0, replace, 0 as *mut mvalue);
        (*hd).now = 1 as libc::c_int as uint64_t;
        (*hd).list = he;
    } else {
        cl = (*hd).list;
        while !cl.is_null() {
            if ((*ht).c)
                .unwrap()(
                ((*cl).key).as_mut_ptr() as *mut libc::c_void,
                ((*he).key).as_mut_ptr() as *mut libc::c_void,
            ) == 0 as libc::c_int
            {
                ret = append_value_to_he(cl, val, type_0, replace, mv);
                pthread_spin_unlock(&mut (*hd).lock);
                free(he as *mut libc::c_void);
                return ret;
            }
            prev = cl;
            cl = (*cl).next;
            debug -= 1;
            debug;
            if debug == 0 as libc::c_int {
                printf(b"error in storage3\n\0" as *const u8 as *const libc::c_char);
                exit(0 as libc::c_int);
            }
        }
        ret = append_value_to_he(he, val, type_0, replace, 0 as *mut mvalue);
        (*prev).next = he;
        (*hd).now = ((*hd).now).wrapping_add(1);
        (*hd).now;
    }
    pthread_spin_unlock(&mut (*hd).lock);
    pthread_spin_lock(&mut (*ht).lock);
    (*ht).now = ((*ht).now).wrapping_add(1);
    (*ht).now;
    pthread_spin_unlock(&mut (*ht).lock);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn find_record_with_ttl(
    mut ht: *mut htable,
    mut key: *mut uchar,
    mut klen: libc::c_int,
    mut type_0: libc::c_int,
    mut val: *mut uchar,
    mut vlen: libc::c_int,
    mut md: *mut mvalue,
    mut hash: *mut hashval_t,
) -> libc::c_int {
    let mut idx: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut oval: *mut uchar = 0 as *mut uchar;
    idx = get_pre_mem_hash(key as *mut libc::c_void, klen, hash) as libc::c_int;
    ret = htable_find(ht.offset(idx as isize), key, klen, type_0, val, vlen, md, hash);
    if ret > 0 as libc::c_int {
        if ttl_expired(val) == 1 as libc::c_int {
            oval = htable_delete(ht.offset(idx as isize), key, klen, type_0, *hash);
            if !oval.is_null() {
                free(oval as *mut libc::c_void);
            }
        } else {
            return ret
        }
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn st_th(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut i: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut key: [uchar; 50] = [
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
    ];
    let mut klen: libc::c_int = 0;
    let mut val: *mut uchar = 0 as *mut uchar;
    let mut pre: libc::c_int = 0 as libc::c_int;
    let mut oval: *mut uchar = 0 as *mut uchar;
    let mut ht: *mut htable = 0 as *mut htable;
    let mut sh: *mut st_hlp = arg as *mut st_hlp;
    let mut hash: hashval_t = 0;
    idx = (*sh).idx;
    ht = (*sh).ht;
    i = idx * 10000 as libc::c_int;
    while i < (idx + 1 as libc::c_int) * 10000 as libc::c_int {
        hash = 0 as libc::c_int as hashval_t;
        sprintf(
            key.as_mut_ptr() as *mut libc::c_char,
            b"%dkey\0" as *const u8 as *const libc::c_char,
            i,
        );
        val = malloc(50 as libc::c_int as libc::c_ulong) as *mut uchar;
        sprintf(
            val as *mut libc::c_char,
            b"%dval\0" as *const u8 as *const libc::c_char,
            i,
        );
        klen = (strlen(key.as_mut_ptr() as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        pre = get_pre_mem_hash(key.as_mut_ptr() as *mut libc::c_void, klen, &mut hash)
            as libc::c_int;
        htable_insert(
            ht.offset(pre as isize),
            key.as_mut_ptr(),
            klen,
            A as libc::c_int,
            val,
            0 as libc::c_int,
            0 as *mut mvalue,
            &mut hash,
        );
        i += 1;
        i;
    }
    if idx == 5 as libc::c_int - 1 as libc::c_int {
        idx = -(1 as libc::c_int);
    }
    sleep(2 as libc::c_int as libc::c_uint);
    i = (idx + 1 as libc::c_int) * 10000 as libc::c_int;
    while i < (idx + 2 as libc::c_int) * 10000 as libc::c_int {
        hash = 0 as libc::c_int as hashval_t;
        sprintf(
            key.as_mut_ptr() as *mut libc::c_char,
            b"%dkey\0" as *const u8 as *const libc::c_char,
            i,
        );
        klen = (strlen(key.as_mut_ptr() as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        pre = get_pre_mem_hash(key.as_mut_ptr() as *mut libc::c_void, klen, &mut hash)
            as libc::c_int;
        oval = htable_delete(
            ht.offset(pre as isize),
            key.as_mut_ptr(),
            klen,
            A as libc::c_int,
            hash,
        );
        if oval.is_null() {
            printf(
                b"error in test %s,%d,%d\n\0" as *const u8 as *const libc::c_char,
                key.as_mut_ptr(),
                idx,
                i,
            );
        } else {
            free(oval as *mut libc::c_void);
        }
        i += 1;
        i;
    }
    sleep(5 as libc::c_int as libc::c_uint);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn storage_test() -> libc::c_int {
    let mut ht: *mut htable = 0 as *mut htable;
    let mut pt: [pthread_t; 5] = [0; 5];
    let mut i: libc::c_int = 0;
    let mut sh: [st_hlp; 5] = [st_hlp {
        ht: 0 as *mut htable,
        idx: 0,
    }; 5];
    if ht.is_null() {
        dns_error(
            0 as libc::c_int,
            b"create htable error\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        sh[i as usize].ht = ht;
        sh[i as usize].idx = i;
        if pthread_create(
            pt.as_mut_ptr().offset(i as isize),
            0 as *const pthread_attr_t,
            Some(st_th as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
            sh.as_mut_ptr().offset(i as isize) as *mut libc::c_void,
        ) != 0
        {
            dns_error(
                0 as libc::c_int,
                b"create pthread\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        pthread_join(pt[i as usize], 0 as *mut *mut libc::c_void);
        i += 1;
        i;
    }
    sleep(2 as libc::c_int as libc::c_uint);
    return 0 as libc::c_int;
}
