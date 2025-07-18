use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type aesrand;
    pub type iterator;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type output_module;
    fn sendto(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: *const sockaddr,
        __addr_len: socklen_t,
    ) -> ssize_t;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    fn shard_get_cur_ip(shard: *mut shard_t) -> uint32_t;
    fn shard_get_next_ip(shard: *mut shard_t) -> uint32_t;
    fn iterator_init(
        num_threads: uint8_t,
        shard: uint16_t,
        num_shards: uint16_t,
    ) -> *mut iterator_t;
    static mut stdout: *mut FILE;
    fn perror(__s: *const libc::c_char);
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
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
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn log_fatal(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> !;
    fn log_error(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn log_warn(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn log_info(
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
    fn blocklist_count_allowed() -> uint64_t;
    fn lock_file(f: *mut FILE) -> libc::c_int;
    fn unlock_file(f: *mut FILE) -> libc::c_int;
    fn pbm_check(b: *mut *mut uint8_t, v: uint32_t) -> libc::c_int;
    fn get_iface_hw_addr(
        iface: *mut libc::c_char,
        hw_mac: *mut libc::c_uchar,
    ) -> libc::c_int;
    static mut zrecv: state_recv;
    static mut zsend: state_send;
    static mut zconf: state_conf;
    fn validate_init();
    fn validate_gen(src: uint32_t, dst: uint32_t, output: *mut uint8_t);
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
}
pub type __u_char = libc::c_uchar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __caddr_t = *mut libc::c_char;
pub type __socklen_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type u_char = __u_char;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct ip {
    #[bitfield(name = "ip_hl", ty = "libc::c_uint", bits = "0..=3")]
    #[bitfield(name = "ip_v", ty = "libc::c_uint", bits = "4..=7")]
    pub ip_hl_ip_v: [u8; 1],
    pub ip_tos: uint8_t,
    pub ip_len: libc::c_ushort,
    pub ip_id: libc::c_ushort,
    pub ip_off: libc::c_ushort,
    pub ip_ttl: uint8_t,
    pub ip_p: uint8_t,
    pub ip_sum: libc::c_ushort,
    pub ip_src: in_addr,
    pub ip_dst: in_addr,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ether_header {
    pub ether_dhost: [uint8_t; 6],
    pub ether_shost: [uint8_t; 6],
    pub ether_type: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifmap {
    pub mem_start: libc::c_ulong,
    pub mem_end: libc::c_ulong,
    pub base_addr: libc::c_ushort,
    pub irq: libc::c_uchar,
    pub dma: libc::c_uchar,
    pub port: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifreq {
    pub ifr_ifrn: C2RustUnnamed_0,
    pub ifr_ifru: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ifru_addr: sockaddr,
    pub ifru_dstaddr: sockaddr,
    pub ifru_broadaddr: sockaddr,
    pub ifru_netmask: sockaddr,
    pub ifru_hwaddr: sockaddr,
    pub ifru_flags: libc::c_short,
    pub ifru_ivalue: libc::c_int,
    pub ifru_mtu: libc::c_int,
    pub ifru_map: ifmap,
    pub ifru_slave: [libc::c_char; 16],
    pub ifru_newname: [libc::c_char; 16],
    pub ifru_data: __caddr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ifrn_name: [libc::c_char; 16],
}
pub type aesrand_t = aesrand;
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
pub type iterator_t = iterator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sock_t {
    pub sock: libc::c_int,
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
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
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
pub type macaddr_t = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct probe_module {
    pub name: *const libc::c_char,
    pub max_packet_length: size_t,
    pub pcap_filter: *const libc::c_char,
    pub pcap_snaplen: size_t,
    pub port_args: uint8_t,
    pub global_initialize: probe_global_init_cb,
    pub thread_initialize: probe_thread_init_cb,
    pub make_packet: probe_make_packet_cb,
    pub print_packet: probe_print_packet_cb,
    pub validate_packet: probe_validate_packet_cb,
    pub process_packet: probe_classify_packet_cb,
    pub close: probe_close_cb,
    pub output_type: libc::c_int,
    pub fields: *mut fielddef_t,
    pub numfields: libc::c_int,
    pub helptext: *const libc::c_char,
}
pub type probe_close_cb = Option::<
    unsafe extern "C" fn(
        *mut state_conf,
        *mut state_send,
        *mut state_recv,
    ) -> libc::c_int,
>;
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
pub type probe_classify_packet_cb = Option::<
    unsafe extern "C" fn(
        *const u_char,
        uint32_t,
        *mut fieldset_t,
        *mut uint32_t,
        timespec,
    ) -> (),
>;
pub type fieldset_t = fieldset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fieldset {
    pub len: libc::c_int,
    pub fields: [field_t; 128],
    pub fds: *mut fielddefset_t,
    pub inner_type: libc::c_int,
    pub type_0: libc::c_int,
    pub free_: libc::c_int,
}
pub type field_t = field;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field {
    pub name: *const libc::c_char,
    pub type_0: libc::c_int,
    pub free_: libc::c_int,
    pub len: size_t,
    pub value: field_val_t,
}
pub type field_val_t = field_val;
#[derive(Copy, Clone)]
#[repr(C)]
pub union field_val {
    pub ptr: *mut libc::c_void,
    pub num: uint64_t,
}
pub type probe_validate_packet_cb = Option::<
    unsafe extern "C" fn(
        *const ip,
        uint32_t,
        *mut uint32_t,
        *mut uint32_t,
    ) -> libc::c_int,
>;
pub type probe_print_packet_cb = Option::<
    unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> (),
>;
pub type probe_make_packet_cb = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut size_t,
        ipaddr_n_t,
        ipaddr_n_t,
        uint8_t,
        *mut uint32_t,
        libc::c_int,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type ipaddr_n_t = uint32_t;
pub type probe_thread_init_cb = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut macaddr_t,
        *mut macaddr_t,
        port_n_t,
        *mut *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type port_n_t = uint16_t;
pub type probe_global_init_cb = Option::<
    unsafe extern "C" fn(*mut state_conf) -> libc::c_int,
>;
pub type port_h_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_ll {
    pub sll_family: libc::c_ushort,
    pub sll_protocol: libc::c_ushort,
    pub sll_ifindex: libc::c_int,
    pub sll_hatype: libc::c_ushort,
    pub sll_pkttype: libc::c_uchar,
    pub sll_halen: libc::c_uchar,
    pub sll_addr: [libc::c_uchar; 8],
}
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_1 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_1 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_1 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_1 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_1 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_1 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_1 = 1;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
unsafe extern "C" fn send_run_init(mut s: sock_t) -> libc::c_int {
    let mut sock: libc::c_int = s.sock;
    let mut if_idx: ifreq = ifreq {
        ifr_ifrn: C2RustUnnamed_0 {
            ifrn_name: [0; 16],
        },
        ifr_ifru: C2RustUnnamed {
            ifru_addr: sockaddr {
                sa_family: 0,
                sa_data: [0; 14],
            },
        },
    };
    memset(
        &mut if_idx as *mut ifreq as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ifreq>() as libc::c_ulong,
    );
    if strlen(zconf.iface) >= 16 as libc::c_int as libc::c_ulong {
        log_error(
            b"send\0" as *const u8 as *const libc::c_char,
            b"device interface name (%s) too long\n\0" as *const u8
                as *const libc::c_char,
            zconf.iface,
        );
        return 1 as libc::c_int;
    }
    strncpy(
        (if_idx.ifr_ifrn.ifrn_name).as_mut_ptr(),
        zconf.iface,
        (16 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
    );
    if ioctl(sock, 0x8933 as libc::c_int as libc::c_ulong, &mut if_idx as *mut ifreq)
        < 0 as libc::c_int
    {
        perror(b"SIOCGIFINDEX\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    let mut ifindex: libc::c_int = if_idx.ifr_ifru.ifru_ivalue;
    memset(
        &mut sockaddr as *mut sockaddr_ll as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_ll>() as libc::c_ulong,
    );
    sockaddr.sll_ifindex = ifindex;
    sockaddr.sll_halen = 6 as libc::c_int as libc::c_uchar;
    if zconf.send_ip_pkts != 0 {
        sockaddr.sll_protocol = __bswap_16(0x800 as libc::c_int as __uint16_t);
    }
    memcpy(
        (sockaddr.sll_addr).as_mut_ptr() as *mut libc::c_void,
        (zconf.gw_mac).as_mut_ptr() as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
static mut sockaddr: sockaddr_ll = sockaddr_ll {
    sll_family: 0,
    sll_protocol: 0,
    sll_ifindex: 0,
    sll_hatype: 0,
    sll_pkttype: 0,
    sll_halen: 0,
    sll_addr: [0; 8],
};
unsafe extern "C" fn send_packet(
    mut sock: sock_t,
    mut buf: *mut libc::c_void,
    mut len: libc::c_int,
    mut idx: uint32_t,
) -> libc::c_int {
    return sendto(
        sock.sock,
        buf,
        len as size_t,
        0 as libc::c_int,
        &mut sockaddr as *mut sockaddr_ll as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_ll>() as libc::c_ulong as socklen_t,
    ) as libc::c_int;
}
static mut send_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut num_src_ports: uint16_t = 0;
pub unsafe extern "C" fn sig_handler_increase_speed(mut signal_0: libc::c_int) {
    let mut old_rate: libc::c_int = zconf.rate;
    zconf
        .rate = (zconf.rate as libc::c_double + zconf.rate as libc::c_double * 0.05f64)
        as libc::c_int;
    log_info(
        b"send\0" as *const u8 as *const libc::c_char,
        b"send rate increased from %i to %i pps.\0" as *const u8 as *const libc::c_char,
        old_rate,
        zconf.rate,
    );
}
pub unsafe extern "C" fn sig_handler_decrease_speed(mut signal_0: libc::c_int) {
    let mut old_rate: libc::c_int = zconf.rate;
    zconf
        .rate = (zconf.rate as libc::c_double - zconf.rate as libc::c_double * 0.05f64)
        as libc::c_int;
    log_info(
        b"send\0" as *const u8 as *const libc::c_char,
        b"send rate decreased from %i to %i pps.\0" as *const u8 as *const libc::c_char,
        old_rate,
        zconf.rate,
    );
}
pub unsafe extern "C" fn send_init() -> *mut iterator_t {
    let mut it: *mut iterator_t = 0 as *mut iterator_t;
    let mut num_subshards: uint32_t = (zconf.senders as uint32_t)
        .wrapping_mul(zconf.total_shards as uint32_t);
    if num_subshards as libc::c_ulong > blocklist_count_allowed() {
        log_fatal(
            b"send\0" as *const u8 as *const libc::c_char,
            b"senders * shards > allowed probes\0" as *const u8 as *const libc::c_char,
        );
    }
    if zsend.max_targets != 0 && num_subshards > zsend.max_targets {
        log_fatal(
            b"send\0" as *const u8 as *const libc::c_char,
            b"senders * shards > max targets\0" as *const u8 as *const libc::c_char,
        );
    }
    it = iterator_init(zconf.senders, zconf.shard_num, zconf.total_shards);
    let mut temp: in_addr = in_addr { s_addr: 0 };
    temp.s_addr = zconf.source_ip_addresses[0 as libc::c_int as usize];
    log_debug(
        b"send\0" as *const u8 as *const libc::c_char,
        b"srcip_first: %s\0" as *const u8 as *const libc::c_char,
        inet_ntoa(temp),
    );
    temp
        .s_addr = zconf
        .source_ip_addresses[(zconf.number_source_ips)
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize];
    log_debug(
        b"send\0" as *const u8 as *const libc::c_char,
        b"srcip_last: %s\0" as *const u8 as *const libc::c_char,
        inet_ntoa(temp),
    );
    num_src_ports = (zconf.source_port_last as libc::c_int
        - zconf.source_port_first as libc::c_int + 1 as libc::c_int) as uint16_t;
    log_debug(
        b"send\0" as *const u8 as *const libc::c_char,
        b"will send from %u address%s on %hu source ports\0" as *const u8
            as *const libc::c_char,
        zconf.number_source_ips,
        if zconf.number_source_ips == 1 as libc::c_int as libc::c_uint {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"es\0" as *const u8 as *const libc::c_char
        },
        num_src_ports as libc::c_int,
    );
    if !(zconf.probe_module).is_null() {} else {
        __assert_fail(
            b"zconf.probe_module\0" as *const u8 as *const libc::c_char,
            b"send.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"iterator_t *send_init(void)\0"))
                .as_ptr(),
        );
    }
    'c_2968: {
        if !(zconf.probe_module).is_null() {} else {
            __assert_fail(
                b"zconf.probe_module\0" as *const u8 as *const libc::c_char,
                b"send.c\0" as *const u8 as *const libc::c_char,
                103 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"iterator_t *send_init(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if ((*zconf.probe_module).global_initialize).is_some() {
        if ((*zconf.probe_module).global_initialize).unwrap()(&mut zconf) != 0 {
            log_fatal(
                b"send\0" as *const u8 as *const libc::c_char,
                b"global initialization for probe module failed.\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if zconf.bandwidth > 0 as libc::c_int as libc::c_ulong
        && zconf.rate > 0 as libc::c_int
    {
        log_fatal(
            b"send\0" as *const u8 as *const libc::c_char,
            b"must specify rate or bandwidth, or neither, not both.\0" as *const u8
                as *const libc::c_char,
        );
    }
    if zconf.bandwidth > 0 as libc::c_int as libc::c_ulong {
        let mut pkt_len: size_t = (*zconf.probe_module).max_packet_length;
        pkt_len = (pkt_len as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong) as size_t as size_t;
        pkt_len = (pkt_len as libc::c_ulong)
            .wrapping_add((8 as libc::c_int * 24 as libc::c_int) as libc::c_ulong)
            as size_t as size_t;
        if pkt_len < (84 as libc::c_int * 8 as libc::c_int) as libc::c_ulong {
            pkt_len = (84 as libc::c_int * 8 as libc::c_int) as size_t;
        }
        if (zconf.bandwidth).wrapping_div(pkt_len)
            > 0xffffffff as libc::c_uint as libc::c_ulong
        {
            zconf.rate = 0 as libc::c_int;
        } else {
            zconf.rate = (zconf.bandwidth).wrapping_div(pkt_len) as libc::c_int;
            if zconf.rate == 0 as libc::c_int {
                log_warn(
                    b"send\0" as *const u8 as *const libc::c_char,
                    b"bandwidth %lu bit/s is slower than 1 pkt/s, setting rate to 1 pkt/s\0"
                        as *const u8 as *const libc::c_char,
                    zconf.bandwidth,
                );
                zconf.rate = 1 as libc::c_int;
            }
        }
        log_debug(
            b"send\0" as *const u8 as *const libc::c_char,
            b"using bandwidth %lu bits/s for %zu byte probe, rate set to %d pkt/s\0"
                as *const u8 as *const libc::c_char,
            zconf.bandwidth,
            pkt_len.wrapping_div(8 as libc::c_int as libc::c_ulong),
            zconf.rate,
        );
    }
    if zconf.rate == -(1 as libc::c_int) {
        zconf.rate = 10000 as libc::c_int;
    }
    if zconf.rate < 0 as libc::c_int {
        log_fatal(
            b"send\0" as *const u8 as *const libc::c_char,
            b"rate impossibly slow\0" as *const u8 as *const libc::c_char,
        );
    }
    if zconf.rate > 0 as libc::c_int
        && zconf.bandwidth <= 0 as libc::c_int as libc::c_ulong
    {
        log_debug(
            b"send\0" as *const u8 as *const libc::c_char,
            b"rate set to %d pkt/s\0" as *const u8 as *const libc::c_char,
            zconf.rate,
        );
    }
    if zconf.hw_mac_set == 0 {
        if get_iface_hw_addr(zconf.iface, (zconf.hw_mac).as_mut_ptr()) != 0 {
            log_fatal(
                b"send\0" as *const u8 as *const libc::c_char,
                b"ZMap could not retrieve the hardware (MAC) address for the interface \"%s\". You likely do not privileges to open a raw packet socket. Are you running as root or with the CAP_NET_RAW capability? If you are, you may need to manually set the source MAC address with the \"--source-mac\" flag.\0"
                    as *const u8 as *const libc::c_char,
                zconf.iface,
            );
        }
        log_debug(
            b"send\0" as *const u8 as *const libc::c_char,
            b"no source MAC provided. automatically detected %02x:%02x:%02x:%02x:%02x:%02x as hw interface for %s\0"
                as *const u8 as *const libc::c_char,
            zconf.hw_mac[0 as libc::c_int as usize] as libc::c_int,
            zconf.hw_mac[1 as libc::c_int as usize] as libc::c_int,
            zconf.hw_mac[2 as libc::c_int as usize] as libc::c_int,
            zconf.hw_mac[3 as libc::c_int as usize] as libc::c_int,
            zconf.hw_mac[4 as libc::c_int as usize] as libc::c_int,
            zconf.hw_mac[5 as libc::c_int as usize] as libc::c_int,
            zconf.iface,
        );
    }
    log_debug(
        b"send\0" as *const u8 as *const libc::c_char,
        b"source MAC address %02x:%02x:%02x:%02x:%02x:%02x\0" as *const u8
            as *const libc::c_char,
        zconf.hw_mac[0 as libc::c_int as usize] as libc::c_int,
        zconf.hw_mac[1 as libc::c_int as usize] as libc::c_int,
        zconf.hw_mac[2 as libc::c_int as usize] as libc::c_int,
        zconf.hw_mac[3 as libc::c_int as usize] as libc::c_int,
        zconf.hw_mac[4 as libc::c_int as usize] as libc::c_int,
        zconf.hw_mac[5 as libc::c_int as usize] as libc::c_int,
    );
    if zconf.dryrun != 0 {
        log_info(
            b"send\0" as *const u8 as *const libc::c_char,
            b"dryrun mode -- won't actually send packets\0" as *const u8
                as *const libc::c_char,
        );
    }
    validate_init();
    signal(
        10 as libc::c_int,
        Some(sig_handler_increase_speed as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    signal(
        12 as libc::c_int,
        Some(sig_handler_decrease_speed as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    zsend.start = now();
    return it;
}
#[inline]
unsafe extern "C" fn get_src_ip(
    mut dst: ipaddr_n_t,
    mut local_offset: libc::c_int,
) -> ipaddr_n_t {
    if zconf.number_source_ips == 1 as libc::c_int as libc::c_uint {
        return zconf.source_ip_addresses[0 as libc::c_int as usize];
    }
    return zconf
        .source_ip_addresses[(__bswap_32(dst))
        .wrapping_add(local_offset as libc::c_uint)
        .wrapping_rem(zconf.number_source_ips) as usize];
}
pub unsafe extern "C" fn send_run(mut st: sock_t, mut s: *mut shard_t) -> libc::c_int {
    let mut attempts: libc::c_int = 0;
    let mut idx: uint32_t = 0;
    let mut current_block: u64;
    log_debug(
        b"send\0" as *const u8 as *const libc::c_char,
        b"send thread started\0" as *const u8 as *const libc::c_char,
    );
    pthread_mutex_lock(&mut send_mutex);
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    memset(
        buf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        4096 as libc::c_int as libc::c_ulong,
    );
    if send_run_init(st) != 0 {
        pthread_mutex_unlock(&mut send_mutex);
        return -(1 as libc::c_int);
    }
    let mut mac_buf: [libc::c_char; 18] = [0; 18];
    let mut p: *mut libc::c_char = mac_buf.as_mut_ptr();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        if i == 6 as libc::c_int - 1 as libc::c_int {
            snprintf(
                p,
                3 as libc::c_int as libc::c_ulong,
                b"%.2x\0" as *const u8 as *const libc::c_char,
                zconf.hw_mac[i as usize] as libc::c_int,
            );
            p = p.offset(2 as libc::c_int as isize);
        } else {
            snprintf(
                p,
                4 as libc::c_int as libc::c_ulong,
                b"%.2x:\0" as *const u8 as *const libc::c_char,
                zconf.hw_mac[i as usize] as libc::c_int,
            );
            p = p.offset(3 as libc::c_int as isize);
        }
        i += 1;
        i;
    }
    log_debug(
        b"send\0" as *const u8 as *const libc::c_char,
        b"source MAC address %s\0" as *const u8 as *const libc::c_char,
        mac_buf.as_mut_ptr(),
    );
    let mut probe_data: *mut libc::c_void = 0 as *mut libc::c_void;
    if ((*zconf.probe_module).thread_initialize).is_some() {
        ((*zconf.probe_module).thread_initialize)
            .unwrap()(
            buf.as_mut_ptr() as *mut libc::c_void,
            (zconf.hw_mac).as_mut_ptr(),
            (zconf.gw_mac).as_mut_ptr(),
            zconf.target_port,
            &mut probe_data,
        );
    }
    pthread_mutex_unlock(&mut send_mutex);
    let mut count: uint64_t = 0 as libc::c_int as uint64_t;
    let mut last_count: uint64_t = count;
    let mut last_time: libc::c_double = now();
    let mut delay: uint32_t = 0 as libc::c_int as uint32_t;
    let mut interval: libc::c_int = 0 as libc::c_int;
    let mut vi: libc::c_int = 0;
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut rem: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut send_rate: libc::c_double = zconf.rate as libc::c_double
        / (zconf.senders as libc::c_double * zconf.batch as libc::c_int as libc::c_double
            * zconf.packet_streams as libc::c_double);
    let slow_rate: libc::c_double = 50 as libc::c_int as libc::c_double;
    let mut nsec_per_sec: libc::c_long = (1000 as libc::c_int * 1000 as libc::c_int
        * 1000 as libc::c_int) as libc::c_long;
    let mut sleep_time: libc::c_longlong = nsec_per_sec as libc::c_longlong;
    if zconf.rate > 0 as libc::c_int {
        delay = 10000 as libc::c_int as uint32_t;
        if send_rate < slow_rate {
            sleep_time = (nsec_per_sec as libc::c_double / send_rate)
                as libc::c_longlong;
            last_time = now() - 1.0f64 / send_rate;
        } else {
            ::std::ptr::write_volatile(
                &mut vi as *mut libc::c_int,
                delay as libc::c_int,
            );
            loop {
                let fresh0 = ::std::ptr::read_volatile::<
                    libc::c_int,
                >(&vi as *const libc::c_int);
                ::std::ptr::write_volatile(
                    &mut vi as *mut libc::c_int,
                    ::std::ptr::read_volatile::<libc::c_int>(&vi as *const libc::c_int)
                        - 1,
                );
                if !(fresh0 != 0) {
                    break;
                }
            }
            delay = (delay as libc::c_double
                * (1 as libc::c_int as libc::c_double / (now() - last_time)
                    / (zconf.rate as libc::c_double
                        / (zconf.senders as libc::c_double
                            * zconf.batch as libc::c_int as libc::c_double))))
                as uint32_t;
            interval = (zconf.rate as libc::c_double
                / (zconf.senders as libc::c_double
                    * zconf.batch as libc::c_int as libc::c_double)
                / 20 as libc::c_int as libc::c_double) as libc::c_int;
            last_time = now();
        }
    }
    let mut current_ip: uint32_t = shard_get_cur_ip(s);
    if !(zconf.list_of_ips_filename).is_null() {
        current_block = 14434620278749266018;
    } else {
        current_block = 7746103178988627676;
    }
    loop {
        match current_block {
            7746103178988627676 => {
                attempts = zconf.num_retries + 1 as libc::c_int;
                idx = 0 as libc::c_int as uint32_t;
                current_block = 7226443171521532240;
                break;
            }
            _ => {
                if !(pbm_check(zsend.list_of_ips_pbm, current_ip) == 0) {
                    current_block = 7746103178988627676;
                    continue;
                }
                current_ip = shard_get_next_ip(s);
                if !(current_ip == 0 as libc::c_int as libc::c_uint) {
                    current_block = 14434620278749266018;
                    continue;
                }
                log_debug(
                    b"send\0" as *const u8 as *const libc::c_char,
                    b"never made it to send loop in send thread %i\0" as *const u8
                        as *const libc::c_char,
                    (*s).thread_id as libc::c_int,
                );
                current_block = 12986049523627189704;
                break;
            }
        }
    }
    loop {
        match current_block {
            12986049523627189704 => {
                ((*s).cb).unwrap()((*s).thread_id, (*s).arg);
                break;
            }
            _ => {
                if count != 0 && delay > 0 as libc::c_int as libc::c_uint {
                    if send_rate < slow_rate {
                        let mut t: libc::c_double = now();
                        let mut last_rate: libc::c_double = 1.0f64 / (t - last_time);
                        sleep_time = (sleep_time as libc::c_double
                            * ((last_rate / send_rate
                                + 1 as libc::c_int as libc::c_double)
                                / 2 as libc::c_int as libc::c_double)) as libc::c_longlong;
                        ts
                            .tv_sec = (sleep_time / nsec_per_sec as libc::c_longlong)
                            as __time_t;
                        ts
                            .tv_nsec = (sleep_time % nsec_per_sec as libc::c_longlong)
                            as __syscall_slong_t;
                        log_debug(
                            b"sleep\0" as *const u8 as *const libc::c_char,
                            b"sleep for %d sec, %ld nanoseconds\0" as *const u8
                                as *const libc::c_char,
                            ts.tv_sec,
                            ts.tv_nsec,
                        );
                        while nanosleep(&mut ts, &mut rem) == -(1 as libc::c_int) {}
                        last_time = t;
                    } else {
                        ::std::ptr::write_volatile(
                            &mut vi as *mut libc::c_int,
                            delay as libc::c_int,
                        );
                        loop {
                            let fresh1 = ::std::ptr::read_volatile::<
                                libc::c_int,
                            >(&vi as *const libc::c_int);
                            ::std::ptr::write_volatile(
                                &mut vi as *mut libc::c_int,
                                ::std::ptr::read_volatile::<
                                    libc::c_int,
                                >(&vi as *const libc::c_int) - 1,
                            );
                            if !(fresh1 != 0) {
                                break;
                            }
                        }
                        if interval == 0
                            || count.wrapping_rem(interval as libc::c_ulong)
                                == 0 as libc::c_int as libc::c_ulong
                        {
                            let mut t_0: libc::c_double = now();
                            if count > last_count {} else {
                                __assert_fail(
                                    b"count > last_count\0" as *const u8 as *const libc::c_char,
                                    b"send.c\0" as *const u8 as *const libc::c_char,
                                    320 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 32],
                                        &[libc::c_char; 32],
                                    >(b"int send_run(sock_t, shard_t *)\0"))
                                        .as_ptr(),
                                );
                            }
                            'c_4021: {
                                if count > last_count {} else {
                                    __assert_fail(
                                        b"count > last_count\0" as *const u8 as *const libc::c_char,
                                        b"send.c\0" as *const u8 as *const libc::c_char,
                                        320 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 32],
                                            &[libc::c_char; 32],
                                        >(b"int send_run(sock_t, shard_t *)\0"))
                                            .as_ptr(),
                                    );
                                }
                            };
                            if t_0 > last_time {} else {
                                __assert_fail(
                                    b"t > last_time\0" as *const u8 as *const libc::c_char,
                                    b"send.c\0" as *const u8 as *const libc::c_char,
                                    321 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 32],
                                        &[libc::c_char; 32],
                                    >(b"int send_run(sock_t, shard_t *)\0"))
                                        .as_ptr(),
                                );
                            }
                            'c_3981: {
                                if t_0 > last_time {} else {
                                    __assert_fail(
                                        b"t > last_time\0" as *const u8 as *const libc::c_char,
                                        b"send.c\0" as *const u8 as *const libc::c_char,
                                        321 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 32],
                                            &[libc::c_char; 32],
                                        >(b"int send_run(sock_t, shard_t *)\0"))
                                            .as_ptr(),
                                    );
                                }
                            };
                            let mut multiplier: libc::c_double = count
                                .wrapping_sub(last_count) as libc::c_double
                                / (t_0 - last_time)
                                / (zconf.rate / zconf.senders as libc::c_int)
                                    as libc::c_double;
                            let mut old_delay: uint32_t = delay;
                            delay = (delay as libc::c_double * multiplier) as uint32_t;
                            if delay == old_delay {
                                if multiplier > 1.0f64 {
                                    delay = (delay as libc::c_uint)
                                        .wrapping_mul(2 as libc::c_int as libc::c_uint) as uint32_t
                                        as uint32_t;
                                } else if multiplier < 1.0f64 {
                                    delay = (delay as libc::c_double * 0.5f64) as uint32_t;
                                }
                            }
                            last_count = count;
                            last_time = t_0;
                        }
                    }
                }
                if zrecv.complete != 0 {
                    current_block = 12986049523627189704;
                    continue;
                }
                if zconf.max_runtime != 0
                    && zconf.max_runtime as libc::c_double <= now() - zsend.start
                {
                    current_block = 12986049523627189704;
                    continue;
                }
                let mut b: libc::c_int = 0 as libc::c_int;
                's_349: loop {
                    if !(b < zconf.batch as libc::c_int) {
                        current_block = 7226443171521532240;
                        break;
                    }
                    if (*s).state.max_hosts != 0
                        && (*s).state.hosts_scanned >= (*s).state.max_hosts
                    {
                        log_debug(
                            b"send\0" as *const u8 as *const libc::c_char,
                            b"send thread %hhu finished (max targets of %u reached)\0"
                                as *const u8 as *const libc::c_char,
                            (*s).thread_id as libc::c_int,
                            (*s).state.max_hosts,
                        );
                        current_block = 12986049523627189704;
                        break;
                    } else if (*s).state.max_packets != 0
                        && (*s).state.packets_sent
                            >= (*s).state.max_packets as libc::c_ulong
                    {
                        log_debug(
                            b"send\0" as *const u8 as *const libc::c_char,
                            b"send thread %hhu finished (max packets of %u reached)\0"
                                as *const u8 as *const libc::c_char,
                            (*s).thread_id as libc::c_int,
                            (*s).state.max_packets,
                        );
                        current_block = 12986049523627189704;
                        break;
                    } else if current_ip == 0 as libc::c_int as libc::c_uint {
                        log_debug(
                            b"send\0" as *const u8 as *const libc::c_char,
                            b"send thread %hhu finished, shard depleted\0" as *const u8
                                as *const libc::c_char,
                            (*s).thread_id as libc::c_int,
                        );
                        current_block = 12986049523627189704;
                        break;
                    } else {
                        let mut i_0: libc::c_int = 0 as libc::c_int;
                        while i_0 < zconf.packet_streams {
                            count = count.wrapping_add(1);
                            count;
                            let mut src_ip: uint32_t = get_src_ip(current_ip, i_0);
                            let mut validation: [uint32_t; 4] = [0; 4];
                            validate_gen(
                                src_ip,
                                current_ip,
                                validation.as_mut_ptr() as *mut uint8_t,
                            );
                            let mut ttl: uint8_t = zconf.probe_ttl;
                            let mut length: size_t = 0 as libc::c_int as size_t;
                            ((*zconf.probe_module).make_packet)
                                .unwrap()(
                                buf.as_mut_ptr() as *mut libc::c_void,
                                &mut length,
                                src_ip,
                                current_ip,
                                ttl,
                                validation.as_mut_ptr(),
                                i_0,
                                probe_data,
                            );
                            if length > 4096 as libc::c_int as libc::c_ulong {
                                log_fatal(
                                    b"send\0" as *const u8 as *const libc::c_char,
                                    b"send thread %hhu set length (%zu) larger than MAX (%zu)\0"
                                        as *const u8 as *const libc::c_char,
                                    (*s).thread_id as libc::c_int,
                                    length,
                                    4096 as libc::c_int,
                                );
                            }
                            if zconf.dryrun != 0 {
                                lock_file(stdout);
                                ((*zconf.probe_module).print_packet)
                                    .unwrap()(stdout, buf.as_mut_ptr() as *mut libc::c_void);
                                unlock_file(stdout);
                            } else {
                                let mut contents: *mut libc::c_void = buf
                                    .as_mut_ptr()
                                    .offset(
                                        (zconf.send_ip_pkts as libc::c_ulong)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<ether_header>() as libc::c_ulong,
                                            ) as isize,
                                    ) as *mut libc::c_void;
                                length = (length as libc::c_ulong)
                                    .wrapping_sub(
                                        (zconf.send_ip_pkts as libc::c_ulong)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<ether_header>() as libc::c_ulong,
                                            ),
                                    ) as size_t as size_t;
                                let mut any_sends_successful: libc::c_int = 0
                                    as libc::c_int;
                                let mut i_1: libc::c_int = 0 as libc::c_int;
                                while i_1 < attempts {
                                    let mut rc: libc::c_int = send_packet(
                                        st,
                                        contents,
                                        length as libc::c_int,
                                        idx,
                                    );
                                    if rc < 0 as libc::c_int {
                                        let mut addr: in_addr = in_addr { s_addr: 0 };
                                        addr.s_addr = current_ip;
                                        let mut addr_str_buf: [libc::c_char; 16] = [0; 16];
                                        let mut addr_str: *const libc::c_char = inet_ntop(
                                            2 as libc::c_int,
                                            &mut addr as *mut in_addr as *const libc::c_void,
                                            addr_str_buf.as_mut_ptr(),
                                            16 as libc::c_int as socklen_t,
                                        );
                                        if !addr_str.is_null() {
                                            log_debug(
                                                b"send\0" as *const u8 as *const libc::c_char,
                                                b"send_packet failed for %s. %s\0" as *const u8
                                                    as *const libc::c_char,
                                                addr_str,
                                                strerror(*__errno_location()),
                                            );
                                        }
                                        i_1 += 1;
                                        i_1;
                                    } else {
                                        any_sends_successful = 1 as libc::c_int;
                                        break;
                                    }
                                }
                                if any_sends_successful == 0 {
                                    (*s)
                                        .state
                                        .packets_failed = ((*s).state.packets_failed)
                                        .wrapping_add(1);
                                    (*s).state.packets_failed;
                                }
                                idx = idx.wrapping_add(1);
                                idx;
                                idx &= 0xff as libc::c_int as libc::c_uint;
                            }
                            (*s)
                                .state
                                .packets_sent = ((*s).state.packets_sent).wrapping_add(1);
                            (*s).state.packets_sent;
                            i_0 += 1;
                            i_0;
                        }
                        (*s)
                            .state
                            .hosts_scanned = ((*s).state.hosts_scanned).wrapping_add(1);
                        (*s).state.hosts_scanned;
                        current_ip = shard_get_next_ip(s);
                        if !(zconf.list_of_ips_filename).is_null()
                            && current_ip != 0 as libc::c_int as libc::c_uint
                        {
                            while pbm_check(zsend.list_of_ips_pbm, current_ip) == 0 {
                                current_ip = shard_get_next_ip(s);
                                if !(current_ip == 0 as libc::c_int as libc::c_uint) {
                                    continue;
                                }
                                log_debug(
                                    b"send\0" as *const u8 as *const libc::c_char,
                                    b"send thread %hhu shard finished in get_next_ip_loop depleted\0"
                                        as *const u8 as *const libc::c_char,
                                    (*s).thread_id as libc::c_int,
                                );
                                current_block = 12986049523627189704;
                                break 's_349;
                            }
                        }
                        b += 1;
                        b;
                    }
                }
            }
        }
    }
    if zconf.dryrun != 0 {
        lock_file(stdout);
        fflush(stdout);
        unlock_file(stdout);
    }
    log_debug(
        b"send\0" as *const u8 as *const libc::c_char,
        b"thread %hu cleanly finished\0" as *const u8 as *const libc::c_char,
        (*s).thread_id as libc::c_int,
    );
    return 0 as libc::c_int;
}
