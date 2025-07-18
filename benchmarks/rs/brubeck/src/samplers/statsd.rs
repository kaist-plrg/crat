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
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type brubeck_tags_t;
    pub type brubeck_hashtable_t;
    fn recvfrom(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    fn recvmmsg(
        __fd: libc::c_int,
        __vmessages: *mut mmsghdr,
        __vlen: libc::c_uint,
        __flags: libc::c_int,
        __tmo: *mut timespec,
    ) -> libc::c_int;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_cancel(__th: pthread_t) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn json_unpack_ex(
        root: *mut json_t,
        error: *mut json_error_t,
        flags: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn brubeck_sampler_init_inet(
        sampler: *mut brubeck_sampler,
        server: *mut brubeck_server,
        url: *const libc::c_char,
        port: libc::c_int,
    );
    fn brubeck_sampler_socket(
        sampler: *mut brubeck_sampler,
        multisock: libc::c_int,
    ) -> libc::c_int;
    fn brubeck_metric_find(
        server: *mut brubeck_server,
        _: *const libc::c_char,
        _: size_t,
        _: uint8_t,
    ) -> *mut brubeck_metric;
    fn brubeck_metric_record(
        metric: *mut brubeck_metric,
        value: value_t,
        sample_rate: value_t,
        modifiers: uint8_t,
    );
    fn gh_log_write(message: *const libc::c_char, _: ...);
    fn gh_log_die() -> !;
    fn gh_log_instance() -> *const libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type pthread_spinlock_t = libc::c_int;
pub type socklen_t = __socklen_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut libc::c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut libc::c_void,
    pub msg_controllen: size_t,
    pub msg_flags: libc::c_int,
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
    pub __in6_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub struct mmsghdr {
    pub msg_hdr: msghdr,
    pub msg_len: libc::c_uint,
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
    pub live: C2RustUnnamed_1,
    pub sample: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
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
    pub as_0: C2RustUnnamed_2,
    pub key: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub gauge: C2RustUnnamed_4,
    pub meter: C2RustUnnamed_4,
    pub counter: C2RustUnnamed_3,
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
pub struct C2RustUnnamed_3 {
    pub value: value_t,
    pub previous: value_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
pub type brubeck_metric_t = libc::c_uint;
pub const BRUBECK_MT_INTERNAL_STATS: brubeck_metric_t = 5;
pub const BRUBECK_MT_TIMER: brubeck_metric_t = 4;
pub const BRUBECK_MT_HISTO: brubeck_metric_t = 3;
pub const BRUBECK_MT_COUNTER: brubeck_metric_t = 2;
pub const BRUBECK_MT_METER: brubeck_metric_t = 1;
pub const BRUBECK_MT_GAUGE: brubeck_metric_t = 0;
pub type brubeck_metric_mod_t = libc::c_uint;
pub const BRUBECK_MOD_RELATIVE_VALUE: brubeck_metric_mod_t = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_statsd_msg {
    pub key: *mut libc::c_char,
    pub key_len: uint16_t,
    pub type_0: uint16_t,
    pub value: value_t,
    pub sample_freq: value_t,
    pub modifiers: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_statsd {
    pub sampler: brubeck_sampler,
    pub workers: *mut pthread_t,
    pub worker_count: libc::c_uint,
    pub mmsg_count: libc::c_uint,
    pub scale_timers_by: libc::c_double,
}
#[inline]
unsafe extern "C" fn xmalloc(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = malloc(size);
    if (ptr == 0 as *mut libc::c_void) as libc::c_int as libc::c_long != 0 {
        fprintf(stderr, b"[FATAL]: oom\n\0" as *const u8 as *const libc::c_char);
        gh_log_die();
    }
    return ptr;
}
unsafe extern "C" fn statsd_run_recvmmsg(
    mut statsd: *mut brubeck_statsd,
    mut sock: libc::c_int,
) {
    let SIM_PACKETS: libc::c_uint = (*statsd).mmsg_count;
    let mut server: *mut brubeck_server = (*statsd).sampler.server;
    let mut i: libc::c_uint = 0;
    let vla = SIM_PACKETS as usize;
    let mut iovecs: Vec::<iovec> = ::std::vec::from_elem(
        iovec {
            iov_base: 0 as *mut libc::c_void,
            iov_len: 0,
        },
        vla,
    );
    let vla_0 = SIM_PACKETS as usize;
    let mut msgs: Vec::<mmsghdr> = ::std::vec::from_elem(
        mmsghdr {
            msg_hdr: msghdr {
                msg_name: 0 as *mut libc::c_void,
                msg_namelen: 0,
                msg_iov: 0 as *mut iovec,
                msg_iovlen: 0,
                msg_control: 0 as *mut libc::c_void,
                msg_controllen: 0,
                msg_flags: 0,
            },
            msg_len: 0,
        },
        vla_0,
    );
    memset(
        msgs.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (vla_0 * ::std::mem::size_of::<mmsghdr>()) as libc::c_ulong,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i < SIM_PACKETS {
        let ref mut fresh0 = (*iovecs.as_mut_ptr().offset(i as isize)).iov_base;
        *fresh0 = xmalloc(8192 as libc::c_int as size_t);
        (*iovecs.as_mut_ptr().offset(i as isize))
            .iov_len = (8192 as libc::c_int - 1 as libc::c_int) as size_t;
        let ref mut fresh1 = (*msgs.as_mut_ptr().offset(i as isize)).msg_hdr.msg_iov;
        *fresh1 = &mut *iovecs.as_mut_ptr().offset(i as isize) as *mut iovec;
        (*msgs.as_mut_ptr().offset(i as isize))
            .msg_hdr
            .msg_iovlen = 1 as libc::c_int as size_t;
        i = i.wrapping_add(1);
        i;
    }
    gh_log_write(
        b"instance=%s sampler=statsd event=worker_online syscall=recvmmsg socket=%d\n\0"
            as *const u8 as *const libc::c_char,
        gh_log_instance(),
        sock,
    );
    loop {
        let mut res: libc::c_int = recvmmsg(
            sock,
            msgs.as_mut_ptr(),
            SIM_PACKETS,
            MSG_WAITFORONE as libc::c_int,
            0 as *mut timespec,
        );
        if res < 0 as libc::c_int {
            if *__errno_location() == 11 as libc::c_int
                || *__errno_location() == 4 as libc::c_int
            {
                continue;
            }
            gh_log_write(
                b"instance=%s sampler=statsd event=failed_read errno=%d msg=\"%s\"\n\0"
                    as *const u8 as *const libc::c_char,
                gh_log_instance(),
                *__errno_location(),
                strerror(*__errno_location()),
            );
            let fresh2 = &mut (*server).internal_stats.live.errors;
            let fresh3 = 1 as libc::c_int as uint32_t;
            ::std::intrinsics::atomic_xadd_seqcst(fresh2, fresh3) + fresh3;
        } else {
            let fresh4 = &mut (*statsd).sampler.inflow;
            let fresh5 = res as size_t;
            ::std::intrinsics::atomic_xadd_seqcst(fresh4, fresh5) + fresh5;
            i = 0 as libc::c_int as libc::c_uint;
            while i < res as libc::c_uint {
                let mut buf: *mut libc::c_char = (*(*msgs
                    .as_mut_ptr()
                    .offset(i as isize))
                    .msg_hdr
                    .msg_iov)
                    .iov_base as *mut libc::c_char;
                let mut end: *mut libc::c_char = buf
                    .offset((*msgs.as_mut_ptr().offset(i as isize)).msg_len as isize);
                brubeck_statsd_packet_parse(server, buf, end, (*statsd).scale_timers_by);
                i = i.wrapping_add(1);
                i;
            }
        }
    };
}
unsafe extern "C" fn statsd_run_recvmsg(
    mut statsd: *mut brubeck_statsd,
    mut sock: libc::c_int,
) {
    let mut server: *mut brubeck_server = (*statsd).sampler.server;
    let mut buffer: *mut libc::c_char = xmalloc(8192 as libc::c_int as size_t)
        as *mut libc::c_char;
    let mut reporter: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut reporter_len: socklen_t = ::std::mem::size_of::<sockaddr_in>()
        as libc::c_ulong as socklen_t;
    memset(
        &mut reporter as *mut sockaddr_in as *mut libc::c_void,
        0 as libc::c_int,
        reporter_len as libc::c_ulong,
    );
    gh_log_write(
        b"instance=%s sampler=statsd event=worker_online syscall=recvmsg socket=%d\n\0"
            as *const u8 as *const libc::c_char,
        gh_log_instance(),
        sock,
    );
    loop {
        let mut res: libc::c_int = recvfrom(
            sock,
            buffer as *mut libc::c_void,
            (8192 as libc::c_int - 1 as libc::c_int) as size_t,
            0 as libc::c_int,
            __SOCKADDR_ARG {
                __sockaddr__: &mut reporter as *mut sockaddr_in as *mut sockaddr,
            },
            &mut reporter_len,
        ) as libc::c_int;
        if res < 0 as libc::c_int {
            if *__errno_location() == 11 as libc::c_int
                || *__errno_location() == 4 as libc::c_int
            {
                continue;
            }
            gh_log_write(
                b"instance=%s sampler=statsd event=failed_read from=%s errno=%d msg=\"%s\"\n\0"
                    as *const u8 as *const libc::c_char,
                gh_log_instance(),
                inet_ntoa(reporter.sin_addr),
                *__errno_location(),
                strerror(*__errno_location()),
            );
            let fresh6 = &mut (*server).internal_stats.live.errors;
            let fresh7 = 1 as libc::c_int as uint32_t;
            ::std::intrinsics::atomic_xadd_seqcst(fresh6, fresh7) + fresh7;
        } else {
            let fresh8 = &mut (*statsd).sampler.inflow;
            let fresh9 = 1 as libc::c_int as size_t;
            ::std::intrinsics::atomic_xadd_seqcst(fresh8, fresh9) + fresh9;
            brubeck_statsd_packet_parse(
                server,
                buffer,
                buffer.offset(res as isize),
                (*statsd).scale_timers_by,
            );
        }
    };
}
#[inline]
unsafe extern "C" fn parse_float(
    mut buffer: *mut libc::c_char,
    mut result: *mut value_t,
    mut mods: *mut uint8_t,
) -> *mut libc::c_char {
    let mut negative: libc::c_int = 0 as libc::c_int;
    let mut start: *mut libc::c_char = buffer;
    let mut value: value_t = 0.0f64;
    if *buffer as libc::c_int == '-' as i32 {
        buffer = buffer.offset(1);
        buffer;
        negative = 1 as libc::c_int;
        *mods = (*mods as libc::c_int | BRUBECK_MOD_RELATIVE_VALUE as libc::c_int)
            as uint8_t;
    } else if *buffer as libc::c_int == '+' as i32 {
        buffer = buffer.offset(1);
        buffer;
        *mods = (*mods as libc::c_int | BRUBECK_MOD_RELATIVE_VALUE as libc::c_int)
            as uint8_t;
    }
    while *buffer as libc::c_int >= '0' as i32 && *buffer as libc::c_int <= '9' as i32 {
        value = value * 10.0f64
            + (*buffer as libc::c_int - '0' as i32) as libc::c_double;
        buffer = buffer.offset(1);
        buffer;
    }
    if *buffer as libc::c_int == '.' as i32 {
        let mut f: libc::c_double = 0.0f64;
        let mut n: libc::c_int = 0 as libc::c_int;
        buffer = buffer.offset(1);
        buffer;
        while *buffer as libc::c_int >= '0' as i32
            && *buffer as libc::c_int <= '9' as i32
        {
            f = f * 10.0f64 + (*buffer as libc::c_int - '0' as i32) as libc::c_double;
            buffer = buffer.offset(1);
            buffer;
            n += 1;
            n;
        }
        value += f / pow(10.0f64, n as libc::c_double);
    }
    if negative != 0 {
        value = -value;
    }
    if (*buffer as libc::c_int == 'e' as i32 || *buffer as libc::c_int == 'E' as i32)
        as libc::c_int as libc::c_long != 0
    {
        value = strtod(start, &mut buffer);
    }
    *result = value;
    return buffer;
}
pub unsafe extern "C" fn brubeck_statsd_msg_parse(
    mut msg: *mut brubeck_statsd_msg,
    mut buffer: *mut libc::c_char,
    mut end: *mut libc::c_char,
    scale_timers_by: libc::c_double,
) -> libc::c_int {
    *end = '\0' as i32 as libc::c_char;
    (*msg).key = buffer;
    (*msg).key_len = 0 as libc::c_int as uint16_t;
    while *buffer as libc::c_int != ':' as i32 && *buffer as libc::c_int != '\0' as i32 {
        buffer = buffer.offset(1);
        buffer;
    }
    if *buffer as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    (*msg).key_len = buffer.offset_from((*msg).key) as libc::c_long as uint16_t;
    let fresh10 = buffer;
    buffer = buffer.offset(1);
    *fresh10 = '\0' as i32 as libc::c_char;
    if *((*msg).key).offset(((*msg).key_len as libc::c_int - 1 as libc::c_int) as isize)
        as libc::c_int == '.' as i32
    {
        return -(1 as libc::c_int);
    }
    (*msg).modifiers = 0 as libc::c_int as uint8_t;
    buffer = parse_float(buffer, &mut (*msg).value, &mut (*msg).modifiers);
    if *buffer as libc::c_int != '|' as i32 {
        return -(1 as libc::c_int);
    }
    buffer = buffer.offset(1);
    buffer;
    let mut current_block_26: u64;
    match *buffer as libc::c_int {
        103 => {
            (*msg).type_0 = BRUBECK_MT_GAUGE as libc::c_int as uint16_t;
            current_block_26 = 11194104282611034094;
        }
        99 => {
            (*msg).type_0 = BRUBECK_MT_METER as libc::c_int as uint16_t;
            current_block_26 = 11194104282611034094;
        }
        67 => {
            (*msg).type_0 = BRUBECK_MT_COUNTER as libc::c_int as uint16_t;
            current_block_26 = 11194104282611034094;
        }
        104 => {
            (*msg).type_0 = BRUBECK_MT_HISTO as libc::c_int as uint16_t;
            current_block_26 = 11194104282611034094;
        }
        109 => {
            buffer = buffer.offset(1);
            buffer;
            if *buffer as libc::c_int == 's' as i32 {
                (*msg).type_0 = BRUBECK_MT_TIMER as libc::c_int as uint16_t;
                (*msg).value *= scale_timers_by;
                current_block_26 = 11194104282611034094;
            } else {
                current_block_26 = 15131786372230249361;
            }
        }
        _ => {
            current_block_26 = 15131786372230249361;
        }
    }
    match current_block_26 {
        11194104282611034094 => {}
        _ => return -(1 as libc::c_int),
    }
    buffer = buffer.offset(1);
    buffer;
    if *buffer.offset(0 as libc::c_int as isize) as libc::c_int == '|' as i32
        && *buffer.offset(1 as libc::c_int as isize) as libc::c_int == '@' as i32
    {
        let mut sample_rate: libc::c_double = 0.;
        let mut dummy: uint8_t = 0;
        buffer = parse_float(
            buffer.offset(2 as libc::c_int as isize),
            &mut sample_rate,
            &mut dummy,
        );
        if sample_rate <= 0.0f64 || sample_rate > 1.0f64 {
            return -(1 as libc::c_int);
        }
        (*msg).sample_freq = 1.0f64 / sample_rate;
    } else {
        (*msg).sample_freq = 1.0f64;
    }
    if *buffer.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || *buffer.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32
            && *buffer.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn brubeck_statsd_packet_parse(
    mut server: *mut brubeck_server,
    mut buffer: *mut libc::c_char,
    mut end: *mut libc::c_char,
    scale_timers_by: libc::c_double,
) {
    let mut msg: brubeck_statsd_msg = brubeck_statsd_msg {
        key: 0 as *mut libc::c_char,
        key_len: 0,
        type_0: 0,
        value: 0.,
        sample_freq: 0.,
        modifiers: 0,
    };
    let mut metric: *mut brubeck_metric = 0 as *mut brubeck_metric;
    while buffer < end {
        let mut stat_end: *mut libc::c_char = memchr(
            buffer as *const libc::c_void,
            '\n' as i32,
            end.offset_from(buffer) as libc::c_long as libc::c_ulong,
        ) as *mut libc::c_char;
        if stat_end.is_null() {
            stat_end = end;
        }
        if brubeck_statsd_msg_parse(&mut msg, buffer, stat_end, scale_timers_by)
            < 0 as libc::c_int
        {
            let fresh11 = &mut (*server).internal_stats.live.errors;
            let fresh12 = 1 as libc::c_int as uint32_t;
            ::std::intrinsics::atomic_xadd_seqcst(fresh11, fresh12) + fresh12;
            gh_log_write(
                b"instance=%s sampler=statsd event=packet_drop\n\0" as *const u8
                    as *const libc::c_char,
                gh_log_instance(),
            );
        } else {
            let fresh13 = &mut (*server).internal_stats.live.metrics;
            let fresh14 = 1 as libc::c_int as uint32_t;
            ::std::intrinsics::atomic_xadd_seqcst(fresh13, fresh14) + fresh14;
            metric = brubeck_metric_find(
                server,
                msg.key,
                msg.key_len as size_t,
                msg.type_0 as uint8_t,
            );
            if !metric.is_null() {
                brubeck_metric_record(metric, msg.value, msg.sample_freq, msg.modifiers);
            }
        }
        buffer = stat_end.offset(1 as libc::c_int as isize);
    }
}
unsafe extern "C" fn statsd__thread(mut _in: *mut libc::c_void) -> *mut libc::c_void {
    let mut statsd: *mut brubeck_statsd = _in as *mut brubeck_statsd;
    let mut sock: libc::c_int = (*statsd).sampler.in_sock;
    if sock < 0 as libc::c_int {
        sock = brubeck_sampler_socket(&mut (*statsd).sampler, 1 as libc::c_int);
    }
    if (*statsd).mmsg_count > 1 as libc::c_int as libc::c_uint {
        statsd_run_recvmmsg(statsd, sock);
        return 0 as *mut libc::c_void;
    }
    statsd_run_recvmsg(statsd, sock);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn run_worker_threads(mut statsd: *mut brubeck_statsd) {
    let mut i: libc::c_uint = 0;
    (*statsd)
        .workers = xmalloc(
        ((*statsd).worker_count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<pthread_t>() as libc::c_ulong),
    ) as *mut pthread_t;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*statsd).worker_count {
        if pthread_create(
            &mut *((*statsd).workers).offset(i as isize),
            0 as *const pthread_attr_t,
            Some(
                statsd__thread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            statsd as *mut libc::c_void,
        ) != 0 as libc::c_int
        {
            fprintf(
                stderr,
                b"[FATAL]: failed to start sampler thread\n\0" as *const u8
                    as *const libc::c_char,
            );
            gh_log_die();
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn shutdown_sampler(mut sampler: *mut brubeck_sampler) {
    let mut statsd: *mut brubeck_statsd = sampler as *mut brubeck_statsd;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*statsd).worker_count as libc::c_ulong {
        pthread_cancel(*((*statsd).workers).offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn brubeck_statsd_new(
    mut server: *mut brubeck_server,
    mut settings: *mut json_t,
) -> *mut brubeck_sampler {
    let mut std: *mut brubeck_statsd = xmalloc(
        ::std::mem::size_of::<brubeck_statsd>() as libc::c_ulong,
    ) as *mut brubeck_statsd;
    let mut address: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut port: libc::c_int = 0;
    let mut multisock: libc::c_int = 0 as libc::c_int;
    (*std).sampler.type_0 = BRUBECK_SAMPLER_STATSD;
    (*std)
        .sampler
        .shutdown = Some(
        shutdown_sampler as unsafe extern "C" fn(*mut brubeck_sampler) -> (),
    );
    (*std).sampler.in_sock = -(1 as libc::c_int);
    (*std).worker_count = 4 as libc::c_int as libc::c_uint;
    (*std).mmsg_count = 1 as libc::c_int as libc::c_uint;
    (*std).scale_timers_by = 1.0f64;
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
        b"{s:s, s:i, s?:i, s?:i, s?:b, s?:F}\0" as *const u8 as *const libc::c_char,
        b"address\0" as *const u8 as *const libc::c_char,
        &mut address as *mut *mut libc::c_char,
        b"port\0" as *const u8 as *const libc::c_char,
        &mut port as *mut libc::c_int,
        b"workers\0" as *const u8 as *const libc::c_char,
        &mut (*std).worker_count as *mut libc::c_uint,
        b"multimsg\0" as *const u8 as *const libc::c_char,
        &mut (*std).mmsg_count as *mut libc::c_uint,
        b"multisock\0" as *const u8 as *const libc::c_char,
        &mut multisock as *mut libc::c_int,
        b"scale_timers_by\0" as *const u8 as *const libc::c_char,
        &mut (*std).scale_timers_by as *mut libc::c_double,
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"[FATAL]: config error: %s\n\0" as *const u8 as *const libc::c_char,
            (_error_j.text).as_mut_ptr(),
        );
        gh_log_die();
    }
    brubeck_sampler_init_inet(&mut (*std).sampler, server, address, port);
    if multisock == 0 {
        (*std)
            .sampler
            .in_sock = brubeck_sampler_socket(&mut (*std).sampler, 0 as libc::c_int);
    }
    run_worker_threads(std);
    return &mut (*std).sampler;
}
