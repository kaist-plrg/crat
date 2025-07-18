use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    fn free_skb(skb: *mut sk_buff);
    fn checksum(
        addr: *mut libc::c_void,
        count: libc::c_int,
        start_sum: libc::c_int,
    ) -> uint16_t;
    fn icmpv4_incoming(skb: *mut sk_buff);
    fn tcp_in(skb: *mut sk_buff);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub struct netdev {
    pub addr: uint32_t,
    pub addr_len: uint8_t,
    pub hwaddr: [uint8_t; 6],
    pub mtu: uint32_t,
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
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eth_hdr {
    pub dmac: [uint8_t; 6],
    pub smac: [uint8_t; 6],
    pub ethertype: uint16_t,
    pub payload: [uint8_t; 0],
}
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
#[inline]
unsafe extern "C" fn ip_hdr(mut skb: *const sk_buff) -> *mut iphdr {
    return ((*skb).head)
        .offset(::std::mem::size_of::<eth_hdr>() as libc::c_ulong as isize)
        as *mut iphdr;
}
unsafe extern "C" fn ip_init_pkt(mut ih: *mut iphdr) {
    (*ih).saddr = ntohl((*ih).saddr);
    (*ih).daddr = ntohl((*ih).daddr);
    (*ih).len = ntohs((*ih).len);
    (*ih).id = ntohs((*ih).id);
}
pub unsafe extern "C" fn ip_rcv(mut skb: *mut sk_buff) -> libc::c_int {
    let mut ih: *mut iphdr = ip_hdr(skb);
    let mut csum: uint16_t = -(1 as libc::c_int) as uint16_t;
    if (*ih).version() as libc::c_int != 0x4 as libc::c_int {
        fprintf(
            stderr,
            b"Datagram version was not IPv4\n\0" as *const u8 as *const libc::c_char,
        );
    } else if ((*ih).ihl() as libc::c_int) < 5 as libc::c_int {
        fprintf(
            stderr,
            b"IPv4 header length must be at least 5\n\0" as *const u8
                as *const libc::c_char,
        );
    } else if (*ih).ttl as libc::c_int == 0 as libc::c_int {
        fprintf(
            stderr,
            b"Time to live of datagram reached 0\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        csum = checksum(
            ih as *mut libc::c_void,
            (*ih).ihl() as libc::c_int * 4 as libc::c_int,
            0 as libc::c_int,
        );
        if !(csum as libc::c_int != 0 as libc::c_int) {
            ip_init_pkt(ih);
            match (*ih).proto as libc::c_int {
                1 => {
                    icmpv4_incoming(skb);
                    return 0 as libc::c_int;
                }
                6 => {
                    tcp_in(skb);
                    return 0 as libc::c_int;
                }
                _ => {
                    fprintf(
                        stderr,
                        b"Unknown IP header proto\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
        }
    }
    free_skb(skb);
    return 0 as libc::c_int;
}
