use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type brubeck_tags_t;
    pub type brubeck_hashtable_t;
    pub type rd_kafka_s;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn signalfd(
        __fd: libc::c_int,
        __mask: *const sigset_t,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn timerfd_create(__clock_id: __clockid_t, __flags: libc::c_int) -> libc::c_int;
    fn timerfd_settime(
        __ufd: libc::c_int,
        __flags: libc::c_int,
        __utmr: *const itimerspec,
        __otmr: *mut itimerspec,
    ) -> libc::c_int;
    fn pthread_cancel(__th: pthread_t) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn __errno_location() -> *mut libc::c_int;
    fn json_object_get(object: *const json_t, key: *const libc::c_char) -> *mut json_t;
    fn json_array_size(array: *const json_t) -> size_t;
    fn json_array_get(array: *const json_t, index: size_t) -> *mut json_t;
    fn json_string_value(string: *const json_t) -> *const libc::c_char;
    fn json_unpack_ex(
        root: *mut json_t,
        error: *mut json_error_t,
        flags: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn json_load_file(
        path: *const libc::c_char,
        flags: size_t,
        error: *mut json_error_t,
    ) -> *mut json_t;
    fn brubeck_carbon_new(
        server: *mut brubeck_server,
        settings: *mut json_t,
        shard_n: libc::c_int,
    ) -> *mut brubeck_backend;
    fn setproctitle(prog: *const libc::c_char, txt: *const libc::c_char);
    fn brubeck_internal__init(server: *mut brubeck_server);
    fn brubeck_http_endpoint_init(
        server: *mut brubeck_server,
        listen_on: *const libc::c_char,
    );
    fn brubeck_tags_create(size: uint64_t) -> *mut brubeck_tags_t;
    fn brubeck_slab_init(slab: *mut brubeck_slab);
    fn brubeck_statsd_new(
        server: *mut brubeck_server,
        settings: *mut json_t,
    ) -> *mut brubeck_sampler;
    fn gh_log_write(message: *const libc::c_char, _: ...);
    fn gh_log_die() -> !;
    fn gh_log_reopen();
    fn gh_log_set_instance(instance: *const libc::c_char);
    fn gh_log_instance() -> *const libc::c_char;
    fn brubeck_hashtable_foreach(
        ht: *mut brubeck_hashtable_t,
        callback: Option::<
            unsafe extern "C" fn(*mut brubeck_metric, *mut libc::c_void) -> (),
        >,
        payload: *mut libc::c_void,
    );
    fn brubeck_hashtable_new(size: uint64_t) -> *mut brubeck_hashtable_t;
    fn brubeck_kafka_new(
        server: *mut brubeck_server,
        settings: *mut json_t,
        shard_n: libc::c_int,
    ) -> *mut brubeck_backend;
}
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type size_t = libc::c_ulong;
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
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct signalfd_siginfo {
    pub ssi_signo: uint32_t,
    pub ssi_errno: int32_t,
    pub ssi_code: int32_t,
    pub ssi_pid: uint32_t,
    pub ssi_uid: uint32_t,
    pub ssi_fd: int32_t,
    pub ssi_tid: uint32_t,
    pub ssi_band: uint32_t,
    pub ssi_overrun: uint32_t,
    pub ssi_trapno: uint32_t,
    pub ssi_status: int32_t,
    pub ssi_int: int32_t,
    pub ssi_ptr: uint64_t,
    pub ssi_utime: uint64_t,
    pub ssi_stime: uint64_t,
    pub ssi_addr: uint64_t,
    pub ssi_addr_lsb: uint16_t,
    pub __pad2: uint16_t,
    pub ssi_syscall: int32_t,
    pub ssi_call_addr: uint64_t,
    pub ssi_arch: uint32_t,
    pub __pad: [uint8_t; 28],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}
pub type ssize_t = __ssize_t;
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
    pub live: C2RustUnnamed,
    pub sample: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
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
    pub as_0: C2RustUnnamed_0,
    pub key: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub gauge: C2RustUnnamed_2,
    pub meter: C2RustUnnamed_2,
    pub counter: C2RustUnnamed_1,
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
pub struct C2RustUnnamed_1 {
    pub value: value_t,
    pub previous: value_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub type rd_kafka_t = rd_kafka_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_kafka_document {
    pub json: *mut json_t,
    pub is_dirty: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_kafka {
    pub backend: brubeck_backend,
    pub rk: *mut rd_kafka_t,
    pub connected: bool,
    pub topic: *const libc::c_char,
    pub tag_subdocument: *const libc::c_char,
    pub bytes_sent: size_t,
    pub documents: *mut *mut brubeck_kafka_document,
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
unsafe extern "C" fn update_flows(mut server: *mut brubeck_server) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*server).active_samplers {
        let mut sampler: *mut brubeck_sampler = (*server).samplers[i as usize];
        (*sampler).current_flow = (*sampler).inflow;
        (*sampler).inflow = 0 as libc::c_int as size_t;
        i += 1;
        i;
    }
}
unsafe extern "C" fn update_proctitle(mut server: *mut brubeck_server) {
    if (*server).set_proctitle {
        static mut size_suffix: [*const libc::c_char; 7] = [
            b"b\0" as *const u8 as *const libc::c_char,
            b"kb\0" as *const u8 as *const libc::c_char,
            b"mb\0" as *const u8 as *const libc::c_char,
            b"gb\0" as *const u8 as *const libc::c_char,
            b"tb\0" as *const u8 as *const libc::c_char,
            b"pb\0" as *const u8 as *const libc::c_char,
            b"eb\0" as *const u8 as *const libc::c_char,
        ];
        let mut buf: [libc::c_char; 2048] = [0; 2048];
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut pos: libc::c_int = 0 as libc::c_int;
        pos
            += snprintf(
                buf.as_mut_ptr().offset(pos as isize),
                (::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
                    .wrapping_sub(pos as libc::c_ulong),
                b"[%s] [ \xE2\x86\x91\0" as *const u8 as *const libc::c_char,
                (*server).config_name,
            );
        let mut bytes_sent: libc::c_double = 0.0f64;
        let mut connected: bool = 0 as libc::c_int != 0;
        i = 0 as libc::c_int;
        while i < (*server).active_backends {
            let mut backend: *mut brubeck_backend = (*server).backends[i as usize];
            if (*backend).type_0 as libc::c_uint
                == BRUBECK_BACKEND_CARBON as libc::c_int as libc::c_uint
            {
                let mut carbon: *mut brubeck_carbon = backend as *mut brubeck_carbon;
                bytes_sent += (*carbon).bytes_sent as libc::c_double;
                connected = connected as libc::c_int != 0
                    || (*carbon).out_sock >= 0 as libc::c_int;
            } else if (*backend).type_0 as libc::c_uint
                == BRUBECK_BACKEND_KAFKA as libc::c_int as libc::c_uint
            {
                let mut kafka: *mut brubeck_kafka = backend as *mut brubeck_kafka;
                bytes_sent += (*kafka).bytes_sent as libc::c_double;
                connected = connected as libc::c_int != 0
                    || (*kafka).connected as libc::c_int != 0;
            }
            i += 1;
            i;
        }
        j = 0 as libc::c_int;
        while j < 7 as libc::c_int && bytes_sent >= 1024.0f64 {
            bytes_sent /= 1024.0f64;
            j += 1;
            j;
        }
        pos
            += snprintf(
                buf.as_mut_ptr().offset(pos as isize),
                (::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
                    .wrapping_sub(pos as libc::c_ulong),
                b"%s #%d %.1f%s%s\0" as *const u8 as *const libc::c_char,
                if i > 0 as libc::c_int {
                    b",\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                i + 1 as libc::c_int,
                bytes_sent,
                size_suffix[j as usize],
                if connected as libc::c_int != 0 {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    b" (dc)\0" as *const u8 as *const libc::c_char
                },
            );
        pos
            += snprintf(
                buf.as_mut_ptr().offset(pos as isize),
                (::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
                    .wrapping_sub(pos as libc::c_ulong),
                b" ] [ \xE2\x86\x93\0" as *const u8 as *const libc::c_char,
            );
        i = 0 as libc::c_int;
        while i < (*server).active_samplers {
            let mut sampler: *mut brubeck_sampler = (*server).samplers[i as usize];
            pos
                += snprintf(
                    buf.as_mut_ptr().offset(pos as isize),
                    (::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
                        .wrapping_sub(pos as libc::c_ulong),
                    b"%s :%d %d/s\0" as *const u8 as *const libc::c_char,
                    if i > 0 as libc::c_int {
                        b",\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                    __bswap_16((*sampler).addr.sin_port) as libc::c_int,
                    (*sampler).current_flow as libc::c_int,
                );
            i += 1;
            i;
        }
        pos
            += snprintf(
                buf.as_mut_ptr().offset(pos as isize),
                (::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
                    .wrapping_sub(pos as libc::c_ulong),
                b" ]\0" as *const u8 as *const libc::c_char,
            );
        setproctitle(b"brubeck\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
    }
}
unsafe extern "C" fn dump_metric(
    mut mt: *mut brubeck_metric,
    mut out_file: *mut libc::c_void,
) {
    static mut METRIC_NAMES: [*const libc::c_char; 6] = [
        b"g\0" as *const u8 as *const libc::c_char,
        b"c\0" as *const u8 as *const libc::c_char,
        b"C\0" as *const u8 as *const libc::c_char,
        b"h\0" as *const u8 as *const libc::c_char,
        b"ms\0" as *const u8 as *const libc::c_char,
        b"internal\0" as *const u8 as *const libc::c_char,
    ];
    fprintf(
        out_file as *mut FILE,
        b"%s|%s\n\0" as *const u8 as *const libc::c_char,
        ((*mt).key).as_mut_ptr(),
        METRIC_NAMES[(*mt).type_0 as usize],
    );
}
unsafe extern "C" fn dump_all_metrics(mut server: *mut brubeck_server) {
    let mut dump: *mut FILE = 0 as *mut FILE;
    gh_log_write(
        b"instance=%s event=dump_metrics\n\0" as *const u8 as *const libc::c_char,
        gh_log_instance(),
    );
    if !((*server).dump_path).is_null() {
        dump = fopen((*server).dump_path, b"w+\0" as *const u8 as *const libc::c_char);
    }
    if dump.is_null() {
        gh_log_write(
            b"instance=%s event=dump_failed errno=%d msg=\"%s\"\n\0" as *const u8
                as *const libc::c_char,
            gh_log_instance(),
            *__errno_location(),
            strerror(*__errno_location()),
        );
        return;
    }
    brubeck_hashtable_foreach(
        (*server).metrics,
        Some(
            dump_metric
                as unsafe extern "C" fn(*mut brubeck_metric, *mut libc::c_void) -> (),
        ),
        dump as *mut libc::c_void,
    );
    fclose(dump);
}
unsafe extern "C" fn load_backends(
    mut server: *mut brubeck_server,
    mut backends: *mut json_t,
) {
    let mut idx: size_t = 0;
    let mut b: *mut json_t = 0 as *mut json_t;
    idx = 0 as libc::c_int as size_t;
    while idx < json_array_size(backends)
        && {
            b = json_array_get(backends, idx);
            !b.is_null()
        }
    {
        let mut type_0: *const libc::c_char = json_string_value(
            json_object_get(b, b"type\0" as *const u8 as *const libc::c_char),
        );
        let mut backend: *mut brubeck_backend = 0 as *mut brubeck_backend;
        if !type_0.is_null()
            && strcmp(type_0, b"carbon\0" as *const u8 as *const libc::c_char) == 0
        {
            backend = brubeck_carbon_new(server, b, (*server).active_backends);
            let fresh0 = (*server).active_backends;
            (*server).active_backends = (*server).active_backends + 1;
            (*server).backends[fresh0 as usize] = backend;
        } else if !type_0.is_null()
            && strcmp(type_0, b"kafka\0" as *const u8 as *const libc::c_char) == 0
        {
            backend = brubeck_kafka_new(server, b, (*server).active_backends);
            let fresh1 = (*server).active_backends;
            (*server).active_backends = (*server).active_backends + 1;
            (*server).backends[fresh1 as usize] = backend;
        } else {
            gh_log_write(
                b"instance=%s backend=%s event=invalid_backend\n\0" as *const u8
                    as *const libc::c_char,
                gh_log_instance(),
                type_0,
            );
        }
        idx = idx.wrapping_add(1);
        idx;
    }
    if (*server).active_backends == 0 as libc::c_int {
        fprintf(
            stderr,
            b"[FATAL]: no backends were loaded\n\0" as *const u8 as *const libc::c_char,
        );
        gh_log_die();
    }
}
unsafe extern "C" fn load_samplers(
    mut server: *mut brubeck_server,
    mut samplers: *mut json_t,
) {
    let mut idx: size_t = 0;
    let mut s: *mut json_t = 0 as *mut json_t;
    idx = 0 as libc::c_int as size_t;
    while idx < json_array_size(samplers)
        && {
            s = json_array_get(samplers, idx);
            !s.is_null()
        }
    {
        let mut type_0: *const libc::c_char = json_string_value(
            json_object_get(s, b"type\0" as *const u8 as *const libc::c_char),
        );
        if !type_0.is_null()
            && strcmp(type_0, b"statsd\0" as *const u8 as *const libc::c_char) == 0
        {
            let fresh2 = (*server).active_samplers;
            (*server).active_samplers = (*server).active_samplers + 1;
            (*server).samplers[fresh2 as usize] = brubeck_statsd_new(server, s);
        } else {
            gh_log_write(
                b"instance=%s sampler=%s event=invalid_sampler\n\0" as *const u8
                    as *const libc::c_char,
                gh_log_instance(),
                type_0,
            );
        }
        idx = idx.wrapping_add(1);
        idx;
    }
}
unsafe extern "C" fn load_timerfd(mut interval: libc::c_int) -> libc::c_int {
    let mut timer: itimerspec = itimerspec {
        it_interval: timespec { tv_sec: 0, tv_nsec: 0 },
        it_value: timespec { tv_sec: 0, tv_nsec: 0 },
    };
    let mut timerfd: libc::c_int = timerfd_create(1 as libc::c_int, 0 as libc::c_int);
    if timerfd < 0 as libc::c_int {
        fprintf(
            stderr,
            b"[FATAL]: failed to create timer\n\0" as *const u8 as *const libc::c_char,
        );
        gh_log_die();
    }
    memset(
        &mut timer as *mut itimerspec as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<itimerspec>() as libc::c_ulong,
    );
    timer.it_value.tv_sec = interval as __time_t;
    timer.it_value.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
    timer.it_interval.tv_sec = interval as __time_t;
    timer.it_interval.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
    if timerfd_settime(timerfd, 0 as libc::c_int, &mut timer, 0 as *mut itimerspec)
        < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"[FATAL]: failed to set system timer\n\0" as *const u8
                as *const libc::c_char,
        );
        gh_log_die();
    }
    return timerfd;
}
unsafe extern "C" fn load_signalfd() -> libc::c_int {
    let mut mask: sigset_t = sigset_t { __val: [0; 16] };
    sigemptyset(&mut mask);
    sigaddset(&mut mask, 2 as libc::c_int);
    sigaddset(&mut mask, 15 as libc::c_int);
    sigaddset(&mut mask, 1 as libc::c_int);
    sigaddset(&mut mask, 12 as libc::c_int);
    if sigprocmask(0 as libc::c_int, &mut mask, 0 as *mut sigset_t)
        == -(1 as libc::c_int)
    {
        fprintf(
            stderr,
            b"[FATAL]: failed to sigprocmask the needed signals\n\0" as *const u8
                as *const libc::c_char,
        );
        gh_log_die();
    }
    return signalfd(-(1 as libc::c_int), &mut mask, 0 as libc::c_int);
}
unsafe extern "C" fn get_config_name(
    mut full_path: *const libc::c_char,
) -> *mut libc::c_char {
    let mut filename: *const libc::c_char = strrchr(full_path, '/' as i32);
    let mut config_name: *mut libc::c_char = strdup(
        if !filename.is_null() {
            filename.offset(1 as libc::c_int as isize)
        } else {
            full_path
        },
    );
    let mut ext: *mut libc::c_char = strrchr(config_name, '.' as i32);
    if !ext.is_null() {
        *ext = '\0' as i32 as libc::c_char;
    }
    return config_name;
}
unsafe extern "C" fn load_config(
    mut server: *mut brubeck_server,
    mut path: *const libc::c_char,
) {
    let mut error: json_error_t = json_error_t {
        line: 0,
        column: 0,
        position: 0,
        source: [0; 80],
        text: [0; 160],
    };
    let mut capacity: libc::c_int = 0;
    let mut backends: *mut json_t = 0 as *mut json_t;
    let mut samplers: *mut json_t = 0 as *mut json_t;
    let mut http: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tag_capacity: libc::c_int = 0 as libc::c_int;
    (*server).name = b"brubeck\0" as *const u8 as *const libc::c_char;
    (*server).config_name = get_config_name(path);
    (*server).dump_path = 0 as *const libc::c_char;
    (*server).config = json_load_file(path, 0 as libc::c_int as size_t, &mut error);
    if ((*server).config).is_null() {
        fprintf(
            stderr,
            b"[FATAL]: failed to load config file, %s (%s:%d:%d)\n\0" as *const u8
                as *const libc::c_char,
            (error.text).as_mut_ptr(),
            (error.source).as_mut_ptr(),
            error.line,
            error.column,
        );
        gh_log_die();
    }
    let mut _error_j: json_error_t = json_error_t {
        line: 0,
        column: 0,
        position: 0,
        source: [0; 80],
        text: [0; 160],
    };
    if json_unpack_ex(
        (*server).config,
        &mut _error_j as *mut json_error_t,
        0 as libc::c_int as size_t,
        b"{s?:s, s:s, s:i, s?:i, s:o, s:o, s?:s}\0" as *const u8 as *const libc::c_char,
        b"server_name\0" as *const u8 as *const libc::c_char,
        &mut (*server).name as *mut *const libc::c_char,
        b"dumpfile\0" as *const u8 as *const libc::c_char,
        &mut (*server).dump_path as *mut *const libc::c_char,
        b"capacity\0" as *const u8 as *const libc::c_char,
        &mut capacity as *mut libc::c_int,
        b"tag_capacity\0" as *const u8 as *const libc::c_char,
        &mut tag_capacity as *mut libc::c_int,
        b"backends\0" as *const u8 as *const libc::c_char,
        &mut backends as *mut *mut json_t,
        b"samplers\0" as *const u8 as *const libc::c_char,
        &mut samplers as *mut *mut json_t,
        b"http\0" as *const u8 as *const libc::c_char,
        &mut http as *mut *mut libc::c_char,
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"[FATAL]: config error: %s\n\0" as *const u8 as *const libc::c_char,
            (_error_j.text).as_mut_ptr(),
        );
        gh_log_die();
    }
    gh_log_set_instance((*server).name);
    (*server)
        .metrics = brubeck_hashtable_new(((1 as libc::c_int) << capacity) as uint64_t);
    if ((*server).metrics).is_null() {
        fprintf(
            stderr,
            b"[FATAL]: failed to initialize hash table (size: %lu)\n\0" as *const u8
                as *const libc::c_char,
            (1 as libc::c_ulong) << capacity,
        );
        gh_log_die();
    }
    if tag_capacity != 0 {
        (*server)
            .tags = brubeck_tags_create(
            ((1 as libc::c_int) << tag_capacity) as uint64_t,
        );
        if ((*server).tags).is_null() {
            fprintf(
                stderr,
                b"[FATAL]: failed to initialize tags (size: %lu)\n\0" as *const u8
                    as *const libc::c_char,
                (1 as libc::c_ulong) << tag_capacity,
            );
            gh_log_die();
        }
        gh_log_write(
            b"instance=%s event=tagging_initialized\n\0" as *const u8
                as *const libc::c_char,
            gh_log_instance(),
        );
    }
    load_backends(server, backends);
    load_samplers(server, samplers);
    if !http.is_null() {
        brubeck_http_endpoint_init(server, http);
    }
}
pub unsafe extern "C" fn brubeck_server_init(
    mut server: *mut brubeck_server,
    mut config: *const libc::c_char,
) {
    signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    (*server).fd_signal = load_signalfd();
    (*server).fd_update = load_timerfd(1 as libc::c_int);
    brubeck_slab_init(&mut (*server).slab);
    load_config(server, config);
    brubeck_internal__init(server);
}
unsafe extern "C" fn timer_elapsed(mut fd: *mut pollfd) -> libc::c_int {
    if (*fd).revents as libc::c_int & 0x1 as libc::c_int != 0 {
        let mut timer: uint64_t = 0;
        let mut s: libc::c_int = read(
            (*fd).fd,
            &mut timer as *mut uint64_t as *mut libc::c_void,
            ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
        ) as libc::c_int;
        return (s as libc::c_ulong == ::std::mem::size_of::<uint64_t>() as libc::c_ulong)
            as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn signal_triggered(mut fd: *mut pollfd) -> libc::c_int {
    if (*fd).revents as libc::c_int & 0x1 as libc::c_int != 0 {
        let mut fdsi: signalfd_siginfo = signalfd_siginfo {
            ssi_signo: 0,
            ssi_errno: 0,
            ssi_code: 0,
            ssi_pid: 0,
            ssi_uid: 0,
            ssi_fd: 0,
            ssi_tid: 0,
            ssi_band: 0,
            ssi_overrun: 0,
            ssi_trapno: 0,
            ssi_status: 0,
            ssi_int: 0,
            ssi_ptr: 0,
            ssi_utime: 0,
            ssi_stime: 0,
            ssi_addr: 0,
            ssi_addr_lsb: 0,
            __pad2: 0,
            ssi_syscall: 0,
            ssi_call_addr: 0,
            ssi_arch: 0,
            __pad: [0; 28],
        };
        let mut s: libc::c_int = read(
            (*fd).fd,
            &mut fdsi as *mut signalfd_siginfo as *mut libc::c_void,
            ::std::mem::size_of::<signalfd_siginfo>() as libc::c_ulong,
        ) as libc::c_int;
        if s as libc::c_ulong
            == ::std::mem::size_of::<signalfd_siginfo>() as libc::c_ulong
        {
            return fdsi.ssi_signo as libc::c_int;
        }
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn brubeck_server_run(
    mut server: *mut brubeck_server,
) -> libc::c_int {
    let mut fds: [pollfd; 2] = [pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    }; 2];
    let mut nfd: libc::c_int = 2 as libc::c_int;
    let mut i: size_t = 0;
    memset(
        fds.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[pollfd; 2]>() as libc::c_ulong,
    );
    fds[0 as libc::c_int as usize].fd = (*server).fd_signal;
    fds[0 as libc::c_int as usize].events = 0x1 as libc::c_int as libc::c_short;
    fds[1 as libc::c_int as usize].fd = (*server).fd_update;
    fds[1 as libc::c_int as usize].events = 0x1 as libc::c_int as libc::c_short;
    (*server).running = 1 as libc::c_int;
    gh_log_write(
        b"instance=%s event=listening\n\0" as *const u8 as *const libc::c_char,
        gh_log_instance(),
    );
    while (*server).running != 0 {
        if poll(fds.as_mut_ptr(), nfd as nfds_t, -(1 as libc::c_int)) < 0 as libc::c_int
        {
            continue;
        }
        match signal_triggered(
            &mut *fds.as_mut_ptr().offset(0 as libc::c_int as isize),
        ) {
            1 => {
                gh_log_reopen();
                gh_log_write(
                    b"instance=%s event=reload_log\n\0" as *const u8
                        as *const libc::c_char,
                    gh_log_instance(),
                );
            }
            12 => {
                dump_all_metrics(server);
            }
            2 | 15 => {
                (*server).running = 0 as libc::c_int;
            }
            _ => {}
        }
        if timer_elapsed(&mut *fds.as_mut_ptr().offset(1 as libc::c_int as isize)) != 0 {
            update_flows(server);
            update_proctitle(server);
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < (*server).active_backends as libc::c_ulong {
        pthread_cancel((*(*server).backends[i as usize]).thread);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < (*server).active_samplers as libc::c_ulong {
        let mut sampler: *mut brubeck_sampler = (*server).samplers[i as usize];
        if ((*sampler).shutdown).is_some() {
            ((*sampler).shutdown).unwrap()(sampler);
        }
        i = i.wrapping_add(1);
        i;
    }
    gh_log_write(
        b"instance=%s event=shutdown\n\0" as *const u8 as *const libc::c_char,
        gh_log_instance(),
    );
    return 0 as libc::c_int;
}
