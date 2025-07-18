use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type aesrand;
    pub type output_module;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn fs_add_constchar(
        fs: *mut fieldset_t,
        name: *const libc::c_char,
        value: *const libc::c_char,
    );
    fn fs_add_null(fs: *mut fieldset_t, name: *const libc::c_char);
    fn fs_add_uint64(fs: *mut fieldset_t, name: *const libc::c_char, value: uint64_t);
    fn fs_add_bool(fs: *mut fieldset_t, name: *const libc::c_char, value: libc::c_int);
    static mut zconf: state_conf;
    fn make_eth_header(
        ethh: *mut ether_header,
        src: *mut macaddr_t,
        dst: *mut macaddr_t,
    );
    fn make_ip_header(iph: *mut ip, _: uint8_t, _: uint16_t);
    fn make_tcp_header(_: *mut tcphdr, _: port_h_t, _: uint16_t);
    fn set_mss_option(tcp_header: *mut tcphdr) -> size_t;
    fn icmp_helper_validate(
        ip_hdr: *const ip,
        len: uint32_t,
        min_l4_len: size_t,
        probe_pkt: *mut *mut ip,
        probe_len: *mut size_t,
    ) -> libc::c_int;
    fn fs_add_null_icmp(fs: *mut fieldset_t);
    fn fs_populate_icmp_from_iphdr(ip: *mut ip, len: size_t, fs: *mut fieldset_t);
    fn blocklist_is_allowed(s_addr: uint32_t) -> libc::c_int;
    fn validate_gen(src: uint32_t, dst: uint32_t, output: *mut uint8_t);
    fn synscan_print_packet(fp: *mut FILE, packet: *mut libc::c_void);
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
pub type tcp_seq = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tcphdr {
    pub c2rust_unnamed: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub c2rust_unnamed_0: C2RustUnnamed_4,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub source: uint16_t,
    pub dest: uint16_t,
    pub seq: uint32_t,
    pub ack_seq: uint32_t,
    #[bitfield(name = "res1", ty = "uint16_t", bits = "0..=3")]
    #[bitfield(name = "doff", ty = "uint16_t", bits = "4..=7")]
    #[bitfield(name = "fin", ty = "uint16_t", bits = "8..=8")]
    #[bitfield(name = "syn", ty = "uint16_t", bits = "9..=9")]
    #[bitfield(name = "rst", ty = "uint16_t", bits = "10..=10")]
    #[bitfield(name = "psh", ty = "uint16_t", bits = "11..=11")]
    #[bitfield(name = "ack", ty = "uint16_t", bits = "12..=12")]
    #[bitfield(name = "urg", ty = "uint16_t", bits = "13..=13")]
    #[bitfield(name = "res2", ty = "uint16_t", bits = "14..=15")]
    pub res1_doff_fin_syn_rst_psh_ack_urg_res2: [u8; 2],
    pub window: uint16_t,
    pub check: uint16_t,
    pub urg_ptr: uint16_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub th_sport: uint16_t,
    pub th_dport: uint16_t,
    pub th_seq: tcp_seq,
    pub th_ack: tcp_seq,
    #[bitfield(name = "th_x2", ty = "uint8_t", bits = "0..=3")]
    #[bitfield(name = "th_off", ty = "uint8_t", bits = "4..=7")]
    pub th_x2_th_off: [u8; 1],
    pub th_flags: uint8_t,
    pub th_win: uint16_t,
    pub th_sum: uint16_t,
    pub th_urp: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ether_header {
    pub ether_dhost: [uint8_t; 6],
    pub ether_shost: [uint8_t; 6],
    pub ether_type: uint16_t,
}
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
pub type aesrand_t = aesrand;
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
pub type alias_unsigned_short = libc::c_ushort;
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
unsafe extern "C" fn get_ip_header(
    mut packet: *const u_char,
    mut len: uint32_t,
) -> *mut ip {
    if (len as libc::c_ulong) < ::std::mem::size_of::<ether_header>() as libc::c_ulong {
        return 0 as *mut ip;
    }
    return &*packet
        .offset(::std::mem::size_of::<ether_header>() as libc::c_ulong as isize)
        as *const u_char as *mut ip;
}
#[inline]
unsafe extern "C" fn get_tcp_header(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
) -> *mut tcphdr {
    if ((4 as libc::c_int * (*ip_hdr).ip_hl() as libc::c_int) as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<tcphdr>() as libc::c_ulong)
        > len as libc::c_ulong
    {
        return 0 as *mut tcphdr;
    }
    return (ip_hdr as *mut libc::c_char)
        .offset((4 as libc::c_int * (*ip_hdr).ip_hl() as libc::c_int) as isize)
        as *mut tcphdr;
}
#[inline]
unsafe extern "C" fn tcp_checksum(
    mut len_tcp: libc::c_ushort,
    mut saddr: uint32_t,
    mut daddr: uint32_t,
    mut tcp_pkt: *mut tcphdr,
) -> uint16_t {
    let mut src_addr: *mut alias_unsigned_short = &mut saddr as *mut uint32_t
        as *mut alias_unsigned_short;
    let mut dest_addr: *mut alias_unsigned_short = &mut daddr as *mut uint32_t
        as *mut alias_unsigned_short;
    let mut prot_tcp: libc::c_uchar = 6 as libc::c_int as libc::c_uchar;
    let mut sum: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut nleft: libc::c_int = len_tcp as libc::c_int;
    let mut w: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    w = tcp_pkt as *mut libc::c_ushort;
    while nleft > 1 as libc::c_int {
        let fresh1 = w;
        w = w.offset(1);
        sum = sum.wrapping_add(*fresh1 as libc::c_ulong);
        nleft -= 2 as libc::c_int;
    }
    if nleft > 0 as libc::c_int {
        sum = sum
            .wrapping_add(
                (*w as libc::c_int
                    & __bswap_16(0xff00 as libc::c_int as __uint16_t) as libc::c_int)
                    as libc::c_ulong,
            );
    }
    sum = sum.wrapping_add(*src_addr.offset(0 as libc::c_int as isize) as libc::c_ulong);
    sum = sum.wrapping_add(*src_addr.offset(1 as libc::c_int as isize) as libc::c_ulong);
    sum = sum
        .wrapping_add(*dest_addr.offset(0 as libc::c_int as isize) as libc::c_ulong);
    sum = sum
        .wrapping_add(*dest_addr.offset(1 as libc::c_int as isize) as libc::c_ulong);
    sum = sum.wrapping_add(__bswap_16(len_tcp) as libc::c_ulong);
    sum = sum.wrapping_add(__bswap_16(prot_tcp as __uint16_t) as libc::c_ulong);
    sum = (sum >> 16 as libc::c_int)
        .wrapping_add(sum & 0xffff as libc::c_int as libc::c_ulong);
    sum = sum.wrapping_add(sum >> 16 as libc::c_int);
    return !sum as libc::c_ushort;
}
static mut num_ports: uint32_t = 0;
unsafe extern "C" fn synackscan_global_initialize(
    mut state: *mut state_conf,
) -> libc::c_int {
    num_ports = ((*state).source_port_last as libc::c_int
        - (*state).source_port_first as libc::c_int + 1 as libc::c_int) as uint32_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn synackscan_init_perthread(
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
            .wrapping_add(24 as libc::c_int as libc::c_ulong) as __uint16_t,
    );
    make_ip_header(ip_header, IPPROTO_TCP as libc::c_int as uint8_t, len);
    let mut tcp_header: *mut tcphdr = &mut *ip_header.offset(1 as libc::c_int as isize)
        as *mut ip as *mut tcphdr;
    make_tcp_header(
        tcp_header,
        dst_port,
        (0x2 as libc::c_int | 0x10 as libc::c_int) as uint16_t,
    );
    set_mss_option(tcp_header);
    return 0 as libc::c_int;
}
unsafe extern "C" fn synackscan_make_packet(
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
    let mut tcp_header: *mut tcphdr = &mut *ip_header.offset(1 as libc::c_int as isize)
        as *mut ip as *mut tcphdr;
    let mut tcp_seq: uint32_t = *validation.offset(0 as libc::c_int as isize);
    let mut tcp_ack: uint32_t = *validation.offset(2 as libc::c_int as isize);
    (*ip_header).ip_src.s_addr = src_ip;
    (*ip_header).ip_dst.s_addr = dst_ip;
    (*ip_header).ip_ttl = ttl;
    (*tcp_header)
        .c2rust_unnamed
        .c2rust_unnamed
        .th_sport = __bswap_16(
        get_src_port(num_ports as libc::c_int, probe_num, validation),
    );
    (*tcp_header).c2rust_unnamed.c2rust_unnamed.th_seq = tcp_seq;
    (*tcp_header).c2rust_unnamed.c2rust_unnamed.th_ack = tcp_ack;
    (*tcp_header).c2rust_unnamed.c2rust_unnamed.th_sum = 0 as libc::c_int as uint16_t;
    (*tcp_header)
        .c2rust_unnamed
        .c2rust_unnamed
        .th_sum = tcp_checksum(
        24 as libc::c_int as libc::c_ushort,
        (*ip_header).ip_src.s_addr,
        (*ip_header).ip_dst.s_addr,
        tcp_header,
    );
    (*ip_header).ip_sum = 0 as libc::c_int as libc::c_ushort;
    (*ip_header).ip_sum = zmap_ip_checksum(ip_header as *mut libc::c_ushort);
    *buf_len = 58 as libc::c_int as size_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn synackscan_validate_packet(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
    mut src_ip: *mut uint32_t,
    mut validation: *mut uint32_t,
) -> libc::c_int {
    if (*ip_hdr).ip_p as libc::c_int == IPPROTO_TCP as libc::c_int {
        let mut tcp: *mut tcphdr = get_tcp_header(ip_hdr, len);
        if tcp.is_null() {
            return 0 as libc::c_int;
        }
        let mut sport: uint16_t = __bswap_16(
            (*tcp).c2rust_unnamed.c2rust_unnamed.th_sport,
        );
        let mut dport: uint16_t = __bswap_16(
            (*tcp).c2rust_unnamed.c2rust_unnamed.th_dport,
        );
        if sport as libc::c_int != zconf.target_port as libc::c_int {
            return 0 as libc::c_int;
        }
        if check_dst_port(dport, num_ports as libc::c_int, validation) == 0 {
            return 0 as libc::c_int;
        }
        if blocklist_is_allowed(*src_ip) == 0 {
            return 0 as libc::c_int;
        }
        if (*tcp).c2rust_unnamed.c2rust_unnamed.th_flags as libc::c_int
            & 0x4 as libc::c_int != 0
        {
            if __bswap_32((*tcp).c2rust_unnamed.c2rust_unnamed.th_ack)
                != (__bswap_32(*validation.offset(0 as libc::c_int as isize)))
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                && __bswap_32((*tcp).c2rust_unnamed.c2rust_unnamed.th_seq)
                    != __bswap_32(*validation.offset(2 as libc::c_int as isize))
                && __bswap_32((*tcp).c2rust_unnamed.c2rust_unnamed.th_seq)
                    != (__bswap_32(*validation.offset(2 as libc::c_int as isize)))
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
            {
                return 0 as libc::c_int;
            }
        } else if __bswap_32((*tcp).c2rust_unnamed.c2rust_unnamed.th_ack)
            != (__bswap_32(*validation.offset(0 as libc::c_int as isize)))
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        {
            return 0 as libc::c_int
        }
    } else if (*ip_hdr).ip_p as libc::c_int == IPPROTO_ICMP as libc::c_int {
        let mut ip_inner: *mut ip = 0 as *mut ip;
        let mut ip_inner_len: size_t = 0;
        if icmp_helper_validate(
            ip_hdr,
            len,
            ::std::mem::size_of::<udphdr>() as libc::c_ulong,
            &mut ip_inner,
            &mut ip_inner_len,
        ) == 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        let mut tcp_0: *mut tcphdr = get_tcp_header(ip_inner, ip_inner_len as uint32_t);
        if tcp_0.is_null() {
            return 0 as libc::c_int;
        }
        let mut sport_0: uint16_t = __bswap_16(
            (*tcp_0).c2rust_unnamed.c2rust_unnamed.th_sport,
        );
        let mut dport_0: uint16_t = __bswap_16(
            (*tcp_0).c2rust_unnamed.c2rust_unnamed.th_dport,
        );
        if dport_0 as libc::c_int != zconf.target_port as libc::c_int {
            return 0 as libc::c_int;
        }
        validate_gen(
            (*ip_hdr).ip_dst.s_addr,
            (*ip_inner).ip_dst.s_addr,
            validation as *mut uint8_t,
        );
        if check_dst_port(sport_0, num_ports as libc::c_int, validation) == 0 {
            return 0 as libc::c_int;
        }
    } else {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn synackscan_process_packet(
    mut packet: *const u_char,
    mut len: uint32_t,
    mut fs: *mut fieldset_t,
    mut validation: *mut uint32_t,
    mut ts: timespec,
) {
    let mut ip_hdr: *mut ip = get_ip_header(packet, len);
    if (*ip_hdr).ip_p as libc::c_int == IPPROTO_TCP as libc::c_int {
        let mut tcp: *mut tcphdr = get_tcp_header(ip_hdr, len);
        fs_add_uint64(
            fs,
            b"sport\0" as *const u8 as *const libc::c_char,
            __bswap_16((*tcp).c2rust_unnamed.c2rust_unnamed.th_sport) as uint64_t,
        );
        fs_add_uint64(
            fs,
            b"dport\0" as *const u8 as *const libc::c_char,
            __bswap_16((*tcp).c2rust_unnamed.c2rust_unnamed.th_dport) as uint64_t,
        );
        fs_add_uint64(
            fs,
            b"seqnum\0" as *const u8 as *const libc::c_char,
            __bswap_32((*tcp).c2rust_unnamed.c2rust_unnamed.th_seq) as uint64_t,
        );
        fs_add_uint64(
            fs,
            b"acknum\0" as *const u8 as *const libc::c_char,
            __bswap_32((*tcp).c2rust_unnamed.c2rust_unnamed.th_ack) as uint64_t,
        );
        fs_add_uint64(
            fs,
            b"window\0" as *const u8 as *const libc::c_char,
            __bswap_16((*tcp).c2rust_unnamed.c2rust_unnamed.th_win) as uint64_t,
        );
        if (*tcp).c2rust_unnamed.c2rust_unnamed.th_flags as libc::c_int
            & 0x4 as libc::c_int != 0
        {
            fs_add_constchar(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"rst\0" as *const u8 as *const libc::c_char,
            );
        } else {
            fs_add_constchar(
                fs,
                b"classification\0" as *const u8 as *const libc::c_char,
                b"synack\0" as *const u8 as *const libc::c_char,
            );
        }
        fs_add_bool(
            fs,
            b"success\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
        fs_add_null_icmp(fs);
    } else if (*ip_hdr).ip_p as libc::c_int == IPPROTO_ICMP as libc::c_int {
        fs_add_null(fs, b"sport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"dport\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"seqnum\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"acknum\0" as *const u8 as *const libc::c_char);
        fs_add_null(fs, b"window\0" as *const u8 as *const libc::c_char);
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
        fs_populate_icmp_from_iphdr(ip_hdr, len as size_t, fs);
    }
}
static mut fields: [fielddef_t; 11] = [
    {
        let mut init = field_def {
            name: b"sport\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"TCP source port\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"dport\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"TCP destination port\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"seqnum\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"TCP sequence number\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"acknum\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"TCP acknowledgement number\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = field_def {
            name: b"window\0" as *const u8 as *const libc::c_char,
            type_0: b"int\0" as *const u8 as *const libc::c_char,
            desc: b"TCP window\0" as *const u8 as *const libc::c_char,
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
pub static mut module_tcp_synackscan: probe_module_t = probe_module_t {
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
    module_tcp_synackscan = {
        let mut init = probe_module {
            name: b"tcp_synackscan\0" as *const u8 as *const libc::c_char,
            max_packet_length: 58 as libc::c_int as size_t,
            pcap_filter: b"(tcp && tcp[13] & 4 != 0 || tcp[13] == 18) || icmp\0"
                as *const u8 as *const libc::c_char,
            pcap_snaplen: 96 as libc::c_int as size_t,
            port_args: 1 as libc::c_int as uint8_t,
            global_initialize: Some(
                synackscan_global_initialize
                    as unsafe extern "C" fn(*mut state_conf) -> libc::c_int,
            ),
            thread_initialize: Some(
                synackscan_init_perthread
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut macaddr_t,
                        *mut macaddr_t,
                        port_h_t,
                        *mut *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            make_packet: Some(
                synackscan_make_packet
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
                synscan_print_packet
                    as unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> (),
            ),
            validate_packet: Some(
                synackscan_validate_packet
                    as unsafe extern "C" fn(
                        *const ip,
                        uint32_t,
                        *mut uint32_t,
                        *mut uint32_t,
                    ) -> libc::c_int,
            ),
            process_packet: Some(
                synackscan_process_packet
                    as unsafe extern "C" fn(
                        *const u_char,
                        uint32_t,
                        *mut fieldset_t,
                        *mut uint32_t,
                        timespec,
                    ) -> (),
            ),
            close: None,
            output_type: 1 as libc::c_int,
            fields: fields.as_mut_ptr(),
            numfields: (::std::mem::size_of::<[fielddef_t; 11]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<fielddef_t>() as libc::c_ulong)
                as libc::c_int,
            helptext: b"Probe module that sends a TCP SYNACK packet to a specific port. Possible classifications are: synack and rst. A SYN-ACK packet is considered a failure and a reset packet is considered a success.\0"
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
