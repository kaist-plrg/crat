use ::libc;
extern "C" {
    fn time(__timer: *mut time_t) -> time_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn sblist_new(itemsize: size_t, blockitems: size_t) -> *mut sblist;
    fn sblist_free(l: *mut sblist);
    fn sblist_get(l: *mut sblist, item: size_t) -> *mut libc::c_void;
    fn sblist_add(l: *mut sblist, item: *mut libc::c_void) -> libc::c_int;
    fn sblist_delete(l: *mut sblist, item: size_t);
    fn ntohs(__netshort: uint16_t) -> uint16_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
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
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
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
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sblist {
    pub itemsize: size_t,
    pub blockitems: size_t,
    pub count: size_t,
    pub capa: size_t,
    pub items: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sockaddr_union {
    pub v4: sockaddr_in,
    pub v6: sockaddr_in6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_record {
    pub addr: sockaddr_union,
    pub tstamp: time_t,
}
static mut loop_records: *mut sblist = 0 as *const sblist as *mut sblist;
static mut loop_records_lock: pthread_mutex_t = pthread_mutex_t {
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
pub unsafe extern "C" fn loop_records_init() {
    loop_records = sblist_new(
        ::std::mem::size_of::<loop_record>() as libc::c_ulong,
        32 as libc::c_int as size_t,
    );
}
pub unsafe extern "C" fn loop_records_destroy() {
    sblist_free(loop_records);
    loop_records = 0 as *mut sblist;
}
pub unsafe extern "C" fn loop_records_add(mut addr: *mut sockaddr_union) {
    let mut now: time_t = time(0 as *mut time_t);
    let mut rec: loop_record = loop_record {
        addr: sockaddr_union {
            v4: sockaddr_in {
                sin_family: 0,
                sin_port: 0,
                sin_addr: in_addr { s_addr: 0 },
                sin_zero: [0; 8],
            },
        },
        tstamp: 0,
    };
    pthread_mutex_lock(&mut loop_records_lock);
    rec.tstamp = now;
    rec.addr = *addr;
    sblist_add(loop_records, &mut rec as *mut loop_record as *mut libc::c_void);
    pthread_mutex_unlock(&mut loop_records_lock);
}
pub unsafe extern "C" fn connection_loops(mut addr: *mut sockaddr_union) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut af: libc::c_int = 0;
    let mut our_af: libc::c_int = (*addr).v4.sin_family as libc::c_int;
    let mut ipdata: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut our_ipdata: *mut libc::c_void = if our_af == 2 as libc::c_int {
        &mut (*addr).v4.sin_addr.s_addr as *mut in_addr_t as *mut libc::c_void
    } else {
        &mut (*addr).v6.sin6_addr.__in6_u.__u6_addr8 as *mut [uint8_t; 16]
            as *mut libc::c_void
    };
    let mut i: size_t = 0;
    let mut cmp_len: size_t = if our_af == 2 as libc::c_int {
        ::std::mem::size_of::<in_addr_t>() as libc::c_ulong
    } else {
        ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong
    };
    let mut port: libc::c_uint = 0;
    let mut our_port: libc::c_uint = ntohs(
        (if our_af == 2 as libc::c_int {
            (*addr).v4.sin_port as libc::c_int
        } else {
            (*addr).v6.sin6_port as libc::c_int
        }) as uint16_t,
    ) as libc::c_uint;
    let mut now: time_t = time(0 as *mut time_t);
    pthread_mutex_lock(&mut loop_records_lock);
    i = 0 as libc::c_int as size_t;
    while i < (*loop_records).count {
        let mut rec: *mut loop_record = sblist_get(loop_records, i) as *mut loop_record;
        if ((*rec).tstamp + 15 as libc::c_int as libc::c_long) < now {
            sblist_delete(loop_records, i);
        } else {
            if ret == 0 {
                af = (*rec).addr.v4.sin_family as libc::c_int;
                if !(af != our_af) {
                    port = ntohs(
                        (if af == 2 as libc::c_int {
                            (*rec).addr.v4.sin_port as libc::c_int
                        } else {
                            (*rec).addr.v6.sin6_port as libc::c_int
                        }) as uint16_t,
                    ) as libc::c_uint;
                    if !(port != our_port) {
                        ipdata = if af == 2 as libc::c_int {
                            &mut (*rec).addr.v4.sin_addr.s_addr as *mut in_addr_t
                                as *mut libc::c_void
                        } else {
                            &mut (*rec).addr.v6.sin6_addr.__in6_u.__u6_addr8
                                as *mut [uint8_t; 16] as *mut libc::c_void
                        };
                        if memcmp(ipdata, our_ipdata, cmp_len) == 0 {
                            ret = 1 as libc::c_int;
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    pthread_mutex_unlock(&mut loop_records_lock);
    return ret;
}
