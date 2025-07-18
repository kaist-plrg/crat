use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type aesrand;
    pub type output_module;
    fn free(__ptr: *mut libc::c_void);
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    fn fs_new_fieldset(_: *mut fielddefset_t) -> *mut fieldset_t;
    fn fs_new_repeated_fieldset() -> *mut fieldset_t;
    fn fs_add_null(fs: *mut fieldset_t, name: *const libc::c_char);
    fn fs_add_uint64(fs: *mut fieldset_t, name: *const libc::c_char, value: uint64_t);
    fn fs_add_bool(fs: *mut fieldset_t, name: *const libc::c_char, value: libc::c_int);
    fn fs_add_string(
        fs: *mut fieldset_t,
        name: *const libc::c_char,
        value: *mut libc::c_char,
        free_: libc::c_int,
    );
    fn fs_add_unsafe_string(
        fs: *mut fieldset_t,
        name: *const libc::c_char,
        value: *mut libc::c_char,
        free_: libc::c_int,
    );
    fn fs_add_constchar(
        fs: *mut fieldset_t,
        name: *const libc::c_char,
        value: *const libc::c_char,
    );
    fn fs_add_binary(
        fs: *mut fieldset_t,
        name: *const libc::c_char,
        len: size_t,
        value: *mut libc::c_void,
        free_: libc::c_int,
    );
    fn fs_add_fieldset(
        fs: *mut fieldset_t,
        name: *const libc::c_char,
        child: *mut fieldset_t,
    );
    fn fs_add_repeated(
        fs: *mut fieldset_t,
        name: *const libc::c_char,
        child: *mut fieldset_t,
    );
    static mut zconf: state_conf;
    fn make_eth_header(
        ethh: *mut ether_header,
        src: *mut macaddr_t,
        dst: *mut macaddr_t,
    );
    fn make_ip_header(iph: *mut ip, _: uint8_t, _: uint16_t);
    fn make_udp_header(udp_header: *mut udphdr, dest_port: port_h_t, len: uint16_t);
    fn fprintf_ip_header(fp: *mut FILE, iph: *mut ip);
    fn fprintf_eth_header(fp: *mut FILE, ethh: *mut ether_header);
    fn fs_add_null_icmp(fs: *mut fieldset_t);
    fn fs_populate_icmp_from_iphdr(ip: *mut ip, len: size_t, fs: *mut fieldset_t);
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
    fn udp_do_validate_packet(
        ip_hdr: *const ip,
        len: uint32_t,
        src_ip: *mut uint32_t,
        validation: *mut uint32_t,
        num_ports_0: libc::c_int,
        expected_port: libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __u_char = libc::c_uchar;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type u_char = __u_char;
pub type int8_t = __int8_t;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C, packed)]
pub struct dns_header {
    pub id: uint16_t,
    #[bitfield(name = "rd", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "tc", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "aa", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "opcode", ty = "libc::c_uint", bits = "3..=6")]
    #[bitfield(name = "qr", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "rcode", ty = "libc::c_uint", bits = "8..=11")]
    #[bitfield(name = "cd", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "ad", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "z", ty = "libc::c_uint", bits = "14..=14")]
    #[bitfield(name = "ra", ty = "libc::c_uint", bits = "15..=15")]
    pub rd_tc_aa_opcode_qr_rcode_cd_ad_z_ra: [u8; 2],
    pub qdcount: uint16_t,
    pub ancount: uint16_t,
    pub nscount: uint16_t,
    pub arcount: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct dns_question_tail {
    pub qtype: uint16_t,
    pub qclass: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct dns_answer_tail {
    pub type_0: uint16_t,
    pub class: uint16_t,
    pub ttl: uint32_t,
    pub rdlength: uint16_t,
    pub rdata: [libc::c_char; 0],
}
pub type dns_qtype = libc::c_uint;
pub const DNS_QTYPE_ALL: dns_qtype = 255;
pub const DNS_QTYPE_RRSIG: dns_qtype = 46;
pub const DNS_QTYPE_AAAA: dns_qtype = 28;
pub const DNS_QTYPE_TXT: dns_qtype = 16;
pub const DNS_QTYPE_MX: dns_qtype = 15;
pub const DNS_QTYPE_PTR: dns_qtype = 12;
pub const DNS_QTYPE_SOA: dns_qtype = 6;
pub const DNS_QTYPE_CNAME: dns_qtype = 5;
pub const DNS_QTYPE_NS: dns_qtype = 2;
pub const DNS_QTYPE_A: dns_qtype = 1;
pub type C2RustUnnamed = libc::c_uint;
pub const DNS_RCODE_QRYREFUSED: C2RustUnnamed = 5;
pub const DNS_RCODE_QTYPENOTIMPL: C2RustUnnamed = 4;
pub const DNS_RCODE_NXDOMAIN: C2RustUnnamed = 3;
pub const DNS_RCODE_SRVFAILURE: C2RustUnnamed = 2;
pub const DNS_RCODE_FORMATERR: C2RustUnnamed = 1;
pub const DNS_RCODE_NOERR: C2RustUnnamed = 0;
pub type sa_family_t = libc::c_ushort;
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_0 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_0 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_0 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_0 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_0 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_0 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_0 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_0 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_0 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_0 = 92;
pub const IPPROTO_AH: C2RustUnnamed_0 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_0 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_0 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_0 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_0 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_0 = 33;
pub const IPPROTO_TP: C2RustUnnamed_0 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_0 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_0 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_0 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_0 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_0 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_0 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_0 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_0 = 1;
pub const IPPROTO_IP: C2RustUnnamed_0 = 0;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
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
#[repr(C)]
pub struct udphdr {
    pub c2rust_unnamed: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub c2rust_unnamed_0: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub source: uint16_t,
    pub dest: uint16_t,
    pub len: uint16_t,
    pub check: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
pub type bool_0 = uint8_t;
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
static mut num_ports: libc::c_int = 0;
pub static mut default_domain: [libc::c_char; 15] = unsafe {
    *::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"www.google.com\0")
};
pub static mut default_qtype: uint16_t = DNS_QTYPE_A as libc::c_int as uint16_t;
static mut dns_packets: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut dns_packet_lens: *mut uint16_t = 0 as *const uint16_t as *mut uint16_t;
static mut qname_lens: *mut uint16_t = 0 as *const uint16_t as *mut uint16_t;
static mut qnames: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut qtypes: *mut uint16_t = 0 as *const uint16_t as *mut uint16_t;
static mut num_questions: libc::c_int = 0 as libc::c_int;
pub static mut qtype_strs: [*const libc::c_char; 10] = [
    b"A\0" as *const u8 as *const libc::c_char,
    b"NS\0" as *const u8 as *const libc::c_char,
    b"CNAME\0" as *const u8 as *const libc::c_char,
    b"SOA\0" as *const u8 as *const libc::c_char,
    b"PTR\0" as *const u8 as *const libc::c_char,
    b"MX\0" as *const u8 as *const libc::c_char,
    b"TXT\0" as *const u8 as *const libc::c_char,
    b"AAAA\0" as *const u8 as *const libc::c_char,
    b"RRSIG\0" as *const u8 as *const libc::c_char,
    b"ALL\0" as *const u8 as *const libc::c_char,
];
pub static mut qtype_strs_len: libc::c_int = 10 as libc::c_int;
pub static mut qtype_strid_to_qtype: [dns_qtype; 10] = [
    DNS_QTYPE_A,
    DNS_QTYPE_NS,
    DNS_QTYPE_CNAME,
    DNS_QTYPE_SOA,
    DNS_QTYPE_PTR,
    DNS_QTYPE_MX,
    DNS_QTYPE_TXT,
    DNS_QTYPE_AAAA,
    DNS_QTYPE_RRSIG,
    DNS_QTYPE_ALL,
];
pub static mut qtype_qtype_to_strid: [int8_t; 256] = [
    -(1 as libc::c_int) as int8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
pub unsafe extern "C" fn setup_qtype_str_map() {
    qtype_qtype_to_strid[DNS_QTYPE_A as libc::c_int
        as usize] = 0 as libc::c_int as int8_t;
    qtype_qtype_to_strid[DNS_QTYPE_NS as libc::c_int
        as usize] = 1 as libc::c_int as int8_t;
    qtype_qtype_to_strid[DNS_QTYPE_CNAME as libc::c_int
        as usize] = 2 as libc::c_int as int8_t;
    qtype_qtype_to_strid[DNS_QTYPE_SOA as libc::c_int
        as usize] = 3 as libc::c_int as int8_t;
    qtype_qtype_to_strid[DNS_QTYPE_PTR as libc::c_int
        as usize] = 4 as libc::c_int as int8_t;
    qtype_qtype_to_strid[DNS_QTYPE_MX as libc::c_int
        as usize] = 5 as libc::c_int as int8_t;
    qtype_qtype_to_strid[DNS_QTYPE_TXT as libc::c_int
        as usize] = 6 as libc::c_int as int8_t;
    qtype_qtype_to_strid[DNS_QTYPE_AAAA as libc::c_int
        as usize] = 7 as libc::c_int as int8_t;
    qtype_qtype_to_strid[DNS_QTYPE_RRSIG as libc::c_int
        as usize] = 8 as libc::c_int as int8_t;
    qtype_qtype_to_strid[DNS_QTYPE_ALL as libc::c_int
        as usize] = 9 as libc::c_int as int8_t;
}
unsafe extern "C" fn qtype_str_to_code(mut str: *const libc::c_char) -> uint16_t {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < qtype_strs_len {
        if strcmp(qtype_strs[i as usize], str) == 0 as libc::c_int {
            return qtype_strid_to_qtype[i as usize] as uint16_t;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int as uint16_t;
}
unsafe extern "C" fn domain_to_qname(
    mut qname_handle: *mut *mut libc::c_char,
    mut domain: *const libc::c_char,
) -> uint16_t {
    let mut len: uint16_t = (strlen(domain))
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as uint16_t;
    let mut qname: *mut libc::c_char = xmalloc(len as size_t) as *mut libc::c_char;
    *qname.offset(0 as libc::c_int as isize) = '.' as i32 as libc::c_char;
    strcpy(qname.offset(1 as libc::c_int as isize), domain);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len as libc::c_int {
        if *qname.offset(i as isize) as libc::c_int == '.' as i32 {
            let mut j: libc::c_int = 0;
            j = i + 1 as libc::c_int;
            while j < len as libc::c_int - 1 as libc::c_int {
                if *qname.offset(j as isize) as libc::c_int == '.' as i32 {
                    break;
                }
                j += 1;
                j;
            }
            *qname.offset(i as isize) = (j - i - 1 as libc::c_int) as libc::c_char;
        }
        i += 1;
        i;
    }
    *qname_handle = qname;
    if *(*qname_handle).offset((len as libc::c_int - 1 as libc::c_int) as isize)
        as libc::c_int == '\0' as i32
    {} else {
        __assert_fail(
            b"(*qname_handle)[len - 1] == '\\0'\0" as *const u8 as *const libc::c_char,
            b"probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
            152 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"uint16_t domain_to_qname(char **, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_10285: {
        if *(*qname_handle).offset((len as libc::c_int - 1 as libc::c_int) as isize)
            as libc::c_int == '\0' as i32
        {} else {
            __assert_fail(
                b"(*qname_handle)[len - 1] == '\\0'\0" as *const u8
                    as *const libc::c_char,
                b"probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
                152 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"uint16_t domain_to_qname(char **, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    return len;
}
unsafe extern "C" fn build_global_dns_packets(
    mut domains: *mut *mut libc::c_char,
    mut num_domains: libc::c_int,
    mut max_len: *mut size_t,
) -> libc::c_int {
    let mut _max_len: size_t = 0 as libc::c_int as size_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < num_domains {
        *qname_lens
            .offset(
                i as isize,
            ) = domain_to_qname(
            &mut *qnames.offset(i as isize),
            *domains.offset(i as isize),
        );
        if *domains.offset(i as isize) != default_domain.as_ptr() as *mut libc::c_char {
            free(*domains.offset(i as isize) as *mut libc::c_void);
        }
        let mut len: uint16_t = (::std::mem::size_of::<dns_header>() as libc::c_ulong)
            .wrapping_add(*qname_lens.offset(i as isize) as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<dns_question_tail>() as libc::c_ulong)
            as uint16_t;
        *dns_packet_lens.offset(i as isize) = len;
        if len as libc::c_ulong > _max_len {
            _max_len = len as size_t;
        }
        if *dns_packet_lens.offset(i as isize) as libc::c_int > 512 as libc::c_int {
            log_fatal(
                b"dns\0" as *const u8 as *const libc::c_char,
                b"DNS packet bigger (%d) than our limit (%d)\0" as *const u8
                    as *const libc::c_char,
                *dns_packet_lens.offset(i as isize) as libc::c_int,
                512 as libc::c_int,
            );
        }
        let ref mut fresh1 = *dns_packets.offset(i as isize);
        *fresh1 = xmalloc(*dns_packet_lens.offset(i as isize) as size_t)
            as *mut libc::c_char;
        let mut dns_header_p: *mut dns_header = *dns_packets.offset(i as isize)
            as *mut dns_header;
        let mut qname_p: *mut libc::c_char = (*dns_packets.offset(i as isize))
            .offset(::std::mem::size_of::<dns_header>() as libc::c_ulong as isize);
        let mut tail_p: *mut dns_question_tail = (*dns_packets.offset(i as isize))
            .offset(::std::mem::size_of::<dns_header>() as libc::c_ulong as isize)
            .offset(*qname_lens.offset(i as isize) as libc::c_int as isize)
            as *mut dns_question_tail;
        (*dns_header_p).set_rd(1 as libc::c_int as libc::c_uint);
        (*dns_header_p).qdcount = __bswap_16(1 as libc::c_int as __uint16_t);
        memcpy(
            qname_p as *mut libc::c_void,
            *qnames.offset(i as isize) as *const libc::c_void,
            *qname_lens.offset(i as isize) as libc::c_ulong,
        );
        (*tail_p).qtype = __bswap_16(*qtypes.offset(i as isize));
        (*tail_p).qclass = __bswap_16(0x1 as libc::c_int as __uint16_t);
        i += 1;
        i;
    }
    *max_len = _max_len;
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_name_helper(
    mut data: *const libc::c_char,
    mut data_len: uint16_t,
    mut payload: *const libc::c_char,
    mut payload_len: uint16_t,
    mut name: *mut libc::c_char,
    mut name_len: uint16_t,
    mut recursion_level: uint16_t,
) -> uint16_t {
    if data_len as libc::c_int == 0 as libc::c_int
        || name_len as libc::c_int == 0 as libc::c_int
        || payload_len as libc::c_int == 0 as libc::c_int
    {
        return 0 as libc::c_int as uint16_t;
    }
    if recursion_level as libc::c_int > 10 as libc::c_int {
        return 0 as libc::c_int as uint16_t;
    }
    let mut bytes_consumed: uint16_t = 0 as libc::c_int as uint16_t;
    while data_len as libc::c_int > 0 as libc::c_int {
        let mut byte: uint8_t = *data.offset(0 as libc::c_int as isize) as uint8_t;
        if byte as libc::c_int >= 0xc0 as libc::c_int {
            if (data_len as libc::c_int) < 2 as libc::c_int {
                return 0 as libc::c_int as uint16_t;
            }
            let mut offset: uint16_t = ((byte as libc::c_int & 0x3 as libc::c_int)
                << 8 as libc::c_int
                | *data.offset(1 as libc::c_int as isize) as uint8_t as libc::c_int)
                as uint16_t;
            if offset as libc::c_int >= payload_len as libc::c_int {
                return 0 as libc::c_int as uint16_t;
            }
            if recursion_level as libc::c_int > 0 as libc::c_int
                || bytes_consumed as libc::c_int > 0 as libc::c_int
            {
                if (name_len as libc::c_int) < 1 as libc::c_int {
                    log_warn(
                        b"dns\0" as *const u8 as *const libc::c_char,
                        b"Exceeded static name field allocation.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return 0 as libc::c_int as uint16_t;
                }
                *name.offset(0 as libc::c_int as isize) = '.' as i32 as libc::c_char;
                name = name.offset(1);
                name;
                name_len = name_len.wrapping_sub(1);
                name_len;
            }
            let mut rec_bytes_consumed: uint16_t = get_name_helper(
                payload.offset(offset as libc::c_int as isize),
                (payload_len as libc::c_int - offset as libc::c_int) as uint16_t,
                payload,
                payload_len,
                name,
                name_len,
                (recursion_level as libc::c_int + 1 as libc::c_int) as uint16_t,
            );
            if rec_bytes_consumed as libc::c_int == 0 as libc::c_int {
                return 0 as libc::c_int as uint16_t
            } else {
                bytes_consumed = (bytes_consumed as libc::c_int + 2 as libc::c_int)
                    as uint16_t;
                return bytes_consumed;
            }
        } else if byte as libc::c_int == '\0' as i32 {
            bytes_consumed = (bytes_consumed as libc::c_int + 1 as libc::c_int)
                as uint16_t;
            return bytes_consumed;
        } else {
            data = data.offset(1);
            data;
            data_len = data_len.wrapping_sub(1);
            data_len;
            if byte as libc::c_int + 1 as libc::c_int > data_len as libc::c_int {
                return 0 as libc::c_int as uint16_t;
            }
            if bytes_consumed as libc::c_int > 0 as libc::c_int {
                if (name_len as libc::c_int) < 1 as libc::c_int {
                    log_warn(
                        b"dns\0" as *const u8 as *const libc::c_char,
                        b"Exceeded static name field allocation.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return 0 as libc::c_int as uint16_t;
                }
                *name.offset(0 as libc::c_int as isize) = '.' as i32 as libc::c_char;
                name = name.offset(1);
                name;
                name_len = name_len.wrapping_sub(1);
                name_len;
            }
            bytes_consumed = bytes_consumed.wrapping_add(1);
            bytes_consumed;
            if byte as libc::c_int > name_len as libc::c_int {
                log_warn(
                    b"dns\0" as *const u8 as *const libc::c_char,
                    b"Exceeded static name field allocation.\0" as *const u8
                        as *const libc::c_char,
                );
                return 0 as libc::c_int as uint16_t;
            }
            if data_len as libc::c_int > 0 as libc::c_int {} else {
                __assert_fail(
                    b"data_len > 0\0" as *const u8 as *const libc::c_char,
                    b"probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
                    335 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 101],
                        &[libc::c_char; 101],
                    >(
                        b"uint16_t get_name_helper(const char *, uint16_t, const char *, uint16_t, char *, uint16_t, uint16_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_7644: {
                if data_len as libc::c_int > 0 as libc::c_int {} else {
                    __assert_fail(
                        b"data_len > 0\0" as *const u8 as *const libc::c_char,
                        b"probe_modules/module_dns.c\0" as *const u8
                            as *const libc::c_char,
                        335 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 101],
                            &[libc::c_char; 101],
                        >(
                            b"uint16_t get_name_helper(const char *, uint16_t, const char *, uint16_t, char *, uint16_t, uint16_t)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            memcpy(
                name as *mut libc::c_void,
                data as *const libc::c_void,
                byte as libc::c_ulong,
            );
            name = name.offset(byte as libc::c_int as isize);
            name_len = (name_len as libc::c_int - byte as libc::c_int) as uint16_t;
            data_len = (data_len as libc::c_int - byte as libc::c_int) as uint16_t;
            data = data.offset(byte as libc::c_int as isize);
            bytes_consumed = (bytes_consumed as libc::c_int + byte as libc::c_int)
                as uint16_t;
            if data_len as libc::c_int > 0 as libc::c_int {} else {
                __assert_fail(
                    b"data_len > 0\0" as *const u8 as *const libc::c_char,
                    b"probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
                    343 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 101],
                        &[libc::c_char; 101],
                    >(
                        b"uint16_t get_name_helper(const char *, uint16_t, const char *, uint16_t, char *, uint16_t, uint16_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_7559: {
                if data_len as libc::c_int > 0 as libc::c_int {} else {
                    __assert_fail(
                        b"data_len > 0\0" as *const u8 as *const libc::c_char,
                        b"probe_modules/module_dns.c\0" as *const u8
                            as *const libc::c_char,
                        343 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 101],
                            &[libc::c_char; 101],
                        >(
                            b"uint16_t get_name_helper(const char *, uint16_t, const char *, uint16_t, char *, uint16_t, uint16_t)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
        }
    }
    __assert_fail(
        b"0\0" as *const u8 as *const libc::c_char,
        b"probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
        352 as libc::c_int as libc::c_uint,
        (*::std::mem::transmute::<
            &[u8; 101],
            &[libc::c_char; 101],
        >(
            b"uint16_t get_name_helper(const char *, uint16_t, const char *, uint16_t, char *, uint16_t, uint16_t)\0",
        ))
            .as_ptr(),
    );
    'c_7501: {
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
            352 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 101],
                &[libc::c_char; 101],
            >(
                b"uint16_t get_name_helper(const char *, uint16_t, const char *, uint16_t, char *, uint16_t, uint16_t)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int as uint16_t;
}
unsafe extern "C" fn get_name(
    mut data: *const libc::c_char,
    mut data_len: uint16_t,
    mut payload: *const libc::c_char,
    mut payload_len: uint16_t,
    mut bytes_consumed: *mut uint16_t,
) -> *mut libc::c_char {
    let mut name: *mut libc::c_char = xmalloc(512 as libc::c_int as size_t)
        as *mut libc::c_char;
    *bytes_consumed = get_name_helper(
        data,
        data_len,
        payload,
        payload_len,
        name,
        (512 as libc::c_int - 1 as libc::c_int) as uint16_t,
        0 as libc::c_int as uint16_t,
    );
    if *bytes_consumed as libc::c_int == 0 as libc::c_int {
        free(name as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    if *name.offset((512 as libc::c_int - 1 as libc::c_int) as isize) as libc::c_int
        == '\0' as i32
    {} else {
        __assert_fail(
            b"name[MAX_NAME_LENGTH - 1] == '\\0'\0" as *const u8 as *const libc::c_char,
            b"probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
            370 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"char *get_name(const char *, uint16_t, const char *, uint16_t, uint16_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_7387: {
        if *name.offset((512 as libc::c_int - 1 as libc::c_int) as isize) as libc::c_int
            == '\0' as i32
        {} else {
            __assert_fail(
                b"name[MAX_NAME_LENGTH - 1] == '\\0'\0" as *const u8
                    as *const libc::c_char,
                b"probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
                370 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"char *get_name(const char *, uint16_t, const char *, uint16_t, uint16_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return name;
}
unsafe extern "C" fn process_response_question(
    mut data: *mut *mut libc::c_char,
    mut data_len: *mut uint16_t,
    mut payload: *const libc::c_char,
    mut payload_len: uint16_t,
    mut list: *mut fieldset_t,
) -> bool_0 {
    let mut bytes_consumed: uint16_t = 0 as libc::c_int as uint16_t;
    let mut question_name: *mut libc::c_char = get_name(
        *data,
        *data_len,
        payload,
        payload_len,
        &mut bytes_consumed,
    );
    if question_name.is_null() {
        return 1 as libc::c_int as bool_0;
    }
    if bytes_consumed as libc::c_int > 0 as libc::c_int {} else {
        __assert_fail(
            b"bytes_consumed > 0\0" as *const u8 as *const libc::c_char,
            b"probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"bool process_response_question(char **, uint16_t *, const char *, uint16_t, fieldset_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8842: {
        if bytes_consumed as libc::c_int > 0 as libc::c_int {} else {
            __assert_fail(
                b"bytes_consumed > 0\0" as *const u8 as *const libc::c_char,
                b"probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
                393 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 90],
                    &[libc::c_char; 90],
                >(
                    b"bool process_response_question(char **, uint16_t *, const char *, uint16_t, fieldset_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (bytes_consumed as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<dns_question_tail>() as libc::c_ulong)
        > *data_len as libc::c_ulong
    {
        free(question_name as *mut libc::c_void);
        return 1 as libc::c_int as bool_0;
    }
    let mut tail: *mut dns_question_tail = (*data)
        .offset(bytes_consumed as libc::c_int as isize) as *mut dns_question_tail;
    let mut qtype: uint16_t = __bswap_16((*tail).qtype);
    let mut qclass: uint16_t = __bswap_16((*tail).qclass);
    let mut qfs: *mut fieldset_t = fs_new_fieldset(0 as *mut fielddefset_t);
    fs_add_unsafe_string(
        qfs,
        b"name\0" as *const u8 as *const libc::c_char,
        question_name,
        1 as libc::c_int,
    );
    fs_add_uint64(
        qfs,
        b"qtype\0" as *const u8 as *const libc::c_char,
        qtype as uint64_t,
    );
    if qtype as libc::c_int > 255 as libc::c_int
        || qtype_qtype_to_strid[qtype as usize] as libc::c_int == -(1 as libc::c_int)
    {
        fs_add_string(
            qfs,
            b"qtype_str\0" as *const u8 as *const libc::c_char,
            b"BAD QTYPE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
    } else {
        fs_add_string(
            qfs,
            b"qtype_str\0" as *const u8 as *const libc::c_char,
            qtype_strs[qtype_qtype_to_strid[qtype as usize] as usize]
                as *mut libc::c_char,
            0 as libc::c_int,
        );
    }
    fs_add_uint64(
        qfs,
        b"qclass\0" as *const u8 as *const libc::c_char,
        qclass as uint64_t,
    );
    fs_add_fieldset(list, 0 as *const libc::c_char, qfs);
    *data = (*data)
        .offset(bytes_consumed as libc::c_int as isize)
        .offset(::std::mem::size_of::<dns_question_tail>() as libc::c_ulong as isize);
    *data_len = ((*data_len as libc::c_int - bytes_consumed as libc::c_int)
        as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<dns_question_tail>() as libc::c_ulong)
        as uint16_t;
    return 0 as libc::c_int as bool_0;
}
unsafe extern "C" fn process_response_answer(
    mut data: *mut *mut libc::c_char,
    mut data_len: *mut uint16_t,
    mut payload: *const libc::c_char,
    mut payload_len: uint16_t,
    mut list: *mut fieldset_t,
) -> bool_0 {
    let mut bytes_consumed: uint16_t = 0 as libc::c_int as uint16_t;
    let mut answer_name: *mut libc::c_char = get_name(
        *data,
        *data_len,
        payload,
        payload_len,
        &mut bytes_consumed,
    );
    if answer_name.is_null() {
        return 1 as libc::c_int as bool_0;
    }
    if bytes_consumed as libc::c_int > 0 as libc::c_int {} else {
        __assert_fail(
            b"bytes_consumed > 0\0" as *const u8 as *const libc::c_char,
            b"probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
            440 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 88],
                &[libc::c_char; 88],
            >(
                b"bool process_response_answer(char **, uint16_t *, const char *, uint16_t, fieldset_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8403: {
        if bytes_consumed as libc::c_int > 0 as libc::c_int {} else {
            __assert_fail(
                b"bytes_consumed > 0\0" as *const u8 as *const libc::c_char,
                b"probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
                440 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 88],
                    &[libc::c_char; 88],
                >(
                    b"bool process_response_answer(char **, uint16_t *, const char *, uint16_t, fieldset_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (bytes_consumed as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<dns_answer_tail>() as libc::c_ulong)
        > *data_len as libc::c_ulong
    {
        free(answer_name as *mut libc::c_void);
        return 1 as libc::c_int as bool_0;
    }
    let mut tail: *mut dns_answer_tail = (*data)
        .offset(bytes_consumed as libc::c_int as isize) as *mut dns_answer_tail;
    let mut type_0: uint16_t = __bswap_16((*tail).type_0);
    let mut class: uint16_t = __bswap_16((*tail).class);
    let mut ttl: uint32_t = __bswap_32((*tail).ttl);
    let mut rdlength: uint16_t = __bswap_16((*tail).rdlength);
    let mut rdata: *mut libc::c_char = ((*tail).rdata).as_mut_ptr();
    if ((rdlength as libc::c_int + bytes_consumed as libc::c_int) as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<dns_answer_tail>() as libc::c_ulong)
        > *data_len as libc::c_ulong
    {
        free(answer_name as *mut libc::c_void);
        return 1 as libc::c_int as bool_0;
    }
    let mut afs: *mut fieldset_t = fs_new_fieldset(0 as *mut fielddefset_t);
    fs_add_unsafe_string(
        afs,
        b"name\0" as *const u8 as *const libc::c_char,
        answer_name,
        1 as libc::c_int,
    );
    fs_add_uint64(
        afs,
        b"type\0" as *const u8 as *const libc::c_char,
        type_0 as uint64_t,
    );
    if type_0 as libc::c_int > 255 as libc::c_int
        || qtype_qtype_to_strid[type_0 as usize] as libc::c_int == -(1 as libc::c_int)
    {
        fs_add_string(
            afs,
            b"type_str\0" as *const u8 as *const libc::c_char,
            b"BAD QTYPE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
    } else {
        fs_add_string(
            afs,
            b"type_str\0" as *const u8 as *const libc::c_char,
            qtype_strs[qtype_qtype_to_strid[type_0 as usize] as usize]
                as *mut libc::c_char,
            0 as libc::c_int,
        );
    }
    fs_add_uint64(
        afs,
        b"class\0" as *const u8 as *const libc::c_char,
        class as uint64_t,
    );
    fs_add_uint64(afs, b"ttl\0" as *const u8 as *const libc::c_char, ttl as uint64_t);
    fs_add_uint64(
        afs,
        b"rdlength\0" as *const u8 as *const libc::c_char,
        rdlength as uint64_t,
    );
    if type_0 as libc::c_int == DNS_QTYPE_NS as libc::c_int
        || type_0 as libc::c_int == DNS_QTYPE_CNAME as libc::c_int
    {
        let mut rdata_bytes_consumed: uint16_t = 0 as libc::c_int as uint16_t;
        let mut rdata_name: *mut libc::c_char = get_name(
            rdata,
            rdlength,
            payload,
            payload_len,
            &mut rdata_bytes_consumed,
        );
        if rdata_name.is_null() {
            fs_add_uint64(
                afs,
                b"rdata_is_parsed\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as uint64_t,
            );
            fs_add_binary(
                afs,
                b"rdata\0" as *const u8 as *const libc::c_char,
                rdlength as size_t,
                rdata as *mut libc::c_void,
                0 as libc::c_int,
            );
        } else {
            fs_add_uint64(
                afs,
                b"rdata_is_parsed\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as uint64_t,
            );
            fs_add_unsafe_string(
                afs,
                b"rdata\0" as *const u8 as *const libc::c_char,
                rdata_name,
                1 as libc::c_int,
            );
        }
    } else if type_0 as libc::c_int == DNS_QTYPE_MX as libc::c_int {
        let mut rdata_bytes_consumed_0: uint16_t = 0 as libc::c_int as uint16_t;
        if rdlength as libc::c_int <= 4 as libc::c_int {
            fs_add_uint64(
                afs,
                b"rdata_is_parsed\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as uint64_t,
            );
            fs_add_binary(
                afs,
                b"rdata\0" as *const u8 as *const libc::c_char,
                rdlength as size_t,
                rdata as *mut libc::c_void,
                0 as libc::c_int,
            );
        } else {
            let mut rdata_name_0: *mut libc::c_char = get_name(
                rdata.offset(2 as libc::c_int as isize),
                (rdlength as libc::c_int - 2 as libc::c_int) as uint16_t,
                payload,
                payload_len,
                &mut rdata_bytes_consumed_0,
            );
            if rdata_name_0.is_null() {
                fs_add_uint64(
                    afs,
                    b"rdata_is_parsed\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int as uint64_t,
                );
                fs_add_binary(
                    afs,
                    b"rdata\0" as *const u8 as *const libc::c_char,
                    rdlength as size_t,
                    rdata as *mut libc::c_void,
                    0 as libc::c_int,
                );
            } else {
                let mut rdata_with_pref: *mut libc::c_char = xmalloc(
                    ((5 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_add(strlen(rdata_name_0))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                let mut num_printed: uint8_t = snprintf(
                    rdata_with_pref,
                    6 as libc::c_int as libc::c_ulong,
                    b"%hu \0" as *const u8 as *const libc::c_char,
                    __bswap_16(*(rdata as *mut uint16_t)) as libc::c_int,
                ) as uint8_t;
                memcpy(
                    rdata_with_pref.offset(num_printed as libc::c_int as isize)
                        as *mut libc::c_void,
                    rdata_name_0 as *const libc::c_void,
                    strlen(rdata_name_0),
                );
                fs_add_uint64(
                    afs,
                    b"rdata_is_parsed\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int as uint64_t,
                );
                fs_add_unsafe_string(
                    afs,
                    b"rdata\0" as *const u8 as *const libc::c_char,
                    rdata_with_pref,
                    1 as libc::c_int,
                );
            }
        }
    } else if type_0 as libc::c_int == DNS_QTYPE_TXT as libc::c_int {
        if rdlength as libc::c_int >= 1 as libc::c_int
            && rdlength as libc::c_int - 1 as libc::c_int
                != *(rdata as *mut uint8_t) as libc::c_int
        {
            log_warn(
                b"dns\0" as *const u8 as *const libc::c_char,
                b"TXT record with wrong TXT len. Not processing.\0" as *const u8
                    as *const libc::c_char,
            );
            fs_add_uint64(
                afs,
                b"rdata_is_parsed\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as uint64_t,
            );
            fs_add_binary(
                afs,
                b"rdata\0" as *const u8 as *const libc::c_char,
                rdlength as size_t,
                rdata as *mut libc::c_void,
                0 as libc::c_int,
            );
        } else {
            fs_add_uint64(
                afs,
                b"rdata_is_parsed\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as uint64_t,
            );
            let mut txt: *mut libc::c_char = xmalloc(rdlength as size_t)
                as *mut libc::c_char;
            memcpy(
                txt as *mut libc::c_void,
                rdata.offset(1 as libc::c_int as isize) as *const libc::c_void,
                (rdlength as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            );
            fs_add_unsafe_string(
                afs,
                b"rdata\0" as *const u8 as *const libc::c_char,
                txt,
                1 as libc::c_int,
            );
        }
    } else if type_0 as libc::c_int == DNS_QTYPE_A as libc::c_int {
        if rdlength as libc::c_int != 4 as libc::c_int {
            log_warn(
                b"dns\0" as *const u8 as *const libc::c_char,
                b"A record with IP of length %d. Not processing.\0" as *const u8
                    as *const libc::c_char,
                rdlength as libc::c_int,
            );
            fs_add_uint64(
                afs,
                b"rdata_is_parsed\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as uint64_t,
            );
            fs_add_binary(
                afs,
                b"rdata\0" as *const u8 as *const libc::c_char,
                rdlength as size_t,
                rdata as *mut libc::c_void,
                0 as libc::c_int,
            );
        } else {
            fs_add_uint64(
                afs,
                b"rdata_is_parsed\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as uint64_t,
            );
            let mut addr: *mut libc::c_char = strdup(
                inet_ntoa(*(rdata as *mut in_addr)),
            );
            fs_add_unsafe_string(
                afs,
                b"rdata\0" as *const u8 as *const libc::c_char,
                addr,
                1 as libc::c_int,
            );
        }
    } else if type_0 as libc::c_int == DNS_QTYPE_AAAA as libc::c_int {
        if rdlength as libc::c_int != 16 as libc::c_int {
            log_warn(
                b"dns\0" as *const u8 as *const libc::c_char,
                b"AAAA record with IP of length %d. Not processing.\0" as *const u8
                    as *const libc::c_char,
                rdlength as libc::c_int,
            );
            fs_add_uint64(
                afs,
                b"rdata_is_parsed\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as uint64_t,
            );
            fs_add_binary(
                afs,
                b"rdata\0" as *const u8 as *const libc::c_char,
                rdlength as size_t,
                rdata as *mut libc::c_void,
                0 as libc::c_int,
            );
        } else {
            fs_add_uint64(
                afs,
                b"rdata_is_parsed\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as uint64_t,
            );
            let mut ipv6_str: *mut libc::c_char = xmalloc(46 as libc::c_int as size_t)
                as *mut libc::c_char;
            inet_ntop(
                10 as libc::c_int,
                rdata as *mut sockaddr_in6 as *const libc::c_void,
                ipv6_str,
                46 as libc::c_int as socklen_t,
            );
            fs_add_unsafe_string(
                afs,
                b"rdata\0" as *const u8 as *const libc::c_char,
                ipv6_str,
                1 as libc::c_int,
            );
        }
    } else {
        fs_add_uint64(
            afs,
            b"rdata_is_parsed\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as uint64_t,
        );
        fs_add_binary(
            afs,
            b"rdata\0" as *const u8 as *const libc::c_char,
            rdlength as size_t,
            rdata as *mut libc::c_void,
            0 as libc::c_int,
        );
    }
    fs_add_fieldset(list, 0 as *const libc::c_char, afs);
    *data = (*data)
        .offset(bytes_consumed as libc::c_int as isize)
        .offset(::std::mem::size_of::<dns_answer_tail>() as libc::c_ulong as isize)
        .offset(rdlength as libc::c_int as isize);
    *data_len = ((*data_len as libc::c_int - bytes_consumed as libc::c_int)
        as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<dns_answer_tail>() as libc::c_ulong)
        .wrapping_sub(rdlength as libc::c_ulong) as uint16_t;
    return 0 as libc::c_int as bool_0;
}
unsafe extern "C" fn dns_global_initialize(mut conf: *mut state_conf) -> libc::c_int {
    num_questions = (*conf).packet_streams;
    if num_questions < 1 as libc::c_int {
        log_fatal(
            b"dns\0" as *const u8 as *const libc::c_char,
            b"Invalid number of probes for the DNS module: %i\0" as *const u8
                as *const libc::c_char,
            num_questions,
        );
    }
    dns_packets = xmalloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(num_questions as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    dns_packet_lens = xmalloc(
        (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
            .wrapping_mul(num_questions as libc::c_ulong),
    ) as *mut uint16_t;
    qname_lens = xmalloc(
        (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
            .wrapping_mul(num_questions as libc::c_ulong),
    ) as *mut uint16_t;
    qnames = xmalloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(num_questions as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    qtypes = xmalloc(
        (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
            .wrapping_mul(num_questions as libc::c_ulong),
    ) as *mut uint16_t;
    let mut qtype_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut domains: *mut *mut libc::c_char = xmalloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(num_questions as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < num_questions {
        let ref mut fresh2 = *domains.offset(i as isize);
        *fresh2 = default_domain.as_ptr() as *mut libc::c_char;
        *qtypes.offset(i as isize) = default_qtype;
        i += 1;
        i;
    }
    num_ports = (*conf).source_port_last as libc::c_int
        - (*conf).source_port_first as libc::c_int + 1 as libc::c_int;
    setup_qtype_str_map();
    if !((*conf).probe_args).is_null() {
        let mut arg_strlen: libc::c_int = strlen((*conf).probe_args) as libc::c_int;
        let mut arg_pos: *mut libc::c_char = (*conf).probe_args;
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < num_questions {
            if arg_pos >= ((*conf).probe_args).offset(arg_strlen as isize) {
                log_fatal(
                    b"dns\0" as *const u8 as *const libc::c_char,
                    b"More probes than questions configured. Add additional questions.\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            let mut probe_q_delimiter_p: *mut libc::c_char = strchr(arg_pos, ',' as i32);
            let mut probe_arg_delimiter_p: *mut libc::c_char = strchr(
                arg_pos,
                ';' as i32,
            );
            if probe_q_delimiter_p.is_null() || probe_q_delimiter_p == arg_pos
                || arg_pos.offset(strlen(arg_pos) as isize)
                    == probe_q_delimiter_p.offset(1 as libc::c_int as isize)
                || probe_arg_delimiter_p.is_null()
                    && i_0 + 1 as libc::c_int != num_questions
            {
                log_fatal(
                    b"dns\0" as *const u8 as *const libc::c_char,
                    b"Invalid probe args. Format: \"A,google.com\" or \"A,google.com;A,example.com\"\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            let mut domain_len: libc::c_int = 0 as libc::c_int;
            if !probe_arg_delimiter_p.is_null() {
                domain_len = (probe_arg_delimiter_p.offset_from(probe_q_delimiter_p)
                    as libc::c_long - 1 as libc::c_int as libc::c_long) as libc::c_int;
            } else {
                domain_len = strlen(probe_q_delimiter_p) as libc::c_int;
            }
            if domain_len > 0 as libc::c_int {} else {
                __assert_fail(
                    b"domain_len > 0\0" as *const u8 as *const libc::c_char,
                    b"probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
                    637 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 47],
                        &[libc::c_char; 47],
                    >(b"int dns_global_initialize(struct state_conf *)\0"))
                        .as_ptr(),
                );
            }
            'c_10751: {
                if domain_len > 0 as libc::c_int {} else {
                    __assert_fail(
                        b"domain_len > 0\0" as *const u8 as *const libc::c_char,
                        b"probe_modules/module_dns.c\0" as *const u8
                            as *const libc::c_char,
                        637 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 47],
                            &[libc::c_char; 47],
                        >(b"int dns_global_initialize(struct state_conf *)\0"))
                            .as_ptr(),
                    );
                }
            };
            let ref mut fresh3 = *domains.offset(i_0 as isize);
            *fresh3 = xmalloc((domain_len + 1 as libc::c_int) as size_t)
                as *mut libc::c_char;
            strncpy(
                *domains.offset(i_0 as isize),
                probe_q_delimiter_p.offset(1 as libc::c_int as isize),
                domain_len as libc::c_ulong,
            );
            *(*domains.offset(i_0 as isize))
                .offset(domain_len as isize) = '\0' as i32 as libc::c_char;
            qtype_str = xmalloc(
                (probe_q_delimiter_p.offset_from(arg_pos) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as size_t,
            ) as *mut libc::c_char;
            strncpy(
                qtype_str,
                arg_pos,
                probe_q_delimiter_p.offset_from(arg_pos) as libc::c_long as libc::c_ulong,
            );
            *qtype_str
                .offset(
                    probe_q_delimiter_p.offset_from(arg_pos) as libc::c_long as isize,
                ) = '\0' as i32 as libc::c_char;
            *qtypes.offset(i_0 as isize) = qtype_str_to_code(qtype_str);
            free(qtype_str as *mut libc::c_void);
            if *qtypes.offset(i_0 as isize) == 0 {
                log_fatal(
                    b"dns\0" as *const u8 as *const libc::c_char,
                    b"Incorrect qtype supplied. %s\0" as *const u8
                        as *const libc::c_char,
                    qtype_str,
                );
            }
            arg_pos = probe_q_delimiter_p
                .offset(domain_len as isize)
                .offset(2 as libc::c_int as isize);
            i_0 += 1;
            i_0;
        }
        if arg_pos
            != ((*conf).probe_args)
                .offset(arg_strlen as isize)
                .offset(2 as libc::c_int as isize)
        {
            log_fatal(
                b"dns\0" as *const u8 as *const libc::c_char,
                b"More args than probes passed. Add additional probes.\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    let mut max_payload_len: size_t = 0;
    let mut ret: libc::c_int = build_global_dns_packets(
        domains,
        num_questions,
        &mut max_payload_len,
    );
    module_dns
        .max_packet_length = max_payload_len
        .wrapping_add(::std::mem::size_of::<ether_header>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong);
    return ret;
}
unsafe extern "C" fn dns_global_cleanup(
    mut zconf_0: *mut state_conf,
    mut zsend: *mut state_send,
    mut zrecv: *mut state_recv,
) -> libc::c_int {
    if !dns_packets.is_null() {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < num_questions {
            if !(*dns_packets.offset(i as isize)).is_null() {
                free(*dns_packets.offset(i as isize) as *mut libc::c_void);
            }
            i += 1;
            i;
        }
        free(dns_packets as *mut libc::c_void);
    }
    dns_packets = 0 as *mut *mut libc::c_char;
    if !qnames.is_null() {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < num_questions {
            if !(*qnames.offset(i_0 as isize)).is_null() {
                free(*qnames.offset(i_0 as isize) as *mut libc::c_void);
            }
            i_0 += 1;
            i_0;
        }
        free(qnames as *mut libc::c_void);
    }
    qnames = 0 as *mut *mut libc::c_char;
    if !dns_packet_lens.is_null() {
        free(dns_packet_lens as *mut libc::c_void);
    }
    if !qname_lens.is_null() {
        free(qname_lens as *mut libc::c_void);
    }
    if !qtypes.is_null() {
        free(qtypes as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dns_init_perthread(
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
    let mut len: uint16_t = __bswap_16(
        (::std::mem::size_of::<ip>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
            .wrapping_add(
                *dns_packet_lens.offset(0 as libc::c_int as isize) as libc::c_ulong,
            ) as __uint16_t,
    );
    make_ip_header(ip_header, IPPROTO_UDP as libc::c_int as uint8_t, len);
    let mut udp_header: *mut udphdr = &mut *ip_header.offset(1 as libc::c_int as isize)
        as *mut ip as *mut udphdr;
    len = (::std::mem::size_of::<udphdr>() as libc::c_ulong)
        .wrapping_add(
            *dns_packet_lens.offset(0 as libc::c_int as isize) as libc::c_ulong,
        ) as uint16_t;
    make_udp_header(udp_header, zconf.target_port, len);
    let mut payload: *mut libc::c_char = &mut *udp_header
        .offset(1 as libc::c_int as isize) as *mut udphdr as *mut libc::c_char;
    memcpy(
        payload as *mut libc::c_void,
        *dns_packets.offset(0 as libc::c_int as isize) as *const libc::c_void,
        *dns_packet_lens.offset(0 as libc::c_int as isize) as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dns_make_packet(
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
    if num_questions > 1 as libc::c_int {
        let mut encoded_len: uint16_t = __bswap_16(
            (::std::mem::size_of::<ip>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
                .wrapping_add(
                    *dns_packet_lens.offset(probe_num as isize) as libc::c_ulong,
                ) as __uint16_t,
        );
        make_ip_header(ip_header, IPPROTO_UDP as libc::c_int as uint8_t, encoded_len);
        encoded_len = (::std::mem::size_of::<udphdr>() as libc::c_ulong)
            .wrapping_add(*dns_packet_lens.offset(probe_num as isize) as libc::c_ulong)
            as uint16_t;
        make_udp_header(
            udp_header,
            __bswap_16((*udp_header).c2rust_unnamed.c2rust_unnamed.uh_dport),
            encoded_len,
        );
        let mut payload: *mut libc::c_char = &mut *udp_header
            .offset(1 as libc::c_int as isize) as *mut udphdr as *mut libc::c_char;
        *buf_len = (::std::mem::size_of::<ether_header>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
            .wrapping_add(*dns_packet_lens.offset(probe_num as isize) as libc::c_ulong);
        if *buf_len <= 4096 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"*buf_len <= MAX_PACKET_SIZE\0" as *const u8 as *const libc::c_char,
                b"probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
                766 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 96],
                    &[libc::c_char; 96],
                >(
                    b"int dns_make_packet(void *, size_t *, ipaddr_n_t, ipaddr_n_t, uint8_t, uint32_t *, int, void *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_9832: {
            if *buf_len <= 4096 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"*buf_len <= MAX_PACKET_SIZE\0" as *const u8 as *const libc::c_char,
                    b"probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
                    766 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 96],
                        &[libc::c_char; 96],
                    >(
                        b"int dns_make_packet(void *, size_t *, ipaddr_n_t, ipaddr_n_t, uint8_t, uint32_t *, int, void *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        memcpy(
            payload as *mut libc::c_void,
            *dns_packets.offset(probe_num as isize) as *const libc::c_void,
            *dns_packet_lens.offset(probe_num as isize) as libc::c_ulong,
        );
    }
    (*ip_header).ip_src.s_addr = src_ip;
    (*ip_header).ip_dst.s_addr = dst_ip;
    (*ip_header).ip_ttl = ttl;
    (*udp_header)
        .c2rust_unnamed
        .c2rust_unnamed
        .uh_sport = __bswap_16(get_src_port(num_ports, probe_num, validation));
    let mut dns_header_p: *mut dns_header = &mut *udp_header
        .offset(1 as libc::c_int as isize) as *mut udphdr as *mut dns_header;
    (*dns_header_p)
        .id = (*validation.offset(2 as libc::c_int as isize)
        & 0xffff as libc::c_int as libc::c_uint) as uint16_t;
    (*ip_header).ip_sum = 0 as libc::c_int as libc::c_ushort;
    (*ip_header).ip_sum = zmap_ip_checksum(ip_header as *mut libc::c_ushort);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dns_print_packet(
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
        b"------------------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        fp,
        b"dns { source: %u | dest: %u | checksum: %#04X }\n\0" as *const u8
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
pub unsafe extern "C" fn dns_validate_packet(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
    mut src_ip: *mut uint32_t,
    mut validation: *mut uint32_t,
) -> libc::c_int {
    if udp_do_validate_packet(
        ip_hdr,
        len,
        src_ip,
        validation,
        num_ports,
        zconf.target_port as libc::c_int,
    ) == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*ip_hdr).ip_p as libc::c_int == IPPROTO_UDP as libc::c_int {
        let mut udp: *mut udphdr = get_udp_header(ip_hdr, len);
        if udp.is_null() {
            return 0 as libc::c_int;
        }
        let mut udp_len: uint16_t = __bswap_16(
            (*udp).c2rust_unnamed.c2rust_unnamed.uh_ulen,
        );
        let mut match_0: libc::c_int = 0 as libc::c_int;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < num_questions {
            if udp_len as libc::c_int
                >= *dns_packet_lens.offset(i as isize) as libc::c_int
            {
                match_0 += 1 as libc::c_int;
            }
            i += 1;
            i;
        }
        if match_0 == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        if len < udp_len as libc::c_uint {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn dns_add_null_fs(mut fs: *mut fieldset_t) {
    fs_add_null(fs, b"dns_id\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_rd\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_tc\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_aa\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_opcode\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_qr\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_rcode\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_cd\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_ad\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_z\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_ra\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_qdcount\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_ancount\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_nscount\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"dns_arcount\0" as *const u8 as *const libc::c_char);
    fs_add_repeated(
        fs,
        b"dns_questions\0" as *const u8 as *const libc::c_char,
        fs_new_repeated_fieldset(),
    );
    fs_add_repeated(
        fs,
        b"dns_answers\0" as *const u8 as *const libc::c_char,
        fs_new_repeated_fieldset(),
    );
    fs_add_repeated(
        fs,
        b"dns_authorities\0" as *const u8 as *const libc::c_char,
        fs_new_repeated_fieldset(),
    );
    fs_add_repeated(
        fs,
        b"dns_additionals\0" as *const u8 as *const libc::c_char,
        fs_new_repeated_fieldset(),
    );
    fs_add_uint64(
        fs,
        b"dns_parse_err\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as uint64_t,
    );
    fs_add_uint64(
        fs,
        b"dns_unconsumed_bytes\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as uint64_t,
    );
}
pub unsafe extern "C" fn dns_process_packet(
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
        let mut udp_hdr: *mut udphdr = get_udp_header(ip_hdr, len);
        if !udp_hdr.is_null() {} else {
            __assert_fail(
                b"udp_hdr\0" as *const u8 as *const libc::c_char,
                b"probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
                869 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 93],
                    &[libc::c_char; 93],
                >(
                    b"void dns_process_packet(const u_char *, uint32_t, fieldset_t *, uint32_t *, struct timespec)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_9407: {
            if !udp_hdr.is_null() {} else {
                __assert_fail(
                    b"udp_hdr\0" as *const u8 as *const libc::c_char,
                    b"probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
                    869 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 93],
                        &[libc::c_char; 93],
                    >(
                        b"void dns_process_packet(const u_char *, uint32_t, fieldset_t *, uint32_t *, struct timespec)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        let mut udp_len: uint16_t = __bswap_16(
            (*udp_hdr).c2rust_unnamed.c2rust_unnamed.uh_ulen,
        );
        let mut match_0: libc::c_int = 0 as libc::c_int;
        let mut is_valid: bool_0 = 0 as libc::c_int as bool_0;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < num_questions {
            if !((udp_len as libc::c_int)
                < *dns_packet_lens.offset(i as isize) as libc::c_int)
            {
                match_0 += 1 as libc::c_int;
                let mut qname_p: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut tail_p: *mut dns_question_tail = 0 as *mut dns_question_tail;
                let mut dns_header_p: *mut dns_header = &mut *udp_hdr
                    .offset(1 as libc::c_int as isize) as *mut udphdr as *mut dns_header;
                if (*dns_header_p).id as libc::c_uint
                    == *validation.offset(2 as libc::c_int as isize)
                        & 0xffff as libc::c_int as libc::c_uint
                {
                    qname_p = (dns_header_p as *mut libc::c_char)
                        .offset(
                            ::std::mem::size_of::<dns_header>() as libc::c_ulong as isize,
                        );
                    tail_p = (*dns_packets.offset(i as isize))
                        .offset(
                            ::std::mem::size_of::<dns_header>() as libc::c_ulong as isize,
                        )
                        .offset(*qname_lens.offset(i as isize) as libc::c_int as isize)
                        as *mut dns_question_tail;
                    if strcmp(*qnames.offset(i as isize), qname_p) == 0 as libc::c_int {
                        if (*tail_p).qtype as libc::c_int
                            == __bswap_16(*qtypes.offset(i as isize)) as libc::c_int
                            && (*tail_p).qclass as libc::c_int
                                == __bswap_16(0x1 as libc::c_int as __uint16_t)
                                    as libc::c_int
                        {
                            is_valid = 1 as libc::c_int as bool_0;
                            break;
                        }
                    }
                }
            }
            i += 1;
            i;
        }
        if match_0 > 0 as libc::c_int {} else {
            __assert_fail(
                b"match > 0\0" as *const u8 as *const libc::c_char,
                b"probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
                903 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 93],
                    &[libc::c_char; 93],
                >(
                    b"void dns_process_packet(const u_char *, uint32_t, fieldset_t *, uint32_t *, struct timespec)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_9218: {
            if match_0 > 0 as libc::c_int {} else {
                __assert_fail(
                    b"match > 0\0" as *const u8 as *const libc::c_char,
                    b"probe_modules/module_dns.c\0" as *const u8 as *const libc::c_char,
                    903 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 93],
                        &[libc::c_char; 93],
                    >(
                        b"void dns_process_packet(const u_char *, uint32_t, fieldset_t *, uint32_t *, struct timespec)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        let mut dns_hdr: *mut dns_header = &mut *udp_hdr
            .offset(1 as libc::c_int as isize) as *mut udphdr as *mut dns_header;
        let mut qr: uint16_t = (*dns_hdr).qr() as uint16_t;
        let mut rcode: uint16_t = (*dns_hdr).rcode() as uint16_t;
        fs_add_uint64(
            fs,
            b"sport\0" as *const u8 as *const libc::c_char,
            __bswap_16((*udp_hdr).c2rust_unnamed.c2rust_unnamed.uh_sport) as uint64_t,
        );
        fs_add_uint64(
            fs,
            b"dport\0" as *const u8 as *const libc::c_char,
            __bswap_16((*udp_hdr).c2rust_unnamed.c2rust_unnamed.uh_dport) as uint64_t,
        );
        fs_add_string(
            fs,
            b"classification\0" as *const u8 as *const libc::c_char,
            b"dns\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
        fs_add_bool(
            fs,
            b"success\0" as *const u8 as *const libc::c_char,
            is_valid as libc::c_int,
        );
        fs_add_bool(
            fs,
            b"app_success\0" as *const u8 as *const libc::c_char,
            (is_valid as libc::c_int != 0 && qr as libc::c_int == 1 as libc::c_int
                && rcode as libc::c_int == DNS_RCODE_NOERR as libc::c_int) as libc::c_int,
        );
        fs_add_null_icmp(fs);
        fs_add_uint64(
            fs,
            b"udp_len\0" as *const u8 as *const libc::c_char,
            udp_len as uint64_t,
        );
        if is_valid == 0 {
            dns_add_null_fs(fs);
        } else {
            fs_add_uint64(
                fs,
                b"dns_id\0" as *const u8 as *const libc::c_char,
                __bswap_16((*dns_hdr).id) as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_rd\0" as *const u8 as *const libc::c_char,
                (*dns_hdr).rd() as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_tc\0" as *const u8 as *const libc::c_char,
                (*dns_hdr).tc() as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_aa\0" as *const u8 as *const libc::c_char,
                (*dns_hdr).aa() as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_opcode\0" as *const u8 as *const libc::c_char,
                (*dns_hdr).opcode() as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_qr\0" as *const u8 as *const libc::c_char,
                qr as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_rcode\0" as *const u8 as *const libc::c_char,
                rcode as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_cd\0" as *const u8 as *const libc::c_char,
                (*dns_hdr).cd() as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_ad\0" as *const u8 as *const libc::c_char,
                (*dns_hdr).ad() as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_z\0" as *const u8 as *const libc::c_char,
                (*dns_hdr).z() as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_ra\0" as *const u8 as *const libc::c_char,
                (*dns_hdr).ra() as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_qdcount\0" as *const u8 as *const libc::c_char,
                __bswap_16((*dns_hdr).qdcount) as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_ancount\0" as *const u8 as *const libc::c_char,
                __bswap_16((*dns_hdr).ancount) as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_nscount\0" as *const u8 as *const libc::c_char,
                __bswap_16((*dns_hdr).nscount) as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_arcount\0" as *const u8 as *const libc::c_char,
                __bswap_16((*dns_hdr).arcount) as uint64_t,
            );
            let mut data: *mut libc::c_char = (dns_hdr as *mut libc::c_char)
                .offset(::std::mem::size_of::<dns_header>() as libc::c_ulong as isize);
            let mut data_len: uint16_t = (udp_len as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<*mut udphdr>() as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<dns_header>() as libc::c_ulong)
                as uint16_t;
            let mut err: bool_0 = 0 as libc::c_int as bool_0;
            let mut list: *mut fieldset_t = fs_new_repeated_fieldset();
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < __bswap_16((*dns_hdr).qdcount) as libc::c_int && err == 0 {
                err = process_response_question(
                    &mut data,
                    &mut data_len,
                    dns_hdr as *mut libc::c_char,
                    udp_len,
                    list,
                );
                i_0 += 1;
                i_0;
            }
            fs_add_repeated(
                fs,
                b"dns_questions\0" as *const u8 as *const libc::c_char,
                list,
            );
            list = fs_new_repeated_fieldset();
            let mut i_1: libc::c_int = 0 as libc::c_int;
            while i_1 < __bswap_16((*dns_hdr).ancount) as libc::c_int && err == 0 {
                err = process_response_answer(
                    &mut data,
                    &mut data_len,
                    dns_hdr as *mut libc::c_char,
                    udp_len,
                    list,
                );
                i_1 += 1;
                i_1;
            }
            fs_add_repeated(
                fs,
                b"dns_answers\0" as *const u8 as *const libc::c_char,
                list,
            );
            list = fs_new_repeated_fieldset();
            let mut i_2: libc::c_int = 0 as libc::c_int;
            while i_2 < __bswap_16((*dns_hdr).nscount) as libc::c_int && err == 0 {
                err = process_response_answer(
                    &mut data,
                    &mut data_len,
                    dns_hdr as *mut libc::c_char,
                    udp_len,
                    list,
                );
                i_2 += 1;
                i_2;
            }
            fs_add_repeated(
                fs,
                b"dns_authorities\0" as *const u8 as *const libc::c_char,
                list,
            );
            list = fs_new_repeated_fieldset();
            let mut i_3: libc::c_int = 0 as libc::c_int;
            while i_3 < __bswap_16((*dns_hdr).arcount) as libc::c_int && err == 0 {
                err = process_response_answer(
                    &mut data,
                    &mut data_len,
                    dns_hdr as *mut libc::c_char,
                    udp_len,
                    list,
                );
                i_3 += 1;
                i_3;
            }
            fs_add_repeated(
                fs,
                b"dns_additionals\0" as *const u8 as *const libc::c_char,
                list,
            );
            if data_len as libc::c_int != 0 as libc::c_int {
                err = 1 as libc::c_int as bool_0;
            }
            fs_add_uint64(
                fs,
                b"dns_parse_err\0" as *const u8 as *const libc::c_char,
                err as uint64_t,
            );
            fs_add_uint64(
                fs,
                b"dns_unconsumed_bytes\0" as *const u8 as *const libc::c_char,
                data_len as uint64_t,
            );
        }
        fs_add_binary(
            fs,
            b"raw_data\0" as *const u8 as *const libc::c_char,
            (udp_len as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<udphdr>() as libc::c_ulong),
            &mut *udp_hdr.offset(1 as libc::c_int as isize) as *mut udphdr
                as *mut libc::c_void,
            0 as libc::c_int,
        );
    } else if (*ip_hdr).ip_p as libc::c_int == IPPROTO_ICMP as libc::c_int {
        fs_add_null(fs, b"sport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"dport\0" as *const u8 as *const libc::c_char);
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
        fs_add_bool(
            fs,
            b"app_success\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        fs_populate_icmp_from_iphdr(ip_hdr, len as size_t, fs);
        fs_add_null(fs, b"udp_len\0" as *const u8 as *const libc::c_char);
        dns_add_null_fs(fs);
        fs_add_binary(
            fs,
            b"raw_data\0" as *const u8 as *const libc::c_char,
            len as size_t,
            packet as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        );
    } else {
        log_fatal(
            b"dns\0" as *const u8 as *const libc::c_char,
            b"Die. This can only happen if you change the pcap filter and don't update the process function.\0"
                as *const u8 as *const libc::c_char,
        );
    };
}
static mut fields: [fielddef_t; 32] = [
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
            name: b"app_success\0" as *const u8 as *const libc::c_char,
            type_0: b"bool\0" as *const u8 as *const libc::c_char,
            desc: b"Is the RA bit set with no error code?\0" as *const u8
                as *const libc::c_char,
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
    {
        let mut init = field_def {
            name: b"udp_len\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"UDP packet lenght\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_id\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS transaction ID\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_rd\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS recursion desired\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_tc\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS packet truncated\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_aa\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS authoritative answer\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_opcode\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS opcode (query type)\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_qr\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS query(0) or response (1)\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_rcode\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS response code\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_cd\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS checking disabled\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_ad\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS authenticated data\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_z\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS reserved\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_ra\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS recursion available\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_qdcount\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS number questions\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_ancount\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS number answer RR's\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_nscount\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS number NS RR's in authority section\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_arcount\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"DNS number additional RR's\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_questions\0" as *const u8 as *const libc::c_char,
            type_0: b"repeated\0" as *const u8 as *const libc::c_char,
            desc: b"DNS question list\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_answers\0" as *const u8 as *const libc::c_char,
            type_0: b"repeated\0" as *const u8 as *const libc::c_char,
            desc: b"DNS answer list\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_authorities\0" as *const u8 as *const libc::c_char,
            type_0: b"repeated\0" as *const u8 as *const libc::c_char,
            desc: b"DNS authority list\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_additionals\0" as *const u8 as *const libc::c_char,
            type_0: b"repeated\0" as *const u8 as *const libc::c_char,
            desc: b"DNS additional list\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_parse_err\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"Problem parsing the DNS response\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dns_unconsumed_bytes\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"Bytes left over when parsing the DNS response\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"raw_data\0" as *const u8 as *const libc::c_char,
            type_0: b"binary\0" as *const u8 as *const libc::c_char,
            desc: b"UDP payload\0" as *const u8 as *const libc::c_char,
        };
        init
    },
];
pub static mut module_dns: probe_module_t = probe_module_t {
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
    module_dns = {
        let mut init = probe_module {
            name: b"dns\0" as *const u8 as *const libc::c_char,
            max_packet_length: 0 as libc::c_int as size_t,
            pcap_filter: b"udp || icmp\0" as *const u8 as *const libc::c_char,
            pcap_snaplen: 1500 as libc::c_int as size_t,
            port_args: 1 as libc::c_int as uint8_t,
            global_initialize: Some(
                dns_global_initialize
                    as unsafe extern "C" fn(*mut state_conf) -> libc::c_int,
            ),
            thread_initialize: Some(
                dns_init_perthread
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut macaddr_t,
                        *mut macaddr_t,
                        port_h_t,
                        *mut *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            make_packet: Some(
                dns_make_packet
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
                dns_print_packet
                    as unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> (),
            ),
            validate_packet: Some(
                dns_validate_packet
                    as unsafe extern "C" fn(
                        *const ip,
                        uint32_t,
                        *mut uint32_t,
                        *mut uint32_t,
                    ) -> libc::c_int,
            ),
            process_packet: Some(
                dns_process_packet
                    as unsafe extern "C" fn(
                        *const u_char,
                        uint32_t,
                        *mut fieldset_t,
                        *mut uint32_t,
                        timespec,
                    ) -> (),
            ),
            close: Some(
                dns_global_cleanup
                    as unsafe extern "C" fn(
                        *mut state_conf,
                        *mut state_send,
                        *mut state_recv,
                    ) -> libc::c_int,
            ),
            output_type: 2 as libc::c_int,
            fields: fields.as_mut_ptr(),
            numfields: (::std::mem::size_of::<[fielddef_t; 32]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<fielddef_t>() as libc::c_ulong)
                as libc::c_int,
            helptext: b"This module sends out DNS queries and parses basic responses. By default, the module will perform an A record lookup for google.com. You can specify other queries using the --probe-args argument in the form: 'type,query', e.g. 'A,google.com'. The module supports sending the the following types: of queries: A, NS, CNAME, SOA, PTR, MX, TXT, AAAA, RRSIG, and ALL. The module will accept and attempt to parse all DNS responses. There is currently support for parsing out full data from A, NS, CNAME, MX, TXT, and AAAA. Any other types will be output in raw form.\0"
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
