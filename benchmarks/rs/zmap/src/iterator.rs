use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type aesrand;
    pub type output_module;
    pub type probe_module;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn blocklist_count_allowed() -> uint64_t;
    fn log_debug(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn now() -> libc::c_double;
    fn xcalloc(count: size_t, size: size_t) -> *mut libc::c_void;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    fn shard_init(
        shard: *mut shard_t,
        shard_idx: uint16_t,
        num_shards: uint16_t,
        thread_idx: uint8_t,
        num_threads: uint8_t,
        max_total_targets: uint32_t,
        cycle: *const cycle_t,
        cb: shard_complete_cb,
        arg: *mut libc::c_void,
    );
    fn get_group(min_size: uint64_t) -> *const cyclic_group_t;
    fn make_cycle(group: *const cyclic_group_t, aes: *mut aesrand_t) -> cycle_t;
    static mut zconf: state_conf;
    static mut zsend: state_send;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type in_addr_t = uint32_t;
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
pub type aesrand_t = aesrand;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cyclic_group {
    pub prime: uint64_t,
    pub known_primroot: uint64_t,
    pub num_prime_factors: size_t,
    pub prime_factors: [uint64_t; 10],
}
pub type cyclic_group_t = cyclic_group;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cycle {
    pub group: *const cyclic_group_t,
    pub generator: uint64_t,
    pub order: uint64_t,
    pub offset: uint32_t,
}
pub type cycle_t = cycle;
pub type shard_complete_cb = Option::<
    unsafe extern "C" fn(uint8_t, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shard {
    pub state: shard_state,
    pub params: shard_params,
    pub current: uint64_t,
    pub iterations: uint64_t,
    pub thread_id: uint8_t,
    pub cb: shard_complete_cb,
    pub arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shard_params {
    pub first: uint64_t,
    pub last: uint64_t,
    pub factor: uint64_t,
    pub modulus: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shard_state {
    pub packets_sent: uint64_t,
    pub hosts_scanned: uint32_t,
    pub max_hosts: uint32_t,
    pub max_packets: uint32_t,
    pub hosts_blocklisted: uint32_t,
    pub hosts_allowlisted: uint32_t,
    pub packets_failed: uint32_t,
    pub first_scanned: uint32_t,
}
pub type shard_t = shard;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iterator {
    pub cycle: cycle_t,
    pub num_threads: uint8_t,
    pub thread_shards: *mut shard_t,
    pub complete: *mut uint8_t,
    pub mutex: pthread_mutex_t,
    pub curr_threads: uint32_t,
}
pub type iterator_t = iterator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_conf {
    pub log_level: libc::c_int,
    pub target_port: port_h_t,
    pub source_port_first: port_h_t,
    pub source_port_last: port_h_t,
    pub max_targets: uint32_t,
    pub max_runtime: uint32_t,
    pub max_results: uint32_t,
    pub iface: *mut libc::c_char,
    pub rate: libc::c_int,
    pub bandwidth: uint64_t,
    pub cooldown_secs: libc::c_int,
    pub senders: uint8_t,
    pub batch: uint8_t,
    pub pin_cores_len: uint32_t,
    pub pin_cores: *mut uint32_t,
    pub seed_provided: libc::c_int,
    pub seed: uint64_t,
    pub aes: *mut aesrand_t,
    pub generator: uint32_t,
    pub shard_num: uint16_t,
    pub total_shards: uint16_t,
    pub packet_streams: libc::c_int,
    pub probe_module: *mut probe_module,
    pub output_module_name: *mut libc::c_char,
    pub output_module: *mut output_module,
    pub probe_args: *mut libc::c_char,
    pub probe_ttl: uint8_t,
    pub output_args: *mut libc::c_char,
    pub gw_mac: [macaddr_t; 6],
    pub hw_mac: [macaddr_t; 6],
    pub gw_ip: uint32_t,
    pub gw_mac_set: libc::c_int,
    pub hw_mac_set: libc::c_int,
    pub source_ip_addresses: [in_addr_t; 256],
    pub number_source_ips: uint32_t,
    pub send_ip_pkts: libc::c_int,
    pub output_filename: *mut libc::c_char,
    pub blocklist_filename: *mut libc::c_char,
    pub allowlist_filename: *mut libc::c_char,
    pub list_of_ips_filename: *mut libc::c_char,
    pub list_of_ips_count: uint32_t,
    pub metadata_filename: *mut libc::c_char,
    pub metadata_file: *mut FILE,
    pub notes: *mut libc::c_char,
    pub custom_metadata_str: *mut libc::c_char,
    pub destination_cidrs: *mut *mut libc::c_char,
    pub destination_cidrs_len: libc::c_int,
    pub raw_output_fields: *const libc::c_char,
    pub output_fields: *mut *const libc::c_char,
    pub filter: output_filter,
    pub output_filter_str: *mut libc::c_char,
    pub fsconf: fieldset_conf,
    pub output_fields_len: libc::c_int,
    pub log_file: *mut libc::c_char,
    pub log_directory: *mut libc::c_char,
    pub status_updates_file: *mut libc::c_char,
    pub dryrun: libc::c_int,
    pub quiet: libc::c_int,
    pub ignore_invalid_hosts: libc::c_int,
    pub syslog: libc::c_int,
    pub recv_ready: libc::c_int,
    pub num_retries: libc::c_int,
    pub total_allowed: uint64_t,
    pub total_disallowed: uint64_t,
    pub max_sendto_failures: libc::c_int,
    pub min_hitrate: libc::c_float,
    pub data_link_size: libc::c_int,
    pub default_mode: libc::c_int,
    pub no_header_row: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fieldset_conf {
    pub defs: fielddefset_t,
    pub outdefs: fielddefset_t,
    pub translation: translation_t,
    pub success_index: libc::c_int,
    pub app_success_index: libc::c_int,
    pub classification_index: libc::c_int,
}
pub type translation_t = translation;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct translation {
    pub len: libc::c_int,
    pub translation: [libc::c_int; 128],
}
pub type fielddefset_t = fielddef_set;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fielddef_set {
    pub fielddefs: [fielddef_t; 128],
    pub len: libc::c_int,
}
pub type fielddef_t = field_def;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field_def {
    pub name: *const libc::c_char,
    pub type_0: *const libc::c_char,
    pub desc: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_filter {
    pub expression: *mut node_t,
}
pub type node_t = node_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_st {
    pub left_child: *mut node_st,
    pub right_child: *mut node_st,
    pub type_0: node_type,
    pub value: node_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union node_value {
    pub field: field_id,
    pub string_literal: *mut libc::c_char,
    pub int_literal: uint64_t,
    pub op: operation,
}
pub type operation = libc::c_uint;
pub const GT_EQ: operation = 7;
pub const LT_EQ: operation = 6;
pub const OR: operation = 5;
pub const AND: operation = 4;
pub const NEQ: operation = 3;
pub const EQ: operation = 2;
pub const LT: operation = 1;
pub const GT: operation = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field_id {
    pub index: libc::c_int,
    pub fieldname: *mut libc::c_char,
}
pub type node_type = libc::c_uint;
pub const INT: node_type = 3;
pub const STRING: node_type = 2;
pub const FIELD: node_type = 1;
pub const OP: node_type = 0;
pub type macaddr_t = libc::c_uchar;
pub type port_h_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_send {
    pub start: libc::c_double,
    pub finish: libc::c_double,
    pub packets_sent: uint64_t,
    pub hosts_scanned: uint64_t,
    pub blocklisted: uint64_t,
    pub allowlisted: uint64_t,
    pub warmup: libc::c_int,
    pub complete: libc::c_int,
    pub first_scanned: uint32_t,
    pub max_targets: uint32_t,
    pub sendto_failures: uint32_t,
    pub max_index: uint32_t,
    pub list_of_ips_pbm: *mut *mut uint8_t,
}
pub unsafe extern "C" fn shard_complete(
    mut thread_id: uint8_t,
    mut arg: *mut libc::c_void,
) {
    let mut it: *mut iterator_t = arg as *mut iterator_t;
    if (thread_id as libc::c_int) < (*it).num_threads as libc::c_int {} else {
        __assert_fail(
            b"thread_id < it->num_threads\0" as *const u8 as *const libc::c_char,
            b"iterator.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"void shard_complete(uint8_t, void *)\0"))
                .as_ptr(),
        );
    }
    'c_4658: {
        if (thread_id as libc::c_int) < (*it).num_threads as libc::c_int {} else {
            __assert_fail(
                b"thread_id < it->num_threads\0" as *const u8 as *const libc::c_char,
                b"iterator.c\0" as *const u8 as *const libc::c_char,
                37 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"void shard_complete(uint8_t, void *)\0"))
                    .as_ptr(),
            );
        }
    };
    pthread_mutex_lock(&mut (*it).mutex);
    *((*it).complete).offset(thread_id as isize) = 1 as libc::c_int as uint8_t;
    (*it).curr_threads = ((*it).curr_threads).wrapping_sub(1);
    (*it).curr_threads;
    let mut s: *mut shard_t = &mut *((*it).thread_shards).offset(thread_id as isize)
        as *mut shard_t;
    zsend
        .packets_sent = (zsend.packets_sent as libc::c_ulong)
        .wrapping_add((*s).state.packets_sent) as uint64_t as uint64_t;
    zsend
        .hosts_scanned = (zsend.hosts_scanned as libc::c_ulong)
        .wrapping_add((*s).state.hosts_scanned as libc::c_ulong) as uint64_t as uint64_t;
    zsend
        .blocklisted = (zsend.blocklisted as libc::c_ulong)
        .wrapping_add((*s).state.hosts_blocklisted as libc::c_ulong) as uint64_t
        as uint64_t;
    zsend
        .allowlisted = (zsend.allowlisted as libc::c_ulong)
        .wrapping_add((*s).state.hosts_allowlisted as libc::c_ulong) as uint64_t
        as uint64_t;
    zsend
        .sendto_failures = (zsend.sendto_failures as libc::c_uint)
        .wrapping_add((*s).state.packets_failed) as uint32_t as uint32_t;
    let mut done: uint8_t = 1 as libc::c_int as uint8_t;
    let mut i: uint8_t = 0 as libc::c_int as uint8_t;
    while done as libc::c_int != 0
        && (i as libc::c_int) < (*it).num_threads as libc::c_int
    {
        done = (done as libc::c_int != 0
            && *((*it).complete).offset(i as isize) as libc::c_int != 0) as libc::c_int
            as uint8_t;
        i = i.wrapping_add(1);
        i;
    }
    if done != 0 {
        zsend.finish = now();
        zsend.complete = 1 as libc::c_int;
        zsend
            .first_scanned = (*((*it).thread_shards).offset(0 as libc::c_int as isize))
            .state
            .first_scanned;
    }
    pthread_mutex_unlock(&mut (*it).mutex);
}
pub unsafe extern "C" fn iterator_init(
    mut num_threads: uint8_t,
    mut shard: uint16_t,
    mut num_shards: uint16_t,
) -> *mut iterator_t {
    let mut num_addrs: uint64_t = blocklist_count_allowed();
    let mut group_min_size: uint64_t = num_addrs;
    if !(zconf.list_of_ips_filename).is_null() {
        log_debug(
            b"send\0" as *const u8 as *const libc::c_char,
            b"forcing max group size for compatibility with -I\0" as *const u8
                as *const libc::c_char,
        );
        group_min_size = 0xffffffff as libc::c_uint as uint64_t;
    }
    let mut it: *mut iterator_t = xmalloc(
        ::std::mem::size_of::<iterator>() as libc::c_ulong,
    ) as *mut iterator_t;
    let mut group: *const cyclic_group_t = get_group(group_min_size);
    if num_addrs as libc::c_ulonglong
        > ((1 as libc::c_longlong) << 32 as libc::c_int) as libc::c_ulonglong
    {
        zsend.max_index = 0xffffffff as libc::c_uint;
    } else {
        zsend.max_index = num_addrs as uint32_t;
    }
    log_debug(
        b"iterator\0" as *const u8 as *const libc::c_char,
        b"max index %u\0" as *const u8 as *const libc::c_char,
        zsend.max_index,
    );
    (*it).cycle = make_cycle(group, zconf.aes);
    (*it).num_threads = num_threads;
    (*it).curr_threads = num_threads as uint32_t;
    (*it)
        .thread_shards = xcalloc(
        num_threads as size_t,
        ::std::mem::size_of::<shard_t>() as libc::c_ulong,
    ) as *mut shard_t;
    (*it)
        .complete = xcalloc(
        (*it).num_threads as size_t,
        ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
    ) as *mut uint8_t;
    pthread_mutex_init(&mut (*it).mutex, 0 as *const pthread_mutexattr_t);
    let mut i: uint8_t = 0 as libc::c_int as uint8_t;
    while (i as libc::c_int) < num_threads as libc::c_int {
        shard_init(
            &mut *((*it).thread_shards).offset(i as isize),
            shard,
            num_shards,
            i,
            num_threads,
            zsend.max_targets,
            &mut (*it).cycle,
            Some(
                shard_complete as unsafe extern "C" fn(uint8_t, *mut libc::c_void) -> (),
            ),
            it as *mut libc::c_void,
        );
        i = i.wrapping_add(1);
        i;
    }
    zconf.generator = (*it).cycle.generator as uint32_t;
    return it;
}
pub unsafe extern "C" fn iterator_get_sent(mut it: *mut iterator_t) -> uint64_t {
    let mut sent: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i: uint8_t = 0 as libc::c_int as uint8_t;
    while (i as libc::c_int) < (*it).num_threads as libc::c_int {
        sent = (sent as libc::c_ulong)
            .wrapping_add((*((*it).thread_shards).offset(i as isize)).state.packets_sent)
            as uint64_t as uint64_t;
        i = i.wrapping_add(1);
        i;
    }
    return sent;
}
pub unsafe extern "C" fn iterator_get_iterations(mut it: *mut iterator_t) -> uint64_t {
    let mut iterations: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i: uint8_t = 0 as libc::c_int as uint8_t;
    while (i as libc::c_int) < (*it).num_threads as libc::c_int {
        iterations = (iterations as libc::c_ulong)
            .wrapping_add((*((*it).thread_shards).offset(i as isize)).iterations)
            as uint64_t as uint64_t;
        i = i.wrapping_add(1);
        i;
    }
    return iterations;
}
pub unsafe extern "C" fn iterator_get_fail(mut it: *mut iterator_t) -> uint32_t {
    let mut fails: uint32_t = 0 as libc::c_int as uint32_t;
    let mut i: uint8_t = 0 as libc::c_int as uint8_t;
    while (i as libc::c_int) < (*it).num_threads as libc::c_int {
        fails = (fails as libc::c_uint)
            .wrapping_add(
                (*((*it).thread_shards).offset(i as isize)).state.packets_failed,
            ) as uint32_t as uint32_t;
        i = i.wrapping_add(1);
        i;
    }
    return fails;
}
pub unsafe extern "C" fn get_shard(
    mut it: *mut iterator_t,
    mut thread_id: uint8_t,
) -> *mut shard_t {
    if (thread_id as libc::c_int) < (*it).num_threads as libc::c_int {} else {
        __assert_fail(
            b"thread_id < it->num_threads\0" as *const u8 as *const libc::c_char,
            b"iterator.c\0" as *const u8 as *const libc::c_char,
            120 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"shard_t *get_shard(iterator_t *, uint8_t)\0"))
                .as_ptr(),
        );
    }
    'c_5054: {
        if (thread_id as libc::c_int) < (*it).num_threads as libc::c_int {} else {
            __assert_fail(
                b"thread_id < it->num_threads\0" as *const u8 as *const libc::c_char,
                b"iterator.c\0" as *const u8 as *const libc::c_char,
                120 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"shard_t *get_shard(iterator_t *, uint8_t)\0"))
                    .as_ptr(),
            );
        }
    };
    return &mut *((*it).thread_shards).offset(thread_id as isize) as *mut shard_t;
}
pub unsafe extern "C" fn iterator_get_curr_send_threads(
    mut it: *mut iterator_t,
) -> uint32_t {
    if !it.is_null() {} else {
        __assert_fail(
            b"it\0" as *const u8 as *const libc::c_char,
            b"iterator.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"uint32_t iterator_get_curr_send_threads(iterator_t *)\0"))
                .as_ptr(),
        );
    }
    'c_5005: {
        if !it.is_null() {} else {
            __assert_fail(
                b"it\0" as *const u8 as *const libc::c_char,
                b"iterator.c\0" as *const u8 as *const libc::c_char,
                126 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"uint32_t iterator_get_curr_send_threads(iterator_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    return (*it).curr_threads;
}
