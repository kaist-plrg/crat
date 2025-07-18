use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type brubeck_tags_t;
    pub type brubeck_hashtable_t;
    pub type rd_kafka_s;
    pub type MHD_Daemon;
    pub type MHD_Connection;
    pub type MHD_Response;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn getpid() -> __pid_t;
    static mut stderr: *mut FILE;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    fn json_array() -> *mut json_t;
    fn json_delete(json: *mut json_t);
    fn json_array_append_new(array: *mut json_t, value: *mut json_t) -> libc::c_int;
    fn json_pack(fmt: *const libc::c_char, _: ...) -> *mut json_t;
    fn json_dumps(json: *const json_t, flags: size_t) -> *mut libc::c_char;
    fn brubeck_hashtable_find(
        ht: *mut brubeck_hashtable_t,
        key: *const libc::c_char,
        key_len: uint16_t,
    ) -> *mut brubeck_metric;
    fn gh_log_instance() -> *const libc::c_char;
    fn gh_log_die() -> !;
    fn gh_log_write(message: *const libc::c_char, _: ...);
    fn MHD_start_daemon(
        flags: libc::c_uint,
        port: uint16_t,
        apc: MHD_AcceptPolicyCallback,
        apc_cls: *mut libc::c_void,
        dh: MHD_AccessHandlerCallback,
        dh_cls: *mut libc::c_void,
        _: ...
    ) -> *mut MHD_Daemon;
    fn MHD_queue_response(
        connection: *mut MHD_Connection,
        status_code: libc::c_uint,
        response: *mut MHD_Response,
    ) -> libc::c_int;
    fn MHD_create_response_from_buffer(
        size: size_t,
        buffer: *mut libc::c_void,
        mode: MHD_ResponseMemoryMode,
    ) -> *mut MHD_Response;
    fn MHD_destroy_response(response: *mut MHD_Response);
    fn MHD_add_response_header(
        response: *mut MHD_Response,
        header: *const libc::c_char,
        content: *const libc::c_char,
    ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __socklen_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub type socklen_t = __socklen_t;
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
pub type json_int_t = libc::c_longlong;
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const BRUBECK_STATE_ACTIVE: C2RustUnnamed_3 = 2;
pub const BRUBECK_STATE_INACTIVE: C2RustUnnamed_3 = 1;
pub const BRUBECK_STATE_DISABLED: C2RustUnnamed_3 = 0;
pub const MHD_OPTION_END: MHD_OPTION = 0;
pub const MHD_OPTION_CONNECTION_TIMEOUT: MHD_OPTION = 3;
pub type MHD_ResponseMemoryMode = libc::c_uint;
pub const MHD_RESPMEM_MUST_COPY: MHD_ResponseMemoryMode = 2;
pub const MHD_RESPMEM_MUST_FREE: MHD_ResponseMemoryMode = 1;
pub const MHD_RESPMEM_PERSISTENT: MHD_ResponseMemoryMode = 0;
pub type MHD_AcceptPolicyCallback = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const sockaddr, socklen_t) -> libc::c_int,
>;
pub const MHD_USE_SELECT_INTERNALLY: MHD_FLAG = 8;
pub type MHD_AccessHandlerCallback = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut MHD_Connection,
        *const libc::c_char,
        *const libc::c_char,
        *const libc::c_char,
        *const libc::c_char,
        *mut size_t,
        *mut *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type MHD_FLAG = libc::c_uint;
