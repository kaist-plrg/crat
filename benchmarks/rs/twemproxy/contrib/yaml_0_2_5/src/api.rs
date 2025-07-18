use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type size_t = libc::c_ulong;
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
pub type yaml_char_t = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_version_directive_s {
    pub major: libc::c_int,
    pub minor: libc::c_int,
}
pub type yaml_version_directive_t = yaml_version_directive_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_tag_directive_s {
    pub handle: *mut yaml_char_t,
    pub prefix: *mut yaml_char_t,
}
pub type yaml_tag_directive_t = yaml_tag_directive_s;
pub type yaml_encoding_e = libc::c_uint;
pub const YAML_UTF16BE_ENCODING: yaml_encoding_e = 3;
pub const YAML_UTF16LE_ENCODING: yaml_encoding_e = 2;
pub const YAML_UTF8_ENCODING: yaml_encoding_e = 1;
pub const YAML_ANY_ENCODING: yaml_encoding_e = 0;
pub type yaml_encoding_t = yaml_encoding_e;
pub type yaml_break_e = libc::c_uint;
pub const YAML_CRLN_BREAK: yaml_break_e = 3;
pub const YAML_LN_BREAK: yaml_break_e = 2;
pub const YAML_CR_BREAK: yaml_break_e = 1;
pub const YAML_ANY_BREAK: yaml_break_e = 0;
pub type yaml_break_t = yaml_break_e;
pub type yaml_error_type_e = libc::c_uint;
pub const YAML_EMITTER_ERROR: yaml_error_type_e = 7;
pub const YAML_WRITER_ERROR: yaml_error_type_e = 6;
pub const YAML_COMPOSER_ERROR: yaml_error_type_e = 5;
pub const YAML_PARSER_ERROR: yaml_error_type_e = 4;
pub const YAML_SCANNER_ERROR: yaml_error_type_e = 3;
pub const YAML_READER_ERROR: yaml_error_type_e = 2;
pub const YAML_MEMORY_ERROR: yaml_error_type_e = 1;
pub const YAML_NO_ERROR: yaml_error_type_e = 0;
pub type yaml_error_type_t = yaml_error_type_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_mark_s {
    pub index: size_t,
    pub line: size_t,
    pub column: size_t,
}
pub type yaml_mark_t = yaml_mark_s;
pub type yaml_scalar_style_e = libc::c_uint;
pub const YAML_FOLDED_SCALAR_STYLE: yaml_scalar_style_e = 5;
pub const YAML_LITERAL_SCALAR_STYLE: yaml_scalar_style_e = 4;
pub const YAML_DOUBLE_QUOTED_SCALAR_STYLE: yaml_scalar_style_e = 3;
pub const YAML_SINGLE_QUOTED_SCALAR_STYLE: yaml_scalar_style_e = 2;
pub const YAML_PLAIN_SCALAR_STYLE: yaml_scalar_style_e = 1;
pub const YAML_ANY_SCALAR_STYLE: yaml_scalar_style_e = 0;
pub type yaml_scalar_style_t = yaml_scalar_style_e;
pub type yaml_sequence_style_e = libc::c_uint;
pub const YAML_FLOW_SEQUENCE_STYLE: yaml_sequence_style_e = 2;
pub const YAML_BLOCK_SEQUENCE_STYLE: yaml_sequence_style_e = 1;
pub const YAML_ANY_SEQUENCE_STYLE: yaml_sequence_style_e = 0;
pub type yaml_sequence_style_t = yaml_sequence_style_e;
pub type yaml_mapping_style_e = libc::c_uint;
pub const YAML_FLOW_MAPPING_STYLE: yaml_mapping_style_e = 2;
pub const YAML_BLOCK_MAPPING_STYLE: yaml_mapping_style_e = 1;
pub const YAML_ANY_MAPPING_STYLE: yaml_mapping_style_e = 0;
pub type yaml_mapping_style_t = yaml_mapping_style_e;
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
pub type yaml_token_type_t = yaml_token_type_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_token_s {
    pub type_0: yaml_token_type_t,
    pub data: C2RustUnnamed,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
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
pub type yaml_token_t = yaml_token_s;
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
pub type yaml_event_type_t = yaml_event_type_e;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: libc::c_int,
    pub style: yaml_sequence_style_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub encoding: yaml_encoding_t,
}
pub type yaml_event_t = yaml_event_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub error: yaml_error_type_t,
}
pub type yaml_node_type_e = libc::c_uint;
pub const YAML_MAPPING_NODE: yaml_node_type_e = 3;
pub const YAML_SEQUENCE_NODE: yaml_node_type_e = 2;
pub const YAML_SCALAR_NODE: yaml_node_type_e = 1;
pub const YAML_NO_NODE: yaml_node_type_e = 0;
pub type yaml_node_type_t = yaml_node_type_e;
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
pub type yaml_node_t = yaml_node_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_document_s {
    pub nodes: C2RustUnnamed_25,
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: C2RustUnnamed_24,
    pub start_implicit: libc::c_int,
    pub end_implicit: libc::c_int,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub start: *mut yaml_node_t,
    pub end: *mut yaml_node_t,
    pub top: *mut yaml_node_t,
}
pub type yaml_document_t = yaml_document_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub start: *mut yaml_node_t,
    pub end: *mut yaml_node_t,
    pub top: *mut yaml_node_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub error: yaml_error_type_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub error: yaml_error_type_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub error: yaml_error_type_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
    pub start: *mut yaml_node_pair_t,
    pub end: *mut yaml_node_pair_t,
    pub top: *mut yaml_node_pair_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_33 {
    pub error: yaml_error_type_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub error: yaml_error_type_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub error: yaml_error_type_t,
}
pub type yaml_read_handler_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_uchar,
    size_t,
    *mut size_t,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_simple_key_s {
    pub possible: libc::c_int,
    pub required: libc::c_int,
    pub token_number: size_t,
    pub mark: yaml_mark_t,
}
pub type yaml_simple_key_t = yaml_simple_key_s;
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
pub type yaml_parser_state_t = yaml_parser_state_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_alias_data_s {
    pub anchor: *mut yaml_char_t,
    pub index: libc::c_int,
    pub mark: yaml_mark_t,
}
pub type yaml_alias_data_t = yaml_alias_data_s;
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
    pub input: C2RustUnnamed_45,
    pub eof: libc::c_int,
    pub buffer: C2RustUnnamed_44,
    pub unread: size_t,
    pub raw_buffer: C2RustUnnamed_43,
    pub encoding: yaml_encoding_t,
    pub offset: size_t,
    pub mark: yaml_mark_t,
    pub stream_start_produced: libc::c_int,
    pub stream_end_produced: libc::c_int,
    pub flow_level: libc::c_int,
    pub tokens: C2RustUnnamed_42,
    pub tokens_parsed: size_t,
    pub token_available: libc::c_int,
    pub indents: C2RustUnnamed_41,
    pub indent: libc::c_int,
    pub simple_key_allowed: libc::c_int,
    pub simple_keys: C2RustUnnamed_40,
    pub states: C2RustUnnamed_39,
    pub state: yaml_parser_state_t,
    pub marks: C2RustUnnamed_38,
    pub tag_directives: C2RustUnnamed_37,
    pub aliases: C2RustUnnamed_36,
    pub document: *mut yaml_document_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_36 {
    pub start: *mut yaml_alias_data_t,
    pub end: *mut yaml_alias_data_t,
    pub top: *mut yaml_alias_data_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_37 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_38 {
    pub start: *mut yaml_mark_t,
    pub end: *mut yaml_mark_t,
    pub top: *mut yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_39 {
    pub start: *mut yaml_parser_state_t,
    pub end: *mut yaml_parser_state_t,
    pub top: *mut yaml_parser_state_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_40 {
    pub start: *mut yaml_simple_key_t,
    pub end: *mut yaml_simple_key_t,
    pub top: *mut yaml_simple_key_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_41 {
    pub start: *mut libc::c_int,
    pub end: *mut libc::c_int,
    pub top: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_42 {
    pub start: *mut yaml_token_t,
    pub end: *mut yaml_token_t,
    pub head: *mut yaml_token_t,
    pub tail: *mut yaml_token_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_43 {
    pub start: *mut libc::c_uchar,
    pub end: *mut libc::c_uchar,
    pub pointer: *mut libc::c_uchar,
    pub last: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_44 {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_45 {
    pub string: C2RustUnnamed_46,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_46 {
    pub start: *const libc::c_uchar,
    pub end: *const libc::c_uchar,
    pub current: *const libc::c_uchar,
}
pub type yaml_parser_t = yaml_parser_s;
pub type yaml_write_handler_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_uchar,
    size_t,
) -> libc::c_int;
pub type yaml_emitter_state_e = libc::c_uint;
pub const YAML_EMIT_END_STATE: yaml_emitter_state_e = 17;
pub const YAML_EMIT_BLOCK_MAPPING_VALUE_STATE: yaml_emitter_state_e = 16;
pub const YAML_EMIT_BLOCK_MAPPING_SIMPLE_VALUE_STATE: yaml_emitter_state_e = 15;
pub const YAML_EMIT_BLOCK_MAPPING_KEY_STATE: yaml_emitter_state_e = 14;
pub const YAML_EMIT_BLOCK_MAPPING_FIRST_KEY_STATE: yaml_emitter_state_e = 13;
pub const YAML_EMIT_BLOCK_SEQUENCE_ITEM_STATE: yaml_emitter_state_e = 12;
pub const YAML_EMIT_BLOCK_SEQUENCE_FIRST_ITEM_STATE: yaml_emitter_state_e = 11;
pub const YAML_EMIT_FLOW_MAPPING_VALUE_STATE: yaml_emitter_state_e = 10;
pub const YAML_EMIT_FLOW_MAPPING_SIMPLE_VALUE_STATE: yaml_emitter_state_e = 9;
pub const YAML_EMIT_FLOW_MAPPING_KEY_STATE: yaml_emitter_state_e = 8;
pub const YAML_EMIT_FLOW_MAPPING_FIRST_KEY_STATE: yaml_emitter_state_e = 7;
pub const YAML_EMIT_FLOW_SEQUENCE_ITEM_STATE: yaml_emitter_state_e = 6;
pub const YAML_EMIT_FLOW_SEQUENCE_FIRST_ITEM_STATE: yaml_emitter_state_e = 5;
pub const YAML_EMIT_DOCUMENT_END_STATE: yaml_emitter_state_e = 4;
pub const YAML_EMIT_DOCUMENT_CONTENT_STATE: yaml_emitter_state_e = 3;
pub const YAML_EMIT_DOCUMENT_START_STATE: yaml_emitter_state_e = 2;
pub const YAML_EMIT_FIRST_DOCUMENT_START_STATE: yaml_emitter_state_e = 1;
pub const YAML_EMIT_STREAM_START_STATE: yaml_emitter_state_e = 0;
pub type yaml_emitter_state_t = yaml_emitter_state_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_anchors_s {
    pub references: libc::c_int,
    pub anchor: libc::c_int,
    pub serialized: libc::c_int,
}
pub type yaml_anchors_t = yaml_anchors_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_emitter_s {
    pub error: yaml_error_type_t,
    pub problem: *const libc::c_char,
    pub write_handler: Option::<yaml_write_handler_t>,
    pub write_handler_data: *mut libc::c_void,
    pub output: C2RustUnnamed_56,
    pub buffer: C2RustUnnamed_55,
    pub raw_buffer: C2RustUnnamed_54,
    pub encoding: yaml_encoding_t,
    pub canonical: libc::c_int,
    pub best_indent: libc::c_int,
    pub best_width: libc::c_int,
    pub unicode: libc::c_int,
    pub line_break: yaml_break_t,
    pub states: C2RustUnnamed_53,
    pub state: yaml_emitter_state_t,
    pub events: C2RustUnnamed_52,
    pub indents: C2RustUnnamed_51,
    pub tag_directives: C2RustUnnamed_50,
    pub indent: libc::c_int,
    pub flow_level: libc::c_int,
    pub root_context: libc::c_int,
    pub sequence_context: libc::c_int,
    pub mapping_context: libc::c_int,
    pub simple_key_context: libc::c_int,
    pub line: libc::c_int,
    pub column: libc::c_int,
    pub whitespace: libc::c_int,
    pub indention: libc::c_int,
    pub open_ended: libc::c_int,
    pub anchor_data: C2RustUnnamed_49,
    pub tag_data: C2RustUnnamed_48,
    pub scalar_data: C2RustUnnamed_47,
    pub opened: libc::c_int,
    pub closed: libc::c_int,
    pub anchors: *mut yaml_anchors_t,
    pub last_anchor_id: libc::c_int,
    pub document: *mut yaml_document_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_47 {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub multiline: libc::c_int,
    pub flow_plain_allowed: libc::c_int,
    pub block_plain_allowed: libc::c_int,
    pub single_quoted_allowed: libc::c_int,
    pub block_allowed: libc::c_int,
    pub style: yaml_scalar_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_48 {
    pub handle: *mut yaml_char_t,
    pub handle_length: size_t,
    pub suffix: *mut yaml_char_t,
    pub suffix_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_49 {
    pub anchor: *mut yaml_char_t,
    pub anchor_length: size_t,
    pub alias: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_50 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_51 {
    pub start: *mut libc::c_int,
    pub end: *mut libc::c_int,
    pub top: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_52 {
    pub start: *mut yaml_event_t,
    pub end: *mut yaml_event_t,
    pub head: *mut yaml_event_t,
    pub tail: *mut yaml_event_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_53 {
    pub start: *mut yaml_emitter_state_t,
    pub end: *mut yaml_emitter_state_t,
    pub top: *mut yaml_emitter_state_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_54 {
    pub start: *mut libc::c_uchar,
    pub end: *mut libc::c_uchar,
    pub pointer: *mut libc::c_uchar,
    pub last: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_55 {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_56 {
    pub string: C2RustUnnamed_57,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_57 {
    pub buffer: *mut libc::c_uchar,
    pub size: size_t,
    pub size_written: *mut size_t,
}
pub type yaml_emitter_t = yaml_emitter_s;
pub unsafe extern "C" fn yaml_get_version_string() -> *const libc::c_char {
    return b"0.2.5\0" as *const u8 as *const libc::c_char;
}
pub unsafe extern "C" fn yaml_get_version(
    mut major: *mut libc::c_int,
    mut minor: *mut libc::c_int,
    mut patch: *mut libc::c_int,
) {
    *major = 0 as libc::c_int;
    *minor = 2 as libc::c_int;
    *patch = 5 as libc::c_int;
}
pub unsafe extern "C" fn yaml_malloc(mut size: size_t) -> *mut libc::c_void {
    return malloc(if size != 0 { size } else { 1 as libc::c_int as libc::c_ulong });
}
pub unsafe extern "C" fn yaml_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return if !ptr.is_null() {
        realloc(ptr, if size != 0 { size } else { 1 as libc::c_int as libc::c_ulong })
    } else {
        malloc(if size != 0 { size } else { 1 as libc::c_int as libc::c_ulong })
    };
}
pub unsafe extern "C" fn yaml_free(mut ptr: *mut libc::c_void) {
    if !ptr.is_null() {
        free(ptr);
    }
}
pub unsafe extern "C" fn yaml_strdup(mut str: *const yaml_char_t) -> *mut yaml_char_t {
    if str.is_null() {
        return 0 as *mut yaml_char_t;
    }
    return strdup(str as *mut libc::c_char) as *mut yaml_char_t;
}
pub unsafe extern "C" fn yaml_string_extend(
    mut start: *mut *mut yaml_char_t,
    mut pointer: *mut *mut yaml_char_t,
    mut end: *mut *mut yaml_char_t,
) -> libc::c_int {
    let mut new_start: *mut yaml_char_t = yaml_realloc(
        *start as *mut libc::c_void,
        ((*end).offset_from(*start) as libc::c_long * 2 as libc::c_int as libc::c_long)
            as size_t,
    ) as *mut yaml_char_t;
    if new_start.is_null() {
        return 0 as libc::c_int;
    }
    memset(
        new_start.offset((*end).offset_from(*start) as libc::c_long as isize)
            as *mut libc::c_void,
        0 as libc::c_int,
        (*end).offset_from(*start) as libc::c_long as libc::c_ulong,
    );
    *pointer = new_start.offset((*pointer).offset_from(*start) as libc::c_long as isize);
    *end = new_start
        .offset(
            ((*end).offset_from(*start) as libc::c_long
                * 2 as libc::c_int as libc::c_long) as isize,
        );
    *start = new_start;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn yaml_string_join(
    mut a_start: *mut *mut yaml_char_t,
    mut a_pointer: *mut *mut yaml_char_t,
    mut a_end: *mut *mut yaml_char_t,
    mut b_start: *mut *mut yaml_char_t,
    mut b_pointer: *mut *mut yaml_char_t,
    mut b_end: *mut *mut yaml_char_t,
) -> libc::c_int {
    if *b_start == *b_pointer {
        return 1 as libc::c_int;
    }
    while (*a_end).offset_from(*a_pointer) as libc::c_long
        <= (*b_pointer).offset_from(*b_start) as libc::c_long
    {
        if yaml_string_extend(a_start, a_pointer, a_end) == 0 {
            return 0 as libc::c_int;
        }
    }
    memcpy(
        *a_pointer as *mut libc::c_void,
        *b_start as *const libc::c_void,
        (*b_pointer).offset_from(*b_start) as libc::c_long as libc::c_ulong,
    );
    *a_pointer = (*a_pointer)
        .offset((*b_pointer).offset_from(*b_start) as libc::c_long as isize);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn yaml_stack_extend(
    mut start: *mut *mut libc::c_void,
    mut top: *mut *mut libc::c_void,
    mut end: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut new_start: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*end as *mut libc::c_char).offset_from(*start as *mut libc::c_char)
        as libc::c_long >= (2147483647 as libc::c_int / 2 as libc::c_int) as libc::c_long
    {
        return 0 as libc::c_int;
    }
    new_start = yaml_realloc(
        *start,
        ((*end as *mut libc::c_char).offset_from(*start as *mut libc::c_char)
            as libc::c_long * 2 as libc::c_int as libc::c_long) as size_t,
    );
    if new_start.is_null() {
        return 0 as libc::c_int;
    }
    *top = (new_start as *mut libc::c_char)
        .offset(
            (*top as *mut libc::c_char).offset_from(*start as *mut libc::c_char)
                as libc::c_long as isize,
        ) as *mut libc::c_void;
    *end = (new_start as *mut libc::c_char)
        .offset(
            ((*end as *mut libc::c_char).offset_from(*start as *mut libc::c_char)
                as libc::c_long * 2 as libc::c_int as libc::c_long) as isize,
        ) as *mut libc::c_void;
    *start = new_start;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn yaml_queue_extend(
    mut start: *mut *mut libc::c_void,
    mut head: *mut *mut libc::c_void,
    mut tail: *mut *mut libc::c_void,
    mut end: *mut *mut libc::c_void,
) -> libc::c_int {
    if *start == *head && *tail == *end {
        let mut new_start: *mut libc::c_void = yaml_realloc(
            *start,
            ((*end as *mut libc::c_char).offset_from(*start as *mut libc::c_char)
                as libc::c_long * 2 as libc::c_int as libc::c_long) as size_t,
        );
        if new_start.is_null() {
            return 0 as libc::c_int;
        }
        *head = (new_start as *mut libc::c_char)
            .offset(
                (*head as *mut libc::c_char).offset_from(*start as *mut libc::c_char)
                    as libc::c_long as isize,
            ) as *mut libc::c_void;
        *tail = (new_start as *mut libc::c_char)
            .offset(
                (*tail as *mut libc::c_char).offset_from(*start as *mut libc::c_char)
                    as libc::c_long as isize,
            ) as *mut libc::c_void;
        *end = (new_start as *mut libc::c_char)
            .offset(
                ((*end as *mut libc::c_char).offset_from(*start as *mut libc::c_char)
                    as libc::c_long * 2 as libc::c_int as libc::c_long) as isize,
            ) as *mut libc::c_void;
        *start = new_start;
    }
    if *tail == *end {
        if *head != *tail {
            memmove(
                *start,
                *head,
                (*tail as *mut libc::c_char).offset_from(*head as *mut libc::c_char)
                    as libc::c_long as libc::c_ulong,
            );
        }
        *tail = (*start as *mut libc::c_char)
            .offset(
                (*tail as *mut libc::c_char).offset_from(*head as *mut libc::c_char)
                    as libc::c_long as isize,
            ) as *mut libc::c_void;
        *head = *start;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn yaml_parser_initialize(
    mut parser: *mut yaml_parser_t,
) -> libc::c_int {
    if !parser.is_null() {} else {
        __assert_fail(
            b"parser\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            179 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"int yaml_parser_initialize(yaml_parser_t *)\0"))
                .as_ptr(),
        );
    }
    'c_10024: {
        if !parser.is_null() {} else {
            __assert_fail(
                b"parser\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                179 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"int yaml_parser_initialize(yaml_parser_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        parser as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_parser_t>() as libc::c_ulong,
    );
    (*parser)
        .raw_buffer
        .start = yaml_malloc(16384 as libc::c_int as size_t) as *mut yaml_char_t;
    if !(if !((*parser).raw_buffer.start).is_null() {
        (*parser).raw_buffer.pointer = (*parser).raw_buffer.start;
        (*parser).raw_buffer.last = (*parser).raw_buffer.pointer;
        (*parser)
            .raw_buffer
            .end = ((*parser).raw_buffer.start).offset(16384 as libc::c_int as isize);
        1 as libc::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0)
    {
        (*parser)
            .buffer
            .start = yaml_malloc((16384 as libc::c_int * 3 as libc::c_int) as size_t)
            as *mut yaml_char_t;
        if !(if !((*parser).buffer.start).is_null() {
            (*parser).buffer.pointer = (*parser).buffer.start;
            (*parser).buffer.last = (*parser).buffer.pointer;
            (*parser)
                .buffer
                .end = ((*parser).buffer.start)
                .offset((16384 as libc::c_int * 3 as libc::c_int) as isize);
            1 as libc::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0)
        {
            (*parser)
                .tokens
                .start = yaml_malloc(
                (16 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<yaml_token_t>() as libc::c_ulong),
            ) as *mut yaml_token_t;
            if !(if !((*parser).tokens.start).is_null() {
                (*parser).tokens.tail = (*parser).tokens.start;
                (*parser).tokens.head = (*parser).tokens.tail;
                (*parser)
                    .tokens
                    .end = ((*parser).tokens.start).offset(16 as libc::c_int as isize);
                1 as libc::c_int
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as libc::c_int
            } == 0)
            {
                (*parser)
                    .indents
                    .start = yaml_malloc(
                    (16 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ),
                ) as *mut libc::c_int;
                if !(if !((*parser).indents.start).is_null() {
                    (*parser).indents.top = (*parser).indents.start;
                    (*parser)
                        .indents
                        .end = ((*parser).indents.start)
                        .offset(16 as libc::c_int as isize);
                    1 as libc::c_int
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } == 0)
                {
                    (*parser)
                        .simple_keys
                        .start = yaml_malloc(
                        (16 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<yaml_simple_key_t>() as libc::c_ulong,
                            ),
                    ) as *mut yaml_simple_key_t;
                    if !(if !((*parser).simple_keys.start).is_null() {
                        (*parser).simple_keys.top = (*parser).simple_keys.start;
                        (*parser)
                            .simple_keys
                            .end = ((*parser).simple_keys.start)
                            .offset(16 as libc::c_int as isize);
                        1 as libc::c_int
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0 as libc::c_int
                    } == 0)
                    {
                        (*parser)
                            .states
                            .start = yaml_malloc(
                            (16 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<yaml_parser_state_t>()
                                        as libc::c_ulong,
                                ),
                        ) as *mut yaml_parser_state_t;
                        if !(if !((*parser).states.start).is_null() {
                            (*parser).states.top = (*parser).states.start;
                            (*parser)
                                .states
                                .end = ((*parser).states.start)
                                .offset(16 as libc::c_int as isize);
                            1 as libc::c_int
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as libc::c_int
                        } == 0)
                        {
                            (*parser)
                                .marks
                                .start = yaml_malloc(
                                (16 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<yaml_mark_t>() as libc::c_ulong,
                                    ),
                            ) as *mut yaml_mark_t;
                            if !(if !((*parser).marks.start).is_null() {
                                (*parser).marks.top = (*parser).marks.start;
                                (*parser)
                                    .marks
                                    .end = ((*parser).marks.start)
                                    .offset(16 as libc::c_int as isize);
                                1 as libc::c_int
                            } else {
                                (*parser).error = YAML_MEMORY_ERROR;
                                0 as libc::c_int
                            } == 0)
                            {
                                (*parser)
                                    .tag_directives
                                    .start = yaml_malloc(
                                    (16 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<yaml_tag_directive_t>()
                                                as libc::c_ulong,
                                        ),
                                ) as *mut yaml_tag_directive_t;
                                if !(if !((*parser).tag_directives.start).is_null() {
                                    (*parser)
                                        .tag_directives
                                        .top = (*parser).tag_directives.start;
                                    (*parser)
                                        .tag_directives
                                        .end = ((*parser).tag_directives.start)
                                        .offset(16 as libc::c_int as isize);
                                    1 as libc::c_int
                                } else {
                                    (*parser).error = YAML_MEMORY_ERROR;
                                    0 as libc::c_int
                                } == 0)
                                {
                                    return 1 as libc::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    yaml_free((*parser).raw_buffer.start as *mut libc::c_void);
    (*parser).raw_buffer.end = 0 as *mut libc::c_uchar;
    (*parser).raw_buffer.pointer = (*parser).raw_buffer.end;
    (*parser).raw_buffer.start = (*parser).raw_buffer.pointer;
    yaml_free((*parser).buffer.start as *mut libc::c_void);
    (*parser).buffer.end = 0 as *mut yaml_char_t;
    (*parser).buffer.pointer = (*parser).buffer.end;
    (*parser).buffer.start = (*parser).buffer.pointer;
    yaml_free((*parser).tokens.start as *mut libc::c_void);
    (*parser).tokens.end = 0 as *mut yaml_token_t;
    (*parser).tokens.tail = (*parser).tokens.end;
    (*parser).tokens.head = (*parser).tokens.tail;
    (*parser).tokens.start = (*parser).tokens.head;
    yaml_free((*parser).indents.start as *mut libc::c_void);
    (*parser).indents.end = 0 as *mut libc::c_int;
    (*parser).indents.top = (*parser).indents.end;
    (*parser).indents.start = (*parser).indents.top;
    yaml_free((*parser).simple_keys.start as *mut libc::c_void);
    (*parser).simple_keys.end = 0 as *mut yaml_simple_key_t;
    (*parser).simple_keys.top = (*parser).simple_keys.end;
    (*parser).simple_keys.start = (*parser).simple_keys.top;
    yaml_free((*parser).states.start as *mut libc::c_void);
    (*parser).states.end = 0 as *mut yaml_parser_state_t;
    (*parser).states.top = (*parser).states.end;
    (*parser).states.start = (*parser).states.top;
    yaml_free((*parser).marks.start as *mut libc::c_void);
    (*parser).marks.end = 0 as *mut yaml_mark_t;
    (*parser).marks.top = (*parser).marks.end;
    (*parser).marks.start = (*parser).marks.top;
    yaml_free((*parser).tag_directives.start as *mut libc::c_void);
    (*parser).tag_directives.end = 0 as *mut yaml_tag_directive_t;
    (*parser).tag_directives.top = (*parser).tag_directives.end;
    (*parser).tag_directives.start = (*parser).tag_directives.top;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn yaml_parser_delete(mut parser: *mut yaml_parser_t) {
    if !parser.is_null() {} else {
        __assert_fail(
            b"parser\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            222 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"void yaml_parser_delete(yaml_parser_t *)\0"))
                .as_ptr(),
        );
    }
    'c_10420: {
        if !parser.is_null() {} else {
            __assert_fail(
                b"parser\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                222 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"void yaml_parser_delete(yaml_parser_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    yaml_free((*parser).raw_buffer.start as *mut libc::c_void);
    (*parser).raw_buffer.end = 0 as *mut libc::c_uchar;
    (*parser).raw_buffer.pointer = (*parser).raw_buffer.end;
    (*parser).raw_buffer.start = (*parser).raw_buffer.pointer;
    yaml_free((*parser).buffer.start as *mut libc::c_void);
    (*parser).buffer.end = 0 as *mut yaml_char_t;
    (*parser).buffer.pointer = (*parser).buffer.end;
    (*parser).buffer.start = (*parser).buffer.pointer;
    while !((*parser).tokens.head == (*parser).tokens.tail) {
        let fresh0 = (*parser).tokens.head;
        (*parser).tokens.head = ((*parser).tokens.head).offset(1);
        yaml_token_delete(fresh0);
    }
    yaml_free((*parser).tokens.start as *mut libc::c_void);
    (*parser).tokens.end = 0 as *mut yaml_token_t;
    (*parser).tokens.tail = (*parser).tokens.end;
    (*parser).tokens.head = (*parser).tokens.tail;
    (*parser).tokens.start = (*parser).tokens.head;
    yaml_free((*parser).indents.start as *mut libc::c_void);
    (*parser).indents.end = 0 as *mut libc::c_int;
    (*parser).indents.top = (*parser).indents.end;
    (*parser).indents.start = (*parser).indents.top;
    yaml_free((*parser).simple_keys.start as *mut libc::c_void);
    (*parser).simple_keys.end = 0 as *mut yaml_simple_key_t;
    (*parser).simple_keys.top = (*parser).simple_keys.end;
    (*parser).simple_keys.start = (*parser).simple_keys.top;
    yaml_free((*parser).states.start as *mut libc::c_void);
    (*parser).states.end = 0 as *mut yaml_parser_state_t;
    (*parser).states.top = (*parser).states.end;
    (*parser).states.start = (*parser).states.top;
    yaml_free((*parser).marks.start as *mut libc::c_void);
    (*parser).marks.end = 0 as *mut yaml_mark_t;
    (*parser).marks.top = (*parser).marks.end;
    (*parser).marks.start = (*parser).marks.top;
    while !((*parser).tag_directives.start == (*parser).tag_directives.top) {
        (*parser).tag_directives.top = ((*parser).tag_directives.top).offset(-1);
        let mut tag_directive: yaml_tag_directive_t = *(*parser).tag_directives.top;
        yaml_free(tag_directive.handle as *mut libc::c_void);
        yaml_free(tag_directive.prefix as *mut libc::c_void);
    }
    yaml_free((*parser).tag_directives.start as *mut libc::c_void);
    (*parser).tag_directives.end = 0 as *mut yaml_tag_directive_t;
    (*parser).tag_directives.top = (*parser).tag_directives.end;
    (*parser).tag_directives.start = (*parser).tag_directives.top;
    memset(
        parser as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_parser_t>() as libc::c_ulong,
    );
}
unsafe extern "C" fn yaml_string_read_handler(
    mut data: *mut libc::c_void,
    mut buffer: *mut libc::c_uchar,
    mut size: size_t,
    mut size_read: *mut size_t,
) -> libc::c_int {
    let mut parser: *mut yaml_parser_t = data as *mut yaml_parser_t;
    if (*parser).input.string.current == (*parser).input.string.end {
        *size_read = 0 as libc::c_int as size_t;
        return 1 as libc::c_int;
    }
    if size
        > ((*parser).input.string.end).offset_from((*parser).input.string.current)
            as libc::c_long as size_t
    {
        size = ((*parser).input.string.end).offset_from((*parser).input.string.current)
            as libc::c_long as size_t;
    }
    memcpy(
        buffer as *mut libc::c_void,
        (*parser).input.string.current as *const libc::c_void,
        size,
    );
    (*parser)
        .input
        .string
        .current = ((*parser).input.string.current).offset(size as isize);
    *size_read = size;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_file_read_handler(
    mut data: *mut libc::c_void,
    mut buffer: *mut libc::c_uchar,
    mut size: size_t,
    mut size_read: *mut size_t,
) -> libc::c_int {
    let mut parser: *mut yaml_parser_t = data as *mut yaml_parser_t;
    *size_read = fread(
        buffer as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        size,
        (*parser).input.file,
    );
    return (ferror((*parser).input.file) == 0) as libc::c_int;
}
pub unsafe extern "C" fn yaml_parser_set_input_string(
    mut parser: *mut yaml_parser_t,
    mut input: *const libc::c_uchar,
    mut size: size_t,
) {
    if !parser.is_null() {} else {
        __assert_fail(
            b"parser\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            292 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void yaml_parser_set_input_string(yaml_parser_t *, const unsigned char *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_10685: {
        if !parser.is_null() {} else {
            __assert_fail(
                b"parser\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                292 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 82],
                    &[libc::c_char; 82],
                >(
                    b"void yaml_parser_set_input_string(yaml_parser_t *, const unsigned char *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if ((*parser).read_handler).is_none() {} else {
        __assert_fail(
            b"!parser->read_handler\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            293 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void yaml_parser_set_input_string(yaml_parser_t *, const unsigned char *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_10647: {
        if ((*parser).read_handler).is_none() {} else {
            __assert_fail(
                b"!parser->read_handler\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                293 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 82],
                    &[libc::c_char; 82],
                >(
                    b"void yaml_parser_set_input_string(yaml_parser_t *, const unsigned char *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !input.is_null() {} else {
        __assert_fail(
            b"input\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            294 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 82],
                &[libc::c_char; 82],
            >(
                b"void yaml_parser_set_input_string(yaml_parser_t *, const unsigned char *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_10614: {
        if !input.is_null() {} else {
            __assert_fail(
                b"input\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                294 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 82],
                    &[libc::c_char; 82],
                >(
                    b"void yaml_parser_set_input_string(yaml_parser_t *, const unsigned char *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    (*parser)
        .read_handler = Some(
        yaml_string_read_handler
            as unsafe extern "C" fn(
                *mut libc::c_void,
                *mut libc::c_uchar,
                size_t,
                *mut size_t,
            ) -> libc::c_int,
    );
    (*parser).read_handler_data = parser as *mut libc::c_void;
    (*parser).input.string.start = input;
    (*parser).input.string.current = input;
    (*parser).input.string.end = input.offset(size as isize);
}
pub unsafe extern "C" fn yaml_parser_set_input_file(
    mut parser: *mut yaml_parser_t,
    mut file: *mut FILE,
) {
    if !parser.is_null() {} else {
        __assert_fail(
            b"parser\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            311 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"void yaml_parser_set_input_file(yaml_parser_t *, FILE *)\0"))
                .as_ptr(),
        );
    }
    'c_10862: {
        if !parser.is_null() {} else {
            __assert_fail(
                b"parser\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                311 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"void yaml_parser_set_input_file(yaml_parser_t *, FILE *)\0"))
                    .as_ptr(),
            );
        }
    };
    if ((*parser).read_handler).is_none() {} else {
        __assert_fail(
            b"!parser->read_handler\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"void yaml_parser_set_input_file(yaml_parser_t *, FILE *)\0"))
                .as_ptr(),
        );
    }
    'c_10824: {
        if ((*parser).read_handler).is_none() {} else {
            __assert_fail(
                b"!parser->read_handler\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                312 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"void yaml_parser_set_input_file(yaml_parser_t *, FILE *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !file.is_null() {} else {
        __assert_fail(
            b"file\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            313 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"void yaml_parser_set_input_file(yaml_parser_t *, FILE *)\0"))
                .as_ptr(),
        );
    }
    'c_10790: {
        if !file.is_null() {} else {
            __assert_fail(
                b"file\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                313 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"void yaml_parser_set_input_file(yaml_parser_t *, FILE *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*parser)
        .read_handler = Some(
        yaml_file_read_handler
            as unsafe extern "C" fn(
                *mut libc::c_void,
                *mut libc::c_uchar,
                size_t,
                *mut size_t,
            ) -> libc::c_int,
    );
    (*parser).read_handler_data = parser as *mut libc::c_void;
    (*parser).input.file = file;
}
pub unsafe extern "C" fn yaml_parser_set_input(
    mut parser: *mut yaml_parser_t,
    mut handler: Option::<yaml_read_handler_t>,
    mut data: *mut libc::c_void,
) {
    if !parser.is_null() {} else {
        __assert_fail(
            b"parser\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            329 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"void yaml_parser_set_input(yaml_parser_t *, yaml_read_handler_t *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_10989: {
        if !parser.is_null() {} else {
            __assert_fail(
                b"parser\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                329 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"void yaml_parser_set_input(yaml_parser_t *, yaml_read_handler_t *, void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if ((*parser).read_handler).is_none() {} else {
        __assert_fail(
            b"!parser->read_handler\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"void yaml_parser_set_input(yaml_parser_t *, yaml_read_handler_t *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_10951: {
        if ((*parser).read_handler).is_none() {} else {
            __assert_fail(
                b"!parser->read_handler\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                330 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"void yaml_parser_set_input(yaml_parser_t *, yaml_read_handler_t *, void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if handler.is_some() {} else {
        __assert_fail(
            b"handler\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            331 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"void yaml_parser_set_input(yaml_parser_t *, yaml_read_handler_t *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_10917: {
        if handler.is_some() {} else {
            __assert_fail(
                b"handler\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                331 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"void yaml_parser_set_input(yaml_parser_t *, yaml_read_handler_t *, void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    (*parser).read_handler = handler;
    (*parser).read_handler_data = data;
}
pub unsafe extern "C" fn yaml_parser_set_encoding(
    mut parser: *mut yaml_parser_t,
    mut encoding: yaml_encoding_t,
) {
    if !parser.is_null() {} else {
        __assert_fail(
            b"parser\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            344 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"void yaml_parser_set_encoding(yaml_parser_t *, yaml_encoding_t)\0"))
                .as_ptr(),
        );
    }
    'c_11074: {
        if !parser.is_null() {} else {
            __assert_fail(
                b"parser\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                344 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 64],
                    &[libc::c_char; 64],
                >(b"void yaml_parser_set_encoding(yaml_parser_t *, yaml_encoding_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*parser).encoding as u64 == 0 {} else {
        __assert_fail(
            b"!parser->encoding\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            345 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"void yaml_parser_set_encoding(yaml_parser_t *, yaml_encoding_t)\0"))
                .as_ptr(),
        );
    }
    'c_11035: {
        if (*parser).encoding as u64 == 0 {} else {
            __assert_fail(
                b"!parser->encoding\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                345 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 64],
                    &[libc::c_char; 64],
                >(b"void yaml_parser_set_encoding(yaml_parser_t *, yaml_encoding_t)\0"))
                    .as_ptr(),
            );
        }
    };
    (*parser).encoding = encoding;
}
pub unsafe extern "C" fn yaml_emitter_initialize(
    mut emitter: *mut yaml_emitter_t,
) -> libc::c_int {
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            357 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"int yaml_emitter_initialize(yaml_emitter_t *)\0"))
                .as_ptr(),
        );
    }
    'c_11916: {
        if !emitter.is_null() {} else {
            __assert_fail(
                b"emitter\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                357 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 46],
                    &[libc::c_char; 46],
                >(b"int yaml_emitter_initialize(yaml_emitter_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        emitter as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_emitter_t>() as libc::c_ulong,
    );
    (*emitter)
        .buffer
        .start = yaml_malloc(16384 as libc::c_int as size_t) as *mut yaml_char_t;
    if !(if !((*emitter).buffer.start).is_null() {
        (*emitter).buffer.pointer = (*emitter).buffer.start;
        (*emitter).buffer.last = (*emitter).buffer.pointer;
        (*emitter)
            .buffer
            .end = ((*emitter).buffer.start).offset(16384 as libc::c_int as isize);
        1 as libc::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0)
    {
        (*emitter)
            .raw_buffer
            .start = yaml_malloc(
            (16384 as libc::c_int * 2 as libc::c_int + 2 as libc::c_int) as size_t,
        ) as *mut yaml_char_t;
        if !(if !((*emitter).raw_buffer.start).is_null() {
            (*emitter).raw_buffer.pointer = (*emitter).raw_buffer.start;
            (*emitter).raw_buffer.last = (*emitter).raw_buffer.pointer;
            (*emitter)
                .raw_buffer
                .end = ((*emitter).raw_buffer.start)
                .offset(
                    (16384 as libc::c_int * 2 as libc::c_int + 2 as libc::c_int) as isize,
                );
            1 as libc::c_int
        } else {
            (*emitter).error = YAML_MEMORY_ERROR;
            0 as libc::c_int
        } == 0)
        {
            (*emitter)
                .states
                .start = yaml_malloc(
                (16 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<yaml_emitter_state_t>() as libc::c_ulong,
                    ),
            ) as *mut yaml_emitter_state_t;
            if !(if !((*emitter).states.start).is_null() {
                (*emitter).states.top = (*emitter).states.start;
                (*emitter)
                    .states
                    .end = ((*emitter).states.start).offset(16 as libc::c_int as isize);
                1 as libc::c_int
            } else {
                (*emitter).error = YAML_MEMORY_ERROR;
                0 as libc::c_int
            } == 0)
            {
                (*emitter)
                    .events
                    .start = yaml_malloc(
                    (16 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
                        ),
                ) as *mut yaml_event_t;
                if !(if !((*emitter).events.start).is_null() {
                    (*emitter).events.tail = (*emitter).events.start;
                    (*emitter).events.head = (*emitter).events.tail;
                    (*emitter)
                        .events
                        .end = ((*emitter).events.start)
                        .offset(16 as libc::c_int as isize);
                    1 as libc::c_int
                } else {
                    (*emitter).error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } == 0)
                {
                    (*emitter)
                        .indents
                        .start = yaml_malloc(
                        (16 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    ) as *mut libc::c_int;
                    if !(if !((*emitter).indents.start).is_null() {
                        (*emitter).indents.top = (*emitter).indents.start;
                        (*emitter)
                            .indents
                            .end = ((*emitter).indents.start)
                            .offset(16 as libc::c_int as isize);
                        1 as libc::c_int
                    } else {
                        (*emitter).error = YAML_MEMORY_ERROR;
                        0 as libc::c_int
                    } == 0)
                    {
                        (*emitter)
                            .tag_directives
                            .start = yaml_malloc(
                            (16 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<yaml_tag_directive_t>()
                                        as libc::c_ulong,
                                ),
                        ) as *mut yaml_tag_directive_t;
                        if !(if !((*emitter).tag_directives.start).is_null() {
                            (*emitter)
                                .tag_directives
                                .top = (*emitter).tag_directives.start;
                            (*emitter)
                                .tag_directives
                                .end = ((*emitter).tag_directives.start)
                                .offset(16 as libc::c_int as isize);
                            1 as libc::c_int
                        } else {
                            (*emitter).error = YAML_MEMORY_ERROR;
                            0 as libc::c_int
                        } == 0)
                        {
                            return 1 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    yaml_free((*emitter).buffer.start as *mut libc::c_void);
    (*emitter).buffer.end = 0 as *mut yaml_char_t;
    (*emitter).buffer.pointer = (*emitter).buffer.end;
    (*emitter).buffer.start = (*emitter).buffer.pointer;
    yaml_free((*emitter).raw_buffer.start as *mut libc::c_void);
    (*emitter).raw_buffer.end = 0 as *mut libc::c_uchar;
    (*emitter).raw_buffer.pointer = (*emitter).raw_buffer.end;
    (*emitter).raw_buffer.start = (*emitter).raw_buffer.pointer;
    yaml_free((*emitter).states.start as *mut libc::c_void);
    (*emitter).states.end = 0 as *mut yaml_emitter_state_t;
    (*emitter).states.top = (*emitter).states.end;
    (*emitter).states.start = (*emitter).states.top;
    yaml_free((*emitter).events.start as *mut libc::c_void);
    (*emitter).events.end = 0 as *mut yaml_event_t;
    (*emitter).events.tail = (*emitter).events.end;
    (*emitter).events.head = (*emitter).events.tail;
    (*emitter).events.start = (*emitter).events.head;
    yaml_free((*emitter).indents.start as *mut libc::c_void);
    (*emitter).indents.end = 0 as *mut libc::c_int;
    (*emitter).indents.top = (*emitter).indents.end;
    (*emitter).indents.start = (*emitter).indents.top;
    yaml_free((*emitter).tag_directives.start as *mut libc::c_void);
    (*emitter).tag_directives.end = 0 as *mut yaml_tag_directive_t;
    (*emitter).tag_directives.top = (*emitter).tag_directives.end;
    (*emitter).tag_directives.start = (*emitter).tag_directives.top;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn yaml_emitter_delete(mut emitter: *mut yaml_emitter_t) {
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            394 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"void yaml_emitter_delete(yaml_emitter_t *)\0"))
                .as_ptr(),
        );
    }
    'c_12253: {
        if !emitter.is_null() {} else {
            __assert_fail(
                b"emitter\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                394 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"void yaml_emitter_delete(yaml_emitter_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    yaml_free((*emitter).buffer.start as *mut libc::c_void);
    (*emitter).buffer.end = 0 as *mut yaml_char_t;
    (*emitter).buffer.pointer = (*emitter).buffer.end;
    (*emitter).buffer.start = (*emitter).buffer.pointer;
    yaml_free((*emitter).raw_buffer.start as *mut libc::c_void);
    (*emitter).raw_buffer.end = 0 as *mut libc::c_uchar;
    (*emitter).raw_buffer.pointer = (*emitter).raw_buffer.end;
    (*emitter).raw_buffer.start = (*emitter).raw_buffer.pointer;
    yaml_free((*emitter).states.start as *mut libc::c_void);
    (*emitter).states.end = 0 as *mut yaml_emitter_state_t;
    (*emitter).states.top = (*emitter).states.end;
    (*emitter).states.start = (*emitter).states.top;
    while !((*emitter).events.head == (*emitter).events.tail) {
        let fresh1 = (*emitter).events.head;
        (*emitter).events.head = ((*emitter).events.head).offset(1);
        yaml_event_delete(fresh1);
    }
    yaml_free((*emitter).events.start as *mut libc::c_void);
    (*emitter).events.end = 0 as *mut yaml_event_t;
    (*emitter).events.tail = (*emitter).events.end;
    (*emitter).events.head = (*emitter).events.tail;
    (*emitter).events.start = (*emitter).events.head;
    yaml_free((*emitter).indents.start as *mut libc::c_void);
    (*emitter).indents.end = 0 as *mut libc::c_int;
    (*emitter).indents.top = (*emitter).indents.end;
    (*emitter).indents.start = (*emitter).indents.top;
    while !((*emitter).tag_directives.start == (*emitter).tag_directives.top) {
        (*emitter).tag_directives.top = ((*emitter).tag_directives.top).offset(-1);
        let mut tag_directive: yaml_tag_directive_t = *(*emitter).tag_directives.top;
        yaml_free(tag_directive.handle as *mut libc::c_void);
        yaml_free(tag_directive.prefix as *mut libc::c_void);
    }
    yaml_free((*emitter).tag_directives.start as *mut libc::c_void);
    (*emitter).tag_directives.end = 0 as *mut yaml_tag_directive_t;
    (*emitter).tag_directives.top = (*emitter).tag_directives.end;
    (*emitter).tag_directives.start = (*emitter).tag_directives.top;
    yaml_free((*emitter).anchors as *mut libc::c_void);
    memset(
        emitter as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_emitter_t>() as libc::c_ulong,
    );
}
unsafe extern "C" fn yaml_string_write_handler(
    mut data: *mut libc::c_void,
    mut buffer: *mut libc::c_uchar,
    mut size: size_t,
) -> libc::c_int {
    let mut emitter: *mut yaml_emitter_t = data as *mut yaml_emitter_t;
    if ((*emitter).output.string.size)
        .wrapping_sub(*(*emitter).output.string.size_written) < size
    {
        memcpy(
            ((*emitter).output.string.buffer)
                .offset(*(*emitter).output.string.size_written as isize)
                as *mut libc::c_void,
            buffer as *const libc::c_void,
            ((*emitter).output.string.size)
                .wrapping_sub(*(*emitter).output.string.size_written),
        );
        *(*emitter).output.string.size_written = (*emitter).output.string.size;
        return 0 as libc::c_int;
    }
    memcpy(
        ((*emitter).output.string.buffer)
            .offset(*(*emitter).output.string.size_written as isize)
            as *mut libc::c_void,
        buffer as *const libc::c_void,
        size,
    );
    *(*emitter)
        .output
        .string
        .size_written = (*(*emitter).output.string.size_written as libc::c_ulong)
        .wrapping_add(size) as size_t as size_t;
    return 1 as libc::c_int;
}
unsafe extern "C" fn yaml_file_write_handler(
    mut data: *mut libc::c_void,
    mut buffer: *mut libc::c_uchar,
    mut size: size_t,
) -> libc::c_int {
    let mut emitter: *mut yaml_emitter_t = data as *mut yaml_emitter_t;
    return (fwrite(
        buffer as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        size,
        (*emitter).output.file,
    ) == size) as libc::c_int;
}
pub unsafe extern "C" fn yaml_emitter_set_output_string(
    mut emitter: *mut yaml_emitter_t,
    mut output: *mut libc::c_uchar,
    mut size: size_t,
    mut size_written: *mut size_t,
) {
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            460 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 89],
                &[libc::c_char; 89],
            >(
                b"void yaml_emitter_set_output_string(yaml_emitter_t *, unsigned char *, size_t, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_12541: {
        if !emitter.is_null() {} else {
            __assert_fail(
                b"emitter\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                460 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 89],
                    &[libc::c_char; 89],
                >(
                    b"void yaml_emitter_set_output_string(yaml_emitter_t *, unsigned char *, size_t, size_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if ((*emitter).write_handler).is_none() {} else {
        __assert_fail(
            b"!emitter->write_handler\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            461 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 89],
                &[libc::c_char; 89],
            >(
                b"void yaml_emitter_set_output_string(yaml_emitter_t *, unsigned char *, size_t, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_12502: {
        if ((*emitter).write_handler).is_none() {} else {
            __assert_fail(
                b"!emitter->write_handler\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                461 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 89],
                    &[libc::c_char; 89],
                >(
                    b"void yaml_emitter_set_output_string(yaml_emitter_t *, unsigned char *, size_t, size_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !output.is_null() {} else {
        __assert_fail(
            b"output\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            462 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 89],
                &[libc::c_char; 89],
            >(
                b"void yaml_emitter_set_output_string(yaml_emitter_t *, unsigned char *, size_t, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_12469: {
        if !output.is_null() {} else {
            __assert_fail(
                b"output\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                462 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 89],
                    &[libc::c_char; 89],
                >(
                    b"void yaml_emitter_set_output_string(yaml_emitter_t *, unsigned char *, size_t, size_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    (*emitter)
        .write_handler = Some(
        yaml_string_write_handler
            as unsafe extern "C" fn(
                *mut libc::c_void,
                *mut libc::c_uchar,
                size_t,
            ) -> libc::c_int,
    );
    (*emitter).write_handler_data = emitter as *mut libc::c_void;
    (*emitter).output.string.buffer = output;
    (*emitter).output.string.size = size;
    (*emitter).output.string.size_written = size_written;
    *size_written = 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn yaml_emitter_set_output_file(
    mut emitter: *mut yaml_emitter_t,
    mut file: *mut FILE,
) {
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            480 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"void yaml_emitter_set_output_file(yaml_emitter_t *, FILE *)\0"))
                .as_ptr(),
        );
    }
    'c_12705: {
        if !emitter.is_null() {} else {
            __assert_fail(
                b"emitter\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                480 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"void yaml_emitter_set_output_file(yaml_emitter_t *, FILE *)\0"))
                    .as_ptr(),
            );
        }
    };
    if ((*emitter).write_handler).is_none() {} else {
        __assert_fail(
            b"!emitter->write_handler\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            481 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"void yaml_emitter_set_output_file(yaml_emitter_t *, FILE *)\0"))
                .as_ptr(),
        );
    }
    'c_12667: {
        if ((*emitter).write_handler).is_none() {} else {
            __assert_fail(
                b"!emitter->write_handler\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                481 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"void yaml_emitter_set_output_file(yaml_emitter_t *, FILE *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !file.is_null() {} else {
        __assert_fail(
            b"file\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            482 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"void yaml_emitter_set_output_file(yaml_emitter_t *, FILE *)\0"))
                .as_ptr(),
        );
    }
    'c_12635: {
        if !file.is_null() {} else {
            __assert_fail(
                b"file\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                482 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"void yaml_emitter_set_output_file(yaml_emitter_t *, FILE *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*emitter)
        .write_handler = Some(
        yaml_file_write_handler
            as unsafe extern "C" fn(
                *mut libc::c_void,
                *mut libc::c_uchar,
                size_t,
            ) -> libc::c_int,
    );
    (*emitter).write_handler_data = emitter as *mut libc::c_void;
    (*emitter).output.file = file;
}
pub unsafe extern "C" fn yaml_emitter_set_output(
    mut emitter: *mut yaml_emitter_t,
    mut handler: Option::<yaml_write_handler_t>,
    mut data: *mut libc::c_void,
) {
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            498 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"void yaml_emitter_set_output(yaml_emitter_t *, yaml_write_handler_t *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_12831: {
        if !emitter.is_null() {} else {
            __assert_fail(
                b"emitter\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                498 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 79],
                    &[libc::c_char; 79],
                >(
                    b"void yaml_emitter_set_output(yaml_emitter_t *, yaml_write_handler_t *, void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if ((*emitter).write_handler).is_none() {} else {
        __assert_fail(
            b"!emitter->write_handler\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            499 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"void yaml_emitter_set_output(yaml_emitter_t *, yaml_write_handler_t *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_12793: {
        if ((*emitter).write_handler).is_none() {} else {
            __assert_fail(
                b"!emitter->write_handler\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                499 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 79],
                    &[libc::c_char; 79],
                >(
                    b"void yaml_emitter_set_output(yaml_emitter_t *, yaml_write_handler_t *, void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if handler.is_some() {} else {
        __assert_fail(
            b"handler\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            500 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"void yaml_emitter_set_output(yaml_emitter_t *, yaml_write_handler_t *, void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_12760: {
        if handler.is_some() {} else {
            __assert_fail(
                b"handler\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                500 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 79],
                    &[libc::c_char; 79],
                >(
                    b"void yaml_emitter_set_output(yaml_emitter_t *, yaml_write_handler_t *, void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    (*emitter).write_handler = handler;
    (*emitter).write_handler_data = data;
}
pub unsafe extern "C" fn yaml_emitter_set_encoding(
    mut emitter: *mut yaml_emitter_t,
    mut encoding: yaml_encoding_t,
) {
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            513 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"void yaml_emitter_set_encoding(yaml_emitter_t *, yaml_encoding_t)\0"))
                .as_ptr(),
        );
    }
    'c_12916: {
        if !emitter.is_null() {} else {
            __assert_fail(
                b"emitter\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                513 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"void yaml_emitter_set_encoding(yaml_emitter_t *, yaml_encoding_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*emitter).encoding as u64 == 0 {} else {
        __assert_fail(
            b"!emitter->encoding\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            514 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"void yaml_emitter_set_encoding(yaml_emitter_t *, yaml_encoding_t)\0"))
                .as_ptr(),
        );
    }
    'c_12877: {
        if (*emitter).encoding as u64 == 0 {} else {
            __assert_fail(
                b"!emitter->encoding\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                514 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"void yaml_emitter_set_encoding(yaml_emitter_t *, yaml_encoding_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    (*emitter).encoding = encoding;
}
pub unsafe extern "C" fn yaml_emitter_set_canonical(
    mut emitter: *mut yaml_emitter_t,
    mut canonical: libc::c_int,
) {
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            526 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"void yaml_emitter_set_canonical(yaml_emitter_t *, int)\0"))
                .as_ptr(),
        );
    }
    'c_12964: {
        if !emitter.is_null() {} else {
            __assert_fail(
                b"emitter\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                526 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"void yaml_emitter_set_canonical(yaml_emitter_t *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    (*emitter).canonical = (canonical != 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn yaml_emitter_set_indent(
    mut emitter: *mut yaml_emitter_t,
    mut indent: libc::c_int,
) {
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            538 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"void yaml_emitter_set_indent(yaml_emitter_t *, int)\0"))
                .as_ptr(),
        );
    }
    'c_13020: {
        if !emitter.is_null() {} else {
            __assert_fail(
                b"emitter\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                538 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"void yaml_emitter_set_indent(yaml_emitter_t *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    (*emitter)
        .best_indent = if (1 as libc::c_int) < indent && indent < 10 as libc::c_int {
        indent
    } else {
        2 as libc::c_int
    };
}
pub unsafe extern "C" fn yaml_emitter_set_width(
    mut emitter: *mut yaml_emitter_t,
    mut width: libc::c_int,
) {
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            550 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"void yaml_emitter_set_width(yaml_emitter_t *, int)\0"))
                .as_ptr(),
        );
    }
    'c_13073: {
        if !emitter.is_null() {} else {
            __assert_fail(
                b"emitter\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                550 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"void yaml_emitter_set_width(yaml_emitter_t *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    (*emitter)
        .best_width = if width >= 0 as libc::c_int {
        width
    } else {
        -(1 as libc::c_int)
    };
}
pub unsafe extern "C" fn yaml_emitter_set_unicode(
    mut emitter: *mut yaml_emitter_t,
    mut unicode: libc::c_int,
) {
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            562 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"void yaml_emitter_set_unicode(yaml_emitter_t *, int)\0"))
                .as_ptr(),
        );
    }
    'c_13121: {
        if !emitter.is_null() {} else {
            __assert_fail(
                b"emitter\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                562 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"void yaml_emitter_set_unicode(yaml_emitter_t *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    (*emitter).unicode = (unicode != 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn yaml_emitter_set_break(
    mut emitter: *mut yaml_emitter_t,
    mut line_break: yaml_break_t,
) {
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            574 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"void yaml_emitter_set_break(yaml_emitter_t *, yaml_break_t)\0"))
                .as_ptr(),
        );
    }
    'c_13166: {
        if !emitter.is_null() {} else {
            __assert_fail(
                b"emitter\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                574 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"void yaml_emitter_set_break(yaml_emitter_t *, yaml_break_t)\0"))
                    .as_ptr(),
            );
        }
    };
    (*emitter).line_break = line_break;
}
pub unsafe extern "C" fn yaml_token_delete(mut token: *mut yaml_token_t) {
    if !token.is_null() {} else {
        __assert_fail(
            b"token\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            586 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void yaml_token_delete(yaml_token_t *)\0"))
                .as_ptr(),
        );
    }
    'c_2518: {
        if !token.is_null() {} else {
            __assert_fail(
                b"token\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                586 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void yaml_token_delete(yaml_token_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    match (*token).type_0 as libc::c_uint {
        4 => {
            yaml_free((*token).data.tag_directive.handle as *mut libc::c_void);
            yaml_free((*token).data.tag_directive.prefix as *mut libc::c_void);
        }
        18 => {
            yaml_free((*token).data.alias.value as *mut libc::c_void);
        }
        19 => {
            yaml_free((*token).data.anchor.value as *mut libc::c_void);
        }
        20 => {
            yaml_free((*token).data.tag.handle as *mut libc::c_void);
            yaml_free((*token).data.tag.suffix as *mut libc::c_void);
        }
        21 => {
            yaml_free((*token).data.scalar.value as *mut libc::c_void);
        }
        _ => {}
    }
    memset(
        token as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_token_t>() as libc::c_ulong,
    );
}
unsafe extern "C" fn yaml_check_utf8(
    mut start: *const yaml_char_t,
    mut length: size_t,
) -> libc::c_int {
    let mut end: *const yaml_char_t = start.offset(length as isize);
    let mut pointer: *const yaml_char_t = start;
    while pointer < end {
        let mut octet: libc::c_uchar = 0;
        let mut width: libc::c_uint = 0;
        let mut value: libc::c_uint = 0;
        let mut k: size_t = 0;
        octet = *pointer.offset(0 as libc::c_int as isize);
        width = (if octet as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
            1 as libc::c_int
        } else if octet as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int {
            2 as libc::c_int
        } else if octet as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int {
            3 as libc::c_int
        } else if octet as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int {
            4 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint;
        value = (if octet as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
            octet as libc::c_int & 0x7f as libc::c_int
        } else if octet as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int {
            octet as libc::c_int & 0x1f as libc::c_int
        } else if octet as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int {
            octet as libc::c_int & 0xf as libc::c_int
        } else if octet as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int {
            octet as libc::c_int & 0x7 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint;
        if width == 0 {
            return 0 as libc::c_int;
        }
        if pointer.offset(width as isize) > end {
            return 0 as libc::c_int;
        }
        k = 1 as libc::c_int as size_t;
        while k < width as libc::c_ulong {
            octet = *pointer.offset(k as isize);
            if octet as libc::c_int & 0xc0 as libc::c_int != 0x80 as libc::c_int {
                return 0 as libc::c_int;
            }
            value = (value << 6 as libc::c_int)
                .wrapping_add(
                    (octet as libc::c_int & 0x3f as libc::c_int) as libc::c_uint,
                );
            k = k.wrapping_add(1);
            k;
        }
        if !(width == 1 as libc::c_int as libc::c_uint
            || width == 2 as libc::c_int as libc::c_uint
                && value >= 0x80 as libc::c_int as libc::c_uint
            || width == 3 as libc::c_int as libc::c_uint
                && value >= 0x800 as libc::c_int as libc::c_uint
            || width == 4 as libc::c_int as libc::c_uint
                && value >= 0x10000 as libc::c_int as libc::c_uint)
        {
            return 0 as libc::c_int;
        }
        pointer = pointer.offset(width as isize);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn yaml_stream_start_event_initialize(
    mut event: *mut yaml_event_t,
    mut encoding: yaml_encoding_t,
) -> libc::c_int {
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            674 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int yaml_stream_start_event_initialize(yaml_event_t *, yaml_encoding_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2729: {
        if !event.is_null() {} else {
            __assert_fail(
                b"event\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                674 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"int yaml_stream_start_event_initialize(yaml_event_t *, yaml_encoding_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    memset(
        event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_STREAM_START_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    (*event).data.stream_start.encoding = encoding;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn yaml_stream_end_event_initialize(
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            690 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"int yaml_stream_end_event_initialize(yaml_event_t *)\0"))
                .as_ptr(),
        );
    }
    'c_2829: {
        if !event.is_null() {} else {
            __assert_fail(
                b"event\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                690 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"int yaml_stream_end_event_initialize(yaml_event_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_STREAM_END_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn yaml_document_start_event_initialize(
    mut event: *mut yaml_event_t,
    mut version_directive: *mut yaml_version_directive_t,
    mut tag_directives_start: *mut yaml_tag_directive_t,
    mut tag_directives_end: *mut yaml_tag_directive_t,
    mut implicit: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut context: C2RustUnnamed_17 = C2RustUnnamed_17 {
        error: YAML_NO_ERROR,
    };
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut version_directive_copy: *mut yaml_version_directive_t = 0
        as *mut yaml_version_directive_t;
    let mut tag_directives_copy: C2RustUnnamed_16 = {
        let mut init = C2RustUnnamed_16 {
            start: 0 as *mut yaml_tag_directive_t,
            end: 0 as *mut yaml_tag_directive_t,
            top: 0 as *mut yaml_tag_directive_t,
        };
        init
    };
    let mut value: yaml_tag_directive_t = {
        let mut init = yaml_tag_directive_s {
            handle: 0 as *mut yaml_char_t,
            prefix: 0 as *mut yaml_char_t,
        };
        init
    };
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            720 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 138],
                &[libc::c_char; 138],
            >(
                b"int yaml_document_start_event_initialize(yaml_event_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3988: {
        if !event.is_null() {} else {
            __assert_fail(
                b"event\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                720 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 138],
                    &[libc::c_char; 138],
                >(
                    b"int yaml_document_start_event_initialize(yaml_event_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !tag_directives_start.is_null() && !tag_directives_end.is_null()
        || tag_directives_start == tag_directives_end
    {} else {
        __assert_fail(
            b"(tag_directives_start && tag_directives_end) || (tag_directives_start == tag_directives_end)\0"
                as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            722 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 138],
                &[libc::c_char; 138],
            >(
                b"int yaml_document_start_event_initialize(yaml_event_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3933: {
        if !tag_directives_start.is_null() && !tag_directives_end.is_null()
            || tag_directives_start == tag_directives_end
        {} else {
            __assert_fail(
                b"(tag_directives_start && tag_directives_end) || (tag_directives_start == tag_directives_end)\0"
                    as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                722 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 138],
                    &[libc::c_char; 138],
                >(
                    b"int yaml_document_start_event_initialize(yaml_event_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !version_directive.is_null() {
        version_directive_copy = yaml_malloc(
            ::std::mem::size_of::<yaml_version_directive_t>() as libc::c_ulong,
        ) as *mut yaml_version_directive_t;
        if version_directive_copy.is_null() {
            current_block = 3791942351067618062;
        } else {
            (*version_directive_copy).major = (*version_directive).major;
            (*version_directive_copy).minor = (*version_directive).minor;
            current_block = 11006700562992250127;
        }
    } else {
        current_block = 11006700562992250127;
    }
    match current_block {
        11006700562992250127 => {
            if tag_directives_start != tag_directives_end {
                let mut tag_directive: *mut yaml_tag_directive_t = 0
                    as *mut yaml_tag_directive_t;
                tag_directives_copy
                    .start = yaml_malloc(
                    (16 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<yaml_tag_directive_t>()
                                as libc::c_ulong,
                        ),
                ) as *mut yaml_tag_directive_t;
                if if !(tag_directives_copy.start).is_null() {
                    tag_directives_copy.top = tag_directives_copy.start;
                    tag_directives_copy
                        .end = (tag_directives_copy.start)
                        .offset(16 as libc::c_int as isize);
                    1 as libc::c_int
                } else {
                    context.error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } == 0
                {
                    current_block = 3791942351067618062;
                } else {
                    tag_directive = tag_directives_start;
                    loop {
                        if !(tag_directive != tag_directives_end) {
                            current_block = 4808432441040389987;
                            break;
                        }
                        if !((*tag_directive).handle).is_null() {} else {
                            __assert_fail(
                                b"tag_directive->handle\0" as *const u8
                                    as *const libc::c_char,
                                b"api.c\0" as *const u8 as *const libc::c_char,
                                738 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 138],
                                    &[libc::c_char; 138],
                                >(
                                    b"int yaml_document_start_event_initialize(yaml_event_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_3777: {
                            if !((*tag_directive).handle).is_null() {} else {
                                __assert_fail(
                                    b"tag_directive->handle\0" as *const u8
                                        as *const libc::c_char,
                                    b"api.c\0" as *const u8 as *const libc::c_char,
                                    738 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 138],
                                        &[libc::c_char; 138],
                                    >(
                                        b"int yaml_document_start_event_initialize(yaml_event_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        if !((*tag_directive).prefix).is_null() {} else {
                            __assert_fail(
                                b"tag_directive->prefix\0" as *const u8
                                    as *const libc::c_char,
                                b"api.c\0" as *const u8 as *const libc::c_char,
                                739 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 138],
                                    &[libc::c_char; 138],
                                >(
                                    b"int yaml_document_start_event_initialize(yaml_event_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_3739: {
                            if !((*tag_directive).prefix).is_null() {} else {
                                __assert_fail(
                                    b"tag_directive->prefix\0" as *const u8
                                        as *const libc::c_char,
                                    b"api.c\0" as *const u8 as *const libc::c_char,
                                    739 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 138],
                                        &[libc::c_char; 138],
                                    >(
                                        b"int yaml_document_start_event_initialize(yaml_event_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        if yaml_check_utf8(
                            (*tag_directive).handle,
                            strlen((*tag_directive).handle as *mut libc::c_char),
                        ) == 0
                        {
                            current_block = 3791942351067618062;
                            break;
                        }
                        if yaml_check_utf8(
                            (*tag_directive).prefix,
                            strlen((*tag_directive).prefix as *mut libc::c_char),
                        ) == 0
                        {
                            current_block = 3791942351067618062;
                            break;
                        }
                        value.handle = yaml_strdup((*tag_directive).handle);
                        value.prefix = yaml_strdup((*tag_directive).prefix);
                        if (value.handle).is_null() || (value.prefix).is_null() {
                            current_block = 3791942351067618062;
                            break;
                        }
                        if if tag_directives_copy.top != tag_directives_copy.end
                            || yaml_stack_extend(
                                &mut tag_directives_copy.start
                                    as *mut *mut yaml_tag_directive_t as *mut *mut libc::c_void,
                                &mut tag_directives_copy.top
                                    as *mut *mut yaml_tag_directive_t as *mut *mut libc::c_void,
                                &mut tag_directives_copy.end
                                    as *mut *mut yaml_tag_directive_t as *mut *mut libc::c_void,
                            ) != 0
                        {
                            let fresh2 = tag_directives_copy.top;
                            tag_directives_copy
                                .top = (tag_directives_copy.top).offset(1);
                            *fresh2 = value;
                            1 as libc::c_int
                        } else {
                            context.error = YAML_MEMORY_ERROR;
                            0 as libc::c_int
                        } == 0
                        {
                            current_block = 3791942351067618062;
                            break;
                        }
                        value.handle = 0 as *mut yaml_char_t;
                        value.prefix = 0 as *mut yaml_char_t;
                        tag_directive = tag_directive.offset(1);
                        tag_directive;
                    }
                }
            } else {
                current_block = 4808432441040389987;
            }
            match current_block {
                3791942351067618062 => {}
                _ => {
                    memset(
                        event as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
                    );
                    (*event).type_0 = YAML_DOCUMENT_START_EVENT;
                    (*event).start_mark = mark;
                    (*event).end_mark = mark;
                    (*event)
                        .data
                        .document_start
                        .version_directive = version_directive_copy;
                    (*event)
                        .data
                        .document_start
                        .tag_directives
                        .start = tag_directives_copy.start;
                    (*event)
                        .data
                        .document_start
                        .tag_directives
                        .end = tag_directives_copy.top;
                    (*event).data.document_start.implicit = implicit;
                    return 1 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    yaml_free(version_directive_copy as *mut libc::c_void);
    while !(tag_directives_copy.start == tag_directives_copy.top) {
        tag_directives_copy.top = (tag_directives_copy.top).offset(-1);
        let mut value_0: yaml_tag_directive_t = *tag_directives_copy.top;
        yaml_free(value_0.handle as *mut libc::c_void);
        yaml_free(value_0.prefix as *mut libc::c_void);
    }
    yaml_free(tag_directives_copy.start as *mut libc::c_void);
    tag_directives_copy.end = 0 as *mut yaml_tag_directive_t;
    tag_directives_copy.top = tag_directives_copy.end;
    tag_directives_copy.start = tag_directives_copy.top;
    yaml_free(value.handle as *mut libc::c_void);
    yaml_free(value.prefix as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn yaml_document_end_event_initialize(
    mut event: *mut yaml_event_t,
    mut implicit: libc::c_int,
) -> libc::c_int {
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            785 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"int yaml_document_end_event_initialize(yaml_event_t *, int)\0"))
                .as_ptr(),
        );
    }
    'c_4100: {
        if !event.is_null() {} else {
            __assert_fail(
                b"event\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                785 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"int yaml_document_end_event_initialize(yaml_event_t *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_DOCUMENT_END_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    (*event).data.document_end.implicit = implicit;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn yaml_alias_event_initialize(
    mut event: *mut yaml_event_t,
    mut anchor: *const yaml_char_t,
) -> libc::c_int {
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut anchor_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            802 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"int yaml_alias_event_initialize(yaml_event_t *, const yaml_char_t *)\0"))
                .as_ptr(),
        );
    }
    'c_4284: {
        if !event.is_null() {} else {
            __assert_fail(
                b"event\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                802 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"int yaml_alias_event_initialize(yaml_event_t *, const yaml_char_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !anchor.is_null() {} else {
        __assert_fail(
            b"anchor\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            803 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"int yaml_alias_event_initialize(yaml_event_t *, const yaml_char_t *)\0"))
                .as_ptr(),
        );
    }
    'c_4250: {
        if !anchor.is_null() {} else {
            __assert_fail(
                b"anchor\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                803 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"int yaml_alias_event_initialize(yaml_event_t *, const yaml_char_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if yaml_check_utf8(anchor, strlen(anchor as *mut libc::c_char)) == 0 {
        return 0 as libc::c_int;
    }
    anchor_copy = yaml_strdup(anchor);
    if anchor_copy.is_null() {
        return 0 as libc::c_int;
    }
    memset(
        event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_ALIAS_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    (*event).data.alias.anchor = anchor_copy;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn yaml_scalar_event_initialize(
    mut event: *mut yaml_event_t,
    mut anchor: *const yaml_char_t,
    mut tag: *const yaml_char_t,
    mut value: *const yaml_char_t,
    mut length: libc::c_int,
    mut plain_implicit: libc::c_int,
    mut quoted_implicit: libc::c_int,
    mut style: yaml_scalar_style_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut anchor_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut tag_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut value_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            832 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 148],
                &[libc::c_char; 148],
            >(
                b"int yaml_scalar_event_initialize(yaml_event_t *, const yaml_char_t *, const yaml_char_t *, const yaml_char_t *, int, int, int, yaml_scalar_style_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4684: {
        if !event.is_null() {} else {
            __assert_fail(
                b"event\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                832 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 148],
                    &[libc::c_char; 148],
                >(
                    b"int yaml_scalar_event_initialize(yaml_event_t *, const yaml_char_t *, const yaml_char_t *, const yaml_char_t *, int, int, int, yaml_scalar_style_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !value.is_null() {} else {
        __assert_fail(
            b"value\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            833 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 148],
                &[libc::c_char; 148],
            >(
                b"int yaml_scalar_event_initialize(yaml_event_t *, const yaml_char_t *, const yaml_char_t *, const yaml_char_t *, int, int, int, yaml_scalar_style_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4651: {
        if !value.is_null() {} else {
            __assert_fail(
                b"value\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                833 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 148],
                    &[libc::c_char; 148],
                >(
                    b"int yaml_scalar_event_initialize(yaml_event_t *, const yaml_char_t *, const yaml_char_t *, const yaml_char_t *, int, int, int, yaml_scalar_style_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !anchor.is_null() {
        if yaml_check_utf8(anchor, strlen(anchor as *mut libc::c_char)) == 0 {
            current_block = 2872089151430502379;
        } else {
            anchor_copy = yaml_strdup(anchor);
            if anchor_copy.is_null() {
                current_block = 2872089151430502379;
            } else {
                current_block = 2473556513754201174;
            }
        }
    } else {
        current_block = 2473556513754201174;
    }
    match current_block {
        2473556513754201174 => {
            if !tag.is_null() {
                if yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)) == 0 {
                    current_block = 2872089151430502379;
                } else {
                    tag_copy = yaml_strdup(tag);
                    if tag_copy.is_null() {
                        current_block = 2872089151430502379;
                    } else {
                        current_block = 2868539653012386629;
                    }
                }
            } else {
                current_block = 2868539653012386629;
            }
            match current_block {
                2872089151430502379 => {}
                _ => {
                    if length < 0 as libc::c_int {
                        length = strlen(value as *mut libc::c_char) as libc::c_int;
                    }
                    if !(yaml_check_utf8(value, length as size_t) == 0) {
                        value_copy = yaml_malloc((length + 1 as libc::c_int) as size_t)
                            as *mut yaml_char_t;
                        if !value_copy.is_null() {
                            memcpy(
                                value_copy as *mut libc::c_void,
                                value as *const libc::c_void,
                                length as libc::c_ulong,
                            );
                            *value_copy
                                .offset(length as isize) = '\0' as i32 as yaml_char_t;
                            memset(
                                event as *mut libc::c_void,
                                0 as libc::c_int,
                                ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
                            );
                            (*event).type_0 = YAML_SCALAR_EVENT;
                            (*event).start_mark = mark;
                            (*event).end_mark = mark;
                            (*event).data.scalar.anchor = anchor_copy;
                            (*event).data.scalar.tag = tag_copy;
                            (*event).data.scalar.value = value_copy;
                            (*event).data.scalar.length = length as size_t;
                            (*event).data.scalar.plain_implicit = plain_implicit;
                            (*event).data.scalar.quoted_implicit = quoted_implicit;
                            (*event).data.scalar.style = style;
                            return 1 as libc::c_int;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    yaml_free(anchor_copy as *mut libc::c_void);
    yaml_free(tag_copy as *mut libc::c_void);
    yaml_free(value_copy as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn yaml_sequence_start_event_initialize(
    mut event: *mut yaml_event_t,
    mut anchor: *const yaml_char_t,
    mut tag: *const yaml_char_t,
    mut implicit: libc::c_int,
    mut style: yaml_sequence_style_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut anchor_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut tag_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            883 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 127],
                &[libc::c_char; 127],
            >(
                b"int yaml_sequence_start_event_initialize(yaml_event_t *, const yaml_char_t *, const yaml_char_t *, int, yaml_sequence_style_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4928: {
        if !event.is_null() {} else {
            __assert_fail(
                b"event\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                883 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 127],
                    &[libc::c_char; 127],
                >(
                    b"int yaml_sequence_start_event_initialize(yaml_event_t *, const yaml_char_t *, const yaml_char_t *, int, yaml_sequence_style_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !anchor.is_null() {
        if yaml_check_utf8(anchor, strlen(anchor as *mut libc::c_char)) == 0 {
            current_block = 10097870881477334335;
        } else {
            anchor_copy = yaml_strdup(anchor);
            if anchor_copy.is_null() {
                current_block = 10097870881477334335;
            } else {
                current_block = 15427931788582360902;
            }
        }
    } else {
        current_block = 15427931788582360902;
    }
    match current_block {
        15427931788582360902 => {
            if !tag.is_null() {
                if yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)) == 0 {
                    current_block = 10097870881477334335;
                } else {
                    tag_copy = yaml_strdup(tag);
                    if tag_copy.is_null() {
                        current_block = 10097870881477334335;
                    } else {
                        current_block = 1394248824506584008;
                    }
                }
            } else {
                current_block = 1394248824506584008;
            }
            match current_block {
                10097870881477334335 => {}
                _ => {
                    memset(
                        event as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
                    );
                    (*event).type_0 = YAML_SEQUENCE_START_EVENT;
                    (*event).start_mark = mark;
                    (*event).end_mark = mark;
                    (*event).data.sequence_start.anchor = anchor_copy;
                    (*event).data.sequence_start.tag = tag_copy;
                    (*event).data.sequence_start.implicit = implicit;
                    (*event).data.sequence_start.style = style;
                    return 1 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    yaml_free(anchor_copy as *mut libc::c_void);
    yaml_free(tag_copy as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn yaml_sequence_end_event_initialize(
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            918 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"int yaml_sequence_end_event_initialize(yaml_event_t *)\0"))
                .as_ptr(),
        );
    }
    'c_5027: {
        if !event.is_null() {} else {
            __assert_fail(
                b"event\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                918 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"int yaml_sequence_end_event_initialize(yaml_event_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_SEQUENCE_END_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn yaml_mapping_start_event_initialize(
    mut event: *mut yaml_event_t,
    mut anchor: *const yaml_char_t,
    mut tag: *const yaml_char_t,
    mut implicit: libc::c_int,
    mut style: yaml_mapping_style_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut anchor_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut tag_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            938 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 125],
                &[libc::c_char; 125],
            >(
                b"int yaml_mapping_start_event_initialize(yaml_event_t *, const yaml_char_t *, const yaml_char_t *, int, yaml_mapping_style_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5272: {
        if !event.is_null() {} else {
            __assert_fail(
                b"event\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                938 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 125],
                    &[libc::c_char; 125],
                >(
                    b"int yaml_mapping_start_event_initialize(yaml_event_t *, const yaml_char_t *, const yaml_char_t *, int, yaml_mapping_style_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !anchor.is_null() {
        if yaml_check_utf8(anchor, strlen(anchor as *mut libc::c_char)) == 0 {
            current_block = 13574469675791580290;
        } else {
            anchor_copy = yaml_strdup(anchor);
            if anchor_copy.is_null() {
                current_block = 13574469675791580290;
            } else {
                current_block = 15427931788582360902;
            }
        }
    } else {
        current_block = 15427931788582360902;
    }
    match current_block {
        15427931788582360902 => {
            if !tag.is_null() {
                if yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)) == 0 {
                    current_block = 13574469675791580290;
                } else {
                    tag_copy = yaml_strdup(tag);
                    if tag_copy.is_null() {
                        current_block = 13574469675791580290;
                    } else {
                        current_block = 1394248824506584008;
                    }
                }
            } else {
                current_block = 1394248824506584008;
            }
            match current_block {
                13574469675791580290 => {}
                _ => {
                    memset(
                        event as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
                    );
                    (*event).type_0 = YAML_MAPPING_START_EVENT;
                    (*event).start_mark = mark;
                    (*event).end_mark = mark;
                    (*event).data.mapping_start.anchor = anchor_copy;
                    (*event).data.mapping_start.tag = tag_copy;
                    (*event).data.mapping_start.implicit = implicit;
                    (*event).data.mapping_start.style = style;
                    return 1 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    yaml_free(anchor_copy as *mut libc::c_void);
    yaml_free(tag_copy as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn yaml_mapping_end_event_initialize(
    mut event: *mut yaml_event_t,
) -> libc::c_int {
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            973 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"int yaml_mapping_end_event_initialize(yaml_event_t *)\0"))
                .as_ptr(),
        );
    }
    'c_5371: {
        if !event.is_null() {} else {
            __assert_fail(
                b"event\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                973 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"int yaml_mapping_end_event_initialize(yaml_event_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_0 = YAML_MAPPING_END_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn yaml_event_delete(mut event: *mut yaml_event_t) {
    let mut tag_directive: *mut yaml_tag_directive_t = 0 as *mut yaml_tag_directive_t;
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            989 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void yaml_event_delete(yaml_event_t *)\0"))
                .as_ptr(),
        );
    }
    'c_5607: {
        if !event.is_null() {} else {
            __assert_fail(
                b"event\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                989 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void yaml_event_delete(yaml_event_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    match (*event).type_0 as libc::c_uint {
        3 => {
            yaml_free(
                (*event).data.document_start.version_directive as *mut libc::c_void,
            );
            tag_directive = (*event).data.document_start.tag_directives.start;
            while tag_directive != (*event).data.document_start.tag_directives.end {
                yaml_free((*tag_directive).handle as *mut libc::c_void);
                yaml_free((*tag_directive).prefix as *mut libc::c_void);
                tag_directive = tag_directive.offset(1);
                tag_directive;
            }
            yaml_free(
                (*event).data.document_start.tag_directives.start as *mut libc::c_void,
            );
        }
        5 => {
            yaml_free((*event).data.alias.anchor as *mut libc::c_void);
        }
        6 => {
            yaml_free((*event).data.scalar.anchor as *mut libc::c_void);
            yaml_free((*event).data.scalar.tag as *mut libc::c_void);
            yaml_free((*event).data.scalar.value as *mut libc::c_void);
        }
        7 => {
            yaml_free((*event).data.sequence_start.anchor as *mut libc::c_void);
            yaml_free((*event).data.sequence_start.tag as *mut libc::c_void);
        }
        9 => {
            yaml_free((*event).data.mapping_start.anchor as *mut libc::c_void);
            yaml_free((*event).data.mapping_start.tag as *mut libc::c_void);
        }
        _ => {}
    }
    memset(
        event as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_event_t>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn yaml_document_initialize(
    mut document: *mut yaml_document_t,
    mut version_directive: *mut yaml_version_directive_t,
    mut tag_directives_start: *mut yaml_tag_directive_t,
    mut tag_directives_end: *mut yaml_tag_directive_t,
    mut start_implicit: libc::c_int,
    mut end_implicit: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut context: C2RustUnnamed_28 = C2RustUnnamed_28 {
        error: YAML_NO_ERROR,
    };
    let mut nodes: C2RustUnnamed_27 = {
        let mut init = C2RustUnnamed_27 {
            start: 0 as *mut yaml_node_t,
            end: 0 as *mut yaml_node_t,
            top: 0 as *mut yaml_node_t,
        };
        init
    };
    let mut version_directive_copy: *mut yaml_version_directive_t = 0
        as *mut yaml_version_directive_t;
    let mut tag_directives_copy: C2RustUnnamed_26 = {
        let mut init = C2RustUnnamed_26 {
            start: 0 as *mut yaml_tag_directive_t,
            end: 0 as *mut yaml_tag_directive_t,
            top: 0 as *mut yaml_tag_directive_t,
        };
        init
    };
    let mut value: yaml_tag_directive_t = {
        let mut init = yaml_tag_directive_s {
            handle: 0 as *mut yaml_char_t,
            prefix: 0 as *mut yaml_char_t,
        };
        init
    };
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    if !document.is_null() {} else {
        __assert_fail(
            b"document\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1059 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 134],
                &[libc::c_char; 134],
            >(
                b"int yaml_document_initialize(yaml_document_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_6505: {
        if !document.is_null() {} else {
            __assert_fail(
                b"document\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                1059 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 134],
                    &[libc::c_char; 134],
                >(
                    b"int yaml_document_initialize(yaml_document_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !tag_directives_start.is_null() && !tag_directives_end.is_null()
        || tag_directives_start == tag_directives_end
    {} else {
        __assert_fail(
            b"(tag_directives_start && tag_directives_end) || (tag_directives_start == tag_directives_end)\0"
                as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1061 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 134],
                &[libc::c_char; 134],
            >(
                b"int yaml_document_initialize(yaml_document_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_6451: {
        if !tag_directives_start.is_null() && !tag_directives_end.is_null()
            || tag_directives_start == tag_directives_end
        {} else {
            __assert_fail(
                b"(tag_directives_start && tag_directives_end) || (tag_directives_start == tag_directives_end)\0"
                    as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                1061 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 134],
                    &[libc::c_char; 134],
                >(
                    b"int yaml_document_initialize(yaml_document_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    nodes
        .start = yaml_malloc(
        (16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<yaml_node_t>() as libc::c_ulong),
    ) as *mut yaml_node_t;
    if !(if !(nodes.start).is_null() {
        nodes.top = nodes.start;
        nodes.end = (nodes.start).offset(16 as libc::c_int as isize);
        1 as libc::c_int
    } else {
        context.error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0)
    {
        if !version_directive.is_null() {
            version_directive_copy = yaml_malloc(
                ::std::mem::size_of::<yaml_version_directive_t>() as libc::c_ulong,
            ) as *mut yaml_version_directive_t;
            if version_directive_copy.is_null() {
                current_block = 14085896879340676071;
            } else {
                (*version_directive_copy).major = (*version_directive).major;
                (*version_directive_copy).minor = (*version_directive).minor;
                current_block = 8515828400728868193;
            }
        } else {
            current_block = 8515828400728868193;
        }
        match current_block {
            14085896879340676071 => {}
            _ => {
                if tag_directives_start != tag_directives_end {
                    let mut tag_directive: *mut yaml_tag_directive_t = 0
                        as *mut yaml_tag_directive_t;
                    tag_directives_copy
                        .start = yaml_malloc(
                        (16 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<yaml_tag_directive_t>()
                                    as libc::c_ulong,
                            ),
                    ) as *mut yaml_tag_directive_t;
                    if if !(tag_directives_copy.start).is_null() {
                        tag_directives_copy.top = tag_directives_copy.start;
                        tag_directives_copy
                            .end = (tag_directives_copy.start)
                            .offset(16 as libc::c_int as isize);
                        1 as libc::c_int
                    } else {
                        context.error = YAML_MEMORY_ERROR;
                        0 as libc::c_int
                    } == 0
                    {
                        current_block = 14085896879340676071;
                    } else {
                        tag_directive = tag_directives_start;
                        loop {
                            if !(tag_directive != tag_directives_end) {
                                current_block = 15089075282327824602;
                                break;
                            }
                            if !((*tag_directive).handle).is_null() {} else {
                                __assert_fail(
                                    b"tag_directive->handle\0" as *const u8
                                        as *const libc::c_char,
                                    b"api.c\0" as *const u8 as *const libc::c_char,
                                    1079 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 134],
                                        &[libc::c_char; 134],
                                    >(
                                        b"int yaml_document_initialize(yaml_document_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_6258: {
                                if !((*tag_directive).handle).is_null() {} else {
                                    __assert_fail(
                                        b"tag_directive->handle\0" as *const u8
                                            as *const libc::c_char,
                                        b"api.c\0" as *const u8 as *const libc::c_char,
                                        1079 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 134],
                                            &[libc::c_char; 134],
                                        >(
                                            b"int yaml_document_initialize(yaml_document_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int, int)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            if !((*tag_directive).prefix).is_null() {} else {
                                __assert_fail(
                                    b"tag_directive->prefix\0" as *const u8
                                        as *const libc::c_char,
                                    b"api.c\0" as *const u8 as *const libc::c_char,
                                    1080 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 134],
                                        &[libc::c_char; 134],
                                    >(
                                        b"int yaml_document_initialize(yaml_document_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_6221: {
                                if !((*tag_directive).prefix).is_null() {} else {
                                    __assert_fail(
                                        b"tag_directive->prefix\0" as *const u8
                                            as *const libc::c_char,
                                        b"api.c\0" as *const u8 as *const libc::c_char,
                                        1080 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 134],
                                            &[libc::c_char; 134],
                                        >(
                                            b"int yaml_document_initialize(yaml_document_t *, yaml_version_directive_t *, yaml_tag_directive_t *, yaml_tag_directive_t *, int, int)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            if yaml_check_utf8(
                                (*tag_directive).handle,
                                strlen((*tag_directive).handle as *mut libc::c_char),
                            ) == 0
                            {
                                current_block = 14085896879340676071;
                                break;
                            }
                            if yaml_check_utf8(
                                (*tag_directive).prefix,
                                strlen((*tag_directive).prefix as *mut libc::c_char),
                            ) == 0
                            {
                                current_block = 14085896879340676071;
                                break;
                            }
                            value.handle = yaml_strdup((*tag_directive).handle);
                            value.prefix = yaml_strdup((*tag_directive).prefix);
                            if (value.handle).is_null() || (value.prefix).is_null() {
                                current_block = 14085896879340676071;
                                break;
                            }
                            if if tag_directives_copy.top != tag_directives_copy.end
                                || yaml_stack_extend(
                                    &mut tag_directives_copy.start
                                        as *mut *mut yaml_tag_directive_t as *mut *mut libc::c_void,
                                    &mut tag_directives_copy.top
                                        as *mut *mut yaml_tag_directive_t as *mut *mut libc::c_void,
                                    &mut tag_directives_copy.end
                                        as *mut *mut yaml_tag_directive_t as *mut *mut libc::c_void,
                                ) != 0
                            {
                                let fresh3 = tag_directives_copy.top;
                                tag_directives_copy
                                    .top = (tag_directives_copy.top).offset(1);
                                *fresh3 = value;
                                1 as libc::c_int
                            } else {
                                context.error = YAML_MEMORY_ERROR;
                                0 as libc::c_int
                            } == 0
                            {
                                current_block = 14085896879340676071;
                                break;
                            }
                            value.handle = 0 as *mut yaml_char_t;
                            value.prefix = 0 as *mut yaml_char_t;
                            tag_directive = tag_directive.offset(1);
                            tag_directive;
                        }
                    }
                } else {
                    current_block = 15089075282327824602;
                }
                match current_block {
                    14085896879340676071 => {}
                    _ => {
                        memset(
                            document as *mut libc::c_void,
                            0 as libc::c_int,
                            ::std::mem::size_of::<yaml_document_t>() as libc::c_ulong,
                        );
                        (*document).nodes.start = nodes.start;
                        (*document).nodes.end = nodes.end;
                        (*document).nodes.top = nodes.start;
                        (*document).version_directive = version_directive_copy;
                        (*document).tag_directives.start = tag_directives_copy.start;
                        (*document).tag_directives.end = tag_directives_copy.top;
                        (*document).start_implicit = start_implicit;
                        (*document).end_implicit = end_implicit;
                        (*document).start_mark = mark;
                        (*document).end_mark = mark;
                        return 1 as libc::c_int;
                    }
                }
            }
        }
    }
    yaml_free(nodes.start as *mut libc::c_void);
    nodes.end = 0 as *mut yaml_node_t;
    nodes.top = nodes.end;
    nodes.start = nodes.top;
    yaml_free(version_directive_copy as *mut libc::c_void);
    while !(tag_directives_copy.start == tag_directives_copy.top) {
        tag_directives_copy.top = (tag_directives_copy.top).offset(-1);
        let mut value_0: yaml_tag_directive_t = *tag_directives_copy.top;
        yaml_free(value_0.handle as *mut libc::c_void);
        yaml_free(value_0.prefix as *mut libc::c_void);
    }
    yaml_free(tag_directives_copy.start as *mut libc::c_void);
    tag_directives_copy.end = 0 as *mut yaml_tag_directive_t;
    tag_directives_copy.top = tag_directives_copy.end;
    tag_directives_copy.start = tag_directives_copy.top;
    yaml_free(value.handle as *mut libc::c_void);
    yaml_free(value.prefix as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn yaml_document_delete(mut document: *mut yaml_document_t) {
    let mut tag_directive: *mut yaml_tag_directive_t = 0 as *mut yaml_tag_directive_t;
    if !document.is_null() {} else {
        __assert_fail(
            b"document\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1127 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"void yaml_document_delete(yaml_document_t *)\0"))
                .as_ptr(),
        );
    }
    'c_6820: {
        if !document.is_null() {} else {
            __assert_fail(
                b"document\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                1127 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"void yaml_document_delete(yaml_document_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    while !((*document).nodes.start == (*document).nodes.top) {
        (*document).nodes.top = ((*document).nodes.top).offset(-1);
        let mut node: yaml_node_t = *(*document).nodes.top;
        yaml_free(node.tag as *mut libc::c_void);
        match node.type_0 as libc::c_uint {
            1 => {
                yaml_free(node.data.scalar.value as *mut libc::c_void);
            }
            2 => {
                yaml_free(node.data.sequence.items.start as *mut libc::c_void);
                node.data.sequence.items.end = 0 as *mut yaml_node_item_t;
                node.data.sequence.items.top = node.data.sequence.items.end;
                node.data.sequence.items.start = node.data.sequence.items.top;
            }
            3 => {
                yaml_free(node.data.mapping.pairs.start as *mut libc::c_void);
                node.data.mapping.pairs.end = 0 as *mut yaml_node_pair_t;
                node.data.mapping.pairs.top = node.data.mapping.pairs.end;
                node.data.mapping.pairs.start = node.data.mapping.pairs.top;
            }
            _ => {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"api.c\0" as *const u8 as *const libc::c_char,
                    1143 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 45],
                        &[libc::c_char; 45],
                    >(b"void yaml_document_delete(yaml_document_t *)\0"))
                        .as_ptr(),
                );
                'c_6660: {
                    __assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"api.c\0" as *const u8 as *const libc::c_char,
                        1143 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 45],
                            &[libc::c_char; 45],
                        >(b"void yaml_document_delete(yaml_document_t *)\0"))
                            .as_ptr(),
                    );
                };
            }
        }
    }
    yaml_free((*document).nodes.start as *mut libc::c_void);
    (*document).nodes.end = 0 as *mut yaml_node_t;
    (*document).nodes.top = (*document).nodes.end;
    (*document).nodes.start = (*document).nodes.top;
    yaml_free((*document).version_directive as *mut libc::c_void);
    tag_directive = (*document).tag_directives.start;
    while tag_directive != (*document).tag_directives.end {
        yaml_free((*tag_directive).handle as *mut libc::c_void);
        yaml_free((*tag_directive).prefix as *mut libc::c_void);
        tag_directive = tag_directive.offset(1);
        tag_directive;
    }
    yaml_free((*document).tag_directives.start as *mut libc::c_void);
    memset(
        document as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<yaml_document_t>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn yaml_document_get_node(
    mut document: *mut yaml_document_t,
    mut index: libc::c_int,
) -> *mut yaml_node_t {
    if !document.is_null() {} else {
        __assert_fail(
            b"document\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1167 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"yaml_node_t *yaml_document_get_node(yaml_document_t *, int)\0"))
                .as_ptr(),
        );
    }
    'c_6895: {
        if !document.is_null() {} else {
            __assert_fail(
                b"document\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                1167 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"yaml_node_t *yaml_document_get_node(yaml_document_t *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if index > 0 as libc::c_int
        && ((*document).nodes.start).offset(index as isize) <= (*document).nodes.top
    {
        return ((*document).nodes.start)
            .offset(index as isize)
            .offset(-(1 as libc::c_int as isize));
    }
    return 0 as *mut yaml_node_t;
}
pub unsafe extern "C" fn yaml_document_get_root_node(
    mut document: *mut yaml_document_t,
) -> *mut yaml_node_t {
    if !document.is_null() {} else {
        __assert_fail(
            b"document\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1182 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"yaml_node_t *yaml_document_get_root_node(yaml_document_t *)\0"))
                .as_ptr(),
        );
    }
    'c_6956: {
        if !document.is_null() {} else {
            __assert_fail(
                b"document\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                1182 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"yaml_node_t *yaml_document_get_root_node(yaml_document_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*document).nodes.top != (*document).nodes.start {
        return (*document).nodes.start;
    }
    return 0 as *mut yaml_node_t;
}
pub unsafe extern "C" fn yaml_document_add_scalar(
    mut document: *mut yaml_document_t,
    mut tag: *const yaml_char_t,
    mut value: *const yaml_char_t,
    mut length: libc::c_int,
    mut style: yaml_scalar_style_t,
) -> libc::c_int {
    let mut context: C2RustUnnamed_29 = C2RustUnnamed_29 {
        error: YAML_NO_ERROR,
    };
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut tag_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut value_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut node: yaml_node_t = yaml_node_t {
        type_0: YAML_NO_NODE,
        tag: 0 as *mut yaml_char_t,
        data: C2RustUnnamed_18 {
            scalar: C2RustUnnamed_23 {
                value: 0 as *mut yaml_char_t,
                length: 0,
                style: YAML_ANY_SCALAR_STYLE,
            },
        },
        start_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        end_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
    };
    if !document.is_null() {} else {
        __assert_fail(
            b"document\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1207 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 116],
                &[libc::c_char; 116],
            >(
                b"int yaml_document_add_scalar(yaml_document_t *, const yaml_char_t *, const yaml_char_t *, int, yaml_scalar_style_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_7354: {
        if !document.is_null() {} else {
            __assert_fail(
                b"document\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                1207 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 116],
                    &[libc::c_char; 116],
                >(
                    b"int yaml_document_add_scalar(yaml_document_t *, const yaml_char_t *, const yaml_char_t *, int, yaml_scalar_style_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !value.is_null() {} else {
        __assert_fail(
            b"value\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1208 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 116],
                &[libc::c_char; 116],
            >(
                b"int yaml_document_add_scalar(yaml_document_t *, const yaml_char_t *, const yaml_char_t *, int, yaml_scalar_style_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_7321: {
        if !value.is_null() {} else {
            __assert_fail(
                b"value\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                1208 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 116],
                    &[libc::c_char; 116],
                >(
                    b"int yaml_document_add_scalar(yaml_document_t *, const yaml_char_t *, const yaml_char_t *, int, yaml_scalar_style_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if tag.is_null() {
        tag = b"tag:yaml.org,2002:str\0" as *const u8 as *const libc::c_char
            as *mut yaml_char_t;
    }
    if !(yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)) == 0) {
        tag_copy = yaml_strdup(tag);
        if !tag_copy.is_null() {
            if length < 0 as libc::c_int {
                length = strlen(value as *mut libc::c_char) as libc::c_int;
            }
            if !(yaml_check_utf8(value, length as size_t) == 0) {
                value_copy = yaml_malloc((length + 1 as libc::c_int) as size_t)
                    as *mut yaml_char_t;
                if !value_copy.is_null() {
                    memcpy(
                        value_copy as *mut libc::c_void,
                        value as *const libc::c_void,
                        length as libc::c_ulong,
                    );
                    *value_copy.offset(length as isize) = '\0' as i32 as yaml_char_t;
                    memset(
                        &mut node as *mut yaml_node_t as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<yaml_node_t>() as libc::c_ulong,
                    );
                    node.type_0 = YAML_SCALAR_NODE;
                    node.tag = tag_copy;
                    node.start_mark = mark;
                    node.end_mark = mark;
                    node.data.scalar.value = value_copy;
                    node.data.scalar.length = length as size_t;
                    node.data.scalar.style = style;
                    if !(if (*document).nodes.top != (*document).nodes.end
                        || yaml_stack_extend(
                            &mut (*document).nodes.start as *mut *mut yaml_node_t
                                as *mut *mut libc::c_void,
                            &mut (*document).nodes.top as *mut *mut yaml_node_t
                                as *mut *mut libc::c_void,
                            &mut (*document).nodes.end as *mut *mut yaml_node_t
                                as *mut *mut libc::c_void,
                        ) != 0
                    {
                        let fresh4 = (*document).nodes.top;
                        (*document).nodes.top = ((*document).nodes.top).offset(1);
                        *fresh4 = node;
                        1 as libc::c_int
                    } else {
                        context.error = YAML_MEMORY_ERROR;
                        0 as libc::c_int
                    } == 0)
                    {
                        return ((*document).nodes.top)
                            .offset_from((*document).nodes.start) as libc::c_long
                            as libc::c_int;
                    }
                }
            }
        }
    }
    yaml_free(tag_copy as *mut libc::c_void);
    yaml_free(value_copy as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn yaml_document_add_sequence(
    mut document: *mut yaml_document_t,
    mut tag: *const yaml_char_t,
    mut style: yaml_sequence_style_t,
) -> libc::c_int {
    let mut context: C2RustUnnamed_31 = C2RustUnnamed_31 {
        error: YAML_NO_ERROR,
    };
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut tag_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut items: C2RustUnnamed_30 = {
        let mut init = C2RustUnnamed_30 {
            start: 0 as *mut yaml_node_item_t,
            end: 0 as *mut yaml_node_item_t,
            top: 0 as *mut yaml_node_item_t,
        };
        init
    };
    let mut node: yaml_node_t = yaml_node_t {
        type_0: YAML_NO_NODE,
        tag: 0 as *mut yaml_char_t,
        data: C2RustUnnamed_18 {
            scalar: C2RustUnnamed_23 {
                value: 0 as *mut yaml_char_t,
                length: 0,
                style: YAML_ANY_SCALAR_STYLE,
            },
        },
        start_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        end_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
    };
    if !document.is_null() {} else {
        __assert_fail(
            b"document\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1260 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 94],
                &[libc::c_char; 94],
            >(
                b"int yaml_document_add_sequence(yaml_document_t *, const yaml_char_t *, yaml_sequence_style_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_7752: {
        if !document.is_null() {} else {
            __assert_fail(
                b"document\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                1260 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 94],
                    &[libc::c_char; 94],
                >(
                    b"int yaml_document_add_sequence(yaml_document_t *, const yaml_char_t *, yaml_sequence_style_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if tag.is_null() {
        tag = b"tag:yaml.org,2002:seq\0" as *const u8 as *const libc::c_char
            as *mut yaml_char_t;
    }
    if !(yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)) == 0) {
        tag_copy = yaml_strdup(tag);
        if !tag_copy.is_null() {
            items
                .start = yaml_malloc(
                (16 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<yaml_node_item_t>() as libc::c_ulong,
                    ),
            ) as *mut yaml_node_item_t;
            if !(if !(items.start).is_null() {
                items.top = items.start;
                items.end = (items.start).offset(16 as libc::c_int as isize);
                1 as libc::c_int
            } else {
                context.error = YAML_MEMORY_ERROR;
                0 as libc::c_int
            } == 0)
            {
                memset(
                    &mut node as *mut yaml_node_t as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<yaml_node_t>() as libc::c_ulong,
                );
                node.type_0 = YAML_SEQUENCE_NODE;
                node.tag = tag_copy;
                node.start_mark = mark;
                node.end_mark = mark;
                node.data.sequence.items.start = items.start;
                node.data.sequence.items.end = items.end;
                node.data.sequence.items.top = items.start;
                node.data.sequence.style = style;
                if !(if (*document).nodes.top != (*document).nodes.end
                    || yaml_stack_extend(
                        &mut (*document).nodes.start as *mut *mut yaml_node_t
                            as *mut *mut libc::c_void,
                        &mut (*document).nodes.top as *mut *mut yaml_node_t
                            as *mut *mut libc::c_void,
                        &mut (*document).nodes.end as *mut *mut yaml_node_t
                            as *mut *mut libc::c_void,
                    ) != 0
                {
                    let fresh5 = (*document).nodes.top;
                    (*document).nodes.top = ((*document).nodes.top).offset(1);
                    *fresh5 = node;
                    1 as libc::c_int
                } else {
                    context.error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } == 0)
                {
                    return ((*document).nodes.top).offset_from((*document).nodes.start)
                        as libc::c_long as libc::c_int;
                }
            }
        }
    }
    yaml_free(items.start as *mut libc::c_void);
    items.end = 0 as *mut yaml_node_item_t;
    items.top = items.end;
    items.start = items.top;
    yaml_free(tag_copy as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn yaml_document_add_mapping(
    mut document: *mut yaml_document_t,
    mut tag: *const yaml_char_t,
    mut style: yaml_mapping_style_t,
) -> libc::c_int {
    let mut context: C2RustUnnamed_33 = C2RustUnnamed_33 {
        error: YAML_NO_ERROR,
    };
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as libc::c_int as size_t,
            line: 0 as libc::c_int as size_t,
            column: 0 as libc::c_int as size_t,
        };
        init
    };
    let mut tag_copy: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut pairs: C2RustUnnamed_32 = {
        let mut init = C2RustUnnamed_32 {
            start: 0 as *mut yaml_node_pair_t,
            end: 0 as *mut yaml_node_pair_t,
            top: 0 as *mut yaml_node_pair_t,
        };
        init
    };
    let mut node: yaml_node_t = yaml_node_t {
        type_0: YAML_NO_NODE,
        tag: 0 as *mut yaml_char_t,
        data: C2RustUnnamed_18 {
            scalar: C2RustUnnamed_23 {
                value: 0 as *mut yaml_char_t,
                length: 0,
                style: YAML_ANY_SCALAR_STYLE,
            },
        },
        start_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        end_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
    };
    if !document.is_null() {} else {
        __assert_fail(
            b"document\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1305 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 92],
                &[libc::c_char; 92],
            >(
                b"int yaml_document_add_mapping(yaml_document_t *, const yaml_char_t *, yaml_mapping_style_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8151: {
        if !document.is_null() {} else {
            __assert_fail(
                b"document\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                1305 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 92],
                    &[libc::c_char; 92],
                >(
                    b"int yaml_document_add_mapping(yaml_document_t *, const yaml_char_t *, yaml_mapping_style_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if tag.is_null() {
        tag = b"tag:yaml.org,2002:map\0" as *const u8 as *const libc::c_char
            as *mut yaml_char_t;
    }
    if !(yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)) == 0) {
        tag_copy = yaml_strdup(tag);
        if !tag_copy.is_null() {
            pairs
                .start = yaml_malloc(
                (16 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<yaml_node_pair_t>() as libc::c_ulong,
                    ),
            ) as *mut yaml_node_pair_t;
            if !(if !(pairs.start).is_null() {
                pairs.top = pairs.start;
                pairs.end = (pairs.start).offset(16 as libc::c_int as isize);
                1 as libc::c_int
            } else {
                context.error = YAML_MEMORY_ERROR;
                0 as libc::c_int
            } == 0)
            {
                memset(
                    &mut node as *mut yaml_node_t as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<yaml_node_t>() as libc::c_ulong,
                );
                node.type_0 = YAML_MAPPING_NODE;
                node.tag = tag_copy;
                node.start_mark = mark;
                node.end_mark = mark;
                node.data.mapping.pairs.start = pairs.start;
                node.data.mapping.pairs.end = pairs.end;
                node.data.mapping.pairs.top = pairs.start;
                node.data.mapping.style = style;
                if !(if (*document).nodes.top != (*document).nodes.end
                    || yaml_stack_extend(
                        &mut (*document).nodes.start as *mut *mut yaml_node_t
                            as *mut *mut libc::c_void,
                        &mut (*document).nodes.top as *mut *mut yaml_node_t
                            as *mut *mut libc::c_void,
                        &mut (*document).nodes.end as *mut *mut yaml_node_t
                            as *mut *mut libc::c_void,
                    ) != 0
                {
                    let fresh6 = (*document).nodes.top;
                    (*document).nodes.top = ((*document).nodes.top).offset(1);
                    *fresh6 = node;
                    1 as libc::c_int
                } else {
                    context.error = YAML_MEMORY_ERROR;
                    0 as libc::c_int
                } == 0)
                {
                    return ((*document).nodes.top).offset_from((*document).nodes.start)
                        as libc::c_long as libc::c_int;
                }
            }
        }
    }
    yaml_free(pairs.start as *mut libc::c_void);
    pairs.end = 0 as *mut yaml_node_pair_t;
    pairs.top = pairs.end;
    pairs.start = pairs.top;
    yaml_free(tag_copy as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn yaml_document_append_sequence_item(
    mut document: *mut yaml_document_t,
    mut sequence: libc::c_int,
    mut item: libc::c_int,
) -> libc::c_int {
    let mut context: C2RustUnnamed_34 = C2RustUnnamed_34 {
        error: YAML_NO_ERROR,
    };
    if !document.is_null() {} else {
        __assert_fail(
            b"document\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1342 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"int yaml_document_append_sequence_item(yaml_document_t *, int, int)\0"))
                .as_ptr(),
        );
    }
    'c_8528: {
        if !document.is_null() {} else {
            __assert_fail(
                b"document\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                1342 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 68],
                    &[libc::c_char; 68],
                >(
                    b"int yaml_document_append_sequence_item(yaml_document_t *, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if sequence > 0 as libc::c_int
        && ((*document).nodes.start).offset(sequence as isize) <= (*document).nodes.top
    {} else {
        __assert_fail(
            b"sequence > 0 && document->nodes.start + sequence <= document->nodes.top\0"
                as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1344 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"int yaml_document_append_sequence_item(yaml_document_t *, int, int)\0"))
                .as_ptr(),
        );
    }
    'c_8462: {
        if sequence > 0 as libc::c_int
            && ((*document).nodes.start).offset(sequence as isize)
                <= (*document).nodes.top
        {} else {
            __assert_fail(
                b"sequence > 0 && document->nodes.start + sequence <= document->nodes.top\0"
                    as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                1344 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 68],
                    &[libc::c_char; 68],
                >(
                    b"int yaml_document_append_sequence_item(yaml_document_t *, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*((*document).nodes.start).offset((sequence - 1 as libc::c_int) as isize)).type_0
        as libc::c_uint == YAML_SEQUENCE_NODE as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"document->nodes.start[sequence-1].type == YAML_SEQUENCE_NODE\0"
                as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1346 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"int yaml_document_append_sequence_item(yaml_document_t *, int, int)\0"))
                .as_ptr(),
        );
    }
    'c_8401: {
        if (*((*document).nodes.start).offset((sequence - 1 as libc::c_int) as isize))
            .type_0 as libc::c_uint == YAML_SEQUENCE_NODE as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"document->nodes.start[sequence-1].type == YAML_SEQUENCE_NODE\0"
                    as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                1346 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 68],
                    &[libc::c_char; 68],
                >(
                    b"int yaml_document_append_sequence_item(yaml_document_t *, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if item > 0 as libc::c_int
        && ((*document).nodes.start).offset(item as isize) <= (*document).nodes.top
    {} else {
        __assert_fail(
            b"item > 0 && document->nodes.start + item <= document->nodes.top\0"
                as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1348 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"int yaml_document_append_sequence_item(yaml_document_t *, int, int)\0"))
                .as_ptr(),
        );
    }
    'c_8333: {
        if item > 0 as libc::c_int
            && ((*document).nodes.start).offset(item as isize) <= (*document).nodes.top
        {} else {
            __assert_fail(
                b"item > 0 && document->nodes.start + item <= document->nodes.top\0"
                    as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                1348 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 68],
                    &[libc::c_char; 68],
                >(
                    b"int yaml_document_append_sequence_item(yaml_document_t *, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if if (*((*document).nodes.start).offset((sequence - 1 as libc::c_int) as isize))
        .data
        .sequence
        .items
        .top
        != (*((*document).nodes.start).offset((sequence - 1 as libc::c_int) as isize))
            .data
            .sequence
            .items
            .end
        || yaml_stack_extend(
            &mut (*((*document).nodes.start)
                .offset((sequence - 1 as libc::c_int) as isize))
                .data
                .sequence
                .items
                .start as *mut *mut yaml_node_item_t as *mut *mut libc::c_void,
            &mut (*((*document).nodes.start)
                .offset((sequence - 1 as libc::c_int) as isize))
                .data
                .sequence
                .items
                .top as *mut *mut yaml_node_item_t as *mut *mut libc::c_void,
            &mut (*((*document).nodes.start)
                .offset((sequence - 1 as libc::c_int) as isize))
                .data
                .sequence
                .items
                .end as *mut *mut yaml_node_item_t as *mut *mut libc::c_void,
        ) != 0
    {
        let ref mut fresh7 = (*((*document).nodes.start)
            .offset((sequence - 1 as libc::c_int) as isize))
            .data
            .sequence
            .items
            .top;
        let fresh8 = *fresh7;
        *fresh7 = (*fresh7).offset(1);
        *fresh8 = item;
        1 as libc::c_int
    } else {
        context.error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn yaml_document_append_mapping_pair(
    mut document: *mut yaml_document_t,
    mut mapping: libc::c_int,
    mut key: libc::c_int,
    mut value: libc::c_int,
) -> libc::c_int {
    let mut context: C2RustUnnamed_35 = C2RustUnnamed_35 {
        error: YAML_NO_ERROR,
    };
    let mut pair: yaml_node_pair_t = yaml_node_pair_t {
        key: 0,
        value: 0,
    };
    if !document.is_null() {} else {
        __assert_fail(
            b"document\0" as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1372 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int yaml_document_append_mapping_pair(yaml_document_t *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8987: {
        if !document.is_null() {} else {
            __assert_fail(
                b"document\0" as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                1372 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"int yaml_document_append_mapping_pair(yaml_document_t *, int, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if mapping > 0 as libc::c_int
        && ((*document).nodes.start).offset(mapping as isize) <= (*document).nodes.top
    {} else {
        __assert_fail(
            b"mapping > 0 && document->nodes.start + mapping <= document->nodes.top\0"
                as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1374 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int yaml_document_append_mapping_pair(yaml_document_t *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8920: {
        if mapping > 0 as libc::c_int
            && ((*document).nodes.start).offset(mapping as isize)
                <= (*document).nodes.top
        {} else {
            __assert_fail(
                b"mapping > 0 && document->nodes.start + mapping <= document->nodes.top\0"
                    as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                1374 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"int yaml_document_append_mapping_pair(yaml_document_t *, int, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*((*document).nodes.start).offset((mapping - 1 as libc::c_int) as isize)).type_0
        as libc::c_uint == YAML_MAPPING_NODE as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"document->nodes.start[mapping-1].type == YAML_MAPPING_NODE\0" as *const u8
                as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1376 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int yaml_document_append_mapping_pair(yaml_document_t *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8859: {
        if (*((*document).nodes.start).offset((mapping - 1 as libc::c_int) as isize))
            .type_0 as libc::c_uint == YAML_MAPPING_NODE as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"document->nodes.start[mapping-1].type == YAML_MAPPING_NODE\0"
                    as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                1376 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"int yaml_document_append_mapping_pair(yaml_document_t *, int, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if key > 0 as libc::c_int
        && ((*document).nodes.start).offset(key as isize) <= (*document).nodes.top
    {} else {
        __assert_fail(
            b"key > 0 && document->nodes.start + key <= document->nodes.top\0"
                as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1378 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int yaml_document_append_mapping_pair(yaml_document_t *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8792: {
        if key > 0 as libc::c_int
            && ((*document).nodes.start).offset(key as isize) <= (*document).nodes.top
        {} else {
            __assert_fail(
                b"key > 0 && document->nodes.start + key <= document->nodes.top\0"
                    as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                1378 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"int yaml_document_append_mapping_pair(yaml_document_t *, int, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if value > 0 as libc::c_int
        && ((*document).nodes.start).offset(value as isize) <= (*document).nodes.top
    {} else {
        __assert_fail(
            b"value > 0 && document->nodes.start + value <= document->nodes.top\0"
                as *const u8 as *const libc::c_char,
            b"api.c\0" as *const u8 as *const libc::c_char,
            1380 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int yaml_document_append_mapping_pair(yaml_document_t *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8725: {
        if value > 0 as libc::c_int
            && ((*document).nodes.start).offset(value as isize) <= (*document).nodes.top
        {} else {
            __assert_fail(
                b"value > 0 && document->nodes.start + value <= document->nodes.top\0"
                    as *const u8 as *const libc::c_char,
                b"api.c\0" as *const u8 as *const libc::c_char,
                1380 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"int yaml_document_append_mapping_pair(yaml_document_t *, int, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    pair.key = key;
    pair.value = value;
    if if (*((*document).nodes.start).offset((mapping - 1 as libc::c_int) as isize))
        .data
        .mapping
        .pairs
        .top
        != (*((*document).nodes.start).offset((mapping - 1 as libc::c_int) as isize))
            .data
            .mapping
            .pairs
            .end
        || yaml_stack_extend(
            &mut (*((*document).nodes.start)
                .offset((mapping - 1 as libc::c_int) as isize))
                .data
                .mapping
                .pairs
                .start as *mut *mut yaml_node_pair_t as *mut *mut libc::c_void,
            &mut (*((*document).nodes.start)
                .offset((mapping - 1 as libc::c_int) as isize))
                .data
                .mapping
                .pairs
                .top as *mut *mut yaml_node_pair_t as *mut *mut libc::c_void,
            &mut (*((*document).nodes.start)
                .offset((mapping - 1 as libc::c_int) as isize))
                .data
                .mapping
                .pairs
                .end as *mut *mut yaml_node_pair_t as *mut *mut libc::c_void,
        ) != 0
    {
        let ref mut fresh9 = (*((*document).nodes.start)
            .offset((mapping - 1 as libc::c_int) as isize))
            .data
            .mapping
            .pairs
            .top;
        let fresh10 = *fresh9;
        *fresh9 = (*fresh9).offset(1);
        *fresh10 = pair;
        1 as libc::c_int
    } else {
        context.error = YAML_MEMORY_ERROR;
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
