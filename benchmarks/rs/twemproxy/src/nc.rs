use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type epoll_event;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn getpid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn fork() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut opterr: libc::c_int;
    static mut optopt: libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn uname(__name: *mut utsname) -> libc::c_int;
    fn core_loop(ctx: *mut context) -> rstatus_t;
    fn core_stop(ctx: *mut context);
    fn core_start(nci: *mut instance) -> *mut context;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn _nc_atoi(line: *const uint8_t, n: size_t) -> libc::c_int;
    fn nc_valid_port(n: libc::c_int) -> bool;
    fn log_init(level: libc::c_int, filename: *const libc::c_char) -> libc::c_int;
    fn log_deinit();
    fn log_loggable(level: libc::c_int) -> libc::c_int;
    fn _log(
        file: *const libc::c_char,
        line: libc::c_int,
        panic: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn _log_stderr(fmt: *const libc::c_char, _: ...);
    fn stats_describe();
    fn conf_create(filename: *const libc::c_char) -> *mut conf;
    fn conf_destroy(cf: *mut conf);
    fn signal_init() -> rstatus_t;
    fn signal_deinit();
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
pub type pid_t = __pid_t;
pub type int64_t = __int64_t;
pub type pthread_t = libc::c_ulong;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub domainname: [libc::c_char; 65],
}
pub type rstatus_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array {
    pub nelem: uint32_t,
    pub elem: *mut libc::c_void,
    pub size: size_t,
    pub nalloc: uint32_t,
}
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string {
    pub len: uint32_t,
    pub data: *mut uint8_t,
}
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context {
    pub id: uint32_t,
    pub cf: *mut conf,
    pub stats: *mut stats,
    pub pool: array,
    pub evb: *mut event_base,
    pub max_timeout: libc::c_int,
    pub timeout: libc::c_int,
    pub max_nfd: uint32_t,
    pub max_ncconn: uint32_t,
    pub max_nsconn: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_base {
    pub ep: libc::c_int,
    pub event: *mut epoll_event,
    pub nevent: libc::c_int,
    pub cb: event_cb_t,
}
pub type event_cb_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, uint32_t) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats {
    pub port: uint16_t,
    pub interval: libc::c_int,
    pub addr: string,
    pub start_ts: int64_t,
    pub buf: stats_buffer,
    pub current: array,
    pub shadow: array,
    pub sum: array,
    pub tid: pthread_t,
    pub sd: libc::c_int,
    pub service_str: string,
    pub service: string,
    pub source_str: string,
    pub source: string,
    pub version_str: string,
    pub version: string,
    pub uptime_str: string,
    pub timestamp_str: string,
    pub ntotal_conn_str: string,
    pub ncurr_conn_str: string,
    pub aggregate: libc::c_int,
    pub updated: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_buffer {
    pub len: size_t,
    pub data: *mut uint8_t,
    pub size: size_t,
}
pub type uint16_t = __uint16_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct conf {
    pub fname: *const libc::c_char,
    pub fh: *mut FILE,
    pub arg: array,
    pub pool: array,
    pub depth: uint32_t,
    pub parser: yaml_parser_t,
    pub event: yaml_event_t,
    pub token: yaml_token_t,
    #[bitfield(name = "seq", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "valid_parser", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "valid_event", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "valid_token", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "sound", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "parsed", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "valid", ty = "libc::c_uint", bits = "6..=6")]
    pub seq_valid_parser_valid_event_valid_token_sound_parsed_valid: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type yaml_token_t = yaml_token_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_token_s {
    pub type_0: yaml_token_type_t,
    pub data: C2RustUnnamed,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
pub type yaml_mark_t = yaml_mark_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_mark_s {
    pub index: size_t,
    pub line: size_t,
    pub column: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub stream_start: C2RustUnnamed_6,
    pub alias: C2RustUnnamed_5,
    pub anchor: C2RustUnnamed_4,
    pub tag: C2RustUnnamed_3,
    pub scalar: C2RustUnnamed_2,
    pub version_directive: C2RustUnnamed_1,
    pub tag_directive: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub handle: *mut yaml_char_t,
    pub prefix: *mut yaml_char_t,
}
pub type yaml_char_t = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub major: libc::c_int,
    pub minor: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}
pub type yaml_scalar_style_t = yaml_scalar_style_e;
pub type yaml_scalar_style_e = libc::c_uint;
pub const YAML_FOLDED_SCALAR_STYLE: yaml_scalar_style_e = 5;
pub const YAML_LITERAL_SCALAR_STYLE: yaml_scalar_style_e = 4;
pub const YAML_DOUBLE_QUOTED_SCALAR_STYLE: yaml_scalar_style_e = 3;
pub const YAML_SINGLE_QUOTED_SCALAR_STYLE: yaml_scalar_style_e = 2;
pub const YAML_PLAIN_SCALAR_STYLE: yaml_scalar_style_e = 1;
pub const YAML_ANY_SCALAR_STYLE: yaml_scalar_style_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub handle: *mut yaml_char_t,
    pub suffix: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub value: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub value: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub encoding: yaml_encoding_t,
}
pub type yaml_encoding_t = yaml_encoding_e;
pub type yaml_encoding_e = libc::c_uint;
pub const YAML_UTF16BE_ENCODING: yaml_encoding_e = 3;
pub const YAML_UTF16LE_ENCODING: yaml_encoding_e = 2;
pub const YAML_UTF8_ENCODING: yaml_encoding_e = 1;
pub const YAML_ANY_ENCODING: yaml_encoding_e = 0;
pub type yaml_token_type_t = yaml_token_type_e;
pub type yaml_token_type_e = libc::c_uint;
pub const YAML_SCALAR_TOKEN: yaml_token_type_e = 21;
pub const YAML_TAG_TOKEN: yaml_token_type_e = 20;
pub const YAML_ANCHOR_TOKEN: yaml_token_type_e = 19;
pub const YAML_ALIAS_TOKEN: yaml_token_type_e = 18;
pub const YAML_VALUE_TOKEN: yaml_token_type_e = 17;
pub const YAML_KEY_TOKEN: yaml_token_type_e = 16;
pub const YAML_FLOW_ENTRY_TOKEN: yaml_token_type_e = 15;
pub const YAML_BLOCK_ENTRY_TOKEN: yaml_token_type_e = 14;
pub const YAML_FLOW_MAPPING_END_TOKEN: yaml_token_type_e = 13;
pub const YAML_FLOW_MAPPING_START_TOKEN: yaml_token_type_e = 12;
pub const YAML_FLOW_SEQUENCE_END_TOKEN: yaml_token_type_e = 11;
pub const YAML_FLOW_SEQUENCE_START_TOKEN: yaml_token_type_e = 10;
pub const YAML_BLOCK_END_TOKEN: yaml_token_type_e = 9;
pub const YAML_BLOCK_MAPPING_START_TOKEN: yaml_token_type_e = 8;
pub const YAML_BLOCK_SEQUENCE_START_TOKEN: yaml_token_type_e = 7;
pub const YAML_DOCUMENT_END_TOKEN: yaml_token_type_e = 6;
pub const YAML_DOCUMENT_START_TOKEN: yaml_token_type_e = 5;
pub const YAML_TAG_DIRECTIVE_TOKEN: yaml_token_type_e = 4;
pub const YAML_VERSION_DIRECTIVE_TOKEN: yaml_token_type_e = 3;
pub const YAML_STREAM_END_TOKEN: yaml_token_type_e = 2;
pub const YAML_STREAM_START_TOKEN: yaml_token_type_e = 1;
pub const YAML_NO_TOKEN: yaml_token_type_e = 0;
pub type yaml_event_t = yaml_event_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_event_s {
    pub type_0: yaml_event_type_t,
    pub data: C2RustUnnamed_7,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub stream_start: C2RustUnnamed_15,
    pub document_start: C2RustUnnamed_13,
    pub document_end: C2RustUnnamed_12,
    pub alias: C2RustUnnamed_11,
    pub scalar: C2RustUnnamed_10,
    pub sequence_start: C2RustUnnamed_9,
    pub mapping_start: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: libc::c_int,
    pub style: yaml_mapping_style_t,
}
pub type yaml_mapping_style_t = yaml_mapping_style_e;
pub type yaml_mapping_style_e = libc::c_uint;
pub const YAML_FLOW_MAPPING_STYLE: yaml_mapping_style_e = 2;
pub const YAML_BLOCK_MAPPING_STYLE: yaml_mapping_style_e = 1;
pub const YAML_ANY_MAPPING_STYLE: yaml_mapping_style_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: libc::c_int,
    pub style: yaml_sequence_style_t,
}
pub type yaml_sequence_style_t = yaml_sequence_style_e;
pub type yaml_sequence_style_e = libc::c_uint;
pub const YAML_FLOW_SEQUENCE_STYLE: yaml_sequence_style_e = 2;
pub const YAML_BLOCK_SEQUENCE_STYLE: yaml_sequence_style_e = 1;
pub const YAML_ANY_SEQUENCE_STYLE: yaml_sequence_style_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub plain_implicit: libc::c_int,
    pub quoted_implicit: libc::c_int,
    pub style: yaml_scalar_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub anchor: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub implicit: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: C2RustUnnamed_14,
    pub implicit: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
