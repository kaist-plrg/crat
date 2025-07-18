use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type aesrand;
    pub type probe_module;
    pub type output_module;
    fn random() -> libc::c_long;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    fn fs_add_null(fs: *mut fieldset_t, name: *const libc::c_char);
    fn fs_add_uint64(fs: *mut fieldset_t, name: *const libc::c_char, value: uint64_t);
    fn fs_add_string(
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
    fn fs_modify_string(
        fs: *mut fieldset_t,
        name: *const libc::c_char,
        value: *mut libc::c_char,
        free_: libc::c_int,
    );
    static mut zconf: state_conf;
    fn blocklist_is_allowed(s_addr: uint32_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __caddr_t = *mut libc::c_char;
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
#[repr(C)]
pub struct udphdr {
    pub c2rust_unnamed: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub c2rust_unnamed: C2RustUnnamed_6,
    pub c2rust_unnamed_0: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub source: uint16_t,
    pub dest: uint16_t,
    pub len: uint16_t,
    pub check: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub uh_sport: uint16_t,
    pub uh_dport: uint16_t,
    pub uh_ulen: uint16_t,
    pub uh_sum: uint16_t,
}
pub type tcp_seq = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tcphdr {
    pub c2rust_unnamed: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub c2rust_unnamed: C2RustUnnamed_9,
    pub c2rust_unnamed_0: C2RustUnnamed_8,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
pub struct C2RustUnnamed_9 {
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
    pub ifr_ifrn: C2RustUnnamed_11,
    pub ifr_ifru: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
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
pub union C2RustUnnamed_11 {
    pub ifrn_name: [libc::c_char; 16],
}
pub type aesrand_t = aesrand;
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
pub struct fieldset_conf {
    pub defs: fielddefset_t,
    pub outdefs: fielddefset_t,
    pub translation: translation_t,
    pub success_index: libc::c_int,
    pub app_success_index: libc::c_int,
    pub classification_index: libc::c_int,
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
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
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
#[inline]
unsafe extern "C" fn get_inner_ip_header(
    mut icmp: *const icmp,
    mut len: uint32_t,
) -> *mut ip {
    if (len as libc::c_ulong)
        < (8 as libc::c_int as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
    {
        return 0 as *mut ip;
    }
    return (icmp as *mut libc::c_char).offset(8 as libc::c_int as isize) as *mut ip;
}
pub unsafe extern "C" fn print_macaddr(mut i: *mut ifreq) {
    printf(
        b"Device %s -> Ethernet %02x:%02x:%02x:%02x:%02x:%02x\n\0" as *const u8
            as *const libc::c_char,
        ((*i).ifr_ifrn.ifrn_name).as_mut_ptr(),
        *(&mut (*i).ifr_ifru.ifru_addr.sa_data as *mut [libc::c_char; 14]
            as *mut libc::c_uchar)
            .offset(0 as libc::c_int as isize) as libc::c_int,
        *(&mut (*i).ifr_ifru.ifru_addr.sa_data as *mut [libc::c_char; 14]
            as *mut libc::c_uchar)
            .offset(1 as libc::c_int as isize) as libc::c_int,
        *(&mut (*i).ifr_ifru.ifru_addr.sa_data as *mut [libc::c_char; 14]
            as *mut libc::c_uchar)
            .offset(2 as libc::c_int as isize) as libc::c_int,
        *(&mut (*i).ifr_ifru.ifru_addr.sa_data as *mut [libc::c_char; 14]
            as *mut libc::c_uchar)
            .offset(3 as libc::c_int as isize) as libc::c_int,
        *(&mut (*i).ifr_ifru.ifru_addr.sa_data as *mut [libc::c_char; 14]
            as *mut libc::c_uchar)
            .offset(4 as libc::c_int as isize) as libc::c_int,
        *(&mut (*i).ifr_ifru.ifru_addr.sa_data as *mut [libc::c_char; 14]
            as *mut libc::c_uchar)
            .offset(5 as libc::c_int as isize) as libc::c_int,
    );
}
pub unsafe extern "C" fn fprintf_ip_header(mut fp: *mut FILE, mut iph: *mut ip) {
    let mut s: *mut in_addr = &mut (*iph).ip_src as *mut in_addr;
    let mut d: *mut in_addr = &mut (*iph).ip_dst as *mut in_addr;
    let mut srcip: [libc::c_char; 21] = [0; 21];
    let mut dstip: [libc::c_char; 21] = [0; 21];
    strncpy(
        srcip.as_mut_ptr(),
        inet_ntoa(*s),
        (20 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
    );
    strncpy(
        dstip.as_mut_ptr(),
        inet_ntoa(*d),
        (20 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
    );
    srcip[20 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    dstip[20 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    fprintf(
        fp,
        b"ip { saddr: %s | daddr: %s | checksum: %#04X }\n\0" as *const u8
            as *const libc::c_char,
        srcip.as_mut_ptr(),
        dstip.as_mut_ptr(),
        __bswap_16((*iph).ip_sum) as libc::c_int,
    );
}
pub unsafe extern "C" fn fprintf_eth_header(
    mut fp: *mut FILE,
    mut ethh: *mut ether_header,
) {
    if zconf.send_ip_pkts == 0 {
        fprintf(
            fp,
            b"eth { shost: %02x:%02x:%02x:%02x:%02x:%02x | dhost: %02x:%02x:%02x:%02x:%02x:%02x }\n\0"
                as *const u8 as *const libc::c_char,
            *(((*ethh).ether_shost).as_mut_ptr() as *mut libc::c_uchar)
                .offset(0 as libc::c_int as isize) as libc::c_int,
            *(((*ethh).ether_shost).as_mut_ptr() as *mut libc::c_uchar)
                .offset(1 as libc::c_int as isize) as libc::c_int,
            *(((*ethh).ether_shost).as_mut_ptr() as *mut libc::c_uchar)
                .offset(2 as libc::c_int as isize) as libc::c_int,
            *(((*ethh).ether_shost).as_mut_ptr() as *mut libc::c_uchar)
                .offset(3 as libc::c_int as isize) as libc::c_int,
            *(((*ethh).ether_shost).as_mut_ptr() as *mut libc::c_uchar)
                .offset(4 as libc::c_int as isize) as libc::c_int,
            *(((*ethh).ether_shost).as_mut_ptr() as *mut libc::c_uchar)
                .offset(5 as libc::c_int as isize) as libc::c_int,
            *(((*ethh).ether_dhost).as_mut_ptr() as *mut libc::c_uchar)
                .offset(0 as libc::c_int as isize) as libc::c_int,
            *(((*ethh).ether_dhost).as_mut_ptr() as *mut libc::c_uchar)
                .offset(1 as libc::c_int as isize) as libc::c_int,
            *(((*ethh).ether_dhost).as_mut_ptr() as *mut libc::c_uchar)
                .offset(2 as libc::c_int as isize) as libc::c_int,
            *(((*ethh).ether_dhost).as_mut_ptr() as *mut libc::c_uchar)
                .offset(3 as libc::c_int as isize) as libc::c_int,
            *(((*ethh).ether_dhost).as_mut_ptr() as *mut libc::c_uchar)
                .offset(4 as libc::c_int as isize) as libc::c_int,
            *(((*ethh).ether_dhost).as_mut_ptr() as *mut libc::c_uchar)
                .offset(5 as libc::c_int as isize) as libc::c_int,
        );
    }
}
pub unsafe extern "C" fn make_eth_header(
    mut ethh: *mut ether_header,
    mut src: *mut macaddr_t,
    mut dst: *mut macaddr_t,
) {
    memcpy(
        ((*ethh).ether_shost).as_mut_ptr() as *mut libc::c_void,
        src as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        ((*ethh).ether_dhost).as_mut_ptr() as *mut libc::c_void,
        dst as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    );
    (*ethh).ether_type = __bswap_16(0x800 as libc::c_int as __uint16_t);
}
pub unsafe extern "C" fn make_ip_header(
    mut iph: *mut ip,
    mut protocol: uint8_t,
    mut len: uint16_t,
) {
    (*iph).set_ip_hl(5 as libc::c_int as libc::c_uint);
    (*iph).set_ip_v(4 as libc::c_int as libc::c_uint);
    (*iph).ip_tos = 0 as libc::c_int as uint8_t;
    (*iph).ip_len = len;
    (*iph).ip_id = __bswap_16(54321 as libc::c_int as __uint16_t);
    (*iph).ip_off = 0 as libc::c_int as libc::c_ushort;
    (*iph).ip_ttl = 255 as libc::c_int as uint8_t;
    (*iph).ip_p = protocol;
    (*iph).ip_sum = 0 as libc::c_int as libc::c_ushort;
}
pub unsafe extern "C" fn make_icmp_header(mut buf: *mut icmp) {
    memset(
        buf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<icmp>() as libc::c_ulong,
    );
    (*buf).icmp_type = 8 as libc::c_int as uint8_t;
    (*buf).icmp_code = 0 as libc::c_int as uint8_t;
    (*buf).icmp_hun.ih_idseq.icd_seq = 0 as libc::c_int as uint16_t;
}
pub unsafe extern "C" fn make_tcp_header(
    mut tcp_header: *mut tcphdr,
    mut dest_port: port_h_t,
    mut th_flags: uint16_t,
) {
    (*tcp_header).c2rust_unnamed.c2rust_unnamed.th_seq = random() as tcp_seq;
    (*tcp_header).c2rust_unnamed.c2rust_unnamed.th_ack = 0 as libc::c_int as tcp_seq;
    ((*tcp_header).c2rust_unnamed.c2rust_unnamed).set_th_x2(0 as libc::c_int as uint8_t);
    ((*tcp_header).c2rust_unnamed.c2rust_unnamed)
        .set_th_off(5 as libc::c_int as uint8_t);
    (*tcp_header).c2rust_unnamed.c2rust_unnamed.th_flags = 0 as libc::c_int as uint8_t;
    (*tcp_header)
        .c2rust_unnamed
        .c2rust_unnamed
        .th_flags = ((*tcp_header).c2rust_unnamed.c2rust_unnamed.th_flags as libc::c_int
        | th_flags as libc::c_int) as uint8_t;
    (*tcp_header)
        .c2rust_unnamed
        .c2rust_unnamed
        .th_win = __bswap_16(65535 as libc::c_int as __uint16_t);
    (*tcp_header).c2rust_unnamed.c2rust_unnamed.th_sum = 0 as libc::c_int as uint16_t;
    (*tcp_header).c2rust_unnamed.c2rust_unnamed.th_urp = 0 as libc::c_int as uint16_t;
    (*tcp_header).c2rust_unnamed.c2rust_unnamed.th_dport = __bswap_16(dest_port);
}
pub unsafe extern "C" fn set_mss_option(mut tcp_header: *mut tcphdr) -> size_t {
    let mut header_size: size_t = (((*tcp_header).c2rust_unnamed.c2rust_unnamed).th_off()
        as libc::c_int * 4 as libc::c_int) as size_t;
    let mut base: *mut uint8_t = tcp_header as *mut uint8_t;
    let mut last_opt: *mut uint8_t = base.offset(header_size as isize);
    *last_opt.offset(0 as libc::c_int as isize) = 2 as libc::c_int as uint8_t;
    *last_opt.offset(1 as libc::c_int as isize) = 4 as libc::c_int as uint8_t;
    *last_opt.offset(2 as libc::c_int as isize) = 0x5 as libc::c_int as uint8_t;
    *last_opt.offset(3 as libc::c_int as isize) = 0xb4 as libc::c_int as uint8_t;
    ((*tcp_header).c2rust_unnamed.c2rust_unnamed)
        .set_th_off(
            ((*tcp_header).c2rust_unnamed.c2rust_unnamed).th_off()
                + 1 as libc::c_int as uint8_t,
        );
    return (((*tcp_header).c2rust_unnamed.c2rust_unnamed).th_off() as libc::c_int
        * 4 as libc::c_int) as size_t;
}
pub unsafe extern "C" fn make_udp_header(
    mut udp_header: *mut udphdr,
    mut dest_port: port_h_t,
    mut len: uint16_t,
) {
    (*udp_header).c2rust_unnamed.c2rust_unnamed.uh_dport = __bswap_16(dest_port);
    (*udp_header).c2rust_unnamed.c2rust_unnamed.uh_ulen = __bswap_16(len);
    (*udp_header).c2rust_unnamed.c2rust_unnamed.uh_sum = 0 as libc::c_int as uint16_t;
}
pub unsafe extern "C" fn icmp_helper_validate(
    mut ip_hdr: *const ip,
    mut len: uint32_t,
    mut min_l4_len: size_t,
    mut probe_pkt: *mut *mut ip,
    mut probe_len: *mut size_t,
) -> libc::c_int {
    if (*ip_hdr).ip_p as libc::c_int == IPPROTO_ICMP as libc::c_int {} else {
        __assert_fail(
            b"ip_hdr->ip_p == IPPROTO_ICMP\0" as *const u8 as *const libc::c_char,
            b"probe_modules/packet.c\0" as *const u8 as *const libc::c_char,
            153 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 86],
                &[libc::c_char; 86],
            >(
                b"int icmp_helper_validate(const struct ip *, uint32_t, size_t, struct ip **, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5609: {
        if (*ip_hdr).ip_p as libc::c_int == IPPROTO_ICMP as libc::c_int {} else {
            __assert_fail(
                b"ip_hdr->ip_p == IPPROTO_ICMP\0" as *const u8 as *const libc::c_char,
                b"probe_modules/packet.c\0" as *const u8 as *const libc::c_char,
                153 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 86],
                    &[libc::c_char; 86],
                >(
                    b"int icmp_helper_validate(const struct ip *, uint32_t, size_t, struct ip **, size_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let min_len: uint32_t = ((4 as libc::c_int * (*ip_hdr).ip_hl() as libc::c_int
        + 8 as libc::c_int) as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ip>() as libc::c_ulong)
        .wrapping_add(min_l4_len) as uint32_t;
    if len < min_len {
        return 0 as libc::c_int;
    }
    let mut icmp: *mut icmp = (ip_hdr as *mut libc::c_char)
        .offset((4 as libc::c_int * (*ip_hdr).ip_hl() as libc::c_int) as isize)
        as *mut icmp;
    if !((*icmp).icmp_type as libc::c_int == 3 as libc::c_int
        || (*icmp).icmp_type as libc::c_int == 4 as libc::c_int
        || (*icmp).icmp_type as libc::c_int == 5 as libc::c_int
        || (*icmp).icmp_type as libc::c_int == 11 as libc::c_int)
    {
        return 0 as libc::c_int;
    }
    let mut ip_inner: *mut ip = (icmp as *mut libc::c_char)
        .offset(8 as libc::c_int as isize) as *mut ip;
    let mut inner_packet_len: size_t = len
        .wrapping_sub(
            (4 as libc::c_int * (*ip_hdr).ip_hl() as libc::c_int + 8 as libc::c_int)
                as libc::c_uint,
        ) as size_t;
    if inner_packet_len
        < ((4 as libc::c_int * (*ip_inner).ip_hl() as libc::c_int) as libc::c_ulong)
            .wrapping_add(min_l4_len)
    {
        return 0 as libc::c_int;
    }
    let mut dest: uint32_t = (*ip_inner).ip_dst.s_addr;
    if blocklist_is_allowed(dest) == 0 {
        return 0 as libc::c_int;
    }
    *probe_pkt = ip_inner;
    *probe_len = inner_packet_len;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn fs_add_null_icmp(mut fs: *mut fieldset_t) {
    fs_add_null(fs, b"icmp_responder\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"icmp_type\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"icmp_code\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"icmp_unreach_str\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn fs_add_failure_no_port(mut fs: *mut fieldset_t) {
    fs_add_null(fs, b"icmp_responder\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"icmp_type\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"icmp_code\0" as *const u8 as *const libc::c_char);
    fs_add_null(fs, b"icmp_unreach_str\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn fs_populate_icmp_from_iphdr(
    mut ip: *mut ip,
    mut len: size_t,
    mut fs: *mut fieldset_t,
) {
    if !ip.is_null()
        && !(b"no ip header provide to fs_populate_icmp_from_iphdr\0" as *const u8
            as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"ip && \"no ip header provide to fs_populate_icmp_from_iphdr\"\0"
                as *const u8 as *const libc::c_char,
            b"probe_modules/packet.c\0" as *const u8 as *const libc::c_char,
            215 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"void fs_populate_icmp_from_iphdr(struct ip *, size_t, fieldset_t *)\0"))
                .as_ptr(),
        );
    }
    'c_5904: {
        if !ip.is_null()
            && !(b"no ip header provide to fs_populate_icmp_from_iphdr\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"ip && \"no ip header provide to fs_populate_icmp_from_iphdr\"\0"
                    as *const u8 as *const libc::c_char,
                b"probe_modules/packet.c\0" as *const u8 as *const libc::c_char,
                215 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 68],
                    &[libc::c_char; 68],
                >(
                    b"void fs_populate_icmp_from_iphdr(struct ip *, size_t, fieldset_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !fs.is_null()
        && !(b"no fieldset provided to fs_populate_icmp_from_iphdr\0" as *const u8
            as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"fs && \"no fieldset provided to fs_populate_icmp_from_iphdr\"\0"
                as *const u8 as *const libc::c_char,
            b"probe_modules/packet.c\0" as *const u8 as *const libc::c_char,
            216 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"void fs_populate_icmp_from_iphdr(struct ip *, size_t, fieldset_t *)\0"))
                .as_ptr(),
        );
    }
    'c_5864: {
        if !fs.is_null()
            && !(b"no fieldset provided to fs_populate_icmp_from_iphdr\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"fs && \"no fieldset provided to fs_populate_icmp_from_iphdr\"\0"
                    as *const u8 as *const libc::c_char,
                b"probe_modules/packet.c\0" as *const u8 as *const libc::c_char,
                216 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 68],
                    &[libc::c_char; 68],
                >(
                    b"void fs_populate_icmp_from_iphdr(struct ip *, size_t, fieldset_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut icmp: *mut icmp = get_icmp_header(ip, len as uint32_t);
    if !icmp.is_null() {} else {
        __assert_fail(
            b"icmp\0" as *const u8 as *const libc::c_char,
            b"probe_modules/packet.c\0" as *const u8 as *const libc::c_char,
            218 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"void fs_populate_icmp_from_iphdr(struct ip *, size_t, fieldset_t *)\0"))
                .as_ptr(),
        );
    }
    'c_5830: {
        if !icmp.is_null() {} else {
            __assert_fail(
                b"icmp\0" as *const u8 as *const libc::c_char,
                b"probe_modules/packet.c\0" as *const u8 as *const libc::c_char,
                218 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 68],
                    &[libc::c_char; 68],
                >(
                    b"void fs_populate_icmp_from_iphdr(struct ip *, size_t, fieldset_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut ip_inner: *mut ip = get_inner_ip_header(icmp, len as uint32_t);
    fs_modify_string(
        fs,
        b"saddr\0" as *const u8 as *const libc::c_char,
        make_ip_str((*ip_inner).ip_dst.s_addr),
        1 as libc::c_int,
    );
    fs_add_string(
        fs,
        b"icmp_responder\0" as *const u8 as *const libc::c_char,
        make_ip_str((*ip).ip_src.s_addr),
        1 as libc::c_int,
    );
    fs_add_uint64(
        fs,
        b"icmp_type\0" as *const u8 as *const libc::c_char,
        (*icmp).icmp_type as uint64_t,
    );
    fs_add_uint64(
        fs,
        b"icmp_code\0" as *const u8 as *const libc::c_char,
        (*icmp).icmp_code as uint64_t,
    );
    if (*icmp).icmp_code as libc::c_int <= 15 as libc::c_int {
        fs_add_constchar(
            fs,
            b"icmp_unreach_str\0" as *const u8 as *const libc::c_char,
            *icmp_unreach_strings.as_mut_ptr().offset((*icmp).icmp_code as isize),
        );
    } else {
        fs_add_constchar(
            fs,
            b"icmp_unreach_str\0" as *const u8 as *const libc::c_char,
            b"unknown\0" as *const u8 as *const libc::c_char,
        );
    };
}
pub unsafe extern "C" fn make_ip_str(mut ip: uint32_t) -> *mut libc::c_char {
    let mut t: in_addr = in_addr { s_addr: 0 };
    t.s_addr = ip;
    let mut temp: *const libc::c_char = inet_ntoa(t);
    let mut retv: *mut libc::c_char = xmalloc(
        (strlen(temp)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    strcpy(retv, temp);
    return retv;
}
pub static mut icmp_unreach_strings: [*const libc::c_char; 16] = [
    b"network unreachable\0" as *const u8 as *const libc::c_char,
    b"host unreachable\0" as *const u8 as *const libc::c_char,
    b"protocol unreachable\0" as *const u8 as *const libc::c_char,
    b"port unreachable\0" as *const u8 as *const libc::c_char,
    b"fragments required\0" as *const u8 as *const libc::c_char,
    b"source route failed\0" as *const u8 as *const libc::c_char,
    b"network unknown\0" as *const u8 as *const libc::c_char,
    b"host unknown\0" as *const u8 as *const libc::c_char,
    b"source host isolated\0" as *const u8 as *const libc::c_char,
    b"network admin. prohibited\0" as *const u8 as *const libc::c_char,
    b"host admin. prohibited\0" as *const u8 as *const libc::c_char,
    b"network unreachable TOS\0" as *const u8 as *const libc::c_char,
    b"host unreachable TOS\0" as *const u8 as *const libc::c_char,
    b"communication admin. prohibited\0" as *const u8 as *const libc::c_char,
    b"host presdence violation\0" as *const u8 as *const libc::c_char,
    b"precedence cutoff\0" as *const u8 as *const libc::c_char,
];
