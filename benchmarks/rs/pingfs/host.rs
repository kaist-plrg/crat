use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn strndup(__string: *const libc::c_char, __n: size_t) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_error(__req: *mut gaicb) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn net_send(
        host: *mut host,
        id: uint16_t,
        seqno: uint16_t,
        data: *const uint8_t,
        len: size_t,
    );
    fn net_recv(
        tv: *mut timeval,
        recv_fn: net_recv_fn_t,
        recv_data: *mut libc::c_void,
    ) -> libc::c_int;
    fn net_inc_rx(packetsize: libc::c_int);
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
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
pub type uint64_t = __uint64_t;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gaicb {
    pub ar_name: *const libc::c_char,
    pub ar_service: *const libc::c_char,
    pub ar_request: *const addrinfo,
    pub ar_result: *mut addrinfo,
    pub __return: libc::c_int,
    pub __glibc_reserved: [libc::c_int; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct host {
    pub next: *mut host,
    pub sockaddr: sockaddr_storage,
    pub sockaddr_len: socklen_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linked_gaicb {
    pub gaicb: gaicb,
    pub next: *mut linked_gaicb,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eval_host {
    pub host: *mut host,
    pub sendtime: timespec,
    pub cur_seqno: uint16_t,
    pub id: uint16_t,
    pub payload: *mut uint8_t,
    pub payload_len: size_t,
    pub done: libc::c_int,
    pub num_tx: libc::c_int,
    pub num_rx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct evaldata {
    pub hosts: *mut eval_host,
    pub count: libc::c_int,
}
pub type net_recv_fn_t = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut sockaddr_storage,
        size_t,
        uint16_t,
        uint16_t,
        *mut *mut uint8_t,
        size_t,
    ) -> (),
>;
static mut addr_request: addrinfo = {
    let mut init = addrinfo {
        ai_flags: 0,
        ai_family: 0 as libc::c_int,
        ai_socktype: SOCK_RAW as libc::c_int,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *const sockaddr as *mut sockaddr,
        ai_canonname: 0 as *const libc::c_char as *mut libc::c_char,
        ai_next: 0 as *const addrinfo as *mut addrinfo,
    };
    init
};
pub unsafe extern "C" fn host_make_resolvlist(
    mut file: *mut FILE,
    mut list: *mut *mut *mut gaicb,
) -> libc::c_int {
    let mut hosts: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut l: *mut *mut gaicb = 0 as *mut *mut gaicb;
    let mut head: *mut linked_gaicb = 0 as *mut linked_gaicb;
    let mut tail: *mut linked_gaicb = 0 as *mut linked_gaicb;
    let mut lg: *mut linked_gaicb = 0 as *mut linked_gaicb;
    loop {
        let mut hostname: [libc::c_char; 300] = [0; 300];
        let mut res: libc::c_int = 0;
        memset(
            hostname.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[libc::c_char; 300]>() as libc::c_ulong,
        );
        res = fscanf(
            file,
            b"%256s\0" as *const u8 as *const libc::c_char,
            hostname.as_mut_ptr(),
        );
        if res == -(1 as libc::c_int) {
            break;
        }
        lg = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<linked_gaicb>() as libc::c_ulong,
        ) as *mut linked_gaicb;
        if lg.is_null() {
            return 0 as libc::c_int;
        }
        (*lg)
            .gaicb
            .ar_name = strndup(hostname.as_mut_ptr(), strlen(hostname.as_mut_ptr()));
        (*lg).gaicb.ar_request = &addr_request;
        if head.is_null() {
            head = lg;
        }
        if !tail.is_null() {
            (*tail).next = lg;
        }
        tail = lg;
        hosts += 1;
        hosts;
    }
    l = calloc(
        hosts as libc::c_ulong,
        ::std::mem::size_of::<*mut gaicb>() as libc::c_ulong,
    ) as *mut *mut gaicb;
    if l.is_null() {
        return 0 as libc::c_int;
    }
    lg = head;
    i = 0 as libc::c_int;
    while i < hosts {
        let ref mut fresh0 = *l.offset(i as isize);
        *fresh0 = &mut (*lg).gaicb;
        lg = (*lg).next;
        i += 1;
        i;
    }
    *list = l;
    return hosts;
}
pub unsafe extern "C" fn host_free_resolvlist(
    mut list: *mut *mut gaicb,
    mut length: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < length {
        free(
            (**list.offset(i as isize)).ar_name as *mut libc::c_char as *mut libc::c_void,
        );
        if !((**list.offset(i as isize)).ar_result).is_null() {
            freeaddrinfo((**list.offset(i as isize)).ar_result);
        }
        free(*list.offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    free(list as *mut libc::c_void);
}
pub unsafe extern "C" fn host_create(
    mut list: *mut *mut gaicb,
    mut listlength: libc::c_int,
) -> *mut host {
    let mut hosts: *mut host = 0 as *mut host;
    let mut last: *mut host = 0 as *mut host;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < listlength {
        if gai_error(*list.offset(i as isize)) == 0 as libc::c_int {
            let mut result: *mut addrinfo = (**list.offset(i as isize)).ar_result;
            while !result.is_null() {
                let mut h: *mut host = 0 as *mut host;
                h = calloc(
                    1 as libc::c_int as libc::c_ulong,
                    ::std::mem::size_of::<host>() as libc::c_ulong,
                ) as *mut host;
                if h.is_null() {
                    return 0 as *mut host;
                }
                memcpy(
                    &mut (*h).sockaddr as *mut sockaddr_storage as *mut libc::c_void,
                    (*result).ai_addr as *const libc::c_void,
                    (*result).ai_addrlen as libc::c_ulong,
                );
                (*h).sockaddr_len = (*result).ai_addrlen;
                if hosts.is_null() {
                    hosts = h;
                }
                if !last.is_null() {
                    (*last).next = h;
                }
                last = h;
                result = (*result).ai_next;
            }
        }
        i += 1;
        i;
    }
    return hosts;
}
static mut latency_sum_us: uint64_t = 0;
static mut latency_count: uint32_t = 0;
unsafe extern "C" fn diff_add(mut start: *mut timespec, mut end: *mut timespec) {
    let mut us: uint64_t = (((*end).tv_sec - (*start).tv_sec)
        * 1000000 as libc::c_int as libc::c_long) as uint64_t;
    us = (us as libc::c_ulong)
        .wrapping_sub(
            ((*start).tv_nsec / 1000 as libc::c_int as libc::c_long) as libc::c_ulong,
        ) as uint64_t as uint64_t;
    us = (us as libc::c_ulong)
        .wrapping_add(
            ((*end).tv_nsec / 1000 as libc::c_int as libc::c_long) as libc::c_ulong,
        ) as uint64_t as uint64_t;
    latency_sum_us = (latency_sum_us as libc::c_ulong).wrapping_add(us) as uint64_t
        as uint64_t;
    latency_count = latency_count.wrapping_add(1);
    latency_count;
}
unsafe extern "C" fn eval_reply(
    mut userdata: *mut libc::c_void,
    mut addr: *mut sockaddr_storage,
    mut addrlen: size_t,
    mut id: uint16_t,
    mut seqno: uint16_t,
    mut data: *mut *mut uint8_t,
    mut len: size_t,
) {
    let mut i: libc::c_int = 0;
    let mut eval: *mut evaldata = userdata as *mut evaldata;
    let mut recvtime: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(4 as libc::c_int, &mut recvtime);
    i = 0 as libc::c_int;
    while i < (*eval).count {
        let mut eh: *mut eval_host = &mut *((*eval).hosts).offset(i as isize)
            as *mut eval_host;
        if addrlen == (*(*eh).host).sockaddr_len as libc::c_ulong
            && memcmp(
                addr as *const libc::c_void,
                &mut (*(*eh).host).sockaddr as *mut sockaddr_storage
                    as *const libc::c_void,
                addrlen,
            ) == 0 as libc::c_int && (*eh).payload_len == len
            && memcmp(
                *data as *const libc::c_void,
                (*eh).payload as *const libc::c_void,
                (*eh).payload_len,
            ) == 0 as libc::c_int && (*eh).id as libc::c_int == id as libc::c_int
            && (*eh).cur_seqno as libc::c_int == seqno as libc::c_int
        {
            (*eh).num_rx += 1;
            (*eh).num_rx;
            (*eh).done = 1 as libc::c_int;
            net_inc_rx((*eh).payload_len as libc::c_int);
            (*eh).cur_seqno = ((*eh).cur_seqno).wrapping_add(1);
            (*eh).cur_seqno;
            diff_add(&mut (*eh).sendtime, &mut recvtime);
            break;
        } else {
            i += 1;
            i;
        }
    }
}
pub unsafe extern "C" fn host_evaluate(
    mut hosts: *mut *mut host,
    mut length: libc::c_int,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut addr: libc::c_int = 0;
    let mut good_hosts: libc::c_int = 0;
    let mut host: *mut host = 0 as *mut host;
    let mut prev: *mut host = 0 as *mut host;
    let mut evaldata: evaldata = evaldata {
        hosts: 0 as *mut eval_host,
        count: 0,
    };
    let mut eval_payload: [uint8_t; 1024] = [0; 1024];
    evaldata.count = length;
    evaldata
        .hosts = calloc(
        length as libc::c_ulong,
        ::std::mem::size_of::<eval_host>() as libc::c_ulong,
    ) as *mut eval_host;
    if (evaldata.hosts).is_null() {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < ::std::mem::size_of::<[uint8_t; 1024]>() as libc::c_ulong
    {
        eval_payload[i as usize] = (i & 0xff as libc::c_int) as uint8_t;
        i += 1;
        i;
    }
    addr = 0 as libc::c_int;
    host = *hosts;
    i = 0 as libc::c_int;
    while i < length {
        let ref mut fresh1 = (*(evaldata.hosts).offset(addr as isize)).host;
        *fresh1 = host;
        (*(evaldata.hosts).offset(addr as isize)).id = addr as uint16_t;
        (*(evaldata.hosts).offset(addr as isize))
            .cur_seqno = (addr * 2 as libc::c_int) as uint16_t;
        let ref mut fresh2 = (*(evaldata.hosts).offset(addr as isize)).payload;
        *fresh2 = eval_payload.as_mut_ptr();
        (*(evaldata.hosts).offset(addr as isize))
            .payload_len = ::std::mem::size_of::<[uint8_t; 1024]>() as libc::c_ulong;
        addr += 1;
        addr;
        host = (*host).next;
        i += 1;
        i;
    }
    printf(
        b"Evaluating %d hosts (timeout=%ds).\0" as *const u8 as *const libc::c_char,
        length,
        timeout,
    );
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        let mut h: libc::c_int = 0;
        let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
        printf(b".\0" as *const u8 as *const libc::c_char);
        fflush(stdout);
        h = 0 as libc::c_int;
        while h < length {
            let mut eh: *mut eval_host = &mut *(evaldata.hosts).offset(h as isize)
                as *mut eval_host;
            clock_gettime(4 as libc::c_int, &mut (*eh).sendtime);
            (*eh).done = 0 as libc::c_int;
            (*eh).num_tx += 1;
            (*eh).num_tx;
            net_send(
                (*eh).host,
                (*eh).id,
                (*eh).cur_seqno,
                (*eh).payload,
                (*eh).payload_len,
            );
            h += 1;
            h;
        }
        tv.tv_sec = timeout as __time_t;
        tv.tv_usec = 0 as libc::c_int as __suseconds_t;
        loop {
            let mut alldone: libc::c_int = 1 as libc::c_int;
            let mut res: libc::c_int = 0;
            h = 0 as libc::c_int;
            while h < length {
                alldone &= (*(evaldata.hosts).offset(h as isize)).done;
                h += 1;
                h;
            }
            if alldone != 0 {
                break;
            }
            res = net_recv(
                &mut tv,
                Some(
                    eval_reply
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut sockaddr_storage,
                            size_t,
                            uint16_t,
                            uint16_t,
                            *mut *mut uint8_t,
                            size_t,
                        ) -> (),
                ),
                &mut evaldata as *mut evaldata as *mut libc::c_void,
            );
            if !(res == 0) {
                continue;
            }
            break;
        }
        i += 1;
        i;
    }
    printf(b" done.\n\0" as *const u8 as *const libc::c_char);
    good_hosts = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < length {
        let mut eh_0: *mut eval_host = &mut *(evaldata.hosts).offset(i as isize)
            as *mut eval_host;
        if (*eh_0).num_tx == 0 as libc::c_int || (*eh_0).num_tx != (*eh_0).num_rx {
            (*(*eh_0).host).sockaddr_len = 0 as libc::c_int as socklen_t;
        } else {
            good_hosts += 1;
            good_hosts;
        }
        i += 1;
        i;
    }
    host = *hosts;
    prev = 0 as *mut host;
    while !host.is_null() {
        let mut next: *mut host = (*host).next;
        if (*host).sockaddr_len == 0 as libc::c_int as libc::c_uint {
            if host == *hosts {
                *hosts = next;
            }
            if !prev.is_null() {
                (*prev).next = next;
            }
            free(host as *mut libc::c_void);
        } else {
            prev = host;
        }
        host = next;
    }
    free(evaldata.hosts as *mut libc::c_void);
    printf(
        b"%d of %d hosts responded correctly to all pings\0" as *const u8
            as *const libc::c_char,
        good_hosts,
        length,
    );
    if good_hosts != 0 {
        printf(
            b" (average RTT %.02f ms)\0" as *const u8 as *const libc::c_char,
            (latency_sum_us as libc::c_float
                / (latency_count as libc::c_float * 1000.0f32)) as libc::c_double,
        );
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    return good_hosts;
}
static mut hosts_start: *mut host = 0 as *const host as *mut host;
static mut hosts_cur: *mut host = 0 as *const host as *mut host;
pub unsafe extern "C" fn host_use(mut hosts: *mut host) {
    hosts_start = hosts;
}
pub unsafe extern "C" fn host_get_next() -> *mut host {
    let mut h: *mut host = 0 as *mut host;
    if !hosts_start.is_null() {} else {
        __assert_fail(
            b"hosts_start\0" as *const u8 as *const libc::c_char,
            b"host.c\0" as *const u8 as *const libc::c_char,
            307 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"struct host *host_get_next()\0"))
                .as_ptr(),
        );
    };
    if hosts_cur.is_null() {
        hosts_cur = hosts_start;
    }
    h = hosts_cur;
    hosts_cur = (*hosts_cur).next;
    return h;
}
