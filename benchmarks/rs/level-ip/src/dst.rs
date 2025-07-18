use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    fn netdev_transmit(
        skb: *mut sk_buff,
        dst: *mut uint8_t,
        ethertype: uint16_t,
    ) -> libc::c_int;
    fn arp_request(sip: uint32_t, dip: uint32_t, netdev: *mut netdev) -> libc::c_int;
    fn arp_get_hwaddr(sip: uint32_t) -> *mut libc::c_uchar;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
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
pub unsafe extern "C" fn dst_neigh_output(mut skb: *mut sk_buff) -> libc::c_int {
    let mut iphdr: *mut iphdr = ip_hdr(skb);
    let mut netdev: *mut netdev = (*skb).dev;
    let mut rt: *mut rtentry = (*skb).rt;
    let mut daddr: uint32_t = ntohl((*iphdr).daddr);
    let mut saddr: uint32_t = ntohl((*iphdr).saddr);
    let mut dmac: *mut uint8_t = 0 as *mut uint8_t;
    if (*rt).flags as libc::c_int & 0x2 as libc::c_int != 0 {
        daddr = (*rt).gateway;
    }
    dmac = arp_get_hwaddr(daddr);
    if !dmac.is_null() {
        return netdev_transmit(skb, dmac, 0x800 as libc::c_int as uint16_t)
    } else {
        arp_request(saddr, daddr, netdev);
        return -(1 as libc::c_int);
    };
}
