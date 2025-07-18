use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type brubeck_tags_t;
    pub type brubeck_hashtable_t;
    pub type rd_kafka_s;
    pub type rd_kafka_topic_s;
    pub type rd_kafka_conf_s;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn brubeck_backend_run_threaded(_: *mut brubeck_backend);
    fn json_object() -> *mut json_t;
    fn json_string(value: *const libc::c_char) -> *mut json_t;
    fn json_integer(value: json_int_t) -> *mut json_t;
    fn json_real(value: libc::c_double) -> *mut json_t;
    fn json_delete(json: *mut json_t);
    fn json_object_set_new_nocheck(
        object: *mut json_t,
        key: *const libc::c_char,
        value: *mut json_t,
    ) -> libc::c_int;
    fn json_object_clear(object: *mut json_t) -> libc::c_int;
    fn json_object_iter(object: *mut json_t) -> *mut libc::c_void;
    fn json_object_key_to_iter(key: *const libc::c_char) -> *mut libc::c_void;
    fn json_object_iter_next(
        object: *mut json_t,
        iter: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn json_object_iter_key(iter: *mut libc::c_void) -> *const libc::c_char;
    fn json_object_iter_value(iter: *mut libc::c_void) -> *mut json_t;
    fn json_string_value(string: *const json_t) -> *const libc::c_char;
    fn json_unpack_ex(
        root: *mut json_t,
        error: *mut json_error_t,
        flags: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn json_dumps(json: *const json_t, flags: size_t) -> *mut libc::c_char;
    fn rd_kafka_err2str(err: rd_kafka_resp_err_t) -> *const libc::c_char;
    fn rd_kafka_err2name(err: rd_kafka_resp_err_t) -> *const libc::c_char;
    fn rd_kafka_fatal_error(
        rk: *mut rd_kafka_t,
        errstr_0: *mut libc::c_char,
        errstr_size: size_t,
    ) -> rd_kafka_resp_err_t;
    fn rd_kafka_conf_new() -> *mut rd_kafka_conf_t;
    fn rd_kafka_conf_destroy(conf: *mut rd_kafka_conf_t);
    fn rd_kafka_conf_set(
        conf: *mut rd_kafka_conf_t,
        name: *const libc::c_char,
        value: *const libc::c_char,
        errstr_0: *mut libc::c_char,
        errstr_size: size_t,
    ) -> rd_kafka_conf_res_t;
    fn rd_kafka_conf_set_dr_msg_cb(
        conf: *mut rd_kafka_conf_t,
        dr_msg_cb_0: Option::<
            unsafe extern "C" fn(
                *mut rd_kafka_t,
                *const rd_kafka_message_t,
                *mut libc::c_void,
            ) -> (),
        >,
    );
    fn rd_kafka_conf_set_error_cb(
        conf: *mut rd_kafka_conf_t,
        error_cb_0: Option::<
            unsafe extern "C" fn(
                *mut rd_kafka_t,
                libc::c_int,
                *const libc::c_char,
                *mut libc::c_void,
            ) -> (),
        >,
    );
    fn rd_kafka_conf_set_opaque(conf: *mut rd_kafka_conf_t, opaque: *mut libc::c_void);
    fn rd_kafka_new(
        type_0: rd_kafka_type_t,
        conf: *mut rd_kafka_conf_t,
        errstr_0: *mut libc::c_char,
        errstr_size: size_t,
    ) -> *mut rd_kafka_t;
    fn rd_kafka_destroy(rk: *mut rd_kafka_t);
    fn rd_kafka_poll(rk: *mut rd_kafka_t, timeout_ms: libc::c_int) -> libc::c_int;
    fn rd_kafka_producev(rk: *mut rd_kafka_t, _: ...) -> rd_kafka_resp_err_t;
    fn rd_kafka_flush(
        rk: *mut rd_kafka_t,
        timeout_ms: libc::c_int,
    ) -> rd_kafka_resp_err_t;
    fn gh_log_die() -> !;
    fn gh_log_instance() -> *const libc::c_char;
    fn gh_log_write(message: *const libc::c_char, _: ...);
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
pub type json_int_t = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_error_t {
    pub line: libc::c_int,
    pub column: libc::c_int,
    pub position: libc::c_int,
    pub source: [libc::c_char; 80],
    pub text: [libc::c_char; 160],
}
pub type rd_kafka_type_t = libc::c_uint;
pub const RD_KAFKA_CONSUMER: rd_kafka_type_t = 1;
pub const RD_KAFKA_PRODUCER: rd_kafka_type_t = 0;
pub type rd_kafka_t = rd_kafka_s;
pub type rd_kafka_topic_t = rd_kafka_topic_s;
pub type rd_kafka_conf_t = rd_kafka_conf_s;
pub type rd_kafka_resp_err_t = libc::c_int;
pub const RD_KAFKA_RESP_ERR_END_ALL: rd_kafka_resp_err_t = 82;
pub const RD_KAFKA_RESP_ERR_GROUP_MAX_SIZE_REACHED: rd_kafka_resp_err_t = 81;
pub const RD_KAFKA_RESP_ERR_PREFERRED_LEADER_NOT_AVAILABLE: rd_kafka_resp_err_t = 80;
pub const RD_KAFKA_RESP_ERR_MEMBER_ID_REQUIRED: rd_kafka_resp_err_t = 79;
pub const RD_KAFKA_RESP_ERR_OFFSET_NOT_AVAILABLE: rd_kafka_resp_err_t = 78;
pub const RD_KAFKA_RESP_ERR_STALE_BROKER_EPOCH: rd_kafka_resp_err_t = 77;
pub const RD_KAFKA_RESP_ERR_UNSUPPORTED_COMPRESSION_TYPE: rd_kafka_resp_err_t = 76;
pub const RD_KAFKA_RESP_ERR_UNKNOWN_LEADER_EPOCH: rd_kafka_resp_err_t = 75;
pub const RD_KAFKA_RESP_ERR_FENCED_LEADER_EPOCH: rd_kafka_resp_err_t = 74;
pub const RD_KAFKA_RESP_ERR_TOPIC_DELETION_DISABLED: rd_kafka_resp_err_t = 73;
pub const RD_KAFKA_RESP_ERR_LISTENER_NOT_FOUND: rd_kafka_resp_err_t = 72;
pub const RD_KAFKA_RESP_ERR_INVALID_FETCH_SESSION_EPOCH: rd_kafka_resp_err_t = 71;
pub const RD_KAFKA_RESP_ERR_FETCH_SESSION_ID_NOT_FOUND: rd_kafka_resp_err_t = 70;
pub const RD_KAFKA_RESP_ERR_GROUP_ID_NOT_FOUND: rd_kafka_resp_err_t = 69;
pub const RD_KAFKA_RESP_ERR_NON_EMPTY_GROUP: rd_kafka_resp_err_t = 68;
pub const RD_KAFKA_RESP_ERR_INVALID_PRINCIPAL_TYPE: rd_kafka_resp_err_t = 67;
pub const RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_EXPIRED: rd_kafka_resp_err_t = 66;
pub const RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_AUTHORIZATION_FAILED: rd_kafka_resp_err_t = 65;
pub const RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_REQUEST_NOT_ALLOWED: rd_kafka_resp_err_t = 64;
pub const RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_OWNER_MISMATCH: rd_kafka_resp_err_t = 63;
pub const RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_NOT_FOUND: rd_kafka_resp_err_t = 62;
pub const RD_KAFKA_RESP_ERR_DELEGATION_TOKEN_AUTH_DISABLED: rd_kafka_resp_err_t = 61;
pub const RD_KAFKA_RESP_ERR_REASSIGNMENT_IN_PROGRESS: rd_kafka_resp_err_t = 60;
pub const RD_KAFKA_RESP_ERR_UNKNOWN_PRODUCER_ID: rd_kafka_resp_err_t = 59;
pub const RD_KAFKA_RESP_ERR_SASL_AUTHENTICATION_FAILED: rd_kafka_resp_err_t = 58;
pub const RD_KAFKA_RESP_ERR_LOG_DIR_NOT_FOUND: rd_kafka_resp_err_t = 57;
pub const RD_KAFKA_RESP_ERR_KAFKA_STORAGE_ERROR: rd_kafka_resp_err_t = 56;
pub const RD_KAFKA_RESP_ERR_OPERATION_NOT_ATTEMPTED: rd_kafka_resp_err_t = 55;
pub const RD_KAFKA_RESP_ERR_SECURITY_DISABLED: rd_kafka_resp_err_t = 54;
pub const RD_KAFKA_RESP_ERR_TRANSACTIONAL_ID_AUTHORIZATION_FAILED: rd_kafka_resp_err_t = 53;
pub const RD_KAFKA_RESP_ERR_TRANSACTION_COORDINATOR_FENCED: rd_kafka_resp_err_t = 52;
pub const RD_KAFKA_RESP_ERR_CONCURRENT_TRANSACTIONS: rd_kafka_resp_err_t = 51;
pub const RD_KAFKA_RESP_ERR_INVALID_TRANSACTION_TIMEOUT: rd_kafka_resp_err_t = 50;
pub const RD_KAFKA_RESP_ERR_INVALID_PRODUCER_ID_MAPPING: rd_kafka_resp_err_t = 49;
pub const RD_KAFKA_RESP_ERR_INVALID_TXN_STATE: rd_kafka_resp_err_t = 48;
pub const RD_KAFKA_RESP_ERR_INVALID_PRODUCER_EPOCH: rd_kafka_resp_err_t = 47;
pub const RD_KAFKA_RESP_ERR_DUPLICATE_SEQUENCE_NUMBER: rd_kafka_resp_err_t = 46;
pub const RD_KAFKA_RESP_ERR_OUT_OF_ORDER_SEQUENCE_NUMBER: rd_kafka_resp_err_t = 45;
pub const RD_KAFKA_RESP_ERR_POLICY_VIOLATION: rd_kafka_resp_err_t = 44;
pub const RD_KAFKA_RESP_ERR_UNSUPPORTED_FOR_MESSAGE_FORMAT: rd_kafka_resp_err_t = 43;
pub const RD_KAFKA_RESP_ERR_INVALID_REQUEST: rd_kafka_resp_err_t = 42;
pub const RD_KAFKA_RESP_ERR_NOT_CONTROLLER: rd_kafka_resp_err_t = 41;
pub const RD_KAFKA_RESP_ERR_INVALID_CONFIG: rd_kafka_resp_err_t = 40;
pub const RD_KAFKA_RESP_ERR_INVALID_REPLICA_ASSIGNMENT: rd_kafka_resp_err_t = 39;
pub const RD_KAFKA_RESP_ERR_INVALID_REPLICATION_FACTOR: rd_kafka_resp_err_t = 38;
pub const RD_KAFKA_RESP_ERR_INVALID_PARTITIONS: rd_kafka_resp_err_t = 37;
pub const RD_KAFKA_RESP_ERR_TOPIC_ALREADY_EXISTS: rd_kafka_resp_err_t = 36;
pub const RD_KAFKA_RESP_ERR_UNSUPPORTED_VERSION: rd_kafka_resp_err_t = 35;
pub const RD_KAFKA_RESP_ERR_ILLEGAL_SASL_STATE: rd_kafka_resp_err_t = 34;
pub const RD_KAFKA_RESP_ERR_UNSUPPORTED_SASL_MECHANISM: rd_kafka_resp_err_t = 33;
pub const RD_KAFKA_RESP_ERR_INVALID_TIMESTAMP: rd_kafka_resp_err_t = 32;
pub const RD_KAFKA_RESP_ERR_CLUSTER_AUTHORIZATION_FAILED: rd_kafka_resp_err_t = 31;
pub const RD_KAFKA_RESP_ERR_GROUP_AUTHORIZATION_FAILED: rd_kafka_resp_err_t = 30;
pub const RD_KAFKA_RESP_ERR_TOPIC_AUTHORIZATION_FAILED: rd_kafka_resp_err_t = 29;
pub const RD_KAFKA_RESP_ERR_INVALID_COMMIT_OFFSET_SIZE: rd_kafka_resp_err_t = 28;
pub const RD_KAFKA_RESP_ERR_REBALANCE_IN_PROGRESS: rd_kafka_resp_err_t = 27;
pub const RD_KAFKA_RESP_ERR_INVALID_SESSION_TIMEOUT: rd_kafka_resp_err_t = 26;
pub const RD_KAFKA_RESP_ERR_UNKNOWN_MEMBER_ID: rd_kafka_resp_err_t = 25;
pub const RD_KAFKA_RESP_ERR_INVALID_GROUP_ID: rd_kafka_resp_err_t = 24;
pub const RD_KAFKA_RESP_ERR_INCONSISTENT_GROUP_PROTOCOL: rd_kafka_resp_err_t = 23;
pub const RD_KAFKA_RESP_ERR_ILLEGAL_GENERATION: rd_kafka_resp_err_t = 22;
pub const RD_KAFKA_RESP_ERR_INVALID_REQUIRED_ACKS: rd_kafka_resp_err_t = 21;
pub const RD_KAFKA_RESP_ERR_NOT_ENOUGH_REPLICAS_AFTER_APPEND: rd_kafka_resp_err_t = 20;
pub const RD_KAFKA_RESP_ERR_NOT_ENOUGH_REPLICAS: rd_kafka_resp_err_t = 19;
pub const RD_KAFKA_RESP_ERR_RECORD_LIST_TOO_LARGE: rd_kafka_resp_err_t = 18;
pub const RD_KAFKA_RESP_ERR_TOPIC_EXCEPTION: rd_kafka_resp_err_t = 17;
pub const RD_KAFKA_RESP_ERR_NOT_COORDINATOR_FOR_GROUP: rd_kafka_resp_err_t = 16;
pub const RD_KAFKA_RESP_ERR_GROUP_COORDINATOR_NOT_AVAILABLE: rd_kafka_resp_err_t = 15;
pub const RD_KAFKA_RESP_ERR_GROUP_LOAD_IN_PROGRESS: rd_kafka_resp_err_t = 14;
pub const RD_KAFKA_RESP_ERR_NETWORK_EXCEPTION: rd_kafka_resp_err_t = 13;
pub const RD_KAFKA_RESP_ERR_OFFSET_METADATA_TOO_LARGE: rd_kafka_resp_err_t = 12;
pub const RD_KAFKA_RESP_ERR_STALE_CTRL_EPOCH: rd_kafka_resp_err_t = 11;
pub const RD_KAFKA_RESP_ERR_MSG_SIZE_TOO_LARGE: rd_kafka_resp_err_t = 10;
pub const RD_KAFKA_RESP_ERR_REPLICA_NOT_AVAILABLE: rd_kafka_resp_err_t = 9;
pub const RD_KAFKA_RESP_ERR_BROKER_NOT_AVAILABLE: rd_kafka_resp_err_t = 8;
pub const RD_KAFKA_RESP_ERR_REQUEST_TIMED_OUT: rd_kafka_resp_err_t = 7;
pub const RD_KAFKA_RESP_ERR_NOT_LEADER_FOR_PARTITION: rd_kafka_resp_err_t = 6;
pub const RD_KAFKA_RESP_ERR_LEADER_NOT_AVAILABLE: rd_kafka_resp_err_t = 5;
pub const RD_KAFKA_RESP_ERR_INVALID_MSG_SIZE: rd_kafka_resp_err_t = 4;
pub const RD_KAFKA_RESP_ERR_UNKNOWN_TOPIC_OR_PART: rd_kafka_resp_err_t = 3;
pub const RD_KAFKA_RESP_ERR_INVALID_MSG: rd_kafka_resp_err_t = 2;
pub const RD_KAFKA_RESP_ERR_OFFSET_OUT_OF_RANGE: rd_kafka_resp_err_t = 1;
pub const RD_KAFKA_RESP_ERR_NO_ERROR: rd_kafka_resp_err_t = 0;
pub const RD_KAFKA_RESP_ERR_UNKNOWN: rd_kafka_resp_err_t = -1;
pub const RD_KAFKA_RESP_ERR__END: rd_kafka_resp_err_t = -100;
pub const RD_KAFKA_RESP_ERR__MAX_POLL_EXCEEDED: rd_kafka_resp_err_t = -147;
pub const RD_KAFKA_RESP_ERR__GAPLESS_GUARANTEE: rd_kafka_resp_err_t = -148;
pub const RD_KAFKA_RESP_ERR__INCONSISTENT: rd_kafka_resp_err_t = -149;
pub const RD_KAFKA_RESP_ERR__FATAL: rd_kafka_resp_err_t = -150;
pub const RD_KAFKA_RESP_ERR__PURGE_INFLIGHT: rd_kafka_resp_err_t = -151;
pub const RD_KAFKA_RESP_ERR__PURGE_QUEUE: rd_kafka_resp_err_t = -152;
pub const RD_KAFKA_RESP_ERR__RETRY: rd_kafka_resp_err_t = -153;
pub const RD_KAFKA_RESP_ERR__INVALID_TYPE: rd_kafka_resp_err_t = -154;
pub const RD_KAFKA_RESP_ERR__UNDERFLOW: rd_kafka_resp_err_t = -155;
pub const RD_KAFKA_RESP_ERR__NOENT: rd_kafka_resp_err_t = -156;
pub const RD_KAFKA_RESP_ERR__READ_ONLY: rd_kafka_resp_err_t = -157;
pub const RD_KAFKA_RESP_ERR__PARTIAL: rd_kafka_resp_err_t = -158;
pub const RD_KAFKA_RESP_ERR__VALUE_DESERIALIZATION: rd_kafka_resp_err_t = -159;
pub const RD_KAFKA_RESP_ERR__KEY_DESERIALIZATION: rd_kafka_resp_err_t = -160;
pub const RD_KAFKA_RESP_ERR__VALUE_SERIALIZATION: rd_kafka_resp_err_t = -161;
pub const RD_KAFKA_RESP_ERR__KEY_SERIALIZATION: rd_kafka_resp_err_t = -162;
pub const RD_KAFKA_RESP_ERR__INTR: rd_kafka_resp_err_t = -163;
pub const RD_KAFKA_RESP_ERR__WAIT_CACHE: rd_kafka_resp_err_t = -164;
pub const RD_KAFKA_RESP_ERR__UNSUPPORTED_FEATURE: rd_kafka_resp_err_t = -165;
pub const RD_KAFKA_RESP_ERR__TIMED_OUT_QUEUE: rd_kafka_resp_err_t = -166;
pub const RD_KAFKA_RESP_ERR__OUTDATED: rd_kafka_resp_err_t = -167;
pub const RD_KAFKA_RESP_ERR__NO_OFFSET: rd_kafka_resp_err_t = -168;
pub const RD_KAFKA_RESP_ERR__AUTHENTICATION: rd_kafka_resp_err_t = -169;
pub const RD_KAFKA_RESP_ERR__NOT_IMPLEMENTED: rd_kafka_resp_err_t = -170;
pub const RD_KAFKA_RESP_ERR__UNKNOWN_PROTOCOL: rd_kafka_resp_err_t = -171;
pub const RD_KAFKA_RESP_ERR__STATE: rd_kafka_resp_err_t = -172;
pub const RD_KAFKA_RESP_ERR__CONFLICT: rd_kafka_resp_err_t = -173;
pub const RD_KAFKA_RESP_ERR__REVOKE_PARTITIONS: rd_kafka_resp_err_t = -174;
pub const RD_KAFKA_RESP_ERR__ASSIGN_PARTITIONS: rd_kafka_resp_err_t = -175;
pub const RD_KAFKA_RESP_ERR__EXISTING_SUBSCRIPTION: rd_kafka_resp_err_t = -176;
pub const RD_KAFKA_RESP_ERR__PREV_IN_PROGRESS: rd_kafka_resp_err_t = -177;
pub const RD_KAFKA_RESP_ERR__IN_PROGRESS: rd_kafka_resp_err_t = -178;
pub const RD_KAFKA_RESP_ERR__UNKNOWN_GROUP: rd_kafka_resp_err_t = -179;
pub const RD_KAFKA_RESP_ERR__WAIT_COORD: rd_kafka_resp_err_t = -180;
pub const RD_KAFKA_RESP_ERR__SSL: rd_kafka_resp_err_t = -181;
pub const RD_KAFKA_RESP_ERR__NODE_UPDATE: rd_kafka_resp_err_t = -182;
pub const RD_KAFKA_RESP_ERR__ISR_INSUFF: rd_kafka_resp_err_t = -183;
pub const RD_KAFKA_RESP_ERR__QUEUE_FULL: rd_kafka_resp_err_t = -184;
pub const RD_KAFKA_RESP_ERR__TIMED_OUT: rd_kafka_resp_err_t = -185;
pub const RD_KAFKA_RESP_ERR__INVALID_ARG: rd_kafka_resp_err_t = -186;
pub const RD_KAFKA_RESP_ERR__ALL_BROKERS_DOWN: rd_kafka_resp_err_t = -187;
pub const RD_KAFKA_RESP_ERR__UNKNOWN_TOPIC: rd_kafka_resp_err_t = -188;
pub const RD_KAFKA_RESP_ERR__FS: rd_kafka_resp_err_t = -189;
pub const RD_KAFKA_RESP_ERR__UNKNOWN_PARTITION: rd_kafka_resp_err_t = -190;
pub const RD_KAFKA_RESP_ERR__PARTITION_EOF: rd_kafka_resp_err_t = -191;
pub const RD_KAFKA_RESP_ERR__MSG_TIMED_OUT: rd_kafka_resp_err_t = -192;
pub const RD_KAFKA_RESP_ERR__RESOLVE: rd_kafka_resp_err_t = -193;
pub const RD_KAFKA_RESP_ERR__CRIT_SYS_RESOURCE: rd_kafka_resp_err_t = -194;
pub const RD_KAFKA_RESP_ERR__TRANSPORT: rd_kafka_resp_err_t = -195;
pub const RD_KAFKA_RESP_ERR__FAIL: rd_kafka_resp_err_t = -196;
pub const RD_KAFKA_RESP_ERR__DESTROY: rd_kafka_resp_err_t = -197;
pub const RD_KAFKA_RESP_ERR__BAD_COMPRESSION: rd_kafka_resp_err_t = -198;
pub const RD_KAFKA_RESP_ERR__BAD_MSG: rd_kafka_resp_err_t = -199;
pub const RD_KAFKA_RESP_ERR__BEGIN: rd_kafka_resp_err_t = -200;
pub type rd_kafka_vtype_t = libc::c_uint;
pub const RD_KAFKA_VTYPE_HEADERS: rd_kafka_vtype_t = 10;
pub const RD_KAFKA_VTYPE_HEADER: rd_kafka_vtype_t = 9;
pub const RD_KAFKA_VTYPE_TIMESTAMP: rd_kafka_vtype_t = 8;
pub const RD_KAFKA_VTYPE_MSGFLAGS: rd_kafka_vtype_t = 7;
pub const RD_KAFKA_VTYPE_OPAQUE: rd_kafka_vtype_t = 6;
pub const RD_KAFKA_VTYPE_KEY: rd_kafka_vtype_t = 5;
pub const RD_KAFKA_VTYPE_VALUE: rd_kafka_vtype_t = 4;
pub const RD_KAFKA_VTYPE_PARTITION: rd_kafka_vtype_t = 3;
pub const RD_KAFKA_VTYPE_RKT: rd_kafka_vtype_t = 2;
pub const RD_KAFKA_VTYPE_TOPIC: rd_kafka_vtype_t = 1;
pub const RD_KAFKA_VTYPE_END: rd_kafka_vtype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rd_kafka_message_s {
    pub err: rd_kafka_resp_err_t,
    pub rkt: *mut rd_kafka_topic_t,
    pub partition: int32_t,
    pub payload: *mut libc::c_void,
    pub len: size_t,
    pub key: *mut libc::c_void,
    pub key_len: size_t,
    pub offset: int64_t,
    pub _private: *mut libc::c_void,
}
pub type rd_kafka_message_t = rd_kafka_message_s;
pub type rd_kafka_conf_res_t = libc::c_int;
pub const RD_KAFKA_CONF_OK: rd_kafka_conf_res_t = 0;
pub const RD_KAFKA_CONF_INVALID: rd_kafka_conf_res_t = -1;
pub const RD_KAFKA_CONF_UNKNOWN: rd_kafka_conf_res_t = -2;
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
unsafe extern "C" fn json_incref(mut json: *mut json_t) -> *mut json_t {
    if !json.is_null() && (*json).refcount != -(1 as libc::c_int) as size_t {
        let fresh0 = &mut (*json).refcount;
        let fresh1 = 1 as libc::c_int as size_t;
        ::std::intrinsics::atomic_xadd_acquire(fresh0, fresh1) + fresh1;
    }
    return json;
}
#[inline]
unsafe extern "C" fn json_decref(mut json: *mut json_t) {
    if !json.is_null() && (*json).refcount != -(1 as libc::c_int) as size_t
        && {
            let fresh2 = &mut (*json).refcount as *mut size_t;
            let fresh3 = 1 as libc::c_int as size_t;
            ::std::intrinsics::atomic_xsub_release(fresh2, fresh3) - fresh3
                == 0 as libc::c_int as libc::c_ulong
        }
    {
        json_delete(json);
    }
}
#[inline]
unsafe extern "C" fn json_object_set_nocheck(
    mut object: *mut json_t,
    mut key: *const libc::c_char,
    mut value: *mut json_t,
) -> libc::c_int {
    return json_object_set_new_nocheck(object, key, json_incref(value));
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
static mut errstr: [libc::c_char; 512] = [0; 512];
unsafe extern "C" fn kafka_shutdown(mut backend: *mut libc::c_void) -> libc::c_int {
    let mut self_0: *mut brubeck_kafka = backend as *mut brubeck_kafka;
    let mut err: rd_kafka_resp_err_t = RD_KAFKA_RESP_ERR_NO_ERROR;
    (*self_0).connected = 0 as libc::c_int != 0;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (if !((*self_0).documents).is_null() {
            *((*self_0).documents as *mut size_t).offset(-(2 as libc::c_int) as isize)
        } else {
            0 as libc::c_int as size_t
        })
    {
        if !(*((*self_0).documents).offset(i as isize)).is_null() {
            json_decref((**((*self_0).documents).offset(i as isize)).json);
        }
        i += 1;
        i;
    }
    err = rd_kafka_fatal_error(
        (*self_0).rk,
        errstr.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
    );
    if err as u64 != 0 {
        gh_log_write(
            b"instance=%s backend=kafka event=fatal_error reason=%s msg=\"%s\"\n\0"
                as *const u8 as *const libc::c_char,
            gh_log_instance(),
            rd_kafka_err2name(err),
            errstr.as_mut_ptr(),
        );
    }
    gh_log_write(
        b"instance=%s backend=kafka event=flushing-outstanding-messages\n\0" as *const u8
            as *const libc::c_char,
        gh_log_instance(),
    );
    rd_kafka_flush((*self_0).rk, 10 as libc::c_int * 1000 as libc::c_int);
    rd_kafka_destroy((*self_0).rk);
    return err as libc::c_int;
}
unsafe extern "C" fn dr_msg_cb(
    mut rk: *mut rd_kafka_t,
    mut rkmessage: *const rd_kafka_message_t,
    mut opaque: *mut libc::c_void,
) {
    if (*rkmessage).err as u64 != 0 {
        gh_log_write(
            b"instance=%s backend=kafka event=delivery_error msg=\"%s\"\n\0" as *const u8
                as *const libc::c_char,
            gh_log_instance(),
            rd_kafka_err2name((*rkmessage).err),
        );
    }
}
unsafe extern "C" fn error_cb(
    mut rk: *mut rd_kafka_t,
    mut err: libc::c_int,
    mut reason: *const libc::c_char,
    mut opaque: *mut libc::c_void,
) {
    let mut self_0: *mut brubeck_kafka = opaque as *mut brubeck_kafka;
    gh_log_write(
        b"instance=%s backend=kafka event=producer_error reason=%s msg=\"%s\"\n\0"
            as *const u8 as *const libc::c_char,
        gh_log_instance(),
        rd_kafka_err2name(err as rd_kafka_resp_err_t),
        reason,
    );
    if err != RD_KAFKA_RESP_ERR__FATAL as libc::c_int {
        return;
    }
    if kafka_shutdown(self_0 as *mut libc::c_void) != 0 {
        exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn kafka_is_connected(mut backend: *mut libc::c_void) -> bool {
    let mut self_0: *mut brubeck_kafka = backend as *mut brubeck_kafka;
    return (*self_0).connected;
}
unsafe extern "C" fn kafka_connect(mut backend: *mut libc::c_void) -> libc::c_int {
    let mut self_0: *mut brubeck_kafka = backend as *mut brubeck_kafka;
    if (*self_0).connected {
        return 0 as libc::c_int
    } else {
        return -(1 as libc::c_int)
    };
}
unsafe extern "C" fn each_metric(
    mut metric: *const brubeck_metric,
    mut key: *const libc::c_char,
    mut value: value_t,
    mut backend: *mut libc::c_void,
) {
    let mut self_0: *mut brubeck_kafka = backend as *mut brubeck_kafka;
    let mut tag_index: uint32_t = 0 as libc::c_int as uint32_t;
    if !((*metric).tags).is_null() {
        tag_index = (*(*metric).tags).index;
    }
    let mut doc: *mut brubeck_kafka_document = if tag_index as libc::c_ulong
        >= (if !((*self_0).documents).is_null() {
            *((*self_0).documents as *mut size_t).offset(-(2 as libc::c_int) as isize)
        } else {
            0 as libc::c_int as size_t
        })
    {
        0 as *mut brubeck_kafka_document
    } else {
        *((*self_0).documents).offset(tag_index as isize)
    };
    if doc.is_null() {
        doc = malloc(::std::mem::size_of::<brubeck_kafka_document>() as libc::c_ulong)
            as *mut brubeck_kafka_document;
        (*doc).is_dirty = 0 as libc::c_int != 0;
        (*doc).json = json_object();
        let mut __cap: size_t = if !((*self_0).documents).is_null() {
            *((*self_0).documents as *mut size_t).offset(-(1 as libc::c_int) as isize)
        } else {
            0 as libc::c_int as size_t
        };
        if __cap <= tag_index as libc::c_ulong {
            if __cap == 0 {
                __cap = 1 as libc::c_int as size_t;
            }
            while __cap <= tag_index as libc::c_ulong {
                __cap = __cap << 1 as libc::c_int;
            }
            let __size: size_t = __cap
                .wrapping_mul(
                    ::std::mem::size_of::<*mut brubeck_kafka_document>() as libc::c_ulong,
                )
                .wrapping_add(
                    (::std::mem::size_of::<size_t>() as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong),
                );
            if ((*self_0).documents).is_null() {
                let mut __p: *mut size_t = malloc(__size) as *mut size_t;
                memset(__p as *mut libc::c_void, 0 as libc::c_int, __size);
                (*self_0)
                    .documents = &mut *__p.offset(2 as libc::c_int as isize)
                    as *mut size_t as *mut libc::c_void
                    as *mut *mut brubeck_kafka_document;
                if !((*self_0).documents).is_null() {
                    *((*self_0).documents as *mut size_t)
                        .offset(-(1 as libc::c_int) as isize) = __cap;
                }
                if !((*self_0).documents).is_null() {
                    *((*self_0).documents as *mut size_t)
                        .offset(
                            -(2 as libc::c_int) as isize,
                        ) = 0 as libc::c_int as size_t;
                }
            } else {
                let __prev_size: size_t = (if !((*self_0).documents).is_null() {
                    *((*self_0).documents as *mut size_t)
                        .offset(-(1 as libc::c_int) as isize)
                } else {
                    0 as libc::c_int as size_t
                })
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut brubeck_kafka_document>()
                            as libc::c_ulong,
                    )
                    .wrapping_add(
                        (::std::mem::size_of::<size_t>() as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong),
                    );
                let mut __p1: *mut size_t = &mut *((*self_0).documents as *mut size_t)
                    .offset(-(2 as libc::c_int) as isize) as *mut size_t;
                let mut __p2: *mut size_t = realloc(__p1 as *mut libc::c_void, __size)
                    as *mut size_t;
                memset(
                    (__p2 as *mut libc::c_char).offset(__prev_size as isize)
                        as *mut libc::c_void,
                    0 as libc::c_int,
                    __size.wrapping_sub(__prev_size),
                );
                (*self_0)
                    .documents = &mut *__p2.offset(2 as libc::c_int as isize)
                    as *mut size_t as *mut libc::c_void
                    as *mut *mut brubeck_kafka_document;
                if !((*self_0).documents).is_null() {
                    *((*self_0).documents as *mut size_t)
                        .offset(-(1 as libc::c_int) as isize) = __cap;
                }
            }
        }
        let ref mut fresh4 = *((*self_0).documents).offset(tag_index as isize);
        *fresh4 = doc;
        if !((*self_0).documents).is_null() {
            *((*self_0).documents as *mut size_t)
                .offset(
                    -(2 as libc::c_int) as isize,
                ) = tag_index.wrapping_add(1 as libc::c_int as libc::c_uint) as size_t;
        }
    }
    if (*doc).is_dirty as libc::c_int == 0 as libc::c_int && !((*metric).tags).is_null()
        && (*(*metric).tags).num_tags as libc::c_int > 0 as libc::c_int
    {
        let mut tag_destination: *mut json_t = (*doc).json;
        if !((*self_0).tag_subdocument).is_null() {
            tag_destination = json_object();
            json_object_set_new_nocheck(
                (*doc).json,
                (*self_0).tag_subdocument,
                tag_destination,
            );
        }
        let mut i: uint16_t = 0;
        i = 0 as libc::c_int as uint16_t;
        while (i as libc::c_int) < (*(*metric).tags).num_tags as libc::c_int {
            json_object_set_new_nocheck(
                tag_destination,
                (*((*(*metric).tags).tags).as_ptr().offset(i as isize)).key,
                json_string(
                    (*((*(*metric).tags).tags).as_ptr().offset(i as isize)).value,
                ),
            );
            i = i.wrapping_add(1);
            i;
        }
    }
    json_object_set_new_nocheck((*doc).json, key, json_real(value));
    (*doc).is_dirty = 1 as libc::c_int != 0;
}
unsafe extern "C" fn kafka_flush(mut backend: *mut libc::c_void) {
    let mut self_0: *mut brubeck_kafka = backend as *mut brubeck_kafka;
    let mut err: rd_kafka_resp_err_t = RD_KAFKA_RESP_ERR_NO_ERROR;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut epoch_ms: int64_t = 0;
    let mut json_epoch_ms: *mut json_t = 0 as *mut json_t;
    epoch_ms = (*self_0).backend.tick_time as int64_t;
    epoch_ms *= 1000 as libc::c_int as libc::c_long;
    json_epoch_ms = json_integer(epoch_ms as json_int_t);
    let mut i: libc::c_int = 0;
    let mut doc: *mut brubeck_kafka_document = 0 as *mut brubeck_kafka_document;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (if !((*self_0).documents).is_null() {
            *((*self_0).documents as *mut size_t).offset(-(2 as libc::c_int) as isize)
        } else {
            0 as libc::c_int as size_t
        })
    {
        doc = *((*self_0).documents).offset(i as isize);
        if !doc.is_null() && (*doc).is_dirty as libc::c_int != 0 {
            json_object_set_nocheck(
                (*doc).json,
                b"@timestamp\0" as *const u8 as *const libc::c_char,
                json_epoch_ms,
            );
            buf = json_dumps((*doc).json, 0x20 as libc::c_int as size_t);
            len = strlen(buf);
            err = rd_kafka_producev(
                (*self_0).rk,
                ({ RD_KAFKA_VTYPE_TOPIC as libc::c_int }),
                (*self_0).topic,
                ({ RD_KAFKA_VTYPE_MSGFLAGS as libc::c_int }),
                0x1 as libc::c_int,
                ({ RD_KAFKA_VTYPE_VALUE as libc::c_int }),
                buf as *mut libc::c_void,
                len,
                ({ RD_KAFKA_VTYPE_OPAQUE as libc::c_int }),
                0 as *mut libc::c_void,
                RD_KAFKA_VTYPE_END as libc::c_int,
            );
            if err as u64 != 0 {
                gh_log_write(
                    b"instance=%s backend=kafka event=failed_to_enqueue msg=\"%s\"\n\0"
                        as *const u8 as *const libc::c_char,
                    gh_log_instance(),
                    rd_kafka_err2str(err),
                );
                free(buf as *mut libc::c_void);
            } else {
                (*self_0)
                    .bytes_sent = ((*self_0).bytes_sent as libc::c_ulong)
                    .wrapping_add(len) as size_t as size_t;
            }
            json_object_clear((*doc).json);
            (*doc).is_dirty = 0 as libc::c_int != 0;
            rd_kafka_poll((*self_0).rk, 0 as libc::c_int);
        }
        i += 1;
        i;
    }
    json_decref(json_epoch_ms);
}
unsafe extern "C" fn build_rdkafka_config(
    mut json: *mut json_t,
) -> *mut rd_kafka_conf_t {
    let mut conf: *mut rd_kafka_conf_t = 0 as *mut rd_kafka_conf_t;
    let mut retval: libc::c_int = 0;
    let mut key: *const libc::c_char = 0 as *const libc::c_char;
    let mut value: *mut json_t = 0 as *mut json_t;
    conf = rd_kafka_conf_new();
    key = json_object_iter_key(json_object_iter(json));
    while !key.is_null()
        && {
            value = json_object_iter_value(json_object_key_to_iter(key));
            !value.is_null()
        }
    {
        retval = rd_kafka_conf_set(
            conf,
            key,
            json_string_value(value),
            errstr.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
        ) as libc::c_int;
        if retval != RD_KAFKA_CONF_OK as libc::c_int {
            gh_log_write(
                b"instance=%s backend=kafka event=conf_error key=%s code=%s msg=\"%s\"\n\0"
                    as *const u8 as *const libc::c_char,
                gh_log_instance(),
                key,
                rd_kafka_err2name(retval as rd_kafka_resp_err_t),
                errstr.as_mut_ptr(),
            );
            rd_kafka_conf_destroy(conf);
            exit(1 as libc::c_int);
        }
        key = json_object_iter_key(
            json_object_iter_next(json, json_object_key_to_iter(key)),
        );
    }
    return conf;
}
pub unsafe extern "C" fn brubeck_kafka_new(
    mut server: *mut brubeck_server,
    mut settings: *mut json_t,
    mut shard_n: libc::c_int,
) -> *mut brubeck_backend {
    let mut self_0: *mut brubeck_kafka = xmalloc(
        ::std::mem::size_of::<brubeck_kafka>() as libc::c_ulong,
    ) as *mut brubeck_kafka;
    memset(
        self_0 as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<brubeck_kafka>() as libc::c_ulong,
    );
    let mut frequency: libc::c_int = 0 as libc::c_int;
    let mut rdkafka_config: *mut json_t = 0 as *mut json_t;
    let mut conf: *mut rd_kafka_conf_t = 0 as *mut rd_kafka_conf_t;
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
        b"{s:s, s:i, s:o, s?:s}\0" as *const u8 as *const libc::c_char,
        b"topic\0" as *const u8 as *const libc::c_char,
        &mut (*self_0).topic as *mut *const libc::c_char,
        b"frequency\0" as *const u8 as *const libc::c_char,
        &mut frequency as *mut libc::c_int,
        b"rdkafka_config\0" as *const u8 as *const libc::c_char,
        &mut rdkafka_config as *mut *mut json_t,
        b"tag_subdocument\0" as *const u8 as *const libc::c_char,
        &mut (*self_0).tag_subdocument as *mut *const libc::c_char,
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"[FATAL]: config error: %s\n\0" as *const u8 as *const libc::c_char,
            (_error_j.text).as_mut_ptr(),
        );
        gh_log_die();
    }
    conf = build_rdkafka_config(rdkafka_config);
    (*self_0).connected = 1 as libc::c_int != 0;
    (*self_0).backend.type_0 = BRUBECK_BACKEND_KAFKA;
    (*self_0)
        .backend
        .connect = Some(
        kafka_connect as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
    );
    (*self_0)
        .backend
        .is_connected = Some(
        kafka_is_connected as unsafe extern "C" fn(*mut libc::c_void) -> bool,
    );
    (*self_0)
        .backend
        .sample = Some(
        each_metric
            as unsafe extern "C" fn(
                *const brubeck_metric,
                *const libc::c_char,
                value_t,
                *mut libc::c_void,
            ) -> (),
    );
    (*self_0)
        .backend
        .flush = Some(kafka_flush as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*self_0).backend.sample_freq = frequency;
    (*self_0).backend.server = server;
    rd_kafka_conf_set_dr_msg_cb(
        conf,
        Some(
            dr_msg_cb
                as unsafe extern "C" fn(
                    *mut rd_kafka_t,
                    *const rd_kafka_message_t,
                    *mut libc::c_void,
                ) -> (),
        ),
    );
    rd_kafka_conf_set_error_cb(
        conf,
        Some(
            error_cb
                as unsafe extern "C" fn(
                    *mut rd_kafka_t,
                    libc::c_int,
                    *const libc::c_char,
                    *mut libc::c_void,
                ) -> (),
        ),
    );
    rd_kafka_conf_set_opaque(conf, self_0 as *mut libc::c_void);
    (*self_0)
        .rk = rd_kafka_new(
        RD_KAFKA_PRODUCER,
        conf,
        errstr.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
    );
    if ((*self_0).rk).is_null() {
        gh_log_write(
            b"instance=%s backend=kafka event=producer_creation_failed error=\"%s\"\n\0"
                as *const u8 as *const libc::c_char,
            gh_log_instance(),
            errstr.as_mut_ptr(),
        );
        rd_kafka_conf_destroy(conf);
        exit(1 as libc::c_int);
    }
    gh_log_write(
        b"instance=%s backend=kafka event=ready\n\0" as *const u8 as *const libc::c_char,
        gh_log_instance(),
    );
    brubeck_backend_run_threaded(self_0 as *mut brubeck_backend);
    gh_log_write(
        b"instance=%s backend=kafka event=started\n\0" as *const u8
            as *const libc::c_char,
        gh_log_instance(),
    );
    return self_0 as *mut brubeck_backend;
}
