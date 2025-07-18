use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type brubeck_tags_t;
    pub type brubeck_hashtable_t;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn __errno_location() -> *mut libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn brubeck_backend_run_threaded(_: *mut brubeck_backend);
    fn json_unpack_ex(
        root: *mut json_t,
        error: *mut json_error_t,
        flags: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn gh_log_die() -> !;
    fn gh_log_instance() -> *const libc::c_char;
    fn gh_log_write(message: *const libc::c_char, _: ...);
    fn url_to_inaddr2(
        addr: *mut sockaddr_in,
        url: *const libc::c_char,
        port: libc::c_int,
    );
    fn brubeck_itoa(ptr: *mut libc::c_char, number: uint64_t) -> libc::c_int;
    fn brubeck_ftoa(outbuf: *mut libc::c_char, f: libc::c_float) -> libc::c_int;
    fn sock_enlarge_out(fd: libc::c_int);
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
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
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type pthread_spinlock_t = libc::c_int;
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
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type C2RustUnnamed = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed = 256;
pub const IPPROTO_RAW: C2RustUnnamed = 255;
pub const IPPROTO_MPLS: C2RustUnnamed = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed = 136;
pub const IPPROTO_SCTP: C2RustUnnamed = 132;
pub const IPPROTO_COMP: C2RustUnnamed = 108;
pub const IPPROTO_PIM: C2RustUnnamed = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed = 94;
pub const IPPROTO_MTP: C2RustUnnamed = 92;
pub const IPPROTO_AH: C2RustUnnamed = 51;
pub const IPPROTO_ESP: C2RustUnnamed = 50;
pub const IPPROTO_GRE: C2RustUnnamed = 47;
pub const IPPROTO_RSVP: C2RustUnnamed = 46;
pub const IPPROTO_IPV6: C2RustUnnamed = 41;
pub const IPPROTO_DCCP: C2RustUnnamed = 33;
pub const IPPROTO_TP: C2RustUnnamed = 29;
pub const IPPROTO_IDP: C2RustUnnamed = 22;
pub const IPPROTO_UDP: C2RustUnnamed = 17;
pub const IPPROTO_PUP: C2RustUnnamed = 12;
pub const IPPROTO_EGP: C2RustUnnamed = 8;
pub const IPPROTO_TCP: C2RustUnnamed = 6;
pub const IPPROTO_IPIP: C2RustUnnamed = 4;
pub const IPPROTO_IGMP: C2RustUnnamed = 2;
pub const IPPROTO_ICMP: C2RustUnnamed = 1;
pub const IPPROTO_IP: C2RustUnnamed = 0;
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
pub type value_t = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_server {
    pub name: *const libc::c_char,
    pub dump_path: *const libc::c_char,
    pub config_name: *const libc::c_char,
    pub running: libc::c_int,
    pub active_backends: libc::c_int,
    pub active_samplers: libc::c_int,
    pub set_proctitle: bool,
    pub fd_signal: libc::c_int,
    pub fd_expire: libc::c_int,
    pub fd_update: libc::c_int,
    pub slab: brubeck_slab,
    pub metrics: *mut brubeck_hashtable_t,
    pub tags: *mut brubeck_tags_t,
    pub at_capacity: libc::c_int,
    pub samplers: [*mut brubeck_sampler; 8],
    pub backends: [*mut brubeck_backend; 8],
    pub config: *mut json_t,
    pub internal_stats: brubeck_internal_stats,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_internal_stats {
    pub sample_freq: libc::c_int,
    pub live: C2RustUnnamed_0,
    pub sample: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub metrics: uint32_t,
    pub errors: uint32_t,
    pub unique_keys: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_t {
    pub type_0: json_type,
    pub refcount: size_t,
}
pub type json_type = libc::c_uint;
pub const JSON_NULL: json_type = 7;
pub const JSON_FALSE: json_type = 6;
pub const JSON_TRUE: json_type = 5;
pub const JSON_REAL: json_type = 4;
pub const JSON_INTEGER: json_type = 3;
pub const JSON_STRING: json_type = 2;
pub const JSON_ARRAY: json_type = 1;
pub const JSON_OBJECT: json_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_backend {
    pub type_0: brubeck_backend_t,
    pub server: *mut brubeck_server,
    pub sample_freq: libc::c_int,
    pub shard_n: libc::c_int,
    pub connect: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub is_connected: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub sample: Option::<
        unsafe extern "C" fn(
            *const brubeck_metric,
            *const libc::c_char,
            value_t,
            *mut libc::c_void,
        ) -> (),
    >,
    pub flush: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub tick_time: uint32_t,
    pub thread: pthread_t,
    pub queue: *mut brubeck_metric,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_metric {
    pub next: *mut brubeck_metric,
    pub tags: *const brubeck_tag_set,
    pub lock: pthread_spinlock_t,
    pub key_len: uint16_t,
    pub type_0: uint8_t,
    pub private_state: uint8_t,
    pub as_0: C2RustUnnamed_1,
    pub key: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub gauge: C2RustUnnamed_3,
    pub meter: C2RustUnnamed_3,
    pub counter: C2RustUnnamed_2,
    pub histogram: brubeck_histo,
    pub other: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_histo {
    pub values: *mut value_t,
    pub count: uint32_t,
    pub alloc: uint16_t,
    pub size: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub value: value_t,
    pub previous: value_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub value: value_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_tag_set {
    pub index: uint32_t,
    pub tag_len: uint16_t,
    pub num_tags: uint16_t,
    pub tags: [brubeck_tag; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_tag {
    pub key: *const libc::c_char,
    pub value: *const libc::c_char,
}
pub type brubeck_backend_t = libc::c_uint;
pub const BRUBECK_BACKEND_KAFKA: brubeck_backend_t = 1;
pub const BRUBECK_BACKEND_CARBON: brubeck_backend_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_sampler {
    pub type_0: brubeck_sampler_t,
    pub server: *mut brubeck_server,
    pub in_sock: libc::c_int,
    pub addr: sockaddr_in,
    pub inflow: size_t,
    pub current_flow: size_t,
    pub shutdown: Option::<unsafe extern "C" fn(*mut brubeck_sampler) -> ()>,
}
pub type brubeck_sampler_t = libc::c_uint;
pub const BRUBECK_SAMPLER_STATSD: brubeck_sampler_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_slab {
    pub current: *mut brubeck_slab_node,
    pub total_alloc: size_t,
    pub lock: pthread_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_slab_node {
    pub next: *mut brubeck_slab_node,
    pub alloc: size_t,
    pub heap: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_error_t {
    pub line: libc::c_int,
    pub column: libc::c_int,
    pub position: libc::c_int,
    pub source: [libc::c_char; 80],
    pub text: [libc::c_char; 160],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_carbon {
    pub backend: brubeck_backend,
    pub out_sock: libc::c_int,
    pub out_sockaddr: sockaddr_in,
    pub pickler: pickler,
    pub bytes_sent: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pickler {
    pub ptr: *mut libc::c_char,
    pub pos: uint16_t,
    pub pt: uint16_t,
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
#[inline]
unsafe extern "C" fn xcalloc(mut n: size_t, mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = calloc(n, size);
    if (ptr == 0 as *mut libc::c_void) as libc::c_int as libc::c_long != 0 {
        fprintf(stderr, b"[FATAL]: oom\n\0" as *const u8 as *const libc::c_char);
        gh_log_die();
    }
    return ptr;
}
#[inline]
unsafe extern "C" fn write_in_full(
    mut fd: libc::c_int,
    mut buf: *const libc::c_void,
    mut count: size_t,
) -> ssize_t {
    let mut p: *const libc::c_char = buf as *const libc::c_char;
    let mut total: ssize_t = 0 as libc::c_int as ssize_t;
    while count > 0 as libc::c_int as libc::c_ulong {
        let mut written: ssize_t = xwrite(fd, p as *const libc::c_void, count);
        if written < 0 as libc::c_int as libc::c_long {
            return -(1 as libc::c_int) as ssize_t;
        }
        if written == 0 {
            *__errno_location() = 28 as libc::c_int;
            return -(1 as libc::c_int) as ssize_t;
        }
        count = (count as libc::c_ulong).wrapping_sub(written as libc::c_ulong) as size_t
            as size_t;
        p = p.offset(written as isize);
        total += written;
    }
    return total;
}
#[inline]
unsafe extern "C" fn xwrite(
    mut fd: libc::c_int,
    mut buf: *const libc::c_void,
    mut len: size_t,
) -> ssize_t {
    let mut nr: ssize_t = 0;
    loop {
        nr = write(fd, buf, len);
        if nr < 0 as libc::c_int as libc::c_long
            && (*__errno_location() == 11 as libc::c_int
                || *__errno_location() == 4 as libc::c_int)
        {
            continue;
        }
        return nr;
    };
}
unsafe extern "C" fn carbon_is_connected(mut backend: *mut libc::c_void) -> bool {
    let mut self_0: *mut brubeck_carbon = backend as *mut brubeck_carbon;
    return (*self_0).out_sock >= 0 as libc::c_int;
}
unsafe extern "C" fn carbon_connect(mut backend: *mut libc::c_void) -> libc::c_int {
    let mut self_0: *mut brubeck_carbon = backend as *mut brubeck_carbon;
    if carbon_is_connected(self_0 as *mut libc::c_void) {
        return 0 as libc::c_int;
    }
    (*self_0)
        .out_sock = socket(
        2 as libc::c_int,
        SOCK_STREAM as libc::c_int,
        IPPROTO_TCP as libc::c_int,
    );
    if (*self_0).out_sock >= 0 as libc::c_int {
        let mut rc: libc::c_int = connect(
            (*self_0).out_sock,
            &mut (*self_0).out_sockaddr as *mut sockaddr_in as *mut sockaddr,
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
        );
        if rc == 0 as libc::c_int {
            gh_log_write(
                b"instance=%s backend=carbon event=connected\n\0" as *const u8
                    as *const libc::c_char,
                gh_log_instance(),
            );
            sock_enlarge_out((*self_0).out_sock);
            return 0 as libc::c_int;
        }
        close((*self_0).out_sock);
        (*self_0).out_sock = -(1 as libc::c_int);
    }
    gh_log_write(
        b"instance=%s backend=carbon event=failed_to_connect errno=%d msg=\"%s\"\n\0"
            as *const u8 as *const libc::c_char,
        gh_log_instance(),
        *__errno_location(),
        strerror(*__errno_location()),
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn carbon_disconnect(mut self_0: *mut brubeck_carbon) {
    gh_log_write(
        b"instance=%s backend=carbon event=disconnected errno=%d msg=\"%s\"\n\0"
            as *const u8 as *const libc::c_char,
        gh_log_instance(),
        *__errno_location(),
        strerror(*__errno_location()),
    );
    close((*self_0).out_sock);
    (*self_0).out_sock = -(1 as libc::c_int);
}
unsafe extern "C" fn plaintext_each(
    mut metric: *const brubeck_metric,
    mut key: *const libc::c_char,
    mut value: value_t,
    mut backend: *mut libc::c_void,
) {
    let mut carbon: *mut brubeck_carbon = backend as *mut brubeck_carbon;
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut ptr: *mut libc::c_char = buffer.as_mut_ptr();
    let mut key_len: size_t = strlen(key);
    let mut wr: ssize_t = 0;
    if !carbon_is_connected(carbon as *mut libc::c_void) {
        return;
    }
    if !(strchr(key, ' ' as i32)).is_null() {
        return;
    }
    memcpy(ptr as *mut libc::c_void, key as *const libc::c_void, key_len);
    ptr = ptr.offset(key_len as isize);
    let fresh0 = ptr;
    ptr = ptr.offset(1);
    *fresh0 = ' ' as i32 as libc::c_char;
    ptr = ptr.offset(brubeck_ftoa(ptr, value as libc::c_float) as isize);
    let fresh1 = ptr;
    ptr = ptr.offset(1);
    *fresh1 = ' ' as i32 as libc::c_char;
    ptr = ptr
        .offset(brubeck_itoa(ptr, (*carbon).backend.tick_time as uint64_t) as isize);
    let fresh2 = ptr;
    ptr = ptr.offset(1);
    *fresh2 = '\n' as i32 as libc::c_char;
    wr = write_in_full(
        (*carbon).out_sock,
        buffer.as_mut_ptr() as *const libc::c_void,
        ptr.offset_from(buffer.as_mut_ptr()) as libc::c_long as size_t,
    );
    if wr < 0 as libc::c_int as libc::c_long {
        carbon_disconnect(carbon);
        return;
    }
    (*carbon)
        .bytes_sent = ((*carbon).bytes_sent as libc::c_ulong)
        .wrapping_add(wr as libc::c_ulong) as size_t as size_t;
}
#[inline]
unsafe extern "C" fn pickle1_int32(
    mut ptr: *mut libc::c_char,
    mut _src: *mut libc::c_void,
) -> size_t {
    *ptr = 'J' as i32 as libc::c_char;
    memcpy(
        ptr.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        _src,
        4 as libc::c_int as libc::c_ulong,
    );
    return 5 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn pickle1_double(
    mut ptr: *mut libc::c_char,
    mut _src: *mut libc::c_void,
) -> size_t {
    let mut source: *mut uint8_t = _src as *mut uint8_t;
    let fresh3 = ptr;
    ptr = ptr.offset(1);
    *fresh3 = 'G' as i32 as libc::c_char;
    *ptr
        .offset(
            0 as libc::c_int as isize,
        ) = *source.offset(7 as libc::c_int as isize) as libc::c_char;
    *ptr
        .offset(
            1 as libc::c_int as isize,
        ) = *source.offset(6 as libc::c_int as isize) as libc::c_char;
    *ptr
        .offset(
            2 as libc::c_int as isize,
        ) = *source.offset(5 as libc::c_int as isize) as libc::c_char;
    *ptr
        .offset(
            3 as libc::c_int as isize,
        ) = *source.offset(4 as libc::c_int as isize) as libc::c_char;
    *ptr
        .offset(
            4 as libc::c_int as isize,
        ) = *source.offset(3 as libc::c_int as isize) as libc::c_char;
    *ptr
        .offset(
            5 as libc::c_int as isize,
        ) = *source.offset(2 as libc::c_int as isize) as libc::c_char;
    *ptr
        .offset(
            6 as libc::c_int as isize,
        ) = *source.offset(1 as libc::c_int as isize) as libc::c_char;
    *ptr
        .offset(
            7 as libc::c_int as isize,
        ) = *source.offset(0 as libc::c_int as isize) as libc::c_char;
    return 9 as libc::c_int as size_t;
}
unsafe extern "C" fn pickle1_push(
    mut buf: *mut pickler,
    mut key: *const libc::c_char,
    mut key_len: uint8_t,
    mut timestamp: uint32_t,
    mut value: value_t,
) {
    let mut ptr: *mut libc::c_char = ((*buf).ptr)
        .offset((*buf).pos as libc::c_int as isize);
    let fresh4 = ptr;
    ptr = ptr.offset(1);
    *fresh4 = '(' as i32 as libc::c_char;
    let fresh5 = ptr;
    ptr = ptr.offset(1);
    *fresh5 = 'U' as i32 as libc::c_char;
    let fresh6 = ptr;
    ptr = ptr.offset(1);
    *fresh6 = key_len as libc::c_char;
    memcpy(
        ptr as *mut libc::c_void,
        key as *const libc::c_void,
        key_len as libc::c_ulong,
    );
    ptr = ptr.offset(key_len as libc::c_int as isize);
    let fresh7 = ptr;
    ptr = ptr.offset(1);
    *fresh7 = 'q' as i32 as libc::c_char;
    let fresh8 = (*buf).pt;
    (*buf).pt = ((*buf).pt).wrapping_add(1);
    let fresh9 = ptr;
    ptr = ptr.offset(1);
    *fresh9 = fresh8 as libc::c_char;
    let fresh10 = ptr;
    ptr = ptr.offset(1);
    *fresh10 = '(' as i32 as libc::c_char;
    ptr = ptr
        .offset(
            pickle1_int32(ptr, &mut timestamp as *mut uint32_t as *mut libc::c_void)
                as isize,
        );
    ptr = ptr
        .offset(
            pickle1_double(ptr, &mut value as *mut value_t as *mut libc::c_void) as isize,
        );
    let fresh11 = ptr;
    ptr = ptr.offset(1);
    *fresh11 = 't' as i32 as libc::c_char;
    let fresh12 = ptr;
    ptr = ptr.offset(1);
    *fresh12 = 'q' as i32 as libc::c_char;
    let fresh13 = (*buf).pt;
    (*buf).pt = ((*buf).pt).wrapping_add(1);
    let fresh14 = ptr;
    ptr = ptr.offset(1);
    *fresh14 = fresh13 as libc::c_char;
    let fresh15 = ptr;
    ptr = ptr.offset(1);
    *fresh15 = 't' as i32 as libc::c_char;
    let fresh16 = ptr;
    ptr = ptr.offset(1);
    *fresh16 = 'q' as i32 as libc::c_char;
    let fresh17 = (*buf).pt;
    (*buf).pt = ((*buf).pt).wrapping_add(1);
    let fresh18 = ptr;
    ptr = ptr.offset(1);
    *fresh18 = fresh17 as libc::c_char;
    (*buf).pos = ptr.offset_from((*buf).ptr) as libc::c_long as uint16_t;
}
#[inline]
unsafe extern "C" fn pickle1_init(mut buf: *mut pickler) {
    static mut lead: [uint8_t; 4] = [
        ']' as i32 as uint8_t,
        'q' as i32 as uint8_t,
        0 as libc::c_int as uint8_t,
        '(' as i32 as uint8_t,
    ];
    memcpy(
        ((*buf).ptr).offset(4 as libc::c_int as isize) as *mut libc::c_void,
        lead.as_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong,
    );
    (*buf)
        .pos = (4 as libc::c_int as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong)
        as uint16_t;
    (*buf).pt = 1 as libc::c_int as uint16_t;
}
unsafe extern "C" fn pickle1_flush(mut backend: *mut libc::c_void) {
    static mut trail: [uint8_t; 2] = ['e' as i32 as uint8_t, '.' as i32 as uint8_t];
    let mut carbon: *mut brubeck_carbon = backend as *mut brubeck_carbon;
    let mut buf: *mut pickler = &mut (*carbon).pickler;
    let mut buf_lead: *mut uint32_t = 0 as *mut uint32_t;
    let mut wr: ssize_t = 0;
    if (*buf).pt as libc::c_int == 1 as libc::c_int
        || !carbon_is_connected(carbon as *mut libc::c_void)
    {
        return;
    }
    memcpy(
        ((*buf).ptr).offset((*buf).pos as libc::c_int as isize) as *mut libc::c_void,
        trail.as_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint8_t; 2]>() as libc::c_ulong,
    );
    (*buf)
        .pos = ((*buf).pos as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<[uint8_t; 2]>() as libc::c_ulong) as uint16_t
        as uint16_t;
    buf_lead = (*buf).ptr as *mut uint32_t;
    *buf_lead = __bswap_32(
        ((*buf).pos as uint32_t).wrapping_sub(4 as libc::c_int as libc::c_uint),
    );
    wr = write_in_full(
        (*carbon).out_sock,
        (*buf).ptr as *const libc::c_void,
        (*buf).pos as size_t,
    );
    pickle1_init(&mut (*carbon).pickler);
    if wr < 0 as libc::c_int as libc::c_long {
        carbon_disconnect(carbon);
        return;
    }
    (*carbon)
        .bytes_sent = ((*carbon).bytes_sent as libc::c_ulong)
        .wrapping_add(wr as libc::c_ulong) as size_t as size_t;
}
unsafe extern "C" fn pickle1_each(
    mut metric: *const brubeck_metric,
    mut key: *const libc::c_char,
    mut value: value_t,
    mut backend: *mut libc::c_void,
) {
    let mut carbon: *mut brubeck_carbon = backend as *mut brubeck_carbon;
    if !(strchr(key, ' ' as i32)).is_null() {
        return;
    }
    let mut key_len: uint8_t = strlen(key) as uint8_t;
    if (*carbon).pickler.pos as libc::c_int
        + (32 as libc::c_int + key_len as libc::c_int) >= 4096 as libc::c_int
    {
        pickle1_flush(carbon as *mut libc::c_void);
    }
    if !carbon_is_connected(carbon as *mut libc::c_void) {
        return;
    }
    pickle1_push(
        &mut (*carbon).pickler,
        key,
        key_len,
        (*carbon).backend.tick_time,
        value,
    );
}
pub unsafe extern "C" fn brubeck_carbon_new(
    mut server: *mut brubeck_server,
    mut settings: *mut json_t,
    mut shard_n: libc::c_int,
) -> *mut brubeck_backend {
    let mut carbon: *mut brubeck_carbon = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<brubeck_carbon>() as libc::c_ulong,
    ) as *mut brubeck_carbon;
    let mut address: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut port: libc::c_int = 0;
    let mut frequency: libc::c_int = 0;
    let mut pickle: libc::c_int = 0 as libc::c_int;
    let mut _error_j: json_error_t = json_error_t {
        line: 0,
        column: 0,
        position: 0,
        source: [0; 80],
        text: [0; 160],
    };
    if json_unpack_ex(
        settings,
        &mut _error_j as *mut json_error_t,
        0 as libc::c_int as size_t,
        b"{s:s, s:i, s?:b, s:i}\0" as *const u8 as *const libc::c_char,
        b"address\0" as *const u8 as *const libc::c_char,
        &mut address as *mut *mut libc::c_char,
        b"port\0" as *const u8 as *const libc::c_char,
        &mut port as *mut libc::c_int,
        b"pickle\0" as *const u8 as *const libc::c_char,
        &mut pickle as *mut libc::c_int,
        b"frequency\0" as *const u8 as *const libc::c_char,
        &mut frequency as *mut libc::c_int,
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"[FATAL]: config error: %s\n\0" as *const u8 as *const libc::c_char,
            (_error_j.text).as_mut_ptr(),
        );
        gh_log_die();
    }
    (*carbon).backend.type_0 = BRUBECK_BACKEND_CARBON;
    (*carbon).backend.shard_n = shard_n;
    (*carbon)
        .backend
        .connect = Some(
        carbon_connect as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
    );
    (*carbon)
        .backend
        .is_connected = Some(
        carbon_is_connected as unsafe extern "C" fn(*mut libc::c_void) -> bool,
    );
    if pickle != 0 {
        (*carbon)
            .backend
            .sample = Some(
            pickle1_each
                as unsafe extern "C" fn(
                    *const brubeck_metric,
                    *const libc::c_char,
                    value_t,
                    *mut libc::c_void,
                ) -> (),
        );
        (*carbon)
            .backend
            .flush = Some(
            pickle1_flush as unsafe extern "C" fn(*mut libc::c_void) -> (),
        );
        (*carbon)
            .pickler
            .ptr = malloc(4096 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
        pickle1_init(&mut (*carbon).pickler);
    } else {
        (*carbon)
            .backend
            .sample = Some(
            plaintext_each
                as unsafe extern "C" fn(
                    *const brubeck_metric,
                    *const libc::c_char,
                    value_t,
                    *mut libc::c_void,
                ) -> (),
        );
        (*carbon).backend.flush = None;
    }
    (*carbon).backend.sample_freq = frequency;
    (*carbon).backend.server = server;
    (*carbon).out_sock = -(1 as libc::c_int);
    url_to_inaddr2(&mut (*carbon).out_sockaddr, address, port);
    brubeck_backend_run_threaded(carbon as *mut brubeck_backend);
    gh_log_write(
        b"instance=%s backend=carbon event=started\n\0" as *const u8
            as *const libc::c_char,
        gh_log_instance(),
    );
    return carbon as *mut brubeck_backend;
}
