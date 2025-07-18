use ::libc;
extern "C" {
    pub type aesrand;
    pub type iterator;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type output_module;
    pub type probe_module;
    fn iterator_get_curr_send_threads(it: *mut iterator_t) -> uint32_t;
    fn iterator_get_fail(it: *mut iterator_t) -> uint32_t;
    fn iterator_get_iterations(it: *mut iterator_t) -> uint64_t;
    fn iterator_get_sent(it: *mut iterator_t) -> uint64_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn recv_update_stats() -> libc::c_int;
    static mut zrecv: state_recv;
    static mut zsend: state_send;
    static mut zconf: state_conf;
    fn lock_file(f: *mut FILE) -> libc::c_int;
    fn unlock_file(f: *mut FILE) -> libc::c_int;
    fn log_fatal(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> !;
    fn log_warn(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn log_debug(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn now() -> libc::c_double;
    fn time_string(
        time: uint32_t,
        est: libc::c_int,
        buf: *mut libc::c_char,
        len: size_t,
    );
    fn number_string(n: uint32_t, buf: *mut libc::c_char, len: size_t);
    fn xmalloc(size: size_t) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type in_addr_t = uint32_t;
pub type aesrand_t = aesrand;
pub type iterator_t = iterator;
pub type FILE = _IO_FILE;
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
pub type export_status_t = export_scan_status;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct export_scan_status {
    pub total_sent: uint64_t,
    pub total_tried_sent: uint64_t,
    pub recv_success_unique: uint32_t,
    pub app_recv_success_unique: uint32_t,
    pub total_recv: uint64_t,
    pub complete: uint32_t,
    pub send_threads: uint32_t,
    pub percent_complete: libc::c_double,
    pub hitrate: libc::c_double,
    pub app_hitrate: libc::c_double,
    pub send_rate: libc::c_double,
    pub send_rate_str: [libc::c_char; 20],
    pub send_rate_avg: libc::c_double,
    pub send_rate_avg_str: [libc::c_char; 20],
    pub recv_rate: libc::c_double,
    pub recv_rate_str: [libc::c_char; 20],
    pub recv_avg: libc::c_double,
    pub recv_avg_str: [libc::c_char; 20],
    pub recv_total_rate: libc::c_double,
    pub recv_total_avg: libc::c_double,
    pub app_success_rate: libc::c_double,
    pub app_success_rate_str: [libc::c_char; 20],
    pub app_success_avg: libc::c_double,
    pub app_success_avg_str: [libc::c_char; 20],
    pub pcap_drop: uint32_t,
    pub pcap_ifdrop: uint32_t,
    pub pcap_drop_total: uint32_t,
    pub pcap_drop_total_str: [libc::c_char; 20],
    pub pcap_drop_last: libc::c_double,
    pub pcap_drop_last_str: [libc::c_char; 20],
    pub pcap_drop_avg: libc::c_double,
    pub pcap_drop_avg_str: [libc::c_char; 20],
    pub time_remaining: uint32_t,
    pub time_remaining_str: [libc::c_char; 20],
    pub time_past: uint32_t,
    pub time_past_str: [libc::c_char; 20],
    pub fail_total: uint32_t,
    pub fail_avg: libc::c_double,
    pub fail_last: libc::c_double,
    pub seconds_under_min_hitrate: libc::c_float,
}
pub type int_status_t = internal_scan_status;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_scan_status {
    pub last_now: libc::c_double,
    pub last_sent: uint64_t,
    pub last_tried_sent: uint64_t,
    pub last_send_failures: uint32_t,
    pub last_recv_net_success: uint32_t,
    pub last_recv_app_success: uint32_t,
    pub last_recv_total: uint32_t,
    pub last_pcap_drop: uint32_t,
    pub min_hitrate_start: libc::c_double,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_recv {
    pub success_total: uint32_t,
    pub success_unique: uint32_t,
    pub app_success_total: uint32_t,
    pub app_success_unique: uint32_t,
    pub cooldown_total: uint32_t,
    pub cooldown_unique: uint32_t,
    pub failure_total: uint32_t,
    pub filter_success: uint64_t,
    pub ip_fragments: uint32_t,
    pub validation_passed: uint32_t,
    pub validation_failed: uint32_t,
    pub complete: libc::c_int,
    pub start: libc::c_double,
    pub finish: libc::c_double,
    pub pcap_recv: uint32_t,
    pub pcap_drop: uint32_t,
    pub pcap_ifdrop: uint32_t,
}
static mut status_fd: *mut FILE = 0 as *const FILE as *mut FILE;
unsafe extern "C" fn min_d(
    mut array: *mut libc::c_double,
    mut n: libc::c_int,
) -> libc::c_double {
    let mut value: libc::c_double = ::std::f32::INFINITY as libc::c_double;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        if *array.offset(i as isize) < value {
            value = *array.offset(i as isize);
        }
        i += 1;
        i;
    }
    return value;
}
pub unsafe extern "C" fn compute_remaining_time(
    mut age: libc::c_double,
    mut packets_sent: uint64_t,
    mut iterations: uint64_t,
) -> libc::c_double {
    if zsend.complete == 0 {
        let mut remaining: [libc::c_double; 5] = [
            ::std::f32::INFINITY as libc::c_double,
            ::std::f32::INFINITY as libc::c_double,
            ::std::f32::INFINITY as libc::c_double,
            ::std::f32::INFINITY as libc::c_double,
            ::std::f32::INFINITY as libc::c_double,
        ];
        if !(zsend.list_of_ips_pbm).is_null() {
            let mut done: libc::c_double = iterations as libc::c_double
                / (0xffffffff as libc::c_uint as uint64_t)
                    .wrapping_div(zconf.total_shards as libc::c_ulong) as libc::c_double;
            remaining[0 as libc::c_int
                as usize] = (1.0f64 - done) * (age / done)
                + zconf.cooldown_secs as libc::c_double;
        }
        if zsend.max_targets != 0 {
            let mut done_0: libc::c_double = packets_sent as libc::c_double
                / (zsend.max_targets as uint64_t)
                    .wrapping_mul(zconf.packet_streams as libc::c_ulong)
                    .wrapping_div(zconf.total_shards as libc::c_ulong) as libc::c_double;
            remaining[1 as libc::c_int
                as usize] = (1.0f64 - done_0) * (age / done_0)
                + zconf.cooldown_secs as libc::c_double;
        }
        if zconf.max_runtime != 0 {
            remaining[2 as libc::c_int
                as usize] = zconf.max_runtime as libc::c_double - age
                + zconf.cooldown_secs as libc::c_double;
        }
        if zconf.max_results != 0 {
            let mut done_1: libc::c_double = zrecv.filter_success as libc::c_double
                / zconf.max_results as libc::c_double;
            remaining[3 as libc::c_int as usize] = (1.0f64 - done_1) * (age / done_1);
        }
        if zsend.max_index != 0 {
            let mut done_2: libc::c_double = packets_sent as libc::c_double
                / (zsend.max_index)
                    .wrapping_mul(zconf.packet_streams as libc::c_uint)
                    .wrapping_div(zconf.total_shards as libc::c_uint) as libc::c_double;
            remaining[4 as libc::c_int
                as usize] = (1.0f64 - done_2) * (age / done_2)
                + zconf.cooldown_secs as libc::c_double;
        }
        return min_d(
            remaining.as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_double; 5]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                as libc::c_int,
        );
    } else {
        return zconf.cooldown_secs as libc::c_double - (now() - zsend.finish)
    };
}
unsafe extern "C" fn update_pcap_stats(mut recv_ready_mutex: *mut pthread_mutex_t) {
    pthread_mutex_lock(recv_ready_mutex);
    recv_update_stats();
    pthread_mutex_unlock(recv_ready_mutex);
}
unsafe extern "C" fn export_stats(
    mut intrnl: *mut int_status_t,
    mut exp: *mut export_status_t,
    mut it: *mut iterator_t,
) {
    let mut total_sent: uint64_t = iterator_get_sent(it);
    let mut total_iterations: uint64_t = iterator_get_iterations(it);
    let mut total_fail: uint32_t = iterator_get_fail(it);
    let mut total_recv: uint64_t = zrecv.pcap_recv as uint64_t;
    let mut recv_success: uint64_t = zrecv.filter_success;
    let mut app_success: uint32_t = zrecv.app_success_unique;
    let mut cur_time: libc::c_double = now();
    let mut age: libc::c_double = cur_time - zsend.start;
    let mut delta: libc::c_double = cur_time - (*intrnl).last_now;
    let mut remaining_secs: libc::c_double = compute_remaining_time(
        age,
        total_sent,
        total_iterations,
    );
    if age < 5 as libc::c_int as libc::c_double {
        (*exp)
            .time_remaining_str[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    } else {
        let mut buf: [libc::c_char; 20] = [0; 20];
        time_string(
            ceil(remaining_secs) as uint32_t,
            1 as libc::c_int,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
        );
        snprintf(
            ((*exp).time_remaining_str).as_mut_ptr(),
            20 as libc::c_int as libc::c_ulong,
            b" (%s left)\0" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    (*exp).time_past = age as uint32_t;
    (*exp).time_remaining = remaining_secs as uint32_t;
    time_string(
        age as libc::c_int as uint32_t,
        0 as libc::c_int,
        ((*exp).time_past_str).as_mut_ptr(),
        20 as libc::c_int as size_t,
    );
    (*exp)
        .recv_rate = ceil(
        recv_success.wrapping_sub((*intrnl).last_recv_net_success as libc::c_ulong)
            as libc::c_double / delta,
    );
    number_string(
        (*exp).recv_rate as uint32_t,
        ((*exp).recv_rate_str).as_mut_ptr(),
        20 as libc::c_int as size_t,
    );
    (*exp).recv_avg = recv_success as libc::c_double / age;
    number_string(
        (*exp).recv_avg as uint32_t,
        ((*exp).recv_avg_str).as_mut_ptr(),
        20 as libc::c_int as size_t,
    );
    (*exp)
        .recv_total_rate = total_recv
        .wrapping_sub((*intrnl).last_recv_total as libc::c_ulong) as libc::c_double
        / delta;
    (*exp).recv_total_avg = total_recv as libc::c_double / age;
    if zconf.fsconf.app_success_index >= 0 as libc::c_int {
        (*exp)
            .app_success_rate = app_success.wrapping_sub((*intrnl).last_recv_app_success)
            as libc::c_double / delta;
        number_string(
            (*exp).app_success_rate as uint32_t,
            ((*exp).app_success_rate_str).as_mut_ptr(),
            20 as libc::c_int as size_t,
        );
        (*exp).app_success_avg = app_success as libc::c_double / age;
        number_string(
            (*exp).app_success_avg as uint32_t,
            ((*exp).app_success_avg_str).as_mut_ptr(),
            20 as libc::c_int as size_t,
        );
    }
    if total_sent == 0 {
        (*exp).hitrate = 0 as libc::c_int as libc::c_double;
        (*exp).app_hitrate = 0 as libc::c_int as libc::c_double;
    } else {
        (*exp)
            .hitrate = recv_success as libc::c_double * 100.0f64
            / total_sent as libc::c_double;
        (*exp)
            .app_hitrate = app_success as libc::c_double * 100.0f64
            / total_sent as libc::c_double;
    }
    if age > 5 as libc::c_int as libc::c_double
        && (*exp).hitrate < zconf.min_hitrate as libc::c_double
    {
        if fabs((*intrnl).min_hitrate_start) < 0.00001f64 {
            (*intrnl).min_hitrate_start = cur_time;
        }
    } else {
        (*intrnl).min_hitrate_start = 0.0f64;
    }
    if fabs((*intrnl).min_hitrate_start) < 0.00001f64 {
        (*exp).seconds_under_min_hitrate = 0 as libc::c_int as libc::c_float;
    } else {
        (*exp)
            .seconds_under_min_hitrate = (cur_time - (*intrnl).min_hitrate_start)
            as libc::c_float;
    }
    if zsend.complete == 0 {
        (*exp)
            .send_rate = ceil(
            total_sent.wrapping_sub((*intrnl).last_sent) as libc::c_double / delta,
        );
        number_string(
            (*exp).send_rate as uint32_t,
            ((*exp).send_rate_str).as_mut_ptr(),
            20 as libc::c_int as size_t,
        );
        (*exp).send_rate_avg = total_sent as libc::c_double / age;
        number_string(
            (*exp).send_rate_avg as uint32_t,
            ((*exp).send_rate_avg_str).as_mut_ptr(),
            20 as libc::c_int as size_t,
        );
    } else {
        (*exp)
            .send_rate_avg = total_sent as libc::c_double / (zsend.finish - zsend.start);
        number_string(
            (*exp).send_rate_avg as uint32_t,
            ((*exp).send_rate_avg_str).as_mut_ptr(),
            20 as libc::c_int as size_t,
        );
    }
    (*exp).total_sent = total_sent;
    (*exp).total_tried_sent = total_iterations;
    (*exp).percent_complete = 100.0f64 * age / (age + remaining_secs);
    (*exp).recv_success_unique = recv_success as uint32_t;
    (*exp).app_recv_success_unique = app_success;
    (*exp).total_recv = total_recv;
    (*exp).complete = zsend.complete as uint32_t;
    (*exp).pcap_drop = zrecv.pcap_drop;
    (*exp).pcap_ifdrop = zrecv.pcap_ifdrop;
    (*exp).pcap_drop_total = ((*exp).pcap_drop).wrapping_add((*exp).pcap_ifdrop);
    (*exp)
        .pcap_drop_last = ((*exp).pcap_drop_total).wrapping_sub((*intrnl).last_pcap_drop)
        as libc::c_double / delta;
    (*exp).pcap_drop_avg = (*exp).pcap_drop_total as libc::c_double / age;
    number_string(
        (*exp).pcap_drop_total,
        ((*exp).pcap_drop_total_str).as_mut_ptr(),
        20 as libc::c_int as size_t,
    );
    number_string(
        (*exp).pcap_drop_last as uint32_t,
        ((*exp).pcap_drop_last_str).as_mut_ptr(),
        20 as libc::c_int as size_t,
    );
    number_string(
        (*exp).pcap_drop_avg as uint32_t,
        ((*exp).pcap_drop_avg_str).as_mut_ptr(),
        20 as libc::c_int as size_t,
    );
    zsend.sendto_failures = total_fail;
    (*exp).fail_total = zsend.sendto_failures;
    (*exp)
        .fail_last = ((*exp).fail_total).wrapping_sub((*intrnl).last_send_failures)
        as libc::c_double / delta;
    (*exp).fail_avg = (*exp).fail_total as libc::c_double / age;
    (*exp).send_threads = iterator_get_curr_send_threads(it);
    (*intrnl).last_now = cur_time;
    (*intrnl).last_sent = (*exp).total_sent;
    (*intrnl).last_recv_net_success = (*exp).recv_success_unique;
    (*intrnl).last_recv_app_success = (*exp).app_recv_success_unique;
    (*intrnl).last_pcap_drop = (*exp).pcap_drop_total;
    (*intrnl).last_send_failures = (*exp).fail_total;
    (*intrnl).last_recv_total = (*exp).total_recv as uint32_t;
}
unsafe extern "C" fn log_drop_warnings(mut exp: *mut export_status_t) {
    if (*exp).pcap_drop_last / (*exp).recv_rate > 0.05f64 {
        log_warn(
            b"monitor\0" as *const u8 as *const libc::c_char,
            b"Dropped %.0f packets in the last second, (%u total dropped (pcap: %u + iface: %u))\0"
                as *const u8 as *const libc::c_char,
            (*exp).pcap_drop_last,
            (*exp).pcap_drop_total,
            (*exp).pcap_drop,
            (*exp).pcap_ifdrop,
        );
    }
    if (*exp).fail_last / (*exp).send_rate > 0.01f64 {
        log_warn(
            b"monitor\0" as *const u8 as *const libc::c_char,
            b"Failed to send %.0f packets/sec (%u total failures)\0" as *const u8
                as *const libc::c_char,
            (*exp).fail_last,
            (*exp).fail_total,
        );
    }
}
unsafe extern "C" fn onscreen_appsuccess(mut exp: *mut export_status_t) {
    if (*exp).complete == 0 {
        fprintf(
            stderr,
            b"%5s %0.0f%%%s; sent: %lu %sp/s (%sp/s avg); recv: %u %sp/s (%sp/s avg); app success: %u %sp/s (%sp/s avg); drops: %sp/s (%sp/s avg); hitrate: %0.2f%% app hitrate: %0.2f%%\n\0"
                as *const u8 as *const libc::c_char,
            ((*exp).time_past_str).as_mut_ptr(),
            (*exp).percent_complete,
            ((*exp).time_remaining_str).as_mut_ptr(),
            (*exp).total_sent,
            ((*exp).send_rate_str).as_mut_ptr(),
            ((*exp).send_rate_avg_str).as_mut_ptr(),
            (*exp).recv_success_unique,
            ((*exp).recv_rate_str).as_mut_ptr(),
            ((*exp).recv_avg_str).as_mut_ptr(),
            (*exp).app_recv_success_unique,
            ((*exp).app_success_rate_str).as_mut_ptr(),
            ((*exp).app_success_avg_str).as_mut_ptr(),
            ((*exp).pcap_drop_last_str).as_mut_ptr(),
            ((*exp).pcap_drop_avg_str).as_mut_ptr(),
            (*exp).hitrate,
            (*exp).app_hitrate,
        );
    } else {
        fprintf(
            stderr,
            b"%5s %0.0f%%%s; sent: %lu done (%sp/s avg); recv: %u %sp/s (%sp/s avg); app success: %u %sp/s (%sp/s avg); drops: %sp/s (%sp/s avg); hitrate: %0.2f%% app hitrate: %0.2f%%\n\0"
                as *const u8 as *const libc::c_char,
            ((*exp).time_past_str).as_mut_ptr(),
            (*exp).percent_complete,
            ((*exp).time_remaining_str).as_mut_ptr(),
            (*exp).total_sent,
            ((*exp).send_rate_avg_str).as_mut_ptr(),
            (*exp).recv_success_unique,
            ((*exp).recv_rate_str).as_mut_ptr(),
            ((*exp).recv_avg_str).as_mut_ptr(),
            (*exp).app_recv_success_unique,
            ((*exp).app_success_rate_str).as_mut_ptr(),
            ((*exp).app_success_avg_str).as_mut_ptr(),
            ((*exp).pcap_drop_last_str).as_mut_ptr(),
            ((*exp).pcap_drop_avg_str).as_mut_ptr(),
            (*exp).hitrate,
            (*exp).app_hitrate,
        );
    };
}
unsafe extern "C" fn onscreen_generic(mut exp: *mut export_status_t) {
    if (*exp).complete == 0 {
        fprintf(
            stderr,
            b"%5s %0.0f%%%s; send: %lu %sp/s (%sp/s avg); recv: %u %sp/s (%sp/s avg); drops: %sp/s (%sp/s avg); hitrate: %0.2f%%\n\0"
                as *const u8 as *const libc::c_char,
            ((*exp).time_past_str).as_mut_ptr(),
            (*exp).percent_complete,
            ((*exp).time_remaining_str).as_mut_ptr(),
            (*exp).total_sent,
            ((*exp).send_rate_str).as_mut_ptr(),
            ((*exp).send_rate_avg_str).as_mut_ptr(),
            (*exp).recv_success_unique,
            ((*exp).recv_rate_str).as_mut_ptr(),
            ((*exp).recv_avg_str).as_mut_ptr(),
            ((*exp).pcap_drop_last_str).as_mut_ptr(),
            ((*exp).pcap_drop_avg_str).as_mut_ptr(),
            (*exp).hitrate,
        );
    } else {
        fprintf(
            stderr,
            b"%5s %0.0f%%%s; send: %lu done (%sp/s avg); recv: %u %sp/s (%sp/s avg); drops: %sp/s (%sp/s avg); hitrate: %0.2f%%\n\0"
                as *const u8 as *const libc::c_char,
            ((*exp).time_past_str).as_mut_ptr(),
            (*exp).percent_complete,
            ((*exp).time_remaining_str).as_mut_ptr(),
            (*exp).total_sent,
            ((*exp).send_rate_avg_str).as_mut_ptr(),
            (*exp).recv_success_unique,
            ((*exp).recv_rate_str).as_mut_ptr(),
            ((*exp).recv_avg_str).as_mut_ptr(),
            ((*exp).pcap_drop_last_str).as_mut_ptr(),
            ((*exp).pcap_drop_avg_str).as_mut_ptr(),
            (*exp).hitrate,
        );
    }
    fflush(stderr);
}
unsafe extern "C" fn init_status_update_file(mut path: *mut libc::c_char) -> *mut FILE {
    let mut f: *mut FILE = fopen(path, b"w\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        log_fatal(
            b"csv\0" as *const u8 as *const libc::c_char,
            b"could not open status updates file (%s): %s\0" as *const u8
                as *const libc::c_char,
            zconf.status_updates_file,
            strerror(*__errno_location()),
        );
    }
    log_debug(
        b"monitor\0" as *const u8 as *const libc::c_char,
        b"status updates CSV will be saved to %s\0" as *const u8 as *const libc::c_char,
        zconf.status_updates_file,
    );
    fprintf(
        f,
        b"real-time,time-elapsed,time-remaining,percent-complete,hit-rate,active-send-threads,sent-total,sent-last-one-sec,sent-avg-per-sec,recv-success-total,recv-success-last-one-sec,recv-success-avg-per-sec,recv-total,recv-total-last-one-sec,recv-total-avg-per-sec,pcap-drop-total,drop-last-one-sec,drop-avg-per-sec,sendto-fail-total,sendto-fail-last-one-sec,sendto-fail-avg-per-sec\n\0"
            as *const u8 as *const libc::c_char,
    );
    fflush(f);
    return f;
}
unsafe extern "C" fn update_status_updates_file(
    mut exp: *mut export_status_t,
    mut f: *mut FILE,
) {
    let mut now_0: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut timestamp: [libc::c_char; 256] = [0; 256];
    gettimeofday(&mut now_0, 0 as *mut libc::c_void);
    let mut sec: time_t = now_0.tv_sec;
    let mut ptm: *mut tm = localtime(&mut sec);
    strftime(
        timestamp.as_mut_ptr(),
        20 as libc::c_int as size_t,
        b"%Y-%m-%d %H:%M:%S\0" as *const u8 as *const libc::c_char,
        ptm,
    );
    fprintf(
        f,
        b"%s,%u,%u,%f,%f,%u,%lu,%.0f,%.0f,%u,%.0f,%.0f,%lu,%.0f,%.0f,%u,%.0f,%.0f,%u,%.0f,%.0f\n\0"
            as *const u8 as *const libc::c_char,
        timestamp.as_mut_ptr(),
        (*exp).time_past,
        (*exp).time_remaining,
        (*exp).percent_complete,
        (*exp).hitrate,
        (*exp).send_threads,
        (*exp).total_sent,
        (*exp).send_rate,
        (*exp).send_rate_avg,
        (*exp).recv_success_unique,
        (*exp).recv_rate,
        (*exp).recv_avg,
        (*exp).total_recv,
        (*exp).recv_total_rate,
        (*exp).recv_total_avg,
        (*exp).pcap_drop_total,
        (*exp).pcap_drop_last,
        (*exp).pcap_drop_avg,
        (*exp).fail_total,
        (*exp).fail_last,
        (*exp).fail_avg,
    );
    fflush(f);
}
#[inline]
unsafe extern "C" fn check_min_hitrate(mut exp: *mut export_status_t) {
    if (*exp).seconds_under_min_hitrate >= 5 as libc::c_int as libc::c_float {
        log_fatal(
            b"monitor\0" as *const u8 as *const libc::c_char,
            b"hitrate below %.0f for %.0f seconds. aborting scan.\0" as *const u8
                as *const libc::c_char,
            zconf.min_hitrate as libc::c_double,
            (*exp).seconds_under_min_hitrate as libc::c_double,
        );
    }
}
#[inline]
unsafe extern "C" fn check_max_sendto_failures(mut exp: *mut export_status_t) {
    if zconf.max_sendto_failures >= 0 as libc::c_int
        && (*exp).fail_total > zconf.max_sendto_failures as uint32_t
    {
        log_fatal(
            b"monitor\0" as *const u8 as *const libc::c_char,
            b"maximum number of sendto failures (%i) exceeded\0" as *const u8
                as *const libc::c_char,
            zconf.max_sendto_failures,
        );
    }
}
pub unsafe extern "C" fn monitor_init() {
    if !(zconf.status_updates_file).is_null() {
        status_fd = init_status_update_file(zconf.status_updates_file);
        if !status_fd.is_null() {} else {
            __assert_fail(
                b"status_fd\0" as *const u8 as *const libc::c_char,
                b"monitor.c\0" as *const u8 as *const libc::c_char,
                449 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 24],
                    &[libc::c_char; 24],
                >(b"void monitor_init(void)\0"))
                    .as_ptr(),
            );
        }
        'c_5077: {
            if !status_fd.is_null() {} else {
                __assert_fail(
                    b"status_fd\0" as *const u8 as *const libc::c_char,
                    b"monitor.c\0" as *const u8 as *const libc::c_char,
                    449 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 24],
                        &[libc::c_char; 24],
                    >(b"void monitor_init(void)\0"))
                        .as_ptr(),
                );
            }
        };
    }
}
pub unsafe extern "C" fn monitor_run(
    mut it: *mut iterator_t,
    mut lock: *mut pthread_mutex_t,
) {
    let mut internal_status: *mut int_status_t = xmalloc(
        ::std::mem::size_of::<int_status_t>() as libc::c_ulong,
    ) as *mut int_status_t;
    let mut export_status: *mut export_status_t = xmalloc(
        ::std::mem::size_of::<export_status_t>() as libc::c_ulong,
    ) as *mut export_status_t;
    while !(zsend.complete != 0 && zrecv.complete != 0) {
        update_pcap_stats(lock);
        export_stats(internal_status, export_status, it);
        log_drop_warnings(export_status);
        check_min_hitrate(export_status);
        check_max_sendto_failures(export_status);
        if zconf.quiet == 0 {
            lock_file(stderr);
            if zconf.fsconf.app_success_index >= 0 as libc::c_int {
                onscreen_appsuccess(export_status);
            } else {
                onscreen_generic(export_status);
            }
            unlock_file(stderr);
        }
        if !status_fd.is_null() {
            update_status_updates_file(export_status, status_fd);
        }
        sleep(1 as libc::c_int as libc::c_uint);
    }
    if zconf.quiet == 0 {
        lock_file(stderr);
        fflush(stderr);
        unlock_file(stderr);
    }
    if !status_fd.is_null() {
        fflush(status_fd);
        fclose(status_fd);
    }
}
