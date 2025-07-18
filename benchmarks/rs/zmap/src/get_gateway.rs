use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn __errno_location() -> *mut libc::c_int;
    fn pcap_lookupdev(_: *mut libc::c_char) -> *mut libc::c_char;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn if_indextoname(
        __ifindex: libc::c_uint,
        __ifname: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn if_nametoindex(__ifname: *const libc::c_char) -> libc::c_uint;
    fn log_fatal(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> !;
    fn log_error(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn log_debug(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __caddr_t = *mut libc::c_char;
pub type ssize_t = __ssize_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
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
pub type __u8 = libc::c_uchar;
pub type __u16 = libc::c_ushort;
pub type __s32 = libc::c_int;
pub type __u32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifmap {
    pub mem_start: libc::c_ulong,
    pub mem_end: libc::c_ulong,
    pub base_addr: libc::c_ushort,
    pub irq: libc::c_uchar,
    pub dma: libc::c_uchar,
    pub port: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifreq {
    pub ifr_ifrn: C2RustUnnamed_0,
    pub ifr_ifru: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ifru_addr: sockaddr,
    pub ifru_dstaddr: sockaddr,
    pub ifru_broadaddr: sockaddr,
    pub ifru_netmask: sockaddr,
    pub ifru_hwaddr: sockaddr,
    pub ifru_flags: libc::c_short,
    pub ifru_ivalue: libc::c_int,
    pub ifru_mtu: libc::c_int,
    pub ifru_map: ifmap,
    pub ifru_slave: [libc::c_char; 16],
    pub ifru_newname: [libc::c_char; 16],
    pub ifru_data: __caddr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ifrn_name: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nlmsghdr {
    pub nlmsg_len: __u32,
    pub nlmsg_type: __u16,
    pub nlmsg_flags: __u16,
    pub nlmsg_seq: __u32,
    pub nlmsg_pid: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ndmsg {
    pub ndm_family: __u8,
    pub ndm_pad1: __u8,
    pub ndm_pad2: __u16,
    pub ndm_ifindex: __s32,
    pub ndm_state: __u16,
    pub ndm_flags: __u8,
    pub ndm_type: __u8,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const __NDA_MAX: C2RustUnnamed_1 = 13;
pub const NDA_PROTOCOL: C2RustUnnamed_1 = 12;
pub const NDA_SRC_VNI: C2RustUnnamed_1 = 11;
pub const NDA_LINK_NETNSID: C2RustUnnamed_1 = 10;
pub const NDA_MASTER: C2RustUnnamed_1 = 9;
pub const NDA_IFINDEX: C2RustUnnamed_1 = 8;
pub const NDA_VNI: C2RustUnnamed_1 = 7;
pub const NDA_PORT: C2RustUnnamed_1 = 6;
pub const NDA_VLAN: C2RustUnnamed_1 = 5;
pub const NDA_PROBES: C2RustUnnamed_1 = 4;
pub const NDA_CACHEINFO: C2RustUnnamed_1 = 3;
pub const NDA_LLADDR: C2RustUnnamed_1 = 2;
pub const NDA_DST: C2RustUnnamed_1 = 1;
pub const NDA_UNSPEC: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const __RTM_MAX: C2RustUnnamed_2 = 107;
pub const RTM_GETNEXTHOP: C2RustUnnamed_2 = 106;
pub const RTM_DELNEXTHOP: C2RustUnnamed_2 = 105;
pub const RTM_NEWNEXTHOP: C2RustUnnamed_2 = 104;
pub const RTM_GETCHAIN: C2RustUnnamed_2 = 102;
pub const RTM_DELCHAIN: C2RustUnnamed_2 = 101;
pub const RTM_NEWCHAIN: C2RustUnnamed_2 = 100;
pub const RTM_NEWCACHEREPORT: C2RustUnnamed_2 = 96;
pub const RTM_GETSTATS: C2RustUnnamed_2 = 94;
pub const RTM_NEWSTATS: C2RustUnnamed_2 = 92;
pub const RTM_GETNSID: C2RustUnnamed_2 = 90;
pub const RTM_DELNSID: C2RustUnnamed_2 = 89;
pub const RTM_NEWNSID: C2RustUnnamed_2 = 88;
pub const RTM_GETMDB: C2RustUnnamed_2 = 86;
pub const RTM_DELMDB: C2RustUnnamed_2 = 85;
pub const RTM_NEWMDB: C2RustUnnamed_2 = 84;
pub const RTM_GETNETCONF: C2RustUnnamed_2 = 82;
pub const RTM_DELNETCONF: C2RustUnnamed_2 = 81;
pub const RTM_NEWNETCONF: C2RustUnnamed_2 = 80;
pub const RTM_SETDCB: C2RustUnnamed_2 = 79;
pub const RTM_GETDCB: C2RustUnnamed_2 = 78;
pub const RTM_GETADDRLABEL: C2RustUnnamed_2 = 74;
pub const RTM_DELADDRLABEL: C2RustUnnamed_2 = 73;
pub const RTM_NEWADDRLABEL: C2RustUnnamed_2 = 72;
pub const RTM_NEWNDUSEROPT: C2RustUnnamed_2 = 68;
pub const RTM_SETNEIGHTBL: C2RustUnnamed_2 = 67;
pub const RTM_GETNEIGHTBL: C2RustUnnamed_2 = 66;
pub const RTM_NEWNEIGHTBL: C2RustUnnamed_2 = 64;
pub const RTM_GETANYCAST: C2RustUnnamed_2 = 62;
pub const RTM_GETMULTICAST: C2RustUnnamed_2 = 58;
pub const RTM_NEWPREFIX: C2RustUnnamed_2 = 52;
pub const RTM_GETACTION: C2RustUnnamed_2 = 50;
pub const RTM_DELACTION: C2RustUnnamed_2 = 49;
pub const RTM_NEWACTION: C2RustUnnamed_2 = 48;
pub const RTM_GETTFILTER: C2RustUnnamed_2 = 46;
pub const RTM_DELTFILTER: C2RustUnnamed_2 = 45;
pub const RTM_NEWTFILTER: C2RustUnnamed_2 = 44;
pub const RTM_GETTCLASS: C2RustUnnamed_2 = 42;
pub const RTM_DELTCLASS: C2RustUnnamed_2 = 41;
pub const RTM_NEWTCLASS: C2RustUnnamed_2 = 40;
pub const RTM_GETQDISC: C2RustUnnamed_2 = 38;
pub const RTM_DELQDISC: C2RustUnnamed_2 = 37;
pub const RTM_NEWQDISC: C2RustUnnamed_2 = 36;
pub const RTM_GETRULE: C2RustUnnamed_2 = 34;
pub const RTM_DELRULE: C2RustUnnamed_2 = 33;
pub const RTM_NEWRULE: C2RustUnnamed_2 = 32;
pub const RTM_GETNEIGH: C2RustUnnamed_2 = 30;
pub const RTM_DELNEIGH: C2RustUnnamed_2 = 29;
pub const RTM_NEWNEIGH: C2RustUnnamed_2 = 28;
pub const RTM_GETROUTE: C2RustUnnamed_2 = 26;
pub const RTM_DELROUTE: C2RustUnnamed_2 = 25;
pub const RTM_NEWROUTE: C2RustUnnamed_2 = 24;
pub const RTM_GETADDR: C2RustUnnamed_2 = 22;
pub const RTM_DELADDR: C2RustUnnamed_2 = 21;
pub const RTM_NEWADDR: C2RustUnnamed_2 = 20;
pub const RTM_SETLINK: C2RustUnnamed_2 = 19;
pub const RTM_GETLINK: C2RustUnnamed_2 = 18;
pub const RTM_DELLINK: C2RustUnnamed_2 = 17;
pub const RTM_NEWLINK: C2RustUnnamed_2 = 16;
pub const RTM_BASE: C2RustUnnamed_2 = 16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtattr {
    pub rta_len: libc::c_ushort,
    pub rta_type: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtmsg {
    pub rtm_family: libc::c_uchar,
    pub rtm_dst_len: libc::c_uchar,
    pub rtm_src_len: libc::c_uchar,
    pub rtm_tos: libc::c_uchar,
    pub rtm_table: libc::c_uchar,
    pub rtm_protocol: libc::c_uchar,
    pub rtm_scope: libc::c_uchar,
    pub rtm_type: libc::c_uchar,
    pub rtm_flags: libc::c_uint,
}
pub type rt_class_t = libc::c_uint;
pub const RT_TABLE_MAX: rt_class_t = 4294967295;
pub const RT_TABLE_LOCAL: rt_class_t = 255;
pub const RT_TABLE_MAIN: rt_class_t = 254;
pub const RT_TABLE_DEFAULT: rt_class_t = 253;
pub const RT_TABLE_COMPAT: rt_class_t = 252;
pub const RT_TABLE_UNSPEC: rt_class_t = 0;
pub type rtattr_type_t = libc::c_uint;
pub const __RTA_MAX: rtattr_type_t = 31;
pub const RTA_NH_ID: rtattr_type_t = 30;
pub const RTA_DPORT: rtattr_type_t = 29;
pub const RTA_SPORT: rtattr_type_t = 28;
pub const RTA_IP_PROTO: rtattr_type_t = 27;
pub const RTA_TTL_PROPAGATE: rtattr_type_t = 26;
pub const RTA_UID: rtattr_type_t = 25;
pub const RTA_PAD: rtattr_type_t = 24;
pub const RTA_EXPIRES: rtattr_type_t = 23;
pub const RTA_ENCAP: rtattr_type_t = 22;
pub const RTA_ENCAP_TYPE: rtattr_type_t = 21;
pub const RTA_PREF: rtattr_type_t = 20;
pub const RTA_NEWDST: rtattr_type_t = 19;
pub const RTA_VIA: rtattr_type_t = 18;
pub const RTA_MFC_STATS: rtattr_type_t = 17;
pub const RTA_MARK: rtattr_type_t = 16;
pub const RTA_TABLE: rtattr_type_t = 15;
pub const RTA_MP_ALGO: rtattr_type_t = 14;
pub const RTA_SESSION: rtattr_type_t = 13;
pub const RTA_CACHEINFO: rtattr_type_t = 12;
pub const RTA_FLOW: rtattr_type_t = 11;
pub const RTA_PROTOINFO: rtattr_type_t = 10;
pub const RTA_MULTIPATH: rtattr_type_t = 9;
pub const RTA_METRICS: rtattr_type_t = 8;
pub const RTA_PREFSRC: rtattr_type_t = 7;
pub const RTA_PRIORITY: rtattr_type_t = 6;
pub const RTA_GATEWAY: rtattr_type_t = 5;
pub const RTA_OIF: rtattr_type_t = 4;
pub const RTA_IIF: rtattr_type_t = 3;
pub const RTA_SRC: rtattr_type_t = 2;
pub const RTA_DST: rtattr_type_t = 1;
pub const RTA_UNSPEC: rtattr_type_t = 0;
pub unsafe extern "C" fn get_default_iface() -> *mut libc::c_char {
    let mut errbuf: [libc::c_char; 256] = [0; 256];
    let mut iface: *mut libc::c_char = pcap_lookupdev(errbuf.as_mut_ptr());
    if iface.is_null() {
        log_fatal(
            b"send\0" as *const u8 as *const libc::c_char,
            b"ZMap could not detect your default network interface. You likely do not privileges to open a raw packet socket. Are you running as root or with the CAP_NET_RAW capability? If you are, you may need to manually set interface using the \"-i\" flag.\0"
                as *const u8 as *const libc::c_char,
        );
    }
    return iface;
}
pub unsafe extern "C" fn read_nl_sock(
    mut sock: libc::c_int,
    mut buf: *mut libc::c_char,
    mut buf_len: libc::c_int,
) -> libc::c_int {
    let mut msg_len: libc::c_int = 0 as libc::c_int;
    let mut pbuf: *mut libc::c_char = buf;
    loop {
        let mut len: libc::c_int = recv(
            sock,
            pbuf as *mut libc::c_void,
            (buf_len - msg_len) as size_t,
            0 as libc::c_int,
        ) as libc::c_int;
        if len <= 0 as libc::c_int {
            log_debug(
                b"get-gw\0" as *const u8 as *const libc::c_char,
                b"recv failed: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return -(1 as libc::c_int);
        }
        let mut nlhdr: *mut nlmsghdr = pbuf as *mut nlmsghdr;
        if (len as libc::c_uint
            >= ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong as libc::c_int
                as libc::c_uint
            && (*nlhdr).nlmsg_len as libc::c_ulong
                >= ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong
            && (*nlhdr).nlmsg_len <= len as libc::c_uint) as libc::c_int
            == 0 as libc::c_int
            || (*nlhdr).nlmsg_type as libc::c_int == 0x2 as libc::c_int
        {
            log_debug(
                b"get-gw\0" as *const u8 as *const libc::c_char,
                b"recv failed: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return -(1 as libc::c_int);
        }
        if (*nlhdr).nlmsg_type as libc::c_int == 0x3 as libc::c_int {
            break;
        }
        msg_len += len;
        pbuf = pbuf.offset(len as isize);
        if (*nlhdr).nlmsg_flags as libc::c_int & 0x2 as libc::c_int == 0 as libc::c_int {
            break;
        }
    }
    return msg_len;
}
pub unsafe extern "C" fn get_iface_hw_addr(
    mut iface: *mut libc::c_char,
    mut hw_mac: *mut libc::c_uchar,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut buffer: ifreq = ifreq {
        ifr_ifrn: C2RustUnnamed_0 {
            ifrn_name: [0; 16],
        },
        ifr_ifru: C2RustUnnamed {
            ifru_addr: sockaddr {
                sa_family: 0,
                sa_data: [0; 14],
            },
        },
    };
    s = socket(2 as libc::c_int, SOCK_DGRAM as libc::c_int, 0 as libc::c_int);
    if s < 0 as libc::c_int {
        log_error(
            b"get_iface_hw_addr\0" as *const u8 as *const libc::c_char,
            b"Unable to open socket: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 1 as libc::c_int;
    }
    memset(
        &mut buffer as *mut ifreq as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ifreq>() as libc::c_ulong,
    );
    strncpy(
        (buffer.ifr_ifrn.ifrn_name).as_mut_ptr(),
        iface,
        16 as libc::c_int as libc::c_ulong,
    );
    ioctl(s, 0x8927 as libc::c_int as libc::c_ulong, &mut buffer as *mut ifreq);
    close(s);
    memcpy(
        hw_mac as *mut libc::c_void,
        (buffer.ifr_ifru.ifru_hwaddr.sa_data).as_mut_ptr() as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn send_nl_req(
    mut msg_type: uint16_t,
    mut seq: uint32_t,
    mut payload: *mut libc::c_void,
    mut payload_len: uint32_t,
) -> libc::c_int {
    let mut sock: libc::c_int = socket(
        16 as libc::c_int,
        SOCK_DGRAM as libc::c_int,
        0 as libc::c_int,
    );
    if sock < 0 as libc::c_int {
        log_error(
            b"get-gw\0" as *const u8 as *const libc::c_char,
            b"unable to get socket: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    if payload_len
        .wrapping_add(
            ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                .wrapping_add(4 as libc::c_uint as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                & !(4 as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as libc::c_ulong) as libc::c_int as libc::c_uint,
        )
        .wrapping_add(4 as libc::c_uint)
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        & !(4 as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
        < payload_len
    {
        close(sock);
        return -(1 as libc::c_int);
    }
    let mut nlmsg: *mut nlmsghdr = 0 as *mut nlmsghdr;
    nlmsg = xmalloc(
        (payload_len
            .wrapping_add(
                ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                    .wrapping_add(4 as libc::c_uint as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    & !(4 as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as libc::c_ulong) as libc::c_int as libc::c_uint,
            )
            .wrapping_add(4 as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            & !(4 as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint))
            as size_t,
    ) as *mut nlmsghdr;
    memset(
        nlmsg as *mut libc::c_void,
        0 as libc::c_int,
        (payload_len
            .wrapping_add(
                ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                    .wrapping_add(4 as libc::c_uint as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    & !(4 as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as libc::c_ulong) as libc::c_int as libc::c_uint,
            )
            .wrapping_add(4 as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            & !(4 as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint))
            as libc::c_ulong,
    );
    memcpy(
        (nlmsg as *mut libc::c_char)
            .offset(
                (0 as libc::c_int
                    + ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                        .wrapping_add(4 as libc::c_uint as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        & !(4 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong) as libc::c_int) as isize,
            ) as *mut libc::c_void,
        payload,
        payload_len as libc::c_ulong,
    );
    (*nlmsg).nlmsg_type = msg_type;
    (*nlmsg)
        .nlmsg_len = payload_len
        .wrapping_add(
            ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                .wrapping_add(4 as libc::c_uint as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                & !(4 as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as libc::c_ulong) as libc::c_int as libc::c_uint,
        );
    (*nlmsg)
        .nlmsg_flags = (0x100 as libc::c_int | 0x200 as libc::c_int | 0x1 as libc::c_int)
        as __u16;
    (*nlmsg).nlmsg_seq = seq;
    (*nlmsg).nlmsg_pid = getpid() as __u32;
    if send(
        sock,
        nlmsg as *const libc::c_void,
        (*nlmsg).nlmsg_len as size_t,
        0 as libc::c_int,
    ) < 0 as libc::c_int as libc::c_long
    {
        log_error(
            b"get-gw\0" as *const u8 as *const libc::c_char,
            b"failure sending: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    free(nlmsg as *mut libc::c_void);
    return sock;
}
pub unsafe extern "C" fn get_iface_ip(
    mut iface: *mut libc::c_char,
    mut ip: *mut in_addr,
) -> libc::c_int {
    let mut sock: libc::c_int = 0;
    let mut ifr: ifreq = ifreq {
        ifr_ifrn: C2RustUnnamed_0 {
            ifrn_name: [0; 16],
        },
        ifr_ifru: C2RustUnnamed {
            ifru_addr: sockaddr {
                sa_family: 0,
                sa_data: [0; 14],
            },
        },
    };
    memset(
        &mut ifr as *mut ifreq as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ifreq>() as libc::c_ulong,
    );
    sock = socket(2 as libc::c_int, SOCK_DGRAM as libc::c_int, 0 as libc::c_int);
    if sock < 0 as libc::c_int {
        log_fatal(
            b"get-iface-ip\0" as *const u8 as *const libc::c_char,
            b"failure opening socket: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    ifr.ifr_ifru.ifru_addr.sa_family = 2 as libc::c_int as sa_family_t;
    strncpy(
        (ifr.ifr_ifrn.ifrn_name).as_mut_ptr(),
        iface,
        (16 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
    );
    if ioctl(sock, 0x8915 as libc::c_int as libc::c_ulong, &mut ifr as *mut ifreq)
        < 0 as libc::c_int
    {
        close(sock);
        log_fatal(
            b"get-iface-ip\0" as *const u8 as *const libc::c_char,
            b"Unable to automatically identify the correct source address for %s interface. ioctl failure: %s. If this is the unexpected interface, you can manually specify the correct interface with \"-i\" flag. If this is the correct interface, you likely need to manually specify the source IP address to use with the \"-S\" flag.\0"
                as *const u8 as *const libc::c_char,
            iface,
            strerror(*__errno_location()),
        );
    }
    (*ip)
        .s_addr = (*(&mut ifr.ifr_ifru.ifru_addr as *mut sockaddr as *mut sockaddr_in))
        .sin_addr
        .s_addr;
    close(sock);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn get_hw_addr(
    mut gw_ip: *mut in_addr,
    mut iface: *mut libc::c_char,
    mut hw_mac: *mut libc::c_uchar,
) -> libc::c_int {
    let mut req: ndmsg = ndmsg {
        ndm_family: 0,
        ndm_pad1: 0,
        ndm_pad2: 0,
        ndm_ifindex: 0,
        ndm_state: 0,
        ndm_flags: 0,
        ndm_type: 0,
    };
    memset(
        &mut req as *mut ndmsg as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ndmsg>() as libc::c_ulong,
    );
    if gw_ip.is_null() || hw_mac.is_null() {
        return -(1 as libc::c_int);
    }
    req.ndm_family = 2 as libc::c_int as __u8;
    req.ndm_ifindex = if_nametoindex(iface) as __s32;
    req.ndm_state = 0x2 as libc::c_int as __u16;
    req.ndm_type = NDA_LLADDR as libc::c_int as __u8;
    let mut sock: libc::c_int = send_nl_req(
        RTM_GETNEIGH as libc::c_int as uint16_t,
        1 as libc::c_int as uint32_t,
        &mut req as *mut ndmsg as *mut libc::c_void,
        ::std::mem::size_of::<ndmsg>() as libc::c_ulong as uint32_t,
    );
    let mut buf: *mut libc::c_char = xmalloc(64000 as libc::c_int as size_t)
        as *mut libc::c_char;
    let mut nl_len: libc::c_int = read_nl_sock(sock, buf, 64000 as libc::c_int);
    if nl_len <= 0 as libc::c_int {
        free(buf as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    let mut nlhdr: *mut nlmsghdr = buf as *mut nlmsghdr;
    while nl_len >= ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong as libc::c_int
        && (*nlhdr).nlmsg_len as libc::c_ulong
            >= ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong
        && (*nlhdr).nlmsg_len <= nl_len as libc::c_uint
    {
        let mut rt_attr: *mut rtattr = 0 as *mut rtattr;
        let mut rt_msg: *mut rtmsg = 0 as *mut rtmsg;
        let mut rt_len: libc::c_int = 0;
        let mut mac: [libc::c_uchar; 6] = [0; 6];
        let mut dst_ip: in_addr = in_addr { s_addr: 0 };
        let mut correct_ip: libc::c_int = 0 as libc::c_int;
        rt_msg = (nlhdr as *mut libc::c_char)
            .offset(
                (0 as libc::c_int
                    + ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                        .wrapping_add(4 as libc::c_uint as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        & !(4 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong) as libc::c_int) as isize,
            ) as *mut libc::c_void as *mut rtmsg;
        if (*rt_msg).rtm_family as libc::c_int != 2 as libc::c_int {
            free(buf as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        rt_attr = (rt_msg as *mut libc::c_char)
            .offset(
                ((::std::mem::size_of::<rtmsg>() as libc::c_ulong)
                    .wrapping_add(4 as libc::c_uint as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    & !(4 as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as libc::c_ulong) as isize,
            ) as *mut rtattr;
        rt_len = ((*nlhdr).nlmsg_len as libc::c_ulong)
            .wrapping_sub(
                (::std::mem::size_of::<rtmsg>() as libc::c_ulong)
                    .wrapping_add(
                        ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                            .wrapping_add(4 as libc::c_uint as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            & !(4 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                as libc::c_ulong) as libc::c_int as libc::c_ulong,
                    )
                    .wrapping_add(4 as libc::c_uint as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    & !(4 as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as libc::c_ulong,
            ) as libc::c_int;
        while rt_len >= ::std::mem::size_of::<rtattr>() as libc::c_ulong as libc::c_int
            && (*rt_attr).rta_len as libc::c_ulong
                >= ::std::mem::size_of::<rtattr>() as libc::c_ulong
            && (*rt_attr).rta_len as libc::c_int <= rt_len
        {
            match (*rt_attr).rta_type as libc::c_int {
                2 => {
                    if ((*rt_attr).rta_len as libc::c_int as libc::c_ulong)
                        .wrapping_sub(
                            ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
                                .wrapping_add(4 as libc::c_uint as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                & !(4 as libc::c_uint)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    as libc::c_ulong)
                                .wrapping_add(0 as libc::c_int as libc::c_ulong),
                        ) != 6 as libc::c_int as libc::c_ulong
                    {
                        log_fatal(
                            b"get_gateway\0" as *const u8 as *const libc::c_char,
                            b"Unexpected hardware address length (%d). If you are using a VPN, supply the --iplayer flag (and provide an interface via -i)\0"
                                as *const u8 as *const libc::c_char,
                            ((*rt_attr).rta_len as libc::c_int as libc::c_ulong)
                                .wrapping_sub(
                                    ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
                                        .wrapping_add(4 as libc::c_uint as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        & !(4 as libc::c_uint)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                            as libc::c_ulong)
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                                ),
                        );
                    }
                    memcpy(
                        mac.as_mut_ptr() as *mut libc::c_void,
                        (rt_attr as *mut libc::c_char)
                            .offset(
                                ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
                                    .wrapping_add(4 as libc::c_uint as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    & !(4 as libc::c_uint)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                        as libc::c_ulong)
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
                            ) as *mut libc::c_void,
                        6 as libc::c_int as libc::c_ulong,
                    );
                }
                1 => {
                    if ((*rt_attr).rta_len as libc::c_int as libc::c_ulong)
                        .wrapping_sub(
                            ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
                                .wrapping_add(4 as libc::c_uint as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                & !(4 as libc::c_uint)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    as libc::c_ulong)
                                .wrapping_add(0 as libc::c_int as libc::c_ulong),
                        ) != ::std::mem::size_of::<in_addr>() as libc::c_ulong
                    {
                        log_fatal(
                            b"get_gateway\0" as *const u8 as *const libc::c_char,
                            b"Unexpected IP address length (%d). If you are using a VPN, supply the --iplayer flag (and provide an interface via -i)\0"
                                as *const u8 as *const libc::c_char,
                            ((*rt_attr).rta_len as libc::c_int as libc::c_ulong)
                                .wrapping_sub(
                                    ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
                                        .wrapping_add(4 as libc::c_uint as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        & !(4 as libc::c_uint)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                            as libc::c_ulong)
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong),
                                ),
                        );
                    }
                    memcpy(
                        &mut dst_ip as *mut in_addr as *mut libc::c_void,
                        (rt_attr as *mut libc::c_char)
                            .offset(
                                ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
                                    .wrapping_add(4 as libc::c_uint as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    & !(4 as libc::c_uint)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                        as libc::c_ulong)
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
                            ) as *mut libc::c_void,
                        ::std::mem::size_of::<in_addr>() as libc::c_ulong,
                    );
                    if memcmp(
                        &mut dst_ip as *mut in_addr as *const libc::c_void,
                        gw_ip as *const libc::c_void,
                        ::std::mem::size_of::<in_addr>() as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        correct_ip = 1 as libc::c_int;
                    }
                }
                _ => {}
            }
            rt_len = (rt_len as libc::c_uint)
                .wrapping_sub(
                    ((*rt_attr).rta_len as libc::c_uint)
                        .wrapping_add(4 as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        & !(4 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                ) as libc::c_int as libc::c_int;
            rt_attr = (rt_attr as *mut libc::c_char)
                .offset(
                    (((*rt_attr).rta_len as libc::c_uint)
                        .wrapping_add(4 as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        & !(4 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)) as isize,
                ) as *mut rtattr;
        }
        if correct_ip != 0 {
            memcpy(
                hw_mac as *mut libc::c_void,
                mac.as_mut_ptr() as *const libc::c_void,
                6 as libc::c_int as libc::c_ulong,
            );
            free(buf as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        nl_len = (nl_len as libc::c_uint)
            .wrapping_sub(
                ((*nlhdr).nlmsg_len)
                    .wrapping_add(4 as libc::c_uint)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    & !(4 as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint),
            ) as libc::c_int as libc::c_int;
        nlhdr = (nlhdr as *mut libc::c_char)
            .offset(
                (((*nlhdr).nlmsg_len)
                    .wrapping_add(4 as libc::c_uint)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    & !(4 as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)) as isize,
            ) as *mut nlmsghdr;
    }
    free(buf as *mut libc::c_void);
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn get_default_gw(
    mut gw: *mut in_addr,
    mut iface: *mut libc::c_char,
) -> libc::c_int {
    let mut _iface: [libc::c_char; 16] = [0; 16];
    memset(
        _iface.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        16 as libc::c_int as libc::c_ulong,
    );
    _get_default_gw(gw, _iface.as_mut_ptr());
    if strcmp(iface, _iface.as_mut_ptr()) != 0 {
        log_fatal(
            b"get-gateway\0" as *const u8 as *const libc::c_char,
            b"The specified network (\"%s\") does not match the interface associated with the default gateway (%s). You will need to manually specify the MAC address of your gateway using the \"--gateway-mac\" flag.\0"
                as *const u8 as *const libc::c_char,
            iface,
            _iface.as_mut_ptr(),
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _get_default_gw(
    mut gw: *mut in_addr,
    mut iface: *mut libc::c_char,
) -> libc::c_int {
    let mut req: rtmsg = rtmsg {
        rtm_family: 0,
        rtm_dst_len: 0,
        rtm_src_len: 0,
        rtm_tos: 0,
        rtm_table: 0,
        rtm_protocol: 0,
        rtm_scope: 0,
        rtm_type: 0,
        rtm_flags: 0,
    };
    let mut nl_len: libc::c_uint = 0;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut nlhdr: *mut nlmsghdr = 0 as *mut nlmsghdr;
    if gw.is_null() || iface.is_null() {
        return -(1 as libc::c_int);
    }
    memset(
        &mut req as *mut rtmsg as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<rtmsg>() as libc::c_ulong,
    );
    let mut sock: libc::c_int = send_nl_req(
        RTM_GETROUTE as libc::c_int as uint16_t,
        0 as libc::c_int as uint32_t,
        &mut req as *mut rtmsg as *mut libc::c_void,
        ::std::mem::size_of::<rtmsg>() as libc::c_ulong as uint32_t,
    );
    nl_len = read_nl_sock(
        sock,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as libc::c_int,
    ) as libc::c_uint;
    if nl_len <= 0 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int);
    }
    nlhdr = buf.as_mut_ptr() as *mut nlmsghdr;
    while nl_len
        >= ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong as libc::c_int
            as libc::c_uint
        && (*nlhdr).nlmsg_len as libc::c_ulong
            >= ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong
        && (*nlhdr).nlmsg_len <= nl_len
    {
        let mut rt_attr: *mut rtattr = 0 as *mut rtattr;
        let mut rt_msg: *mut rtmsg = 0 as *mut rtmsg;
        let mut rt_len: libc::c_int = 0;
        let mut has_gw: libc::c_int = 0 as libc::c_int;
        rt_msg = (nlhdr as *mut libc::c_char)
            .offset(
                (0 as libc::c_int
                    + ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                        .wrapping_add(4 as libc::c_uint as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        & !(4 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong) as libc::c_int) as isize,
            ) as *mut libc::c_void as *mut rtmsg;
        if (*rt_msg).rtm_family as libc::c_int != 2 as libc::c_int
            || (*rt_msg).rtm_table as libc::c_int != RT_TABLE_MAIN as libc::c_int
        {
            nl_len = nl_len
                .wrapping_sub(
                    ((*nlhdr).nlmsg_len)
                        .wrapping_add(4 as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        & !(4 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                );
            nlhdr = (nlhdr as *mut libc::c_char)
                .offset(
                    (((*nlhdr).nlmsg_len)
                        .wrapping_add(4 as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        & !(4 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)) as isize,
                ) as *mut nlmsghdr;
        } else {
            rt_attr = (rt_msg as *mut libc::c_char)
                .offset(
                    ((::std::mem::size_of::<rtmsg>() as libc::c_ulong)
                        .wrapping_add(4 as libc::c_uint as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        & !(4 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong) as isize,
                ) as *mut rtattr;
            rt_len = ((*nlhdr).nlmsg_len as libc::c_ulong)
                .wrapping_sub(
                    (::std::mem::size_of::<rtmsg>() as libc::c_ulong)
                        .wrapping_add(
                            ((::std::mem::size_of::<nlmsghdr>() as libc::c_ulong)
                                .wrapping_add(4 as libc::c_uint as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                & !(4 as libc::c_uint)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    as libc::c_ulong) as libc::c_int as libc::c_ulong,
                        )
                        .wrapping_add(4 as libc::c_uint as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        & !(4 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong,
                ) as libc::c_int;
            while rt_len
                >= ::std::mem::size_of::<rtattr>() as libc::c_ulong as libc::c_int
                && (*rt_attr).rta_len as libc::c_ulong
                    >= ::std::mem::size_of::<rtattr>() as libc::c_ulong
                && (*rt_attr).rta_len as libc::c_int <= rt_len
            {
                match (*rt_attr).rta_type as libc::c_int {
                    4 => {
                        if_indextoname(
                            *((rt_attr as *mut libc::c_char)
                                .offset(
                                    ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
                                        .wrapping_add(4 as libc::c_uint as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        & !(4 as libc::c_uint)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                            as libc::c_ulong)
                                        .wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
                                ) as *mut libc::c_void as *mut libc::c_int) as libc::c_uint,
                            iface,
                        );
                    }
                    5 => {
                        (*gw)
                            .s_addr = *((rt_attr as *mut libc::c_char)
                            .offset(
                                ((::std::mem::size_of::<rtattr>() as libc::c_ulong)
                                    .wrapping_add(4 as libc::c_uint as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    & !(4 as libc::c_uint)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                        as libc::c_ulong)
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
                            ) as *mut libc::c_void as *mut libc::c_uint);
                        has_gw = 1 as libc::c_int;
                    }
                    _ => {}
                }
                rt_len = (rt_len as libc::c_uint)
                    .wrapping_sub(
                        ((*rt_attr).rta_len as libc::c_uint)
                            .wrapping_add(4 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            & !(4 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    ) as libc::c_int as libc::c_int;
                rt_attr = (rt_attr as *mut libc::c_char)
                    .offset(
                        (((*rt_attr).rta_len as libc::c_uint)
                            .wrapping_add(4 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            & !(4 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint)) as isize,
                    ) as *mut rtattr;
            }
            if has_gw != 0 {
                return 0 as libc::c_int;
            }
            nl_len = nl_len
                .wrapping_sub(
                    ((*nlhdr).nlmsg_len)
                        .wrapping_add(4 as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        & !(4 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                );
            nlhdr = (nlhdr as *mut libc::c_char)
                .offset(
                    (((*nlhdr).nlmsg_len)
                        .wrapping_add(4 as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        & !(4 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)) as isize,
                ) as *mut nlmsghdr;
        }
    }
    return -(1 as libc::c_int);
}
