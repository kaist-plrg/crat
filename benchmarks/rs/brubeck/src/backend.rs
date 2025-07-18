use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type brubeck_tags_t;
    pub type brubeck_hashtable_t;
    fn clock_nanosleep(
        __clock_id: clockid_t,
        __flags: libc::c_int,
        __req: *const timespec,
        __rem: *mut timespec,
    ) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn gh_log_die() -> !;
    fn brubeck_metric_sample(
        metric: *mut brubeck_metric,
        cb: brubeck_sample_cb,
        backend: *mut libc::c_void,
    );
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type clockid_t = __clockid_t;
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
pub const BRUBECK_STATE_DISABLED: C2RustUnnamed_3 = 0;
pub type brubeck_sample_cb = Option::<
    unsafe extern "C" fn(
        *const brubeck_metric,
        *const libc::c_char,
        value_t,
        *mut libc::c_void,
    ) -> (),
>;
pub const BRUBECK_STATE_INACTIVE: C2RustUnnamed_3 = 1;
pub const BRUBECK_STATE_ACTIVE: C2RustUnnamed_3 = 2;
pub type C2RustUnnamed_3 = libc::c_uint;
#[inline]
unsafe extern "C" fn brubeck_metric_get_state(
    mut metric: *const brubeck_metric,
) -> uint8_t {
    return ::std::intrinsics::atomic_load_seqcst(&(*metric).private_state);
}
#[inline]
unsafe extern "C" fn brubeck_metric_set_state_if_equal(
    mut metric: *mut brubeck_metric,
    mut expected: uint8_t,
    state: uint8_t,
) -> bool {
    let fresh0 = ::std::intrinsics::atomic_cxchg_seqcst_seqcst(
        &mut (*metric).private_state,
        *&mut expected,
        state,
    );
    *&mut expected = fresh0.0;
    return fresh0.1;
}
pub unsafe extern "C" fn brubeck_backend_register_metric(
    mut self_0: *mut brubeck_backend,
    mut metric: *mut brubeck_metric,
) {
    loop {
        let mut next: *mut brubeck_metric = (*self_0).queue;
        (*metric).next = next;
        if (::std::intrinsics::atomic_cxchg_seqcst_seqcst(
            &mut (*self_0).queue as *mut *mut brubeck_metric,
            next,
            metric,
        ))
            .1
        {
            break;
        }
    };
}
unsafe extern "C" fn backend__thread(mut _ptr: *mut libc::c_void) -> *mut libc::c_void {
    let mut self_0: *mut brubeck_backend = _ptr as *mut brubeck_backend;
    loop {
        let mut now: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        let mut then: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        clock_gettime(1 as libc::c_int, &mut then);
        then.tv_sec += (*self_0).sample_freq as libc::c_long;
        if ((*self_0).connect).unwrap()(self_0 as *mut libc::c_void) == 0 {
            let mut mt: *mut brubeck_metric = 0 as *mut brubeck_metric;
            clock_gettime(0 as libc::c_int, &mut now);
            (*self_0).tick_time = now.tv_sec as uint32_t;
            mt = (*self_0).queue;
            while !mt.is_null() {
                let state: uint8_t = brubeck_metric_get_state(mt);
                if state as libc::c_int == BRUBECK_STATE_ACTIVE as libc::c_int {
                    brubeck_metric_sample(
                        mt,
                        (*self_0).sample,
                        self_0 as *mut libc::c_void,
                    );
                    brubeck_metric_set_state_if_equal(
                        mt,
                        state,
                        BRUBECK_STATE_INACTIVE as libc::c_int as uint8_t,
                    );
                } else if state as libc::c_int == BRUBECK_STATE_INACTIVE as libc::c_int {
                    brubeck_metric_sample(
                        mt,
                        (*self_0).sample,
                        self_0 as *mut libc::c_void,
                    );
                    brubeck_metric_set_state_if_equal(
                        mt,
                        state,
                        BRUBECK_STATE_DISABLED as libc::c_int as uint8_t,
                    );
                }
                mt = (*mt).next;
            }
            if ((*self_0).flush).is_some() {
                ((*self_0).flush).unwrap()(self_0 as *mut libc::c_void);
            }
        }
        clock_nanosleep(
            1 as libc::c_int,
            1 as libc::c_int,
            &mut then,
            0 as *mut timespec,
        );
    };
}
pub unsafe extern "C" fn brubeck_backend_run_threaded(mut self_0: *mut brubeck_backend) {
    if pthread_create(
        &mut (*self_0).thread,
        0 as *const pthread_attr_t,
        Some(
            backend__thread
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        self_0 as *mut libc::c_void,
    ) != 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"[FATAL]: failed to start backend thread\n\0" as *const u8
                as *const libc::c_char,
        );
        gh_log_die();
    }
}
