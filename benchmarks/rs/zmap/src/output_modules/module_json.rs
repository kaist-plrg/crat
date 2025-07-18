use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type json_object;
    pub type aesrand;
    static mut stdout: *mut FILE;
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
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    fn json_object_put(obj: *mut json_object) -> libc::c_int;
    fn json_object_to_json_string(obj: *mut json_object) -> *const libc::c_char;
    fn json_object_to_json_string_ext(
        obj: *mut json_object,
        flags: libc::c_int,
    ) -> *const libc::c_char;
    fn json_object_new_object() -> *mut json_object;
    fn json_object_object_add(
        obj: *mut json_object,
        key: *const libc::c_char,
        val: *mut json_object,
    ) -> libc::c_int;
    fn json_object_new_array() -> *mut json_object;
    fn json_object_array_add(
        obj: *mut json_object,
        val: *mut json_object,
    ) -> libc::c_int;
    fn json_object_new_boolean(b: json_bool) -> *mut json_object;
    fn json_object_new_int64(i: int64_t) -> *mut json_object;
    fn json_object_new_string(s: *const libc::c_char) -> *mut json_object;
    fn log_fatal(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> !;
    fn check_and_log_file_error(file_0: *mut FILE, name: *const libc::c_char);
}
pub type size_t = libc::c_ulong;
pub type __u_char = libc::c_uchar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
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
pub type u_char = __u_char;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type json_bool = libc::c_int;
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
pub type output_update_cb = Option::<
    unsafe extern "C" fn(
        *mut state_conf,
        *mut state_send,
        *mut state_recv,
    ) -> libc::c_int,
