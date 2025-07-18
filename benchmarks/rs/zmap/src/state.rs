use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type aesrand;
    pub type probe_module;
    pub type output_module;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub static mut zconf: state_conf = {
    let mut init = state_conf {
        log_level: 6 as libc::c_int,
        target_port: 0 as libc::c_int as port_h_t,
        source_port_first: 32768 as libc::c_int as port_h_t,
        source_port_last: 61000 as libc::c_int as port_h_t,
        max_targets: 0xffffffff as libc::c_uint,
        max_runtime: 0 as libc::c_int as uint32_t,
        max_results: 0 as libc::c_int as uint32_t,
        iface: 0 as *const libc::c_char as *mut libc::c_char,
        rate: -(1 as libc::c_int),
        bandwidth: 0 as libc::c_int as uint64_t,
        cooldown_secs: 0 as libc::c_int,
        senders: 1 as libc::c_int as uint8_t,
        batch: 1 as libc::c_int as uint8_t,
        pin_cores_len: 0,
        pin_cores: 0 as *const uint32_t as *mut uint32_t,
        seed_provided: 0 as libc::c_int,
        seed: 0 as libc::c_int as uint64_t,
        aes: 0 as *const aesrand_t as *mut aesrand_t,
        generator: 0,
        shard_num: 0,
        total_shards: 0,
        packet_streams: 1 as libc::c_int,
        probe_module: 0 as *const probe_module as *mut probe_module,
        output_module_name: 0 as *const libc::c_char as *mut libc::c_char,
        output_module: 0 as *const output_module as *mut output_module,
        probe_args: 0 as *const libc::c_char as *mut libc::c_char,
        probe_ttl: 255 as libc::c_int as uint8_t,
        output_args: 0 as *const libc::c_char as *mut libc::c_char,
        gw_mac: [0 as libc::c_int as macaddr_t, 0, 0, 0, 0, 0],
        hw_mac: [0 as libc::c_int as macaddr_t, 0, 0, 0, 0, 0],
        gw_ip: 0 as libc::c_int as uint32_t,
        gw_mac_set: 0 as libc::c_int,
        hw_mac_set: 0 as libc::c_int,
        source_ip_addresses: [0; 256],
        number_source_ips: 0 as libc::c_int as uint32_t,
        send_ip_pkts: 0 as libc::c_int,
        output_filename: 0 as *const libc::c_char as *mut libc::c_char,
        blocklist_filename: 0 as *const libc::c_char as *mut libc::c_char,
        allowlist_filename: 0 as *const libc::c_char as *mut libc::c_char,
        list_of_ips_filename: 0 as *const libc::c_char as *mut libc::c_char,
        list_of_ips_count: 0 as libc::c_int as uint32_t,
        metadata_filename: 0 as *const libc::c_char as *mut libc::c_char,
        metadata_file: 0 as *const FILE as *mut FILE,
        notes: 0 as *const libc::c_char as *mut libc::c_char,
        custom_metadata_str: 0 as *const libc::c_char as *mut libc::c_char,
        destination_cidrs: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        destination_cidrs_len: 0,
        raw_output_fields: 0 as *const libc::c_char,
        output_fields: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        filter: output_filter {
            expression: 0 as *const node_t as *mut node_t,
        },
        output_filter_str: 0 as *const libc::c_char as *mut libc::c_char,
        fsconf: fieldset_conf {
            defs: fielddefset_t {
                fielddefs: [fielddef_t {
                    name: 0 as *const libc::c_char,
                    type_0: 0 as *const libc::c_char,
                    desc: 0 as *const libc::c_char,
                }; 128],
                len: 0,
            },
            outdefs: fielddefset_t {
                fielddefs: [fielddef_t {
                    name: 0 as *const libc::c_char,
                    type_0: 0 as *const libc::c_char,
                    desc: 0 as *const libc::c_char,
                }; 128],
                len: 0,
            },
            translation: translation_t {
                len: 0,
                translation: [0; 128],
            },
            success_index: 0,
            app_success_index: 0,
            classification_index: 0,
        },
        output_fields_len: 0 as libc::c_int,
        log_file: 0 as *const libc::c_char as *mut libc::c_char,
        log_directory: 0 as *const libc::c_char as *mut libc::c_char,
        status_updates_file: 0 as *const libc::c_char as *mut libc::c_char,
        dryrun: 0 as libc::c_int,
        quiet: 0 as libc::c_int,
        ignore_invalid_hosts: 0,
        syslog: 1 as libc::c_int,
        recv_ready: 0 as libc::c_int,
        num_retries: 0,
        total_allowed: 0,
        total_disallowed: 0,
        max_sendto_failures: -(1 as libc::c_int),
        min_hitrate: 0.0f64 as libc::c_float,
        data_link_size: 0 as libc::c_int,
        default_mode: 0 as libc::c_int,
        no_header_row: 0 as libc::c_int,
    };
    init
};
pub unsafe extern "C" fn init_empty_global_configuration(mut c: *mut state_conf) {
    memset(
        ((*c).source_ip_addresses).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[in_addr_t; 256]>() as libc::c_ulong,
    );
}
pub static mut zsend: state_send = {
    let mut init = state_send {
        start: 0.0f64,
        finish: 0.0f64,
        packets_sent: 0 as libc::c_int as uint64_t,
        hosts_scanned: 0 as libc::c_int as uint64_t,
        blocklisted: 0 as libc::c_int as uint64_t,
        allowlisted: 0 as libc::c_int as uint64_t,
        warmup: 1 as libc::c_int,
        complete: 0 as libc::c_int,
        first_scanned: 0,
        max_targets: 0 as libc::c_int as uint32_t,
        sendto_failures: 0 as libc::c_int as uint32_t,
        max_index: 0,
        list_of_ips_pbm: 0 as *const *mut uint8_t as *mut *mut uint8_t,
    };
    init
};
pub static mut zrecv: state_recv = {
    let mut init = state_recv {
        success_total: 0 as libc::c_int as uint32_t,
        success_unique: 0 as libc::c_int as uint32_t,
        app_success_total: 0 as libc::c_int as uint32_t,
        app_success_unique: 0 as libc::c_int as uint32_t,
        cooldown_total: 0 as libc::c_int as uint32_t,
        cooldown_unique: 0 as libc::c_int as uint32_t,
        failure_total: 0 as libc::c_int as uint32_t,
        filter_success: 0 as libc::c_int as uint64_t,
        ip_fragments: 0 as libc::c_int as uint32_t,
        validation_passed: 0 as libc::c_int as uint32_t,
        validation_failed: 0 as libc::c_int as uint32_t,
        complete: 0 as libc::c_int,
        start: 0.,
        finish: 0.,
        pcap_recv: 0 as libc::c_int as uint32_t,
        pcap_drop: 0 as libc::c_int as uint32_t,
        pcap_ifdrop: 0 as libc::c_int as uint32_t,
    };
    init
};
