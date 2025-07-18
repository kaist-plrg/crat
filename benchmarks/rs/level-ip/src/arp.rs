use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn alloc_skb(size: libc::c_uint) -> *mut sk_buff;
    fn free_skb(skb: *mut sk_buff);
    fn skb_push(skb: *mut sk_buff, len: libc::c_uint) -> *mut uint8_t;
    fn skb_reserve(skb: *mut sk_buff, len: libc::c_uint) -> *mut libc::c_void;
    fn netdev_transmit(
        skb: *mut sk_buff,
        dst: *mut uint8_t,
        ethertype: uint16_t,
    ) -> libc::c_int;
    fn netdev_get(sip: uint32_t) -> *mut netdev;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
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
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
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
pub struct eth_hdr {
    pub dmac: [uint8_t; 6],
    pub smac: [uint8_t; 6],
    pub ethertype: uint16_t,
    pub payload: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arp_hdr {
    pub hwtype: uint16_t,
    pub protype: uint16_t,
    pub hwsize: uint8_t,
    pub prosize: uint8_t,
    pub opcode: uint16_t,
    pub data: [libc::c_uchar; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arp_ipv4 {
    pub smac: [libc::c_uchar; 6],
    pub sip: uint32_t,
    pub dmac: [libc::c_uchar; 6],
    pub dip: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arp_cache_entry {
    pub list: list_head,
    pub hwtype: uint16_t,
    pub sip: uint32_t,
    pub smac: [libc::c_uchar; 6],
    pub state: libc::c_uint,
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
unsafe extern "C" fn arp_hdr(mut skb: *mut sk_buff) -> *mut arp_hdr {
    return ((*skb).head)
        .offset(::std::mem::size_of::<eth_hdr>() as libc::c_ulong as isize)
        as *mut arp_hdr;
}
static mut broadcast_hw: [uint8_t; 6] = [
    0xff as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
];
static mut arp_cache: list_head = unsafe {
    {
        let mut init = list_head {
            next: &arp_cache as *const list_head as *mut list_head,
            prev: &arp_cache as *const list_head as *mut list_head,
        };
        init
    }
};
static mut lock: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
unsafe extern "C" fn arp_alloc_skb() -> *mut sk_buff {
    let mut skb: *mut sk_buff = alloc_skb(
        (::std::mem::size_of::<eth_hdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<arp_hdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<arp_ipv4>() as libc::c_ulong)
            as libc::c_uint,
    );
    skb_reserve(
        skb,
        (::std::mem::size_of::<eth_hdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<arp_hdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<arp_ipv4>() as libc::c_ulong)
            as libc::c_uint,
    );
    (*skb).protocol = htons(0x806 as libc::c_int as uint16_t);
    return skb;
}
unsafe extern "C" fn arp_entry_alloc(
    mut hdr: *mut arp_hdr,
    mut data: *mut arp_ipv4,
) -> *mut arp_cache_entry {
    let mut entry: *mut arp_cache_entry = malloc(
        ::std::mem::size_of::<arp_cache_entry>() as libc::c_ulong,
    ) as *mut arp_cache_entry;
    list_init(&mut (*entry).list);
    (*entry).state = 2 as libc::c_int as libc::c_uint;
    (*entry).hwtype = (*hdr).hwtype;
    (*entry).sip = (*data).sip;
    memcpy(
        ((*entry).smac).as_mut_ptr() as *mut libc::c_void,
        ((*data).smac).as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 6]>() as libc::c_ulong,
    );
    return entry;
}
unsafe extern "C" fn insert_arp_translation_table(
    mut hdr: *mut arp_hdr,
    mut data: *mut arp_ipv4,
) -> libc::c_int {
    let mut entry: *mut arp_cache_entry = arp_entry_alloc(hdr, data);
    pthread_mutex_lock(&mut lock);
    list_add_tail(&mut (*entry).list, &mut arp_cache);
    pthread_mutex_unlock(&mut lock);
    return 0 as libc::c_int;
}
unsafe extern "C" fn update_arp_translation_table(
    mut hdr: *mut arp_hdr,
    mut data: *mut arp_ipv4,
) -> libc::c_int {
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut entry: *mut arp_cache_entry = 0 as *mut arp_cache_entry;
    pthread_mutex_lock(&mut lock);
    item = arp_cache.next;
    while item != &mut arp_cache as *mut list_head {
        entry = (item as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut arp_cache_entry;
        if (*entry).hwtype as libc::c_int == (*hdr).hwtype as libc::c_int
            && (*entry).sip == (*data).sip
        {
            memcpy(
                ((*entry).smac).as_mut_ptr() as *mut libc::c_void,
                ((*data).smac).as_mut_ptr() as *const libc::c_void,
                6 as libc::c_int as libc::c_ulong,
            );
            pthread_mutex_unlock(&mut lock);
            return 1 as libc::c_int;
        }
        item = (*item).next;
    }
    pthread_mutex_unlock(&mut lock);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn arp_init() {}
pub unsafe extern "C" fn arp_rcv(mut skb: *mut sk_buff) {
    let mut arphdr: *mut arp_hdr = 0 as *mut arp_hdr;
    let mut arpdata: *mut arp_ipv4 = 0 as *mut arp_ipv4;
    let mut netdev: *mut netdev = 0 as *mut netdev;
    let mut merge: libc::c_int = 0 as libc::c_int;
    arphdr = arp_hdr(skb);
    (*arphdr).hwtype = ntohs((*arphdr).hwtype);
    (*arphdr).protype = ntohs((*arphdr).protype);
    (*arphdr).opcode = ntohs((*arphdr).opcode);
    if (*arphdr).hwtype as libc::c_int != 0x1 as libc::c_int {
        printf(b"ARP: Unsupported HW type\n\0" as *const u8 as *const libc::c_char);
    } else if (*arphdr).protype as libc::c_int != 0x800 as libc::c_int {
        printf(b"ARP: Unsupported protocol\n\0" as *const u8 as *const libc::c_char);
    } else {
        arpdata = ((*arphdr).data).as_mut_ptr() as *mut arp_ipv4;
        (*arpdata).sip = ntohl((*arpdata).sip);
        (*arpdata).dip = ntohl((*arpdata).dip);
        merge = update_arp_translation_table(arphdr, arpdata);
        netdev = netdev_get((*arpdata).dip);
        if netdev.is_null() {
            printf(b"ARP was not for us\n\0" as *const u8 as *const libc::c_char);
        } else if merge == 0
            && insert_arp_translation_table(arphdr, arpdata) != 0 as libc::c_int
        {
            fprintf(
                stderr,
                b"ERR: No free space in ARP translation table\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            match (*arphdr).opcode as libc::c_int {
                1 => {
                    arp_reply(skb, netdev);
                    return;
                }
                _ => {
                    printf(
                        b"ARP: Opcode not supported\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
        }
    }
    free_skb(skb);
}
pub unsafe extern "C" fn arp_request(
    mut sip: uint32_t,
    mut dip: uint32_t,
    mut netdev: *mut netdev,
) -> libc::c_int {
    let mut skb: *mut sk_buff = 0 as *mut sk_buff;
    let mut arp: *mut arp_hdr = 0 as *mut arp_hdr;
    let mut payload: *mut arp_ipv4 = 0 as *mut arp_ipv4;
    let mut rc: libc::c_int = 0 as libc::c_int;
    skb = arp_alloc_skb();
    if skb.is_null() {
        return -(1 as libc::c_int);
    }
    (*skb).dev = netdev;
    payload = skb_push(
        skb,
        ::std::mem::size_of::<arp_ipv4>() as libc::c_ulong as libc::c_uint,
    ) as *mut arp_ipv4;
    memcpy(
        ((*payload).smac).as_mut_ptr() as *mut libc::c_void,
        ((*netdev).hwaddr).as_mut_ptr() as *const libc::c_void,
        (*netdev).addr_len as libc::c_ulong,
    );
    (*payload).sip = sip;
    memcpy(
        ((*payload).dmac).as_mut_ptr() as *mut libc::c_void,
        broadcast_hw.as_mut_ptr() as *const libc::c_void,
        (*netdev).addr_len as libc::c_ulong,
    );
    (*payload).dip = dip;
    arp = skb_push(
        skb,
        ::std::mem::size_of::<arp_hdr>() as libc::c_ulong as libc::c_uint,
    ) as *mut arp_hdr;
    (*arp).opcode = htons(0x1 as libc::c_int as uint16_t);
    (*arp).hwtype = htons(0x1 as libc::c_int as uint16_t);
    (*arp).protype = htons(0x800 as libc::c_int as uint16_t);
    (*arp).hwsize = (*netdev).addr_len;
    (*arp).prosize = 4 as libc::c_int as uint8_t;
    (*payload).sip = htonl((*payload).sip);
    (*payload).dip = htonl((*payload).dip);
    rc = netdev_transmit(
        skb,
        broadcast_hw.as_mut_ptr(),
        0x806 as libc::c_int as uint16_t,
    );
    free_skb(skb);
    return rc;
}
pub unsafe extern "C" fn arp_reply(mut skb: *mut sk_buff, mut netdev: *mut netdev) {
    let mut arphdr: *mut arp_hdr = 0 as *mut arp_hdr;
    let mut arpdata: *mut arp_ipv4 = 0 as *mut arp_ipv4;
    arphdr = arp_hdr(skb);
    skb_reserve(
        skb,
        (::std::mem::size_of::<eth_hdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<arp_hdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<arp_ipv4>() as libc::c_ulong)
            as libc::c_uint,
    );
    skb_push(
        skb,
        (::std::mem::size_of::<arp_hdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<arp_ipv4>() as libc::c_ulong)
            as libc::c_uint,
    );
    arpdata = ((*arphdr).data).as_mut_ptr() as *mut arp_ipv4;
    memcpy(
        ((*arpdata).dmac).as_mut_ptr() as *mut libc::c_void,
        ((*arpdata).smac).as_mut_ptr() as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    );
    (*arpdata).dip = (*arpdata).sip;
    memcpy(
        ((*arpdata).smac).as_mut_ptr() as *mut libc::c_void,
        ((*netdev).hwaddr).as_mut_ptr() as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    );
    (*arpdata).sip = (*netdev).addr;
    (*arphdr).opcode = 0x2 as libc::c_int as uint16_t;
    (*arphdr).opcode = htons((*arphdr).opcode);
    (*arphdr).hwtype = htons((*arphdr).hwtype);
    (*arphdr).protype = htons((*arphdr).protype);
    (*arpdata).sip = htonl((*arpdata).sip);
    (*arpdata).dip = htonl((*arpdata).dip);
    (*skb).dev = netdev;
    netdev_transmit(
        skb,
        ((*arpdata).dmac).as_mut_ptr(),
        0x806 as libc::c_int as uint16_t,
    );
    free_skb(skb);
}
pub unsafe extern "C" fn arp_get_hwaddr(mut sip: uint32_t) -> *mut libc::c_uchar {
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut entry: *mut arp_cache_entry = 0 as *mut arp_cache_entry;
    pthread_mutex_lock(&mut lock);
    item = arp_cache.next;
    while item != &mut arp_cache as *mut list_head {
        entry = (item as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut arp_cache_entry;
        if (*entry).state == 2 as libc::c_int as libc::c_uint && (*entry).sip == sip {
            let mut copy: *mut uint8_t = ((*entry).smac).as_mut_ptr();
            pthread_mutex_unlock(&mut lock);
            return copy;
        }
        item = (*item).next;
    }
    pthread_mutex_unlock(&mut lock);
    return 0 as *mut libc::c_uchar;
}
pub unsafe extern "C" fn free_arp() {
    let mut item: *mut list_head = 0 as *mut list_head;
    let mut tmp: *mut list_head = 0 as *mut list_head;
    let mut entry: *mut arp_cache_entry = 0 as *mut arp_cache_entry;
    item = arp_cache.next;
    tmp = (*item).next;
    while item != &mut arp_cache as *mut list_head {
        entry = (item as *mut libc::c_char).offset(-(0 as libc::c_ulong as isize))
            as *mut arp_cache_entry;
        list_del(item);
        free(entry as *mut libc::c_void);
        item = tmp;
        tmp = (*item).next;
    }
}
