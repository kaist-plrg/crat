use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type aesrand;
    pub type output_module;
    fn free(__ptr: *mut libc::c_void);
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    fn fs_add_null(fs: *mut fieldset_t, name: *const libc::c_char);
    fn fs_add_uint64(fs: *mut fieldset_t, name: *const libc::c_char, value: uint64_t);
    fn fs_add_bool(fs: *mut fieldset_t, name: *const libc::c_char, value: libc::c_int);
    fn fs_add_string(
        fs: *mut fieldset_t,
        name: *const libc::c_char,
        value: *mut libc::c_char,
        free_: libc::c_int,
    );
    fn fs_add_binary(
        fs: *mut fieldset_t,
        name: *const libc::c_char,
        len: size_t,
        value: *mut libc::c_void,
        free_: libc::c_int,
    );
    fn make_eth_header(
        ethh: *mut ether_header,
        src: *mut macaddr_t,
        dst: *mut macaddr_t,
    );
    fn make_ip_header(iph: *mut ip, _: uint8_t, _: uint16_t);
    fn icmp_helper_validate(
        ip_hdr: *const ip,
        len: uint32_t,
        min_l4_len: size_t,
        probe_pkt: *mut *mut ip,
        probe_len: *mut size_t,
    ) -> libc::c_int;
    fn fprintf_eth_header(fp: *mut FILE, ethh: *mut ether_header);
    fn make_icmp_header(_: *mut icmp);
    fn fprintf_ip_header(fp: *mut FILE, iph: *mut ip);
    fn log_error(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn validate_gen(src: uint32_t, dst: uint32_t, output: *mut uint8_t);
}
pub type size_t = libc::c_ulong;
pub type __u_char = libc::c_uchar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type u_char = __u_char;
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
pub struct icmp_ra_addr {
    pub ira_addr: uint32_t,
    pub ira_preference: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp {
    pub icmp_type: uint8_t,
    pub icmp_code: uint8_t,
    pub icmp_cksum: uint16_t,
    pub icmp_hun: C2RustUnnamed_3,
    pub icmp_dun: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub id_ts: C2RustUnnamed_2,
    pub id_ip: C2RustUnnamed_1,
    pub id_radv: icmp_ra_addr,
    pub id_mask: uint32_t,
    pub id_data: [uint8_t; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub idi_ip: ip,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub its_otime: uint32_t,
    pub its_rtime: uint32_t,
    pub its_ttime: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub ih_pptr: libc::c_uchar,
    pub ih_gwaddr: in_addr,
    pub ih_idseq: ih_idseq,
    pub ih_void: uint32_t,
    pub ih_pmtu: ih_pmtu,
    pub ih_rtradv: ih_rtradv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ih_rtradv {
    pub irt_num_addrs: uint8_t,
    pub irt_wpa: uint8_t,
    pub irt_lifetime: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ih_pmtu {
    pub ipm_void: uint16_t,
    pub ipm_nextmtu: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ih_idseq {
    pub icd_id: uint16_t,
    pub icd_seq: uint16_t,
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
unsafe extern "C" fn in_icmp_checksum(
    mut ip_pkt: *mut libc::c_ushort,
    mut len: libc::c_int,
) -> libc::c_ushort {
    let mut sum: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut nwords: libc::c_int = len / 2 as libc::c_int;
    while nwords > 0 as libc::c_int {
        let fresh1 = ip_pkt;
        ip_pkt = ip_pkt.offset(1);
        sum = sum.wrapping_add(*fresh1 as libc::c_ulong);
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
unsafe extern "C" fn icmp_checksum(
    mut buf: *mut libc::c_ushort,
    mut buflen: size_t,
) -> libc::c_ushort {
    return in_icmp_checksum(buf, buflen as libc::c_int);
}
#[inline]
unsafe extern "C" fn get_icmp_header(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
) -> *mut icmp {
    if ((4 as libc::c_int * (*ip_hdr).ip_hl() as libc::c_int) as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<icmp>() as libc::c_ulong)
        > len as libc::c_ulong
    {
        return 0 as *mut icmp;
    }
    return (ip_hdr as *mut libc::c_char)
        .offset((4 as libc::c_int * (*ip_hdr).ip_hl() as libc::c_int) as isize)
        as *mut icmp;
}
pub static mut icmp_usage_error: *const libc::c_char = b"unknown ICMP probe specification (expected file:/path or text:STRING or hex:01020304)\0"
    as *const u8 as *const libc::c_char;
static mut icmp_payload_len: size_t = 0 as libc::c_int as size_t;
static mut icmp_payload_default_len: size_t = 20 as libc::c_int as size_t;
static mut icmp_payload: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub unsafe extern "C" fn icmp_global_initialize(
    mut conf: *mut state_conf,
) -> libc::c_int {
    if !(!((*conf).probe_args).is_null()
        && strlen((*conf).probe_args) > 0 as libc::c_int as libc::c_ulong)
    {
        icmp_payload = xmalloc(icmp_payload_default_len) as *mut libc::c_char;
        icmp_payload_len = icmp_payload_default_len;
        return 0 as libc::c_int;
    }
    let mut c: *mut libc::c_char = strchr((*conf).probe_args, ':' as i32);
    if c.is_null() {
        log_error(b"icmp\0" as *const u8 as *const libc::c_char, icmp_usage_error);
        return 1 as libc::c_int;
    }
    c = c.offset(1);
    c;
    if strncmp(
        (*conf).probe_args,
        b"text\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        icmp_payload = strdup(c);
        icmp_payload_len = strlen(icmp_payload);
    } else if strncmp(
        (*conf).probe_args,
        b"file\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        let mut inp: *mut FILE = fopen(c, b"rb\0" as *const u8 as *const libc::c_char);
        if inp.is_null() {
            log_error(
                b"icmp\0" as *const u8 as *const libc::c_char,
                b"could not open ICMP data file '%s'\0" as *const u8
                    as *const libc::c_char,
                c,
            );
            return 1 as libc::c_int;
        }
        if fseek(inp, 0 as libc::c_int as libc::c_long, 2 as libc::c_int) != 0 {
            log_error(
                b"icmp\0" as *const u8 as *const libc::c_char,
                b"unable to get size of ICMP data file '%s'\0" as *const u8
                    as *const libc::c_char,
                c,
            );
            return 1 as libc::c_int;
        }
        let mut input_size: size_t = ftell(inp) as size_t;
        if input_size > 1458 as libc::c_int as libc::c_ulong {
            log_error(
                b"icmp\0" as *const u8 as *const libc::c_char,
                b"input file larger than %d bytes and will not fit on the wire (%llu bytes provided)\0"
                    as *const u8 as *const libc::c_char,
                1458 as libc::c_int,
                input_size,
            );
            return 1 as libc::c_int;
        }
        if fseek(inp, 0 as libc::c_int as libc::c_long, 0 as libc::c_int) != 0 {
            log_error(
                b"icmp\0" as *const u8 as *const libc::c_char,
                b"unable to read ICMP data file '%s'\0" as *const u8
                    as *const libc::c_char,
                c,
            );
            return 1 as libc::c_int;
        }
        icmp_payload = xmalloc(1458 as libc::c_int as size_t) as *mut libc::c_char;
        icmp_payload_len = fread(
            icmp_payload as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            1458 as libc::c_int as libc::c_ulong,
            inp,
        );
        fclose(inp);
    } else if strncmp(
        (*conf).probe_args,
        b"hex\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        if (strlen(c)).wrapping_rem(2 as libc::c_int as libc::c_ulong)
            != 0 as libc::c_int as libc::c_ulong
        {
            log_error(
                b"icmp\0" as *const u8 as *const libc::c_char,
                b"invalid hex input (length must be a multiple of 2)\0" as *const u8
                    as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
        icmp_payload_len = (strlen(c)).wrapping_div(2 as libc::c_int as libc::c_ulong);
        icmp_payload = xmalloc(icmp_payload_len) as *mut libc::c_char;
        let mut n: libc::c_uint = 0;
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < icmp_payload_len {
            if sscanf(
                c.offset(i.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize),
                b"%2x\0" as *const u8 as *const libc::c_char,
                &mut n as *mut libc::c_uint,
            ) != 1 as libc::c_int
            {
                free(icmp_payload as *mut libc::c_void);
                log_error(
                    b"icmp\0" as *const u8 as *const libc::c_char,
                    b"non-hex character: '%c'\0" as *const u8 as *const libc::c_char,
                    *c.offset(i.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize)
                        as libc::c_int,
                );
                return 1 as libc::c_int;
            }
            *icmp_payload
                .offset(
                    i as isize,
                ) = (n & 0xff as libc::c_int as libc::c_uint) as libc::c_char;
            i = i.wrapping_add(1);
            i;
        }
    } else {
        log_error(b"icmp\0" as *const u8 as *const libc::c_char, icmp_usage_error);
        return 1 as libc::c_int;
    }
    if icmp_payload_len > 1458 as libc::c_int as libc::c_ulong {
        log_error(
            b"icmp\0" as *const u8 as *const libc::c_char,
            b"reducing ICMP payload must be at most %d bytes to fit on the wire (%d were provided)\n\0"
                as *const u8 as *const libc::c_char,
            1458 as libc::c_int,
            icmp_payload_len,
        );
        return 1 as libc::c_int;
    }
    module_icmp_echo
        .max_packet_length = (::std::mem::size_of::<ether_header>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(8 as libc::c_int as libc::c_ulong)
        .wrapping_add(icmp_payload_len);
    if module_icmp_echo.max_packet_length <= 1500 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"module_icmp_echo.max_packet_length <= 1500\0" as *const u8
                as *const libc::c_char,
            b"probe_modules/module_icmp_echo.c\0" as *const u8 as *const libc::c_char,
            112 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"int icmp_global_initialize(struct state_conf *)\0"))
                .as_ptr(),
        );
    }
    'c_6789: {
        if module_icmp_echo.max_packet_length <= 1500 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"module_icmp_echo.max_packet_length <= 1500\0" as *const u8
                    as *const libc::c_char,
                b"probe_modules/module_icmp_echo.c\0" as *const u8
                    as *const libc::c_char,
                112 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"int icmp_global_initialize(struct state_conf *)\0"))
                    .as_ptr(),
            );
        }
    };
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn icmp_global_cleanup(
    mut zconf: *mut state_conf,
    mut zsend: *mut state_send,
    mut zrecv: *mut state_recv,
) -> libc::c_int {
    if !icmp_payload.is_null() {
        free(icmp_payload as *mut libc::c_void);
        icmp_payload = 0 as *mut libc::c_char;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn icmp_echo_init_perthread(
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
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
            .wrapping_add(icmp_payload_len) as __uint16_t,
    );
    make_ip_header(ip_header, IPPROTO_ICMP as libc::c_int as uint8_t, len);
    let mut icmp_header: *mut icmp = &mut *ip_header.offset(1 as libc::c_int as isize)
        as *mut ip as *mut icmp;
    make_icmp_header(icmp_header);
    let mut payload: *mut libc::c_char = (icmp_header as *mut libc::c_char)
        .offset(8 as libc::c_int as isize);
    memcpy(
        payload as *mut libc::c_void,
        icmp_payload as *const libc::c_void,
        icmp_payload_len,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn icmp_echo_make_packet(
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
    let mut icmp_header: *mut icmp = &mut *ip_header.offset(1 as libc::c_int as isize)
        as *mut ip as *mut icmp;
    let mut icmp_idnum: uint16_t = (*validation.offset(1 as libc::c_int as isize)
        & 0xffff as libc::c_int as libc::c_uint) as uint16_t;
    let mut icmp_seqnum: uint16_t = (*validation.offset(2 as libc::c_int as isize)
        & 0xffff as libc::c_int as libc::c_uint) as uint16_t;
    (*ip_header).ip_src.s_addr = src_ip;
    (*ip_header).ip_dst.s_addr = dst_ip;
    (*ip_header).ip_ttl = ttl;
    (*icmp_header).icmp_hun.ih_idseq.icd_id = icmp_idnum;
    (*icmp_header).icmp_hun.ih_idseq.icd_seq = icmp_seqnum;
    (*icmp_header).icmp_cksum = 0 as libc::c_int as uint16_t;
    (*icmp_header)
        .icmp_cksum = icmp_checksum(
        icmp_header as *mut libc::c_ushort,
        (8 as libc::c_int as libc::c_ulong).wrapping_add(icmp_payload_len),
    );
    let mut ip_len: size_t = (::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(8 as libc::c_int as libc::c_ulong)
        .wrapping_add(icmp_payload_len);
    (*ip_header).ip_len = __bswap_16(ip_len as __uint16_t);
    (*ip_header).ip_sum = 0 as libc::c_int as libc::c_ushort;
    (*ip_header).ip_sum = zmap_ip_checksum(ip_header as *mut libc::c_ushort);
    *buf_len = ip_len
        .wrapping_add(::std::mem::size_of::<ether_header>() as libc::c_ulong);
    return 0 as libc::c_int;
}
unsafe extern "C" fn icmp_echo_print_packet(
    mut fp: *mut FILE,
    mut packet: *mut libc::c_void,
) {
    let mut ethh: *mut ether_header = packet as *mut ether_header;
    let mut iph: *mut ip = &mut *ethh.offset(1 as libc::c_int as isize)
        as *mut ether_header as *mut ip;
    let mut icmp_header: *mut icmp = &mut *iph.offset(1 as libc::c_int as isize)
        as *mut ip as *mut icmp;
    fprintf(
        fp,
        b"icmp { type: %u | code: %u | checksum: %#04X | id: %u | seq: %u }\n\0"
            as *const u8 as *const libc::c_char,
        (*icmp_header).icmp_type as libc::c_int,
        (*icmp_header).icmp_code as libc::c_int,
        __bswap_16((*icmp_header).icmp_cksum) as libc::c_int,
        __bswap_16((*icmp_header).icmp_hun.ih_idseq.icd_id) as libc::c_int,
        __bswap_16((*icmp_header).icmp_hun.ih_idseq.icd_seq) as libc::c_int,
    );
    fprintf_ip_header(fp, iph);
    fprintf_eth_header(fp, ethh);
    fprintf(
        fp,
        b"------------------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn imcp_validate_id_seq(
    mut icmp_h: *mut icmp,
    mut validation: *mut uint32_t,
) -> libc::c_int {
    if (*icmp_h).icmp_hun.ih_idseq.icd_id as libc::c_uint
        != *validation.offset(1 as libc::c_int as isize)
            & 0xffff as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if (*icmp_h).icmp_hun.ih_idseq.icd_seq as libc::c_uint
        != *validation.offset(2 as libc::c_int as isize)
            & 0xffff as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn icmp_validate_packet(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
    mut src_ip: *mut uint32_t,
    mut validation: *mut uint32_t,
) -> libc::c_int {
    if (*ip_hdr).ip_p as libc::c_int != IPPROTO_ICMP as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut icmp_h: *mut icmp = get_icmp_header(ip_hdr, len);
    if icmp_h.is_null() {
        return 0 as libc::c_int;
    }
    if (*icmp_h).icmp_type as libc::c_int == 0 as libc::c_int {
        return imcp_validate_id_seq(icmp_h, validation)
    } else {
        let mut ip_inner: *mut ip = 0 as *mut ip;
        let mut ip_inner_len: size_t = 0;
        let mut icmp_inner_valid: libc::c_int = icmp_helper_validate(
            ip_hdr,
            len,
            ::std::mem::size_of::<icmp>() as libc::c_ulong,
            &mut ip_inner,
            &mut ip_inner_len,
        );
        if icmp_inner_valid == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        let mut icmp_inner: *mut icmp = get_icmp_header(
            ip_inner,
            ip_inner_len as uint32_t,
        );
        if icmp_inner.is_null() {
            return 0 as libc::c_int;
        }
        validate_gen(
            (*ip_hdr).ip_dst.s_addr,
            (*ip_inner).ip_dst.s_addr,
            validation as *mut uint8_t,
        );
        return imcp_validate_id_seq(icmp_inner, validation);
    };
}
unsafe extern "C" fn icmp_echo_process_packet(
    mut packet: *const u_char,
    mut len: uint32_t,
    mut fs: *mut fieldset_t,
    mut validation: *mut uint32_t,
    mut ts: timespec,
) {
    let mut ip_hdr: *mut ip = &*packet
        .offset(::std::mem::size_of::<ether_header>() as libc::c_ulong as isize)
        as *const u_char as *mut ip;
    let mut icmp_hdr: *mut icmp = (ip_hdr as *mut libc::c_char)
        .offset((4 as libc::c_int * (*ip_hdr).ip_hl() as libc::c_int) as isize)
        as *mut icmp;
    fs_add_uint64(
        fs,
        b"type\0" as *const u8 as *const libc::c_char,
        (*icmp_hdr).icmp_type as uint64_t,
    );
    fs_add_uint64(
        fs,
        b"code\0" as *const u8 as *const libc::c_char,
        (*icmp_hdr).icmp_code as uint64_t,
    );
    fs_add_uint64(
        fs,
        b"icmp_id\0" as *const u8 as *const libc::c_char,
        __bswap_16((*icmp_hdr).icmp_hun.ih_idseq.icd_id) as uint64_t,
    );
    fs_add_uint64(
        fs,
        b"seq\0" as *const u8 as *const libc::c_char,
        __bswap_16((*icmp_hdr).icmp_hun.ih_idseq.icd_seq) as uint64_t,
    );
    let mut hdrlen: uint32_t = (::std::mem::size_of::<ether_header>() as libc::c_ulong)
        .wrapping_add(
            (4 as libc::c_int * (*ip_hdr).ip_hl() as libc::c_int) as libc::c_ulong,
        )
        .wrapping_add(4 as libc::c_int as libc::c_ulong) as uint32_t;
    match (*icmp_hdr).icmp_type as libc::c_int {
        0 => {
            fs_add_string(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"echoreply\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
            );
            fs_add_uint64(
                fs,
                b"success\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as uint64_t,
            );
        }
        3 => {
            fs_add_string(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"unreach\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
            );
            fs_add_bool(
                fs,
                b"success\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        4 => {
            fs_add_string(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"sourcequench\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as libc::c_int,
            );
            fs_add_bool(
                fs,
                b"success\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        5 => {
            fs_add_string(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"redirect\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
            );
            fs_add_bool(
                fs,
                b"success\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        11 => {
            fs_add_string(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"timxceed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
            );
            fs_add_bool(
                fs,
                b"success\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        _ => {
            fs_add_string(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"other\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as libc::c_int,
            );
            fs_add_bool(
                fs,
                b"success\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
    }
    let mut datalen: libc::c_int = len.wrapping_sub(hdrlen) as libc::c_int;
    if datalen > 0 as libc::c_int {
        let mut data: *const uint8_t = &*packet.offset(hdrlen as isize) as *const u_char
            as *mut uint8_t;
        fs_add_binary(
            fs,
            b"data\0" as *const u8 as *const libc::c_char,
            datalen as size_t,
            data as *mut libc::c_void,
            0 as libc::c_int,
        );
    } else {
        fs_add_null(fs, b"data\0" as *const u8 as *const libc::c_char);
    };
}
static mut fields: [fielddef_t; 7] = [
    {
        let mut init = field_def {
            name: b"type\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp message type\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"code\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp message sub type code\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"icmp_id\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp id number\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"seq\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"icmp sequence number\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"classification\0" as *const u8 as *const libc::c_char,
            type_0: b"string\0" as *const u8 as *const libc::c_char,
            desc: b"probe module classification\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"success\0" as *const u8 as *const libc::c_char,
            type_0: b"bool\0" as *const u8 as *const libc::c_char,
            desc: b"did probe module classify response as success\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"data\0" as *const u8 as *const libc::c_char,
            type_0: b"binary\0" as *const u8 as *const libc::c_char,
            desc: b"ICMP payload\0" as *const u8 as *const libc::c_char,
        };
        init
    },
];
pub static mut module_icmp_echo: probe_module_t = unsafe {
    {
        let mut init = probe_module {
            name: b"icmp_echoscan\0" as *const u8 as *const libc::c_char,
            max_packet_length: 48 as libc::c_int as size_t,
            pcap_filter: b"icmp and icmp[0]!=8\0" as *const u8 as *const libc::c_char,
            pcap_snaplen: 96 as libc::c_int as size_t,
            port_args: 0 as libc::c_int as uint8_t,
            global_initialize: Some(
                icmp_global_initialize
                    as unsafe extern "C" fn(*mut state_conf) -> libc::c_int,
            ),
            thread_initialize: Some(
                icmp_echo_init_perthread
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut macaddr_t,
                        *mut macaddr_t,
                        port_h_t,
                        *mut *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            make_packet: Some(
                icmp_echo_make_packet
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
                icmp_echo_print_packet
                    as unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> (),
            ),
            validate_packet: Some(
                icmp_validate_packet
                    as unsafe extern "C" fn(
                        *const ip,
                        uint32_t,
                        *mut uint32_t,
                        *mut uint32_t,
                    ) -> libc::c_int,
            ),
            process_packet: Some(
                icmp_echo_process_packet
                    as unsafe extern "C" fn(
                        *const u_char,
                        uint32_t,
                        *mut fieldset_t,
                        *mut uint32_t,
                        timespec,
                    ) -> (),
            ),
            close: Some(
                icmp_global_cleanup
                    as unsafe extern "C" fn(
                        *mut state_conf,
                        *mut state_send,
                        *mut state_recv,
                    ) -> libc::c_int,
            ),
            output_type: 1 as libc::c_int,
            fields: fields.as_ptr() as *mut _,
            numfields: 7 as libc::c_int,
            helptext: b"Probe module that sends ICMP echo requests to hosts.\nPayload of ICMP packets will consist of zeroes unless you customize it with\n --probe-args=file:/path_to_payload_file\n --probe-args=text:SomeText\n --probe-args=hex:5061796c6f6164\0"
                as *const u8 as *const libc::c_char,
        };
        init
    }
};
