use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct http_parser {
    #[bitfield(name = "type_0", ty = "libc::c_uint", bits = "0..=1")]
    #[bitfield(name = "flags", ty = "libc::c_uint", bits = "2..=9")]
    #[bitfield(name = "state", ty = "libc::c_uint", bits = "10..=16")]
    #[bitfield(name = "header_state", ty = "libc::c_uint", bits = "17..=23")]
    #[bitfield(name = "index", ty = "libc::c_uint", bits = "24..=30")]
    #[bitfield(name = "lenient_http_headers", ty = "libc::c_uint", bits = "31..=31")]
    pub type_0_flags_state_header_state_index_lenient_http_headers: [u8; 4],
    pub nread: uint32_t,
    pub content_length: uint64_t,
    pub http_major: libc::c_ushort,
    pub http_minor: libc::c_ushort,
    #[bitfield(name = "status_code", ty = "libc::c_uint", bits = "0..=15")]
    #[bitfield(name = "method", ty = "libc::c_uint", bits = "16..=23")]
    #[bitfield(name = "http_errno", ty = "libc::c_uint", bits = "24..=30")]
    #[bitfield(name = "upgrade", ty = "libc::c_uint", bits = "31..=31")]
    pub status_code_method_http_errno_upgrade: [u8; 4],
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_parser_settings {
    pub on_message_begin: http_cb,
    pub on_url: http_data_cb,
    pub on_status: http_data_cb,
    pub on_header_field: http_data_cb,
    pub on_header_value: http_data_cb,
    pub on_headers_complete: http_cb,
    pub on_body: http_data_cb,
    pub on_message_complete: http_cb,
    pub on_chunk_header: http_cb,
    pub on_chunk_complete: http_cb,
}
pub type http_cb = Option::<unsafe extern "C" fn(*mut http_parser) -> libc::c_int>;
pub type http_data_cb = Option::<
    unsafe extern "C" fn(*mut http_parser, *const libc::c_char, size_t) -> libc::c_int,
>;
pub type http_method = libc::c_uint;
pub const HTTP_UNLINK: http_method = 32;
pub const HTTP_LINK: http_method = 31;
pub const HTTP_MKCALENDAR: http_method = 30;
pub const HTTP_PURGE: http_method = 29;
pub const HTTP_PATCH: http_method = 28;
pub const HTTP_UNSUBSCRIBE: http_method = 27;
pub const HTTP_SUBSCRIBE: http_method = 26;
pub const HTTP_NOTIFY: http_method = 25;
pub const HTTP_MSEARCH: http_method = 24;
pub const HTTP_MERGE: http_method = 23;
pub const HTTP_CHECKOUT: http_method = 22;
pub const HTTP_MKACTIVITY: http_method = 21;
pub const HTTP_REPORT: http_method = 20;
pub const HTTP_ACL: http_method = 19;
pub const HTTP_UNBIND: http_method = 18;
pub const HTTP_REBIND: http_method = 17;
pub const HTTP_BIND: http_method = 16;
pub const HTTP_UNLOCK: http_method = 15;
pub const HTTP_SEARCH: http_method = 14;
pub const HTTP_PROPPATCH: http_method = 13;
pub const HTTP_PROPFIND: http_method = 12;
pub const HTTP_MOVE: http_method = 11;
pub const HTTP_MKCOL: http_method = 10;
pub const HTTP_LOCK: http_method = 9;
pub const HTTP_COPY: http_method = 8;
pub const HTTP_TRACE: http_method = 7;
pub const HTTP_OPTIONS: http_method = 6;
pub const HTTP_CONNECT: http_method = 5;
pub const HTTP_PUT: http_method = 4;
pub const HTTP_POST: http_method = 3;
pub const HTTP_HEAD: http_method = 2;
pub const HTTP_GET: http_method = 1;
pub const HTTP_DELETE: http_method = 0;
pub type http_parser_type = libc::c_uint;
pub const HTTP_BOTH: http_parser_type = 2;
pub const HTTP_RESPONSE: http_parser_type = 1;
pub const HTTP_REQUEST: http_parser_type = 0;
pub type flags = libc::c_uint;
pub const F_CONTENTLENGTH: flags = 128;
pub const F_SKIPBODY: flags = 64;
pub const F_UPGRADE: flags = 32;
pub const F_TRAILING: flags = 16;
pub const F_CONNECTION_UPGRADE: flags = 8;
pub const F_CONNECTION_CLOSE: flags = 4;
pub const F_CONNECTION_KEEP_ALIVE: flags = 2;
pub const F_CHUNKED: flags = 1;
pub type http_errno = libc::c_uint;
pub const HPE_UNKNOWN: http_errno = 32;
pub const HPE_PAUSED: http_errno = 31;
pub const HPE_STRICT: http_errno = 30;
pub const HPE_INVALID_INTERNAL_STATE: http_errno = 29;
pub const HPE_INVALID_CONSTANT: http_errno = 28;
pub const HPE_INVALID_CHUNK_SIZE: http_errno = 27;
pub const HPE_UNEXPECTED_CONTENT_LENGTH: http_errno = 26;
pub const HPE_INVALID_CONTENT_LENGTH: http_errno = 25;
pub const HPE_INVALID_HEADER_TOKEN: http_errno = 24;
pub const HPE_LF_EXPECTED: http_errno = 23;
pub const HPE_INVALID_FRAGMENT: http_errno = 22;
pub const HPE_INVALID_QUERY_STRING: http_errno = 21;
pub const HPE_INVALID_PATH: http_errno = 20;
pub const HPE_INVALID_PORT: http_errno = 19;
pub const HPE_INVALID_HOST: http_errno = 18;
pub const HPE_INVALID_URL: http_errno = 17;
pub const HPE_INVALID_METHOD: http_errno = 16;
pub const HPE_INVALID_STATUS: http_errno = 15;
pub const HPE_INVALID_VERSION: http_errno = 14;
pub const HPE_CLOSED_CONNECTION: http_errno = 13;
pub const HPE_HEADER_OVERFLOW: http_errno = 12;
pub const HPE_INVALID_EOF_STATE: http_errno = 11;
pub const HPE_CB_chunk_complete: http_errno = 10;
pub const HPE_CB_chunk_header: http_errno = 9;
pub const HPE_CB_status: http_errno = 8;
pub const HPE_CB_message_complete: http_errno = 7;
pub const HPE_CB_body: http_errno = 6;
pub const HPE_CB_headers_complete: http_errno = 5;
pub const HPE_CB_header_value: http_errno = 4;
pub const HPE_CB_header_field: http_errno = 3;
pub const HPE_CB_url: http_errno = 2;
pub const HPE_CB_message_begin: http_errno = 1;
pub const HPE_OK: http_errno = 0;
pub type http_parser_url_fields = libc::c_uint;
pub const UF_MAX: http_parser_url_fields = 7;
pub const UF_USERINFO: http_parser_url_fields = 6;
pub const UF_FRAGMENT: http_parser_url_fields = 5;
pub const UF_QUERY: http_parser_url_fields = 4;
pub const UF_PATH: http_parser_url_fields = 3;
pub const UF_PORT: http_parser_url_fields = 2;
pub const UF_HOST: http_parser_url_fields = 1;
pub const UF_SCHEMA: http_parser_url_fields = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_parser_url {
    pub field_set: uint16_t,
    pub port: uint16_t,
    pub field_data: [C2RustUnnamed; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub off: uint16_t,
    pub len: uint16_t,
}
pub const s_start_req_or_res: state = 2;
pub const s_start_res: state = 4;
pub const s_start_req: state = 18;
pub type state = libc::c_uint;
pub const s_message_done: state = 62;
pub const s_body_identity_eof: state = 61;
pub const s_body_identity: state = 60;
pub const s_chunk_data_done: state = 59;
pub const s_chunk_data_almost_done: state = 58;
pub const s_chunk_data: state = 57;
pub const s_headers_done: state = 56;
pub const s_headers_almost_done: state = 55;
pub const s_chunk_size_almost_done: state = 54;
pub const s_chunk_parameters: state = 53;
pub const s_chunk_size: state = 52;
pub const s_chunk_size_start: state = 51;
pub const s_header_almost_done: state = 50;
pub const s_header_value_lws: state = 49;
pub const s_header_value: state = 48;
pub const s_header_value_start: state = 47;
pub const s_header_value_discard_lws: state = 46;
pub const s_header_value_discard_ws_almost_done: state = 45;
pub const s_header_value_discard_ws: state = 44;
pub const s_header_field: state = 43;
pub const s_header_field_start: state = 42;
pub const s_req_line_almost_done: state = 41;
pub const s_req_http_end: state = 40;
pub const s_req_http_minor: state = 39;
pub const s_req_http_dot: state = 38;
pub const s_req_http_major: state = 37;
pub const s_req_http_HTTP: state = 36;
pub const s_req_http_HTT: state = 35;
pub const s_req_http_HT: state = 34;
pub const s_req_http_H: state = 33;
pub const s_req_http_start: state = 32;
pub const s_req_fragment: state = 31;
pub const s_req_fragment_start: state = 30;
pub const s_req_query_string: state = 29;
pub const s_req_query_string_start: state = 28;
pub const s_req_path: state = 27;
pub const s_req_server_with_at: state = 26;
pub const s_req_server: state = 25;
pub const s_req_server_start: state = 24;
pub const s_req_schema_slash_slash: state = 23;
pub const s_req_schema_slash: state = 22;
pub const s_req_schema: state = 21;
pub const s_req_spaces_before_url: state = 20;
pub const s_req_method: state = 19;
pub const s_res_line_almost_done: state = 17;
pub const s_res_status: state = 16;
pub const s_res_status_start: state = 15;
pub const s_res_status_code: state = 14;
pub const s_res_first_status_code: state = 13;
pub const s_res_http_end: state = 12;
pub const s_res_http_minor: state = 11;
pub const s_res_http_dot: state = 10;
pub const s_res_http_major: state = 9;
pub const s_res_HTTP: state = 8;
pub const s_res_HTT: state = 7;
pub const s_res_HT: state = 6;
pub const s_res_H: state = 5;
pub const s_res_or_resp_H: state = 3;
pub const s_dead: state = 1;
pub const h_transfer_encoding_chunked: header_states = 19;
pub const h_connection_upgrade: header_states = 22;
pub const h_connection_close: header_states = 21;
pub const h_connection_keep_alive: header_states = 20;
pub type header_states = libc::c_uint;
pub const h_matching_connection_token: header_states = 18;
pub const h_matching_connection_upgrade: header_states = 17;
pub const h_matching_connection_close: header_states = 16;
pub const h_matching_connection_keep_alive: header_states = 15;
pub const h_matching_connection_token_start: header_states = 14;
pub const h_matching_transfer_encoding_chunked: header_states = 13;
pub const h_upgrade: header_states = 12;
pub const h_transfer_encoding: header_states = 11;
pub const h_content_length: header_states = 10;
pub const h_connection: header_states = 9;
pub const h_matching_upgrade: header_states = 8;
pub const h_matching_transfer_encoding: header_states = 7;
pub const h_matching_content_length: header_states = 6;
pub const h_matching_proxy_connection: header_states = 5;
pub const h_matching_connection: header_states = 4;
pub const h_CON: header_states = 3;
pub const h_CO: header_states = 2;
pub const h_C: header_states = 1;
pub const h_general: header_states = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub name: *const libc::c_char,
    pub description: *const libc::c_char,
}
pub const s_http_userinfo_start: http_host_state = 2;
pub const s_http_userinfo: http_host_state = 3;
pub const s_http_host_port_start: http_host_state = 11;
pub const s_http_host_v6_zone: http_host_state = 10;
pub const s_http_host_v6_zone_start: http_host_state = 9;
pub const s_http_host_v6: http_host_state = 7;
pub const s_http_host_v6_start: http_host_state = 5;
pub const s_http_host_start: http_host_state = 4;
pub type http_host_state = libc::c_uint;
pub const s_http_host_port: http_host_state = 12;
pub const s_http_host_v6_end: http_host_state = 8;
pub const s_http_host: http_host_state = 6;
pub const s_http_host_dead: http_host_state = 1;
static mut method_strings: [*const libc::c_char; 33] = [
    b"DELETE\0" as *const u8 as *const libc::c_char,
    b"GET\0" as *const u8 as *const libc::c_char,
    b"HEAD\0" as *const u8 as *const libc::c_char,
    b"POST\0" as *const u8 as *const libc::c_char,
    b"PUT\0" as *const u8 as *const libc::c_char,
    b"CONNECT\0" as *const u8 as *const libc::c_char,
    b"OPTIONS\0" as *const u8 as *const libc::c_char,
    b"TRACE\0" as *const u8 as *const libc::c_char,
    b"COPY\0" as *const u8 as *const libc::c_char,
    b"LOCK\0" as *const u8 as *const libc::c_char,
    b"MKCOL\0" as *const u8 as *const libc::c_char,
    b"MOVE\0" as *const u8 as *const libc::c_char,
    b"PROPFIND\0" as *const u8 as *const libc::c_char,
    b"PROPPATCH\0" as *const u8 as *const libc::c_char,
    b"SEARCH\0" as *const u8 as *const libc::c_char,
    b"UNLOCK\0" as *const u8 as *const libc::c_char,
    b"BIND\0" as *const u8 as *const libc::c_char,
    b"REBIND\0" as *const u8 as *const libc::c_char,
    b"UNBIND\0" as *const u8 as *const libc::c_char,
    b"ACL\0" as *const u8 as *const libc::c_char,
    b"REPORT\0" as *const u8 as *const libc::c_char,
    b"MKACTIVITY\0" as *const u8 as *const libc::c_char,
    b"CHECKOUT\0" as *const u8 as *const libc::c_char,
    b"MERGE\0" as *const u8 as *const libc::c_char,
    b"M-SEARCH\0" as *const u8 as *const libc::c_char,
    b"NOTIFY\0" as *const u8 as *const libc::c_char,
    b"SUBSCRIBE\0" as *const u8 as *const libc::c_char,
    b"UNSUBSCRIBE\0" as *const u8 as *const libc::c_char,
    b"PATCH\0" as *const u8 as *const libc::c_char,
    b"PURGE\0" as *const u8 as *const libc::c_char,
    b"MKCALENDAR\0" as *const u8 as *const libc::c_char,
    b"LINK\0" as *const u8 as *const libc::c_char,
    b"UNLINK\0" as *const u8 as *const libc::c_char,
];
static mut tokens: [libc::c_char; 256] = [
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '!' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '#' as i32 as libc::c_char,
    '$' as i32 as libc::c_char,
    '%' as i32 as libc::c_char,
    '&' as i32 as libc::c_char,
    '\'' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '*' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '-' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '^' as i32 as libc::c_char,
    '_' as i32 as libc::c_char,
    '`' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '|' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '~' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
static mut unhex: [int8_t; 256] = [
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    0 as libc::c_int as int8_t,
    1 as libc::c_int as int8_t,
    2 as libc::c_int as int8_t,
    3 as libc::c_int as int8_t,
    4 as libc::c_int as int8_t,
    5 as libc::c_int as int8_t,
    6 as libc::c_int as int8_t,
    7 as libc::c_int as int8_t,
    8 as libc::c_int as int8_t,
    9 as libc::c_int as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    10 as libc::c_int as int8_t,
    11 as libc::c_int as int8_t,
    12 as libc::c_int as int8_t,
    13 as libc::c_int as int8_t,
    14 as libc::c_int as int8_t,
    15 as libc::c_int as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    10 as libc::c_int as int8_t,
    11 as libc::c_int as int8_t,
    12 as libc::c_int as int8_t,
    13 as libc::c_int as int8_t,
    14 as libc::c_int as int8_t,
    15 as libc::c_int as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    -(1 as libc::c_int) as int8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
static mut normal_url_char: [uint8_t; 32] = [
    (0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int
        | 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int)
        as uint8_t,
    (0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int
        | 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int)
        as uint8_t,
    (0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int
        | 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int)
        as uint8_t,
    (0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int
        | 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int)
        as uint8_t,
    (0 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 0 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 128 as libc::c_int)
        as uint8_t,
    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 128 as libc::c_int)
        as uint8_t,
    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 128 as libc::c_int)
        as uint8_t,
    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 0 as libc::c_int)
        as uint8_t,
    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 128 as libc::c_int)
        as uint8_t,
    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 128 as libc::c_int)
        as uint8_t,
    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 128 as libc::c_int)
        as uint8_t,
    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 128 as libc::c_int)
        as uint8_t,
    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 128 as libc::c_int)
        as uint8_t,
    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 128 as libc::c_int)
        as uint8_t,
    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 128 as libc::c_int)
        as uint8_t,
    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
        | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int | 0 as libc::c_int)
        as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
