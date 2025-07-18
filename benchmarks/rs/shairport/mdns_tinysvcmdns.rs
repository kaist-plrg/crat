use ::libc;
extern "C" {
    pub type mdnsd;
    pub type mdns_service;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn getifaddrs(__ifap: *mut *mut ifaddrs) -> libc::c_int;
    fn freeifaddrs(__ifa: *mut ifaddrs);
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    static mut config: shairport_cfg;
    fn warn(format: *mut libc::c_char, _: ...);
    fn rr_create_aaaa(name: *mut uint8_t, addr: *mut in6_addr) -> *mut rr_entry;
    fn rr_create_a(name: *mut uint8_t, addr: uint32_t) -> *mut rr_entry;
    fn create_nlabel(name: *const libc::c_char) -> *mut uint8_t;
    fn mdnsd_start() -> *mut mdnsd;
    fn mdnsd_stop(s: *mut mdnsd);
    fn mdnsd_set_hostname(
        svr_0: *mut mdnsd,
        hostname: *const libc::c_char,
        ip: uint32_t,
    );
    fn mdnsd_set_hostname_v6(
        svr_0: *mut mdnsd,
        hostname: *const libc::c_char,
        addr: *mut in6_addr,
    );
    fn mdnsd_add_rr(svr_0: *mut mdnsd, rr: *mut rr_entry);
    fn mdnsd_register_svc(
        svr_0: *mut mdnsd,
        instance_name: *const libc::c_char,
        type_0: *const libc::c_char,
        port: uint16_t,
        hostname: *const libc::c_char,
        txt: *mut *const libc::c_char,
    ) -> *mut mdns_service;
    fn mdns_service_destroy(srv: *mut mdns_service);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type C2RustUnnamed = libc::c_uint;
pub const IFF_DYNAMIC: C2RustUnnamed = 32768;
pub const IFF_AUTOMEDIA: C2RustUnnamed = 16384;
pub const IFF_PORTSEL: C2RustUnnamed = 8192;
pub const IFF_MULTICAST: C2RustUnnamed = 4096;
pub const IFF_SLAVE: C2RustUnnamed = 2048;
pub const IFF_MASTER: C2RustUnnamed = 1024;
pub const IFF_ALLMULTI: C2RustUnnamed = 512;
pub const IFF_PROMISC: C2RustUnnamed = 256;
pub const IFF_NOARP: C2RustUnnamed = 128;
pub const IFF_RUNNING: C2RustUnnamed = 64;
pub const IFF_NOTRAILERS: C2RustUnnamed = 32;
pub const IFF_POINTOPOINT: C2RustUnnamed = 16;
pub const IFF_LOOPBACK: C2RustUnnamed = 8;
pub const IFF_DEBUG: C2RustUnnamed = 4;
pub const IFF_BROADCAST: C2RustUnnamed = 2;
pub const IFF_UP: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifaddrs {
    pub ifa_next: *mut ifaddrs,
    pub ifa_name: *mut libc::c_char,
    pub ifa_flags: libc::c_uint,
    pub ifa_addr: *mut sockaddr,
    pub ifa_netmask: *mut sockaddr,
    pub ifa_ifu: C2RustUnnamed_0,
    pub ifa_data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ifu_broadaddr: *mut sockaddr,
    pub ifu_dstaddr: *mut sockaddr,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
    pub __in6_u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
pub struct audio_output {
    pub help: Option::<unsafe extern "C" fn() -> ()>,
    pub name: *mut libc::c_char,
    pub init: Option::<
        unsafe extern "C" fn(libc::c_int, *mut *mut libc::c_char) -> libc::c_int,
    >,
    pub deinit: Option::<unsafe extern "C" fn() -> ()>,
    pub start: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub play: Option::<unsafe extern "C" fn(*mut libc::c_short, libc::c_int) -> ()>,
    pub stop: Option::<unsafe extern "C" fn() -> ()>,
    pub volume: Option::<unsafe extern "C" fn(libc::c_double) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdns_backend {
    pub name: *mut libc::c_char,
    pub mdns_register: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub mdns_unregister: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shairport_cfg {
    pub password: *mut libc::c_char,
    pub apname: *mut libc::c_char,
    pub hw_addr: [uint8_t; 6],
    pub port: libc::c_int,
    pub output_name: *mut libc::c_char,
    pub output: *mut audio_output,
    pub mdns_name: *mut libc::c_char,
    pub mdns: *mut mdns_backend,
    pub buffer_start_fill: libc::c_int,
    pub daemonise: libc::c_int,
    pub cmd_start: *mut libc::c_char,
    pub cmd_stop: *mut libc::c_char,
    pub cmd_blocking: libc::c_int,
    pub meta_dir: *mut libc::c_char,
    pub pidfile: *mut libc::c_char,
    pub logfile: *mut libc::c_char,
    pub errfile: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_data_srv {
    pub priority: uint16_t,
    pub weight: uint16_t,
    pub port: uint16_t,
    pub target: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_data_txt {
    pub next: *mut rr_data_txt,
    pub txt: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_data_nsec {
    pub bitmap: [uint8_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_data_ptr {
    pub name: *mut uint8_t,
    pub entry: *mut rr_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_entry {
    pub name: *mut uint8_t,
    pub type_0: rr_type,
    pub ttl: uint32_t,
    pub unicast_query: libc::c_char,
    pub cache_flush: libc::c_char,
    pub rr_class: uint16_t,
    pub data: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub NSEC: rr_data_nsec,
    pub SRV: rr_data_srv,
    pub TXT: rr_data_txt,
    pub PTR: rr_data_ptr,
    pub A: rr_data_a,
    pub AAAA: rr_data_aaaa,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_data_aaaa {
    pub addr: *mut in6_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_data_a {
    pub addr: uint32_t,
}
pub type rr_type = libc::c_uint;
pub const RR_ANY: rr_type = 255;
pub const RR_NSEC: rr_type = 47;
pub const RR_SRV: rr_type = 33;
pub const RR_AAAA: rr_type = 28;
pub const RR_TXT: rr_type = 16;
pub const RR_PTR: rr_type = 12;
pub const RR_A: rr_type = 1;
static mut svr: *mut mdnsd = 0 as *const mdnsd as *mut mdnsd;
unsafe extern "C" fn mdns_tinysvcmdns_register(
    mut apname: *mut libc::c_char,
    mut port: libc::c_int,
) -> libc::c_int {
    let mut ifalist: *mut ifaddrs = 0 as *mut ifaddrs;
    let mut ifa: *mut ifaddrs = 0 as *mut ifaddrs;
    svr = mdnsd_start();
    if svr.is_null() {
        warn(
            b"tinysvcmdns: mdnsd_start() failed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut hostname: [libc::c_char; 106] = [0; 106];
    gethostname(hostname.as_mut_ptr(), 99 as libc::c_int as size_t);
    hostname[99 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    let mut hostend: *mut libc::c_char = hostname
        .as_mut_ptr()
        .offset(strlen(hostname.as_mut_ptr()) as isize);
    if strlen(hostname.as_mut_ptr()) > 6 as libc::c_int as libc::c_ulong
        && strcmp(
            hostend.offset(-(6 as libc::c_int as isize)),
            b".local\0" as *const u8 as *const libc::c_char,
        ) != 0
    {
        strcat(hostname.as_mut_ptr(), b".local\0" as *const u8 as *const libc::c_char);
    }
    if getifaddrs(&mut ifalist) < 0 as libc::c_int {
        warn(
            b"tinysvcmdns: getifaddrs() failed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    ifa = ifalist;
    ifa = ifalist;
    while !ifa.is_null() {
        if (*ifa).ifa_flags & IFF_LOOPBACK as libc::c_int as libc::c_uint == 0
            && !((*ifa).ifa_addr).is_null()
            && (*(*ifa).ifa_addr).sa_family as libc::c_int == 2 as libc::c_int
        {
            let mut main_ip: uint32_t = (*((*ifa).ifa_addr as *mut sockaddr_in))
                .sin_addr
                .s_addr;
            mdnsd_set_hostname(svr, hostname.as_mut_ptr(), main_ip);
            break;
        } else if (*ifa).ifa_flags & IFF_LOOPBACK as libc::c_int as libc::c_uint == 0
            && !((*ifa).ifa_addr).is_null()
            && (*(*ifa).ifa_addr).sa_family as libc::c_int == 10 as libc::c_int
        {
            let mut addr: *mut in6_addr = &mut (*((*ifa).ifa_addr as *mut sockaddr_in6))
                .sin6_addr;
            mdnsd_set_hostname_v6(svr, hostname.as_mut_ptr(), addr);
            break;
        } else {
            ifa = (*ifa).ifa_next;
        }
    }
    if ifa.is_null() {
        warn(
            b"tinysvcmdns: no non-loopback ipv4 or ipv6 interface found\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    ifa = (*ifa).ifa_next;
    while !ifa.is_null() {
        if !((*ifa).ifa_flags & IFF_LOOPBACK as libc::c_int as libc::c_uint != 0) {
            match (*(*ifa).ifa_addr).sa_family as libc::c_int {
                2 => {
                    let mut ip: uint32_t = (*((*ifa).ifa_addr as *mut sockaddr_in))
                        .sin_addr
                        .s_addr;
                    let mut a_e: *mut rr_entry = rr_create_a(
                        create_nlabel(hostname.as_mut_ptr()),
                        ip,
                    );
                    mdnsd_add_rr(svr, a_e);
                }
                10 => {
                    let mut addr_0: *mut in6_addr = &mut (*((*ifa).ifa_addr
                        as *mut sockaddr_in6))
                        .sin6_addr;
                    let mut aaaa_e: *mut rr_entry = rr_create_aaaa(
                        create_nlabel(hostname.as_mut_ptr()),
                        addr_0,
                    );
                    mdnsd_add_rr(svr, aaaa_e);
                }
                _ => {}
            }
        }
        ifa = (*ifa).ifa_next;
    }
    freeifaddrs(ifa);
    let mut txt: [*const libc::c_char; 14] = [
        b"tp=UDP\0" as *const u8 as *const libc::c_char,
        b"sm=false\0" as *const u8 as *const libc::c_char,
        b"ek=1\0" as *const u8 as *const libc::c_char,
        b"et=0,1\0" as *const u8 as *const libc::c_char,
        b"cn=0,1\0" as *const u8 as *const libc::c_char,
        b"ch=2\0" as *const u8 as *const libc::c_char,
        b"ss=16\0" as *const u8 as *const libc::c_char,
        b"sr=44100\0" as *const u8 as *const libc::c_char,
        b"vn=3\0" as *const u8 as *const libc::c_char,
        b"txtvers=1\0" as *const u8 as *const libc::c_char,
        b"da=true\0" as *const u8 as *const libc::c_char,
        b"md=0,1,2\0" as *const u8 as *const libc::c_char,
        if !(config.password).is_null() {
            b"pw=true\0" as *const u8 as *const libc::c_char
        } else {
            b"pw=false\0" as *const u8 as *const libc::c_char
        },
        0 as *const libc::c_char,
    ];
    let mut svc: *mut mdns_service = mdnsd_register_svc(
        svr,
        apname,
        b"_raop._tcp.local\0" as *const u8 as *const libc::c_char,
        port as uint16_t,
        0 as *const libc::c_char,
        txt.as_mut_ptr(),
    );
    mdns_service_destroy(svc);
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdns_tinysvcmdns_unregister() {
    if !svr.is_null() {
        mdnsd_stop(svr);
        svr = 0 as *mut mdnsd;
    }
}
pub static mut mdns_tinysvcmdns: mdns_backend = unsafe {
    {
        let mut init = mdns_backend {
            name: b"tinysvcmdns\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            mdns_register: Some(
                mdns_tinysvcmdns_register
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            mdns_unregister: Some(
                mdns_tinysvcmdns_unregister as unsafe extern "C" fn() -> (),
            ),
        };
        init
    }
};
