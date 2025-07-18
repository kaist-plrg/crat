use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn perror(__s: *const libc::c_char);
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
    static mut netdev: *mut netdev;
    #[link_name = "loop"]
    static mut loop_0: *mut netdev;
    static mut tapaddr: *mut libc::c_char;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
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
pub struct netdev {
    pub addr: uint32_t,
    pub addr_len: uint8_t,
    pub hwaddr: [uint8_t; 6],
    pub mtu: uint32_t,
}
#[inline]
unsafe extern "C" fn list_init(mut head: *mut list_head) {
    (*head).next = head;
    (*head).prev = (*head).next;
}
#[inline]
unsafe extern "C" fn list_add_tail(mut new: *mut list_head, mut head: *mut list_head) {
    (*(*head).prev).next = new;
    (*new).prev = (*head).prev;
    (*new).next = head;
    (*head).prev = new;
}
#[inline]
unsafe extern "C" fn list_del(mut elem: *mut list_head) {
    let mut prev: *mut list_head = (*elem).prev;
    let mut next: *mut list_head = (*elem).next;
    (*prev).next = next;
    (*next).prev = prev;
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
static mut routes: list_head = unsafe {
    {
        let mut init = list_head {
            next: &routes as *const list_head as *mut list_head,
            prev: &routes as *const list_head as *mut list_head,
        };
        init
    }
};
unsafe extern "C" fn route_alloc(
    mut dst: uint32_t,
    mut gateway: uint32_t,
    mut netmask: uint32_t,
    mut flags: uint8_t,
    mut metric: uint32_t,
    mut dev: *mut netdev,
) -> *mut rtentry {
    let mut rt: *mut rtentry = malloc(::std::mem::size_of::<rtentry>() as libc::c_ulong)
        as *mut rtentry;
    list_init(&mut (*rt).list);
    (*rt).dst = dst;
    (*rt).gateway = gateway;
    (*rt).netmask = netmask;
    (*rt).flags = flags;
    (*rt).metric = metric;
    (*rt).dev = dev;
    return rt;
}
pub unsafe extern "C" fn route_add(
    mut dst: uint32_t,
    mut gateway: uint32_t,
    mut netmask: uint32_t,
    mut flags: uint8_t,
    mut metric: uint32_t,
    mut dev: *mut netdev,
) {
    let mut rt: *mut rtentry = route_alloc(dst, gateway, netmask, flags, metric, dev);
    list_add_tail(&mut (*rt).list, &mut routes);
}
pub unsafe extern "C" fn route_init() {
    route_add(
        (*loop_0).addr,
        0 as libc::c_int as uint32_t,
        0xff000000 as libc::c_uint,
        0x1 as libc::c_int as uint8_t,
        0 as libc::c_int as uint32_t,
        loop_0,
    );
    route_add(
        (*netdev).addr,
        0 as libc::c_int as uint32_t,
        0xffffff00 as libc::c_uint,
        0x4 as libc::c_int as uint8_t,
        0 as libc::c_int as uint32_t,
        netdev,
    );
    route_add(
        0 as libc::c_int as uint32_t,
        ip_parse(tapaddr),
        0 as libc::c_int as uint32_t,
        0x2 as libc::c_int as uint8_t,
        0 as libc::c_int as uint32_t,
        netdev,
    );
}
pub unsafe extern "C" fn route_lookup(mut daddr: uint32_t) -> *mut rtentry {
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut rt: *mut rtentry = 0 as *mut rtentry;
    item = routes.next;
    while item != &mut routes as *mut list_head {
        rt = (item as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut rtentry;
        if daddr & (*rt).netmask == (*rt).dst & (*rt).netmask {
            break;
        }
        item = (*item).next;
    }
    return rt;
}
pub unsafe extern "C" fn free_routes() {
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut tmp: *mut list_head = 0 as *mut list_head;
    let mut rt: *mut rtentry = 0 as *mut rtentry;
    item = routes.next;
    tmp = (*item).next;
    while item != &mut routes as *mut list_head {
        rt = (item as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut rtentry;
        list_del(item);
        free(rt as *mut libc::c_void);
        item = tmp;
        tmp = (*item).next;
    }
}
