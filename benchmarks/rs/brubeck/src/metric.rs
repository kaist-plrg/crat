use ::libc;
extern "C" {
    pub type brubeck_tags_t;
    pub type brubeck_hashtable_t;
    fn pthread_spin_init(
        __lock: *mut pthread_spinlock_t,
        __pshared: libc::c_int,
    ) -> libc::c_int;
    fn pthread_spin_lock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    fn pthread_spin_unlock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
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
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn brubeck_backend_register_metric(
        self_0: *mut brubeck_backend,
        metric: *mut brubeck_metric,
    );
    fn brubeck_histo_push(
        histo: *mut brubeck_histo,
        value: value_t,
        sample_rate: value_t,
    );
    fn brubeck_histo_sample(
        sample: *mut brubeck_histo_sample,
        histo: *mut brubeck_histo,
    );
    fn brubeck_hashtable_find(
        ht: *mut brubeck_hashtable_t,
        key: *const libc::c_char,
        key_len: uint16_t,
    ) -> *mut brubeck_metric;
    fn brubeck_hashtable_insert(
        ht: *mut brubeck_hashtable_t,
        key: *const libc::c_char,
        key_len: uint16_t,
        val: *mut brubeck_metric,
    ) -> bool;
    fn brubeck_internal__sample(
        metric: *mut brubeck_metric,
        sample: brubeck_sample_cb,
        opaque: *mut libc::c_void,
    );
    fn CityHash32(s: *const libc::c_char, len: size_t) -> uint32_t;
    fn brubeck_slab_alloc(slab: *mut brubeck_slab, need: size_t) -> *mut libc::c_void;
    fn brubeck_get_tag_set(
        _: *mut brubeck_tags_t,
        key_str: *const libc::c_char,
        key_len: uint16_t,
    ) -> *const brubeck_tag_set;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
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
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_PROCESS_SHARED: C2RustUnnamed = 1;
pub const PTHREAD_PROCESS_PRIVATE: C2RustUnnamed = 0;
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
pub struct brubeck_histo_sample {
    pub sum: value_t,
    pub min: value_t,
    pub max: value_t,
    pub mean: value_t,
    pub median: value_t,
    pub count: value_t,
    pub percentile: [value_t; 5],
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const PC_999: C2RustUnnamed_4 = 4;
pub const PC_99: C2RustUnnamed_4 = 3;
pub const PC_98: C2RustUnnamed_4 = 2;
pub const PC_95: C2RustUnnamed_4 = 1;
pub const PC_75: C2RustUnnamed_4 = 0;
pub type brubeck_metric_mod_t = libc::c_uint;
pub const BRUBECK_MOD_RELATIVE_VALUE: brubeck_metric_mod_t = 1;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const BRUBECK_STATE_ACTIVE: C2RustUnnamed_5 = 2;
pub const BRUBECK_STATE_INACTIVE: C2RustUnnamed_5 = 1;
pub const BRUBECK_STATE_DISABLED: C2RustUnnamed_5 = 0;
pub type brubeck_sample_cb = Option::<
    unsafe extern "C" fn(
        *const brubeck_metric,
        *const libc::c_char,
        value_t,
        *mut libc::c_void,
    ) -> (),
>;
pub type mt_prototype_sample = Option::<
    unsafe extern "C" fn(*mut brubeck_metric, brubeck_sample_cb, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_metric__proto {
    pub record: mt_prototype_record,
    pub sample: mt_prototype_sample,
}
pub type mt_prototype_record = Option::<
    unsafe extern "C" fn(*mut brubeck_metric, value_t, value_t, uint8_t) -> (),
>;
#[inline]
unsafe extern "C" fn brubeck_metric_set_state(
    mut metric: *mut brubeck_metric,
    state: uint8_t,
) {
    ::std::intrinsics::atomic_store_seqcst(&mut (*metric).private_state, state);
}
#[inline]
unsafe extern "C" fn new_metric(
    mut server: *mut brubeck_server,
    mut key: *const libc::c_char,
    mut key_len: size_t,
    mut type_0: uint8_t,
) -> *mut brubeck_metric {
    let mut metric: *mut brubeck_metric = 0 as *mut brubeck_metric;
    let mut tags: *const brubeck_tag_set = 0 as *const brubeck_tag_set;
    if !((*server).tags).is_null() {
        tags = brubeck_get_tag_set((*server).tags, key, key_len as uint16_t);
        if !tags.is_null() {
            key_len = key_len.wrapping_sub((*tags).tag_len as libc::c_ulong);
        }
    }
    metric = brubeck_slab_alloc(
        &mut (*server).slab,
        (::std::mem::size_of::<brubeck_metric>() as libc::c_ulong)
            .wrapping_add(key_len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut brubeck_metric;
    memset(
        metric as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<brubeck_metric>() as libc::c_ulong,
    );
    (*metric).tags = tags;
    memcpy(
        ((*metric).key).as_mut_ptr() as *mut libc::c_void,
        key as *const libc::c_void,
        key_len,
    );
    *((*metric).key).as_mut_ptr().offset(key_len as isize) = '\0' as i32 as libc::c_char;
    (*metric).key_len = key_len as uint16_t;
    brubeck_metric_set_state(metric, BRUBECK_STATE_ACTIVE as libc::c_int as uint8_t);
    (*metric).type_0 = type_0;
    pthread_spin_init(&mut (*metric).lock, PTHREAD_PROCESS_PRIVATE as libc::c_int);
    return metric;
}
unsafe extern "C" fn gauge__record(
    mut metric: *mut brubeck_metric,
    mut value: value_t,
    mut sample_freq: value_t,
    mut modifiers: uint8_t,
) {
    pthread_spin_lock(&mut (*metric).lock);
    if modifiers as libc::c_int & BRUBECK_MOD_RELATIVE_VALUE as libc::c_int != 0 {
        (*metric).as_0.gauge.value += value;
    } else {
        (*metric).as_0.gauge.value = value;
    }
    pthread_spin_unlock(&mut (*metric).lock);
}
unsafe extern "C" fn gauge__sample(
    mut metric: *mut brubeck_metric,
    mut sample: brubeck_sample_cb,
    mut opaque: *mut libc::c_void,
) {
    let mut value: value_t = 0.;
    pthread_spin_lock(&mut (*metric).lock);
    value = (*metric).as_0.gauge.value;
    pthread_spin_unlock(&mut (*metric).lock);
    sample.unwrap()(metric, ((*metric).key).as_mut_ptr(), value, opaque);
}
unsafe extern "C" fn meter__record(
    mut metric: *mut brubeck_metric,
    mut value: value_t,
    mut sample_freq: value_t,
    mut modifiers: uint8_t,
) {
    value *= sample_freq;
    pthread_spin_lock(&mut (*metric).lock);
    (*metric).as_0.meter.value += value;
    pthread_spin_unlock(&mut (*metric).lock);
}
unsafe extern "C" fn meter__sample(
    mut metric: *mut brubeck_metric,
    mut sample: brubeck_sample_cb,
    mut opaque: *mut libc::c_void,
) {
    let mut value: value_t = 0.;
    pthread_spin_lock(&mut (*metric).lock);
    value = (*metric).as_0.meter.value;
    (*metric).as_0.meter.value = 0.0f64;
    pthread_spin_unlock(&mut (*metric).lock);
    sample.unwrap()(metric, ((*metric).key).as_mut_ptr(), value, opaque);
}
unsafe extern "C" fn counter__record(
    mut metric: *mut brubeck_metric,
    mut value: value_t,
    mut sample_freq: value_t,
    mut modifiers: uint8_t,
) {
    value *= sample_freq;
    pthread_spin_lock(&mut (*metric).lock);
    if (*metric).as_0.counter.previous > 0.0f64 {
        let mut diff: value_t = if value >= (*metric).as_0.counter.previous {
            value - (*metric).as_0.counter.previous
        } else {
            value
        };
        (*metric).as_0.counter.value += diff;
    }
    (*metric).as_0.counter.previous = value;
    pthread_spin_unlock(&mut (*metric).lock);
}
unsafe extern "C" fn counter__sample(
    mut metric: *mut brubeck_metric,
    mut sample: brubeck_sample_cb,
    mut opaque: *mut libc::c_void,
) {
    let mut value: value_t = 0.;
    pthread_spin_lock(&mut (*metric).lock);
    value = (*metric).as_0.counter.value;
    (*metric).as_0.counter.value = 0.0f64;
    pthread_spin_unlock(&mut (*metric).lock);
    sample.unwrap()(metric, ((*metric).key).as_mut_ptr(), value, opaque);
}
unsafe extern "C" fn histogram__record(
    mut metric: *mut brubeck_metric,
    mut value: value_t,
    mut sample_freq: value_t,
    mut modifiers: uint8_t,
) {
    pthread_spin_lock(&mut (*metric).lock);
    brubeck_histo_push(&mut (*metric).as_0.histogram, value, sample_freq);
    pthread_spin_unlock(&mut (*metric).lock);
}
unsafe extern "C" fn histogram__sample(
    mut metric: *mut brubeck_metric,
    mut sample: brubeck_sample_cb,
    mut opaque: *mut libc::c_void,
) {
    let mut hsample: brubeck_histo_sample = brubeck_histo_sample {
        sum: 0.,
        min: 0.,
        max: 0.,
        mean: 0.,
        median: 0.,
        count: 0.,
        percentile: [0.; 5],
    };
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    pthread_spin_lock(&mut (*metric).lock);
    brubeck_histo_sample(&mut hsample, &mut (*metric).as_0.histogram);
    pthread_spin_unlock(&mut (*metric).lock);
    let mut fresh0 = ::std::vec::from_elem(
        0,
        ((*metric).key_len as libc::c_ulong)
            .wrapping_add(
                strlen(b".percentile.999\0" as *const u8 as *const libc::c_char),
            )
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
    );
    key = fresh0.leak().as_mut_ptr() as *mut libc::c_char;
    memcpy(
        key as *mut libc::c_void,
        ((*metric).key).as_mut_ptr() as *const libc::c_void,
        (*metric).key_len as libc::c_ulong,
    );
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".count\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        (strlen(b".count\0" as *const u8 as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    sample.unwrap()(metric, key, hsample.count, opaque);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".count_ps\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        (strlen(b".count_ps\0" as *const u8 as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    let mut backend: *mut brubeck_backend = opaque as *mut brubeck_backend;
    sample
        .unwrap()(
        metric,
        key,
        hsample.count / (*backend).sample_freq as libc::c_double,
        opaque,
    );
    if hsample.count == 0.0f64 {
        return;
    }
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".min\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        (strlen(b".min\0" as *const u8 as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    sample.unwrap()(metric, key, hsample.min, opaque);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".max\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        (strlen(b".max\0" as *const u8 as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    sample.unwrap()(metric, key, hsample.max, opaque);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".sum\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        (strlen(b".sum\0" as *const u8 as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    sample.unwrap()(metric, key, hsample.sum, opaque);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".mean\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        (strlen(b".mean\0" as *const u8 as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    sample.unwrap()(metric, key, hsample.mean, opaque);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".median\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        (strlen(b".median\0" as *const u8 as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    sample.unwrap()(metric, key, hsample.median, opaque);
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".percentile.75\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        (strlen(b".percentile.75\0" as *const u8 as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    sample
        .unwrap()(
        metric,
        key,
        hsample.percentile[PC_75 as libc::c_int as usize],
        opaque,
    );
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".percentile.95\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        (strlen(b".percentile.95\0" as *const u8 as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    sample
        .unwrap()(
        metric,
        key,
        hsample.percentile[PC_95 as libc::c_int as usize],
        opaque,
    );
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".percentile.98\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        (strlen(b".percentile.98\0" as *const u8 as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    sample
        .unwrap()(
        metric,
        key,
        hsample.percentile[PC_98 as libc::c_int as usize],
        opaque,
    );
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".percentile.99\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        (strlen(b".percentile.99\0" as *const u8 as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    sample
        .unwrap()(
        metric,
        key,
        hsample.percentile[PC_99 as libc::c_int as usize],
        opaque,
    );
    memcpy(
        key.offset((*metric).key_len as libc::c_int as isize) as *mut libc::c_void,
        b".percentile.999\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        (strlen(b".percentile.999\0" as *const u8 as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    sample
        .unwrap()(
        metric,
        key,
        hsample.percentile[PC_999 as libc::c_int as usize],
        opaque,
    );
}
static mut _prototypes: [brubeck_metric__proto; 6] = unsafe {
    [
        {
            let mut init = brubeck_metric__proto {
                record: Some(
                    gauge__record
                        as unsafe extern "C" fn(
                            *mut brubeck_metric,
                            value_t,
                            value_t,
                            uint8_t,
                        ) -> (),
                ),
                sample: Some(
                    gauge__sample
                        as unsafe extern "C" fn(
                            *mut brubeck_metric,
                            brubeck_sample_cb,
                            *mut libc::c_void,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = brubeck_metric__proto {
                record: Some(
                    meter__record
                        as unsafe extern "C" fn(
                            *mut brubeck_metric,
                            value_t,
                            value_t,
                            uint8_t,
                        ) -> (),
                ),
                sample: Some(
                    meter__sample
                        as unsafe extern "C" fn(
                            *mut brubeck_metric,
                            brubeck_sample_cb,
                            *mut libc::c_void,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = brubeck_metric__proto {
                record: Some(
                    counter__record
                        as unsafe extern "C" fn(
                            *mut brubeck_metric,
                            value_t,
                            value_t,
                            uint8_t,
                        ) -> (),
                ),
                sample: Some(
                    counter__sample
                        as unsafe extern "C" fn(
                            *mut brubeck_metric,
                            brubeck_sample_cb,
                            *mut libc::c_void,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = brubeck_metric__proto {
                record: Some(
                    histogram__record
                        as unsafe extern "C" fn(
                            *mut brubeck_metric,
                            value_t,
                            value_t,
                            uint8_t,
                        ) -> (),
                ),
                sample: Some(
                    histogram__sample
                        as unsafe extern "C" fn(
                            *mut brubeck_metric,
                            brubeck_sample_cb,
                            *mut libc::c_void,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = brubeck_metric__proto {
                record: Some(
                    histogram__record
                        as unsafe extern "C" fn(
                            *mut brubeck_metric,
                            value_t,
                            value_t,
                            uint8_t,
                        ) -> (),
                ),
                sample: Some(
                    histogram__sample
                        as unsafe extern "C" fn(
                            *mut brubeck_metric,
                            brubeck_sample_cb,
                            *mut libc::c_void,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = brubeck_metric__proto {
                record: None,
                sample: Some(
                    brubeck_internal__sample
                        as unsafe extern "C" fn(
                            *mut brubeck_metric,
                            brubeck_sample_cb,
                            *mut libc::c_void,
                        ) -> (),
                ),
            };
            init
        },
    ]
};
pub unsafe extern "C" fn brubeck_metric_sample(
    mut metric: *mut brubeck_metric,
    mut cb: brubeck_sample_cb,
    mut backend: *mut libc::c_void,
) {
    (_prototypes[(*metric).type_0 as usize].sample).unwrap()(metric, cb, backend);
}
pub unsafe extern "C" fn brubeck_metric_record(
    mut metric: *mut brubeck_metric,
    mut value: value_t,
    mut sample_freq: value_t,
    mut modifiers: uint8_t,
) {
    brubeck_metric_set_state(metric, BRUBECK_STATE_ACTIVE as libc::c_int as uint8_t);
    (_prototypes[(*metric).type_0 as usize].record)
        .unwrap()(metric, value, sample_freq, modifiers);
}
pub unsafe extern "C" fn brubeck_metric_shard(
    mut server: *mut brubeck_server,
    mut metric: *mut brubeck_metric,
) -> *mut brubeck_backend {
    let mut shard: libc::c_int = 0 as libc::c_int;
    if (*server).active_backends > 1 as libc::c_int {
        shard = (CityHash32(((*metric).key).as_mut_ptr(), (*metric).key_len as size_t))
            .wrapping_rem((*server).active_backends as libc::c_uint) as libc::c_int;
    }
    return (*server).backends[shard as usize];
}
pub unsafe extern "C" fn brubeck_metric_new(
    mut server: *mut brubeck_server,
    mut key: *const libc::c_char,
    mut key_len: size_t,
    mut type_0: uint8_t,
) -> *mut brubeck_metric {
    let mut metric: *mut brubeck_metric = 0 as *mut brubeck_metric;
    let mut key_for_ht: *mut libc::c_char = strndup(key, key_len);
    metric = new_metric(server, key, key_len, type_0);
    if metric.is_null() {
        return 0 as *mut brubeck_metric;
    }
    if !brubeck_hashtable_insert(
        (*server).metrics,
        key_for_ht,
        key_len as uint16_t,
        metric,
    ) {
        free(key_for_ht as *mut libc::c_void);
        return brubeck_hashtable_find((*server).metrics, key, key_len as uint16_t);
    }
    brubeck_backend_register_metric(brubeck_metric_shard(server, metric), metric);
    let fresh1 = &mut (*server).internal_stats.live.unique_keys;
    let fresh2 = 1 as libc::c_int as uint32_t;
    ::std::intrinsics::atomic_xadd_seqcst(fresh1, fresh2) + fresh2;
    return metric;
}
pub unsafe extern "C" fn brubeck_metric_find(
    mut server: *mut brubeck_server,
    mut key: *const libc::c_char,
    mut key_len: size_t,
    mut type_0: uint8_t,
) -> *mut brubeck_metric {
    let mut metric: *mut brubeck_metric = 0 as *mut brubeck_metric;
    metric = brubeck_hashtable_find((*server).metrics, key, key_len as uint16_t);
    if (metric == 0 as *mut libc::c_void as *mut brubeck_metric) as libc::c_int
        as libc::c_long != 0
    {
        if (*server).at_capacity != 0 {
            return 0 as *mut brubeck_metric;
        }
        return brubeck_metric_new(server, key, key_len, type_0);
    }
    return metric;
}
