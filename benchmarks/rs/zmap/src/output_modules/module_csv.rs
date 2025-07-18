use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type aesrand;
    pub type probe_module;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
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
    fn check_and_log_file_error(file_0: *mut FILE, name: *const libc::c_char);
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
pub type output_init_cb = Option::<
    unsafe extern "C" fn(
        *mut state_conf,
        *mut *const libc::c_char,
        libc::c_int,
    ) -> libc::c_int,
>;
pub type output_module_t = output_module;
static mut file: *mut FILE = 0 as *const FILE as *mut FILE;
pub unsafe extern "C" fn csv_init(
    mut conf: *mut state_conf,
    mut fields: *mut *const libc::c_char,
    mut fieldlens: libc::c_int,
) -> libc::c_int {
    if !conf.is_null() {} else {
        __assert_fail(
            b"conf\0" as *const u8 as *const libc::c_char,
            b"output_modules/module_csv.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"int csv_init(struct state_conf *, const char **, int)\0"))
                .as_ptr(),
        );
    }
    'c_4387: {
        if !conf.is_null() {} else {
            __assert_fail(
                b"conf\0" as *const u8 as *const libc::c_char,
                b"output_modules/module_csv.c\0" as *const u8 as *const libc::c_char,
                29 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"int csv_init(struct state_conf *, const char **, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*conf).output_filename).is_null() {
        if strcmp((*conf).output_filename, b"-\0" as *const u8 as *const libc::c_char)
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
                    b"csv\0" as *const u8 as *const libc::c_char,
                    b"could not open CSV output file (%s): %s\0" as *const u8
                        as *const libc::c_char,
                    (*conf).output_filename,
                    strerror(*__errno_location()),
                );
            }
        }
    } else {
        file = stdout;
        log_debug(
            b"csv\0" as *const u8 as *const libc::c_char,
            b"no output file selected, will use stdout\0" as *const u8
                as *const libc::c_char,
        );
    }
    if (*conf).no_header_row == 0 {
        log_debug(
            b"csv\0" as *const u8 as *const libc::c_char,
            b"more than one field, will add headers\0" as *const u8
                as *const libc::c_char,
        );
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < fieldlens {
            if i != 0 {
                fprintf(file, b",\0" as *const u8 as *const libc::c_char);
            }
            fprintf(
                file,
                b"%s\0" as *const u8 as *const libc::c_char,
                *fields.offset(i as isize),
            );
            i += 1;
            i;
        }
        fprintf(file, b"\n\0" as *const u8 as *const libc::c_char);
    }
    check_and_log_file_error(file, b"csv\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn csv_close(
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
unsafe extern "C" fn hex_encode(
    mut f: *mut FILE,
    mut readbuf: *mut libc::c_uchar,
    mut len: size_t,
) {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < len {
        fprintf(
            f,
            b"%02x\0" as *const u8 as *const libc::c_char,
            *readbuf.offset(i as isize) as libc::c_int,
        );
        i = i.wrapping_add(1);
        i;
    }
    check_and_log_file_error(f, b"csv\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn csv_process(mut fs: *mut fieldset_t) -> libc::c_int {
    if file.is_null() {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*fs).len {
        let mut f: *mut field_t = &mut *((*fs).fields).as_mut_ptr().offset(i as isize)
            as *mut field_t;
        if i != 0 {
            fprintf(file, b",\0" as *const u8 as *const libc::c_char);
        }
        if (*f).type_0 == 1 as libc::c_int {
            if !(strchr((*f).value.ptr as *mut libc::c_char, ',' as i32)).is_null() {
                fprintf(
                    file,
                    b"\"%s\"\0" as *const u8 as *const libc::c_char,
                    (*f).value.ptr as *mut libc::c_char,
                );
            } else {
                fprintf(
                    file,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    (*f).value.ptr as *mut libc::c_char,
                );
            }
        } else if (*f).type_0 == 2 as libc::c_int {
            fprintf(file, b"%lu\0" as *const u8 as *const libc::c_char, (*f).value.num);
        } else if (*f).type_0 == 7 as libc::c_int {
            fprintf(
                file,
                b"%i\0" as *const u8 as *const libc::c_char,
                (*f).value.num as libc::c_int,
            );
        } else if (*f).type_0 == 3 as libc::c_int {
            hex_encode(file, (*f).value.ptr as *mut libc::c_uchar, (*f).len);
        } else if (*f).type_0 == 4 as libc::c_int {} else {
            log_fatal(
                b"csv\0" as *const u8 as *const libc::c_char,
                b"received unknown output type\0" as *const u8 as *const libc::c_char,
            );
        }
        i += 1;
        i;
    }
    fprintf(file, b"\n\0" as *const u8 as *const libc::c_char);
    fflush(file);
    check_and_log_file_error(file, b"csv\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub static mut module_csv_file: output_module_t = {
    let mut init = output_module {
        name: b"csv\0" as *const u8 as *const libc::c_char,
        supports_dynamic_output: 0 as libc::c_int,
        update_interval: 0 as libc::c_int as libc::c_uint,
        init: Some(
            csv_init
                as unsafe extern "C" fn(
                    *mut state_conf,
                    *mut *const libc::c_char,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        start: None,
        update: None,
        close: Some(
            csv_close
                as unsafe extern "C" fn(
                    *mut state_conf,
                    *mut state_send,
                    *mut state_recv,
                ) -> libc::c_int,
        ),
        process_ip: Some(
            csv_process as unsafe extern "C" fn(*mut fieldset_t) -> libc::c_int,
        ),
        helptext: b"Outputs one or more output fields as a comma-delimited file. By default, the probe module does not filter out duplicates or limit to successful fields, but rather includes all received packets. Fields can be controlled by setting --output-fields. Filtering out failures and duplicate packets can be achieved by setting an --output-filter.\0"
            as *const u8 as *const libc::c_char,
    };
    init
};