>;
pub type output_init_cb = Option::<
    unsafe extern "C" fn(
        *mut state_conf,
        *mut *const libc::c_char,
        libc::c_int,
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
pub type output_module_t = output_module;
static mut file: *mut FILE = 0 as *const FILE as *mut FILE;
pub unsafe extern "C" fn json_output_file_init(
    mut conf: *mut state_conf,
    mut fields: *mut *const libc::c_char,
    mut fieldlens: libc::c_int,
) -> libc::c_int {
    if !conf.is_null() {} else {
        __assert_fail(
            b"conf\0" as *const u8 as *const libc::c_char,
            b"output_modules/module_json.c\0" as *const u8 as *const libc::c_char,
            39 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"int json_output_file_init(struct state_conf *, const char **, int)\0"))
                .as_ptr(),
        );
    }
    'c_5270: {
        if !conf.is_null() {} else {
            __assert_fail(
                b"conf\0" as *const u8 as *const libc::c_char,
                b"output_modules/module_json.c\0" as *const u8 as *const libc::c_char,
                39 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"int json_output_file_init(struct state_conf *, const char **, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if ((*conf).output_filename).is_null() {
        file = stdout;
    } else if strcmp((*conf).output_filename, b"-\0" as *const u8 as *const libc::c_char)
        == 0
    {
        file = stdout;
    } else {
        file = fopen(
            (*conf).output_filename,
            b"w\0" as *const u8 as *const libc::c_char,
        );
        if file.is_null() {
            log_fatal(
                b"output-json\0" as *const u8 as *const libc::c_char,
                b"could not open JSON output file (%s): %s\0" as *const u8
                    as *const libc::c_char,
                (*conf).output_filename,
                strerror(*__errno_location()),
            );
        }
    }
    check_and_log_file_error(file, b"json\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn hex_encode(
    mut packet: *mut libc::c_uchar,
    mut buflen: libc::c_int,
) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = xmalloc(
        (2 as libc::c_int * buflen + 1 as libc::c_int) as size_t,
    ) as *mut libc::c_char;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < buflen {
        snprintf(
            buf.offset((i * 2 as libc::c_int) as isize),
            3 as libc::c_int as libc::c_ulong,
            b"%.2x\0" as *const u8 as *const libc::c_char,
            *packet.offset(i as isize) as libc::c_int,
        );
        i += 1;
        i;
    }
    *buf.offset((buflen * 2 as libc::c_int) as isize) = 0 as libc::c_int as libc::c_char;
    return buf;
}
pub unsafe extern "C" fn field_to_jsonobj(mut f: *mut field_t) -> *mut json_object {
    if (*f).type_0 == 1 as libc::c_int {
        return json_object_new_string((*f).value.ptr as *mut libc::c_char)
    } else if (*f).type_0 == 2 as libc::c_int {
        return json_object_new_int64((*f).value.num as int64_t)
    } else if (*f).type_0 == 7 as libc::c_int {
        return json_object_new_boolean((*f).value.num as json_bool)
    } else if (*f).type_0 == 3 as libc::c_int {
        let mut encoded: *mut libc::c_char = hex_encode(
            (*f).value.ptr as *mut libc::c_uchar,
            (*f).len as libc::c_int,
        );
        let mut t: *mut json_object = json_object_new_string(encoded);
        free(encoded as *mut libc::c_void);
        return t;
    } else if (*f).type_0 == 4 as libc::c_int {
        return 0 as *mut json_object
    } else if (*f).type_0 == 5 as libc::c_int {
        return fs_to_jsonobj((*f).value.ptr as *mut fieldset_t)
    } else if (*f).type_0 == 6 as libc::c_int {
        return repeated_to_jsonobj((*f).value.ptr as *mut fieldset_t)
    } else {
        log_fatal(
            b"json\0" as *const u8 as *const libc::c_char,
            b"received unknown output type: %i\0" as *const u8 as *const libc::c_char,
            (*f).type_0,
        );
    };
}
pub unsafe extern "C" fn repeated_to_jsonobj(
    mut fs: *mut fieldset_t,
) -> *mut json_object {
    let mut obj: *mut json_object = json_object_new_array();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*fs).len {
        let mut f: *mut field_t = &mut *((*fs).fields).as_mut_ptr().offset(i as isize)
            as *mut field_t;
        json_object_array_add(obj, field_to_jsonobj(f));
        i += 1;
        i;
    }
    return obj;
}
pub unsafe extern "C" fn fs_to_jsonobj(mut fs: *mut fieldset_t) -> *mut json_object {
    let mut obj: *mut json_object = json_object_new_object();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*fs).len {
        let mut f: *mut field_t = &mut *((*fs).fields).as_mut_ptr().offset(i as isize)
            as *mut field_t;
        if (*f).type_0 != 4 as libc::c_int {
            json_object_object_add(obj, (*f).name, field_to_jsonobj(f));
        }
        i += 1;
        i;
    }
    return obj;
}
pub unsafe extern "C" fn json_output_to_file(mut fs: *mut fieldset_t) -> libc::c_int {
    if file.is_null() {
        return 0 as libc::c_int;
    }
    let mut record: *mut json_object = fs_to_jsonobj(fs);
    fprintf(
        file,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        json_object_to_json_string_ext(record, 0 as libc::c_int),
    );
    fflush(file);
    check_and_log_file_error(file, b"json\0" as *const u8 as *const libc::c_char);
    json_object_put(record);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn json_output_file_close(
    mut c: *mut state_conf,
    mut s: *mut state_send,
    mut r: *mut state_recv,
) -> libc::c_int {
    if !file.is_null() {
        fflush(file);
        fclose(file);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn print_json_fieldset(mut fs: *mut fieldset_t) -> libc::c_int {
    let mut record: *mut json_object = fs_to_jsonobj(fs);
    fprintf(
        stdout,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        json_object_to_json_string(record),
    );
    json_object_put(record);
    return 0 as libc::c_int;
}
pub static mut module_json_file: output_module_t = {
    let mut init = output_module {
        name: b"json\0" as *const u8 as *const libc::c_char,
        supports_dynamic_output: 1 as libc::c_int,
        update_interval: 0 as libc::c_int as libc::c_uint,
        init: Some(
            json_output_file_init
                as unsafe extern "C" fn(
                    *mut state_conf,
                    *mut *const libc::c_char,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        start: None,
        update: None,
        close: Some(
            json_output_file_close
                as unsafe extern "C" fn(
                    *mut state_conf,
                    *mut state_send,
                    *mut state_recv,
                ) -> libc::c_int,
        ),
        process_ip: Some(
            json_output_to_file as unsafe extern "C" fn(*mut fieldset_t) -> libc::c_int,
        ),
        helptext: b"Outputs one or more output fileds as a json valid file. By default, the \nprobe module does not filter out duplicates or limit to successful fields, \nbut rather includes all received packets. Fields can be controlled by \nsetting --output-fields. Filtering out failures and duplicate pakcets can \nbe achieved by setting an --output-filter.\0"
            as *const u8 as *const libc::c_char,
    };
    init
};