pub type yaml_tag_directive_t = yaml_tag_directive_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_tag_directive_s {
    pub handle: *mut yaml_char_t,
    pub prefix: *mut yaml_char_t,
}
pub type yaml_version_directive_t = yaml_version_directive_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_version_directive_s {
    pub major: libc::c_int,
    pub minor: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub encoding: yaml_encoding_t,
}
pub type yaml_event_type_t = yaml_event_type_e;
pub type yaml_event_type_e = libc::c_uint;
pub const YAML_MAPPING_END_EVENT: yaml_event_type_e = 10;
pub const YAML_MAPPING_START_EVENT: yaml_event_type_e = 9;
pub const YAML_SEQUENCE_END_EVENT: yaml_event_type_e = 8;
pub const YAML_SEQUENCE_START_EVENT: yaml_event_type_e = 7;
pub const YAML_SCALAR_EVENT: yaml_event_type_e = 6;
pub const YAML_ALIAS_EVENT: yaml_event_type_e = 5;
pub const YAML_DOCUMENT_END_EVENT: yaml_event_type_e = 4;
pub const YAML_DOCUMENT_START_EVENT: yaml_event_type_e = 3;
pub const YAML_STREAM_END_EVENT: yaml_event_type_e = 2;
pub const YAML_STREAM_START_EVENT: yaml_event_type_e = 1;
pub const YAML_NO_EVENT: yaml_event_type_e = 0;
pub type yaml_parser_t = yaml_parser_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_parser_s {
    pub error: yaml_error_type_t,
    pub problem: *const libc::c_char,
    pub problem_offset: size_t,
    pub problem_value: libc::c_int,
    pub problem_mark: yaml_mark_t,
    pub context: *const libc::c_char,
    pub context_mark: yaml_mark_t,
    pub read_handler: Option::<yaml_read_handler_t>,
    pub read_handler_data: *mut libc::c_void,
    pub input: C2RustUnnamed_33,
    pub eof: libc::c_int,
    pub buffer: C2RustUnnamed_32,
    pub unread: size_t,
    pub raw_buffer: C2RustUnnamed_31,
    pub encoding: yaml_encoding_t,
    pub offset: size_t,
    pub mark: yaml_mark_t,
    pub stream_start_produced: libc::c_int,
    pub stream_end_produced: libc::c_int,
    pub flow_level: libc::c_int,
    pub tokens: C2RustUnnamed_30,
    pub tokens_parsed: size_t,
    pub token_available: libc::c_int,
    pub indents: C2RustUnnamed_29,
    pub indent: libc::c_int,
    pub simple_key_allowed: libc::c_int,
    pub simple_keys: C2RustUnnamed_28,
    pub states: C2RustUnnamed_27,
    pub state: yaml_parser_state_t,
    pub marks: C2RustUnnamed_26,
    pub tag_directives: C2RustUnnamed_25,
    pub aliases: C2RustUnnamed_24,
    pub document: *mut yaml_document_t,
}
pub type yaml_document_t = yaml_document_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_document_s {
    pub nodes: C2RustUnnamed_17,
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: C2RustUnnamed_16,
    pub start_implicit: libc::c_int,
    pub end_implicit: libc::c_int,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub start: *mut yaml_node_t,
    pub end: *mut yaml_node_t,
    pub top: *mut yaml_node_t,
}
pub type yaml_node_t = yaml_node_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_node_s {
    pub type_0: yaml_node_type_t,
    pub tag: *mut yaml_char_t,
    pub data: C2RustUnnamed_18,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_18 {
    pub scalar: C2RustUnnamed_23,
    pub sequence: C2RustUnnamed_21,
    pub mapping: C2RustUnnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub pairs: C2RustUnnamed_20,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub start: *mut yaml_node_pair_t,
    pub end: *mut yaml_node_pair_t,
    pub top: *mut yaml_node_pair_t,
}
pub type yaml_node_pair_t = yaml_node_pair_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_node_pair_s {
    pub key: libc::c_int,
    pub value: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub items: C2RustUnnamed_22,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}
pub type yaml_node_item_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}
pub type yaml_node_type_t = yaml_node_type_e;
pub type yaml_node_type_e = libc::c_uint;
pub const YAML_MAPPING_NODE: yaml_node_type_e = 3;
pub const YAML_SEQUENCE_NODE: yaml_node_type_e = 2;
pub const YAML_SCALAR_NODE: yaml_node_type_e = 1;
pub const YAML_NO_NODE: yaml_node_type_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub start: *mut yaml_alias_data_t,
    pub end: *mut yaml_alias_data_t,
    pub top: *mut yaml_alias_data_t,
}
pub type yaml_alias_data_t = yaml_alias_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_alias_data_s {
    pub anchor: *mut yaml_char_t,
    pub index: libc::c_int,
    pub mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub start: *mut yaml_mark_t,
    pub end: *mut yaml_mark_t,
    pub top: *mut yaml_mark_t,
}
pub type yaml_parser_state_t = yaml_parser_state_e;
pub type yaml_parser_state_e = libc::c_uint;
pub const YAML_PARSE_END_STATE: yaml_parser_state_e = 23;
pub const YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE: yaml_parser_state_e = 22;
pub const YAML_PARSE_FLOW_MAPPING_VALUE_STATE: yaml_parser_state_e = 21;
pub const YAML_PARSE_FLOW_MAPPING_KEY_STATE: yaml_parser_state_e = 20;
pub const YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE: yaml_parser_state_e = 19;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE: yaml_parser_state_e = 18;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE: yaml_parser_state_e = 17;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE: yaml_parser_state_e = 16;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE: yaml_parser_state_e = 15;
pub const YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE: yaml_parser_state_e = 14;
pub const YAML_PARSE_BLOCK_MAPPING_VALUE_STATE: yaml_parser_state_e = 13;
pub const YAML_PARSE_BLOCK_MAPPING_KEY_STATE: yaml_parser_state_e = 12;
pub const YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE: yaml_parser_state_e = 11;
pub const YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE: yaml_parser_state_e = 10;
pub const YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE: yaml_parser_state_e = 9;
pub const YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE: yaml_parser_state_e = 8;
pub const YAML_PARSE_FLOW_NODE_STATE: yaml_parser_state_e = 7;
pub const YAML_PARSE_BLOCK_NODE_OR_INDENTLESS_SEQUENCE_STATE: yaml_parser_state_e = 6;
pub const YAML_PARSE_BLOCK_NODE_STATE: yaml_parser_state_e = 5;
pub const YAML_PARSE_DOCUMENT_END_STATE: yaml_parser_state_e = 4;
pub const YAML_PARSE_DOCUMENT_CONTENT_STATE: yaml_parser_state_e = 3;
pub const YAML_PARSE_DOCUMENT_START_STATE: yaml_parser_state_e = 2;
pub const YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE: yaml_parser_state_e = 1;
pub const YAML_PARSE_STREAM_START_STATE: yaml_parser_state_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub start: *mut yaml_parser_state_t,
    pub end: *mut yaml_parser_state_t,
    pub top: *mut yaml_parser_state_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub start: *mut yaml_simple_key_t,
    pub end: *mut yaml_simple_key_t,
    pub top: *mut yaml_simple_key_t,
}
pub type yaml_simple_key_t = yaml_simple_key_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_simple_key_s {
    pub possible: libc::c_int,
    pub required: libc::c_int,
    pub token_number: size_t,
    pub mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub start: *mut libc::c_int,
    pub end: *mut libc::c_int,
    pub top: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub start: *mut yaml_token_t,
    pub end: *mut yaml_token_t,
    pub head: *mut yaml_token_t,
    pub tail: *mut yaml_token_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub start: *mut libc::c_uchar,
    pub end: *mut libc::c_uchar,
    pub pointer: *mut libc::c_uchar,
    pub last: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_33 {
    pub string: C2RustUnnamed_34,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub start: *const libc::c_uchar,
    pub end: *const libc::c_uchar,
    pub current: *const libc::c_uchar,
}
pub type yaml_read_handler_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_uchar,
    size_t,
    *mut size_t,
) -> libc::c_int;
pub type yaml_error_type_t = yaml_error_type_e;
pub type yaml_error_type_e = libc::c_uint;
pub const YAML_EMITTER_ERROR: yaml_error_type_e = 7;
pub const YAML_WRITER_ERROR: yaml_error_type_e = 6;
pub const YAML_COMPOSER_ERROR: yaml_error_type_e = 5;
pub const YAML_PARSER_ERROR: yaml_error_type_e = 4;
pub const YAML_SCANNER_ERROR: yaml_error_type_e = 3;
pub const YAML_READER_ERROR: yaml_error_type_e = 2;
pub const YAML_MEMORY_ERROR: yaml_error_type_e = 1;
pub const YAML_NO_ERROR: yaml_error_type_e = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct instance {
    pub ctx: *mut context,
    pub log_level: libc::c_int,
    pub log_filename: *const libc::c_char,
    pub conf_filename: *const libc::c_char,
    pub stats_port: uint16_t,
    pub stats_interval: libc::c_int,
    pub stats_addr: *const libc::c_char,
    pub hostname: [libc::c_char; 256],
    pub mbuf_chunk_size: size_t,
    pub pid: pid_t,
    pub pid_filename: *const libc::c_char,
    #[bitfield(name = "pidfile", ty = "libc::c_uint", bits = "0..=0")]
    pub pidfile: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
static mut show_help: libc::c_int = 0;
static mut show_version: libc::c_int = 0;
static mut test_conf: libc::c_int = 0;
static mut daemonize: libc::c_int = 0;
static mut describe_stats: libc::c_int = 0;
static mut long_options: [option; 14] = [
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'V' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"test-conf\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"daemonize\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"describe-stats\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'D' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"output\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"conf-file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"stats-port\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"stats-interval\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"stats-addr\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"pid-file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"mbuf-size\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'm' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
static mut short_options: [libc::c_char; 22] = unsafe {
    *::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"hVtdDv:o:c:s:i:a:p:m:\0")
};
unsafe extern "C" fn nc_daemonize(mut dump_core: libc::c_int) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut pid: pid_t = 0;
    let mut sid: pid_t = 0;
    let mut fd: libc::c_int = 0;
    pid = fork();
    match pid {
        -1 => {
            if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
                _log(
                    b"nc.c\0" as *const u8 as *const libc::c_char,
                    83 as libc::c_int,
                    0 as libc::c_int,
                    b"fork() failed: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            return -(1 as libc::c_int);
        }
        0 => {}
        _ => {
            _exit(0 as libc::c_int);
        }
    }
    sid = setsid();
    if sid < 0 as libc::c_int {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc.c\0" as *const u8 as *const libc::c_char,
                98 as libc::c_int,
                0 as libc::c_int,
                b"setsid() failed: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return -(1 as libc::c_int);
    }
    if signal(
        1 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    )
        == ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(-(1 as libc::c_int) as libc::intptr_t)
    {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc.c\0" as *const u8 as *const libc::c_char,
                103 as libc::c_int,
                0 as libc::c_int,
                b"signal(SIGHUP, SIG_IGN) failed: %s\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return -(1 as libc::c_int);
    }
    pid = fork();
    match pid {
        -1 => {
            if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
                _log(
                    b"nc.c\0" as *const u8 as *const libc::c_char,
                    110 as libc::c_int,
                    0 as libc::c_int,
                    b"fork() failed: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            return -(1 as libc::c_int);
        }
        0 => {}
        _ => {
            _exit(0 as libc::c_int);
        }
    }
    if dump_core == 0 as libc::c_int {
        status = chdir(b"/\0" as *const u8 as *const libc::c_char);
        if status < 0 as libc::c_int {
            if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
                _log(
                    b"nc.c\0" as *const u8 as *const libc::c_char,
                    127 as libc::c_int,
                    0 as libc::c_int,
                    b"chdir(\"/\") failed: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            return -(1 as libc::c_int);
        }
    }
    umask(0 as libc::c_int as __mode_t);
    fd = open(b"/dev/null\0" as *const u8 as *const libc::c_char, 0o2 as libc::c_int);
    if fd < 0 as libc::c_int {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc.c\0" as *const u8 as *const libc::c_char,
                139 as libc::c_int,
                0 as libc::c_int,
                b"open(\"/dev/null\") failed: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return -(1 as libc::c_int);
    }
    status = dup2(fd, 0 as libc::c_int);
    if status < 0 as libc::c_int {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc.c\0" as *const u8 as *const libc::c_char,
                145 as libc::c_int,
                0 as libc::c_int,
                b"dup2(%d, STDIN) failed: %s\0" as *const u8 as *const libc::c_char,
                fd,
                strerror(*__errno_location()),
            );
        }
        close(fd);
        return -(1 as libc::c_int);
    }
    status = dup2(fd, 1 as libc::c_int);
    if status < 0 as libc::c_int {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc.c\0" as *const u8 as *const libc::c_char,
                152 as libc::c_int,
                0 as libc::c_int,
                b"dup2(%d, STDOUT) failed: %s\0" as *const u8 as *const libc::c_char,
                fd,
                strerror(*__errno_location()),
            );
        }
        close(fd);
        return -(1 as libc::c_int);
    }
    status = dup2(fd, 2 as libc::c_int);
    if status < 0 as libc::c_int {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc.c\0" as *const u8 as *const libc::c_char,
                159 as libc::c_int,
                0 as libc::c_int,
                b"dup2(%d, STDERR) failed: %s\0" as *const u8 as *const libc::c_char,
                fd,
                strerror(*__errno_location()),
            );
        }
        close(fd);
        return -(1 as libc::c_int);
    }
    if fd > 2 as libc::c_int {
        status = close(fd);
        if status < 0 as libc::c_int {
            if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
                _log(
                    b"nc.c\0" as *const u8 as *const libc::c_char,
                    167 as libc::c_int,
                    0 as libc::c_int,
                    b"close(%d) failed: %s\0" as *const u8 as *const libc::c_char,
                    fd,
                    strerror(*__errno_location()),
                );
            }
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn nc_print_run(mut nci: *const instance) {
    let mut status: libc::c_int = 0;
    let mut name: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    status = uname(&mut name);
    if status < 0 as libc::c_int {
        _log(
            b"nc.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            0 as libc::c_int,
            b"nutcracker-%s started on pid %d\0" as *const u8 as *const libc::c_char,
            b"0.5.0\0" as *const u8 as *const libc::c_char,
            (*nci).pid,
        );
    } else {
        _log(
            b"nc.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            0 as libc::c_int,
            b"nutcracker-%s built for %s %s %s started on pid %d\0" as *const u8
                as *const libc::c_char,
            b"0.5.0\0" as *const u8 as *const libc::c_char,
            (name.sysname).as_mut_ptr(),
            (name.release).as_mut_ptr(),
            (name.machine).as_mut_ptr(),
            (*nci).pid,
        );
    }
    _log(
        b"nc.c\0" as *const u8 as *const libc::c_char,
        192 as libc::c_int,
        0 as libc::c_int,
        b"run, rabbit run / dig that hole, forget the sun / and when at last the work is done / don't sit down / it's time to dig another one\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn nc_print_done() {
    _log(
        b"nc.c\0" as *const u8 as *const libc::c_char,
        198 as libc::c_int,
        0 as libc::c_int,
        b"done, rabbit done\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn nc_show_usage() {
    _log_stderr(
        b"Usage: nutcracker [-?hVdDt] [-v verbosity level] [-o output file]\r\n                  [-c conf file] [-s stats port] [-a stats addr]\r\n                  [-i stats interval] [-p pid file] [-m mbuf size]\r\n\0"
            as *const u8 as *const libc::c_char,
    );
    _log_stderr(
        b"Options:\r\n  -h, --help             : this help\r\n  -V, --version          : show version and exit\r\n  -t, --test-conf        : test configuration for syntax errors and exit\r\n  -d, --daemonize        : run as a daemon\r\n  -D, --describe-stats   : print stats description and exit\0"
            as *const u8 as *const libc::c_char,
    );
    _log_stderr(
        b"  -v, --verbose=N        : set logging level (default: %d, min: %d, max: %d)\r\n  -o, --output=S         : set logging file (default: %s)\r\n  -c, --conf-file=S      : set configuration file (default: %s)\r\n  -s, --stats-port=N     : set stats monitoring port (default: %d)\r\n  -a, --stats-addr=S     : set stats monitoring ip (default: %s)\r\n  -i, --stats-interval=N : set stats aggregation interval in msec (default: %d msec)\r\n  -p, --pid-file=S       : set pid file (default: %s)\r\n  -m, --mbuf-size=N      : set size of mbuf chunk in bytes (default: %d bytes)\r\n\0"
            as *const u8 as *const libc::c_char,
        5 as libc::c_int,
        0 as libc::c_int,
        11 as libc::c_int,
        if !(0 as *mut libc::c_void).is_null() {
            0 as *const libc::c_char
        } else {
            b"stderr\0" as *const u8 as *const libc::c_char
        },
        b"conf/nutcracker.yml\0" as *const u8 as *const libc::c_char,
        22222 as libc::c_int,
        b"0.0.0.0\0" as *const u8 as *const libc::c_char,
        30 as libc::c_int * 1000 as libc::c_int,
        if !(0 as *mut libc::c_void).is_null() {
            0 as *const libc::c_char
        } else {
            b"off\0" as *const u8 as *const libc::c_char
        },
        16384 as libc::c_int,
    );
}
unsafe extern "C" fn nc_create_pidfile(mut nci: *mut instance) -> rstatus_t {
    let mut pid: [libc::c_char; 21] = [0; 21];
    let mut fd: libc::c_int = 0;
    let mut pid_len: libc::c_int = 0;
    let mut n: ssize_t = 0;
    fd = open(
        (*nci).pid_filename,
        0o1 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
        0o644 as libc::c_int,
    );
    if fd < 0 as libc::c_int {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc.c\0" as *const u8 as *const libc::c_char,
                244 as libc::c_int,
                0 as libc::c_int,
                b"opening pid file '%s' failed: %s\0" as *const u8
                    as *const libc::c_char,
                (*nci).pid_filename,
                strerror(*__errno_location()),
            );
        }
        return -(1 as libc::c_int);
    }
    (*nci).set_pidfile(1 as libc::c_int as libc::c_uint);
    pid_len = snprintf(
        pid.as_mut_ptr(),
        (20 as libc::c_int + 1 as libc::c_int) as size_t,
        b"%d\0" as *const u8 as *const libc::c_char,
        (*nci).pid,
    );
    n = write(fd, pid.as_mut_ptr() as *const libc::c_void, pid_len as size_t);
    if n < 0 as libc::c_int as libc::c_long {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc.c\0" as *const u8 as *const libc::c_char,
                254 as libc::c_int,
                0 as libc::c_int,
                b"write to pid file '%s' failed: %s\0" as *const u8
                    as *const libc::c_char,
                (*nci).pid_filename,
                strerror(*__errno_location()),
            );
        }
        return -(1 as libc::c_int);
    }
    close(fd);
    return 0 as libc::c_int;
}
unsafe extern "C" fn nc_remove_pidfile(mut nci: *mut instance) {
    let mut status: libc::c_int = 0;
    status = unlink((*nci).pid_filename);
    if status < 0 as libc::c_int {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc.c\0" as *const u8 as *const libc::c_char,
                271 as libc::c_int,
                0 as libc::c_int,
                b"unlink of pid file '%s' failed, ignored: %s\0" as *const u8
                    as *const libc::c_char,
                (*nci).pid_filename,
                strerror(*__errno_location()),
            );
        }
    }
}
unsafe extern "C" fn nc_set_default_options(mut nci: *mut instance) {
    let mut status: libc::c_int = 0;
    (*nci).ctx = 0 as *mut context;
    (*nci).log_level = 5 as libc::c_int;
    (*nci).log_filename = 0 as *const libc::c_char;
    (*nci).conf_filename = b"conf/nutcracker.yml\0" as *const u8 as *const libc::c_char;
    (*nci).stats_port = 22222 as libc::c_int as uint16_t;
    (*nci).stats_addr = b"0.0.0.0\0" as *const u8 as *const libc::c_char;
    (*nci).stats_interval = 30 as libc::c_int * 1000 as libc::c_int;
    status = gethostname(((*nci).hostname).as_mut_ptr(), 256 as libc::c_int as size_t);
    if status < 0 as libc::c_int {
        if log_loggable(4 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc.c\0" as *const u8 as *const libc::c_char,
                293 as libc::c_int,
                0 as libc::c_int,
                b"gethostname failed, ignored: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        snprintf(
            ((*nci).hostname).as_mut_ptr(),
            256 as libc::c_int as size_t,
            b"unknown\0" as *const u8 as *const libc::c_char,
        );
    }
    (*nci)
        .hostname[(256 as libc::c_int - 1 as libc::c_int)
        as usize] = '\0' as i32 as libc::c_char;
    (*nci).mbuf_chunk_size = 16384 as libc::c_int as size_t;
    (*nci).pid = -(1 as libc::c_int);
    (*nci).pid_filename = 0 as *const libc::c_char;
    (*nci).set_pidfile(0 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn nc_get_options(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut nci: *mut instance,
) -> rstatus_t {
    let mut c: libc::c_int = 0;
    let mut value: libc::c_int = 0;
    opterr = 0 as libc::c_int;
    loop {
        c = getopt_long(
            argc,
            argv,
            short_options.as_ptr(),
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if c == -(1 as libc::c_int) {
            break;
        }
        match c {
            104 => {
                show_version = 1 as libc::c_int;
                show_help = 1 as libc::c_int;
            }
            86 => {
                show_version = 1 as libc::c_int;
            }
            116 => {
                test_conf = 1 as libc::c_int;
            }
            100 => {
                daemonize = 1 as libc::c_int;
            }
            68 => {
                describe_stats = 1 as libc::c_int;
                show_version = 1 as libc::c_int;
            }
            118 => {
                value = _nc_atoi(optarg as *mut uint8_t, strlen(optarg));
                if value < 0 as libc::c_int {
                    _log_stderr(
                        b"nutcracker: option -v requires a number\0" as *const u8
                            as *const libc::c_char,
                    );
                    return -(1 as libc::c_int);
                }
                (*nci).log_level = value;
            }
            111 => {
                (*nci).log_filename = optarg;
            }
            99 => {
                (*nci).conf_filename = optarg;
            }
            115 => {
                value = _nc_atoi(optarg as *mut uint8_t, strlen(optarg));
                if value < 0 as libc::c_int {
                    _log_stderr(
                        b"nutcracker: option -s requires a number\0" as *const u8
                            as *const libc::c_char,
                    );
                    return -(1 as libc::c_int);
                }
                if !nc_valid_port(value) {
                    _log_stderr(
                        b"nutcracker: option -s value %d is not a valid port\0"
                            as *const u8 as *const libc::c_char,
                        value,
                    );
                    return -(1 as libc::c_int);
                }
                (*nci).stats_port = value as uint16_t;
            }
            105 => {
                value = _nc_atoi(optarg as *mut uint8_t, strlen(optarg));
                if value < 0 as libc::c_int {
                    _log_stderr(
                        b"nutcracker: option -i requires a number\0" as *const u8
                            as *const libc::c_char,
                    );
                    return -(1 as libc::c_int);
                }
                (*nci).stats_interval = value;
            }
            97 => {
                (*nci).stats_addr = optarg;
            }
            112 => {
                (*nci).pid_filename = optarg;
            }
            109 => {
                value = _nc_atoi(optarg as *mut uint8_t, strlen(optarg));
                if value <= 0 as libc::c_int {
                    _log_stderr(
                        b"nutcracker: option -m requires a non-zero number\0"
                            as *const u8 as *const libc::c_char,
                    );
                    return -(1 as libc::c_int);
                }
                if value < 512 as libc::c_int || value > 16777216 as libc::c_int {
                    _log_stderr(
                        b"nutcracker: mbuf chunk size must be between %d and %d bytes\0"
                            as *const u8 as *const libc::c_char,
                        512 as libc::c_int,
                        16777216 as libc::c_int,
                    );
                    return -(1 as libc::c_int);
                }
                (*nci).mbuf_chunk_size = value as size_t;
            }
            63 => {
                match optopt {
                    111 | 99 | 112 => {
                        _log_stderr(
                            b"nutcracker: option -%c requires a file name\0" as *const u8
                                as *const libc::c_char,
                            optopt,
                        );
                    }
                    109 | 118 | 115 | 105 => {
                        _log_stderr(
                            b"nutcracker: option -%c requires a number\0" as *const u8
                                as *const libc::c_char,
                            optopt,
                        );
                    }
                    97 => {
                        _log_stderr(
                            b"nutcracker: option -%c requires a string\0" as *const u8
                                as *const libc::c_char,
                            optopt,
                        );
                    }
                    _ => {
                        _log_stderr(
                            b"nutcracker: invalid option -- '%c'\0" as *const u8
                                as *const libc::c_char,
                            optopt,
                        );
                    }
                }
                return -(1 as libc::c_int);
            }
            _ => {
                _log_stderr(
                    b"nutcracker: invalid option -- '%c'\0" as *const u8
                        as *const libc::c_char,
                    optopt,
                );
                return -(1 as libc::c_int);
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn nc_test_conf(mut nci: *const instance) -> bool {
    let mut cf: *mut conf = 0 as *mut conf;
    cf = conf_create((*nci).conf_filename);
    if cf.is_null() {
        _log_stderr(
            b"nutcracker: configuration file '%s' syntax is invalid\0" as *const u8
                as *const libc::c_char,
            (*nci).conf_filename,
        );
        return 0 as libc::c_int != 0;
    }
    conf_destroy(cf);
    _log_stderr(
        b"nutcracker: configuration file '%s' syntax is ok\0" as *const u8
            as *const libc::c_char,
        (*nci).conf_filename,
    );
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn nc_pre_run(mut nci: *mut instance) -> rstatus_t {
    let mut status: rstatus_t = 0;
    status = log_init((*nci).log_level, (*nci).log_filename);
    if status != 0 as libc::c_int {
        return status;
    }
    if daemonize != 0 {
        status = nc_daemonize(1 as libc::c_int);
        if status != 0 as libc::c_int {
            return status;
        }
    }
    (*nci).pid = getpid();
    status = signal_init();
    if status != 0 as libc::c_int {
        return status;
    }
    if !((*nci).pid_filename).is_null() {
        status = nc_create_pidfile(nci);
        if status != 0 as libc::c_int {
            return status;
        }
    }
    nc_print_run(nci);
    return 0 as libc::c_int;
}
unsafe extern "C" fn nc_post_run(mut nci: *mut instance) {
    if (*nci).pidfile() != 0 {
        nc_remove_pidfile(nci);
    }
    signal_deinit();
    nc_print_done();
    log_deinit();
}
unsafe extern "C" fn nc_run(mut nci: *mut instance) {
    let mut status: rstatus_t = 0;
    let mut ctx: *mut context = 0 as *mut context;
    ctx = core_start(nci);
    if ctx.is_null() {
        return;
    }
    loop {
        status = core_loop(ctx);
        if status != 0 as libc::c_int {
            break;
        }
    }
    core_stop(ctx);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut status: rstatus_t = 0;
    let mut nci: instance = instance {
        ctx: 0 as *mut context,
        log_level: 0,
        log_filename: 0 as *const libc::c_char,
        conf_filename: 0 as *const libc::c_char,
        stats_port: 0,
        stats_interval: 0,
        stats_addr: 0 as *const libc::c_char,
        hostname: [0; 256],
        mbuf_chunk_size: 0,
        pid: 0,
        pid_filename: 0 as *const libc::c_char,
        pidfile: [0; 1],
        c2rust_padding: [0; 7],
    };
    nc_set_default_options(&mut nci);
    status = nc_get_options(argc, argv, &mut nci);
    if status != 0 as libc::c_int {
        nc_show_usage();
        exit(1 as libc::c_int);
    }
    if show_version != 0 {
        _log_stderr(
            b"This is nutcracker-%s\0" as *const u8 as *const libc::c_char,
            b"0.5.0\0" as *const u8 as *const libc::c_char,
        );
        _log_stderr(b"async event backend: epoll\0" as *const u8 as *const libc::c_char);
        _log_stderr(b"\0" as *const u8 as *const libc::c_char);
        if show_help != 0 {
            nc_show_usage();
        }
        if describe_stats != 0 {
            stats_describe();
        }
        exit(0 as libc::c_int);
    }
    if test_conf != 0 {
        if !nc_test_conf(&mut nci) {
            exit(1 as libc::c_int);
        }
        exit(0 as libc::c_int);
    }
    status = nc_pre_run(&mut nci);
    if status != 0 as libc::c_int {
        nc_post_run(&mut nci);
        exit(1 as libc::c_int);
    }
    nc_run(&mut nci);
    nc_post_run(&mut nci);
    exit(1 as libc::c_int);
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
