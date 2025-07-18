use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type json_object;
    pub type aesrand;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn gethostbyname(__name: *const libc::c_char) -> *mut hostent;
    fn log_error(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn dstrftime(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: libc::c_double,
    ) -> size_t;
    fn get_blocklisted_cidrs() -> *mut bl_cidr_node_t;
    fn get_allowlisted_cidrs() -> *mut bl_cidr_node_t;
    static mut zconf: state_conf;
    static mut zsend: state_send;
    static mut zrecv: state_recv;
    fn json_object_put(obj: *mut json_object) -> libc::c_int;
    fn json_object_to_json_string(obj: *mut json_object) -> *const libc::c_char;
    fn json_object_new_array() -> *mut json_object;
    fn json_object_object_add(
        obj: *mut json_object,
        key: *const libc::c_char,
        val: *mut json_object,
    ) -> libc::c_int;
    fn json_object_new_object() -> *mut json_object;
    fn json_object_array_add(
        obj: *mut json_object,
        val: *mut json_object,
    ) -> libc::c_int;
    fn json_tokener_parse(str: *const libc::c_char) -> *mut json_object;
    fn json_object_new_int(i: int32_t) -> *mut json_object;
    fn json_object_new_int64(i: int64_t) -> *mut json_object;
    fn json_object_new_double(d: libc::c_double) -> *mut json_object;
    fn json_object_new_string(s: *const libc::c_char) -> *mut json_object;
}
pub type size_t = libc::c_ulong;
pub type __u_char = libc::c_uchar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type bl_cidr_node_t = bl_cidr_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bl_cidr_node {
    pub ip_address: uint32_t,
    pub prefix_len: libc::c_int,
    pub next: *mut bl_cidr_node,
}
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
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
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_module {
    pub name: *const libc::c_char,
    pub supports_dynamic_output: libc::c_int,
    pub update_interval: libc::c_uint,
    pub init: output_init_cb,
    pub start: output_update_cb,
    pub update: output_update_cb,
    pub close: output_update_cb,
    pub process_ip: output_packet_cb,
    pub helptext: *const libc::c_char,
}
pub type output_packet_cb = Option::<
    unsafe extern "C" fn(*mut fieldset_t) -> libc::c_int,
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
pub type output_update_cb = Option::<
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
pub type output_init_cb = Option::<
    unsafe extern "C" fn(
        *mut state_conf,
        *mut *const libc::c_char,
        libc::c_int,
    ) -> libc::c_int,
