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
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn pthread_kill(__threadid: pthread_t, __signo: libc::c_int) -> libc::c_int;
    fn reload_config(reload_logging: libc::c_int) -> libc::c_int;
    static mut received_sighup: libc::c_uint;
    static mut config: *mut config_s;
    fn sblist_new(itemsize: size_t, blockitems: size_t) -> *mut sblist;
    fn sblist_free(l: *mut sblist);
    fn sblist_get(l: *mut sblist, item: size_t) -> *mut libc::c_void;
    fn sblist_add(l: *mut sblist, item: *mut libc::c_void) -> libc::c_int;
    fn sblist_delete(l: *mut sblist, item: size_t);
    fn filter_reload();
    fn log_message(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn handle_connection(_: *mut conn_s, addr: *mut sockaddr_union);
    fn listen_sock(
        addr: *const libc::c_char,
        port: uint16_t,
        listen_fds_0: *mut sblist,
    ) -> libc::c_int;
    fn conn_struct_init(connptr: *mut conn_s);
    fn mypoll(
        fds: *mut pollfd_struct,
        nfds: libc::c_int,
        timeout: libc::c_int,
    ) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_attr_init(__attr: *mut pthread_attr_t) -> libc::c_int;
    fn pthread_attr_setstacksize(
        __attr: *mut pthread_attr_t,
        __stacksize: size_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __useconds_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type ssize_t = __ssize_t;
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
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
pub type pollfd_struct = pollfd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct child {
    pub thread: pthread_t,
    pub client: client,
    pub conn: conn_s,
    pub done: libc::c_int,
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
pub struct client {
    pub addr: sockaddr_union,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sockaddr_union {
    pub v4: sockaddr_in,
    pub v6: sockaddr_in6,
}
static mut listen_fds: *mut sblist = 0 as *const sblist as *mut sblist;
unsafe extern "C" fn child_thread(mut data: *mut libc::c_void) -> *mut libc::c_void {
    let mut c: *mut child = data as *mut child;
    handle_connection(&mut (*c).conn, &mut (*c).client.addr);
    ::std::ptr::write_volatile(&mut (*c).done as *mut libc::c_int, 1 as libc::c_int);
    return 0 as *mut libc::c_void;
}
static mut childs: *mut sblist = 0 as *const sblist as *mut sblist;
unsafe extern "C" fn collect_threads() {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*childs).count {
        let mut c: *mut child = *(sblist_get(childs, i) as *mut *mut child);
        if (*c).done != 0 {
            pthread_join((*c).thread, 0 as *mut *mut libc::c_void);
            sblist_delete(childs, i);
            free(c as *mut libc::c_void);
            c = 0 as *mut child;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
}
pub unsafe extern "C" fn child_main_loop() {
    let mut connfd: libc::c_int = 0;
    let mut cliaddr_storage: sockaddr_union = sockaddr_union {
        v4: sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        },
    };
    let mut cliaddr: *mut sockaddr = &mut cliaddr_storage as *mut sockaddr_union
        as *mut libc::c_void as *mut sockaddr;
    let mut clilen: socklen_t = 0;
    let mut nfds: libc::c_int = (*listen_fds).count as libc::c_int;
    let mut fds: *mut pollfd_struct = calloc(
        nfds as libc::c_ulong,
        ::std::mem::size_of::<pollfd_struct>() as libc::c_ulong,
    ) as *mut pollfd_struct;
    let mut i: ssize_t = 0;
    let mut ret: libc::c_int = 0;
    let mut listenfd: libc::c_int = 0;
    let mut was_full: libc::c_int = 0 as libc::c_int;
    let mut attrp: *mut pthread_attr_t = 0 as *mut pthread_attr_t;
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    let mut child: *mut child = 0 as *mut child;
    childs = sblist_new(
        ::std::mem::size_of::<*mut child>() as libc::c_ulong,
        (*config).maxclients as size_t,
    );
    i = 0 as libc::c_int as ssize_t;
    while i < nfds as libc::c_long {
        let mut fd: *mut libc::c_int = sblist_get(listen_fds, i as size_t)
            as *mut libc::c_int;
        (*fds.offset(i as isize)).fd = *fd;
        let ref mut fresh0 = (*fds.offset(i as isize)).events;
        *fresh0 = (*fresh0 as libc::c_int | 0x1 as libc::c_int) as libc::c_short;
        i += 1;
        i;
    }
    while (*config).quit == 0 {
        collect_threads();
        if (*childs).count >= (*config).maxclients as libc::c_ulong {
            if was_full == 0 {
                log_message(
                    4 as libc::c_int,
                    b"Maximum number of connections reached. Refusing new connections.\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            was_full = 1 as libc::c_int;
            usleep(16 as libc::c_int as __useconds_t);
        } else {
            was_full = 0 as libc::c_int;
            listenfd = -(1 as libc::c_int);
            if received_sighup != 0 {
                reload_config(1 as libc::c_int);
                filter_reload();
                received_sighup = 0 as libc::c_int as libc::c_uint;
            }
            ret = mypoll(fds, nfds, -(1 as libc::c_int));
            if ret == -(1 as libc::c_int) {
                if *__errno_location() == 4 as libc::c_int {
                    continue;
                }
                log_message(
                    3 as libc::c_int,
                    b"error calling poll: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            } else if ret == 0 as libc::c_int {
                log_message(
                    4 as libc::c_int,
                    b"Strange: poll returned 0 but we did not specify a timeout...\0"
                        as *const u8 as *const libc::c_char,
                );
            } else {
                i = 0 as libc::c_int as ssize_t;
                while i < nfds as libc::c_long {
                    if (*fds.offset(i as isize)).revents as libc::c_int
                        & 0x1 as libc::c_int != 0
                    {
                        listenfd = (*fds.offset(i as isize)).fd;
                        break;
                    } else {
                        i += 1;
                        i;
                    }
                }
                if listenfd == -(1 as libc::c_int) {
                    log_message(
                        4 as libc::c_int,
                        b"Strange: None of our listen fds was readable after poll\0"
                            as *const u8 as *const libc::c_char,
                    );
                } else {
                    clilen = ::std::mem::size_of::<sockaddr_union>() as libc::c_ulong
                        as socklen_t;
                    connfd = accept(
                        listenfd,
                        __SOCKADDR_ARG {
                            __sockaddr__: cliaddr,
                        },
                        &mut clilen,
                    );
                    if connfd < 0 as libc::c_int {
                        log_message(
                            3 as libc::c_int,
                            b"Accept returned an error (%s) ... retrying.\0" as *const u8
                                as *const libc::c_char,
                            strerror(*__errno_location()),
                        );
                    } else {
                        child = calloc(
                            1 as libc::c_int as libc::c_ulong,
                            ::std::mem::size_of::<child>() as libc::c_ulong,
                        ) as *mut child;
                        if !child.is_null() {
                            ::std::ptr::write_volatile(
                                &mut (*child).done as *mut libc::c_int,
                                0 as libc::c_int,
                            );
                            if sblist_add(
                                childs,
                                &mut child as *mut *mut child as *mut libc::c_void,
                            ) == 0
                            {
                                free(child as *mut libc::c_void);
                            } else {
                                conn_struct_init(&mut (*child).conn);
                                (*child).conn.client_fd = connfd;
                                memcpy(
                                    &mut (*child).client.addr as *mut sockaddr_union
                                        as *mut libc::c_void,
                                    &mut cliaddr_storage as *mut sockaddr_union
                                        as *const libc::c_void,
                                    ::std::mem::size_of::<sockaddr_union>() as libc::c_ulong,
                                );
                                attrp = 0 as *mut pthread_attr_t;
                                if pthread_attr_init(&mut attr) == 0 as libc::c_int {
                                    attrp = &mut attr;
                                    pthread_attr_setstacksize(
                                        attrp,
                                        (256 as libc::c_int * 1024 as libc::c_int) as size_t,
                                    );
                                }
                                if !(pthread_create(
                                    &mut (*child).thread,
                                    attrp,
                                    Some(
                                        child_thread
                                            as unsafe extern "C" fn(
                                                *mut libc::c_void,
                                            ) -> *mut libc::c_void,
                                    ),
                                    child as *mut libc::c_void,
                                ) != 0 as libc::c_int)
                                {
                                    continue;
                                }
                                sblist_delete(
                                    childs,
                                    ((*childs).count)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                );
                                free(child as *mut libc::c_void);
                            }
                        }
                        close(connfd);
                        log_message(
                            2 as libc::c_int,
                            b"Could not allocate memory for child.\0" as *const u8
                                as *const libc::c_char,
                        );
                        usleep(16 as libc::c_int as __useconds_t);
                    }
                }
            }
        }
    }
    free(fds as *mut libc::c_void);
    fds = 0 as *mut pollfd_struct;
}
pub unsafe extern "C" fn child_kill_children(mut sig: libc::c_int) {
    let mut i: size_t = 0;
    let mut tries: size_t = 0 as libc::c_int as size_t;
    if sig != 15 as libc::c_int {
        return;
    }
    log_message(
        6 as libc::c_int,
        b"trying to bring down %zu threads...\0" as *const u8 as *const libc::c_char,
        (*childs).count,
    );
    loop {
        i = 0 as libc::c_int as size_t;
        while i < (*childs).count {
            let mut c: *mut child = *(sblist_get(childs, i) as *mut *mut child);
            if (*c).done == 0 {
                pthread_kill((*c).thread, 17 as libc::c_int);
            }
            i = i.wrapping_add(1);
            i;
        }
        usleep(8192 as libc::c_int as __useconds_t);
        collect_threads();
        if !((*childs).count != 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        let fresh1 = tries;
        tries = tries.wrapping_add(1);
        if !(fresh1 < 8 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    if (*childs).count != 0 as libc::c_int as libc::c_ulong {
        log_message(
            2 as libc::c_int,
            b"child_kill_children: %zu threads still alive!\0" as *const u8
                as *const libc::c_char,
            (*childs).count,
        );
    }
}
pub unsafe extern "C" fn child_free_children() {
    sblist_free(childs);
    childs = 0 as *mut sblist;
}
pub unsafe extern "C" fn child_listening_sockets(
    mut listen_addrs: *mut sblist,
    mut port: uint16_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut i: size_t = 0;
    if listen_fds.is_null() {
        listen_fds = sblist_new(
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            16 as libc::c_int as size_t,
        );
        if listen_fds.is_null() {
            log_message(
                3 as libc::c_int,
                b"Could not create the list of listening fds\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    if listen_addrs.is_null() || (*listen_addrs).count == 0 {
        ret = listen_sock(0 as *const libc::c_char, port, listen_fds);
        return ret;
    }
    i = 0 as libc::c_int as size_t;
    while i < (*listen_addrs).count {
        let mut addr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        addr = sblist_get(listen_addrs, i) as *mut *mut libc::c_char;
        if addr.is_null() || (*addr).is_null() {
            log_message(
                4 as libc::c_int,
                b"got NULL from listen_addrs - skipping\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            ret = listen_sock(*addr, port, listen_fds);
            if ret != 0 as libc::c_int {
                return ret;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn child_close_sock() {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*listen_fds).count {
        let mut fd: *mut libc::c_int = sblist_get(listen_fds, i) as *mut libc::c_int;
        close(*fd);
        i = i.wrapping_add(1);
        i;
    }
    sblist_free(listen_fds);
    listen_fds = 0 as *mut sblist;
}
