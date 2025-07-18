use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type aesrand;
    pub type iterator;
    pub type probe_module;
    pub type output_module;
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn blocklist_init(
        allowlist: *mut libc::c_char,
        blocklist: *mut libc::c_char,
        allowlist_entries: *mut *mut libc::c_char,
        allowlist_entries_len: size_t,
        blocklist_entries: *mut *mut libc::c_char,
        blocklist_entries_len: size_t,
        ignore_invalid_hosts: libc::c_int,
    ) -> libc::c_int;
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
    fn log_init(
        stream: *mut FILE,
        level: LogLevel,
        syslog_enabled: libc::c_int,
        syslog_app: *const libc::c_char,
    ) -> libc::c_int;
    fn random_bytes(dst: *mut libc::c_void, n: size_t) -> libc::c_int;
    fn parse_max_hosts(max_targets: *mut libc::c_char) -> uint32_t;
    fn enforce_range(
        name: *const libc::c_char,
        v: libc::c_int,
        min: libc::c_int,
        max: libc::c_int,
    );
    fn shard_get_cur_ip(shard: *mut shard_t) -> uint32_t;
    fn shard_get_next_ip(shard: *mut shard_t) -> uint32_t;
    fn iterator_init(
        num_threads: uint8_t,
        shard: uint16_t,
        num_shards: uint16_t,
    ) -> *mut iterator_t;
    fn aesrand_init_from_seed(_: uint64_t) -> *mut aesrand_t;
    fn get_shard(it: *mut iterator_t, thread_id: uint8_t) -> *mut shard_t;
    static mut zconf: state_conf;
    fn cmdline_parser_ext(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        args_info: *mut gengetopt_args_info,
        params: *mut cmdline_parser_params,
    ) -> libc::c_int;
    fn cmdline_parser_print_help();
    fn cmdline_parser_print_version();
    fn cmdline_parser_params_create() -> *mut cmdline_parser_params;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