static mut http_strerror_tab: [C2RustUnnamed_0; 33] = [
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_OK\0" as *const u8 as *const libc::c_char,
            description: b"success\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_CB_message_begin\0" as *const u8 as *const libc::c_char,
            description: b"the on_message_begin callback failed\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_CB_url\0" as *const u8 as *const libc::c_char,
            description: b"the on_url callback failed\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_CB_header_field\0" as *const u8 as *const libc::c_char,
            description: b"the on_header_field callback failed\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_CB_header_value\0" as *const u8 as *const libc::c_char,
            description: b"the on_header_value callback failed\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_CB_headers_complete\0" as *const u8 as *const libc::c_char,
            description: b"the on_headers_complete callback failed\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_CB_body\0" as *const u8 as *const libc::c_char,
            description: b"the on_body callback failed\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_CB_message_complete\0" as *const u8 as *const libc::c_char,
            description: b"the on_message_complete callback failed\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_CB_status\0" as *const u8 as *const libc::c_char,
            description: b"the on_status callback failed\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_CB_chunk_header\0" as *const u8 as *const libc::c_char,
            description: b"the on_chunk_header callback failed\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_CB_chunk_complete\0" as *const u8 as *const libc::c_char,
            description: b"the on_chunk_complete callback failed\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_INVALID_EOF_STATE\0" as *const u8 as *const libc::c_char,
            description: b"stream ended at an unexpected time\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_HEADER_OVERFLOW\0" as *const u8 as *const libc::c_char,
            description: b"too many header bytes seen; overflow detected\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_CLOSED_CONNECTION\0" as *const u8 as *const libc::c_char,
            description: b"data received after completed connection: close message\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_INVALID_VERSION\0" as *const u8 as *const libc::c_char,
            description: b"invalid HTTP version\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_INVALID_STATUS\0" as *const u8 as *const libc::c_char,
            description: b"invalid HTTP status code\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_INVALID_METHOD\0" as *const u8 as *const libc::c_char,
            description: b"invalid HTTP method\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_INVALID_URL\0" as *const u8 as *const libc::c_char,
            description: b"invalid URL\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_INVALID_HOST\0" as *const u8 as *const libc::c_char,
            description: b"invalid host\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_INVALID_PORT\0" as *const u8 as *const libc::c_char,
            description: b"invalid port\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_INVALID_PATH\0" as *const u8 as *const libc::c_char,
            description: b"invalid path\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_INVALID_QUERY_STRING\0" as *const u8 as *const libc::c_char,
            description: b"invalid query string\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_INVALID_FRAGMENT\0" as *const u8 as *const libc::c_char,
            description: b"invalid fragment\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_LF_EXPECTED\0" as *const u8 as *const libc::c_char,
            description: b"LF character expected\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_INVALID_HEADER_TOKEN\0" as *const u8 as *const libc::c_char,
            description: b"invalid character in header\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_INVALID_CONTENT_LENGTH\0" as *const u8 as *const libc::c_char,
            description: b"invalid character in content-length header\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_UNEXPECTED_CONTENT_LENGTH\0" as *const u8 as *const libc::c_char,
            description: b"unexpected content-length header\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_INVALID_CHUNK_SIZE\0" as *const u8 as *const libc::c_char,
            description: b"invalid character in chunk size header\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_INVALID_CONSTANT\0" as *const u8 as *const libc::c_char,
            description: b"invalid constant string\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_INVALID_INTERNAL_STATE\0" as *const u8 as *const libc::c_char,
            description: b"encountered unexpected internal state\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_STRICT\0" as *const u8 as *const libc::c_char,
            description: b"strict mode assertion failed\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_PAUSED\0" as *const u8 as *const libc::c_char,
            description: b"parser is paused\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"HPE_UNKNOWN\0" as *const u8 as *const libc::c_char,
            description: b"an unknown error occurred\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
];
unsafe extern "C" fn parse_url_char(mut s: state, ch: libc::c_char) -> state {
    if ch as libc::c_int == ' ' as i32 || ch as libc::c_int == '\r' as i32
        || ch as libc::c_int == '\n' as i32
    {
        return s_dead;
    }
    if ch as libc::c_int == '\t' as i32 || ch as libc::c_int == '\u{c}' as i32 {
        return s_dead;
    }
    let mut current_block_61: u64;
    match s as libc::c_uint {
        20 => {
            if ch as libc::c_int == '/' as i32 || ch as libc::c_int == '*' as i32 {
                return s_req_path;
            }
            if (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar as libc::c_int
                >= 'a' as i32
                && (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                    as libc::c_int <= 'z' as i32
            {
                return s_req_schema;
            }
            current_block_61 = 7018308795614528254;
        }
        21 => {
            if (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar as libc::c_int
                >= 'a' as i32
                && (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                    as libc::c_int <= 'z' as i32
            {
                return s;
            }
            if ch as libc::c_int == ':' as i32 {
                return s_req_schema_slash;
            }
            current_block_61 = 7018308795614528254;
        }
        22 => {
            if ch as libc::c_int == '/' as i32 {
                return s_req_schema_slash_slash;
            }
            current_block_61 = 7018308795614528254;
        }
        23 => {
            if ch as libc::c_int == '/' as i32 {
                return s_req_server_start;
            }
            current_block_61 = 7018308795614528254;
        }
        26 => {
            if ch as libc::c_int == '@' as i32 {
                return s_dead;
            }
            current_block_61 = 13486725532101017904;
        }
        24 | 25 => {
            current_block_61 = 13486725532101017904;
        }
        27 => {
            if normal_url_char[(ch as libc::c_uchar as libc::c_uint >> 3 as libc::c_int)
                as usize] as libc::c_uint
                & ((1 as libc::c_int)
                    << (ch as libc::c_uchar as libc::c_uint
                        & 7 as libc::c_int as libc::c_uint)) as libc::c_uint != 0
            {
                return s;
            }
            match ch as libc::c_int {
                63 => return s_req_query_string_start,
                35 => return s_req_fragment_start,
                _ => {}
            }
            current_block_61 = 7018308795614528254;
        }
        28 | 29 => {
            if normal_url_char[(ch as libc::c_uchar as libc::c_uint >> 3 as libc::c_int)
                as usize] as libc::c_uint
                & ((1 as libc::c_int)
                    << (ch as libc::c_uchar as libc::c_uint
                        & 7 as libc::c_int as libc::c_uint)) as libc::c_uint != 0
            {
                return s_req_query_string;
            }
            match ch as libc::c_int {
                63 => return s_req_query_string,
                35 => return s_req_fragment_start,
                _ => {}
            }
            current_block_61 = 7018308795614528254;
        }
        30 => {
            if normal_url_char[(ch as libc::c_uchar as libc::c_uint >> 3 as libc::c_int)
                as usize] as libc::c_uint
                & ((1 as libc::c_int)
                    << (ch as libc::c_uchar as libc::c_uint
                        & 7 as libc::c_int as libc::c_uint)) as libc::c_uint != 0
            {
                return s_req_fragment;
            }
            match ch as libc::c_int {
                63 => return s_req_fragment,
                35 => return s,
                _ => {}
            }
            current_block_61 = 7018308795614528254;
        }
        31 => {
            if normal_url_char[(ch as libc::c_uchar as libc::c_uint >> 3 as libc::c_int)
                as usize] as libc::c_uint
                & ((1 as libc::c_int)
                    << (ch as libc::c_uchar as libc::c_uint
                        & 7 as libc::c_int as libc::c_uint)) as libc::c_uint != 0
            {
                return s;
            }
            match ch as libc::c_int {
                63 | 35 => return s,
                _ => {}
            }
            current_block_61 = 7018308795614528254;
        }
        _ => {
            current_block_61 = 7018308795614528254;
        }
    }
    match current_block_61 {
        13486725532101017904 => {
            if ch as libc::c_int == '/' as i32 {
                return s_req_path;
            }
            if ch as libc::c_int == '?' as i32 {
                return s_req_query_string_start;
            }
            if ch as libc::c_int == '@' as i32 {
                return s_req_server_with_at;
            }
            if (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar as libc::c_int
                >= 'a' as i32
                && (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                    as libc::c_int <= 'z' as i32
                || ch as libc::c_int >= '0' as i32 && ch as libc::c_int <= '9' as i32
                || (ch as libc::c_int == '-' as i32 || ch as libc::c_int == '_' as i32
                    || ch as libc::c_int == '.' as i32 || ch as libc::c_int == '!' as i32
                    || ch as libc::c_int == '~' as i32 || ch as libc::c_int == '*' as i32
                    || ch as libc::c_int == '\'' as i32
                    || ch as libc::c_int == '(' as i32
                    || ch as libc::c_int == ')' as i32)
                || ch as libc::c_int == '%' as i32 || ch as libc::c_int == ';' as i32
                || ch as libc::c_int == ':' as i32 || ch as libc::c_int == '&' as i32
                || ch as libc::c_int == '=' as i32 || ch as libc::c_int == '+' as i32
                || ch as libc::c_int == '$' as i32 || ch as libc::c_int == ',' as i32
                || ch as libc::c_int == '[' as i32 || ch as libc::c_int == ']' as i32
            {
                return s_req_server;
            }
        }
        _ => {}
    }
    return s_dead;
}
pub unsafe extern "C" fn http_parser_execute(
    mut parser: *mut http_parser,
    mut settings: *const http_parser_settings,
    mut data: *const libc::c_char,
    mut len: size_t,
) -> size_t {
    let mut hasBody: libc::c_int = 0;
    let mut start_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut h_state: header_states = h_general;
    let mut t_0: uint64_t = 0;
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut matcher: *const libc::c_char = 0 as *const libc::c_char;
    let mut current_block: u64;
    let mut c: libc::c_char = 0;
    let mut ch: libc::c_char = 0;
    let mut unhex_val: int8_t = 0;
    let mut p: *const libc::c_char = data;
    let mut header_field_mark: *const libc::c_char = 0 as *const libc::c_char;
    let mut header_value_mark: *const libc::c_char = 0 as *const libc::c_char;
    let mut url_mark: *const libc::c_char = 0 as *const libc::c_char;
    let mut body_mark: *const libc::c_char = 0 as *const libc::c_char;
    let mut status_mark: *const libc::c_char = 0 as *const libc::c_char;
    let mut p_state: state = (*parser).state() as state;
    let lenient: libc::c_uint = (*parser).lenient_http_headers();
    if (*parser).http_errno() as http_errno as libc::c_uint
        != HPE_OK as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int as size_t;
    }
    if len == 0 as libc::c_int as libc::c_ulong {
        match p_state as libc::c_uint {
            61 => {
                if (*parser).http_errno() as http_errno as libc::c_uint
                    == HPE_OK as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                            as *const libc::c_char,
                        b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                        658 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 94],
                            &[libc::c_char; 94],
                        >(
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                };
                if ((*settings).on_message_complete).is_some() as libc::c_int
                    as libc::c_long != 0
                {
                    (*parser).set_state(p_state as libc::c_uint);
                    if (0 as libc::c_int
                        != ((*settings).on_message_complete).unwrap()(parser))
                        as libc::c_int as libc::c_long != 0
                    {
                        (*parser)
                            .set_http_errno(
                                HPE_CB_message_complete as libc::c_int as libc::c_uint,
                            );
                    }
                    p_state = (*parser).state() as state;
                    if ((*parser).http_errno() as http_errno as libc::c_uint
                        != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                        as libc::c_long != 0
                    {
                        return p.offset_from(data) as libc::c_long as size_t;
                    }
                }
                return 0 as libc::c_int as size_t;
            }
            1 | 2 | 4 | 18 => return 0 as libc::c_int as size_t,
            _ => {
                (*parser)
                    .set_http_errno(
                        HPE_INVALID_EOF_STATE as libc::c_int as libc::c_uint,
                    );
                return 1 as libc::c_int as size_t;
            }
        }
    }
    if p_state as libc::c_uint == s_header_field as libc::c_int as libc::c_uint {
        header_field_mark = data;
    }
    if p_state as libc::c_uint == s_header_value as libc::c_int as libc::c_uint {
        header_value_mark = data;
    }
    match p_state as libc::c_uint {
        27 | 21 | 22 | 23 | 24 | 25 | 26 | 28 | 29 | 30 | 31 => {
            url_mark = data;
        }
        16 => {
            status_mark = data;
        }
        _ => {}
    }
    p = data;
    's_197: loop {
        if !(p != data.offset(len as isize)) {
            current_block = 16625613143405125062;
            break;
        }
        ch = *p;
        if p_state as libc::c_uint <= s_headers_done as libc::c_int as libc::c_uint {
            (*parser)
                .nread = ((*parser).nread as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
            if ((*parser).nread
                > (80 as libc::c_int * 1024 as libc::c_int) as libc::c_uint)
                as libc::c_int as libc::c_long != 0
            {
                (*parser)
                    .set_http_errno(HPE_HEADER_OVERFLOW as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
        }
        '_reexecute: loop {
            match p_state as libc::c_uint {
                1 => {
                    if (ch as libc::c_int == '\r' as i32
                        || ch as libc::c_int == '\n' as i32) as libc::c_int
                        as libc::c_long != 0
                    {
                        current_block = 9007357115414505193;
                        break;
                    } else {
                        current_block = 1423531122933789233;
                        break;
                    }
                }
                2 => {
                    if ch as libc::c_int == '\r' as i32
                        || ch as libc::c_int == '\n' as i32
                    {
                        current_block = 9007357115414505193;
                        break;
                    }
                    (*parser).set_flags(0 as libc::c_int as libc::c_uint);
                    (*parser)
                        .content_length = (9223372036854775807 as libc::c_longlong
                        as libc::c_ulonglong)
                        .wrapping_mul(2 as libc::c_ulonglong)
                        .wrapping_add(1 as libc::c_ulonglong) as uint64_t;
                    if ch as libc::c_int == 'H' as i32 {
                        p_state = s_res_or_resp_H;
                        if (*parser).http_errno() as http_errno as libc::c_uint
                            == HPE_OK as libc::c_int as libc::c_uint
                        {} else {
                            __assert_fail(
                                b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                    as *const libc::c_char,
                                b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                                728 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 94],
                                    &[libc::c_char; 94],
                                >(
                                    b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                                ))
                                    .as_ptr(),
                            );
                        };
                        if ((*settings).on_message_begin).is_some() as libc::c_int
                            as libc::c_long != 0
                        {
                            (*parser).set_state(p_state as libc::c_uint);
                            if (0 as libc::c_int
                                != ((*settings).on_message_begin).unwrap()(parser))
                                as libc::c_int as libc::c_long != 0
                            {
                                (*parser)
                                    .set_http_errno(
                                        HPE_CB_message_begin as libc::c_int as libc::c_uint,
                                    );
                            }
                            p_state = (*parser).state() as state;
                            if ((*parser).http_errno() as http_errno as libc::c_uint
                                != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                                as libc::c_long != 0
                            {
                                return (p.offset_from(data) as libc::c_long
                                    + 1 as libc::c_int as libc::c_long) as size_t;
                            }
                        }
                        current_block = 9007357115414505193;
                        break;
                    } else {
                        (*parser)
                            .set_type_0(HTTP_REQUEST as libc::c_int as libc::c_uint);
                        p_state = s_start_req;
                    }
                }
                3 => {
                    if ch as libc::c_int == 'T' as i32 {
                        current_block = 919954187481050311;
                        break;
                    } else {
                        current_block = 10778260831612459202;
                        break;
                    }
                }
                4 => {
                    (*parser).set_flags(0 as libc::c_int as libc::c_uint);
                    (*parser)
                        .content_length = (9223372036854775807 as libc::c_longlong
                        as libc::c_ulonglong)
                        .wrapping_mul(2 as libc::c_ulonglong)
                        .wrapping_add(1 as libc::c_ulonglong) as uint64_t;
                    match ch as libc::c_int {
                        72 => {
                            current_block = 10873520258093198934;
                            break;
                        }
                        13 | 10 => {
                            current_block = 18425699056680496821;
                            break;
                        }
                        _ => {
                            current_block = 10382007559576128104;
                            break;
                        }
                    }
                }
                5 => {
                    if ch as libc::c_int != 'T' as i32 {
                        current_block = 10863493864285401582;
                        break;
                    } else {
                        current_block = 8602574157404971894;
                        break;
                    }
                }
                6 => {
                    if ch as libc::c_int != 'T' as i32 {
                        current_block = 14027225908442187354;
                        break;
                    } else {
                        current_block = 13215501469961642988;
                        break;
                    }
                }
                7 => {
                    if ch as libc::c_int != 'P' as i32 {
                        current_block = 18039443766442739006;
                        break;
                    } else {
                        current_block = 2872334340672008580;
                        break;
                    }
                }
                8 => {
                    if ch as libc::c_int != '/' as i32 {
                        current_block = 15460309861373144675;
                        break;
                    } else {
                        current_block = 9587810615301548814;
                        break;
                    }
                }
                9 => {
                    if !(ch as libc::c_int >= '0' as i32
                        && ch as libc::c_int <= '9' as i32) as libc::c_int
                        as libc::c_long != 0
                    {
                        current_block = 9838996637140935403;
                        break;
                    } else {
                        current_block = 5151888778912688305;
                        break;
                    }
                }
                10 => {
                    if (ch as libc::c_int != '.' as i32) as libc::c_int as libc::c_long
                        != 0
                    {
                        current_block = 10601179871800211547;
                        break;
                    } else {
                        current_block = 5832582820025303349;
                        break;
                    }
                }
                11 => {
                    if !(ch as libc::c_int >= '0' as i32
                        && ch as libc::c_int <= '9' as i32) as libc::c_int
                        as libc::c_long != 0
                    {
                        current_block = 2544535129495155983;
                        break;
                    } else {
                        current_block = 5388205036907793036;
                        break;
                    }
                }
                12 => {
                    if (ch as libc::c_int != ' ' as i32) as libc::c_int as libc::c_long
                        != 0
                    {
                        current_block = 7058451906849221989;
                        break;
                    } else {
                        current_block = 12969817083969514432;
                        break;
                    }
                }
                13 => {
                    if !(ch as libc::c_int >= '0' as i32
                        && ch as libc::c_int <= '9' as i32)
                    {
                        current_block = 9216188846964669005;
                        break;
                    } else {
                        current_block = 3098209481605707636;
                        break;
                    }
                }
                14 => {
                    if !(ch as libc::c_int >= '0' as i32
                        && ch as libc::c_int <= '9' as i32)
                    {
                        match ch as libc::c_int {
                            32 => {
                                p_state = s_res_status_start;
                                current_block = 9007357115414505193;
                                break;
                            }
                            13 | 10 => {
                                p_state = s_res_status_start;
                            }
                            _ => {
                                (*parser)
                                    .set_http_errno(
                                        HPE_INVALID_STATUS as libc::c_int as libc::c_uint,
                                    );
                                current_block = 9020038052434785689;
                                break 's_197;
                            }
                        }
                    } else {
                        (*parser)
                            .set_status_code(
                                (*parser).status_code() * 10 as libc::c_int as libc::c_uint,
                            );
                        (*parser)
                            .set_status_code(
                                (*parser).status_code()
                                    + (ch as libc::c_int - '0' as i32) as libc::c_uint,
                            );
                        if ((*parser).status_code() as libc::c_int > 999 as libc::c_int)
                            as libc::c_int as libc::c_long != 0
                        {
                            current_block = 10194589593280242392;
                            break;
                        } else {
                            current_block = 9007357115414505193;
                            break;
                        }
                    }
                }
                15 => {
                    if status_mark.is_null() {
                        status_mark = p;
                    }
                    p_state = s_res_status;
                    (*parser).set_index(0 as libc::c_int as libc::c_uint);
                    if !(ch as libc::c_int == '\r' as i32
                        || ch as libc::c_int == '\n' as i32)
                    {
                        current_block = 9007357115414505193;
                        break;
                    }
                }
                16 => {
                    if ch as libc::c_int == '\r' as i32 {
                        current_block = 16185292562584120790;
                        break;
                    } else {
                        current_block = 3906822848181906220;
                        break;
                    }
                }
                17 => {
                    if ch as libc::c_int != '\n' as i32 {
                        current_block = 17034918949615525785;
                        break;
                    } else {
                        current_block = 12032176231992402880;
                        break;
                    }
                }
                18 => {
                    if ch as libc::c_int == '\r' as i32
                        || ch as libc::c_int == '\n' as i32
                    {
                        current_block = 9007357115414505193;
                        break;
                    } else {
                        current_block = 8590355382656639939;
                        break;
                    }
                }
                19 => {
                    matcher = 0 as *const libc::c_char;
                    if (ch as libc::c_int == '\0' as i32) as libc::c_int as libc::c_long
                        != 0
                    {
                        current_block = 13543149951580262689;
                        break;
                    } else {
                        current_block = 15775903806355281185;
                        break;
                    }
                }
                20 => {
                    if ch as libc::c_int == ' ' as i32 {
                        current_block = 9007357115414505193;
                        break;
                    } else {
                        current_block = 4961777833250047394;
                        break;
                    }
                }
                21 | 22 | 23 | 24 => {
                    match ch as libc::c_int {
                        32 | 13 | 10 => {
                            current_block = 10304134390680427967;
                            break;
                        }
                        _ => {
                            current_block = 1550381671659267203;
                            break;
                        }
                    }
                }
                25 | 26 | 27 | 28 | 29 | 30 | 31 => {
                    match ch as libc::c_int {
                        32 => {
                            current_block = 2513510894289357104;
                            break;
                        }
                        13 | 10 => {
                            current_block = 6818318202340592218;
                            break;
                        }
                        _ => {
                            current_block = 8911980980495988282;
                            break;
                        }
                    }
                }
                32 => {
                    match ch as libc::c_int {
                        72 => {
                            current_block = 8077860765343440262;
                            break;
                        }
                        32 => {
                            current_block = 9007357115414505193;
                            break;
                        }
                        _ => {
                            current_block = 8221142977249714136;
                            break;
                        }
                    }
                }
                33 => {
                    if ch as libc::c_int != 'T' as i32 {
                        current_block = 768347334755947778;
                        break;
                    } else {
                        current_block = 10059826840140668507;
                        break;
                    }
                }
                34 => {
                    if ch as libc::c_int != 'T' as i32 {
                        current_block = 8155699268269024736;
                        break;
                    } else {
                        current_block = 11888593055720107986;
                        break;
                    }
                }
                35 => {
                    if ch as libc::c_int != 'P' as i32 {
                        current_block = 11814237681575821830;
                        break;
                    } else {
                        current_block = 5561851013817067674;
                        break;
                    }
                }
                36 => {
                    if ch as libc::c_int != '/' as i32 {
                        current_block = 29051446364946957;
                        break;
                    } else {
                        current_block = 3149046158064628027;
                        break;
                    }
                }
                37 => {
                    if !(ch as libc::c_int >= '0' as i32
                        && ch as libc::c_int <= '9' as i32) as libc::c_int
                        as libc::c_long != 0
                    {
                        current_block = 10725100205392175504;
                        break;
                    } else {
                        current_block = 18250077801122006663;
                        break;
                    }
                }
                38 => {
                    if (ch as libc::c_int != '.' as i32) as libc::c_int as libc::c_long
                        != 0
                    {
                        current_block = 16939702945023487982;
                        break;
                    } else {
                        current_block = 5282966931892662513;
                        break;
                    }
                }
                39 => {
                    if !(ch as libc::c_int >= '0' as i32
                        && ch as libc::c_int <= '9' as i32) as libc::c_int
                        as libc::c_long != 0
                    {
                        current_block = 13990052500608980534;
                        break;
                    } else {
                        current_block = 8419401312455247533;
                        break;
                    }
                }
                40 => {
                    if ch as libc::c_int == '\r' as i32 {
                        current_block = 4692175290644916;
                        break;
                    } else {
                        current_block = 6626987748108865425;
                        break;
                    }
                }
                41 => {
                    if (ch as libc::c_int != '\n' as i32) as libc::c_int as libc::c_long
                        != 0
                    {
                        current_block = 15201865807353435331;
                        break;
                    } else {
                        current_block = 3526436446048920899;
                        break;
                    }
                }
                42 => {
                    if ch as libc::c_int == '\r' as i32 {
                        p_state = s_headers_almost_done;
                        current_block = 9007357115414505193;
                        break;
                    } else if ch as libc::c_int == '\n' as i32 {
                        p_state = s_headers_almost_done;
                    } else {
                        c = tokens[ch as libc::c_uchar as usize];
                        if (c == 0) as libc::c_int as libc::c_long != 0 {
                            current_block = 3791882670782541151;
                            break;
                        } else {
                            current_block = 3101959855821210295;
                            break;
                        }
                    }
                }
                43 => {
                    start = p;
                    while p != data.offset(len as isize) {
                        ch = *p;
                        c = tokens[ch as libc::c_uchar as usize];
                        if c == 0 {
                            break;
                        }
                        match (*parser).header_state() as libc::c_int {
                            0 => {}
                            1 => {
                                (*parser).set_index((*parser).index() + 1);
                                (*parser).index();
                                (*parser)
                                    .set_header_state(
                                        (if c as libc::c_int == 'o' as i32 {
                                            h_CO as libc::c_int
                                        } else {
                                            h_general as libc::c_int
                                        }) as libc::c_uint,
                                    );
                            }
                            2 => {
                                (*parser).set_index((*parser).index() + 1);
                                (*parser).index();
                                (*parser)
                                    .set_header_state(
                                        (if c as libc::c_int == 'n' as i32 {
                                            h_CON as libc::c_int
                                        } else {
                                            h_general as libc::c_int
                                        }) as libc::c_uint,
                                    );
                            }
                            3 => {
                                (*parser).set_index((*parser).index() + 1);
                                (*parser).index();
                                match c as libc::c_int {
                                    110 => {
                                        (*parser)
                                            .set_header_state(
                                                h_matching_connection as libc::c_int as libc::c_uint,
                                            );
                                    }
                                    116 => {
                                        (*parser)
                                            .set_header_state(
                                                h_matching_content_length as libc::c_int as libc::c_uint,
                                            );
                                    }
                                    _ => {
                                        (*parser)
                                            .set_header_state(h_general as libc::c_int as libc::c_uint);
                                    }
                                }
                            }
                            4 => {
                                (*parser).set_index((*parser).index() + 1);
                                (*parser).index();
                                if (*parser).index() as libc::c_ulong
                                    > (::std::mem::size_of::<[libc::c_char; 11]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    || c as libc::c_int
                                        != (*::std::mem::transmute::<
                                            &[u8; 11],
                                            &[libc::c_char; 11],
                                        >(b"connection\0"))[(*parser).index() as usize]
                                            as libc::c_int
                                {
                                    (*parser)
                                        .set_header_state(h_general as libc::c_int as libc::c_uint);
                                } else if (*parser).index() as libc::c_ulong
                                    == (::std::mem::size_of::<[libc::c_char; 11]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                {
                                    (*parser)
                                        .set_header_state(
                                            h_connection as libc::c_int as libc::c_uint,
                                        );
                                }
                            }
                            5 => {
                                (*parser).set_index((*parser).index() + 1);
                                (*parser).index();
                                if (*parser).index() as libc::c_ulong
                                    > (::std::mem::size_of::<[libc::c_char; 17]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    || c as libc::c_int
                                        != (*::std::mem::transmute::<
                                            &[u8; 17],
                                            &[libc::c_char; 17],
                                        >(b"proxy-connection\0"))[(*parser).index() as usize]
                                            as libc::c_int
                                {
                                    (*parser)
                                        .set_header_state(h_general as libc::c_int as libc::c_uint);
                                } else if (*parser).index() as libc::c_ulong
                                    == (::std::mem::size_of::<[libc::c_char; 17]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                {
                                    (*parser)
                                        .set_header_state(
                                            h_connection as libc::c_int as libc::c_uint,
                                        );
                                }
                            }
                            6 => {
                                (*parser).set_index((*parser).index() + 1);
                                (*parser).index();
                                if (*parser).index() as libc::c_ulong
                                    > (::std::mem::size_of::<[libc::c_char; 15]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    || c as libc::c_int
                                        != (*::std::mem::transmute::<
                                            &[u8; 15],
                                            &[libc::c_char; 15],
                                        >(b"content-length\0"))[(*parser).index() as usize]
                                            as libc::c_int
                                {
                                    (*parser)
                                        .set_header_state(h_general as libc::c_int as libc::c_uint);
                                } else if (*parser).index() as libc::c_ulong
                                    == (::std::mem::size_of::<[libc::c_char; 15]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                {
                                    (*parser)
                                        .set_header_state(
                                            h_content_length as libc::c_int as libc::c_uint,
                                        );
                                }
                            }
                            7 => {
                                (*parser).set_index((*parser).index() + 1);
                                (*parser).index();
                                if (*parser).index() as libc::c_ulong
                                    > (::std::mem::size_of::<[libc::c_char; 18]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    || c as libc::c_int
                                        != (*::std::mem::transmute::<
                                            &[u8; 18],
                                            &[libc::c_char; 18],
                                        >(b"transfer-encoding\0"))[(*parser).index() as usize]
                                            as libc::c_int
                                {
                                    (*parser)
                                        .set_header_state(h_general as libc::c_int as libc::c_uint);
                                } else if (*parser).index() as libc::c_ulong
                                    == (::std::mem::size_of::<[libc::c_char; 18]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                {
                                    (*parser)
                                        .set_header_state(
                                            h_transfer_encoding as libc::c_int as libc::c_uint,
                                        );
                                }
                            }
                            8 => {
                                (*parser).set_index((*parser).index() + 1);
                                (*parser).index();
                                if (*parser).index() as libc::c_ulong
                                    > (::std::mem::size_of::<[libc::c_char; 8]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    || c as libc::c_int
                                        != (*::std::mem::transmute::<
                                            &[u8; 8],
                                            &[libc::c_char; 8],
                                        >(b"upgrade\0"))[(*parser).index() as usize] as libc::c_int
                                {
                                    (*parser)
                                        .set_header_state(h_general as libc::c_int as libc::c_uint);
                                } else if (*parser).index() as libc::c_ulong
                                    == (::std::mem::size_of::<[libc::c_char; 8]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                {
                                    (*parser)
                                        .set_header_state(h_upgrade as libc::c_int as libc::c_uint);
                                }
                            }
                            9 | 10 | 11 | 12 => {
                                if ch as libc::c_int != ' ' as i32 {
                                    (*parser)
                                        .set_header_state(h_general as libc::c_int as libc::c_uint);
                                }
                            }
                            _ => {
                                if 0 as libc::c_int != 0
                                    && !(b"Unknown header_state\0" as *const u8
                                        as *const libc::c_char)
                                        .is_null()
                                {} else {
                                    __assert_fail(
                                        b"0 && \"Unknown header_state\"\0" as *const u8
                                            as *const libc::c_char,
                                        b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                                        1335 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 94],
                                            &[libc::c_char; 94],
                                        >(
                                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                };
                            }
                        }
                        p = p.offset(1);
                        p;
                    }
                    (*parser)
                        .nread = ((*parser).nread as libc::c_long
                        + p.offset_from(start) as libc::c_long) as uint32_t;
                    if ((*parser).nread
                        > (80 as libc::c_int * 1024 as libc::c_int) as libc::c_uint)
                        as libc::c_int as libc::c_long != 0
                    {
                        current_block = 6874267722315749555;
                        break;
                    } else {
                        current_block = 10410764204934602138;
                        break;
                    }
                }
                44 => {
                    if ch as libc::c_int == ' ' as i32
                        || ch as libc::c_int == '\t' as i32
                    {
                        current_block = 9007357115414505193;
                        break;
                    } else {
                        current_block = 6759408996052215043;
                        break;
                    }
                }
                47 => {
                    current_block = 12021670973791885667;
                    break;
                }
                48 => {
                    start_0 = p;
                    h_state = (*parser).header_state() as header_states;
                    loop {
                        if !(p != data.offset(len as isize)) {
                            current_block = 13818206340797152425;
                            break '_reexecute;
                        }
                        ch = *p;
                        if ch as libc::c_int == '\r' as i32 {
                            p_state = s_header_almost_done;
                            (*parser).set_header_state(h_state as libc::c_uint);
                            if (*parser).http_errno() as http_errno as libc::c_uint
                                == HPE_OK as libc::c_int as libc::c_uint
                            {} else {
                                __assert_fail(
                                    b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                        as *const libc::c_char,
                                    b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                                    1445 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 94],
                                        &[libc::c_char; 94],
                                    >(
                                        b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                                    ))
                                        .as_ptr(),
                                );
                            };
                            if !header_value_mark.is_null() {
                                if ((*settings).on_header_value).is_some() as libc::c_int
                                    as libc::c_long != 0
                                {
                                    (*parser).set_state(p_state as libc::c_uint);
                                    if (0 as libc::c_int
                                        != ((*settings).on_header_value)
                                            .unwrap()(
                                            parser,
                                            header_value_mark,
                                            p.offset_from(header_value_mark) as libc::c_long as size_t,
                                        )) as libc::c_int as libc::c_long != 0
                                    {
                                        (*parser)
                                            .set_http_errno(
                                                HPE_CB_header_value as libc::c_int as libc::c_uint,
                                            );
                                    }
                                    p_state = (*parser).state() as state;
                                    if ((*parser).http_errno() as http_errno as libc::c_uint
                                        != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                                        as libc::c_long != 0
                                    {
                                        return (p.offset_from(data) as libc::c_long
                                            + 1 as libc::c_int as libc::c_long) as size_t;
                                    }
                                }
                                header_value_mark = 0 as *const libc::c_char;
                            }
                            current_block = 13818206340797152425;
                            break '_reexecute;
                        } else if ch as libc::c_int == '\n' as i32 {
                            p_state = s_header_almost_done;
                            (*parser)
                                .nread = ((*parser).nread as libc::c_long
                                + p.offset_from(start_0) as libc::c_long) as uint32_t;
                            if ((*parser).nread
                                > (80 as libc::c_int * 1024 as libc::c_int) as libc::c_uint)
                                as libc::c_int as libc::c_long != 0
                            {
                                (*parser)
                                    .set_http_errno(
                                        HPE_HEADER_OVERFLOW as libc::c_int as libc::c_uint,
                                    );
                                current_block = 9020038052434785689;
                                break 's_197;
                            } else {
                                (*parser).set_header_state(h_state as libc::c_uint);
                                if (*parser).http_errno() as http_errno as libc::c_uint
                                    == HPE_OK as libc::c_int as libc::c_uint
                                {} else {
                                    __assert_fail(
                                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                            as *const libc::c_char,
                                        b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                                        1453 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 94],
                                            &[libc::c_char; 94],
                                        >(
                                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                };
                                if !header_value_mark.is_null() {
                                    if ((*settings).on_header_value).is_some() as libc::c_int
                                        as libc::c_long != 0
                                    {
                                        (*parser).set_state(p_state as libc::c_uint);
                                        if (0 as libc::c_int
                                            != ((*settings).on_header_value)
                                                .unwrap()(
                                                parser,
                                                header_value_mark,
                                                p.offset_from(header_value_mark) as libc::c_long as size_t,
                                            )) as libc::c_int as libc::c_long != 0
                                        {
                                            (*parser)
                                                .set_http_errno(
                                                    HPE_CB_header_value as libc::c_int as libc::c_uint,
                                                );
                                        }
                                        p_state = (*parser).state() as state;
                                        if ((*parser).http_errno() as http_errno as libc::c_uint
                                            != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                                            as libc::c_long != 0
                                        {
                                            return p.offset_from(data) as libc::c_long as size_t;
                                        }
                                    }
                                    header_value_mark = 0 as *const libc::c_char;
                                }
                                break;
                            }
                        } else if lenient == 0
                            && !(ch as libc::c_int == '\r' as i32
                                || ch as libc::c_int == '\n' as i32
                                || ch as libc::c_int == 9 as libc::c_int
                                || ch as libc::c_uchar as libc::c_int > 31 as libc::c_int
                                    && ch as libc::c_int != 127 as libc::c_int)
                        {
                            (*parser)
                                .set_http_errno(
                                    HPE_INVALID_HEADER_TOKEN as libc::c_int as libc::c_uint,
                                );
                            current_block = 9020038052434785689;
                            break 's_197;
                        } else {
                            c = (ch as libc::c_int | 0x20 as libc::c_int)
                                as libc::c_uchar as libc::c_char;
                            match h_state as libc::c_uint {
                                0 => {
                                    let mut p_cr: *const libc::c_char = 0
                                        as *const libc::c_char;
                                    let mut p_lf: *const libc::c_char = 0
                                        as *const libc::c_char;
                                    let mut limit: size_t = data
                                        .offset(len as isize)
                                        .offset_from(p) as libc::c_long as size_t;
                                    limit = if limit
                                        < (80 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
                                    {
                                        limit
                                    } else {
                                        (80 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
                                    };
                                    p_cr = memchr(p as *const libc::c_void, '\r' as i32, limit)
                                        as *const libc::c_char;
                                    p_lf = memchr(p as *const libc::c_void, '\n' as i32, limit)
                                        as *const libc::c_char;
                                    if !p_cr.is_null() {
                                        if !p_lf.is_null() && p_cr >= p_lf {
                                            p = p_lf;
                                        } else {
                                            p = p_cr;
                                        }
                                    } else if !p_lf.is_null() as libc::c_int as libc::c_long
                                        != 0
                                    {
                                        p = p_lf;
                                    } else {
                                        p = data.offset(len as isize);
                                    }
                                    p = p.offset(-1);
                                    p;
                                }
                                9 | 11 => {
                                    if 0 as libc::c_int != 0
                                        && !(b"Shouldn't get here.\0" as *const u8
                                            as *const libc::c_char)
                                            .is_null()
                                    {} else {
                                        __assert_fail(
                                            b"0 && \"Shouldn't get here.\"\0" as *const u8
                                                as *const libc::c_char,
                                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                                            1492 as libc::c_int as libc::c_uint,
                                            (*::std::mem::transmute::<
                                                &[u8; 94],
                                                &[libc::c_char; 94],
                                            >(
                                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    };
                                }
                                10 => {
                                    let mut t: uint64_t = 0;
                                    if !(ch as libc::c_int == ' ' as i32) {
                                        if !(ch as libc::c_int >= '0' as i32
                                            && ch as libc::c_int <= '9' as i32) as libc::c_int
                                            as libc::c_long != 0
                                        {
                                            (*parser)
                                                .set_http_errno(
                                                    HPE_INVALID_CONTENT_LENGTH as libc::c_int as libc::c_uint,
                                                );
                                            (*parser).set_header_state(h_state as libc::c_uint);
                                            current_block = 9020038052434785689;
                                            break 's_197;
                                        } else {
                                            t = (*parser).content_length;
                                            t = (t as libc::c_ulong)
                                                .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                                                as uint64_t as uint64_t;
                                            t = (t as libc::c_ulong)
                                                .wrapping_add(
                                                    (ch as libc::c_int - '0' as i32) as libc::c_ulong,
                                                ) as uint64_t as uint64_t;
                                            if ((9223372036854775807 as libc::c_longlong
                                                as libc::c_ulonglong)
                                                .wrapping_mul(2 as libc::c_ulonglong)
                                                .wrapping_add(1 as libc::c_ulonglong)
                                                .wrapping_sub(10 as libc::c_int as libc::c_ulonglong)
                                                .wrapping_div(10 as libc::c_int as libc::c_ulonglong)
                                                < (*parser).content_length as libc::c_ulonglong)
                                                as libc::c_int as libc::c_long != 0
                                            {
                                                (*parser)
                                                    .set_http_errno(
                                                        HPE_INVALID_CONTENT_LENGTH as libc::c_int as libc::c_uint,
                                                    );
                                                (*parser).set_header_state(h_state as libc::c_uint);
                                                current_block = 9020038052434785689;
                                                break 's_197;
                                            } else {
                                                (*parser).content_length = t;
                                            }
                                        }
                                    }
                                }
                                13 => {
                                    (*parser).set_index((*parser).index() + 1);
                                    (*parser).index();
                                    if (*parser).index() as libc::c_ulong
                                        > (::std::mem::size_of::<[libc::c_char; 8]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 8],
                                                &[libc::c_char; 8],
                                            >(b"chunked\0"))[(*parser).index() as usize] as libc::c_int
                                    {
                                        h_state = h_general;
                                    } else if (*parser).index() as libc::c_ulong
                                        == (::std::mem::size_of::<[libc::c_char; 8]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        h_state = h_transfer_encoding_chunked;
                                    }
                                }
                                14 => {
                                    if c as libc::c_int == 'k' as i32 {
                                        h_state = h_matching_connection_keep_alive;
                                    } else if c as libc::c_int == 'c' as i32 {
                                        h_state = h_matching_connection_close;
                                    } else if c as libc::c_int == 'u' as i32 {
                                        h_state = h_matching_connection_upgrade;
                                    } else if tokens[c as libc::c_uchar as usize] != 0 {
                                        h_state = h_matching_connection_token;
                                    } else if !(c as libc::c_int == ' ' as i32
                                        || c as libc::c_int == '\t' as i32)
                                    {
                                        h_state = h_general;
                                    }
                                }
                                15 => {
                                    (*parser).set_index((*parser).index() + 1);
                                    (*parser).index();
                                    if (*parser).index() as libc::c_ulong
                                        > (::std::mem::size_of::<[libc::c_char; 11]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 11],
                                                &[libc::c_char; 11],
                                            >(b"keep-alive\0"))[(*parser).index() as usize]
                                                as libc::c_int
                                    {
                                        h_state = h_matching_connection_token;
                                    } else if (*parser).index() as libc::c_ulong
                                        == (::std::mem::size_of::<[libc::c_char; 11]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        h_state = h_connection_keep_alive;
                                    }
                                }
                                16 => {
                                    (*parser).set_index((*parser).index() + 1);
                                    (*parser).index();
                                    if (*parser).index() as libc::c_ulong
                                        > (::std::mem::size_of::<[libc::c_char; 6]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 6],
                                                &[libc::c_char; 6],
                                            >(b"close\0"))[(*parser).index() as usize] as libc::c_int
                                    {
                                        h_state = h_matching_connection_token;
                                    } else if (*parser).index() as libc::c_ulong
                                        == (::std::mem::size_of::<[libc::c_char; 6]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        h_state = h_connection_close;
                                    }
                                }
                                17 => {
                                    (*parser).set_index((*parser).index() + 1);
                                    (*parser).index();
                                    if (*parser).index() as libc::c_ulong
                                        > (::std::mem::size_of::<[libc::c_char; 8]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        || c as libc::c_int
                                            != (*::std::mem::transmute::<
                                                &[u8; 8],
                                                &[libc::c_char; 8],
                                            >(b"upgrade\0"))[(*parser).index() as usize] as libc::c_int
                                    {
                                        h_state = h_matching_connection_token;
                                    } else if (*parser).index() as libc::c_ulong
                                        == (::std::mem::size_of::<[libc::c_char; 8]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    {
                                        h_state = h_connection_upgrade;
                                    }
                                }
                                18 => {
                                    if ch as libc::c_int == ',' as i32 {
                                        h_state = h_matching_connection_token_start;
                                        (*parser).set_index(0 as libc::c_int as libc::c_uint);
                                    }
                                }
                                19 => {
                                    if ch as libc::c_int != ' ' as i32 {
                                        h_state = h_general;
                                    }
                                }
                                20 | 21 | 22 => {
                                    if ch as libc::c_int == ',' as i32 {
                                        if h_state as libc::c_uint
                                            == h_connection_keep_alive as libc::c_int as libc::c_uint
                                        {
                                            (*parser)
                                                .set_flags(
                                                    (*parser).flags()
                                                        | F_CONNECTION_KEEP_ALIVE as libc::c_int as libc::c_uint,
                                                );
                                        } else if h_state as libc::c_uint
                                            == h_connection_close as libc::c_int as libc::c_uint
                                        {
                                            (*parser)
                                                .set_flags(
                                                    (*parser).flags()
                                                        | F_CONNECTION_CLOSE as libc::c_int as libc::c_uint,
                                                );
                                        } else if h_state as libc::c_uint
                                            == h_connection_upgrade as libc::c_int as libc::c_uint
                                        {
                                            (*parser)
                                                .set_flags(
                                                    (*parser).flags()
                                                        | F_CONNECTION_UPGRADE as libc::c_int as libc::c_uint,
                                                );
                                        }
                                        h_state = h_matching_connection_token_start;
                                        (*parser).set_index(0 as libc::c_int as libc::c_uint);
                                    } else if ch as libc::c_int != ' ' as i32 {
                                        h_state = h_matching_connection_token;
                                    }
                                }
                                _ => {
                                    p_state = s_header_value;
                                    h_state = h_general;
                                }
                            }
                            p = p.offset(1);
                            p;
                        }
                    }
                }
                50 => {
                    if (ch as libc::c_int != '\n' as i32) as libc::c_int as libc::c_long
                        != 0
                    {
                        current_block = 1781577627025608192;
                        break;
                    } else {
                        current_block = 8062549956965633527;
                        break;
                    }
                }
                49 => {
                    if ch as libc::c_int == ' ' as i32
                        || ch as libc::c_int == '\t' as i32
                    {
                        p_state = s_header_value_start;
                    } else {
                        match (*parser).header_state() as libc::c_int {
                            20 => {
                                (*parser)
                                    .set_flags(
                                        (*parser).flags()
                                            | F_CONNECTION_KEEP_ALIVE as libc::c_int as libc::c_uint,
                                    );
                            }
                            21 => {
                                (*parser)
                                    .set_flags(
                                        (*parser).flags()
                                            | F_CONNECTION_CLOSE as libc::c_int as libc::c_uint,
                                    );
                            }
                            19 => {
                                (*parser)
                                    .set_flags(
                                        (*parser).flags() | F_CHUNKED as libc::c_int as libc::c_uint,
                                    );
                            }
                            22 => {
                                (*parser)
                                    .set_flags(
                                        (*parser).flags()
                                            | F_CONNECTION_UPGRADE as libc::c_int as libc::c_uint,
                                    );
                            }
                            _ => {}
                        }
                        p_state = s_header_field_start;
                    }
                }
                45 => {
                    if ch as libc::c_int != '\n' as i32 {
                        current_block = 5407058732526620246;
                        break;
                    } else {
                        current_block = 9725360004668457823;
                        break;
                    }
                }
                46 => {
                    if ch as libc::c_int == ' ' as i32
                        || ch as libc::c_int == '\t' as i32
                    {
                        p_state = s_header_value_discard_ws;
                        current_block = 9007357115414505193;
                        break;
                    } else {
                        match (*parser).header_state() as libc::c_int {
                            20 => {
                                (*parser)
                                    .set_flags(
                                        (*parser).flags()
                                            | F_CONNECTION_KEEP_ALIVE as libc::c_int as libc::c_uint,
                                    );
                            }
                            21 => {
                                (*parser)
                                    .set_flags(
                                        (*parser).flags()
                                            | F_CONNECTION_CLOSE as libc::c_int as libc::c_uint,
                                    );
                            }
                            22 => {
                                (*parser)
                                    .set_flags(
                                        (*parser).flags()
                                            | F_CONNECTION_UPGRADE as libc::c_int as libc::c_uint,
                                    );
                            }
                            19 => {
                                (*parser)
                                    .set_flags(
                                        (*parser).flags() | F_CHUNKED as libc::c_int as libc::c_uint,
                                    );
                            }
                            _ => {}
                        }
                        if header_value_mark.is_null() {
                            header_value_mark = p;
                        }
                        p_state = s_header_field_start;
                        if (*parser).http_errno() as http_errno as libc::c_uint
                            == HPE_OK as libc::c_int as libc::c_uint
                        {} else {
                            __assert_fail(
                                b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                    as *const libc::c_char,
                                b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                                1700 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 94],
                                    &[libc::c_char; 94],
                                >(
                                    b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                                ))
                                    .as_ptr(),
                            );
                        };
                        if !header_value_mark.is_null() {
                            if ((*settings).on_header_value).is_some() as libc::c_int
                                as libc::c_long != 0
                            {
                                (*parser).set_state(p_state as libc::c_uint);
                                if (0 as libc::c_int
                                    != ((*settings).on_header_value)
                                        .unwrap()(
                                        parser,
                                        header_value_mark,
                                        p.offset_from(header_value_mark) as libc::c_long as size_t,
                                    )) as libc::c_int as libc::c_long != 0
                                {
                                    (*parser)
                                        .set_http_errno(
                                            HPE_CB_header_value as libc::c_int as libc::c_uint,
                                        );
                                }
                                p_state = (*parser).state() as state;
                                if ((*parser).http_errno() as http_errno as libc::c_uint
                                    != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                                    as libc::c_long != 0
                                {
                                    return p.offset_from(data) as libc::c_long as size_t;
                                }
                            }
                            header_value_mark = 0 as *const libc::c_char;
                        }
                    }
                }
                55 => {
                    if ch as libc::c_int != '\n' as i32 {
                        (*parser)
                            .set_http_errno(HPE_STRICT as libc::c_int as libc::c_uint);
                        current_block = 9020038052434785689;
                        break 's_197;
                    } else if (*parser).flags() as libc::c_int
                        & F_TRAILING as libc::c_int != 0
                    {
                        p_state = s_message_done;
                        if (*parser).http_errno() as http_errno as libc::c_uint
                            == HPE_OK as libc::c_int as libc::c_uint
                        {} else {
                            __assert_fail(
                                b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                    as *const libc::c_char,
                                b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                                1712 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 94],
                                    &[libc::c_char; 94],
                                >(
                                    b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                                ))
                                    .as_ptr(),
                            );
                        };
                        if ((*settings).on_chunk_complete).is_some() as libc::c_int
                            as libc::c_long != 0
                        {
                            (*parser).set_state(p_state as libc::c_uint);
                            if (0 as libc::c_int
                                != ((*settings).on_chunk_complete).unwrap()(parser))
                                as libc::c_int as libc::c_long != 0
                            {
                                (*parser)
                                    .set_http_errno(
                                        HPE_CB_chunk_complete as libc::c_int as libc::c_uint,
                                    );
                            }
                            p_state = (*parser).state() as state;
                            if ((*parser).http_errno() as http_errno as libc::c_uint
                                != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                                as libc::c_long != 0
                            {
                                return p.offset_from(data) as libc::c_long as size_t;
                            }
                        }
                    } else if (*parser).flags() as libc::c_int & F_CHUNKED as libc::c_int
                        != 0
                        && (*parser).flags() as libc::c_int
                            & F_CONTENTLENGTH as libc::c_int != 0
                    {
                        (*parser)
                            .set_http_errno(
                                HPE_UNEXPECTED_CONTENT_LENGTH as libc::c_int as libc::c_uint,
                            );
                        current_block = 9020038052434785689;
                        break 's_197;
                    } else {
                        p_state = s_headers_done;
                        if (*parser).flags() as libc::c_int & F_UPGRADE as libc::c_int
                            != 0
                            && (*parser).flags() as libc::c_int
                                & F_CONNECTION_UPGRADE as libc::c_int != 0
                        {
                            (*parser)
                                .set_upgrade(
                                    ((*parser).type_0() as libc::c_int
                                        == HTTP_REQUEST as libc::c_int
                                        || (*parser).status_code() as libc::c_int
                                            == 101 as libc::c_int) as libc::c_int as libc::c_uint,
                                );
                        } else {
                            (*parser)
                                .set_upgrade(
                                    ((*parser).method() as libc::c_int
                                        == HTTP_CONNECT as libc::c_int) as libc::c_int
                                        as libc::c_uint,
                                );
                        }
                        if ((*settings).on_headers_complete).is_some() {
                            let mut current_block_819: u64;
                            match ((*settings).on_headers_complete).unwrap()(parser) {
                                0 => {
                                    current_block_819 = 129348190070600045;
                                }
                                2 => {
                                    (*parser).set_upgrade(1 as libc::c_int as libc::c_uint);
                                    current_block_819 = 8982081673687713566;
                                }
                                1 => {
                                    current_block_819 = 8982081673687713566;
                                }
                                _ => {
                                    (*parser)
                                        .set_http_errno(
                                            HPE_CB_headers_complete as libc::c_int as libc::c_uint,
                                        );
                                    (*parser).set_state(p_state as libc::c_uint);
                                    return p.offset_from(data) as libc::c_long as size_t;
                                }
                            }
                            match current_block_819 {
                                8982081673687713566 => {
                                    (*parser)
                                        .set_flags(
                                            (*parser).flags()
                                                | F_SKIPBODY as libc::c_int as libc::c_uint,
                                        );
                                }
                                _ => {}
                            }
                        }
                        if (*parser).http_errno() as http_errno as libc::c_uint
                            != HPE_OK as libc::c_int as libc::c_uint
                        {
                            (*parser).set_state(p_state as libc::c_uint);
                            return p.offset_from(data) as libc::c_long as size_t;
                        }
                    }
                }
                56 => {
                    hasBody = 0;
                    if ch as libc::c_int != '\n' as i32 {
                        current_block = 9580941621949456922;
                        break;
                    } else {
                        current_block = 3733147086002097443;
                        break;
                    }
                }
                60 => {
                    let mut to_read: uint64_t = if (*parser).content_length
                        < data.offset(len as isize).offset_from(p) as libc::c_long
                            as uint64_t
                    {
                        (*parser).content_length
                    } else {
                        data.offset(len as isize).offset_from(p) as libc::c_long
                            as uint64_t
                    };
                    if (*parser).content_length != 0 as libc::c_int as libc::c_ulong
                        && (*parser).content_length as libc::c_ulonglong
                            != (9223372036854775807 as libc::c_longlong
                                as libc::c_ulonglong)
                                .wrapping_mul(2 as libc::c_ulonglong)
                                .wrapping_add(1 as libc::c_ulonglong)
                    {} else {
                        __assert_fail(
                            b"parser->content_length != 0 && parser->content_length != ULLONG_MAX\0"
                                as *const u8 as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1826 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 94],
                                &[libc::c_char; 94],
                            >(
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    if body_mark.is_null() {
                        body_mark = p;
                    }
                    (*parser)
                        .content_length = ((*parser).content_length as libc::c_ulong)
                        .wrapping_sub(to_read) as uint64_t as uint64_t;
                    p = p
                        .offset(
                            to_read.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        );
                    if !((*parser).content_length == 0 as libc::c_int as libc::c_ulong) {
                        current_block = 9007357115414505193;
                        break;
                    }
                    p_state = s_message_done;
                    if (*parser).http_errno() as http_errno as libc::c_uint
                        == HPE_OK as libc::c_int as libc::c_uint
                    {} else {
                        __assert_fail(
                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1849 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 94],
                                &[libc::c_char; 94],
                            >(
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    if !body_mark.is_null() {
                        if ((*settings).on_body).is_some() as libc::c_int as libc::c_long
                            != 0
                        {
                            (*parser).set_state(p_state as libc::c_uint);
                            if (0 as libc::c_int
                                != ((*settings).on_body)
                                    .unwrap()(
                                    parser,
                                    body_mark,
                                    (p.offset_from(body_mark) as libc::c_long
                                        + 1 as libc::c_int as libc::c_long) as size_t,
                                )) as libc::c_int as libc::c_long != 0
                            {
                                (*parser)
                                    .set_http_errno(HPE_CB_body as libc::c_int as libc::c_uint);
                            }
                            p_state = (*parser).state() as state;
                            if ((*parser).http_errno() as http_errno as libc::c_uint
                                != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                                as libc::c_long != 0
                            {
                                return p.offset_from(data) as libc::c_long as size_t;
                            }
                        }
                        body_mark = 0 as *const libc::c_char;
                    }
                }
                61 => {
                    if body_mark.is_null() {
                        body_mark = p;
                    }
                    p = data.offset(len as isize).offset(-(1 as libc::c_int as isize));
                    current_block = 9007357115414505193;
                    break;
                }
                62 => {
                    p_state = (if http_should_keep_alive(parser) != 0 {
                        if (*parser).type_0() as libc::c_int
                            == HTTP_REQUEST as libc::c_int
                        {
                            s_start_req as libc::c_int
                        } else {
                            s_start_res as libc::c_int
                        }
                    } else {
                        s_dead as libc::c_int
                    }) as state;
                    if (*parser).http_errno() as http_errno as libc::c_uint
                        == HPE_OK as libc::c_int as libc::c_uint
                    {} else {
                        __assert_fail(
                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1865 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 94],
                                &[libc::c_char; 94],
                            >(
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    if ((*settings).on_message_complete).is_some() as libc::c_int
                        as libc::c_long != 0
                    {
                        (*parser).set_state(p_state as libc::c_uint);
                        if (0 as libc::c_int
                            != ((*settings).on_message_complete).unwrap()(parser))
                            as libc::c_int as libc::c_long != 0
                        {
                            (*parser)
                                .set_http_errno(
                                    HPE_CB_message_complete as libc::c_int as libc::c_uint,
                                );
                        }
                        p_state = (*parser).state() as state;
                        if ((*parser).http_errno() as http_errno as libc::c_uint
                            != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                            as libc::c_long != 0
                        {
                            return (p.offset_from(data) as libc::c_long
                                + 1 as libc::c_int as libc::c_long) as size_t;
                        }
                    }
                    if (*parser).upgrade() != 0 {
                        (*parser).set_state(p_state as libc::c_uint);
                        return (p.offset_from(data) as libc::c_long
                            + 1 as libc::c_int as libc::c_long) as size_t;
                    }
                    current_block = 9007357115414505193;
                    break;
                }
                51 => {
                    if (*parser).nread == 1 as libc::c_int as libc::c_uint {} else {
                        __assert_fail(
                            b"parser->nread == 1\0" as *const u8 as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1874 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 94],
                                &[libc::c_char; 94],
                            >(
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    if (*parser).flags() as libc::c_int & F_CHUNKED as libc::c_int != 0
                    {} else {
                        __assert_fail(
                            b"parser->flags & F_CHUNKED\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1875 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 94],
                                &[libc::c_char; 94],
                            >(
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    unhex_val = unhex[ch as libc::c_uchar as usize];
                    if (unhex_val as libc::c_int == -(1 as libc::c_int)) as libc::c_int
                        as libc::c_long != 0
                    {
                        current_block = 12940301851547707723;
                        break;
                    } else {
                        current_block = 14235270916792176347;
                        break;
                    }
                }
                52 => {
                    t_0 = 0;
                    if (*parser).flags() as libc::c_int & F_CHUNKED as libc::c_int != 0
                    {} else {
                        __assert_fail(
                            b"parser->flags & F_CHUNKED\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1892 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 94],
                                &[libc::c_char; 94],
                            >(
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    if ch as libc::c_int == '\r' as i32 {
                        current_block = 6829280784076477810;
                        break;
                    } else {
                        current_block = 11673179503852012964;
                        break;
                    }
                }
                53 => {
                    if (*parser).flags() as libc::c_int & F_CHUNKED as libc::c_int != 0
                    {} else {
                        __assert_fail(
                            b"parser->flags & F_CHUNKED\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1927 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 94],
                                &[libc::c_char; 94],
                            >(
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    if ch as libc::c_int == '\r' as i32 {
                        current_block = 8758648760486203175;
                        break;
                    } else {
                        current_block = 9007357115414505193;
                        break;
                    }
                }
                54 => {
                    if (*parser).flags() as libc::c_int & F_CHUNKED as libc::c_int != 0
                    {} else {
                        __assert_fail(
                            b"parser->flags & F_CHUNKED\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1938 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 94],
                                &[libc::c_char; 94],
                            >(
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    if ch as libc::c_int != '\n' as i32 {
                        current_block = 9611983732549257940;
                        break;
                    } else {
                        current_block = 16519603272705762099;
                        break;
                    }
                }
                57 => {
                    let mut to_read_0: uint64_t = if (*parser).content_length
                        < data.offset(len as isize).offset_from(p) as libc::c_long
                            as uint64_t
                    {
                        (*parser).content_length
                    } else {
                        data.offset(len as isize).offset_from(p) as libc::c_long
                            as uint64_t
                    };
                    if (*parser).flags() as libc::c_int & F_CHUNKED as libc::c_int != 0
                    {} else {
                        __assert_fail(
                            b"parser->flags & F_CHUNKED\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1958 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 94],
                                &[libc::c_char; 94],
                            >(
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    if (*parser).content_length != 0 as libc::c_int as libc::c_ulong
                        && (*parser).content_length as libc::c_ulonglong
                            != (9223372036854775807 as libc::c_longlong
                                as libc::c_ulonglong)
                                .wrapping_mul(2 as libc::c_ulonglong)
                                .wrapping_add(1 as libc::c_ulonglong)
                    {} else {
                        __assert_fail(
                            b"parser->content_length != 0 && parser->content_length != ULLONG_MAX\0"
                                as *const u8 as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1960 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 94],
                                &[libc::c_char; 94],
                            >(
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    if body_mark.is_null() {
                        body_mark = p;
                    }
                    (*parser)
                        .content_length = ((*parser).content_length as libc::c_ulong)
                        .wrapping_sub(to_read_0) as uint64_t as uint64_t;
                    p = p
                        .offset(
                            to_read_0.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        );
                    if (*parser).content_length == 0 as libc::c_int as libc::c_ulong {
                        p_state = s_chunk_data_almost_done;
                    }
                    current_block = 9007357115414505193;
                    break;
                }
                58 => {
                    if (*parser).flags() as libc::c_int & F_CHUNKED as libc::c_int != 0
                    {} else {
                        __assert_fail(
                            b"parser->flags & F_CHUNKED\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1977 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 94],
                                &[libc::c_char; 94],
                            >(
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    if (*parser).content_length == 0 as libc::c_int as libc::c_ulong
                    {} else {
                        __assert_fail(
                            b"parser->content_length == 0\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1978 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 94],
                                &[libc::c_char; 94],
                            >(
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    if ch as libc::c_int != '\r' as i32 {
                        current_block = 5286729433347123073;
                        break;
                    } else {
                        current_block = 4542590401787960760;
                        break;
                    }
                }
                59 => {
                    if (*parser).flags() as libc::c_int & F_CHUNKED as libc::c_int != 0
                    {} else {
                        __assert_fail(
                            b"parser->flags & F_CHUNKED\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1985 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 94],
                                &[libc::c_char; 94],
                            >(
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    if ch as libc::c_int != '\n' as i32 {
                        current_block = 12761086676529082117;
                        break;
                    } else {
                        current_block = 17424578316536045879;
                        break;
                    }
                }
                _ => {
                    if 0 as libc::c_int != 0
                        && !(b"unhandled state\0" as *const u8 as *const libc::c_char)
                            .is_null()
                    {} else {
                        __assert_fail(
                            b"0 && \"unhandled state\"\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1993 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 94],
                                &[libc::c_char; 94],
                            >(
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    (*parser)
                        .set_http_errno(
                            HPE_INVALID_INTERNAL_STATE as libc::c_int as libc::c_uint,
                        );
                    current_block = 9020038052434785689;
                    break 's_197;
                }
            }
        }
        match current_block {
            14235270916792176347 => {
                (*parser).content_length = unhex_val as uint64_t;
                p_state = s_chunk_size;
                current_block = 9007357115414505193;
            }
            3733147086002097443 => {
                (*parser).nread = 0 as libc::c_int as uint32_t;
                hasBody = ((*parser).flags() as libc::c_int & F_CHUNKED as libc::c_int
                    != 0
                    || (*parser).content_length > 0 as libc::c_int as libc::c_ulong
                        && (*parser).content_length as libc::c_ulonglong
                            != (9223372036854775807 as libc::c_longlong
                                as libc::c_ulonglong)
                                .wrapping_mul(2 as libc::c_ulonglong)
                                .wrapping_add(1 as libc::c_ulonglong)) as libc::c_int;
                if (*parser).upgrade() as libc::c_int != 0
                    && ((*parser).method() as libc::c_int == HTTP_CONNECT as libc::c_int
                        || (*parser).flags() as libc::c_int & F_SKIPBODY as libc::c_int
                            != 0 || hasBody == 0)
                {
                    p_state = (if http_should_keep_alive(parser) != 0 {
                        if (*parser).type_0() as libc::c_int
                            == HTTP_REQUEST as libc::c_int
                        {
                            s_start_req as libc::c_int
                        } else {
                            s_start_res as libc::c_int
                        }
                    } else {
                        s_dead as libc::c_int
                    }) as state;
                    if (*parser).http_errno() as http_errno as libc::c_uint
                        == HPE_OK as libc::c_int as libc::c_uint
                    {} else {
                        __assert_fail(
                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1787 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 94],
                                &[libc::c_char; 94],
                            >(
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    if ((*settings).on_message_complete).is_some() as libc::c_int
                        as libc::c_long != 0
                    {
                        (*parser).set_state(p_state as libc::c_uint);
                        if (0 as libc::c_int
                            != ((*settings).on_message_complete).unwrap()(parser))
                            as libc::c_int as libc::c_long != 0
                        {
                            (*parser)
                                .set_http_errno(
                                    HPE_CB_message_complete as libc::c_int as libc::c_uint,
                                );
                        }
                        p_state = (*parser).state() as state;
                        if ((*parser).http_errno() as http_errno as libc::c_uint
                            != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                            as libc::c_long != 0
                        {
                            return (p.offset_from(data) as libc::c_long
                                + 1 as libc::c_int as libc::c_long) as size_t;
                        }
                    }
                    (*parser).set_state(p_state as libc::c_uint);
                    return (p.offset_from(data) as libc::c_long
                        + 1 as libc::c_int as libc::c_long) as size_t;
                }
                if (*parser).flags() as libc::c_int & F_SKIPBODY as libc::c_int != 0 {
                    p_state = (if http_should_keep_alive(parser) != 0 {
                        if (*parser).type_0() as libc::c_int
                            == HTTP_REQUEST as libc::c_int
                        {
                            s_start_req as libc::c_int
                        } else {
                            s_start_res as libc::c_int
                        }
                    } else {
                        s_dead as libc::c_int
                    }) as state;
                    if (*parser).http_errno() as http_errno as libc::c_uint
                        == HPE_OK as libc::c_int as libc::c_uint
                    {} else {
                        __assert_fail(
                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1793 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 94],
                                &[libc::c_char; 94],
                            >(
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    if ((*settings).on_message_complete).is_some() as libc::c_int
                        as libc::c_long != 0
                    {
                        (*parser).set_state(p_state as libc::c_uint);
                        if (0 as libc::c_int
                            != ((*settings).on_message_complete).unwrap()(parser))
                            as libc::c_int as libc::c_long != 0
                        {
                            (*parser)
                                .set_http_errno(
                                    HPE_CB_message_complete as libc::c_int as libc::c_uint,
                                );
                        }
                        p_state = (*parser).state() as state;
                        if ((*parser).http_errno() as http_errno as libc::c_uint
                            != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                            as libc::c_long != 0
                        {
                            return (p.offset_from(data) as libc::c_long
                                + 1 as libc::c_int as libc::c_long) as size_t;
                        }
                    }
                } else if (*parser).flags() as libc::c_int & F_CHUNKED as libc::c_int
                    != 0
                {
                    p_state = s_chunk_size_start;
                } else if (*parser).content_length == 0 as libc::c_int as libc::c_ulong {
                    p_state = (if http_should_keep_alive(parser) != 0 {
                        if (*parser).type_0() as libc::c_int
                            == HTTP_REQUEST as libc::c_int
                        {
                            s_start_req as libc::c_int
                        } else {
                            s_start_res as libc::c_int
                        }
                    } else {
                        s_dead as libc::c_int
                    }) as state;
                    if (*parser).http_errno() as http_errno as libc::c_uint
                        == HPE_OK as libc::c_int as libc::c_uint
                    {} else {
                        __assert_fail(
                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1801 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 94],
                                &[libc::c_char; 94],
                            >(
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    if ((*settings).on_message_complete).is_some() as libc::c_int
                        as libc::c_long != 0
                    {
                        (*parser).set_state(p_state as libc::c_uint);
                        if (0 as libc::c_int
                            != ((*settings).on_message_complete).unwrap()(parser))
                            as libc::c_int as libc::c_long != 0
                        {
                            (*parser)
                                .set_http_errno(
                                    HPE_CB_message_complete as libc::c_int as libc::c_uint,
                                );
                        }
                        p_state = (*parser).state() as state;
                        if ((*parser).http_errno() as http_errno as libc::c_uint
                            != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                            as libc::c_long != 0
                        {
                            return (p.offset_from(data) as libc::c_long
                                + 1 as libc::c_int as libc::c_long) as size_t;
                        }
                    }
                } else if (*parser).content_length as libc::c_ulonglong
                    != (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                        .wrapping_mul(2 as libc::c_ulonglong)
                        .wrapping_add(1 as libc::c_ulonglong)
                {
                    p_state = s_body_identity;
                } else if http_message_needs_eof(parser) == 0 {
                    p_state = (if http_should_keep_alive(parser) != 0 {
                        if (*parser).type_0() as libc::c_int
                            == HTTP_REQUEST as libc::c_int
                        {
                            s_start_req as libc::c_int
                        } else {
                            s_start_res as libc::c_int
                        }
                    } else {
                        s_dead as libc::c_int
                    }) as state;
                    if (*parser).http_errno() as http_errno as libc::c_uint
                        == HPE_OK as libc::c_int as libc::c_uint
                    {} else {
                        __assert_fail(
                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1809 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 94],
                                &[libc::c_char; 94],
                            >(
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    if ((*settings).on_message_complete).is_some() as libc::c_int
                        as libc::c_long != 0
                    {
                        (*parser).set_state(p_state as libc::c_uint);
                        if (0 as libc::c_int
                            != ((*settings).on_message_complete).unwrap()(parser))
                            as libc::c_int as libc::c_long != 0
                        {
                            (*parser)
                                .set_http_errno(
                                    HPE_CB_message_complete as libc::c_int as libc::c_uint,
                                );
                        }
                        p_state = (*parser).state() as state;
                        if ((*parser).http_errno() as http_errno as libc::c_uint
                            != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                            as libc::c_long != 0
                        {
                            return (p.offset_from(data) as libc::c_long
                                + 1 as libc::c_int as libc::c_long) as size_t;
                        }
                    }
                } else {
                    p_state = s_body_identity_eof;
                }
                current_block = 9007357115414505193;
            }
            13818206340797152425 => {
                (*parser).set_header_state(h_state as libc::c_uint);
                (*parser)
                    .nread = ((*parser).nread as libc::c_long
                    + p.offset_from(start_0) as libc::c_long) as uint32_t;
                if ((*parser).nread
                    > (80 as libc::c_int * 1024 as libc::c_int) as libc::c_uint)
                    as libc::c_int as libc::c_long != 0
                {
                    (*parser)
                        .set_http_errno(
                            HPE_HEADER_OVERFLOW as libc::c_int as libc::c_uint,
                        );
                    current_block = 9020038052434785689;
                    break;
                } else if p == data.offset(len as isize) {
                    p = p.offset(-1);
                    p;
                }
                current_block = 9007357115414505193;
            }
            11673179503852012964 => {
                unhex_val = unhex[ch as libc::c_uchar as usize];
                if unhex_val as libc::c_int == -(1 as libc::c_int) {
                    if ch as libc::c_int == ';' as i32 || ch as libc::c_int == ' ' as i32
                    {
                        p_state = s_chunk_parameters;
                    } else {
                        (*parser)
                            .set_http_errno(
                                HPE_INVALID_CHUNK_SIZE as libc::c_int as libc::c_uint,
                            );
                        current_block = 9020038052434785689;
                        break;
                    }
                } else {
                    t_0 = (*parser).content_length;
                    t_0 = (t_0 as libc::c_ulong)
                        .wrapping_mul(16 as libc::c_int as libc::c_ulong) as uint64_t
                        as uint64_t;
                    t_0 = (t_0 as libc::c_ulong).wrapping_add(unhex_val as libc::c_ulong)
                        as uint64_t as uint64_t;
                    if ((9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                        .wrapping_mul(2 as libc::c_ulonglong)
                        .wrapping_add(1 as libc::c_ulonglong)
                        .wrapping_sub(16 as libc::c_int as libc::c_ulonglong)
                        .wrapping_div(16 as libc::c_int as libc::c_ulonglong)
                        < (*parser).content_length as libc::c_ulonglong) as libc::c_int
                        as libc::c_long != 0
                    {
                        (*parser)
                            .set_http_errno(
                                HPE_INVALID_CONTENT_LENGTH as libc::c_int as libc::c_uint,
                            );
                        current_block = 9020038052434785689;
                        break;
                    } else {
                        (*parser).content_length = t_0;
                    }
                }
                current_block = 9007357115414505193;
            }
            6759408996052215043 => {
                if ch as libc::c_int == '\r' as i32 {
                    p_state = s_header_value_discard_ws_almost_done;
                    current_block = 9007357115414505193;
                } else if ch as libc::c_int == '\n' as i32 {
                    p_state = s_header_value_discard_lws;
                    current_block = 9007357115414505193;
                } else {
                    current_block = 12021670973791885667;
                }
            }
            10410764204934602138 => {
                if p == data.offset(len as isize) {
                    p = p.offset(-1);
                    p;
                } else if ch as libc::c_int == ':' as i32 {
                    p_state = s_header_value_discard_ws;
                    if (*parser).http_errno() as http_errno as libc::c_uint
                        == HPE_OK as libc::c_int as libc::c_uint
                    {} else {
                        __assert_fail(
                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            1349 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 94],
                                &[libc::c_char; 94],
                            >(
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    if !header_field_mark.is_null() {
                        if ((*settings).on_header_field).is_some() as libc::c_int
                            as libc::c_long != 0
                        {
                            (*parser).set_state(p_state as libc::c_uint);
                            if (0 as libc::c_int
                                != ((*settings).on_header_field)
                                    .unwrap()(
                                    parser,
                                    header_field_mark,
                                    p.offset_from(header_field_mark) as libc::c_long as size_t,
                                )) as libc::c_int as libc::c_long != 0
                            {
                                (*parser)
                                    .set_http_errno(
                                        HPE_CB_header_field as libc::c_int as libc::c_uint,
                                    );
                            }
                            p_state = (*parser).state() as state;
                            if ((*parser).http_errno() as http_errno as libc::c_uint
                                != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                                as libc::c_long != 0
                            {
                                return (p.offset_from(data) as libc::c_long
                                    + 1 as libc::c_int as libc::c_long) as size_t;
                            }
                        }
                        header_field_mark = 0 as *const libc::c_char;
                    }
                } else {
                    (*parser)
                        .set_http_errno(
                            HPE_INVALID_HEADER_TOKEN as libc::c_int as libc::c_uint,
                        );
                    current_block = 9020038052434785689;
                    break;
                }
                current_block = 9007357115414505193;
            }
            3101959855821210295 => {
                if header_field_mark.is_null() {
                    header_field_mark = p;
                }
                (*parser).set_index(0 as libc::c_int as libc::c_uint);
                p_state = s_header_field;
                match c as libc::c_int {
                    99 => {
                        (*parser).set_header_state(h_C as libc::c_int as libc::c_uint);
                    }
                    112 => {
                        (*parser)
                            .set_header_state(
                                h_matching_proxy_connection as libc::c_int as libc::c_uint,
                            );
                    }
                    116 => {
                        (*parser)
                            .set_header_state(
                                h_matching_transfer_encoding as libc::c_int as libc::c_uint,
                            );
                    }
                    117 => {
                        (*parser)
                            .set_header_state(
                                h_matching_upgrade as libc::c_int as libc::c_uint,
                            );
                    }
                    _ => {
                        (*parser)
                            .set_header_state(h_general as libc::c_int as libc::c_uint);
                    }
                }
                current_block = 9007357115414505193;
            }
            6626987748108865425 => {
                if ch as libc::c_int == '\n' as i32 {
                    p_state = s_header_field_start;
                } else {
                    (*parser)
                        .set_http_errno(
                            HPE_INVALID_VERSION as libc::c_int as libc::c_uint,
                        );
                    current_block = 9020038052434785689;
                    break;
                }
                current_block = 9007357115414505193;
            }
            8419401312455247533 => {
                (*parser)
                    .http_minor = (ch as libc::c_int - '0' as i32) as libc::c_ushort;
                p_state = s_req_http_end;
                current_block = 9007357115414505193;
            }
            18250077801122006663 => {
                (*parser)
                    .http_major = (ch as libc::c_int - '0' as i32) as libc::c_ushort;
                p_state = s_req_http_dot;
                current_block = 9007357115414505193;
            }
            8911980980495988282 => {
                p_state = parse_url_char(p_state, ch);
                if (p_state as libc::c_uint == s_dead as libc::c_int as libc::c_uint)
                    as libc::c_int as libc::c_long != 0
                {
                    (*parser)
                        .set_http_errno(HPE_INVALID_URL as libc::c_int as libc::c_uint);
                    current_block = 9020038052434785689;
                    break;
                } else {
                    current_block = 9007357115414505193;
                }
            }
            6818318202340592218 => {
                (*parser).http_major = 0 as libc::c_int as libc::c_ushort;
                (*parser).http_minor = 9 as libc::c_int as libc::c_ushort;
                p_state = (if ch as libc::c_int == '\r' as i32 {
                    s_req_line_almost_done as libc::c_int
                } else {
                    s_header_field_start as libc::c_int
                }) as state;
                if (*parser).http_errno() as http_errno as libc::c_uint
                    == HPE_OK as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                            as *const libc::c_char,
                        b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                        1073 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 94],
                            &[libc::c_char; 94],
                        >(
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                };
                if !url_mark.is_null() {
                    if ((*settings).on_url).is_some() as libc::c_int as libc::c_long != 0
                    {
                        (*parser).set_state(p_state as libc::c_uint);
                        if (0 as libc::c_int
                            != ((*settings).on_url)
                                .unwrap()(
                                parser,
                                url_mark,
                                p.offset_from(url_mark) as libc::c_long as size_t,
                            )) as libc::c_int as libc::c_long != 0
                        {
                            (*parser)
                                .set_http_errno(HPE_CB_url as libc::c_int as libc::c_uint);
                        }
                        p_state = (*parser).state() as state;
                        if ((*parser).http_errno() as http_errno as libc::c_uint
                            != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                            as libc::c_long != 0
                        {
                            return (p.offset_from(data) as libc::c_long
                                + 1 as libc::c_int as libc::c_long) as size_t;
                        }
                    }
                    url_mark = 0 as *const libc::c_char;
                }
                current_block = 9007357115414505193;
            }
            2513510894289357104 => {
                p_state = s_req_http_start;
                if (*parser).http_errno() as http_errno as libc::c_uint
                    == HPE_OK as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                            as *const libc::c_char,
                        b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                        1064 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 94],
                            &[libc::c_char; 94],
                        >(
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                };
                if !url_mark.is_null() {
                    if ((*settings).on_url).is_some() as libc::c_int as libc::c_long != 0
                    {
                        (*parser).set_state(p_state as libc::c_uint);
                        if (0 as libc::c_int
                            != ((*settings).on_url)
                                .unwrap()(
                                parser,
                                url_mark,
                                p.offset_from(url_mark) as libc::c_long as size_t,
                            )) as libc::c_int as libc::c_long != 0
                        {
                            (*parser)
                                .set_http_errno(HPE_CB_url as libc::c_int as libc::c_uint);
                        }
                        p_state = (*parser).state() as state;
                        if ((*parser).http_errno() as http_errno as libc::c_uint
                            != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                            as libc::c_long != 0
                        {
                            return (p.offset_from(data) as libc::c_long
                                + 1 as libc::c_int as libc::c_long) as size_t;
                        }
                    }
                    url_mark = 0 as *const libc::c_char;
                }
                current_block = 9007357115414505193;
            }
            1550381671659267203 => {
                p_state = parse_url_char(p_state, ch);
                if (p_state as libc::c_uint == s_dead as libc::c_int as libc::c_uint)
                    as libc::c_int as libc::c_long != 0
                {
                    (*parser)
                        .set_http_errno(HPE_INVALID_URL as libc::c_int as libc::c_uint);
                    current_block = 9020038052434785689;
                    break;
                } else {
                    current_block = 9007357115414505193;
                }
            }
            4961777833250047394 => {
                if url_mark.is_null() {
                    url_mark = p;
                }
                if (*parser).method() as libc::c_int == HTTP_CONNECT as libc::c_int {
                    p_state = s_req_server_start;
                }
                p_state = parse_url_char(p_state, ch);
                if (p_state as libc::c_uint == s_dead as libc::c_int as libc::c_uint)
                    as libc::c_int as libc::c_long != 0
                {
                    (*parser)
                        .set_http_errno(HPE_INVALID_URL as libc::c_int as libc::c_uint);
                    current_block = 9020038052434785689;
                    break;
                } else {
                    current_block = 9007357115414505193;
                }
            }
            15775903806355281185 => {
                matcher = method_strings[(*parser).method() as usize];
                if ch as libc::c_int == ' ' as i32
                    && *matcher.offset((*parser).index() as isize) as libc::c_int
                        == '\0' as i32
                {
                    p_state = s_req_spaces_before_url;
                } else if !(ch as libc::c_int
                    == *matcher.offset((*parser).index() as isize) as libc::c_int)
                {
                    if ch as libc::c_int >= 'A' as i32 && ch as libc::c_int <= 'Z' as i32
                        || ch as libc::c_int == '-' as i32
                    {
                        match ((*parser).method() as libc::c_int) << 16 as libc::c_int
                            | ((*parser).index() as libc::c_int) << 8 as libc::c_int
                            | ch as libc::c_int
                        {
                            196949 => {
                                (*parser)
                                    .set_method(HTTP_PUT as libc::c_int as libc::c_uint);
                            }
                            196929 => {
                                (*parser)
                                    .set_method(HTTP_PATCH as libc::c_int as libc::c_uint);
                            }
                            196946 => {
                                (*parser)
                                    .set_method(HTTP_PROPFIND as libc::c_int as libc::c_uint);
                            }
                            262738 => {
                                (*parser)
                                    .set_method(HTTP_PURGE as libc::c_int as libc::c_uint);
                            }
                            328008 => {
                                (*parser)
                                    .set_method(HTTP_CHECKOUT as libc::c_int as libc::c_uint);
                            }
                            328272 => {
                                (*parser)
                                    .set_method(HTTP_COPY as libc::c_int as libc::c_uint);
                            }
                            655695 => {
                                (*parser)
                                    .set_method(HTTP_MOVE as libc::c_int as libc::c_uint);
                            }
                            655685 => {
                                (*parser)
                                    .set_method(HTTP_MERGE as libc::c_int as libc::c_uint);
                            }
                            655661 => {
                                (*parser)
                                    .set_method(HTTP_MSEARCH as libc::c_int as libc::c_uint);
                            }
                            655937 => {
                                (*parser)
                                    .set_method(HTTP_MKACTIVITY as libc::c_int as libc::c_uint);
                            }
                            656193 => {
                                (*parser)
                                    .set_method(HTTP_MKCALENDAR as libc::c_int as libc::c_uint);
                            }
                            1704261 => {
                                (*parser)
                                    .set_method(HTTP_SEARCH as libc::c_int as libc::c_uint);
                            }
                            1311298 => {
                                (*parser)
                                    .set_method(HTTP_REBIND as libc::c_int as libc::c_uint);
                            }
                            787536 => {
                                (*parser)
                                    .set_method(HTTP_PROPPATCH as libc::c_int as libc::c_uint);
                            }
                            590153 => {
                                (*parser)
                                    .set_method(HTTP_LINK as libc::c_int as libc::c_uint);
                            }
                            983635 => {
                                (*parser)
                                    .set_method(
                                        HTTP_UNSUBSCRIBE as libc::c_int as libc::c_uint,
                                    );
                            }
                            983618 => {
                                (*parser)
                                    .set_method(HTTP_UNBIND as libc::c_int as libc::c_uint);
                            }
                            983881 => {
                                (*parser)
                                    .set_method(HTTP_UNLINK as libc::c_int as libc::c_uint);
                            }
                            _ => {
                                (*parser)
                                    .set_http_errno(
                                        HPE_INVALID_METHOD as libc::c_int as libc::c_uint,
                                    );
                                current_block = 9020038052434785689;
                                break;
                            }
                        }
                    } else {
                        (*parser)
                            .set_http_errno(
                                HPE_INVALID_METHOD as libc::c_int as libc::c_uint,
                            );
                        current_block = 9020038052434785689;
                        break;
                    }
                }
                (*parser).set_index((*parser).index() + 1);
                (*parser).index();
                current_block = 9007357115414505193;
            }
            8590355382656639939 => {
                (*parser).set_flags(0 as libc::c_int as libc::c_uint);
                (*parser)
                    .content_length = (9223372036854775807 as libc::c_longlong
                    as libc::c_ulonglong)
                    .wrapping_mul(2 as libc::c_ulonglong)
                    .wrapping_add(1 as libc::c_ulonglong) as uint64_t;
                if !((ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                    as libc::c_int >= 'a' as i32
                    && (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                        as libc::c_int <= 'z' as i32) as libc::c_int as libc::c_long != 0
                {
                    (*parser)
                        .set_http_errno(
                            HPE_INVALID_METHOD as libc::c_int as libc::c_uint,
                        );
                    current_block = 9020038052434785689;
                    break;
                } else {
                    (*parser).set_method(HTTP_DELETE as libc::c_uint);
                    (*parser).set_index(1 as libc::c_int as libc::c_uint);
                    match ch as libc::c_int {
                        65 => {
                            (*parser)
                                .set_method(HTTP_ACL as libc::c_int as libc::c_uint);
                        }
                        66 => {
                            (*parser)
                                .set_method(HTTP_BIND as libc::c_int as libc::c_uint);
                        }
                        67 => {
                            (*parser)
                                .set_method(HTTP_CONNECT as libc::c_int as libc::c_uint);
                        }
                        68 => {
                            (*parser)
                                .set_method(HTTP_DELETE as libc::c_int as libc::c_uint);
                        }
                        71 => {
                            (*parser)
                                .set_method(HTTP_GET as libc::c_int as libc::c_uint);
                        }
                        72 => {
                            (*parser)
                                .set_method(HTTP_HEAD as libc::c_int as libc::c_uint);
                        }
                        76 => {
                            (*parser)
                                .set_method(HTTP_LOCK as libc::c_int as libc::c_uint);
                        }
                        77 => {
                            (*parser)
                                .set_method(HTTP_MKCOL as libc::c_int as libc::c_uint);
                        }
                        78 => {
                            (*parser)
                                .set_method(HTTP_NOTIFY as libc::c_int as libc::c_uint);
                        }
                        79 => {
                            (*parser)
                                .set_method(HTTP_OPTIONS as libc::c_int as libc::c_uint);
                        }
                        80 => {
                            (*parser)
                                .set_method(HTTP_POST as libc::c_int as libc::c_uint);
                        }
                        82 => {
                            (*parser)
                                .set_method(HTTP_REPORT as libc::c_int as libc::c_uint);
                        }
                        83 => {
                            (*parser)
                                .set_method(HTTP_SUBSCRIBE as libc::c_int as libc::c_uint);
                        }
                        84 => {
                            (*parser)
                                .set_method(HTTP_TRACE as libc::c_int as libc::c_uint);
                        }
                        85 => {
                            (*parser)
                                .set_method(HTTP_UNLOCK as libc::c_int as libc::c_uint);
                        }
                        _ => {
                            (*parser)
                                .set_http_errno(
                                    HPE_INVALID_METHOD as libc::c_int as libc::c_uint,
                                );
                            current_block = 9020038052434785689;
                            break;
                        }
                    }
                    p_state = s_req_method;
                    if (*parser).http_errno() as http_errno as libc::c_uint
                        == HPE_OK as libc::c_int as libc::c_uint
                    {} else {
                        __assert_fail(
                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            955 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 94],
                                &[libc::c_char; 94],
                            >(
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    if ((*settings).on_message_begin).is_some() as libc::c_int
                        as libc::c_long != 0
                    {
                        (*parser).set_state(p_state as libc::c_uint);
                        if (0 as libc::c_int
                            != ((*settings).on_message_begin).unwrap()(parser))
                            as libc::c_int as libc::c_long != 0
                        {
                            (*parser)
                                .set_http_errno(
                                    HPE_CB_message_begin as libc::c_int as libc::c_uint,
                                );
                        }
                        p_state = (*parser).state() as state;
                        if ((*parser).http_errno() as http_errno as libc::c_uint
                            != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                            as libc::c_long != 0
                        {
                            return (p.offset_from(data) as libc::c_long
                                + 1 as libc::c_int as libc::c_long) as size_t;
                        }
                    }
                }
                current_block = 9007357115414505193;
            }
            3906822848181906220 => {
                if ch as libc::c_int == '\n' as i32 {
                    p_state = s_header_field_start;
                    if (*parser).http_errno() as http_errno as libc::c_uint
                        == HPE_OK as libc::c_int as libc::c_uint
                    {} else {
                        __assert_fail(
                            b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                                as *const libc::c_char,
                            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                            906 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 94],
                                &[libc::c_char; 94],
                            >(
                                b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    if !status_mark.is_null() {
                        if ((*settings).on_status).is_some() as libc::c_int
                            as libc::c_long != 0
                        {
                            (*parser).set_state(p_state as libc::c_uint);
                            if (0 as libc::c_int
                                != ((*settings).on_status)
                                    .unwrap()(
                                    parser,
                                    status_mark,
                                    p.offset_from(status_mark) as libc::c_long as size_t,
                                )) as libc::c_int as libc::c_long != 0
                            {
                                (*parser)
                                    .set_http_errno(
                                        HPE_CB_status as libc::c_int as libc::c_uint,
                                    );
                            }
                            p_state = (*parser).state() as state;
                            if ((*parser).http_errno() as http_errno as libc::c_uint
                                != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                                as libc::c_long != 0
                            {
                                return (p.offset_from(data) as libc::c_long
                                    + 1 as libc::c_int as libc::c_long) as size_t;
                            }
                        }
                        status_mark = 0 as *const libc::c_char;
                    }
                    current_block = 9007357115414505193;
                } else {
                    current_block = 9007357115414505193;
                }
            }
            16185292562584120790 => {
                p_state = s_res_line_almost_done;
                if (*parser).http_errno() as http_errno as libc::c_uint
                    == HPE_OK as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                            as *const libc::c_char,
                        b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                        900 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 94],
                            &[libc::c_char; 94],
                        >(
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                };
                if !status_mark.is_null() {
                    if ((*settings).on_status).is_some() as libc::c_int as libc::c_long
                        != 0
                    {
                        (*parser).set_state(p_state as libc::c_uint);
                        if (0 as libc::c_int
                            != ((*settings).on_status)
                                .unwrap()(
                                parser,
                                status_mark,
                                p.offset_from(status_mark) as libc::c_long as size_t,
                            )) as libc::c_int as libc::c_long != 0
                        {
                            (*parser)
                                .set_http_errno(
                                    HPE_CB_status as libc::c_int as libc::c_uint,
                                );
                        }
                        p_state = (*parser).state() as state;
                        if ((*parser).http_errno() as http_errno as libc::c_uint
                            != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                            as libc::c_long != 0
                        {
                            return (p.offset_from(data) as libc::c_long
                                + 1 as libc::c_int as libc::c_long) as size_t;
                        }
                    }
                    status_mark = 0 as *const libc::c_char;
                }
                current_block = 9007357115414505193;
            }
            4542590401787960760 => {
                p_state = s_chunk_data_done;
                if (*parser).http_errno() as http_errno as libc::c_uint
                    == HPE_OK as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                            as *const libc::c_char,
                        b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                        1981 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 94],
                            &[libc::c_char; 94],
                        >(
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                };
                if !body_mark.is_null() {
                    if ((*settings).on_body).is_some() as libc::c_int as libc::c_long
                        != 0
                    {
                        (*parser).set_state(p_state as libc::c_uint);
                        if (0 as libc::c_int
                            != ((*settings).on_body)
                                .unwrap()(
                                parser,
                                body_mark,
                                p.offset_from(body_mark) as libc::c_long as size_t,
                            )) as libc::c_int as libc::c_long != 0
                        {
                            (*parser)
                                .set_http_errno(HPE_CB_body as libc::c_int as libc::c_uint);
                        }
                        p_state = (*parser).state() as state;
                        if ((*parser).http_errno() as http_errno as libc::c_uint
                            != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                            as libc::c_long != 0
                        {
                            return (p.offset_from(data) as libc::c_long
                                + 1 as libc::c_int as libc::c_long) as size_t;
                        }
                    }
                    body_mark = 0 as *const libc::c_char;
                }
                current_block = 9007357115414505193;
            }
            16519603272705762099 => {
                (*parser).nread = 0 as libc::c_int as uint32_t;
                if (*parser).content_length == 0 as libc::c_int as libc::c_ulong {
                    (*parser)
                        .set_flags(
                            (*parser).flags() | F_TRAILING as libc::c_int as libc::c_uint,
                        );
                    p_state = s_header_field_start;
                } else {
                    p_state = s_chunk_data;
                }
                if (*parser).http_errno() as http_errno as libc::c_uint
                    == HPE_OK as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                            as *const libc::c_char,
                        b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                        1949 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 94],
                            &[libc::c_char; 94],
                        >(
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                };
                if ((*settings).on_chunk_header).is_some() as libc::c_int as libc::c_long
                    != 0
                {
                    (*parser).set_state(p_state as libc::c_uint);
                    if (0 as libc::c_int
                        != ((*settings).on_chunk_header).unwrap()(parser)) as libc::c_int
                        as libc::c_long != 0
                    {
                        (*parser)
                            .set_http_errno(
                                HPE_CB_chunk_header as libc::c_int as libc::c_uint,
                            );
                    }
                    p_state = (*parser).state() as state;
                    if ((*parser).http_errno() as http_errno as libc::c_uint
                        != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                        as libc::c_long != 0
                    {
                        return (p.offset_from(data) as libc::c_long
                            + 1 as libc::c_int as libc::c_long) as size_t;
                    }
                }
                current_block = 9007357115414505193;
            }
            3098209481605707636 => {
                (*parser)
                    .set_status_code((ch as libc::c_int - '0' as i32) as libc::c_uint);
                p_state = s_res_status_code;
                current_block = 9007357115414505193;
            }
            9216188846964669005 => {
                if ch as libc::c_int == ' ' as i32 {
                    current_block = 9007357115414505193;
                } else {
                    (*parser)
                        .set_http_errno(
                            HPE_INVALID_STATUS as libc::c_int as libc::c_uint,
                        );
                    current_block = 9020038052434785689;
                    break;
                }
            }
            5388205036907793036 => {
                (*parser)
                    .http_minor = (ch as libc::c_int - '0' as i32) as libc::c_ushort;
                p_state = s_res_http_end;
                current_block = 9007357115414505193;
            }
            5151888778912688305 => {
                (*parser)
                    .http_major = (ch as libc::c_int - '0' as i32) as libc::c_ushort;
                p_state = s_res_http_dot;
                current_block = 9007357115414505193;
            }
            10778260831612459202 => {
                if (ch as libc::c_int != 'E' as i32) as libc::c_int as libc::c_long != 0
                {
                    (*parser)
                        .set_http_errno(
                            HPE_INVALID_CONSTANT as libc::c_int as libc::c_uint,
                        );
                    current_block = 9020038052434785689;
                    break;
                } else {
                    (*parser).set_type_0(HTTP_REQUEST as libc::c_int as libc::c_uint);
                    (*parser).set_method(HTTP_HEAD as libc::c_int as libc::c_uint);
                    (*parser).set_index(2 as libc::c_int as libc::c_uint);
                    p_state = s_req_method;
                }
                current_block = 9007357115414505193;
            }
            17424578316536045879 => {
                (*parser).nread = 0 as libc::c_int as uint32_t;
                p_state = s_chunk_size_start;
                if (*parser).http_errno() as http_errno as libc::c_uint
                    == HPE_OK as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                            as *const libc::c_char,
                        b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                        1989 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 94],
                            &[libc::c_char; 94],
                        >(
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                };
                if ((*settings).on_chunk_complete).is_some() as libc::c_int
                    as libc::c_long != 0
                {
                    (*parser).set_state(p_state as libc::c_uint);
                    if (0 as libc::c_int
                        != ((*settings).on_chunk_complete).unwrap()(parser))
                        as libc::c_int as libc::c_long != 0
                    {
                        (*parser)
                            .set_http_errno(
                                HPE_CB_chunk_complete as libc::c_int as libc::c_uint,
                            );
                    }
                    p_state = (*parser).state() as state;
                    if ((*parser).http_errno() as http_errno as libc::c_uint
                        != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                        as libc::c_long != 0
                    {
                        return (p.offset_from(data) as libc::c_long
                            + 1 as libc::c_int as libc::c_long) as size_t;
                    }
                }
                current_block = 9007357115414505193;
            }
            10725100205392175504 => {
                (*parser)
                    .set_http_errno(HPE_INVALID_VERSION as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            1423531122933789233 => {
                (*parser)
                    .set_http_errno(
                        HPE_CLOSED_CONNECTION as libc::c_int as libc::c_uint,
                    );
                current_block = 9020038052434785689;
                break;
            }
            10382007559576128104 => {
                (*parser)
                    .set_http_errno(HPE_INVALID_CONSTANT as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            8602574157404971894 => {
                p_state = s_res_HT;
                current_block = 9007357115414505193;
            }
            13215501469961642988 => {
                p_state = s_res_HTT;
                current_block = 9007357115414505193;
            }
            2872334340672008580 => {
                p_state = s_res_HTTP;
                current_block = 9007357115414505193;
            }
            9587810615301548814 => {
                p_state = s_res_http_major;
                current_block = 9007357115414505193;
            }
            5832582820025303349 => {
                p_state = s_res_http_minor;
                current_block = 9007357115414505193;
            }
            12969817083969514432 => {
                p_state = s_res_first_status_code;
                current_block = 9007357115414505193;
            }
            12032176231992402880 => {
                p_state = s_header_field_start;
                current_block = 9007357115414505193;
            }
            8221142977249714136 => {
                (*parser)
                    .set_http_errno(HPE_INVALID_CONSTANT as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            10059826840140668507 => {
                p_state = s_req_http_HT;
                current_block = 9007357115414505193;
            }
            11888593055720107986 => {
                p_state = s_req_http_HTT;
                current_block = 9007357115414505193;
            }
            5561851013817067674 => {
                p_state = s_req_http_HTTP;
                current_block = 9007357115414505193;
            }
            3149046158064628027 => {
                p_state = s_req_http_major;
                current_block = 9007357115414505193;
            }
            5282966931892662513 => {
                p_state = s_req_http_minor;
                current_block = 9007357115414505193;
            }
            3526436446048920899 => {
                p_state = s_header_field_start;
                current_block = 9007357115414505193;
            }
            8062549956965633527 => {
                p_state = s_header_value_lws;
                current_block = 9007357115414505193;
            }
            9725360004668457823 => {
                p_state = s_header_value_discard_lws;
                current_block = 9007357115414505193;
            }
            9580941621949456922 => {
                (*parser).set_http_errno(HPE_STRICT as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            5407058732526620246 => {
                (*parser).set_http_errno(HPE_STRICT as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            8758648760486203175 => {
                p_state = s_chunk_size_almost_done;
                current_block = 9007357115414505193;
            }
            9611983732549257940 => {
                (*parser).set_http_errno(HPE_STRICT as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            12761086676529082117 => {
                (*parser).set_http_errno(HPE_STRICT as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            5286729433347123073 => {
                (*parser).set_http_errno(HPE_STRICT as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            10194589593280242392 => {
                (*parser)
                    .set_http_errno(HPE_INVALID_STATUS as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            6829280784076477810 => {
                p_state = s_chunk_size_almost_done;
                current_block = 9007357115414505193;
            }
            12940301851547707723 => {
                (*parser)
                    .set_http_errno(
                        HPE_INVALID_CHUNK_SIZE as libc::c_int as libc::c_uint,
                    );
                current_block = 9020038052434785689;
                break;
            }
            1781577627025608192 => {
                (*parser).set_http_errno(HPE_LF_EXPECTED as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            3791882670782541151 => {
                (*parser)
                    .set_http_errno(
                        HPE_INVALID_HEADER_TOKEN as libc::c_int as libc::c_uint,
                    );
                current_block = 9020038052434785689;
                break;
            }
            6874267722315749555 => {
                (*parser)
                    .set_http_errno(HPE_HEADER_OVERFLOW as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            15201865807353435331 => {
                (*parser).set_http_errno(HPE_LF_EXPECTED as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            4692175290644916 => {
                p_state = s_req_line_almost_done;
                current_block = 9007357115414505193;
            }
            13990052500608980534 => {
                (*parser)
                    .set_http_errno(HPE_INVALID_VERSION as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            16939702945023487982 => {
                (*parser)
                    .set_http_errno(HPE_INVALID_VERSION as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            919954187481050311 => {
                (*parser).set_type_0(HTTP_RESPONSE as libc::c_int as libc::c_uint);
                p_state = s_res_HT;
                current_block = 9007357115414505193;
            }
            10873520258093198934 => {
                p_state = s_res_H;
                current_block = 18425699056680496821;
            }
            10863493864285401582 => {
                (*parser).set_http_errno(HPE_STRICT as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            14027225908442187354 => {
                (*parser).set_http_errno(HPE_STRICT as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            18039443766442739006 => {
                (*parser).set_http_errno(HPE_STRICT as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            15460309861373144675 => {
                (*parser).set_http_errno(HPE_STRICT as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            9838996637140935403 => {
                (*parser)
                    .set_http_errno(HPE_INVALID_VERSION as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            10601179871800211547 => {
                (*parser)
                    .set_http_errno(HPE_INVALID_VERSION as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            2544535129495155983 => {
                (*parser)
                    .set_http_errno(HPE_INVALID_VERSION as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            7058451906849221989 => {
                (*parser)
                    .set_http_errno(HPE_INVALID_VERSION as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            17034918949615525785 => {
                (*parser).set_http_errno(HPE_STRICT as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            13543149951580262689 => {
                (*parser)
                    .set_http_errno(HPE_INVALID_METHOD as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            10304134390680427967 => {
                (*parser).set_http_errno(HPE_INVALID_URL as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            8077860765343440262 => {
                p_state = s_req_http_H;
                current_block = 9007357115414505193;
            }
            768347334755947778 => {
                (*parser).set_http_errno(HPE_STRICT as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            8155699268269024736 => {
                (*parser).set_http_errno(HPE_STRICT as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            11814237681575821830 => {
                (*parser).set_http_errno(HPE_STRICT as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            29051446364946957 => {
                (*parser).set_http_errno(HPE_STRICT as libc::c_int as libc::c_uint);
                current_block = 9020038052434785689;
                break;
            }
            _ => {}
        }
        match current_block {
            12021670973791885667 => {
                if header_value_mark.is_null() {
                    header_value_mark = p;
                }
                p_state = s_header_value;
                (*parser).set_index(0 as libc::c_int as libc::c_uint);
                c = (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                    as libc::c_char;
                match (*parser).header_state() as libc::c_int {
                    12 => {
                        current_block = 17908549510646023812;
                        match current_block {
                            15488721613467431513 => {
                                (*parser)
                                    .set_header_state(h_general as libc::c_int as libc::c_uint);
                            }
                            17908549510646023812 => {
                                (*parser)
                                    .set_flags(
                                        (*parser).flags() | F_UPGRADE as libc::c_int as libc::c_uint,
                                    );
                                (*parser)
                                    .set_header_state(h_general as libc::c_int as libc::c_uint);
                            }
                            9275925785692447356 => {
                                if 'c' as i32 == c as libc::c_int {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_transfer_encoding_chunked as libc::c_int
                                                as libc::c_uint,
                                        );
                                } else {
                                    (*parser)
                                        .set_header_state(h_general as libc::c_int as libc::c_uint);
                                }
                            }
                            10853407604259456941 => {
                                if c as libc::c_int == 'k' as i32 {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_connection_keep_alive as libc::c_int
                                                as libc::c_uint,
                                        );
                                } else if c as libc::c_int == 'c' as i32 {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_connection_close as libc::c_int as libc::c_uint,
                                        );
                                } else if c as libc::c_int == 'u' as i32 {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_connection_upgrade as libc::c_int as libc::c_uint,
                                        );
                                } else {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_connection_token as libc::c_int as libc::c_uint,
                                        );
                                }
                            }
                            _ => {
                                if !(ch as libc::c_int >= '0' as i32
                                    && ch as libc::c_int <= '9' as i32) as libc::c_int
                                    as libc::c_long != 0
                                {
                                    (*parser)
                                        .set_http_errno(
                                            HPE_INVALID_CONTENT_LENGTH as libc::c_int as libc::c_uint,
                                        );
                                    current_block = 9020038052434785689;
                                    break;
                                } else if (*parser).flags() as libc::c_int
                                    & F_CONTENTLENGTH as libc::c_int != 0
                                {
                                    (*parser)
                                        .set_http_errno(
                                            HPE_UNEXPECTED_CONTENT_LENGTH as libc::c_int as libc::c_uint,
                                        );
                                    current_block = 9020038052434785689;
                                    break;
                                } else {
                                    (*parser)
                                        .set_flags(
                                            (*parser).flags()
                                                | F_CONTENTLENGTH as libc::c_int as libc::c_uint,
                                        );
                                    (*parser)
                                        .content_length = (ch as libc::c_int - '0' as i32)
                                        as uint64_t;
                                }
                            }
                        }
                    }
                    11 => {
                        current_block = 9275925785692447356;
                        match current_block {
                            15488721613467431513 => {
                                (*parser)
                                    .set_header_state(h_general as libc::c_int as libc::c_uint);
                            }
                            17908549510646023812 => {
                                (*parser)
                                    .set_flags(
                                        (*parser).flags() | F_UPGRADE as libc::c_int as libc::c_uint,
                                    );
                                (*parser)
                                    .set_header_state(h_general as libc::c_int as libc::c_uint);
                            }
                            9275925785692447356 => {
                                if 'c' as i32 == c as libc::c_int {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_transfer_encoding_chunked as libc::c_int
                                                as libc::c_uint,
                                        );
                                } else {
                                    (*parser)
                                        .set_header_state(h_general as libc::c_int as libc::c_uint);
                                }
                            }
                            10853407604259456941 => {
                                if c as libc::c_int == 'k' as i32 {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_connection_keep_alive as libc::c_int
                                                as libc::c_uint,
                                        );
                                } else if c as libc::c_int == 'c' as i32 {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_connection_close as libc::c_int as libc::c_uint,
                                        );
                                } else if c as libc::c_int == 'u' as i32 {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_connection_upgrade as libc::c_int as libc::c_uint,
                                        );
                                } else {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_connection_token as libc::c_int as libc::c_uint,
                                        );
                                }
                            }
                            _ => {
                                if !(ch as libc::c_int >= '0' as i32
                                    && ch as libc::c_int <= '9' as i32) as libc::c_int
                                    as libc::c_long != 0
                                {
                                    (*parser)
                                        .set_http_errno(
                                            HPE_INVALID_CONTENT_LENGTH as libc::c_int as libc::c_uint,
                                        );
                                    current_block = 9020038052434785689;
                                    break;
                                } else if (*parser).flags() as libc::c_int
                                    & F_CONTENTLENGTH as libc::c_int != 0
                                {
                                    (*parser)
                                        .set_http_errno(
                                            HPE_UNEXPECTED_CONTENT_LENGTH as libc::c_int as libc::c_uint,
                                        );
                                    current_block = 9020038052434785689;
                                    break;
                                } else {
                                    (*parser)
                                        .set_flags(
                                            (*parser).flags()
                                                | F_CONTENTLENGTH as libc::c_int as libc::c_uint,
                                        );
                                    (*parser)
                                        .content_length = (ch as libc::c_int - '0' as i32)
                                        as uint64_t;
                                }
                            }
                        }
                    }
                    10 => {
                        current_block = 10589553256965311190;
                        match current_block {
                            15488721613467431513 => {
                                (*parser)
                                    .set_header_state(h_general as libc::c_int as libc::c_uint);
                            }
                            17908549510646023812 => {
                                (*parser)
                                    .set_flags(
                                        (*parser).flags() | F_UPGRADE as libc::c_int as libc::c_uint,
                                    );
                                (*parser)
                                    .set_header_state(h_general as libc::c_int as libc::c_uint);
                            }
                            9275925785692447356 => {
                                if 'c' as i32 == c as libc::c_int {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_transfer_encoding_chunked as libc::c_int
                                                as libc::c_uint,
                                        );
                                } else {
                                    (*parser)
                                        .set_header_state(h_general as libc::c_int as libc::c_uint);
                                }
                            }
                            10853407604259456941 => {
                                if c as libc::c_int == 'k' as i32 {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_connection_keep_alive as libc::c_int
                                                as libc::c_uint,
                                        );
                                } else if c as libc::c_int == 'c' as i32 {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_connection_close as libc::c_int as libc::c_uint,
                                        );
                                } else if c as libc::c_int == 'u' as i32 {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_connection_upgrade as libc::c_int as libc::c_uint,
                                        );
                                } else {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_connection_token as libc::c_int as libc::c_uint,
                                        );
                                }
                            }
                            _ => {
                                if !(ch as libc::c_int >= '0' as i32
                                    && ch as libc::c_int <= '9' as i32) as libc::c_int
                                    as libc::c_long != 0
                                {
                                    (*parser)
                                        .set_http_errno(
                                            HPE_INVALID_CONTENT_LENGTH as libc::c_int as libc::c_uint,
                                        );
                                    current_block = 9020038052434785689;
                                    break;
                                } else if (*parser).flags() as libc::c_int
                                    & F_CONTENTLENGTH as libc::c_int != 0
                                {
                                    (*parser)
                                        .set_http_errno(
                                            HPE_UNEXPECTED_CONTENT_LENGTH as libc::c_int as libc::c_uint,
                                        );
                                    current_block = 9020038052434785689;
                                    break;
                                } else {
                                    (*parser)
                                        .set_flags(
                                            (*parser).flags()
                                                | F_CONTENTLENGTH as libc::c_int as libc::c_uint,
                                        );
                                    (*parser)
                                        .content_length = (ch as libc::c_int - '0' as i32)
                                        as uint64_t;
                                }
                            }
                        }
                    }
                    9 => {
                        current_block = 10853407604259456941;
                        match current_block {
                            15488721613467431513 => {
                                (*parser)
                                    .set_header_state(h_general as libc::c_int as libc::c_uint);
                            }
                            17908549510646023812 => {
                                (*parser)
                                    .set_flags(
                                        (*parser).flags() | F_UPGRADE as libc::c_int as libc::c_uint,
                                    );
                                (*parser)
                                    .set_header_state(h_general as libc::c_int as libc::c_uint);
                            }
                            9275925785692447356 => {
                                if 'c' as i32 == c as libc::c_int {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_transfer_encoding_chunked as libc::c_int
                                                as libc::c_uint,
                                        );
                                } else {
                                    (*parser)
                                        .set_header_state(h_general as libc::c_int as libc::c_uint);
                                }
                            }
                            10853407604259456941 => {
                                if c as libc::c_int == 'k' as i32 {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_connection_keep_alive as libc::c_int
                                                as libc::c_uint,
                                        );
                                } else if c as libc::c_int == 'c' as i32 {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_connection_close as libc::c_int as libc::c_uint,
                                        );
                                } else if c as libc::c_int == 'u' as i32 {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_connection_upgrade as libc::c_int as libc::c_uint,
                                        );
                                } else {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_connection_token as libc::c_int as libc::c_uint,
                                        );
                                }
                            }
                            _ => {
                                if !(ch as libc::c_int >= '0' as i32
                                    && ch as libc::c_int <= '9' as i32) as libc::c_int
                                    as libc::c_long != 0
                                {
                                    (*parser)
                                        .set_http_errno(
                                            HPE_INVALID_CONTENT_LENGTH as libc::c_int as libc::c_uint,
                                        );
                                    current_block = 9020038052434785689;
                                    break;
                                } else if (*parser).flags() as libc::c_int
                                    & F_CONTENTLENGTH as libc::c_int != 0
                                {
                                    (*parser)
                                        .set_http_errno(
                                            HPE_UNEXPECTED_CONTENT_LENGTH as libc::c_int as libc::c_uint,
                                        );
                                    current_block = 9020038052434785689;
                                    break;
                                } else {
                                    (*parser)
                                        .set_flags(
                                            (*parser).flags()
                                                | F_CONTENTLENGTH as libc::c_int as libc::c_uint,
                                        );
                                    (*parser)
                                        .content_length = (ch as libc::c_int - '0' as i32)
                                        as uint64_t;
                                }
                            }
                        }
                    }
                    14 => {}
                    _ => {
                        current_block = 15488721613467431513;
                        match current_block {
                            15488721613467431513 => {
                                (*parser)
                                    .set_header_state(h_general as libc::c_int as libc::c_uint);
                            }
                            17908549510646023812 => {
                                (*parser)
                                    .set_flags(
                                        (*parser).flags() | F_UPGRADE as libc::c_int as libc::c_uint,
                                    );
                                (*parser)
                                    .set_header_state(h_general as libc::c_int as libc::c_uint);
                            }
                            9275925785692447356 => {
                                if 'c' as i32 == c as libc::c_int {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_transfer_encoding_chunked as libc::c_int
                                                as libc::c_uint,
                                        );
                                } else {
                                    (*parser)
                                        .set_header_state(h_general as libc::c_int as libc::c_uint);
                                }
                            }
                            10853407604259456941 => {
                                if c as libc::c_int == 'k' as i32 {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_connection_keep_alive as libc::c_int
                                                as libc::c_uint,
                                        );
                                } else if c as libc::c_int == 'c' as i32 {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_connection_close as libc::c_int as libc::c_uint,
                                        );
                                } else if c as libc::c_int == 'u' as i32 {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_connection_upgrade as libc::c_int as libc::c_uint,
                                        );
                                } else {
                                    (*parser)
                                        .set_header_state(
                                            h_matching_connection_token as libc::c_int as libc::c_uint,
                                        );
                                }
                            }
                            _ => {
                                if !(ch as libc::c_int >= '0' as i32
                                    && ch as libc::c_int <= '9' as i32) as libc::c_int
                                    as libc::c_long != 0
                                {
                                    (*parser)
                                        .set_http_errno(
                                            HPE_INVALID_CONTENT_LENGTH as libc::c_int as libc::c_uint,
                                        );
                                    current_block = 9020038052434785689;
                                    break;
                                } else if (*parser).flags() as libc::c_int
                                    & F_CONTENTLENGTH as libc::c_int != 0
                                {
                                    (*parser)
                                        .set_http_errno(
                                            HPE_UNEXPECTED_CONTENT_LENGTH as libc::c_int as libc::c_uint,
                                        );
                                    current_block = 9020038052434785689;
                                    break;
                                } else {
                                    (*parser)
                                        .set_flags(
                                            (*parser).flags()
                                                | F_CONTENTLENGTH as libc::c_int as libc::c_uint,
                                        );
                                    (*parser)
                                        .content_length = (ch as libc::c_int - '0' as i32)
                                        as uint64_t;
                                }
                            }
                        }
                    }
                }
            }
            18425699056680496821 => {
                if (*parser).http_errno() as http_errno as libc::c_uint
                    == HPE_OK as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                            as *const libc::c_char,
                        b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                        774 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 94],
                            &[libc::c_char; 94],
                        >(
                            b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                };
                if ((*settings).on_message_begin).is_some() as libc::c_int
                    as libc::c_long != 0
                {
                    (*parser).set_state(p_state as libc::c_uint);
                    if (0 as libc::c_int
                        != ((*settings).on_message_begin).unwrap()(parser))
                        as libc::c_int as libc::c_long != 0
                    {
                        (*parser)
                            .set_http_errno(
                                HPE_CB_message_begin as libc::c_int as libc::c_uint,
                            );
                    }
                    p_state = (*parser).state() as state;
                    if ((*parser).http_errno() as http_errno as libc::c_uint
                        != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                        as libc::c_long != 0
                    {
                        return (p.offset_from(data) as libc::c_long
                            + 1 as libc::c_int as libc::c_long) as size_t;
                    }
                }
            }
            _ => {}
        }
        p = p.offset(1);
        p;
    }
    match current_block {
        9020038052434785689 => {
            if (*parser).http_errno() as http_errno as libc::c_uint
                == HPE_OK as libc::c_int as libc::c_uint
            {
                (*parser).set_http_errno(HPE_UNKNOWN as libc::c_int as libc::c_uint);
            }
            (*parser).set_state(p_state as libc::c_uint);
            return p.offset_from(data) as libc::c_long as size_t;
        }
        _ => {
            if (if !header_field_mark.is_null() {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            })
                + (if !header_value_mark.is_null() {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                })
                + (if !url_mark.is_null() { 1 as libc::c_int } else { 0 as libc::c_int })
                + (if !body_mark.is_null() {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                })
                + (if !status_mark.is_null() {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) <= 1 as libc::c_int
            {} else {
                __assert_fail(
                    b"((header_field_mark ? 1 : 0) + (header_value_mark ? 1 : 0) + (url_mark ? 1 : 0) + (body_mark ? 1 : 0) + (status_mark ? 1 : 0)) <= 1\0"
                        as *const u8 as *const libc::c_char,
                    b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                    2013 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 94],
                        &[libc::c_char; 94],
                    >(
                        b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            };
            if (*parser).http_errno() as http_errno as libc::c_uint
                == HPE_OK as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                        as *const libc::c_char,
                    b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                    2015 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 94],
                        &[libc::c_char; 94],
                    >(
                        b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            };
            if !header_field_mark.is_null() {
                if ((*settings).on_header_field).is_some() as libc::c_int as libc::c_long
                    != 0
                {
                    (*parser).set_state(p_state as libc::c_uint);
                    if (0 as libc::c_int
                        != ((*settings).on_header_field)
                            .unwrap()(
                            parser,
                            header_field_mark,
                            p.offset_from(header_field_mark) as libc::c_long as size_t,
                        )) as libc::c_int as libc::c_long != 0
                    {
                        (*parser)
                            .set_http_errno(
                                HPE_CB_header_field as libc::c_int as libc::c_uint,
                            );
                    }
                    p_state = (*parser).state() as state;
                    if ((*parser).http_errno() as http_errno as libc::c_uint
                        != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                        as libc::c_long != 0
                    {
                        return p.offset_from(data) as libc::c_long as size_t;
                    }
                }
                header_field_mark = 0 as *const libc::c_char;
            }
            if (*parser).http_errno() as http_errno as libc::c_uint
                == HPE_OK as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                        as *const libc::c_char,
                    b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                    2016 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 94],
                        &[libc::c_char; 94],
                    >(
                        b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            };
            if !header_value_mark.is_null() {
                if ((*settings).on_header_value).is_some() as libc::c_int as libc::c_long
                    != 0
                {
                    (*parser).set_state(p_state as libc::c_uint);
                    if (0 as libc::c_int
                        != ((*settings).on_header_value)
                            .unwrap()(
                            parser,
                            header_value_mark,
                            p.offset_from(header_value_mark) as libc::c_long as size_t,
                        )) as libc::c_int as libc::c_long != 0
                    {
                        (*parser)
                            .set_http_errno(
                                HPE_CB_header_value as libc::c_int as libc::c_uint,
                            );
                    }
                    p_state = (*parser).state() as state;
                    if ((*parser).http_errno() as http_errno as libc::c_uint
                        != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                        as libc::c_long != 0
                    {
                        return p.offset_from(data) as libc::c_long as size_t;
                    }
                }
                header_value_mark = 0 as *const libc::c_char;
            }
            if (*parser).http_errno() as http_errno as libc::c_uint
                == HPE_OK as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                        as *const libc::c_char,
                    b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                    2017 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 94],
                        &[libc::c_char; 94],
                    >(
                        b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            };
            if !url_mark.is_null() {
                if ((*settings).on_url).is_some() as libc::c_int as libc::c_long != 0 {
                    (*parser).set_state(p_state as libc::c_uint);
                    if (0 as libc::c_int
                        != ((*settings).on_url)
                            .unwrap()(
                            parser,
                            url_mark,
                            p.offset_from(url_mark) as libc::c_long as size_t,
                        )) as libc::c_int as libc::c_long != 0
                    {
                        (*parser)
                            .set_http_errno(HPE_CB_url as libc::c_int as libc::c_uint);
                    }
                    p_state = (*parser).state() as state;
                    if ((*parser).http_errno() as http_errno as libc::c_uint
                        != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                        as libc::c_long != 0
                    {
                        return p.offset_from(data) as libc::c_long as size_t;
                    }
                }
                url_mark = 0 as *const libc::c_char;
            }
            if (*parser).http_errno() as http_errno as libc::c_uint
                == HPE_OK as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                        as *const libc::c_char,
                    b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                    2018 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 94],
                        &[libc::c_char; 94],
                    >(
                        b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            };
            if !body_mark.is_null() {
                if ((*settings).on_body).is_some() as libc::c_int as libc::c_long != 0 {
                    (*parser).set_state(p_state as libc::c_uint);
                    if (0 as libc::c_int
                        != ((*settings).on_body)
                            .unwrap()(
                            parser,
                            body_mark,
                            p.offset_from(body_mark) as libc::c_long as size_t,
                        )) as libc::c_int as libc::c_long != 0
                    {
                        (*parser)
                            .set_http_errno(HPE_CB_body as libc::c_int as libc::c_uint);
                    }
                    p_state = (*parser).state() as state;
                    if ((*parser).http_errno() as http_errno as libc::c_uint
                        != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                        as libc::c_long != 0
                    {
                        return p.offset_from(data) as libc::c_long as size_t;
                    }
                }
                body_mark = 0 as *const libc::c_char;
            }
            if (*parser).http_errno() as http_errno as libc::c_uint
                == HPE_OK as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"HTTP_PARSER_ERRNO(parser) == HPE_OK\0" as *const u8
                        as *const libc::c_char,
                    b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                    2019 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 94],
                        &[libc::c_char; 94],
                    >(
                        b"size_t http_parser_execute(http_parser *, const http_parser_settings *, const char *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            };
            if !status_mark.is_null() {
                if ((*settings).on_status).is_some() as libc::c_int as libc::c_long != 0
                {
                    (*parser).set_state(p_state as libc::c_uint);
                    if (0 as libc::c_int
                        != ((*settings).on_status)
                            .unwrap()(
                            parser,
                            status_mark,
                            p.offset_from(status_mark) as libc::c_long as size_t,
                        )) as libc::c_int as libc::c_long != 0
                    {
                        (*parser)
                            .set_http_errno(
                                HPE_CB_status as libc::c_int as libc::c_uint,
                            );
                    }
                    p_state = (*parser).state() as state;
                    if ((*parser).http_errno() as http_errno as libc::c_uint
                        != HPE_OK as libc::c_int as libc::c_uint) as libc::c_int
                        as libc::c_long != 0
                    {
                        return p.offset_from(data) as libc::c_long as size_t;
                    }
                }
                status_mark = 0 as *const libc::c_char;
            }
            (*parser).set_state(p_state as libc::c_uint);
            return len;
        }
    };
}
pub unsafe extern "C" fn http_message_needs_eof(
    mut parser: *const http_parser,
) -> libc::c_int {
    if (*parser).type_0() as libc::c_int == HTTP_REQUEST as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*parser).status_code() as libc::c_int / 100 as libc::c_int == 1 as libc::c_int
        || (*parser).status_code() as libc::c_int == 204 as libc::c_int
        || (*parser).status_code() as libc::c_int == 304 as libc::c_int
        || (*parser).flags() as libc::c_int & F_SKIPBODY as libc::c_int != 0
    {
        return 0 as libc::c_int;
    }
    if (*parser).flags() as libc::c_int & F_CHUNKED as libc::c_int != 0
        || (*parser).content_length as libc::c_ulonglong
            != (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                .wrapping_mul(2 as libc::c_ulonglong)
                .wrapping_add(1 as libc::c_ulonglong)
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn http_should_keep_alive(
    mut parser: *const http_parser,
) -> libc::c_int {
    if (*parser).http_major as libc::c_int > 0 as libc::c_int
        && (*parser).http_minor as libc::c_int > 0 as libc::c_int
    {
        if (*parser).flags() as libc::c_int & F_CONNECTION_CLOSE as libc::c_int != 0 {
            return 0 as libc::c_int;
        }
    } else if (*parser).flags() as libc::c_int & F_CONNECTION_KEEP_ALIVE as libc::c_int
        == 0
    {
        return 0 as libc::c_int
    }
    return (http_message_needs_eof(parser) == 0) as libc::c_int;
}
pub unsafe extern "C" fn http_method_str(mut m: http_method) -> *const libc::c_char {
    return if (m as libc::c_uint as libc::c_ulong)
        < (::std::mem::size_of::<[*const libc::c_char; 33]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        method_strings[m as usize]
    } else {
        b"<unknown>\0" as *const u8 as *const libc::c_char
    };
}
pub unsafe extern "C" fn http_parser_init(
    mut parser: *mut http_parser,
    mut t: http_parser_type,
) {
    let mut data: *mut libc::c_void = (*parser).data;
    memset(
        parser as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<http_parser>() as libc::c_ulong,
    );
    (*parser).data = data;
    (*parser).set_type_0(t as libc::c_uint);
    (*parser)
        .set_state(
            (if t as libc::c_uint == HTTP_REQUEST as libc::c_int as libc::c_uint {
                s_start_req as libc::c_int
            } else if t as libc::c_uint == HTTP_RESPONSE as libc::c_int as libc::c_uint {
                s_start_res as libc::c_int
            } else {
                s_start_req_or_res as libc::c_int
            }) as libc::c_uint,
        );
    (*parser).set_http_errno(HPE_OK as libc::c_int as libc::c_uint);
}
pub unsafe extern "C" fn http_parser_settings_init(
    mut settings: *mut http_parser_settings,
) {
    memset(
        settings as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<http_parser_settings>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn http_errno_name(mut err: http_errno) -> *const libc::c_char {
    if (err as size_t)
        < (::std::mem::size_of::<[C2RustUnnamed_0; 33]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong)
    {} else {
        __assert_fail(
            b"((size_t) err) < ARRAY_SIZE(http_strerror_tab)\0" as *const u8
                as *const libc::c_char,
            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
            2101 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"const char *http_errno_name(enum http_errno)\0"))
                .as_ptr(),
        );
    };
    return http_strerror_tab[err as usize].name;
}
pub unsafe extern "C" fn http_errno_description(
    mut err: http_errno,
) -> *const libc::c_char {
    if (err as size_t)
        < (::std::mem::size_of::<[C2RustUnnamed_0; 33]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong)
    {} else {
        __assert_fail(
            b"((size_t) err) < ARRAY_SIZE(http_strerror_tab)\0" as *const u8
                as *const libc::c_char,
            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
            2107 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"const char *http_errno_description(enum http_errno)\0"))
                .as_ptr(),
        );
    };
    return http_strerror_tab[err as usize].description;
}
unsafe extern "C" fn http_parse_host_char(
    mut s: http_host_state,
    ch: libc::c_char,
) -> http_host_state {
    let mut current_block_35: u64;
    match s as libc::c_uint {
        3 | 2 => {
            if ch as libc::c_int == '@' as i32 {
                return s_http_host_start;
            }
            if (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar as libc::c_int
                >= 'a' as i32
                && (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                    as libc::c_int <= 'z' as i32
                || ch as libc::c_int >= '0' as i32 && ch as libc::c_int <= '9' as i32
                || (ch as libc::c_int == '-' as i32 || ch as libc::c_int == '_' as i32
                    || ch as libc::c_int == '.' as i32 || ch as libc::c_int == '!' as i32
                    || ch as libc::c_int == '~' as i32 || ch as libc::c_int == '*' as i32
                    || ch as libc::c_int == '\'' as i32
                    || ch as libc::c_int == '(' as i32
                    || ch as libc::c_int == ')' as i32)
                || ch as libc::c_int == '%' as i32 || ch as libc::c_int == ';' as i32
                || ch as libc::c_int == ':' as i32 || ch as libc::c_int == '&' as i32
                || ch as libc::c_int == '=' as i32 || ch as libc::c_int == '+' as i32
                || ch as libc::c_int == '$' as i32 || ch as libc::c_int == ',' as i32
            {
                return s_http_userinfo;
            }
            current_block_35 = 2891135413264362348;
        }
        4 => {
            if ch as libc::c_int == '[' as i32 {
                return s_http_host_v6_start;
            }
            if (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar as libc::c_int
                >= 'a' as i32
                && (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                    as libc::c_int <= 'z' as i32
                || ch as libc::c_int >= '0' as i32 && ch as libc::c_int <= '9' as i32
                || ch as libc::c_int == '.' as i32 || ch as libc::c_int == '-' as i32
            {
                return s_http_host;
            }
            current_block_35 = 2891135413264362348;
        }
        6 => {
            if (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar as libc::c_int
                >= 'a' as i32
                && (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                    as libc::c_int <= 'z' as i32
                || ch as libc::c_int >= '0' as i32 && ch as libc::c_int <= '9' as i32
                || ch as libc::c_int == '.' as i32 || ch as libc::c_int == '-' as i32
            {
                return s_http_host;
            }
            current_block_35 = 9476677655169352389;
        }
        8 => {
            current_block_35 = 9476677655169352389;
        }
        7 => {
            if ch as libc::c_int == ']' as i32 {
                return s_http_host_v6_end;
            }
            current_block_35 = 206894081381495661;
        }
        5 => {
            current_block_35 = 206894081381495661;
        }
        10 => {
            if ch as libc::c_int == ']' as i32 {
                return s_http_host_v6_end;
            }
            current_block_35 = 3389598844371035468;
        }
        9 => {
            current_block_35 = 3389598844371035468;
        }
        12 | 11 => {
            if ch as libc::c_int >= '0' as i32 && ch as libc::c_int <= '9' as i32 {
                return s_http_host_port;
            }
            current_block_35 = 2891135413264362348;
        }
        _ => {
            current_block_35 = 2891135413264362348;
        }
    }
    match current_block_35 {
        206894081381495661 => {
            if ch as libc::c_int >= '0' as i32 && ch as libc::c_int <= '9' as i32
                || (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                    as libc::c_int >= 'a' as i32
                    && (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                        as libc::c_int <= 'f' as i32 || ch as libc::c_int == ':' as i32
                || ch as libc::c_int == '.' as i32
            {
                return s_http_host_v6;
            }
            if s as libc::c_uint == s_http_host_v6 as libc::c_int as libc::c_uint
                && ch as libc::c_int == '%' as i32
            {
                return s_http_host_v6_zone_start;
            }
        }
        9476677655169352389 => {
            if ch as libc::c_int == ':' as i32 {
                return s_http_host_port_start;
            }
        }
        3389598844371035468 => {
            if (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar as libc::c_int
                >= 'a' as i32
                && (ch as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar
                    as libc::c_int <= 'z' as i32
                || ch as libc::c_int >= '0' as i32 && ch as libc::c_int <= '9' as i32
                || ch as libc::c_int == '%' as i32 || ch as libc::c_int == '.' as i32
                || ch as libc::c_int == '-' as i32 || ch as libc::c_int == '_' as i32
                || ch as libc::c_int == '~' as i32
            {
                return s_http_host_v6_zone;
            }
        }
        _ => {}
    }
    return s_http_host_dead;
}
unsafe extern "C" fn http_parse_host(
    mut buf: *const libc::c_char,
    mut u: *mut http_parser_url,
    mut found_at: libc::c_int,
) -> libc::c_int {
    let mut s: http_host_state = 0 as http_host_state;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut buflen: size_t = ((*u).field_data[UF_HOST as libc::c_int as usize].off
        as libc::c_int
        + (*u).field_data[UF_HOST as libc::c_int as usize].len as libc::c_int) as size_t;
    if (*u).field_set as libc::c_int & (1 as libc::c_int) << UF_HOST as libc::c_int != 0
    {} else {
        __assert_fail(
            b"u->field_set & (1 << UF_HOST)\0" as *const u8 as *const libc::c_char,
            b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
            2200 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"int http_parse_host(const char *, struct http_parser_url *, int)\0"))
                .as_ptr(),
        );
    };
    (*u).field_data[UF_HOST as libc::c_int as usize].len = 0 as libc::c_int as uint16_t;
    s = (if found_at != 0 {
        s_http_userinfo_start as libc::c_int
    } else {
        s_http_host_start as libc::c_int
    }) as http_host_state;
    p = buf
        .offset(
            (*u).field_data[UF_HOST as libc::c_int as usize].off as libc::c_int as isize,
        );
    while p < buf.offset(buflen as isize) {
        let mut new_s: http_host_state = http_parse_host_char(s, *p);
        if new_s as libc::c_uint == s_http_host_dead as libc::c_int as libc::c_uint {
            return 1 as libc::c_int;
        }
        match new_s as libc::c_uint {
            6 => {
                if s as libc::c_uint != s_http_host as libc::c_int as libc::c_uint {
                    (*u)
                        .field_data[UF_HOST as libc::c_int as usize]
                        .off = p.offset_from(buf) as libc::c_long as uint16_t;
                }
                (*u)
                    .field_data[UF_HOST as libc::c_int as usize]
                    .len = ((*u).field_data[UF_HOST as libc::c_int as usize].len)
                    .wrapping_add(1);
                (*u).field_data[UF_HOST as libc::c_int as usize].len;
            }
            7 => {
                if s as libc::c_uint != s_http_host_v6 as libc::c_int as libc::c_uint {
                    (*u)
                        .field_data[UF_HOST as libc::c_int as usize]
                        .off = p.offset_from(buf) as libc::c_long as uint16_t;
                }
                (*u)
                    .field_data[UF_HOST as libc::c_int as usize]
                    .len = ((*u).field_data[UF_HOST as libc::c_int as usize].len)
                    .wrapping_add(1);
                (*u).field_data[UF_HOST as libc::c_int as usize].len;
            }
            9 | 10 => {
                (*u)
                    .field_data[UF_HOST as libc::c_int as usize]
                    .len = ((*u).field_data[UF_HOST as libc::c_int as usize].len)
                    .wrapping_add(1);
                (*u).field_data[UF_HOST as libc::c_int as usize].len;
            }
            12 => {
                if s as libc::c_uint != s_http_host_port as libc::c_int as libc::c_uint {
                    (*u)
                        .field_data[UF_PORT as libc::c_int as usize]
                        .off = p.offset_from(buf) as libc::c_long as uint16_t;
                    (*u)
                        .field_data[UF_PORT as libc::c_int as usize]
                        .len = 0 as libc::c_int as uint16_t;
                    (*u)
                        .field_set = ((*u).field_set as libc::c_int
                        | (1 as libc::c_int) << UF_PORT as libc::c_int) as uint16_t;
                }
                (*u)
                    .field_data[UF_PORT as libc::c_int as usize]
                    .len = ((*u).field_data[UF_PORT as libc::c_int as usize].len)
                    .wrapping_add(1);
                (*u).field_data[UF_PORT as libc::c_int as usize].len;
            }
            3 => {
                if s as libc::c_uint != s_http_userinfo as libc::c_int as libc::c_uint {
                    (*u)
                        .field_data[UF_USERINFO as libc::c_int as usize]
                        .off = p.offset_from(buf) as libc::c_long as uint16_t;
                    (*u)
                        .field_data[UF_USERINFO as libc::c_int as usize]
                        .len = 0 as libc::c_int as uint16_t;
                    (*u)
                        .field_set = ((*u).field_set as libc::c_int
                        | (1 as libc::c_int) << UF_USERINFO as libc::c_int) as uint16_t;
                }
                (*u)
                    .field_data[UF_USERINFO as libc::c_int as usize]
                    .len = ((*u).field_data[UF_USERINFO as libc::c_int as usize].len)
                    .wrapping_add(1);
                (*u).field_data[UF_USERINFO as libc::c_int as usize].len;
            }
            _ => {}
        }
        s = new_s;
        p = p.offset(1);
        p;
    }
    match s as libc::c_uint {
        4 | 5 | 7 | 9 | 10 | 11 | 3 | 2 => return 1 as libc::c_int,
        _ => {}
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn http_parser_url_init(mut u: *mut http_parser_url) {
    memset(
        u as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<http_parser_url>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn http_parser_parse_url(
    mut buf: *const libc::c_char,
    mut buflen: size_t,
    mut is_connect: libc::c_int,
    mut u: *mut http_parser_url,
) -> libc::c_int {
    let mut s: state = 0 as state;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut uf: http_parser_url_fields = UF_SCHEMA;
    let mut old_uf: http_parser_url_fields = UF_SCHEMA;
    let mut found_at: libc::c_int = 0 as libc::c_int;
    (*u).field_set = 0 as libc::c_int as uint16_t;
    (*u).port = (*u).field_set;
    s = (if is_connect != 0 {
        s_req_server_start as libc::c_int
    } else {
        s_req_spaces_before_url as libc::c_int
    }) as state;
    old_uf = UF_MAX;
    let mut current_block_18: u64;
    p = buf;
    while p < buf.offset(buflen as isize) {
        s = parse_url_char(s, *p);
        match s as libc::c_uint {
            1 => return 1 as libc::c_int,
            22 | 23 | 24 | 28 | 30 => {
                current_block_18 = 17778012151635330486;
            }
            21 => {
                uf = UF_SCHEMA;
                current_block_18 = 5948590327928692120;
            }
            26 => {
                found_at = 1 as libc::c_int;
                current_block_18 = 5231757375760494377;
            }
            25 => {
                current_block_18 = 5231757375760494377;
            }
            27 => {
                uf = UF_PATH;
                current_block_18 = 5948590327928692120;
            }
            29 => {
                uf = UF_QUERY;
                current_block_18 = 5948590327928692120;
            }
            31 => {
                uf = UF_FRAGMENT;
                current_block_18 = 5948590327928692120;
            }
            _ => {
                if (b"Unexpected state\0" as *const u8 as *const libc::c_char).is_null()
                {} else {
                    __assert_fail(
                        b"!\"Unexpected state\"\0" as *const u8 as *const libc::c_char,
                        b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                        2334 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 79],
                            &[libc::c_char; 79],
                        >(
                            b"int http_parser_parse_url(const char *, size_t, int, struct http_parser_url *)\0",
                        ))
                            .as_ptr(),
                    );
                };
                return 1 as libc::c_int;
            }
        }
        match current_block_18 {
            5231757375760494377 => {
                uf = UF_HOST;
                current_block_18 = 5948590327928692120;
            }
            _ => {}
        }
        match current_block_18 {
            5948590327928692120 => {
                if uf as libc::c_uint == old_uf as libc::c_uint {
                    (*u)
                        .field_data[uf as usize]
                        .len = ((*u).field_data[uf as usize].len).wrapping_add(1);
                    (*u).field_data[uf as usize].len;
                } else {
                    (*u)
                        .field_data[uf as usize]
                        .off = p.offset_from(buf) as libc::c_long as uint16_t;
                    (*u).field_data[uf as usize].len = 1 as libc::c_int as uint16_t;
                    (*u)
                        .field_set = ((*u).field_set as libc::c_int
                        | (1 as libc::c_int) << uf as libc::c_uint) as uint16_t;
                    old_uf = uf;
                }
            }
            _ => {}
        }
        p = p.offset(1);
        p;
    }
    if (*u).field_set as libc::c_int & (1 as libc::c_int) << UF_SCHEMA as libc::c_int
        != 0
        && (*u).field_set as libc::c_int & (1 as libc::c_int) << UF_HOST as libc::c_int
            == 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if (*u).field_set as libc::c_int & (1 as libc::c_int) << UF_HOST as libc::c_int != 0
    {
        if http_parse_host(buf, u, found_at) != 0 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    if is_connect != 0
        && (*u).field_set as libc::c_int
            != (1 as libc::c_int) << UF_HOST as libc::c_int
                | (1 as libc::c_int) << UF_PORT as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if (*u).field_set as libc::c_int & (1 as libc::c_int) << UF_PORT as libc::c_int != 0
    {
        let mut v: libc::c_ulong = strtoul(
            buf
                .offset(
                    (*u).field_data[UF_PORT as libc::c_int as usize].off as libc::c_int
                        as isize,
                ),
            0 as *mut *mut libc::c_char,
            10 as libc::c_int,
        );
        if v > 0xffff as libc::c_int as libc::c_ulong {
            return 1 as libc::c_int;
        }
        (*u).port = v as uint16_t;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn http_parser_pause(
    mut parser: *mut http_parser,
    mut paused: libc::c_int,
) {
    if (*parser).http_errno() as http_errno as libc::c_uint
        == HPE_OK as libc::c_int as libc::c_uint
        || (*parser).http_errno() as http_errno as libc::c_uint
            == HPE_PAUSED as libc::c_int as libc::c_uint
    {
        (*parser)
            .set_http_errno(
                (if paused != 0 {
                    HPE_PAUSED as libc::c_int
                } else {
                    HPE_OK as libc::c_int
                }) as libc::c_uint,
            );
    } else {
        if 0 as libc::c_int != 0
            && !(b"Attempting to pause parser in error state\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"0 && \"Attempting to pause parser in error state\"\0" as *const u8
                    as *const libc::c_char,
                b"src/http_parser.c\0" as *const u8 as *const libc::c_char,
                2394 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"void http_parser_pause(http_parser *, int)\0"))
                    .as_ptr(),
            );
        };
    };
}
pub unsafe extern "C" fn http_body_is_final(
    mut parser: *const http_parser,
) -> libc::c_int {
    return ((*parser).state() as libc::c_int == s_message_done as libc::c_int)
        as libc::c_int;
}
pub unsafe extern "C" fn http_parser_version() -> libc::c_ulong {
    return (2 as libc::c_int * 0x10000 as libc::c_int
        | 7 as libc::c_int * 0x100 as libc::c_int
        | 1 as libc::c_int * 0x1 as libc::c_int) as libc::c_ulong;
}
