use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type aesrand;
    pub type output_module;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn blocklist_is_allowed(s_addr: uint32_t) -> libc::c_int;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    fn xcalloc(count: size_t, size: size_t) -> *mut libc::c_void;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    fn xrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn lock_file(f: *mut FILE) -> libc::c_int;
    fn unlock_file(f: *mut FILE) -> libc::c_int;
    fn log_warn(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn log_error(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn log_fatal(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> !;
    fn aesrand_init_from_seed(_: uint64_t) -> *mut aesrand_t;
    fn aesrand_getword(aes: *mut aesrand_t) -> uint64_t;
    static mut zconf: state_conf;
    fn fs_add_binary(
        fs: *mut fieldset_t,
        name: *const libc::c_char,
        len: size_t,
        value: *mut libc::c_void,
        free_: libc::c_int,
    );
    fn fs_add_bool(fs: *mut fieldset_t, name: *const libc::c_char, value: libc::c_int);
    fn fs_add_uint64(fs: *mut fieldset_t, name: *const libc::c_char, value: uint64_t);
    fn fs_add_null(fs: *mut fieldset_t, name: *const libc::c_char);
    fn fs_add_constchar(
        fs: *mut fieldset_t,
        name: *const libc::c_char,
        value: *const libc::c_char,
    );
    fn make_eth_header(
        ethh: *mut ether_header,
        src: *mut macaddr_t,
        dst: *mut macaddr_t,
    );
    fn make_ip_header(iph: *mut ip, _: uint8_t, _: uint16_t);
    fn make_udp_header(udp_header: *mut udphdr, dest_port: port_h_t, len: uint16_t);
    fn fprintf_ip_header(fp: *mut FILE, iph: *mut ip);
    fn fprintf_eth_header(fp: *mut FILE, ethh: *mut ether_header);
    fn icmp_helper_validate(
        ip_hdr: *const ip,
        len: uint32_t,
        min_l4_len: size_t,
        probe_pkt: *mut *mut ip,
        probe_len: *mut size_t,
    ) -> libc::c_int;
    fn fs_add_null_icmp(fs: *mut fieldset_t);
    fn fs_populate_icmp_from_iphdr(ip: *mut ip, len: size_t, fs: *mut fieldset_t);
}
pub type size_t = libc::c_ulong;
pub type __u_char = libc::c_uchar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type u_char = __u_char;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type socklen_t = __socklen_t;
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type C2RustUnnamed = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed = 256;
pub const IPPROTO_RAW: C2RustUnnamed = 255;
pub const IPPROTO_MPLS: C2RustUnnamed = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed = 136;
pub const IPPROTO_SCTP: C2RustUnnamed = 132;
pub const IPPROTO_COMP: C2RustUnnamed = 108;
pub const IPPROTO_PIM: C2RustUnnamed = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed = 94;
pub const IPPROTO_MTP: C2RustUnnamed = 92;
pub const IPPROTO_AH: C2RustUnnamed = 51;
pub const IPPROTO_ESP: C2RustUnnamed = 50;
pub const IPPROTO_GRE: C2RustUnnamed = 47;
pub const IPPROTO_RSVP: C2RustUnnamed = 46;
pub const IPPROTO_IPV6: C2RustUnnamed = 41;
pub const IPPROTO_DCCP: C2RustUnnamed = 33;
pub const IPPROTO_TP: C2RustUnnamed = 29;
pub const IPPROTO_IDP: C2RustUnnamed = 22;
pub const IPPROTO_UDP: C2RustUnnamed = 17;
pub const IPPROTO_PUP: C2RustUnnamed = 12;
pub const IPPROTO_EGP: C2RustUnnamed = 8;
pub const IPPROTO_TCP: C2RustUnnamed = 6;
pub const IPPROTO_IPIP: C2RustUnnamed = 4;
pub const IPPROTO_IGMP: C2RustUnnamed = 2;
pub const IPPROTO_ICMP: C2RustUnnamed = 1;
pub const IPPROTO_IP: C2RustUnnamed = 0;
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
#[repr(C)]
pub struct udphdr {
    pub c2rust_unnamed: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub c2rust_unnamed_0: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub source: uint16_t,
    pub dest: uint16_t,
    pub len: uint16_t,
    pub check: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub uh_sport: uint16_t,
    pub uh_dport: uint16_t,
    pub uh_ulen: uint16_t,
    pub uh_sum: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ether_header {
    pub ether_dhost: [uint8_t; 6],
    pub ether_shost: [uint8_t; 6],
    pub ether_type: uint16_t,
}
pub type aesrand_t = aesrand;
pub type ipaddr_n_t = uint32_t;
pub type port_n_t = uint16_t;
pub type port_h_t = uint16_t;
pub type macaddr_t = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field_def {
    pub name: *const libc::c_char,
    pub type_0: *const libc::c_char,
    pub desc: *const libc::c_char,
}
pub type fielddef_t = field_def;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fielddef_set {
    pub fielddefs: [fielddef_t; 128],
    pub len: libc::c_int,
}
pub type fielddefset_t = fielddef_set;
#[derive(Copy, Clone)]
#[repr(C)]
pub union field_val {
    pub ptr: *mut libc::c_void,
    pub num: uint64_t,
}
pub type field_val_t = field_val;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field {
    pub name: *const libc::c_char,
    pub type_0: libc::c_int,
    pub free_: libc::c_int,
    pub len: size_t,
    pub value: field_val_t,
}
pub type field_t = field;
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
pub type fieldset_t = fieldset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct translation {
    pub len: libc::c_int,
    pub translation: [libc::c_int; 128],
}
pub type translation_t = translation;
pub type operation = libc::c_uint;
pub const GT_EQ: operation = 7;
pub const LT_EQ: operation = 6;
pub const OR: operation = 5;
pub const AND: operation = 4;
pub const NEQ: operation = 3;
pub const EQ: operation = 2;
pub const LT: operation = 1;
pub const GT: operation = 0;
pub type node_type = libc::c_uint;
pub const INT: node_type = 3;
pub const STRING: node_type = 2;
pub const FIELD: node_type = 1;
pub const OP: node_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field_id {
    pub index: libc::c_int,
    pub fieldname: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union node_value {
    pub field: field_id,
    pub string_literal: *mut libc::c_char,
    pub int_literal: uint64_t,
    pub op: operation,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_st {
    pub left_child: *mut node_st,
    pub right_child: *mut node_st,
    pub type_0: node_type,
    pub value: node_value,
}
pub type node_t = node_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_filter {
    pub expression: *mut node_t,
}
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
pub type probe_classify_packet_cb = Option::<
    unsafe extern "C" fn(
        *const u_char,
        uint32_t,
        *mut fieldset_t,
        *mut uint32_t,
        timespec,
    ) -> (),
>;
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
pub type probe_thread_init_cb = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut macaddr_t,
        *mut macaddr_t,
        port_n_t,
        *mut *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type probe_global_init_cb = Option::<
    unsafe extern "C" fn(*mut state_conf) -> libc::c_int,
>;
pub type probe_module_t = probe_module;
pub type udp_payload_field_type = libc::c_uint;
pub const UDP_RAND_ALPHANUM: udp_payload_field_type = 12;
pub const UDP_RAND_ALPHA: udp_payload_field_type = 11;
pub const UDP_RAND_DIGIT: udp_payload_field_type = 10;
pub const UDP_RAND_BYTE: udp_payload_field_type = 9;
pub const UDP_DPORT_A: udp_payload_field_type = 8;
pub const UDP_DPORT_N: udp_payload_field_type = 7;
pub const UDP_SPORT_A: udp_payload_field_type = 6;
pub const UDP_SPORT_N: udp_payload_field_type = 5;
pub const UDP_DADDR_A: udp_payload_field_type = 4;
pub const UDP_DADDR_N: udp_payload_field_type = 3;
pub const UDP_SADDR_A: udp_payload_field_type = 2;
pub const UDP_SADDR_N: udp_payload_field_type = 1;
pub const UDP_DATA: udp_payload_field_type = 0;
pub type udp_payload_field_type_t = udp_payload_field_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct udp_payload_field_type_def {
    pub name: *const libc::c_char,
    pub desc: *const libc::c_char,
    pub max_length: size_t,
    pub ftype: udp_payload_field_type_t,
}
pub type udp_payload_field_type_def_t = udp_payload_field_type_def;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct udp_payload_field {
    pub ftype: udp_payload_field_type,
    pub length: size_t,
    pub data: *mut libc::c_char,
}
pub type udp_payload_field_t = udp_payload_field;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct udp_payload_template {
    pub fcount: libc::c_uint,
    pub fields: *mut *mut udp_payload_field,
}
pub type udp_payload_template_t = udp_payload_template;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn in_checksum(
    mut ip_pkt: *mut libc::c_ushort,
    mut len: libc::c_int,
) -> libc::c_ushort {
    let mut sum: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut nwords: libc::c_int = len / 2 as libc::c_int;
    while nwords > 0 as libc::c_int {
        let fresh0 = ip_pkt;
        ip_pkt = ip_pkt.offset(1);
        sum = sum.wrapping_add(*fresh0 as libc::c_ulong);
        nwords -= 1;
        nwords;
    }
    if len % 2 as libc::c_int == 1 as libc::c_int {
        sum = sum.wrapping_add(*(ip_pkt as *mut libc::c_uchar) as libc::c_ulong);
    }
    sum = (sum >> 16 as libc::c_int)
        .wrapping_add(sum & 0xffff as libc::c_int as libc::c_ulong);
    return !sum as libc::c_ushort;
}
#[inline]
unsafe extern "C" fn zmap_ip_checksum(mut buf: *mut libc::c_ushort) -> libc::c_ushort {
    return in_checksum(buf, ::std::mem::size_of::<ip>() as libc::c_ulong as libc::c_int);
}
#[inline]
unsafe extern "C" fn check_dst_port(
    mut port: uint16_t,
    mut num_ports_0: libc::c_int,
    mut validation: *mut uint32_t,
) -> libc::c_int {
    if port as libc::c_int > zconf.source_port_last as libc::c_int
        || (port as libc::c_int) < zconf.source_port_first as libc::c_int
    {
        return 0 as libc::c_int;
    }
    let mut to_validate: int32_t = port as libc::c_int
        - zconf.source_port_first as libc::c_int;
    let mut min: int32_t = (*validation.offset(1 as libc::c_int as isize))
        .wrapping_rem(num_ports_0 as libc::c_uint) as int32_t;
    let mut max: int32_t = (*validation.offset(1 as libc::c_int as isize))
        .wrapping_add(zconf.packet_streams as libc::c_uint)
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        .wrapping_rem(num_ports_0 as libc::c_uint) as int32_t;
    return ((max - min) % num_ports_0 >= (to_validate - min) % num_ports_0)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn get_src_port(
    mut num_ports_0: libc::c_int,
    mut probe_num: libc::c_int,
    mut validation: *mut uint32_t,
) -> uint16_t {
    return (zconf.source_port_first as libc::c_uint)
        .wrapping_add(
            (*validation.offset(1 as libc::c_int as isize))
                .wrapping_add(probe_num as libc::c_uint)
                .wrapping_rem(num_ports_0 as libc::c_uint),
        ) as uint16_t;
}
#[inline]
unsafe extern "C" fn get_udp_header(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
) -> *mut udphdr {
    if ((4 as libc::c_int * (*ip_hdr).ip_hl() as libc::c_int) as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
        > len as libc::c_ulong
    {
        return 0 as *mut udphdr;
    }
    return (ip_hdr as *mut libc::c_char)
        .offset((4 as libc::c_int * (*ip_hdr).ip_hl() as libc::c_int) as isize)
        as *mut udphdr;
}
static mut udp_fixed_payload: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
static mut udp_fixed_payload_len: size_t = 0 as libc::c_int as size_t;
static mut udp_template: *mut udp_payload_template_t = 0 as *const udp_payload_template_t
    as *mut udp_payload_template_t;
pub static mut udp_usage_error: *const libc::c_char = b"unknown UDP probe specification (expected file:/path or text:STRING or hex:01020304 or template:/path or template-fields)\0"
    as *const u8 as *const libc::c_char;
pub static mut charset_alphanum: *const libc::c_uchar = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789\0"
    as *const u8 as *const libc::c_char as *mut libc::c_uchar;
pub static mut charset_alpha: *const libc::c_uchar = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ\0"
    as *const u8 as *const libc::c_char as *mut libc::c_uchar;
pub static mut charset_digit: *const libc::c_uchar = b"0123456789\0" as *const u8
    as *const libc::c_char as *mut libc::c_uchar;
pub static mut charset_all: [libc::c_uchar; 257] = [
    0x1 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0x3 as libc::c_int as libc::c_uchar,
    0x4 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0x6 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0x9 as libc::c_int as libc::c_uchar,
    0xa as libc::c_int as libc::c_uchar,
    0xb as libc::c_int as libc::c_uchar,
    0xc as libc::c_int as libc::c_uchar,
    0xd as libc::c_int as libc::c_uchar,
    0xe as libc::c_int as libc::c_uchar,
    0xf as libc::c_int as libc::c_uchar,
    0x10 as libc::c_int as libc::c_uchar,
    0x11 as libc::c_int as libc::c_uchar,
    0x12 as libc::c_int as libc::c_uchar,
    0x13 as libc::c_int as libc::c_uchar,
    0x14 as libc::c_int as libc::c_uchar,
    0x15 as libc::c_int as libc::c_uchar,
    0x16 as libc::c_int as libc::c_uchar,
    0x17 as libc::c_int as libc::c_uchar,
    0x18 as libc::c_int as libc::c_uchar,
    0x19 as libc::c_int as libc::c_uchar,
    0x1a as libc::c_int as libc::c_uchar,
    0x1b as libc::c_int as libc::c_uchar,
    0x1c as libc::c_int as libc::c_uchar,
    0x1d as libc::c_int as libc::c_uchar,
    0x1e as libc::c_int as libc::c_uchar,
    0x1f as libc::c_int as libc::c_uchar,
    0x20 as libc::c_int as libc::c_uchar,
    0x21 as libc::c_int as libc::c_uchar,
    0x22 as libc::c_int as libc::c_uchar,
    0x23 as libc::c_int as libc::c_uchar,
    0x24 as libc::c_int as libc::c_uchar,
    0x25 as libc::c_int as libc::c_uchar,
    0x26 as libc::c_int as libc::c_uchar,
    0x27 as libc::c_int as libc::c_uchar,
    0x28 as libc::c_int as libc::c_uchar,
    0x29 as libc::c_int as libc::c_uchar,
    0x2a as libc::c_int as libc::c_uchar,
    0x2b as libc::c_int as libc::c_uchar,
    0x2c as libc::c_int as libc::c_uchar,
    0x2d as libc::c_int as libc::c_uchar,
    0x2e as libc::c_int as libc::c_uchar,
    0x2f as libc::c_int as libc::c_uchar,
    0x30 as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0x32 as libc::c_int as libc::c_uchar,
    0x33 as libc::c_int as libc::c_uchar,
    0x34 as libc::c_int as libc::c_uchar,
    0x35 as libc::c_int as libc::c_uchar,
    0x36 as libc::c_int as libc::c_uchar,
    0x37 as libc::c_int as libc::c_uchar,
    0x38 as libc::c_int as libc::c_uchar,
    0x39 as libc::c_int as libc::c_uchar,
    0x3a as libc::c_int as libc::c_uchar,
    0x3b as libc::c_int as libc::c_uchar,
    0x3c as libc::c_int as libc::c_uchar,
    0x3d as libc::c_int as libc::c_uchar,
    0x3e as libc::c_int as libc::c_uchar,
    0x3f as libc::c_int as libc::c_uchar,
    0x40 as libc::c_int as libc::c_uchar,
    0x41 as libc::c_int as libc::c_uchar,
    0x42 as libc::c_int as libc::c_uchar,
    0x43 as libc::c_int as libc::c_uchar,
    0x44 as libc::c_int as libc::c_uchar,
    0x45 as libc::c_int as libc::c_uchar,
    0x46 as libc::c_int as libc::c_uchar,
    0x47 as libc::c_int as libc::c_uchar,
    0x48 as libc::c_int as libc::c_uchar,
    0x49 as libc::c_int as libc::c_uchar,
    0x4a as libc::c_int as libc::c_uchar,
    0x4b as libc::c_int as libc::c_uchar,
    0x4c as libc::c_int as libc::c_uchar,
    0x4d as libc::c_int as libc::c_uchar,
    0x4e as libc::c_int as libc::c_uchar,
    0x4f as libc::c_int as libc::c_uchar,
    0x50 as libc::c_int as libc::c_uchar,
    0x51 as libc::c_int as libc::c_uchar,
    0x52 as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0x54 as libc::c_int as libc::c_uchar,
    0x55 as libc::c_int as libc::c_uchar,
    0x56 as libc::c_int as libc::c_uchar,
    0x57 as libc::c_int as libc::c_uchar,
    0x58 as libc::c_int as libc::c_uchar,
    0x59 as libc::c_int as libc::c_uchar,
    0x5a as libc::c_int as libc::c_uchar,
    0x5b as libc::c_int as libc::c_uchar,
    0x5c as libc::c_int as libc::c_uchar,
    0x5d as libc::c_int as libc::c_uchar,
    0x5e as libc::c_int as libc::c_uchar,
    0x5f as libc::c_int as libc::c_uchar,
    0x60 as libc::c_int as libc::c_uchar,
    0x61 as libc::c_int as libc::c_uchar,
    0x62 as libc::c_int as libc::c_uchar,
    0x63 as libc::c_int as libc::c_uchar,
    0x64 as libc::c_int as libc::c_uchar,
    0x65 as libc::c_int as libc::c_uchar,
    0x66 as libc::c_int as libc::c_uchar,
    0x67 as libc::c_int as libc::c_uchar,
    0x68 as libc::c_int as libc::c_uchar,
    0x69 as libc::c_int as libc::c_uchar,
    0x6a as libc::c_int as libc::c_uchar,
    0x6b as libc::c_int as libc::c_uchar,
    0x6c as libc::c_int as libc::c_uchar,
    0x6d as libc::c_int as libc::c_uchar,
    0x6e as libc::c_int as libc::c_uchar,
    0x6f as libc::c_int as libc::c_uchar,
    0x70 as libc::c_int as libc::c_uchar,
    0x71 as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
    0x73 as libc::c_int as libc::c_uchar,
    0x74 as libc::c_int as libc::c_uchar,
    0x75 as libc::c_int as libc::c_uchar,
    0x76 as libc::c_int as libc::c_uchar,
    0x77 as libc::c_int as libc::c_uchar,
    0x78 as libc::c_int as libc::c_uchar,
    0x79 as libc::c_int as libc::c_uchar,
    0x7a as libc::c_int as libc::c_uchar,
    0x7b as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0x7d as libc::c_int as libc::c_uchar,
    0x7e as libc::c_int as libc::c_uchar,
    0x7f as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0x81 as libc::c_int as libc::c_uchar,
    0x82 as libc::c_int as libc::c_uchar,
    0x83 as libc::c_int as libc::c_uchar,
    0x84 as libc::c_int as libc::c_uchar,
    0x85 as libc::c_int as libc::c_uchar,
    0x86 as libc::c_int as libc::c_uchar,
    0x87 as libc::c_int as libc::c_uchar,
    0x88 as libc::c_int as libc::c_uchar,
    0x89 as libc::c_int as libc::c_uchar,
    0x8a as libc::c_int as libc::c_uchar,
    0x8b as libc::c_int as libc::c_uchar,
    0x8c as libc::c_int as libc::c_uchar,
    0x8d as libc::c_int as libc::c_uchar,
    0x8e as libc::c_int as libc::c_uchar,
    0x8f as libc::c_int as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    0x91 as libc::c_int as libc::c_uchar,
    0x92 as libc::c_int as libc::c_uchar,
    0x93 as libc::c_int as libc::c_uchar,
    0x94 as libc::c_int as libc::c_uchar,
    0x95 as libc::c_int as libc::c_uchar,
    0x96 as libc::c_int as libc::c_uchar,
    0x97 as libc::c_int as libc::c_uchar,
    0x98 as libc::c_int as libc::c_uchar,
    0x99 as libc::c_int as libc::c_uchar,
    0x9a as libc::c_int as libc::c_uchar,
    0x9b as libc::c_int as libc::c_uchar,
    0x9c as libc::c_int as libc::c_uchar,
    0x9d as libc::c_int as libc::c_uchar,
    0x9e as libc::c_int as libc::c_uchar,
    0x9f as libc::c_int as libc::c_uchar,
    0xa0 as libc::c_int as libc::c_uchar,
    0xa1 as libc::c_int as libc::c_uchar,
    0xa2 as libc::c_int as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0xa4 as libc::c_int as libc::c_uchar,
    0xa5 as libc::c_int as libc::c_uchar,
    0xa6 as libc::c_int as libc::c_uchar,
    0xa7 as libc::c_int as libc::c_uchar,
    0xa8 as libc::c_int as libc::c_uchar,
    0xa9 as libc::c_int as libc::c_uchar,
    0xaa as libc::c_int as libc::c_uchar,
    0xab as libc::c_int as libc::c_uchar,
    0xac as libc::c_int as libc::c_uchar,
    0xad as libc::c_int as libc::c_uchar,
    0xae as libc::c_int as libc::c_uchar,
    0xaf as libc::c_int as libc::c_uchar,
    0xb0 as libc::c_int as libc::c_uchar,
    0xb1 as libc::c_int as libc::c_uchar,
    0xb2 as libc::c_int as libc::c_uchar,
    0xb3 as libc::c_int as libc::c_uchar,
    0xb4 as libc::c_int as libc::c_uchar,
    0xb5 as libc::c_int as libc::c_uchar,
    0xb6 as libc::c_int as libc::c_uchar,
    0xb7 as libc::c_int as libc::c_uchar,
    0xb8 as libc::c_int as libc::c_uchar,
    0xb9 as libc::c_int as libc::c_uchar,
    0xba as libc::c_int as libc::c_uchar,
    0xbb as libc::c_int as libc::c_uchar,
    0xbc as libc::c_int as libc::c_uchar,
    0xbd as libc::c_int as libc::c_uchar,
    0xbe as libc::c_int as libc::c_uchar,
    0xbf as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0xc1 as libc::c_int as libc::c_uchar,
    0xc2 as libc::c_int as libc::c_uchar,
    0xc3 as libc::c_int as libc::c_uchar,
    0xc4 as libc::c_int as libc::c_uchar,
    0xc5 as libc::c_int as libc::c_uchar,
    0xc6 as libc::c_int as libc::c_uchar,
    0xc7 as libc::c_int as libc::c_uchar,
    0xc8 as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0xca as libc::c_int as libc::c_uchar,
    0xcb as libc::c_int as libc::c_uchar,
    0xcc as libc::c_int as libc::c_uchar,
    0xcd as libc::c_int as libc::c_uchar,
    0xce as libc::c_int as libc::c_uchar,
    0xcf as libc::c_int as libc::c_uchar,
    0xd0 as libc::c_int as libc::c_uchar,
    0xd1 as libc::c_int as libc::c_uchar,
    0xd2 as libc::c_int as libc::c_uchar,
    0xd3 as libc::c_int as libc::c_uchar,
    0xd4 as libc::c_int as libc::c_uchar,
    0xd5 as libc::c_int as libc::c_uchar,
    0xd6 as libc::c_int as libc::c_uchar,
    0xd7 as libc::c_int as libc::c_uchar,
    0xd8 as libc::c_int as libc::c_uchar,
    0xd9 as libc::c_int as libc::c_uchar,
    0xda as libc::c_int as libc::c_uchar,
    0xdb as libc::c_int as libc::c_uchar,
    0xdc as libc::c_int as libc::c_uchar,
    0xdd as libc::c_int as libc::c_uchar,
    0xde as libc::c_int as libc::c_uchar,
    0xdf as libc::c_int as libc::c_uchar,
    0xe0 as libc::c_int as libc::c_uchar,
    0xe1 as libc::c_int as libc::c_uchar,
    0xe2 as libc::c_int as libc::c_uchar,
    0xe3 as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0xe5 as libc::c_int as libc::c_uchar,
    0xe6 as libc::c_int as libc::c_uchar,
    0xe7 as libc::c_int as libc::c_uchar,
    0xe8 as libc::c_int as libc::c_uchar,
    0xe9 as libc::c_int as libc::c_uchar,
    0xea as libc::c_int as libc::c_uchar,
    0xeb as libc::c_int as libc::c_uchar,
    0xec as libc::c_int as libc::c_uchar,
    0xed as libc::c_int as libc::c_uchar,
    0xee as libc::c_int as libc::c_uchar,
    0xef as libc::c_int as libc::c_uchar,
    0xf0 as libc::c_int as libc::c_uchar,
    0xf1 as libc::c_int as libc::c_uchar,
    0xf2 as libc::c_int as libc::c_uchar,
    0xf3 as libc::c_int as libc::c_uchar,
    0xf4 as libc::c_int as libc::c_uchar,
    0xf5 as libc::c_int as libc::c_uchar,
    0xf6 as libc::c_int as libc::c_uchar,
    0xf7 as libc::c_int as libc::c_uchar,
    0xf8 as libc::c_int as libc::c_uchar,
    0xf9 as libc::c_int as libc::c_uchar,
    0xfa as libc::c_int as libc::c_uchar,
    0xfb as libc::c_int as libc::c_uchar,
    0xfc as libc::c_int as libc::c_uchar,
    0xfd as libc::c_int as libc::c_uchar,
    0xfe as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0,
];
static mut num_ports: libc::c_int = 0;
static mut udp_num_template_field_types: uint32_t = 12 as libc::c_int as uint32_t;
static mut udp_payload_template_fields: [udp_payload_field_type_def_t; 12] = [
    {
        let mut init = udp_payload_field_type_def {
            name: b"SADDR_N\0" as *const u8 as *const libc::c_char,
            desc: b"Source IP address in network byte order\0" as *const u8
                as *const libc::c_char,
            max_length: 4 as libc::c_int as size_t,
            ftype: UDP_SADDR_N,
        };
        init
    },
    {
        let mut init = udp_payload_field_type_def {
            name: b"SADDR\0" as *const u8 as *const libc::c_char,
            desc: b"Source IP address in dotted-quad format\0" as *const u8
                as *const libc::c_char,
            max_length: 15 as libc::c_int as size_t,
            ftype: UDP_SADDR_A,
        };
        init
    },
    {
        let mut init = udp_payload_field_type_def {
            name: b"DADDR_N\0" as *const u8 as *const libc::c_char,
            desc: b"Destination IP address in network byte order\0" as *const u8
                as *const libc::c_char,
            max_length: 4 as libc::c_int as size_t,
            ftype: UDP_DADDR_N,
        };
        init
    },
    {
        let mut init = udp_payload_field_type_def {
            name: b"DADDR\0" as *const u8 as *const libc::c_char,
            desc: b"Destination IP address in dotted-quad format\0" as *const u8
                as *const libc::c_char,
            max_length: 15 as libc::c_int as size_t,
            ftype: UDP_DADDR_A,
        };
        init
    },
    {
        let mut init = udp_payload_field_type_def {
            name: b"SPORT_N\0" as *const u8 as *const libc::c_char,
            desc: b"UDP source port in netowrk byte order\0" as *const u8
                as *const libc::c_char,
            max_length: 2 as libc::c_int as size_t,
            ftype: UDP_SPORT_N,
        };
        init
    },
    {
        let mut init = udp_payload_field_type_def {
            name: b"SPORT\0" as *const u8 as *const libc::c_char,
            desc: b"UDP source port in ascii format\0" as *const u8
                as *const libc::c_char,
            max_length: 5 as libc::c_int as size_t,
            ftype: UDP_SPORT_A,
        };
        init
    },
    {
        let mut init = udp_payload_field_type_def {
            name: b"DPORT_N\0" as *const u8 as *const libc::c_char,
            desc: b"UDP destination port in network byte order\0" as *const u8
                as *const libc::c_char,
            max_length: 2 as libc::c_int as size_t,
            ftype: UDP_DPORT_N,
        };
        init
    },
    {
        let mut init = udp_payload_field_type_def {
            name: b"DPORT\0" as *const u8 as *const libc::c_char,
            desc: b"UDP destination port in ascii format\0" as *const u8
                as *const libc::c_char,
            max_length: 5 as libc::c_int as size_t,
            ftype: UDP_DPORT_A,
        };
        init
    },
    {
        let mut init = udp_payload_field_type_def {
            name: b"RAND_BYTE\0" as *const u8 as *const libc::c_char,
            desc: b"Random bytes from 0-255\0" as *const u8 as *const libc::c_char,
            max_length: 0 as libc::c_int as size_t,
            ftype: UDP_RAND_BYTE,
        };
        init
    },
    {
        let mut init = udp_payload_field_type_def {
            name: b"RAND_DIGIT\0" as *const u8 as *const libc::c_char,
            desc: b"Random digits from 0-9\0" as *const u8 as *const libc::c_char,
            max_length: 0 as libc::c_int as size_t,
            ftype: UDP_RAND_DIGIT,
        };
        init
    },
    {
        let mut init = udp_payload_field_type_def {
            name: b"RAND_ALPHA\0" as *const u8 as *const libc::c_char,
            desc: b"Random mixed-case letters (a-z)\0" as *const u8
                as *const libc::c_char,
            max_length: 0 as libc::c_int as size_t,
            ftype: UDP_RAND_ALPHA,
        };
        init
    },
    {
        let mut init = udp_payload_field_type_def {
            name: b"RAND_ALPHANUM\0" as *const u8 as *const libc::c_char,
            desc: b"Random mixed-case letters (a-z) and numbers\0" as *const u8
                as *const libc::c_char,
            max_length: 0 as libc::c_int as size_t,
            ftype: UDP_RAND_ALPHANUM,
        };
        init
    },
];
pub unsafe extern "C" fn udp_set_num_ports(mut x: libc::c_int) {
    num_ports = x;
}
pub unsafe extern "C" fn udp_global_initialize(
    mut conf: *mut state_conf,
) -> libc::c_int {
    let mut udp_template_max_len: uint32_t = 0 as libc::c_int as uint32_t;
    num_ports = (*conf).source_port_last as libc::c_int
        - (*conf).source_port_first as libc::c_int + 1 as libc::c_int;
    if ((*conf).probe_args).is_null() {
        log_error(
            b"udp\0" as *const u8 as *const libc::c_char,
            b"%s\0" as *const u8 as *const libc::c_char,
            b"--probe-args are required, run --probe-module=udp --help for a longer description of the arguments\0"
                as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    let mut args: *const libc::c_char = (*conf).probe_args;
    if strcmp(args, b"template-fields\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        lock_file(stderr);
        fprintf(
            stderr,
            b"%s\0" as *const u8 as *const libc::c_char,
            b"List of allowed UDP template fields (name: description)\n\n\0" as *const u8
                as *const libc::c_char,
        );
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < udp_num_template_field_types {
            fprintf(
                stderr,
                b"%s: %s\n\0" as *const u8 as *const libc::c_char,
                udp_payload_template_fields[i as usize].name,
                udp_payload_template_fields[i as usize].desc,
            );
            i = i.wrapping_add(1);
            i;
        }
        fprintf(
            stderr,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
        fflush(stderr);
        unlock_file(stderr);
        exit(0 as libc::c_int);
    }
    let mut c: *const libc::c_char = strchr(args, ':' as i32);
    if c.is_null() {
        log_fatal(b"udp\0" as *const u8 as *const libc::c_char, udp_usage_error);
    }
    let mut arg_name_len: size_t = c.offset_from(args) as libc::c_long as size_t;
    c = c.offset(1);
    c;
    if strncmp(args, b"text\0" as *const u8 as *const libc::c_char, arg_name_len)
        == 0 as libc::c_int
    {
        udp_fixed_payload = strdup(c) as *mut uint8_t;
        udp_fixed_payload_len = strlen(c);
    } else if strncmp(args, b"file\0" as *const u8 as *const libc::c_char, arg_name_len)
        == 0 as libc::c_int
    {
        udp_fixed_payload = xmalloc(1472 as libc::c_int as size_t) as *mut uint8_t;
        let mut f: *mut FILE = fopen(c, b"rb\0" as *const u8 as *const libc::c_char);
        if f.is_null() {
            log_fatal(
                b"udp\0" as *const u8 as *const libc::c_char,
                b"could not open UDP data file '%s'\n\0" as *const u8
                    as *const libc::c_char,
                c,
            );
        }
        udp_fixed_payload_len = fread(
            udp_fixed_payload as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            1472 as libc::c_int as libc::c_ulong,
            f,
        );
        fclose(f);
    } else if strncmp(
        args,
        b"template\0" as *const u8 as *const libc::c_char,
        arg_name_len,
    ) == 0 as libc::c_int
    {
        let mut in_0: [uint8_t; 1472] = [0; 1472];
        let mut f_0: *mut FILE = fopen(c, b"rb\0" as *const u8 as *const libc::c_char);
        if f_0.is_null() {
            log_fatal(
                b"udp\0" as *const u8 as *const libc::c_char,
                b"could not open UDP data file '%s'\n\0" as *const u8
                    as *const libc::c_char,
                c,
            );
        }
        let mut in_len: size_t = fread(
            in_0.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            1472 as libc::c_int as libc::c_ulong,
            f_0,
        );
        fclose(f_0);
        udp_template = udp_template_load(
            in_0.as_mut_ptr(),
            in_len as uint32_t,
            &mut udp_template_max_len,
        );
        module_udp
            .make_packet = Some(
            udp_make_templated_packet
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut size_t,
                    ipaddr_n_t,
                    ipaddr_n_t,
                    uint8_t,
                    *mut uint32_t,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> libc::c_int,
        );
    } else if strncmp(args, b"hex\0" as *const u8 as *const libc::c_char, arg_name_len)
        == 0 as libc::c_int
    {
        udp_fixed_payload_len = (strlen(c))
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
        udp_fixed_payload = xmalloc(udp_fixed_payload_len) as *mut uint8_t;
        let mut n: libc::c_uint = 0;
        let mut i_0: size_t = 0 as libc::c_int as size_t;
        while i_0 < udp_fixed_payload_len {
            if sscanf(
                c.offset(i_0.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize),
                b"%2x\0" as *const u8 as *const libc::c_char,
                &mut n as *mut libc::c_uint,
            ) != 1 as libc::c_int
            {
                log_fatal(
                    b"udp\0" as *const u8 as *const libc::c_char,
                    b"non-hex character: '%c'\0" as *const u8 as *const libc::c_char,
                    *c
                        .offset(
                            i_0.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int,
                );
            }
            *udp_fixed_payload
                .offset(
                    i_0 as isize,
                ) = (n & 0xff as libc::c_int as libc::c_uint) as uint8_t;
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
    } else {
        log_fatal(b"udp\0" as *const u8 as *const libc::c_char, udp_usage_error);
    }
    if udp_fixed_payload_len > 1472 as libc::c_int as libc::c_ulong {
        log_warn(
            b"udp\0" as *const u8 as *const libc::c_char,
            b"warning: reducing fixed UDP payload to %d bytes (from %d) to fit on the wire\n\0"
                as *const u8 as *const libc::c_char,
            1472 as libc::c_int,
            udp_fixed_payload_len,
        );
        udp_fixed_payload_len = 1472 as libc::c_int as size_t;
    }
    let mut header_len: size_t = (::std::mem::size_of::<ether_header>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong);
    if udp_fixed_payload_len > 0 as libc::c_int as libc::c_ulong {
        module_udp.max_packet_length = header_len.wrapping_add(udp_fixed_payload_len);
    } else if udp_template_max_len > 0 as libc::c_int as libc::c_uint {
        module_udp
            .max_packet_length = header_len
            .wrapping_add(udp_template_max_len as libc::c_ulong);
    }
    if module_udp.max_packet_length != 0 {} else {
        __assert_fail(
            b"module_udp.max_packet_length\0" as *const u8 as *const libc::c_char,
            b"probe_modules/module_udp.c\0" as *const u8 as *const libc::c_char,
            224 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"int udp_global_initialize(struct state_conf *)\0"))
                .as_ptr(),
        );
    }
    'c_8444: {
        if module_udp.max_packet_length != 0 {} else {
            __assert_fail(
                b"module_udp.max_packet_length\0" as *const u8 as *const libc::c_char,
                b"probe_modules/module_udp.c\0" as *const u8 as *const libc::c_char,
                224 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"int udp_global_initialize(struct state_conf *)\0"))
                    .as_ptr(),
            );
        }
    };
    if module_udp.max_packet_length <= 4096 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"module_udp.max_packet_length <= MAX_PACKET_SIZE\0" as *const u8
                as *const libc::c_char,
            b"probe_modules/module_udp.c\0" as *const u8 as *const libc::c_char,
            225 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"int udp_global_initialize(struct state_conf *)\0"))
                .as_ptr(),
        );
    }
    'c_7510: {
        if module_udp.max_packet_length <= 4096 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"module_udp.max_packet_length <= MAX_PACKET_SIZE\0" as *const u8
                    as *const libc::c_char,
                b"probe_modules/module_udp.c\0" as *const u8 as *const libc::c_char,
                225 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"int udp_global_initialize(struct state_conf *)\0"))
                    .as_ptr(),
            );
        }
    };
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn udp_global_cleanup(
    mut zconf_0: *mut state_conf,
    mut zsend: *mut state_send,
    mut zrecv: *mut state_recv,
) -> libc::c_int {
    if !udp_fixed_payload.is_null() {
        free(udp_fixed_payload as *mut libc::c_void);
        udp_fixed_payload = 0 as *mut uint8_t;
    }
    if !udp_template.is_null() {
        udp_template_free(udp_template);
        udp_template = 0 as *mut udp_payload_template_t;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn udp_init_perthread(
    mut buf: *mut libc::c_void,
    mut src: *mut macaddr_t,
    mut gw: *mut macaddr_t,
    mut dst_port: port_h_t,
    mut arg_ptr: *mut *mut libc::c_void,
) -> libc::c_int {
    memset(buf, 0 as libc::c_int, 4096 as libc::c_int as libc::c_ulong);
    let mut eth_header: *mut ether_header = buf as *mut ether_header;
    make_eth_header(eth_header, src, gw);
    let mut ip_header: *mut ip = &mut *eth_header.offset(1 as libc::c_int as isize)
        as *mut ether_header as *mut ip;
    let mut ip_len: uint16_t = __bswap_16(
        (::std::mem::size_of::<ip>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
            .wrapping_add(udp_fixed_payload_len) as __uint16_t,
    );
    make_ip_header(ip_header, IPPROTO_UDP as libc::c_int as uint8_t, ip_len);
    let mut udp_header: *mut udphdr = &mut *ip_header.offset(1 as libc::c_int as isize)
        as *mut ip as *mut udphdr;
    let mut udp_len: uint16_t = (::std::mem::size_of::<udphdr>() as libc::c_ulong)
        .wrapping_add(udp_fixed_payload_len) as uint16_t;
    make_udp_header(udp_header, zconf.target_port, udp_len);
    if !udp_fixed_payload.is_null() {
        let mut payload: *mut libc::c_void = &mut *udp_header
            .offset(1 as libc::c_int as isize) as *mut udphdr as *mut libc::c_void;
        memcpy(payload, udp_fixed_payload as *const libc::c_void, udp_fixed_payload_len);
    }
    let mut seed: uint32_t = aesrand_getword(zconf.aes) as uint32_t;
    let mut aes: *mut aesrand_t = aesrand_init_from_seed(seed as uint64_t);
    *arg_ptr = aes as *mut libc::c_void;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn udp_make_packet(
    mut buf: *mut libc::c_void,
    mut buf_len: *mut size_t,
    mut src_ip: ipaddr_n_t,
    mut dst_ip: ipaddr_n_t,
    mut ttl: uint8_t,
    mut validation: *mut uint32_t,
    mut probe_num: libc::c_int,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut eth_header: *mut ether_header = buf as *mut ether_header;
    let mut ip_header: *mut ip = &mut *eth_header.offset(1 as libc::c_int as isize)
        as *mut ether_header as *mut ip;
    let mut udp_header: *mut udphdr = &mut *ip_header.offset(1 as libc::c_int as isize)
        as *mut ip as *mut udphdr;
    let mut headers_len: size_t = (::std::mem::size_of::<ether_header>()
        as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong);
    (*ip_header).ip_src.s_addr = src_ip;
    (*ip_header).ip_dst.s_addr = dst_ip;
    (*ip_header).ip_ttl = ttl;
    (*udp_header)
        .c2rust_unnamed
        .c2rust_unnamed
        .uh_sport = __bswap_16(get_src_port(num_ports, probe_num, validation));
    (*ip_header).ip_sum = 0 as libc::c_int as libc::c_ushort;
    (*ip_header).ip_sum = zmap_ip_checksum(ip_header as *mut libc::c_ushort);
    *buf_len = headers_len.wrapping_add(udp_fixed_payload_len);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn udp_make_templated_packet(
    mut buf: *mut libc::c_void,
    mut buf_len: *mut size_t,
    mut src_ip: ipaddr_n_t,
    mut dst_ip: ipaddr_n_t,
    mut ttl: uint8_t,
    mut validation: *mut uint32_t,
    mut probe_num: libc::c_int,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut eth_header: *mut ether_header = buf as *mut ether_header;
    let mut ip_header: *mut ip = &mut *eth_header.offset(1 as libc::c_int as isize)
        as *mut ether_header as *mut ip;
    let mut udp_header: *mut udphdr = &mut *ip_header.offset(1 as libc::c_int as isize)
        as *mut ip as *mut udphdr;
    let mut headers_len: size_t = (::std::mem::size_of::<ether_header>()
        as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong);
    (*ip_header).ip_src.s_addr = src_ip;
    (*ip_header).ip_dst.s_addr = dst_ip;
    (*ip_header).ip_ttl = ttl;
    (*udp_header)
        .c2rust_unnamed
        .c2rust_unnamed
        .uh_sport = __bswap_16(get_src_port(num_ports, probe_num, validation));
    let mut payload: *mut libc::c_char = &mut *udp_header
        .offset(1 as libc::c_int as isize) as *mut udphdr as *mut libc::c_char;
    memset(
        payload as *mut libc::c_void,
        0 as libc::c_int,
        1472 as libc::c_int as libc::c_ulong,
    );
    let mut aes: *mut aesrand_t = arg as *mut aesrand_t;
    let mut payload_len: libc::c_int = udp_template_build(
        udp_template,
        payload,
        1472 as libc::c_int as libc::c_uint,
        ip_header,
        udp_header,
        aes,
    );
    if payload_len <= 0 as libc::c_int {
        log_fatal(
            b"udp\0" as *const u8 as *const libc::c_char,
            b"UDP payload template generated an empty payload\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*ip_header)
        .ip_len = __bswap_16(
        (::std::mem::size_of::<ip>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
            .wrapping_add(payload_len as libc::c_ulong) as __uint16_t,
    );
    (*udp_header)
        .c2rust_unnamed
        .c2rust_unnamed
        .uh_ulen = __bswap_16(
        (::std::mem::size_of::<udphdr>() as libc::c_ulong)
            .wrapping_add(payload_len as libc::c_ulong) as __uint16_t,
    );
    (*ip_header).ip_sum = 0 as libc::c_int as libc::c_ushort;
    (*ip_header).ip_sum = zmap_ip_checksum(ip_header as *mut libc::c_ushort);
    *buf_len = headers_len.wrapping_add(payload_len as libc::c_ulong);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn udp_print_packet(
    mut fp: *mut FILE,
    mut packet: *mut libc::c_void,
) {
    let mut ethh: *mut ether_header = packet as *mut ether_header;
    let mut iph: *mut ip = &mut *ethh.offset(1 as libc::c_int as isize)
        as *mut ether_header as *mut ip;
    let mut udph: *mut udphdr = &mut *iph.offset(1 as libc::c_int as isize) as *mut ip
        as *mut udphdr;
    fprintf(
        fp,
        b"udp { source: %u | dest: %u | checksum: %#04X }\n\0" as *const u8
            as *const libc::c_char,
        __bswap_16((*udph).c2rust_unnamed.c2rust_unnamed.uh_sport) as libc::c_int,
        __bswap_16((*udph).c2rust_unnamed.c2rust_unnamed.uh_dport) as libc::c_int,
        __bswap_16((*udph).c2rust_unnamed.c2rust_unnamed.uh_sum) as libc::c_int,
    );
    fprintf_ip_header(fp, iph);
    fprintf_eth_header(fp, ethh);
    fprintf(
        fp,
        b"------------------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
}
pub unsafe extern "C" fn udp_process_packet(
    mut packet: *const u_char,
    mut len: uint32_t,
    mut fs: *mut fieldset_t,
    mut validation: *mut uint32_t,
    mut ts: timespec,
) {
    let mut ip_hdr: *mut ip = &*packet
        .offset(::std::mem::size_of::<ether_header>() as libc::c_ulong as isize)
        as *const u_char as *mut ip;
    if (*ip_hdr).ip_p as libc::c_int == IPPROTO_UDP as libc::c_int {
        let mut udp: *mut udphdr = get_udp_header(ip_hdr, len);
        fs_add_constchar(
            fs,
            b"classification\0" as *const u8 as *const libc::c_char,
            b"udp\0" as *const u8 as *const libc::c_char,
        );
        fs_add_bool(
            fs,
            b"success\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
        fs_add_uint64(
            fs,
            b"sport\0" as *const u8 as *const libc::c_char,
            __bswap_16((*udp).c2rust_unnamed.c2rust_unnamed.uh_sport) as uint64_t,
        );
        fs_add_uint64(
            fs,
            b"dport\0" as *const u8 as *const libc::c_char,
            __bswap_16((*udp).c2rust_unnamed.c2rust_unnamed.uh_dport) as uint64_t,
        );
        fs_add_uint64(
            fs,
            b"udp_pkt_size\0" as *const u8 as *const libc::c_char,
            __bswap_16((*udp).c2rust_unnamed.c2rust_unnamed.uh_ulen) as uint64_t,
        );
        let mut data_len: uint16_t = __bswap_16(
            (*udp).c2rust_unnamed.c2rust_unnamed.uh_ulen,
        );
        if data_len as libc::c_ulong > ::std::mem::size_of::<udphdr>() as libc::c_ulong {
            let mut overhead: uint32_t = (::std::mem::size_of::<udphdr>()
                as libc::c_ulong)
                .wrapping_add(
                    ((*ip_hdr).ip_hl() as libc::c_int * 4 as libc::c_int)
                        as libc::c_ulong,
                ) as uint32_t;
            let mut max_rlen: uint32_t = len.wrapping_sub(overhead);
            let mut max_ilen: uint32_t = (__bswap_16((*ip_hdr).ip_len) as libc::c_uint)
                .wrapping_sub(overhead);
            if data_len as libc::c_uint > max_rlen {
                data_len = max_rlen as uint16_t;
            }
            if data_len as libc::c_uint > max_ilen {
                data_len = max_ilen as uint16_t;
            }
            fs_add_binary(
                fs,
                b"data\0" as *const u8 as *const libc::c_char,
                data_len as size_t,
                &mut *udp.offset(1 as libc::c_int as isize) as *mut udphdr
                    as *mut libc::c_void,
                0 as libc::c_int,
            );
        } else {
            fs_add_null(fs, b"data\0" as *const u8 as *const libc::c_char);
        }
        fs_add_null_icmp(fs);
    } else if (*ip_hdr).ip_p as libc::c_int == IPPROTO_ICMP as libc::c_int {
        fs_add_constchar(
            fs,
            b"classification\0" as *const u8 as *const libc::c_char,
            b"icmp\0" as *const u8 as *const libc::c_char,
        );
        fs_add_bool(
            fs,
            b"success\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        fs_add_null(fs, b"sport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"dport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"udp_pkt_size\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"data\0" as *const u8 as *const libc::c_char);
        fs_populate_icmp_from_iphdr(ip_hdr, len as size_t, fs);
    } else {
        fs_add_constchar(
            fs,
            b"classification\0" as *const u8 as *const libc::c_char,
            b"other\0" as *const u8 as *const libc::c_char,
        );
        fs_add_bool(
            fs,
            b"success\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        fs_add_null(fs, b"sport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"dport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"udp_pkt_size\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"data\0" as *const u8 as *const libc::c_char);
        fs_add_null_icmp(fs);
    };
}
pub unsafe extern "C" fn udp_validate_packet(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
    mut src_ip: *mut uint32_t,
    mut validation: *mut uint32_t,
) -> libc::c_int {
    return udp_do_validate_packet(
        ip_hdr,
        len,
        src_ip,
        validation,
        num_ports,
        -(1 as libc::c_int),
    );
}
pub unsafe extern "C" fn udp_do_validate_packet(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
    mut src_ip: *mut uint32_t,
    mut validation: *mut uint32_t,
    mut num_ports_0: libc::c_int,
    mut expected_port: libc::c_int,
) -> libc::c_int {
    if (*ip_hdr).ip_p as libc::c_int == IPPROTO_UDP as libc::c_int {
        let mut udp: *mut udphdr = get_udp_header(ip_hdr, len);
        if udp.is_null() {
            return 0 as libc::c_int;
        }
        let mut dport: uint16_t = __bswap_16(
            (*udp).c2rust_unnamed.c2rust_unnamed.uh_dport,
        );
        if check_dst_port(dport, num_ports_0, validation) == 0 {
            return 0 as libc::c_int;
        }
        if blocklist_is_allowed(*src_ip) == 0 {
            return 0 as libc::c_int;
        }
        if expected_port != -(1 as libc::c_int) {
            let mut ep: uint16_t = expected_port as uint16_t;
            let mut sport: uint16_t = __bswap_16(
                (*udp).c2rust_unnamed.c2rust_unnamed.uh_sport,
            );
            if sport as libc::c_int != ep as libc::c_int {
                return 0 as libc::c_int;
            }
        }
    } else if (*ip_hdr).ip_p as libc::c_int == IPPROTO_ICMP as libc::c_int {
        let mut ip_inner: *mut ip = 0 as *mut ip;
        let mut ip_inner_len: size_t = 0;
        icmp_helper_validate(
            ip_hdr,
            len,
            ::std::mem::size_of::<udphdr>() as libc::c_ulong,
            &mut ip_inner,
            &mut ip_inner_len,
        ) == 0 as libc::c_int;
        let mut udp_0: *mut udphdr = get_udp_header(ip_inner, ip_inner_len as uint32_t);
        let mut dport_0: uint16_t = __bswap_16(
            (*udp_0).c2rust_unnamed.c2rust_unnamed.uh_dport,
        );
        let mut sport_0: uint16_t = __bswap_16(
            (*udp_0).c2rust_unnamed.c2rust_unnamed.uh_sport,
        );
        if dport_0 as libc::c_int != zconf.target_port as libc::c_int {
            return 0 as libc::c_int;
        }
        if check_dst_port(sport_0, num_ports_0, validation) == 0 {
            return 0 as libc::c_int;
        }
    } else {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn udp_template_add_field(
    mut t: *mut udp_payload_template_t,
    mut ftype: udp_payload_field_type_t,
    mut length: libc::c_uint,
    mut data: *mut libc::c_char,
) {
    let mut c: *mut udp_payload_field_t = 0 as *mut udp_payload_field_t;
    (*t).fcount = ((*t).fcount).wrapping_add(1);
    (*t).fcount;
    (*t)
        .fields = xrealloc(
        (*t).fields as *mut libc::c_void,
        (::std::mem::size_of::<udp_payload_field_t>() as libc::c_ulong)
            .wrapping_mul((*t).fcount as libc::c_ulong),
    ) as *mut *mut udp_payload_field;
    let ref mut fresh1 = *((*t).fields)
        .offset(((*t).fcount).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
    *fresh1 = xmalloc(::std::mem::size_of::<udp_payload_field_t>() as libc::c_ulong)
        as *mut udp_payload_field;
    c = *((*t).fields)
        .offset(((*t).fcount).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
    if !c.is_null() {} else {
        __assert_fail(
            b"c\0" as *const u8 as *const libc::c_char,
            b"probe_modules/module_udp.c\0" as *const u8 as *const libc::c_char,
            486 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 102],
                &[libc::c_char; 102],
            >(
                b"void udp_template_add_field(udp_payload_template_t *, udp_payload_field_type_t, unsigned int, char *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8840: {
        if !c.is_null() {} else {
            __assert_fail(
                b"c\0" as *const u8 as *const libc::c_char,
                b"probe_modules/module_udp.c\0" as *const u8 as *const libc::c_char,
                486 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 102],
                    &[libc::c_char; 102],
                >(
                    b"void udp_template_add_field(udp_payload_template_t *, udp_payload_field_type_t, unsigned int, char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    (*c).ftype = ftype;
    (*c).length = length as size_t;
    (*c).data = data;
}
pub unsafe extern "C" fn udp_template_free(mut t: *mut udp_payload_template_t) {
    let mut x: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while x < (*t).fcount {
        if !((**((*t).fields).offset(x as isize)).data).is_null() {
            free((**((*t).fields).offset(x as isize)).data as *mut libc::c_void);
            let ref mut fresh2 = (**((*t).fields).offset(x as isize)).data;
            *fresh2 = 0 as *mut libc::c_char;
        }
        free(*((*t).fields).offset(x as isize) as *mut libc::c_void);
        let ref mut fresh3 = *((*t).fields).offset(x as isize);
        *fresh3 = 0 as *mut udp_payload_field;
        x = x.wrapping_add(1);
        x;
    }
    free((*t).fields as *mut libc::c_void);
    (*t).fields = 0 as *mut *mut udp_payload_field;
    (*t).fcount = 0 as libc::c_int as libc::c_uint;
    free(t as *mut libc::c_void);
}
pub unsafe extern "C" fn udp_random_bytes(
    mut dst: *mut libc::c_char,
    mut len: libc::c_int,
    mut charset: *const libc::c_uchar,
    mut charset_len: libc::c_int,
    mut aes: *mut aesrand_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        let fresh4 = dst;
        dst = dst.offset(1);
        *fresh4 = *charset
            .offset(
                (aesrand_getword(aes) & 0xffffffff as libc::c_uint as libc::c_ulong)
                    .wrapping_rem(charset_len as libc::c_ulong) as isize,
            ) as libc::c_char;
        i += 1;
        i;
    }
    return i;
}
pub unsafe extern "C" fn udp_template_build(
    mut t: *mut udp_payload_template_t,
    mut out: *mut libc::c_char,
    mut len: libc::c_uint,
    mut ip_hdr: *mut ip,
    mut udp_hdr: *mut udphdr,
    mut aes: *mut aesrand_t,
) -> libc::c_int {
    let mut c: *mut udp_payload_field_t = 0 as *mut udp_payload_field_t;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut max: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: [libc::c_char; 256] = [0; 256];
    let mut full: libc::c_int = 0 as libc::c_int;
    let mut x: libc::c_uint = 0;
    let mut y: libc::c_uint = 0;
    let mut u32: *mut uint32_t = 0 as *mut uint32_t;
    let mut u16: *mut uint16_t = 0 as *mut uint16_t;
    max = out.offset(len as isize);
    p = out;
    x = 0 as libc::c_int as libc::c_uint;
    while x < (*t).fcount {
        c = *((*t).fields).offset(x as isize);
        if p.offset((*c).length as isize) >= max {
            full = 1 as libc::c_int;
            return 0 as libc::c_int;
        }
        match (*c).ftype as libc::c_uint {
            0 => {
                if !((*c).data).is_null() && (*c).length != 0 {
                    memcpy(
                        p as *mut libc::c_void,
                        (*c).data as *const libc::c_void,
                        (*c).length,
                    );
                    p = p.offset((*c).length as isize);
                }
            }
            10 => {
                p = p
                    .offset(
                        udp_random_bytes(
                            p,
                            (*c).length as libc::c_int,
                            charset_digit,
                            10 as libc::c_int,
                            aes,
                        ) as isize,
                    );
            }
            11 => {
                p = p
                    .offset(
                        udp_random_bytes(
                            p,
                            (*c).length as libc::c_int,
                            charset_alpha,
                            52 as libc::c_int,
                            aes,
                        ) as isize,
                    );
            }
            12 => {
                p = p
                    .offset(
                        udp_random_bytes(
                            p,
                            (*c).length as libc::c_int,
                            charset_alphanum,
                            62 as libc::c_int,
                            aes,
                        ) as isize,
                    );
            }
            9 => {
                p = p
                    .offset(
                        udp_random_bytes(
                            p,
                            (*c).length as libc::c_int,
                            charset_all.as_ptr(),
                            256 as libc::c_int,
                            aes,
                        ) as isize,
                    );
            }
            2 => {
                if p.offset(15 as libc::c_int as isize) >= max {
                    full = 1 as libc::c_int;
                } else {
                    inet_ntop(
                        2 as libc::c_int,
                        &mut (*ip_hdr).ip_src as *mut in_addr as *mut libc::c_char
                            as *const libc::c_void,
                        tmp.as_mut_ptr(),
                        (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as socklen_t,
                    );
                    memcpy(
                        p as *mut libc::c_void,
                        tmp.as_mut_ptr() as *const libc::c_void,
                        strlen(tmp.as_mut_ptr()),
                    );
                    p = p.offset(strlen(tmp.as_mut_ptr()) as isize);
                }
            }
            4 => {
                if p.offset(15 as libc::c_int as isize) >= max {
                    full = 1 as libc::c_int;
                } else {
                    inet_ntop(
                        2 as libc::c_int,
                        &mut (*ip_hdr).ip_dst as *mut in_addr as *mut libc::c_char
                            as *const libc::c_void,
                        tmp.as_mut_ptr(),
                        (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as socklen_t,
                    );
                    memcpy(
                        p as *mut libc::c_void,
                        tmp.as_mut_ptr() as *const libc::c_void,
                        strlen(tmp.as_mut_ptr()),
                    );
                    p = p.offset(strlen(tmp.as_mut_ptr()) as isize);
                }
            }
            1 => {
                if p.offset(4 as libc::c_int as isize) >= max {
                    full = 1 as libc::c_int;
                } else {
                    u32 = p as *mut uint32_t;
                    *u32 = (*ip_hdr).ip_src.s_addr;
                    p = p.offset(4 as libc::c_int as isize);
                }
            }
            3 => {
                if p.offset(4 as libc::c_int as isize) >= max {
                    full = 1 as libc::c_int;
                } else {
                    u32 = p as *mut uint32_t;
                    *u32 = (*ip_hdr).ip_dst.s_addr;
                    p = p.offset(4 as libc::c_int as isize);
                }
            }
            5 => {
                if p.offset(2 as libc::c_int as isize) >= max {
                    full = 1 as libc::c_int;
                } else {
                    u16 = p as *mut uint16_t;
                    *u16 = (*udp_hdr).c2rust_unnamed.c2rust_unnamed.uh_sport;
                    p = p.offset(2 as libc::c_int as isize);
                }
            }
            7 => {
                if p.offset(2 as libc::c_int as isize) >= max {
                    full = 1 as libc::c_int;
                } else {
                    u16 = p as *mut uint16_t;
                    *u16 = (*udp_hdr).c2rust_unnamed.c2rust_unnamed.uh_sport;
                    p = p.offset(2 as libc::c_int as isize);
                }
            }
            6 => {
                if p.offset(5 as libc::c_int as isize) >= max {
                    full = 1 as libc::c_int;
                } else {
                    y = snprintf(
                        tmp.as_mut_ptr(),
                        6 as libc::c_int as libc::c_ulong,
                        b"%d\0" as *const u8 as *const libc::c_char,
                        __bswap_16((*udp_hdr).c2rust_unnamed.c2rust_unnamed.uh_sport)
                            as libc::c_int,
                    ) as libc::c_uint;
                    memcpy(
                        p as *mut libc::c_void,
                        tmp.as_mut_ptr() as *const libc::c_void,
                        y as libc::c_ulong,
                    );
                    p = p.offset(y as isize);
                }
            }
            8 => {
                if p.offset(5 as libc::c_int as isize) >= max {
                    full = 1 as libc::c_int;
                } else {
                    y = snprintf(
                        tmp.as_mut_ptr(),
                        6 as libc::c_int as libc::c_ulong,
                        b"%d\0" as *const u8 as *const libc::c_char,
                        __bswap_16((*udp_hdr).c2rust_unnamed.c2rust_unnamed.uh_sport)
                            as libc::c_int,
                    ) as libc::c_uint;
                    memcpy(
                        p as *mut libc::c_void,
                        tmp.as_mut_ptr() as *const libc::c_void,
                        y as libc::c_ulong,
                    );
                    p = p.offset(y as isize);
                }
            }
            _ => {}
        }
        if full == 1 as libc::c_int {
            return 0 as libc::c_int;
        }
        x = x.wrapping_add(1);
        x;
    }
    return p.offset_from(out) as libc::c_long as libc::c_int;
}
static mut fcount: size_t = 0;
pub unsafe extern "C" fn udp_template_field_lookup(
    mut vname: *const libc::c_char,
    mut c: *mut udp_payload_field_t,
) -> libc::c_int {
    let mut vname_len: size_t = strlen(vname);
    let mut type_name_len: size_t = vname_len;
    let mut param: *const libc::c_char = strstr(
        vname,
        b"=\0" as *const u8 as *const libc::c_char,
    );
    if !param.is_null() {
        type_name_len = param.offset_from(vname) as libc::c_long as size_t;
        param = param.offset(1);
        param;
    }
    let mut olen: libc::c_long = 0 as libc::c_int as libc::c_long;
    if !param.is_null() && *param == 0 {
        log_fatal(
            b"udp\0" as *const u8 as *const libc::c_char,
            b"invalid template: field spec %s is invalid (missing length)\0" as *const u8
                as *const libc::c_char,
            vname,
        );
    }
    if !param.is_null() {
        let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
        *__errno_location() = 0 as libc::c_int;
        olen = strtol(param, &mut end, 10 as libc::c_int);
        if *__errno_location() != 0 {
            log_fatal(
                b"udp\0" as *const u8 as *const libc::c_char,
                b"invalid template: unable to read length from %s: %s\0" as *const u8
                    as *const libc::c_char,
                vname,
                strerror(*__errno_location()),
            );
        }
        if end.is_null() || end != vname.offset(vname_len as isize) as *mut libc::c_char
        {
            log_fatal(
                b"udp\0" as *const u8 as *const libc::c_char,
                b"invalid template: unable to read length from %s\0" as *const u8
                    as *const libc::c_char,
                vname,
            );
        }
        if olen < 0 as libc::c_int as libc::c_long
            || olen > 1472 as libc::c_int as libc::c_long
        {
            log_fatal(
                b"udp\0" as *const u8 as *const libc::c_char,
                b"invalid template: field size %d is larger than the max (%d)\0"
                    as *const u8 as *const libc::c_char,
                olen,
                1472 as libc::c_int,
            );
        }
    }
    let mut f: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (f as libc::c_ulong) < fcount {
        let mut ftype: *const udp_payload_field_type_def_t = &mut *udp_payload_template_fields
            .as_mut_ptr()
            .offset(f as isize) as *mut udp_payload_field_type_def_t;
        if strncmp(vname, (*ftype).name, type_name_len) == 0 as libc::c_int
            && strlen((*ftype).name) == type_name_len
        {
            (*c).ftype = (*ftype).ftype;
            (*c)
                .length = if (*ftype).max_length != 0 {
                (*ftype).max_length
            } else {
                olen as size_t
            };
            (*c).data = 0 as *mut libc::c_char;
            return 1 as libc::c_int;
        }
        f = f.wrapping_add(1);
        f;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn udp_template_load(
    mut buf: *mut uint8_t,
    mut buf_len: uint32_t,
    mut max_pkt_len: *mut uint32_t,
) -> *mut udp_payload_template_t {
    let mut t: *mut udp_payload_template_t = xmalloc(
        ::std::mem::size_of::<udp_payload_template_t>() as libc::c_ulong,
    ) as *mut udp_payload_template_t;
    let mut _max_pkt_len: uint32_t = 0 as libc::c_int as uint32_t;
    let mut dollar: *mut uint8_t = 0 as *mut uint8_t;
    let mut lbrack: *mut uint8_t = 0 as *mut uint8_t;
    let mut s: *mut uint8_t = buf;
    let mut p: *mut uint8_t = buf;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tlen: libc::c_uint = 0;
    let mut c: udp_payload_field_t = udp_payload_field_t {
        ftype: UDP_DATA,
        length: 0,
        data: 0 as *mut libc::c_char,
    };
    (*t).fcount = 0 as libc::c_int as libc::c_uint;
    (*t).fields = 0 as *mut *mut udp_payload_field;
    while p < buf.offset(buf_len as isize) {
        match *p as libc::c_int {
            36 => {
                if !dollar.is_null() && lbrack.is_null() || dollar.is_null() {
                    dollar = p;
                }
                p = p.offset(1);
                p;
                continue;
            }
            123 => {
                if !dollar.is_null() && lbrack.is_null() {
                    lbrack = p;
                }
                p = p.offset(1);
                p;
                continue;
            }
            125 => {
                if !(!dollar.is_null() && !lbrack.is_null()) {
                    p = p.offset(1);
                    p;
                    continue;
                } else {
                    tlen = dollar.offset_from(s) as libc::c_long as libc::c_uint;
                    if tlen > 0 as libc::c_int as libc::c_uint {
                        tmp = xmalloc(tlen as size_t) as *mut libc::c_char;
                        memcpy(
                            tmp as *mut libc::c_void,
                            s as *const libc::c_void,
                            tlen as libc::c_ulong,
                        );
                        udp_template_add_field(t, UDP_DATA, tlen, tmp);
                        _max_pkt_len = (_max_pkt_len as libc::c_uint).wrapping_add(tlen)
                            as uint32_t as uint32_t;
                    }
                    tmp = xcalloc(
                        1 as libc::c_int as size_t,
                        p.offset_from(lbrack) as libc::c_long as size_t,
                    ) as *mut libc::c_char;
                    memcpy(
                        tmp as *mut libc::c_void,
                        lbrack.offset(1 as libc::c_int as isize) as *const libc::c_void,
                        (p.offset_from(lbrack) as libc::c_long
                            - 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                    );
                    if udp_template_field_lookup(tmp, &mut c) != 0 {
                        udp_template_add_field(
                            t,
                            c.ftype,
                            c.length as libc::c_uint,
                            c.data,
                        );
                        _max_pkt_len = (_max_pkt_len as libc::c_ulong)
                            .wrapping_add(c.length) as uint32_t as uint32_t;
                        s = p.offset(1 as libc::c_int as isize);
                    } else {
                        s = dollar;
                    }
                    free(tmp as *mut libc::c_void);
                }
            }
            _ => {
                if !dollar.is_null() && !lbrack.is_null() {
                    p = p.offset(1);
                    p;
                    continue;
                }
            }
        }
        dollar = 0 as *mut uint8_t;
        lbrack = 0 as *mut uint8_t;
        p = p.offset(1);
        p;
    }
    if s < p {
        tlen = p.offset_from(s) as libc::c_long as libc::c_uint;
        tmp = xmalloc(tlen as size_t) as *mut libc::c_char;
        memcpy(
            tmp as *mut libc::c_void,
            s as *const libc::c_void,
            tlen as libc::c_ulong,
        );
        udp_template_add_field(t, UDP_DATA, tlen, tmp);
        _max_pkt_len = (_max_pkt_len as libc::c_uint).wrapping_add(tlen) as uint32_t
            as uint32_t;
    }
    *max_pkt_len = _max_pkt_len;
    return t;
}
static mut fields: [fielddef_t; 10] = [
    {
        let mut init = field_def {
            name: b"classification\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"packet classification\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"success\0" as *const u8 as *const libc::c_char,
            type_0: b"bool\0" as *const u8 as *const libc::c_char,
            desc: b"is response considered success\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"sport\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"UDP source port\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dport\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"UDP destination port\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"udp_pkt_size\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"UDP packet length\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"data\0" as *const u8 as *const libc::c_char,
            type_0: b"binary\0" as *const u8 as *const libc::c_char,
            desc: b"UDP payload\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_responder\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"Source IP of ICMP_UNREACH messages\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_type\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp message type\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_code\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp message sub type code\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_unreach_str\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"for icmp_unreach responses, the string version of icmp_code (e.g. network-unreach)\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
];
pub static mut module_udp: probe_module_t = probe_module_t {
    name: 0 as *const libc::c_char,
    max_packet_length: 0,
    pcap_filter: 0 as *const libc::c_char,
    pcap_snaplen: 0,
    port_args: 0,
    global_initialize: None,
    thread_initialize: None,
    make_packet: None,
    print_packet: None,
    validate_packet: None,
    process_packet: None,
    close: None,
    output_type: 0,
    fields: 0 as *const fielddef_t as *mut fielddef_t,
    numfields: 0,
    helptext: 0 as *const libc::c_char,
};
unsafe extern "C" fn run_static_initializers() {
    fcount = (::std::mem::size_of::<[udp_payload_field_type_def_t; 12]>()
        as libc::c_ulong)
        .wrapping_div(
            ::std::mem::size_of::<udp_payload_field_type_def_t>() as libc::c_ulong,
        );
    module_udp = {
        let mut init = probe_module {
            name: b"udp\0" as *const u8 as *const libc::c_char,
            max_packet_length: 0 as libc::c_int as size_t,
            pcap_filter: b"udp || icmp\0" as *const u8 as *const libc::c_char,
            pcap_snaplen: 1500 as libc::c_int as size_t,
            port_args: 1 as libc::c_int as uint8_t,
            global_initialize: Some(
                udp_global_initialize
                    as unsafe extern "C" fn(*mut state_conf) -> libc::c_int,
            ),
            thread_initialize: Some(
                udp_init_perthread
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut macaddr_t,
                        *mut macaddr_t,
                        port_h_t,
                        *mut *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            make_packet: Some(
                udp_make_packet
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut size_t,
                        ipaddr_n_t,
                        ipaddr_n_t,
                        uint8_t,
                        *mut uint32_t,
                        libc::c_int,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            print_packet: Some(
                udp_print_packet
                    as unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> (),
            ),
            validate_packet: Some(
                udp_validate_packet
                    as unsafe extern "C" fn(
                        *const ip,
                        uint32_t,
                        *mut uint32_t,
                        *mut uint32_t,
                    ) -> libc::c_int,
            ),
            process_packet: Some(
                udp_process_packet
                    as unsafe extern "C" fn(
                        *const u_char,
                        uint32_t,
                        *mut fieldset_t,
                        *mut uint32_t,
                        timespec,
                    ) -> (),
            ),
            close: Some(
                udp_global_cleanup
                    as unsafe extern "C" fn(
                        *mut state_conf,
                        *mut state_send,
                        *mut state_recv,
                    ) -> libc::c_int,
            ),
            output_type: 0,
            fields: fields.as_mut_ptr(),
            numfields: (::std::mem::size_of::<[fielddef_t; 10]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<fielddef_t>() as libc::c_ulong)
                as libc::c_int,
            helptext: b"Probe module that sends UDP packets to hosts. Packets can optionally be templated based on destination host. Specify packet file with --probe-args=file:/path_to_packet_file and templates with template:/path_to_template_file.\0"
                as *const u8 as *const libc::c_char,
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
