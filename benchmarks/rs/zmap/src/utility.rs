use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type output_module;
    pub type probe_module;
    pub type aesrand;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    static mut zconf: state_conf;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn log_fatal(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> !;
    fn log_debug(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
}
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
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
pub type uint64_t = __uint64_t;
pub type __uint64_t = libc::c_ulong;
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
pub type size_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub type in_addr_t = uint32_t;
pub type macaddr_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
pub type aesrand_t = aesrand;
pub type port_h_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
pub unsafe extern "C" fn string_to_ip_address(mut t: *mut libc::c_char) -> in_addr_t {
    let mut r: in_addr_t = inet_addr(t);
    if r == 0xffffffff as libc::c_uint {
        log_fatal(
            b"send\0" as *const u8 as *const libc::c_char,
            b"invalid ip address: `%s'\0" as *const u8 as *const libc::c_char,
            t,
        );
    }
    return r;
}
pub unsafe extern "C" fn add_to_array(mut to_add: *mut libc::c_char) {
    if zconf.number_source_ips >= 256 as libc::c_int as libc::c_uint {
        log_fatal(
            b"parse\0" as *const u8 as *const libc::c_char,
            b"over 256 source IP addresses provided\0" as *const u8
                as *const libc::c_char,
        );
    }
    log_debug(
        b"SEND\0" as *const u8 as *const libc::c_char,
        b"ipaddress: %s\n\0" as *const u8 as *const libc::c_char,
        to_add,
    );
    zconf
        .source_ip_addresses[zconf.number_source_ips
        as usize] = string_to_ip_address(to_add);
    zconf.number_source_ips = (zconf.number_source_ips).wrapping_add(1);
    zconf.number_source_ips;
}
pub unsafe extern "C" fn parse_source_ip_addresses(mut given_string: *mut libc::c_char) {
    let mut dash: *mut libc::c_char = strchr(
        given_string as *const libc::c_char,
        '-' as i32,
    );
    let mut comma: *mut libc::c_char = strchr(
        given_string as *const libc::c_char,
        ',' as i32,
    );
    if !dash.is_null() && !comma.is_null() {
        *comma = '\0' as i32 as libc::c_char;
        parse_source_ip_addresses(given_string);
        parse_source_ip_addresses(comma.offset(1 as libc::c_int as isize));
    } else if !comma.is_null() {
        while !comma.is_null() {
            *comma = '\0' as i32 as libc::c_char;
            add_to_array(given_string);
            given_string = comma.offset(1 as libc::c_int as isize);
            comma = strchr(given_string as *const libc::c_char, ',' as i32);
            if comma.is_null() {
                add_to_array(given_string);
            }
        }
    } else if !dash.is_null() {
        *dash = '\0' as i32 as libc::c_char;
        log_debug(
            b"SEND\0" as *const u8 as *const libc::c_char,
            b"address: %s\n\0" as *const u8 as *const libc::c_char,
            given_string,
        );
        log_debug(
            b"SEND\0" as *const u8 as *const libc::c_char,
            b"address: %s\n\0" as *const u8 as *const libc::c_char,
            dash.offset(1 as libc::c_int as isize),
        );
        let mut start: in_addr_t = __bswap_32(string_to_ip_address(given_string));
        let mut end: in_addr_t = (__bswap_32(
            string_to_ip_address(dash.offset(1 as libc::c_int as isize)),
        ))
            .wrapping_add(1 as libc::c_int as libc::c_uint);
        while start != end {
            let mut temp: in_addr = in_addr { s_addr: 0 };
            temp.s_addr = __bswap_32(start);
            add_to_array(strdup(inet_ntoa(temp)));
            start = start.wrapping_add(1);
            start;
        }
    } else {
        add_to_array(given_string);
    };
}
