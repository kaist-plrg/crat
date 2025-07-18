use ::libc;
extern "C" {
    pub type htab;
    pub type buffer_s;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn atol(__nptr: *const libc::c_char) -> libc::c_long;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __n: size_t,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
    static mut config: *mut config_s;
    fn set_socket_timeout(fd: libc::c_int);
    fn opensock(
        host: *const libc::c_char,
        port: libc::c_int,
        bind_to: *const libc::c_char,
    ) -> libc::c_int;
    fn socket_nonblocking(sock: libc::c_int) -> libc::c_int;
    fn socket_blocking(sock: libc::c_int) -> libc::c_int;
    fn check_acl(
        ip_address: *const libc::c_char,
        addr: *mut sockaddr_union,
        access_list: acl_list_t,
    ) -> libc::c_int;
    fn sblist_get(l: *mut sblist, item: size_t) -> *mut libc::c_void;
    fn getsock_ip(fd: libc::c_int, ipaddr: *mut libc::c_char) -> libc::c_int;
    fn getpeer_information(
        addr: *mut sockaddr_union,
        ipaddr: *mut libc::c_char,
        ipaddr_len: size_t,
    );
    fn is_anonymous_enabled(conf: *mut config_s) -> libc::c_short;
    fn anonymous_search(conf: *mut config_s, s: *const libc::c_char) -> libc::c_int;
    fn buffer_size(buffptr: *mut buffer_s) -> size_t;
    fn read_buffer(fd: libc::c_int, buffptr: *mut buffer_s) -> ssize_t;
    fn write_buffer(fd: libc::c_int, buffptr: *mut buffer_s) -> ssize_t;
    fn conn_init_contents(
        connptr: *mut conn_s,
        ipaddr: *const libc::c_char,
        sock_ipaddr: *const libc::c_char,
    ) -> libc::c_int;
    fn conn_destroy_contents(connptr: *mut conn_s);
    fn filter_run(str: *const libc::c_char) -> libc::c_int;
    fn orderedmap_create(nbuckets: size_t) -> *mut orderedmap;
    fn orderedmap_destroy(o: *mut orderedmap) -> *mut libc::c_void;
    fn orderedmap_append(
        o: *mut orderedmap,
        key: *const libc::c_char,
        value: *mut libc::c_char,
    ) -> libc::c_int;
    fn orderedmap_find(
        o: *mut orderedmap,
        key: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn orderedmap_remove(o: *mut orderedmap, key: *const libc::c_char) -> libc::c_int;
    fn orderedmap_next(
        o: *mut orderedmap,
        iter: size_t,
        key: *mut *mut libc::c_char,
        value: *mut *mut libc::c_char,
    ) -> size_t;
    fn send_http_error_message(connptr: *mut conn_s) -> libc::c_int;
    fn indicate_http_error(
        connptr: *mut conn_s,
        number: libc::c_int,
        message: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn send_http_headers(
        connptr: *mut conn_s,
        code: libc::c_int,
        message: *const libc::c_char,
        extra: *const libc::c_char,
    ) -> libc::c_int;
    fn log_message(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn safe_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
    fn write_message(fd: libc::c_int, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn readline(fd: libc::c_int, whole_buffer: *mut *mut libc::c_char) -> ssize_t;
    fn showstats(connptr: *mut conn_s) -> libc::c_int;
    fn update_stats(update_level: status_t) -> libc::c_int;
    fn strlcpy(dst: *mut libc::c_char, src: *const libc::c_char, size: size_t) -> size_t;
    fn chomp(buffer: *mut libc::c_char, length: size_t) -> ssize_t;
    fn reverse_rewrite_url(
        connptr: *mut conn_s,
        hashofheaders: orderedmap_0,
        url: *mut libc::c_char,
        status: *mut libc::c_int,
    ) -> *mut libc::c_char;
    fn do_transparent_proxy(
        connptr: *mut conn_s,
        hashofheaders: orderedmap_0,
        request: *mut request_s,
        config_0: *mut config_s,
        url: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn proxy_type_name(type_0: proxy_type) -> *const libc::c_char;
    fn upstream_get(host: *mut libc::c_char, up: *mut upstream) -> *mut upstream;
    fn check_allowed_connect_ports(
        port: libc::c_int,
        connect_ports: *mut sblist,
    ) -> libc::c_int;
    fn basicauth_check(
        authlist: *mut sblist,
        authstring: *const libc::c_char,
    ) -> libc::c_int;
    fn connection_loops(addr: *mut sockaddr_union) -> libc::c_int;
    fn mypoll(
        fds: *mut pollfd_struct,
        nfds: libc::c_int,
        timeout: libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type ssize_t = __ssize_t;
pub type sa_family_t = libc::c_ushort;
pub type C2RustUnnamed = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed = 67108864;
pub const MSG_BATCH: C2RustUnnamed = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed = 65536;
pub const MSG_MORE: C2RustUnnamed = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed = 8192;
pub const MSG_RST: C2RustUnnamed = 4096;
pub const MSG_CONFIRM: C2RustUnnamed = 2048;
pub const MSG_SYN: C2RustUnnamed = 1024;
pub const MSG_FIN: C2RustUnnamed = 512;
pub const MSG_WAITALL: C2RustUnnamed = 256;
pub const MSG_EOR: C2RustUnnamed = 128;
pub const MSG_DONTWAIT: C2RustUnnamed = 64;
pub const MSG_TRUNC: C2RustUnnamed = 32;
pub const MSG_PROXY: C2RustUnnamed = 16;
pub const MSG_CTRUNC: C2RustUnnamed = 8;
pub const MSG_TRYHARD: C2RustUnnamed = 4;
pub const MSG_DONTROUTE: C2RustUnnamed = 4;
pub const MSG_PEEK: C2RustUnnamed = 2;
pub const MSG_OOB: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const SHUT_RDWR: C2RustUnnamed_0 = 2;
pub const SHUT_WR: C2RustUnnamed_0 = 1;
pub const SHUT_RD: C2RustUnnamed_0 = 0;
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
    pub __in6_u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
pub struct upstream {
    pub next: *mut upstream,
    pub host: *mut libc::c_char,
    pub ua: C2RustUnnamed_4,
    pub pass: *mut libc::c_char,
    pub port: libc::c_int,
    pub target: hostspec,
    pub type_0: proxy_type,
}
pub type proxy_type = libc::c_uint;
pub const PT_SOCKS5: proxy_type = 3;
pub const PT_SOCKS4: proxy_type = 2;
pub const PT_HTTP: proxy_type = 1;
pub const PT_NONE: proxy_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostspec {
    pub type_0: hostspec_type,
    pub address: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub string: *mut libc::c_char,
    pub ip: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub network: [libc::c_uchar; 16],
    pub mask: [libc::c_uchar; 16],
}
pub type hostspec_type = libc::c_uint;
pub const HST_NUMERIC: hostspec_type = 2;
pub const HST_STRING: hostspec_type = 1;
pub const HST_NONE: hostspec_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub user: *mut libc::c_char,
    pub authstr: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct reversepath {
    pub next: *mut reversepath,
    pub path: *mut libc::c_char,
    pub url: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sockaddr_union {
    pub v4: sockaddr_in,
    pub v6: sockaddr_in6,
}
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
    pub content_length: C2RustUnnamed_6,
    pub server_ip_addr: *mut libc::c_char,
    pub client_ip_addr: *mut libc::c_char,
    pub protocol: C2RustUnnamed_5,
    pub reversepath: *mut libc::c_char,
    pub upstream_proxy: *mut upstream,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub major: libc::c_uint,
    pub minor: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub server: libc::c_long,
    pub client: libc::c_long,
}
pub type filter_options = libc::c_uint;
pub const FILTER_OPT_TYPE_FNMATCH: filter_options = 1024;
pub const FILTER_OPT_TYPE_ERE: filter_options = 512;
pub const FILTER_OPT_TYPE_BRE: filter_options = 256;
pub const FILTER_OPT_DEFAULT_DENY: filter_options = 4;
pub const FILTER_OPT_URL: filter_options = 2;
pub const FILTER_OPT_CASESENSITIVE: filter_options = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct orderedmap {
    pub values: *mut sblist,
    pub map: *mut htab,
}
pub type orderedmap_0 = *mut orderedmap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct request_s {
    pub method: *mut libc::c_char,
    pub protocol: *mut libc::c_char,
    pub host: *mut libc::c_char,
    pub port: uint16_t,
    pub path: *mut libc::c_char,
}
pub type pollfd_struct = pollfd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type status_t = libc::c_uint;
pub const STAT_DENIED: status_t = 4;
pub const STAT_REFUSE: status_t = 3;
pub const STAT_CLOSE: status_t = 2;
pub const STAT_OPEN: status_t = 1;
pub const STAT_BADCONN: status_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_header_t {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
}
unsafe extern "C" fn read_request_line(mut connptr: *mut conn_s) -> libc::c_int {
    let mut len: ssize_t = 0;
    loop {
        len = readline((*connptr).client_fd, &mut (*connptr).request_line);
        if len <= 0 as libc::c_int as libc::c_long {
            log_message(
                3 as libc::c_int,
                b"read_request_line: Client (file descriptor: %d) closed socket before read.\0"
                    as *const u8 as *const libc::c_char,
                (*connptr).client_fd,
            );
            return -(1 as libc::c_int);
        }
        if !(chomp((*connptr).request_line, len as size_t) == len) {
            break;
        }
        free((*connptr).request_line as *mut libc::c_void);
        (*connptr).request_line = 0 as *mut libc::c_char;
    }
    log_message(
        8 as libc::c_int,
        b"Request (file descriptor %d): %s\0" as *const u8 as *const libc::c_char,
        (*connptr).client_fd,
        (*connptr).request_line,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn free_request_struct(mut request: *mut request_s) {
    if request.is_null() {
        return;
    }
    free((*request).method as *mut libc::c_void);
    (*request).method = 0 as *mut libc::c_char;
    free((*request).protocol as *mut libc::c_void);
    (*request).protocol = 0 as *mut libc::c_char;
    if !((*request).host).is_null() {
        free((*request).host as *mut libc::c_void);
        (*request).host = 0 as *mut libc::c_char;
    }
    if !((*request).path).is_null() {
        free((*request).path as *mut libc::c_void);
        (*request).path = 0 as *mut libc::c_char;
    }
    free(request as *mut libc::c_void);
    request = 0 as *mut request_s;
}
unsafe extern "C" fn strip_username_password(mut host: *mut libc::c_char) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = strchr(host, '@' as i32);
    if p.is_null() {
        return;
    }
    p = p.offset(1);
    p;
    while *p != 0 {
        let fresh0 = p;
        p = p.offset(1);
        let fresh1 = host;
        host = host.offset(1);
        *fresh1 = *fresh0;
    }
    *host = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn strip_return_port(mut host: *mut libc::c_char) -> libc::c_int {
    let mut ptr1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut port: libc::c_int = 0;
    ptr1 = strrchr(host, ':' as i32);
    if ptr1.is_null() {
        return 0 as libc::c_int;
    }
    ptr2 = strchr(ptr1, ']' as i32);
    if !ptr2.is_null() {
        return 0 as libc::c_int;
    }
    let fresh2 = ptr1;
    ptr1 = ptr1.offset(1);
    *fresh2 = '\0' as i32 as libc::c_char;
    if sscanf(
        ptr1,
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut port as *mut libc::c_int,
    ) != 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return port;
}
unsafe extern "C" fn extract_url(
    mut url: *const libc::c_char,
    mut default_port: libc::c_int,
    mut request: *mut request_s,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut port: libc::c_int = 0;
    p = strchr(url, '/' as i32);
    if !p.is_null() {
        let mut len: libc::c_int = 0;
        len = p.offset_from(url) as libc::c_long as libc::c_int;
        (*request)
            .host = malloc((len + 1 as libc::c_int) as libc::c_ulong)
            as *mut libc::c_char;
        memcpy(
            (*request).host as *mut libc::c_void,
            url as *const libc::c_void,
            len as libc::c_ulong,
        );
        *((*request).host).offset(len as isize) = '\0' as i32 as libc::c_char;
        (*request).path = strdup(p);
    } else {
        (*request).host = strdup(url);
        (*request).path = strdup(b"/\0" as *const u8 as *const libc::c_char);
    }
    if ((*request).host).is_null() || ((*request).path).is_null() {
        if !((*request).host).is_null() {
            free((*request).host as *mut libc::c_void);
            (*request).host = 0 as *mut libc::c_char;
        }
        if !((*request).path).is_null() {
            free((*request).path as *mut libc::c_void);
            (*request).path = 0 as *mut libc::c_char;
        }
        return -(1 as libc::c_int);
    } else {
        strip_username_password((*request).host);
        port = strip_return_port((*request).host);
        (*request)
            .port = (if port != 0 as libc::c_int { port } else { default_port })
            as uint16_t;
        p = strrchr((*request).host, ']' as i32);
        if !p.is_null() && *(*request).host as libc::c_int == '[' as i32 {
            memmove(
                (*request).host as *mut libc::c_void,
                ((*request).host).offset(1 as libc::c_int as isize)
                    as *const libc::c_void,
                (strlen((*request).host)).wrapping_sub(2 as libc::c_int as libc::c_ulong),
            );
            *p = '\0' as i32 as libc::c_char;
            p = p.offset(-1);
            p;
            *p = '\0' as i32 as libc::c_char;
        }
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn establish_http_connection(
    mut connptr: *mut conn_s,
    mut request: *mut request_s,
) -> libc::c_int {
    let mut portbuff: [libc::c_char; 7] = [0; 7];
    let mut dst: [libc::c_char; 16] = [0; 16];
    if (*request).port as libc::c_int != 80 as libc::c_int
        && (*request).port as libc::c_int != 443 as libc::c_int
    {
        snprintf(
            portbuff.as_mut_ptr(),
            7 as libc::c_int as libc::c_ulong,
            b":%u\0" as *const u8 as *const libc::c_char,
            (*request).port as libc::c_int,
        );
    } else {
        portbuff[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }
    if inet_pton(
        10 as libc::c_int,
        (*request).host,
        dst.as_mut_ptr() as *mut libc::c_void,
    ) > 0 as libc::c_int
    {
        return write_message(
            (*connptr).server_fd,
            b"%s %s HTTP/1.%u\r\nHost: [%s]%s\r\nConnection: close\r\n\0" as *const u8
                as *const libc::c_char,
            (*request).method,
            (*request).path,
            if (*connptr).protocol.major != 1 as libc::c_int as libc::c_uint {
                0 as libc::c_int as libc::c_uint
            } else {
                (*connptr).protocol.minor
            },
            (*request).host,
            portbuff.as_mut_ptr(),
        )
    } else if !((*connptr).upstream_proxy).is_null()
        && (*(*connptr).upstream_proxy).type_0 as libc::c_uint
            == PT_HTTP as libc::c_int as libc::c_uint
        && !((*(*connptr).upstream_proxy).ua.authstr).is_null()
    {
        return write_message(
            (*connptr).server_fd,
            b"%s %s HTTP/1.%u\r\nHost: %s%s\r\nConnection: close\r\nProxy-Authorization: Basic %s\r\n\0"
                as *const u8 as *const libc::c_char,
            (*request).method,
            (*request).path,
            if (*connptr).protocol.major != 1 as libc::c_int as libc::c_uint {
                0 as libc::c_int as libc::c_uint
            } else {
                (*connptr).protocol.minor
            },
            (*request).host,
            portbuff.as_mut_ptr(),
            (*(*connptr).upstream_proxy).ua.authstr,
        )
    } else {
        return write_message(
            (*connptr).server_fd,
            b"%s %s HTTP/1.%u\r\nHost: %s%s\r\nConnection: close\r\n\0" as *const u8
                as *const libc::c_char,
            (*request).method,
            (*request).path,
            if (*connptr).protocol.major != 1 as libc::c_int as libc::c_uint {
                0 as libc::c_int as libc::c_uint
            } else {
                (*connptr).protocol.minor
            },
            (*request).host,
            portbuff.as_mut_ptr(),
        )
    };
}
unsafe extern "C" fn send_connect_method_response(
    mut connptr: *mut conn_s,
) -> libc::c_int {
    return write_message(
        (*connptr).client_fd,
        b"HTTP/1.%u 200 Connection established\r\nProxy-agent: tinyproxy/1.11.2\r\n\r\n\0"
            as *const u8 as *const libc::c_char,
        if (*connptr).protocol.major != 1 as libc::c_int as libc::c_uint {
            0 as libc::c_int as libc::c_uint
        } else {
            (*connptr).protocol.minor
        },
    );
}
unsafe extern "C" fn process_request(
    mut connptr: *mut conn_s,
    mut hashofheaders: orderedmap_0,
) -> *mut request_s {
    let mut current_block: u64;
    let mut url: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut request: *mut request_s = 0 as *mut request_s;
    let mut ret: libc::c_int = 0;
    let mut skip_trans: libc::c_int = 0;
    let mut request_len: size_t = 0;
    skip_trans = 0 as libc::c_int;
    request = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<request_s>() as libc::c_ulong,
    ) as *mut request_s;
    if request.is_null() {
        return 0 as *mut request_s;
    }
    request_len = (strlen((*connptr).request_line))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    (*request).method = malloc(request_len) as *mut libc::c_char;
    url = malloc(request_len) as *mut libc::c_char;
    (*request).protocol = malloc(request_len) as *mut libc::c_char;
    if !(((*request).method).is_null() || url.is_null()
        || ((*request).protocol).is_null())
    {
        let ref mut fresh3 = *((*request).protocol).offset(0 as libc::c_int as isize);
        *fresh3 = 0 as libc::c_int as libc::c_char;
        let ref mut fresh4 = *url.offset(0 as libc::c_int as isize);
        *fresh4 = *fresh3;
        *((*request).method).offset(0 as libc::c_int as isize) = *fresh4;
        ret = sscanf(
            (*connptr).request_line,
            b"%[^ ] %[^ ] %[^ ]\0" as *const u8 as *const libc::c_char,
            (*request).method,
            url,
            (*request).protocol,
        );
        if ret == 2 as libc::c_int
            && strcasecmp(
                (*request).method,
                b"GET\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            *((*request).protocol)
                .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
            (*connptr).protocol.major = 0 as libc::c_int as libc::c_uint;
            (*connptr).protocol.minor = 9 as libc::c_int as libc::c_uint;
            current_block = 6009453772311597924;
        } else {
            if ret == 3 as libc::c_int
                && strncasecmp(
                    (*request).protocol,
                    b"HTTP/\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as size_t,
                ) == 0
            {
                ret = sscanf(
                    ((*request).protocol).offset(5 as libc::c_int as isize),
                    b"%u.%u\0" as *const u8 as *const libc::c_char,
                    &mut (*connptr).protocol.major as *mut libc::c_uint,
                    &mut (*connptr).protocol.minor as *mut libc::c_uint,
                );
                if ret != 2 as libc::c_int {
                    current_block = 2388924797621951110;
                } else {
                    current_block = 6009453772311597924;
                }
            } else {
                current_block = 2388924797621951110;
            }
            match current_block {
                6009453772311597924 => {}
                _ => {
                    log_message(
                        3 as libc::c_int,
                        b"process_request: Bad Request on file descriptor %d\0"
                            as *const u8 as *const libc::c_char,
                        (*connptr).client_fd,
                    );
                    indicate_http_error(
                        connptr,
                        400 as libc::c_int,
                        b"Bad Request\0" as *const u8 as *const libc::c_char,
                        b"detail\0" as *const u8 as *const libc::c_char,
                        b"Request has an invalid format\0" as *const u8
                            as *const libc::c_char,
                        b"url\0" as *const u8 as *const libc::c_char,
                        url,
                        0 as *mut libc::c_void,
                    );
                    current_block = 3346813360592490146;
                }
            }
        }
        match current_block {
            3346813360592490146 => {}
            _ => {
                if !((*config).reversepath_list).is_null() {
                    let mut reverse_url: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut reverse_status: libc::c_int = 0;
                    reverse_url = reverse_rewrite_url(
                        connptr,
                        hashofheaders,
                        url,
                        &mut reverse_status,
                    );
                    if !reverse_url.is_null() {
                        if reverse_status == 301 as libc::c_int {
                            let mut buf: [libc::c_char; 4096] = [0; 4096];
                            snprintf(
                                buf.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 4096]>()
                                    as libc::c_ulong,
                                b"Location: %s\r\n\0" as *const u8 as *const libc::c_char,
                                reverse_url,
                            );
                            send_http_headers(
                                connptr,
                                301 as libc::c_int,
                                b"Moved Permanently\0" as *const u8 as *const libc::c_char,
                                buf.as_mut_ptr(),
                            );
                            current_block = 3346813360592490146;
                        } else {
                            free(url as *mut libc::c_void);
                            url = 0 as *mut libc::c_char;
                            url = reverse_url;
                            skip_trans = 1 as libc::c_int;
                            current_block = 5689316957504528238;
                        }
                    } else if (*config).reverseonly != 0 {
                        log_message(
                            3 as libc::c_int,
                            b"Bad request, no mapping for '%s' found\0" as *const u8
                                as *const libc::c_char,
                            url,
                        );
                        indicate_http_error(
                            connptr,
                            400 as libc::c_int,
                            b"Bad Request\0" as *const u8 as *const libc::c_char,
                            b"detail\0" as *const u8 as *const libc::c_char,
                            b"No mapping found for requested url\0" as *const u8
                                as *const libc::c_char,
                            b"url\0" as *const u8 as *const libc::c_char,
                            url,
                            0 as *mut libc::c_void,
                        );
                        current_block = 3346813360592490146;
                    } else {
                        current_block = 5689316957504528238;
                    }
                } else {
                    current_block = 5689316957504528238;
                }
                match current_block {
                    3346813360592490146 => {}
                    _ => {
                        if strncasecmp(
                            url,
                            b"http://\0" as *const u8 as *const libc::c_char,
                            7 as libc::c_int as size_t,
                        ) == 0 as libc::c_int
                            || !((*config).upstream_list).is_null()
                                && strncasecmp(
                                    url,
                                    b"ftp://\0" as *const u8 as *const libc::c_char,
                                    6 as libc::c_int as size_t,
                                ) == 0 as libc::c_int
                        {
                            let mut skipped_type: *mut libc::c_char = (strstr(
                                url,
                                b"//\0" as *const u8 as *const libc::c_char,
                            ))
                                .offset(2 as libc::c_int as isize);
                            if extract_url(skipped_type, 80 as libc::c_int, request)
                                < 0 as libc::c_int
                            {
                                indicate_http_error(
                                    connptr,
                                    400 as libc::c_int,
                                    b"Bad Request\0" as *const u8 as *const libc::c_char,
                                    b"detail\0" as *const u8 as *const libc::c_char,
                                    b"Could not parse URL\0" as *const u8
                                        as *const libc::c_char,
                                    b"url\0" as *const u8 as *const libc::c_char,
                                    url,
                                    0 as *mut libc::c_void,
                                );
                                current_block = 3346813360592490146;
                            } else {
                                current_block = 6717214610478484138;
                            }
                        } else if strcmp(
                            (*request).method,
                            b"CONNECT\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            if extract_url(url, 443 as libc::c_int, request)
                                < 0 as libc::c_int
                            {
                                indicate_http_error(
                                    connptr,
                                    400 as libc::c_int,
                                    b"Bad Request\0" as *const u8 as *const libc::c_char,
                                    b"detail\0" as *const u8 as *const libc::c_char,
                                    b"Could not parse URL\0" as *const u8
                                        as *const libc::c_char,
                                    b"url\0" as *const u8 as *const libc::c_char,
                                    url,
                                    0 as *mut libc::c_void,
                                );
                                current_block = 3346813360592490146;
                            } else if check_allowed_connect_ports(
                                (*request).port as libc::c_int,
                                (*config).connect_ports,
                            ) == 0
                            {
                                indicate_http_error(
                                    connptr,
                                    403 as libc::c_int,
                                    b"Access violation\0" as *const u8 as *const libc::c_char,
                                    b"detail\0" as *const u8 as *const libc::c_char,
                                    b"The CONNECT method not allowed with the port you tried to use.\0"
                                        as *const u8 as *const libc::c_char,
                                    b"url\0" as *const u8 as *const libc::c_char,
                                    url,
                                    0 as *mut libc::c_void,
                                );
                                log_message(
                                    6 as libc::c_int,
                                    b"Refused CONNECT method on port %d\0" as *const u8
                                        as *const libc::c_char,
                                    (*request).port as libc::c_int,
                                );
                                current_block = 3346813360592490146;
                            } else {
                                (*connptr)
                                    .connect_method = (0 as libc::c_int == 0) as libc::c_int
                                    as libc::c_uint;
                                current_block = 6717214610478484138;
                            }
                        } else if skip_trans == 0 {
                            if do_transparent_proxy(
                                connptr,
                                hashofheaders,
                                request,
                                config,
                                &mut url,
                            ) == 0
                            {
                                current_block = 3346813360592490146;
                            } else {
                                current_block = 6717214610478484138;
                            }
                        } else {
                            indicate_http_error(
                                connptr,
                                501 as libc::c_int,
                                b"Not Implemented\0" as *const u8 as *const libc::c_char,
                                b"detail\0" as *const u8 as *const libc::c_char,
                                b"Unknown method or unsupported protocol.\0" as *const u8
                                    as *const libc::c_char,
                                b"url\0" as *const u8 as *const libc::c_char,
                                url,
                                0 as *mut libc::c_void,
                            );
                            log_message(
                                6 as libc::c_int,
                                b"Unknown method (%s) or protocol (%s)\0" as *const u8
                                    as *const libc::c_char,
                                (*request).method,
                                url,
                            );
                            current_block = 3346813360592490146;
                        }
                        match current_block {
                            3346813360592490146 => {}
                            _ => {
                                if !((*config).filter).is_null() {
                                    let mut fu: libc::c_int = ((*config).filter_opts
                                        & FILTER_OPT_URL as libc::c_int as libc::c_uint)
                                        as libc::c_int;
                                    ret = filter_run(
                                        if fu != 0 { url } else { (*request).host },
                                    );
                                    if ret != 0 {
                                        update_stats(STAT_DENIED);
                                        log_message(
                                            5 as libc::c_int,
                                            b"Proxying refused on filtered %s \"%s\"\0" as *const u8
                                                as *const libc::c_char,
                                            if fu != 0 {
                                                b"url\0" as *const u8 as *const libc::c_char
                                            } else {
                                                b"domain\0" as *const u8 as *const libc::c_char
                                            },
                                            if fu != 0 { url } else { (*request).host },
                                        );
                                        indicate_http_error(
                                            connptr,
                                            403 as libc::c_int,
                                            b"Filtered\0" as *const u8 as *const libc::c_char,
                                            b"detail\0" as *const u8 as *const libc::c_char,
                                            b"The request you made has been filtered\0" as *const u8
                                                as *const libc::c_char,
                                            b"url\0" as *const u8 as *const libc::c_char,
                                            url,
                                            0 as *mut libc::c_void,
                                        );
                                        current_block = 3346813360592490146;
                                    } else {
                                        current_block = 9627623479216730126;
                                    }
                                } else {
                                    current_block = 9627623479216730126;
                                }
                                match current_block {
                                    3346813360592490146 => {}
                                    _ => {
                                        if !((*config).stathost).is_null()
                                            && strcmp((*config).stathost, (*request).host)
                                                == 0 as libc::c_int
                                        {
                                            log_message(
                                                5 as libc::c_int,
                                                b"Request for the stathost.\0" as *const u8
                                                    as *const libc::c_char,
                                            );
                                            (*connptr)
                                                .show_stats = (0 as libc::c_int == 0) as libc::c_int
                                                as libc::c_uint;
                                        } else {
                                            free(url as *mut libc::c_void);
                                            url = 0 as *mut libc::c_char;
                                            return request;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free(url as *mut libc::c_void);
    url = 0 as *mut libc::c_char;
    free_request_struct(request);
    return 0 as *mut request_s;
}
unsafe extern "C" fn pull_client_data(
    mut connptr: *mut conn_s,
    mut length: libc::c_long,
    mut iehack: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: ssize_t = 0;
    let mut ret: libc::c_int = 0;
    buffer = malloc(
        if ((1024 as libc::c_int * 96 as libc::c_int) as size_t)
            < length as libc::c_ulong
        {
            (1024 as libc::c_int * 96 as libc::c_int) as size_t
        } else {
            length as libc::c_ulong
        },
    ) as *mut libc::c_char;
    if buffer.is_null() {
        return -(1 as libc::c_int);
    }
    loop {
        len = safe_read(
            (*connptr).client_fd,
            buffer as *mut libc::c_void,
            if ((1024 as libc::c_int * 96 as libc::c_int) as size_t)
                < length as libc::c_ulong
            {
                (1024 as libc::c_int * 96 as libc::c_int) as size_t
            } else {
                length as libc::c_ulong
            },
        );
        if len <= 0 as libc::c_int as libc::c_long {
            current_block = 14688615916496267165;
            break;
        }
        if ((*connptr).error_variables).is_null() {
            if safe_write(
                (*connptr).server_fd,
                buffer as *const libc::c_void,
                len as size_t,
            ) < 0 as libc::c_int as libc::c_long
            {
                current_block = 14688615916496267165;
                break;
            }
        }
        length -= len;
        if !(length > 0 as libc::c_int as libc::c_long) {
            current_block = 3276175668257526147;
            break;
        }
    }
    match current_block {
        3276175668257526147 => {
            if iehack != 0 {
                ret = socket_nonblocking((*connptr).client_fd);
                if ret != 0 as libc::c_int {
                    log_message(
                        3 as libc::c_int,
                        b"Failed to set the client socket to non-blocking: %s\0"
                            as *const u8 as *const libc::c_char,
                        strerror(*__errno_location()),
                    );
                    current_block = 14688615916496267165;
                } else {
                    len = recv(
                        (*connptr).client_fd,
                        buffer as *mut libc::c_void,
                        2 as libc::c_int as size_t,
                        MSG_PEEK as libc::c_int,
                    );
                    ret = socket_blocking((*connptr).client_fd);
                    if ret != 0 as libc::c_int {
                        log_message(
                            3 as libc::c_int,
                            b"Failed to set the client socket to blocking: %s\0"
                                as *const u8 as *const libc::c_char,
                            strerror(*__errno_location()),
                        );
                        current_block = 14688615916496267165;
                    } else if len < 0 as libc::c_int as libc::c_long
                        && *__errno_location() != 11 as libc::c_int
                    {
                        current_block = 14688615916496267165;
                    } else {
                        if len == 2 as libc::c_int as libc::c_long
                            && (len == 1 as libc::c_int as libc::c_long
                                && *buffer.offset(0 as libc::c_int as isize) as libc::c_int
                                    == '\n' as i32
                                || len == 2 as libc::c_int as libc::c_long
                                    && *buffer.offset(0 as libc::c_int as isize) as libc::c_int
                                        == '\r' as i32
                                    && *buffer.offset(1 as libc::c_int as isize) as libc::c_int
                                        == '\n' as i32)
                        {
                            let mut bytes_read: ssize_t = 0;
                            bytes_read = read(
                                (*connptr).client_fd,
                                buffer as *mut libc::c_void,
                                2 as libc::c_int as size_t,
                            );
                            if bytes_read == -(1 as libc::c_int) as libc::c_long {
                                log_message(
                                    4 as libc::c_int,
                                    b"Could not read two bytes from POST message\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                        }
                        current_block = 14576567515993809846;
                    }
                }
            } else {
                current_block = 14576567515993809846;
            }
            match current_block {
                14688615916496267165 => {}
                _ => {
                    free(buffer as *mut libc::c_void);
                    buffer = 0 as *mut libc::c_char;
                    return 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    free(buffer as *mut libc::c_void);
    buffer = 0 as *mut libc::c_char;
    return -(1 as libc::c_int);
}
unsafe extern "C" fn pull_client_data_chunked(mut connptr: *mut conn_s) -> libc::c_int {
    let mut current_block: u64;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: ssize_t = 0;
    let mut chunklen: libc::c_long = 0;
    loop {
        if !buffer.is_null() {
            free(buffer as *mut libc::c_void);
            buffer = 0 as *mut libc::c_char;
        }
        len = readline((*connptr).client_fd, &mut buffer);
        if len <= 0 as libc::c_int as libc::c_long {
            current_block = 8529876984301436933;
            break;
        }
        if ((*connptr).error_variables).is_null() {
            if safe_write(
                (*connptr).server_fd,
                buffer as *const libc::c_void,
                len as size_t,
            ) < 0 as libc::c_int as libc::c_long
            {
                current_block = 8529876984301436933;
                break;
            }
        }
        chunklen = strtol(buffer, 0 as *mut *mut libc::c_char, 16 as libc::c_int);
        if pull_client_data(
            connptr,
            chunklen + 2 as libc::c_int as libc::c_long,
            0 as libc::c_int,
        ) < 0 as libc::c_int
        {
            current_block = 8529876984301436933;
            break;
        }
        if chunklen == 0 {
            current_block = 11650488183268122163;
            break;
        }
    }
    match current_block {
        8529876984301436933 => {
            free(buffer as *mut libc::c_void);
            buffer = 0 as *mut libc::c_char;
            return -(1 as libc::c_int);
        }
        _ => {
            free(buffer as *mut libc::c_void);
            buffer = 0 as *mut libc::c_char;
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn add_xtinyproxy_header(mut connptr: *mut conn_s) -> libc::c_int {
    return write_message(
        (*connptr).server_fd,
        b"X-Tinyproxy: %s\r\n\0" as *const u8 as *const libc::c_char,
        (*connptr).client_ip_addr,
    );
}
unsafe extern "C" fn add_header_to_connection(
    mut hashofheaders: orderedmap_0,
    mut header: *mut libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut sep: *mut libc::c_char = 0 as *mut libc::c_char;
    len = (len as libc::c_ulong).wrapping_sub(chomp(header, len) as libc::c_ulong)
        as size_t as size_t;
    sep = strchr(header, ':' as i32);
    if sep.is_null() {
        return 0 as libc::c_int;
    }
    while *sep as libc::c_int == ':' as i32 || *sep as libc::c_int == ' ' as i32
        || *sep as libc::c_int == '\t' as i32
    {
        let fresh5 = sep;
        sep = sep.offset(1);
        *fresh5 = '\0' as i32 as libc::c_char;
    }
    len = (len as libc::c_ulong)
        .wrapping_sub(
            (sep.offset_from(header) as libc::c_long - 1 as libc::c_int as libc::c_long)
                as libc::c_ulong,
        ) as size_t as size_t;
    return orderedmap_append(hashofheaders, header, sep);
}
unsafe extern "C" fn get_all_headers(
    mut fd: libc::c_int,
    mut hashofheaders: orderedmap_0,
) -> libc::c_int {
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut header: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut count: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut linelen: ssize_t = 0;
    let mut len: ssize_t = 0 as libc::c_int as ssize_t;
    let mut double_cgi: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    count = 0 as libc::c_int;
    while count < 10000 as libc::c_int {
        linelen = readline(fd, &mut line);
        if linelen <= 0 as libc::c_int as libc::c_long {
            free(header as *mut libc::c_void);
            header = 0 as *mut libc::c_char;
            free(line as *mut libc::c_void);
            line = 0 as *mut libc::c_char;
            return -(1 as libc::c_int);
        }
        if linelen == 1 as libc::c_int as libc::c_long
            && *line.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32
            || linelen == 2 as libc::c_int as libc::c_long
                && *line.offset(0 as libc::c_int as isize) as libc::c_int == '\r' as i32
                && *line.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32
            || !(linelen > 0 as libc::c_int as libc::c_long
                && (*line.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32
                    || *line.offset(0 as libc::c_int as isize) as libc::c_int
                        == '\t' as i32))
        {
            if double_cgi == 0 && len > 0 as libc::c_int as libc::c_long
                && add_header_to_connection(hashofheaders, header, len as size_t)
                    < 0 as libc::c_int
            {
                free(header as *mut libc::c_void);
                header = 0 as *mut libc::c_char;
                free(line as *mut libc::c_void);
                line = 0 as *mut libc::c_char;
                return -(1 as libc::c_int);
            }
            len = 0 as libc::c_int as ssize_t;
        }
        if linelen == 1 as libc::c_int as libc::c_long
            && *line.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32
            || linelen == 2 as libc::c_int as libc::c_long
                && *line.offset(0 as libc::c_int as isize) as libc::c_int == '\r' as i32
                && *line.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32
        {
            free(header as *mut libc::c_void);
            header = 0 as *mut libc::c_char;
            free(line as *mut libc::c_void);
            line = 0 as *mut libc::c_char;
            return 0 as libc::c_int;
        }
        if linelen >= 5 as libc::c_int as libc::c_long
            && strncasecmp(
                line,
                b"HTTP/\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as size_t,
            ) == 0 as libc::c_int
        {
            double_cgi = (0 as libc::c_int == 0) as libc::c_int as libc::c_uint;
        }
        tmp = realloc(header as *mut libc::c_void, (len + linelen) as libc::c_ulong)
            as *mut libc::c_char;
        if tmp.is_null() {
            free(header as *mut libc::c_void);
            header = 0 as *mut libc::c_char;
            free(line as *mut libc::c_void);
            line = 0 as *mut libc::c_char;
            return -(1 as libc::c_int);
        }
        header = tmp;
        memcpy(
            header.offset(len as isize) as *mut libc::c_void,
            line as *const libc::c_void,
            linelen as libc::c_ulong,
        );
        len += linelen;
        free(line as *mut libc::c_void);
        line = 0 as *mut libc::c_char;
        count += 1;
        count;
    }
    free(header as *mut libc::c_void);
    header = 0 as *mut libc::c_char;
    free(line as *mut libc::c_void);
    line = 0 as *mut libc::c_char;
    return -(1 as libc::c_int);
}
unsafe extern "C" fn remove_connection_headers(
    mut hashofheaders: orderedmap_0,
) -> libc::c_int {
    static mut headers: [*const libc::c_char; 2] = [
        b"connection\0" as *const u8 as *const libc::c_char,
        b"proxy-connection\0" as *const u8 as *const libc::c_char,
    ];
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: ssize_t = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut df: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i as libc::c_ulong
        != (::std::mem::size_of::<[*const libc::c_char; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        data = orderedmap_find(hashofheaders, headers[i as usize]);
        if data.is_null() {
            return 0 as libc::c_int;
        }
        len = strlen(data) as ssize_t;
        ptr = data;
        loop {
            ptr = strpbrk(
                ptr,
                b"()<>@,;:\\\"/[]?={} \t\0" as *const u8 as *const libc::c_char,
            );
            if ptr.is_null() {
                break;
            }
            let fresh6 = ptr;
            ptr = ptr.offset(1);
            *fresh6 = '\0' as i32 as libc::c_char;
        }
        ptr = data;
        while ptr < data.offset(len as isize) {
            df = 0 as libc::c_int;
            j = 0 as libc::c_int;
            while j as libc::c_ulong
                != (::std::mem::size_of::<[*const libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    )
            {
                if strcasecmp(ptr, headers[j as usize]) == 0 {
                    df = 1 as libc::c_int;
                }
                j += 1;
                j;
            }
            if df == 0 {
                orderedmap_remove(hashofheaders, ptr);
            }
            ptr = ptr
                .offset(
                    (strlen(ptr)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as isize,
                );
            while ptr < data.offset(len as isize) && *ptr as libc::c_int == '\0' as i32 {
                ptr = ptr.offset(1);
                ptr;
            }
        }
        orderedmap_remove(hashofheaders, headers[i as usize]);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_content_length(
    mut hashofheaders: orderedmap_0,
) -> libc::c_long {
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut content_length: libc::c_long = -(1 as libc::c_int) as libc::c_long;
    data = orderedmap_find(
        hashofheaders,
        b"content-length\0" as *const u8 as *const libc::c_char,
    );
    if !data.is_null() {
        content_length = atol(data);
    }
    return content_length;
}
unsafe extern "C" fn is_chunked_transfer(
    mut hashofheaders: orderedmap_0,
) -> libc::c_int {
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    data = orderedmap_find(
        hashofheaders,
        b"transfer-encoding\0" as *const u8 as *const libc::c_char,
    );
    return if !data.is_null() {
        (strcmp(data, b"chunked\0" as *const u8 as *const libc::c_char) == 0)
            as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn write_via_header(
    mut fd: libc::c_int,
    mut hashofheaders: orderedmap_0,
    mut major: libc::c_uint,
    mut minor: libc::c_uint,
) -> libc::c_int {
    let mut hostname: [libc::c_char; 512] = [0; 512];
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    if (*config).disable_viaheader != 0 {
        ret = 0 as libc::c_int;
    } else {
        if !((*config).via_proxy_name).is_null() {
            strlcpy(
                hostname.as_mut_ptr(),
                (*config).via_proxy_name,
                ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
            );
        } else if gethostname(
            hostname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
        ) < 0 as libc::c_int
        {
            strlcpy(
                hostname.as_mut_ptr(),
                b"unknown\0" as *const u8 as *const libc::c_char,
                512 as libc::c_int as size_t,
            );
        }
        data = orderedmap_find(
            hashofheaders,
            b"via\0" as *const u8 as *const libc::c_char,
        );
        if !data.is_null() {
            ret = write_message(
                fd,
                b"Via: %s, %hu.%hu %s (%s/%s)\r\n\0" as *const u8 as *const libc::c_char,
                data,
                major,
                minor,
                hostname.as_mut_ptr(),
                b"tinyproxy\0" as *const u8 as *const libc::c_char,
                b"1.11.2\0" as *const u8 as *const libc::c_char,
            );
            orderedmap_remove(
                hashofheaders,
                b"via\0" as *const u8 as *const libc::c_char,
            );
        } else {
            ret = write_message(
                fd,
                b"Via: %hu.%hu %s (%s/%s)\r\n\0" as *const u8 as *const libc::c_char,
                major,
                minor,
                hostname.as_mut_ptr(),
                b"tinyproxy\0" as *const u8 as *const libc::c_char,
                b"1.11.2\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    return ret;
}
unsafe extern "C" fn process_client_headers(
    mut connptr: *mut conn_s,
    mut hashofheaders: orderedmap_0,
) -> libc::c_int {
    let mut current_block: u64;
    static mut skipheaders: [*const libc::c_char; 6] = [
        b"host\0" as *const u8 as *const libc::c_char,
        b"keep-alive\0" as *const u8 as *const libc::c_char,
        b"proxy-connection\0" as *const u8 as *const libc::c_char,
        b"te\0" as *const u8 as *const libc::c_char,
        b"trailers\0" as *const u8 as *const libc::c_char,
        b"upgrade\0" as *const u8 as *const libc::c_char,
    ];
    let mut i: libc::c_int = 0;
    let mut iter: size_t = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut header: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*connptr).server_fd == -(1 as libc::c_int) || (*connptr).show_stats != 0
        || (*connptr).connect_method != 0
            && !(!((*connptr).upstream_proxy).is_null()
                && (*(*connptr).upstream_proxy).type_0 as libc::c_uint
                    == PT_HTTP as libc::c_int as libc::c_uint)
    {
        log_message(
            6 as libc::c_int,
            b"Not sending client headers to remote machine\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    (*connptr).content_length.client = get_content_length(hashofheaders);
    if (*connptr).content_length.client == -(1 as libc::c_int) as libc::c_long
        && is_chunked_transfer(hashofheaders) != 0
    {
        (*connptr).content_length.client = -(2 as libc::c_int) as libc::c_long;
    }
    remove_connection_headers(hashofheaders);
    i = 0 as libc::c_int;
    while i as libc::c_ulong
        != (::std::mem::size_of::<[*const libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        orderedmap_remove(hashofheaders, skipheaders[i as usize]);
        i += 1;
        i;
    }
    ret = write_via_header(
        (*connptr).server_fd,
        hashofheaders,
        (*connptr).protocol.major,
        (*connptr).protocol.minor,
    );
    if ret < 0 as libc::c_int {
        indicate_http_error(
            connptr,
            503 as libc::c_int,
            b"Could not send data to remote server\0" as *const u8
                as *const libc::c_char,
            b"detail\0" as *const u8 as *const libc::c_char,
            b"A network error occurred while trying to write data to the remote web server.\0"
                as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
    } else {
        iter = 0 as libc::c_int as size_t;
        loop {
            iter = orderedmap_next(hashofheaders, iter, &mut data, &mut header);
            if !(iter != 0) {
                current_block = 12124785117276362961;
                break;
            }
            if !(is_anonymous_enabled(config) == 0
                || anonymous_search(config, data) > 0 as libc::c_int)
            {
                continue;
            }
            ret = write_message(
                (*connptr).server_fd,
                b"%s: %s\r\n\0" as *const u8 as *const libc::c_char,
                data,
                header,
            );
            if !(ret < 0 as libc::c_int) {
                continue;
            }
            indicate_http_error(
                connptr,
                503 as libc::c_int,
                b"Could not send data to remote server\0" as *const u8
                    as *const libc::c_char,
                b"detail\0" as *const u8 as *const libc::c_char,
                b"A network error occurred while trying to write data to the remote web server.\0"
                    as *const u8 as *const libc::c_char,
                0 as *mut libc::c_void,
            );
            current_block = 17765375434977606156;
            break;
        }
        match current_block {
            17765375434977606156 => {}
            _ => {
                if (*config).add_xtinyproxy != 0 {
                    add_xtinyproxy_header(connptr);
                }
                if safe_write(
                    (*connptr).server_fd,
                    b"\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    2 as libc::c_int as size_t,
                ) < 0 as libc::c_int as libc::c_long
                {
                    return -(1 as libc::c_int);
                }
            }
        }
    }
    if (*connptr).content_length.client > 0 as libc::c_int as libc::c_long {
        ret = pull_client_data(
            connptr,
            (*connptr).content_length.client,
            1 as libc::c_int,
        );
    } else if (*connptr).content_length.client == -(2 as libc::c_int) as libc::c_long {
        ret = pull_client_data_chunked(connptr);
    }
    return ret;
}
unsafe extern "C" fn process_server_headers(mut connptr: *mut conn_s) -> libc::c_int {
    let mut current_block: u64;
    static mut skipheaders: [*const libc::c_char; 4] = [
        b"keep-alive\0" as *const u8 as *const libc::c_char,
        b"proxy-authenticate\0" as *const u8 as *const libc::c_char,
        b"proxy-authorization\0" as *const u8 as *const libc::c_char,
        b"proxy-connection\0" as *const u8 as *const libc::c_char,
    ];
    let mut response_line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hashofheaders: orderedmap_0 = 0 as *mut orderedmap;
    let mut iter: size_t = 0;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut header: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: ssize_t = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut reverse: *mut reversepath = (*config).reversepath_list;
    loop {
        len = readline((*connptr).server_fd, &mut response_line);
        if len <= 0 as libc::c_int as libc::c_long {
            return -(1 as libc::c_int);
        }
        if !(chomp(response_line, len as size_t) == len) {
            break;
        }
        free(response_line as *mut libc::c_void);
        response_line = 0 as *mut libc::c_char;
    }
    hashofheaders = orderedmap_create(32 as libc::c_int as size_t);
    if hashofheaders.is_null() {
        free(response_line as *mut libc::c_void);
        response_line = 0 as *mut libc::c_char;
        return -(1 as libc::c_int);
    }
    if get_all_headers((*connptr).server_fd, hashofheaders) < 0 as libc::c_int {
        log_message(
            4 as libc::c_int,
            b"Could not retrieve all the headers from the remote server.\0" as *const u8
                as *const libc::c_char,
        );
        orderedmap_destroy(hashofheaders);
        free(response_line as *mut libc::c_void);
        response_line = 0 as *mut libc::c_char;
        indicate_http_error(
            connptr,
            503 as libc::c_int,
            b"Could not retrieve all the headers\0" as *const u8 as *const libc::c_char,
            b"detail\0" as *const u8 as *const libc::c_char,
            b"Tinyproxy was unable to retrieve and process headers from the remote web server.\0"
                as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        return -(1 as libc::c_int);
    }
    if (*connptr).protocol.major < 1 as libc::c_int as libc::c_uint {
        orderedmap_destroy(hashofheaders);
        free(response_line as *mut libc::c_void);
        response_line = 0 as *mut libc::c_char;
        return 0 as libc::c_int;
    }
    ret = write_message(
        (*connptr).client_fd,
        b"%s\r\n\0" as *const u8 as *const libc::c_char,
        response_line,
    );
    free(response_line as *mut libc::c_void);
    response_line = 0 as *mut libc::c_char;
    if !(ret < 0 as libc::c_int) {
        (*connptr).content_length.server = get_content_length(hashofheaders);
        remove_connection_headers(hashofheaders);
        i = 0 as libc::c_int;
        while i as libc::c_ulong
            != (::std::mem::size_of::<[*const libc::c_char; 4]>() as libc::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                )
        {
            orderedmap_remove(hashofheaders, skipheaders[i as usize]);
            i += 1;
            i;
        }
        ret = write_via_header(
            (*connptr).client_fd,
            hashofheaders,
            (*connptr).protocol.major,
            (*connptr).protocol.minor,
        );
        if !(ret < 0 as libc::c_int) {
            if (*config).reversemagic != 0 && !((*connptr).reversepath).is_null() {
                ret = write_message(
                    (*connptr).client_fd,
                    b"Set-Cookie: yummy_magical_cookie=%s; path=/\r\n\0" as *const u8
                        as *const libc::c_char,
                    (*connptr).reversepath,
                );
                if ret < 0 as libc::c_int {
                    current_block = 3840173124024789963;
                } else {
                    current_block = 11932355480408055363;
                }
            } else {
                current_block = 11932355480408055363;
            }
            match current_block {
                3840173124024789963 => {}
                _ => {
                    if !((*config).reversebaseurl).is_null()
                        && {
                            header = orderedmap_find(
                                hashofheaders,
                                b"location\0" as *const u8 as *const libc::c_char,
                            );
                            !header.is_null()
                        }
                    {
                        while !reverse.is_null() {
                            len = strlen((*reverse).url) as ssize_t;
                            if strncasecmp(header, (*reverse).url, len as size_t)
                                == 0 as libc::c_int
                            {
                                break;
                            }
                            reverse = (*reverse).next;
                        }
                        if !reverse.is_null() {
                            ret = write_message(
                                (*connptr).client_fd,
                                b"Location: %s%s%s\r\n\0" as *const u8
                                    as *const libc::c_char,
                                (*config).reversebaseurl,
                                ((*reverse).path).offset(1 as libc::c_int as isize),
                                header.offset(len as isize),
                            );
                            if ret < 0 as libc::c_int {
                                current_block = 3840173124024789963;
                            } else {
                                log_message(
                                    6 as libc::c_int,
                                    b"Rewriting HTTP redirect: %s -> %s%s%s\0" as *const u8
                                        as *const libc::c_char,
                                    header,
                                    (*config).reversebaseurl,
                                    ((*reverse).path).offset(1 as libc::c_int as isize),
                                    header.offset(len as isize),
                                );
                                orderedmap_remove(
                                    hashofheaders,
                                    b"location\0" as *const u8 as *const libc::c_char,
                                );
                                current_block = 13131896068329595644;
                            }
                        } else {
                            current_block = 13131896068329595644;
                        }
                    } else {
                        current_block = 13131896068329595644;
                    }
                    match current_block {
                        3840173124024789963 => {}
                        _ => {
                            iter = 0 as libc::c_int as size_t;
                            loop {
                                iter = orderedmap_next(
                                    hashofheaders,
                                    iter,
                                    &mut data,
                                    &mut header,
                                );
                                if !(iter != 0) {
                                    current_block = 2480299350034459858;
                                    break;
                                }
                                ret = write_message(
                                    (*connptr).client_fd,
                                    b"%s: %s\r\n\0" as *const u8 as *const libc::c_char,
                                    data,
                                    header,
                                );
                                if ret < 0 as libc::c_int {
                                    current_block = 3840173124024789963;
                                    break;
                                }
                            }
                            match current_block {
                                3840173124024789963 => {}
                                _ => {
                                    orderedmap_destroy(hashofheaders);
                                    if safe_write(
                                        (*connptr).client_fd,
                                        b"\r\n\0" as *const u8 as *const libc::c_char
                                            as *const libc::c_void,
                                        2 as libc::c_int as size_t,
                                    ) < 0 as libc::c_int as libc::c_long
                                    {
                                        return -(1 as libc::c_int);
                                    }
                                    return 0 as libc::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    orderedmap_destroy(hashofheaders);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn relay_connection(mut connptr: *mut conn_s) {
    let mut ret: libc::c_int = 0;
    let mut bytes_received: ssize_t = 0;
    loop {
        let mut fds: [pollfd_struct; 2] = [
            {
                let mut init = pollfd {
                    fd: 0 as libc::c_int,
                    events: 0,
                    revents: 0,
                };
                init
            },
            pollfd_struct {
                fd: 0,
                events: 0,
                revents: 0,
            },
        ];
        fds[0 as libc::c_int as usize].fd = (*connptr).client_fd;
        fds[1 as libc::c_int as usize].fd = (*connptr).server_fd;
        if buffer_size((*connptr).sbuffer) > 0 as libc::c_int as libc::c_ulong {
            fds[0 as libc::c_int as usize]
                .events = (fds[0 as libc::c_int as usize].events as libc::c_int
                | 0x4 as libc::c_int) as libc::c_short;
        }
        if buffer_size((*connptr).cbuffer) > 0 as libc::c_int as libc::c_ulong {
            fds[1 as libc::c_int as usize]
                .events = (fds[1 as libc::c_int as usize].events as libc::c_int
                | 0x4 as libc::c_int) as libc::c_short;
        }
        if buffer_size((*connptr).sbuffer)
            < (1024 as libc::c_int * 96 as libc::c_int) as size_t
        {
            fds[1 as libc::c_int as usize]
                .events = (fds[1 as libc::c_int as usize].events as libc::c_int
                | 0x1 as libc::c_int) as libc::c_short;
        }
        if buffer_size((*connptr).cbuffer)
            < (1024 as libc::c_int * 96 as libc::c_int) as size_t
        {
            fds[0 as libc::c_int as usize]
                .events = (fds[0 as libc::c_int as usize].events as libc::c_int
                | 0x1 as libc::c_int) as libc::c_short;
        }
        ret = mypoll(
            fds.as_mut_ptr(),
            2 as libc::c_int,
            (*config).idletimeout as libc::c_int,
        );
        if ret == 0 as libc::c_int {
            log_message(
                6 as libc::c_int,
                b"Idle Timeout (after poll)\0" as *const u8 as *const libc::c_char,
            );
            return;
        } else if ret < 0 as libc::c_int {
            log_message(
                3 as libc::c_int,
                b"relay_connection: poll() error \"%s\". Closing connection (client_fd:%d, server_fd:%d)\0"
                    as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
                (*connptr).client_fd,
                (*connptr).server_fd,
            );
            return;
        }
        if fds[1 as libc::c_int as usize].revents as libc::c_int & 0x1 as libc::c_int
            != 0
        {
            bytes_received = read_buffer((*connptr).server_fd, (*connptr).sbuffer);
            if bytes_received < 0 as libc::c_int as libc::c_long {
                break;
            }
            (*connptr).content_length.server -= bytes_received;
            if (*connptr).content_length.server == 0 as libc::c_int as libc::c_long {
                break;
            }
        }
        if fds[0 as libc::c_int as usize].revents as libc::c_int & 0x1 as libc::c_int
            != 0
            && read_buffer((*connptr).client_fd, (*connptr).cbuffer)
                < 0 as libc::c_int as libc::c_long
        {
            break;
        }
        if fds[1 as libc::c_int as usize].revents as libc::c_int & 0x4 as libc::c_int
            != 0
            && write_buffer((*connptr).server_fd, (*connptr).cbuffer)
                < 0 as libc::c_int as libc::c_long
        {
            break;
        }
        if fds[0 as libc::c_int as usize].revents as libc::c_int & 0x4 as libc::c_int
            != 0
            && write_buffer((*connptr).client_fd, (*connptr).sbuffer)
                < 0 as libc::c_int as libc::c_long
        {
            break;
        }
    }
    while buffer_size((*connptr).sbuffer) > 0 as libc::c_int as libc::c_ulong {
        if write_buffer((*connptr).client_fd, (*connptr).sbuffer)
            < 0 as libc::c_int as libc::c_long
        {
            break;
        }
    }
    shutdown((*connptr).client_fd, SHUT_WR as libc::c_int);
    ret = socket_blocking((*connptr).server_fd);
    if ret != 0 as libc::c_int {
        log_message(
            3 as libc::c_int,
            b"Failed to set server socket to blocking: %s\0" as *const u8
                as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return;
    }
    while buffer_size((*connptr).cbuffer) > 0 as libc::c_int as libc::c_ulong {
        if write_buffer((*connptr).server_fd, (*connptr).cbuffer)
            < 0 as libc::c_int as libc::c_long
        {
            break;
        }
    }
}
unsafe extern "C" fn connect_to_upstream_proxy(
    mut connptr: *mut conn_s,
    mut request: *mut request_s,
) -> libc::c_int {
    let mut len: libc::c_uint = 0;
    let mut buff: [libc::c_uchar; 512] = [0; 512];
    let mut port: libc::c_ushort = 0;
    let mut ulen: size_t = 0;
    let mut passlen: size_t = 0;
    let mut cur_upstream: *mut upstream = (*connptr).upstream_proxy;
    ulen = if !((*cur_upstream).ua.user).is_null() {
        strlen((*cur_upstream).ua.user)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    passlen = if !((*cur_upstream).pass).is_null() {
        strlen((*cur_upstream).pass)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    log_message(
        8 as libc::c_int,
        b"Established connection to %s proxy \"%s\" using file descriptor %d.\0"
            as *const u8 as *const libc::c_char,
        proxy_type_name((*cur_upstream).type_0),
        (*cur_upstream).host,
        (*connptr).server_fd,
    );
    if (*cur_upstream).type_0 as libc::c_uint == PT_SOCKS4 as libc::c_int as libc::c_uint
    {
        buff[0 as libc::c_int as usize] = 4 as libc::c_int as libc::c_uchar;
        buff[1 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
        port = htons((*request).port);
        memcpy(
            &mut *buff.as_mut_ptr().offset(2 as libc::c_int as isize)
                as *mut libc::c_uchar as *mut libc::c_void,
            &mut port as *mut libc::c_ushort as *const libc::c_void,
            2 as libc::c_int as libc::c_ulong,
        );
        memcpy(
            &mut *buff.as_mut_ptr().offset(4 as libc::c_int as isize)
                as *mut libc::c_uchar as *mut libc::c_void,
            b"\0\0\0\x01\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            5 as libc::c_int as libc::c_ulong,
        );
        len = strlen((*request).host) as libc::c_uint;
        if len > 255 as libc::c_int as libc::c_uint {
            return -(1 as libc::c_int);
        }
        memcpy(
            &mut *buff.as_mut_ptr().offset(9 as libc::c_int as isize)
                as *mut libc::c_uchar as *mut libc::c_void,
            (*request).host as *const libc::c_void,
            len.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
        );
        if (9 as libc::c_int as libc::c_uint)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_long
            != safe_write(
                (*connptr).server_fd,
                buff.as_mut_ptr() as *const libc::c_void,
                (9 as libc::c_int as libc::c_uint)
                    .wrapping_add(len)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
            )
        {
            return -(1 as libc::c_int);
        }
        if 8 as libc::c_int as libc::c_long
            != safe_read(
                (*connptr).server_fd,
                buff.as_mut_ptr() as *mut libc::c_void,
                8 as libc::c_int as size_t,
            )
        {
            return -(1 as libc::c_int);
        }
        if buff[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
            || buff[1 as libc::c_int as usize] as libc::c_int != 90 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
    } else if (*cur_upstream).type_0 as libc::c_uint
        == PT_SOCKS5 as libc::c_int as libc::c_uint
    {
        let mut n_methods: libc::c_int = if ulen != 0 {
            2 as libc::c_int
        } else {
            1 as libc::c_int
        };
        buff[0 as libc::c_int as usize] = 5 as libc::c_int as libc::c_uchar;
        buff[1 as libc::c_int as usize] = n_methods as libc::c_uchar;
        buff[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
        if ulen != 0 {
            buff[3 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
        }
        if (2 as libc::c_int + n_methods) as libc::c_long
            != safe_write(
                (*connptr).server_fd,
                buff.as_mut_ptr() as *const libc::c_void,
                (2 as libc::c_int + n_methods) as size_t,
            )
        {
            return -(1 as libc::c_int);
        }
        if 2 as libc::c_int as libc::c_long
            != safe_read(
                (*connptr).server_fd,
                buff.as_mut_ptr() as *mut libc::c_void,
                2 as libc::c_int as size_t,
            )
        {
            return -(1 as libc::c_int);
        }
        if buff[0 as libc::c_int as usize] as libc::c_int != 5 as libc::c_int
            || buff[1 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
                && buff[1 as libc::c_int as usize] as libc::c_int != 2 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        if buff[1 as libc::c_int as usize] as libc::c_int == 2 as libc::c_int {
            let mut in_0: [libc::c_char; 2] = [0; 2];
            let mut out: [libc::c_char; 515] = [0; 515];
            let mut cur: *mut libc::c_char = out.as_mut_ptr();
            let mut c: size_t = 0;
            let fresh7 = cur;
            cur = cur.offset(1);
            *fresh7 = 1 as libc::c_int as libc::c_char;
            c = ulen & 0xff as libc::c_int as libc::c_ulong;
            let fresh8 = cur;
            cur = cur.offset(1);
            *fresh8 = c as libc::c_char;
            memcpy(
                cur as *mut libc::c_void,
                (*cur_upstream).ua.user as *const libc::c_void,
                c,
            );
            cur = cur.offset(c as isize);
            c = passlen & 0xff as libc::c_int as libc::c_ulong;
            let fresh9 = cur;
            cur = cur.offset(1);
            *fresh9 = c as libc::c_char;
            memcpy(
                cur as *mut libc::c_void,
                (*cur_upstream).pass as *const libc::c_void,
                c,
            );
            cur = cur.offset(c as isize);
            if cur.offset_from(out.as_mut_ptr()) as libc::c_long
                != safe_write(
                    (*connptr).server_fd,
                    out.as_mut_ptr() as *const libc::c_void,
                    cur.offset_from(out.as_mut_ptr()) as libc::c_long as size_t,
                )
            {
                return -(1 as libc::c_int);
            }
            if 2 as libc::c_int as libc::c_long
                != safe_read(
                    (*connptr).server_fd,
                    in_0.as_mut_ptr() as *mut libc::c_void,
                    2 as libc::c_int as size_t,
                )
            {
                return -(1 as libc::c_int);
            }
            if in_0[1 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
                || !(in_0[0 as libc::c_int as usize] as libc::c_int == 5 as libc::c_int
                    || in_0[0 as libc::c_int as usize] as libc::c_int
                        == 1 as libc::c_int)
            {
                return -(1 as libc::c_int);
            }
        }
        buff[0 as libc::c_int as usize] = 5 as libc::c_int as libc::c_uchar;
        buff[1 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
        buff[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
        buff[3 as libc::c_int as usize] = 3 as libc::c_int as libc::c_uchar;
        len = strlen((*request).host) as libc::c_uint;
        if len > 255 as libc::c_int as libc::c_uint {
            return -(1 as libc::c_int);
        }
        buff[4 as libc::c_int as usize] = len as libc::c_uchar;
        memcpy(
            &mut *buff.as_mut_ptr().offset(5 as libc::c_int as isize)
                as *mut libc::c_uchar as *mut libc::c_void,
            (*request).host as *const libc::c_void,
            len as libc::c_ulong,
        );
        port = htons((*request).port);
        memcpy(
            &mut *buff
                .as_mut_ptr()
                .offset((5 as libc::c_int as libc::c_uint).wrapping_add(len) as isize)
                as *mut libc::c_uchar as *mut libc::c_void,
            &mut port as *mut libc::c_ushort as *const libc::c_void,
            2 as libc::c_int as libc::c_ulong,
        );
        if (7 as libc::c_int as libc::c_uint).wrapping_add(len) as libc::c_long
            != safe_write(
                (*connptr).server_fd,
                buff.as_mut_ptr() as *const libc::c_void,
                (7 as libc::c_int as libc::c_uint).wrapping_add(len) as size_t,
            )
        {
            return -(1 as libc::c_int);
        }
        if 4 as libc::c_int as libc::c_long
            != safe_read(
                (*connptr).server_fd,
                buff.as_mut_ptr() as *mut libc::c_void,
                4 as libc::c_int as size_t,
            )
        {
            return -(1 as libc::c_int);
        }
        if buff[0 as libc::c_int as usize] as libc::c_int != 5 as libc::c_int
            || buff[1 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        match buff[3 as libc::c_int as usize] as libc::c_int {
            1 => {
                len = 4 as libc::c_int as libc::c_uint;
            }
            4 => {
                len = 16 as libc::c_int as libc::c_uint;
            }
            3 => {
                if 1 as libc::c_int as libc::c_long
                    != safe_read(
                        (*connptr).server_fd,
                        buff.as_mut_ptr() as *mut libc::c_void,
                        1 as libc::c_int as size_t,
                    )
                {
                    return -(1 as libc::c_int);
                }
                len = buff[0 as libc::c_int as usize] as libc::c_uint;
            }
            _ => return -(1 as libc::c_int),
        }
        if (2 as libc::c_int as libc::c_uint).wrapping_add(len) as libc::c_long
            != safe_read(
                (*connptr).server_fd,
                buff.as_mut_ptr() as *mut libc::c_void,
                (2 as libc::c_int as libc::c_uint).wrapping_add(len) as size_t,
            )
        {
            return -(1 as libc::c_int);
        }
    } else {
        return -(1 as libc::c_int)
    }
    if (*connptr).connect_method != 0 {
        return 0 as libc::c_int;
    }
    return establish_http_connection(connptr, request);
}
unsafe extern "C" fn connect_to_upstream(
    mut connptr: *mut conn_s,
    mut request: *mut request_s,
) -> libc::c_int {
    let mut combined_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut cur_upstream: *mut upstream = (*connptr).upstream_proxy;
    if cur_upstream.is_null() {
        log_message(
            4 as libc::c_int,
            b"No upstream proxy defined for %s.\0" as *const u8 as *const libc::c_char,
            (*request).host,
        );
        indicate_http_error(
            connptr,
            502 as libc::c_int,
            b"Unable to connect to upstream proxy.\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*connptr)
        .server_fd = opensock(
        (*cur_upstream).host,
        (*cur_upstream).port,
        (*connptr).server_ip_addr,
    );
    if (*connptr).server_fd < 0 as libc::c_int {
        log_message(
            4 as libc::c_int,
            b"Could not connect to upstream proxy.\0" as *const u8 as *const libc::c_char,
        );
        indicate_http_error(
            connptr,
            502 as libc::c_int,
            b"Unable to connect to upstream proxy\0" as *const u8 as *const libc::c_char,
            b"detail\0" as *const u8 as *const libc::c_char,
            b"A network error occurred while trying to connect to the upstream web proxy.\0"
                as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        return -(1 as libc::c_int);
    }
    if (*cur_upstream).type_0 as libc::c_uint != PT_HTTP as libc::c_int as libc::c_uint {
        return connect_to_upstream_proxy(connptr, request);
    }
    log_message(
        8 as libc::c_int,
        b"Established connection to upstream proxy \"%s\" using file descriptor %d.\0"
            as *const u8 as *const libc::c_char,
        (*cur_upstream).host,
        (*connptr).server_fd,
    );
    if (*connptr).connect_method != 0 {
        len = (strlen((*request).host)).wrapping_add(7 as libc::c_int as libc::c_ulong)
            as libc::c_int;
        combined_string = malloc(len as libc::c_ulong) as *mut libc::c_char;
        if combined_string.is_null() {
            return -(1 as libc::c_int);
        }
        snprintf(
            combined_string,
            len as libc::c_ulong,
            b"%s:%d\0" as *const u8 as *const libc::c_char,
            (*request).host,
            (*request).port as libc::c_int,
        );
    } else {
        len = (strlen((*request).host))
            .wrapping_add(strlen((*request).path))
            .wrapping_add(14 as libc::c_int as libc::c_ulong) as libc::c_int;
        combined_string = malloc(len as libc::c_ulong) as *mut libc::c_char;
        if combined_string.is_null() {
            return -(1 as libc::c_int);
        }
        snprintf(
            combined_string,
            len as libc::c_ulong,
            b"http://%s:%d%s\0" as *const u8 as *const libc::c_char,
            (*request).host,
            (*request).port as libc::c_int,
            (*request).path,
        );
    }
    if !((*request).path).is_null() {
        free((*request).path as *mut libc::c_void);
        (*request).path = 0 as *mut libc::c_char;
    }
    (*request).path = combined_string;
    return establish_http_connection(connptr, request);
}
unsafe extern "C" fn get_request_entity(mut connptr: *mut conn_s) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut fds: [pollfd_struct; 1] = [
        {
            let mut init = pollfd {
                fd: 0 as libc::c_int,
                events: 0,
                revents: 0,
            };
            init
        },
    ];
    fds[0 as libc::c_int as usize].fd = (*connptr).client_fd;
    fds[0 as libc::c_int as usize]
        .events = (fds[0 as libc::c_int as usize].events as libc::c_int
        | 0x1 as libc::c_int) as libc::c_short;
    ret = mypoll(
        fds.as_mut_ptr(),
        1 as libc::c_int,
        (*config).idletimeout as libc::c_int,
    );
    if ret == -(1 as libc::c_int) {
        log_message(
            3 as libc::c_int,
            b"Error calling poll on client fd %d: %s\0" as *const u8
                as *const libc::c_char,
            (*connptr).client_fd,
            strerror(*__errno_location()),
        );
    } else if ret == 0 as libc::c_int {
        log_message(
            6 as libc::c_int,
            b"no entity\0" as *const u8 as *const libc::c_char,
        );
    } else if ret == 1 as libc::c_int
        && fds[0 as libc::c_int as usize].revents as libc::c_int & 0x1 as libc::c_int
            != 0
    {
        let mut nread: ssize_t = 0;
        nread = read_buffer((*connptr).client_fd, (*connptr).cbuffer);
        if nread < 0 as libc::c_int as libc::c_long {
            log_message(
                3 as libc::c_int,
                b"Error reading readable client_fd %d (%s)\0" as *const u8
                    as *const libc::c_char,
                (*connptr).client_fd,
                strerror(*__errno_location()),
            );
            ret = -(1 as libc::c_int);
        } else {
            log_message(
                6 as libc::c_int,
                b"Read request entity of %ld bytes\0" as *const u8
                    as *const libc::c_char,
                nread,
            );
            ret = 0 as libc::c_int;
        }
    } else {
        log_message(
            3 as libc::c_int,
            b"strange situation after poll: ret = %d, but client_fd (%d) is not readable...\0"
                as *const u8 as *const libc::c_char,
            ret,
            (*connptr).client_fd,
        );
        ret = -(1 as libc::c_int);
    }
    return ret;
}
unsafe extern "C" fn handle_connection_failure(
    mut connptr: *mut conn_s,
    mut got_headers: libc::c_int,
) {
    if got_headers == 0 && get_request_entity(connptr) < 0 as libc::c_int {
        log_message(
            4 as libc::c_int,
            b"Could not retrieve request entity\0" as *const u8 as *const libc::c_char,
        );
        indicate_http_error(
            connptr,
            400 as libc::c_int,
            b"Bad Request\0" as *const u8 as *const libc::c_char,
            b"detail\0" as *const u8 as *const libc::c_char,
            b"Could not retrieve the request entity the client.\0" as *const u8
                as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        update_stats(STAT_BADCONN);
    }
    if !((*connptr).error_variables).is_null() {
        send_http_error_message(connptr);
    } else if (*connptr).show_stats != 0 {
        showstats(connptr);
    }
}
pub unsafe extern "C" fn handle_connection(
    mut connptr: *mut conn_s,
    mut addr: *mut sockaddr_union,
) {
    let mut current_block: u64;
    let mut got_headers: libc::c_int = 0 as libc::c_int;
    let mut fd: libc::c_int = (*connptr).client_fd;
    let mut i: size_t = 0;
    let mut request: *mut request_s = 0 as *mut request_s;
    let mut hashofheaders: orderedmap_0 = 0 as orderedmap_0;
    let mut sock_ipaddr: [libc::c_char; 48] = [0; 48];
    let mut peer_ipaddr: [libc::c_char; 48] = [0; 48];
    getpeer_information(
        addr,
        peer_ipaddr.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong,
    );
    if (*config).bindsame != 0 {
        getsock_ip(fd, sock_ipaddr.as_mut_ptr());
    }
    log_message(
        8 as libc::c_int,
        if (*config).bindsame != 0 {
            b"Connect (file descriptor %d): %s at [%s]\0" as *const u8
                as *const libc::c_char
        } else {
            b"Connect (file descriptor %d): %s\0" as *const u8 as *const libc::c_char
        },
        fd,
        peer_ipaddr.as_mut_ptr(),
        sock_ipaddr.as_mut_ptr(),
    );
    if conn_init_contents(
        connptr,
        peer_ipaddr.as_mut_ptr(),
        if (*config).bindsame != 0 {
            sock_ipaddr.as_mut_ptr()
        } else {
            0 as *mut libc::c_char
        },
    ) == 0
    {
        close(fd);
        return;
    }
    set_socket_timeout(fd);
    if connection_loops(addr) != 0 {
        log_message(
            8 as libc::c_int,
            b"Prevented endless loop (file descriptor %d): %s\0" as *const u8
                as *const libc::c_char,
            fd,
            peer_ipaddr.as_mut_ptr(),
        );
        indicate_http_error(
            connptr,
            400 as libc::c_int,
            b"Bad Request\0" as *const u8 as *const libc::c_char,
            b"detail\0" as *const u8 as *const libc::c_char,
            b"You tried to connect to the machine the proxy is running on\0" as *const u8
                as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        handle_connection_failure(connptr, got_headers);
    } else if check_acl(peer_ipaddr.as_mut_ptr(), addr, (*config).access_list)
        <= 0 as libc::c_int
    {
        update_stats(STAT_DENIED);
        indicate_http_error(
            connptr,
            403 as libc::c_int,
            b"Access denied\0" as *const u8 as *const libc::c_char,
            b"detail\0" as *const u8 as *const libc::c_char,
            b"The administrator of this proxy has not configured it to service requests from your host.\0"
                as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
        );
        handle_connection_failure(connptr, got_headers);
    } else if read_request_line(connptr) < 0 as libc::c_int {
        update_stats(STAT_BADCONN);
    } else {
        hashofheaders = orderedmap_create(32 as libc::c_int as size_t);
        if hashofheaders.is_null() {
            update_stats(STAT_BADCONN);
            indicate_http_error(
                connptr,
                503 as libc::c_int,
                b"Internal error\0" as *const u8 as *const libc::c_char,
                b"detail\0" as *const u8 as *const libc::c_char,
                b"An internal server error occurred while processing your request. Please contact the administrator.\0"
                    as *const u8 as *const libc::c_char,
                0 as *mut libc::c_void,
            );
            handle_connection_failure(connptr, got_headers);
        } else if get_all_headers((*connptr).client_fd, hashofheaders) < 0 as libc::c_int
        {
            log_message(
                4 as libc::c_int,
                b"Could not retrieve all the headers from the client\0" as *const u8
                    as *const libc::c_char,
            );
            indicate_http_error(
                connptr,
                400 as libc::c_int,
                b"Bad Request\0" as *const u8 as *const libc::c_char,
                b"detail\0" as *const u8 as *const libc::c_char,
                b"Could not retrieve all the headers from the client.\0" as *const u8
                    as *const libc::c_char,
                0 as *mut libc::c_void,
            );
            update_stats(STAT_BADCONN);
            handle_connection_failure(connptr, got_headers);
        } else {
            got_headers = 1 as libc::c_int;
            if !((*config).basicauth_list).is_null() {
                let mut authstring: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut failure: libc::c_int = 1 as libc::c_int;
                let mut stathost_connect: libc::c_int = 0 as libc::c_int;
                authstring = orderedmap_find(
                    hashofheaders,
                    b"proxy-authorization\0" as *const u8 as *const libc::c_char,
                );
                if authstring.is_null() && !((*config).stathost).is_null() {
                    authstring = orderedmap_find(
                        hashofheaders,
                        b"host\0" as *const u8 as *const libc::c_char,
                    );
                    if !authstring.is_null()
                        && strncmp(
                            authstring,
                            (*config).stathost,
                            strlen((*config).stathost),
                        ) == 0
                    {
                        authstring = orderedmap_find(
                            hashofheaders,
                            b"authorization\0" as *const u8 as *const libc::c_char,
                        );
                        stathost_connect = 1 as libc::c_int;
                    } else {
                        authstring = 0 as *mut libc::c_char;
                    }
                }
                if authstring.is_null() {
                    if stathost_connect != 0 {
                        current_block = 14275407423676000822;
                    } else {
                        update_stats(STAT_DENIED);
                        indicate_http_error(
                            connptr,
                            407 as libc::c_int,
                            b"Proxy Authentication Required\0" as *const u8
                                as *const libc::c_char,
                            b"detail\0" as *const u8 as *const libc::c_char,
                            b"This proxy requires authentication.\0" as *const u8
                                as *const libc::c_char,
                            0 as *mut libc::c_void,
                        );
                        handle_connection_failure(connptr, got_headers);
                        current_block = 2650907734299715635;
                    }
                } else {
                    if (strncmp(
                        authstring,
                        b"Basic \0" as *const u8 as *const libc::c_char,
                        6 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                        || strncmp(
                            authstring,
                            b"basic \0" as *const u8 as *const libc::c_char,
                            6 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int)
                        && basicauth_check(
                            (*config).basicauth_list,
                            authstring.offset(6 as libc::c_int as isize),
                        ) == 1 as libc::c_int
                    {
                        failure = 0 as libc::c_int;
                    }
                    if failure != 0 {
                        current_block = 14275407423676000822;
                    } else {
                        orderedmap_remove(
                            hashofheaders,
                            b"proxy-authorization\0" as *const u8 as *const libc::c_char,
                        );
                        current_block = 13281731871476506071;
                    }
                }
                match current_block {
                    13281731871476506071 => {}
                    2650907734299715635 => {}
                    _ => {
                        update_stats(STAT_DENIED);
                        log_message(
                            6 as libc::c_int,
                            b"Failed auth attempt (file descriptor: %d), ip %s\0"
                                as *const u8 as *const libc::c_char,
                            (*connptr).client_fd,
                            (*connptr).client_ip_addr,
                        );
                        indicate_http_error(
                            connptr,
                            401 as libc::c_int,
                            b"Unauthorized\0" as *const u8 as *const libc::c_char,
                            b"detail\0" as *const u8 as *const libc::c_char,
                            b"The administrator of this proxy has not configured it to service requests from you.\0"
                                as *const u8 as *const libc::c_char,
                            0 as *mut libc::c_void,
                        );
                        handle_connection_failure(connptr, got_headers);
                        current_block = 2650907734299715635;
                    }
                }
            } else {
                current_block = 13281731871476506071;
            }
            match current_block {
                2650907734299715635 => {}
                _ => {
                    if !((*config).add_headers).is_null() {
                        i = 0 as libc::c_int as size_t;
                        while i < (*(*config).add_headers).count {
                            let mut header: *mut http_header_t = sblist_get(
                                (*config).add_headers,
                                i,
                            ) as *mut http_header_t;
                            orderedmap_append(
                                hashofheaders,
                                (*header).name,
                                (*header).value,
                            );
                            i = i.wrapping_add(1);
                            i;
                        }
                    }
                    request = process_request(connptr, hashofheaders);
                    if request.is_null() {
                        if (*connptr).show_stats == 0 {
                            update_stats(STAT_BADCONN);
                        }
                        handle_connection_failure(connptr, got_headers);
                    } else {
                        (*connptr)
                            .upstream_proxy = upstream_get(
                            (*request).host,
                            (*config).upstream_list,
                        );
                        if !((*connptr).upstream_proxy).is_null() {
                            if connect_to_upstream(connptr, request) < 0 as libc::c_int {
                                handle_connection_failure(connptr, got_headers);
                                current_block = 2650907734299715635;
                            } else {
                                current_block = 6033931424626438518;
                            }
                        } else {
                            (*connptr)
                                .server_fd = opensock(
                                (*request).host,
                                (*request).port as libc::c_int,
                                (*connptr).server_ip_addr,
                            );
                            if (*connptr).server_fd < 0 as libc::c_int {
                                indicate_http_error(
                                    connptr,
                                    500 as libc::c_int,
                                    b"Unable to connect\0" as *const u8 as *const libc::c_char,
                                    b"detail\0" as *const u8 as *const libc::c_char,
                                    b"Tinyproxy was unable to connect to the remote web server.\0"
                                        as *const u8 as *const libc::c_char,
                                    b"error\0" as *const u8 as *const libc::c_char,
                                    strerror(*__errno_location()),
                                    0 as *mut libc::c_void,
                                );
                                handle_connection_failure(connptr, got_headers);
                                current_block = 2650907734299715635;
                            } else {
                                log_message(
                                    8 as libc::c_int,
                                    b"Established connection to host \"%s\" using file descriptor %d.\0"
                                        as *const u8 as *const libc::c_char,
                                    (*request).host,
                                    (*connptr).server_fd,
                                );
                                if (*connptr).connect_method == 0 {
                                    establish_http_connection(connptr, request);
                                }
                                current_block = 6033931424626438518;
                            }
                        }
                        match current_block {
                            2650907734299715635 => {}
                            _ => {
                                if process_client_headers(connptr, hashofheaders)
                                    < 0 as libc::c_int
                                {
                                    update_stats(STAT_BADCONN);
                                    log_message(
                                        6 as libc::c_int,
                                        b"process_client_headers failed: %s. host \"%s\" using file descriptor %d.\0"
                                            as *const u8 as *const libc::c_char,
                                        strerror(*__errno_location()),
                                        (*request).host,
                                        (*connptr).server_fd,
                                    );
                                    handle_connection_failure(connptr, got_headers);
                                } else {
                                    if (*connptr).connect_method == 0
                                        || !((*connptr).upstream_proxy).is_null()
                                            && (*(*connptr).upstream_proxy).type_0 as libc::c_uint
                                                == PT_HTTP as libc::c_int as libc::c_uint
                                    {
                                        if process_server_headers(connptr) < 0 as libc::c_int {
                                            update_stats(STAT_BADCONN);
                                            log_message(
                                                6 as libc::c_int,
                                                b"process_server_headers failed: %s. host \"%s\" using file descriptor %d.\0"
                                                    as *const u8 as *const libc::c_char,
                                                strerror(*__errno_location()),
                                                (*request).host,
                                                (*connptr).server_fd,
                                            );
                                            handle_connection_failure(connptr, got_headers);
                                            current_block = 2650907734299715635;
                                        } else {
                                            current_block = 2467484839200770573;
                                        }
                                    } else if send_connect_method_response(connptr)
                                        < 0 as libc::c_int
                                    {
                                        log_message(
                                            3 as libc::c_int,
                                            b"handle_connection: Could not send CONNECT method greeting to client.\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                        update_stats(STAT_BADCONN);
                                        handle_connection_failure(connptr, got_headers);
                                        current_block = 2650907734299715635;
                                    } else {
                                        current_block = 2467484839200770573;
                                    }
                                    match current_block {
                                        2650907734299715635 => {}
                                        _ => {
                                            relay_connection(connptr);
                                            log_message(
                                                6 as libc::c_int,
                                                b"Closed connection between local client (fd:%d) and remote client (fd:%d)\0"
                                                    as *const u8 as *const libc::c_char,
                                                (*connptr).client_fd,
                                                (*connptr).server_fd,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free_request_struct(request);
    orderedmap_destroy(hashofheaders);
    conn_destroy_contents(connptr);
}
