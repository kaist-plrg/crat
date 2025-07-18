use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
#[inline]
unsafe extern "C" fn list_init(mut head: *mut list_head) {
    (*head).next = head;
    (*head).prev = (*head).next;
}
pub unsafe extern "C" fn alloc_skb(mut size: libc::c_uint) -> *mut sk_buff {
    let mut skb: *mut sk_buff = malloc(::std::mem::size_of::<sk_buff>() as libc::c_ulong)
        as *mut sk_buff;
    memset(
        skb as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sk_buff>() as libc::c_ulong,
    );
    (*skb).data = malloc(size as libc::c_ulong) as *mut uint8_t;
    memset((*skb).data as *mut libc::c_void, 0 as libc::c_int, size as libc::c_ulong);
    (*skb).refcnt = 0 as libc::c_int;
    (*skb).head = (*skb).data;
    (*skb).end = ((*skb).data).offset(size as isize);
    list_init(&mut (*skb).list);
    return skb;
}
pub unsafe extern "C" fn free_skb(mut skb: *mut sk_buff) {
    if (*skb).refcnt < 1 as libc::c_int {
        free((*skb).head as *mut libc::c_void);
        free(skb as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn skb_reserve(
    mut skb: *mut sk_buff,
    mut len: libc::c_uint,
) -> *mut libc::c_void {
    (*skb).data = ((*skb).data).offset(len as isize);
    return (*skb).data as *mut libc::c_void;
}
pub unsafe extern "C" fn skb_push(
    mut skb: *mut sk_buff,
    mut len: libc::c_uint,
) -> *mut uint8_t {
    (*skb).data = ((*skb).data).offset(-(len as isize));
    (*skb).len = ((*skb).len as libc::c_uint).wrapping_add(len) as uint32_t as uint32_t;
    return (*skb).data;
}
pub unsafe extern "C" fn skb_head(mut skb: *mut sk_buff) -> *mut uint8_t {
    return (*skb).head;
}
pub unsafe extern "C" fn skb_reset_header(mut skb: *mut sk_buff) {
    (*skb).data = ((*skb).end).offset(-((*skb).dlen as isize));
    (*skb).len = (*skb).dlen;
}
