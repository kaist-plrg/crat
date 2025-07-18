use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type output_module;
    pub type probe_module;
    pub type aesrand;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut zconf: state_conf;
    fn yy_delete_buffer(b: YY_BUFFER_STATE);
    fn yy_scan_string(yy_str: *const libc::c_char) -> YY_BUFFER_STATE;
    fn log_error(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn yyparse() -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type in_addr_t = uint32_t;
pub type aesrand_t = aesrand;
pub type YY_BUFFER_STATE = *mut yy_buffer_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yy_buffer_state {
    pub yy_input_file: *mut FILE,
    pub yy_ch_buf: *mut libc::c_char,
    pub yy_buf_pos: *mut libc::c_char,
    pub yy_buf_size: libc::c_int,
    pub yy_n_chars: libc::c_int,
    pub yy_is_our_buffer: libc::c_int,
    pub yy_is_interactive: libc::c_int,
    pub yy_at_bol: libc::c_int,
    pub yy_bs_lineno: libc::c_int,
    pub yy_bs_column: libc::c_int,
    pub yy_fill_buffer: libc::c_int,
    pub yy_buffer_status: libc::c_int,
}
pub static mut zfilter: *mut node_t = 0 as *const node_t as *mut node_t;
unsafe extern "C" fn validate_node(
    mut node: *mut node_t,
    mut fields: *mut fielddefset_t,
) -> libc::c_int {
    let mut index: libc::c_int = 0;
    let mut found: libc::c_int = 0 as libc::c_int;
    if (*node).type_0 as libc::c_uint == OP as libc::c_int as libc::c_uint {
        if (*node).value.op as libc::c_uint == AND as libc::c_int as libc::c_uint
            || (*node).value.op as libc::c_uint == OR as libc::c_int as libc::c_uint
        {
            return 1 as libc::c_int;
        }
        index = 0 as libc::c_int;
        while index < (*fields).len {
            if !((*fields).fielddefs[index as usize].name).is_null() {
                if strcmp(
                    (*fields).fielddefs[index as usize].name,
                    (*(*node).left_child).value.field.fieldname,
                ) == 0 as libc::c_int
                {
                    (*(*node).left_child).value.field.index = index;
                    found = 1 as libc::c_int;
                    break;
                }
            }
            index += 1;
            index;
        }
        if found == 0 {
            fprintf(
                stderr,
                b"Field '%s' does not exist\n\0" as *const u8 as *const libc::c_char,
                (*(*node).left_child).value.field.fieldname,
            );
            return 0 as libc::c_int;
        }
        match (*(*node).right_child).type_0 as libc::c_uint {
            2 => {
                if strcmp(
                    (*fields).fielddefs[index as usize].type_0,
                    b"string\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    return 1 as libc::c_int
                } else {
                    fprintf(
                        stderr,
                        b"Field '%s' is not of type 'string'\n\0" as *const u8
                            as *const libc::c_char,
                        (*fields).fielddefs[index as usize].name,
                    );
                    return 0 as libc::c_int;
                }
            }
            3 => {
                if strcmp(
                    (*fields).fielddefs[index as usize].type_0,
                    b"int\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                    || strcmp(
                        (*fields).fielddefs[index as usize].type_0,
                        b"bool\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                {
                    return 1 as libc::c_int
                } else {
                    fprintf(
                        stderr,
                        b"Field '%s' is not of type 'int'\n\0" as *const u8
                            as *const libc::c_char,
                        (*fields).fielddefs[index as usize].name,
                    );
                    return 0 as libc::c_int;
                }
            }
            _ => return 0 as libc::c_int,
        }
    } else {
        return 1 as libc::c_int
    };
}
pub unsafe extern "C" fn parse_filter_string(
    mut filter: *mut libc::c_char,
) -> libc::c_int {
    let mut buffer_state: YY_BUFFER_STATE = yy_scan_string(filter);
    let mut status: libc::c_int = yyparse();
    yy_delete_buffer(buffer_state);
    if status != 0 {
        log_error(
            b"zmap\0" as *const u8 as *const libc::c_char,
            b"Unable to parse filter string: '%s'\0" as *const u8 as *const libc::c_char,
            filter,
        );
        return 0 as libc::c_int;
    }
    zconf.filter.expression = zfilter;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn validate_filter(
    mut root: *mut node_t,
    mut fields: *mut fielddefset_t,
) -> libc::c_int {
    let mut valid: libc::c_int = 0;
    if root.is_null() {
        return 1 as libc::c_int;
    }
    valid = validate_node(root, fields);
    if valid == 0 {
        return 0 as libc::c_int;
    }
    return (validate_filter((*root).left_child, fields) != 0
        && validate_filter((*root).right_child, fields) != 0) as libc::c_int;
}
