use ::libc;
use std::arch::asm;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn dns_error(_: libc::c_int, _: *mut libc::c_char);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type ushort = libc::c_ushort;
pub type uint = libc::c_uint;
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
pub static mut mbuf_ring: *mut mbuf_ring = 0 as *const mbuf_ring as *mut mbuf_ring;
pub unsafe extern "C" fn mbuf_ring_create(mut count: uint32_t) -> *mut mbuf_ring {
    let mut r: *mut mbuf_ring = 0 as *mut mbuf_ring;
    let mut ring_size: uint64_t = 0;
    ring_size = (count as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<mbuf_ring>() as libc::c_ulong);
    r = malloc(ring_size) as *mut mbuf_ring;
    if !r.is_null() {
        memset(
            r as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<mbuf_ring>() as libc::c_ulong,
        );
        (*r).prod.watermark = count;
        (*r).cons.size = count;
        (*r).prod.size = (*r).cons.size;
        (*r).cons.mask = count.wrapping_sub(1 as libc::c_int as libc::c_uint);
        (*r).prod.mask = (*r).cons.mask;
        ::std::ptr::write_volatile(
            &mut (*r).cons.head as *mut uint32_t,
            0 as libc::c_int as uint32_t,
        );
        ::std::ptr::write_volatile(
            &mut (*r).prod.head as *mut uint32_t,
            ::std::ptr::read_volatile::<uint32_t>(&(*r).cons.head as *const uint32_t),
        );
        ::std::ptr::write_volatile(
            &mut (*r).cons.tail as *mut uint32_t,
            0 as libc::c_int as uint32_t,
        );
        ::std::ptr::write_volatile(
            &mut (*r).prod.tail as *mut uint32_t,
            ::std::ptr::read_volatile::<uint32_t>(&(*r).cons.tail as *const uint32_t),
        );
    }
    return r;
}
pub unsafe extern "C" fn mempool_create(mut num: uint32_t) -> libc::c_int {
    let mut tmp: *mut mbuf_type = 0 as *mut mbuf_type;
    let mut i: libc::c_int = 0;
    mbuf_ring = mbuf_ring_create(num);
    if mbuf_ring.is_null() {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < num {
        tmp = malloc(::std::mem::size_of::<mbuf_type>() as libc::c_ulong)
            as *mut mbuf_type;
        if tmp.is_null() {
            return -(1 as libc::c_int);
        }
        (*tmp).mbuf = mbuf_ring;
        let ref mut fresh0 = *((*mbuf_ring).ring).as_mut_ptr().offset(i as isize);
        *fresh0 = tmp as *mut libc::c_void;
        i += 1;
        i;
    }
    ::std::ptr::write_volatile(
        &mut (*mbuf_ring).prod.tail as *mut uint32_t,
        num.wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    ::std::ptr::write_volatile(
        &mut (*mbuf_ring).prod.head as *mut uint32_t,
        ::std::ptr::read_volatile::<uint32_t>(&(*mbuf_ring).prod.tail as *const uint32_t),
    );
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn rte_atomic32_cmpset(
    mut dst: *mut uint32_t,
    mut exp: uint32_t,
    mut src: uint32_t,
) -> libc::c_int {
    let mut res: uint8_t = 0;
    asm!(
"lock cmpxchg [{dst}], {src}",
"sete {res}",
dst = in(reg) dst,
src = in(reg) src,
res = out(reg_byte) res,
inout("eax") exp => _,
options(nostack, preserves_flags),
    );
    return res as libc::c_int;
}
pub unsafe extern "C" fn mbuf_alloc() -> *mut mbuf_type {
    let mut cons_head: uint32_t = 0;
    let mut prod_tail: uint32_t = 0;
    let mut cons_next: uint32_t = 0;
    let mut entries: uint32_t = 0;
    let mut success: libc::c_int = 0;
    let mut mask: uint32_t = (*mbuf_ring).prod.mask;
    let mut mbuf: *mut mbuf_type = 0 as *mut mbuf_type;
    cons_head = (*mbuf_ring).cons.head;
    prod_tail = (*mbuf_ring).prod.tail;
    entries = prod_tail.wrapping_sub(cons_head);
    if 0 as libc::c_int as libc::c_uint == entries {
        dns_error(
            1 as libc::c_int,
            b"out of mbuf ring(memory pool)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as *mut mbuf_type;
    }
    cons_next = cons_head.wrapping_add(1 as libc::c_int as libc::c_uint);
    success = rte_atomic32_cmpset(&mut (*mbuf_ring).cons.head, cons_head, cons_next);
    if success != 1 as libc::c_int {
        return 0 as *mut mbuf_type;
    }
    mbuf = *((*mbuf_ring).ring).as_mut_ptr().offset((cons_head & mask) as isize)
        as *mut mbuf_type;
    if !mbuf.is_null() {} else {
        __assert_fail(
            b"mbuf != NULL\0" as *const u8 as *const libc::c_char,
            b"memory.c\0" as *const u8 as *const libc::c_char,
            129 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"mbuf_type *mbuf_alloc()\0"))
                .as_ptr(),
        );
    }
    'c_4603: {
        if !mbuf.is_null() {} else {
            __assert_fail(
                b"mbuf != NULL\0" as *const u8 as *const libc::c_char,
                b"memory.c\0" as *const u8 as *const libc::c_char,
                129 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 24],
                    &[libc::c_char; 24],
                >(b"mbuf_type *mbuf_alloc()\0"))
                    .as_ptr(),
            );
        }
    };
    asm!("", options(preserves_flags, att_syntax));
    while (*mbuf_ring).cons.tail != cons_head {}
    ::std::ptr::write_volatile(&mut (*mbuf_ring).cons.tail as *mut uint32_t, cons_next);
    return mbuf;
}
pub unsafe extern "C" fn mbuf_free(mut mbuf: *mut mbuf_type) -> libc::c_int {
    let mut prod_head: uint32_t = 0;
    let mut prod_next: uint32_t = 0;
    let mut cons_tail: uint32_t = 0;
    let mut free_entries: uint32_t = 0;
    let mut success: libc::c_int = 0;
    let mut mask: uint32_t = (*mbuf_ring).prod.mask;
    if mbuf.is_null() {
        return 0 as libc::c_int;
    }
    loop {
        prod_head = (*mbuf_ring).prod.head;
        cons_tail = (*mbuf_ring).cons.tail;
        free_entries = mask
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_add(cons_tail)
            .wrapping_sub(prod_head);
        if free_entries > 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"free_entries > 0\0" as *const u8 as *const libc::c_char,
                b"memory.c\0" as *const u8 as *const libc::c_char,
                155 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"int mbuf_free(mbuf_type *)\0"))
                    .as_ptr(),
            );
        }
        'c_4862: {
            if free_entries > 0 as libc::c_int as libc::c_uint {} else {
                __assert_fail(
                    b"free_entries > 0\0" as *const u8 as *const libc::c_char,
                    b"memory.c\0" as *const u8 as *const libc::c_char,
                    155 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 27],
                        &[libc::c_char; 27],
                    >(b"int mbuf_free(mbuf_type *)\0"))
                        .as_ptr(),
                );
            }
        };
        prod_next = prod_head.wrapping_add(1 as libc::c_int as libc::c_uint);
        success = rte_atomic32_cmpset(&mut (*mbuf_ring).prod.head, prod_head, prod_next);
        if !(0 as libc::c_int == success) {
            break;
        }
    }
    let ref mut fresh1 = *((*mbuf_ring).ring)
        .as_mut_ptr()
        .offset((prod_head & mask) as isize);
    *fresh1 = mbuf as *mut libc::c_void;
    asm!("", options(preserves_flags, att_syntax));
    while (*mbuf_ring).prod.tail != prod_head {}
    ::std::ptr::write_volatile(&mut (*mbuf_ring).prod.tail as *mut uint32_t, prod_next);
    return 0 as libc::c_int;
}