>;
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
pub type probe_classify_packet_cb = Option::<
    unsafe extern "C" fn(
        *const u_char,
        uint32_t,
        *mut fieldset_t,
        *mut uint32_t,
        timespec,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type u_char = __u_char;
pub type probe_validate_packet_cb = Option::<
    unsafe extern "C" fn(
        *const ip,
        uint32_t,
        *mut uint32_t,
        *mut uint32_t,
    ) -> libc::c_int,
>;
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
pub type uint16_t = __uint16_t;
pub type probe_global_init_cb = Option::<
    unsafe extern "C" fn(*mut state_conf) -> libc::c_int,
>;
pub type aesrand_t = aesrand;
pub type port_h_t = uint16_t;
pub type int32_t = __int32_t;
pub type uint = libc::c_uint;
pub type output_module_t = output_module;
pub type probe_module_t = probe_module;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
    pub h_name: *mut libc::c_char,
    pub h_aliases: *mut *mut libc::c_char,
    pub h_addrtype: libc::c_int,
    pub h_length: libc::c_int,
    pub h_addr_list: *mut *mut libc::c_char,
}
pub unsafe extern "C" fn json_metadata(mut file: *mut FILE) {
    let mut send_start_time: [libc::c_char; 1025] = [0; 1025];
    if dstrftime(
        send_start_time.as_mut_ptr(),
        1024 as libc::c_int as size_t,
        b"%Y-%m-%dT%H:%M:%S%z\0" as *const u8 as *const libc::c_char,
        zsend.start,
    ) != 0
    {} else {
        __assert_fail(
            b"dstrftime(send_start_time, STRTIME_LEN, \"%Y-%m-%dT%H:%M:%S%z\", zsend.start)\0"
                as *const u8 as *const libc::c_char,
            b"summary.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void json_metadata(FILE *)\0"))
                .as_ptr(),
        );
    }
    'c_3221: {
        if dstrftime(
            send_start_time.as_mut_ptr(),
            1024 as libc::c_int as size_t,
            b"%Y-%m-%dT%H:%M:%S%z\0" as *const u8 as *const libc::c_char,
            zsend.start,
        ) != 0
        {} else {
            __assert_fail(
                b"dstrftime(send_start_time, STRTIME_LEN, \"%Y-%m-%dT%H:%M:%S%z\", zsend.start)\0"
                    as *const u8 as *const libc::c_char,
                b"summary.c\0" as *const u8 as *const libc::c_char,
                34 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void json_metadata(FILE *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut send_end_time: [libc::c_char; 1025] = [0; 1025];
    if dstrftime(
        send_end_time.as_mut_ptr(),
        1024 as libc::c_int as size_t,
        b"%Y-%m-%dT%H:%M:%S%z\0" as *const u8 as *const libc::c_char,
        zsend.finish,
    ) != 0
    {} else {
        __assert_fail(
            b"dstrftime(send_end_time, STRTIME_LEN, \"%Y-%m-%dT%H:%M:%S%z\", zsend.finish)\0"
                as *const u8 as *const libc::c_char,
            b"summary.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void json_metadata(FILE *)\0"))
                .as_ptr(),
        );
    }
    'c_3169: {
        if dstrftime(
            send_end_time.as_mut_ptr(),
            1024 as libc::c_int as size_t,
            b"%Y-%m-%dT%H:%M:%S%z\0" as *const u8 as *const libc::c_char,
            zsend.finish,
        ) != 0
        {} else {
            __assert_fail(
                b"dstrftime(send_end_time, STRTIME_LEN, \"%Y-%m-%dT%H:%M:%S%z\", zsend.finish)\0"
                    as *const u8 as *const libc::c_char,
                b"summary.c\0" as *const u8 as *const libc::c_char,
                37 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void json_metadata(FILE *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut recv_start_time: [libc::c_char; 1025] = [0; 1025];
    if dstrftime(
        recv_start_time.as_mut_ptr(),
        1024 as libc::c_int as size_t,
        b"%Y-%m-%dT%H:%M:%S%z\0" as *const u8 as *const libc::c_char,
        zrecv.start,
    ) != 0
    {} else {
        __assert_fail(
            b"dstrftime(recv_start_time, STRTIME_LEN, \"%Y-%m-%dT%H:%M:%S%z\", zrecv.start)\0"
                as *const u8 as *const libc::c_char,
            b"summary.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void json_metadata(FILE *)\0"))
                .as_ptr(),
        );
    }
    'c_3116: {
        if dstrftime(
            recv_start_time.as_mut_ptr(),
            1024 as libc::c_int as size_t,
            b"%Y-%m-%dT%H:%M:%S%z\0" as *const u8 as *const libc::c_char,
            zrecv.start,
        ) != 0
        {} else {
            __assert_fail(
                b"dstrftime(recv_start_time, STRTIME_LEN, \"%Y-%m-%dT%H:%M:%S%z\", zrecv.start)\0"
                    as *const u8 as *const libc::c_char,
                b"summary.c\0" as *const u8 as *const libc::c_char,
                40 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void json_metadata(FILE *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut recv_end_time: [libc::c_char; 1025] = [0; 1025];
    if dstrftime(
        recv_end_time.as_mut_ptr(),
        1024 as libc::c_int as size_t,
        b"%Y-%m-%dT%H:%M:%S%z\0" as *const u8 as *const libc::c_char,
        zrecv.finish,
    ) != 0
    {} else {
        __assert_fail(
            b"dstrftime(recv_end_time, STRTIME_LEN, \"%Y-%m-%dT%H:%M:%S%z\", zrecv.finish)\0"
                as *const u8 as *const libc::c_char,
            b"summary.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void json_metadata(FILE *)\0"))
                .as_ptr(),
        );
    }
    'c_3047: {
        if dstrftime(
            recv_end_time.as_mut_ptr(),
            1024 as libc::c_int as size_t,
            b"%Y-%m-%dT%H:%M:%S%z\0" as *const u8 as *const libc::c_char,
            zrecv.finish,
        ) != 0
        {} else {
            __assert_fail(
                b"dstrftime(recv_end_time, STRTIME_LEN, \"%Y-%m-%dT%H:%M:%S%z\", zrecv.finish)\0"
                    as *const u8 as *const libc::c_char,
                b"summary.c\0" as *const u8 as *const libc::c_char,
                43 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void json_metadata(FILE *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut hitrate: libc::c_double = 100 as libc::c_int as libc::c_double
        * zrecv.success_unique as libc::c_double / zsend.hosts_scanned as libc::c_double;
    let mut obj: *mut json_object = json_object_new_object();
    let mut hostname: [libc::c_char; 1024] = [0; 1024];
    if gethostname(hostname.as_mut_ptr(), 1023 as libc::c_int as size_t)
        < 0 as libc::c_int
    {
        log_error(
            b"json_metadata\0" as *const u8 as *const libc::c_char,
            b"unable to retrieve local hostname\0" as *const u8 as *const libc::c_char,
        );
    } else {
        hostname[1023 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        json_object_object_add(
            obj,
            b"local_hostname\0" as *const u8 as *const libc::c_char,
            json_object_new_string(hostname.as_mut_ptr()),
        );
        let mut h: *mut hostent = gethostbyname(hostname.as_mut_ptr());
        if !h.is_null() {
            json_object_object_add(
                obj,
                b"full_hostname\0" as *const u8 as *const libc::c_char,
                json_object_new_string((*h).h_name),
            );
        } else {
            log_error(
                b"json_metadata\0" as *const u8 as *const libc::c_char,
                b"unable to retrieve complete hostname\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    json_object_object_add(
        obj,
        b"target_port\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zconf.target_port as int32_t),
    );
    json_object_object_add(
        obj,
        b"source_port_first\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zconf.source_port_first as int32_t),
    );
    json_object_object_add(
        obj,
        b"source_port_last\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zconf.source_port_last as int32_t),
    );
    json_object_object_add(
        obj,
        b"max_targets\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zconf.max_targets as int32_t),
    );
    json_object_object_add(
        obj,
        b"max_runtime\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zconf.max_runtime as int32_t),
    );
    json_object_object_add(
        obj,
        b"max_results\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zconf.max_results as int32_t),
    );
    json_object_object_add(
        obj,
        b"output_results\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zrecv.filter_success as int32_t),
    );
    if !(zconf.iface).is_null() {
        json_object_object_add(
            obj,
            b"iface\0" as *const u8 as *const libc::c_char,
            json_object_new_string(zconf.iface),
        );
    }
    json_object_object_add(
        obj,
        b"rate\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zconf.rate),
    );
    json_object_object_add(
        obj,
        b"bandwidth\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zconf.bandwidth as int32_t),
    );
    json_object_object_add(
        obj,
        b"cooldown_secs\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zconf.cooldown_secs),
    );
    json_object_object_add(
        obj,
        b"senders\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zconf.senders as int32_t),
    );
    json_object_object_add(
        obj,
        b"seed\0" as *const u8 as *const libc::c_char,
        json_object_new_int64(zconf.seed as int64_t),
    );
    json_object_object_add(
        obj,
        b"seed_provided\0" as *const u8 as *const libc::c_char,
        json_object_new_int64(zconf.seed_provided as int64_t),
    );
    json_object_object_add(
        obj,
        b"generator\0" as *const u8 as *const libc::c_char,
        json_object_new_int64(zconf.generator as int64_t),
    );
    json_object_object_add(
        obj,
        b"hitrate\0" as *const u8 as *const libc::c_char,
        json_object_new_double(hitrate),
    );
    json_object_object_add(
        obj,
        b"shard_num\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zconf.shard_num as int32_t),
    );
    json_object_object_add(
        obj,
        b"total_shards\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zconf.total_shards as int32_t),
    );
    json_object_object_add(
        obj,
        b"min_hitrate\0" as *const u8 as *const libc::c_char,
        json_object_new_double(zconf.min_hitrate as libc::c_double),
    );
    json_object_object_add(
        obj,
        b"max_sendto_failures\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zconf.max_sendto_failures),
    );
    json_object_object_add(
        obj,
        b"syslog\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zconf.syslog),
    );
    json_object_object_add(
        obj,
        b"default_mode\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zconf.default_mode),
    );
    json_object_object_add(
        obj,
        b"pcap_recv\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zrecv.pcap_recv as int32_t),
    );
    json_object_object_add(
        obj,
        b"pcap_drop\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zrecv.pcap_drop as int32_t),
    );
    json_object_object_add(
        obj,
        b"pcap_ifdrop\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zrecv.pcap_ifdrop as int32_t),
    );
    json_object_object_add(
        obj,
        b"ip_fragments\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zrecv.ip_fragments as int32_t),
    );
    json_object_object_add(
        obj,
        b"blocklist_total_allowed\0" as *const u8 as *const libc::c_char,
        json_object_new_int64(zconf.total_allowed as int64_t),
    );
    json_object_object_add(
        obj,
        b"blocklist_total_not_allowed\0" as *const u8 as *const libc::c_char,
        json_object_new_int64(zconf.total_disallowed as int64_t),
    );
    json_object_object_add(
        obj,
        b"validation_passed\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zrecv.validation_passed as int32_t),
    );
    json_object_object_add(
        obj,
        b"validation_failed\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zrecv.validation_failed as int32_t),
    );
    json_object_object_add(
        obj,
        b"first_scanned\0" as *const u8 as *const libc::c_char,
        json_object_new_int64(zsend.first_scanned as int64_t),
    );
    json_object_object_add(
        obj,
        b"send_to_failures\0" as *const u8 as *const libc::c_char,
        json_object_new_int64(zsend.sendto_failures as int64_t),
    );
    json_object_object_add(
        obj,
        b"packets_sent\0" as *const u8 as *const libc::c_char,
        json_object_new_int64(zsend.packets_sent as int64_t),
    );
    json_object_object_add(
        obj,
        b"hosts_scanned\0" as *const u8 as *const libc::c_char,
        json_object_new_int64(zsend.hosts_scanned as int64_t),
    );
    json_object_object_add(
        obj,
        b"success_total\0" as *const u8 as *const libc::c_char,
        json_object_new_int64(zrecv.success_total as int64_t),
    );
    json_object_object_add(
        obj,
        b"success_unique\0" as *const u8 as *const libc::c_char,
        json_object_new_int64(zrecv.success_unique as int64_t),
    );
    if zconf.fsconf.app_success_index >= 0 as libc::c_int {
        json_object_object_add(
            obj,
            b"app_success_total\0" as *const u8 as *const libc::c_char,
            json_object_new_int64(zrecv.app_success_total as int64_t),
        );
        json_object_object_add(
            obj,
            b"app_success_unique\0" as *const u8 as *const libc::c_char,
            json_object_new_int64(zrecv.app_success_unique as int64_t),
        );
    }
    json_object_object_add(
        obj,
        b"success_cooldown_total\0" as *const u8 as *const libc::c_char,
        json_object_new_int64(zrecv.cooldown_total as int64_t),
    );
    json_object_object_add(
        obj,
        b"success_cooldown_unique\0" as *const u8 as *const libc::c_char,
        json_object_new_int64(zrecv.cooldown_unique as int64_t),
    );
    json_object_object_add(
        obj,
        b"failure_total\0" as *const u8 as *const libc::c_char,
        json_object_new_int64(zrecv.failure_total as int64_t),
    );
    json_object_object_add(
        obj,
        b"packet_streams\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zconf.packet_streams),
    );
    json_object_object_add(
        obj,
        b"probe_module\0" as *const u8 as *const libc::c_char,
        json_object_new_string((*(zconf.probe_module as *mut probe_module_t)).name),
    );
    json_object_object_add(
        obj,
        b"output_module\0" as *const u8 as *const libc::c_char,
        json_object_new_string((*(zconf.output_module as *mut output_module_t)).name),
    );
    json_object_object_add(
        obj,
        b"send_start_time\0" as *const u8 as *const libc::c_char,
        json_object_new_string(send_start_time.as_mut_ptr()),
    );
    json_object_object_add(
        obj,
        b"send_end_time\0" as *const u8 as *const libc::c_char,
        json_object_new_string(send_end_time.as_mut_ptr()),
    );
    json_object_object_add(
        obj,
        b"recv_start_time\0" as *const u8 as *const libc::c_char,
        json_object_new_string(recv_start_time.as_mut_ptr()),
    );
    json_object_object_add(
        obj,
        b"recv_end_time\0" as *const u8 as *const libc::c_char,
        json_object_new_string(recv_end_time.as_mut_ptr()),
    );
    if !(zconf.output_filter_str).is_null() {
        json_object_object_add(
            obj,
            b"output_filter\0" as *const u8 as *const libc::c_char,
            json_object_new_string(zconf.output_filter_str),
        );
    }
    if !(zconf.log_file).is_null() {
        json_object_object_add(
            obj,
            b"log_file\0" as *const u8 as *const libc::c_char,
            json_object_new_string(zconf.log_file),
        );
    }
    if !(zconf.log_directory).is_null() {
        json_object_object_add(
            obj,
            b"log_directory\0" as *const u8 as *const libc::c_char,
            json_object_new_string(zconf.log_directory),
        );
    }
    if zconf.destination_cidrs_len != 0 {
        let mut cli_dest_cidrs: *mut json_object = json_object_new_array();
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < zconf.destination_cidrs_len {
            json_object_array_add(
                cli_dest_cidrs,
                json_object_new_string(*(zconf.destination_cidrs).offset(i as isize)),
            );
            i += 1;
            i;
        }
        json_object_object_add(
            obj,
            b"cli_cidr_destinations\0" as *const u8 as *const libc::c_char,
            cli_dest_cidrs,
        );
    }
    if !(zconf.probe_args).is_null() {
        json_object_object_add(
            obj,
            b"probe_args\0" as *const u8 as *const libc::c_char,
            json_object_new_string(zconf.probe_args),
        );
    }
    if zconf.probe_ttl != 0 {
        json_object_object_add(
            obj,
            b"probe_ttl\0" as *const u8 as *const libc::c_char,
            json_object_new_int(zconf.probe_ttl as int32_t),
        );
    }
    if !(zconf.output_args).is_null() {
        json_object_object_add(
            obj,
            b"output_args\0" as *const u8 as *const libc::c_char,
            json_object_new_string(zconf.output_args),
        );
    }
    let mut mac_buf: [libc::c_char; 18] = [0; 18];
    memset(
        mac_buf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong,
    );
    let mut p: *mut libc::c_char = mac_buf.as_mut_ptr();
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 6 as libc::c_int {
        if i_0 == 6 as libc::c_int - 1 as libc::c_int {
            snprintf(
                p,
                3 as libc::c_int as libc::c_ulong,
                b"%.2x\0" as *const u8 as *const libc::c_char,
                zconf.gw_mac[i_0 as usize] as libc::c_int,
            );
            p = p.offset(2 as libc::c_int as isize);
        } else {
            snprintf(
                p,
                4 as libc::c_int as libc::c_ulong,
                b"%.2x:\0" as *const u8 as *const libc::c_char,
                zconf.gw_mac[i_0 as usize] as libc::c_int,
            );
            p = p.offset(3 as libc::c_int as isize);
        }
        i_0 += 1;
        i_0;
    }
    json_object_object_add(
        obj,
        b"gateway_mac\0" as *const u8 as *const libc::c_char,
        json_object_new_string(mac_buf.as_mut_ptr()),
    );
    if zconf.gw_ip != 0 {
        let mut addr: in_addr = in_addr { s_addr: 0 };
        addr.s_addr = zconf.gw_ip;
        json_object_object_add(
            obj,
            b"gateway_ip\0" as *const u8 as *const libc::c_char,
            json_object_new_string(inet_ntoa(addr)),
        );
    }
    let mut mac_buf_0: [libc::c_char; 18] = [0; 18];
    let mut p_0: *mut libc::c_char = mac_buf_0.as_mut_ptr();
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < 6 as libc::c_int {
        if i_1 == 6 as libc::c_int - 1 as libc::c_int {
            snprintf(
                p_0,
                3 as libc::c_int as libc::c_ulong,
                b"%.2x\0" as *const u8 as *const libc::c_char,
                zconf.hw_mac[i_1 as usize] as libc::c_int,
            );
            p_0 = p_0.offset(2 as libc::c_int as isize);
        } else {
            snprintf(
                p_0,
                4 as libc::c_int as libc::c_ulong,
                b"%.2x:\0" as *const u8 as *const libc::c_char,
                zconf.hw_mac[i_1 as usize] as libc::c_int,
            );
            p_0 = p_0.offset(3 as libc::c_int as isize);
        }
        i_1 += 1;
        i_1;
    }
    json_object_object_add(
        obj,
        b"source_mac\0" as *const u8 as *const libc::c_char,
        json_object_new_string(mac_buf_0.as_mut_ptr()),
    );
    let mut source_ips: *mut json_object = json_object_new_array();
    let mut i_2: uint = 0 as libc::c_int as uint;
    while i_2 < zconf.number_source_ips {
        let mut temp: in_addr = in_addr { s_addr: 0 };
        temp.s_addr = zconf.source_ip_addresses[i_2 as usize];
        json_object_array_add(
            source_ips,
            json_object_new_string(strdup(inet_ntoa(temp))),
        );
        i_2 = i_2.wrapping_add(1);
        i_2;
    }
    json_object_object_add(
        obj,
        b"source_ips\0" as *const u8 as *const libc::c_char,
        source_ips,
    );
    if !(zconf.output_filename).is_null() {
        json_object_object_add(
            obj,
            b"output_filename\0" as *const u8 as *const libc::c_char,
            json_object_new_string(zconf.output_filename),
        );
    }
    if !(zconf.blocklist_filename).is_null() {
        json_object_object_add(
            obj,
            b"blocklist_filename\0" as *const u8 as *const libc::c_char,
            json_object_new_string(zconf.blocklist_filename),
        );
    }
    if !(zconf.allowlist_filename).is_null() {
        json_object_object_add(
            obj,
            b"allowlist_filename\0" as *const u8 as *const libc::c_char,
            json_object_new_string(zconf.allowlist_filename),
        );
    }
    if !(zconf.list_of_ips_filename).is_null() {
        json_object_object_add(
            obj,
            b"list_of_ips_filename\0" as *const u8 as *const libc::c_char,
            json_object_new_string(zconf.list_of_ips_filename),
        );
        json_object_object_add(
            obj,
            b"list_of_ips_count\0" as *const u8 as *const libc::c_char,
            json_object_new_int(zconf.list_of_ips_count as int32_t),
        );
    }
    json_object_object_add(
        obj,
        b"dryrun\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zconf.dryrun),
    );
    json_object_object_add(
        obj,
        b"quiet\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zconf.quiet),
    );
    json_object_object_add(
        obj,
        b"log_level\0" as *const u8 as *const libc::c_char,
        json_object_new_int(zconf.log_level),
    );
    if !(zconf.custom_metadata_str).is_null() {
        let mut user: *mut json_object = json_tokener_parse(zconf.custom_metadata_str);
        if user.is_null() {
            log_error(
                b"json-metadata\0" as *const u8 as *const libc::c_char,
                b"unable to parse user metadata\0" as *const u8 as *const libc::c_char,
            );
        } else {
            json_object_object_add(
                obj,
                b"user-metadata\0" as *const u8 as *const libc::c_char,
                user,
            );
        }
    }
    if !(zconf.notes).is_null() {
        json_object_object_add(
            obj,
            b"notes\0" as *const u8 as *const libc::c_char,
            json_object_new_string(zconf.notes),
        );
    }
    let mut b: *mut bl_cidr_node_t = get_blocklisted_cidrs();
    if !b.is_null() {
        let mut blocklisted_cidrs: *mut json_object = json_object_new_array();
        loop {
            let mut cidr: [libc::c_char; 50] = [0; 50];
            let mut addr_0: in_addr = in_addr { s_addr: 0 };
            addr_0.s_addr = (*b).ip_address;
            sprintf(
                cidr.as_mut_ptr(),
                b"%s/%i\0" as *const u8 as *const libc::c_char,
                inet_ntoa(addr_0),
                (*b).prefix_len,
            );
            json_object_array_add(
                blocklisted_cidrs,
                json_object_new_string(cidr.as_mut_ptr()),
            );
            if !(!b.is_null()
                && {
                    b = (*b).next;
                    !b.is_null()
                })
            {
                break;
            }
        }
        json_object_object_add(
            obj,
            b"blocklisted_networks\0" as *const u8 as *const libc::c_char,
            blocklisted_cidrs,
        );
    }
    b = get_allowlisted_cidrs();
    if !b.is_null() {
        let mut allowlisted_cidrs: *mut json_object = json_object_new_array();
        loop {
            let mut cidr_0: [libc::c_char; 50] = [0; 50];
            let mut addr_1: in_addr = in_addr { s_addr: 0 };
            addr_1.s_addr = (*b).ip_address;
            sprintf(
                cidr_0.as_mut_ptr(),
                b"%s/%i\0" as *const u8 as *const libc::c_char,
                inet_ntoa(addr_1),
                (*b).prefix_len,
            );
            json_object_array_add(
                allowlisted_cidrs,
                json_object_new_string(cidr_0.as_mut_ptr()),
            );
            if !(!b.is_null()
                && {
                    b = (*b).next;
                    !b.is_null()
                })
            {
                break;
            }
        }
        json_object_object_add(
            obj,
            b"allowlisted_networks\0" as *const u8 as *const libc::c_char,
            allowlisted_cidrs,
        );
    }
    fprintf(
        file,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        json_object_to_json_string(obj),
    );
    json_object_put(obj);
}