pub type LogLevel = libc::c_uint;
pub const ZNUM_LOGLEVELS: LogLevel = 6;
pub const ZLOG_TRACE: LogLevel = 5;
pub const ZLOG_DEBUG: LogLevel = 4;
pub const ZLOG_INFO: LogLevel = 3;
pub const ZLOG_WARN: LogLevel = 2;
pub const ZLOG_ERROR: LogLevel = 1;
pub const ZLOG_FATAL: LogLevel = 0;
pub type port_h_t = uint16_t;
pub type macaddr_t = libc::c_uchar;
pub type aesrand_t = aesrand;
pub type shard_complete_cb = Option::<
    unsafe extern "C" fn(uint8_t, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shard {
    pub state: shard_state,
    pub params: shard_params,
    pub current: uint64_t,
    pub iterations: uint64_t,
    pub thread_id: uint8_t,
    pub cb: shard_complete_cb,
    pub arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shard_params {
    pub first: uint64_t,
    pub last: uint64_t,
    pub factor: uint64_t,
    pub modulus: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shard_state {
    pub packets_sent: uint64_t,
    pub hosts_scanned: uint32_t,
    pub max_hosts: uint32_t,
    pub max_packets: uint32_t,
    pub hosts_blocklisted: uint32_t,
    pub hosts_allowlisted: uint32_t,
    pub packets_failed: uint32_t,
    pub first_scanned: uint32_t,
}
pub type shard_t = shard;
pub type iterator_t = iterator;
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
pub struct gengetopt_args_info {
    pub blocklist_file_arg: *mut libc::c_char,
    pub blocklist_file_orig: *mut libc::c_char,
    pub blocklist_file_help: *const libc::c_char,
    pub allowlist_file_arg: *mut libc::c_char,
    pub allowlist_file_orig: *mut libc::c_char,
    pub allowlist_file_help: *const libc::c_char,
    pub log_file_arg: *mut libc::c_char,
    pub log_file_orig: *mut libc::c_char,
    pub log_file_help: *const libc::c_char,
    pub verbosity_arg: libc::c_int,
    pub verbosity_orig: *mut libc::c_char,
    pub verbosity_help: *const libc::c_char,
    pub ignore_blocklist_errors_help: *const libc::c_char,
    pub seed_arg: libc::c_long,
    pub seed_orig: *mut libc::c_char,
    pub seed_help: *const libc::c_char,
    pub max_targets_arg: *mut libc::c_char,
    pub max_targets_orig: *mut libc::c_char,
    pub max_targets_help: *const libc::c_char,
    pub disable_syslog_help: *const libc::c_char,
    pub shards_arg: libc::c_int,
    pub shards_orig: *mut libc::c_char,
    pub shards_help: *const libc::c_char,
    pub shard_arg: libc::c_int,
    pub shard_orig: *mut libc::c_char,
    pub shard_help: *const libc::c_char,
    pub help_help: *const libc::c_char,
    pub version_help: *const libc::c_char,
    pub blocklist_file_given: libc::c_uint,
    pub allowlist_file_given: libc::c_uint,
    pub log_file_given: libc::c_uint,
    pub verbosity_given: libc::c_uint,
    pub ignore_blocklist_errors_given: libc::c_uint,
    pub seed_given: libc::c_uint,
    pub max_targets_given: libc::c_uint,
    pub disable_syslog_given: libc::c_uint,
    pub shards_given: libc::c_uint,
    pub shard_given: libc::c_uint,
    pub help_given: libc::c_uint,
    pub version_given: libc::c_uint,
    pub inputs: *mut *mut libc::c_char,
    pub inputs_num: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmdline_parser_params {
    pub override_0: libc::c_int,
    pub initialize: libc::c_int,
    pub check_required: libc::c_int,
    pub check_ambiguity: libc::c_int,
    pub print_errors: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zit_conf {
    pub blocklist_filename: *mut libc::c_char,
    pub allowlist_filename: *mut libc::c_char,
    pub destination_cidrs: *mut *mut libc::c_char,
    pub destination_cidrs_len: libc::c_int,
    pub log_filename: *mut libc::c_char,
    pub check_duplicates: libc::c_int,
    pub ignore_errors: libc::c_int,
    pub verbosity: libc::c_int,
    pub disable_syslog: libc::c_int,
    pub shard_num: uint16_t,
    pub total_shards: uint16_t,
    pub seed: uint64_t,
    pub aes: *mut aesrand_t,
    pub max_hosts: uint32_t,
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut conf: zit_conf = zit_conf {
        blocklist_filename: 0 as *mut libc::c_char,
        allowlist_filename: 0 as *mut libc::c_char,
        destination_cidrs: 0 as *mut *mut libc::c_char,
        destination_cidrs_len: 0,
        log_filename: 0 as *mut libc::c_char,
        check_duplicates: 0,
        ignore_errors: 0,
        verbosity: 0,
        disable_syslog: 0,
        shard_num: 0,
        total_shards: 0,
        seed: 0,
        aes: 0 as *mut aesrand_t,
        max_hosts: 0,
    };
    memset(
        &mut conf as *mut zit_conf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<zit_conf>() as libc::c_ulong,
    );
    conf.verbosity = 3 as libc::c_int;
    conf.ignore_errors = 0 as libc::c_int;
    let mut args: gengetopt_args_info = gengetopt_args_info {
        blocklist_file_arg: 0 as *mut libc::c_char,
        blocklist_file_orig: 0 as *mut libc::c_char,
        blocklist_file_help: 0 as *const libc::c_char,
        allowlist_file_arg: 0 as *mut libc::c_char,
        allowlist_file_orig: 0 as *mut libc::c_char,
        allowlist_file_help: 0 as *const libc::c_char,
        log_file_arg: 0 as *mut libc::c_char,
        log_file_orig: 0 as *mut libc::c_char,
        log_file_help: 0 as *const libc::c_char,
        verbosity_arg: 0,
        verbosity_orig: 0 as *mut libc::c_char,
        verbosity_help: 0 as *const libc::c_char,
        ignore_blocklist_errors_help: 0 as *const libc::c_char,
        seed_arg: 0,
        seed_orig: 0 as *mut libc::c_char,
        seed_help: 0 as *const libc::c_char,
        max_targets_arg: 0 as *mut libc::c_char,
        max_targets_orig: 0 as *mut libc::c_char,
        max_targets_help: 0 as *const libc::c_char,
        disable_syslog_help: 0 as *const libc::c_char,
        shards_arg: 0,
        shards_orig: 0 as *mut libc::c_char,
        shards_help: 0 as *const libc::c_char,
        shard_arg: 0,
        shard_orig: 0 as *mut libc::c_char,
        shard_help: 0 as *const libc::c_char,
        help_help: 0 as *const libc::c_char,
        version_help: 0 as *const libc::c_char,
        blocklist_file_given: 0,
        allowlist_file_given: 0,
        log_file_given: 0,
        verbosity_given: 0,
        ignore_blocklist_errors_given: 0,
        seed_given: 0,
        max_targets_given: 0,
        disable_syslog_given: 0,
        shards_given: 0,
        shard_given: 0,
        help_given: 0,
        version_given: 0,
        inputs: 0 as *mut *mut libc::c_char,
        inputs_num: 0,
    };
    let mut params: *mut cmdline_parser_params = 0 as *mut cmdline_parser_params;
    params = cmdline_parser_params_create();
    if !params.is_null() {} else {
        __assert_fail(
            b"params\0" as *const u8 as *const libc::c_char,
            b"ziterate.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    }
    'c_6582: {
        if !params.is_null() {} else {
            __assert_fail(
                b"params\0" as *const u8 as *const libc::c_char,
                b"ziterate.c\0" as *const u8 as *const libc::c_char,
                72 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
    };
    (*params).initialize = 1 as libc::c_int;
    (*params).override_0 = 0 as libc::c_int;
    (*params).check_required = 0 as libc::c_int;
    if cmdline_parser_ext(argc, argv, &mut args, params) != 0 as libc::c_int {
        exit(0 as libc::c_int);
    }
    if args.help_given != 0 {
        cmdline_parser_print_help();
        exit(0 as libc::c_int);
    }
    if args.version_given != 0 {
        cmdline_parser_print_version();
        exit(0 as libc::c_int);
    }
    if args.log_file_given != 0 {
        conf.log_filename = strdup(args.log_file_arg);
    }
    if args.verbosity_given != 0 {
        conf.verbosity = args.verbosity_arg;
    }
    if args.ignore_blocklist_errors_given != 0 {
        conf.ignore_errors = 1 as libc::c_int;
    }
    if args.disable_syslog_given != 0 {
        conf.disable_syslog = 1 as libc::c_int;
    }
    let mut logfile: *mut FILE = stderr;
    if !(conf.log_filename).is_null() {
        logfile = fopen(conf.log_filename, b"w\0" as *const u8 as *const libc::c_char);
        if logfile.is_null() {
            fprintf(
                stderr,
                b"FATAL: unable to open specified logfile (%s)\n\0" as *const u8
                    as *const libc::c_char,
                conf.log_filename,
            );
            exit(1 as libc::c_int);
        }
    }
    if log_init(
        logfile,
        conf.verbosity as LogLevel,
        (conf.disable_syslog == 0) as libc::c_int,
        b"ziterate\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        fprintf(
            stderr,
            b"FATAL: unable able to initialize logging\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if args.blocklist_file_given != 0 {
        conf.blocklist_filename = strdup(args.blocklist_file_arg);
    }
    if args.allowlist_file_given != 0 {
        conf.allowlist_filename = strdup(args.allowlist_file_arg);
    }
    conf.destination_cidrs = args.inputs;
    conf.destination_cidrs_len = args.inputs_num as libc::c_int;
    if args.max_targets_given != 0 {
        conf.max_hosts = parse_max_hosts(args.max_targets_arg);
    }
    if !(conf.blocklist_filename).is_null() {
        log_debug(
            b"ziterate\0" as *const u8 as *const libc::c_char,
            b"blocklist file at %s to be used\0" as *const u8 as *const libc::c_char,
            conf.blocklist_filename,
        );
    } else {
        log_debug(
            b"ziterate\0" as *const u8 as *const libc::c_char,
            b"no blocklist file specified\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(conf.blocklist_filename).is_null()
        && access(conf.blocklist_filename, 4 as libc::c_int) == -(1 as libc::c_int)
    {
        log_fatal(
            b"ziterate\0" as *const u8 as *const libc::c_char,
            b"unable to read specified blocklist file (%s)\0" as *const u8
                as *const libc::c_char,
            conf.blocklist_filename,
        );
    }
    if !(conf.allowlist_filename).is_null() {
        log_debug(
            b"ziterate\0" as *const u8 as *const libc::c_char,
            b"allowlist file at %s to be used\0" as *const u8 as *const libc::c_char,
            conf.allowlist_filename,
        );
    } else {
        log_debug(
            b"ziterate\0" as *const u8 as *const libc::c_char,
            b"no allowlist file specified\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(conf.allowlist_filename).is_null()
        && access(conf.allowlist_filename, 4 as libc::c_int) == -(1 as libc::c_int)
    {
        log_fatal(
            b"ziterate\0" as *const u8 as *const libc::c_char,
            b"unable to read specified allowlist file (%s)\0" as *const u8
                as *const libc::c_char,
            conf.allowlist_filename,
        );
    }
    if blocklist_init(
        conf.allowlist_filename,
        conf.blocklist_filename,
        conf.destination_cidrs,
        conf.destination_cidrs_len as size_t,
        0 as *mut *mut libc::c_char,
        0 as libc::c_int as size_t,
        conf.ignore_errors,
    ) != 0
    {
        log_fatal(
            b"ziterate\0" as *const u8 as *const libc::c_char,
            b"unable to initialize blocklist / allowlist\0" as *const u8
                as *const libc::c_char,
        );
    }
    conf.shard_num = 0 as libc::c_int as uint16_t;
    conf.total_shards = 1 as libc::c_int as uint16_t;
    if (args.shard_given != 0 || args.shards_given != 0) && args.seed_given == 0 {
        log_fatal(
            b"ziterate\0" as *const u8 as *const libc::c_char,
            b"Need to specify seed if sharding a scan\0" as *const u8
                as *const libc::c_char,
        );
    }
    if args.shard_given ^ args.shards_given != 0 {
        log_fatal(
            b"ziterate\0" as *const u8 as *const libc::c_char,
            b"Need to specify both shard number and total number of shards\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if args.shard_given != 0 {
        enforce_range(
            b"shard\0" as *const u8 as *const libc::c_char,
            args.shard_arg,
            0 as libc::c_int,
            65534 as libc::c_int,
        );
        conf.shard_num = args.shard_arg as uint16_t;
    }
    if args.shards_given != 0 {
        enforce_range(
            b"shards\0" as *const u8 as *const libc::c_char,
            args.shards_arg,
            1 as libc::c_int,
            65535 as libc::c_int,
        );
        conf.total_shards = args.shards_arg as uint16_t;
    }
    if conf.shard_num as libc::c_int >= conf.total_shards as libc::c_int {
        log_fatal(
            b"ziterate\0" as *const u8 as *const libc::c_char,
            b"With %hhu total shards, shard number (%hhu) must be in range [0, %hhu)\0"
                as *const u8 as *const libc::c_char,
            conf.total_shards as libc::c_int,
            conf.shard_num as libc::c_int,
            conf.total_shards as libc::c_int,
        );
    }
    log_debug(
        b"ziterate\0" as *const u8 as *const libc::c_char,
        b"Initializing sharding (%d shards, shard number %d, seed %llu)\0" as *const u8
            as *const libc::c_char,
        conf.total_shards as libc::c_int,
        conf.shard_num as libc::c_int,
        conf.seed,
    );
    if args.seed_given != 0 {
        conf.seed = args.seed_arg as uint64_t;
    } else if random_bytes(
        &mut conf.seed as *mut uint64_t as *mut libc::c_void,
        ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
    ) == 0
    {
        log_fatal(
            b"ziterate\0" as *const u8 as *const libc::c_char,
            b"unable to generate random bytes needed for seed\0" as *const u8
                as *const libc::c_char,
        );
    }
    zconf.aes = aesrand_init_from_seed(conf.seed);
    let mut it: *mut iterator_t = iterator_init(
        1 as libc::c_int as uint8_t,
        conf.shard_num,
        conf.total_shards,
    );
    let mut shard: *mut shard_t = get_shard(it, 0 as libc::c_int as uint8_t);
    let mut next_int: uint32_t = shard_get_cur_ip(shard);
    let mut next_ip: in_addr = in_addr { s_addr: 0 };
    let mut count: uint32_t = 0 as libc::c_int as uint32_t;
    while next_int != 0 {
        if conf.max_hosts != 0 && count >= conf.max_hosts {
            break;
        }
        next_ip.s_addr = next_int;
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, inet_ntoa(next_ip));
        next_int = shard_get_next_ip(shard);
        count = count.wrapping_add(1);
        count;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
