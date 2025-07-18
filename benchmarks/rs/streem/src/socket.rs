use ::libc;
extern "C" {
    pub type strm_queue;
    pub type node_error;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> libc::c_int;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strm_cfunc_value(_: strm_cfunc) -> strm_value;
    fn strm_value_int(_: strm_value) -> strm_int;
    fn strm_number_p(_: strm_value) -> libc::c_int;
    fn strm_ptr_value(_: *mut libc::c_void) -> strm_value;
    fn strm_str_cstr(_: strm_string, buf: *mut libc::c_char) -> *const libc::c_char;
    fn strm_stream_new(
        mode: strm_stream_mode,
        start: strm_callback,
        close_0: strm_callback,
        data: *mut libc::c_void,
    ) -> *mut strm_stream;
    fn strm_io_emit(
        strm: *mut strm_stream,
        data: strm_value,
        fd: libc::c_int,
        cb: strm_callback,
    );
    fn strm_raise(_: *mut strm_stream, _: *const libc::c_char);
    fn strm_parse_args(
        _: *mut strm_stream,
        _: libc::c_int,
        _: *mut strm_value,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strm_var_def(
        _: *mut strm_state,
        _: *const libc::c_char,
        _: strm_value,
    ) -> libc::c_int;
    fn strm_io_new(fd: libc::c_int, mode: libc::c_int) -> strm_value;
    fn strm_io_start_read(strm: *mut strm_stream, fd: libc::c_int, cb: strm_callback);
}
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __socklen_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type socklen_t = __socklen_t;
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
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
pub type strm_value = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_state {
    pub env: *mut libc::c_void,
    pub prev: *mut strm_state,
    pub flags: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_stream {
    pub type_0: strm_ptr_type,
    pub flags: libc::c_uint,
    pub mode: strm_stream_mode,
    pub start_func: strm_callback,
    pub close_func: strm_callback,
    pub data: *mut libc::c_void,
    pub dst: *mut strm_stream,
    pub rest: *mut *mut strm_stream,
    pub rsize: size_t,
    pub rcapa: size_t,
    pub exc: *mut node_error,
    pub refcnt: strm_int,
    pub queue: *mut strm_queue,
    pub excl: strm_int,
}
pub type strm_int = int32_t;
pub type strm_callback = Option::<
    unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
>;
pub type strm_stream_mode = libc::c_uint;
pub const strm_killed: strm_stream_mode = 4;
pub const strm_dying: strm_stream_mode = 3;
pub const strm_consumer: strm_stream_mode = 2;
pub const strm_filter: strm_stream_mode = 1;
pub const strm_producer: strm_stream_mode = 0;
pub type strm_ptr_type = libc::c_uint;
pub const STRM_PTR_AUX: strm_ptr_type = 4;
pub const STRM_PTR_IO: strm_ptr_type = 3;
pub const STRM_PTR_GENFUNC: strm_ptr_type = 2;
pub const STRM_PTR_LAMBDA: strm_ptr_type = 1;
pub const STRM_PTR_STREAM: strm_ptr_type = 0;
pub type strm_cfunc = Option::<
    unsafe extern "C" fn(
        *mut strm_stream,
        libc::c_int,
        *mut strm_value,
        *mut strm_value,
    ) -> libc::c_int,
>;
pub type strm_string = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct socket_data {
    pub sock: libc::c_int,
}
unsafe extern "C" fn accept_cb(
    mut task: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut sd: *mut socket_data = (*task).data as *mut socket_data;
    let mut writer_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut writer_len: socklen_t = 0;
    let mut sock: libc::c_int = 0;
    memset(
        &mut writer_addr as *mut sockaddr_in as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    writer_len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    sock = accept(
        (*sd).sock,
        &mut writer_addr as *mut sockaddr_in as *mut sockaddr,
        &mut writer_len,
    );
    if sock < 0 as libc::c_int {
        close(sock);
        strm_raise(task, b"socket error: listen\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    strm_io_emit(
        task,
        strm_io_new(sock, 1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int),
        (*sd).sock,
        Some(
            accept_cb
                as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
        ),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn server_accept(
    mut task: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut sd: *mut socket_data = (*task).data as *mut socket_data;
    strm_io_start_read(
        task,
        (*sd).sock,
        Some(
            accept_cb
                as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
        ),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn server_close(
    mut task: *mut strm_stream,
    mut d: strm_value,
) -> libc::c_int {
    let mut sd: *mut socket_data = (*task).data as *mut socket_data;
    close((*sd).sock);
    free(sd as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn tcp_server(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut result: *mut addrinfo = 0 as *mut addrinfo;
    let mut rp: *mut addrinfo = 0 as *mut addrinfo;
    let mut sock: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut srv: strm_value = 0;
    let mut service: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: [libc::c_char; 12] = [0; 12];
    let mut sd: *mut socket_data = 0 as *mut socket_data;
    let mut t: *mut strm_stream = 0 as *mut strm_stream;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"v\0" as *const u8 as *const libc::c_char,
        &mut srv as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if strm_number_p(srv) != 0 {
        sprintf(
            buf.as_mut_ptr(),
            b"%d\0" as *const u8 as *const libc::c_char,
            strm_value_int(srv),
        );
        service = buf.as_mut_ptr();
    } else {
        let mut str: strm_string = srv;
        service = strm_str_cstr(str, buf.as_mut_ptr());
    }
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = 0 as libc::c_int;
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    hints.ai_flags = 0x1 as libc::c_int;
    hints.ai_protocol = 0 as libc::c_int;
    loop {
        s = getaddrinfo(0 as *const libc::c_char, service, &mut hints, &mut result);
        if !(s != 0 as libc::c_int) {
            break;
        }
        if s == -(3 as libc::c_int) {
            continue;
        }
        strm_raise(strm, gai_strerror(s));
        return 1 as libc::c_int;
    }
    rp = result;
    while !rp.is_null() {
        sock = socket((*rp).ai_family, (*rp).ai_socktype, (*rp).ai_protocol);
        if !(sock == -(1 as libc::c_int)) {
            if bind(sock, (*rp).ai_addr, (*rp).ai_addrlen) == 0 as libc::c_int {
                break;
            }
            close(sock);
        }
        rp = (*rp).ai_next;
    }
    freeaddrinfo(result);
    if rp.is_null() {
        strm_raise(strm, b"socket error: bind\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    if listen(sock, 5 as libc::c_int) < 0 as libc::c_int {
        close(sock);
        strm_raise(strm, b"socket error: listen\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    sd = malloc(::std::mem::size_of::<socket_data>() as libc::c_ulong)
        as *mut socket_data;
    (*sd).sock = sock;
    t = strm_stream_new(
        strm_producer,
        Some(
            server_accept
                as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
        ),
        Some(
            server_close
                as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
        ),
        sd as *mut libc::c_void,
    );
    *ret = strm_ptr_value(t as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn tcp_socket(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut result: *mut addrinfo = 0 as *mut addrinfo;
    let mut rp: *mut addrinfo = 0 as *mut addrinfo;
    let mut sock: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut service: *const libc::c_char = 0 as *const libc::c_char;
    let mut sbuf: [libc::c_char; 12] = [0; 12];
    let mut hbuf: [libc::c_char; 7] = [0; 7];
    let mut host: strm_string = 0;
    let mut srv: strm_value = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"Sv\0" as *const u8 as *const libc::c_char,
        &mut host as *mut strm_string,
        &mut srv as *mut strm_value,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if strm_number_p(srv) != 0 {
        sprintf(
            sbuf.as_mut_ptr(),
            b"%d\0" as *const u8 as *const libc::c_char,
            strm_value_int(srv),
        );
        service = sbuf.as_mut_ptr();
    } else {
        let mut str: strm_string = srv;
        service = strm_str_cstr(str, sbuf.as_mut_ptr());
    }
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = 0 as libc::c_int;
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    hints.ai_protocol = 0 as libc::c_int;
    s = getaddrinfo(
        strm_str_cstr(host, hbuf.as_mut_ptr()),
        service,
        &mut hints,
        &mut result,
    );
    if s != 0 as libc::c_int {
        strm_raise(strm, gai_strerror(s));
        return 1 as libc::c_int;
    }
    rp = result;
    while !rp.is_null() {
        sock = socket((*rp).ai_family, (*rp).ai_socktype, (*rp).ai_protocol);
        if !(sock == -(1 as libc::c_int)) {
            if connect(sock, (*rp).ai_addr, (*rp).ai_addrlen) != -(1 as libc::c_int) {
                break;
            }
            close(sock);
        }
        rp = (*rp).ai_next;
    }
    freeaddrinfo(result);
    if rp.is_null() {
        strm_raise(strm, b"socket error: connect\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    *ret = strm_io_new(sock, 1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strm_socket_init(mut state: *mut strm_state) {
    strm_var_def(
        state,
        b"tcp_server\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                tcp_server
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        state,
        b"tcp_socket\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                tcp_socket
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
}
