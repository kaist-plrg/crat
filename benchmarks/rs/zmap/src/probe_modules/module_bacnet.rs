use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type aesrand;
    pub type output_module;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut zconf: state_conf;
    fn aesrand_init_from_seed(_: uint64_t) -> *mut aesrand_t;
    fn aesrand_getword(aes: *mut aesrand_t) -> uint64_t;
    fn fs_add_null(fs: *mut fieldset_t, name: *const libc::c_char);
    fn fs_add_uint64(fs: *mut fieldset_t, name: *const libc::c_char, value: uint64_t);
    fn fs_add_bool(fs: *mut fieldset_t, name: *const libc::c_char, value: libc::c_int);
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
    fn make_udp_header(udp_header: *mut udphdr, dest_port: port_h_t, len: uint16_t);
    fn fs_populate_icmp_from_iphdr(ip: *mut ip, len: size_t, fs: *mut fieldset_t);
    fn fs_add_null_icmp(fs: *mut fieldset_t);
    fn make_eth_header(
        ethh: *mut ether_header,
        src: *mut macaddr_t,
        dst: *mut macaddr_t,
    );
    fn make_ip_header(iph: *mut ip, _: uint8_t, _: uint16_t);
    fn udp_print_packet(fp: *mut FILE, packet: *mut libc::c_void);
    fn udp_do_validate_packet(
        ip_hdr: *const ip,
        len: uint32_t,
        src_ip: *mut uint32_t,
        validation: *mut uint32_t,
        num_ports_0: libc::c_int,
        expected_port: libc::c_int,
    ) -> libc::c_int;
    fn udp_global_cleanup(
        zconf_0: *mut state_conf,
        zsend: *mut state_send,
        zrecv: *mut state_recv,
    ) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct bacnet_vlc {
    pub type_0: uint8_t,
    pub function: uint8_t,
    pub length: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct bacnet_apdu {
    pub type_flags: uint8_t,
    pub max_segments_apdu: uint8_t,
    pub invoke_id: uint8_t,
    pub server_choice: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct bacnet_probe {
    pub vlc: bacnet_vlc,
    pub npdu: bacnet_npdu,
    pub apdu: bacnet_apdu,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct bacnet_npdu {
    pub version: uint8_t,
    pub control: uint8_t,
}
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
#[inline]
unsafe extern "C" fn get_udp_payload(
    mut udp: *const udphdr,
    mut len: uint32_t,
) -> *mut uint8_t {
    return &*udp.offset(1 as libc::c_int as isize) as *const udphdr as *mut uint8_t;
}
pub static mut module_bacnet: probe_module_t = probe_module_t {
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
static mut num_ports: libc::c_int = 0;
static mut bacnet_body: [uint8_t; 7] = [
    0xc as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x3f as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0x19 as libc::c_int as uint8_t,
    0x4b as libc::c_int as uint8_t,
];
#[inline]
unsafe extern "C" fn get_invoke_id(mut validation: *mut uint32_t) -> uint8_t {
    return (*validation.offset(1 as libc::c_int as isize) >> 24 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as uint8_t;
}
pub unsafe extern "C" fn bacnet_init_perthread(
    mut buf: *mut libc::c_void,
    mut src: *mut macaddr_t,
    mut gw: *mut macaddr_t,
    mut dst_port: port_h_t,
    mut arg: *mut *mut libc::c_void,
) -> libc::c_int {
    memset(buf, 0 as libc::c_int, 4096 as libc::c_int as libc::c_ulong);
    let mut eth_header: *mut ether_header = buf as *mut ether_header;
    let mut ip_header: *mut ip = &mut *eth_header.offset(1 as libc::c_int as isize)
        as *mut ether_header as *mut ip;
    let mut udp_header: *mut udphdr = &mut *ip_header.offset(1 as libc::c_int as isize)
        as *mut ip as *mut udphdr;
    let mut bnp: *mut bacnet_probe = &mut *udp_header.offset(1 as libc::c_int as isize)
        as *mut udphdr as *mut bacnet_probe;
    let mut body: *mut uint8_t = &mut *bnp.offset(1 as libc::c_int as isize)
        as *mut bacnet_probe as *mut uint8_t;
    make_eth_header(eth_header, src, gw);
    let mut ip_len: uint16_t = (::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
        .wrapping_add(0x11 as libc::c_int as libc::c_ulong) as uint16_t;
    if ip_len as libc::c_int <= 4096 as libc::c_int {} else {
        __assert_fail(
            b"ip_len <= MAX_PACKET_SIZE\0" as *const u8 as *const libc::c_char,
            b"probe_modules/module_bacnet.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"int bacnet_init_perthread(void *, macaddr_t *, macaddr_t *, port_h_t, void **)\0",
            ))
                .as_ptr(),
        );
    }
    'c_6534: {
        if ip_len as libc::c_int <= 4096 as libc::c_int {} else {
            __assert_fail(
                b"ip_len <= MAX_PACKET_SIZE\0" as *const u8 as *const libc::c_char,
                b"probe_modules/module_bacnet.c\0" as *const u8 as *const libc::c_char,
                51 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 79],
                    &[libc::c_char; 79],
                >(
                    b"int bacnet_init_perthread(void *, macaddr_t *, macaddr_t *, port_h_t, void **)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    make_ip_header(ip_header, IPPROTO_UDP as libc::c_int as uint8_t, __bswap_16(ip_len));
    let mut udp_len: uint16_t = (::std::mem::size_of::<udphdr>() as libc::c_ulong)
        .wrapping_add(0x11 as libc::c_int as libc::c_ulong) as uint16_t;
    make_udp_header(udp_header, zconf.target_port, udp_len);
    (*bnp).vlc.type_0 = 0x81 as libc::c_int as uint8_t;
    (*bnp).vlc.function = 0xa as libc::c_int as uint8_t;
    (*bnp).vlc.length = __bswap_16(0x11 as libc::c_int as __uint16_t);
    (*bnp).npdu.version = 0x1 as libc::c_int as uint8_t;
    (*bnp).npdu.control = 0x4 as libc::c_int as uint8_t;
    (*bnp).apdu.type_flags = 0 as libc::c_int as uint8_t;
    (*bnp).apdu.max_segments_apdu = 0x5 as libc::c_int as uint8_t;
    (*bnp).apdu.server_choice = 0xc as libc::c_int as uint8_t;
    memcpy(
        body as *mut libc::c_void,
        bacnet_body.as_mut_ptr() as *const libc::c_void,
        7 as libc::c_int as libc::c_ulong,
    );
    let mut seed: uint32_t = aesrand_getword(zconf.aes) as uint32_t;
    let mut aes: *mut aesrand_t = aesrand_init_from_seed(seed as uint64_t);
    *arg = aes as *mut libc::c_void;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn bacnet_make_packet(
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
    let mut bnp: *mut bacnet_probe = &mut *udp_header.offset(1 as libc::c_int as isize)
        as *mut udphdr as *mut bacnet_probe;
    (*ip_header).ip_src.s_addr = src_ip;
    (*ip_header).ip_dst.s_addr = dst_ip;
    (*ip_header).ip_ttl = ttl;
    (*ip_header).ip_sum = 0 as libc::c_int as libc::c_ushort;
    (*udp_header)
        .c2rust_unnamed
        .c2rust_unnamed
        .uh_sport = __bswap_16(get_src_port(num_ports, probe_num, validation));
    (*bnp).apdu.invoke_id = get_invoke_id(validation);
    (*ip_header).ip_sum = zmap_ip_checksum(ip_header as *mut libc::c_ushort);
    *buf_len = (::std::mem::size_of::<ether_header>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
        .wrapping_add(0x11 as libc::c_int as libc::c_ulong);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn bacnet_validate_packet(
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
        let min_len: size_t = (::std::mem::size_of::<udphdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<bacnet_vlc>() as libc::c_ulong);
        if ((*udp).c2rust_unnamed.c2rust_unnamed.uh_ulen as libc::c_ulong) < min_len {
            return 0 as libc::c_int;
        }
        let mut vlc: *mut bacnet_vlc = get_udp_payload(udp, len) as *mut bacnet_vlc;
        if (*vlc).type_0 as libc::c_int != 0x81 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn bacnet_process_packet(
    mut packet: *const u_char,
    mut len: uint32_t,
    mut fs: *mut fieldset_t,
    mut validation: *mut uint32_t,
    mut ts: timespec,
) {
    let mut ip_hdr: *mut ip = get_ip_header(packet, len);
    if !ip_hdr.is_null() {} else {
        __assert_fail(
            b"ip_hdr\0" as *const u8 as *const libc::c_char,
            b"probe_modules/module_bacnet.c\0" as *const u8 as *const libc::c_char,
            134 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"void bacnet_process_packet(const u_char *, uint32_t, fieldset_t *, uint32_t *, struct timespec)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5960: {
        if !ip_hdr.is_null() {} else {
            __assert_fail(
                b"ip_hdr\0" as *const u8 as *const libc::c_char,
                b"probe_modules/module_bacnet.c\0" as *const u8 as *const libc::c_char,
                134 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 96],
                    &[libc::c_char; 96],
                >(
                    b"void bacnet_process_packet(const u_char *, uint32_t, fieldset_t *, uint32_t *, struct timespec)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*ip_hdr).ip_p as libc::c_int == IPPROTO_UDP as libc::c_int {
        let mut udp: *mut udphdr = get_udp_header(ip_hdr, len);
        if !udp.is_null() {} else {
            __assert_fail(
                b"udp\0" as *const u8 as *const libc::c_char,
                b"probe_modules/module_bacnet.c\0" as *const u8 as *const libc::c_char,
                137 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 96],
                    &[libc::c_char; 96],
                >(
                    b"void bacnet_process_packet(const u_char *, uint32_t, fieldset_t *, uint32_t *, struct timespec)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_5921: {
            if !udp.is_null() {} else {
                __assert_fail(
                    b"udp\0" as *const u8 as *const libc::c_char,
                    b"probe_modules/module_bacnet.c\0" as *const u8
                        as *const libc::c_char,
                    137 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 96],
                        &[libc::c_char; 96],
                    >(
                        b"void bacnet_process_packet(const u_char *, uint32_t, fieldset_t *, uint32_t *, struct timespec)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
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
        fs_add_constchar(
            fs,
            b"classification\0" as *const u8 as *const libc::c_char,
            b"bacnet\0" as *const u8 as *const libc::c_char,
        );
        fs_add_bool(
            fs,
            b"success\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
        fs_add_null_icmp(fs);
        let mut udp_offset: uint32_t = (::std::mem::size_of::<ether_header>()
            as libc::c_ulong)
            .wrapping_add(
                ((*ip_hdr).ip_hl() as libc::c_int * 4 as libc::c_int) as libc::c_ulong,
            ) as uint32_t;
        let mut payload_offset: uint32_t = (udp_offset as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong) as uint32_t;
        if payload_offset < len {} else {
            __assert_fail(
                b"payload_offset < len\0" as *const u8 as *const libc::c_char,
                b"probe_modules/module_bacnet.c\0" as *const u8 as *const libc::c_char,
                145 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 96],
                    &[libc::c_char; 96],
                >(
                    b"void bacnet_process_packet(const u_char *, uint32_t, fieldset_t *, uint32_t *, struct timespec)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_5822: {
            if payload_offset < len {} else {
                __assert_fail(
                    b"payload_offset < len\0" as *const u8 as *const libc::c_char,
                    b"probe_modules/module_bacnet.c\0" as *const u8
                        as *const libc::c_char,
                    145 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 96],
                        &[libc::c_char; 96],
                    >(
                        b"void bacnet_process_packet(const u_char *, uint32_t, fieldset_t *, uint32_t *, struct timespec)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        let mut payload: *mut uint8_t = get_udp_payload(udp, len);
        let mut payload_len: uint32_t = len.wrapping_sub(payload_offset);
        fs_add_binary(
            fs,
            b"udp_payload\0" as *const u8 as *const libc::c_char,
            payload_len as size_t,
            payload as *mut libc::c_void,
            0 as libc::c_int,
        );
        fs_add_null_icmp(fs);
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
        fs_add_null(fs, b"udp_payload\0" as *const u8 as *const libc::c_char);
        fs_populate_icmp_from_iphdr(ip_hdr, len as size_t, fs);
    }
}
pub unsafe extern "C" fn bacnet_global_initialize(
    mut conf: *mut state_conf,
) -> libc::c_int {
    num_ports = (*conf).source_port_last as libc::c_int
        - (*conf).source_port_first as libc::c_int + 1 as libc::c_int;
    return 0 as libc::c_int;
}
static mut fields: [fielddef_t; 9] = [
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
            name: b"udp_payload\0" as *const u8 as *const libc::c_char,
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
unsafe extern "C" fn run_static_initializers() {
    module_bacnet = {
        let mut init = probe_module {
            name: b"bacnet\0" as *const u8 as *const libc::c_char,
            max_packet_length: (::std::mem::size_of::<ether_header>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<udphdr>() as libc::c_ulong)
                .wrapping_add(0x11 as libc::c_int as libc::c_ulong),
            pcap_filter: b"udp || icmp\0" as *const u8 as *const libc::c_char,
            pcap_snaplen: 1500 as libc::c_int as size_t,
            port_args: 1 as libc::c_int as uint8_t,
            global_initialize: Some(
                bacnet_global_initialize
                    as unsafe extern "C" fn(*mut state_conf) -> libc::c_int,
            ),
            thread_initialize: Some(
                bacnet_init_perthread
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut macaddr_t,
                        *mut macaddr_t,
                        port_h_t,
                        *mut *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            make_packet: Some(
                bacnet_make_packet
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
                bacnet_validate_packet
                    as unsafe extern "C" fn(
                        *const ip,
                        uint32_t,
                        *mut uint32_t,
                        *mut uint32_t,
                    ) -> libc::c_int,
            ),
            process_packet: Some(
                bacnet_process_packet
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
            output_type: 1 as libc::c_int,
            fields: fields.as_mut_ptr(),
            numfields: (::std::mem::size_of::<[fielddef_t; 9]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<fielddef_t>() as libc::c_ulong)
                as libc::c_int,
            helptext: 0 as *const libc::c_char,
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
