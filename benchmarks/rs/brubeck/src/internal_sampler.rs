use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type brubeck_tags_t;
    pub type brubeck_hashtable_t;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn gh_log_die() -> !;
    fn brubeck_metric_new(
        server: *mut brubeck_server,
        _: *const libc::c_char,
        _: size_t,
        _: uint8_t,
    ) -> *mut brubeck_metric;
    fn brubeck_metric_shard(
        server: *mut brubeck_server,
        _: *mut brubeck_metric,
    ) -> *mut brubeck_backend;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type brubeck_metric_t = libc::c_uint;
pub const BRUBECK_MT_INTERNAL_STATS: brubeck_metric_t = 5;
pub const BRUBECK_MT_TIMER: brubeck_metric_t = 4;
pub const BRUBECK_MT_HISTO: brubeck_metric_t = 3;
pub const BRUBECK_MT_COUNTER: brubeck_metric_t = 2;
pub const BRUBECK_MT_METER: brubeck_metric_t = 1;
pub const BRUBECK_MT_GAUGE: brubeck_metric_t = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const BRUBECK_STATE_ACTIVE: C2RustUnnamed_3 = 2;
pub const BRUBECK_STATE_INACTIVE: C2RustUnnamed_3 = 1;
pub const BRUBECK_STATE_DISABLED: C2RustUnnamed_3 = 0;
pub type brubeck_sample_cb = Option::<
    unsafe extern "C" fn(
        *const brubeck_metric,
        *const libc::c_char,
        value_t,
        *mut libc::c_void,
    ) -> (),
>;
#[inline]
unsafe extern "C" fn brubeck_metric_set_state(
    mut metric: *mut brubeck_metric,
    state: uint8_t,
) {
    ::std::intrinsics::atomic_store_seqcst(&mut (*metric).private_state, state);
}
pub unsafe extern "C" fn brubeck_internal__sample(
    mut metric: *mut brubeck_metric,
    mut sample: brubeck_sample_cb,
    mut opaque: *mut libc::c_void,
) {
    let mut stats: *mut brubeck_internal_stats = (*metric).as_0.other
        as *mut brubeck_internal_stats;
    let mut value: uint32_t = 0;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        ((*metric).key_len as libc::c_ulong)
            .wrapping_add(strlen(b".unique_keys\0" as *const u8 as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
    );
    let mut key: *mut libc::c_char = fresh0.leak().as_mut_ptr() as *mut libc::c_char;
    memcpy(
        key as *mut libc::c_void,
        ((*metric).key).as_mut_ptr() as *const libc::c_void,
        (*metric).key_len as libc::c_ulong,
    );
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".metrics\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        (strlen(b".metrics\0" as *const u8 as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    value = ::std::intrinsics::atomic_xchg_acquire(
        &mut (*stats).live.metrics,
        0 as libc::c_int as uint32_t,
    );
    (*stats).sample.metrics = value;
    sample.unwrap()(metric, key, value as value_t, opaque);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".errors\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        (strlen(b".errors\0" as *const u8 as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    value = ::std::intrinsics::atomic_xchg_acquire(
        &mut (*stats).live.errors,
        0 as libc::c_int as uint32_t,
    );
    (*stats).sample.errors = value;
    sample.unwrap()(metric, key, value as value_t, opaque);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".unique_keys\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        (strlen(b".unique_keys\0" as *const u8 as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    let fresh1 = &mut (*stats).live.unique_keys;
    let fresh2 = 0 as libc::c_int as uint32_t;
    value = ::std::intrinsics::atomic_xadd_seqcst(fresh1, fresh2) + fresh2;
    (*stats).sample.unique_keys = value;
    sample.unwrap()(metric, key, value as value_t, opaque);
    brubeck_metric_set_state(metric, BRUBECK_STATE_ACTIVE as libc::c_int as uint8_t);
}
pub unsafe extern "C" fn brubeck_internal__init(mut server: *mut brubeck_server) {
    let mut internal: *mut brubeck_metric = 0 as *mut brubeck_metric;
    let mut backend: *mut brubeck_backend = 0 as *mut brubeck_backend;
    internal = brubeck_metric_new(
        server,
        (*server).name,
        strlen((*server).name),
        BRUBECK_MT_INTERNAL_STATS as libc::c_int as uint8_t,
    );
    if internal.is_null() {
        fprintf(
            stderr,
            b"[FATAL]: Failed to initialize internal stats sampler\n\0" as *const u8
                as *const libc::c_char,
        );
        gh_log_die();
    }
    (*internal)
        .as_0
        .other = &mut (*server).internal_stats as *mut brubeck_internal_stats
        as *mut libc::c_void;
    backend = brubeck_metric_shard(server, internal);
    (*server).internal_stats.sample_freq = (*backend).sample_freq;
}
