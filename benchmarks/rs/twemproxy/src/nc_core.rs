use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type epoll_event;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn server_pool_deinit(server_pool: *mut array);
    fn server_pool_init(
        server_pool: *mut array,
        conf_pool: *mut array,
        ctx: *mut context,
    ) -> rstatus_t;
    fn server_pool_disconnect(ctx: *mut context);
    fn server_pool_preconnect(ctx: *mut context) -> rstatus_t;
    fn __errno_location() -> *mut libc::c_int;
    fn conn_deinit();
    fn conn_init();
    fn conn_to_ctx(conn: *const conn) -> *mut context;
    fn getrlimit(__resource: __rlimit_resource_t, __rlimits: *mut rlimit) -> libc::c_int;
    fn nc_get_soerror(sd: libc::c_int) -> libc::c_int;
    fn _nc_alloc(
        size: size_t,
        name: *const libc::c_char,
        line: libc::c_int,
    ) -> *mut libc::c_void;
    fn _nc_free(ptr: *mut libc::c_void, name: *const libc::c_char, line: libc::c_int);
    fn nc_msec_now() -> int64_t;
    fn nc_unresolve_addr(addr: *mut sockaddr, addrlen: socklen_t) -> *const libc::c_char;
    fn nc_unresolve_peer_desc(sd: libc::c_int) -> *const libc::c_char;
    fn log_loggable(level: libc::c_int) -> libc::c_int;
    fn _log(
        file: *const libc::c_char,
        line: libc::c_int,
        panic: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn event_base_create(size: libc::c_int, cb: event_cb_t) -> *mut event_base;
    fn event_base_destroy(evb: *mut event_base);
    fn event_del_conn(evb: *mut event_base, c: *mut conn) -> libc::c_int;
    fn event_wait(evb: *mut event_base, timeout: libc::c_int) -> libc::c_int;
    fn stats_create(
        stats_port: uint16_t,
        stats_ip: *const libc::c_char,
        stats_interval: libc::c_int,
        source: *const libc::c_char,
        server_pool: *const array,
    ) -> *mut stats;
    fn stats_destroy(stats: *mut stats);
    fn stats_swap(stats: *mut stats);
    fn mbuf_init(nci: *const instance);
    fn mbuf_deinit();
    fn msg_tmo_min() -> *mut msg;
    fn msg_tmo_delete(msg: *mut msg);
    fn msg_init();
    fn msg_deinit();
    fn conf_destroy(cf: *mut conf);
    fn conf_create(filename: *const libc::c_char) -> *mut conf;
    fn proxy_init(ctx: *mut context) -> rstatus_t;
    fn proxy_deinit(ctx: *mut context);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __rlim_t = libc::c_ulong;
pub type __socklen_t = libc::c_uint;
pub type mode_t = __mode_t;
pub type pid_t = __pid_t;
pub type int64_t = __int64_t;
pub type pthread_t = libc::c_ulong;
pub type socklen_t = __socklen_t;
pub type rstatus_t = libc::c_int;
pub type err_t = libc::c_int;
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
pub type _IO_lock_t = ();
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
pub struct conn {
    pub conn_tqe: C2RustUnnamed_41,
    pub owner: *mut libc::c_void,
    pub sd: libc::c_int,
    pub family: libc::c_int,
    pub addrlen: socklen_t,
    pub addr: *mut sockaddr,
    pub imsg_q: msg_tqh,
    pub omsg_q: msg_tqh,
    pub rmsg: *mut msg,
    pub smsg: *mut msg,
    pub recv: conn_recv_t,
    pub recv_next: conn_recv_next_t,
    pub recv_done: conn_recv_done_t,
    pub send: conn_send_t,
    pub send_next: conn_send_next_t,
    pub send_done: conn_send_done_t,
    pub close: conn_close_t,
    pub active: conn_active_t,
    pub post_connect: conn_post_connect_t,
    pub swallow_msg: conn_swallow_msg_t,
    pub ref_0: conn_ref_t,
    pub unref: conn_unref_t,
    pub enqueue_inq: conn_msgq_t,
    pub dequeue_inq: conn_msgq_t,
    pub enqueue_outq: conn_msgq_t,
    pub dequeue_outq: conn_msgq_t,
    pub recv_bytes: size_t,
    pub send_bytes: size_t,
    pub events: uint32_t,
    pub err: err_t,
    #[bitfield(name = "recv_active", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "recv_ready", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "send_active", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "send_ready", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "client", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "proxy", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "connecting", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "connected", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "eof", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "done", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "redis", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "authenticated", ty = "libc::c_uint", bits = "11..=11")]
    pub recv_active_recv_ready_send_active_send_ready_client_proxy_connecting_connected_eof_done_redis_authenticated: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
pub type conn_msgq_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut msg) -> (),
>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct msg {
    pub c_tqe: C2RustUnnamed_38,
    pub s_tqe: C2RustUnnamed_37,
    pub m_tqe: C2RustUnnamed_36,
    pub id: uint64_t,
    pub peer: *mut msg,
    pub owner: *mut conn,
    pub tmo_rbe: rbnode,
    pub mhdr: mhdr,
    pub mlen: uint32_t,
    pub start_ts: int64_t,
    pub state: libc::c_int,
    pub pos: *mut uint8_t,
    pub token: *mut uint8_t,
    pub parser: msg_parse_t,
    pub result: msg_parse_result_t,
    pub fragment: msg_fragment_t,
    pub reply: msg_reply_t,
    pub add_auth: msg_add_auth_t,
    pub failure: msg_failure_t,
    pub pre_coalesce: msg_coalesce_t,
    pub post_coalesce: msg_coalesce_t,
    pub type_0: msg_type_t,
    pub keys: *mut array,
    pub vlen: uint32_t,
    pub end: *mut uint8_t,
    pub narg_start: *mut uint8_t,
    pub narg_end: *mut uint8_t,
    pub narg: uint32_t,
    pub rnarg: uint32_t,
    pub rlen: uint32_t,
    pub integer: uint32_t,
    pub is_top_level: uint8_t,
    pub frag_owner: *mut msg,
    pub nfrag: uint32_t,
    pub nfrag_done: uint32_t,
    pub frag_id: uint64_t,
    pub frag_seq: *mut *mut msg,
    pub err: err_t,
    #[bitfield(name = "error", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "ferror", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "request", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "quit", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "noreply", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "noforward", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "done", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "fdone", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "swallow", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "redis", ty = "libc::c_uint", bits = "9..=9")]
    pub error_ferror_request_quit_noreply_noforward_done_fdone_swallow_redis: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
}
pub type uint64_t = __uint64_t;
pub type msg_type_t = msg_type;
pub type msg_type = libc::c_uint;
pub const MSG_SENTINEL: msg_type = 184;
pub const MSG_RSP_REDIS_MULTIBULK: msg_type = 183;
pub const MSG_RSP_REDIS_BULK: msg_type = 182;
pub const MSG_RSP_REDIS_INTEGER: msg_type = 181;
pub const MSG_RSP_REDIS_ERROR_NOREPLICAS: msg_type = 180;
pub const MSG_RSP_REDIS_ERROR_MASTERDOWN: msg_type = 179;
pub const MSG_RSP_REDIS_ERROR_EXECABORT: msg_type = 178;
pub const MSG_RSP_REDIS_ERROR_WRONGTYPE: msg_type = 177;
pub const MSG_RSP_REDIS_ERROR_READONLY: msg_type = 176;
pub const MSG_RSP_REDIS_ERROR_NOSCRIPT: msg_type = 175;
pub const MSG_RSP_REDIS_ERROR_MISCONF: msg_type = 174;
pub const MSG_RSP_REDIS_ERROR_BUSYKEY: msg_type = 173;
pub const MSG_RSP_REDIS_ERROR_LOADING: msg_type = 172;
pub const MSG_RSP_REDIS_ERROR_NOAUTH: msg_type = 171;
pub const MSG_RSP_REDIS_ERROR_BUSY: msg_type = 170;
pub const MSG_RSP_REDIS_ERROR_OOM: msg_type = 169;
pub const MSG_RSP_REDIS_ERROR_ERR: msg_type = 168;
pub const MSG_RSP_REDIS_ERROR: msg_type = 167;
pub const MSG_RSP_REDIS_STATUS: msg_type = 166;
pub const MSG_REQ_REDIS_LOLWUT: msg_type = 165;
pub const MSG_REQ_REDIS_COMMAND: msg_type = 164;
pub const MSG_REQ_REDIS_SELECT: msg_type = 163;
pub const MSG_REQ_REDIS_AUTH: msg_type = 162;
pub const MSG_REQ_REDIS_QUIT: msg_type = 161;
pub const MSG_REQ_REDIS_PING: msg_type = 160;
pub const MSG_REQ_REDIS_EVALSHA: msg_type = 159;
pub const MSG_REQ_REDIS_EVAL: msg_type = 158;
pub const MSG_REQ_REDIS_GEOSEARCHSTORE: msg_type = 157;
pub const MSG_REQ_REDIS_GEOSEARCH: msg_type = 156;
pub const MSG_REQ_REDIS_GEOPOS: msg_type = 155;
pub const MSG_REQ_REDIS_GEORADIUSBYMEMBER: msg_type = 154;
pub const MSG_REQ_REDIS_GEORADIUS: msg_type = 153;
pub const MSG_REQ_REDIS_GEOHASH: msg_type = 152;
pub const MSG_REQ_REDIS_GEODIST: msg_type = 151;
pub const MSG_REQ_REDIS_GEOADD: msg_type = 150;
pub const MSG_REQ_REDIS_ZUNIONSTORE: msg_type = 149;
pub const MSG_REQ_REDIS_ZSCORE: msg_type = 148;
pub const MSG_REQ_REDIS_ZSCAN: msg_type = 147;
pub const MSG_REQ_REDIS_ZUNION: msg_type = 146;
pub const MSG_REQ_REDIS_ZREVRANK: msg_type = 145;
pub const MSG_REQ_REDIS_ZREVRANGEBYSCORE: msg_type = 144;
pub const MSG_REQ_REDIS_ZREVRANGEBYLEX: msg_type = 143;
pub const MSG_REQ_REDIS_ZREVRANGE: msg_type = 142;
pub const MSG_REQ_REDIS_ZREMRANGEBYSCORE: msg_type = 141;
pub const MSG_REQ_REDIS_ZREMRANGEBYLEX: msg_type = 140;
pub const MSG_REQ_REDIS_ZREMRANGEBYRANK: msg_type = 139;
pub const MSG_REQ_REDIS_ZREM: msg_type = 138;
pub const MSG_REQ_REDIS_ZRANK: msg_type = 137;
pub const MSG_REQ_REDIS_ZRANGESTORE: msg_type = 136;
pub const MSG_REQ_REDIS_ZRANGEBYSCORE: msg_type = 135;
pub const MSG_REQ_REDIS_ZRANGEBYLEX: msg_type = 134;
pub const MSG_REQ_REDIS_ZRANGE: msg_type = 133;
pub const MSG_REQ_REDIS_ZRANDMEMBER: msg_type = 132;
pub const MSG_REQ_REDIS_ZPOPMAX: msg_type = 131;
pub const MSG_REQ_REDIS_ZPOPMIN: msg_type = 130;
pub const MSG_REQ_REDIS_ZMSCORE: msg_type = 129;
pub const MSG_REQ_REDIS_ZLEXCOUNT: msg_type = 128;
pub const MSG_REQ_REDIS_ZINTERSTORE: msg_type = 127;
pub const MSG_REQ_REDIS_ZINTER: msg_type = 126;
pub const MSG_REQ_REDIS_ZINCRBY: msg_type = 125;
pub const MSG_REQ_REDIS_ZDIFFSTORE: msg_type = 124;
pub const MSG_REQ_REDIS_ZDIFF: msg_type = 123;
pub const MSG_REQ_REDIS_ZCOUNT: msg_type = 122;
pub const MSG_REQ_REDIS_ZCARD: msg_type = 121;
pub const MSG_REQ_REDIS_ZADD: msg_type = 120;
pub const MSG_REQ_REDIS_SSCAN: msg_type = 119;
pub const MSG_REQ_REDIS_SUNIONSTORE: msg_type = 118;
pub const MSG_REQ_REDIS_SUNION: msg_type = 117;
pub const MSG_REQ_REDIS_SREM: msg_type = 116;
pub const MSG_REQ_REDIS_SRANDMEMBER: msg_type = 115;
pub const MSG_REQ_REDIS_SPOP: msg_type = 114;
pub const MSG_REQ_REDIS_SMOVE: msg_type = 113;
pub const MSG_REQ_REDIS_SMEMBERS: msg_type = 112;
pub const MSG_REQ_REDIS_SMISMEMBER: msg_type = 111;
pub const MSG_REQ_REDIS_SISMEMBER: msg_type = 110;
pub const MSG_REQ_REDIS_SINTERSTORE: msg_type = 109;
pub const MSG_REQ_REDIS_SINTER: msg_type = 108;
pub const MSG_REQ_REDIS_SDIFFSTORE: msg_type = 107;
pub const MSG_REQ_REDIS_SDIFF: msg_type = 106;
pub const MSG_REQ_REDIS_SCARD: msg_type = 105;
pub const MSG_REQ_REDIS_SADD: msg_type = 104;
pub const MSG_REQ_REDIS_RPUSHX: msg_type = 103;
pub const MSG_REQ_REDIS_RPUSH: msg_type = 102;
pub const MSG_REQ_REDIS_RPOPLPUSH: msg_type = 101;
pub const MSG_REQ_REDIS_RPOP: msg_type = 100;
pub const MSG_REQ_REDIS_PFMERGE: msg_type = 99;
pub const MSG_REQ_REDIS_PFCOUNT: msg_type = 98;
pub const MSG_REQ_REDIS_PFADD: msg_type = 97;
pub const MSG_REQ_REDIS_LTRIM: msg_type = 96;
pub const MSG_REQ_REDIS_LSET: msg_type = 95;
pub const MSG_REQ_REDIS_LREM: msg_type = 94;
pub const MSG_REQ_REDIS_LRANGE: msg_type = 93;
pub const MSG_REQ_REDIS_LPUSHX: msg_type = 92;
pub const MSG_REQ_REDIS_LPUSH: msg_type = 91;
pub const MSG_REQ_REDIS_LPOS: msg_type = 90;
pub const MSG_REQ_REDIS_LPOP: msg_type = 89;
pub const MSG_REQ_REDIS_LMOVE: msg_type = 88;
pub const MSG_REQ_REDIS_LLEN: msg_type = 87;
pub const MSG_REQ_REDIS_LINSERT: msg_type = 86;
pub const MSG_REQ_REDIS_LINDEX: msg_type = 85;
pub const MSG_REQ_REDIS_HVALS: msg_type = 84;
pub const MSG_REQ_REDIS_HSTRLEN: msg_type = 83;
pub const MSG_REQ_REDIS_HSCAN: msg_type = 82;
pub const MSG_REQ_REDIS_HSETNX: msg_type = 81;
pub const MSG_REQ_REDIS_HSET: msg_type = 80;
pub const MSG_REQ_REDIS_HRANDFIELD: msg_type = 79;
pub const MSG_REQ_REDIS_HMSET: msg_type = 78;
pub const MSG_REQ_REDIS_HMGET: msg_type = 77;
pub const MSG_REQ_REDIS_HLEN: msg_type = 76;
pub const MSG_REQ_REDIS_HKEYS: msg_type = 75;
pub const MSG_REQ_REDIS_HINCRBYFLOAT: msg_type = 74;
pub const MSG_REQ_REDIS_HINCRBY: msg_type = 73;
pub const MSG_REQ_REDIS_HGETALL: msg_type = 72;
pub const MSG_REQ_REDIS_HGET: msg_type = 71;
pub const MSG_REQ_REDIS_HEXISTS: msg_type = 70;
pub const MSG_REQ_REDIS_HDEL: msg_type = 69;
pub const MSG_REQ_REDIS_STRLEN: msg_type = 68;
pub const MSG_REQ_REDIS_SETRANGE: msg_type = 67;
pub const MSG_REQ_REDIS_SETNX: msg_type = 66;
pub const MSG_REQ_REDIS_SETEX: msg_type = 65;
pub const MSG_REQ_REDIS_SETBIT: msg_type = 64;
pub const MSG_REQ_REDIS_SET: msg_type = 63;
pub const MSG_REQ_REDIS_RESTORE: msg_type = 62;
pub const MSG_REQ_REDIS_PSETEX: msg_type = 61;
pub const MSG_REQ_REDIS_MSET: msg_type = 60;
pub const MSG_REQ_REDIS_MGET: msg_type = 59;
pub const MSG_REQ_REDIS_INCRBYFLOAT: msg_type = 58;
pub const MSG_REQ_REDIS_INCRBY: msg_type = 57;
pub const MSG_REQ_REDIS_INCR: msg_type = 56;
pub const MSG_REQ_REDIS_GETSET: msg_type = 55;
pub const MSG_REQ_REDIS_GETRANGE: msg_type = 54;
pub const MSG_REQ_REDIS_GETEX: msg_type = 53;
pub const MSG_REQ_REDIS_GETDEL: msg_type = 52;
pub const MSG_REQ_REDIS_GETBIT: msg_type = 51;
pub const MSG_REQ_REDIS_GET: msg_type = 50;
pub const MSG_REQ_REDIS_DUMP: msg_type = 49;
pub const MSG_REQ_REDIS_DECRBY: msg_type = 48;
pub const MSG_REQ_REDIS_DECR: msg_type = 47;
pub const MSG_REQ_REDIS_BITPOS: msg_type = 46;
pub const MSG_REQ_REDIS_BITFIELD: msg_type = 45;
pub const MSG_REQ_REDIS_BITCOUNT: msg_type = 44;
pub const MSG_REQ_REDIS_APPEND: msg_type = 43;
pub const MSG_REQ_REDIS_UNLINK: msg_type = 42;
pub const MSG_REQ_REDIS_TYPE: msg_type = 41;
pub const MSG_REQ_REDIS_TTL: msg_type = 40;
pub const MSG_REQ_REDIS_TOUCH: msg_type = 39;
pub const MSG_REQ_REDIS_SORT: msg_type = 38;
pub const MSG_REQ_REDIS_PTTL: msg_type = 37;
pub const MSG_REQ_REDIS_PERSIST: msg_type = 36;
pub const MSG_REQ_REDIS_PEXPIREAT: msg_type = 35;
pub const MSG_REQ_REDIS_PEXPIRE: msg_type = 34;
pub const MSG_REQ_REDIS_MOVE: msg_type = 33;
pub const MSG_REQ_REDIS_EXPIREAT: msg_type = 32;
pub const MSG_REQ_REDIS_EXPIRE: msg_type = 31;
pub const MSG_REQ_REDIS_EXISTS: msg_type = 30;
pub const MSG_REQ_REDIS_DEL: msg_type = 29;
pub const MSG_REQ_REDIS_COPY: msg_type = 28;
pub const MSG_RSP_MC_SERVER_ERROR: msg_type = 27;
pub const MSG_RSP_MC_CLIENT_ERROR: msg_type = 26;
pub const MSG_RSP_MC_ERROR: msg_type = 25;
pub const MSG_RSP_MC_VERSION: msg_type = 24;
pub const MSG_RSP_MC_TOUCHED: msg_type = 23;
pub const MSG_RSP_MC_DELETED: msg_type = 22;
pub const MSG_RSP_MC_VALUE: msg_type = 21;
pub const MSG_RSP_MC_END: msg_type = 20;
pub const MSG_RSP_MC_NOT_FOUND: msg_type = 19;
pub const MSG_RSP_MC_EXISTS: msg_type = 18;
pub const MSG_RSP_MC_NOT_STORED: msg_type = 17;
pub const MSG_RSP_MC_STORED: msg_type = 16;
pub const MSG_RSP_MC_NUM: msg_type = 15;
pub const MSG_REQ_MC_VERSION: msg_type = 14;
pub const MSG_REQ_MC_QUIT: msg_type = 13;
pub const MSG_REQ_MC_TOUCH: msg_type = 12;
pub const MSG_REQ_MC_DECR: msg_type = 11;
pub const MSG_REQ_MC_INCR: msg_type = 10;
pub const MSG_REQ_MC_PREPEND: msg_type = 9;
pub const MSG_REQ_MC_APPEND: msg_type = 8;
pub const MSG_REQ_MC_REPLACE: msg_type = 7;
pub const MSG_REQ_MC_ADD: msg_type = 6;
pub const MSG_REQ_MC_SET: msg_type = 5;
pub const MSG_REQ_MC_CAS: msg_type = 4;
pub const MSG_REQ_MC_DELETE: msg_type = 3;
pub const MSG_REQ_MC_GETS: msg_type = 2;
pub const MSG_REQ_MC_GET: msg_type = 1;
pub const MSG_UNKNOWN: msg_type = 0;
pub type msg_coalesce_t = Option::<unsafe extern "C" fn(*mut msg) -> ()>;
pub type msg_failure_t = Option::<unsafe extern "C" fn(*const msg) -> bool>;
pub type msg_add_auth_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut conn) -> rstatus_t,
>;
pub type msg_reply_t = Option::<unsafe extern "C" fn(*mut msg) -> rstatus_t>;
pub type msg_fragment_t = Option::<
    unsafe extern "C" fn(*mut msg, uint32_t, *mut msg_tqh) -> rstatus_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msg_tqh {
    pub tqh_first: *mut msg,
    pub tqh_last: *mut *mut msg,
}
pub type msg_parse_result_t = msg_parse_result;
pub type msg_parse_result = libc::c_uint;
pub const MSG_PARSE_AGAIN: msg_parse_result = 3;
pub const MSG_PARSE_REPAIR: msg_parse_result = 2;
pub const MSG_PARSE_ERROR: msg_parse_result = 1;
pub const MSG_PARSE_OK: msg_parse_result = 0;
pub type msg_parse_t = Option::<unsafe extern "C" fn(*mut msg) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mhdr {
    pub stqh_first: *mut mbuf,
    pub stqh_last: *mut *mut mbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbuf {
    pub magic: uint32_t,
    pub next: C2RustUnnamed_35,
    pub pos: *mut uint8_t,
    pub last: *mut uint8_t,
    pub start: *mut uint8_t,
    pub end: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub stqe_next: *mut mbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rbnode {
    pub left: *mut rbnode,
    pub right: *mut rbnode,
    pub parent: *mut rbnode,
    pub key: int64_t,
    pub data: *mut libc::c_void,
    pub color: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_36 {
    pub tqe_next: *mut msg,
    pub tqe_prev: *mut *mut msg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_37 {
    pub tqe_next: *mut msg,
    pub tqe_prev: *mut *mut msg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_38 {
    pub tqe_next: *mut msg,
    pub tqe_prev: *mut *mut msg,
}
pub type conn_unref_t = Option::<unsafe extern "C" fn(*mut conn) -> ()>;
pub type conn_ref_t = Option::<unsafe extern "C" fn(*mut conn, *mut libc::c_void) -> ()>;
pub type conn_swallow_msg_t = Option::<
    unsafe extern "C" fn(*mut conn, *mut msg, *mut msg) -> (),
>;
pub type conn_post_connect_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut server) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server {
    pub idx: uint32_t,
    pub owner: *mut server_pool,
    pub pname: string,
    pub name: string,
    pub addrstr: string,
    pub port: uint16_t,
    pub weight: uint32_t,
    pub info: sockinfo,
    pub ns_conn_q: uint32_t,
    pub s_conn_q: conn_tqh,
    pub next_retry: int64_t,
    pub failure_count: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conn_tqh {
    pub tqh_first: *mut conn,
    pub tqh_last: *mut *mut conn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockinfo {
    pub family: libc::c_int,
    pub addrlen: socklen_t,
    pub addr: C2RustUnnamed_39,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_39 {
    pub in_0: sockaddr_in,
    pub in6: sockaddr_in6,
    pub un: sockaddr_un,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_40,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_40 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct server_pool {
    pub idx: uint32_t,
    pub ctx: *mut context,
    pub p_conn: *mut conn,
    pub nc_conn_q: uint32_t,
    pub c_conn_q: conn_tqh,
    pub server: array,
    pub ncontinuum: uint32_t,
    pub nserver_continuum: uint32_t,
    pub continuum: *mut continuum,
    pub nlive_server: uint32_t,
    pub next_rebuild: int64_t,
    pub name: string,
    pub addrstr: string,
    pub port: uint16_t,
    pub info: sockinfo,
    pub perm: mode_t,
    pub dist_type: libc::c_int,
    pub key_hash_type: libc::c_int,
    pub key_hash: hash_t,
    pub hash_tag: string,
    pub timeout: libc::c_int,
    pub backlog: libc::c_int,
    pub redis_db: libc::c_int,
    pub client_connections: uint32_t,
    pub server_connections: uint32_t,
    pub server_retry_timeout: int64_t,
    pub server_failure_limit: uint32_t,
    pub redis_auth: string,
    pub require_auth: libc::c_uint,
    #[bitfield(name = "auto_eject_hosts", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "preconnect", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "redis", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "tcpkeepalive", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "reuseport", ty = "libc::c_uint", bits = "4..=4")]
    pub auto_eject_hosts_preconnect_redis_tcpkeepalive_reuseport: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type hash_t = Option::<
    unsafe extern "C" fn(*const libc::c_char, size_t) -> uint32_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct continuum {
    pub index: uint32_t,
    pub value: uint32_t,
}
pub type conn_active_t = Option::<unsafe extern "C" fn(*const conn) -> bool>;
pub type conn_close_t = Option::<unsafe extern "C" fn(*mut context, *mut conn) -> ()>;
pub type conn_send_done_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut msg) -> (),
>;
pub type conn_send_next_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn) -> *mut msg,
>;
pub type conn_send_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn) -> rstatus_t,
>;
pub type conn_recv_done_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut msg, *mut msg) -> (),
>;
pub type conn_recv_next_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, bool) -> *mut msg,
>;
pub type conn_recv_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn) -> rstatus_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_41 {
    pub tqe_next: *mut conn,
    pub tqe_prev: *mut *mut conn,
}
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
pub type __rlimit_resource = libc::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type rlim_t = __rlim_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = __rlimit_resource;
#[inline]
unsafe extern "C" fn array_null(mut a: *mut array) {
    (*a).nelem = 0 as libc::c_int as uint32_t;
    (*a).elem = 0 as *mut libc::c_void;
    (*a).size = 0 as libc::c_int as size_t;
    (*a).nalloc = 0 as libc::c_int as uint32_t;
}
static mut ctx_id: uint32_t = 0;
unsafe extern "C" fn core_calc_connections(mut ctx: *mut context) -> rstatus_t {
    let mut status: libc::c_int = 0;
    let mut limit: rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };
    status = getrlimit(RLIMIT_NOFILE, &mut limit);
    if status < 0 as libc::c_int {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_core.c\0" as *const u8 as *const libc::c_char,
                35 as libc::c_int,
                0 as libc::c_int,
                b"getrlimit failed: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return -(1 as libc::c_int);
    }
    (*ctx).max_nfd = limit.rlim_cur as uint32_t;
    (*ctx)
        .max_ncconn = ((*ctx).max_nfd)
        .wrapping_sub((*ctx).max_nsconn)
        .wrapping_sub(32 as libc::c_int as libc::c_uint);
    return 0 as libc::c_int;
}
unsafe extern "C" fn core_ctx_create(mut nci: *mut instance) -> *mut context {
    let mut status: rstatus_t = 0;
    let mut ctx: *mut context = 0 as *mut context;
    ctx = _nc_alloc(
        ::std::mem::size_of::<context>() as libc::c_ulong,
        b"nc_core.c\0" as *const u8 as *const libc::c_char,
        54 as libc::c_int,
    ) as *mut context;
    if ctx.is_null() {
        return 0 as *mut context;
    }
    ctx_id = ctx_id.wrapping_add(1);
    (*ctx).id = ctx_id;
    (*ctx).cf = 0 as *mut conf;
    (*ctx).stats = 0 as *mut stats;
    (*ctx).evb = 0 as *mut event_base;
    array_null(&mut (*ctx).pool);
    (*ctx).max_timeout = (*nci).stats_interval;
    (*ctx).timeout = (*ctx).max_timeout;
    (*ctx).max_nfd = 0 as libc::c_int as uint32_t;
    (*ctx).max_ncconn = 0 as libc::c_int as uint32_t;
    (*ctx).max_nsconn = 0 as libc::c_int as uint32_t;
    (*ctx).cf = conf_create((*nci).conf_filename);
    if ((*ctx).cf).is_null() {
        _nc_free(
            ctx as *mut libc::c_void,
            b"nc_core.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int,
        );
        ctx = 0 as *mut context;
        return 0 as *mut context;
    }
    status = server_pool_init(&mut (*ctx).pool, &mut (*(*ctx).cf).pool, ctx);
    if status != 0 as libc::c_int {
        conf_destroy((*ctx).cf);
        _nc_free(
            ctx as *mut libc::c_void,
            b"nc_core.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int,
        );
        ctx = 0 as *mut context;
        return 0 as *mut context;
    }
    status = core_calc_connections(ctx);
    if status != 0 as libc::c_int {
        server_pool_deinit(&mut (*ctx).pool);
        conf_destroy((*ctx).cf);
        _nc_free(
            ctx as *mut libc::c_void,
            b"nc_core.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
        );
        ctx = 0 as *mut context;
        return 0 as *mut context;
    }
    (*ctx)
        .stats = stats_create(
        (*nci).stats_port,
        (*nci).stats_addr,
        (*nci).stats_interval,
        ((*nci).hostname).as_mut_ptr(),
        &mut (*ctx).pool,
    );
    if ((*ctx).stats).is_null() {
        server_pool_deinit(&mut (*ctx).pool);
        conf_destroy((*ctx).cf);
        _nc_free(
            ctx as *mut libc::c_void,
            b"nc_core.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int,
        );
        ctx = 0 as *mut context;
        return 0 as *mut context;
    }
    (*ctx)
        .evb = event_base_create(
        1024 as libc::c_int,
        Some(core_core as unsafe extern "C" fn(*mut libc::c_void, uint32_t) -> rstatus_t),
    );
    if ((*ctx).evb).is_null() {
        stats_destroy((*ctx).stats);
        server_pool_deinit(&mut (*ctx).pool);
        conf_destroy((*ctx).cf);
        _nc_free(
            ctx as *mut libc::c_void,
            b"nc_core.c\0" as *const u8 as *const libc::c_char,
            112 as libc::c_int,
        );
        ctx = 0 as *mut context;
        return 0 as *mut context;
    }
    status = server_pool_preconnect(ctx);
    if status != 0 as libc::c_int {
        server_pool_disconnect(ctx);
        event_base_destroy((*ctx).evb);
        stats_destroy((*ctx).stats);
        server_pool_deinit(&mut (*ctx).pool);
        conf_destroy((*ctx).cf);
        _nc_free(
            ctx as *mut libc::c_void,
            b"nc_core.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
        );
        ctx = 0 as *mut context;
        return 0 as *mut context;
    }
    status = proxy_init(ctx);
    if status != 0 as libc::c_int {
        server_pool_disconnect(ctx);
        event_base_destroy((*ctx).evb);
        stats_destroy((*ctx).stats);
        server_pool_deinit(&mut (*ctx).pool);
        conf_destroy((*ctx).cf);
        _nc_free(
            ctx as *mut libc::c_void,
            b"nc_core.c\0" as *const u8 as *const libc::c_char,
            136 as libc::c_int,
        );
        ctx = 0 as *mut context;
        return 0 as *mut context;
    }
    return ctx;
}
unsafe extern "C" fn core_ctx_destroy(mut ctx: *mut context) {
    proxy_deinit(ctx);
    server_pool_disconnect(ctx);
    event_base_destroy((*ctx).evb);
    stats_destroy((*ctx).stats);
    server_pool_deinit(&mut (*ctx).pool);
    conf_destroy((*ctx).cf);
    _nc_free(
        ctx as *mut libc::c_void,
        b"nc_core.c\0" as *const u8 as *const libc::c_char,
        155 as libc::c_int,
    );
    ctx = 0 as *mut context;
}
pub unsafe extern "C" fn core_start(mut nci: *mut instance) -> *mut context {
    let mut ctx: *mut context = 0 as *mut context;
    mbuf_init(nci);
    msg_init();
    conn_init();
    ctx = core_ctx_create(nci);
    if !ctx.is_null() {
        (*nci).ctx = ctx;
        return ctx;
    }
    conn_deinit();
    msg_deinit();
    mbuf_deinit();
    return 0 as *mut context;
}
pub unsafe extern "C" fn core_stop(mut ctx: *mut context) {
    conn_deinit();
    msg_deinit();
    mbuf_deinit();
    core_ctx_destroy(ctx);
}
unsafe extern "C" fn core_recv(mut ctx: *mut context, mut conn: *mut conn) -> rstatus_t {
    let mut status: rstatus_t = 0;
    status = ((*conn).recv).unwrap()(ctx, conn);
    status != 0 as libc::c_int;
    return status;
}
unsafe extern "C" fn core_send(mut ctx: *mut context, mut conn: *mut conn) -> rstatus_t {
    let mut status: rstatus_t = 0;
    status = ((*conn).send).unwrap()(ctx, conn);
    status != 0 as libc::c_int;
    return status;
}
unsafe extern "C" fn core_close(mut ctx: *mut context, mut conn: *mut conn) {
    let mut status: rstatus_t = 0;
    let mut type_0: libc::c_char = 0;
    let mut addrstr: *const libc::c_char = 0 as *const libc::c_char;
    if (*conn).client() != 0 {
        type_0 = 'c' as i32 as libc::c_char;
        addrstr = nc_unresolve_peer_desc((*conn).sd);
    } else {
        type_0 = (if (*conn).proxy() as libc::c_int != 0 {
            'p' as i32
        } else {
            's' as i32
        }) as libc::c_char;
        addrstr = nc_unresolve_addr((*conn).addr, (*conn).addrlen);
    }
    status = event_del_conn((*ctx).evb, conn);
    if status < 0 as libc::c_int {
        if log_loggable(4 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_core.c\0" as *const u8 as *const libc::c_char,
                243 as libc::c_int,
                0 as libc::c_int,
                b"event del conn %c %d failed, ignored: %s\0" as *const u8
                    as *const libc::c_char,
                type_0 as libc::c_int,
                (*conn).sd,
                strerror(*__errno_location()),
            );
        }
    }
    ((*conn).close).unwrap()(ctx, conn);
}
unsafe extern "C" fn core_error(mut ctx: *mut context, mut conn: *mut conn) {
    let mut status: rstatus_t = 0;
    let mut type_0: libc::c_char = (if (*conn).client() as libc::c_int != 0 {
        'c' as i32
    } else if (*conn).proxy() as libc::c_int != 0 {
        'p' as i32
    } else {
        's' as i32
    }) as libc::c_char;
    status = nc_get_soerror((*conn).sd);
    if status < 0 as libc::c_int {
        if log_loggable(4 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_core.c\0" as *const u8 as *const libc::c_char,
                258 as libc::c_int,
                0 as libc::c_int,
                b"get soerr on %c %d failed, ignored: %s\0" as *const u8
                    as *const libc::c_char,
                type_0 as libc::c_int,
                (*conn).sd,
                strerror(*__errno_location()),
            );
        }
    }
    (*conn).err = *__errno_location();
    core_close(ctx, conn);
}
unsafe extern "C" fn core_timeout(mut ctx: *mut context) {
    loop {
        let mut msg: *mut msg = 0 as *mut msg;
        let mut conn: *mut conn = 0 as *mut conn;
        let mut now: int64_t = 0;
        let mut then: int64_t = 0;
        msg = msg_tmo_min();
        if msg.is_null() {
            (*ctx).timeout = (*ctx).max_timeout;
            return;
        }
        if (*msg).error() as libc::c_int != 0 || (*msg).done() as libc::c_int != 0 {
            msg_tmo_delete(msg);
        } else {
            conn = (*msg).tmo_rbe.data as *mut conn;
            then = (*msg).tmo_rbe.key;
            now = nc_msec_now();
            if now < then {
                let mut delta: libc::c_int = (then - now) as libc::c_int;
                (*ctx)
                    .timeout = if delta < (*ctx).max_timeout {
                    delta
                } else {
                    (*ctx).max_timeout
                };
                return;
            }
            msg_tmo_delete(msg);
            (*conn).err = 110 as libc::c_int;
            core_close(ctx, conn);
        }
    };
}
pub unsafe extern "C" fn core_core(
    mut arg: *mut libc::c_void,
    mut events: uint32_t,
) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut conn: *mut conn = arg as *mut conn;
    let mut ctx: *mut context = 0 as *mut context;
    if ((*conn).owner).is_null() {
        if log_loggable(4 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_core.c\0" as *const u8 as *const libc::c_char,
                318 as libc::c_int,
                0 as libc::c_int,
                b"conn is already unrefed!\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    ctx = conn_to_ctx(conn);
    (*conn).events = events;
    if events & 0xff0000 as libc::c_int as libc::c_uint != 0 {
        core_error(ctx, conn);
        return -(1 as libc::c_int);
    }
    if events & 0xff as libc::c_int as libc::c_uint != 0 {
        status = core_recv(ctx, conn);
        if status != 0 as libc::c_int || (*conn).done() as libc::c_int != 0
            || (*conn).err != 0
        {
            core_close(ctx, conn);
            return -(1 as libc::c_int);
        }
    }
    if events & 0xff00 as libc::c_int as libc::c_uint != 0 {
        status = core_send(ctx, conn);
        if status != 0 as libc::c_int || (*conn).done() as libc::c_int != 0
            || (*conn).err != 0
        {
            core_close(ctx, conn);
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn core_loop(mut ctx: *mut context) -> rstatus_t {
    let mut nsd: libc::c_int = 0;
    nsd = event_wait((*ctx).evb, (*ctx).timeout);
    if nsd < 0 as libc::c_int {
        return nsd;
    }
    core_timeout(ctx);
    stats_swap((*ctx).stats);
    return 0 as libc::c_int;
}