pub const MHD_USE_INSECURE_TLS_EARLY_DATA: MHD_FLAG = 262144;
pub const MHD_USE_POST_HANDSHAKE_AUTH_SUPPORT: MHD_FLAG = 131072;
pub const MHD_USE_AUTO_INTERNAL_THREAD: MHD_FLAG = 65544;
pub const MHD_USE_AUTO: MHD_FLAG = 65536;
pub const MHD_ALLOW_UPGRADE: MHD_FLAG = 32768;
pub const MHD_USE_TCP_FASTOPEN: MHD_FLAG = 16384;
pub const MHD_USE_SUSPEND_RESUME: MHD_FLAG = 9216;
pub const MHD_ALLOW_SUSPEND_RESUME: MHD_FLAG = 9216;
pub const MHD_USE_EPOLL_TURBO: MHD_FLAG = 4096;
pub const MHD_USE_TURBO: MHD_FLAG = 4096;
pub const MHD_USE_DUAL_STACK: MHD_FLAG = 2064;
pub const MHD_USE_PIPE_FOR_SHUTDOWN: MHD_FLAG = 1024;
pub const MHD_USE_ITC: MHD_FLAG = 1024;
pub const MHD_USE_EPOLL_INTERNALLY_LINUX_ONLY: MHD_FLAG = 520;
pub const MHD_USE_EPOLL_INTERNALLY: MHD_FLAG = 520;
pub const MHD_USE_EPOLL_INTERNAL_THREAD: MHD_FLAG = 520;
pub const MHD_USE_EPOLL_LINUX_ONLY: MHD_FLAG = 512;
pub const MHD_USE_EPOLL: MHD_FLAG = 512;
pub const MHD_USE_NO_LISTEN_SOCKET: MHD_FLAG = 256;
pub const MHD_SUPPRESS_DATE_NO_CLOCK: MHD_FLAG = 128;
pub const MHD_USE_SUPPRESS_DATE_NO_CLOCK: MHD_FLAG = 128;
pub const MHD_USE_POLL_INTERNALLY: MHD_FLAG = 72;
pub const MHD_USE_POLL_INTERNAL_THREAD: MHD_FLAG = 72;
pub const MHD_USE_POLL: MHD_FLAG = 64;
pub const MHD_USE_PEDANTIC_CHECKS: MHD_FLAG = 32;
pub const MHD_USE_IPv6: MHD_FLAG = 16;
pub const MHD_USE_INTERNAL_POLLING_THREAD: MHD_FLAG = 8;
pub const MHD_USE_THREAD_PER_CONNECTION: MHD_FLAG = 4;
pub const MHD_USE_SSL: MHD_FLAG = 2;
pub const MHD_USE_TLS: MHD_FLAG = 2;
pub const MHD_USE_DEBUG: MHD_FLAG = 1;
pub const MHD_USE_ERROR_LOG: MHD_FLAG = 1;
pub const MHD_NO_FLAG: MHD_FLAG = 0;
pub type MHD_OPTION = libc::c_uint;
pub const MHD_OPTION_HTTPS_CERT_CALLBACK2: MHD_OPTION = 31;
pub const MHD_OPTION_GNUTLS_PSK_CRED_HANDLER: MHD_OPTION = 30;
pub const MHD_OPTION_STRICT_FOR_CLIENT: MHD_OPTION = 29;
pub const MHD_OPTION_LISTEN_BACKLOG_SIZE: MHD_OPTION = 28;
pub const MHD_OPTION_NOTIFY_CONNECTION: MHD_OPTION = 27;
pub const MHD_OPTION_HTTPS_KEY_PASSWORD: MHD_OPTION = 26;
pub const MHD_OPTION_LISTENING_ADDRESS_REUSE: MHD_OPTION = 25;
pub const MHD_OPTION_HTTPS_MEM_DHPARAMS: MHD_OPTION = 24;
pub const MHD_OPTION_TCP_FASTOPEN_QUEUE_SIZE: MHD_OPTION = 23;
pub const MHD_OPTION_HTTPS_CERT_CALLBACK: MHD_OPTION = 22;
pub const MHD_OPTION_CONNECTION_MEMORY_INCREMENT: MHD_OPTION = 21;
pub const MHD_OPTION_HTTPS_MEM_TRUST: MHD_OPTION = 20;
pub const MHD_OPTION_THREAD_STACK_SIZE: MHD_OPTION = 19;
pub const MHD_OPTION_NONCE_NC_SIZE: MHD_OPTION = 18;
pub const MHD_OPTION_DIGEST_AUTH_RANDOM: MHD_OPTION = 17;
pub const MHD_OPTION_UNESCAPE_CALLBACK: MHD_OPTION = 16;
pub const MHD_OPTION_ARRAY: MHD_OPTION = 15;
pub const MHD_OPTION_THREAD_POOL_SIZE: MHD_OPTION = 14;
pub const MHD_OPTION_EXTERNAL_LOGGER: MHD_OPTION = 13;
pub const MHD_OPTION_LISTEN_SOCKET: MHD_OPTION = 12;
pub const MHD_OPTION_HTTPS_PRIORITIES: MHD_OPTION = 11;
pub const MHD_OPTION_HTTPS_CRED_TYPE: MHD_OPTION = 10;
pub const MHD_OPTION_HTTPS_MEM_CERT: MHD_OPTION = 9;
pub const MHD_OPTION_HTTPS_MEM_KEY: MHD_OPTION = 8;
pub const MHD_OPTION_URI_LOG_CALLBACK: MHD_OPTION = 7;
pub const MHD_OPTION_SOCK_ADDR: MHD_OPTION = 6;
pub const MHD_OPTION_PER_IP_CONNECTION_LIMIT: MHD_OPTION = 5;
pub const MHD_OPTION_NOTIFY_COMPLETED: MHD_OPTION = 4;
pub const MHD_OPTION_CONNECTION_LIMIT: MHD_OPTION = 2;
pub const MHD_OPTION_CONNECTION_MEMORY_LIMIT: MHD_OPTION = 1;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn json_decref(mut json: *mut json_t) {
    if !json.is_null() && (*json).refcount != -(1 as libc::c_int) as size_t
        && {
            let fresh0 = &mut (*json).refcount as *mut size_t;
            let fresh1 = 1 as libc::c_int as size_t;
            ::std::intrinsics::atomic_xsub_release(fresh0, fresh1) - fresh1
                == 0 as libc::c_int as libc::c_ulong
        }
    {
        json_delete(json);
    }
}
#[inline]
unsafe extern "C" fn brubeck_metric_get_state(
    mut metric: *const brubeck_metric,
) -> uint8_t {
    return ::std::intrinsics::atomic_load_seqcst(&(*metric).private_state);
}
#[inline]
unsafe extern "C" fn brubeck_metric_set_state(
    mut metric: *mut brubeck_metric,
    state: uint8_t,
) {
    ::std::intrinsics::atomic_store_seqcst(&mut (*metric).private_state, state);
}
#[inline]
unsafe extern "C" fn starts_with(
    mut str: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> libc::c_int {
    loop {
        if *prefix == 0 {
            return 1 as libc::c_int
        } else if *str as libc::c_int != *prefix as libc::c_int {
            return 0 as libc::c_int
        }
        str = str.offset(1);
        str;
        prefix = prefix.offset(1);
        prefix;
    };
}
unsafe extern "C" fn flow_stats(mut server: *mut brubeck_server) -> *mut MHD_Response {
    return 0 as *mut MHD_Response;
}
unsafe extern "C" fn safe_lookup_metric(
    mut server: *mut brubeck_server,
    mut key: *const libc::c_char,
) -> *mut brubeck_metric {
    return brubeck_hashtable_find((*server).metrics, key, strlen(key) as uint16_t);
}
unsafe extern "C" fn expire_metric(
    mut server: *mut brubeck_server,
    mut url: *const libc::c_char,
) -> *mut MHD_Response {
    let mut metric: *mut brubeck_metric = safe_lookup_metric(
        server,
        url.offset(strlen(b"/expire/\0" as *const u8 as *const libc::c_char) as isize),
    );
    if !metric.is_null() {
        brubeck_metric_set_state(
            metric,
            BRUBECK_STATE_DISABLED as libc::c_int as uint8_t,
        );
        return MHD_create_response_from_buffer(
            0 as libc::c_int as size_t,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
            MHD_RESPMEM_PERSISTENT,
        );
    }
    return 0 as *mut MHD_Response;
}
unsafe extern "C" fn send_metric(
    mut server: *mut brubeck_server,
    mut url: *const libc::c_char,
) -> *mut MHD_Response {
    static mut metric_types: [*const libc::c_char; 6] = [
        b"gauge\0" as *const u8 as *const libc::c_char,
        b"meter\0" as *const u8 as *const libc::c_char,
        b"counter\0" as *const u8 as *const libc::c_char,
        b"histogram\0" as *const u8 as *const libc::c_char,
        b"timer\0" as *const u8 as *const libc::c_char,
        b"internal\0" as *const u8 as *const libc::c_char,
    ];
    static mut expire_status: [*const libc::c_char; 3] = [
        b"disabled\0" as *const u8 as *const libc::c_char,
        b"inactive\0" as *const u8 as *const libc::c_char,
        b"active\0" as *const u8 as *const libc::c_char,
    ];
    let mut metric: *mut brubeck_metric = safe_lookup_metric(
        server,
        url.offset(strlen(b"/metric/\0" as *const u8 as *const libc::c_char) as isize),
    );
    if !metric.is_null() {
        let mut mj: *mut json_t = json_pack(
            b"{s:s, s:s, s:i, s:s}\0" as *const u8 as *const libc::c_char,
            b"key\0" as *const u8 as *const libc::c_char,
            ((*metric).key).as_mut_ptr(),
            b"type\0" as *const u8 as *const libc::c_char,
            metric_types[(*metric).type_0 as usize],
            b"shard\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"expire\0" as *const u8 as *const libc::c_char,
            expire_status[brubeck_metric_get_state(metric) as usize],
        );
        let mut jsonr: *mut libc::c_char = json_dumps(
            mj,
            (4 as libc::c_int & 0x1f as libc::c_int | 0x100 as libc::c_int) as size_t,
        );
        json_decref(mj);
        return MHD_create_response_from_buffer(
            strlen(jsonr),
            jsonr as *mut libc::c_void,
            MHD_RESPMEM_MUST_FREE,
        );
    }
    return 0 as *mut MHD_Response;
}
unsafe extern "C" fn send_stats(mut brubeck: *mut brubeck_server) -> *mut MHD_Response {
    let mut jsonr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stats: *mut json_t = 0 as *mut json_t;
    let mut backends: *mut json_t = 0 as *mut json_t;
    let mut samplers: *mut json_t = 0 as *mut json_t;
    let mut i: libc::c_int = 0;
    backends = json_array();
    i = 0 as libc::c_int;
    while i < (*brubeck).active_backends {
        let mut backend: *mut brubeck_backend = (*brubeck).backends[i as usize];
        if (*backend).type_0 as libc::c_uint
            == BRUBECK_BACKEND_CARBON as libc::c_int as libc::c_uint
        {
            let mut carbon: *mut brubeck_carbon = backend as *mut brubeck_carbon;
            let mut address: *mut sockaddr_in = &mut (*carbon).out_sockaddr;
            let mut addr: [libc::c_char; 16] = [0; 16];
            json_array_append_new(
                backends,
                json_pack(
                    b"{s:s, s:i, s:b, s:s, s:i, s:I}\0" as *const u8
                        as *const libc::c_char,
                    b"type\0" as *const u8 as *const libc::c_char,
                    b"carbon\0" as *const u8 as *const libc::c_char,
                    b"sample_freq\0" as *const u8 as *const libc::c_char,
                    (*carbon).backend.sample_freq,
                    b"connected\0" as *const u8 as *const libc::c_char,
                    ((*carbon).out_sock >= 0 as libc::c_int) as libc::c_int,
                    b"address\0" as *const u8 as *const libc::c_char,
                    inet_ntop(
                        2 as libc::c_int,
                        &mut (*address).sin_addr.s_addr as *mut in_addr_t
                            as *const libc::c_void,
                        addr.as_mut_ptr(),
                        16 as libc::c_int as socklen_t,
                    ),
                    b"port\0" as *const u8 as *const libc::c_char,
                    __bswap_16((*address).sin_port) as libc::c_int,
                    b"bytes_sent\0" as *const u8 as *const libc::c_char,
                    (*carbon).bytes_sent as json_int_t,
                ),
            );
        } else if (*backend).type_0 as libc::c_uint
            == BRUBECK_BACKEND_KAFKA as libc::c_int as libc::c_uint
        {
            let mut kafka: *mut brubeck_kafka = backend as *mut brubeck_kafka;
            json_array_append_new(
                backends,
                json_pack(
                    b"{s:s, s:i, s:b, s:I}\0" as *const u8 as *const libc::c_char,
                    b"type\0" as *const u8 as *const libc::c_char,
                    b"kafka\0" as *const u8 as *const libc::c_char,
                    b"sample_freq\0" as *const u8 as *const libc::c_char,
                    (*kafka).backend.sample_freq,
                    b"connected\0" as *const u8 as *const libc::c_char,
                    (*kafka).connected as libc::c_int,
                    b"bytes_sent\0" as *const u8 as *const libc::c_char,
                    (*kafka).bytes_sent as json_int_t,
                ),
            );
        }
        i += 1;
        i;
    }
    samplers = json_array();
    i = 0 as libc::c_int;
    while i < (*brubeck).active_samplers {
        let mut sampler: *mut brubeck_sampler = (*brubeck).samplers[i as usize];
        let mut address_0: *mut sockaddr_in = &mut (*sampler).addr;
        let mut addr_0: [libc::c_char; 16] = [0; 16];
        let mut sampler_name: *const libc::c_char = 0 as *const libc::c_char;
        match (*sampler).type_0 as libc::c_uint {
            0 => {
                sampler_name = b"statsd\0" as *const u8 as *const libc::c_char;
            }
            _ => {}
        }
        json_array_append_new(
            samplers,
            json_pack(
                b"{s:s, s:f, s:s, s:i}\0" as *const u8 as *const libc::c_char,
                b"type\0" as *const u8 as *const libc::c_char,
                sampler_name,
                b"sample_freq\0" as *const u8 as *const libc::c_char,
                (*sampler).current_flow as libc::c_double,
                b"address\0" as *const u8 as *const libc::c_char,
                inet_ntop(
                    2 as libc::c_int,
                    &mut (*address_0).sin_addr.s_addr as *mut in_addr_t
                        as *const libc::c_void,
                    addr_0.as_mut_ptr(),
                    16 as libc::c_int as socklen_t,
                ),
                b"port\0" as *const u8 as *const libc::c_char,
                __bswap_16((*address_0).sin_port) as libc::c_int,
            ),
        );
        i += 1;
        i;
    }
    stats = json_pack(
        b"{s:s, s:i, s:i, s:i, s:o, s:o}\0" as *const u8 as *const libc::c_char,
        b"version\0" as *const u8 as *const libc::c_char,
        b"brubeck \0" as *const u8 as *const libc::c_char,
        b"metrics\0" as *const u8 as *const libc::c_char,
        (*brubeck).internal_stats.sample.metrics,
        b"errors\0" as *const u8 as *const libc::c_char,
        (*brubeck).internal_stats.sample.errors,
        b"unique_keys\0" as *const u8 as *const libc::c_char,
        (*brubeck).internal_stats.sample.unique_keys,
        b"backends\0" as *const u8 as *const libc::c_char,
        backends,
        b"samplers\0" as *const u8 as *const libc::c_char,
        samplers,
    );
    jsonr = json_dumps(
        stats,
        (4 as libc::c_int & 0x1f as libc::c_int | 0x100 as libc::c_int) as size_t,
    );
    json_decref(stats);
    return MHD_create_response_from_buffer(
        strlen(jsonr),
        jsonr as *mut libc::c_void,
        MHD_RESPMEM_MUST_FREE,
    );
}
unsafe extern "C" fn send_ping(mut brubeck: *mut brubeck_server) -> *mut MHD_Response {
    let frequency: value_t = (*brubeck).internal_stats.sample_freq as libc::c_double;
    let mut status: *const libc::c_char = b"OK\0" as *const u8 as *const libc::c_char;
    let mut jsonr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stats: *mut json_t = 0 as *mut json_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*brubeck).active_backends {
        let mut backend: *mut brubeck_backend = (*brubeck).backends[i as usize];
        if !((*backend).is_connected).unwrap()(backend as *mut libc::c_void) {
            status = b"ERROR (backend disconnected)\0" as *const u8
                as *const libc::c_char;
            break;
        } else {
            i += 1;
            i;
        }
    }
    stats = json_pack(
        b"{s:s, s:i, s:s, s:f, s:f, s:i}\0" as *const u8 as *const libc::c_char,
        b"version\0" as *const u8 as *const libc::c_char,
        b"brubeck \0" as *const u8 as *const libc::c_char,
        b"pid\0" as *const u8 as *const libc::c_char,
        getpid(),
        b"status\0" as *const u8 as *const libc::c_char,
        status,
        b"metrics_per_second\0" as *const u8 as *const libc::c_char,
        (*brubeck).internal_stats.sample.metrics as value_t / frequency,
        b"errors_per_second\0" as *const u8 as *const libc::c_char,
        (*brubeck).internal_stats.sample.errors as value_t / frequency,
        b"unique_keys\0" as *const u8 as *const libc::c_char,
        (*brubeck).internal_stats.sample.unique_keys,
    );
    jsonr = json_dumps(
        stats,
        (4 as libc::c_int & 0x1f as libc::c_int | 0x100 as libc::c_int) as size_t,
    );
    json_decref(stats);
    return MHD_create_response_from_buffer(
        strlen(jsonr),
        jsonr as *mut libc::c_void,
        MHD_RESPMEM_MUST_FREE,
    );
}
unsafe extern "C" fn handle_request(
    mut cls: *mut libc::c_void,
    mut connection: *mut MHD_Connection,
    mut url: *const libc::c_char,
    mut method: *const libc::c_char,
    mut version: *const libc::c_char,
    mut upload_data: *const libc::c_char,
    mut upload_data_size: *mut size_t,
    mut con_cls: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut response: *mut MHD_Response = 0 as *mut MHD_Response;
    let mut brubeck: *mut brubeck_server = cls as *mut brubeck_server;
    if strcmp(method, b"GET\0" as *const u8 as *const libc::c_char) == 0 {
        if strcmp(url, b"/_ping\0" as *const u8 as *const libc::c_char) == 0
            || strcmp(url, b"/ping\0" as *const u8 as *const libc::c_char) == 0
        {
            response = send_ping(brubeck);
        } else if strcmp(url, b"/stats\0" as *const u8 as *const libc::c_char) == 0 {
            response = send_stats(brubeck);
        } else if strcmp(url, b"/flow_stats\0" as *const u8 as *const libc::c_char) == 0
        {
            response = flow_stats(brubeck);
        } else if starts_with(url, b"/metric/\0" as *const u8 as *const libc::c_char)
            != 0
        {
            response = send_metric(brubeck, url);
        }
    } else if strcmp(method, b"POST\0" as *const u8 as *const libc::c_char) == 0 {
        if starts_with(url, b"/expire/\0" as *const u8 as *const libc::c_char) != 0 {
            response = expire_metric(brubeck, url);
        }
    }
    if response.is_null() {
        static mut NOT_FOUND: *const libc::c_char = b"404 not found\0" as *const u8
            as *const libc::c_char;
        response = MHD_create_response_from_buffer(
            strlen(NOT_FOUND),
            NOT_FOUND as *mut libc::c_void,
            MHD_RESPMEM_PERSISTENT,
        );
        MHD_add_response_header(
            response,
            b"Connection\0" as *const u8 as *const libc::c_char,
            b"close\0" as *const u8 as *const libc::c_char,
        );
        ret = MHD_queue_response(
            connection,
            404 as libc::c_int as libc::c_uint,
            response,
        );
    } else {
        MHD_add_response_header(
            response,
            b"Connection\0" as *const u8 as *const libc::c_char,
            b"close\0" as *const u8 as *const libc::c_char,
        );
        MHD_add_response_header(
            response,
            b"Content-Type\0" as *const u8 as *const libc::c_char,
            b"application/json\0" as *const u8 as *const libc::c_char,
        );
        ret = MHD_queue_response(
            connection,
            200 as libc::c_int as libc::c_uint,
            response,
        );
    }
    MHD_destroy_response(response);
    return ret;
}
pub unsafe extern "C" fn brubeck_http_endpoint_init(
    mut server: *mut brubeck_server,
    mut listen: *const libc::c_char,
) {
    let mut daemon: *mut MHD_Daemon = 0 as *mut MHD_Daemon;
    let mut port: *const libc::c_char = strrchr(listen, ':' as i32);
    port = if !port.is_null() { port.offset(1 as libc::c_int as isize) } else { listen };
    daemon = MHD_start_daemon(
        MHD_USE_SELECT_INTERNALLY as libc::c_int as libc::c_uint,
        atoi(port) as uint16_t,
        None,
        0 as *mut libc::c_void,
        Some(
            handle_request
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut MHD_Connection,
                    *const libc::c_char,
                    *const libc::c_char,
                    *const libc::c_char,
                    *const libc::c_char,
                    *mut size_t,
                    *mut *mut libc::c_void,
                ) -> libc::c_int,
        ),
        server as *mut libc::c_void,
        MHD_OPTION_CONNECTION_TIMEOUT as libc::c_int,
        10 as libc::c_int as libc::c_uint,
        MHD_OPTION_END as libc::c_int,
    );
    if daemon.is_null() {
        fprintf(
            stderr,
            b"[FATAL]: failed to start HTTP endpoint\n\0" as *const u8
                as *const libc::c_char,
        );
        gh_log_die();
    }
    gh_log_write(
        b"instance=%s event=http_server listen=%s\n\0" as *const u8
            as *const libc::c_char,
        gh_log_instance(),
        port,
    );
}
