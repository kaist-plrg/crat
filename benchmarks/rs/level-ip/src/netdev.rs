use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
    fn skb_push(skb: *mut sk_buff, len: libc::c_uint) -> *mut uint8_t;
    fn alloc_skb(size: libc::c_uint) -> *mut sk_buff;
    fn free_skb(skb: *mut sk_buff);
    fn skb_head(skb: *mut sk_buff) -> *mut uint8_t;
    fn arp_rcv(skb: *mut sk_buff);
    fn ip_rcv(skb: *mut sk_buff) -> libc::c_int;
    fn tun_read(buf: *mut libc::c_char, len: libc::c_int) -> libc::c_int;
    fn tun_write(buf: *mut libc::c_char, len: libc::c_int) -> libc::c_int;
    static mut running: libc::c_int;
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
#[inline]
unsafe extern "C" fn eth_hdr(mut skb: *mut sk_buff) -> *mut eth_hdr {
    let mut hdr: *mut eth_hdr = skb_head(skb) as *mut eth_hdr;
    (*hdr).ethertype = ntohs((*hdr).ethertype);
    return hdr;
}
#[inline]
unsafe extern "C" fn ip_parse(mut addr: *mut libc::c_char) -> uint32_t {
    let mut dst: uint32_t = 0 as libc::c_int as uint32_t;
    if inet_pton(2 as libc::c_int, addr, &mut dst as *mut uint32_t as *mut libc::c_void)
        != 1 as libc::c_int
    {
        perror(
            b"ERR: Parsing inet address failed\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    return ntohl(dst);
}
#[export_name = "loop"]
pub static mut loop_0: *mut netdev = 0 as *const netdev as *mut netdev;
pub static mut netdev: *mut netdev = 0 as *const netdev as *mut netdev;
unsafe extern "C" fn netdev_alloc(
    mut addr: *mut libc::c_char,
    mut hwaddr: *mut libc::c_char,
    mut mtu: uint32_t,
) -> *mut netdev {
    let mut dev: *mut netdev = malloc(::std::mem::size_of::<netdev>() as libc::c_ulong)
        as *mut netdev;
    (*dev).addr = ip_parse(addr);
    sscanf(
        hwaddr,
        b"%hhx:%hhx:%hhx:%hhx:%hhx:%hhx\0" as *const u8 as *const libc::c_char,
        &mut *((*dev).hwaddr).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut uint8_t,
        &mut *((*dev).hwaddr).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut uint8_t,
        &mut *((*dev).hwaddr).as_mut_ptr().offset(2 as libc::c_int as isize)
            as *mut uint8_t,
        &mut *((*dev).hwaddr).as_mut_ptr().offset(3 as libc::c_int as isize)
            as *mut uint8_t,
        &mut *((*dev).hwaddr).as_mut_ptr().offset(4 as libc::c_int as isize)
            as *mut uint8_t,
        &mut *((*dev).hwaddr).as_mut_ptr().offset(5 as libc::c_int as isize)
            as *mut uint8_t,
    );
    (*dev).addr_len = 6 as libc::c_int as uint8_t;
    (*dev).mtu = mtu;
    return dev;
}
pub unsafe extern "C" fn netdev_init(
    mut addr: *mut libc::c_char,
    mut hwaddr: *mut libc::c_char,
) {
    loop_0 = netdev_alloc(
        b"127.0.0.1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"00:00:00:00:00:00\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1500 as libc::c_int as uint32_t,
    );
    netdev = netdev_alloc(
        b"10.0.0.4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"00:0c:29:6d:50:25\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1500 as libc::c_int as uint32_t,
    );
}
pub unsafe extern "C" fn netdev_transmit(
    mut skb: *mut sk_buff,
    mut dst_hw: *mut uint8_t,
    mut ethertype: uint16_t,
) -> libc::c_int {
    let mut dev: *mut netdev = 0 as *mut netdev;
    let mut hdr: *mut eth_hdr = 0 as *mut eth_hdr;
    let mut ret: libc::c_int = 0 as libc::c_int;
    dev = (*skb).dev;
    skb_push(skb, ::std::mem::size_of::<eth_hdr>() as libc::c_ulong as libc::c_uint);
    hdr = (*skb).data as *mut eth_hdr;
    memcpy(
        ((*hdr).dmac).as_mut_ptr() as *mut libc::c_void,
        dst_hw as *const libc::c_void,
        (*dev).addr_len as libc::c_ulong,
    );
    memcpy(
        ((*hdr).smac).as_mut_ptr() as *mut libc::c_void,
        ((*dev).hwaddr).as_mut_ptr() as *const libc::c_void,
        (*dev).addr_len as libc::c_ulong,
    );
    (*hdr).ethertype = htons(ethertype);
    ret = tun_write((*skb).data as *mut libc::c_char, (*skb).len as libc::c_int);
    return ret;
}
unsafe extern "C" fn netdev_receive(mut skb: *mut sk_buff) -> libc::c_int {
    let mut hdr: *mut eth_hdr = eth_hdr(skb);
    match (*hdr).ethertype as libc::c_int {
        2054 => {
            arp_rcv(skb);
        }
        2048 => {
            ip_rcv(skb);
        }
        34525 | _ => {
            printf(
                b"Unsupported ethertype %x\n\0" as *const u8 as *const libc::c_char,
                (*hdr).ethertype as libc::c_int,
            );
            free_skb(skb);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn netdev_rx_loop() -> *mut libc::c_void {
    while running != 0 {
        let mut skb: *mut sk_buff = alloc_skb(1600 as libc::c_int as libc::c_uint);
        if tun_read((*skb).data as *mut libc::c_char, 1600 as libc::c_int)
            < 0 as libc::c_int
        {
            perror(b"ERR: Read from tun_fd\0" as *const u8 as *const libc::c_char);
            free_skb(skb);
            return 0 as *mut libc::c_void;
        }
        netdev_receive(skb);
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn netdev_get(mut sip: uint32_t) -> *mut netdev {
    if (*netdev).addr == sip { return netdev } else { return 0 as *mut netdev };
}
pub unsafe extern "C" fn free_netdev() {
    free(loop_0 as *mut libc::c_void);
    free(netdev as *mut libc::c_void);
}
