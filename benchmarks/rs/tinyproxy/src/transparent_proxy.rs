use ::libc;
extern "C" {
    pub type sockaddr_x25;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    pub type htab;
    pub type upstream;
    pub type reversepath;
    pub type buffer_s;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn getsockname(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    fn sblist_get(l: *mut sblist, item: size_t) -> *mut libc::c_void;
    fn orderedmap_find(
        o: *mut orderedmap,
        key: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn indicate_http_error(
        connptr: *mut conn_s,
        number: libc::c_int,
        message: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn log_message(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn strlcpy(dst: *mut libc::c_char, src: *const libc::c_char, size: size_t) -> size_t;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __socklen_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type socklen_t = __socklen_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
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
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
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
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_s {
    pub basicauth_list: *mut sblist,
    pub logf_name: *mut libc::c_char,
    pub syslog: libc::c_uint,
    pub port: libc::c_uint,
    pub stathost: *mut libc::c_char,
    pub quit: libc::c_uint,
    pub maxclients: libc::c_uint,
    pub user: *mut libc::c_char,
    pub group: *mut libc::c_char,
    pub listen_addrs: *mut sblist,
    pub filter: *mut libc::c_char,
    pub filter_opts: libc::c_uint,
    pub add_xtinyproxy: libc::c_uint,
    pub reversepath_list: *mut reversepath,
    pub reverseonly: libc::c_uint,
    pub reversemagic: libc::c_uint,
    pub reversebaseurl: *mut libc::c_char,
    pub upstream_list: *mut upstream,
    pub pidpath: *mut libc::c_char,
    pub idletimeout: libc::c_uint,
    pub bind_addrs: *mut sblist,
    pub bindsame: libc::c_uint,
    pub via_proxy_name: *mut libc::c_char,
    pub disable_viaheader: libc::c_uint,
    pub errorpages: *mut htab,
    pub errorpage_undef: *mut libc::c_char,
    pub statpage: *mut libc::c_char,
    pub access_list: acl_list_t,
    pub connect_ports: *mut sblist,
    pub anonymous_map: *mut htab,
    pub add_headers: *mut sblist,
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
pub type acl_list_t = *mut sblist;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conn_s {
    pub client_fd: libc::c_int,
    pub server_fd: libc::c_int,
    pub cbuffer: *mut buffer_s,
    pub sbuffer: *mut buffer_s,
    pub request_line: *mut libc::c_char,
    pub connect_method: libc::c_uint,
    pub show_stats: libc::c_uint,
    pub error_variables: *mut htab,
    pub error_number: libc::c_int,
    pub error_string: *mut libc::c_char,
    pub content_length: C2RustUnnamed_1,
    pub server_ip_addr: *mut libc::c_char,
    pub client_ip_addr: *mut libc::c_char,
    pub protocol: C2RustUnnamed_0,
    pub reversepath: *mut libc::c_char,
    pub upstream_proxy: *mut upstream,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub major: libc::c_uint,
    pub minor: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub server: libc::c_long,
    pub client: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct orderedmap {
    pub values: *mut sblist,
    pub map: *mut htab,
}
pub type orderedmap_0 = *mut orderedmap;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sockaddr_union {
    pub v4: sockaddr_in,
    pub v6: sockaddr_in6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct request_s {
    pub method: *mut libc::c_char,
    pub protocol: *mut libc::c_char,
    pub host: *mut libc::c_char,
    pub port: uint16_t,
    pub path: *mut libc::c_char,
}
unsafe extern "C" fn build_url(
    mut url: *mut *mut libc::c_char,
    mut host: *const libc::c_char,
    mut port: libc::c_int,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    len = (strlen(host))
        .wrapping_add(strlen(path))
        .wrapping_add(14 as libc::c_int as libc::c_ulong) as libc::c_int;
    *url = realloc(*url as *mut libc::c_void, len as libc::c_ulong) as *mut libc::c_char;
    if (*url).is_null() {
        return -(1 as libc::c_int);
    }
    return snprintf(
        *url,
        len as libc::c_ulong,
        b"http://%s:%d%s\0" as *const u8 as *const libc::c_char,
        host,
        port,
        path,
    );
}
pub unsafe extern "C" fn do_transparent_proxy(
    mut connptr: *mut conn_s,
    mut hashofheaders: orderedmap_0,
    mut request: *mut request_s,
    mut conf: *mut config_s,
    mut url: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut length: socklen_t = 0;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ulen: size_t = strlen(*url);
    let mut i: size_t = 0;
    data = orderedmap_find(hashofheaders, b"host\0" as *const u8 as *const libc::c_char);
    if data.is_null() {
        's_76: {
            let mut dest_addr: sockaddr_union = sockaddr_union {
                v4: sockaddr_in {
                    sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr { s_addr: 0 },
                    sin_zero: [0; 8],
                },
            };
            let mut dest_inaddr: *const libc::c_void = 0 as *const libc::c_void;
            let mut namebuf: [libc::c_char; 47] = [0; 47];
            let mut af: libc::c_int = 0;
            length = ::std::mem::size_of::<sockaddr_union>() as libc::c_ulong
                as socklen_t;
            if !(getsockname(
                (*connptr).client_fd,
                __SOCKADDR_ARG {
                    __sockaddr__: &mut dest_addr as *mut sockaddr_union
                        as *mut libc::c_void as *mut sockaddr,
                },
                &mut length,
            ) < 0 as libc::c_int
                || length as libc::c_ulong
                    > ::std::mem::size_of::<sockaddr_union>() as libc::c_ulong)
            {
                af = dest_addr.v4.sin_family as libc::c_int;
                dest_inaddr = if dest_addr.v4.sin_family as libc::c_int
                    == 2 as libc::c_int
                {
                    &mut dest_addr.v4.sin_addr as *mut in_addr as *mut libc::c_void
                } else if dest_addr.v4.sin_family as libc::c_int == 10 as libc::c_int {
                    &mut dest_addr.v6.sin6_addr as *mut in6_addr as *mut libc::c_void
                } else {
                    0 as *mut libc::c_void
                };
                if !(inet_ntop(
                    af,
                    dest_inaddr,
                    namebuf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 47]>() as libc::c_ulong
                        as socklen_t,
                ))
                    .is_null()
                {
                    (*request).host = strdup(namebuf.as_mut_ptr());
                    (*request)
                        .port = ntohs(
                        (if dest_addr.v4.sin_family as libc::c_int == 2 as libc::c_int {
                            dest_addr.v4.sin_port as libc::c_int
                        } else if dest_addr.v4.sin_family as libc::c_int
                            == 10 as libc::c_int
                        {
                            dest_addr.v6.sin6_port as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as uint16_t,
                    );
                    (*request)
                        .path = malloc(
                        ulen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as *mut libc::c_char;
                    strlcpy(
                        (*request).path,
                        *url,
                        ulen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    );
                    build_url(
                        url,
                        (*request).host,
                        (*request).port as libc::c_int,
                        (*request).path,
                    );
                    log_message(
                        6 as libc::c_int,
                        b"process_request: trans IP %s %s for %d\0" as *const u8
                            as *const libc::c_char,
                        (*request).method,
                        *url,
                        (*connptr).client_fd,
                    );
                    break 's_76;
                }
            }
            log_message(
                3 as libc::c_int,
                b"process_request: cannot get destination IP for %d\0" as *const u8
                    as *const libc::c_char,
                (*connptr).client_fd,
            );
            indicate_http_error(
                connptr,
                400 as libc::c_int,
                b"Bad Request\0" as *const u8 as *const libc::c_char,
                b"detail\0" as *const u8 as *const libc::c_char,
                b"Unknown destination\0" as *const u8 as *const libc::c_char,
                b"url\0" as *const u8 as *const libc::c_char,
                *url,
                0 as *mut libc::c_void,
            );
            return 0 as libc::c_int;
        }
    } else {
        length = strlen(data) as socklen_t;
        (*request)
            .host = malloc(
            length.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
        ) as *mut libc::c_char;
        if sscanf(
            data,
            b"%[^:]:%hu\0" as *const u8 as *const libc::c_char,
            (*request).host,
            &mut (*request).port as *mut uint16_t,
        ) != 2 as libc::c_int
        {
            strlcpy(
                (*request).host,
                data,
                length.wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
            );
            (*request).port = 80 as libc::c_int as uint16_t;
        }
        (*request)
            .path = malloc(ulen.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        strlcpy(
            (*request).path,
            *url,
            ulen.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        build_url(url, (*request).host, (*request).port as libc::c_int, (*request).path);
        log_message(
            6 as libc::c_int,
            b"process_request: trans Host %s %s for %d\0" as *const u8
                as *const libc::c_char,
            (*request).method,
            *url,
            (*connptr).client_fd,
        );
    }
    if ((*conf).listen_addrs).is_null() {
        return 1 as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < (*(*conf).listen_addrs).count {
        let mut addr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        addr = sblist_get((*conf).listen_addrs, i) as *mut *mut libc::c_char;
        if !addr.is_null() && !(*addr).is_null()
            && strcmp((*request).host, *addr) == 0 as libc::c_int
        {
            log_message(
                3 as libc::c_int,
                b"transparent: destination IP %s is local on socket fd %d\0" as *const u8
                    as *const libc::c_char,
                (*request).host,
                (*connptr).client_fd,
            );
            indicate_http_error(
                connptr,
                400 as libc::c_int,
                b"Bad Request\0" as *const u8 as *const libc::c_char,
                b"detail\0" as *const u8 as *const libc::c_char,
                b"You tried to connect to the machine the proxy is running on\0"
                    as *const u8 as *const libc::c_char,
                b"url\0" as *const u8 as *const libc::c_char,
                *url,
                0 as *mut libc::c_void,
            );
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int;
}
